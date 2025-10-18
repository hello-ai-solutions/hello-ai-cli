use aws_config::Region;
use aws_sdk_bedrockruntime::Client as BedrockClient;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

pub async fn start_web_server() -> Result<(), Box<dyn std::error::Error>> {
    let conversation_history = Arc::new(Mutex::new(Vec::<serde_json::Value>::new()));

    // Load config and initialize Bedrock client
    let app_config = crate::load_config();
    let mut aws_config_builder = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(Region::new(app_config.cloud.aws_region.clone()));

    if !app_config.cloud.aws_profile.is_empty() {
        aws_config_builder = aws_config_builder.profile_name(app_config.cloud.aws_profile.clone());
    }

    let config = aws_config_builder.load().await;
    let client = Arc::new(BedrockClient::new(&config));

    // Serve static HTML
    let index = warp::path::end().map(|| warp::reply::html(include_str!("../web/index.html")));

    // API endpoint for chat with real LLM
    let chat_history = conversation_history.clone();
    let bedrock_client = client.clone();
    let chat = warp::path("api")
        .and(warp::path("chat"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |message: serde_json::Value| {
            let history = chat_history.clone();
            let client = bedrock_client.clone();
            async move {
                let mut hist = history.lock().await;
                hist.push(message.clone());

                // Get AI response using Bedrock
                let user_input = message["content"].as_str().unwrap_or("");
                println!("Processing message: {}", user_input);
                let ai_response = match get_ai_response(&client, user_input, &hist).await {
                    Ok(response) => {
                        println!("AI response received successfully");
                        response
                    }
                    Err(e) => {
                        println!("Error getting AI response: {}", e);
                        format!("Error: {}", e)
                    }
                };

                let response = json!({
                    "role": "assistant",
                    "content": ai_response
                });
                hist.push(response.clone());

                Ok::<_, warp::Rejection>(warp::reply::json(&response))
            }
        });

    // Get conversation history
    let get_history = warp::path("api")
        .and(warp::path("history"))
        .and(warp::get())
        .and_then(move || {
            let history = conversation_history.clone();
            async move {
                let hist = history.lock().await;
                Ok::<_, warp::Rejection>(warp::reply::json(&*hist))
            }
        });

    let routes = index.or(chat).or(get_history);

    println!("🌐 Web GUI starting at http://localhost:3031");
    println!("📱 Open your browser to access the Hello AI web interface");

    warp::serve(routes).run(([127, 0, 0, 1], 3031)).await;

    Ok(())
}

async fn get_ai_response(
    client: &BedrockClient,
    user_input: &str,
    history: &[serde_json::Value],
) -> Result<String, Box<dyn std::error::Error>> {
    // Build conversation context
    let mut messages = Vec::new();
    for msg in history.iter().rev().take(10).rev() {
        // Last 10 messages for context
        if let (Some(role), Some(content)) = (msg["role"].as_str(), msg["content"].as_str()) {
            messages.push(json!({
                "role": role,
                "content": content
            }));
        }
    }

    // Add current user message
    messages.push(json!({
        "role": "user",
        "content": user_input
    }));

    let request_body = json!({
        "anthropic_version": "bedrock-2023-05-31",
        "max_tokens": 4000,
        "messages": messages,
        "system": "You are Hello AI, a helpful AI assistant. Provide concise, accurate responses."
    });

    println!(
        "Sending request to Bedrock with {} messages",
        messages.len()
    );

    let response = client
        .invoke_model()
        .model_id("anthropic.claude-3-5-sonnet-20241022-v2:0")
        .content_type("application/json")
        .body(aws_sdk_bedrockruntime::primitives::Blob::new(
            request_body.to_string(),
        ))
        .send()
        .await
        .map_err(|e| {
            println!("Bedrock API error: {}", e);
            e
        })?;

    let response_body = response.body().as_ref();
    let response_json: serde_json::Value = serde_json::from_slice(response_body)?;

    if let Some(content) = response_json["content"][0]["text"].as_str() {
        Ok(content.to_string())
    } else {
        Ok("Sorry, I couldn't generate a response.".to_string())
    }
}
