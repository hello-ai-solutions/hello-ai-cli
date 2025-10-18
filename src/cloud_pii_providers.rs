use serde_json::{json, Value};
use std::env;

#[allow(dead_code)]
pub async fn scan_with_azure_text_analytics(
    text: &str,
    api_key: &str,
    endpoint: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let key = if api_key.is_empty() {
        env::var("AZURE_TEXT_ANALYTICS_KEY").unwrap_or_default()
    } else {
        api_key.to_string()
    };

    if key.is_empty() || endpoint.is_empty() {
        return Err("Azure Text Analytics key and endpoint required. Set AZURE_TEXT_ANALYTICS_KEY env var and endpoint in config.toml".into());
    }

    let client = reqwest::Client::new();
    let url = format!("{}/text/analytics/v3.1/entities/recognition/pii", endpoint);

    let payload = json!({
        "documents": [
            {
                "id": "1",
                "language": "en",
                "text": text
            }
        ]
    });

    let response = client
        .post(&url)
        .header("Ocp-Apim-Subscription-Key", key)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    let response_text = response.text().await?;
    let json: Value = serde_json::from_str(&response_text)?;

    if let Some(documents) = json["documents"].as_array() {
        if let Some(first_doc) = documents.first() {
            if let Some(entities) = first_doc["entities"].as_array() {
                return Ok(!entities.is_empty());
            }
        }
    }

    Ok(false)
}

#[allow(dead_code)]
pub async fn scan_with_gcp_dlp(
    text: &str,
    project_id: &str,
    credentials_path: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let project = if project_id.is_empty() {
        env::var("GCP_PROJECT_ID").unwrap_or_default()
    } else {
        project_id.to_string()
    };

    let _creds_path = if credentials_path.is_empty() {
        env::var("GOOGLE_APPLICATION_CREDENTIALS").unwrap_or_default()
    } else {
        credentials_path.to_string()
    };

    if project.is_empty() {
        return Err(
            "GCP Project ID required. Set GCP_PROJECT_ID env var or configure in config.toml"
                .into(),
        );
    }

    // For now, return a placeholder implementation
    // In a full implementation, you would use the Google Cloud DLP client library
    // This would require adding google-cloud-dlp dependency

    // Basic heuristic check as fallback
    let has_pii = text.contains("@")
        || text.chars().filter(|c| c.is_numeric()).count() >= 10
        || text.to_lowercase().contains("ssn")
        || text.to_lowercase().contains("social security");

    Ok(has_pii)
}

#[allow(dead_code)]
pub async fn scan_with_aws_comprehend(
    text: &str,
    _region: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    // This would use the existing AWS Comprehend implementation
    // For now, return a placeholder
    let has_pii = text.contains("@") || text.chars().filter(|c| c.is_numeric()).count() >= 10;
    Ok(has_pii)
}
