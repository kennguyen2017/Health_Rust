mod app;
mod config;
mod dto;
mod docs;
mod errors;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod state;

use std::net::SocketAddr;

use tokio::net::TcpListener;
use tracing::info;

use crate::app::build_app;
use crate::config::AppConfig;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();

    let config = AppConfig::from_env();
    let address = SocketAddr::from((config.host, config.port));
    let pool = config
        .create_db_pool()
        .await
        .expect("failed to create postgres connection pool");
    let state = AppState::new(pool);

    let listener = TcpListener::bind(address)
        .await
        .expect("failed to bind TCP listener");

    info!(%address, "health_rust_backend is starting");

    axum::serve(listener, build_app(state))
        .await
        .expect("server exited unexpectedly");
}

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,tower_http=info".to_string()),
        )
        .with_target(false)
        .compact()
        .init();
}
