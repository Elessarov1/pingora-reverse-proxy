use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct ProxyConfig {
    pub proxy: ProxySection,
}

#[derive(Debug, Deserialize)]
pub struct ProxySection {
    pub listen: String,
    pub backend: String,
}

impl ProxyConfig {
    pub fn load_from_file(path: &str) -> Self {
        let content = fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("Failed to read config file: {}", e));
        toml::from_str(&content)
            .unwrap_or_else(|e| panic!("Failed to parse config: {}", e))
    }
}