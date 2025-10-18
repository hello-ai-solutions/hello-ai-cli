use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseGcp {
    pub service_name: String,
    pub operation_name: String,
    pub parameters: serde_json::Value,
    pub region: Option<String>,
}

impl UseGcp {
    pub async fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!(
            "GCP CLI not implemented yet. Would execute: gcloud {} {}",
            self.service_name, self.operation_name
        ))
    }
}
