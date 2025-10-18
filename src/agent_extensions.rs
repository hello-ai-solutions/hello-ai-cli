use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionConfig {
    pub name: String,
    pub enabled: bool,
    pub config: HashMap<String, String>,
}

#[async_trait]
pub trait AgentExtension: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn execute(&self, command: &str, args: &[String]) -> Result<String, Box<dyn std::error::Error>>;
    fn commands(&self) -> Vec<String>;
}

pub struct ExtensionManager {
    extensions: HashMap<String, Box<dyn AgentExtension>>,
}

impl ExtensionManager {
    pub fn new() -> Self {
        let mut manager = Self {
            extensions: HashMap::new(),
        };
        
        // Register built-in extensions
        manager.register_extension(Box::new(NewRelicExtension::new()));
        manager.register_extension(Box::new(WizExtension::new()));
        manager.register_extension(Box::new(PostgreSQLExtension::new()));
        
        manager
    }
    
    pub fn register_extension(&mut self, extension: Box<dyn AgentExtension>) {
        let name = extension.name().to_string();
        self.extensions.insert(name, extension);
    }
    
    pub async fn execute_command(&self, extension_name: &str, command: &str, args: &[String]) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(extension) = self.extensions.get(extension_name) {
            extension.execute(command, args).await
        } else {
            Err(format!("Extension '{}' not found", extension_name).into())
        }
    }
    
    pub fn list_extensions(&self) -> Vec<(&str, &str)> {
        self.extensions.iter()
            .map(|(name, ext)| (name.as_str(), ext.description()))
            .collect()
    }
    
    pub fn get_commands(&self, extension_name: &str) -> Option<Vec<String>> {
        self.extensions.get(extension_name).map(|ext| ext.commands())
    }
}

// NewRelic Extension
pub struct NewRelicExtension {
    api_key: Option<String>,
}

impl NewRelicExtension {
    pub fn new() -> Self {
        Self {
            api_key: std::env::var("NEW_RELIC_API_KEY").ok(),
        }
    }
}

#[async_trait]
impl AgentExtension for NewRelicExtension {
    fn name(&self) -> &str { "newrelic" }
    
    fn description(&self) -> &str { "NewRelic monitoring and alerting integration" }
    
    fn commands(&self) -> Vec<String> {
        vec![
            "alerts".to_string(),
            "metrics".to_string(),
            "apps".to_string(),
            "deploy".to_string(),
        ]
    }
    
    async fn execute(&self, command: &str, args: &[String]) -> Result<String, Box<dyn std::error::Error>> {
        match command {
            "alerts" => Ok("📊 NewRelic Alerts: Fetching current alerts...".to_string()),
            "metrics" => Ok(format!("📈 NewRelic Metrics for: {}", args.get(0).unwrap_or(&"all".to_string()))),
            "apps" => Ok("🚀 NewRelic Applications: Listing monitored apps...".to_string()),
            "deploy" => Ok(format!("🔄 NewRelic Deployment marker created for: {}", args.get(0).unwrap_or(&"unknown".to_string()))),
            _ => Err(format!("Unknown NewRelic command: {}", command).into()),
        }
    }
}

// Wiz Security Extension
pub struct WizExtension;

impl WizExtension {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl AgentExtension for WizExtension {
    fn name(&self) -> &str { "wiz" }
    
    fn description(&self) -> &str { "Wiz cloud security scanning and compliance" }
    
    fn commands(&self) -> Vec<String> {
        vec![
            "scan".to_string(),
            "compliance".to_string(),
            "vulnerabilities".to_string(),
            "policies".to_string(),
        ]
    }
    
    async fn execute(&self, command: &str, args: &[String]) -> Result<String, Box<dyn std::error::Error>> {
        match command {
            "scan" => Ok(format!("🔍 Wiz Security Scan initiated for: {}", args.get(0).unwrap_or(&"current-project".to_string()))),
            "compliance" => Ok("📋 Wiz Compliance: Checking compliance status...".to_string()),
            "vulnerabilities" => Ok("🚨 Wiz Vulnerabilities: Scanning for security issues...".to_string()),
            "policies" => Ok("📜 Wiz Policies: Reviewing security policies...".to_string()),
            _ => Err(format!("Unknown Wiz command: {}", command).into()),
        }
    }
}

// PostgreSQL Extension
pub struct PostgreSQLExtension;

impl PostgreSQLExtension {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl AgentExtension for PostgreSQLExtension {
    fn name(&self) -> &str { "postgres" }
    
    fn description(&self) -> &str { "PostgreSQL database management and operations" }
    
    fn commands(&self) -> Vec<String> {
        vec![
            "query".to_string(),
            "backup".to_string(),
            "restore".to_string(),
            "analyze".to_string(),
            "vacuum".to_string(),
        ]
    }
    
    async fn execute(&self, command: &str, args: &[String]) -> Result<String, Box<dyn std::error::Error>> {
        match command {
            "query" => Ok(format!("🔍 PostgreSQL Query: {}", args.get(0).unwrap_or(&"SELECT version();".to_string()))),
            "backup" => Ok(format!("💾 PostgreSQL Backup: Creating backup for database '{}'", args.get(0).unwrap_or(&"default".to_string()))),
            "restore" => Ok(format!("🔄 PostgreSQL Restore: Restoring from backup '{}'", args.get(0).unwrap_or(&"latest".to_string()))),
            "analyze" => Ok("📊 PostgreSQL Analyze: Running table statistics analysis...".to_string()),
            "vacuum" => Ok("🧹 PostgreSQL Vacuum: Cleaning up database...".to_string()),
            _ => Err(format!("Unknown PostgreSQL command: {}", command).into()),
        }
    }
}
