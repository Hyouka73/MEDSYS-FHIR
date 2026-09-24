//! Configuración parametrizada del servidor Axum.

use std::env;

/// Parámetros de configuración del servidor HTTP y capas adyacentes.
#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub rules_path: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: env::var("SERVER_HOST")
                .or_else(|_| env::var("MEDSYS_HOST"))
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("SERVER_PORT")
                .or_else(|_| env::var("MEDSYS_PORT"))
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                "postgres://medsys_user:medsys_secure_pass_2026@localhost:5432/medsys_legacy"
                    .to_string()
            }),
            rules_path: env::var("MAPPING_RULES_PATH")
                .unwrap_or_else(|_| "mapping_rules_specification.yaml".to_string()),
        }
    }
}

impl ServerConfig {
    /// Genera la dirección socket en formato "host:port".
    pub fn socket_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
