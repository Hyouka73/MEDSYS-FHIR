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

    let legacy_paciente = match state.mapping_rules.resources.iter().find(|r| r.resource_type == "Patient") {
        Some(m) if m.source_table != "tbl_pacientes" => {
            let row = state.repositories.dynamic.find_by_id(m, id).await?;
            medsys_db::row_to_legacy_paciente(&row)?
        }
        _ => state.repositories.pacientes.find_by_id(id).await?,
    };
    let fhir_patient = transform_patient(&legacy_paciente, Some(&state.mapping_rules))?;

    Ok(FhirResponse(Resource::Patient(Box::new(fhir_patient))))
}

/// Endpoint canónico: `GET /fhir/r4/Patient`
/// Retorna la colección de pacientes encapsulada en un `Bundle` FHIR de tipo `searchset`.
pub async fn list_patients(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<FhirResponse, ServerError> {
    let pacientes = match state.mapping_rules.resources.iter().find(|r| r.resource_type == "Patient") {
        Some(m) if m.source_table != "tbl_pacientes" => {
            let rows = state
                .repositories
                .dynamic
                .find_all_raw(m, params.limit(), params.offset())
                .await?;
            let mut list = Vec::with_capacity(rows.len());
            for r in &rows {
                list.push(medsys_db::row_to_legacy_paciente(r)?);
            }
            list
        }
        _ => {
            let all = state.repositories.pacientes.find_all().await?;
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

    let count = pacientes.len();
    let mut resources = Vec::with_capacity(count);

    for p in pacientes {
        let fhir_p = transform_patient(&p, Some(&state.mapping_rules))?;
        resources.push(Resource::Patient(Box::new(fhir_p)));
    }

    let bundle = create_searchset_bundle(Some("bundle-patients"), resources, Some(count));

    Ok(FhirResponse(Resource::Bundle(Box::new(bundle))))
}
