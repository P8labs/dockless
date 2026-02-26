use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub listen_port: u16,
    pub data_dir: String,
    pub node_id: String,
}

pub fn load_config() -> Result<Config> {
    let config_path = std::env::var("DLESS_CONFIG").unwrap_or_else(|_| "dockless.toml".to_string());

    if std::path::Path::new(&config_path).exists() {
        let content = std::fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    } else {
        info!("Config file not found, using defaults");
        Ok(default_config())
    }
}

fn default_config() -> Config {
    Config {
        node_id: "./node_id".to_string(),
        data_dir: std::env::var("DLESS_DATA_PATH").unwrap_or_else(|_| "./dle_data".to_string()),
        listen_port: std::env::var("DLESS_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8000),
    }
}
