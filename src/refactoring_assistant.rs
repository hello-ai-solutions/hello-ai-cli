use serde::{Deserialize, Serialize};
use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefactoringSuggestion {
    pub suggestion_type: String,
    pub line: usize,
    pub original: String,
    pub improved: String,
    pub reason: String,
    pub impact: String,
}

pub struct RefactoringAssistant {
    rules: Vec<RefactoringRule>,
}

struct RefactoringRule {
    name: String,
    pattern: String,
    replacement: String,
    reason: String,
    impact: String,
}

impl RefactoringAssistant {
    pub fn new() -> Self {
        let rules = vec![
            RefactoringRule {
                name: "Replace unwrap with expect".to_string(),
                pattern: ".unwrap()".to_string(),
                replacement: ".expect(\"descriptive message\")".to_string(),
                reason: "Provides better error messages when panicking".to_string(),
                impact: "Improved debugging experience".to_string(),
            },
            RefactoringRule {
                name: "Use if let instead of match".to_string(),
                pattern: "match ".to_string(),
                replacement: "if let ".to_string(),
                reason: "Simpler syntax for single pattern matching".to_string(),
                impact: "Reduced code complexity".to_string(),
            },
            RefactoringRule {
                name: "Extract magic numbers".to_string(),
                pattern: "".to_string(), // Special handling needed
                replacement: "const ".to_string(),
                reason: "Magic numbers should be named constants".to_string(),
                impact: "Improved maintainability".to_string(),
            },
        ];
        
        Self { rules }
    }

    pub async fn analyze_file(&self, file_path: &str) -> Result<Vec<RefactoringSuggestion>, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let mut suggestions = Vec::new();
        
        // Analyze for various refactoring opportunities
        suggestions.extend(self.find_unwrap_usage(&content).await?);
        suggestions.extend(self.find_long_functions(&content).await?);
        suggestions.extend(self.find_duplicate_code(&content).await?);
        suggestions.extend(self.find_magic_numbers(&content).await?);
        suggestions.extend(self.find_complex_conditions(&content).await?);
        
        Ok(suggestions)
    }

    async fn find_unwrap_usage(&self, content: &str) -> Result<Vec<RefactoringSuggestion>, Box<dyn std::error::Error>> {
        let mut suggestions = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            if line.contains(".unwrap()") {
                suggestions.push(RefactoringSuggestion {
                    suggestion_type: "Error Handling".to_string(),
                    line: line_num + 1,
                    original: line.trim().to_string(),
                    improved: line.replace(".unwrap()", ".expect(\"Add descriptive error message\")"),
                    reason: "unwrap() can cause panics without context".to_string(),
                    impact: "Better error messages and debugging".to_string(),
                });
            }
        }
        
        Ok(suggestions)
    }

    async fn find_long_functions(&self, content: &str) -> Result<Vec<RefactoringSuggestion>, Box<dyn std::error::Error>> {
        let mut suggestions = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut current_function = None;
        let mut function_start = 0;
        let mut brace_count = 0;
        
        for (line_num, line) in lines.iter().enumerate() {
            if line.trim_start().starts_with("fn ") {
                current_function = Some(line.trim());
                function_start = line_num;
                brace_count = 0;
            }
            
            if current_function.is_some() {
                brace_count += line.chars().filter(|&c| c == '{').count() as i32;
                brace_count -= line.chars().filter(|&c| c == '}').count() as i32;
                
                if brace_count == 0 && line.contains('}') {
                    let function_length = line_num - function_start + 1;
                    if function_length > 50 {
                        suggestions.push(RefactoringSuggestion {
                            suggestion_type: "Function Length".to_string(),
                            line: function_start + 1,
                            original: format!("Function has {} lines", function_length),
                            improved: "Consider breaking into smaller functions".to_string(),
                            reason: "Long functions are harder to understand and test".to_string(),
                            impact: "Improved readability and maintainability".to_string(),
                        });
                    }
                    current_function = None;
                }
            }
        }
        
        Ok(suggestions)
    }

    async fn find_duplicate_code(&self, content: &str) -> Result<Vec<RefactoringSuggestion>, Box<dyn std::error::Error>> {
        let mut suggestions = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut line_counts = HashMap::new();
        
        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.len() > 10 && !trimmed.starts_with("//") {
                line_counts.entry(trimmed.to_string())
                    .or_insert_with(Vec::new)
                    .push(line_num + 1);
            }
        }
        
        for (line_content, occurrences) in line_counts {
            if occurrences.len() > 2 {
                suggestions.push(RefactoringSuggestion {
                    suggestion_type: "Code Duplication".to_string(),
                    line: occurrences[0],
                    original: format!("Duplicated {} times: {}", occurrences.len(), line_content),
                    improved: "Extract to a function or constant".to_string(),
                    reason: "Duplicate code increases maintenance burden".to_string(),
                    impact: "Reduced code duplication and easier updates".to_string(),
                });
            }
        }
        
        Ok(suggestions)
    }

    async fn find_magic_numbers(&self, content: &str) -> Result<Vec<RefactoringSuggestion>, Box<dyn std::error::Error>> {
        let mut suggestions = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            // Look for numeric literals that aren't 0, 1, or obvious values
            let words: Vec<&str> = line.split_whitespace().collect();
            for word in words {
                if let Ok(num) = word.parse::<i32>() {
                    if num > 1 && num != 10 && num != 100 && num != 1000 {
                        suggestions.push(RefactoringSuggestion {
                            suggestion_type: "Magic Number".to_string(),
                            line: line_num + 1,
                            original: format!("Magic number: {}", num),
                            improved: format!("const MEANINGFUL_NAME: i32 = {};", num),
                            reason: "Magic numbers should be named constants".to_string(),
                            impact: "Improved code readability and maintainability".to_string(),
                        });
                    }
                }
            }
        }
        
        Ok(suggestions)
    }

    async fn find_complex_conditions(&self, content: &str) -> Result<Vec<RefactoringSuggestion>, Box<dyn std::error::Error>> {
        let mut suggestions = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            if line.contains("if ") {
                let condition_complexity = line.matches("&&").count() + line.matches("||").count();
                if condition_complexity > 2 {
                    suggestions.push(RefactoringSuggestion {
                        suggestion_type: "Complex Condition".to_string(),
                        line: line_num + 1,
                        original: line.trim().to_string(),
                        improved: "Extract condition to a well-named function".to_string(),
                        reason: "Complex conditions are hard to understand".to_string(),
                        impact: "Improved readability and testability".to_string(),
                    });
                }
            }
        }
        
        Ok(suggestions)
    }

    pub fn generate_refactoring_report(&self, suggestions: &[RefactoringSuggestion]) -> String {
        let mut report = String::from("# Refactoring Report\n\n");
        
        let mut by_type = HashMap::new();
        for suggestion in suggestions {
            by_type.entry(suggestion.suggestion_type.clone())
                .or_insert_with(Vec::new)
                .push(suggestion);
        }
        
        report.push_str(&format!("Found {} refactoring opportunities\n\n", suggestions.len()));
        
        for (suggestion_type, items) in by_type {
            report.push_str(&format!("## {} ({} items)\n\n", suggestion_type, items.len()));
            
            for item in items {
                report.push_str(&format!(
                    "**Line {}:**\n\
                    - Current: `{}`\n\
                    - Suggested: `{}`\n\
                    - Reason: {}\n\
                    - Impact: {}\n\n",
                    item.line, item.original, item.improved, item.reason, item.impact
                ));
            }
        }
        
        report
    }

    pub async fn apply_refactoring(&self, file_path: &str, suggestion: &RefactoringSuggestion) -> Result<String, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let lines: Vec<&str> = content.lines().collect();
        
        if suggestion.line > lines.len() {
            return Err("Line number out of range".into());
        }
        
        let mut new_lines = lines.clone();
        new_lines[suggestion.line - 1] = &suggestion.improved;
        
        let new_content = new_lines.join("\n");
        fs::write(file_path, &new_content)?;
        
        Ok(format!("Applied refactoring to line {}", suggestion.line))
    }
}
