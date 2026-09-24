//! Manejadores HTTP para el recurso FHIR `Observation`.
//! Implementa el desacoplamiento canónico de signos vitales (Presión Arterial y Temperatura).

use axum::extract::{Path, Query, State};
use helios_fhir::r4::Resource;
use medsys_core::{
    create_searchset_bundle, transform_observation_blood_pressure,
    transform_observation_temperature,
};
use serde::Deserialize;

use crate::error::ServerError;
use crate::response::FhirResponse;
use crate::state::AppState;

/// Parámetros de consulta para búsqueda de observaciones clínicas.
#[derive(Debug, Deserialize, Default)]
pub struct ObservationQueryParams {
    pub patient: Option<i32>,
    pub encounter: Option<i32>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl ObservationQueryParams {
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(50).clamp(1, 200)
    }

    pub fn offset(&self) -> i64 {
        self.offset.unwrap_or(0).max(0)
    }
}

/// Endpoint canónico: `GET /fhir/r4/Observation/{id}`
/// Permite recuperar una observación específica por su prefijo canónico:
/// - `temp-{id}`: Observación de Temperatura Corporal (LOINC 8310-5, Cel)
/// - `bp-{id}` o `{id}`: Panel de Presión Arterial (LOINC 85354-9, mmHg)
pub async fn get_observation(
    State(state): State<AppState>,
    Path(raw_id): Path<String>,
) -> Result<FhirResponse, ServerError> {
    if let Some(id_str) = raw_id.strip_prefix("temp-") {
        let id: i32 = id_str.parse().map_err(|_| {
            ServerError::InvalidPath(format!("Identificador numérico inválido: '{id_str}'"))
        })?;
        let signo = state.repositories.signos_vitales.find_by_id(id).await?;
        let obs = transform_observation_temperature(&signo, Some(&state.mapping_rules))?;
        Ok(FhirResponse(Resource::Observation(Box::new(obs))))
    } else if let Some(id_str) = raw_id.strip_prefix("bp-") {
        let id: i32 = id_str.parse().map_err(|_| {
            ServerError::InvalidPath(format!("Identificador numérico inválido: '{id_str}'"))
        })?;
        let signo = state.repositories.signos_vitales.find_by_id(id).await?;
        let obs = transform_observation_blood_pressure(&signo, Some(&state.mapping_rules))?;
        Ok(FhirResponse(Resource::Observation(Box::new(obs))))
    } else {
        let id: i32 = raw_id.parse().map_err(|_| {
            ServerError::InvalidPath(format!("Identificador de observación inválido: '{raw_id}'"))
        })?;
        let signo = state.repositories.signos_vitales.find_by_id(id).await?;
        let obs = transform_observation_blood_pressure(&signo, Some(&state.mapping_rules))?;
        Ok(FhirResponse(Resource::Observation(Box::new(obs))))
    }
}

/// Endpoint canónico: `GET /fhir/r4/Observation`
/// Retorna la colección de observaciones clínicas desacopladas (Presión Arterial y Temperatura)
/// empaquetadas en un `Bundle` FHIR de tipo `searchset`.
pub async fn list_observations(
    State(state): State<AppState>,
    Query(params): Query<ObservationQueryParams>,
) -> Result<FhirResponse, ServerError> {
    let mut signos = if let Some(enc_id) = params.encounter {
        state
            .repositories
            .signos_vitales
            .find_by_consulta_id(enc_id)
            .await?
    } else if let Some(patient_id) = params.patient {
        state
            .repositories
            .signos_vitales
            .find_by_paciente_id(patient_id)
            .await?
    } else {
        state.repositories.signos_vitales.find_all().await?
    };

    let total_records = signos.len();
    let offset = params.offset() as usize;
    let limit = params.limit() as usize;
    if offset < total_records {
        signos = signos.into_iter().skip(offset).take(limit).collect();
    } else {
        signos.clear();
    }

    let mut resources = Vec::with_capacity(signos.len() * 2);

    for s in &signos {
        // Recurso 3A: Panel de Presión Arterial
        let obs_bp = transform_observation_blood_pressure(s, Some(&state.mapping_rules))?;
        resources.push(Resource::Observation(Box::new(obs_bp)));

        // Recurso 3B: Temperatura Corporal
        let obs_temp = transform_observation_temperature(s, Some(&state.mapping_rules))?;
        resources.push(Resource::Observation(Box::new(obs_temp)));
    }

    let total = resources.len();
    let bundle = create_searchset_bundle(Some("bundle-observations"), resources, Some(total));

    Ok(FhirResponse(Resource::Bundle(Box::new(bundle))))
}
