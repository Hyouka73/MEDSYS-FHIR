//! medsys-core: Dominio, modelos y motor de interoperabilidad HL7 FHIR R4.

pub mod engine;
pub mod error;
pub mod model;

pub use engine::{
    create_operation_outcome, create_searchset_bundle, parse_mapping_rules, serialize_to_fhir_json,
    transform_condition, transform_encounter, transform_observation_blood_pressure,
    transform_observation_heart_rate, transform_observation_temperature, transform_patient,
    validate_mapping_rules,
};
pub use error::{MedSysError, Result};
pub use model::{
    FieldMapping, LegacyConsulta, LegacyDiagnostico, LegacyPaciente, LegacySignoVital,
    MappingRules, ResourceMapping, SupportedResource,
};
