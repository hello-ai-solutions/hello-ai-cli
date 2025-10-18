use aws_sdk_bedrockruntime::Client as BedrockClient;
use colored::*;
use serde_json::json;
use std::io::{self, Write};

fn filter_response_text(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut filtered_lines = Vec::new();
    let mut skip_yaml = false;
    
    for line in lines {
        if line.trim().starts_with("---") {
            skip_yaml = true;
            continue;
        }
        if skip_yaml && (line.trim().starts_with("...") || line.trim().is_empty()) {
            skip_yaml = false;
            continue;
        }
        if skip_yaml {
            continue;
        }
        
        if line.contains("Available tools:") || 
           line.contains("fs_read") || 
           line.contains("fs_write") || 
           line.contains("execute_bash") ||
           line.contains("intent:") ||
           line.contains("project_type:") ||
           line.contains("create:") ||
           line.contains("run:") {
            continue;
        }
        
        filtered_lines.push(line);
    }
    
    filtered_lines.join("\n")
}

pub struct StreamingClient {
    provider: String,
    bedrock_client: Option<BedrockClient>,
    endpoint: Option<String>,
    model: String,
}

impl StreamingClient {
    pub fn new_local(endpoint: String, model: String) -> Self {
        Self {
            provider: "local".to_string(),
            bedrock_client: None,
            endpoint: Some(endpoint),
            model,
        }
    }
    
    pub fn new_bedrock(client: BedrockClient, model: String) -> Self {
        Self {
            provider: "bedrock".to_string(),
            bedrock_client: Some(client),
            endpoint: None,
            model,
        }
    }
    
    pub async fn stream_response(&self, conversation_history: &[serde_json::Value], system_prompt: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
        match self.provider.as_str() {
            "local" => self.stream_local(conversation_history, system_prompt).await,
            "bedrock" => self.stream_bedrock(conversation_history, system_prompt).await,
            _ => Err("Unsupported provider".into()),
        }
    }
    
    async fn stream_local(&self, conversation_history: &[serde_json::Value], system_prompt: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
        let default_system = "You are Hello AI CLI, an AI assistant.";
        let mut context = system_prompt.unwrap_or(default_system).to_string();
        
        for msg in conversation_history {
            if let Some(role) = msg.get("role").and_then(|r| r.as_str()) {
                if let Some(content) = msg.get("content").and_then(|c| c.as_str()) {
                    context.push_str(&format!("\n\n{}: {}", role, content));
                }
            }
        }
        
        let request_body = json!({
            "model": self.model,
            "prompt": context,
            "stream": false
        });

        let client = reqwest::Client::new();
        let response = client
            .post(self.endpoint.as_ref().unwrap())
            .json(&request_body)
            .send()
            .await?;
            
        let response_text = response.text().await?;
        let parsed: serde_json::Value = serde_json::from_str(&response_text)?;
        
        if let Some(response_content) = parsed["response"].as_str() {
            let filtered = filter_response_text(response_content);
            if !filtered.is_empty() {
                print!("{}", filtered.green());
                io::stdout().flush().unwrap();
            }
            Ok(response_content.to_string())
        } else {
            Err("Invalid response format from local LLM".into())
        }
    }
    
    async fn stream_bedrock(&self, conversation_history: &[serde_json::Value], system_prompt: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
        let default_system = "You are Hello AI CLI, an AI assistant.";
        
        let request_body = json!({
            "anthropic_version": "bedrock-2023-05-31",
            "max_tokens": 4096,
            "messages": conversation_history,
            "system": system_prompt.unwrap_or(default_system)
        });

        let client = self.bedrock_client.as_ref().ok_or("Bedrock client not initialized")?;
        
        match client
            .invoke_model_with_response_stream()
            .model_id(&self.model)
            .content_type("application/json")
            .body(request_body.to_string().as_bytes().to_vec().into())
            .send()
            .await
        {
            Ok(mut response) => {
                let mut full_response = String::new();
                
                while let Some(event) = response.body.recv().await.transpose() {
                    match event {
                        Ok(event) => {
                            if let Ok(chunk) = event.as_chunk() {
                                if let Some(bytes) = chunk.bytes() {
                                    if let Ok(chunk_str) = std::str::from_utf8(bytes.as_ref()) {
                                        if let Ok(chunk_json) = serde_json::from_str::<serde_json::Value>(chunk_str) {
                                            if let Some(delta) = chunk_json.get("delta") {
                                                if let Some(text) = delta.get("text") {
                                                    if let Some(text_str) = text.as_str() {
                                                        let filtered_text = filter_response_text(text_str);
                                                        if !filtered_text.is_empty() {
                                                            print!("{}", filtered_text.green());
                                                            io::stdout().flush().unwrap();
                                                        }
                                                        full_response.push_str(text_str);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => break,
                    }
                }
                
                Ok(full_response)
            }
            Err(e) => Err(Box::new(e))
        }
    }
}
