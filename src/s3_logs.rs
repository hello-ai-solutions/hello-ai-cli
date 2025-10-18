use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub message: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct S3LogConfig {
    pub bucket: String,
    pub prefix: String,
    pub region: String,
    pub enabled: bool,
}

impl Default for S3LogConfig {
    fn default() -> Self {
        Self {
            bucket: "qcli-logs".to_string(),
            prefix: "logs".to_string(),
            region: "ap-southeast-2".to_string(),
            enabled: false,
        }
    }
}

pub struct S3Logger {
    config: S3LogConfig,
    local_buffer: Vec<S3LogEntry>,
}

impl S3Logger {
    pub fn new(config: S3LogConfig) -> Self {
        Self {
            config,
            local_buffer: Vec::new(),
        }
    }

    pub async fn log(&mut self, level: &str, message: &str, metadata: HashMap<String, String>) {
        let entry = S3LogEntry {
            timestamp: Utc::now(),
            level: level.to_string(),
            message: message.to_string(),
            metadata,
        };

        self.local_buffer.push(entry);

        if self.local_buffer.len() >= 5 {
            let _ = self.flush_to_s3().await;
        }
    }

    pub async fn flush_to_s3(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.config.enabled || self.local_buffer.is_empty() {
            return Ok(());
        }

        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let key = format!("{}/qcli_logs_{}.json", self.config.prefix, timestamp);

        let logs_json = serde_json::to_string_pretty(&self.local_buffer)?;
        let temp_path = format!("/tmp/qcli_logs_{}.json", timestamp);
        tokio::fs::write(&temp_path, logs_json).await?;

        let upload_cmd = format!(
            "aws s3 cp {} s3://{}/{} --region {}",
            temp_path, self.config.bucket, key, self.config.region
        );

        let output = if cfg!(target_os = "windows") {
            tokio::process::Command::new("powershell")
                .arg("-Command")
                .arg(&upload_cmd)
                .output()
                .await?
        } else {
            tokio::process::Command::new("sh")
                .arg("-c")
                .arg(&upload_cmd)
                .output()
                .await?
        };

        let _ = tokio::fs::remove_file(&temp_path).await;

        if output.status.success() {
            println!(
                "📤 Logs uploaded to S3: s3://{}/{}",
                self.config.bucket, key
            );
            self.local_buffer.clear();
        } else {
            eprintln!(
                "❌ S3 upload failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(())
    }

    pub async fn load_config() -> S3LogConfig {
        let config_path = ".amazonq/s3_logs_config.json";

        if tokio::fs::metadata(config_path).await.is_ok() {
            if let Ok(content) = tokio::fs::read_to_string(config_path).await {
                if let Ok(config) = serde_json::from_str(&content) {
                    return config;
                }
            }
        }

        S3LogConfig::default()
    }

    pub async fn save_config(config: &S3LogConfig) -> Result<(), Box<dyn std::error::Error>> {
        tokio::fs::create_dir_all(".amazonq").await?;
        let config_json = serde_json::to_string_pretty(config)?;
        tokio::fs::write(".amazonq/s3_logs_config.json", config_json).await?;
        Ok(())
    }
}
