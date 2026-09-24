//! Repositorio de lectura parametrizada para la tabla `tbl_consultas`.
//!
//! Representa encuentros clínicos normativos y garantiza el uso estricto de
//! sentencias parametrizadas ($1, $2) sin operaciones de mutación.

use crate::entities::ConsultaEntity;
use crate::error::map_sqlx_error;
use medsys_core::error::{MedSysError, Result};
use medsys_core::model::legacy::LegacyConsulta;
use sqlx::PgPool;
use tracing::debug;

pub const SQL_FIND_CONSULTA_BY_ID: &str = r#"
    SELECT 
        id_consulta, id_paciente, cedula_medico_tratante, nombre_medico,
        estado_consulta, motivo_consulta, fecha_hora_inicio, fecha_hora_fin,
        unidad_medica
    FROM tbl_consultas
    WHERE id_consulta = $1
"#;

pub const SQL_FIND_CONSULTAS_BY_PACIENTE: &str = r#"
    SELECT 
        id_consulta, id_paciente, cedula_medico_tratante, nombre_medico,
        estado_consulta, motivo_consulta, fecha_hora_inicio, fecha_hora_fin,
        unidad_medica
    FROM tbl_consultas
    WHERE id_paciente = $1
    ORDER BY fecha_hora_inicio ASC
"#;

pub const SQL_FIND_ALL_CONSULTAS: &str = r#"
    SELECT 
        id_consulta, id_paciente, cedula_medico_tratante, nombre_medico,
        estado_consulta, motivo_consulta, fecha_hora_inicio, fecha_hora_fin,
        unidad_medica
    FROM tbl_consultas
    ORDER BY id_consulta ASC
"#;

/// Repositorio de solo lectura para acceder a las consultas médicas legadas.
#[derive(Clone, Debug)]
pub struct ConsultaRepository {
    pool: PgPool,
}

impl ConsultaRepository {
    /// Inicializa el repositorio con el pool de conexiones.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Retorna una referencia al pool de conexiones.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Busca una consulta médica por su clave primaria (`id_consulta`).
    /// Si no existe, retorna `MedSysError::NotFound`.
    pub async fn find_by_id(&self, id: i32) -> Result<LegacyConsulta> {
        debug!(id_consulta = id, "Consultando encuentro médico por ID");
        let entity = sqlx::query_as::<_, ConsultaEntity>(SQL_FIND_CONSULTA_BY_ID)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_sqlx_error)?
            .ok_or_else(|| {
                MedSysError::NotFound(format!(
                    "Consulta médica con ID '{id}' no encontrada en el sistema legado."
                ))
            })?;

        Ok(entity.into())
    }

    /// Busca una consulta médica por su clave primaria retornando `Option<LegacyConsulta>`.
    pub async fn find_optional_by_id(&self, id: i32) -> Result<Option<LegacyConsulta>> {
        debug!(
            id_consulta = id,
            "Consultando opcionalmente encuentro médico por ID"
        );
        let opt = sqlx::query_as::<_, ConsultaEntity>(SQL_FIND_CONSULTA_BY_ID)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(opt.map(Into::into))
    }

    /// Retorna todas las consultas médicas asociadas a un paciente específico.
    pub async fn find_by_paciente_id(&self, id_paciente: i32) -> Result<Vec<LegacyConsulta>> {
        debug!(
            id_paciente = id_paciente,
            "Consultando consultas por ID de paciente"
        );
        let entities = sqlx::query_as::<_, ConsultaEntity>(SQL_FIND_CONSULTAS_BY_PACIENTE)
            .bind(id_paciente)
            .fetch_all(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(entities.into_iter().map(Into::into).collect())
    }

    /// Retorna todas las consultas registradas en el sistema.
    pub async fn find_all(&self) -> Result<Vec<LegacyConsulta>> {
        debug!("Consultando todas las consultas médicas registradas");
        let entities = sqlx::query_as::<_, ConsultaEntity>(SQL_FIND_ALL_CONSULTAS)
            .fetch_all(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(entities.into_iter().map(Into::into).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_queries_safety() {
        assert!(SQL_FIND_CONSULTA_BY_ID.contains("WHERE id_consulta = $1"));
        assert!(SQL_FIND_CONSULTAS_BY_PACIENTE.contains("WHERE id_paciente = $1"));
        assert!(SQL_FIND_ALL_CONSULTAS.contains("ORDER BY id_consulta ASC"));

        for query in &[
            SQL_FIND_CONSULTA_BY_ID,
            SQL_FIND_CONSULTAS_BY_PACIENTE,
            SQL_FIND_ALL_CONSULTAS,
        ] {
            let upper = query.to_uppercase();
            assert!(upper.contains("SELECT"));
            assert!(!upper.contains("DELETE"));
            assert!(!upper.contains("UPDATE"));
            assert!(!upper.contains("INSERT"));
        }
    }
}
