//! Repositorio de lectura parametrizada para la tabla `tbl_diagnosticos`.
//!
//! Almacena diagnósticos clínicos basados en el catálogo internacional CIE-10
//! que serán mapeados a recursos FHIR Condition.

use crate::entities::DiagnosticoEntity;
use crate::error::map_sqlx_error;
use medsys_core::error::{MedSysError, Result};
use medsys_core::model::legacy::LegacyDiagnostico;
use sqlx::PgPool;
use tracing::debug;

pub const SQL_FIND_DIAGNOSTICO_BY_ID: &str = r#"
    SELECT 
        id_diagnostico, id_consulta, id_paciente, codigo_cie10,
        descripcion_diagnostico, tipo_diagnostico, fecha_diagnostico
    FROM tbl_diagnosticos
    WHERE id_diagnostico = $1
"#;

pub const SQL_FIND_DIAGNOSTICOS_BY_CONSULTA: &str = r#"
    SELECT 
        id_diagnostico, id_consulta, id_paciente, codigo_cie10,
        descripcion_diagnostico, tipo_diagnostico, fecha_diagnostico
    FROM tbl_diagnosticos
    WHERE id_consulta = $1
    ORDER BY id_diagnostico ASC
"#;

pub const SQL_FIND_DIAGNOSTICOS_BY_PACIENTE: &str = r#"
    SELECT 
        id_diagnostico, id_consulta, id_paciente, codigo_cie10,
        descripcion_diagnostico, tipo_diagnostico, fecha_diagnostico
    FROM tbl_diagnosticos
    WHERE id_paciente = $1
    ORDER BY fecha_diagnostico ASC
"#;

pub const SQL_FIND_DIAGNOSTICOS_BY_CIE10: &str = r#"
    SELECT 
        id_diagnostico, id_consulta, id_paciente, codigo_cie10,
        descripcion_diagnostico, tipo_diagnostico, fecha_diagnostico
    FROM tbl_diagnosticos
    WHERE codigo_cie10 = $1
    ORDER BY fecha_diagnostico ASC
"#;

pub const SQL_FIND_ALL_DIAGNOSTICOS: &str = r#"
    SELECT 
        id_diagnostico, id_consulta, id_paciente, codigo_cie10,
        descripcion_diagnostico, tipo_diagnostico, fecha_diagnostico
    FROM tbl_diagnosticos
    ORDER BY id_diagnostico ASC
"#;

/// Repositorio de solo lectura para acceder a los diagnósticos CIE-10.
#[derive(Clone, Debug)]
pub struct DiagnosticosRepository {
    pool: PgPool,
}

impl DiagnosticosRepository {
    /// Inicializa el repositorio con el pool de conexiones.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Retorna una referencia al pool de conexiones.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Busca un diagnóstico por su clave primaria (`id_diagnostico`).
    /// Si no existe, retorna `MedSysError::NotFound`.
    pub async fn find_by_id(&self, id: i32) -> Result<LegacyDiagnostico> {
        debug!(id_diagnostico = id, "Consultando diagnóstico por ID");
        let entity = sqlx::query_as::<_, DiagnosticoEntity>(SQL_FIND_DIAGNOSTICO_BY_ID)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_sqlx_error)?
            .ok_or_else(|| {
                MedSysError::NotFound(format!(
                    "Diagnóstico con ID '{id}' no encontrado en el sistema legado."
                ))
            })?;

        Ok(entity.into())
    }

    /// Busca un diagnóstico por su clave primaria retornando `Option<LegacyDiagnostico>`.
    pub async fn find_optional_by_id(&self, id: i32) -> Result<Option<LegacyDiagnostico>> {
        debug!(
            id_diagnostico = id,
            "Consultando opcionalmente diagnóstico por ID"
        );
        let opt = sqlx::query_as::<_, DiagnosticoEntity>(SQL_FIND_DIAGNOSTICO_BY_ID)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(opt.map(Into::into))
    }

    /// Retorna todos los diagnósticos asentados en una consulta médica específica.
    pub async fn find_by_consulta_id(&self, id_consulta: i32) -> Result<Vec<LegacyDiagnostico>> {
        debug!(
            id_consulta = id_consulta,
            "Consultando diagnósticos por consulta"
        );
        let entities = sqlx::query_as::<_, DiagnosticoEntity>(SQL_FIND_DIAGNOSTICOS_BY_CONSULTA)
            .bind(id_consulta)
            .fetch_all(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(entities.into_iter().map(Into::into).collect())
    }

    /// Retorna el historial de diagnósticos de un paciente.
    pub async fn find_by_paciente_id(&self, id_paciente: i32) -> Result<Vec<LegacyDiagnostico>> {
        debug!(
            id_paciente = id_paciente,
            "Consultando diagnósticos por paciente"
        );
        let entities = sqlx::query_as::<_, DiagnosticoEntity>(SQL_FIND_DIAGNOSTICOS_BY_PACIENTE)
            .bind(id_paciente)
            .fetch_all(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(entities.into_iter().map(Into::into).collect())
    }

    /// Retorna todos los diagnósticos que coincidan con un código CIE-10 (e.g. "I10").
    pub async fn find_by_codigo_cie10(&self, cie10: &str) -> Result<Vec<LegacyDiagnostico>> {
        debug!(
            codigo_cie10 = cie10,
            "Consultando diagnósticos por código CIE-10"
        );
        let entities = sqlx::query_as::<_, DiagnosticoEntity>(SQL_FIND_DIAGNOSTICOS_BY_CIE10)
            .bind(cie10)
            .fetch_all(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(entities.into_iter().map(Into::into).collect())
    }

    /// Retorna todos los diagnósticos registrados en el sistema legado.
    pub async fn find_all(&self) -> Result<Vec<LegacyDiagnostico>> {
        debug!("Consultando todos los diagnósticos registrados");
        let entities = sqlx::query_as::<_, DiagnosticoEntity>(SQL_FIND_ALL_DIAGNOSTICOS)
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
        assert!(SQL_FIND_DIAGNOSTICO_BY_ID.contains("WHERE id_diagnostico = $1"));
        assert!(SQL_FIND_DIAGNOSTICOS_BY_CONSULTA.contains("WHERE id_consulta = $1"));
        assert!(SQL_FIND_DIAGNOSTICOS_BY_PACIENTE.contains("WHERE id_paciente = $1"));
        assert!(SQL_FIND_DIAGNOSTICOS_BY_CIE10.contains("WHERE codigo_cie10 = $1"));
        assert!(SQL_FIND_ALL_DIAGNOSTICOS.contains("ORDER BY id_diagnostico ASC"));

        for query in &[
            SQL_FIND_DIAGNOSTICO_BY_ID,
            SQL_FIND_DIAGNOSTICOS_BY_CONSULTA,
            SQL_FIND_DIAGNOSTICOS_BY_PACIENTE,
            SQL_FIND_DIAGNOSTICOS_BY_CIE10,
            SQL_FIND_ALL_DIAGNOSTICOS,
        ] {
            let upper = query.to_uppercase();
            assert!(upper.contains("SELECT"));
            assert!(!upper.contains("DELETE"));
            assert!(!upper.contains("UPDATE"));
            assert!(!upper.contains("INSERT"));
        }
    }
}
