use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseOracle {
    pub service_name: String,
    pub operation_name: String,
    pub parameters: serde_json::Value,
    pub region: Option<String>,
}

impl UseOracle {
    pub async fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!(
            "Oracle CLI not implemented yet. Would execute: oci {} {}",
            self.service_name, self.operation_name
        ))
    }
}
