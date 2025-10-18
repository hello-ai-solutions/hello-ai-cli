use serde::Deserialize;
use std::fs;
use std::path::Path;
use colored::*;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct FsRead {
    pub operations: Vec<FsReadOperation>,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "mode")]
pub enum FsReadOperation {
    Line(FsLine),
    Directory(FsDirectory),
    Search(FsSearch),
}

#[derive(Debug, Clone, Deserialize)]
pub struct FsLine {
    pub path: String,
    #[serde(default = "default_start_line")]
    pub start_line: i32,
    #[serde(default = "default_end_line")]
    pub end_line: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FsDirectory {
    pub path: String,
    #[serde(default)]
    pub depth: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FsSearch {
    pub path: String,
    pub pattern: String,
    #[serde(default = "default_context_lines")]
    pub context_lines: u32,
}

fn default_start_line() -> i32 { 1 }
fn default_end_line() -> i32 { -1 }
fn default_context_lines() -> u32 { 2 }

impl FsRead {
    pub async fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut results = Vec::new();
        
        for operation in &self.operations {
            let result = match operation {
                FsReadOperation::Line(line_op) => self.read_lines(line_op).await?,
                FsReadOperation::Directory(dir_op) => self.read_directory(dir_op).await?,
                FsReadOperation::Search(search_op) => self.search_files(search_op).await?,
            };
            results.push(result);
        }
        
        Ok(results.join("\n\n"))
    }
    
    async fn read_lines(&self, op: &FsLine) -> Result<String, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(&op.path)?;
        let lines: Vec<&str> = content.lines().collect();
        
        let start = if op.start_line < 0 {
            (lines.len() as i32 + op.start_line + 1).max(1) as usize
        } else {
            (op.start_line - 1).max(0) as usize
        };
        
        let end = if op.end_line < 0 {
            lines.len()
        } else {
            (op.end_line as usize).min(lines.len())
        };
        
        let selected_lines: Vec<String> = lines[start..end]
            .iter()
            .enumerate()
            .map(|(i, line)| format!("{:4}: {}", start + i + 1, line))
            .collect();
            
        Ok(format!("File: {}\n{}", op.path, selected_lines.join("\n")))
    }
    
    async fn read_directory(&self, op: &FsDirectory) -> Result<String, Box<dyn std::error::Error>> {
        let mut entries = Vec::new();
        self.read_dir_recursive(Path::new(&op.path), 0, op.depth, &mut entries)?;
        
        Ok(format!("📂 {}: {}\n\n{}", "Directory".bright_blue().bold(), op.path.bright_cyan(), entries.join("\n")))
    }
    
    fn read_dir_recursive(&self, path: &Path, current_depth: u32, max_depth: u32, entries: &mut Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        if current_depth > max_depth {
            return Ok(());
        }
        
        let dir_entries = fs::read_dir(path)?;
        for entry in dir_entries {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let name = entry.file_name().to_string_lossy().to_string();
            let indent = "  ".repeat(current_depth as usize);
            
            if file_type.is_dir() {
                entries.push(format!("{}📁 {}", indent, name.bright_blue().bold()));
                if current_depth < max_depth {
                    self.read_dir_recursive(&entry.path(), current_depth + 1, max_depth, entries)?;
                }
            } else {
                let colored_name = match name.split('.').last() {
                    Some("rs") => name.bright_red(),
                    Some("toml") | Some("yaml") | Some("yml") | Some("json") => name.bright_yellow(),
                    Some("md") => name.bright_green(),
                    Some("txt") => name.white(),
                    _ => name.bright_white(),
                };
                entries.push(format!("{}📄 {}", indent, colored_name));
            }
        }
        
        Ok(())
    }
    
    async fn search_files(&self, op: &FsSearch) -> Result<String, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(&op.path)?;
        let lines: Vec<&str> = content.lines().collect();
        let mut matches = Vec::new();
        
        for (line_num, line) in lines.iter().enumerate() {
            if line.to_lowercase().contains(&op.pattern.to_lowercase()) {
                let start = (line_num as i32 - op.context_lines as i32).max(0) as usize;
                let end = (line_num + op.context_lines as usize + 1).min(lines.len());
                
                let context: Vec<String> = lines[start..end]
                    .iter()
                    .enumerate()
                    .map(|(i, l)| {
                        let actual_line_num = start + i + 1;
                        if actual_line_num == line_num + 1 {
                            format!("→ {:4}: {}", actual_line_num, l)
                        } else {
                            format!("  {:4}: {}", actual_line_num, l)
                        }
                    })
                    .collect();
                    
                matches.push(context.join("\n"));
            }
        }
        
        Ok(format!("Search in {}: '{}'\n{}", op.path, op.pattern, matches.join("\n---\n")))
    }
}
