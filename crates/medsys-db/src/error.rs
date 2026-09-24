//! Manejo tipado de errores de persistencia relacional para MedSys-FHIR.

use medsys_core::error::MedSysError;
use thiserror::Error;

/// Enumerador tipado para errores específicos de persistencia relacional.
#[derive(Error, Debug)]
pub enum DbError {
    #[error("Error de conexión o consulta SQLx: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Configuración de base de datos relacional inválida: {0}")]
    InvalidConfig(String),

    #[error("Fallo de salud (healthcheck) de PostgreSQL: {0}")]
    HealthcheckFailed(String),

    #[error("Timeout al intentar adquirir una conexión del pool ({0}s)")]
    PoolTimeout(u64),
}

/// Mapea de manera uniforme cualquier `sqlx::Error` hacia el tipo canónico `MedSysError`.
/// Esto previene el uso de `unwrap()` y asegura que las fallas de base de datos se traduzcan
/// en respuestas clínicas FHIR OperationOutcome estándar.
pub fn map_sqlx_error(err: sqlx::Error) -> MedSysError {
    match err {
        sqlx::Error::RowNotFound => MedSysError::NotFound(
            "Registro clínico no encontrado en la base de datos relacional".to_string(),
        ),
        sqlx::Error::PoolTimedOut => MedSysError::DatabaseError(
            "Se agotó el tiempo de espera para obtener una conexión del pool PostgreSQL"
                .to_string(),
        ),
        sqlx::Error::PoolClosed => MedSysError::DatabaseError(
            "El pool de conexiones de base de datos se encuentra cerrado".to_string(),
        ),
        other => MedSysError::DatabaseError(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_row_not_found_to_medsys_error() {
        let err = sqlx::Error::RowNotFound;
        let mapped = map_sqlx_error(err);
        match mapped {
            MedSysError::NotFound(msg) => {
                assert!(msg.contains("no encontrado"));
            }
            other => panic!("Esperaba MedSysError::NotFound, obtuve: {:?}", other),
        }
    }

    #[test]
    fn test_map_pool_timed_out() {
        let err = sqlx::Error::PoolTimedOut;
        let mapped = map_sqlx_error(err);
        match mapped {
            MedSysError::DatabaseError(msg) => {
                assert!(msg.contains("tiempo de espera"));
            }
            other => panic!("Esperaba MedSysError::DatabaseError, obtuve: {:?}", other),
        }
    }

    #[test]
    fn test_map_pool_closed() {
        let err = sqlx::Error::PoolClosed;
        let mapped = map_sqlx_error(err);
        match mapped {
            MedSysError::DatabaseError(msg) => {
                assert!(msg.contains("cerrado"));
            }
            other => panic!("Esperaba MedSysError::DatabaseError, obtuve: {:?}", other),
        }
    }
}
