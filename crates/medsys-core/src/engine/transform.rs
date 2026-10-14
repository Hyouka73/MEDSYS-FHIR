use chrono::NaiveTime;
use helios_fhir::r4::{
    Bundle, BundleEntry, Condition, ContactPoint, Encounter, EncounterParticipant, HumanName,
    Observation, ObservationComponent, ObservationComponentValue, ObservationEffective,
    ObservationValue, OperationOutcome, OperationOutcomeIssue, Patient, Period, Quantity,
    Reference, Resource,
};
use helios_fhir::Element;
use rust_decimal::Decimal as RustDecimal;

use crate::engine::evaluator::parse_date_iso8601;
use crate::error::{MedSysError, Result};
use crate::model::fhir_helpers::{
    fhir_code, fhir_concept, fhir_date, fhir_datetime, fhir_decimal, fhir_identifier,
    fhir_reference, fhir_string, fhir_uri,
};
use crate::model::legacy::{LegacyConsulta, LegacyDiagnostico, LegacyPaciente, LegacySignoVital};
use crate::model::mapping::{MappingRules, SupportedResource};

/// Valida la estructura oficial de 18 caracteres de la CURP (RENAPO):
/// 4 letras, fecha `YYMMDD` válida, sexo `[HMX]`, entidad federativa, 3 consonantes internas,
/// homoclave `[A-Z0-9]` y dígito verificador `[0-9]`.
pub fn is_valid_curp(curp: &str) -> bool {
    const ENTIDADES: [&str; 33] = [
        "AS", "BC", "BS", "CC", "CL", "CM", "CS", "CH", "DF", "DG", "GT", "GR", "HG", "JC", "MC",
        "MN", "MS", "NT", "NL", "OC", "PL", "QT", "QR", "SP", "SL", "SR", "TC", "TS", "TL", "VZ",
        "YN", "ZS", "NE",
    ];
    let b = curp.as_bytes();
    if b.len() != 18 {
        return false;
    }
    let is_upper = |c: u8| c.is_ascii_uppercase();
    let is_digit = |c: u8| c.is_ascii_digit();
    let is_consonant = |c: u8| is_upper(c) && !matches!(c, b'A' | b'E' | b'I' | b'O' | b'U');
    if !b[0..4].iter().all(|&c| is_upper(c)) || !b[4..10].iter().all(|&c| is_digit(c)) {
        return false;
    }
    let month = (b[6] - b'0') * 10 + (b[7] - b'0');
    let day = (b[8] - b'0') * 10 + (b[9] - b'0');
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return false;
    }
    if !matches!(b[10], b'H' | b'M' | b'X') {
        return false;
    }
    if !ENTIDADES.iter().any(|e| e.as_bytes() == &b[11..13]) {
        return false;
    }
    b[13..16].iter().all(|&c| is_consonant(c))
        && (is_upper(b[16]) || is_digit(b[16]))
        && is_digit(b[17])
}

/// Transformador canónico de `tbl_pacientes` hacia el recurso oficial `Patient` de HL7 FHIR R4.
/// Implementa la Tarea 2.1 del Sprint 2 con degradación elegante.
pub fn transform_patient(
    paciente: &LegacyPaciente,
    rules: Option<&MappingRules>,
) -> Result<Patient> {
    let patient_mapping = rules.and_then(|r| r.get_resource_mapping(SupportedResource::Patient));

    let curp_rule = patient_mapping.and_then(|res_map| {
        res_map.mappings.iter().find(|m| {
            m.source_column.as_deref() == Some("curp") || m.target_path == "identifier[0].value"
        })
    });

    let (curp_system, curp_use) = if let Some(rule) = curp_rule {
        let sys = rule
            .system
            .as_deref()
            .unwrap_or("urn:oid:2.16.840.1.113883.4.629");
        let u = rule.effective_use().unwrap_or("official");
        (sys, u)
    } else {
        ("urn:oid:2.16.840.1.113883.4.629", "official")
    };

    // Identificador nacional oficial en México: CURP (T5: validación estructural RENAPO)
    if !is_valid_curp(&paciente.curp) {
        return Err(MedSysError::ValidationError(format!(
            "T5 Identificador CURP inválido: '{}'",
            paciente.curp
        )));
    }
    let curp_value = paciente.curp.as_str();

    let identifier = fhir_identifier(Some(curp_system), curp_value, Some(curp_use));

    // Nombres de la persona física
    let mut given_names = vec![fhir_string(paciente.primer_nombre.clone())];
    let mut display_given = vec![paciente.primer_nombre.clone()];
    if let Some(ref sec) = paciente.segundo_nombre {
        if !sec.trim().is_empty() {
            given_names.push(fhir_string(sec.clone()));
            display_given.push(sec.clone());
        }
    }

    let family_name = match &paciente.apellido_materno {
        Some(mat) if !mat.trim().is_empty() => {
            format!("{} {}", paciente.apellido_paterno, mat)
        }
        _ => paciente.apellido_paterno.clone(),
    };

    let human_name = HumanName {
        id: None,
        extension: None,
        r#use: Some(fhir_code("official")),
        text: Some(fhir_string(format!(
            "{} {}",
            display_given.join(" "),
            family_name
        ))),
        family: Some(fhir_string(family_name)),
        given: Some(given_names),
        prefix: None,
        suffix: None,
        period: None,
    };

    // Mapeo de género clínico según catálogo normativo (Fail-Closed: sin sustitución de datos)
    let gender_rule = patient_mapping.and_then(|res| {
        res.mappings.iter().find(|m| {
            m.source_column.as_deref() == Some("sexo_biologico") || m.target_path == "gender"
        })
    });

    let raw_gender = paciente
        .sexo_biologico
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            MedSysError::ProcessingError(
                "Data corruption: Columna 'sexo_biologico' nula o vacía para campo obligatorio"
                    .into(),
            )
        })?;

    let gender_value: String = match gender_rule.and_then(|r| r.dictionary.as_ref()) {
        Some(dict) => dict.get(raw_gender).cloned().ok_or_else(|| {
            MedSysError::ProcessingError(format!(
                "Data corruption: Valor de sexo_biologico '{raw_gender}' no encontrado en diccionario"
            ))
        })?,
        None => match raw_gender {
            "M" => "male".to_string(),
            "F" => "female".to_string(),
            "I" => "other".to_string(),
            _ => {
                return Err(MedSysError::ProcessingError(format!(
                    "Data corruption: Valor de sexo_biologico '{raw_gender}' no reconocido"
                )));
            }
        },
    };
    let gender: Option<helios_fhir::r4::Code> = Some(fhir_code(gender_value));

    // Teléfono de contacto opcional (optional: true): se omite si es NULL o vacío
    let telecom = match paciente.telefono_contacto.as_deref().map(str::trim) {
        Some(tel) if !tel.is_empty() => Some(vec![ContactPoint {
            id: None,
            extension: None,
            system: Some(fhir_code("phone")),
            value: Some(fhir_string(tel)),
            r#use: Some(fhir_code("mobile")),
            rank: None,
            period: None,
        }]),
        _ => None,
    };

    let birth_date = fhir_date(paciente.fecha_nacimiento)?;

    Ok(Patient {
        id: Some(fhir_string(paciente.id_paciente.to_string())),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: Some(vec![identifier]),
        active: Some(Element {
            id: None,
            extension: None,
            value: Some(true),
        }),
        name: Some(vec![human_name]),
        telecom,
        gender,
        birth_date: Some(birth_date),
        deceased: None,
        address: None,
        marital_status: None,
        multiple_birth: None,
        photo: None,
        contact: None,
        communication: None,
        general_practitioner: None,
        managing_organization: None,
        link: None,
    })
}

/// Transformador canónico de `tbl_consultas` hacia el recurso oficial `Encounter` de HL7 FHIR R4.
/// Implementa la Tarea 2.2 del Sprint 2.
pub fn transform_encounter(
    consulta: &LegacyConsulta,
    rules: Option<&MappingRules>,
) -> Result<Encounter> {
    let encounter_mapping =
        rules.and_then(|r| r.get_resource_mapping(SupportedResource::Encounter));

    let (encounter_system, class_code, class_system) = if let Some(res_map) = encounter_mapping {
        let id_rule = res_map
            .mappings
            .iter()
            .find(|m| m.source_column.as_deref() == Some("id_consulta"));
        let sys = id_rule
            .and_then(|m| m.system.as_deref())
            .unwrap_or("https://distritosalud1.chiapas.gob.mx/encounters");

        let class_rule = res_map
            .mappings
            .iter()
            .find(|m| m.constant_value.as_deref() == Some("AMB"));
        let c_code = class_rule
            .and_then(|m| m.constant_value.as_deref())
            .unwrap_or("AMB");
        let c_sys = class_rule
            .and_then(|m| m.system.as_deref())
            .unwrap_or("http://terminology.hl7.org/CodeSystem/v3-ActCode");

        (sys, c_code, c_sys)
    } else {
        (
            "https://distritosalud1.chiapas.gob.mx/encounters",
            "AMB",
            "http://terminology.hl7.org/CodeSystem/v3-ActCode",
        )
    };

    // Identificador de la consulta
    let identifier = fhir_identifier(
        Some(encounter_system),
        &consulta.id_consulta.to_string(),
        Some("official"),
    );

    let status_rule = encounter_mapping.and_then(|res| {
        res.mappings.iter().find(|m| {
            m.source_column.as_deref() == Some("estado_consulta") || m.target_path == "status"
        })
    });

    // Mapeo del estado de la consulta a código oficial FHIR
    let status_code: helios_fhir::r4::Code = match consulta.estado_consulta.as_str() {
        "FINALIZADA" => fhir_code("finished"),
        "EN_CURSO" => fhir_code("in-progress"),
        "CANCELADA" => fhir_code("cancelled"),
        other => {
            match status_rule
                .and_then(|r| r.dictionary.as_ref())
                .and_then(|dict| dict.get(other))
            {
                Some(val) => fhir_code(val.as_str()),
                None => {
                    return Err(MedSysError::ProcessingError(format!(
                        "Data corruption: Estado de consulta desconocido '{other}'"
                    )));
                }
            }
        }
    };

    // Clasificación ambulatoria (AMB)
    let class_coding = helios_fhir::r4::Coding {
        id: None,
        extension: None,
        system: Some(fhir_uri(class_system)),
        version: None,
        code: Some(fhir_code(class_code)),
        display: Some(fhir_string("ambulatory")),
        user_selected: None,
    };

    // Período de inicio y fin de la atención médica
    let period = Period {
        id: None,
        extension: None,
        start: Some(fhir_datetime(consulta.fecha_hora_inicio)?),
        end: Some(fhir_datetime(consulta.fecha_hora_fin)?),
    };

    // Médico tratante con su Cédula Profesional SEP
    let practitioner_identifier = fhir_identifier(
        Some("http://cedulaprofesional.sep.gob.mx"),
        &consulta.cedula_medico_tratante,
        Some("official"),
    );

    let practitioner_ref = Reference {
        id: None,
        extension: None,
        reference: None,
        r#type: Some(fhir_uri("Practitioner")),
        identifier: Some(Box::new(practitioner_identifier)),
        display: Some(fhir_string(consulta.nombre_medico.clone())),
    };

    let participant = EncounterParticipant {
        id: None,
        extension: None,
        modifier_extension: None,
        r#type: Some(vec![fhir_concept(
            Some("http://terminology.hl7.org/CodeSystem/v3-ParticipationType"),
            Some("ATND"),
            Some("attender"),
            Some("Médico Tratante"),
        )]),
        period: None,
        individual: Some(practitioner_ref),
    };

    // Referencia al paciente sujeto de la consulta
    let subject_ref = fhir_reference(&format!("Patient/{}", consulta.id_paciente), None);

    // Motivo de consulta
    let reason_code = Some(vec![fhir_concept(
        None,
        None,
        None,
        Some(&consulta.motivo_consulta),
    )]);

    // Ubicación o unidad médica (e.g. CESSA Tuxtla Poniente)
    let service_type = consulta
        .unidad_medica
        .as_ref()
        .map(|u| fhir_concept(None, None, None, Some(u)));

    Ok(Encounter {
        id: Some(fhir_string(consulta.id_consulta.to_string())),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: Some(vec![identifier]),
        status: status_code,
        status_history: None,
        class: class_coding,
        class_history: None,
        r#type: None,
        service_type,
        priority: None,
        subject: Some(subject_ref),
        episode_of_care: None,
        based_on: None,
        participant: Some(vec![participant]),
        appointment: None,
        period: Some(period),
        length: None,
        reason_code,
        reason_reference: None,
        diagnosis: None,
        account: None,
        hospitalization: None,
        location: None,
        service_provider: None,
        part_of: None,
    })
}

/// Transformador canónico de `tbl_signos_vitales` hacia el recurso oficial `Observation` de HL7 FHIR R4
/// para el Panel de Presión Arterial (LOINC 85354-9, componentes 8480-6 sistólica y 8462-4 diastólica).
/// Implementa la Tarea 2.3 del Sprint 2.
pub fn transform_observation_blood_pressure(
    signo: &LegacySignoVital,
    rules: Option<&MappingRules>,
) -> Result<Observation> {
    let effective_dt = fhir_datetime(signo.fecha_registro)?;

    let bp_mapping = rules.and_then(|r| {
        r.resources
            .iter()
            .find(|res| {
                res.resource_type == "Observation"
                    && res.observation_type.as_deref() == Some("blood_pressure_panel")
            })
            .or_else(|| {
                r.resources.iter().find(|res| {
                    res.resource_type == "Observation"
                        && res
                            .mappings
                            .iter()
                            .any(|m| m.source_column.as_deref() == Some("presion_sistolica"))
                })
            })
    });

    let cat_rule = bp_mapping.and_then(|rm| {
        rm.mappings
            .iter()
            .find(|m| m.target_path == "category[0].coding[0].code")
    });
    let cat_code = cat_rule
        .and_then(|r| r.constant_value.as_deref())
        .unwrap_or("vital-signs");
    let cat_system = cat_rule
        .and_then(|r| r.system.as_deref())
        .unwrap_or("http://terminology.hl7.org/CodeSystem/observation-category");

    let category = vec![fhir_concept(
        Some(cat_system),
        Some(cat_code),
        Some("Vital Signs"),
        None,
    )];

    let panel_code_rule = bp_mapping.and_then(|rm| {
        rm.mappings
            .iter()
            .find(|m| m.target_path == "code.coding[0].code")
    });
    let panel_code = panel_code_rule
        .and_then(|r| r.constant_value.as_deref())
        .unwrap_or("85354-9");
    let panel_system = panel_code_rule
        .and_then(|r| r.system.as_deref())
        .unwrap_or("http://loinc.org");
    let panel_display = panel_code_rule
        .and_then(|r| r.display.as_deref())
        .unwrap_or("Blood pressure panel with all children optional");

    let code = fhir_concept(
        Some(panel_system),
        Some(panel_code),
        Some(panel_display),
        Some("Presión arterial"),
    );

    // Componente Presión Sistólica (LOINC 8480-6)
    let sist_code_rule = bp_mapping.and_then(|rm| {
        rm.mappings
            .iter()
            .find(|m| m.target_path == "component[0].code.coding[0].code")
    });
    let sist_code = sist_code_rule
        .and_then(|r| r.constant_value.as_deref())
        .unwrap_or("8480-6");
    let sist_code_system = sist_code_rule
        .and_then(|r| r.system.as_deref())
        .unwrap_or("http://loinc.org");
    let sist_code_display = sist_code_rule
        .and_then(|r| r.display.as_deref())
        .unwrap_or("Systolic blood pressure");

    let sist_qty_rule = bp_mapping.and_then(|rm| {
        rm.mappings.iter().find(|m| {
            m.target_path == "component[0].valueQuantity.value"
                || m.source_column.as_deref() == Some("presion_sistolica")
        })
    });
    let sist_unit = sist_qty_rule
        .and_then(|r| r.unit.as_deref())
        .unwrap_or("mmHg");
    let sist_ucum_code = sist_qty_rule
        .and_then(|r| r.code.as_deref())
        .unwrap_or("mm[Hg]");
    let sist_ucum_system = sist_qty_rule
        .and_then(|r| r.system.as_deref())
        .unwrap_or("http://unitsofmeasure.org");

    let comp_sistolica = ObservationComponent {
        id: None,
        extension: None,
        modifier_extension: None,
        code: fhir_concept(
            Some(sist_code_system),
            Some(sist_code),
            Some(sist_code_display),
            None,
        ),
        value: Some(ObservationComponentValue::Quantity(Quantity {
            id: None,
            extension: None,
            value: Some(fhir_decimal(RustDecimal::from(signo.presion_sistolica))),
            comparator: None,
            unit: Some(fhir_string(sist_unit)),
            system: Some(fhir_uri(sist_ucum_system)),
            code: Some(fhir_code(sist_ucum_code)),
        })),
        data_absent_reason: None,
        interpretation: None,
        reference_range: None,
    };

    // Componente Presión Diastólica (LOINC 8462-4)
    let diast_code_rule = bp_mapping.and_then(|rm| {
        rm.mappings
            .iter()
            .find(|m| m.target_path == "component[1].code.coding[0].code")
    });
    let diast_code = diast_code_rule
        .and_then(|r| r.constant_value.as_deref())
        .unwrap_or("8462-4");
    let diast_code_system = diast_code_rule
        .and_then(|r| r.system.as_deref())
        .unwrap_or("http://loinc.org");
    let diast_code_display = diast_code_rule
        .and_then(|r| r.display.as_deref())
        .unwrap_or("Diastolic blood pressure");

    let diast_qty_rule = bp_mapping.and_then(|rm| {
        rm.mappings.iter().find(|m| {
            m.target_path == "component[1].valueQuantity.value"
                || m.source_column.as_deref() == Some("presion_diastolica")
        })
    });
    let diast_unit = diast_qty_rule
        .and_then(|r| r.unit.as_deref())
        .unwrap_or("mmHg");
    let diast_ucum_code = diast_qty_rule
        .and_then(|r| r.code.as_deref())
        .unwrap_or("mm[Hg]");
    let diast_ucum_system = diast_qty_rule
        .and_then(|r| r.system.as_deref())
        .unwrap_or("http://unitsofmeasure.org");

    let comp_diastolica = ObservationComponent {
        id: None,
        extension: None,
        modifier_extension: None,
        code: fhir_concept(
            Some(diast_code_system),
            Some(diast_code),
            Some(diast_code_display),
            None,
        ),
        value: Some(ObservationComponentValue::Quantity(Quantity {
            id: None,
            extension: None,
            value: Some(fhir_decimal(RustDecimal::from(signo.presion_diastolica))),
            comparator: None,
            unit: Some(fhir_string(diast_unit)),
            system: Some(fhir_uri(diast_ucum_system)),
            code: Some(fhir_code(diast_ucum_code)),
        })),
        data_absent_reason: None,
        interpretation: None,
        reference_range: None,
    };

    Ok(Observation {
        id: Some(fhir_string(format!("bp-{}", signo.id_signo))),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        based_on: None,
        part_of: None,
        status: fhir_code("final"),
        category: Some(category),
        code,
        subject: Some(fhir_reference(
            &format!("Patient/{}", signo.id_paciente),
            None,
        )),
        focus: None,
        encounter: Some(fhir_reference(
            &format!("Encounter/{}", signo.id_consulta),
            None,
        )),
        effective: Some(ObservationEffective::DateTime(effective_dt)),
        issued: None,
        performer: None,
        value: None,
        data_absent_reason: None,
        interpretation: None,
        note: None,
        body_site: None,
        method: None,
        specimen: None,
        device: None,
        reference_range: None,
        has_member: None,
        derived_from: None,
        component: Some(vec![comp_sistolica, comp_diastolica]),
    })
}

/// Transformador canónico de `tbl_signos_vitales` hacia el recurso oficial `Observation` de HL7 FHIR R4
/// para la medición de Temperatura Corporal (LOINC 8310-5, unidad Cel).
/// Implementa la Tarea 2.3 del Sprint 2.
pub fn transform_observation_temperature(
    signo: &LegacySignoVital,
    _rules: Option<&MappingRules>,
) -> Result<Observation> {
    let effective_dt = fhir_datetime(signo.fecha_registro)?;

    let category = vec![fhir_concept(
        Some("http://terminology.hl7.org/CodeSystem/observation-category"),
        Some("vital-signs"),
        Some("Vital Signs"),
        None,
    )];

    let code = fhir_concept(
        Some("http://loinc.org"),
        Some("8310-5"),
        Some("Body temperature"),
        Some("Temperatura Corporal"),
    );

    let value = ObservationValue::Quantity(Quantity {
        id: None,
        extension: None,
        value: Some(fhir_decimal(signo.temperatura_celsius)),
        comparator: None,
        unit: Some(fhir_string("Cel")),
        system: Some(fhir_uri("http://unitsofmeasure.org")),
        code: Some(fhir_code("Cel")),
    });

    Ok(Observation {
        id: Some(fhir_string(format!("temp-{}", signo.id_signo))),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        based_on: None,
        part_of: None,
        status: fhir_code("final"),
        category: Some(category),
        code,
        subject: Some(fhir_reference(
            &format!("Patient/{}", signo.id_paciente),
            None,
        )),
        focus: None,
        encounter: Some(fhir_reference(
            &format!("Encounter/{}", signo.id_consulta),
            None,
        )),
        effective: Some(ObservationEffective::DateTime(effective_dt)),
        issued: None,
        performer: None,
        value: Some(value),
        data_absent_reason: None,
        interpretation: None,
        note: None,
        body_site: None,
        method: None,
        specimen: None,
        device: None,
        reference_range: None,
        has_member: None,
        derived_from: None,
        component: None,
    })
}

/// Transformador canónico de `tbl_signos_vitales` hacia el recurso oficial `Observation` de HL7 FHIR R4
/// para la medición de Frecuencia Cardíaca (LOINC 8867-4, unidad /min).
pub fn transform_observation_heart_rate(
    signo: &LegacySignoVital,
    _rules: Option<&MappingRules>,
) -> Result<Observation> {
    let effective_dt = fhir_datetime(signo.fecha_registro)?;

    let category = vec![fhir_concept(
        Some("http://terminology.hl7.org/CodeSystem/observation-category"),
        Some("vital-signs"),
        Some("Vital Signs"),
        None,
    )];

    let code = fhir_concept(
        Some("http://loinc.org"),
        Some("8867-4"),
        Some("Heart rate"),
        Some("Frecuencia cardíaca"),
    );

    let value = ObservationValue::Quantity(Quantity {
        id: None,
        extension: None,
        value: Some(fhir_decimal(RustDecimal::from(signo.frecuencia_cardiaca))),
        comparator: None,
        unit: Some(fhir_string("/min")),
        system: Some(fhir_uri("http://unitsofmeasure.org")),
        code: Some(fhir_code("/min")),
    });

    Ok(Observation {
        id: Some(fhir_string(format!("hr-{}", signo.id_signo))),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        based_on: None,
        part_of: None,
        status: fhir_code("final"),
        category: Some(category),
        code,
        subject: Some(fhir_reference(
            &format!("Patient/{}", signo.id_paciente),
            None,
        )),
        focus: None,
        encounter: Some(fhir_reference(
            &format!("Encounter/{}", signo.id_consulta),
            None,
        )),
        effective: Some(ObservationEffective::DateTime(effective_dt)),
        issued: None,
        performer: None,
        value: Some(value),
        data_absent_reason: None,
        interpretation: None,
        note: None,
        body_site: None,
        method: None,
        specimen: None,
        device: None,
        reference_range: None,
        has_member: None,
        derived_from: None,
        component: None,
    })
}

/// Transformador canónico de `tbl_signos_vitales` hacia `Observation` de Frecuencia Respiratoria
/// (LOINC 9279-1, unidad /min). Campo opcional: si `frecuencia_respiratoria` es `NULL` retorna `Ok(None)`.
pub fn transform_observation_respiratory_rate(
    signo: &LegacySignoVital,
    _rules: Option<&MappingRules>,
) -> Result<Option<Observation>> {
    let rr = match signo.frecuencia_respiratoria {
        None => return Ok(None),
        Some(rr) => rr,
    };
    if rr <= 0 {
        return Err(MedSysError::ProcessingError(
            "Frecuencia respiratoria fuera de rango fisiológico positivo".into(),
        ));
    }

    let effective_dt = fhir_datetime(signo.fecha_registro)?;

    let category = vec![fhir_concept(
        Some("http://terminology.hl7.org/CodeSystem/observation-category"),
        Some("vital-signs"),
        Some("Vital Signs"),
        None,
    )];

    let code = fhir_concept(
        Some("http://loinc.org"),
        Some("9279-1"),
        Some("Respiratory rate"),
        Some("Frecuencia respiratoria"),
    );

    let value = ObservationValue::Quantity(Quantity {
        id: None,
        extension: None,
        value: Some(fhir_decimal(RustDecimal::from(rr))),
        comparator: None,
        unit: Some(fhir_string("/min")),
        system: Some(fhir_uri("http://unitsofmeasure.org")),
        code: Some(fhir_code("/min")),
    });

    Ok(Some(Observation {
        id: Some(fhir_string(format!("resp-{}", signo.id_signo))),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        based_on: None,
        part_of: None,
        status: fhir_code("final"),
        category: Some(category),
        code,
        subject: Some(fhir_reference(
            &format!("Patient/{}", signo.id_paciente),
            None,
        )),
        focus: None,
        encounter: Some(fhir_reference(
            &format!("Encounter/{}", signo.id_consulta),
            None,
        )),
        effective: Some(ObservationEffective::DateTime(effective_dt)),
        issued: None,
        performer: None,
        value: Some(value),
        data_absent_reason: None,
        interpretation: None,
        note: None,
        body_site: None,
        method: None,
        specimen: None,
        device: None,
        reference_range: None,
        has_member: None,
        derived_from: None,
        component: None,
    }))
}

/// Transformador canónico de `tbl_diagnosticos` hacia el recurso oficial `Condition` de HL7 FHIR R4
/// codificado bajo el catálogo internacional CIE-10.
/// Implementa la Tarea 2.4 del Sprint 2 con degradación elegante.
pub fn transform_condition(
    diagnostico: &LegacyDiagnostico,
    rules: Option<&MappingRules>,
) -> Result<Condition> {
    let condition_mapping =
        rules.and_then(|r| r.get_resource_mapping(SupportedResource::Condition));
    let ver_rule = condition_mapping.and_then(|res| {
        res.mappings.iter().find(|m| {
            m.source_column.as_deref() == Some("tipo_diagnostico")
                || m.target_path.contains("verificationStatus")
        })
    });

    // Estado clínico activo
    let clinical_status = fhir_concept(
        Some("http://terminology.hl7.org/CodeSystem/condition-clinical"),
        Some("active"),
        Some("Active"),
        None,
    );

    // verificationStatus (0..1): CONFIRMADO -> confirmed, PRESUNTIVO -> provisional.
    // NULL/vacío => se omite (optional: true). Valor fuera de catálogo => Fail-Closed (T3).
    let verification_status = match diagnostico.tipo_diagnostico.as_deref().map(str::trim) {
        None | Some("") => None,
        Some(raw) => {
            let mapped: Option<&str> = match ver_rule.and_then(|r| r.dictionary.as_ref()) {
                Some(dict) => dict.get(raw).map(|s| s.as_str()),
                None => match raw {
                    "CONFIRMADO" => Some("confirmed"),
                    "PRESUNTIVO" => Some("provisional"),
                    _ => None,
                },
            };
            let ver_code = mapped.ok_or_else(|| {
                MedSysError::ProcessingError(format!(
                    "T3 Código fuera de catálogo en tipo_diagnostico: '{raw}'"
                ))
            })?;
            let ver_display = match ver_code {
                "confirmed" => "Confirmed",
                "provisional" => "Provisional",
                other => other,
            };
            Some(fhir_concept(
                Some("http://terminology.hl7.org/CodeSystem/condition-ver-status"),
                Some(ver_code),
                Some(ver_display),
                None,
            ))
        }
    };

    // Código internacional CIE-10 oficial en México (NOM-004-SSA3-2012)
    let code = fhir_concept(
        Some("http://hl7.org/fhir/sid/icd-10"),
        Some(&diagnostico.codigo_cie10),
        Some(&diagnostico.descripcion_diagnostico),
        Some(&diagnostico.descripcion_diagnostico),
    );

    let recorded_date = fhir_datetime(
        diagnostico
            .fecha_diagnostico
            .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap_or_default()),
    )?;

    Ok(Condition {
        id: Some(fhir_string(diagnostico.id_diagnostico.to_string())),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        clinical_status: Some(clinical_status),
        verification_status,
        category: Some(vec![fhir_concept(
            Some("http://terminology.hl7.org/CodeSystem/condition-category"),
            Some("encounter-diagnosis"),
            Some("Encounter Diagnosis"),
            None,
        )]),
        severity: None,
        code: Some(code),
        body_site: None,
        subject: fhir_reference(&format!("Patient/{}", diagnostico.id_paciente), None),
        encounter: Some(fhir_reference(
            &format!("Encounter/{}", diagnostico.id_consulta),
            None,
        )),
        onset: None,
        abatement: None,
        recorded_date: Some(recorded_date),
        recorder: None,
        asserter: None,
        stage: None,
        evidence: None,
        note: None,
    })
}

/// Transforma un diagnóstico clínico desde una representación relacional cruda con soporte de degradación elegante para fechas malformadas.
#[allow(clippy::too_many_arguments)]
pub fn transform_condition_raw(
    id_diagnostico: i32,
    id_consulta: i32,
    id_paciente: i32,
    codigo_cie10: &str,
    descripcion_diagnostico: &str,
    tipo_diagnostico: &str,
    fecha_diagnostico_raw: &str,
    rules: Option<&MappingRules>,
) -> Result<Condition> {
    let parsed_date_str = parse_date_iso8601(fecha_diagnostico_raw).map_err(|err| {
        MedSysError::ProcessingError(format!(
            "Data corruption: Fallo al parsear fecha_diagnostico '{fecha_diagnostico_raw}': {err}"
        ))
    })?;
    let parsed_date = chrono::NaiveDate::parse_from_str(&parsed_date_str, "%Y-%m-%d").map_err(|e| {
        MedSysError::ProcessingError(format!(
            "Data corruption: Fallo al interpretar fecha de diagnóstico resultante '{parsed_date_str}': {e}"
        ))
    })?;

    let diag = LegacyDiagnostico {
        id_diagnostico,
        id_consulta,
        id_paciente,
        codigo_cie10: codigo_cie10.to_string(),
        descripcion_diagnostico: descripcion_diagnostico.to_string(),
        tipo_diagnostico: if tipo_diagnostico.trim().is_empty() {
            None
        } else {
            Some(tipo_diagnostico.to_string())
        },
        fecha_diagnostico: parsed_date,
    };

    transform_condition(&diag, rules)
}

/// Serializa cualquier recurso FHIR hacia una cadena JSON canónica `application/fhir+json`.
pub fn serialize_to_fhir_json<T: serde::Serialize>(resource: &T) -> Result<String> {
    serde_json::to_string_pretty(resource).map_err(MedSysError::from)
}

/// Construye un recurso canónico `OperationOutcome` de HL7 FHIR R4 para reporte técnico de excepciones.
pub fn create_operation_outcome(
    id: Option<&str>,
    severity: &str,
    code: &str,
    diagnostics: &str,
) -> OperationOutcome {
    let issue_details = fhir_concept(
        Some("http://hl7.org/fhir/issue-type"),
        Some(code),
        None,
        Some(diagnostics),
    );

    let issue = OperationOutcomeIssue {
        id: None,
        extension: None,
        modifier_extension: None,
        severity: fhir_code(severity),
        code: fhir_code(code),
        details: Some(issue_details),
        diagnostics: Some(fhir_string(diagnostics)),
        location: None,
        expression: None,
    };

    OperationOutcome {
        id: id.map(fhir_string),
        meta: None,
        implicit_rules: None,
        language: Some(fhir_code("es")),
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        issue: Some(vec![issue]),
    }
}

/// Construye un recurso `Bundle` de tipo `searchset` para responder a consultas de colección en FHIR R4.
pub fn create_searchset_bundle(
    id: Option<&str>,
    entries: Vec<Resource>,
    total: Option<usize>,
) -> Bundle {
    let bundle_entries = if entries.is_empty() {
        None
    } else {
        Some(
            entries
                .into_iter()
                .map(|resource| BundleEntry {
                    id: None,
                    extension: None,
                    modifier_extension: None,
                    link: None,
                    full_url: None,
                    resource: Some(resource),
                    search: None,
                    request: None,
                    response: None,
                })
                .collect(),
        )
    };

    let total_element = total.map(|t| Element {
        id: None,
        extension: None,
        value: Some(t as i32),
    });

    Bundle {
        id: id.map(fhir_string),
        meta: None,
        implicit_rules: None,
        language: None,
        identifier: None,
        r#type: fhir_code("searchset"),
        timestamp: None,
        total: total_element,
        link: None,
        entry: bundle_entries,
        signature: None,
    }
}
