use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use chrono::NaiveDate;
use http_body_util::BodyExt;
use medsys_core::model::fhir_helpers::fhir_datetime;
use medsys_core::{parse_mapping_rules, transform_condition, LegacyDiagnostico};
use medsys_db::{init_pool_lazy, DbConfig, MedsysRepositories};
use medsys_server::{create_router, AppState};
use tower::ServiceExt;

fn disconnected_app() -> axum::Router {
    let cfg = DbConfig {
        database_url: "postgres://synthetic:synthetic@127.0.0.1:9/synthetic".into(),
        acquire_timeout_secs: 1,
        min_connections: 0,
        ..Default::default()
    };
    let pool = init_pool_lazy(&cfg).unwrap();
    create_router(AppState::new(
        MedsysRepositories::new(pool.clone()),
        parse_mapping_rules(include_str!("../../../config/mapping_rules_s0.yaml")).unwrap(),
        pool,
    ))
}

#[test]
fn regression_datetime_requires_offset_and_preserves_fraction() {
    let dt = NaiveDate::from_ymd_opt(2026, 1, 1)
        .unwrap()
        .and_hms_nano_opt(10, 0, 0, 123_456_000)
        .unwrap();
    let value = serde_json::to_value(fhir_datetime(dt).unwrap()).unwrap();
    let text = value.as_str().unwrap();
    assert!(text.ends_with("-06:00"), "observed {text}");
    assert!(text.contains(".123456"), "observed {text}");
    let ambiguous = NaiveDate::from_ymd_opt(2021, 10, 31)
        .unwrap()
        .and_hms_opt(1, 30, 0)
        .unwrap();
    assert!(
        fhir_datetime(ambiguous).is_err(),
        "ambiguous local clock must reject"
    );
}

#[test]
fn regression_condition_preserves_date_precision() {
    let d = LegacyDiagnostico {
        id_diagnostico: 1,
        id_consulta: 1,
        id_paciente: 1,
        codigo_cie10: "E11.9".into(),
        descripcion_diagnostico: "Diabetes".into(),
        tipo_diagnostico: None,
        fecha_diagnostico: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
    };
    let v = serde_json::to_value(transform_condition(&d, None).unwrap()).unwrap();
    assert_eq!(v["recordedDate"], "2026-01-01");
}

#[tokio::test]
async fn regression_query_and_method_are_operation_outcome() {
    let cases: Vec<_> = ["Patient", "Encounter", "Observation", "Condition"]
        .into_iter()
        .flat_map(|resource| {
            [
                (
                    "GET",
                    format!("/fhir/r4/{resource}?limit=wrong"),
                    StatusCode::BAD_REQUEST,
                    "invalid",
                ),
                (
                    "POST",
                    format!("/fhir/r4/{resource}"),
                    StatusCode::METHOD_NOT_ALLOWED,
                    "not-supported",
                ),
            ]
        })
        .collect();
    for (method, uri, status, issue) in cases {
        let res = disconnected_app()
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(uri)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), status);
        assert!(res.headers()["content-type"]
            .to_str()
            .unwrap()
            .starts_with("application/fhir+json"));
        let bytes = res.into_body().collect().await.unwrap().to_bytes();
        let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(v["resourceType"], "OperationOutcome");
        assert_eq!(v["issue"][0]["code"], issue);
        assert_eq!(v["issue"][0]["severity"], "error");
    }
}

#[tokio::test]
async fn regression_disconnected_health_is_service_unavailable() {
    let res = disconnected_app()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::SERVICE_UNAVAILABLE);
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["status"], "degraded");
}

#[test]
fn regression_required_source_cannot_fall_back_to_constant() {
    let rule = medsys_core::FieldMapping {
        source_column: Some("observed".into()),
        constant_value: Some("invented".into()),
        target_path: "status".into(),
        ..Default::default()
    };
    let mut target = serde_json::json!({});
    assert!(medsys_core::evaluate_field_mapping(
        &rule,
        "Observation",
        &serde_json::Map::new(),
        &mut target
    )
    .is_err());
}
