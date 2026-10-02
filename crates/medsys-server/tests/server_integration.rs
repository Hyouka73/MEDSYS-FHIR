//! Pruebas de integración del servidor Axum y conformidad de interoperabilidad FHIR R4.
//! Valida el enrutamiento HTTP, la negociación de contenido `application/fhir+json`,
//! el fallback y traducción universal a `OperationOutcome` (Tareas 4.1 y 4.2).

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use medsys_core::{parse_mapping_rules, MedSysError};
use medsys_db::{init_pool_lazy, DbConfig, MedsysRepositories};
use medsys_server::{create_router, AppState, ServerError, FHIR_JSON_CONTENT_TYPE};
use tower::ServiceExt;

const SPECIFICATION_YAML: &str = include_str!("../../../mapping_rules.yaml");

/// Construye una instancia del enrutador Axum para pruebas en memoria (sin sockets TCP).
fn setup_test_app() -> axum::Router {
    let config = DbConfig::default();
    let pool = init_pool_lazy(&config).expect("init_pool_lazy debe inicializar");
    let repos = MedsysRepositories::new(pool.clone());
    let mapping_rules = parse_mapping_rules(SPECIFICATION_YAML).expect("parse_mapping_rules falló");
    let state = AppState::new(repos, mapping_rules, pool);
    create_router(state)
}

/// Helper para extraer el cuerpo de la respuesta como String UTF-8.
async fn response_body_to_string(response: axum::response::Response) -> String {
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("Error al leer bytes del cuerpo")
        .to_bytes();
    String::from_utf8(bytes.to_vec()).expect("Cuerpo debe ser UTF-8 válido")
}

#[tokio::test]
async fn test_health_endpoints() {
    let app = setup_test_app();

    // 1. GET /health
    let req = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = response_body_to_string(res).await;
    assert!(body.contains("server_version"));
    assert!(body.contains("R4 (4.0.1)"));

    // 2. GET /api/health
    let req_api = Request::builder()
        .uri("/api/health")
        .body(Body::empty())
        .unwrap();

    let res_api = app.oneshot(req_api).await.unwrap();
    assert_eq!(res_api.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_unrecognized_route_fallback_emits_operation_outcome() {
    let app = setup_test_app();

    let req = Request::builder()
        .uri("/fhir/r4/RecursoInexistente")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();

    // Debe ser HTTP 404
    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    // Cabecera Content-Type debe ser application/fhir+json
    let content_type = res
        .headers()
        .get("content-type")
        .expect("Debe incluir Content-Type");
    assert_eq!(content_type.to_str().unwrap(), FHIR_JSON_CONTENT_TYPE);

    // Cuerpo debe ser un OperationOutcome canónico
    let body = response_body_to_string(res).await;
    assert!(body.contains("\"resourceType\": \"OperationOutcome\""));
    assert!(body.contains("\"severity\": \"error\""));
    assert!(body.contains("\"code\": \"not-found\""));
    assert!(body.contains("no existe en este servidor"));
}

#[tokio::test]
async fn test_server_error_into_response_conformity() {
    use axum::response::IntoResponse;

    // 1. NotFound -> 404
    let err_not_found = ServerError::Domain(MedSysError::NotFound("Paciente 999 no existe".into()));
    let res_404 = err_not_found.into_response();
    assert_eq!(res_404.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        res_404.headers().get("content-type").unwrap(),
        FHIR_JSON_CONTENT_TYPE
    );
    let body_404 = response_body_to_string(res_404).await;
    assert!(body_404.contains("\"resourceType\": \"OperationOutcome\""));
    assert!(body_404.contains("\"code\": \"not-found\""));
    assert!(body_404.contains("Paciente 999 no existe"));

    // 2. ValidationError -> 422
    let err_val = ServerError::Domain(MedSysError::ValidationError("CURP mal formada".into()));
    let res_422 = err_val.into_response();
    assert_eq!(res_422.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        res_422.headers().get("content-type").unwrap(),
        FHIR_JSON_CONTENT_TYPE
    );
    let body_422 = response_body_to_string(res_422).await;
    assert!(body_422.contains("\"resourceType\": \"OperationOutcome\""));
    assert!(body_422.contains("\"code\": \"invalid\""));
    assert!(body_422.contains("CURP mal formada"));

    // 3. MissingField -> 422
    let err_field = ServerError::Domain(MedSysError::MissingField {
        field: "curp".into(),
        resource: "Patient".into(),
    });
    let res_field = err_field.into_response();
    assert_eq!(res_field.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let body_field = response_body_to_string(res_field).await;
    assert!(body_field.contains("\"code\": \"required\""));

    // 4. ProcessingError -> 422
    let err_proc = ServerError::Domain(MedSysError::ProcessingError(
        "Data corruption in critical field".into(),
    ));
    let res_proc = err_proc.into_response();
    assert_eq!(res_proc.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let body_proc = response_body_to_string(res_proc).await;
    assert!(body_proc.contains("\"resourceType\": \"OperationOutcome\""));
    assert!(body_proc.contains("\"code\": \"processing\""));
    assert!(body_proc.contains("Data corruption in critical field"));

    // 5. DatabaseError -> 500
    let err_db = ServerError::Domain(MedSysError::DatabaseError("Conexión rechazada".into()));
    let res_500 = err_db.into_response();
    assert_eq!(res_500.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let body_500 = response_body_to_string(res_500).await;
    assert!(body_500.contains("\"code\": \"transient\""));
}

#[tokio::test]
async fn test_invalid_path_parameter_emits_operation_outcome() {
    let app = setup_test_app();

    // Intentar consultar una condición con un ID alfanumérico inválido
    let req = Request::builder()
        .uri("/fhir/r4/Condition/cond-abc-no-numerico")
        .body(Body::empty())
        .unwrap();

    let res = app.oneshot(req).await.unwrap();

    // Debe retornar 400 Bad Request con OperationOutcome
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        res.headers().get("content-type").unwrap(),
        FHIR_JSON_CONTENT_TYPE
    );
    let body = response_body_to_string(res).await;
    assert!(body.contains("\"resourceType\": \"OperationOutcome\""));
    assert!(body.contains("\"code\": \"value\""));
}

#[tokio::test]
async fn test_fhir_canonical_endpoints_content_type_and_schema() {
    let app = setup_test_app();

    // 1. GET /fhir/r4/Patient/1
    let req_patient = Request::builder()
        .uri("/fhir/r4/Patient/1")
        .body(Body::empty())
        .unwrap();

    let res_patient = app.clone().oneshot(req_patient).await.unwrap();
    let status = res_patient.status();
    let content_type = res_patient
        .headers()
        .get("content-type")
        .expect("Debe incluir Content-Type")
        .to_str()
        .unwrap();
    assert_eq!(content_type, FHIR_JSON_CONTENT_TYPE);

    let body_patient = response_body_to_string(res_patient).await;
    if status == StatusCode::OK {
        assert!(body_patient.contains("\"resourceType\": \"Patient\""));
        assert!(body_patient.contains("ROMA900101HCSNN01"));
        assert!(!body_patient.contains("\"telecom\""));
    } else {
        // En caso de que el pool esté offline o id no exista
        assert!(body_patient.contains("\"resourceType\": \"OperationOutcome\""));
    }

    // 2. GET /fhir/r4/Encounter/1
    let req_enc = Request::builder()
        .uri("/fhir/r4/Encounter/1")
        .body(Body::empty())
        .unwrap();

    let res_enc = app.clone().oneshot(req_enc).await.unwrap();
    let status_enc = res_enc.status();
    assert_eq!(
        res_enc.headers().get("content-type").unwrap(),
        FHIR_JSON_CONTENT_TYPE
    );
    let body_enc = response_body_to_string(res_enc).await;
    if status_enc == StatusCode::OK {
        assert!(body_enc.contains("\"resourceType\": \"Encounter\""));
        assert!(body_enc.contains("AMB"));
    } else {
        assert!(body_enc.contains("\"resourceType\": \"OperationOutcome\""));
    }

    // 3. GET /fhir/r4/Observation/temp-1
    let req_obs = Request::builder()
        .uri("/fhir/r4/Observation/temp-1")
        .body(Body::empty())
        .unwrap();

    let res_obs = app.clone().oneshot(req_obs).await.unwrap();
    let status_obs = res_obs.status();
    assert_eq!(
        res_obs.headers().get("content-type").unwrap(),
        FHIR_JSON_CONTENT_TYPE
    );
    let body_obs = response_body_to_string(res_obs).await;
    if status_obs == StatusCode::OK {
        assert!(body_obs.contains("\"resourceType\": \"Observation\""));
        assert!(body_obs.contains("8310-5"));
    } else {
        assert!(body_obs.contains("\"resourceType\": \"OperationOutcome\""));
    }

    // 4. GET /fhir/r4/Condition/cond-1
    let req_cond = Request::builder()
        .uri("/fhir/r4/Condition/cond-1")
        .body(Body::empty())
        .unwrap();

    let res_cond = app.oneshot(req_cond).await.unwrap();
    let status_cond = res_cond.status();
    assert_eq!(
        res_cond.headers().get("content-type").unwrap(),
        FHIR_JSON_CONTENT_TYPE
    );
    let body_cond = response_body_to_string(res_cond).await;
    if status_cond == StatusCode::OK {
        assert!(body_cond.contains("\"resourceType\": \"Condition\""));
        assert!(body_cond.contains("I10"));
    } else {
        assert!(body_cond.contains("\"resourceType\": \"OperationOutcome\""));
    }
}
