use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    pub severity: String,
    pub issue_type: String,
    pub line: usize,
    pub description: String,
    pub remediation: String,
}

pub struct SecurityScanner {
    rules: Vec<SecurityRule>,
}

struct SecurityRule {
    pattern: String,
    severity: String,
    issue_type: String,
    description: String,
    remediation: String,
}

impl SecurityScanner {
    pub fn new() -> Self {
        let rules = vec![
            SecurityRule {
                pattern: "unwrap()".to_string(),
                severity: "Medium".to_string(),
                issue_type: "Panic Risk".to_string(),
                description: "Using unwrap() can cause panics".to_string(),
                remediation: "Use match, if let, or expect() with descriptive message".to_string(),
            },
            SecurityRule {
                pattern: "unsafe".to_string(),
                severity: "High".to_string(),
                issue_type: "Unsafe Code".to_string(),
                description: "Unsafe code block detected".to_string(),
                remediation: "Review unsafe code for memory safety".to_string(),
            },
            SecurityRule {
                pattern: "password".to_string(),
                severity: "Critical".to_string(),
                issue_type: "Hardcoded Secret".to_string(),
                description: "Potential hardcoded password".to_string(),
                remediation: "Use environment variables or secure vault".to_string(),
            },
            SecurityRule {
                pattern: "api_key".to_string(),
                severity: "Critical".to_string(),
                issue_type: "Hardcoded Secret".to_string(),
                description: "Potential hardcoded API key".to_string(),
                remediation: "Use environment variables or secure configuration".to_string(),
            },
        ];

        Self { rules }
    }

    pub async fn scan_file(
        &self,
        file_path: &str,
    ) -> Result<Vec<SecurityIssue>, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let mut issues = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            for rule in &self.rules {
                if line.to_lowercase().contains(&rule.pattern.to_lowercase()) {
                    issues.push(SecurityIssue {
                        severity: rule.severity.clone(),
                        issue_type: rule.issue_type.clone(),
                        line: line_num + 1,
                        description: rule.description.clone(),
                        remediation: rule.remediation.clone(),
                    });
                }
            }
        }

        Ok(issues)
    }

    pub async fn scan_directory(
        &self,
        dir_path: &str,
    ) -> Result<Vec<(String, Vec<SecurityIssue>)>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();

        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if matches!(
                        ext.to_str(),
                        Some("rs") | Some("py") | Some("js") | Some("ts") | Some("java")
                    ) {
                        let file_path = path.to_string_lossy().to_string();
                        let issues = self.scan_file(&file_path).await?;
                        if !issues.is_empty() {
                            results.push((file_path, issues));
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    pub fn generate_report(&self, results: &[(String, Vec<SecurityIssue>)]) -> String {
        let mut report = String::from("# Security Scan Report\n\n");

        let mut critical = 0;
        let mut high = 0;
        let mut medium = 0;

        for (_, issues) in results {
            for issue in issues {
                match issue.severity.as_str() {
                    "Critical" => critical += 1,
                    "High" => high += 1,
                    "Medium" => medium += 1,
                    _ => {}
                }
            }
        }

        report.push_str(&format!(
            "## Summary\n- Critical: {}\n- High: {}\n- Medium: {}\n\n",
            critical, high, medium
        ));

        for (file_path, issues) in results {
            report.push_str(&format!("## {}\n", file_path));
            for issue in issues {
                report.push_str(&format!(
                    "- **{}** (Line {}): {}\n  - Remediation: {}\n\n",
                    issue.severity, issue.line, issue.description, issue.remediation
                ));
            }
        }

        report
    }
}
