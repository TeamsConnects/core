use std::collections::HashMap;

use serde_yaml::Value;

use super::integrations::IntegrationConfig;

pub struct IntegrationPreprocessor {
    directive_functions: HashMap<String, fn() -> String>,
}

impl IntegrationPreprocessor {
    pub fn new() -> Self {
        IntegrationPreprocessor {
            directive_functions: HashMap::new(),
        }
    }

    pub fn add_directive(&mut self, name: &str, func: fn() -> String) {
        self.directive_functions.insert(name.to_string(), func);
    }

    pub fn process_integration<'a>(
        &mut self,
        integration: &'a mut IntegrationConfig,
    ) -> &'a mut IntegrationConfig {
        self.process_environment(&mut integration.env);
        self.update_string(&mut integration.description, &integration.env);
        self.update_string(&mut integration.name, &integration.env);
        self.update_string(&mut integration.container.image, &integration.env);
        self.update_string(&mut integration.container.name, &integration.env);

        for port in &mut integration.container.options.ports {
            self.update_string(port, &integration.env);
        }

        for env in &mut integration.container.options.environment {
            self.update_string(env, &integration.env);
        }

        integration
    }

    fn process_environment(&mut self, integration_env: &mut HashMap<String, Value>) {
        for (_, value) in integration_env.iter_mut() {
            if let Value::String(s) = value {
                println!("{:?}", s);
                while let Some(directive) = extract_directive_name(&s) {
                    if let Some(func) = self.directive_functions.get(&directive) {
                        let generated_value = func();
                        *s = replace_directive(&s, &directive, &generated_value);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn update_string(&mut self, value: &mut String, env: &HashMap<String, Value>) {
        *value = self.replace_all_directives_and_env(value, env);
    }

    fn replace_all_directives_and_env(&mut self, original: &str, env: &HashMap<String, Value>) -> String {
        let mut result = original.to_string();

        while let Some(s) = extract_directive_name(&result) {
            if let Some(func) = self.directive_functions.get(&s) {
                let generated_value = func();
                result = replace_directive(&result, &s, &generated_value);
            }
        }

        while let Some(s) = extract_env_name(&result) {
            if let Some(env_value) = env.get(&s) {
                result = replace_env(&result, &s, &env_value);
            }
        }

        result
    }
}

// Remaining functions remain unchanged


fn extract_directive_name(s: &str) -> Option<String> {
    let directive_start = "{{ directive.";
    let directive_end = " }}";

    if let Some(start_idx) = s.find(directive_start) {
        if let Some(end_idx) = s[start_idx + directive_start.len()..].find(directive_end) {
            let directive_name = &s[start_idx + directive_start.len()..start_idx + directive_start.len() + end_idx];
            return Some(directive_name.to_string());
        }
    }

    None
}

fn extract_env_name(s: &str) -> Option<String> {
    let directive_start = "{{ env.";
    let directive_end = " }}";

    if let Some(start_idx) = s.find(directive_start) {
        if let Some(end_idx) = s[start_idx + directive_start.len()..].find(directive_end) {
            let directive_name = &s[start_idx + directive_start.len()..start_idx + directive_start.len() + end_idx];
            return Some(directive_name.to_string());
        }
    }
    None
}

fn replace_env(original: &str, env_name: &str, env_value: &Value) -> String {
    let directive_start = format!("{{{{ env.{} }}}}", env_name);
    
    let replaced = match env_value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => "null".to_string(),
        _ => "".to_string(),  // Handle other Value types as needed
    };

    original.replace(&directive_start, &replaced)
}

fn replace_directive(original: &str, directive_name: &str, generated_value: &str) -> String {
    let directive_start = format!("{{{{ directive.{} }}}}", directive_name);
    original.replace(&directive_start, generated_value)
}