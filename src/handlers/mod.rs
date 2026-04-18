use axum::extract::{Query, State};
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

use crate::dto::my_record::{MyRecordQuery, MyRecordResponse};
use crate::dto::top::{TopPageQuery, TopPageResponse};
use crate::errors::{AppError, ErrorResponse};
use crate::services;
use crate::state::AppState;

#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    #[schema(example = "ok")]
    status: &'static str,
    #[schema(example = "health_rust_backend")]
    service: &'static str,
}

#[utoipa::path(
    get,
    path = "/",
    tag = "Health",
    responses(
        (status = 200, description = "Root health response", body = HealthResponse)
    )
)]
pub async fn root() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "health_rust_backend",
    })
}

#[utoipa::path(
    get,
    path = "/api/v1/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service health response", body = HealthResponse)
    )
)]
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "health_rust_backend",
    })
}

#[utoipa::path(
    get,
    path = "/api/v1/top",
    tag = "Top Page",
    params(TopPageQuery),
    responses(
        (status = 200, description = "Top page data", body = TopPageResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_top_page(
    State(state): State<AppState>,
    Query(query): Query<TopPageQuery>,
) -> Result<Json<TopPageResponse>, AppError> {
    let response = services::top::get_top_page(&state.pool, query.user_id.unwrap_or(1)).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/my-record",
    tag = "My Record",
    params(MyRecordQuery),
    responses(
        (status = 200, description = "My Record page data", body = MyRecordResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_my_record(
    State(state): State<AppState>,
    Query(query): Query<MyRecordQuery>,
) -> Result<Json<MyRecordResponse>, AppError> {
    let response = services::my_record::get_my_record(&state.pool, query.user_id.unwrap_or(3)).await?;
    Ok(Json(response))
}
