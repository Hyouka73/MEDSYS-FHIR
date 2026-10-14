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

use crate::error::map_sqlx_error;
use medsys_core::error::{MedSysError, Result};
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

    /// Retorna una referencia al pool de conexiones subyacente.
    pub fn pool(&self) -> &PgPool {
        self.pacientes.pool()
    }

    /// Inicia una transacción asíncrona de solo lectura con nivel de aislamiento REPEATABLE READ.
    pub async fn begin_readonly_tx(&self) -> Result<sqlx::Transaction<'_, sqlx::Postgres>> {
        let mut tx = self.pacientes.pool().begin().await.map_err(map_sqlx_error)?;
        sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        Ok(tx)
    }

    /// Verifica activamente que el paciente padre exista en `tbl_pacientes` (categoría de falla T4).
    pub async fn verify_patient_exists(&self, id_paciente: i32) -> Result<()> {
        let mut tx = self.begin_readonly_tx().await?;
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM tbl_pacientes WHERE id_paciente = $1)",
        )
        .bind(id_paciente)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;

        tx.commit().await.map_err(map_sqlx_error)?;

        if !exists {
            return Err(MedSysError::ProcessingError(format!(
                "T4 Violación de integridad referencial: registro padre inexistente en tbl_pacientes con id_paciente={id_paciente}"
            )));
        }

        Ok(())
    }

    /// Verifica activamente que la consulta padre y el paciente existan (categoría de falla T4).
    pub async fn verify_encounter_and_patient_exist(
        &self,
        id_consulta: i32,
        id_paciente: i32,
    ) -> Result<()> {
        let mut tx = self.begin_readonly_tx().await?;

        let patient_exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM tbl_pacientes WHERE id_paciente = $1)",
        )
        .bind(id_paciente)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;

        if !patient_exists {
            return Err(MedSysError::ProcessingError(format!(
                "T4 Violación de integridad referencial: registro padre inexistente en tbl_pacientes con id_paciente={id_paciente}"
            )));
        }

        let encounter_exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM tbl_consultas WHERE id_consulta = $1)",
        )
        .bind(id_consulta)
        .fetch_one(&mut *tx)
        .await
        .map_err(map_sqlx_error)?;

        if !encounter_exists {
            return Err(MedSysError::ProcessingError(format!(
                "T4 Violación de integridad referencial: registro padre inexistente en tbl_consultas con id_consulta={id_consulta}"
            )));
        }

        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(())
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
