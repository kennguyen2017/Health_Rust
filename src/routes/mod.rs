use axum::{routing::{get, post}, Router};

use crate::handlers;
use crate::state::AppState;

pub fn api_router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::root))
        .route("/health", get(handlers::health_check))
        .route("/api/v1/health", get(handlers::health_check))
        .route("/api/v1/top", get(handlers::get_top_page))
        .route("/api/v1/my-record", get(handlers::get_my_record))
    .route("/api/v1/my-record/exercises", get(handlers::get_my_record_exercises))
    .route("/api/v1/my-record/diaries", get(handlers::get_my_record_diaries))
    .route("/api/v1/columns", get(handlers::get_columns))
    .route("/api/v1/columns/:id", get(handlers::get_column_detail))
    .route("/api/v1/auth/google/register", post(handlers::start_google_register))
    .route("/api/v1/auth/google/login", post(handlers::start_google_login))
    .route("/api/v1/auth/google/callback", get(handlers::google_auth_callback))
}
