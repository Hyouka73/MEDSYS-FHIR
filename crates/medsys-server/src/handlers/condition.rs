//! Manejadores HTTP para el recurso FHIR `Condition` (Diagnósticos CIE-10).

use axum::extract::{Path, Query, State};
use helios_fhir::r4::Resource;
use medsys_core::{create_searchset_bundle, transform_condition};
use serde::Deserialize;

use crate::error::ServerError;
use crate::response::FhirResponse;
use crate::state::AppState;

/// Parámetros de consulta para búsqueda de diagnósticos.
#[derive(Debug, Deserialize, Default)]
pub struct ConditionQueryParams {
    pub patient: Option<i32>,
    pub encounter: Option<i32>,
    pub code: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl ConditionQueryParams {
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(50).clamp(1, 200)
    }

    pub fn offset(&self) -> i64 {
        self.offset.unwrap_or(0).max(0)
    }
}

/// Endpoint canónico: `GET /fhir/r4/Condition/{id}`
/// Busca el diagnóstico clínico por `id_diagnostico` (o formato canónico `cond-{id}`)
/// y lo retorna como recurso `Condition` con codificación CIE-10.
pub async fn get_condition(
    State(state): State<AppState>,
    Path(raw_id): Path<String>,
) -> Result<FhirResponse, ServerError> {
    let clean_id = raw_id.strip_prefix("cond-").unwrap_or(&raw_id);
    let id: i32 = clean_id.parse().map_err(|_| {
        ServerError::InvalidPath(format!("Identificador de condición inválido: '{raw_id}'"))
    })?;

    let diagnostico = match state.mapping_rules.resources.iter().find(|r| r.resource_type == "Condition") {
        Some(m) if m.source_table != "tbl_diagnosticos" => {
            let row = state.repositories.dynamic.find_by_id(m, id).await?;
            medsys_db::row_to_legacy_diagnostico(&row)?
        }
        _ => {
            let d = state.repositories.diagnosticos.find_by_id(id).await?;
            state
                .repositories
                .verify_encounter_and_patient_exist(d.id_consulta, d.id_paciente)
                .await?;
            d
        }
    };
    let fhir_cond = transform_condition(&diagnostico, Some(&state.mapping_rules))?;

    Ok(FhirResponse(Resource::Condition(Box::new(fhir_cond))))
}

/// Endpoint canónico: `GET /fhir/r4/Condition`
/// Retorna la colección de afecciones clínicas codificadas en CIE-10
/// empaquetadas en un `Bundle` FHIR de tipo `searchset`.
pub async fn list_conditions(
    State(state): State<AppState>,
    Query(params): Query<ConditionQueryParams>,
) -> Result<FhirResponse, ServerError> {
    let diagnosticos = match state.mapping_rules.resources.iter().find(|r| r.resource_type == "Condition") {
        Some(m) if m.source_table != "tbl_diagnosticos" => {
            let rows = if let Some(enc_id) = params.encounter {
                state
                    .repositories
                    .dynamic
                    .find_filtered_raw(
                        m,
                        "id_consulta",
                        enc_id,
                        params.limit(),
                        params.offset(),
                    )
                    .await?
            } else if let Some(patient_id) = params.patient {
                state
                    .repositories
                    .dynamic
                    .find_filtered_raw(
                        m,
                        "id_paciente",
                        patient_id,
                        params.limit(),
                        params.offset(),
                    )
                    .await?
            } else if let Some(ref cie10) = params.code {
                let all_rows = state
                    .repositories
                    .dynamic
                    .find_all_raw(m, params.limit(), params.offset())
                    .await?;
                all_rows
                    .into_iter()
                    .filter(|r| {
                        r.get("codigo_cie10").and_then(|v| v.as_str()) == Some(cie10.as_str())
                    })
                    .collect()
            } else {
                state
                    .repositories
                    .dynamic
                    .find_all_raw(m, params.limit(), params.offset())
                    .await?
            };
            let mut list = Vec::with_capacity(rows.len());
            for r in &rows {
                list.push(medsys_db::row_to_legacy_diagnostico(r)?);
            }
            list
        }
        _ => {
            let all = if let Some(enc_id) = params.encounter {
                state
                    .repositories
                    .diagnosticos
                    .find_by_consulta_id(enc_id)
                    .await?
            } else if let Some(patient_id) = params.patient {
                state
                    .repositories
                    .diagnosticos
                    .find_by_paciente_id(patient_id)
                    .await?
            } else if let Some(ref cie10) = params.code {
                state
                    .repositories
                    .diagnosticos
                    .find_by_codigo_cie10(cie10)
                    .await?
            } else {
                state.repositories.diagnosticos.find_all().await?
            };

            let total = all.len();
            let offset = params.offset() as usize;
            let limit = params.limit() as usize;
            if offset < total {
                all.into_iter().skip(offset).take(limit).collect()
            } else {
                Vec::new()
            }
        }
    };

    let count = diagnosticos.len();
    let mut resources = Vec::with_capacity(count);

    for d in diagnosticos {
        let fhir_cond = transform_condition(&d, Some(&state.mapping_rules))?;
        resources.push(Resource::Condition(Box::new(fhir_cond)));
    }

    let bundle = create_searchset_bundle(Some("bundle-conditions"), resources, Some(count));

    Ok(FhirResponse(Resource::Bundle(Box::new(bundle))))
}
