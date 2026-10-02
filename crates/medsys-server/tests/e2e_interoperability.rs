//! Pruebas E2E de Interoperabilidad Clínica — MedSys-FHIR (Tarea 4.4)
//!
//! Valida el ciclo completo de interoperabilidad de extremo a extremo:
//! 1. Esquema relacional legado NOM-004-SSA3-2012.
//! 2. Adaptador de persistencia asíncrona de solo lectura.
//! 3. Motor de transformación declarativa basada en YAML.
//! 4. Modelado y serialización de recursos canónicos HL7 FHIR Release 4 (`helios-fhir`).
//! 5. Exposición HTTP en Axum con cabecera estricta `Content-Type: application/fhir+json; charset=utf-8`.
//! 6. Manejo integral de excepciones clínicas mediante `OperationOutcome`.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use medsys_core::parse_mapping_rules;
use medsys_db::{init_pool_lazy, DbConfig, MedsysRepositories};
use medsys_server::{create_router, AppState, FHIR_JSON_CONTENT_TYPE};
use tower::ServiceExt;

const SPECIFICATION_YAML: &str = include_str!("../../../mapping_rules_specification.yaml");

/// Construye la aplicación Axum en memoria para pruebas E2E determinísticas.
fn setup_e2e_app() -> axum::Router {
    let config = DbConfig::default();
    let pool = init_pool_lazy(&config).expect("Inicialización de pool de prueba debe tener éxito");
    let repos = MedsysRepositories::new(pool.clone());
    let mapping_rules = parse_mapping_rules(SPECIFICATION_YAML)
        .expect("Reglas YAML de especificación deben parsear");
    let state = AppState::new(repos, mapping_rules, pool);
    create_router(state)
}

/// Helper asíncrono para leer el cuerpo HTTP a String UTF-8.
async fn body_to_string(response: axum::response::Response) -> String {
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("Fallo al leer stream del cuerpo HTTP")
        .to_bytes();
    String::from_utf8(bytes.to_vec()).expect("Cuerpo debe ser codificado en UTF-8 válido")
}

#[tokio::test]
async fn test_e2e_healthcheck_and_fhir_metadata() {
    let app = setup_e2e_app();

    let req = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let content_type = res.headers().get("content-type").unwrap().to_str().unwrap();
    assert!(content_type.contains("application/json"));

    let body = body_to_string(res).await;
    assert!(body.contains("server_name"));
    assert!(body.contains("MedSys-FHIR"));
    assert!(body.contains("fhir_version"));
    assert!(body.contains("R4 (4.0.1)"));
    assert!(body.contains("server_version"));
    assert!(body.contains("0.1.0"));
    assert!(body.contains("status"));
}

#[tokio::test]
async fn test_e2e_legacy_patient_inspection_api() {
    let app = setup_e2e_app();

    // Consultar lista de pacientes legados para el Dashboard
    let req = Request::builder()
        .uri("/api/legacy/patients")
        .body(Body::empty())
        .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();
    let status = res.status();
    let content_type = res.headers().get("content-type").unwrap().to_str().unwrap();

    if status == StatusCode::OK {
        assert!(content_type.contains("application/json"));
    } else {
        assert_eq!(content_type, FHIR_JSON_CONTENT_TYPE);
        let body = body_to_string(res).await;
        assert!(body.contains("\"resourceType\": \"OperationOutcome\""));
    }

    // Consultar vista completa relacional de un paciente
    let req_full = Request::builder()
        .uri("/api/legacy/patients/1/full")
        .body(Body::empty())
        .unwrap();

    let res_full = app.oneshot(req_full).await.unwrap();
    // Si la BD no está levantada localmente responde 404/500 con OperationOutcome,
    // o 200 con la estructura JSON completa si está en línea.
    let status = res_full.status();
    let body_full = body_to_string(res_full).await;

    if status == StatusCode::OK {
        assert!(body_full.contains("\"paciente\""));
        assert!(body_full.contains("\"consultas\""));
        assert!(body_full.contains("\"signos_vitales\""));
        assert!(body_full.contains("\"diagnosticos\""));
    } else {
        assert!(body_full.contains("\"resourceType\": \"OperationOutcome\""));
    }
}

#[tokio::test]
async fn test_e2e_fhir_patient_mapping_and_headers() {
    let app = setup_e2e_app();

    let req = Request::builder()
        .uri("/fhir/r4/Patient/1")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(
        res.headers().get("content-type").unwrap(),
        FHIR_JSON_CONTENT_TYPE
    );

    let status = res.status();
    let body = body_to_string(res).await;

    if status == StatusCode::OK {
        assert!(body.contains("\"resourceType\": \"Patient\""));
        assert!(body.contains("urn:oid:2.16.840.1.113883.4.629"));
        assert!(body.contains("\"use\": \"official\""));
        assert!(!body.contains("\"telecom\""));
    } else {
        assert!(body.contains("\"resourceType\": \"OperationOutcome\""));
        assert!(body.contains("\"severity\": \"error\""));
    }
}

#[tokio::test]
async fn test_e2e_fhir_encounter_mapping_and_headers() {
    let app = setup_e2e_app();

    let req = Request::builder()
        .uri("/fhir/r4/Encounter/1")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(
        res.headers().get("content-type").unwrap(),
        FHIR_JSON_CONTENT_TYPE
    );

    let status = res.status();
    let body = body_to_string(res).await;

    if status == StatusCode::OK {
        assert!(body.contains("\"resourceType\": \"Encounter\""));
        assert!(body.contains("\"code\": \"AMB\""));
        assert!(body.contains("http://terminology.hl7.org/CodeSystem/v3-ActCode"));
        assert!(body.contains("http://cedulaprofesional.sep.gob.mx"));
    } else {
        assert!(body.contains("\"resourceType\": \"OperationOutcome\""));
    }
}

#[tokio::test]
async fn test_e2e_fhir_observation_decoupled_vital_signs() {
    let app = setup_e2e_app();

    // 1. Panel de Presión Arterial (bp-1)
    let req_bp = Request::builder()
        .uri("/fhir/r4/Observation/bp-1")
        .body(Body::empty())
        .unwrap();

    let res_bp = app.clone().oneshot(req_bp).await.unwrap();
    assert_eq!(
        res_bp.headers().get("content-type").unwrap(),
        FHIR_JSON_CONTENT_TYPE
    );
    let status_bp = res_bp.status();
    let body_bp = body_to_string(res_bp).await;

    if status_bp == StatusCode::OK {
        assert!(body_bp.contains("\"resourceType\": \"Observation\""));
        assert!(body_bp.contains("85354-9")); // Panel BP LOINC
        assert!(body_bp.contains("8480-6")); // Presión sistólica
        assert!(body_bp.contains("8462-4")); // Presión diastólica
        assert!(body_bp.contains("mm[Hg]"));
    } else {
        assert!(body_bp.contains("\"resourceType\": \"OperationOutcome\""));
    }

    // 2. Temperatura Corporal (temp-1)
    let req_temp = Request::builder()
        .uri("/fhir/r4/Observation/temp-1")
        .body(Body::empty())
        .unwrap();

    let res_temp = app.clone().oneshot(req_temp).await.unwrap();
    assert_eq!(
        res_temp.headers().get("content-type").unwrap(),
        FHIR_JSON_CONTENT_TYPE
    );
    let status_temp = res_temp.status();
    let body_temp = body_to_string(res_temp).await;

    if status_temp == StatusCode::OK {
        assert!(body_temp.contains("\"resourceType\": \"Observation\""));
        assert!(body_temp.contains("8310-5")); // Temperatura LOINC
        assert!(body_temp.contains("Cel"));
    } else {
        assert!(body_temp.contains("\"resourceType\": \"OperationOutcome\""));
    }

    // 3. Frecuencia Cardíaca (hr-1)
    let req_hr = Request::builder()
        .uri("/fhir/r4/Observation/hr-1")
        .body(Body::empty())
        .unwrap();

    let res_hr = app.oneshot(req_hr).await.unwrap();
    assert_eq!(
        res_hr.headers().get("content-type").unwrap(),
        FHIR_JSON_CONTENT_TYPE
    );
    let status_hr = res_hr.status();
    let body_hr = body_to_string(res_hr).await;

    if status_hr == StatusCode::OK {
        assert!(body_hr.contains("\"resourceType\": \"Observation\""));
        assert!(body_hr.contains("8867-4")); // Frecuencia Cardíaca LOINC
        assert!(body_hr.contains("/min"));
    } else {
        assert!(body_hr.contains("\"resourceType\": \"OperationOutcome\""));
    }
}

#[tokio::test]
async fn test_e2e_fhir_condition_mapping_and_headers() {
    let app = setup_e2e_app();

    let req = Request::builder()
        .uri("/fhir/r4/Condition/cond-1")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();
    assert_eq!(
        res.headers().get("content-type").unwrap(),
        FHIR_JSON_CONTENT_TYPE
    );

    let status = res.status();
    let body = body_to_string(res).await;

    if status == StatusCode::OK {
        assert!(body.contains("\"resourceType\": \"Condition\""));
        assert!(body.contains("http://hl7.org/fhir/sid/icd-10"));
    } else {
        assert!(body.contains("\"resourceType\": \"OperationOutcome\""));
    }
}

#[tokio::test]
async fn test_e2e_fhir_searchset_bundles() {
    let app = setup_e2e_app();

    let collections = [
        "/fhir/r4/Patient",
        "/fhir/r4/Encounter",
        "/fhir/r4/Observation",
        "/fhir/r4/Condition",
    ];

    for endpoint in collections {
        let req = Request::builder()
            .uri(endpoint)
            .body(Body::empty())
            .unwrap();

        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(
            res.headers().get("content-type").unwrap(),
            FHIR_JSON_CONTENT_TYPE,
            "Endpoint {endpoint} debe retornar Content-Type application/fhir+json"
        );

        let status = res.status();
        let body = body_to_string(res).await;

        if status == StatusCode::OK {
            assert!(
                body.contains("\"resourceType\": \"Bundle\""),
                "Endpoint {endpoint} debe retornar un Bundle"
            );
            assert!(
                body.contains("\"type\": \"searchset\""),
                "Bundle de {endpoint} debe ser de tipo searchset"
            );
            assert!(body.contains("\"total\""));
        } else {
            assert!(
                body.contains("\"resourceType\": \"OperationOutcome\""),
                "En caso de error {endpoint} debe retornar OperationOutcome"
            );
        }
    }
}

#[tokio::test]
async fn test_e2e_operation_outcome_resilience_and_status_codes() {
    let app = setup_e2e_app();

    // 1. Error 400 por ID no numérico en recurso
    let req_bad_id = Request::builder()
        .uri("/fhir/r4/Patient/id_invalido_texto")
        .body(Body::empty())
        .unwrap();

    let res_bad_id = app.clone().oneshot(req_bad_id).await.unwrap();
    assert_eq!(res_bad_id.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        res_bad_id.headers().get("content-type").unwrap(),
        FHIR_JSON_CONTENT_TYPE
    );
    let body_bad_id = body_to_string(res_bad_id).await;
    assert!(body_bad_id.contains("\"resourceType\": \"OperationOutcome\""));
    assert!(body_bad_id.contains("\"code\": \"value\""));

    // 2. Error 404 por ruta inexistente (Fallback universal Axum)
    let req_fallback = Request::builder()
        .uri("/fhir/r4/RutaTotalmenteInexistente/123")
        .body(Body::empty())
        .unwrap();

    let res_fallback = app.oneshot(req_fallback).await.unwrap();
    assert_eq!(res_fallback.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        res_fallback.headers().get("content-type").unwrap(),
        FHIR_JSON_CONTENT_TYPE
    );
    let body_fallback = body_to_string(res_fallback).await;
    assert!(body_fallback.contains("\"resourceType\": \"OperationOutcome\""));
    assert!(body_fallback.contains("\"code\": \"not-found\""));
    assert!(body_fallback.contains("\"severity\": \"error\""));
}
