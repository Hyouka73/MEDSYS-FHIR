pub mod fhir_helpers;
pub mod legacy;
pub mod mapping;

pub use fhir_helpers::*;
pub use legacy::{LegacyConsulta, LegacyDiagnostico, LegacyPaciente, LegacySignoVital};
pub use mapping::{FieldMapping, MappingRules, ResourceMapping, SupportedResource};
