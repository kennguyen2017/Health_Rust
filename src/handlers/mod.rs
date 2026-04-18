use axum::extract::{Query, State};
use axum::Json;
use serde::Serialize;

use crate::dto::top::{TopPageQuery, TopPageResponse};
use crate::errors::AppError;
use crate::services;
use crate::state::AppState;

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

pub async fn get_top_page(
    State(state): State<AppState>,
    Query(query): Query<TopPageQuery>,
) -> Result<Json<TopPageResponse>, AppError> {
    let response = services::top::get_top_page(&state.pool, query.user_id.unwrap_or(1)).await?;
    Ok(Json(response))
}
