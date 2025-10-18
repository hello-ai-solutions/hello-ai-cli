use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseAzure {
    pub service_name: String,
    pub operation_name: String,
    pub parameters: serde_json::Value,
    pub region: Option<String>,
}

impl UseAzure {
    pub async fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!(
            "Azure CLI not implemented yet. Would execute: az {} {}",
            self.service_name, self.operation_name
        ))
    }
}
