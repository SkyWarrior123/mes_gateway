use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt; // for collect
use tower::ServiceExt; 

#[tokio::test]
async fn test_health_endpoints() {
    println!(" Build the app");
    let app = mes_gateway::app();

    println!(" Send GET /health/live");
    let response = app
        .oneshot(Request::builder().uri("/health/live").body(Body::empty()).unwrap())
        .await
        .unwrap();

    println!(" Check response status: {}", response.status());
    assert_eq!(response.status(), StatusCode::OK, "health endpoint should return 200");

    println!(" Test passed");
}

#[tokio::test]
async fn test_ingest_endpoint_success() {
    let app = mes_gateway::app();

    let payload = serde_json::json!({
        "station_id": "ASSEMBLY-02",
        "timestamp": "1710001234",
        "part_code": "BOLT-M8",
        "status": "PASS",
        "metrics": { "cycle_time": 320 }
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/ingest")
                .header("content-type", "application/json")
                .body(Body::from(payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}