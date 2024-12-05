use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::error::Error;

static INSTANCE: OnceLock<Config> = OnceLock::new();

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub pg_url: String,
    pub max_connections: u32,
}

impl Config {
    pub async fn new() -> Result<Self, Error> {
        let pg_url =
            std::env::var("PG_URL").unwrap_or("postgres://user:pass@postgresql/web".to_string());
        let max_connections = std::env::var("MAX_CONNECTIONS")
            .unwrap_or("5".to_string())
            .parse::<u32>()
            .unwrap_or(5);
        Ok(Self {
            pg_url,
            max_connections,
        })
    }

    pub fn global() -> &'static Config {
        INSTANCE.get().expect("config is not initialized")
    }

    fn set_global(value: &Config) {
        let _ = INSTANCE.set(value.clone());
    }

    pub async fn setup() -> Result<Self, Error> {
        let config = Self::new().await?;
        Self::set_global(&config);
        Ok(config)
    }
}
