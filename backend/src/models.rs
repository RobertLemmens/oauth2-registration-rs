
use config::ConfigError;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Credentials {
    pub user: String,
    pub pass: String,
}

#[derive(Deserialize, Serialize)]
pub struct Registration {
    pub user: String,
    pub pass: String,
    pub email: String
}

#[derive(Deserialize, Serialize)]
pub struct OtpToken {
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct AuthorizationCode {
    pub code: String,
}

#[derive(Deserialize, Serialize)]
pub struct AuthorizationParams {
    pub client_id: String,
    pub username: String,
    pub pcke: String,
    pub device: String
}

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    pub msg: String,
}

#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub cert_dir: String,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub bootstrap: bool,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into() // probeer te deserializen in het geselecteerde object
    }
}
