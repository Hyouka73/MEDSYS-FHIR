//! Wrapper de respuesta HTTP conforme al estándar HL7 FHIR R4.
//! Establece cabeceras obligatorias `Content-Type: application/fhir+json; charset=utf-8`.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use helios_fhir::r4::Resource;
use medsys_core::serialize_to_fhir_json;

use crate::error::{ServerError, FHIR_JSON_CONTENT_TYPE};

/// Estructura wrapper que encapsula un recurso FHIR y lo serializa
/// con la cabecera canónica `application/fhir+json`.
pub struct FhirResponse(pub Resource);

impl IntoResponse for FhirResponse {
    fn into_response(self) -> Response {
        match serialize_to_fhir_json(&self.0) {
            Ok(json_str) => (
                StatusCode::OK,
                [("content-type", FHIR_JSON_CONTENT_TYPE)],
                json_str,
            )
                .into_response(),
            Err(e) => ServerError::Domain(e).into_response(),
        }
    }
}
