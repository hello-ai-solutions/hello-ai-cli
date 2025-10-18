use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct McpServer {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct McpManager {
    pub servers: HashMap<String, McpServer>,
}

impl McpManager {
    pub fn new() -> Self {
        Self {
            servers: HashMap::new(),
        }
    }

    pub async fn load_servers(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mcp_config = ".amazonq/mcp-servers.json";
        if std::path::Path::new(mcp_config).exists() {
            let data = fs::read_to_string(mcp_config)?;
            let config: Value = serde_json::from_str(&data)?;

            if let Some(servers) = config["servers"].as_object() {
                for (name, server_config) in servers {
                    if let (Some(command), Some(args)) = (
                        server_config["command"].as_str(),
                        server_config["args"].as_array(),
                    ) {
                        let server = McpServer {
                            name: name.clone(),
                            command: command.to_string(),
                            args: args
                                .iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect(),
                            enabled: server_config["enabled"].as_bool().unwrap_or(true),
                        };
                        self.servers.insert(name.clone(), server);
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn list_servers(&self) -> String {
        if self.servers.is_empty() {
            "No MCP servers configured".to_string()
        } else {
            let mut output = Vec::new();
            output.push("Configured MCP servers:".to_string());
            for (name, server) in &self.servers {
                let status = if server.enabled { "ON" } else { "OFF" };
                output.push(format!("  • {} [{}]: {}", name, status, server.command));
            }
            output.join("\n")
        }
    }

    #[allow(dead_code)]
    pub async fn add_server(
        &mut self,
        name: String,
        command: String,
        args: Vec<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let server = McpServer {
            name: name.clone(),
            command,
            args,
            enabled: true,
        };

        self.servers.insert(name.clone(), server);
        self.save_config().await?;
        Ok(format!("Added MCP server: {}", name))
    }

    #[allow(dead_code)]
    async fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mcp_dir = ".amazonq";
        fs::create_dir_all(mcp_dir)?;

        let mut servers_config = json!({});
        for (name, server) in &self.servers {
            servers_config["servers"][name] = json!({
                "command": server.command,
                "args": server.args,
                "enabled": server.enabled
            });
        }

        let config_file = format!("{}/mcp-servers.json", mcp_dir);
        fs::write(config_file, serde_json::to_string_pretty(&servers_config)?)?;
        Ok(())
    }
}
