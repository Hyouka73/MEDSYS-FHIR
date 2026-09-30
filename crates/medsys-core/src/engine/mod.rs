pub mod transform;

pub use transform::*;

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
    use chrono::{NaiveDate, NaiveDateTime};
    use helios_fhir::r4::Resource;
    use rust_decimal::Decimal;

    use crate::model::legacy::{
        LegacyConsulta, LegacyDiagnostico, LegacyPaciente, LegacySignoVital,
    };

    const SPECIFICATION_YAML: &str = include_str!("../../../../mapping_rules_specification.yaml");

    // =========================================================================
    // PRUEBAS DE SPRINT 1: MOTOR DE MAPEO YAML
    // =========================================================================

    #[test]
    fn test_parse_embedded_specification_yaml() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML)
            .expect("El archivo mapping_rules_specification.yaml debe deserializar sin errores");
        assert_eq!(rules.version, "1.1.0");
        assert_eq!(rules.resources.len(), 5);
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
        assert_eq!(
            curp.system.as_deref(),
            Some("urn:oid:2.16.840.1.113883.4.629")
        );
        assert_eq!(curp.effective_use(), Some("official"));

        let gender = patient
            .mappings
            .iter()
            .find(|m| m.source_column.as_deref() == Some("sexo_biologico"))
            .expect("Debe existir mapeo para sexo_biologico");
        let dict = gender
            .dictionary
            .as_ref()
            .expect("Debe tener diccionario de mapeo");
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
        let observations: Vec<_> = rules
            .resources
            .iter()
            .filter(|r| r.resource_type == "Observation")
            .collect();
        assert_eq!(observations.len(), 2);

        // 3A: Panel de Presión Arterial
        let bp_obs = observations
            .iter()
            .find(|o| {
                o.mappings
                    .iter()
                    .any(|m| m.source_column.as_deref() == Some("presion_sistolica"))
            })
            .expect("Observation BP debe existir");
        assert_eq!(bp_obs.source_table, "tbl_signos_vitales");

        // 3B: Temperatura Corporal
        let temp_obs = observations
            .iter()
            .find(|o| {
                o.mappings
                    .iter()
                    .any(|m| m.source_column.as_deref() == Some("temperatura_celsius"))
            })
            .expect("Observation Temperatura debe existir");
        assert_eq!(temp_obs.source_table, "tbl_signos_vitales");

        let temp = temp_obs
            .mappings
            .iter()
            .find(|m| m.source_column.as_deref() == Some("temperatura_celsius"))
            .expect("Debe existir mapeo de temperatura");
        assert_eq!(temp.unit.as_deref(), Some("Cel"));
        assert_eq!(temp.code.as_deref(), Some("Cel"));
        assert_eq!(temp.system.as_deref(), Some("http://unitsofmeasure.org"));
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

    // =========================================================================
    // PRUEBAS DE SPRINT 2: MODELADO CANÓNICO FHIR R4
    // =========================================================================

    #[test]
    fn test_tarea_2_1_transform_patient_canonical() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML).unwrap();

        let paciente_legado = LegacyPaciente {
            id_paciente: 1,
            curp: "ROMA900101HCSNN01".to_string(),
            primer_nombre: "Alberto".to_string(),
            segundo_nombre: Some("Manuel".to_string()),
            apellido_paterno: "Ramos".to_string(),
            apellido_materno: Some("Gómez".to_string()),
            fecha_nacimiento: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            sexo_biologico: Some("M".to_string()),
            telefono_contacto: Some("9611234567".to_string()),
            fecha_registro: None,
        };

        let fhir_patient = transform_patient(&paciente_legado, Some(&rules))
            .expect("La transformación de Patient no debe fallar");

        // Verificación de identificador nacional oficial CURP
        assert_eq!(
            fhir_patient.id.as_ref().and_then(|i| i.value.as_deref()),
            Some("1")
        );
        let identifiers = fhir_patient
            .identifier
            .as_ref()
            .expect("Debe tener identificadores");
        assert_eq!(identifiers.len(), 1);
        let curp_id = &identifiers[0];
        assert_eq!(
            curp_id.value.as_ref().and_then(|v| v.value.as_deref()),
            Some("ROMA900101HCSNN01")
        );
        assert_eq!(
            curp_id.system.as_ref().and_then(|s| s.value.as_deref()),
            Some("urn:oid:2.16.840.1.113883.4.629")
        );
        assert_eq!(
            curp_id.r#use.as_ref().and_then(|u| u.value.as_deref()),
            Some("official")
        );

        // Verificación de nombres demográficos
        let names = fhir_patient.name.as_ref().expect("Debe contener nombres");
        assert_eq!(names.len(), 1);
        let name = &names[0];
        assert_eq!(
            name.family.as_ref().and_then(|f| f.value.as_deref()),
            Some("Ramos Gómez")
        );
        let givens: Vec<&str> = name
            .given
            .as_ref()
            .expect("Debe tener nombres de pila")
            .iter()
            .filter_map(|g| g.value.as_deref())
            .collect();
        assert_eq!(givens, vec!["Alberto", "Manuel"]);

        // Verificación de género y fecha de nacimiento
        assert_eq!(
            fhir_patient
                .gender
                .as_ref()
                .and_then(|g| g.value.as_deref()),
            Some("male")
        );
        assert!(fhir_patient.birth_date.is_some());

        // Verificación de contacto telefónico
        let telecoms = fhir_patient
            .telecom
            .as_ref()
            .expect("Debe contener telecom");
        assert_eq!(
            telecoms[0].value.as_ref().and_then(|v| v.value.as_deref()),
            Some("9611234567")
        );

        // Serialización canónica a JSON
        let json_str = serialize_to_fhir_json(&Resource::Patient(Box::new(fhir_patient)))
            .expect("Debe serializar a application/fhir+json");
        assert!(json_str.contains("\"resourceType\": \"Patient\""));
        assert!(json_str.contains("ROMA900101HCSNN01"));
    }

    #[test]
    fn test_tarea_2_2_transform_encounter_canonical() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML).unwrap();

        let consulta_legada = LegacyConsulta {
            id_consulta: 1,
            id_paciente: 1,
            cedula_medico_tratante: "8472910".to_string(),
            nombre_medico: "Dra. María Elena Cruz Martínez".to_string(),
            estado_consulta: "FINALIZADA".to_string(),
            motivo_consulta: "Control trimestral de hipertensión arterial".to_string(),
            fecha_hora_inicio: NaiveDateTime::parse_from_str(
                "2026-09-18 09:15:00",
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap(),
            fecha_hora_fin: NaiveDateTime::parse_from_str(
                "2026-09-18 09:40:00",
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap(),
            unidad_medica: Some("CESSA Tuxtla Poniente".to_string()),
        };

        let fhir_encounter = transform_encounter(&consulta_legada, Some(&rules))
            .expect("La transformación de Encounter no debe fallar");

        assert_eq!(
            fhir_encounter.id.as_ref().and_then(|i| i.value.as_deref()),
            Some("1")
        );
        assert_eq!(fhir_encounter.status.value.as_deref(), Some("finished"));

        // Verificación de clase ambulatoria AMB
        assert_eq!(
            fhir_encounter
                .class
                .code
                .as_ref()
                .and_then(|c| c.value.as_deref()),
            Some("AMB")
        );
        assert_eq!(
            fhir_encounter
                .class
                .system
                .as_ref()
                .and_then(|s| s.value.as_deref()),
            Some("http://terminology.hl7.org/CodeSystem/v3-ActCode")
        );

        // Sujeto referenciado
        assert_eq!(
            fhir_encounter
                .subject
                .as_ref()
                .and_then(|s| s.reference.as_ref())
                .and_then(|r| r.value.as_deref()),
            Some("Patient/1")
        );

        // Periodo de consulta
        assert!(fhir_encounter.period.is_some());

        // Participante médico tratante con cédula SEP
        let participants = fhir_encounter
            .participant
            .as_ref()
            .expect("Debe incluir participantes");
        assert_eq!(participants.len(), 1);
        let doctor = participants[0]
            .individual
            .as_ref()
            .expect("Debe incluir datos del médico");
        assert_eq!(
            doctor.display.as_ref().and_then(|d| d.value.as_deref()),
            Some("Dra. María Elena Cruz Martínez")
        );
        let doc_id = doctor.identifier.as_ref().expect("Debe contener cédula");
        assert_eq!(
            doc_id.value.as_ref().and_then(|v| v.value.as_deref()),
            Some("8472910")
        );
        assert_eq!(
            doc_id.system.as_ref().and_then(|s| s.value.as_deref()),
            Some("http://cedulaprofesional.sep.gob.mx")
        );

        // Serialización
        let json_str = serialize_to_fhir_json(&Resource::Encounter(Box::new(fhir_encounter)))
            .expect("Debe serializar Encounter a JSON");
        assert!(json_str.contains("\"resourceType\": \"Encounter\""));
        assert!(json_str.contains("finished"));
        assert!(json_str.contains("8472910"));
    }

    #[test]
    fn test_tarea_2_3_transform_observation_vital_signs() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML).unwrap();

        let signo_legado = LegacySignoVital {
            id_signo: 1,
            id_consulta: 1,
            id_paciente: 1,
            presion_sistolica: 130,
            presion_diastolica: 85,
            frecuencia_cardiaca: 76,
            frecuencia_respiratoria: Some(18),
            temperatura_celsius: Decimal::new(366, 1), // 36.6
            peso_kg: Some(Decimal::new(785, 1)),
            talla_cm: Some(172),
            fecha_registro: NaiveDateTime::parse_from_str(
                "2026-09-18 09:18:00",
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap(),
        };

        // 1. Panel de Presión Arterial
        let fhir_bp = transform_observation_blood_pressure(&signo_legado, Some(&rules))
            .expect("Transformación de presión arterial debe ser exitosa");

        assert_eq!(
            fhir_bp.id.as_ref().and_then(|i| i.value.as_deref()),
            Some("bp-1")
        );
        assert_eq!(fhir_bp.status.value.as_deref(), Some("final"));

        let cat = fhir_bp.category.as_ref().expect("Debe contener categoría");
        let cat_coding = cat[0].coding.as_ref().unwrap();
        assert_eq!(
            cat_coding[0].code.as_ref().and_then(|c| c.value.as_deref()),
            Some("vital-signs")
        );

        let components = fhir_bp
            .component
            .as_ref()
            .expect("Panel de PA debe tener componentes");
        assert_eq!(components.len(), 2);
        // Sistólica
        let comp_sist = &components[0];
        let sist_code = comp_sist.code.coding.as_ref().unwrap();
        assert_eq!(
            sist_code[0].code.as_ref().and_then(|c| c.value.as_deref()),
            Some("8480-6")
        );
        // Diastólica
        let comp_diast = &components[1];
        let diast_code = comp_diast.code.coding.as_ref().unwrap();
        assert_eq!(
            diast_code[0].code.as_ref().and_then(|c| c.value.as_deref()),
            Some("8462-4")
        );

        // 2. Temperatura Corporal
        let fhir_temp = transform_observation_temperature(&signo_legado, Some(&rules))
            .expect("Transformación de temperatura debe ser exitosa");

        assert_eq!(
            fhir_temp.id.as_ref().and_then(|i| i.value.as_deref()),
            Some("temp-1")
        );
        let temp_code = fhir_temp.code.coding.as_ref().unwrap();
        assert_eq!(
            temp_code[0].code.as_ref().and_then(|c| c.value.as_deref()),
            Some("8310-5")
        );

        // 3. Frecuencia Cardíaca
        let fhir_hr = transform_observation_heart_rate(&signo_legado, Some(&rules))
            .expect("Transformación de frecuencia cardíaca debe ser exitosa");

        assert_eq!(
            fhir_hr.id.as_ref().and_then(|i| i.value.as_deref()),
            Some("hr-1")
        );
        let hr_code = fhir_hr.code.coding.as_ref().unwrap();
        assert_eq!(
            hr_code[0].code.as_ref().and_then(|c| c.value.as_deref()),
            Some("8867-4")
        );

        // Serialización
        let json_bp = serialize_to_fhir_json(&Resource::Observation(Box::new(fhir_bp))).unwrap();
        assert!(json_bp.contains("\"resourceType\": \"Observation\""));
        assert!(json_bp.contains("8480-6"));

        let json_temp =
            serialize_to_fhir_json(&Resource::Observation(Box::new(fhir_temp))).unwrap();
        assert!(json_temp.contains("\"resourceType\": \"Observation\""));
        assert!(json_temp.contains("8310-5"));
        assert!(json_temp.contains("Cel"));

        let json_hr =
            serialize_to_fhir_json(&Resource::Observation(Box::new(fhir_hr))).unwrap();
        assert!(json_hr.contains("\"resourceType\": \"Observation\""));
        assert!(json_hr.contains("8867-4"));
        assert!(json_hr.contains("/min"));
    }

    #[test]
    fn test_tarea_2_4_transform_condition_canonical() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML).unwrap();

        let diagnostico_legado = LegacyDiagnostico {
            id_diagnostico: 1,
            id_consulta: 1,
            id_paciente: 1,
            codigo_cie10: "I10".to_string(),
            descripcion_diagnostico: "Hipertensión esencial (primaria)".to_string(),
            tipo_diagnostico: "CONFIRMADO".to_string(),
            fecha_diagnostico: NaiveDate::from_ymd_opt(2026, 9, 18).unwrap(),
        };

        let fhir_condition = transform_condition(&diagnostico_legado, Some(&rules))
            .expect("Transformación de Condition debe ser exitosa");

        assert_eq!(
            fhir_condition.id.as_ref().and_then(|i| i.value.as_deref()),
            Some("1")
        );

        // Estado clínico y de verificación
        let clin_stat = fhir_condition
            .clinical_status
            .as_ref()
            .expect("Debe tener clinicalStatus");
        let clin_code = clin_stat.coding.as_ref().unwrap();
        assert_eq!(
            clin_code[0].code.as_ref().and_then(|c| c.value.as_deref()),
            Some("active")
        );

        let ver_stat = fhir_condition
            .verification_status
            .as_ref()
            .expect("Debe tener verificationStatus");
        let ver_code = ver_stat.coding.as_ref().unwrap();
        assert_eq!(
            ver_code[0].code.as_ref().and_then(|c| c.value.as_deref()),
            Some("confirmed")
        );

        // Código CIE-10 internacional
        let cond_code = fhir_condition
            .code
            .as_ref()
            .expect("Debe tener código CIE-10");
        let cie_coding = cond_code.coding.as_ref().unwrap();
        assert_eq!(
            cie_coding[0].code.as_ref().and_then(|c| c.value.as_deref()),
            Some("I10")
        );
        assert_eq!(
            cie_coding[0]
                .system
                .as_ref()
                .and_then(|s| s.value.as_deref()),
            Some("http://hl7.org/fhir/sid/icd-10")
        );
        assert_eq!(
            cie_coding[0]
                .display
                .as_ref()
                .and_then(|d| d.value.as_deref()),
            Some("Hipertensión esencial (primaria)")
        );

        // Sujeto y encuentro
        assert_eq!(
            fhir_condition
                .subject
                .reference
                .as_ref()
                .and_then(|r| r.value.as_deref()),
            Some("Patient/1")
        );
        assert_eq!(
            fhir_condition
                .encounter
                .as_ref()
                .and_then(|e| e.reference.as_ref())
                .and_then(|r| r.value.as_deref()),
            Some("Encounter/1")
        );

        // Serialización
        let json_cond =
            serialize_to_fhir_json(&Resource::Condition(Box::new(fhir_condition))).unwrap();
        assert!(json_cond.contains("\"resourceType\": \"Condition\""));
        assert!(json_cond.contains("I10"));
        assert!(json_cond.contains("Hipertensión esencial (primaria)"));
    }

    #[test]
    fn test_create_operation_outcome_canonical() {
        let outcome = create_operation_outcome(
            Some("outcome-404"),
            "error",
            "not-found",
            "Paciente con identificador '999' no localizado en la base de datos clínica.",
        );

        let json = serialize_to_fhir_json(&Resource::OperationOutcome(Box::new(outcome)))
            .expect("Debe serializar OperationOutcome");

        assert!(json.contains("\"resourceType\": \"OperationOutcome\""));
        assert!(json.contains("\"severity\": \"error\""));
        assert!(json.contains("\"code\": \"not-found\""));
        assert!(json.contains("Paciente con identificador '999' no localizado"));
        assert!(json.contains("\"language\": \"es\""));
    }

    #[test]
    fn test_create_searchset_bundle_canonical() {
        let outcome = create_operation_outcome(
            Some("outcome-1"),
            "information",
            "informational",
            "Resultado de búsqueda procesado correctamente",
        );

        let bundle = create_searchset_bundle(
            Some("bundle-test"),
            vec![Resource::OperationOutcome(Box::new(outcome))],
            Some(1),
        );

        let json = serialize_to_fhir_json(&Resource::Bundle(Box::new(bundle)))
            .expect("Debe serializar Bundle");

        assert!(json.contains("\"resourceType\": \"Bundle\""));
        assert!(json.contains("\"type\": \"searchset\""));
        assert!(json.contains("\"total\": 1"));
        assert!(json.contains("\"entry\""));
    }
}
