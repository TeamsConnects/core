use serde::Deserialize;
use serde_yaml::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct IntegrationConfig {
    pub start_hook: StartHook,
    pub name: String,
    pub env: HashMap<String, Value>,
    pub description: String,
    pub image: Option<String>,
    pub container: ContainerConfig,
}

#[derive(Debug, Deserialize)]
pub struct ContainerConfig {
    pub name: String,
    pub image: String,
    pub options: ContainerOptions,
}

#[derive(Debug, Deserialize)]
pub enum StartHook {
    #[serde(rename = "PRE_START_UP")]
    PreStartUp,
    // Add other variants as needed
}

#[derive(Debug, Deserialize)]
pub struct ContainerOptions {
    #[serde(default)]
    pub ports: Vec<String>,
    #[serde(default)]
    pub environment: Vec<String>,
}