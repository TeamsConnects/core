pub mod integrations;

use integrations::{IntegrationConfig};
use std::fs;
use std::path::Path;


pub fn load_integrations() -> Vec<IntegrationConfig> {
    let integration_path = Path::new("integrations");

    let mut integrations = Vec::new();

    if let Ok(entries) = fs::read_dir(integration_path) {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let integration_config_path = entry.path().join("init.yaml");

                if let Ok(config_content) = fs::read_to_string(&integration_config_path) {
                    if let Ok(config) = serde_yaml::from_str::<IntegrationConfig>(&config_content) {
                        integrations.push(config);
                    } else {
                        eprintln!("Failed to deserialize config from {:?}", integration_config_path);
                    }
                } else {
                    eprintln!("Failed to read config from {:?}", integration_config_path);
                }
            }
        }
    }

    integrations
}
