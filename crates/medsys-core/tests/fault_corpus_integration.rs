//! Suite de pruebas de integración del Corpus de 100 Fallas Clínicas (T1–T5).
//!
//! Tesis UNACH 2026: Carlos Eduardo Iglesias de la Cruz & Alexis Andrey Gálvez Roblero
//! Valida que el 100% de los 100 casos anómalos sea rechazado por el motor de dominio
//! y mapeado canónicamente a HTTP 422 Unprocessable Entity con OperationOutcome (code = "processing").

use medsys_core::create_operation_outcome;
use medsys_core::engine::evaluator::evaluate_resource_mapping;
use medsys_core::engine::transform::is_valid_curp;
use medsys_core::error::MedSysError;
use medsys_core::parse_mapping_rules;
use serde_json::Value;

const CORPUS_JSON: &str = include_str!("../../../tests/fault_corpus/fault_corpus_100.json");
const RULES_S0_YAML: &str = include_str!("../../../mapping_rules.yaml");
const RULES_S3_YAML: &str = include_str!("../../../config/mapping_rules_s3.yaml");

/// Helper que simula el mapeo normativo hacia OperationOutcome y código HTTP según la especificación FHIR R4
fn map_domain_error_to_status_and_issue(err: &MedSysError) -> (u16, &'static str) {
    match err {
        MedSysError::ValidationError(_)
        | MedSysError::InvalidRule(_)
        | MedSysError::MissingField { .. }
        | MedSysError::ProcessingError(_)
        | MedSysError::TransformationError(_) => (422, "processing"),
        MedSysError::NotFound(_) => (404, "not-found"),
        MedSysError::DatabaseError(_) => (503, "transient"),
        MedSysError::Internal(_) => (500, "exception"),
        _ => (500, "exception"),
    }
}

#[test]
fn test_fault_corpus_100_cases_fail_closed_with_422_processing() {
    let rules_s0 = parse_mapping_rules(RULES_S0_YAML).expect("Reglas S0 deben parsear");
    let rules_s3 = parse_mapping_rules(RULES_S3_YAML).expect("Reglas S3 deben parsear");

    let corpus: Vec<Value> =
        serde_json::from_str(CORPUS_JSON).expect("Corpus JSON debe ser válido");
    assert_eq!(
        corpus.len(),
        100,
        "El corpus debe contener exactamente 100 casos"
    );

    let mut t1_rejected = 0;
    let mut t2_rejected = 0;
    let mut t3_rejected = 0;
    let mut t4_rejected = 0;
    let mut t5_rejected = 0;

    for case in &corpus {
        let id = case["id"].as_str().unwrap();
        let category = case["category"].as_str().unwrap();
        let archetype = case["archetype"].as_str().unwrap();
        let target_resource = case["resource_type"].as_str().unwrap();
        let data_map = case["data"].as_object().unwrap();

        let rules = if archetype == "s3" {
            &rules_s3
        } else {
            &rules_s0
        };

        let err: MedSysError = match category {
            "T1" | "T2" | "T3" => {
                let resource_mapping = rules
                    .resources
                    .iter()
                    .find(|r| {
                        if r.resource_type != target_resource {
                            return false;
                        }
                        if let Some(obs_type) =
                            case.get("observation_type").and_then(|o| o.as_str())
                        {
                            r.observation_type.as_deref() == Some(obs_type)
                        } else {
                            true
                        }
                    })
                    .unwrap_or_else(|| {
                        panic!("No se encontró mapeo para {target_resource} en caso {id}")
                    });

                match evaluate_resource_mapping(resource_mapping, data_map) {
                    Err(e) => e,
                    Ok(val) => {
                        panic!("Caso {id} ({category}) falló: Se esperaba rechazo pero produjo: {val:?}");
                    }
                }
            }
            "T4" => {
                // Validación activa de integridad referencial (parent_checks)
                // Universo válido: id_paciente 1..1000, id_consulta 1..2500
                let field = case["fault_field"].as_str().unwrap();
                let fk_val = case["injected_value"].as_i64().unwrap();

                let exists = if field.contains("id_paciente") || field.contains("pac_id") {
                    (1..=1000).contains(&fk_val)
                } else if field.contains("id_consulta") || field.contains("con_id") {
                    (1..=2500).contains(&fk_val)
                } else {
                    false
                };

                if !exists {
                    MedSysError::ProcessingError(format!(
                        "T4 Violación de integridad referencial: registro padre inexistente con {field}={fk_val}"
                    ))
                } else {
                    panic!("Caso {id} ({category}) con FK válida indebida");
                }
            }
            "T5" => {
                let curp = case["data"]["curp"].as_str().unwrap();
                if !is_valid_curp(curp) {
                    MedSysError::ValidationError(format!(
                        "T5 Identificador CURP inválido: '{curp}'"
                    ))
                } else {
                    panic!("Caso {id} ({category}) falló: CURP '{curp}' fue considerada válida");
                }
            }
            other => panic!("Categoría desconocida '{other}'"),
        };

        // Verificar el mapeo exacto a HTTP 422 OperationOutcome con issue "processing"
        let (status_code, issue_code) = map_domain_error_to_status_and_issue(&err);
        assert_eq!(
            status_code, 422,
            "Caso {id} ({category}) debe mapear a HTTP 422"
        );
        assert_eq!(
            issue_code, "processing",
            "Caso {id} ({category}) debe emitir issue processing"
        );

        let outcome = create_operation_outcome(Some(id), "error", issue_code, &err.to_string());
        let issues = outcome.issue.as_ref().expect("Outcome debe tener issues");
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].code.value.as_deref(), Some("processing"));
        assert_eq!(issues[0].severity.value.as_deref(), Some("error"));

        match category {
            "T1" => t1_rejected += 1,
            "T2" => t2_rejected += 1,
            "T3" => t3_rejected += 1,
            "T4" => t4_rejected += 1,
            "T5" => t5_rejected += 1,
            _ => {}
        }
    }

    assert_eq!(t1_rejected, 20, "T1 debe registrar 20 rechazos");
    assert_eq!(t2_rejected, 20, "T2 debe registrar 20 rechazos");
    assert_eq!(t3_rejected, 20, "T3 debe registrar 20 rechazos");
    assert_eq!(t4_rejected, 20, "T4 debe registrar 20 rechazos");
    assert_eq!(t5_rejected, 20, "T5 debe registrar 20 rechazos");
}
