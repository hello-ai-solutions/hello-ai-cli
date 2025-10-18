use aws_sdk_ses::types::{Body, Content, Destination, Message};
use aws_sdk_ses::Client as SesClient;
use colored::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::process::Command;

pub struct SesAlertService {
    client: SesClient,
    from_email: String,
    to_email: String,
}

impl SesAlertService {
    pub fn new(ses_client: SesClient, from_email: String, to_email: String) -> Self {
        Self {
            client: ses_client,
            from_email,
            to_email,
        }
    }

    pub async fn send_pii_alert(
        &self,
        pii_findings: &[crate::pii_scanner::PiiFindings],
        source: &str,
        content: &str,
        file_path: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let username = self.get_current_user();
        let content_hash = self.hash_content(content);
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");

        let subject = format!(
            "🔴 CRITICAL PII ALERT - {} HIGH severity items detected",
            pii_findings.len()
        );

        let pii_details = pii_findings
            .iter()
            .map(|finding| {
                let severity = match finding.severity {
                    crate::pii_scanner::PiiSeverity::High => "🔴 HIGH",
                    crate::pii_scanner::PiiSeverity::Medium => "🟡 MEDIUM",
                    crate::pii_scanner::PiiSeverity::Low => "🟢 LOW",
                };
                format!("  • {} - {}", severity, finding.pii_type)
            })
            .collect::<Vec<_>>()
            .join("\n");

        let body_text = format!(
            r#"PII SECURITY INCIDENT REPORT
=============================

INCIDENT DETAILS:
• Timestamp: {}
• User: {}
• Source: {}
• File Path: {}
• Content Hash: {}

PII DETECTED ({} items):
{}

SYSTEM INFORMATION:
• Region: ap-southeast-2 (Sydney)
• Service: Q CLI Bedrock
• Action Taken: Content automatically redacted

This is an automated security alert from the Q CLI PII scanning system.
All sensitive data has been automatically redacted to protect privacy.
"#,
            timestamp,
            username,
            source,
            file_path.unwrap_or("N/A"),
            content_hash,
            pii_findings.len(),
            pii_details
        );

        let message = Message::builder()
            .subject(Content::builder().data(subject).build()?)
            .body(
                Body::builder()
                    .text(Content::builder().data(body_text).build()?)
                    .build(),
            )
            .build();

        let destination = Destination::builder().to_addresses(&self.to_email).build();

        match self
            .client
            .send_email()
            .source(&self.from_email)
            .destination(destination)
            .message(message)
            .send()
            .await
        {
            Ok(_) => {
                println!(
                    "{} {}",
                    "📧".bright_blue(),
                    "PII security alert sent via SES".bright_blue()
                );
                Ok(())
            }
            Err(e) => {
                eprintln!(
                    "{} Failed to send PII alert: {}",
                    "❌".red(),
                    e.to_string().red()
                );
                Err(e.into())
            }
        }
    }

    fn get_current_user(&self) -> String {
        match Command::new("whoami").output() {
            Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
            Err(_) => "unknown".to_string(),
        }
    }

    fn hash_content(&self, content: &str) -> String {
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("sha256:{:x}", hasher.finish())
    }
}

pub async fn create_ses_client() -> Result<SesClient, Box<dyn std::error::Error>> {
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new("ap-southeast-2"))
        .load()
        .await;

    Ok(SesClient::new(&config))
}
