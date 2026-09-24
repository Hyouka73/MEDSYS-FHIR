//! Repositorio de lectura parametrizada para la tabla `tbl_pacientes`.
//!
//! Cumple estrictamente con el principio de solo lectura y prevención de inyecciones SQL
//! mediante el uso de consultas parametrizadas ($1, $2).

use crate::entities::PacienteEntity;
use crate::error::map_sqlx_error;
use medsys_core::error::{MedSysError, Result};
use medsys_core::model::legacy::LegacyPaciente;
use sqlx::PgPool;
use tracing::debug;

pub const SQL_FIND_PACIENTE_BY_ID: &str = r#"
    SELECT 
        id_paciente, curp, primer_nombre, segundo_nombre,
        apellido_paterno, apellido_materno, fecha_nacimiento,
        sexo_biologico, telefono_contacto, fecha_registro
    FROM tbl_pacientes
    WHERE id_paciente = $1
"#;

pub const SQL_FIND_PACIENTE_BY_CURP: &str = r#"
    SELECT 
        id_paciente, curp, primer_nombre, segundo_nombre,
        apellido_paterno, apellido_materno, fecha_nacimiento,
        sexo_biologico, telefono_contacto, fecha_registro
    FROM tbl_pacientes
    WHERE curp = $1
"#;

pub const SQL_FIND_ALL_PACIENTES: &str = r#"
    SELECT 
        id_paciente, curp, primer_nombre, segundo_nombre,
        apellido_paterno, apellido_materno, fecha_nacimiento,
        sexo_biologico, telefono_contacto, fecha_registro
    FROM tbl_pacientes
    ORDER BY id_paciente ASC
"#;

/// Repositorio de solo lectura para acceder a los datos de pacientes de la NOM-004.
#[derive(Clone, Debug)]
pub struct PacienteRepository {
    pool: PgPool,
}

impl PacienteRepository {
    /// Inicializa un nuevo repositorio con el pool de conexiones provisto.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Retorna una referencia al pool de conexiones subyacente.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Busca un paciente por su clave primaria (`id_paciente`).
    /// Si no existe, retorna `MedSysError::NotFound`.
    pub async fn find_by_id(&self, id: i32) -> Result<LegacyPaciente> {
        debug!(id_paciente = id, "Consultando paciente por ID");
        let entity = sqlx::query_as::<_, PacienteEntity>(SQL_FIND_PACIENTE_BY_ID)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_sqlx_error)?
            .ok_or_else(|| {
                MedSysError::NotFound(format!(
                    "Paciente con ID '{id}' no encontrado en el sistema legado."
                ))
            })?;

        Ok(entity.into())
    }

    /// Busca un paciente por su clave primaria retornando `Option<LegacyPaciente>`.
    pub async fn find_optional_by_id(&self, id: i32) -> Result<Option<LegacyPaciente>> {
        debug!(
            id_paciente = id,
            "Consultando opcionalmente paciente por ID"
        );
        let opt = sqlx::query_as::<_, PacienteEntity>(SQL_FIND_PACIENTE_BY_ID)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(opt.map(Into::into))
    }

    /// Busca un paciente por su identificador oficial nacional (`curp`).
    /// Si no existe, retorna `MedSysError::NotFound`.
    pub async fn find_by_curp(&self, curp: &str) -> Result<LegacyPaciente> {
        debug!(curp = curp, "Consultando paciente por CURP oficial");
        let entity = sqlx::query_as::<_, PacienteEntity>(SQL_FIND_PACIENTE_BY_CURP)
            .bind(curp)
            .fetch_optional(&self.pool)
            .await
            .map_err(map_sqlx_error)?
            .ok_or_else(|| {
                MedSysError::NotFound(format!(
                    "Paciente con CURP '{curp}' no encontrado en el sistema legado."
                ))
            })?;

        Ok(entity.into())
    }

    /// Retorna todos los pacientes registrados ordenados ascendentemente por su ID.
    pub async fn find_all(&self) -> Result<Vec<LegacyPaciente>> {
        debug!("Consultando todos los pacientes registrados");
        let entities = sqlx::query_as::<_, PacienteEntity>(SQL_FIND_ALL_PACIENTES)
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
    fn test_sql_queries_syntax_and_safety() {
        assert!(SQL_FIND_PACIENTE_BY_ID.contains("WHERE id_paciente = $1"));
        assert!(SQL_FIND_PACIENTE_BY_CURP.contains("WHERE curp = $1"));
        assert!(SQL_FIND_ALL_PACIENTES.contains("ORDER BY id_paciente ASC"));

        // Verificar que no contengan operaciones DDL o DML mutantes
        for query in &[
            SQL_FIND_PACIENTE_BY_ID,
            SQL_FIND_PACIENTE_BY_CURP,
            SQL_FIND_ALL_PACIENTES,
        ] {
            let upper = query.to_uppercase();
            assert!(upper.contains("SELECT"));
            assert!(!upper.contains("DELETE"));
            assert!(!upper.contains("UPDATE"));
            assert!(!upper.contains("INSERT"));
            assert!(!upper.contains("DROP"));
        }
    }
}
