use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct FsWrite {
    pub command: String,
    pub path: String,
    pub file_text: Option<String>,
    pub new_str: Option<String>,
    pub old_str: Option<String>,
    pub insert_line: Option<u32>,
    pub summary: Option<String>,
}

impl FsWrite {
    pub async fn execute(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self.command.as_str() {
            "create" => self.create_file().await,
            "str_replace" => self.replace_string().await,
            "insert" => self.insert_text().await,
            "append" => self.append_text().await,
            _ => Err(format!("Unknown fs_write command: {}", self.command).into()),
        }
    }

    async fn create_file(&self) -> Result<String, Box<dyn std::error::Error>> {
        let content = self
            .file_text
            .as_ref()
            .ok_or("file_text is required for create command")?;

        // Create parent directories if they don't exist
        if let Some(parent) = Path::new(&self.path).parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&self.path, content)?;
        Ok(format!("Created file: {}", self.path))
    }

    async fn replace_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        let old_str = self
            .old_str
            .as_ref()
            .ok_or("old_str is required for str_replace command")?;
        let new_str = self
            .new_str
            .as_ref()
            .ok_or("new_str is required for str_replace command")?;

        let content = fs::read_to_string(&self.path)?;

        if !content.contains(old_str) {
            return Err(format!("String not found in file: {}", old_str).into());
        }

        let new_content = content.replace(old_str, new_str);
        fs::write(&self.path, new_content)?;

        Ok(format!("Replaced text in: {}", self.path))
    }

    async fn insert_text(&self) -> Result<String, Box<dyn std::error::Error>> {
        let insert_line = self
            .insert_line
            .ok_or("insert_line is required for insert command")?;
        let new_str = self
            .new_str
            .as_ref()
            .ok_or("new_str is required for insert command")?;

        let content = fs::read_to_string(&self.path)?;
        let mut lines: Vec<&str> = content.lines().collect();

        if insert_line as usize > lines.len() {
            return Err("insert_line exceeds file length".into());
        }

        lines.insert(insert_line as usize, new_str);
        let new_content = lines.join("\n");
        fs::write(&self.path, new_content)?;

        Ok(format!(
            "Inserted text at line {} in: {}",
            insert_line, self.path
        ))
    }

    async fn append_text(&self) -> Result<String, Box<dyn std::error::Error>> {
        let new_str = self
            .new_str
            .as_ref()
            .ok_or("new_str is required for append command")?;

        let mut content = fs::read_to_string(&self.path).unwrap_or_default();

        // Add newline if file doesn't end with one
        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }

        content.push_str(new_str);
        fs::write(&self.path, content)?;

        Ok(format!("Appended text to: {}", self.path))
    }
}
