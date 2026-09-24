//! Manejadores HTTP para monitoreo de salud del servidor y la base de datos.

use axum::extract::State;
use axum::Json;
use serde::Serialize;

use crate::state::AppState;

/// Estructura de reporte de salud del servidor.
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub server_name: &'static str,
    pub server_version: &'static str,
    pub uptime_seconds: u64,
    pub database_status: &'static str,
    pub fhir_version: &'static str,
}

/// Endpoint: `GET /health` y `GET /api/health`
pub async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    let db_ok = sqlx::query("SELECT 1").execute(&state.pool).await.is_ok();

    Json(HealthResponse {
        status: if db_ok { "ok" } else { "degraded" },
        server_name: "MedSys-FHIR",
        server_version: env!("CARGO_PKG_VERSION"),
        uptime_seconds: state.uptime_secs(),
        database_status: if db_ok { "connected" } else { "disconnected" },
        fhir_version: "R4 (4.0.1)",
    })
}
