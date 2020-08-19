use config::{Config, ConfigError, Environment};
use serde::Deserialize;

pub mod build_info;

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(default)]
pub struct Settings {
    pub http: HttpServerSettings,

    pub docker: DockerClientSettings,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
pub struct HttpServerSettings {
    pub address: String,
    pub port: u16,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(default)]
pub struct DockerClientSettings {
    pub url: String,
}

impl Default for HttpServerSettings {
    fn default() -> Self {
        HttpServerSettings {
            address: "127.0.0.1".to_string(),
            port: 9091,
        }
    }
}

impl Default for DockerClientSettings {
    fn default() -> Self {
        DockerClientSettings {
            url: "unix:///var/run/docker.sock".to_string()
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = Config::new();
        config.merge(Environment::with_prefix("dme").separator("_"))?;

        let settings: Self = config.try_into()?;
        settings.validate()?;

        Ok(settings)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if !self.docker.url.contains("://") {
            return Err(ConfigError::Message(format!("Malformed Docker Client URL '{}'", self.docker.url).to_string()));
        }

        let docker_url_split: Vec<&str> = self.docker.url.split("://").collect();
        let schema = docker_url_split[0].to_lowercase();

        static ALLOWED_SCHEMAS: [&str; 3] = ["unix", "http", "https"];
        if !ALLOWED_SCHEMAS.contains(&schema.as_str()) {
            let message = format!("Docker Client URL schema '{}' must be one of the following: {}.",
                                  schema, ALLOWED_SCHEMAS.join(", "));
            return Err(ConfigError::Message(message.to_string()));
        }

        Ok(())
    }
}
