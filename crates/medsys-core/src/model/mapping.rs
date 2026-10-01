use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::str::FromStr;

use crate::error::MedSysError;

/// Recursos oficiales HL7 FHIR soportados por el alcance de MedSys-FHIR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SupportedResource {
    Patient,
    Encounter,
    Observation,
    Condition,
}

impl FromStr for SupportedResource {
    type Err = MedSysError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Patient" => Ok(Self::Patient),
            "Encounter" => Ok(Self::Encounter),
            "Observation" => Ok(Self::Observation),
            "Condition" => Ok(Self::Condition),
            other => Err(MedSysError::UnknownResource(other.to_string())),
        }
    }
}

impl std::fmt::Display for SupportedResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Patient => write!(f, "Patient"),
            Self::Encounter => write!(f, "Encounter"),
            Self::Observation => write!(f, "Observation"),
            Self::Condition => write!(f, "Condition"),
        }
    }
}

/// Especificación completa de reglas declarativas de mapeo YAML.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MappingRules {
    pub version: String,
    pub description: Option<String>,
    pub resources: Vec<ResourceMapping>,
}

impl MappingRules {
    /// Obtiene las reglas de mapeo correspondientes a un recurso específico.
    pub fn get_resource_mapping(&self, resource: SupportedResource) -> Option<&ResourceMapping> {
        let name = resource.to_string();
        self.resources.iter().find(|r| r.resource_type == name)
    }
}

/// Configuración de mapeo declarativo para un recurso FHIR individual.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceMapping {
    pub resource_type: String,
    pub source_table: String,
    pub primary_key: String,
    pub mappings: Vec<FieldMapping>,
}

impl ResourceMapping {
    /// Parsea y valida el tipo de recurso contra la lista canónica soportada.
    pub fn parsed_resource_type(&self) -> Result<SupportedResource, MedSysError> {
        self.resource_type.parse()
    }
}

/// Regla individual de mapeo campo a campo o valor constante a FHIR target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldMapping {
    #[serde(default)]
    pub source_column: Option<String>,

    #[serde(default)]
    pub constant_value: Option<String>,

    pub target_path: String,

    #[serde(default)]
    pub system: Option<String>,

    #[serde(default)]
    pub use_type: Option<String>,

    #[serde(rename = "use", default)]
    pub use_field: Option<String>,

    #[serde(default)]
    pub optional: Option<bool>,

    #[serde(default)]
    pub transform: Option<String>,

    #[serde(default)]
    pub dictionary: Option<BTreeMap<String, String>>,

    #[serde(default)]
    pub unit: Option<String>,

    #[serde(default)]
    pub code: Option<String>,

    #[serde(default)]
    pub fallback_value: Option<String>,
}

impl FieldMapping {
    /// Determina si el campo es explícitamente opcional.
    pub fn is_optional(&self) -> bool {
        self.optional.unwrap_or(false)
    }

    /// Retorna el atributo de uso ('use' o 'use_type') si está presente.
    pub fn effective_use(&self) -> Option<&str> {
        self.use_field.as_deref().or(self.use_type.as_deref())
    }

    /// Retorna el valor de degradación elegante (fallback) si está configurado en la regla.
    pub fn fallback_value(&self) -> Option<&str> {
        self.fallback_value.as_deref()
    }
}
