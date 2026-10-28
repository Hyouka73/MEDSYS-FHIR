//! Manejadores HTTP para el recurso FHIR `Condition` (Diagnósticos CIE-10).

use axum::extract::{rejection::QueryRejection, OriginalUri, Path, Query, State};
use helios_fhir::r4::Resource;
use medsys_core::transform_condition;
use serde::Deserialize;

use super::common::{bundle_page, normalize_row, output_rules, parse_query, read_mapping};
use crate::error::ServerError;
use crate::response::FhirResponse;
use crate::state::AppState;

/// Parámetros de consulta para búsqueda de diagnósticos.
#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
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

    let rules = output_rules(&state);
    let mapping = read_mapping(&state, "Condition")?;
    let row = state.repositories.dynamic.find_by_id(&mapping, id).await?;
    let diagnostico = medsys_db::row_to_legacy_diagnostico(&normalize_row(&state, row)?)?;

    let fhir_cond = transform_condition(&diagnostico, Some(&rules))?;

    Ok(FhirResponse(Resource::Condition(Box::new(fhir_cond))))
}

/// Endpoint canónico: `GET /fhir/r4/Condition`
/// Retorna la colección de afecciones clínicas codificadas en CIE-10
/// empaquetadas en un `Bundle` FHIR de tipo `searchset`.
pub async fn list_conditions(
    State(state): State<AppState>,
    OriginalUri(uri): OriginalUri,
    query: Result<Query<ConditionQueryParams>, QueryRejection>,
) -> Result<FhirResponse, ServerError> {
    let params = parse_query(query)?;
    let rules = output_rules(&state);
    let mapping = read_mapping(&state, "Condition")?;
    let rows = state
        .repositories
        .dynamic
        .find_all_raw(&mapping, i64::MAX, 0)
        .await?;
    let mut resources = Vec::new();
    for row in rows {
        let diagnostico = medsys_db::row_to_legacy_diagnostico(&normalize_row(&state, row)?)?;
        if params
            .patient
            .is_some_and(|id| diagnostico.id_paciente != id)
        {
            continue;
        }
        if params
            .encounter
            .is_some_and(|id| diagnostico.id_consulta != id)
        {
            continue;
        }
        if params
            .code
            .as_ref()
            .is_some_and(|code| diagnostico.codigo_cie10 != *code)
        {
            continue;
        }
        let value = transform_condition(&diagnostico, Some(&rules))?;
        resources.push(Resource::Condition(Box::new(value)));
    }
    Ok(bundle_page(
        resources,
        params.limit(),
        params.offset(),
        &uri,
    ))
}
