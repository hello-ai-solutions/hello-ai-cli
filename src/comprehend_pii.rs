use aws_sdk_comprehend::{Client as ComprehendClient, types::LanguageCode};
use aws_config::BehaviorVersion;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehendPiiResult {
    pub has_pii: bool,
    pub sanitized_text: String,
    pub entities: Vec<PiiEntity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiEntity {
    pub entity_type: String,
    pub confidence: f32,
    pub begin_offset: i32,
    pub end_offset: i32,
}

pub struct ComprehendPiiScanner {
    client: ComprehendClient,
    excluded_usernames: HashSet<String>,
}

impl ComprehendPiiScanner {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
        let client = ComprehendClient::new(&config);
        
        // Common usernames to exclude from PII detection
        let mut excluded_usernames = HashSet::new();
        excluded_usernames.insert("hans".to_lowercase());
        excluded_usernames.insert("admin".to_lowercase());
        excluded_usernames.insert("user".to_lowercase());
        excluded_usernames.insert("root".to_lowercase());
        excluded_usernames.insert("guest".to_lowercase());
        
        Ok(ComprehendPiiScanner { 
            client,
            excluded_usernames,
        })
    }

    pub async fn scan_text(&self, text: &str) -> Result<ComprehendPiiResult, Box<dyn std::error::Error>> {
        let response = self.client
            .detect_pii_entities()
            .text(text)
            .language_code(LanguageCode::En)
            .send()
            .await?;

        let entities: Vec<PiiEntity> = response.entities()
            .iter()
            .filter_map(|entity| {
                let entity_type = entity.r#type().unwrap().as_str().to_string();
                let begin_offset = entity.begin_offset().unwrap_or(0) as usize;
                let end_offset = entity.end_offset().unwrap_or(0) as usize;
                
                // Extract the detected text
                if begin_offset < text.len() && end_offset <= text.len() {
                    let detected_text = &text[begin_offset..end_offset];
                    
                    // Filter out excluded usernames and file paths
                    if self.should_exclude_entity(&entity_type, detected_text, text, begin_offset) {
                        return None;
                    }
                }
                
                Some(PiiEntity {
                    entity_type,
                    confidence: entity.score().unwrap_or(0.0),
                    begin_offset: entity.begin_offset().unwrap_or(0),
                    end_offset: entity.end_offset().unwrap_or(0),
                })
            })
            .collect();

        let has_pii = !entities.is_empty();
        let sanitized_text = if has_pii {
            self.sanitize_text(text, &entities)
        } else {
            text.to_string()
        };

        Ok(ComprehendPiiResult {
            has_pii,
            sanitized_text,
            entities,
        })
    }

    fn should_exclude_entity(&self, entity_type: &str, detected_text: &str, full_text: &str, begin_offset: usize) -> bool {
        // Exclude USERNAME entities that are in our exclusion list
        if entity_type == "USERNAME" {
            if self.excluded_usernames.contains(&detected_text.to_lowercase()) {
                return true;
            }
        }
        
        // Exclude entities that appear to be part of file paths
        if self.is_in_file_path(full_text, begin_offset, detected_text) {
            return true;
        }
        
        // Exclude very short usernames (likely false positives)
        if entity_type == "USERNAME" && detected_text.len() <= 2 {
            return true;
        }
        
        false
    }
    
    fn is_in_file_path(&self, text: &str, begin_offset: usize, detected_text: &str) -> bool {
        // Check if the detected text is part of a file path
        let context_start = begin_offset.saturating_sub(20);
        let context_end = (begin_offset + detected_text.len() + 20).min(text.len());
        let context = &text[context_start..context_end];
        
        // Look for file path indicators
        context.contains("/Users/") || 
        context.contains("/home/") ||
        context.contains("\\Users\\") ||
        context.contains("/opt/") ||
        context.contains("/var/") ||
        context.contains("/tmp/") ||
        context.contains("C:\\") ||
        context.contains("~/")
    }

    fn sanitize_text(&self, text: &str, entities: &[PiiEntity]) -> String {
        let mut sanitized = text.to_string();
        let mut offset_adjustment = 0i32;

        for entity in entities {
            let start = (entity.begin_offset - offset_adjustment) as usize;
            let end = (entity.end_offset - offset_adjustment) as usize;
            
            if start < sanitized.len() && end <= sanitized.len() {
                let replacement = format!("[{}]", entity.entity_type);
                sanitized.replace_range(start..end, &replacement);
                offset_adjustment += (end - start) as i32 - replacement.len() as i32;
            }
        }

        sanitized
    }
}
