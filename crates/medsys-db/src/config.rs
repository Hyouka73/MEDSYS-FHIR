//! Configuración parametrizada para la persistencia en PostgreSQL de MedSys-FHIR.

use medsys_core::error::{MedSysError, Result};
use std::env;
use std::time::Duration;

/// Configuración de conexión y parámetros del pool de PostgreSQL para MedSys-FHIR.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DbConfig {
    /// Cadena de conexión canónica a PostgreSQL (e.g., postgres://user:pass@localhost:5432/db).
    pub database_url: String,
    /// Número máximo de conexiones simultáneas en el pool.
    pub max_connections: u32,
    /// Número mínimo de conexiones inactivas mantenidas en el pool.
    pub min_connections: u32,
    /// Tiempo máximo en segundos para adquirir una conexión del pool antes de fallar.
    pub acquire_timeout_secs: u64,
    /// Tiempo máximo en segundos que una conexión inactiva puede permanecer en el pool.
    pub idle_timeout_secs: u64,
    /// Tiempo de vida máximo en segundos de una conexión antes de ser reciclada.
    pub max_lifetime_secs: u64,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            database_url:
                "postgres://medsys_user:medsys_secure_pass_2026@localhost:5432/medsys_legacy"
                    .to_string(),
            max_connections: 10,
            min_connections: 2,
            acquire_timeout_secs: 5,
            idle_timeout_secs: 300,
            max_lifetime_secs: 1800,
        }
    }
}

impl DbConfig {
    /// Crea una nueva configuración especificando la URL de la base de datos y valores por defecto para el pool.
    pub fn new(database_url: impl Into<String>) -> Self {
        Self {
            database_url: database_url.into(),
            ..Default::default()
        }
    }

    /// Construye la configuración leyendo variables de entorno del sistema o desde un archivo `.env`.
    pub fn from_env() -> Result<Self> {
        // Cargar variables desde archivo .env si está disponible localmente
        let _ = dotenvy::dotenv();

        let database_url = if let Ok(url) = env::var("DATABASE_URL") {
            url
        } else {
            let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
            let port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
            let db = env::var("POSTGRES_DB").unwrap_or_else(|_| "medsys_legacy".to_string());
            let user = env::var("POSTGRES_USER").unwrap_or_else(|_| "medsys_user".to_string());
            let password = env::var("POSTGRES_PASSWORD")
                .unwrap_or_else(|_| "medsys_secure_pass_2026".to_string());
            format!("postgres://{user}:{password}@{host}:{port}/{db}")
        };

        if database_url.trim().is_empty() {
            return Err(MedSysError::DatabaseError(
                "La URL de conexión a la base de datos no puede estar vacía.".to_string(),
            ));
        }

        let max_connections = env::var("DB_MAX_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10);

        let min_connections = env::var("DB_MIN_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(2);

        let acquire_timeout_secs = env::var("DB_ACQUIRE_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5);

        let idle_timeout_secs = env::var("DB_IDLE_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(300);

        let max_lifetime_secs = env::var("DB_MAX_LIFETIME_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1800);

        Ok(Self {
            database_url,
            max_connections,
            min_connections,
            acquire_timeout_secs,
            idle_timeout_secs,
            max_lifetime_secs,
        })
    }

    /// Retorna la URL de conexión enmascarando contraseñas sensibles para imprimir con seguridad en logs.
    pub fn masked_url(&self) -> String {
        if let Some(proto_end) = self.database_url.find("://") {
            let after_proto = &self.database_url[proto_end + 3..];
            if let Some(at_idx) = after_proto.find('@') {
                let user_pass = &after_proto[..at_idx];
                if let Some(colon_idx) = user_pass.find(':') {
                    let user = &user_pass[..colon_idx];
                    let rest = &after_proto[at_idx..];
                    return format!("{}://{user}:*****{rest}", &self.database_url[..proto_end]);
                }
            }
        }
        self.database_url.clone()
    }

    /// Duración tipada para el timeout de adquisición de conexión.
    pub fn acquire_timeout(&self) -> Duration {
        Duration::from_secs(self.acquire_timeout_secs)
    }

    /// Duración tipada para el timeout de inactividad de conexión.
    pub fn idle_timeout(&self) -> Duration {
        Duration::from_secs(self.idle_timeout_secs)
    }

    /// Duración tipada para el tiempo de vida máximo de una conexión.
    pub fn max_lifetime(&self) -> Duration {
        Duration::from_secs(self.max_lifetime_secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_db_config() {
        let config = DbConfig::default();
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.min_connections, 2);
        assert_eq!(config.acquire_timeout_secs, 5);
        assert_eq!(config.idle_timeout_secs, 300);
        assert_eq!(config.max_lifetime_secs, 1800);
        assert!(config.database_url.contains("medsys_legacy"));
    }

    #[test]
    fn test_masked_url_obscures_password() {
        let config = DbConfig::new(
            "postgres://admin:super_secret_password_123@db.hospital.local:5432/clinica",
        );
        let masked = config.masked_url();
        assert!(!masked.contains("super_secret_password_123"));
        assert!(masked.contains("admin:*****@db.hospital.local:5432/clinica"));
    }

    #[test]
    fn test_masked_url_without_password() {
        let config = DbConfig::new("postgres://localhost:5432/testdb");
        let masked = config.masked_url();
        assert_eq!(masked, "postgres://localhost:5432/testdb");
    }

    #[test]
    fn test_custom_durations() {
        let config = DbConfig {
            acquire_timeout_secs: 12,
            idle_timeout_secs: 60,
            max_lifetime_secs: 900,
            ..Default::default()
        };
        assert_eq!(config.acquire_timeout(), Duration::from_secs(12));
        assert_eq!(config.idle_timeout(), Duration::from_secs(60));
        assert_eq!(config.max_lifetime(), Duration::from_secs(900));
    }
}
