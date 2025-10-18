use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryAnalysis {
    pub project_type: String,
    pub languages: Vec<String>,
    pub total_files: usize,
    pub total_lines: usize,
    pub dependencies: Vec<Dependency>,
    pub architecture: ArchitectureAnalysis,
    pub issues: Vec<ProjectIssue>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: Option<String>,
    pub dep_type: String, // internal, external, dev
    pub used_by: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureAnalysis {
    pub patterns: Vec<String>,
    pub layers: Vec<String>,
    pub coupling: String, // low, medium, high
    pub complexity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectIssue {
    pub issue_type: String,
    pub severity: String,
    pub description: String,
    pub files_affected: Vec<String>,
    pub suggestion: String,
}

pub struct RepositoryAnalyzer {
    root_path: PathBuf,
    file_map: HashMap<String, FileInfo>,
}

#[derive(Debug, Clone)]
struct FileInfo {
    path: String,
    language: String,
    lines: usize,
    imports: Vec<String>,
    exports: Vec<String>,
    functions: Vec<String>,
}

impl RepositoryAnalyzer {
    pub fn new(root_path: &str) -> Self {
        Self {
            root_path: PathBuf::from(root_path),
            file_map: HashMap::new(),
        }
    }

    pub async fn analyze_repository(&mut self) -> Result<RepositoryAnalysis, Box<dyn std::error::Error>> {
        self.scan_files().await?;
        
        let project_type = self.detect_project_type();
        let languages = self.get_languages();
        let dependencies = self.analyze_dependencies().await?;
        let architecture = self.analyze_architecture();
        let issues = self.find_project_issues();
        let recommendations = self.generate_recommendations(&issues, &architecture);

        Ok(RepositoryAnalysis {
            project_type,
            languages,
            total_files: self.file_map.len(),
            total_lines: self.file_map.values().map(|f| f.lines).sum(),
            dependencies,
            architecture,
            issues,
            recommendations,
        })
    }

    async fn scan_files(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.scan_directory(&self.root_path.clone()).await?;
        Ok(())
    }

    async fn scan_directory(&mut self, dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    let dir_name = path.file_name().unwrap().to_str().unwrap();
                    if !["target", "node_modules", ".git", "build", "dist"].contains(&dir_name) {
                        Box::pin(self.scan_directory(&path)).await?;
                    }
                } else if self.is_source_file(&path) {
                    if let Ok(file_info) = self.analyze_file(&path).await {
                        self.file_map.insert(path.to_string_lossy().to_string(), file_info);
                    }
                }
            }
        }
        Ok(())
    }

    fn is_source_file(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            matches!(ext, "rs" | "py" | "js" | "ts" | "java" | "go" | "cpp" | "c" | "h")
        } else {
            false
        }
    }

    async fn analyze_file(&self, path: &Path) -> Result<FileInfo, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let language = self.detect_language(path);
        let lines = content.lines().count();
        let imports = self.extract_imports(&content, &language);
        let exports = self.extract_exports(&content, &language);
        let functions = self.extract_functions(&content, &language);

        Ok(FileInfo {
            path: path.to_string_lossy().to_string(),
            language,
            lines,
            imports,
            exports,
            functions,
        })
    }

    fn detect_language(&self, path: &Path) -> String {
        match path.extension().and_then(|s| s.to_str()) {
            Some("rs") => "rust".to_string(),
            Some("py") => "python".to_string(),
            Some("js") => "javascript".to_string(),
            Some("ts") => "typescript".to_string(),
            Some("java") => "java".to_string(),
            Some("go") => "go".to_string(),
            _ => "unknown".to_string(),
        }
    }

    fn extract_imports(&self, content: &str, language: &str) -> Vec<String> {
        let mut imports = Vec::new();
        
        for line in content.lines() {
            let trimmed = line.trim();
            match language {
                "rust" => {
                    if trimmed.starts_with("use ") {
                        imports.push(trimmed.to_string());
                    }
                },
                "python" => {
                    if trimmed.starts_with("import ") || trimmed.starts_with("from ") {
                        imports.push(trimmed.to_string());
                    }
                },
                "javascript" | "typescript" => {
                    if trimmed.starts_with("import ") || trimmed.contains("require(") {
                        imports.push(trimmed.to_string());
                    }
                },
                _ => {}
            }
        }
        
        imports
    }

    fn extract_exports(&self, content: &str, language: &str) -> Vec<String> {
        let mut exports = Vec::new();
        
        for line in content.lines() {
            let trimmed = line.trim();
            match language {
                "rust" => {
                    if trimmed.starts_with("pub fn ") || trimmed.starts_with("pub struct ") {
                        exports.push(trimmed.to_string());
                    }
                },
                "javascript" | "typescript" => {
                    if trimmed.starts_with("export ") {
                        exports.push(trimmed.to_string());
                    }
                },
                _ => {}
            }
        }
        
        exports
    }

    fn extract_functions(&self, content: &str, language: &str) -> Vec<String> {
        let mut functions = Vec::new();
        
        for line in content.lines() {
            let trimmed = line.trim();
            match language {
                "rust" => {
                    if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
                        if let Some(name) = self.extract_function_name(trimmed, "fn ") {
                            functions.push(name);
                        }
                    }
                },
                "python" => {
                    if trimmed.starts_with("def ") {
                        if let Some(name) = self.extract_function_name(trimmed, "def ") {
                            functions.push(name);
                        }
                    }
                },
                _ => {}
            }
        }
        
        functions
    }

    fn extract_function_name(&self, line: &str, prefix: &str) -> Option<String> {
        if let Some(start) = line.find(prefix) {
            let after_prefix = &line[start + prefix.len()..];
            if let Some(paren_pos) = after_prefix.find('(') {
                Some(after_prefix[..paren_pos].trim().to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn detect_project_type(&self) -> String {
        if self.root_path.join("Cargo.toml").exists() {
            "Rust Project".to_string()
        } else if self.root_path.join("package.json").exists() {
            "Node.js Project".to_string()
        } else if self.root_path.join("requirements.txt").exists() || self.root_path.join("setup.py").exists() {
            "Python Project".to_string()
        } else if self.root_path.join("pom.xml").exists() {
            "Java Maven Project".to_string()
        } else if self.root_path.join("go.mod").exists() {
            "Go Project".to_string()
        } else {
            "Mixed/Unknown Project".to_string()
        }
    }

    fn get_languages(&self) -> Vec<String> {
        let mut languages: HashSet<String> = HashSet::new();
        for file_info in self.file_map.values() {
            languages.insert(file_info.language.clone());
        }
        languages.into_iter().collect()
    }

    async fn analyze_dependencies(&self) -> Result<Vec<Dependency>, Box<dyn std::error::Error>> {
        let mut dependencies = Vec::new();
        
        // Analyze internal dependencies
        let mut internal_deps: HashMap<String, Vec<String>> = HashMap::new();
        
        for (file_path, file_info) in &self.file_map {
            for import in &file_info.imports {
                // Check if import refers to internal module
                if self.is_internal_import(import) {
                    internal_deps.entry(import.clone())
                        .or_insert_with(Vec::new)
                        .push(file_path.clone());
                }
            }
        }
        
        for (dep_name, used_by) in internal_deps {
            dependencies.push(Dependency {
                name: dep_name,
                version: None,
                dep_type: "internal".to_string(),
                used_by,
            });
        }
        
        // Analyze external dependencies from config files
        dependencies.extend(self.analyze_external_dependencies().await?);
        
        Ok(dependencies)
    }

    fn is_internal_import(&self, import: &str) -> bool {
        // Simple heuristic: if import contains local path indicators
        import.contains("./") || import.contains("../") || import.starts_with("crate::")
    }

    async fn analyze_external_dependencies(&self) -> Result<Vec<Dependency>, Box<dyn std::error::Error>> {
        let mut dependencies = Vec::new();
        
        // Rust dependencies
        if let Ok(cargo_toml) = fs::read_to_string(self.root_path.join("Cargo.toml")) {
            dependencies.extend(self.parse_cargo_dependencies(&cargo_toml));
        }
        
        // Node.js dependencies
        if let Ok(package_json) = fs::read_to_string(self.root_path.join("package.json")) {
            dependencies.extend(self.parse_npm_dependencies(&package_json));
        }
        
        Ok(dependencies)
    }

    fn parse_cargo_dependencies(&self, content: &str) -> Vec<Dependency> {
        let mut dependencies = Vec::new();
        let mut in_dependencies = false;
        
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed == "[dependencies]" {
                in_dependencies = true;
                continue;
            }
            if trimmed.starts_with('[') && trimmed != "[dependencies]" {
                in_dependencies = false;
            }
            
            if in_dependencies && trimmed.contains('=') {
                if let Some(eq_pos) = trimmed.find('=') {
                    let name = trimmed[..eq_pos].trim().to_string();
                    let version = trimmed[eq_pos + 1..].trim().trim_matches('"').to_string();
                    dependencies.push(Dependency {
                        name,
                        version: Some(version),
                        dep_type: "external".to_string(),
                        used_by: vec!["Cargo.toml".to_string()],
                    });
                }
            }
        }
        
        dependencies
    }

    fn parse_npm_dependencies(&self, content: &str) -> Vec<Dependency> {
        // Simple JSON parsing for dependencies
        let mut dependencies = Vec::new();
        
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(deps) = json.get("dependencies").and_then(|d| d.as_object()) {
                for (name, version) in deps {
                    dependencies.push(Dependency {
                        name: name.clone(),
                        version: Some(version.as_str().unwrap_or("").to_string()),
                        dep_type: "external".to_string(),
                        used_by: vec!["package.json".to_string()],
                    });
                }
            }
        }
        
        dependencies
    }

    fn analyze_architecture(&self) -> ArchitectureAnalysis {
        let patterns = self.detect_patterns();
        let layers = self.detect_layers();
        let coupling = self.calculate_coupling();
        let complexity = self.calculate_complexity();

        ArchitectureAnalysis {
            patterns,
            layers,
            coupling,
            complexity,
        }
    }

    fn detect_patterns(&self) -> Vec<String> {
        let mut patterns = Vec::new();
        
        // Check for common patterns
        if self.has_mvc_structure() {
            patterns.push("MVC (Model-View-Controller)".to_string());
        }
        if self.has_microservice_structure() {
            patterns.push("Microservices".to_string());
        }
        if self.has_layered_structure() {
            patterns.push("Layered Architecture".to_string());
        }
        
        patterns
    }

    fn has_mvc_structure(&self) -> bool {
        let has_models = self.file_map.keys().any(|path| path.contains("model"));
        let has_views = self.file_map.keys().any(|path| path.contains("view"));
        let has_controllers = self.file_map.keys().any(|path| path.contains("controller"));
        has_models && has_views && has_controllers
    }

    fn has_microservice_structure(&self) -> bool {
        self.file_map.keys().any(|path| path.contains("service")) && 
        self.file_map.len() > 10
    }

    fn has_layered_structure(&self) -> bool {
        let layers = ["src", "lib", "api", "core", "domain"];
        layers.iter().any(|layer| {
            self.file_map.keys().any(|path| path.contains(layer))
        })
    }

    fn detect_layers(&self) -> Vec<String> {
        let mut layers = Vec::new();
        
        let common_layers = [
            ("presentation", vec!["ui", "view", "controller"]),
            ("business", vec!["service", "domain", "core"]),
            ("data", vec!["repository", "dao", "model"]),
            ("infrastructure", vec!["config", "util", "helper"]),
        ];
        
        for (layer_name, keywords) in common_layers {
            if keywords.iter().any(|keyword| {
                self.file_map.keys().any(|path| path.contains(keyword))
            }) {
                layers.push(layer_name.to_string());
            }
        }
        
        layers
    }

    fn calculate_coupling(&self) -> String {
        let total_files = self.file_map.len();
        let total_imports: usize = self.file_map.values().map(|f| f.imports.len()).sum();
        
        if total_files == 0 {
            return "unknown".to_string();
        }
        
        let avg_imports = total_imports as f64 / total_files as f64;
        
        if avg_imports < 3.0 {
            "low".to_string()
        } else if avg_imports < 8.0 {
            "medium".to_string()
        } else {
            "high".to_string()
        }
    }

    fn calculate_complexity(&self) -> String {
        let total_lines: usize = self.file_map.values().map(|f| f.lines).sum();
        
        if total_lines < 1000 {
            "low".to_string()
        } else if total_lines < 10000 {
            "medium".to_string()
        } else {
            "high".to_string()
        }
    }

    fn find_project_issues(&self) -> Vec<ProjectIssue> {
        let mut issues = Vec::new();
        
        // Large files
        for (path, file_info) in &self.file_map {
            if file_info.lines > 500 {
                issues.push(ProjectIssue {
                    issue_type: "Large File".to_string(),
                    severity: "medium".to_string(),
                    description: format!("File has {} lines", file_info.lines),
                    files_affected: vec![path.clone()],
                    suggestion: "Consider breaking into smaller modules".to_string(),
                });
            }
        }
        
        // Circular dependencies (simplified check)
        if self.has_potential_circular_deps() {
            issues.push(ProjectIssue {
                issue_type: "Circular Dependencies".to_string(),
                severity: "high".to_string(),
                description: "Potential circular dependencies detected".to_string(),
                files_affected: vec!["Multiple files".to_string()],
                suggestion: "Review import structure and refactor to remove cycles".to_string(),
            });
        }
        
        issues
    }

    fn has_potential_circular_deps(&self) -> bool {
        // Simplified heuristic: high coupling might indicate circular deps
        self.calculate_coupling() == "high"
    }

    fn generate_recommendations(&self, issues: &[ProjectIssue], architecture: &ArchitectureAnalysis) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if architecture.complexity == "high" {
            recommendations.push("Consider refactoring complex modules into smaller components".to_string());
        }
        
        if architecture.coupling == "high" {
            recommendations.push("Reduce coupling by introducing interfaces and dependency injection".to_string());
        }
        
        if issues.iter().any(|i| i.issue_type == "Large File") {
            recommendations.push("Break large files into focused, single-responsibility modules".to_string());
        }
        
        if self.file_map.values().any(|f| f.functions.len() > 20) {
            recommendations.push("Consider splitting files with many functions into separate modules".to_string());
        }
        
        recommendations
    }

    pub fn generate_report(&self, analysis: &RepositoryAnalysis) -> String {
        format!(
            "# Repository Analysis Report\n\n\
            ## 📊 Project Overview\n\
            - **Type**: {}\n\
            - **Languages**: {}\n\
            - **Files**: {} ({} lines of code)\n\n\
            ## 🏗️ Architecture\n\
            - **Patterns**: {}\n\
            - **Layers**: {}\n\
            - **Coupling**: {}\n\
            - **Complexity**: {}\n\n\
            ## 📦 Dependencies\n\
            - **Total**: {} dependencies\n\
            - **External**: {}\n\
            - **Internal**: {}\n\n\
            ## ⚠️ Issues Found\n{}\n\n\
            ## 💡 Recommendations\n{}\n",
            analysis.project_type,
            analysis.languages.join(", "),
            analysis.total_files,
            analysis.total_lines,
            analysis.architecture.patterns.join(", "),
            analysis.architecture.layers.join(", "),
            analysis.architecture.coupling,
            analysis.architecture.complexity,
            analysis.dependencies.len(),
            analysis.dependencies.iter().filter(|d| d.dep_type == "external").count(),
            analysis.dependencies.iter().filter(|d| d.dep_type == "internal").count(),
            if analysis.issues.is_empty() {
                "- No major issues detected".to_string()
            } else {
                analysis.issues.iter()
                    .map(|i| format!("- **{}**: {} ({})", i.severity.to_uppercase(), i.description, i.suggestion))
                    .collect::<Vec<_>>()
                    .join("\n")
            },
            analysis.recommendations.iter()
                .map(|r| format!("- {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
