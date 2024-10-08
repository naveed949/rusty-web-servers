use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub address: String,
    pub port: u16,
}

impl Config {
    pub fn from_file(_file_path: &str) -> Self {
        // Load and parse the configuration file
        // For simplicity, we'll use default values here
        Config {
            address: "127.0.0.1".to_string(),
            port: 8080,
        }
    }
}