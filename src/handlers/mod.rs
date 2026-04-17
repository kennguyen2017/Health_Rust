use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

pub async fn root() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "health_rust_backend",
    })
}

pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "health_rust_backend",
    })
}
