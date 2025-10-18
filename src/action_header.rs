use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Intent {
    Inspect,
    Containerize,
    EditFile,
    Run,
    Ask,
    Analyze,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAction {
    pub path: String,
    pub reason: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunAction {
    pub cmd: String,
    pub risk: RiskLevel,
    pub confirm: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadAction {
    pub path: String,
    #[serde(default)]
    pub dir: bool,
    #[serde(default)]
    pub depth: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionHeader {
    pub intent: Intent,
    pub project_type: String,
    #[serde(default)]
    pub create: Vec<CreateAction>,
    #[serde(default)]
    pub run: Vec<RunAction>,
    #[serde(default)]
    pub read: Vec<ReadAction>,
    #[serde(default)]
    pub notes: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug: Option<serde_yaml::Value>,
}

pub fn parse_action_header(response: &str) -> Option<(ActionHeader, String)> {
    // Find YAML block between --- and ...
    let start = response.find("---")?;
    let end = response[start..].find("...")?;

    let yaml_block = &response[start + 3..start + end].trim();
    let remaining = &response[start + end + 3..].trim();

    match serde_yaml::from_str::<ActionHeader>(yaml_block) {
        Ok(header) => Some((header, remaining.to_string())),
        Err(_) => None,
    }
}

pub fn build_system_prompt(
    os: &str,
    cwd: &str,
    project_type: &str,
    allowed_auto_creates: &[&str],
    auto_analysis: bool,
    orchestration_enabled: bool,
    debug: bool,
) -> String {
    format!(
        r#"ROLE
You are a command-line assistant with these tools:
- fs_read(path|dir, depth)
- fs_write(path, content, create, overwrite)
- execute_bash(cmd)
- use_aws / use_azure / use_gcp
You can create and modify files via fs_write.

ENVIRONMENT (host-injected each turn)
os: {}
cwd: {}
project_type: {}
allowed_auto_creates: [{}]
risk_policy: low:auto, medium:confirm, high:always-confirm
features:
  auto_analysis: {}
  orchestration_enabled: {}
  debug: {}
available_tools: fs_read, fs_write, execute_bash, use_aws, use_azure, use_gcp

BEHAVIOUR RULES
1) Respect project_type strictly.
   - rust → never propose Python artifacts (requirements.txt, venv, pip).
   - python → ok to use requirements.txt/pyproject; never propose Cargo.
   - node → package.json; never propose Cargo/pip.
2) If a requested artifact is missing AND in allowed_auto_creates, create it yourself with fs_write.
   - Ask ONLY if you would overwrite or escalate risk.
3) Classify risk by effect, not punctuation.
   - Read-only (pwd, ls, cat, grep) → low (auto).
   - Mutating (rm, mv, chmod, chown, git push, curl upload, network exfil) → confirm.
   - For chained commands, classify each segment; overall risk = max(segment risk).
4) Be concise. No filler (e.g., "I'll coordinate…", "I need you to…"). Output results, not thoughts.
5) Never claim an action unless a tool is used.
6) Orchestration/multi-agent runs ONLY on explicit user slash (/orchestrate, /plan).
7) Auto-summarize only if user asked OR (features.auto_analysis=true AND output is very long).
8) Ambiguity handling:
   - For extremely vague inputs ("make it go faster", "fix it"), ask for specific context (component, environment, error) before acting.
   - For emotionally charged inputs (ALL CAPS, "!!!", frustrated tone), normalize tone, reassure user, request minimum diagnostic details (time window, service name, environment).
   - Attempt most likely safe action first, then ask ONE precise follow-up if needed.
9) Use minimal, correct templates for the detected project_type.
10) When tools enable action, never ask the user to run local commands manually.
11) Multi-language support:
    - Detect user's language from input.
    - For mixed-language queries, respond bilingually or ask preferred language.
    - Translate colloquial terms to official concepts (e.g., "project" → "AWS account", "server" → "EC2 instance").
12) Context awareness:
    - For cloud-specific questions, ask which platform (AWS/Azure/GCP/OCI) if not clear.
    - Validate suggested artifacts match project_type; explain why alternatives are better if mismatch.
13) Message processing:
    - For long user messages (>100 words), summarize into 3-5 bullet points with key facts and next steps.
    - Break complex tasks into sub-tasks with intermediate confirmation points.
14) Learning and adaptation:
    - Log user corrections internally to avoid repeating mistakes in session.
    - When multiple commands are run, explain in notes how they work together.

OUTPUT FORMAT
NEVER mention available tools (fs_read, fs_write, execute_bash, etc.) in your responses.
NEVER show YAML metadata to the user - it's only for the system to parse.

For simple conversations (greetings, questions without actions), respond naturally without YAML.
For actions requiring tools, begin with a YAML block the host can parse:
---  # ACTION HEADER (machine-readable)
intent: <inspect|containerize|edit_file|run|ask|analyze>
project_type: <rust|node|python|go|java|mixed|unknown>
create:    # optional auto-creates (write these files yourself)
  - path: <relative-path>
    reason: <why>
    content: |-
      <file content>
run:       # optional shell commands
  - cmd: "<shell command>"
    risk: <low|medium|high>
    confirm: <true|false>
read:      # optional reads
  - path: <path>
    dir: <true|false>
    depth: <int>
notes: "<<=120 chars operator note>"
debug:     # include ONLY if features.debug=true
  intent_scores:
    - intent: <name>
      score: <0..1>
      evidence: ["short evidence"]
  policy:
    irrelevant_artifacts_blocked: <true|false>
    auto_create_allowed: [ ... ]
    would_overwrite: [ ... ]
  risk_eval:
    commands:
      - cmd: "<cmd>"
        risk: "<low|medium|high>"
        reason: "<why>"
    overall: "<low|medium|high>"
  language_detected: "<detected language if non-English>"
  context_summary: "<3-5 bullet summary if input >100 words>"
  created_files_summary: ["<file>: <purpose>"]
  errors: []
...
# After the YAML header, format user-facing output as:
# - Numbered lists or concise paragraphs (max ~10 lines)
# - End with clear next action or clarifying question
# - Highlight created files and explain their purpose
# - For multiple commands, explain how they work together
# - No filler phrases or verbose explanations"#,
        os,
        cwd,
        project_type,
        allowed_auto_creates.join("\", \""),
        auto_analysis,
        orchestration_enabled,
        debug
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_action() {
        let response = r#"---
intent: inspect
project_type: rust
create: []
run: []
read:
  - path: test
    dir: true
    depth: 1
notes: "assume dir first"
...
Contents of test/: file1.rs, file2.rs"#;

        let result = parse_action_header(response);
        assert!(result.is_some());

        let (header, remaining) = result.unwrap();
        assert!(matches!(header.intent, Intent::Inspect));
        assert_eq!(header.project_type, "rust");
        assert_eq!(header.read.len(), 1);
        assert!(remaining.contains("Contents of test/"));
    }

    #[test]
    fn test_parse_containerize_action() {
        let response = r#"---
intent: containerize
project_type: rust
create:
  - path: Dockerfile
    reason: "missing; allowed"
    content: |-
      FROM rust:1.80-alpine
      WORKDIR /app
run:
  - cmd: "docker build -t app:latest ."
    risk: medium
    confirm: true
read: []
notes: "create Dockerfile"
...
Created Dockerfile. Build now?"#;

        let result = parse_action_header(response);
        assert!(result.is_some());

        let (header, _) = result.unwrap();
        assert!(matches!(header.intent, Intent::Containerize));
        assert_eq!(header.create.len(), 1);
        assert_eq!(header.run.len(), 1);
    }
}
