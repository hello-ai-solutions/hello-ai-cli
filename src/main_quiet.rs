mod execute;
mod permissions;

use aws_config::Region;
use aws_sdk_bedrockruntime::Client as BedrockClient;
use colored::*;
use execute::ExecuteCommand;
use permissions::{PermissionChecker, RiskLevel};
use serde_json::json;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Minimal startup message
    println!("Q CLI (Sydney Region)");
    println!("Type 'quit' to exit.\n");

    // Initialize Bedrock client for Sydney region
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(Region::new("ap-southeast-2"))
        .load()
        .await;
    
    let client = BedrockClient::new(&config);
    
    // Initialize permission checker
    let permission_checker = PermissionChecker::default();
    
    // Conversation history
    let mut conversation_history = Vec::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == "quit" || input == "/quit" {
            println!("Goodbye!");
            break;
        }

        // Add user message to history
        conversation_history.push(json!({
            "role": "user",
            "content": input
        }));

        // Build Bedrock request with conversation history
        let request_body = json!({
            "anthropic_version": "bedrock-2023-05-31",
            "max_tokens": 4096,
            "messages": conversation_history,
            "system": "You are Amazon Q, an AI assistant built by AWS. Help users with their questions and provide practical, executable commands when appropriate."
        });

        // Call Bedrock
        match client
            .invoke_model()
            .model_id("apac.anthropic.claude-3-5-sonnet-20240620-v1:0")
            .content_type("application/json")
            .body(request_body.to_string().as_bytes().to_vec().into())
            .send()
            .await
        {
            Ok(response) => {
                let response_bytes = response.body().as_ref().to_vec();
                let response_body = match String::from_utf8(response_bytes) {
                    Ok(body) => body,
                    Err(e) => {
                        eprintln!("Error: UTF-8 conversion error: {}", e);
                        continue;
                    }
                };
                
                let parsed: serde_json::Value = match serde_json::from_str(&response_body) {
                    Ok(parsed) => parsed,
                    Err(e) => {
                        eprintln!("Error: JSON parsing error: {}", e);
                        continue;
                    }
                };
                
                if let Some(content) = parsed["content"].as_array() {
                    if let Some(text) = content.first().and_then(|c| c["text"].as_str()) {
                        // Add AI response to conversation history
                        conversation_history.push(json!({
                            "role": "assistant", 
                            "content": text
                        }));
                        
                        // Check if response contains tool calls
                        if let Some(commands) = extract_tool_calls(text) {
                            println!("{}", text);
                            println!("\nExecuting commands:");
                            
                            let mut confirm_all = false;
                            
                            for command_text in commands {
                                if let Some(cmd) = parse_execute_command(&command_text) {
                                    let risk_level = permission_checker.get_risk_level(&cmd.command);
                                    let requires_confirmation = permission_checker.requires_confirmation(&cmd.command);
                                    
                                    // Show command with risk indicator
                                    let risk_indicator = match risk_level {
                                        RiskLevel::Low => "🟢",
                                        RiskLevel::Medium => "🟡", 
                                        RiskLevel::High => "🔴",
                                    };
                                    
                                    println!("\n> {} {}", risk_indicator, cmd.command);
                                    
                                    // Ask for confirmation if needed
                                    if requires_confirmation && !confirm_all {
                                        print!("Confirm? (y/N/a for all): ");
                                        io::stdout().flush()?;
                                        
                                        let mut confirmation = String::new();
                                        io::stdin().read_line(&mut confirmation)?;
                                        let confirmation = confirmation.trim().to_lowercase();
                                        
                                        if confirmation.starts_with('a') {
                                            confirm_all = true;
                                        } else if !confirmation.starts_with('y') {
                                            println!("Skipped");
                                            continue;
                                        }
                                    }
                                    
                                    match cmd.execute().await {
                                        Ok(output) => {
                                            if !output.trim().is_empty() {
                                                println!("{}", output);
                                            }
                                        }
                                        Err(e) => {
                                            println!("Error: {}", e);
                                        }
                                    }
                                }
                            }
                        } else {
                            println!("{}", text);
                        }
                    } else {
                        println!("No response content found");
                    }
                } else {
                    println!("Invalid response format");
                }
            }
            Err(e) => {
                eprintln!("Error: service error");
                eprintln!("Details: {}", e);
            }
        }
        println!();
    }

    Ok(())
}

fn extract_tool_calls(text: &str) -> Option<Vec<String>> {
    let mut commands = Vec::new();
    
    // Look for code blocks with commands
    let lines: Vec<&str> = text.lines().collect();
    let mut in_code_block = false;
    
    for line in lines {
        let trimmed = line.trim();
        
        // Check for code block markers
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        
        // If we're in a code block or line looks like a command
        if in_code_block || trimmed.starts_with("kubectl ") || trimmed.starts_with("aws ") || 
           trimmed.starts_with("docker ") || trimmed.starts_with("ls ") || trimmed.starts_with("ps ") {
            if !trimmed.is_empty() && !trimmed.starts_with("#") {
                commands.push(trimmed.to_string());
            }
        }
    }
    
    if commands.is_empty() {
        None
    } else {
        Some(commands)
    }
}

fn parse_execute_command(command_text: &str) -> Option<ExecuteCommand> {
    let trimmed = command_text.trim();
    
    // Skip comments and empty lines
    if trimmed.is_empty() || trimmed.starts_with("#") {
        return None;
    }
    
    // Skip commands with placeholder values
    if trimmed.contains("<") && trimmed.contains(">") {
        return None;
    }
    
    // Accept any command that looks executable
    if trimmed.contains(" ") || trimmed.len() > 2 {
        Some(ExecuteCommand {
            command: trimmed.to_string(),
            summary: Some(format!("Execute: {}", trimmed)),
        })
    } else {
        None
    }
}
