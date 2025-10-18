use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Knowledge {
    pub action: String,
    pub content: Option<String>,
    pub query: Option<String>,
    pub path: Option<String>,
}

impl Knowledge {
    pub async fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let knowledge_dir = ".amazonq/knowledge";
        fs::create_dir_all(knowledge_dir)?;

        match self.action.as_str() {
            "store" => {
                if let (Some(content), Some(path)) = (&self.content, &self.path) {
                    let knowledge_file = format!("{}/knowledge.json", knowledge_dir);
                    let mut knowledge_data: HashMap<String, Value> =
                        if Path::new(&knowledge_file).exists() {
                            let data = fs::read_to_string(&knowledge_file)?;
                            serde_json::from_str(&data).unwrap_or_default()
                        } else {
                            HashMap::new()
                        };

                    knowledge_data.insert(
                        path.clone(),
                        json!({
                            "content": content,
                            "timestamp": chrono::Utc::now().to_rfc3339()
                        }),
                    );

                    fs::write(
                        &knowledge_file,
                        serde_json::to_string_pretty(&knowledge_data)?,
                    )?;
                    Ok(format!("Stored knowledge for: {}", path))
                } else {
                    Err("Missing content or path for knowledge storage".into())
                }
            }
            "search" => {
                if let Some(query) = &self.query {
                    let knowledge_file = format!("{}/knowledge.json", knowledge_dir);
                    if Path::new(&knowledge_file).exists() {
                        let data = fs::read_to_string(&knowledge_file)?;
                        let knowledge_data: HashMap<String, Value> = serde_json::from_str(&data)?;

                        let mut results = Vec::new();
                        for (path, entry) in knowledge_data {
                            if let Some(content) = entry.get("content").and_then(|c| c.as_str()) {
                                if content.to_lowercase().contains(&query.to_lowercase()) {
                                    results.push(format!("{}: {}", path, content));
                                }
                            }
                        }

                        if results.is_empty() {
                            Ok("No matching knowledge found".to_string())
                        } else {
                            Ok(results.join("\n"))
                        }
                    } else {
                        Ok("No knowledge base found".to_string())
                    }
                } else {
                    Err("Missing query for knowledge search".into())
                }
            }
            "list" => {
                let knowledge_file = format!("{}/knowledge.json", knowledge_dir);
                if Path::new(&knowledge_file).exists() {
                    let data = fs::read_to_string(&knowledge_file)?;
                    let knowledge_data: HashMap<String, Value> = serde_json::from_str(&data)?;

                    let entries: Vec<String> = knowledge_data.keys().cloned().collect();
                    Ok(format!("Knowledge entries: {}", entries.join(", ")))
                } else {
                    Ok("No knowledge base found".to_string())
                }
            }
            _ => Err(format!("Unknown knowledge action: {}", self.action).into()),
        }
    }
}
