use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    #[serde(default = "default_http")]
    pub http: HttpServerSettings,
}

#[derive(Deserialize, Debug)]
pub struct HttpServerSettings {
    #[serde(default = "default_http_address")]
    pub address: String,
    #[serde(default = "default_http_port")]
    pub port: u16,
}

fn default_http_address() -> String {
    "127.0.0.1".to_string()
}

fn default_http_port() -> u16 {
    8080
}

fn default_http() -> HttpServerSettings {
    HttpServerSettings {
        address: default_http_address(),
        port: default_http_port(),
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = Config::new();
        config.merge(Environment::with_prefix("dme").separator("_"))?;
        config.try_into()
    }
}
