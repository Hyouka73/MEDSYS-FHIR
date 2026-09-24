use thiserror::Error;

/// Enumerador centralizado de errores de dominio del middleware MedSys-FHIR.
/// Todos los errores internos se mapean a este tipo para prevenir el uso de unwrap() y expect().
#[derive(Error, Debug)]
pub enum MedSysError {
    #[error("Error de serialización o deserialización YAML: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("Error de serialización o deserialización JSON: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Error de entrada/salida: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Regla de mapeo inválida: {0}")]
    InvalidRule(String),

    #[error("Recurso FHIR desconocido o no soportado: {0}")]
    UnknownResource(String),

    #[error("Campo requerido faltante '{field}' para el recurso '{resource}'")]
    MissingField { field: String, resource: String },

    #[error("Fallo durante la transformación de datos: {0}")]
    TransformationError(String),

    #[error("Error de validación sintáctica FHIR: {0}")]
    ValidationError(String),

    #[error("Recurso clínico no encontrado: {0}")]
    NotFound(String),

    #[error("Error de base de datos relacional: {0}")]
    DatabaseError(String),

    #[error("Error interno del motor de interoperabilidad: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, MedSysError>;
