use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub address: String,
    pub port: u16,
    pub thread_pool_size: usize,
}

impl Config {
    pub fn from_file(file_path: &str) -> Self {
        // Load and parse the configuration file
        // For simplicity, we'll use default values here
        Config {
            address: "127.0.0.1".to_string(),
            port: 7878,
            thread_pool_size: 4,
        }
    }
}