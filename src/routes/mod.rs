use axum::{routing::get, Router};

use crate::handlers;
use crate::state::AppState;

pub fn api_router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::root))
        .route("/health", get(handlers::health_check))
        .route("/api/v1/health", get(handlers::health_check))
        .route("/api/v1/top", get(handlers::get_top_page))
        .route("/api/v1/my-record", get(handlers::get_my_record))
}
