//! Manejadores HTTP para el recurso FHIR `Encounter`.

use axum::extract::{rejection::QueryRejection, OriginalUri, Path, Query, State};
use helios_fhir::r4::Resource;
use medsys_core::transform_encounter;
use serde::Deserialize;

use super::common::{
    bundle_page, normalize_row, output_rules, parse_query, read_mapping, with_date_precision,
};
use crate::error::ServerError;
use crate::response::FhirResponse;
use crate::state::AppState;

/// Parámetros de consulta para búsqueda de encuentros.
#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct EncounterQueryParams {
    pub patient: Option<i32>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl EncounterQueryParams {
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(50).clamp(1, 200)
    }

    pub fn offset(&self) -> i64 {
        self.offset.unwrap_or(0).max(0)
    }
}

/// Endpoint canónico: `GET /fhir/r4/Encounter/{id}`
/// Busca la consulta médica por `id_consulta` y la transforma en el recurso `Encounter`
/// con clase ambulatoria AMB y cédula SEP del médico tratante.
pub async fn get_encounter(
    State(state): State<AppState>,
    Path(raw_id): Path<String>,
) -> Result<FhirResponse, ServerError> {
    let clean_id = raw_id.strip_prefix("enc-").unwrap_or(&raw_id);
    let id: i32 = clean_id.parse().map_err(|_| {
        ServerError::InvalidPath(format!("Identificador de encuentro inválido: '{raw_id}'"))
    })?;

    let rules = output_rules(&state);
    let mapping = read_mapping(&state, "Encounter")?;
    let row = state.repositories.dynamic.find_by_id(&mapping, id).await?;
    let row = normalize_row(&state, row)?;
    let legacy_consulta = medsys_db::row_to_legacy_consulta(&row)?;

    let fhir_encounter = transform_encounter(&legacy_consulta, Some(&rules))?;

    Ok(FhirResponse(with_date_precision(
        Resource::Encounter(Box::new(fhir_encounter)),
        &row,
        &mapping,
    )?))
}

/// Endpoint canónico: `GET /fhir/r4/Encounter`
/// Retorna la colección de encuentros (opcionalmente filtrados por paciente `?patient=1`)
/// empaquetada en un `Bundle` de tipo `searchset`.
pub async fn list_encounters(
    State(state): State<AppState>,
    OriginalUri(uri): OriginalUri,
    query: Result<Query<EncounterQueryParams>, QueryRejection>,
) -> Result<FhirResponse, ServerError> {
    let params = parse_query(query)?;
    let rules = output_rules(&state);
    let mapping = read_mapping(&state, "Encounter")?;
    let rows = state
        .repositories
        .dynamic
        .find_all_raw(&mapping, i64::MAX, 0)
        .await?;
    let mut resources = Vec::new();
    for row in rows {
        let row = normalize_row(&state, row)?;
        let consulta = medsys_db::row_to_legacy_consulta(&row)?;
        if params.patient.is_some_and(|id| consulta.id_paciente != id) {
            continue;
        }
        let value = transform_encounter(&consulta, Some(&rules))?;
        resources.push(with_date_precision(
            Resource::Encounter(Box::new(value)),
            &row,
            &mapping,
        )?);
    }
    Ok(bundle_page(
        resources,
        params.limit(),
        params.offset(),
        &uri,
    ))
}
