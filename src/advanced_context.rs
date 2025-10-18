use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceContext {
    pub workspace_id: String,
    pub project_type: String,
    pub active_files: Vec<String>,
    pub recent_changes: Vec<FileChange>,
    pub conversation_threads: HashMap<String, ConversationThread>,
    pub project_config: ProjectConfig,
    pub session_data: SessionData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub file_path: String,
    pub change_type: String,
    pub timestamp: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationThread {
    pub thread_id: String,
    pub topic: String,
    pub messages: Vec<ContextMessage>,
    pub created_at: String,
    pub last_updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMessage {
    pub role: String,
    pub content: String,
    pub timestamp: String,
    pub file_references: Vec<String>,
    pub command_used: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub build_commands: Vec<String>,
    pub test_commands: Vec<String>,
    pub custom_settings: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub session_id: String,
    pub start_time: String,
    pub current_directory: String,
    pub environment_vars: HashMap<String, String>,
    pub git_branch: Option<String>,
    pub git_status: Option<String>,
}

pub struct AdvancedContextManager {
    context_dir: String,
    current_workspace: Option<String>,
    memory_limit: usize,
}

impl AdvancedContextManager {
    pub fn new() -> Self {
        let context_dir = ".amazonq/workspace_contexts".to_string();
        fs::create_dir_all(&context_dir).ok();

        Self {
            context_dir,
            current_workspace: None,
            memory_limit: 1000, // Max messages to keep in memory
        }
    }

    pub async fn initialize_workspace(
        &mut self,
        workspace_path: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let workspace_id = self.generate_workspace_id(workspace_path);
        self.current_workspace = Some(workspace_id.clone());

        let mut context = self
            .load_or_create_workspace(&workspace_id, workspace_path)
            .await?;
        context.session_data = self.create_session_data(workspace_path).await?;

        self.save_workspace_context(&context).await?;
        Ok(workspace_id)
    }

    fn generate_workspace_id(&self, workspace_path: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        workspace_path.hash(&mut hasher);
        format!("ws_{:x}", hasher.finish())
    }

    async fn load_or_create_workspace(
        &self,
        workspace_id: &str,
        workspace_path: &str,
    ) -> Result<WorkspaceContext, Box<dyn std::error::Error>> {
        let context_path = format!("{}/{}.json", self.context_dir, workspace_id);

        if Path::new(&context_path).exists() {
            let content = fs::read_to_string(&context_path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(self
                .create_default_workspace(workspace_id, workspace_path)
                .await?)
        }
    }

    async fn create_default_workspace(
        &self,
        workspace_id: &str,
        workspace_path: &str,
    ) -> Result<WorkspaceContext, Box<dyn std::error::Error>> {
        let project_config = self.detect_project_config(workspace_path).await?;

        Ok(WorkspaceContext {
            workspace_id: workspace_id.to_string(),
            project_type: project_config.name.clone(),
            active_files: Vec::new(),
            recent_changes: Vec::new(),
            conversation_threads: HashMap::new(),
            project_config,
            session_data: SessionData {
                session_id: uuid::Uuid::new_v4().to_string(),
                start_time: chrono::Utc::now().to_rfc3339(),
                current_directory: workspace_path.to_string(),
                environment_vars: HashMap::new(),
                git_branch: None,
                git_status: None,
            },
        })
    }

    async fn detect_project_config(
        &self,
        workspace_path: &str,
    ) -> Result<ProjectConfig, Box<dyn std::error::Error>> {
        let root = Path::new(workspace_path);

        // Rust project
        if let Ok(cargo_content) = fs::read_to_string(root.join("Cargo.toml")) {
            return Ok(self.parse_cargo_config(&cargo_content));
        }

        // Node.js project
        if let Ok(package_content) = fs::read_to_string(root.join("package.json")) {
            return Ok(self.parse_package_json(&package_content));
        }

        // Python project
        if root.join("requirements.txt").exists() || root.join("setup.py").exists() {
            return Ok(ProjectConfig {
                name: "Python Project".to_string(),
                version: "unknown".to_string(),
                dependencies: Vec::new(),
                build_commands: vec!["python setup.py build".to_string()],
                test_commands: vec!["pytest".to_string(), "python -m unittest".to_string()],
                custom_settings: HashMap::new(),
            });
        }

        // Default
        Ok(ProjectConfig {
            name: "Unknown Project".to_string(),
            version: "unknown".to_string(),
            dependencies: Vec::new(),
            build_commands: Vec::new(),
            test_commands: Vec::new(),
            custom_settings: HashMap::new(),
        })
    }

    fn parse_cargo_config(&self, content: &str) -> ProjectConfig {
        let mut name = "Rust Project".to_string();
        let mut version = "unknown".to_string();
        let mut dependencies = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("name = ") {
                name = trimmed
                    .split('=')
                    .nth(1)
                    .unwrap_or("")
                    .trim()
                    .trim_matches('"')
                    .to_string();
            } else if trimmed.starts_with("version = ") {
                version = trimmed
                    .split('=')
                    .nth(1)
                    .unwrap_or("")
                    .trim()
                    .trim_matches('"')
                    .to_string();
            } else if trimmed.contains('=') && !trimmed.starts_with('[') {
                let dep_name = trimmed.split('=').next().unwrap_or("").trim();
                if !dep_name.is_empty() && dep_name != "name" && dep_name != "version" {
                    dependencies.push(dep_name.to_string());
                }
            }
        }

        ProjectConfig {
            name,
            version,
            dependencies,
            build_commands: vec!["cargo build".to_string()],
            test_commands: vec!["cargo test".to_string()],
            custom_settings: HashMap::new(),
        }
    }

    fn parse_package_json(&self, content: &str) -> ProjectConfig {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            let name = json
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("Node.js Project")
                .to_string();
            let version = json
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();

            let mut dependencies = Vec::new();
            if let Some(deps) = json.get("dependencies").and_then(|v| v.as_object()) {
                dependencies.extend(deps.keys().cloned());
            }

            ProjectConfig {
                name,
                version,
                dependencies,
                build_commands: vec!["npm run build".to_string()],
                test_commands: vec!["npm test".to_string()],
                custom_settings: HashMap::new(),
            }
        } else {
            ProjectConfig {
                name: "Node.js Project".to_string(),
                version: "unknown".to_string(),
                dependencies: Vec::new(),
                build_commands: vec!["npm run build".to_string()],
                test_commands: vec!["npm test".to_string()],
                custom_settings: HashMap::new(),
            }
        }
    }

    async fn create_session_data(
        &self,
        workspace_path: &str,
    ) -> Result<SessionData, Box<dyn std::error::Error>> {
        let git_branch = self.get_git_branch(workspace_path).await.ok();
        let git_status = self.get_git_status(workspace_path).await.ok();

        Ok(SessionData {
            session_id: uuid::Uuid::new_v4().to_string(),
            start_time: chrono::Utc::now().to_rfc3339(),
            current_directory: workspace_path.to_string(),
            environment_vars: std::env::vars().collect(),
            git_branch,
            git_status,
        })
    }

    async fn get_git_branch(
        &self,
        workspace_path: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let output = std::process::Command::new("git")
            .args(&["branch", "--show-current"])
            .current_dir(workspace_path)
            .output()?;

        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    async fn get_git_status(
        &self,
        workspace_path: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let output = std::process::Command::new("git")
            .args(&["status", "--porcelain"])
            .current_dir(workspace_path)
            .output()?;

        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    #[allow(dead_code)]
    pub async fn add_conversation_message(
        &mut self,
        role: &str,
        content: &str,
        file_refs: Vec<String>,
        command: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(workspace_id) = &self.current_workspace {
            let mut context = self.load_or_create_workspace(workspace_id, "").await?;

            let thread_id = "main".to_string(); // Default thread
            let thread = context
                .conversation_threads
                .entry(thread_id.clone())
                .or_insert_with(|| ConversationThread {
                    thread_id: thread_id.clone(),
                    topic: "General Discussion".to_string(),
                    messages: Vec::new(),
                    created_at: chrono::Utc::now().to_rfc3339(),
                    last_updated: chrono::Utc::now().to_rfc3339(),
                });

            thread.messages.push(ContextMessage {
                role: role.to_string(),
                content: content.to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                file_references: file_refs,
                command_used: command,
            });

            thread.last_updated = chrono::Utc::now().to_rfc3339();

            // Limit memory usage
            if thread.messages.len() > self.memory_limit {
                thread
                    .messages
                    .drain(0..thread.messages.len() - self.memory_limit);
            }

            self.save_workspace_context(&context).await?;
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn get_relevant_context(
        &self,
        query: &str,
        max_messages: usize,
    ) -> Result<Vec<ContextMessage>, Box<dyn std::error::Error>> {
        if let Some(workspace_id) = &self.current_workspace {
            let context = self.load_or_create_workspace(workspace_id, "").await?;

            let mut relevant_messages = Vec::new();

            for thread in context.conversation_threads.values() {
                for message in &thread.messages {
                    if message
                        .content
                        .to_lowercase()
                        .contains(&query.to_lowercase())
                        || message.file_references.iter().any(|f| query.contains(f))
                    {
                        relevant_messages.push(message.clone());
                    }
                }
            }

            // Sort by timestamp and limit
            relevant_messages.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            relevant_messages.truncate(max_messages);

            Ok(relevant_messages)
        } else {
            Ok(Vec::new())
        }
    }

    #[allow(dead_code)]
    pub async fn track_file_change(
        &mut self,
        file_path: &str,
        change_type: &str,
        summary: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(workspace_id) = &self.current_workspace {
            let mut context = self.load_or_create_workspace(workspace_id, "").await?;

            context.recent_changes.push(FileChange {
                file_path: file_path.to_string(),
                change_type: change_type.to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                summary: summary.to_string(),
            });

            // Keep only recent changes (last 100)
            if context.recent_changes.len() > 100 {
                context
                    .recent_changes
                    .drain(0..context.recent_changes.len() - 100);
            }

            self.save_workspace_context(&context).await?;
        }

        Ok(())
    }

    pub async fn get_workspace_summary(&self) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(workspace_id) = &self.current_workspace {
            let context = self.load_or_create_workspace(workspace_id, "").await?;

            let total_messages: usize = context
                .conversation_threads
                .values()
                .map(|t| t.messages.len())
                .sum();

            let recent_files: Vec<String> = context
                .recent_changes
                .iter()
                .take(5)
                .map(|c| c.file_path.clone())
                .collect();

            Ok(format!(
                "# 🏗️ Workspace Context\n\n\
                ## Project Information\n\
                - **Name**: {}\n\
                - **Type**: {}\n\
                - **Version**: {}\n\
                - **Dependencies**: {}\n\n\
                ## Session Data\n\
                - **Directory**: {}\n\
                - **Git Branch**: {}\n\
                - **Session ID**: {}\n\n\
                ## Conversation History\n\
                - **Total Messages**: {}\n\
                - **Active Threads**: {}\n\
                - **Recent Changes**: {}\n\n\
                ## Recent Files\n{}\n",
                context.project_config.name,
                context.project_type,
                context.project_config.version,
                context.project_config.dependencies.len(),
                context.session_data.current_directory,
                context.session_data.git_branch.as_deref().unwrap_or("none"),
                context.session_data.session_id,
                total_messages,
                context.conversation_threads.len(),
                context.recent_changes.len(),
                recent_files
                    .iter()
                    .map(|f| format!("- {}", f))
                    .collect::<Vec<_>>()
                    .join("\n")
            ))
        } else {
            Ok("No active workspace".to_string())
        }
    }

    async fn save_workspace_context(
        &self,
        context: &WorkspaceContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let context_path = format!("{}/{}.json", self.context_dir, context.workspace_id);
        let content = serde_json::to_string_pretty(context)?;
        fs::write(&context_path, content)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn cleanup_old_contexts(
        &self,
        days_old: u64,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let mut cleaned = 0;
        let cutoff = chrono::Utc::now() - chrono::Duration::days(days_old as i64);

        if let Ok(entries) = fs::read_dir(&self.context_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            let modified_time = chrono::DateTime::<chrono::Utc>::from(modified);
                            if modified_time < cutoff {
                                fs::remove_file(entry.path()).ok();
                                cleaned += 1;
                            }
                        }
                    }
                }
            }
        }

        Ok(cleaned)
    }
}
