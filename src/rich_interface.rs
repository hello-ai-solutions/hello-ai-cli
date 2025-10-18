use crate::Config;
use colored::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct RichInterface {
    syntax_highlighter: SyntaxHighlighter,
    formatter: ResponseFormatter,
}

pub struct SyntaxHighlighter {
    language_patterns: HashMap<String, Vec<(String, Color)>>,
}

pub struct ResponseFormatter {
    markdown_enabled: bool,
    code_block_style: String,
}

#[derive(Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    White,
    BrightRed,
    BrightGreen,
    BrightBlue,
    BrightYellow,
}

// Amazon Q color scheme constants
pub struct AmazonQTheme;

impl AmazonQTheme {
    // Amazon Q brand colors
    pub const PRIMARY_ORANGE: (u8, u8, u8) = (255, 153, 0); // #FF9900 - Amazon Orange
    pub const SECONDARY_BLUE: (u8, u8, u8) = (35, 47, 62); // #232F3E - Amazon Dark Blue
    pub const ACCENT_BLUE: (u8, u8, u8) = (0, 123, 255); // #007BFF - Bright Blue
    pub const SUCCESS_GREEN: (u8, u8, u8) = (40, 167, 69); // #28A745 - Success Green
    pub const WARNING_YELLOW: (u8, u8, u8) = (255, 193, 7); // #FFC107 - Warning Yellow
    pub const DANGER_RED: (u8, u8, u8) = (220, 53, 69); // #DC3545 - Danger Red
    pub const LIGHT_GRAY: (u8, u8, u8) = (248, 249, 250); // #F8F9FA - Light Gray
    pub const DARK_GRAY: (u8, u8, u8) = (108, 117, 125); // #6C757D - Dark Gray

    pub fn format_brand_header() -> String {
        format!(
            "{} {} {}",
            "hello".truecolor(255, 153, 0).bold(),
            "ai".truecolor(35, 47, 62).bold(),
            "cli".truecolor(108, 117, 125).bold()
        )
    }

    pub fn format_prompt() -> String {
        "❯".truecolor(0, 123, 255).bold().to_string() // Bright blue
    }

    pub fn format_success(text: &str) -> String {
        text.truecolor(40, 167, 69).to_string()
    }

    pub fn format_warning(text: &str) -> String {
        text.truecolor(255, 193, 7).to_string()
    }

    pub fn format_error(text: &str) -> String {
        text.truecolor(220, 53, 69).to_string()
    }

    pub fn format_info(text: &str) -> String {
        text.truecolor(0, 123, 255).to_string()
    }

    pub fn format_muted(text: &str) -> String {
        text.truecolor(108, 117, 125).to_string()
    }

    pub fn format_code(text: &str) -> String {
        text.truecolor(248, 249, 250)
            .on_truecolor(35, 47, 62)
            .to_string()
    }
}

impl RichInterface {
    pub fn new() -> Self {
        Self {
            syntax_highlighter: SyntaxHighlighter::new(),
            formatter: ResponseFormatter::new(),
        }
    }

    pub fn format_code_response(&self, code: &str, language: &str) -> String {
        let highlighted = self.syntax_highlighter.highlight(code, language);
        self.formatter.format_code_block(&highlighted, language)
    }

    pub fn format_markdown_response(&self, content: &str) -> String {
        self.formatter.format_markdown(content)
    }

    pub fn create_interactive_example(
        &self,
        code: &str,
        language: &str,
        explanation: &str,
    ) -> String {
        format!(
            "{}\n\n{}\n\n{}\n\n{}",
            format!("💡 {}", "Interactive Code Example")
                .truecolor(255, 153, 0)
                .bold(),
            self.format_code_response(code, language),
            format!("📖 {}", "Explanation")
                .truecolor(0, 123, 255)
                .bold(),
            self.formatter.format_explanation(explanation)
        )
    }

    // Amazon Q style startup banner
    pub fn format_startup_banner(&self, nightfall_enabled: bool, config: &Config) -> String {
        let mut banner = String::new();

        // Main header with Amazon Q branding
        banner.push_str(&format!("\n{}\n", "═".repeat(80).truecolor(255, 153, 0)));
        banner.push_str(&format!(
            "{}  {}\n",
            " ".repeat(25),
            AmazonQTheme::format_brand_header()
        ));
        banner.push_str(&format!(
            "{}  {}\n",
            " ".repeat(20),
            "Multi-Region • Local & Cloud LLM Support".truecolor(108, 117, 125)
        ));
        banner.push_str(&format!("{}\n\n", "═".repeat(80).truecolor(255, 153, 0)));

        // Status indicators
        banner.push_str(&format!(
            "{} {}\n",
            "🌏".to_string(),
            AmazonQTheme::format_success(&format!("Region: {}", config.cloud.aws_region))
        ));

        // Dynamic model display based on config
        let model_text = if config.model.use_local_llm {
            format!("Model: {} via Local LLM", config.model.local_llm_model)
        } else {
            "Model: Claude 3.5 Sonnet via Bedrock".to_string()
        };

        banner.push_str(&format!(
            "{} {}\n",
            "🤖".to_string(),
            AmazonQTheme::format_info(&model_text)
        ));
        banner.push_str(&format!(
            "{} {}\n",
            "🛡️".to_string(),
            AmazonQTheme::format_success(&format!("Data Residency: {}", config.cloud.provider))
        ));

        if nightfall_enabled {
            banner.push_str(&format!(
                "{} {}\n",
                "🌙".to_string(),
                AmazonQTheme::format_success("Nightfall DLP: Enabled")
            ));
        } else {
            banner.push_str(&format!(
                "{} {}\n",
                "🌙".to_string(),
                AmazonQTheme::format_muted("Nightfall DLP: Disabled")
            ));
        }

        banner.push('\n');
        banner
    }

    // Amazon Q style tool listing
    pub fn format_tools_section(&self) -> String {
        let mut tools = String::new();

        tools.push_str(&format!(
            "{}\n",
            AmazonQTheme::format_info("Available Tools").bold()
        ));
        tools.push_str(&format!("{}\n", "─".repeat(50).truecolor(108, 117, 125)));

        let tool_list = [
            ("execute_bash", "Execute shell commands with safety checks"),
            ("fs_read", "Read files and directories"),
            ("fs_write", "Create and modify files"),
            ("use_aws", "Execute AWS CLI commands"),
            ("use_azure", "Execute Azure CLI commands"),
            ("use_gcp", "Execute GCP CLI commands"),
            ("use_oracle", "Execute Oracle CLI commands"),
            ("introspect", "Get Q CLI capabilities"),
            ("knowledge", "Persistent context storage"),
            ("todo_list", "Task management"),
            ("thinking", "Complex reasoning processes"),
            ("orchestrated_thinking", "Multi-agent coordination"),
        ];

        for (tool, description) in tool_list {
            tools.push_str(&format!(
                "  {} {}\n",
                format!("▶ {}", tool).truecolor(255, 153, 0).bold(),
                AmazonQTheme::format_muted(description)
            ));
        }

        tools.push('\n');
        tools
    }

    // Amazon Q style safety indicators
    pub fn format_safety_section(&self) -> String {
        let mut safety = String::new();

        safety.push_str(&format!(
            "{}\n",
            AmazonQTheme::format_info("Safety & Permissions").bold()
        ));
        safety.push_str(&format!("{}\n", "─".repeat(50).truecolor(108, 117, 125)));

        safety.push_str(&format!(
            "  {} {}\n",
            "🟢".to_string(),
            AmazonQTheme::format_success("Low Risk - Auto-execute")
        ));
        safety.push_str(&format!(
            "  {} {}\n",
            "🟡".to_string(),
            AmazonQTheme::format_warning("Medium Risk - Confirmation required")
        ));
        safety.push_str(&format!(
            "  {} {}\n",
            "🔴".to_string(),
            AmazonQTheme::format_error("High Risk - Always confirm")
        ));
        safety.push_str(&format!(
            "  {} {}\n",
            "⏱️".to_string(),
            AmazonQTheme::format_muted("2-minute timeout for all commands")
        ));

        safety.push('\n');
        safety
    }

    pub fn create_file_tree(&self, files: &[String]) -> String {
        let mut tree = format!(
            "{}\n",
            format!("📁 {}", "Project Structure")
                .truecolor(255, 153, 0)
                .bold()
        );
        tree.push_str(&format!("{}\n", "─".repeat(40).truecolor(108, 117, 125)));

        for (i, file) in files.iter().enumerate() {
            let is_last = i == files.len() - 1;
            let prefix = if is_last { "└── " } else { "├── " };

            let formatted_file = if file.ends_with(".rs") {
                file.truecolor(220, 53, 69) // Rust files in red
            } else if file.ends_with(".js") || file.ends_with(".ts") {
                file.truecolor(255, 193, 7) // JS/TS files in yellow
            } else if file.ends_with(".py") {
                file.truecolor(0, 123, 255) // Python files in blue
            } else if file.ends_with(".json") || file.ends_with(".yml") || file.ends_with(".yaml") {
                file.truecolor(40, 167, 69) // Config files in green
            } else {
                file.truecolor(248, 249, 250) // Other files in light gray
            };

            tree.push_str(&format!(
                "{}{}\n",
                AmazonQTheme::format_muted(prefix),
                formatted_file
            ));
        }

        tree
    }

    pub fn create_diff_visualization(&self, old_code: &str, new_code: &str) -> String {
        let mut diff = format!(
            "{}\n",
            format!("🔄 {}", "Code Changes")
                .truecolor(255, 153, 0)
                .bold()
        );
        diff.push_str(&format!("{}\n", "─".repeat(60).truecolor(108, 117, 125)));

        let old_lines: Vec<&str> = old_code.lines().collect();
        let new_lines: Vec<&str> = new_code.lines().collect();

        for (i, line) in old_lines.iter().enumerate() {
            if i < new_lines.len() && line != &new_lines[i] {
                diff.push_str(&format!(
                    "{} {}\n",
                    "−".truecolor(220, 53, 69).bold(),
                    line.truecolor(220, 53, 69)
                ));
                diff.push_str(&format!(
                    "{} {}\n",
                    "+".truecolor(40, 167, 69).bold(),
                    new_lines[i].truecolor(40, 167, 69)
                ));
            } else if i >= new_lines.len() {
                diff.push_str(&format!(
                    "{} {}\n",
                    "−".truecolor(220, 53, 69).bold(),
                    line.truecolor(220, 53, 69)
                ));
            } else {
                diff.push_str(&format!("  {}\n", line));
            }
        }

        // Handle new lines that weren't in old code
        for (i, line) in new_lines.iter().enumerate() {
            if i >= old_lines.len() {
                diff.push_str(&format!(
                    "{} {}\n",
                    "+".truecolor(40, 167, 69).bold(),
                    line.truecolor(40, 167, 69)
                ));
            }
        }

        diff
    }

    pub fn format_progress_indicator(&self, current: usize, total: usize, message: &str) -> String {
        let percentage = (current as f32 / total as f32 * 100.0) as usize;
        let filled = percentage / 5; // 20 chars for 100%
        let empty = 20 - filled;

        let progress_bar = format!(
            "[{}{}] {}% {}",
            "█".repeat(filled).truecolor(255, 153, 0),
            "░".repeat(empty).truecolor(108, 117, 125),
            percentage,
            AmazonQTheme::format_info(message)
        );

        format!("\r{}", progress_bar)
    }

    pub fn create_command_palette(&self, commands: &[(&str, &str)]) -> String {
        let mut palette = format!(
            "{}\n",
            format!("⌨️  {}", "Command Palette")
                .truecolor(255, 153, 0)
                .bold()
        );
        palette.push_str(&format!("{}\n", "═".repeat(60).truecolor(255, 153, 0)));

        for (command, description) in commands {
            palette.push_str(&format!(
                "{} {} - {}\n",
                "▶".truecolor(0, 123, 255),
                command.truecolor(255, 153, 0).bold(),
                AmazonQTheme::format_muted(description)
            ));
        }

        palette
    }

    // Amazon Q style PII detection display
    pub fn format_pii_detection(
        &self,
        pii_found: bool,
        details: &str,
        is_nightfall: bool,
    ) -> String {
        if !pii_found {
            return String::new();
        }

        let mut output = String::new();

        if is_nightfall {
            output.push_str(&format!(
                "\n{} {}\n",
                "🌙".to_string(),
                AmazonQTheme::format_warning("NIGHTFALL PII DETECTED").bold()
            ));
        } else {
            output.push_str(&format!(
                "\n{} {}\n",
                "🔒".to_string(),
                AmazonQTheme::format_error("PII DETECTED").bold()
            ));
        }

        output.push_str(&format!("{}\n", "─".repeat(60).truecolor(220, 53, 69)));
        output.push_str(&format!("{}\n", AmazonQTheme::format_error(details)));
        output.push_str(&format!("{}\n", "─".repeat(60).truecolor(220, 53, 69)));

        output
    }

    // Amazon Q style permission prompt
    pub fn format_permission_prompt(&self, command: &str, risk_level: &str) -> String {
        let (icon, color_fn): (&str, fn(&str) -> String) = match risk_level {
            "LOW" => ("🟢", AmazonQTheme::format_success),
            "MEDIUM" => ("🟡", AmazonQTheme::format_warning),
            "HIGH" => ("🔴", AmazonQTheme::format_error),
            _ => ("⚪", AmazonQTheme::format_muted),
        };

        format!(
            "\n{} {} {}\n{} {}\n{}\n",
            icon,
            color_fn(&format!("{} RISK COMMAND", risk_level)),
            AmazonQTheme::format_code(command),
            "❯".truecolor(0, 123, 255).bold(),
            AmazonQTheme::format_muted("Continue? (y/n/a for all):"),
            "─".repeat(50).truecolor(108, 117, 125)
        )
    }
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        let mut language_patterns = HashMap::new();

        // Rust patterns
        language_patterns.insert(
            "rust".to_string(),
            vec![
                ("fn ".to_string(), Color::BrightBlue),
                ("pub ".to_string(), Color::BrightGreen),
                ("let ".to_string(), Color::BrightYellow),
                ("mut ".to_string(), Color::BrightYellow),
                ("struct ".to_string(), Color::BrightBlue),
                ("enum ".to_string(), Color::BrightBlue),
                ("impl ".to_string(), Color::BrightBlue),
                ("use ".to_string(), Color::Magenta),
                ("mod ".to_string(), Color::Magenta),
            ],
        );

        // JavaScript patterns
        language_patterns.insert(
            "javascript".to_string(),
            vec![
                ("function ".to_string(), Color::BrightBlue),
                ("const ".to_string(), Color::BrightYellow),
                ("let ".to_string(), Color::BrightYellow),
                ("var ".to_string(), Color::BrightYellow),
                ("class ".to_string(), Color::BrightBlue),
                ("import ".to_string(), Color::Magenta),
                ("export ".to_string(), Color::Magenta),
                ("async ".to_string(), Color::BrightGreen),
                ("await ".to_string(), Color::BrightGreen),
            ],
        );

        // Python patterns
        language_patterns.insert(
            "python".to_string(),
            vec![
                ("def ".to_string(), Color::BrightBlue),
                ("class ".to_string(), Color::BrightBlue),
                ("import ".to_string(), Color::Magenta),
                ("from ".to_string(), Color::Magenta),
                ("async ".to_string(), Color::BrightGreen),
                ("await ".to_string(), Color::BrightGreen),
            ],
        );

        Self { language_patterns }
    }

    pub fn highlight(&self, code: &str, language: &str) -> String {
        if let Some(patterns) = self.language_patterns.get(language) {
            let mut highlighted = code.to_string();

            for (pattern, color) in patterns {
                highlighted = highlighted.replace(pattern, &self.colorize_text(pattern, color));
            }

            // Highlight strings
            highlighted = self.highlight_strings(&highlighted);
            // Highlight comments
            highlighted = self.highlight_comments(&highlighted, language);

            highlighted
        } else {
            code.to_string()
        }
    }

    fn colorize_text(&self, text: &str, color: &Color) -> String {
        match color {
            Color::Red => text.red().to_string(),
            Color::Green => text.green().to_string(),
            Color::Blue => text.blue().to_string(),
            Color::Yellow => text.yellow().to_string(),
            Color::Magenta => text.magenta().to_string(),
            Color::Cyan => text.cyan().to_string(),
            Color::White => text.white().to_string(),
            Color::BrightRed => text.bright_red().to_string(),
            Color::BrightGreen => text.bright_green().to_string(),
            Color::BrightBlue => text.bright_blue().to_string(),
            Color::BrightYellow => text.bright_yellow().to_string(),
        }
    }

    fn highlight_strings(&self, code: &str) -> String {
        // Simple string highlighting for double quotes
        let mut result = String::new();
        let mut in_string = false;
        let mut chars = code.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '"' && chars.peek() != Some(&'\\') {
                in_string = !in_string;
                if in_string {
                    result.push_str(&format!("{}", "\"".bright_green()));
                } else {
                    result.push_str(&format!("{}", "\"".bright_green()));
                }
            } else if in_string {
                result.push_str(&format!("{}", ch.to_string().bright_green()));
            } else {
                result.push(ch);
            }
        }

        result
    }

    fn highlight_comments(&self, code: &str, language: &str) -> String {
        let comment_prefix = match language {
            "rust" | "javascript" | "java" => "//",
            "python" => "#",
            _ => "//",
        };

        code.lines()
            .map(|line| {
                if let Some(comment_start) = line.find(comment_prefix) {
                    let (before, comment) = line.split_at(comment_start);
                    format!("{}{}", before, comment.bright_black())
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl ResponseFormatter {
    pub fn new() -> Self {
        Self {
            markdown_enabled: true,
            code_block_style: "bordered".to_string(),
        }
    }

    pub fn format_markdown(&self, content: &str) -> String {
        if !self.markdown_enabled {
            return content.to_string();
        }

        let mut formatted = content.to_string();

        // Format headers
        formatted = self.format_headers(&formatted);
        // Format bold text
        formatted = self.format_bold(&formatted);
        // Format code blocks
        formatted = self.format_inline_code(&formatted);
        // Format lists
        formatted = self.format_lists(&formatted);

        formatted
    }

    fn format_headers(&self, content: &str) -> String {
        content
            .lines()
            .map(|line| {
                if line.starts_with("# ") {
                    format!("{}", line.strip_prefix("# ").unwrap().bright_cyan().bold())
                } else if line.starts_with("## ") {
                    format!("{}", line.strip_prefix("## ").unwrap().bright_blue().bold())
                } else if line.starts_with("### ") {
                    format!("{}", line.strip_prefix("### ").unwrap().blue().bold())
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn format_bold(&self, content: &str) -> String {
        // Simple **bold** formatting
        let mut result = String::new();
        let mut chars = content.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '*' && chars.peek() == Some(&'*') {
                chars.next(); // consume second *
                let mut bold_text = String::new();

                // Collect text until closing **
                while let Some(ch) = chars.next() {
                    if ch == '*' && chars.peek() == Some(&'*') {
                        chars.next(); // consume second *
                        result.push_str(&bold_text.bold().to_string());
                        break;
                    } else {
                        bold_text.push(ch);
                    }
                }
            } else {
                result.push(ch);
            }
        }

        result
    }

    fn format_inline_code(&self, content: &str) -> String {
        // Simple `code` formatting
        content.replace("`", &"`".bright_yellow().to_string())
    }

    fn format_lists(&self, content: &str) -> String {
        content
            .lines()
            .map(|line| {
                if line.trim_start().starts_with("- ") {
                    let indent = line.len() - line.trim_start().len();
                    let bullet = "•".bright_blue();
                    format!(
                        "{}{} {}",
                        " ".repeat(indent),
                        bullet,
                        line.trim_start().strip_prefix("- ").unwrap()
                    )
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn format_code_block(&self, code: &str, language: &str) -> String {
        let lang_label = format!(" {} ", language.to_uppercase())
            .truecolor(248, 249, 250)
            .on_truecolor(35, 47, 62)
            .bold();

        let border = "─".repeat(60).truecolor(108, 117, 125);

        format!(
            "{}\n{}\n{}\n{}\n{}",
            border, lang_label, border, code, border
        )
    }

    pub fn format_explanation(&self, explanation: &str) -> String {
        explanation
            .lines()
            .map(|line| format!("  {}", AmazonQTheme::format_muted(line)))
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[allow(dead_code)]
    pub fn create_status_box(&self, title: &str, content: &str, status: &str) -> String {
        let (status_icon, color_fn): (&str, fn(&str) -> String) = match status {
            "success" => ("✅", AmazonQTheme::format_success),
            "warning" => ("⚠️", AmazonQTheme::format_warning),
            "error" => ("❌", AmazonQTheme::format_error),
            "info" => ("ℹ️", AmazonQTheme::format_info),
            _ => ("📋", AmazonQTheme::format_muted),
        };

        let border = "═".repeat(60).truecolor(255, 153, 0);

        format!(
            "{}\n{} {}\n{}\n{}\n{}",
            border,
            status_icon,
            color_fn(title).bold(),
            border,
            content,
            border
        )
    }
}
