//! Configuración e inicialización del pool asíncrono de conexiones PostgreSQL con SQLx.

use crate::config::DbConfig;
use crate::error::map_sqlx_error;
use medsys_core::error::{MedSysError, Result};
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Row;
use tracing::{debug, info};

/// Construye las opciones configuradas del pool de PostgreSQL para SQLx.
pub fn create_pool_options(config: &DbConfig) -> PgPoolOptions {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(config.acquire_timeout())
        .idle_timeout(config.idle_timeout())
        .max_lifetime(config.max_lifetime())
}

/// Inicializa asíncronamente el pool de conexiones a PostgreSQL verificando la conexión inmediata.
pub async fn init_pool(config: &DbConfig) -> Result<PgPool> {
    info!(
        database_url = %config.masked_url(),
        max_connections = config.max_connections,
        min_connections = config.min_connections,
        acquire_timeout_secs = config.acquire_timeout_secs,
        "Inicializando pool de conexiones asíncrono SQLx para PostgreSQL"
    );

    let pool = create_pool_options(config)
        .connect(&config.database_url)
        .await
        .map_err(map_sqlx_error)?;

    debug!("Pool de conexiones SQLx establecido exitosamente.");
    Ok(pool)
}

/// Inicializa el pool de PostgreSQL en modo perezoso (lazy).
/// No bloquea la ejecución en caso de que el contenedor de base de datos tarde en estar disponible.
pub fn init_pool_lazy(config: &DbConfig) -> Result<PgPool> {
    info!(
        database_url = %config.masked_url(),
        max_connections = config.max_connections,
        min_connections = config.min_connections,
        acquire_timeout_secs = config.acquire_timeout_secs,
        "Inicializando pool de conexiones SQLx lazy para PostgreSQL"
    );

    create_pool_options(config)
        .connect_lazy(&config.database_url)
        .map_err(map_sqlx_error)
}

/// Gestor centralizado del ciclo de vida y acceso al pool de PostgreSQL para MedSys-FHIR.
#[derive(Clone, Debug)]
pub struct DbManager {
    pool: PgPool,
    config: DbConfig,
}

impl DbManager {
    /// Construye el gestor a partir de la configuración del entorno (.env / variables del sistema)
    /// conectando de inmediato al motor relacional.
    pub async fn from_env() -> Result<Self> {
        let config = DbConfig::from_env()?;
        Self::new(config).await
    }

    /// Construye el gestor conectando inmediatamente al pool de PostgreSQL.
    pub async fn new(config: DbConfig) -> Result<Self> {
        let pool = init_pool(&config).await?;
        Ok(Self { pool, config })
    }

    /// Construye el gestor con inicialización lazy (perezosa).
    pub fn new_lazy(config: DbConfig) -> Result<Self> {
        let pool = init_pool_lazy(&config)?;
        Ok(Self { pool, config })
    }

    /// Construye el gestor a partir de un pool y configuración existentes.
    pub fn from_pool(pool: PgPool, config: DbConfig) -> Self {
        Self { pool, config }
    }

    /// Referencia al pool subyacente de SQLx.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Referencia a la configuración activa utilizada.
    pub fn config(&self) -> &DbConfig {
        &self.config
    }

    /// Verifica la salud de la conexión a la base de datos ejecutando 'SELECT 1'.
    pub async fn healthcheck(&self) -> Result<()> {
        let row = sqlx::query("SELECT 1 as ping")
            .fetch_one(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        let ping: i32 = row.try_get("ping").map_err(map_sqlx_error)?;
        if ping == 1 {
            debug!("Healthcheck de base de datos PostgreSQL exitoso.");
            Ok(())
        } else {
            Err(MedSysError::DatabaseError(
                "Healthcheck devolvió un valor inesperado.".to_string(),
            ))
        }
    }

    /// Cierra ordenadamente el pool de conexiones.
    pub async fn close(&self) {
        info!("Cerrando pool de conexiones PostgreSQL...");
        self.pool.close().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_pool_options() {
        let config = DbConfig {
            max_connections: 15,
            min_connections: 3,
            acquire_timeout_secs: 10,
            ..Default::default()
        };

        // Debe construir PgPoolOptions sin error ni pánico
        let _options = create_pool_options(&config);
    }

    #[tokio::test]
    async fn test_init_pool_lazy_and_db_manager() {
        let config = DbConfig::default();
        let manager = DbManager::new_lazy(config.clone());
        assert!(manager.is_ok());

        let mgr = manager.unwrap();
        assert_eq!(mgr.config().max_connections, config.max_connections);
        assert_eq!(mgr.config().database_url, config.database_url);
    }

    #[tokio::test]
    async fn test_from_pool_constructor() {
        let config = DbConfig::new("postgres://test_user:test_pass@127.0.0.1:5432/test_db");
        let pool = init_pool_lazy(&config).expect("init_pool_lazy falló");
        let manager = DbManager::from_pool(pool, config.clone());

        assert_eq!(manager.config().database_url, config.database_url);
        assert_eq!(manager.pool().is_closed(), false);
    }
}
