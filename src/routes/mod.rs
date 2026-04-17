use axum::{routing::get, Router};

use crate::handlers;

pub fn api_router() -> Router {
    Router::new()
        .route("/", get(handlers::root))
        .route("/health", get(handlers::health_check))
        .route("/api/v1/health", get(handlers::health_check))
}
