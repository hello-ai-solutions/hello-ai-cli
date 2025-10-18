use aws_config;
use aws_sdk_bedrockruntime::Client as BedrockClient;
use serde_json::json;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Testing Bedrock streaming...");
    
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new("ap-southeast-2"))
        .load()
        .await;
    let client = BedrockClient::new(&config);
    
    let request_body = json!({
        "anthropic_version": "bedrock-2023-05-31",
        "max_tokens": 100,
        "messages": [{"role": "user", "content": "Say hello in exactly 5 words"}],
        "system": "You are a helpful assistant."
    });
    
    print!("🤖 ");
    io::stdout().flush().unwrap();
    
    match client
        .invoke_model_with_response_stream()
        .model_id("anthropic.claude-3-5-sonnet-20241022-v2:0")
        .content_type("application/json")
        .body(request_body.to_string().as_bytes().to_vec().into())
        .send()
        .await
    {
        Ok(mut response) => {
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
                                                    print!("{}", text_str);
                                                    io::stdout().flush().unwrap();
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("\n❌ Stream error: {}", e);
                        break;
                    }
                }
            }
            println!("\n✅ Streaming test complete!");
        }
        Err(e) => {
            println!("❌ Streaming failed: {}", e);
        }
    }
    
    Ok(())
}
