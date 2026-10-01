//! Manejo global de excepciones para el servidor Axum.
//! Traduce todos los errores de dominio, base de datos y enrutamiento hacia
//! el recurso canónico `OperationOutcome` de HL7 FHIR R4 con cabecera `application/fhir+json`.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use helios_fhir::r4::Resource;
use medsys_core::{create_operation_outcome, serialize_to_fhir_json, MedSysError};
use thiserror::Error;

pub const FHIR_JSON_CONTENT_TYPE: &str = "application/fhir+json; charset=utf-8";

/// Enumerador de errores del servidor HTTP MedSys.
#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Error de dominio MedSys: {0}")]
    Domain(#[from] MedSysError),

    #[error("Error de base de datos SQLx: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Identificador de ruta inválido: {0}")]
    InvalidPath(String),

    #[error("Ruta FHIR no encontrada: {0}")]
    NotFound(String),

    #[error("Error de serialización JSON: {0}")]
    Json(#[from] serde_json::Error),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, issue_code, diagnostics) = match &self {
            ServerError::Domain(MedSysError::NotFound(msg)) => {
                (StatusCode::NOT_FOUND, "not-found", msg.clone())
            }
            ServerError::NotFound(msg) => (StatusCode::NOT_FOUND, "not-found", msg.clone()),
            ServerError::Domain(MedSysError::ValidationError(msg))
            | ServerError::Domain(MedSysError::InvalidRule(msg)) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "invalid", msg.clone())
            }
            ServerError::Domain(MedSysError::MissingField { field, resource }) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "required",
                format!("Campo requerido '{field}' faltante para el recurso '{resource}'"),
            ),
            ServerError::Domain(MedSysError::ProcessingError(msg)) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "processing",
                format!("Error de procesamiento o corrupción de datos clínicos: {msg}"),
            ),
            ServerError::InvalidPath(msg) => (StatusCode::BAD_REQUEST, "value", msg.clone()),
            ServerError::Domain(MedSysError::DatabaseError(msg)) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "transient",
                format!("Error de persistencia: {msg}"),
            ),
            ServerError::Database(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "transient",
                format!("Error de persistencia relacional: {err}"),
            ),
            ServerError::Domain(MedSysError::TransformationError(msg)) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "processing",
                format!("Fallo durante la transformación FHIR: {msg}"),
            ),
            ServerError::Domain(MedSysError::Internal(msg)) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "exception",
                format!("Error interno del middleware: {msg}"),
            ),
            ServerError::Domain(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "exception",
                err.to_string(),
            ),
            ServerError::Json(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "exception",
                format!("Error de serialización JSON: {err}"),
            ),
        };

        let outcome = create_operation_outcome(
            Some(&format!(
                "outcome-{}",
                chrono::Utc::now().timestamp_millis()
            )),
            "error",
            issue_code,
            &diagnostics,
        );

        let json_body = match serialize_to_fhir_json(&Resource::OperationOutcome(Box::new(outcome)))
        {
            Ok(body) => body,
            Err(e) => {
                format!(
                    r#"{{"resourceType":"OperationOutcome","issue":[{{"severity":"error","code":"exception","diagnostics":"{}"}}]}}"#,
                    e
                )
            }
        };

        (
            status,
            [("content-type", FHIR_JSON_CONTENT_TYPE)],
            json_body,
        )
            .into_response()
    }
}

/// Fallback canónico para cualquier ruta HTTP no reconocida en el servidor.
/// Asegura el cumplimiento de la regla: NUNCA responder con JSON genérico ni 404 plano,
/// sino con el recurso oficial OperationOutcome y cabecera application/fhir+json.
pub async fn not_found_fallback(uri: axum::http::Uri) -> impl IntoResponse {
    let outcome = create_operation_outcome(
        Some(&format!(
            "outcome-404-{}",
            chrono::Utc::now().timestamp_millis()
        )),
        "error",
        "not-found",
        &format!(
            "El endpoint o recurso FHIR solicitado '{}' no existe en este servidor.",
            uri.path()
        ),
    );

    let json_body = match serialize_to_fhir_json(&Resource::OperationOutcome(Box::new(outcome))) {
        Ok(b) => b,
        Err(_) => r#"{"resourceType":"OperationOutcome","issue":[{"severity":"error","code":"not-found","diagnostics":"Recurso no encontrado"}]}"#.to_string(),
    };

    (
        StatusCode::NOT_FOUND,
        [("content-type", FHIR_JSON_CONTENT_TYPE)],
        json_body,
    )
}
