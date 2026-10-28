//! Manejadores HTTP para el recurso FHIR `Patient`.

use axum::extract::{rejection::QueryRejection, OriginalUri, Path, Query, State};
use helios_fhir::r4::Resource;
use medsys_core::transform_patient;
use serde::Deserialize;

use super::common::{bundle_page, normalize_row, output_rules, parse_query, read_mapping};
use crate::error::ServerError;
use crate::response::FhirResponse;
use crate::state::AppState;

/// Parámetros opcionales de paginación para consultas de colección.
#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct PaginationParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl PaginationParams {
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(50).clamp(1, 200)
    }

    pub fn offset(&self) -> i64 {
        self.offset.unwrap_or(0).max(0)
    }
}

/// Endpoint canónico: `GET /fhir/r4/Patient/{id}`
/// Busca el paciente por su identificador relacional y lo retorna transformado
/// al recurso estándar `Patient` con CURP oficial.
pub async fn get_patient(
    State(state): State<AppState>,
    Path(raw_id): Path<String>,
) -> Result<FhirResponse, ServerError> {
    let clean_id = raw_id.strip_prefix("pat-").unwrap_or(&raw_id);
    let id: i32 = clean_id.parse().map_err(|_| {
        ServerError::InvalidPath(format!("Identificador de paciente inválido: '{raw_id}'"))
    })?;

    let rules = output_rules(&state);
    let mapping = read_mapping(&state, "Patient")?;
    let row = state.repositories.dynamic.find_by_id(&mapping, id).await?;
    let legacy_paciente = medsys_db::row_to_legacy_paciente(&normalize_row(&state, row)?)?;

    let fhir_patient = transform_patient(&legacy_paciente, Some(&rules))?;

    Ok(FhirResponse(Resource::Patient(Box::new(fhir_patient))))
}

/// Endpoint canónico: `GET /fhir/r4/Patient`
/// Retorna la colección de pacientes encapsulada en un `Bundle` FHIR de tipo `searchset`.
pub async fn list_patients(
    State(state): State<AppState>,
    OriginalUri(uri): OriginalUri,
    query: Result<Query<PaginationParams>, QueryRejection>,
) -> Result<FhirResponse, ServerError> {
    let params = parse_query(query)?;
    let rules = output_rules(&state);
    let mapping = read_mapping(&state, "Patient")?;
    let rows = state
        .repositories
        .dynamic
        .find_all_raw(&mapping, i64::MAX, 0)
        .await?;
    let mut resources = Vec::new();
    for row in rows {
        let paciente = medsys_db::row_to_legacy_paciente(&normalize_row(&state, row)?)?;
        let value = transform_patient(&paciente, Some(&rules))?;
        resources.push(Resource::Patient(Box::new(value)));
    }
    Ok(bundle_page(
        resources,
        params.limit(),
        params.offset(),
        &uri,
    ))
}
