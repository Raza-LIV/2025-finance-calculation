use axum::{http::StatusCode, response::Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
}

/// Health check endpoint handler
pub async fn health_check() -> (StatusCode, Json<HealthResponse>) {
    let response = HealthResponse {
        status: "ok".to_string(),
    };
    (StatusCode::OK, Json(response))
}
