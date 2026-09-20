//! medsys-server: Punto de entrada del servidor Axum.

#[tokio::main]
async fn main() {
    println!("MedSys-FHIR Server v{}", env!("CARGO_PKG_VERSION"));
}
