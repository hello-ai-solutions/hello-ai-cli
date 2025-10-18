use std::fs;
use std::path::Path;

pub struct CodeCompletion {
    context_lines: usize,
}

impl CodeCompletion {
    pub fn new() -> Self {
        Self { context_lines: 10 }
    }

    pub async fn suggest_completion(&self, file_path: &str, cursor_line: usize) -> Result<String, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let lines: Vec<&str> = content.lines().collect();
        
        let start = cursor_line.saturating_sub(self.context_lines);
        let end = (cursor_line + self.context_lines).min(lines.len());
        let context = lines[start..end].join("\n");
        
        let language = self.detect_language(file_path);
        
        let prompt = format!(
            "Complete this {} code. Provide only the next logical line(s):\n\n{}\n\nCursor at line {}. Complete:",
            language, context, cursor_line + 1
        );

        // Simulate AI completion (would use actual Bedrock call)
        Ok(self.generate_completion(&prompt, &language).await?)
    }

    fn detect_language(&self, file_path: &str) -> String {
        match Path::new(file_path).extension().and_then(|s| s.to_str()) {
            Some("rs") => "Rust".to_string(),
            Some("py") => "Python".to_string(),
            Some("js") => "JavaScript".to_string(),
            Some("ts") => "TypeScript".to_string(),
            Some("java") => "Java".to_string(),
            Some("go") => "Go".to_string(),
            Some("cs") => "C#".to_string(),
            Some("php") => "PHP".to_string(),
            Some("rb") => "Ruby".to_string(),
            Some("swift") => "Swift".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    async fn generate_completion(&self, _prompt: &str, language: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Mock completion based on language patterns
        let completion = match language {
            "Rust" => "    Ok(result)",
            "Python" => "    return result",
            "JavaScript" | "TypeScript" => "    return result;",
            "Java" => "        return result;",
            _ => "// TODO: Complete implementation",
        };
        
        Ok(completion.to_string())
    }

    pub async fn suggest_imports(&self, file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let mut suggestions = Vec::new();
        
        // Analyze code for missing imports
        if content.contains("HashMap") && !content.contains("use std::collections::HashMap") {
            suggestions.push("use std::collections::HashMap;".to_string());
        }
        if content.contains("Vec") && !content.contains("use std::vec::Vec") {
            suggestions.push("// Vec is in prelude".to_string());
        }
        
        Ok(suggestions)
    }
}
