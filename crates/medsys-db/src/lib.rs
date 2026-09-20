//! medsys-db: Capa de persistencia asíncrona de solo lectura sobre PostgreSQL 16.

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
