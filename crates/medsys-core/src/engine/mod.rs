pub mod evaluator;
pub mod transform;

pub use evaluator::*;
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
    use crate::model::mapping::FieldMapping;

    const SPECIFICATION_YAML: &str = include_str!("../../../../mapping_rules.yaml");

    // =========================================================================
    // PRUEBAS DE SPRINT 1: MOTOR DE MAPEO YAML
    // =========================================================================

    #[test]
    fn test_parse_embedded_specification_yaml() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML)
            .expect("El archivo mapping_rules.yaml debe deserializar sin errores");
        assert_eq!(rules.version, "1.2.0");
        assert_eq!(rules.resources.len(), 7);
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

        let telecom_mapping = patient
            .mappings
            .iter()
            .find(|m| m.source_column.as_deref() == Some("telefono_contacto"))
            .expect("telefono_contacto debe mapearse como opcional");
        assert_eq!(telecom_mapping.target_path, "telecom[0].value");
        assert_eq!(telecom_mapping.system.as_deref(), Some("phone"));
        assert_eq!(telecom_mapping.effective_use(), Some("mobile"));
        assert!(telecom_mapping.is_optional());
        assert!(!gender.is_optional());
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
        assert_eq!(observations.len(), 4);

        // 3A: Panel de Presión Arterial
        let bp_obs = observations
            .iter()
            .find(|o| {
                o.observation_type.as_deref() == Some("blood_pressure_panel")
                    || o.mappings
                        .iter()
                        .any(|m| m.source_column.as_deref() == Some("presion_sistolica"))
            })
            .expect("Observation BP debe existir");
        assert_eq!(bp_obs.source_table, "tbl_signos_vitales");

        // Sistólica LOINC y UCUM
        let sist_code = bp_obs
            .mappings
            .iter()
            .find(|m| m.target_path == "component[0].code.coding[0].code")
            .expect("Debe existir mapeo para código LOINC de sistólica");
        assert_eq!(sist_code.constant_value.as_deref(), Some("8480-6"));
        assert_eq!(sist_code.system.as_deref(), Some("http://loinc.org"));
        assert_eq!(
            sist_code.display.as_deref(),
            Some("Systolic blood pressure")
        );

        let sist_val = bp_obs
            .mappings
            .iter()
            .find(|m| m.target_path == "component[0].valueQuantity.value")
            .expect("Debe existir mapeo para valor de sistólica");
        assert_eq!(sist_val.source_column.as_deref(), Some("presion_sistolica"));
        assert_eq!(sist_val.unit.as_deref(), Some("mmHg"));
        assert_eq!(sist_val.code.as_deref(), Some("mm[Hg]"));
        assert_eq!(
            sist_val.system.as_deref(),
            Some("http://unitsofmeasure.org")
        );

        // Diastólica LOINC y UCUM
        let diast_code = bp_obs
            .mappings
            .iter()
            .find(|m| m.target_path == "component[1].code.coding[0].code")
            .expect("Debe existir mapeo para código LOINC de diastólica");
        assert_eq!(diast_code.constant_value.as_deref(), Some("8462-4"));
        assert_eq!(diast_code.system.as_deref(), Some("http://loinc.org"));
        assert_eq!(
            diast_code.display.as_deref(),
            Some("Diastolic blood pressure")
        );

        let diast_val = bp_obs
            .mappings
            .iter()
            .find(|m| m.target_path == "component[1].valueQuantity.value")
            .expect("Debe existir mapeo para valor de diastólica");
        assert_eq!(
            diast_val.source_column.as_deref(),
            Some("presion_diastolica")
        );
        assert_eq!(diast_val.unit.as_deref(), Some("mmHg"));
        assert_eq!(diast_val.code.as_deref(), Some("mm[Hg]"));
        assert_eq!(
            diast_val.system.as_deref(),
            Some("http://unitsofmeasure.org")
        );

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

        // 3C: Frecuencia Cardiaca (8867-4, obligatoria)
        let hr_obs = observations
            .iter()
            .find(|o| o.observation_type.as_deref() == Some("heart_rate"))
            .expect("Observation heart_rate debe existir");
        let hr_code = hr_obs
            .mappings
            .iter()
            .find(|m| m.target_path == "code.coding[0].code")
            .unwrap();
        assert_eq!(hr_code.constant_value.as_deref(), Some("8867-4"));
        let hr_val = hr_obs
            .mappings
            .iter()
            .find(|m| m.source_column.as_deref() == Some("frecuencia_cardiaca"))
            .expect("Debe mapear frecuencia_cardiaca");
        assert_eq!(hr_val.unit.as_deref(), Some("/min"));
        assert!(!hr_val.is_optional());

        // 3D: Frecuencia Respiratoria (9279-1, optional: true)
        let rr_obs = observations
            .iter()
            .find(|o| o.observation_type.as_deref() == Some("respiratory_rate"))
            .expect("Observation respiratory_rate debe existir");
        let rr_code = rr_obs
            .mappings
            .iter()
            .find(|m| m.target_path == "code.coding[0].code")
            .unwrap();
        assert_eq!(rr_code.constant_value.as_deref(), Some("9279-1"));
        let rr_val = rr_obs
            .mappings
            .iter()
            .find(|m| m.source_column.as_deref() == Some("frecuencia_respiratoria"))
            .expect("Debe mapear frecuencia_respiratoria");
        assert_eq!(rr_val.unit.as_deref(), Some("/min"));
        assert!(rr_val.is_optional());
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

        let tipo_diag = condition
            .mappings
            .iter()
            .find(|m| m.source_column.as_deref() == Some("tipo_diagnostico"))
            .expect("Debe existir mapeo de tipo_diagnostico");
        assert!(tipo_diag.is_optional());
        assert!(tipo_diag.dictionary.is_some());
        assert_eq!(tipo_diag.target_path, "verificationStatus.coding[0].code");
    }

    // =========================================================================
    // PRUEBAS DE SPRINT 2: MODELADO CANÓNICO FHIR R4
    // =========================================================================

    #[test]
    fn test_tarea_2_1_transform_patient_canonical() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML).unwrap();

        let paciente_legado = LegacyPaciente {
            id_paciente: 1,
            curp: "ROMA900101HCSMNL09".to_string(),
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
            Some("ROMA900101HCSMNL09")
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

        // telefono_contacto opcional: presente en origen => telecom[0] con phone/mobile
        let telecom = fhir_patient.telecom.as_ref().expect("Debe incluir telecom");
        assert_eq!(
            telecom[0].value.as_ref().and_then(|v| v.value.as_deref()),
            Some("9611234567")
        );

        // Serialización canónica a JSON
        let json_str = serialize_to_fhir_json(&Resource::Patient(Box::new(fhir_patient)))
            .expect("Debe serializar a application/fhir+json");
        assert!(json_str.contains("\"resourceType\": \"Patient\""));
        assert!(json_str.contains("ROMA900101HCSMNL09"));
        assert!(json_str.contains("\"telecom\""));
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
        assert_eq!(
            sist_code[0]
                .system
                .as_ref()
                .and_then(|s| s.value.as_deref()),
            Some("http://loinc.org")
        );
        assert_eq!(
            sist_code[0]
                .display
                .as_ref()
                .and_then(|d| d.value.as_deref()),
            Some("Systolic blood pressure")
        );
        match &comp_sist.value {
            Some(helios_fhir::r4::ObservationComponentValue::Quantity(q)) => {
                assert!(q.value.is_some());
                assert!(format!("{:?}", q.value).contains("130"));
                assert_eq!(
                    q.unit.as_ref().and_then(|u| u.value.as_deref()),
                    Some("mmHg")
                );
                assert_eq!(
                    q.system.as_ref().and_then(|s| s.value.as_deref()),
                    Some("http://unitsofmeasure.org")
                );
                assert_eq!(
                    q.code.as_ref().and_then(|c| c.value.as_deref()),
                    Some("mm[Hg]")
                );
            }
            other => panic!(
                "Esperaba Quantity para Sistólica, pero se obtuvo {:?}",
                other
            ),
        }

        // Diastólica
        let comp_diast = &components[1];
        let diast_code = comp_diast.code.coding.as_ref().unwrap();
        assert_eq!(
            diast_code[0].code.as_ref().and_then(|c| c.value.as_deref()),
            Some("8462-4")
        );
        assert_eq!(
            diast_code[0]
                .system
                .as_ref()
                .and_then(|s| s.value.as_deref()),
            Some("http://loinc.org")
        );
        assert_eq!(
            diast_code[0]
                .display
                .as_ref()
                .and_then(|d| d.value.as_deref()),
            Some("Diastolic blood pressure")
        );
        match &comp_diast.value {
            Some(helios_fhir::r4::ObservationComponentValue::Quantity(q)) => {
                assert!(q.value.is_some());
                assert!(format!("{:?}", q.value).contains("85"));
                assert_eq!(
                    q.unit.as_ref().and_then(|u| u.value.as_deref()),
                    Some("mmHg")
                );
                assert_eq!(
                    q.system.as_ref().and_then(|s| s.value.as_deref()),
                    Some("http://unitsofmeasure.org")
                );
                assert_eq!(
                    q.code.as_ref().and_then(|c| c.value.as_deref()),
                    Some("mm[Hg]")
                );
            }
            other => panic!(
                "Esperaba Quantity para Diastólica, pero se obtuvo {:?}",
                other
            ),
        }

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
        assert!(json_bp.contains("8462-4"));
        assert!(json_bp.contains("Systolic blood pressure"));
        assert!(json_bp.contains("Diastolic blood pressure"));
        assert!(json_bp.contains("http://loinc.org"));
        assert!(json_bp.contains("http://unitsofmeasure.org"));
        assert!(json_bp.contains("mm[Hg]"));

        let v: serde_json::Value = serde_json::from_str(&json_bp).unwrap();
        assert_eq!(
            v["component"][0]["code"]["coding"][0]["system"],
            "http://loinc.org"
        );
        assert_eq!(v["component"][0]["code"]["coding"][0]["code"], "8480-6");
        assert_eq!(
            v["component"][0]["code"]["coding"][0]["display"],
            "Systolic blood pressure"
        );
        assert_eq!(
            v["component"][0]["valueQuantity"]["system"],
            "http://unitsofmeasure.org"
        );
        assert_eq!(v["component"][0]["valueQuantity"]["code"], "mm[Hg]");
        assert_eq!(v["component"][0]["valueQuantity"]["unit"], "mmHg");
        assert_eq!(v["component"][0]["valueQuantity"]["value"], 130);

        assert_eq!(
            v["component"][1]["code"]["coding"][0]["system"],
            "http://loinc.org"
        );
        assert_eq!(v["component"][1]["code"]["coding"][0]["code"], "8462-4");
        assert_eq!(
            v["component"][1]["code"]["coding"][0]["display"],
            "Diastolic blood pressure"
        );
        assert_eq!(
            v["component"][1]["valueQuantity"]["system"],
            "http://unitsofmeasure.org"
        );
        assert_eq!(v["component"][1]["valueQuantity"]["code"], "mm[Hg]");
        assert_eq!(v["component"][1]["valueQuantity"]["unit"], "mmHg");
        assert_eq!(v["component"][1]["valueQuantity"]["value"], 85);

        let json_temp =
            serialize_to_fhir_json(&Resource::Observation(Box::new(fhir_temp))).unwrap();
        assert!(json_temp.contains("\"resourceType\": \"Observation\""));
        assert!(json_temp.contains("8310-5"));
        assert!(json_temp.contains("Cel"));

        let json_hr = serialize_to_fhir_json(&Resource::Observation(Box::new(fhir_hr))).unwrap();
        assert!(json_hr.contains("\"resourceType\": \"Observation\""));
        assert!(json_hr.contains("8867-4"));
        assert!(json_hr.contains("/min"));
    }

    #[test]
    fn test_evaluator_observation_blood_pressure_declarative() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML).unwrap();
        let bp_mapping = rules
            .get_observation_mapping("blood_pressure_panel")
            .expect("Mapping de Observation BP debe existir");

        let mut row = serde_json::Map::new();
        row.insert("id_signo".to_string(), serde_json::json!(10));
        row.insert("id_paciente".to_string(), serde_json::json!(5));
        row.insert("id_consulta".to_string(), serde_json::json!(20));
        row.insert(
            "fecha_registro".to_string(),
            serde_json::json!("2026-09-18 10:30:00"),
        );
        row.insert("presion_sistolica".to_string(), serde_json::json!(125));
        row.insert("presion_diastolica".to_string(), serde_json::json!(82));

        let evaluated = evaluate_resource_mapping(bp_mapping, &row)
            .expect("Evaluación declarativa de BP debe ser exitosa");

        assert_eq!(evaluated["resourceType"], "Observation");
        assert_eq!(evaluated["status"], "final");
        assert_eq!(evaluated["subject"]["reference"], "Patient/5");
        assert_eq!(evaluated["encounter"]["reference"], "Encounter/20");
        assert_eq!(evaluated["code"]["coding"][0]["code"], "85354-9");
        assert_eq!(evaluated["code"]["coding"][0]["system"], "http://loinc.org");
        assert_eq!(
            evaluated["code"]["coding"][0]["display"],
            "Blood pressure panel with all children optional"
        );

        // Componente Sistólica
        assert_eq!(
            evaluated["component"][0]["code"]["coding"][0]["system"],
            "http://loinc.org"
        );
        assert_eq!(
            evaluated["component"][0]["code"]["coding"][0]["code"],
            "8480-6"
        );
        assert_eq!(
            evaluated["component"][0]["code"]["coding"][0]["display"],
            "Systolic blood pressure"
        );
        assert_eq!(
            evaluated["component"][0]["valueQuantity"]["system"],
            "http://unitsofmeasure.org"
        );
        assert_eq!(evaluated["component"][0]["valueQuantity"]["code"], "mm[Hg]");
        assert_eq!(evaluated["component"][0]["valueQuantity"]["unit"], "mmHg");
        assert_eq!(evaluated["component"][0]["valueQuantity"]["value"], 125);

        // Componente Diastólica
        assert_eq!(
            evaluated["component"][1]["code"]["coding"][0]["system"],
            "http://loinc.org"
        );
        assert_eq!(
            evaluated["component"][1]["code"]["coding"][0]["code"],
            "8462-4"
        );
        assert_eq!(
            evaluated["component"][1]["code"]["coding"][0]["display"],
            "Diastolic blood pressure"
        );
        assert_eq!(
            evaluated["component"][1]["valueQuantity"]["system"],
            "http://unitsofmeasure.org"
        );
        assert_eq!(evaluated["component"][1]["valueQuantity"]["code"], "mm[Hg]");
        assert_eq!(evaluated["component"][1]["valueQuantity"]["unit"], "mmHg");
        assert_eq!(evaluated["component"][1]["valueQuantity"]["value"], 82);
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
            tipo_diagnostico: Some("CONFIRMADO".to_string()),
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

    // =========================================================================
    // PRUEBAS DE RESILIENCIA Y DEGRADACIÓN ELEGANTE (GRACEFUL DEGRADATION)
    // =========================================================================

    #[test]
    fn test_yaml_has_no_fallback_or_data_absent_reason() {
        assert!(!SPECIFICATION_YAML.contains("fallback_value"));
        assert!(!SPECIFICATION_YAML.contains("use_data_absent_reason"));
    }

    #[test]
    fn test_evaluator_corrupted_date_fails_closed() {
        use serde_json::json;

        let rule = FieldMapping {
            source_column: Some("fecha_diagnostico".to_string()),
            target_path: "recordedDate".to_string(),
            transform: Some("date_iso8601".to_string()),
            ..Default::default()
        };
        let mut row = serde_json::Map::new();
        row.insert("fecha_diagnostico".to_string(), json!("FECHA_INVALIDA"));

        let mut target_json = json!({});
        match evaluate_field_mapping(&rule, "Condition", &row, &mut target_json) {
            Err(MedSysError::ProcessingError(msg)) => assert!(msg.contains("Data corruption")),
            other => panic!("Esperaba ProcessingError pero obtuvo {:?}", other),
        }
    }

    #[test]
    fn test_evaluator_optional_corrupt_value_still_fails_closed() {
        use serde_json::json;

        let rule = FieldMapping {
            source_column: Some("tipo_diagnostico".to_string()),
            target_path: "verificationStatus.coding[0].code".to_string(),
            optional: Some(true),
            dictionary: Some(
                [("CONFIRMADO".to_string(), "confirmed".to_string())]
                    .into_iter()
                    .collect(),
            ),
            ..Default::default()
        };
        let mut row = serde_json::Map::new();
        row.insert("tipo_diagnostico".to_string(), json!("DUDOSO"));

        let mut target_json = json!({});
        let res = evaluate_field_mapping(&rule, "Condition", &row, &mut target_json);
        assert!(matches!(res, Err(MedSysError::ProcessingError(_))));
    }

    #[test]
    fn test_evaluator_optional_null_or_empty_is_omitted() {
        use serde_json::json;

        let rule = FieldMapping {
            source_column: Some("telefono_contacto".to_string()),
            target_path: "telecom[0].value".to_string(),
            system: Some("phone".to_string()),
            use_field: Some("mobile".to_string()),
            optional: Some(true),
            ..Default::default()
        };

        for val in [json!(null), json!("")] {
            let mut row = serde_json::Map::new();
            row.insert("telefono_contacto".to_string(), val);
            let mut target_json = json!({});
            evaluate_field_mapping(&rule, "Patient", &row, &mut target_json)
                .expect("Opcional nulo debe omitirse sin error");
            assert!(target_json.get("telecom").is_none());
        }

        let mut row = serde_json::Map::new();
        row.insert("telefono_contacto".to_string(), json!("9611234567"));
        let mut target_json = json!({});
        evaluate_field_mapping(&rule, "Patient", &row, &mut target_json).unwrap();
        assert_eq!(target_json["telecom"][0]["value"], "9611234567");
        assert_eq!(target_json["telecom"][0]["system"], "phone");
        assert_eq!(target_json["telecom"][0]["use"], "mobile");
    }

    #[test]
    fn test_evaluator_mandatory_null_column_fails_closed() {
        use serde_json::json;

        let rule = FieldMapping {
            source_column: Some("sexo_biologico".to_string()),
            target_path: "gender".to_string(),
            ..Default::default()
        };
        let mut row = serde_json::Map::new();
        row.insert("sexo_biologico".to_string(), json!(null));
        let mut target_json = json!({});
        let res = evaluate_field_mapping(&rule, "Patient", &row, &mut target_json);
        assert!(matches!(res, Err(MedSysError::ProcessingError(_))));
    }

    #[test]
    fn test_set_json_path_complex_nested_structures() {
        use serde_json::json;

        let mut root = json!({});
        set_json_path(&mut root, "code.coding[0].code", json!("8480-6")).unwrap();
        set_json_path(
            &mut root,
            "code.coding[0].system",
            json!("http://loinc.org"),
        )
        .unwrap();
        set_json_path(&mut root, "component[0].valueQuantity.value", json!(120.0)).unwrap();
        set_json_path(&mut root, "component[1].valueQuantity.value", json!(80.0)).unwrap();

        assert_eq!(root["code"]["coding"][0]["code"], "8480-6");
        assert_eq!(root["code"]["coding"][0]["system"], "http://loinc.org");
        assert_eq!(root["component"][0]["valueQuantity"]["value"], 120.0);
        assert_eq!(root["component"][1]["valueQuantity"]["value"], 80.0);
    }

    #[test]
    fn test_is_valid_curp() {
        assert!(is_valid_curp("ROMA900101HCSMNL09"));
        assert!(!is_valid_curp(""));
        assert!(!is_valid_curp("ROMA900101HCSMNL0")); // 17 caracteres
        assert!(!is_valid_curp("roma900101HCSMNL09")); // minúsculas
        assert!(!is_valid_curp("ROMA901301HCSMNL09")); // mes 13
        assert!(!is_valid_curp("ROMA900101ZCSMNL09")); // sexo inválido
        assert!(!is_valid_curp("ROMA900101HXXMNL09")); // entidad inválida
        assert!(!is_valid_curp("ROMA900101HCSANL09")); // vocal donde va consonante
        assert!(!is_valid_curp("ROMA900101HCSMNL0A")); // verificador no numérico
        assert!(!is_valid_curp("ROMA900101HCSMNL0é")); // no ASCII sin pánico
    }

    #[test]
    fn test_patient_invalid_curp_fails_closed() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML).unwrap();
        let paciente = LegacyPaciente {
            id_paciente: 9,
            curp: "CURP_MALFORMADA".to_string(),
            primer_nombre: "Ana".to_string(),
            segundo_nombre: None,
            apellido_paterno: "Pérez".to_string(),
            apellido_materno: None,
            fecha_nacimiento: NaiveDate::from_ymd_opt(1995, 5, 20).unwrap(),
            sexo_biologico: Some("F".to_string()),
            telefono_contacto: None,
            fecha_registro: None,
        };
        match transform_patient(&paciente, Some(&rules)) {
            Err(MedSysError::ValidationError(msg)) => assert!(msg.contains("T5")),
            other => panic!("Esperaba ValidationError T5 pero obtuvo {:?}", other),
        }
    }

    fn paciente_base() -> LegacyPaciente {
        LegacyPaciente {
            id_paciente: 99,
            curp: "ROMA900101HCSMNL09".to_string(),
            primer_nombre: "María".to_string(),
            segundo_nombre: None,
            apellido_paterno: "Pérez".to_string(),
            apellido_materno: None,
            fecha_nacimiento: NaiveDate::from_ymd_opt(1995, 5, 20).unwrap(),
            sexo_biologico: Some("F".to_string()),
            telefono_contacto: None,
            fecha_registro: None,
        }
    }

    #[test]
    fn test_transform_patient_corrupt_or_null_gender_fails_closed() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML).unwrap();

        let mut corrupto = paciente_base();
        corrupto.sexo_biologico = Some("ENTROPIA_CLINICA_DESCONOCIDA".to_string());
        assert!(matches!(
            transform_patient(&corrupto, Some(&rules)),
            Err(MedSysError::ProcessingError(_))
        ));

        let mut nulo = paciente_base();
        nulo.sexo_biologico = None;
        assert!(matches!(
            transform_patient(&nulo, Some(&rules)),
            Err(MedSysError::ProcessingError(_))
        ));
    }

    #[test]
    fn test_transform_patient_telecom_optional() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML).unwrap();

        let mut con_tel = paciente_base();
        con_tel.telefono_contacto = Some(" 9611234567 ".to_string());
        let p = transform_patient(&con_tel, Some(&rules)).unwrap();
        let telecom = p.telecom.expect("Debe incluir telecom");
        assert_eq!(telecom.len(), 1);
        assert_eq!(
            telecom[0].value.as_ref().and_then(|v| v.value.as_deref()),
            Some("9611234567")
        );
        assert_eq!(
            telecom[0].system.as_ref().and_then(|v| v.value.as_deref()),
            Some("phone")
        );
        assert_eq!(
            telecom[0].r#use.as_ref().and_then(|v| v.value.as_deref()),
            Some("mobile")
        );

        for tel in [None, Some("   ".to_string())] {
            let mut sin_tel = paciente_base();
            sin_tel.telefono_contacto = tel;
            let p = transform_patient(&sin_tel, Some(&rules)).unwrap();
            assert!(p.telecom.is_none());
        }
    }

    #[test]
    fn test_transform_condition_raw_corrupted_date_fails_closed() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML).unwrap();
        let res = transform_condition_raw(
            101,
            202,
            303,
            "I10",
            "Hipertensión esencial",
            "CONFIRMADO",
            "FECHA_CORRUPTA_MALFORMADA",
            Some(&rules),
        );
        match res {
            Err(MedSysError::ProcessingError(msg)) => assert!(msg.contains("Data corruption")),
            other => panic!("Esperaba ProcessingError pero obtuvo {:?}", other),
        }
    }

    #[test]
    fn test_transform_encounter_corrupt_status_fails_closed() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML).unwrap();
        let consulta_corrupta = LegacyConsulta {
            id_consulta: 50,
            id_paciente: 1,
            cedula_medico_tratante: "1234567".to_string(),
            nombre_medico: "Dr. Carlos".to_string(),
            estado_consulta: "ESTADO_BASURA".to_string(),
            motivo_consulta: "Revisión".to_string(),
            fecha_hora_inicio: NaiveDateTime::parse_from_str(
                "2026-09-18 09:00:00",
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap(),
            fecha_hora_fin: NaiveDateTime::parse_from_str(
                "2026-09-18 09:30:00",
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap(),
            unidad_medica: None,
        };
        match transform_encounter(&consulta_corrupta, Some(&rules)) {
            Err(MedSysError::ProcessingError(msg)) => assert!(msg.contains("Data corruption")),
            other => panic!("Esperaba ProcessingError pero obtuvo {:?}", other),
        }
    }

    fn diagnostico_base(tipo: Option<&str>) -> LegacyDiagnostico {
        LegacyDiagnostico {
            id_diagnostico: 10,
            id_consulta: 20,
            id_paciente: 30,
            codigo_cie10: "E11.9".to_string(),
            descripcion_diagnostico: "Diabetes mellitus tipo 2".to_string(),
            tipo_diagnostico: tipo.map(str::to_string),
            fecha_diagnostico: NaiveDate::from_ymd_opt(2026, 9, 20).unwrap(),
        }
    }

    #[test]
    fn test_transform_condition_omits_verification_status_when_tipo_diagnostico_is_none() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML).unwrap();
        let cond = transform_condition(&diagnostico_base(None), Some(&rules))
            .expect("tipo_diagnostico nulo es opcional y no debe fallar");
        assert!(cond.verification_status.is_none());

        let json_str = serialize_to_fhir_json(&Resource::Condition(Box::new(cond))).unwrap();
        assert!(!json_str.contains("verificationStatus"));
        assert!(!json_str.contains("provisional"));

        let vacio = transform_condition(&diagnostico_base(Some("")), Some(&rules)).unwrap();
        assert!(vacio.verification_status.is_none());
    }

    #[test]
    fn test_transform_condition_fails_closed_when_tipo_diagnostico_invalid() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML).unwrap();
        match transform_condition(&diagnostico_base(Some("DUDOSO")), Some(&rules)) {
            Err(MedSysError::ProcessingError(msg)) => assert!(msg.contains("T3")),
            other => panic!("Esperaba ProcessingError T3 pero obtuvo {:?}", other),
        }
    }

    #[test]
    fn test_transform_condition_maps_catalog_values() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML).unwrap();
        for (raw, expected) in [("CONFIRMADO", "confirmed"), ("PRESUNTIVO", "provisional")] {
            let cond = transform_condition(&diagnostico_base(Some(raw)), Some(&rules)).unwrap();
            let code = cond
                .verification_status
                .and_then(|v| v.coding)
                .and_then(|c| c.into_iter().next())
                .and_then(|c| c.code)
                .and_then(|c| c.value);
            assert_eq!(code.as_deref(), Some(expected));
        }
    }

    #[test]
    fn test_evaluator_condition_declarative_null_tipo_diagnostico_is_omitted() {
        use serde_json::json;

        let rules = parse_mapping_rules(SPECIFICATION_YAML).unwrap();
        let cond_mapping = rules
            .get_resource_mapping(SupportedResource::Condition)
            .expect("Condition mapping debe existir");

        let mut row = serde_json::Map::new();
        row.insert("id_diagnostico".to_string(), json!(77));
        row.insert("id_consulta".to_string(), json!(88));
        row.insert("id_paciente".to_string(), json!(99));
        row.insert("codigo_cie10".to_string(), json!("J00"));
        row.insert(
            "descripcion_diagnostico".to_string(),
            json!("Rinofaringitis aguda (resfriado común)"),
        );
        row.insert("tipo_diagnostico".to_string(), serde_json::Value::Null);
        row.insert("fecha_diagnostico".to_string(), json!("2026-09-18"));

        let target_json = evaluate_resource_mapping(cond_mapping, &row)
            .expect("tipo_diagnostico NULL es opcional");
        assert!(target_json.get("verificationStatus").is_none());
        assert_eq!(target_json["recordedDate"], "2026-09-18");
    }

    fn signo_base(resp: Option<i32>) -> LegacySignoVital {
        LegacySignoVital {
            id_signo: 5,
            id_consulta: 2,
            id_paciente: 1,
            presion_sistolica: 120,
            presion_diastolica: 80,
            frecuencia_cardiaca: 72,
            frecuencia_respiratoria: resp,
            temperatura_celsius: Decimal::new(366, 1),
            peso_kg: None,
            talla_cm: None,
            fecha_registro: NaiveDateTime::parse_from_str(
                "2026-09-18 09:15:00",
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap(),
        }
    }

    #[test]
    fn test_transform_observation_respiratory_rate_some_and_none() {
        let rules = parse_mapping_rules(SPECIFICATION_YAML).unwrap();

        let obs = transform_observation_respiratory_rate(&signo_base(Some(18)), Some(&rules))
            .unwrap()
            .expect("Debe emitir Observation con frecuencia respiratoria");
        assert_eq!(
            obs.id.as_ref().and_then(|i| i.value.as_deref()),
            Some("resp-5")
        );
        let json_str = serialize_to_fhir_json(&Resource::Observation(Box::new(obs))).unwrap();
        assert!(json_str.contains("9279-1"));
        assert!(json_str.contains("\"/min\""));

        assert!(
            transform_observation_respiratory_rate(&signo_base(None), Some(&rules))
                .unwrap()
                .is_none()
        );
        assert!(matches!(
            transform_observation_respiratory_rate(&signo_base(Some(0)), Some(&rules)),
            Err(MedSysError::ProcessingError(_))
        ));
    }

    #[test]
    fn test_parse_archetype_mapping_rules_s0_to_s3() {
        const RULES_S0_YAML: &str = include_str!("../../../../config/mapping_rules_s0.yaml");
        const RULES_S1_YAML: &str = include_str!("../../../../config/mapping_rules_s1.yaml");
        const RULES_S2_YAML: &str = include_str!("../../../../config/mapping_rules_s2.yaml");
        const RULES_S3_YAML: &str = include_str!("../../../../config/mapping_rules_s3.yaml");

        // 1. Validar S0 (Base normalizado)
        let rules_s0 = parse_mapping_rules(RULES_S0_YAML)
            .expect("config/mapping_rules_s0.yaml debe parsear correctamente");
        assert_eq!(rules_s0.version, "1.2.0");
        assert_eq!(rules_s0.resources.len(), 7);

        // 2. Validar S1 (Sábana plana desnormalizada)
        let rules_s1 = parse_mapping_rules(RULES_S1_YAML)
            .expect("config/mapping_rules_s1.yaml debe parsear correctamente");
        assert_eq!(rules_s1.version, "1.2.0");
        assert_eq!(rules_s1.resources.len(), 7);
        let pat_s1 = rules_s1
            .get_resource_mapping(SupportedResource::Patient)
            .expect("Patient debe existir en S1");
        assert_eq!(pat_s1.source_table, "tbl_sabana_consulta_externa");
        assert_eq!(pat_s1.distinct_on.as_deref(), Some("id_paciente"));

        // 3. Validar S2 (Catálogos externos)
        let rules_s2 = parse_mapping_rules(RULES_S2_YAML)
            .expect("config/mapping_rules_s2.yaml debe parsear correctamente");
        assert_eq!(rules_s2.version, "1.2.0");
        assert_eq!(rules_s2.resources.len(), 7);
        let pat_s2 = rules_s2
            .get_resource_mapping(SupportedResource::Patient)
            .expect("Patient debe existir en S2");
        assert_eq!(pat_s2.source_table, "tbl_pacientes_s2");
        let sex_rule = pat_s2
            .mappings
            .iter()
            .find(|m| m.source_column.as_deref() == Some("id_sexo"))
            .expect("id_sexo debe existir en S2");
        assert!(sex_rule.lookup.is_some());
        assert_eq!(
            sex_rule.lookup.as_ref().unwrap().table,
            "cat_sexo_biologico"
        );

        // 4. Validar S3 (Esquema legado con deuda técnica)
        let rules_s3 = parse_mapping_rules(RULES_S3_YAML)
            .expect("config/mapping_rules_s3.yaml debe parsear correctamente");
        assert_eq!(rules_s3.version, "1.2.0");
        assert!(rules_s3.resources.len() >= 7);
        let pat_s3 = rules_s3
            .get_resource_mapping(SupportedResource::Patient)
            .expect("Patient debe existir en S3");
        assert_eq!(pat_s3.source_table, "tbl_pacientes_s3");
        assert_eq!(pat_s3.primary_key, "pac_id");
        let enc_s3 = rules_s3
            .get_resource_mapping(SupportedResource::Encounter)
            .expect("Encounter debe existir en S3");
        assert!(!enc_s3.parent_checks.is_empty());
        assert_eq!(enc_s3.parent_checks[0].foreign_key_column, "pac_id");
        assert_eq!(enc_s3.parent_checks[0].parent_table, "tbl_pacientes_s3");
    }
}
