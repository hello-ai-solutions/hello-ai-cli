#![allow(dead_code)]
// Minimal Local LLM module - only essential functionality

pub struct LocalLLM {
    endpoint: String,
    model: String,
}

impl LocalLLM {
    pub fn new(endpoint: String, model: String) -> Self {
        LocalLLM { endpoint, model }
    }

    pub async fn analyze_code(&self, _code: &str, _language: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        Ok("Analysis not available in minimal mode".to_string())
    }
}
