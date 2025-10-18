use colored::*;
use serde::Deserialize;
use std::time::Duration;
use tokio::process::Command as AsyncCommand;
use tokio::time::timeout;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct ExecuteCommand {
    pub command: String,
    pub summary: Option<String>,
}

impl ExecuteCommand {
    pub async fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Check for Docker commands when Docker daemon might not be running
        if self.command.starts_with("docker ") {
            let docker_check = if cfg!(target_os = "windows") {
                AsyncCommand::new("cmd")
                    .args(["/C", "docker info"])
                    .output()
                    .await
            } else {
                AsyncCommand::new("sh")
                    .arg("-c")
                    .arg("docker info")
                    .output()
                    .await
            };

            if let Ok(output) = docker_check {
                if !output.status.success() {
                    return Err(
                        "Docker daemon not running. Please start Docker Desktop and try again."
                            .into(),
                    );
                }
            }
        }

        // Check for interactive commands that shouldn't be executed directly
        let interactive_commands = [
            "pwsh",
            "powershell",
            "python",
            "node",
            "irb",
            "mysql",
            "psql",
            "mongo",
            "redis-cli",
        ];
        let command_parts: Vec<&str> = self.command.split_whitespace().collect();

        if let Some(first_command) = command_parts.first() {
            if interactive_commands.contains(first_command) && command_parts.len() == 1 {
                return Ok(format!("⚠️ Interactive command '{}' detected. This would start an interactive session. Use with specific arguments or scripts instead.", first_command));
            }
        }

        let command_future = if cfg!(target_os = "windows") {
            AsyncCommand::new("cmd")
                .args(["/C", &self.command])
                .output()
        } else {
            AsyncCommand::new("sh").args(["-c", &self.command]).output()
        };

        // Apply 2-minute timeout for all commands to prevent hanging
        let output = match timeout(Duration::from_secs(120), command_future).await {
            Ok(result) => result?,
            Err(_) => {
                return Ok("⏱️ Command timed out after 2 minutes (no activity)".to_string());
            }
        };

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Check if command failed (non-zero exit status)
        if !output.status.success() {
            let error_msg = if !stderr.is_empty() {
                format!(
                    "Command failed with exit code {}: {}",
                    output.status.code().unwrap_or(-1),
                    stderr
                )
            } else {
                format!(
                    "Command failed with exit code {}",
                    output.status.code().unwrap_or(-1)
                )
            };
            return Err(error_msg.into());
        }

        let result = if !stderr.is_empty() {
            format!(
                "{}: {}\n{}: {}",
                "stdout".truecolor(128, 0, 128), // Dark purple
                stdout.truecolor(128, 0, 128),   // Dark purple
                "stderr".red(),                  // Red
                stderr.red()                     // Red
            )
        } else {
            format!(
                "{}: {}",
                "stdout".truecolor(128, 0, 128), // Dark purple
                stdout.truecolor(128, 0, 128)    // Dark purple
            )
        };

        Ok(result)
    }
}
