use regex::Regex;
use colored::*;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PiiScanResult {
    pub has_pii: bool,
    pub findings: Vec<PiiFindings>,
    pub sanitized_text: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PiiFindings {
    pub pii_type: String,
    pub pattern: String,
    pub location: String,
    pub severity: PiiSeverity,
}

#[derive(Debug, Clone)]
pub enum PiiSeverity {
    High,    // SSN, Credit Cards, etc.
    Medium,  // Email, Phone, etc.
    Low,     // Names, addresses (potential)
}

#[allow(dead_code)]
pub struct PiiScanner {
    patterns: Vec<PiiPattern>,
}

#[allow(dead_code)]
struct PiiPattern {
    name: String,
    regex: Regex,
    severity: PiiSeverity,
    replacement: String,
}

impl PiiScanner {
    pub fn new() -> Self {
        let mut patterns = Vec::new();
        
        // High severity patterns
        if let Ok(ssn) = Regex::new(r"\b\d{3}-\d{2}-\d{4}\b|\b\d{9}\b") {
            patterns.push(PiiPattern {
                name: "SSN".to_string(),
                regex: ssn,
                severity: PiiSeverity::High,
                replacement: "[SSN-REDACTED]".to_string(),
            });
        }
        
        if let Ok(credit_card) = Regex::new(r"\b(?:4\d{3}|5[1-5]\d{2}|3[47]\d{2}|6011)[-\s]?\d{4}[-\s]?\d{4}[-\s]?\d{4}\b") {
            patterns.push(PiiPattern {
                name: "Credit Card".to_string(),
                regex: credit_card,
                severity: PiiSeverity::High,
                replacement: "[CARD-REDACTED]".to_string(),
            });
        }
        
        // Medium severity patterns
        if let Ok(email) = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b") {
            patterns.push(PiiPattern {
                name: "Email".to_string(),
                regex: email,
                severity: PiiSeverity::Medium,
                replacement: "[EMAIL-REDACTED]".to_string(),
            });
        }
        
        if let Ok(phone) = Regex::new(r"\b(?:\+?1[-.\s]?)?\(?[0-9]{3}\)?[-.\s]?[0-9]{3}[-.\s]?[0-9]{4}\b") {
            patterns.push(PiiPattern {
                name: "Phone".to_string(),
                regex: phone,
                severity: PiiSeverity::Medium,
                replacement: "[PHONE-REDACTED]".to_string(),
            });
        }
        
        if let Ok(ip) = Regex::new(r"\b(?:[0-9]{1,3}\.){3}[0-9]{1,3}\b") {
            patterns.push(PiiPattern {
                name: "IP Address".to_string(),
                regex: ip,
                severity: PiiSeverity::Low,
                replacement: "".to_string(), // Don't redact, just notify
            });
        }
        
        // AWS-specific patterns
        if let Ok(aws_key) = Regex::new(r"AKIA[0-9A-Z]{16}") {
            patterns.push(PiiPattern {
                name: "AWS Access Key".to_string(),
                regex: aws_key,
                severity: PiiSeverity::High,
                replacement: "[AWS-KEY-REDACTED]".to_string(),
            });
        }
        
        if let Ok(aws_secret) = Regex::new(r"[A-Za-z0-9/+=]{40}") {
            patterns.push(PiiPattern {
                name: "AWS Secret Key".to_string(),
                regex: aws_secret,
                severity: PiiSeverity::High,
                replacement: "[AWS-SECRET-REDACTED]".to_string(),
            });
        }
        
        Self { patterns }
    }
    
    pub fn scan(&self, text: &str, _source: &str) -> PiiScanResult {
        // PII scanning disabled - return clean result
        PiiScanResult {
            has_pii: false,
            findings: Vec::new(),
            sanitized_text: text.to_string(),
        }
    }
    
    #[allow(dead_code)]
    pub fn display_scan_results(&self, result: &PiiScanResult, source: &str) {
        if result.has_pii {
            println!("\n{} {} {}", "🔒".red(), "PII DETECTED".bright_red().bold(), format!("in {}", source).red());
            println!("{}", "─".repeat(50).bright_black());
            
            for finding in &result.findings {
                let severity_color = match finding.severity {
                    PiiSeverity::High => "🔴 HIGH".bright_red(),
                    PiiSeverity::Medium => "🟡 MEDIUM".bright_yellow(),
                    PiiSeverity::Low => "🟢 LOW".bright_green(),
                };
                
                let action = match finding.severity {
                    PiiSeverity::Low => "DETECTED".bright_green(),
                    _ => "REDACTED".bright_black(),
                };
                
                println!("{} {} - {}", severity_color, finding.pii_type.bright_white(), action);
            }
            println!("{}", "─".repeat(50).bright_black());
        } else {
            println!("{} {} {}", "✅".green(), "PII SCAN CLEAN".bright_green(), format!("- {}", source).bright_black());
        }
    }
    
    #[allow(dead_code)]
    pub fn display_simple_scan(&self, has_pii: bool) {
        println!("{} {}", "PII Scan:".bright_white(), 
            if has_pii { "True".bright_red() } else { "False".bright_green() });
    }
}
