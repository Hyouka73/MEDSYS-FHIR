//! Manejadores HTTP para el recurso FHIR `Observation`.
//! Implementa el desacoplamiento canónico de signos vitales (Presión Arterial, Temperatura, Frecuencia Cardíaca y Frecuencia Respiratoria).

use axum::extract::{rejection::QueryRejection, OriginalUri, Path, Query, State};
use helios_fhir::r4::Resource;
use medsys_core::{
    transform_observation_blood_pressure, transform_observation_heart_rate,
    transform_observation_respiratory_rate, transform_observation_temperature,
};
use serde::Deserialize;

use super::common::{
    bundle_page, normalize_row, output_rules, parse_query, read_mapping, with_date_precision,
};
use crate::error::ServerError;
use crate::response::FhirResponse;
use crate::state::AppState;

/// Parámetros de consulta para búsqueda de observaciones clínicas.
#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
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
    let rules = output_rules(&state);
    let mapping = read_mapping(&state, "Observation")?;
    let row = state.repositories.dynamic.find_by_id(&mapping, id).await?;
    let row = normalize_row(&state, row)?;
    let signo = medsys_db::row_to_legacy_signo_vital(&row)?;
    let rules = Some(&rules);

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
    Ok(FhirResponse(with_date_precision(
        Resource::Observation(Box::new(obs)),
        &row,
        &mapping,
    )?))
}

/// Endpoint canónico: `GET /fhir/r4/Observation`
/// Retorna la colección de observaciones clínicas desacopladas (Presión Arterial, Temperatura, Frecuencia Cardíaca
/// y Frecuencia Respiratoria cuando está presente) empaquetadas en un `Bundle` FHIR de tipo `searchset`.
pub async fn list_observations(
    State(state): State<AppState>,
    OriginalUri(uri): OriginalUri,
    query: Result<Query<ObservationQueryParams>, QueryRejection>,
) -> Result<FhirResponse, ServerError> {
    let params = parse_query(query)?;
    let rules = output_rules(&state);
    let mapping = read_mapping(&state, "Observation")?;
    let rows = state
        .repositories
        .dynamic
        .find_all_raw(&mapping, i64::MAX, 0)
        .await?;
    let mut resources = Vec::new();
    for row in rows {
        let row = normalize_row(&state, row)?;
        let s = medsys_db::row_to_legacy_signo_vital(&row)?;
        if params.patient.is_some_and(|id| s.id_paciente != id) {
            continue;
        }
        if params.encounter.is_some_and(|id| s.id_consulta != id) {
            continue;
        }
        resources.push(with_date_precision(
            Resource::Observation(Box::new(transform_observation_blood_pressure(
                &s,
                Some(&rules),
            )?)),
            &row,
            &mapping,
        )?);
        resources.push(with_date_precision(
            Resource::Observation(Box::new(transform_observation_temperature(
                &s,
                Some(&rules),
            )?)),
            &row,
            &mapping,
        )?);
        resources.push(with_date_precision(
            Resource::Observation(Box::new(transform_observation_heart_rate(
                &s,
                Some(&rules),
            )?)),
            &row,
            &mapping,
        )?);
        if let Some(obs) = transform_observation_respiratory_rate(&s, Some(&rules))? {
            resources.push(with_date_precision(
                Resource::Observation(Box::new(obs)),
                &row,
                &mapping,
            )?);
        }
    }
    Ok(bundle_page(
        resources,
        params.limit(),
        params.offset(),
        &uri,
    ))
}
