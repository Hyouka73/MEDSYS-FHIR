//! Estado compartido e inyección de dependencias para el servidor Axum.

use medsys_core::MappingRules;
use medsys_db::MedsysRepositories;
use sqlx::PgPool;
use std::sync::Arc;

/// Estructura de estado compartida thread-safe para todos los manejadores HTTP.
#[derive(Clone)]
pub struct AppState {
    pub repositories: Arc<MedsysRepositories>,
    pub mapping_rules: Arc<MappingRules>,
    pub pool: PgPool,
    pub start_time: std::time::Instant,
}

impl AppState {
    /// Inicializa una nueva instancia del estado de la aplicación.
    pub fn new(
        repositories: MedsysRepositories,
        mapping_rules: MappingRules,
        pool: PgPool,
    ) -> Self {
        Self {
            repositories: Arc::new(repositories),
            mapping_rules: Arc::new(mapping_rules),
            pool,
            start_time: std::time::Instant::now(),
        }
    }

    /// Retorna el tiempo transcurrido desde el inicio del servidor en segundos.
    pub fn uptime_secs(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
}
