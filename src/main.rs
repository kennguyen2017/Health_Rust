mod app;
mod config;
mod dto;
mod errors;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;

use std::net::SocketAddr;

use tokio::net::TcpListener;
use tracing::info;

use crate::app::build_app;
use crate::config::AppConfig;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();

    let config = AppConfig::from_env();
    let address = SocketAddr::from((config.host, config.port));

    let listener = TcpListener::bind(address)
        .await
        .expect("failed to bind TCP listener");

    info!(%address, "health_rust_backend is starting");

    axum::serve(listener, build_app())
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
