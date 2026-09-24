//! Manejadores HTTP para el recurso FHIR `Encounter`.

use axum::extract::{Path, Query, State};
use helios_fhir::r4::Resource;
use medsys_core::{create_searchset_bundle, transform_encounter};
use serde::Deserialize;

use crate::error::ServerError;
use crate::response::FhirResponse;
use crate::state::AppState;

/// Parámetros de consulta para búsqueda de encuentros.
#[derive(Debug, Deserialize, Default)]
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
    Path(id): Path<i32>,
) -> Result<FhirResponse, ServerError> {
    let legacy_consulta = state.repositories.consultas.find_by_id(id).await?;
    let fhir_encounter = transform_encounter(&legacy_consulta, Some(&state.mapping_rules))?;

    Ok(FhirResponse(Resource::Encounter(Box::new(fhir_encounter))))
}

/// Endpoint canónico: `GET /fhir/r4/Encounter`
/// Retorna la colección de encuentros (opcionalmente filtrados por paciente `?patient=1`)
/// empaquetada en un `Bundle` de tipo `searchset`.
pub async fn list_encounters(
    State(state): State<AppState>,
    Query(params): Query<EncounterQueryParams>,
) -> Result<FhirResponse, ServerError> {
    let mut consultas = if let Some(patient_id) = params.patient {
        state
            .repositories
            .consultas
            .find_by_paciente_id(patient_id)
            .await?
    } else {
        state.repositories.consultas.find_all().await?
    };

    let total = consultas.len();
    let offset = params.offset() as usize;
    let limit = params.limit() as usize;
    if offset < total {
        consultas = consultas.into_iter().skip(offset).take(limit).collect();
    } else {
        consultas.clear();
    }

    let count = consultas.len();
    let mut resources = Vec::with_capacity(count);

    for c in consultas {
        let fhir_enc = transform_encounter(&c, Some(&state.mapping_rules))?;
        resources.push(Resource::Encounter(Box::new(fhir_enc)));
    }

    let bundle = create_searchset_bundle(Some("bundle-encounters"), resources, Some(count));

    Ok(FhirResponse(Resource::Bundle(Box::new(bundle))))
}
