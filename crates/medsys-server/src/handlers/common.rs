//! Contrato de lectura FHIR común a S0–S3.
use crate::{error::ServerError, response::FhirResponse, state::AppState};
use axum::{
    extract::{rejection::QueryRejection, Query},
    http::Uri,
};
use helios_fhir::r4::{BundleLink, Resource};
use medsys_core::model::fhir_helpers::{fhir_string, fhir_uri};
use medsys_core::{create_searchset_bundle, ParentCheckRule, ResourceMapping};

pub fn parse_query<T>(query: Result<Query<T>, QueryRejection>) -> Result<T, ServerError> {
    query
        .map(|Query(params)| params)
        .map_err(|e| ServerError::InvalidPath(e.body_text()))
}

/// Proyecta todas las columnas consumidas por el adaptador tipado. La unión de
/// Observation incluye los cuatro subtipos expuestos; body_weight no se expone.
pub fn read_mapping(state: &AppState, resource_type: &str) -> Result<ResourceMapping, ServerError> {
    let mut mappings = state.mapping_rules.resources.iter().filter(|m| {
        m.resource_type == resource_type
            && (resource_type != "Observation"
                || m.observation_type.as_deref() != Some("body_weight"))
    });
    let mut result = mappings.next().cloned().ok_or_else(|| {
        medsys_core::MedSysError::InvalidRule(format!("Falta mapeo {resource_type}"))
    })?;
    if resource_type == "Observation" {
        result.filter_not_null = None;
        for other in mappings {
            if other.source_table != result.source_table || other.primary_key != result.primary_key
            {
                return Err(medsys_core::MedSysError::InvalidRule("Los subtipos Observation requieren tabla y clave comunes para el adaptador tipado".into()).into());
            }
            for field in &other.mappings {
                if !result.mappings.contains(field) {
                    result.mappings.push(field.clone());
                }
            }
            for parent in &other.parent_checks {
                if !result.parent_checks.contains(parent) {
                    result.parent_checks.push(parent.clone());
                }
            }
        }
    }
    // Las referencias declaradas también se comprueban cuando el DDL carece de FK.
    for (target, parent_type) in [
        ("subject.reference", "Patient"),
        ("encounter.reference", "Encounter"),
    ] {
        if let Some(child_column) = result
            .mappings
            .iter()
            .find(|f| f.target_path == target)
            .and_then(|f| f.source_column.clone())
        {
            let parent = state
                .mapping_rules
                .resources
                .iter()
                .find(|m| m.resource_type == parent_type)
                .ok_or_else(|| {
                    medsys_core::MedSysError::InvalidRule(format!("Falta padre {parent_type}"))
                })?;
            let rule = ParentCheckRule {
                foreign_key_column: child_column,
                parent_table: parent.source_table.clone(),
                parent_pk: parent.primary_key.clone(),
            };
            if !result.parent_checks.contains(&rule) {
                result.parent_checks.push(rule);
            }
        }
    }
    Ok(result)
}

pub fn normalize_row(
    state: &AppState,
    row: serde_json::Map<String, serde_json::Value>,
) -> Result<serde_json::Map<String, serde_json::Value>, ServerError> {
    Ok(medsys_db::normalize_row_timezone(
        row,
        &state.mapping_rules.timezone_default,
    )?)
}

/// Los adaptadores recibieron relojes UTC; los constructores deben conservar
/// esa interpretación, manteniendo el resto de metadatos y diccionarios YAML.
pub fn output_rules(state: &AppState) -> medsys_core::MappingRules {
    let mut rules = state.mapping_rules.as_ref().clone();
    rules.timezone_default = "UTC".into();
    rules
}

/// Conserva precisión de fecha cuando no se observó ninguna hora de origen.
pub fn with_date_precision(
    mut resource: Resource,
    row: &serde_json::Map<String, serde_json::Value>,
    mapping: &ResourceMapping,
) -> Result<Resource, ServerError> {
    for field in &mapping.mappings {
        let Some(column) = field.source_column.as_ref() else {
            continue;
        };
        let Some(date) = row
            .get(&format!("__date_precision_{column}"))
            .and_then(serde_json::Value::as_str)
        else {
            continue;
        };
        let date = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .map_err(|e| medsys_core::MedSysError::Internal(e.to_string()))?;
        let value = medsys_core::model::fhir_helpers::fhir_date_as_datetime(date)?;
        match (&mut resource, field.target_path.as_str()) {
            (Resource::Observation(obs), "effectiveDateTime") => {
                obs.effective = Some(helios_fhir::r4::ObservationEffective::DateTime(value));
            }
            (Resource::Encounter(encounter), "period.start") => {
                if let Some(period) = &mut encounter.period {
                    period.start = Some(value);
                }
            }
            (Resource::Encounter(encounter), "period.end") => {
                if let Some(period) = &mut encounter.period {
                    period.end = Some(value);
                }
            }
            _ => {}
        }
    }
    Ok(resource)
}

/// Pagina recursos, no filas SQL. total cuenta todas las coincidencias antes de paginar.
pub fn bundle_page(resources: Vec<Resource>, limit: i64, offset: i64, uri: &Uri) -> FhirResponse {
    let total = resources.len();
    let page = resources
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect();
    let mut bundle = create_searchset_bundle(None, page, Some(total));
    let base_params: Vec<_> = uri
        .query()
        .unwrap_or("")
        .split('&')
        .filter(|part| {
            let key = part.split('=').next().unwrap_or("");
            !part.is_empty() && key != "limit" && key != "offset"
        })
        .collect();
    let link = |relation: &str, start: i64| {
        let mut params = base_params.clone();
        let count = format!("limit={limit}");
        let page = format!("offset={start}");
        params.push(&count);
        params.push(&page);
        BundleLink {
            relation: fhir_string(relation),
            url: fhir_uri(format!("{}?{}", uri.path(), params.join("&"))),
            ..Default::default()
        }
    };
    let mut links = vec![link("self", offset)];
    if offset.saturating_add(limit) < total as i64 {
        links.push(link("next", offset + limit));
    }
    if offset > 0 {
        links.push(link("previous", offset.saturating_sub(limit).max(0)));
    }
    bundle.link = Some(links);
    FhirResponse(Resource::Bundle(Box::new(bundle)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use medsys_db::{init_pool_lazy, DbConfig, DynamicSqlBuilder, MedsysRepositories};
    #[tokio::test]
    async fn observation_projection_contains_all_adaptor_fields_s0_s3() {
        for (yaml, expected) in [
            (
                include_str!("../../../../config/mapping_rules_s0.yaml"),
                [
                    "presion_sistolica",
                    "frecuencia_cardiaca",
                    "temperatura_celsius",
                    "frecuencia_respiratoria",
                ],
            ),
            (
                include_str!("../../../../config/mapping_rules_s1.yaml"),
                [
                    "presion_sistolica",
                    "frecuencia_cardiaca",
                    "temperatura_celsius",
                    "frecuencia_respiratoria",
                ],
            ),
            (
                include_str!("../../../../config/mapping_rules_s2.yaml"),
                [
                    "presion_sistolica",
                    "frecuencia_cardiaca",
                    "temperatura_celsius",
                    "frecuencia_respiratoria",
                ],
            ),
            (
                include_str!("../../../../config/mapping_rules_s3.yaml"),
                ["presion_arterial_txt", "fc", "temp_celsius", "fr"],
            ),
        ] {
            let pool = init_pool_lazy(&DbConfig::default()).unwrap();
            let state = AppState::new(
                MedsysRepositories::new(pool.clone()),
                medsys_core::parse_mapping_rules(yaml).unwrap(),
                pool,
            );
            let mapping = read_mapping(&state, "Observation").unwrap();
            let sql = DynamicSqlBuilder::select_by_id(&mapping);
            println!("{}: {sql}", mapping.source_table);
            for column in expected {
                assert!(
                    sql.contains(&format!("t0.{column}")),
                    "missing {column}: {sql}"
                );
            }
            assert!(!sql.contains("peso_libras"));
            assert!(!sql.contains("IS NOT NULL"));
            assert_eq!(mapping.parent_checks.len(), 2);
        }
    }
    #[test]
    fn resource_page_total_and_links_are_global() {
        let resources = (0..5).map(|_| Resource::Bundle(Box::default())).collect();
        let FhirResponse(Resource::Bundle(bundle)) = bundle_page(
            resources,
            2,
            2,
            &"/fhir/r4/Observation?patient=1&limit=2&offset=2"
                .parse()
                .unwrap(),
        ) else {
            panic!("bundle");
        };
        let v = serde_json::to_value(bundle).unwrap();
        assert_eq!(v["total"], 5);
        assert_eq!(v["entry"].as_array().unwrap().len(), 2);
        assert_eq!(
            v["link"][1]["url"],
            "/fhir/r4/Observation?patient=1&limit=2&offset=4"
        );
    }
    #[test]
    fn source_date_precision_is_restored_without_observed_midnight() {
        let rules = medsys_core::parse_mapping_rules(include_str!(
            "../../../../config/mapping_rules_s3.yaml"
        ))
        .unwrap();
        let mapping = rules
            .get_observation_mapping("blood_pressure_panel")
            .unwrap();
        let row = serde_json::json!({"__date_precision_fech_reg_txt":"2026-01-01"})
            .as_object()
            .unwrap()
            .clone();
        let resource =
            with_date_precision(Resource::Observation(Box::default()), &row, mapping).unwrap();
        assert_eq!(
            serde_json::to_value(resource).unwrap()["effectiveDateTime"],
            "2026-01-01"
        );
        let mapping = rules
            .get_resource_mapping(medsys_core::SupportedResource::Encounter)
            .unwrap();
        let row = serde_json::json!({"__date_precision_fech_ini_txt":"2026-01-01", "__date_precision_fech_fin_txt":"2026-01-02"}).as_object().unwrap().clone();
        let encounter = helios_fhir::r4::Encounter {
            period: Some(Default::default()),
            ..Default::default()
        };
        let resource =
            with_date_precision(Resource::Encounter(Box::new(encounter)), &row, mapping).unwrap();
        let value = serde_json::to_value(resource).unwrap();
        assert_eq!(value["period"]["start"], "2026-01-01");
        assert_eq!(value["period"]["end"], "2026-01-02");
    }
}
