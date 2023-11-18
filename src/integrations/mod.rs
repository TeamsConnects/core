pub mod integrations;
pub mod preprocessor;
use integrations::IntegrationConfig;
use std::fs;
use std::path::Path;


use self::preprocessor::IntegrationPreprocessor;


pub fn load_integrations() -> Vec<IntegrationConfig> {
    let integration_path = Path::new("integrations");

    let mut integrations = Vec::new();
    let mut preprocessor = IntegrationPreprocessor::new();
    preprocessor.add_directive("generate_username", generate_random_username);
    preprocessor.add_directive("generate_pw", generate_random_password);
    if let Ok(entries) = fs::read_dir(integration_path) {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let integration_config_path = entry.path().join("init.yaml");

                if let Ok(config_content) = fs::read_to_string(&integration_config_path) {
                    if let Ok(mut config) = serde_yaml::from_str::<IntegrationConfig>(&config_content) {
                        preprocessor.process_integration(&mut config);
                        integrations.push(config);
                    } else {
                        eprintln!("Failed to deserialize config from {:?}. Content: {}", integration_config_path, &config_content);
                    }
                } else {
                    eprintln!("Failed to read config from {:?}", integration_config_path);
                }
            }
        }
    }

    integrations
}

fn generate_random_username() -> String {
    "user".to_string()
}

fn generate_random_password() -> String {
    "pw".to_string()
}