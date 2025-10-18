use serde_json::{json, Value};
use std::env;

pub async fn get_openai_response(
    api_key: &str,
    model: &str,
    endpoint: &str,
    message: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let key = if api_key.is_empty() {
        env::var("OPENAI_API_KEY").unwrap_or_default()
    } else {
        api_key.to_string()
    };

    if key.is_empty() {
        return Err("OpenAI API key not found. Set OPENAI_API_KEY environment variable or configure in config.toml".into());
    }

    let client = reqwest::Client::new();
    let payload = json!({
        "model": model,
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
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", key))
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

    Err("Failed to parse OpenAI response".into())
}

pub async fn get_gemini_response(
    api_key: &str,
    model: &str,
    endpoint: &str,
    message: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let key = if api_key.is_empty() {
        env::var("GEMINI_API_KEY").unwrap_or_default()
    } else {
        api_key.to_string()
    };

    if key.is_empty() {
        return Err("Gemini API key not found. Set GEMINI_API_KEY environment variable or configure in config.toml".into());
    }

    let client = reqwest::Client::new();
    let url = format!("{}/{}:generateContent?key={}", endpoint, model, key);

    let payload = json!({
        "contents": [{
            "parts": [{
                "text": message
            }]
        }],
        "generationConfig": {
            "temperature": 0.7,
            "maxOutputTokens": 4000
        }
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    let response_text = response.text().await?;
    let json: Value = serde_json::from_str(&response_text)?;

    if let Some(candidates) = json["candidates"].as_array() {
        if let Some(first_candidate) = candidates.first() {
            if let Some(content) = first_candidate["content"]["parts"].as_array() {
                if let Some(first_part) = content.first() {
                    if let Some(text) = first_part["text"].as_str() {
                        return Ok(text.to_string());
                    }
                }
            }
        }
    }

    Err("Failed to parse Gemini response".into())
}
