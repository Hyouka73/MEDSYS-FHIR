//! medsys-db: Capa de persistencia asíncrona de solo lectura sobre PostgreSQL 16.
//!
//! Provee el adaptador de datos con `SQLx` respetando los lineamientos de la NOM-004-SSA3-2012,
//! configuración parametrizada del pool de conexiones con límites y timeouts, healthchecks,
//! repositorios de solo lectura estrictamente parametrizados ($1, $2) y mapeo tipado a `MedSysError`.

pub mod config;
pub mod entities;
pub mod error;
pub mod pool;
pub mod repository;

pub use config::DbConfig;
pub use entities::{ConsultaEntity, DiagnosticoEntity, PacienteEntity, SignoVitalEntity};
pub use error::{map_sqlx_error, DbError};
pub use pool::{create_pool_options, init_pool, init_pool_lazy, DbManager};
pub use repository::{
    ConsultaRepository, DiagnosticosRepository, MedsysRepositories, PacienteRepository,
    SignosVitalesRepository,
};

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
