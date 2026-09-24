//! Construcción y configuración del enrutador Axum para MedSys-FHIR.

use axum::routing::get;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::error::not_found_fallback;
use crate::handlers::{
    get_condition, get_encounter, get_legacy_patient_full, get_observation, get_patient,
    health_check, list_conditions, list_encounters, list_legacy_patients, list_observations,
    list_patients,
};
use crate::state::AppState;

/// Construye el enrutador HTTP configurado con middlewares y rutas REST FHIR canónicas.
pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // Endpoints canónicos HL7 FHIR R4 (Tarea 4.1)
        .route("/fhir/r4/Patient", get(list_patients))
        .route("/fhir/r4/Patient/{id}", get(get_patient))
        .route("/fhir/r4/Encounter", get(list_encounters))
        .route("/fhir/r4/Encounter/{id}", get(get_encounter))
        .route("/fhir/r4/Observation", get(list_observations))
        .route("/fhir/r4/Observation/{id}", get(get_observation))
        .route("/fhir/r4/Condition", get(list_conditions))
        .route("/fhir/r4/Condition/{id}", get(get_condition))
        // Endpoints de salud y telemetría
        .route("/health", get(health_check))
        .route("/api/health", get(health_check))
        // Endpoints de inspección de datos legados y comparativa (Dashboard React)
        .route("/api/legacy/patients", get(list_legacy_patients))
        .route(
            "/api/legacy/patients/{id}/full",
            get(get_legacy_patient_full),
        )
        // Inyección de estado compartido
        .with_state(state)
        // Middlewares de telemetría y CORS
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        // Manejador fallback: cualquier ruta desconocida retorna OperationOutcome canónico (Tarea 4.2)
        .fallback(not_found_fallback)
}
