pub mod models;
pub mod transformer;

use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use models::RawErpPayload;
use transformer::map_erp_payload;

pub fn app() -> Router {
    Router::new()
        .route("/health/live", get(liveness_probe))
        .route("/health/ready", get(readiness_probe))
        .route("/api/v1/ingest", post(ingest_handler))
}

async fn liveness_probe() -> StatusCode {
    StatusCode::OK
}

async fn readiness_probe() -> StatusCode {
    // Check DB/dependency connections here in production
    StatusCode::OK
}

async fn ingest_handler(Json(payload): Json<RawErpPayload>) -> Result<StatusCode, StatusCode> {
    match map_erp_payload(&payload) {
        Ok(_telemetry) => {
            // Here you would forward to your internal pipeline / database
            Ok(StatusCode::CREATED)
        }
        Err(e) => {
            tracing::error!("Data mapping failed: {:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}