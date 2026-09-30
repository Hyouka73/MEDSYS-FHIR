use chrono::NaiveTime;
use helios_fhir::r4::{
    Bundle, BundleEntry, Condition, ContactPoint, Encounter, EncounterParticipant, HumanName,
    Observation, ObservationComponent, ObservationComponentValue, ObservationEffective,
    ObservationValue, OperationOutcome, OperationOutcomeIssue, Patient, Period, Quantity,
    Reference, Resource,
};
use helios_fhir::Element;
use rust_decimal::Decimal as RustDecimal;

use crate::error::{MedSysError, Result};
use crate::model::fhir_helpers::{
    fhir_code, fhir_concept, fhir_date, fhir_datetime, fhir_decimal, fhir_identifier,
    fhir_reference, fhir_string, fhir_uri,
};
use crate::model::legacy::{LegacyConsulta, LegacyDiagnostico, LegacyPaciente, LegacySignoVital};
use crate::model::mapping::{MappingRules, SupportedResource};

/// Transformador canónico de `tbl_pacientes` hacia el recurso oficial `Patient` de HL7 FHIR R4.
/// Implementa la Tarea 2.1 del Sprint 2.
pub fn transform_patient(
    paciente: &LegacyPaciente,
    rules: Option<&MappingRules>,
) -> Result<Patient> {
    // Si se proporcionan reglas de mapeo, verificamos parámetros clave de Patient
    let (curp_system, curp_use) = if let Some(r) = rules {
        if let Some(res_map) = r.get_resource_mapping(SupportedResource::Patient) {
            let curp_rule = res_map
                .mappings
                .iter()
                .find(|m| m.source_column.as_deref() == Some("curp"));
            let sys = curp_rule
                .and_then(|m| m.system.as_deref())
                .unwrap_or("urn:oid:2.16.840.1.113883.4.629");
            let u = curp_rule
                .and_then(|m| m.effective_use())
                .unwrap_or("official");
            (sys, u)
        } else {
            ("urn:oid:2.16.840.1.113883.4.629", "official")
        }
    } else {
        ("urn:oid:2.16.840.1.113883.4.629", "official")
    };

    // Identificador nacional oficial en México: CURP
    let identifier = fhir_identifier(Some(curp_system), &paciente.curp, Some(curp_use));

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

    // Mapeo de género clínico según catálogo normativo
    let gender_code = match paciente.sexo_biologico.as_deref() {
        Some("M") => "male",
        Some("F") => "female",
        Some("I") => "other",
        _ => "unknown",
    };

    // Medios de contacto (teléfono opcional)
    let telecom = paciente.telefono_contacto.as_ref().map(|tel| {
        vec![ContactPoint {
            id: None,
            extension: None,
            system: Some(fhir_code("phone")),
            value: Some(fhir_string(tel.clone())),
            r#use: Some(fhir_code("mobile")),
            rank: None,
            period: None,
        }]
    });

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
        gender: Some(fhir_code(gender_code)),
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
    let (encounter_system, class_code, class_system) = if let Some(r) = rules {
        if let Some(res_map) = r.get_resource_mapping(SupportedResource::Encounter) {
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
        }
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

    // Mapeo del estado de la consulta a código oficial FHIR
    let status_str = match consulta.estado_consulta.as_str() {
        "FINALIZADA" => "finished",
        "EN_CURSO" => "in-progress",
        "CANCELADA" => "cancelled",
        other => {
            return Err(MedSysError::TransformationError(format!(
                "Estado de consulta desconocido: {other}"
            )))
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
        status: fhir_code(status_str),
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
        Some("85354-9"),
        Some("Blood pressure panel with all children optional"),
        Some("Presión arterial"),
    );

    // Componente Presión Sistólica (LOINC 8480-6)
    let comp_sistolica = ObservationComponent {
        id: None,
        extension: None,
        modifier_extension: None,
        code: fhir_concept(
            Some("http://loinc.org"),
            Some("8480-6"),
            Some("Systolic blood pressure"),
            None,
        ),
        value: Some(ObservationComponentValue::Quantity(Quantity {
            id: None,
            extension: None,
            value: Some(fhir_decimal(RustDecimal::from(signo.presion_sistolica))),
            comparator: None,
            unit: Some(fhir_string("mmHg")),
            system: Some(fhir_uri("http://unitsofmeasure.org")),
            code: Some(fhir_code("mm[Hg]")),
        })),
        data_absent_reason: None,
        interpretation: None,
        reference_range: None,
    };

    // Componente Presión Diastólica (LOINC 8462-4)
    let comp_diastolica = ObservationComponent {
        id: None,
        extension: None,
        modifier_extension: None,
        code: fhir_concept(
            Some("http://loinc.org"),
            Some("8462-4"),
            Some("Diastolic blood pressure"),
            None,
        ),
        value: Some(ObservationComponentValue::Quantity(Quantity {
            id: None,
            extension: None,
            value: Some(fhir_decimal(RustDecimal::from(signo.presion_diastolica))),
            comparator: None,
            unit: Some(fhir_string("mmHg")),
            system: Some(fhir_uri("http://unitsofmeasure.org")),
            code: Some(fhir_code("mm[Hg]")),
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

/// Transformador canónico de `tbl_diagnosticos` hacia el recurso oficial `Condition` de HL7 FHIR R4
/// codificado bajo el catálogo internacional CIE-10.
/// Implementa la Tarea 2.4 del Sprint 2.
pub fn transform_condition(
    diagnostico: &LegacyDiagnostico,
    _rules: Option<&MappingRules>,
) -> Result<Condition> {
    // Estado clínico activo
    let clinical_status = fhir_concept(
        Some("http://terminology.hl7.org/CodeSystem/condition-clinical"),
        Some("active"),
        Some("Active"),
        None,
    );

    // Estado de verificación: CONFIRMADO -> confirmed, PRESUNTIVO -> provisional
    let (ver_code, ver_display) = match diagnostico.tipo_diagnostico.as_str() {
        "CONFIRMADO" => ("confirmed", "Confirmed"),
        "PRESUNTIVO" => ("provisional", "Provisional"),
        _ => ("confirmed", "Confirmed"),
    };

    let verification_status = fhir_concept(
        Some("http://terminology.hl7.org/CodeSystem/condition-ver-status"),
        Some(ver_code),
        Some(ver_display),
        None,
    );

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
        verification_status: Some(verification_status),
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
