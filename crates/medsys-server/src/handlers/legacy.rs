//! Manejadores HTTP para inspección de datos legados relacionales y comparación FHIR.
//! Empleado por el Dashboard React para verificar la trazabilidad y transformación en vivo.

use axum::extract::{Path, State};
use axum::Json;
use helios_fhir::r4::Resource;
use medsys_core::{
    serialize_to_fhir_json, transform_condition, transform_encounter,
    transform_observation_blood_pressure, transform_observation_temperature, transform_patient,
    LegacyConsulta, LegacyDiagnostico, LegacyPaciente, LegacySignoVital,
};
use serde::Serialize;

use crate::error::ServerError;
use crate::state::AppState;

/// Estructura de comparación completa entre datos relacionales NOM-004 y recursos FHIR R4.
#[derive(Serialize)]
pub struct PatientFullComparison {
    pub paciente_legado: LegacyPaciente,
    pub consultas_legadas: Vec<LegacyConsulta>,
    pub signos_vitales_legados: Vec<LegacySignoVital>,
    pub diagnosticos_legados: Vec<LegacyDiagnostico>,
    pub fhir_patient: serde_json::Value,
    pub fhir_encounters: Vec<serde_json::Value>,
    pub fhir_observations: Vec<serde_json::Value>,
    pub fhir_conditions: Vec<serde_json::Value>,
}

/// Endpoint: `GET /api/legacy/patients`
/// Retorna la lista de pacientes en formato relacional puro de `tbl_pacientes`.
pub async fn list_legacy_patients(
    State(state): State<AppState>,
) -> Result<Json<Vec<LegacyPaciente>>, ServerError> {
    let pacientes = state.repositories.pacientes.find_all().await?;
    Ok(Json(pacientes))
}

/// Endpoint: `GET /api/legacy/patients/{id}/full`
/// Retorna la ficha clínica relacional completa del paciente junto a sus recursos FHIR correspondientes.
pub async fn get_legacy_patient_full(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<PatientFullComparison>, ServerError> {
    // 1. Obtener registros relacionales
    let paciente = state.repositories.pacientes.find_by_id(id).await?;
    let consultas = state.repositories.consultas.find_by_paciente_id(id).await?;
    let signos = state
        .repositories
        .signos_vitales
        .find_by_paciente_id(id)
        .await?;
    let diagnosticos = state
        .repositories
        .diagnosticos
        .find_by_paciente_id(id)
        .await?;

    // 2. Transformar a recursos FHIR R4
    let fhir_patient_obj = transform_patient(&paciente, Some(&state.mapping_rules))?;
    let fhir_patient_str = serialize_to_fhir_json(&Resource::Patient(Box::new(fhir_patient_obj)))?;
    let fhir_patient_val: serde_json::Value = serde_json::from_str(&fhir_patient_str)?;

    let mut fhir_encounters = Vec::with_capacity(consultas.len());
    for c in &consultas {
        let enc = transform_encounter(c, Some(&state.mapping_rules))?;
        let enc_str = serialize_to_fhir_json(&Resource::Encounter(Box::new(enc)))?;
        let enc_val: serde_json::Value = serde_json::from_str(&enc_str)?;
        fhir_encounters.push(enc_val);
    }

    let mut fhir_observations = Vec::with_capacity(signos.len() * 2);
    for s in &signos {
        let bp = transform_observation_blood_pressure(s, Some(&state.mapping_rules))?;
        let bp_str = serialize_to_fhir_json(&Resource::Observation(Box::new(bp)))?;
        fhir_observations.push(serde_json::from_str(&bp_str)?);

        let temp = transform_observation_temperature(s, Some(&state.mapping_rules))?;
        let temp_str = serialize_to_fhir_json(&Resource::Observation(Box::new(temp)))?;
        fhir_observations.push(serde_json::from_str(&temp_str)?);
    }

    let mut fhir_conditions = Vec::with_capacity(diagnosticos.len());
    for d in &diagnosticos {
        let cond = transform_condition(d, Some(&state.mapping_rules))?;
        let cond_str = serialize_to_fhir_json(&Resource::Condition(Box::new(cond)))?;
        fhir_conditions.push(serde_json::from_str(&cond_str)?);
    }

    Ok(Json(PatientFullComparison {
        paciente_legado: paciente,
        consultas_legadas: consultas,
        signos_vitales_legados: signos,
        diagnosticos_legados: diagnosticos,
        fhir_patient: fhir_patient_val,
        fhir_encounters,
        fhir_observations,
        fhir_conditions,
    }))
}
