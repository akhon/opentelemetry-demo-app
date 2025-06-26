use serde::{Deserialize, Serialize};
use std::fs;
use std::net::SocketAddr;

fn default_redis_url() -> String {
    "redis://127.0.0.1:6379".to_string()
}

fn default_listen_address() -> String {
    "127.0.0.1:3000".to_string()
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    /// Redis URL to connect to
    #[serde(default = "default_redis_url")]
    pub redis_url: String,

    /// Address to listen on (as string for YAML compatibility)
    #[serde(default = "default_listen_address")]
    pub listen_address: String,
}

impl Config {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match fs::read_to_string(path) {
            Ok(contents) => {
                let mut config: Config = serde_yaml::from_str(&contents)?;
                // Ensure we have defaults for any missing fields
                if config.redis_url.is_empty() {
                    config.redis_url = "redis://127.0.0.1:6379".to_string();
                }
                if config.listen_address.is_empty() {
                    config.listen_address = "127.0.0.1:3000".to_string();
                }
                Ok(config)
            }
            Err(e) => {
                tracing::error!("Error reading config file {}: {}", path, e);
                Err(Box::new(e))
            }
        }
    }

    pub fn listen_socket_addr(&self) -> Result<SocketAddr, Box<dyn std::error::Error>> {
        Ok(self.listen_address.parse()?)
    }
}
