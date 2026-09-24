//! medsys-server: API REST en Axum y Facade de interoperabilidad HL7 FHIR R4.
//!
//! Expone los recursos canónicos `Patient`, `Encounter`, `Observation` y `Condition`
//! bajo el prefijo `/fhir/r4/`, implementa respuestas estándar `OperationOutcome` para
//! todas las condiciones de error con `Content-Type: application/fhir+json`,
//! y provee endpoints de soporte para el Dashboard en React.

pub mod config;
pub mod error;
pub mod handlers;
pub mod response;
pub mod router;
pub mod state;

pub use config::ServerConfig;
pub use error::{ServerError, FHIR_JSON_CONTENT_TYPE};
pub use response::FhirResponse;
pub use router::create_router;
pub use state::AppState;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
