use std::net::{IpAddr, Ipv4Addr};

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: IpAddr,
    pub port: u16,
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let host = std::env::var("APP_HOST")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));

        let port = std::env::var("APP_PORT")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(8080);

        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@localhost:5433/health_rust_backend".to_string()
        });

        Self {
            host,
            port,
            database_url,
        }
    }

    pub async fn create_db_pool(&self) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.database_url)
            .await
    }
}
