use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: IpAddr,
    pub port: u16,
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

        Self { host, port }
    }
}
