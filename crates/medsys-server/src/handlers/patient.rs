//! Manejadores HTTP para el recurso FHIR `Patient`.

use axum::extract::{Path, Query, State};
use helios_fhir::r4::Resource;
use medsys_core::{create_searchset_bundle, transform_patient};
use serde::Deserialize;

use crate::error::ServerError;
use crate::response::FhirResponse;
use crate::state::AppState;

/// Parámetros opcionales de paginación para consultas de colección.
#[derive(Debug, Deserialize, Default)]
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

    let legacy_paciente = state.repositories.pacientes.find_by_id(id).await?;
    let fhir_patient = transform_patient(&legacy_paciente, Some(&state.mapping_rules))?;

    Ok(FhirResponse(Resource::Patient(Box::new(fhir_patient))))
}

/// Endpoint canónico: `GET /fhir/r4/Patient`
/// Retorna la colección de pacientes encapsulada en un `Bundle` FHIR de tipo `searchset`.
pub async fn list_patients(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<FhirResponse, ServerError> {
    let mut pacientes = state.repositories.pacientes.find_all().await?;
    let total = pacientes.len();
    let offset = params.offset() as usize;
    let limit = params.limit() as usize;
    if offset < total {
        pacientes = pacientes.into_iter().skip(offset).take(limit).collect();
    } else {
        pacientes.clear();
    }

    let count = pacientes.len();
    let mut resources = Vec::with_capacity(count);

    for p in pacientes {
        let fhir_p = transform_patient(&p, Some(&state.mapping_rules))?;
        resources.push(Resource::Patient(Box::new(fhir_p)));
    }

    let bundle = create_searchset_bundle(Some("bundle-patients"), resources, Some(count));

    Ok(FhirResponse(Resource::Bundle(Box::new(bundle))))
}
