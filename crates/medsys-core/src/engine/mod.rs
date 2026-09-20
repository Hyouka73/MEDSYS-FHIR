use crate::error::{MedSysError, Result};
use crate::model::mapping::{MappingRules, SupportedResource};

/// Parsea una especificación de reglas de mapeo desde una cadena YAML en memoria.
pub fn parse_mapping_rules(yaml_content: &str) -> Result<MappingRules> {
    let rules: MappingRules = serde_yaml::from_str(yaml_content)?;
    validate_mapping_rules(&rules)?;
    Ok(rules)
}

/// Valida las reglas de mapeo contra las restricciones de negocio y arquitectura de MedSys-FHIR.
pub fn validate_mapping_rules(rules: &MappingRules) -> Result<()> {
    if rules.resources.is_empty() {
        return Err(MedSysError::InvalidRule(
            "La especificación de mapeo no contiene ningún recurso definido".into(),
        ));
    }

    let required_resources = [
        SupportedResource::Patient,
        SupportedResource::Encounter,
        SupportedResource::Observation,
        SupportedResource::Condition,
    ];

    for expected in required_resources {
        let mapping = rules.get_resource_mapping(expected).ok_or_else(|| {
            MedSysError::InvalidRule(format!(
                "Falta la especificación obligatoria para el recurso FHIR '{expected}'"
            ))
        })?;

        if mapping.source_table.trim().is_empty() {
            return Err(MedSysError::InvalidRule(format!(
                "El recurso '{expected}' no define una tabla origen 'source_table'"
            )));
        }

        if mapping.primary_key.trim().is_empty() {
            return Err(MedSysError::InvalidRule(format!(
                "El recurso '{expected}' no define una clave primaria 'primary_key'"
            )));
        }

        if mapping.mappings.is_empty() {
            return Err(MedSysError::InvalidRule(format!(
                "El recurso '{expected}' no contiene reglas de mapeo 'mappings'"
            )));
        }

        for (idx, field) in mapping.mappings.iter().enumerate() {
            if field.source_column.is_none() && field.constant_value.is_none() {
                return Err(MedSysError::InvalidRule(format!(
                    "Regla #{idx} en '{expected}' debe definir 'source_column' o 'constant_value'"
                )));
            }
            if field.target_path.trim().is_empty() {
                return Err(MedSysError::InvalidRule(format!(
                    "Regla #{idx} en '{expected}' define un 'target_path' vacío"
                )));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPECIFICATION_YAML: &str = include_str!("../../../../mapping_rules_specification.yaml");

    #[test]
    fn test_parse_embedded_specification_yaml() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML)
            .expect("El archivo mapping_rules_specification.yaml debe deserializar sin errores");
        assert_eq!(rules.version, "1.0.0");
        assert_eq!(rules.resources.len(), 4);
    }

    #[test]
    fn test_patient_resource_mappings() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML).unwrap();
        let patient = rules
            .get_resource_mapping(SupportedResource::Patient)
            .expect("Patient debe existir");
        assert_eq!(patient.source_table, "tbl_pacientes");
        assert_eq!(patient.primary_key, "id_paciente");

        let curp = patient
            .mappings
            .iter()
            .find(|m| m.source_column.as_deref() == Some("curp"))
            .expect("Debe existir mapeo para CURP");
        assert_eq!(curp.target_path, "identifier[0].value");
        assert_eq!(curp.system.as_deref(), Some("urn:oid:2.16.840.1.113883.4.629"));
        assert_eq!(curp.effective_use(), Some("official"));

        let gender = patient
            .mappings
            .iter()
            .find(|m| m.source_column.as_deref() == Some("sexo_biologico"))
            .expect("Debe existir mapeo para sexo_biologico");
        let dict = gender.dictionary.as_ref().expect("Debe tener diccionario de mapeo");
        assert_eq!(dict.get("M").map(|s| s.as_str()), Some("male"));
        assert_eq!(dict.get("F").map(|s| s.as_str()), Some("female"));
    }

    #[test]
    fn test_encounter_resource_mappings() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML).unwrap();
        let encounter = rules
            .get_resource_mapping(SupportedResource::Encounter)
            .expect("Encounter debe existir");
        assert_eq!(encounter.source_table, "tbl_consultas");

        let class_code = encounter
            .mappings
            .iter()
            .find(|m| m.constant_value.as_deref() == Some("AMB"))
            .expect("Debe existir mapeo de clase ambulatoria AMB");
        assert_eq!(class_code.target_path, "class.code");
        assert_eq!(
            class_code.system.as_deref(),
            Some("http://terminology.hl7.org/CodeSystem/v3-ActCode")
        );
    }

    #[test]
    fn test_observation_resource_mappings() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML).unwrap();
        let observation = rules
            .get_resource_mapping(SupportedResource::Observation)
            .expect("Observation debe existir");
        assert_eq!(observation.source_table, "tbl_signos_vitales");

        let temp = observation
            .mappings
            .iter()
            .find(|m| m.source_column.as_deref() == Some("temperatura_celsius"))
            .expect("Debe existir mapeo de temperatura");
        assert_eq!(temp.unit.as_deref(), Some("Cel"));
        assert_eq!(temp.code.as_deref(), Some("8310-5"));
        assert_eq!(temp.system.as_deref(), Some("http://loinc.org"));
    }

    #[test]
    fn test_condition_resource_mappings() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML).unwrap();
        let condition = rules
            .get_resource_mapping(SupportedResource::Condition)
            .expect("Condition debe existir");
        assert_eq!(condition.source_table, "tbl_diagnosticos");

        let cie10 = condition
            .mappings
            .iter()
            .find(|m| m.source_column.as_deref() == Some("codigo_cie10"))
            .expect("Debe existir mapeo de código CIE-10");
        assert_eq!(cie10.target_path, "code.coding[0].code");
        assert_eq!(
            cie10.system.as_deref(),
            Some("http://hl7.org/fhir/sid/icd-10")
        );
    }
}
