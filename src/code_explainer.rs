use crate::local_llm::LocalLLM;
use std::collections::HashMap;

pub struct CodeExplainer {
    language_patterns: HashMap<String, Vec<String>>,
    llm: LocalLLM,
}

#[allow(dead_code)]
impl CodeExplainer {
    pub fn new() -> Self {
        let mut language_patterns = HashMap::new();
        
        language_patterns.insert("rust".to_string(), vec![
            "fn ".to_string(),
            "impl ".to_string(),
            "struct ".to_string(),
            "enum ".to_string(),
            "trait ".to_string(),
        ]);
        
        language_patterns.insert("python".to_string(), vec![
            "def ".to_string(),
            "class ".to_string(),
            "import ".to_string(),
            "from ".to_string(),
        ]);

        language_patterns.insert("java".to_string(), vec![
            "public ".to_string(),
            "private ".to_string(),
            "class ".to_string(),
            "@Override".to_string(),
        ]);

        language_patterns.insert("csharp".to_string(), vec![
            "using ".to_string(),
            "namespace ".to_string(),
            "public ".to_string(),
            "private ".to_string(),
        ]);

        language_patterns.insert("php".to_string(), vec![
            "function ".to_string(),
            "class ".to_string(),
            "$".to_string(),
            "<?php".to_string(),
        ]);

        language_patterns.insert("ruby".to_string(), vec![
            "def ".to_string(),
            "class ".to_string(),
            "end".to_string(),
            "require ".to_string(),
        ]);

        language_patterns.insert("swift".to_string(), vec![
            "func ".to_string(),
            "var ".to_string(),
            "class ".to_string(),
            "@objc".to_string(),
        ]);
        
        Self { 
            language_patterns,
            llm: LocalLLM::new("localhost".to_string(), "default".to_string()),
        }
    }

    pub async fn explain_code(&self, code: &str, language: &str) -> Result<String, Box<dyn std::error::Error>> {
        let analysis = self.analyze_code_structure(code, language);
        let complexity = self.calculate_complexity(code);
        let patterns = self.identify_patterns(code, language);
        
        // Try enhanced LLM analysis
        let enhanced_analysis = match self.llm.analyze_code(code, language).await {
            Ok(llm_result) => format!("\n\n### 🤖 Enhanced AI Analysis\n{}", llm_result),
            Err(_) => String::new(),
        };
        
        let explanation = format!(
            "## Code Analysis\n\n\
            **Language:** {}\n\
            **Complexity:** {}\n\n\
            ### Structure\n{}\n\n\
            ### Patterns Detected\n{}\n\n\
            ### Explanation\n{}{}",
            language,
            complexity,
            analysis.structure,
            patterns.join("\n"),
            analysis.explanation,
            enhanced_analysis
        );
        
        Ok(explanation)
    }

    pub async fn explain_function(&self, code: &str, function_name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let lines: Vec<&str> = code.lines().collect();
        let mut function_code = String::new();
        let mut in_function = false;
        let mut brace_count = 0;
        
        for line in lines {
            if line.contains(&format!("fn {}", function_name)) {
                in_function = true;
            }
            
            if in_function {
                function_code.push_str(line);
                function_code.push('\n');
                
                brace_count += line.chars().filter(|&c| c == '{').count() as i32;
                brace_count -= line.chars().filter(|&c| c == '}').count() as i32;
                
                if brace_count == 0 && line.contains('}') {
                    break;
                }
            }
        }
        
        if function_code.is_empty() {
            return Ok(format!("Function '{}' not found", function_name));
        }
        
        let explanation = self.analyze_function(&function_code, function_name).await?;
        Ok(explanation)
    }

    async fn analyze_function(&self, code: &str, name: &str) -> Result<String, Box<dyn std::error::Error>> {
        let parameters = self.extract_parameters(code);
        let return_type = self.extract_return_type(code);
        let purpose = self.infer_purpose(code, name);
        
        Ok(format!(
            "## Function: {}\n\n\
            **Parameters:** {}\n\
            **Returns:** {}\n\
            **Purpose:** {}\n\n\
            **Implementation Details:**\n{}",
            name,
            parameters,
            return_type,
            purpose,
            self.analyze_implementation(code)
        ))
    }

    fn analyze_code_structure(&self, code: &str, _language: &str) -> CodeAnalysis {
        let lines = code.lines().count();
        let functions = code.matches("fn ").count();
        let structs = code.matches("struct ").count();
        let imports = code.matches("use ").count();
        
        let structure = format!(
            "- Lines of code: {}\n\
            - Functions: {}\n\
            - Structs: {}\n\
            - Imports: {}",
            lines, functions, structs, imports
        );
        
        let explanation = if functions > 0 {
            "This code defines functions and data structures. ".to_string()
        } else {
            "This appears to be a script or configuration. ".to_string()
        };
        
        CodeAnalysis { structure, explanation }
    }

    fn calculate_complexity(&self, code: &str) -> String {
        let lines = code.lines().count();
        let conditions = code.matches("if ").count() + code.matches("match ").count();
        let loops = code.matches("for ").count() + code.matches("while ").count();
        
        let score = lines + (conditions * 2) + (loops * 3);
        
        match score {
            0..=50 => "Low".to_string(),
            51..=150 => "Medium".to_string(),
            _ => "High".to_string(),
        }
    }

    fn identify_patterns(&self, code: &str, _language: &str) -> Vec<String> {
        let mut patterns = Vec::new();
        
        if code.contains("Result<") {
            patterns.push("- Error handling with Result type".to_string());
        }
        if code.contains("async") {
            patterns.push("- Asynchronous programming".to_string());
        }
        if code.contains("impl ") {
            patterns.push("- Object-oriented design with implementations".to_string());
        }
        if code.contains("match ") {
            patterns.push("- Pattern matching".to_string());
        }
        
        patterns
    }

    fn extract_parameters(&self, code: &str) -> String {
        // Simple parameter extraction
        if let Some(start) = code.find('(') {
            if let Some(end) = code.find(')') {
                let params = &code[start+1..end];
                if params.trim().is_empty() {
                    "None".to_string()
                } else {
                    params.to_string()
                }
            } else {
                "Unknown".to_string()
            }
        } else {
            "None".to_string()
        }
    }

    fn extract_return_type(&self, code: &str) -> String {
        if let Some(arrow_pos) = code.find("-> ") {
            let after_arrow = &code[arrow_pos + 3..];
            if let Some(brace_pos) = after_arrow.find('{') {
                after_arrow[..brace_pos].trim().to_string()
            } else {
                "Unknown".to_string()
            }
        } else {
            "()".to_string()
        }
    }

    fn infer_purpose(&self, _code: &str, name: &str) -> String {
        if name.starts_with("get_") {
            "Retrieves data or values".to_string()
        } else if name.starts_with("set_") {
            "Sets or updates data".to_string()
        } else if name.starts_with("is_") || name.starts_with("has_") {
            "Returns boolean condition check".to_string()
        } else if name.contains("test") {
            "Unit test function".to_string()
        } else {
            "Performs specific business logic".to_string()
        }
    }

    fn analyze_implementation(&self, code: &str) -> String {
        let mut analysis = Vec::new();
        
        if code.contains("println!") {
            analysis.push("- Contains debug/logging output");
        }
        if code.contains("unwrap()") {
            analysis.push("- Uses unwrap() - potential panic risk");
        }
        if code.contains("?") {
            analysis.push("- Uses error propagation with ? operator");
        }
        if code.contains("async") {
            analysis.push("- Asynchronous function with await points");
        }
        
        if analysis.is_empty() {
            "Standard implementation without notable patterns".to_string()
        } else {
            analysis.join("\n")
        }
    }
}

struct CodeAnalysis {
    structure: String,
    explanation: String,
}
