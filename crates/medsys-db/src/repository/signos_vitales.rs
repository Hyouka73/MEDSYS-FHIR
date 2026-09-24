//! Repositorio de lectura parametrizada para la tabla `tbl_signos_vitales`.
//!
//! Almacena observaciones clínicas y signos vitales (presión arterial, frecuencia,
//! temperatura) que serán transformados a recursos FHIR Observation.

use crate::entities::SignoVitalEntity;
use crate::error::map_sqlx_error;
use medsys_core::error::{MedSysError, Result};
use medsys_core::model::legacy::LegacySignoVital;
use sqlx::PgPool;
use tracing::debug;

pub const SQL_FIND_SIGNO_BY_ID: &str = r#"
    SELECT 
        id_signo, id_consulta, id_paciente, presion_sistolica, presion_diastolica,
        frecuencia_cardiaca, frecuencia_respiratoria, temperatura_celsius,
        peso_kg, talla_cm, fecha_registro
    FROM tbl_signos_vitales
    WHERE id_signo = $1
"#;

pub const SQL_FIND_SIGNOS_BY_CONSULTA: &str = r#"
    SELECT 
        id_signo, id_consulta, id_paciente, presion_sistolica, presion_diastolica,
        frecuencia_cardiaca, frecuencia_respiratoria, temperatura_celsius,
        peso_kg, talla_cm, fecha_registro
    FROM tbl_signos_vitales
    WHERE id_consulta = $1
    ORDER BY fecha_registro ASC
"#;

pub const SQL_FIND_SIGNOS_BY_PACIENTE: &str = r#"
    SELECT 
        id_signo, id_consulta, id_paciente, presion_sistolica, presion_diastolica,
        frecuencia_cardiaca, frecuencia_respiratoria, temperatura_celsius,
        peso_kg, talla_cm, fecha_registro
    FROM tbl_signos_vitales
    WHERE id_paciente = $1
    ORDER BY fecha_registro ASC
"#;

pub const SQL_FIND_ALL_SIGNOS: &str = r#"
    SELECT 
        id_signo, id_consulta, id_paciente, presion_sistolica, presion_diastolica,
        frecuencia_cardiaca, frecuencia_respiratoria, temperatura_celsius,
        peso_kg, talla_cm, fecha_registro
    FROM tbl_signos_vitales
    ORDER BY id_signo ASC
"#;

/// Repositorio de solo lectura para acceder a las mediciones de signos vitales.
#[derive(Clone, Debug)]
pub struct SignosVitalesRepository {
    pool: PgPool,
}

impl SignosVitalesRepository {
    /// Inicializa el repositorio con el pool de conexiones.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Retorna una referencia al pool de conexiones.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Busca un registro de signos vitales por su clave primaria (`id_signo`).
    /// Si no existe, retorna `MedSysError::NotFound`.
    pub async fn find_by_id(&self, id: i32) -> Result<LegacySignoVital> {
        debug!(
            id_signo = id,
            "Consultando registro de signos vitales por ID"
        );
        let entity = sqlx::query_as::<_, SignoVitalEntity>(SQL_FIND_SIGNO_BY_ID)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_sqlx_error)?
            .ok_or_else(|| {
                MedSysError::NotFound(format!(
                    "Registro de signos vitales con ID '{id}' no encontrado en el sistema legado."
                ))
            })?;

        Ok(entity.into())
    }

    /// Busca un registro de signos vitales por su clave primaria retornando `Option<LegacySignoVital>`.
    pub async fn find_optional_by_id(&self, id: i32) -> Result<Option<LegacySignoVital>> {
        debug!(
            id_signo = id,
            "Consultando opcionalmente signos vitales por ID"
        );
        let opt = sqlx::query_as::<_, SignoVitalEntity>(SQL_FIND_SIGNO_BY_ID)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(opt.map(Into::into))
    }

    /// Retorna todos los registros de signos vitales tomados durante una consulta médica específica.
    pub async fn find_by_consulta_id(&self, id_consulta: i32) -> Result<Vec<LegacySignoVital>> {
        debug!(
            id_consulta = id_consulta,
            "Consultando signos vitales por consulta"
        );
        let entities = sqlx::query_as::<_, SignoVitalEntity>(SQL_FIND_SIGNOS_BY_CONSULTA)
            .bind(id_consulta)
            .fetch_all(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(entities.into_iter().map(Into::into).collect())
    }

    /// Retorna todos los registros de signos vitales históricos de un paciente.
    pub async fn find_by_paciente_id(&self, id_paciente: i32) -> Result<Vec<LegacySignoVital>> {
        debug!(
            id_paciente = id_paciente,
            "Consultando signos vitales por paciente"
        );
        let entities = sqlx::query_as::<_, SignoVitalEntity>(SQL_FIND_SIGNOS_BY_PACIENTE)
            .bind(id_paciente)
            .fetch_all(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(entities.into_iter().map(Into::into).collect())
    }

    /// Retorna todos los registros de signos vitales registrados.
    pub async fn find_all(&self) -> Result<Vec<LegacySignoVital>> {
        debug!("Consultando todos los registros de signos vitales");
        let entities = sqlx::query_as::<_, SignoVitalEntity>(SQL_FIND_ALL_SIGNOS)
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
        assert!(SQL_FIND_SIGNO_BY_ID.contains("WHERE id_signo = $1"));
        assert!(SQL_FIND_SIGNOS_BY_CONSULTA.contains("WHERE id_consulta = $1"));
        assert!(SQL_FIND_SIGNOS_BY_PACIENTE.contains("WHERE id_paciente = $1"));
        assert!(SQL_FIND_ALL_SIGNOS.contains("ORDER BY id_signo ASC"));

        for query in &[
            SQL_FIND_SIGNO_BY_ID,
            SQL_FIND_SIGNOS_BY_CONSULTA,
            SQL_FIND_SIGNOS_BY_PACIENTE,
            SQL_FIND_ALL_SIGNOS,
        ] {
            let upper = query.to_uppercase();
            assert!(upper.contains("SELECT"));
            assert!(!upper.contains("DELETE"));
            assert!(!upper.contains("UPDATE"));
            assert!(!upper.contains("INSERT"));
        }
    }
}
