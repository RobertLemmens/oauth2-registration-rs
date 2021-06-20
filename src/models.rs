
use config::ConfigError;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Credentials {
    pub user: String,
    pub pass: String,
}

#[derive(Deserialize, Serialize)]
pub struct AuthorizationParams {
    pub client_id: String,
    pub pass: String,
}

#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub name: String,
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
