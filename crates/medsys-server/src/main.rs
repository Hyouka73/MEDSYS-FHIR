//! medsys-server: Punto de entrada del servidor Axum y middleware de interoperabilidad FHIR.

use medsys_core::parse_mapping_rules;
use medsys_db::{init_pool_lazy, DbConfig, MedsysRepositories};
use medsys_server::{create_router, AppState, ServerConfig};
use std::fs;
use tracing::{error, info};

fn db_config_or_default() -> DbConfig {
    DbConfig::from_env().unwrap_or_default()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Cargar variables de entorno desde .env si existe
    let _ = dotenvy::dotenv();

    // 2. Inicializar sistema de telemetría y logs estructurados
    tracing_subscriber::fmt::init();

    info!(
        "Iniciando MedSys-FHIR Server v{}",
        env!("CARGO_PKG_VERSION")
    );

    // 3. Cargar configuración del servidor y base de datos
    let server_config = ServerConfig::default();
    let db_config = db_config_or_default();
    info!(
        "Conectando al pool de persistencia PostgreSQL en: {}",
        db_config.masked_url()
    );

    // 4. Inicializar pool SQLx y repositorios
    let pool = match init_pool_lazy(&db_config) {
        Ok(p) => p,
        Err(e) => {
            error!("Fallo al configurar el pool de conexiones SQLx: {e}");
            return Err(e.into());
        }
    };
    let repositories = MedsysRepositories::new(pool.clone());

    // 5. Cargar y parsear reglas de mapeo YAML
    let yaml_content = match fs::read_to_string(&server_config.rules_path) {
        Ok(c) => c,
        Err(e) => {
            error!(
                "No se pudo leer el archivo de reglas YAML '{}': {e}",
                server_config.rules_path
            );
            return Err(e.into());
        }
    };

    let mapping_rules = match parse_mapping_rules(&yaml_content) {
        Ok(rules) => {
            info!("Reglas de mapeo cargadas exitosamente (v{})", rules.version);
            rules
        }
        Err(e) => {
            error!("Error al parsear especificación de mapeo YAML: {e}");
            return Err(e.into());
        }
    };

    // 6. Construir estado y enrutador Axum
    let state = AppState::new(repositories, mapping_rules, pool);
    let app = create_router(state);

    // 7. Bindeo y escucha en socket TCP
    let addr = server_config.socket_addr();
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Servidor MedSys-FHIR escuchando activamente en http://{addr}");
    info!("Endpoints REST FHIR disponibles en http://{addr}/fhir/r4/");
    info!("Dashboard API de salud en http://{addr}/health");

    axum::serve(listener, app).await?;

    Ok(())
}
