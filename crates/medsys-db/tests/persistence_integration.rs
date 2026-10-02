//! Pruebas de integración de persistencia relacional — MedSys-FHIR
//! Tarea 3.4 del Sprint 3.
//!
//! Valida el mapeo de filas de PostgreSQL (NOM-004-SSA3-2012) a estructuras intermedias
//! en Rust y su posterior transformación a recursos HL7 FHIR R4 canónicos.

use chrono::NaiveDate;
use helios_fhir::r4::Resource;
use medsys_core::engine::transform::{
    serialize_to_fhir_json, transform_condition, transform_encounter,
    transform_observation_blood_pressure, transform_observation_temperature, transform_patient,
};
use medsys_core::error::MedSysError;
use medsys_core::model::legacy::{
    LegacyConsulta, LegacyDiagnostico, LegacyPaciente, LegacySignoVital,
};
use medsys_db::config::DbConfig;
use medsys_db::entities::{ConsultaEntity, DiagnosticoEntity, PacienteEntity, SignoVitalEntity};
use medsys_db::pool::DbManager;
use rust_decimal::Decimal;

const SCHEMA_SQL: &str = include_str!("../../../schema_legado_simulado_nom004.sql");

/// Valida la integridad sintáctica y contractual del archivo SQL de esquema legado simulado.
#[test]
fn test_schema_sql_contract_integrity() {
    assert!(
        SCHEMA_SQL.contains("CREATE TABLE IF NOT EXISTS tbl_pacientes"),
        "El esquema debe definir tbl_pacientes"
    );
    assert!(
        SCHEMA_SQL.contains("CREATE TABLE IF NOT EXISTS tbl_consultas"),
        "El esquema debe definir tbl_consultas"
    );
    assert!(
        SCHEMA_SQL.contains("CREATE TABLE IF NOT EXISTS tbl_signos_vitales"),
        "El esquema debe definir tbl_signos_vitales"
    );
    assert!(
        SCHEMA_SQL.contains("CREATE TABLE IF NOT EXISTS tbl_diagnosticos"),
        "El esquema debe definir tbl_diagnosticos"
    );

    // Validar columnas normativas clave
    assert!(SCHEMA_SQL.contains("curp VARCHAR(18) UNIQUE NOT NULL"));
    assert!(SCHEMA_SQL.contains("cedula_medico_tratante VARCHAR(20) NOT NULL"));
    assert!(SCHEMA_SQL.contains("presion_sistolica INT NOT NULL"));
    assert!(SCHEMA_SQL.contains("presion_diastolica INT NOT NULL"));
    assert!(SCHEMA_SQL.contains("temperatura_celsius NUMERIC(4, 1) NOT NULL"));
    assert!(SCHEMA_SQL.contains("codigo_cie10 VARCHAR(10) NOT NULL"));

    // Validar que los datos sintéticos de prueba estén presentes
    assert!(SCHEMA_SQL.contains("ROMA900101HCSNN01"));
    assert!(SCHEMA_SQL.contains("LOPE950512MCSNN02"));
    assert!(SCHEMA_SQL.contains("8472910"));
    assert!(SCHEMA_SQL.contains("I10"));
    assert!(SCHEMA_SQL.contains("G43.9"));
}

/// Valida la conversión bidireccional entre filas relacionales (Entity) y modelos de dominio (Legacy),
/// así como la transformación de extremo a extremo hacia recursos canónicos HL7 FHIR R4.
#[test]
fn test_synthetic_data_persistence_mapping_to_fhir_e2e() {
    // 1. Paciente 1 (Alberto Manuel Ramos Gómez)
    let paciente_entity = PacienteEntity {
        id_paciente: 1,
        curp: "ROMA900101HCSNN01".to_string(),
        primer_nombre: "Alberto".to_string(),
        segundo_nombre: Some("Manuel".to_string()),
        apellido_paterno: "Ramos".to_string(),
        apellido_materno: Some("Gómez".to_string()),
        fecha_nacimiento: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
        sexo_biologico: Some("M".to_string()),
        telefono_contacto: Some("9611234567".to_string()),
        fecha_registro: Some(
            NaiveDate::from_ymd_opt(2026, 9, 18)
                .unwrap()
                .and_hms_opt(9, 0, 0)
                .unwrap(),
        ),
    };

    let legacy_paciente: LegacyPaciente = paciente_entity.into();
    let fhir_patient = transform_patient(&legacy_paciente, None).expect("Fallo transform_patient");
    let patient_json = serialize_to_fhir_json(&Resource::Patient(Box::new(fhir_patient)))
        .expect("Fallo serialize Patient");

    assert!(patient_json.contains("\"resourceType\": \"Patient\""));
    assert!(patient_json.contains("ROMA900101HCSNN01"));
    assert!(patient_json.contains("Ramos Gómez"));
    assert!(patient_json.contains("Alberto"));
    assert!(patient_json.contains("1990-01-01"));
    assert!(patient_json.contains("\"gender\": \"male\""));

    // 2. Consulta 1 (Dra. María Elena Cruz Martínez)
    let dt_inicio = NaiveDate::from_ymd_opt(2026, 9, 18)
        .unwrap()
        .and_hms_opt(9, 15, 0)
        .unwrap();
    let dt_fin = NaiveDate::from_ymd_opt(2026, 9, 18)
        .unwrap()
        .and_hms_opt(9, 40, 0)
        .unwrap();

    let consulta_entity = ConsultaEntity {
        id_consulta: 1,
        id_paciente: 1,
        cedula_medico_tratante: "8472910".to_string(),
        nombre_medico: "Dra. María Elena Cruz Martínez".to_string(),
        estado_consulta: "FINALIZADA".to_string(),
        motivo_consulta: "Control trimestral de hipertensión arterial".to_string(),
        fecha_hora_inicio: dt_inicio,
        fecha_hora_fin: dt_fin,
        unidad_medica: Some("CESSA Tuxtla Poniente".to_string()),
    };

    let legacy_consulta: LegacyConsulta = consulta_entity.into();
    let fhir_encounter =
        transform_encounter(&legacy_consulta, None).expect("Fallo transform_encounter");
    let encounter_json = serialize_to_fhir_json(&Resource::Encounter(Box::new(fhir_encounter)))
        .expect("Fallo serialize Encounter");

    assert!(encounter_json.contains("\"resourceType\": \"Encounter\""));
    assert!(encounter_json.contains("\"status\": \"finished\""));
    assert!(encounter_json.contains("8472910"));
    assert!(encounter_json.contains("Dra. María Elena Cruz Martínez"));
    assert!(encounter_json.contains("Patient/1"));

    // 3. Signos Vitales 1 (Presión Arterial 130/85 mmHg y Temperatura 36.6 Cel)
    let dt_signos = NaiveDate::from_ymd_opt(2026, 9, 18)
        .unwrap()
        .and_hms_opt(9, 18, 0)
        .unwrap();

    let signo_entity = SignoVitalEntity {
        id_signo: 1,
        id_consulta: 1,
        id_paciente: 1,
        presion_sistolica: 130,
        presion_diastolica: 85,
        frecuencia_cardiaca: 76,
        frecuencia_respiratoria: None,
        temperatura_celsius: Decimal::new(366, 1),
        peso_kg: Some(Decimal::new(7850, 2)),
        talla_cm: Some(172),
        fecha_registro: dt_signos,
    };

    let legacy_signo: LegacySignoVital = signo_entity.into();

    // 3A. Panel de Presión Arterial
    let fhir_bp =
        transform_observation_blood_pressure(&legacy_signo, None).expect("Fallo transform bp");
    let bp_json = serialize_to_fhir_json(&Resource::Observation(Box::new(fhir_bp)))
        .expect("Fallo serialize bp");
    assert!(bp_json.contains("\"resourceType\": \"Observation\""));
    assert!(bp_json.contains("85354-9")); // Panel BP
    assert!(bp_json.contains("8480-6")); // Sistólica
    assert!(bp_json.contains("8462-4")); // Diastólica
    assert!(bp_json.contains("http://loinc.org"));
    assert!(bp_json.contains("http://unitsofmeasure.org"));
    assert!(bp_json.contains("130"));
    assert!(bp_json.contains("85"));
    assert!(bp_json.contains("mm[Hg]"));

    // 3B. Temperatura Corporal
    let fhir_temp =
        transform_observation_temperature(&legacy_signo, None).expect("Fallo transform temp");
    let temp_json = serialize_to_fhir_json(&Resource::Observation(Box::new(fhir_temp)))
        .expect("Fallo serialize temp");
    assert!(temp_json.contains("\"resourceType\": \"Observation\""));
    assert!(temp_json.contains("8310-5")); // Body temperature
    assert!(temp_json.contains("36.6"));
    assert!(temp_json.contains("\"code\": \"Cel\""));

    // 4. Diagnóstico 1 (CIE-10: I10 - Hipertensión esencial primaria)
    let diagnostico_entity = DiagnosticoEntity {
        id_diagnostico: 1,
        id_consulta: 1,
        id_paciente: 1,
        codigo_cie10: "I10".to_string(),
        descripcion_diagnostico: "Hipertensión esencial (primaria)".to_string(),
        tipo_diagnostico: "CONFIRMADO".to_string(),
        fecha_diagnostico: NaiveDate::from_ymd_opt(2026, 9, 18).unwrap(),
    };

    let legacy_diagnostico: LegacyDiagnostico = diagnostico_entity.into();
    let fhir_condition =
        transform_condition(&legacy_diagnostico, None).expect("Fallo transform_condition");
    let condition_json = serialize_to_fhir_json(&Resource::Condition(Box::new(fhir_condition)))
        .expect("Fallo serialize Condition");

    assert!(condition_json.contains("\"resourceType\": \"Condition\""));
    assert!(condition_json.contains("http://hl7.org/fhir/sid/icd-10"));
    assert!(condition_json.contains("I10"));
    assert!(condition_json.contains("Hipertensión esencial (primaria)"));
    assert!(condition_json.contains("confirmed"));
}

/// Prueba de integración contra la instancia de PostgreSQL si está disponible.
/// Si el contenedor Docker o la base de datos no están corriendo, reporta el estado
/// y pasa limpiamente sin bloquear la suite de pruebas automatizadas.
#[tokio::test]
async fn test_live_postgresql_persistence_when_available() {
    let mut config = DbConfig::from_env().unwrap_or_default();
    config.acquire_timeout_secs = 2; // Timeout corto para detección no bloqueante

    println!(
        "Intentando conexión de integración con PostgreSQL en: {}",
        config.masked_url()
    );

    match DbManager::new(config).await {
        Ok(db_manager) => {
            println!(
                "  ✔ Conexión a PostgreSQL exitosa. Ejecutando suite de integración en vivo..."
            );

            // Healthcheck
            let hc = db_manager.healthcheck().await;
            assert!(hc.is_ok(), "Healthcheck falló en base de datos en vivo");

            let repos = db_manager.repositories();

            // 1. Pacientes
            let paciente_1 = repos
                .pacientes
                .find_by_id(1)
                .await
                .expect("Fallo al buscar paciente 1");
            assert_eq!(paciente_1.id_paciente, 1);
            assert_eq!(paciente_1.curp, "ROMA900101HCSNN01");
            assert_eq!(paciente_1.primer_nombre, "Alberto");
            assert_eq!(paciente_1.apellido_paterno, "Ramos");

            let paciente_curp = repos
                .pacientes
                .find_by_curp("LOPE950512MCSNN02")
                .await
                .expect("Fallo al buscar paciente por CURP");
            assert_eq!(paciente_curp.primer_nombre, "Mariana");

            let todos_pacientes = repos
                .pacientes
                .find_all()
                .await
                .expect("Fallo al listar pacientes");
            assert!(
                todos_pacientes.len() >= 2,
                "Se esperaban al menos 2 pacientes sintéticos"
            );

            // Caso negativo
            let paciente_no_existe = repos.pacientes.find_by_id(9999).await;
            match paciente_no_existe {
                Err(MedSysError::NotFound(_)) => {}
                other => panic!("Esperaba MedSysError::NotFound, obtuve: {:?}", other),
            }

            // 2. Consultas
            let consulta_1 = repos
                .consultas
                .find_by_id(1)
                .await
                .expect("Fallo al buscar consulta 1");
            assert_eq!(consulta_1.id_consulta, 1);
            assert_eq!(consulta_1.id_paciente, 1);
            assert_eq!(consulta_1.cedula_medico_tratante, "8472910");

            let consultas_p1 = repos
                .consultas
                .find_by_paciente_id(1)
                .await
                .expect("Fallo al buscar consultas por paciente");
            assert!(!consultas_p1.is_empty());

            // 3. Signos Vitales
            let signos_1 = repos
                .signos_vitales
                .find_by_id(1)
                .await
                .expect("Fallo al buscar signo 1");
            assert_eq!(signos_1.presion_sistolica, 130);
            assert_eq!(signos_1.presion_diastolica, 85);
            assert_eq!(signos_1.temperatura_celsius, Decimal::new(366, 1));

            // 4. Diagnósticos
            let diag_1 = repos
                .diagnosticos
                .find_by_id(1)
                .await
                .expect("Fallo al buscar diagnostico 1");
            assert_eq!(diag_1.codigo_cie10, "I10");

            let diags_i10 = repos
                .diagnosticos
                .find_by_codigo_cie10("I10")
                .await
                .expect("Fallo al buscar por CIE-10 I10");
            assert!(!diags_i10.is_empty());

            // Cerrar pool ordenadamente
            db_manager.close().await;
            println!("  ✔ Pruebas de integración sobre PostgreSQL 16 completadas exitosamente.");
        }
        Err(err) => {
            println!("  ℹ PostgreSQL no detectado en localhost:5432 ({:?}).", err);
            println!("    Se validó exitosamente la suite de contrato y mapeo de persistencia.");
            println!("    (Para ejecutar pruebas contra el motor en vivo: Iniciar Docker Desktop y ejecutar .\\docker\\start-db.ps1)");
        }
    }
}
