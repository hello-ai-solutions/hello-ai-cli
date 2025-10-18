use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalPiiResult {
    pub has_pii: bool,
    pub sanitized_text: String,
    pub entities: Vec<LocalPiiEntity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalPiiEntity {
    pub entity_type: String,
    pub confidence: f32,
    pub begin_offset: i32,
    pub end_offset: i32,
}

pub struct LocalPiiScanner {
    endpoint: String,
    model: String,
}

impl LocalPiiScanner {
    pub fn new(endpoint: String, model: String) -> Self {
        LocalPiiScanner { endpoint, model }
    }

    pub async fn scan_text(
        &self,
        text: &str,
    ) -> Result<LocalPiiResult, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();

        let prompt = format!(
            "You are a PII detection system. Analyze the text for Personally Identifiable Information.

DETECT:
- NAME: Any person's name (e.g., John, Ali, Sarah Smith)
- EMAIL: Email addresses
- PHONE: Phone numbers
- SSN: Social Security Numbers
- CREDIT_CARD: Credit card numbers
- ADDRESS: Physical addresses

IMPORTANT: Even single names like 'Ali', 'John', 'Sarah' are PII and must be detected.

Respond ONLY with valid JSON in this exact format:
{{\"has_pii\": true, \"entities\": [{{\"type\": \"NAME\", \"text\": \"Ali\", \"confidence\": 0.95}}]}}

If no PII found:
{{\"has_pii\": false, \"entities\": []}}

Text to analyze: \"{}\"",
            text
        );

        let request_body = json!({
            "model": self.model,
            "prompt": prompt,
            "stream": false,
            "options": {
                "temperature": 0.1
            }
        });

        let response = client
            .post(&self.endpoint)
            .json(&request_body)
            .send()
            .await?;

        let response_text = response.text().await?;
        let ollama_response: serde_json::Value = serde_json::from_str(&response_text)?;

        if let Some(response_content) = ollama_response["response"].as_str() {
            // Try to parse JSON response from LLM
            if let Ok(pii_analysis) = serde_json::from_str::<serde_json::Value>(response_content) {
                let has_pii = pii_analysis["has_pii"].as_bool().unwrap_or(false);
                let entities = pii_analysis["entities"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .filter_map(|entity| {
                        Some(LocalPiiEntity {
                            entity_type: entity["type"].as_str()?.to_string(),
                            confidence: entity["confidence"].as_f64().unwrap_or(0.0) as f32,
                            begin_offset: 0, // Local LLM doesn't provide exact positions
                            end_offset: 0,
                        })
                    })
                    .collect();

                let sanitized_text = if has_pii {
                    self.sanitize_text_simple(text)
                } else {
                    text.to_string()
                };

                return Ok(LocalPiiResult {
                    has_pii,
                    sanitized_text,
                    entities,
                });
            }
        }

        // Fallback: no PII detected if parsing fails
        Ok(LocalPiiResult {
            has_pii: false,
            sanitized_text: text.to_string(),
            entities: vec![],
        })
    }

    fn sanitize_text_simple(&self, text: &str) -> String {
        // Simple sanitization - replace common PII patterns
        let mut sanitized = text.to_string();

        // Email pattern
        let email_regex =
            regex::Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
        sanitized = email_regex.replace_all(&sanitized, "[EMAIL]").to_string();

        // Phone pattern
        let phone_regex =
            regex::Regex::new(r"\b(?:\+?1[-.\s]?)?\(?[0-9]{3}\)?[-.\s]?[0-9]{3}[-.\s]?[0-9]{4}\b")
                .unwrap();
        sanitized = phone_regex.replace_all(&sanitized, "[PHONE]").to_string();

        sanitized
    }
}
