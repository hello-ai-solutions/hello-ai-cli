use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct UseAws {
    pub region: String,
    pub service_name: String,
    pub operation_name: String,
    pub parameters: HashMap<String, Value>,
    pub profile_name: Option<String>,
    pub label: String,
}

impl UseAws {
    pub async fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut cmd = Command::new("aws");

        // Add profile if specified
        if let Some(profile) = &self.profile_name {
            cmd.arg("--profile").arg(profile);
        }

        // Add region
        cmd.arg("--region").arg(&self.region);

        // Add service and operation
        cmd.arg(&self.service_name).arg(&self.operation_name);

        // Add parameters
        for (key, value) in &self.parameters {
            let param_name = format!("--{}", key.replace('_', "-"));
            cmd.arg(param_name);

            match value {
                Value::String(s) => {
                    cmd.arg(s);
                }
                Value::Bool(true) => {
                    // Boolean flags don't need values
                }
                Value::Bool(false) => {
                    // Skip false boolean flags
                    continue;
                }
                _ => {
                    cmd.arg(value.to_string());
                }
            }
        }

        // Always output JSON for consistency
        cmd.arg("--output").arg("json");

        let output = cmd.output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() {
            return Err(format!("AWS CLI error: {}", stderr).into());
        }

        // Try to format JSON nicely
        if let Ok(json_value) = serde_json::from_str::<Value>(&stdout) {
            Ok(serde_json::to_string_pretty(&json_value)?)
        } else {
            Ok(stdout.to_string())
        }
    }
}
