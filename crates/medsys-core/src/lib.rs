//! medsys-core: Dominio, modelos y motor de interoperabilidad HL7 FHIR R4.

pub mod engine;
pub mod error;
pub mod model;

pub use engine::{
    apply_dictionary, apply_transform, apply_unit_conversion, create_operation_outcome,
    create_searchset_bundle, evaluate_field_mapping, evaluate_resource_mapping, is_valid_curp,
    parse_date_iso8601, parse_datetime_iso8601, parse_datetime_with_formats, parse_mapping_rules,
    serialize_to_fhir_json, set_json_path, transform_condition, transform_condition_raw,
    transform_encounter, transform_observation_blood_pressure, transform_observation_heart_rate,
    transform_observation_respiratory_rate, transform_observation_temperature, transform_patient,
    validate_mapping_rules,
};
pub use error::{AppError, MedSysError, Result};
pub use model::fhir_helpers::{
    fhir_absent_element, fhir_data_absent_reason_extension, FHIR_DATA_ABSENT_REASON_URL,
};
pub use model::{
    FieldMapping, LegacyConsulta, LegacyDiagnostico, LegacyPaciente, LegacySignoVital, LookupRule,
    MappingRules, ParentCheckRule, ResourceMapping, SplitPartRule, SupportedResource,
};
