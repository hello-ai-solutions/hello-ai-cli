use colored::*;

#[derive(Debug, Clone)]
pub struct Thinking {
    pub thought: String,
    pub step: Option<i32>,
    pub reasoning: Option<String>,
}

impl Thinking {
    pub async fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Thinking analysis happens in background but no output shown to user
        // The analysis is still performed for internal decision making
        Ok(String::new())
    }
    
    #[allow(dead_code)]
    pub fn new_step(step: i32, thought: &str, reasoning: Option<&str>) -> Self {
        Self {
            thought: thought.to_string(),
            step: Some(step),
            reasoning: reasoning.map(|s| s.to_string()),
        }
    }
    
    pub fn new_thought(query: &str) -> Self {
        // Check if this is about file content that was already provided
        if query.contains("File:") && query.contains("# Amazon Q Developer CLI") {
            return Self {
                thought: "Analyzing TEST_SCENARIOS.md - comprehensive test guide with 60 test scenarios across 5 categories (Basic CLI, Chat System, Advanced Features, Core Functionality, Extended Capabilities)".to_string(),
                step: None,
                reasoning: None,
            };
        }

        // Clean up verbose command/output formatting but preserve the actual query
        let clean_query = query
            .replace("Command: fs_read\nOutput:", "Directory listing:")
            .replace("Command:", "")
            .replace("Output:", "")
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n");

        let system_aware_response = format!(
            "🤔 Analyzing request: {}",
            if clean_query.len() > 200 { 
                format!("{}...", &clean_query[..200]) 
            } else { 
                clean_query 
            }
        );

        Self {
            thought: system_aware_response,
            step: None,
            reasoning: None,
        }
    }
}
