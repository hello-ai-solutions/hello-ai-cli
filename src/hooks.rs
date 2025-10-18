#![allow(dead_code)]
use serde_json::Value;
use std::fs;
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Hook {
    pub name: String,
    pub command: String,
    pub event: String,
    pub matcher: Option<String>,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone)]
pub struct HooksManager {
    pub hooks: Vec<Hook>,
}

impl HooksManager {
    pub fn new() -> Self {
        Self {
            hooks: Vec::new(),
        }
    }
    
    pub async fn load_hooks(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let hooks_config = ".amazonq/hooks.json";
        if std::path::Path::new(hooks_config).exists() {
            let data = fs::read_to_string(hooks_config)?;
            let config: Value = serde_json::from_str(&data)?;
            
            if let Some(hooks) = config["hooks"].as_array() {
                for hook_config in hooks {
                    if let (Some(name), Some(command), Some(event)) = (
                        hook_config["name"].as_str(),
                        hook_config["command"].as_str(),
                        hook_config["event"].as_str()
                    ) {
                        let hook = Hook {
                            name: name.to_string(),
                            command: command.to_string(),
                            event: event.to_string(),
                            matcher: hook_config["matcher"].as_str().map(|s| s.to_string()),
                            timeout_ms: hook_config["timeout_ms"].as_u64().unwrap_or(30000),
                        };
                        self.hooks.push(hook);
                    }
                }
            }
        }
        Ok(())
    }
    
    pub async fn execute_hooks(&self, event: &str, context: &Value) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        
        for hook in &self.hooks {
            if hook.event == event {
                match self.execute_hook(hook, context).await {
                    Ok(output) => results.push(format!("Hook '{}': {}", hook.name, output)),
                    Err(e) => results.push(format!("Hook '{}' failed: {}", hook.name, e)),
                }
            }
        }
        
        Ok(results)
    }
    
    async fn execute_hook(&self, hook: &Hook, context: &Value) -> Result<String, Box<dyn std::error::Error>> {
        let mut cmd = if cfg!(target_os = "windows") {
            let mut cmd = Command::new("powershell");
            cmd.arg("-Command").arg(&hook.command);
            cmd
        } else {
            let mut cmd = Command::new("sh");
            cmd.arg("-c").arg(&hook.command);
            cmd
        };
        
        cmd.stdin(Stdio::piped())
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());
        
        let mut child = cmd.spawn()?;
        
        // Send context as JSON to stdin
        if let Some(stdin) = child.stdin.as_mut() {
            use std::io::Write;
            let context_json = serde_json::to_string(context)?;
            stdin.write_all(context_json.as_bytes())?;
        }
        
        let output = child.wait_with_output()?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(format!("Hook failed: {}", String::from_utf8_lossy(&output.stderr)).into())
        }
    }
    
    pub async fn list_hooks(&self) -> String {
        if self.hooks.is_empty() {
            "No hooks configured".to_string()
        } else {
            let mut output = Vec::new();
            output.push("Configured hooks:".to_string());
            for hook in &self.hooks {
                output.push(format!("  • {} [{}]: {}", hook.name, hook.event, hook.command));
            }
            output.join("\n")
        }
    }
}
