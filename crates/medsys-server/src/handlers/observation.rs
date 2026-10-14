//! Manejadores HTTP para el recurso FHIR `Observation`.
//! Implementa el desacoplamiento canónico de signos vitales (Presión Arterial, Temperatura, Frecuencia Cardíaca y Frecuencia Respiratoria).

use axum::extract::{Path, Query, State};
use helios_fhir::r4::Resource;
use medsys_core::{
    create_searchset_bundle, transform_observation_blood_pressure,
    transform_observation_heart_rate, transform_observation_respiratory_rate,
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

/// Subtipo de signo vital expuesto como `Observation` independiente (expansión 1:4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VitalKind {
    BloodPressure,
    Temperature,
    HeartRate,
    RespiratoryRate,
}

/// Interpreta identificadores `{kind}-{id}` o `{id}-{kind}` (y `{id}` numérico puro = presión arterial).
fn parse_observation_id(raw_id: &str) -> Result<(VitalKind, i32), ServerError> {
    const KINDS: [(&str, VitalKind); 6] = [
        ("bp", VitalKind::BloodPressure),
        ("temp", VitalKind::Temperature),
        ("hr", VitalKind::HeartRate),
        ("resp", VitalKind::RespiratoryRate),
        ("rr", VitalKind::RespiratoryRate),
        ("", VitalKind::BloodPressure),
    ];

    let parse_id = |id_str: &str| -> Result<i32, ServerError> {
        id_str.parse().map_err(|_| {
            ServerError::InvalidPath(format!("Identificador numérico inválido: '{id_str}'"))
        })
    };

    for (tag, kind) in KINDS.iter().filter(|(tag, _)| !tag.is_empty()) {
        if let Some(id_str) = raw_id.strip_prefix(&format!("{tag}-")) {
            return Ok((*kind, parse_id(id_str)?));
        }
        if let Some(id_str) = raw_id.strip_suffix(&format!("-{tag}")) {
            return Ok((*kind, parse_id(id_str)?));
        }
    }

    raw_id
        .parse()
        .map(|id| (VitalKind::BloodPressure, id))
        .map_err(|_| {
            ServerError::InvalidPath(format!("Identificador de observación inválido: '{raw_id}'"))
        })
}

/// Endpoint canónico: `GET /fhir/r4/Observation/{id}`
/// Permite recuperar una observación específica por prefijo o sufijo canónico:
/// - `temp-{id}` / `{id}-temp`: Temperatura Corporal (LOINC 8310-5, Cel)
/// - `hr-{id}` / `{id}-hr`: Frecuencia Cardíaca (LOINC 8867-4, /min)
/// - `resp-{id}`, `rr-{id}`, `{id}-resp` / `{id}-rr`: Frecuencia Respiratoria (LOINC 9279-1, /min, opcional)
/// - `bp-{id}`, `{id}-bp` o `{id}`: Panel de Presión Arterial (LOINC 85354-9, mmHg)
pub async fn get_observation(
    State(state): State<AppState>,
    Path(raw_id): Path<String>,
) -> Result<FhirResponse, ServerError> {
    let (kind, id) = parse_observation_id(&raw_id)?;
    let signo = state.repositories.signos_vitales.find_by_id(id).await?;
    state
        .repositories
        .verify_encounter_and_patient_exist(signo.id_consulta, signo.id_paciente)
        .await?;
    let rules = Some(state.mapping_rules.as_ref());

    let obs = match kind {
        VitalKind::BloodPressure => transform_observation_blood_pressure(&signo, rules)?,
        VitalKind::Temperature => transform_observation_temperature(&signo, rules)?,
        VitalKind::HeartRate => transform_observation_heart_rate(&signo, rules)?,
        VitalKind::RespiratoryRate => transform_observation_respiratory_rate(&signo, rules)?
            .ok_or_else(|| {
                ServerError::NotFound(format!(
                    "La consulta de signos vitales '{raw_id}' no registra frecuencia respiratoria (campo opcional NULL)."
                ))
            })?,
    };
    Ok(FhirResponse(Resource::Observation(Box::new(obs))))
}

/// Endpoint canónico: `GET /fhir/r4/Observation`
/// Retorna la colección de observaciones clínicas desacopladas (Presión Arterial, Temperatura, Frecuencia Cardíaca
/// y Frecuencia Respiratoria cuando está presente) empaquetadas en un `Bundle` FHIR de tipo `searchset`.
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

    let mut resources = Vec::with_capacity(signos.len() * 4);

    for s in &signos {
        // Recurso 3A: Panel de Presión Arterial
        let obs_bp = transform_observation_blood_pressure(s, Some(&state.mapping_rules))?;
        resources.push(Resource::Observation(Box::new(obs_bp)));

        // Recurso 3B: Temperatura Corporal
        let obs_temp = transform_observation_temperature(s, Some(&state.mapping_rules))?;
        resources.push(Resource::Observation(Box::new(obs_temp)));

        // Recurso 3C: Frecuencia Cardíaca
        let obs_hr = transform_observation_heart_rate(s, Some(&state.mapping_rules))?;
        resources.push(Resource::Observation(Box::new(obs_hr)));

        // Recurso 3D: Frecuencia Respiratoria (opcional: se omite si es NULL)
        if let Some(obs_resp) =
            transform_observation_respiratory_rate(s, Some(&state.mapping_rules))?
        {
            resources.push(Resource::Observation(Box::new(obs_resp)));
        }
    }

    let total = resources.len();
    let bundle = create_searchset_bundle(Some("bundle-observations"), resources, Some(total));

    Ok(FhirResponse(Resource::Bundle(Box::new(bundle))))
}
