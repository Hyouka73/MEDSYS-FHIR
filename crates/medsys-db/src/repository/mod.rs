//! Módulo de repositorios de persistencia relacional de solo lectura.
//!
//! Agrupa los repositorios para `tbl_pacientes`, `tbl_consultas`, `tbl_signos_vitales`
//! y `tbl_diagnosticos`, facilitando su consumo ordenado e inyección de dependencias.

pub mod consultas;
pub mod diagnosticos;
pub mod pacientes;
pub mod signos_vitales;

pub use consultas::ConsultaRepository;
pub use diagnosticos::DiagnosticosRepository;
pub use pacientes::PacienteRepository;
pub use signos_vitales::SignosVitalesRepository;

use sqlx::PgPool;

/// Contenedor unificado de repositorios para el middleware MedSys-FHIR.
#[derive(Clone, Debug)]
pub struct MedsysRepositories {
    pub pacientes: PacienteRepository,
    pub consultas: ConsultaRepository,
    pub signos_vitales: SignosVitalesRepository,
    pub diagnosticos: DiagnosticosRepository,
}

impl MedsysRepositories {
    /// Construye una nueva instancia del bundle de repositorios con el pool de conexiones provisto.
    pub fn new(pool: PgPool) -> Self {
        Self {
            pacientes: PacienteRepository::new(pool.clone()),
            consultas: ConsultaRepository::new(pool.clone()),
            signos_vitales: SignosVitalesRepository::new(pool.clone()),
            diagnosticos: DiagnosticosRepository::new(pool),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DbConfig;
    use crate::pool::init_pool_lazy;

    #[tokio::test]
    async fn test_repositories_bundle_creation() {
        let config = DbConfig::default();
        let pool = init_pool_lazy(&config).expect("init_pool_lazy falló");
        let repos = MedsysRepositories::new(pool);

        assert!(!repos.pacientes.pool().is_closed());
        assert!(!repos.consultas.pool().is_closed());
        assert!(!repos.signos_vitales.pool().is_closed());
        assert!(!repos.diagnosticos.pool().is_closed());
    }
}
