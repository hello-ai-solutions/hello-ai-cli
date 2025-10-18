use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct NightfallClient {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

#[derive(Debug, Serialize)]
struct NightfallScanRequest {
    payload: Vec<String>,
    #[serde(rename = "detectionRules")]
    detection_rules: Vec<DetectionRule>,
    #[serde(rename = "detectionRuleUUIDs")]
    detection_rule_uuids: Vec<String>,
}

#[derive(Debug, Serialize)]
struct DetectionRule {
    #[serde(rename = "detectors")]
    detectors: Vec<Detector>,
    #[serde(rename = "logicalOp")]
    logical_op: String,
}

#[derive(Debug, Serialize)]
struct Detector {
    #[serde(rename = "minNumFindings")]
    min_num_findings: u32,
    #[serde(rename = "minConfidence")]
    min_confidence: String,
    #[serde(rename = "displayName")]
    display_name: String,
    #[serde(rename = "detectorType")]
    detector_type: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NightfallScanResponse {
    pub findings: Vec<NightfallFinding>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct NightfallFinding {
    pub finding: String,
    pub detector: NightfallDetector,
    pub confidence: String,
    pub location: NightfallLocation,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct NightfallDetector {
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "detectorType")]
    pub detector_type: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct NightfallLocation {
    #[serde(rename = "byteRange")]
    pub byte_range: NightfallByteRange,
    #[serde(rename = "codepointRange")]
    pub codepoint_range: NightfallCodepointRange,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct NightfallByteRange {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NightfallCodepointRange {
    pub start: u32,
    pub end: u32,
}

impl NightfallClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.nightfall.ai".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn scan_text(
        &self,
        text: &str,
        _source: &str,
    ) -> Result<NightfallScanResponse, Box<dyn std::error::Error>> {
        let request = NightfallScanRequest {
            payload: vec![text.to_string()],
            detection_rules: vec![DetectionRule {
                detectors: vec![
                    // Credit Card Numbers
                    Detector {
                        min_num_findings: 1,
                        min_confidence: "LIKELY".to_string(),
                        display_name: "Credit Card Number".to_string(),
                        detector_type: "NIGHTFALL_DETECTOR".to_string(),
                    },
                    // SSN
                    Detector {
                        min_num_findings: 1,
                        min_confidence: "LIKELY".to_string(),
                        display_name: "US Social Security Number".to_string(),
                        detector_type: "NIGHTFALL_DETECTOR".to_string(),
                    },
                    // Email
                    Detector {
                        min_num_findings: 1,
                        min_confidence: "LIKELY".to_string(),
                        display_name: "Email Address".to_string(),
                        detector_type: "NIGHTFALL_DETECTOR".to_string(),
                    },
                    // Phone
                    Detector {
                        min_num_findings: 1,
                        min_confidence: "LIKELY".to_string(),
                        display_name: "Phone Number".to_string(),
                        detector_type: "NIGHTFALL_DETECTOR".to_string(),
                    },
                    // API Keys
                    Detector {
                        min_num_findings: 1,
                        min_confidence: "LIKELY".to_string(),
                        display_name: "API Key".to_string(),
                        detector_type: "NIGHTFALL_DETECTOR".to_string(),
                    },
                ],
                logical_op: "ANY".to_string(),
            }],
            detection_rule_uuids: vec![],
        };

        let response = self
            .client
            .post(&format!("{}/v3/scan", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let scan_result: NightfallScanResponse = response.json().await?;
            Ok(scan_result)
        } else {
            let error_text = response.text().await?;
            Err(format!("Nightfall API error: {}", error_text).into())
        }
    }

    #[allow(dead_code)]
    pub fn display_nightfall_results(&self, result: &NightfallScanResponse, source: &str) {
        if !result.findings.is_empty() {
            println!(
                "\n{} {} {}",
                "🌙".purple(),
                "NIGHTFALL PII DETECTED".bright_purple().bold(),
                format!("in {}", source).purple()
            );
            println!("{}", "─".repeat(50).bright_black());

            for finding in &result.findings {
                let confidence_color = match finding.confidence.as_str() {
                    "VERY_LIKELY" => "🔴 VERY_LIKELY".bright_red(),
                    "LIKELY" => "🟡 LIKELY".bright_yellow(),
                    "POSSIBLE" => "🟢 POSSIBLE".bright_green(),
                    _ => finding.confidence.as_str().white(),
                };

                println!(
                    "{} {} - {}",
                    confidence_color,
                    finding.detector.display_name.bright_white(),
                    "DETECTED".bright_black()
                );
            }
            println!("{}", "─".repeat(50).bright_black());
        } else {
            println!(
                "{} {} {}",
                "✅".green(),
                "NIGHTFALL SCAN CLEAN".bright_green(),
                format!("- {}", source).bright_black()
            );
        }
    }

    pub fn display_simple_nightfall_scan(&self, has_pii: bool) {
        println!(
            "{} {}",
            "PII Scan:".bright_white(),
            if has_pii {
                "True".bright_red()
            } else {
                "False".bright_green()
            }
        );
    }

    pub async fn scan_text_simple(&self, text: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let result = self.scan_text(text, "input").await?;
        Ok(!result.findings.is_empty())
    }

    pub fn redact_nightfall_findings(&self, text: &str, findings: &[NightfallFinding]) -> String {
        let mut redacted_text = text.to_string();

        // Sort findings by start position in reverse order to avoid offset issues
        let mut sorted_findings = findings.to_vec();
        sorted_findings.sort_by(|a, b| {
            b.location
                .codepoint_range
                .start
                .cmp(&a.location.codepoint_range.start)
        });

        for finding in sorted_findings {
            let start = finding.location.codepoint_range.start as usize;
            let end = finding.location.codepoint_range.end as usize;

            if start < redacted_text.len() && end <= redacted_text.len() {
                let replacement = format!(
                    "[{}-REDACTED]",
                    finding
                        .detector
                        .display_name
                        .to_uppercase()
                        .replace(" ", "-")
                );
                redacted_text.replace_range(start..end, &replacement);
            }
        }

        redacted_text
    }
}
