use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

use crate::dto::auth::{
    GoogleAuthCallbackQuery, GoogleAuthCallbackResponse, GoogleAuthStartRequest,
    GoogleAuthStartResponse,
};
use crate::dto::column::{
    ColumnDetailResponse, ColumnListQuery, ColumnListResponse, CreateColumnRequest,
    CreateColumnResponse,
};
use crate::dto::my_record::{
    CreateMyRecordRequest, CreateMyRecordResponse, DiaryListResponse, ExerciseListResponse,
    MyRecordQuery, MyRecordResponse,
};
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

#[utoipa::path(
    get,
    path = "/api/v1/my-record/exercises",
    tag = "My Record",
    params(MyRecordQuery),
    responses(
        (status = 200, description = "My Record exercise list", body = ExerciseListResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_my_record_exercises(
    State(state): State<AppState>,
    Query(query): Query<MyRecordQuery>,
) -> Result<Json<ExerciseListResponse>, AppError> {
    let response = services::my_record::get_my_record_exercises(&state.pool, query.user_id.unwrap_or(3)).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/my-record/diaries",
    tag = "My Record",
    params(MyRecordQuery),
    responses(
        (status = 200, description = "My Record diary list", body = DiaryListResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_my_record_diaries(
    State(state): State<AppState>,
    Query(query): Query<MyRecordQuery>,
) -> Result<Json<DiaryListResponse>, AppError> {
    let response = services::my_record::get_my_record_diaries(&state.pool, query.user_id.unwrap_or(3)).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/columns",
    tag = "Columns",
    params(ColumnListQuery),
    responses(
        (status = 200, description = "Column page data", body = ColumnListResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_columns(
    State(state): State<AppState>,
    Query(query): Query<ColumnListQuery>,
) -> Result<Json<ColumnListResponse>, AppError> {
    let response = services::column::get_columns(&state.pool, query).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/columns/{id}",
    tag = "Columns",
    params(("id" = i64, Path, description = "Column ID")),
    responses(
        (status = 200, description = "Column detail", body = ColumnDetailResponse),
        (status = 404, description = "Column not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_column_detail(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ColumnDetailResponse>, AppError> {
    let response = services::column::get_column_detail(&state.pool, id).await?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/v1/columns",
    tag = "Columns",
    request_body = CreateColumnRequest,
    responses(
        (status = 200, description = "Column created", body = CreateColumnResponse),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn create_column(
    State(state): State<AppState>,
    Json(request): Json<CreateColumnRequest>,
) -> Result<Json<CreateColumnResponse>, AppError> {
    let response = services::column::create_column(&state.pool, request).await?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/v1/my-record",
    tag = "My Record",
    request_body = CreateMyRecordRequest,
    responses(
        (status = 200, description = "My Record entry created", body = CreateMyRecordResponse),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn create_my_record(
    State(state): State<AppState>,
    Json(request): Json<CreateMyRecordRequest>,
) -> Result<Json<CreateMyRecordResponse>, AppError> {
    let response = services::my_record::create_my_record(&state.pool, request).await?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/google/register",
    tag = "Auth",
    request_body = GoogleAuthStartRequest,
    responses(
        (status = 200, description = "Google register skeleton response", body = GoogleAuthStartResponse)
    )
)]
pub async fn start_google_register(
    Json(request): Json<GoogleAuthStartRequest>,
) -> Result<Json<GoogleAuthStartResponse>, AppError> {
    let response = services::auth::start_google_register(request).await?;
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/google/login",
    tag = "Auth",
    request_body = GoogleAuthStartRequest,
    responses(
        (status = 200, description = "Google login skeleton response", body = GoogleAuthStartResponse)
    )
)]
pub async fn start_google_login(
    Json(request): Json<GoogleAuthStartRequest>,
) -> Result<Json<GoogleAuthStartResponse>, AppError> {
    let response = services::auth::start_google_login(request).await?;
    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/google/callback",
    tag = "Auth",
    params(GoogleAuthCallbackQuery),
    responses(
        (status = 200, description = "Google auth callback skeleton response", body = GoogleAuthCallbackResponse)
    )
)]
pub async fn google_auth_callback(
    Query(query): Query<GoogleAuthCallbackQuery>,
) -> Result<Json<GoogleAuthCallbackResponse>, AppError> {
    let response = services::auth::handle_google_callback(query).await?;
    Ok(Json(response))
}
