use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CicdAnalysis {
    pub pipeline_type: String,
    pub config_files: Vec<String>,
    pub stages: Vec<PipelineStage>,
    pub issues: Vec<CicdIssue>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStage {
    pub name: String,
    pub stage_type: String,
    pub commands: Vec<String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CicdIssue {
    pub issue_type: String,
    pub severity: String,
    pub description: String,
    pub file: String,
    pub suggestion: String,
}

pub struct CicdIntegrator {
    root_path: String,
}

impl CicdIntegrator {
    pub fn new(root_path: &str) -> Self {
        Self {
            root_path: root_path.to_string(),
        }
    }

    pub async fn analyze_pipeline(&self) -> Result<CicdAnalysis, Box<dyn std::error::Error>> {
        let pipeline_type = self.detect_pipeline_type();
        let config_files = self.find_config_files();
        let stages = self.parse_pipeline_stages(&config_files).await?;
        let issues = self.detect_pipeline_issues(&stages, &config_files);
        let recommendations = self.generate_recommendations(&issues, &stages);

        Ok(CicdAnalysis {
            pipeline_type,
            config_files,
            stages,
            issues,
            recommendations,
        })
    }

    fn detect_pipeline_type(&self) -> String {
        let root = Path::new(&self.root_path);
        
        if root.join(".github/workflows").exists() {
            "GitHub Actions".to_string()
        } else if root.join("Jenkinsfile").exists() {
            "Jenkins".to_string()
        } else if root.join(".gitlab-ci.yml").exists() {
            "GitLab CI".to_string()
        } else if root.join(".circleci/config.yml").exists() {
            "CircleCI".to_string()
        } else {
            "None detected".to_string()
        }
    }

    fn find_config_files(&self) -> Vec<String> {
        let mut files = Vec::new();
        let root = Path::new(&self.root_path);

        // GitHub Actions
        if let Ok(workflows_dir) = fs::read_dir(root.join(".github/workflows")) {
            for entry in workflows_dir {
                if let Ok(entry) = entry {
                    if entry.path().extension().map_or(false, |ext| ext == "yml" || ext == "yaml") {
                        files.push(entry.path().to_string_lossy().to_string());
                    }
                }
            }
        }

        // Jenkins
        if root.join("Jenkinsfile").exists() {
            files.push(root.join("Jenkinsfile").to_string_lossy().to_string());
        }

        // GitLab CI
        if root.join(".gitlab-ci.yml").exists() {
            files.push(root.join(".gitlab-ci.yml").to_string_lossy().to_string());
        }

        files
    }

    async fn parse_pipeline_stages(&self, config_files: &[String]) -> Result<Vec<PipelineStage>, Box<dyn std::error::Error>> {
        let mut stages = Vec::new();

        for file_path in config_files {
            if let Ok(content) = fs::read_to_string(file_path) {
                if file_path.contains(".github/workflows") {
                    stages.extend(self.parse_github_actions(&content));
                } else if file_path.contains("Jenkinsfile") {
                    stages.extend(self.parse_jenkinsfile(&content));
                } else if file_path.contains(".gitlab-ci.yml") {
                    stages.extend(self.parse_gitlab_ci(&content));
                }
            }
        }

        Ok(stages)
    }

    fn parse_github_actions(&self, content: &str) -> Vec<PipelineStage> {
        let mut stages = Vec::new();
        let mut current_job = None;
        let mut current_commands = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            
            if trimmed.ends_with(':') && !trimmed.starts_with('-') && !trimmed.starts_with('#') {
                if let Some(job_name) = current_job.take() {
                    stages.push(PipelineStage {
                        name: job_name,
                        stage_type: "job".to_string(),
                        commands: current_commands.clone(),
                        dependencies: Vec::new(),
                    });
                    current_commands.clear();
                }
                current_job = Some(trimmed.trim_end_matches(':').to_string());
            } else if trimmed.starts_with("- run:") || trimmed.starts_with("run:") {
                let cmd = trimmed.strip_prefix("- run:").or_else(|| trimmed.strip_prefix("run:"))
                    .unwrap_or(trimmed).trim().to_string();
                current_commands.push(cmd);
            }
        }

        if let Some(job_name) = current_job {
            stages.push(PipelineStage {
                name: job_name,
                stage_type: "job".to_string(),
                commands: current_commands,
                dependencies: Vec::new(),
            });
        }

        stages
    }

    fn parse_jenkinsfile(&self, content: &str) -> Vec<PipelineStage> {
        let mut stages = Vec::new();
        let mut in_stage = false;
        let mut current_stage = None;
        let mut current_commands = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("stage(") {
                if let Some(stage_name) = current_stage.take() {
                    stages.push(PipelineStage {
                        name: stage_name,
                        stage_type: "stage".to_string(),
                        commands: current_commands.clone(),
                        dependencies: Vec::new(),
                    });
                    current_commands.clear();
                }
                
                if let Some(start) = trimmed.find('\'') {
                    if let Some(end) = trimmed[start + 1..].find('\'') {
                        current_stage = Some(trimmed[start + 1..start + 1 + end].to_string());
                        in_stage = true;
                    }
                }
            } else if in_stage && (trimmed.starts_with("sh ") || trimmed.contains("sh '")) {
                current_commands.push(trimmed.to_string());
            }
        }

        if let Some(stage_name) = current_stage {
            stages.push(PipelineStage {
                name: stage_name,
                stage_type: "stage".to_string(),
                commands: current_commands,
                dependencies: Vec::new(),
            });
        }

        stages
    }

    fn parse_gitlab_ci(&self, content: &str) -> Vec<PipelineStage> {
        let mut stages = Vec::new();
        let mut current_job = None;
        let mut current_commands = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            
            if trimmed.ends_with(':') && !trimmed.starts_with('-') && !trimmed.starts_with('#') {
                if let Some(job_name) = current_job.take() {
                    stages.push(PipelineStage {
                        name: job_name,
                        stage_type: "job".to_string(),
                        commands: current_commands.clone(),
                        dependencies: Vec::new(),
                    });
                    current_commands.clear();
                }
                current_job = Some(trimmed.trim_end_matches(':').to_string());
            } else if trimmed.starts_with("- ") && current_job.is_some() {
                current_commands.push(trimmed.strip_prefix("- ").unwrap().to_string());
            }
        }

        if let Some(job_name) = current_job {
            stages.push(PipelineStage {
                name: job_name,
                stage_type: "job".to_string(),
                commands: current_commands,
                dependencies: Vec::new(),
            });
        }

        stages
    }

    fn detect_pipeline_issues(&self, stages: &[PipelineStage], config_files: &[String]) -> Vec<CicdIssue> {
        let mut issues = Vec::new();

        // Check for missing security scanning
        let has_security_scan = stages.iter().any(|stage| {
            stage.commands.iter().any(|cmd| {
                cmd.contains("security") || cmd.contains("vulnerability") || cmd.contains("audit")
            })
        });

        if !has_security_scan {
            issues.push(CicdIssue {
                issue_type: "Missing Security Scan".to_string(),
                severity: "high".to_string(),
                description: "No security scanning detected in pipeline".to_string(),
                file: config_files.first().unwrap_or(&"pipeline".to_string()).clone(),
                suggestion: "Add security scanning step (e.g., cargo audit, npm audit, snyk)".to_string(),
            });
        }

        // Check for missing tests
        let has_tests = stages.iter().any(|stage| {
            stage.commands.iter().any(|cmd| {
                cmd.contains("test") || cmd.contains("cargo test") || cmd.contains("npm test")
            })
        });

        if !has_tests {
            issues.push(CicdIssue {
                issue_type: "Missing Tests".to_string(),
                severity: "high".to_string(),
                description: "No test execution detected in pipeline".to_string(),
                file: config_files.first().unwrap_or(&"pipeline".to_string()).clone(),
                suggestion: "Add test execution step".to_string(),
            });
        }

        // Check for hardcoded secrets
        for file_path in config_files {
            if let Ok(content) = fs::read_to_string(file_path) {
                if content.contains("password") || content.contains("api_key") || content.contains("secret") {
                    issues.push(CicdIssue {
                        issue_type: "Potential Hardcoded Secret".to_string(),
                        severity: "critical".to_string(),
                        description: "Potential hardcoded secrets in pipeline configuration".to_string(),
                        file: file_path.clone(),
                        suggestion: "Use environment variables or secret management".to_string(),
                    });
                }
            }
        }

        issues
    }

    fn generate_recommendations(&self, issues: &[CicdIssue], stages: &[PipelineStage]) -> Vec<String> {
        let mut recommendations = Vec::new();

        if issues.iter().any(|i| i.issue_type == "Missing Security Scan") {
            recommendations.push("Add automated security scanning to your pipeline".to_string());
        }

        if issues.iter().any(|i| i.issue_type == "Missing Tests") {
            recommendations.push("Include comprehensive test execution in your CI/CD pipeline".to_string());
        }

        if stages.len() < 3 {
            recommendations.push("Consider adding more pipeline stages (build, test, deploy)".to_string());
        }

        if !stages.iter().any(|s| s.name.to_lowercase().contains("deploy")) {
            recommendations.push("Add deployment automation to your pipeline".to_string());
        }

        recommendations.push("Use parallel execution for independent stages to improve build times".to_string());
        recommendations.push("Implement proper artifact management and caching".to_string());

        recommendations
    }

    pub async fn generate_github_action(&self, project_type: &str) -> String {
        match project_type.to_lowercase().as_str() {
            "rust" => self.generate_rust_github_action(),
            "node.js" => self.generate_nodejs_github_action(),
            "python" => self.generate_python_github_action(),
            _ => self.generate_generic_github_action(),
        }
    }

    fn generate_rust_github_action(&self) -> String {
        r#"name: Rust CI/CD

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    - name: Cache cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
    - name: Run tests
      run: cargo test --verbose
    - name: Security audit
      run: |
        cargo install cargo-audit
        cargo audit
    - name: Lint
      run: |
        rustup component add clippy
        cargo clippy -- -D warnings

  build:
    needs: test
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    - name: Build
      run: cargo build --release
    - name: Upload artifacts
      uses: actions/upload-artifact@v3
      with:
        name: rust-binary
        path: target/release/
"#.to_string()
    }

    fn generate_nodejs_github_action(&self) -> String {
        r#"name: Node.js CI/CD

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        node-version: [16.x, 18.x, 20.x]
    steps:
    - uses: actions/checkout@v3
    - name: Use Node.js ${{ matrix.node-version }}
      uses: actions/setup-node@v3
      with:
        node-version: ${{ matrix.node-version }}
        cache: 'npm'
    - run: npm ci
    - run: npm run build --if-present
    - run: npm test
    - name: Security audit
      run: npm audit
    - name: Lint
      run: npm run lint --if-present

  deploy:
    needs: test
    runs-on: ubuntu-latest
    if: github.ref == 'refs/heads/main'
    steps:
    - uses: actions/checkout@v3
    - name: Setup Node.js
      uses: actions/setup-node@v3
      with:
        node-version: '18.x'
        cache: 'npm'
    - run: npm ci
    - run: npm run build
    - name: Deploy
      run: echo "Add your deployment steps here"
"#.to_string()
    }

    fn generate_python_github_action(&self) -> String {
        r#"name: Python CI/CD

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        python-version: [3.8, 3.9, '3.10', '3.11']
    steps:
    - uses: actions/checkout@v3
    - name: Set up Python ${{ matrix.python-version }}
      uses: actions/setup-python@v4
      with:
        python-version: ${{ matrix.python-version }}
    - name: Install dependencies
      run: |
        python -m pip install --upgrade pip
        pip install flake8 pytest
        if [ -f requirements.txt ]; then pip install -r requirements.txt; fi
    - name: Lint with flake8
      run: |
        flake8 . --count --select=E9,F63,F7,F82 --show-source --statistics
        flake8 . --count --exit-zero --max-complexity=10 --max-line-length=127 --statistics
    - name: Test with pytest
      run: pytest
    - name: Security check
      run: |
        pip install safety
        safety check
"#.to_string()
    }

    fn generate_generic_github_action(&self) -> String {
        r#"name: CI/CD Pipeline

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  build-and-test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Setup environment
      run: echo "Configure your build environment here"
    - name: Install dependencies
      run: echo "Install project dependencies"
    - name: Run tests
      run: echo "Execute your test suite"
    - name: Security scan
      run: echo "Run security scanning tools"
    - name: Build
      run: echo "Build your application"
    - name: Deploy
      if: github.ref == 'refs/heads/main'
      run: echo "Deploy to production"
"#.to_string()
    }

    pub fn generate_report(&self, analysis: &CicdAnalysis) -> String {
        format!(
            "# CI/CD Pipeline Analysis\n\n\
            ## 🔧 Pipeline Configuration\n\
            - **Type**: {}\n\
            - **Config Files**: {}\n\
            - **Stages**: {} detected\n\n\
            ## 📋 Pipeline Stages\n{}\n\n\
            ## ⚠️ Issues Found\n{}\n\n\
            ## 💡 Recommendations\n{}\n",
            analysis.pipeline_type,
            analysis.config_files.len(),
            analysis.stages.len(),
            if analysis.stages.is_empty() {
                "- No stages detected".to_string()
            } else {
                analysis.stages.iter()
                    .map(|s| format!("- **{}** ({}): {} commands", s.name, s.stage_type, s.commands.len()))
                    .collect::<Vec<_>>()
                    .join("\n")
            },
            if analysis.issues.is_empty() {
                "- No issues detected".to_string()
            } else {
                analysis.issues.iter()
                    .map(|i| format!("- **{}**: {} - {}", i.severity.to_uppercase(), i.description, i.suggestion))
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
