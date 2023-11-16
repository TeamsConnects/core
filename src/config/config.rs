use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationConfig {
    pub name: String,
    pub server: ServerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u32,
    pub domain: String,
}

pub fn from_file(file_path: &str) -> Result<ApplicationConfig, String> {
    // Open the configuration file
    let mut file = File::open(file_path).map_err(|e| format!("Failed to open {}: {}", file_path, e))?;

    // Read the file content into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Failed to read {}: {}", file_path, e))?;

    // Deserialize the YAML into a Config struct
    serde_yaml::from_str(&contents).map_err(|e| format!("Failed to deserialize {}: {}", file_path, e))
}



