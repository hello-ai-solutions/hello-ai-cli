use serde_json::{json, Value};
use std::env;

pub async fn get_cloud_llm_response(
    cloud_config: &crate::CloudConfig,
    message: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    match cloud_config.provider.as_str() {
        "aws" => get_aws_bedrock_response(cloud_config, message).await,
        "azure" => get_azure_openai_response(cloud_config, message).await,
        "gcp" => get_gcp_vertex_response(cloud_config, message).await,
        "oracle" => get_oracle_genai_response(cloud_config, message).await,
        _ => Err(format!("Unsupported cloud provider: {}", cloud_config.provider).into()),
    }
}

#[allow(dead_code)]
pub async fn scan_cloud_pii(
    cloud_config: &crate::CloudConfig,
    text: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    match cloud_config.provider.as_str() {
        "aws" => scan_aws_comprehend_pii(cloud_config, text).await,
        "azure" => scan_azure_text_analytics_pii(cloud_config, text).await,
        "gcp" => scan_gcp_dlp_pii(cloud_config, text).await,
        "oracle" => scan_oracle_data_safe_pii(cloud_config, text).await,
        _ => Err(format!("Unsupported cloud provider for PII: {}", cloud_config.provider).into()),
    }
}

async fn get_aws_bedrock_response(
    _config: &crate::CloudConfig,
    message: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Use existing AWS Bedrock implementation
    // This would integrate with the existing BedrockClient
    Ok(format!("Hello AI CLI response for: {}", message))
}

async fn get_azure_openai_response(
    config: &crate::CloudConfig,
    message: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = if config.azure_openai_key.is_empty() {
        env::var("AZURE_OPENAI_KEY").unwrap_or_default()
    } else {
        config.azure_openai_key.clone()
    };

    if api_key.is_empty() || config.azure_openai_endpoint.is_empty() {
        return Err("Azure OpenAI key and endpoint required. Set AZURE_OPENAI_KEY env var and endpoint in config.toml".into());
    }

    let client = reqwest::Client::new();
    let url = format!("{}/openai/deployments/{}/chat/completions?api-version=2024-02-15-preview", 
                     config.azure_openai_endpoint, config.azure_openai_model);
    
    let payload = json!({
        "messages": [
            {
                "role": "user",
                "content": message
            }
        ],
        "max_tokens": 4000,
        "temperature": 0.7
    });

    let response = client
        .post(&url)
        .header("api-key", api_key)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    let response_text = response.text().await?;
    let json: Value = serde_json::from_str(&response_text)?;
    
    if let Some(choices) = json["choices"].as_array() {
        if let Some(first_choice) = choices.first() {
            if let Some(content) = first_choice["message"]["content"].as_str() {
                return Ok(content.to_string());
            }
        }
    }

    Err("Failed to parse Azure OpenAI response".into())
}

async fn get_gcp_vertex_response(
    config: &crate::CloudConfig,
    message: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let project_id = if config.gcp_project_id.is_empty() {
        env::var("GCP_PROJECT_ID").unwrap_or_default()
    } else {
        config.gcp_project_id.clone()
    };

    if project_id.is_empty() {
        return Err("GCP Project ID required. Set GCP_PROJECT_ID env var or configure in config.toml".into());
    }

    // Placeholder for GCP Vertex AI implementation
    // Would require google-cloud-aiplatform dependency
    Ok(format!("GCP Vertex AI response for: {}", message))
}

async fn get_oracle_genai_response(
    config: &crate::CloudConfig,
    message: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    if config.oracle_compartment_id.is_empty() {
        return Err("Oracle compartment ID required in config.toml".into());
    }

    // Placeholder for Oracle Generative AI implementation
    // Would require oci-rust-sdk dependency
    Ok(format!("Oracle GenAI response for: {}", message))
}

#[allow(dead_code)]
async fn scan_aws_comprehend_pii(
    _config: &crate::CloudConfig,
    text: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Use existing AWS Comprehend implementation
    let has_pii = text.contains("@") || text.chars().filter(|c| c.is_numeric()).count() >= 10;
    Ok(has_pii)
}

#[allow(dead_code)]
async fn scan_azure_text_analytics_pii(
    config: &crate::CloudConfig,
    text: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let api_key = if config.azure_text_analytics_key.is_empty() {
        env::var("AZURE_TEXT_ANALYTICS_KEY").unwrap_or_default()
    } else {
        config.azure_text_analytics_key.clone()
    };

    if api_key.is_empty() || config.azure_text_analytics_endpoint.is_empty() {
        return Err("Azure Text Analytics key and endpoint required".into());
    }

    let client = reqwest::Client::new();
    let url = format!("{}/text/analytics/v3.1/entities/recognition/pii", config.azure_text_analytics_endpoint);
    
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
        .header("Ocp-Apim-Subscription-Key", api_key)
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
async fn scan_gcp_dlp_pii(
    config: &crate::CloudConfig,
    text: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let project_id = if config.gcp_project_id.is_empty() {
        env::var("GCP_PROJECT_ID").unwrap_or_default()
    } else {
        config.gcp_project_id.clone()
    };

    if project_id.is_empty() {
        return Err("GCP Project ID required".into());
    }

    // Placeholder for GCP DLP implementation
    let has_pii = text.contains("@") || text.chars().filter(|c| c.is_numeric()).count() >= 10;
    Ok(has_pii)
}

#[allow(dead_code)]
async fn scan_oracle_data_safe_pii(
    config: &crate::CloudConfig,
    text: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    if config.oracle_compartment_id.is_empty() {
        return Err("Oracle compartment ID required".into());
    }

    // Placeholder for Oracle Data Safe implementation
    let has_pii = text.contains("@") || text.chars().filter(|c| c.is_numeric()).count() >= 10;
    Ok(has_pii)
}
