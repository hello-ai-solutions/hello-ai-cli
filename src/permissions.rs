use regex::Regex;

// Safe readonly commands that don't modify system state
pub const READONLY_COMMANDS: &[&str] = &[
    "ls", "cat", "echo", "pwd", "which", "head", "tail", "find", "grep", "dir", "type",
    "kubectl get", "kubectl describe", "kubectl logs", "kubectl config current-context",
    "kubectl cluster-info", "kubectl top", "kubectl explain",
    "aws sts get-caller-identity", "aws iam list-users", "aws iam list-roles", 
    "aws ec2 describe-instances", "aws s3 ls", "aws logs describe-log-groups",
    "docker ps", "docker images", "docker version", "docker info",
    "git status", "git log", "git branch", "git diff --name-only",
    "ps", "top", "df", "free", "uptime", "whoami", "id", "date"
];

// Dangerous patterns that should always require confirmation
const DANGEROUS_PATTERNS: &[&str] = &[
    "rm ", "delete", "destroy", "terminate", "kill", "shutdown", "reboot",
    "format", "mkfs", "dd if=", "chmod 777", "chown root",
    "kubectl delete", "kubectl apply", "kubectl create", "kubectl patch",
    "aws ec2 terminate", "aws s3 rm", "aws iam delete", "aws rds delete",
    "docker rm", "docker rmi", "docker kill", "docker stop",
    "sudo", "su -", "passwd", "userdel", "groupdel",
    ">", ">>", "|", "$", "`", "<(", "$("
];

pub struct PermissionChecker {
    allowed_commands: Vec<String>,
    allow_readonly: bool,
}

impl PermissionChecker {
    pub fn default() -> Self {
        Self {
            allowed_commands: vec![],
            allow_readonly: true,
        }
    }

    pub fn requires_confirmation(&self, command: &str) -> bool {
        // Always require confirmation for multi-line commands
        if command.contains('\n') || command.contains('\r') {
            return true;
        }

        // Check if command matches allowed patterns
        if self.is_explicitly_allowed(command) {
            return false;
        }

        // Pipeline or redirection still escalate (data exfil / mutation)
        if command.contains('|') || command.contains('>') {
            return true;
        }

        // Evaluate chained segments
        if command.contains("&&") || command.contains("||") || command.contains(';') {
            let parts = self.split_chain(command);
            let mut any_medium = false;
            for part in parts {
                if self.contains_dangerous_patterns(&part) {
                    return true; // High risk
                }
                if !self.is_readonly_command(&part) {
                    any_medium = true; // Medium risk
                }
            }
            return any_medium; // Medium → confirm, else Low → no confirm
        }

        // Check for dangerous patterns
        if self.contains_dangerous_patterns(command) {
            return true;
        }

        // Check if it's a readonly command and we allow them
        if self.allow_readonly && self.is_readonly_command(command) {
            return false;
        }

        // Default to requiring confirmation
        true
    }

    fn split_chain(&self, cmd: &str) -> Vec<String> {
        let mut parts = Vec::new();
        let mut buf = String::new();
        let mut chars = cmd.chars().peekable();
        
        while let Some(ch) = chars.next() {
            match ch {
                ';' => {
                    if !buf.trim().is_empty() {
                        parts.push(buf.trim().to_string());
                    }
                    buf.clear();
                }
                '&' => {
                    if chars.peek() == Some(&'&') {
                        chars.next(); // consume second &
                        if !buf.trim().is_empty() {
                            parts.push(buf.trim().to_string());
                        }
                        buf.clear();
                    } else {
                        buf.push(ch);
                    }
                }
                '|' => {
                    if chars.peek() == Some(&'|') {
                        chars.next(); // consume second |
                        if !buf.trim().is_empty() {
                            parts.push(buf.trim().to_string());
                        }
                        buf.clear();
                    } else {
                        buf.push(ch);
                    }
                }
                _ => buf.push(ch),
            }
        }
        
        if !buf.trim().is_empty() {
            parts.push(buf.trim().to_string());
        }
        
        parts
    }

    fn is_explicitly_allowed(&self, command: &str) -> bool {
        self.allowed_commands
            .iter()
            .any(|pattern| {
                if let Ok(regex) = Regex::new(&format!(r"\A{}\z", pattern)) {
                    regex.is_match(command)
                } else {
                    command == pattern
                }
            })
    }

    fn contains_dangerous_patterns(&self, command: &str) -> bool {
        let command_lower = command.to_lowercase();
        DANGEROUS_PATTERNS
            .iter()
            .any(|pattern| command_lower.contains(&pattern.to_lowercase()))
    }

    fn is_readonly_command(&self, command: &str) -> bool {
        let command_lower = command.to_lowercase();
        READONLY_COMMANDS
            .iter()
            .any(|readonly_cmd| command_lower.starts_with(&readonly_cmd.to_lowercase()))
    }

    pub fn get_risk_level(&self, command: &str) -> RiskLevel {
        if self.contains_dangerous_patterns(command) {
            RiskLevel::High
        } else if self.is_readonly_command(command) {
            RiskLevel::Low
        } else {
            RiskLevel::Medium
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RiskLevel {
    Low,    // Readonly commands
    Medium, // Regular commands
    High,   // Dangerous commands
}
