use serde::Deserialize;
use config::ConfigError;
pub struct ServerConfig {
    pub host: String,
    pub port: i32
}


pub struct Config {
    pub server: ServerConfig
}

impl Config{
    pub fn from_env() -> Restust<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environmaent::new())?;
        cfg.try_into()
    }
}