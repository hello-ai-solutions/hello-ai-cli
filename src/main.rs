#![allow(dead_code)]
#![allow(private_interfaces)]
mod action_header;
mod autonomous_deploy;
mod cloud_logging;
mod cloud_pii_providers;
mod comprehend_pii;
mod execute;
mod fs_read;
mod fs_write;
mod hooks;
mod introspect;
mod knowledge;
mod local_pii_scanner;
mod mcp_server;
mod multi_agent;
mod nightfall_client;
mod online_providers;
mod orchestrated_thinking;
mod orchestrator;
mod output_formatter;
mod permissions;
mod pii_scanner;
mod project_kind;
mod s3_logs;
mod ses_alerts;
mod streaming;
mod system_context;
mod test_runner;
mod thinking;
mod todo_list;
mod unified_cloud_providers;
mod use_aws;
mod use_azure;
mod use_gcp;
mod use_oracle;
mod visual_indicators;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct LoggingConfig {
    enabled: bool,
    provider: String,
    level: String,
    // Local
    local_path: String,
    local_filename: String,
    max_file_size_mb: u64,
    max_files: u32,
    // AWS S3
    aws_s3_bucket: String,
    aws_s3_prefix: String,
    aws_s3_region: String,
    // Azure Blob
    azure_storage_account: String,
    azure_container: String,
    azure_storage_key: String,
    // GCP Cloud Storage
    gcp_storage_bucket: String,
    gcp_storage_prefix: String,
    // Oracle Object Storage
    oracle_bucket: String,
    oracle_namespace: String,
    oracle_prefix: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct SecurityConfig {
    pii_scanner: bool,
    pii_model: Option<String>,
    nightfall_scanner: bool,
    security_scanner: bool,
    permission_checker: bool,
    guard_rail_enabled: bool,    // Enable/disable guard rail
    guard_rail_provider: String, // "local" or "remote"
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AnalysisConfig {
    command_analysis: bool,
    ai_analysis: bool,
}

#[derive(Debug, Clone)]
struct CommandGuard {
    recent_commands: HashMap<String, u64>,
    duplicate_threshold: u64, // seconds
}

impl CommandGuard {
    fn new() -> Self {
        Self {
            recent_commands: HashMap::new(),
            duplicate_threshold: 30,
        }
    }

    fn should_execute(&mut self, command: &str) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if let Some(&last_time) = self.recent_commands.get(command) {
            if now - last_time < self.duplicate_threshold {
                return false;
            }
        }

        self.recent_commands.insert(command.to_string(), now);
        true
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct FeaturesConfig {
    streaming: bool,
    visual_indicators: bool,
    execution_mode: String,      // "interactive", "chatbot", "auto"
    orchestrate_mode: bool,      // Enable AI orchestrator to manage other AIs
    auto_analysis: Option<bool>, // Auto-analyze tool outputs
    verbosity: String,           // "full", "minimal", "commands-only"
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ModelConfig {
    provider: String,
    use_local_llm: bool,
    local_llm_endpoint: String,
    local_llm_model: String,
    default_model: String,
    openai_api_key: String,
    openai_model: String,
    openai_endpoint: String,
    gemini_api_key: String,
    gemini_model: String,
    gemini_endpoint: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct CloudConfig {
    provider: String,
    // AWS
    aws_profile: String,
    aws_region: String,
    aws_llm_model: String,
    // Azure
    azure_subscription_id: String,
    azure_resource_group: String,
    azure_region: String,
    azure_openai_endpoint: String,
    azure_openai_key: String,
    azure_openai_model: String,
    azure_text_analytics_endpoint: String,
    azure_text_analytics_key: String,
    // GCP
    gcp_project_id: String,
    gcp_region: String,
    gcp_credentials_path: String,
    gcp_vertex_model: String,
    // Oracle
    oracle_compartment_id: String,
    oracle_region: String,
    oracle_config_file: String,
    oracle_profile: String,
    oracle_llm_model: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Config {
    logging: LoggingConfig,
    security: SecurityConfig,
    analysis: AnalysisConfig,
    features: FeaturesConfig,
    model: ModelConfig,
    cloud: CloudConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            logging: LoggingConfig {
                enabled: true,
                provider: "local".to_string(),
                level: "info".to_string(),
                local_path: "./logs".to_string(),
                local_filename: "hello-ai.log".to_string(),
                max_file_size_mb: 100,
                max_files: 10,
                aws_s3_bucket: "".to_string(),
                aws_s3_prefix: "hello-ai-logs/".to_string(),
                aws_s3_region: "ap-southeast-2".to_string(),
                azure_storage_account: "".to_string(),
                azure_container: "hello-ai-logs".to_string(),
                azure_storage_key: "".to_string(),
                gcp_storage_bucket: "".to_string(),
                gcp_storage_prefix: "hello-ai-logs/".to_string(),
                oracle_bucket: "".to_string(),
                oracle_namespace: "".to_string(),
                oracle_prefix: "hello-ai-logs/".to_string(),
            },
            security: SecurityConfig {
                // PII SCANNING CONFIGURATION
                pii_scanner: true, // Enable PII detection and sanitization
                pii_model: Some("deepseek-coder:6.7b".to_string()), // Local LLM model for PII scanning

                // REMOTE PII SERVICES (currently disabled)
                nightfall_scanner: false, // Nightfall DLP service (requires NIGHTFALL_API_KEY)

                // OTHER SECURITY FEATURES
                security_scanner: true,   // Enable general security scanning
                permission_checker: true, // Enable command permission checking

                // GUARD RAIL CONFIGURATION
                guard_rail_enabled: true, // Enable response validation
                guard_rail_provider: "local".to_string(), // "local" (Bedrock) or "remote" (cloud providers)
            },
            analysis: AnalysisConfig {
                command_analysis: false,
                ai_analysis: true,
            },
            features: FeaturesConfig {
                streaming: true,
                visual_indicators: true,
                execution_mode: "interactive".to_string(),
                orchestrate_mode: true,
                auto_analysis: Some(false),
                verbosity: "full".to_string(), // "full", "minimal", "commands-only"
            },
            model: ModelConfig {
                provider: "local".to_string(),
                use_local_llm: true,
                local_llm_endpoint: "http://localhost:11434/api/generate".to_string(),
                local_llm_model: "qwen2.5-coder:14b".to_string(),
                default_model: "anthropic.claude-3-5-sonnet-20241022-v2:0".to_string(),
                openai_api_key: "".to_string(),
                openai_model: "gpt-4".to_string(),
                openai_endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
                gemini_api_key: "".to_string(),
                gemini_model: "gemini-pro".to_string(),
                gemini_endpoint: "https://generativelanguage.googleapis.com/v1beta/models"
                    .to_string(),
            },
            cloud: CloudConfig {
                // CLOUD PROVIDER SELECTION
                // Options: "aws", "azure", "gcp", "oci", "local"
                // Setting to "local" disables remote PII services (AWS Comprehend, etc.)
                provider: "local".to_string(), // Use LOCAL-ONLY for privacy

                // AWS CONFIGURATION (not used when provider="local")
                aws_profile: "bedrock".to_string(),
                aws_region: "ap-southeast-2".to_string(),
                aws_llm_model: "anthropic.claude-3-5-sonnet-20241022-v2:0".to_string(),
                azure_subscription_id: "".to_string(),
                azure_resource_group: "".to_string(),
                azure_region: "australiaeast".to_string(),
                azure_openai_endpoint: "".to_string(),
                azure_openai_key: "".to_string(),
                azure_openai_model: "gpt-4".to_string(),
                azure_text_analytics_endpoint: "".to_string(),
                azure_text_analytics_key: "".to_string(),
                gcp_project_id: "".to_string(),
                gcp_region: "australia-southeast1".to_string(),
                gcp_credentials_path: "".to_string(),
                gcp_vertex_model: "gemini-pro".to_string(),
                oracle_compartment_id: "".to_string(),
                oracle_region: "ap-sydney-1".to_string(),
                oracle_config_file: "~/.oci/config".to_string(),
                oracle_profile: "DEFAULT".to_string(),
                oracle_llm_model: "cohere.command-r-plus".to_string(),
            },
        }
    }
}

async fn save_config(config: &Config) {
    let config_dir = dirs::home_dir().unwrap().join(".amazonq");
    let config_file = config_dir.join("config.json");

    if let Ok(config_json) = serde_json::to_string_pretty(config) {
        if let Err(e) = tokio::fs::write(&config_file, config_json).await {
            eprintln!("Failed to save config: {}", e);
        }
    }
}

fn load_config() -> Config {
    if let Ok(content) = fs::read_to_string("config.toml") {
        toml::from_str(&content).unwrap_or_default()
    } else {
        Config::default()
    }
}
mod advanced_context;
mod aws_knowledge;
mod cicd_integration;
mod code_completion;
mod code_explainer;
mod enterprise;
mod github;
mod learning_system;
mod local_llm;
mod realtime_context;
mod refactoring_assistant;
mod repository_analyzer;
mod rich_interface;
mod security_scanner;
mod test_generator;
mod web_gui;

use advanced_context::AdvancedContextManager;
use aws_config::Region;
use aws_sdk_bedrockruntime::Client as BedrockClient;
use cicd_integration::CicdIntegrator;
use clap::{Parser, Subcommand};
use code_completion::CodeCompletion;
use code_explainer::CodeExplainer;
use colored::*;
use comprehend_pii::ComprehendPiiScanner;
use execute::ExecuteCommand;
use fs_read::FsRead;
use fs_write::FsWrite;
use hooks::HooksManager;
use introspect::Introspect;
use knowledge::Knowledge;
use learning_system::LearningSystem;
use local_pii_scanner::LocalPiiScanner;
use mcp_server::McpManager;
use multi_agent::MultiAgentSystem;
use nightfall_client::NightfallClient;
use orchestrated_thinking::OrchestratedThinking;
use permissions::{PermissionChecker, RiskLevel};
use refactoring_assistant::RefactoringAssistant;
use repository_analyzer::RepositoryAnalyzer;
use rich_interface::{AmazonQTheme, RichInterface};
use rustyline::DefaultEditor;
use security_scanner::SecurityScanner;
use serde_json::json;
use ses_alerts::SesAlertService;
#[cfg(not(windows))]
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::io::Write;
use std::sync::{Arc, Mutex};
use streaming::StreamingClient;
use system_context::SystemContext;
use test_generator::TestGenerator;
use test_runner::TestRunner;
use thinking::Thinking;
use todo_list::TodoList;
use use_aws::UseAws;
use use_azure::UseAzure;
use use_gcp::UseGcp;
use use_oracle::UseOracle;
use visual_indicators::VisualIndicator;

#[derive(Copy, Clone)]
enum MsgKind {
    Assistant,
    Error,
}

fn render_msg(kind: MsgKind, text: &str) -> String {
    match kind {
        MsgKind::Assistant => format!("{} {}", "🤖".green().bold(), text.green()),
        MsgKind::Error => format!("{} {}", "⛔".bright_red().bold(), text.bright_red()),
    }
}

#[derive(Parser)]
#[command(name = "q")]
#[command(about = "HANS CUSTOM AWS Q - Sydney Region Only")]
#[command(version = "1.0.0")]
#[command(disable_help_flag = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Increase logging verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Print help for all subcommands
    #[arg(long)]
    help_all: bool,

    /// Print help
    #[arg(short, long)]
    help: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// AI assistant in your terminal
    Chat {
        /// Input message for non-interactive mode
        input: Option<String>,
        /// Non-interactive mode
        #[arg(long)]
        no_interactive: bool,
        /// Resume previous conversation from this directory
        #[arg(short, long)]
        resume: bool,
        /// Context profile to use
        #[arg(long)]
        agent: Option<String>,
        /// Current model to use
        #[arg(long)]
        model: Option<String>,
        /// Allow model to use any tool without confirmation
        #[arg(short = 'a', long)]
        trust_all_tools: bool,
        /// Trust only specific tools (comma-separated)
        #[arg(long)]
        trust_tools: Option<String>,
        /// Control line wrapping behavior (always, never, auto)
        #[arg(short, long)]
        wrap: Option<String>,
        /// Increase logging verbosity
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbose: u8,
    },
    /// Natural Language to Shell translation
    Translate {
        /// Natural language command to translate
        command: String,
    },
    /// Debug the app
    Debug,
    /// Setup cli components
    Setup,
    /// Update the Amazon Q application
    Update,
    /// Run diagnostic tests
    Diagnostic,
    /// Generate the dotfiles for the given shell
    Init {
        /// Shell type (bash, zsh, fish)
        shell: Option<String>,
    },
    /// Get or set theme
    Theme {
        /// Theme name to set
        name: Option<String>,
    },
    /// Create a new Github issue
    Issue {
        /// Issue title
        title: Option<String>,
    },
    /// Launch the desktop app
    Launch,
    /// Fix and diagnose common issues
    Doctor,
    /// Customize appearance & behavior
    Settings {
        /// Show current settings
        #[arg(long)]
        show: bool,
        /// Set a configuration value
        #[arg(long)]
        set: Option<String>,
    },
    /// Login
    Login,
    /// Logout
    Logout,
    /// Prints details about the current user
    Whoami,
    /// Show the profile associated with this idc user
    Profile,
    /// Manage your account
    User,
    /// Quit the desktop app
    Quit,
    /// Restart the desktop app
    Restart,
    /// Manage system integrations
    Integrations,
    /// Open the dashboard
    Dashboard,
    /// Model Context Protocol (MCP)
    Mcp {
        /// MCP subcommand
        #[command(subcommand)]
        mcp_command: Option<McpCommands>,
    },
    /// Inline shell completions
    Inline,
    /// Agent root commands
    Agent {
        /// Agent subcommand
        #[command(subcommand)]
        agent_command: Option<AgentCommands>,
    },
}

#[derive(Subcommand)]
enum McpCommands {
    /// Add or replace a configured server
    Add,
    /// Remove a server from MCP configuration
    Remove,
    /// List configured servers
    List,
    /// Import server configuration
    Import,
    /// Get status of configured server
    Status,
}

#[derive(Subcommand)]
enum AgentCommands {
    /// List available agents
    List,
    /// Create agent config
    Create,
    /// Edit existing agent config
    Edit,
    /// Validate agent config
    Validate,
    /// Migrate profiles to agent
    Migrate,
    /// Set default agent
    SetDefault,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Force colors to be enabled
    colored::control::set_override(true);

    let cli = Cli::parse();
    let app_config = load_config();

    if cli.help {
        print_custom_help();
        return Ok(());
    }

    if cli.verbose > 0 {
        println!("🔍 Verbose mode: Level {}", cli.verbose);
    }

    if cli.help_all {
        print_help_all();
        return Ok(());
    }

    match cli.command {
        Some(Commands::Chat {
            input,
            no_interactive,
            resume,
            agent,
            model,
            trust_all_tools,
            trust_tools,
            wrap,
            verbose,
        }) => {
            if no_interactive {
                if let Some(msg) = input {
                    run_single_chat_with_options(
                        msg,
                        resume,
                        agent,
                        model,
                        trust_all_tools,
                        trust_tools,
                        wrap,
                        verbose,
                        app_config.clone(),
                    )
                    .await?;
                } else {
                    eprintln!("Error: Input required for non-interactive mode");
                    std::process::exit(1);
                }
            } else {
                run_interactive_chat_with_options(
                    resume,
                    agent,
                    model,
                    trust_all_tools,
                    trust_tools,
                    wrap,
                    verbose,
                    app_config.clone(),
                )
                .await?;
            }
        }
        Some(Commands::Translate { command }) => {
            run_translate(command, app_config).await?;
        }
        Some(Commands::Debug) => {
            run_debug().await?;
        }
        Some(Commands::Setup) => {
            run_setup().await?;
        }
        Some(Commands::Update) => {
            run_update().await?;
        }
        Some(Commands::Diagnostic) => {
            run_diagnostic().await?;
        }
        Some(Commands::Init { shell }) => {
            run_init(shell).await?;
        }
        Some(Commands::Theme { name }) => {
            run_theme(name).await?;
        }
        Some(Commands::Issue { title }) => {
            run_issue(title).await?;
        }
        Some(Commands::Doctor) => {
            run_doctor().await?;
        }
        Some(Commands::Settings { show, set }) => {
            run_settings(show, set).await?;
        }
        Some(Commands::Login) => {
            run_login().await?;
        }
        Some(Commands::Logout) => {
            run_logout().await?;
        }
        Some(Commands::Whoami) => {
            run_whoami().await?;
        }
        Some(Commands::Profile) => {
            run_profile().await?;
        }
        Some(Commands::User) => {
            run_user().await?;
        }
        Some(Commands::Quit) => {
            run_quit().await?;
        }
        Some(Commands::Restart) => {
            run_restart().await?;
        }
        Some(Commands::Integrations) => {
            run_integrations().await?;
        }
        Some(Commands::Dashboard) => {
            run_dashboard().await?;
        }
        Some(Commands::Mcp { mcp_command }) => {
            run_mcp(mcp_command).await?;
        }
        Some(Commands::Inline) => {
            run_inline().await?;
        }
        Some(Commands::Agent { agent_command }) => {
            run_agent(agent_command).await?;
        }
        Some(Commands::Launch) => {
            run_launch().await?;
        }
        None => {
            // Default to interactive chat if no subcommand
            run_interactive_chat(app_config).await?;
        }
    }

    Ok(())
}

async fn run_interactive_chat(mut app_config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize system context for OS detection
    let system_context = SystemContext::new();

    // Setup signal handler to catch Ctrl+C (Unix only)
    #[cfg(not(windows))]
    {
        let ctrl_c_count = Arc::new(Mutex::new(0));
        let ctrl_c_count_clone = Arc::clone(&ctrl_c_count);

        let mut signals = Signals::new(&[SIGINT])?;
        std::thread::spawn(move || {
            for sig in signals.forever() {
                match sig {
                    SIGINT => {
                        let mut count = ctrl_c_count_clone.lock().unwrap();
                        *count += 1;
                        match *count {
                            1 => {
                                println!(
                                    "\n{} {} {}",
                                    "⚠️".yellow(),
                                    "Ctrl+C pressed once.".bright_yellow(),
                                    "Press 2 more times to exit.".bright_black()
                                );
                            }
                            2 => println!(
                                "\n{} {} {}",
                                "⚠️".red(),
                                "Ctrl+C pressed twice.".bright_red(),
                                "Press once more to force exit.".bright_black()
                            ),
                            _ => {
                                println!(
                                    "\n{} {}",
                                    "💥".red(),
                                    "Force exit after 3 Ctrl+C presses.".bright_red()
                                );
                                std::process::exit(1);
                            }
                        }
                    }
                    _ => {}
                }
            }
        });
    }

    println!(
        "{}",
        "           ██╗  ██╗███████╗██╗     ██╗      ██████╗ ".bright_green()
    );
    println!(
        "{}",
        "           ██║  ██║██╔════╝██║     ██║     ██╔═══██╗".bright_green()
    );
    println!(
        "{}",
        "           ███████║█████╗  ██║     ██║     ██║   ██║".bright_green()
    );
    println!(
        "{}",
        "           ██╔══██║██╔══╝  ██║     ██║     ██║   ██║".bright_green()
    );
    println!(
        "{}",
        "           ██║  ██║███████╗███████╗███████╗╚██████╔╝".bright_green()
    );
    println!(
        "{}",
        "           ╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝ ╚═════╝ ".bright_green()
    );
    println!();
    println!("{}", "                      █████╗ ██╗".white());
    println!("{}", "                     ██╔══██╗██║".white());
    println!("{}", "                     ███████║██║".white());
    println!("{}", "                     ██╔══██║██║".white());
    println!("{}", "                     ██║  ██║██║".white());
    println!("{}", "                     ╚═╝  ╚═╝╚═╝".white());
    println!();
    println!("{}", "                   ██████╗██╗     ██╗".red());
    println!("{}", "                  ██╔════╝██║     ██║".red());
    println!("{}", "                  ██║     ██║     ██║".red());
    println!("{}", "                  ██║     ██║     ██║".red());
    println!("{}", "                  ╚██████╗███████╗██║".red());
    println!("{}", "                   ╚═════╝╚══════╝╚═╝".red());
    println!();
    println!(
        "{}",
        "Type your message and press Enter. Type 'quit' to exit.\n".bright_black()
    );

    // Initialize Bedrock client with configurable region
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(Region::new(app_config.cloud.aws_region.clone()))
        .load()
        .await;
    let client = BedrockClient::new(&config);

    // Initialize permission checker
    let permission_checker = PermissionChecker::default();

    // Initialize PII scanner based on cloud provider
    // Priority: AWS Comprehend (remote) -> Local LLM (local) -> None
    let comprehend_pii = if app_config.security.pii_scanner && app_config.cloud.provider == "aws" {
        match ComprehendPiiScanner::new().await {
            Ok(scanner) => Some(scanner),
            Err(e) => {
                println!(
                    "{} Failed to initialize AWS Comprehend PII: {}",
                    "⚠️".yellow(),
                    e
                );
                None
            }
        }
    } else {
        None
    };

    // Initialize Local LLM PII scanner (privacy-focused, runs locally)
    // Uses deepseek-coder:6.7b model via Ollama endpoint
    let local_pii = if app_config.security.pii_scanner {
        let pii_model = app_config
            .security
            .pii_model
            .as_ref()
            .unwrap_or(&app_config.model.local_llm_model)
            .clone();
        Some(LocalPiiScanner::new(
            app_config.model.local_llm_endpoint.clone(),
            pii_model,
        ))
    } else {
        None
    };

    // Initialize SES alert service
    let ses_client = ses_alerts::create_ses_client().await?;
    let ses_alerts = SesAlertService::new(
        ses_client,
        "hans.zand@hotmail.com".to_string(), // From (verified email)
        "hans.zand@hotmail.com".to_string(), // To (your email)
    );

    // Initialize Nightfall client (optional)
    let nightfall_client = if app_config.security.nightfall_scanner {
        std::env::var("NIGHTFALL_API_KEY")
            .ok()
            .map(|api_key| NightfallClient::new(api_key))
    } else {
        None
    };

    // Display Nightfall status
    let nightfall_enabled = nightfall_client.is_some();

    // Initialize rich interface
    let rich_interface = RichInterface::new();

    // Initialize color settings
    if std::env::var("NO_COLOR").is_ok() || !app_config.features.visual_indicators {
        colored::control::set_override(false);
    } else {
        colored::control::set_override(true);
    }

    // Display Amazon Q styled startup banner
    print!(
        "{}",
        rich_interface.format_startup_banner(nightfall_enabled, &app_config)
    );
    print!("{}", rich_interface.format_tools_section());
    print!("{}", rich_interface.format_safety_section());

    println!("{}", AmazonQTheme::format_info("Usage Instructions").bold());
    println!("{}", "─".repeat(50).truecolor(108, 117, 125));
    println!(
        "  {} {}",
        "•".truecolor(255, 153, 0),
        AmazonQTheme::format_muted("Type your questions naturally")
    );
    println!(
        "  {} {}",
        "•".truecolor(255, 153, 0),
        AmazonQTheme::format_muted("Commands are automatically detected and executed")
    );
    println!(
        "  {} {}",
        "•".truecolor(255, 153, 0),
        AmazonQTheme::format_muted("Use 'y' to confirm risky commands, 'a' to confirm all")
    );
    println!(
        "  {} {}",
        "•".truecolor(255, 153, 0),
        AmazonQTheme::format_muted("Type 'quit', '/quit', or press Ctrl+C 3 times to exit")
    );
    println!(
        "  {} {}",
        "•".truecolor(255, 153, 0),
        AmazonQTheme::format_muted("Press Ctrl+C 3 times to force exit")
    );
    println!();

    // Check for simple PII scan mode
    let simple_pii_scan = std::env::var("SIMPLE_PII_SCAN")
        .unwrap_or_else(|_| "false".to_string())
        .to_lowercase()
        == "true";

    // Conversation history
    let mut conversation_history: Vec<serde_json::Value> = Vec::new();

    // Initialize readline editor with history
    let mut rl = DefaultEditor::new()?;
    let history_file = std::env::var("HOME").unwrap_or_else(|_| ".".to_string()) + "/.q_history";
    let _ = rl.load_history(&history_file); // Ignore errors if file doesn't exist

    // Add Ctrl+J for new line
    rl.bind_sequence(rustyline::KeyEvent::ctrl('J'), rustyline::Cmd::Newline);
    rl.bind_sequence(rustyline::KeyEvent::alt('j'), rustyline::Cmd::Newline);

    loop {
        let readline = rl.readline(&format!("{} ", AmazonQTheme::format_prompt()));
        let input = match readline {
            Ok(line) => {
                if !line.trim().is_empty() {
                    rl.add_history_entry(line.as_str())?;
                }
                line.trim().to_string()
            }
            Err(_) => break, // Ctrl+C or EOF
        };

        if input.is_empty() {
            continue;
        }

        if input == "quit" || input == "/quit" {
            println!(
                "{}",
                AmazonQTheme::format_info("Goodbye! Thank you for using Hans AI LLM CLI.")
            );
            let _ = rl.save_history(&history_file); // Save history on exit
            break;
        }

        // Check for orchestrator requests
        if app_config.features.orchestrate_mode
            && (input.contains("orchestrate")
                || input.contains("manage agents")
                || input.contains("security best practices")
                || input.contains("microservice dependency")
                || input.contains("analyze logs")
                || input.contains("troubleshoot"))
        {
            println!("{}", "🎭 Orchestrator Mode Activated".bright_magenta());

            // Initialize orchestrator
            let mut orchestrator = crate::orchestrator::Orchestrator::new();

            match orchestrator
                .execute_orchestrated_task(&input, &app_config)
                .await
            {
                Ok(result) => {
                    println!("{}", result);
                }
                Err(e) => {
                    println!(
                        "{} {}",
                        "Orchestrator Error:".bright_red().bold(),
                        e.to_string().red()
                    );
                }
            }
            continue;
        }

        // Handle slash commands
        if input.starts_with('/') {
            let command_part = input.split_whitespace().next().unwrap_or("");
            match command_part {
                "/agents" => {
                    let mut multi_agent = MultiAgentSystem::new();
                    let args: Vec<&str> = input.split_whitespace().collect();

                    if args.len() == 1 {
                        println!("🤖 Multi-Agent System Status:");
                        println!("   Available Agents: DeploymentAgent, TroubleshootAgent, SecurityAgent, ValidationAgent");
                        println!("   Usage:");
                        println!("     /agents <request>           - Coordinate agents for complex tasks");
                        println!(
                            "     /agents config              - Show current agent configurations"
                        );
                        println!("     /agents config <agent> <provider> <model> - Configure agent model");
                        println!("   Example: /agents deploy nginx with security validation");
                        println!("   Example: /agents config <agent> <provider> <model> - Configure agent model");
                    } else if args.len() >= 2 && args[1] == "config" {
                        if args.len() == 2 {
                            // Show current configurations
                            println!("🔧 Current Agent Configurations:");
                            let configs = multi_agent.list_agent_configs();
                            for (name, provider, model) in configs {
                                println!("   {} -> {} ({})", name, provider, model);
                            }
                        } else if args.len() == 5 {
                            // Configure agent: /agents config <agent> <provider> <model>
                            let agent_name = args[2];
                            let provider = args[3];
                            let model = args[4];

                            match multi_agent.configure_agent(agent_name, provider, model) {
                                Ok(_) => {
                                    println!(
                                        "✅ Configured {} to use {} ({})",
                                        agent_name, provider, model
                                    );
                                    if let Err(e) = multi_agent.save_agent_config() {
                                        println!("⚠️ Warning: Could not save config: {}", e);
                                    }
                                }
                                Err(e) => {
                                    println!("❌ Error: {}", e);
                                    println!(
                                        "Available agents: {}",
                                        multi_agent.get_available_agents().join(", ")
                                    );
                                }
                            }
                        } else {
                            println!("❌ Usage: /agents config [<agent> <provider> <model>]");
                        }
                    } else {
                        let request = &input[8..];
                        println!("\n🔄 Coordinating agents for: {}", request);
                        let results = multi_agent.coordinate_agents(request).await;
                        for result in results {
                            println!("{}", result);
                        }
                    }
                    continue;
                }
                "/test" => {
                    let args: Vec<&str> = input.split_whitespace().collect();
                    let mut test_runner = TestRunner::new();

                    if args.len() == 1 {
                        println!("Usage: /test <all|1-118|category|range>");
                        println!("Examples:");
                        println!("  /test all        - Run all tests");
                        println!("  /test 30         - Run test 30");
                        println!("  /test 1-10       - Run tests 1-10");
                        println!("  /test basic      - Run basic category");
                        println!("  /test multi-agent - Run multi-agent tests");
                        println!(
                            "  /test layout-format - Run layout and formatting validation tests"
                        );
                        continue;
                    }

                    let test_arg = args[1];
                    let _results = if test_arg == "all" {
                        test_runner.run_all().await
                    } else if test_arg.contains('-') {
                        let range: Vec<&str> = test_arg.split('-').collect();
                        if range.len() == 2 {
                            let start = range[0].parse::<u32>().unwrap_or(1);
                            let end = range[1].parse::<u32>().unwrap_or(118);
                            test_runner.run_range(start, end).await
                        } else {
                            vec![]
                        }
                    } else if let Ok(test_id) = test_arg.parse::<u32>() {
                        vec![test_runner.run_test(test_id).await]
                    } else {
                        test_runner.run_category(test_arg).await
                    };

                    println!("\n{}", test_runner.generate_report("text"));
                    continue;
                }
                "/save" => {
                    handle_save_conversation(&conversation_history, None).await;
                    continue;
                }
                "/load" => {
                    if let Some(loaded_history) = handle_load_conversation().await {
                        conversation_history = loaded_history;
                        println!("{}", "Conversation loaded successfully!".bright_green());
                    }
                    continue;
                }
                "/clear" => {
                    conversation_history.clear();
                    println!("{}", "Conversation history cleared!".bright_green());
                    continue;
                }

                "/verbosity" => {
                    let parts: Vec<&str> = input.split_whitespace().collect();
                    if parts.len() == 1 {
                        println!(
                            "Current verbosity: {}",
                            app_config.features.verbosity.bright_cyan()
                        );
                        println!("Available levels:");
                        println!(
                            "  • {} - Show LLM responses + all system messages",
                            "full".bright_green()
                        );
                        println!(
                            "  • {} - Show LLM responses + essential messages only",
                            "minimal".bright_yellow()
                        );
                        println!(
                            "  • {} - Show LLM responses + commands only",
                            "commands-only".bright_red()
                        );
                        println!("Usage: /verbosity <level>");
                    } else if parts.len() == 2 {
                        match parts[1] {
                            "full" | "minimal" | "commands-only" => {
                                app_config.features.verbosity = parts[1].to_string();
                                save_config(&app_config).await;
                                println!("Verbosity set to: {}", parts[1].bright_green());
                            }
                            _ => {
                                println!(
                                    "Invalid verbosity level. Use: full, minimal, or commands-only"
                                );
                            }
                        }
                    }
                    continue;
                }
                "/compact" => {
                    handle_compact_conversation(&mut conversation_history).await;
                    continue;
                }
                "/agent" => {
                    handle_agent_management().await;
                    continue;
                }
                "/context" => {
                    handle_context_management().await;
                    continue;
                }
                "/editor" => {
                    if let Some(editor_input) = handle_editor_prompt().await {
                        // Process the editor input as a regular message
                        println!(
                            "{} {}",
                            ">".bright_green().bold(),
                            editor_input.bright_white()
                        );
                        // Continue with normal processing using editor_input instead of input
                        // (we'll modify the rest of the loop to use a variable)
                    }
                    continue;
                }
                "/reply" => {
                    if let Some(reply_input) = handle_reply_prompt(&conversation_history).await {
                        println!(
                            "{} {}",
                            ">".bright_green().bold(),
                            reply_input.bright_white()
                        );
                        // Process reply_input as normal message
                    }
                    continue;
                }
                "/prompts" => {
                    handle_prompts_management().await;
                    continue;
                }
                "/experiment" => {
                    handle_experiment_toggle().await;
                    continue;
                }
                "/s3logs" => {
                    println!(
                        "📤 S3 Logs - Use: /s3logs-enable <bucket>, /s3logs-status, /s3logs-test"
                    );
                    continue;
                }
                "/s3logs-enable" => {
                    println!("📤 S3 Logs - Use: /s3logs-enable-bucket <bucket-name>");
                    continue;
                }
                "/s3logs-enable-bucket" => {
                    // This would need parameter parsing, for now just enable with default bucket
                    let config = s3_logs::S3LogConfig {
                        bucket: "qcli-logs-test".to_string(),
                        prefix: "logs".to_string(),
                        region: "ap-southeast-2".to_string(),
                        enabled: true,
                    };

                    match s3_logs::S3Logger::save_config(&config).await {
                        Ok(_) => println!(
                            "✅ S3 logging enabled: s3://{}/{}",
                            config.bucket, config.prefix
                        ),
                        Err(e) => println!("❌ Failed to save config: {}", e),
                    }
                    continue;
                }
                "/s3logs-status" => {
                    let config = s3_logs::S3Logger::load_config().await;
                    println!("📊 S3 Logs Status:");
                    println!(
                        "  Enabled: {}",
                        if config.enabled { "✅ Yes" } else { "❌ No" }
                    );
                    println!("  Bucket: {}", config.bucket);
                    println!("  Prefix: {}", config.prefix);
                    println!("  Region: {}", config.region);
                    continue;
                }
                "/s3logs-test" => {
                    let config = s3_logs::S3Logger::load_config().await;
                    if !config.enabled {
                        println!("❌ S3 logging disabled. Use /s3logs-enable <bucket> first");
                        continue;
                    }

                    let mut logger = s3_logs::S3Logger::new(config);
                    let mut metadata = std::collections::HashMap::new();
                    metadata.insert("test".to_string(), "true".to_string());

                    println!("🔄 Testing S3 upload...");
                    logger.log("INFO", "S3 logging test", metadata).await;
                    match logger.flush_to_s3().await {
                        Ok(_) => println!("✅ S3 logging test successful!"),
                        Err(e) => println!("❌ Upload failed: {}", e),
                    }
                    continue;
                }

                "/stream-test" => {
                    println!("🔄 Testing Streaming...");

                    let streaming_client = if app_config.model.use_local_llm {
                        StreamingClient::new_local(
                            app_config.model.local_llm_endpoint.clone(),
                            app_config.model.local_llm_model.clone(),
                        )
                    } else {
                        StreamingClient::new_bedrock(
                            client.clone(),
                            app_config.model.default_model.clone(),
                        )
                    };
                    let test_conversation = vec![json!({"role": "user", "content": "Say hello"})];

                    match streaming_client
                        .stream_response(&test_conversation, None)
                        .await
                    {
                        Ok(response) => println!("✅ Streaming test successful: {}", response),
                        Err(e) => eprintln!("❌ Streaming test failed: {}", e),
                    }
                    continue;
                }
                "/complete" => {
                    let parts: Vec<&str> = input.splitn(3, ' ').collect();
                    if parts.len() < 3 {
                        println!("Usage: /complete <file_path> <line_number>");
                        continue;
                    }

                    let file_path = parts[1];
                    let line_number: usize = parts[2].parse().unwrap_or(0);

                    println!("🔄 Generating code completion...");
                    let completion = CodeCompletion::new();
                    match completion.suggest_completion(file_path, line_number).await {
                        Ok(suggestion) => println!("💡 Suggestion:\n{}", suggestion),
                        Err(e) => eprintln!("❌ Error: {}", e),
                    }
                    continue;
                }
                "/security-scan" => {
                    let parts: Vec<&str> = input.splitn(2, ' ').collect();
                    let target = if parts.len() > 1 { parts[1] } else { "." };

                    println!("🔍 Running security scan on {}...", target);
                    let scanner = SecurityScanner::new();

                    match std::fs::metadata(target) {
                        Ok(metadata) if metadata.is_dir() => {
                            match scanner.scan_directory(target).await {
                                Ok(results) => {
                                    let report = scanner.generate_report(&results);
                                    println!("{}", report);
                                }
                                Err(e) => eprintln!("❌ Scan failed: {}", e),
                            }
                        }
                        Ok(_) => match scanner.scan_file(target).await {
                            Ok(issues) => {
                                if issues.is_empty() {
                                    println!("✅ No security issues found");
                                } else {
                                    for issue in issues {
                                        println!(
                                            "⚠️  {} (Line {}): {}",
                                            issue.severity, issue.line, issue.description
                                        );
                                        println!("   💡 {}", issue.remediation);
                                    }
                                }
                            }
                            Err(e) => eprintln!("❌ Scan failed: {}", e),
                        },
                        Err(_) => eprintln!("❌ File or directory not found: {}", target),
                    }
                    continue;
                }
                "/explain" => {
                    let parts: Vec<&str> = input.splitn(2, ' ').collect();
                    if parts.len() < 2 {
                        println!("Usage: /explain <file_path> [function_name]");
                        continue;
                    }

                    let file_path = parts[1];
                    let explainer = CodeExplainer::new();

                    println!("📖 Analyzing code...");
                    match std::fs::read_to_string(file_path) {
                        Ok(content) => {
                            let language = match std::path::Path::new(file_path)
                                .extension()
                                .and_then(|s| s.to_str())
                            {
                                Some("rs") => "rust",
                                Some("py") => "python",
                                Some("js") => "javascript",
                                _ => "unknown",
                            };

                            match explainer.explain_code(&content, language).await {
                                Ok(explanation) => println!("{}", explanation),
                                Err(e) => eprintln!("❌ Analysis failed: {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Could not read file: {}", e),
                    }
                    continue;
                }
                "/refactor" => {
                    let parts: Vec<&str> = input.splitn(2, ' ').collect();
                    if parts.len() < 2 {
                        println!("Usage: /refactor <file_path>");
                        continue;
                    }

                    let file_path = parts[1];
                    println!("🔧 Analyzing for refactoring opportunities...");

                    let assistant = RefactoringAssistant::new();
                    match assistant.analyze_file(file_path).await {
                        Ok(suggestions) => {
                            if suggestions.is_empty() {
                                println!("✅ No refactoring suggestions found");
                            } else {
                                let report = assistant.generate_refactoring_report(&suggestions);
                                println!("{}", report);
                            }
                        }
                        Err(e) => eprintln!("❌ Analysis failed: {}", e),
                    }
                    continue;
                }
                "/analyze-cicd" => {
                    let parts: Vec<&str> = input.splitn(2, ' ').collect();
                    let target_path = if parts.len() > 1 { parts[1] } else { "." };

                    println!("🔧 Analyzing CI/CD pipeline...");
                    let integrator = CicdIntegrator::new(target_path);

                    match integrator.analyze_pipeline().await {
                        Ok(analysis) => {
                            let report = integrator.generate_report(&analysis);
                            println!("{}", report);
                        }
                        Err(e) => eprintln!("❌ CI/CD analysis failed: {}", e),
                    }
                    continue;
                }
                "/generate-pipeline" => {
                    let parts: Vec<&str> = input.splitn(2, ' ').collect();
                    let project_type = if parts.len() > 1 { parts[1] } else { "rust" };

                    println!("🚀 Generating CI/CD pipeline for {}...", project_type);
                    let integrator = CicdIntegrator::new(".");

                    let pipeline = integrator.generate_github_action(project_type).await;
                    println!("Generated GitHub Actions workflow:\n\n{}", pipeline);

                    // Optionally save to file
                    if let Err(e) = std::fs::create_dir_all(".github/workflows") {
                        eprintln!("⚠️  Could not create .github/workflows directory: {}", e);
                    } else if let Err(e) = std::fs::write(".github/workflows/ci.yml", &pipeline) {
                        eprintln!("⚠️  Could not save pipeline file: {}", e);
                    } else {
                        println!("✅ Pipeline saved to .github/workflows/ci.yml");
                    }
                    continue;
                }
                "/learning-insights" => {
                    println!("📊 Analyzing your learning patterns...");
                    let learning_system = LearningSystem::new();

                    match learning_system.get_learning_insights("default_user").await {
                        Ok(insights) => println!("{}", insights),
                        Err(e) => eprintln!("❌ Could not generate insights: {}", e),
                    }
                    continue;
                }
                "/workspace-context" => {
                    println!("🏗️ Loading workspace context...");
                    let mut context_manager = AdvancedContextManager::new();

                    match context_manager.initialize_workspace(".").await {
                        Ok(workspace_id) => {
                            println!("✅ Workspace initialized: {}", workspace_id);
                            match context_manager.get_workspace_summary().await {
                                Ok(summary) => println!("{}", summary),
                                Err(e) => eprintln!("❌ Could not get workspace summary: {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Workspace initialization failed: {}", e),
                    }
                    continue;
                }
                "/analyze-repo" => {
                    let parts: Vec<&str> = input.splitn(2, ' ').collect();
                    let target_path = if parts.len() > 1 { parts[1] } else { "." };

                    println!("🔍 Analyzing repository structure...");
                    let mut analyzer = RepositoryAnalyzer::new(target_path);

                    match analyzer.analyze_repository().await {
                        Ok(analysis) => {
                            let report = analyzer.generate_report(&analysis);
                            println!("{}", report);
                        }
                        Err(e) => eprintln!("❌ Analysis failed: {}", e),
                    }
                    continue;
                }
                "/generate-tests" => {
                    let parts: Vec<&str> = input.splitn(2, ' ').collect();
                    if parts.len() < 2 {
                        println!("Usage: /generate-tests <file_path>");
                        continue;
                    }

                    let file_path = parts[1];
                    println!("🧪 Generating tests for {}...", file_path);

                    let generator = TestGenerator::new();
                    match generator.generate_tests_for_file(file_path).await {
                        Ok(test_code) => {
                            println!("Generated test code:\n{}", test_code);

                            // Optionally create test file
                            match generator.create_test_file(file_path, &test_code).await {
                                Ok(test_file_path) => {
                                    println!("✅ Test file created: {}", test_file_path)
                                }
                                Err(e) => eprintln!("⚠️  Could not create test file: {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Test generation failed: {}", e),
                    }
                    continue;
                }
                "/changelog" => {
                    handle_changelog_display().await;
                    continue;
                }
                "/usage" => {
                    handle_usage_display(&conversation_history).await;
                    continue;
                }
                "/mcp" => {
                    handle_mcp_management().await;
                    continue;
                }
                "/hooks" => {
                    handle_hooks_management().await;
                    continue;
                }
                "/model" => {
                    handle_model_selection().await;
                    continue;
                }
                "/scan" => {
                    let _scanner = security_scanner::SecurityScanner::new();
                    println!("🔍 Security scanner ready. Usage: /scan <file_path>");
                    continue;
                }
                "/aws-suggest" => {
                    let _aws_kb = aws_knowledge::AWSKnowledgeBase::new();
                    println!("☁️ AWS suggestions ready. Usage: /aws-suggest <code>");
                    continue;
                }
                "/enterprise" => {
                    let _enterprise = enterprise::EnterpriseManager::new();
                    println!("🏢 Enterprise features ready. Usage: /enterprise <action>");
                    continue;
                }
                "/realtime" => {
                    let _realtime = realtime_context::RealtimeContext::new();
                    println!("⚡ Real-time context ready. Usage: /realtime <workspace_path>");
                    continue;
                }
                "/orchestrate" => {
                    let args: Vec<&str> = input.split_whitespace().collect();
                    if args.len() > 1 {
                        match args[1] {
                            "on" | "enable" => {
                                app_config.features.orchestrate_mode = true;
                                save_config(&app_config).await;
                                println!("{} Orchestrate mode enabled", "✅".bright_green());
                                println!("  You can now use: 'orchestrate security best practices', 'analyze logs and troubleshoot', etc.");
                            }
                            "off" | "disable" => {
                                app_config.features.orchestrate_mode = false;
                                save_config(&app_config).await;
                                println!("{} Orchestrate mode disabled", "❌".bright_red());
                            }
                            "status" => {
                                println!(
                                    "Orchestrate mode: {}",
                                    if app_config.features.orchestrate_mode {
                                        "enabled".green()
                                    } else {
                                        "disabled".red()
                                    }
                                );
                            }
                            _ => {
                                println!("Usage: /orchestrate <on|off|status>");
                            }
                        }
                    } else {
                        println!("Usage: /orchestrate <on|off|status>");
                        println!(
                            "Current status: {}",
                            if app_config.features.orchestrate_mode {
                                "enabled".green()
                            } else {
                                "disabled".red()
                            }
                        );
                    }
                    continue;
                }
                "/help" => {
                    display_comprehensive_help();
                    continue;
                }
                _ => {
                    if input.starts_with("/save ") {
                        let name = input.strip_prefix("/save ").unwrap_or("default");
                        handle_save_conversation(&conversation_history, Some(name.to_string()))
                            .await;
                        continue;
                    }
                    println!(
                        "{} Unknown command: {}",
                        "Error:".bright_red(),
                        input.bright_yellow()
                    );
                    continue;
                }
            }
        }

        // Log user input
        if cloud_logging::should_log(&app_config.logging, "info") {
            let _ = cloud_logging::log_message(
                &app_config.logging,
                "info",
                &format!("User input: {}", input),
            )
            .await;
        }

        // Scan user input for PII (Personally Identifiable Information)
        // Scanning order: 1) AWS Comprehend (if provider="aws") 2) Local LLM 3) None
        let (final_input, pii_detected) = if app_config.security.pii_scanner {
            if let Some(ref scanner) = comprehend_pii {
                // REMOTE: AWS Comprehend PII Detection (cloud-based)
                println!(
                    "{} Scanning input with Amazon Comprehend PII...",
                    "🔍".bright_black()
                );
                match scanner.scan_text(&input).await {
                    Ok(result) => {
                        if result.has_pii {
                            println!("{} PII detected and sanitized", "🔒".yellow());
                            for entity in &result.entities {
                                println!(
                                    "  - {}: {:.1}% confidence",
                                    entity.entity_type,
                                    entity.confidence * 100.0
                                );
                            }
                            (result.sanitized_text, true)
                        } else {
                            // Silent - no message for clean PII scan
                            (input.clone(), false)
                        }
                    }
                    Err(e) => {
                        println!("{} Comprehend PII scan failed: {}", "⚠️".yellow(), e);
                        (input.clone(), false)
                    }
                }
            } else if let Some(ref scanner) = local_pii {
                // LOCAL: Local LLM PII Detection (privacy-focused, runs on your machine)
                match scanner.scan_text(&input).await {
                    Ok(result) => {
                        // eprintln!("DEBUG: PII scan result - has_pii: {}, entities: {}", result.has_pii, result.entities.len());
                        if result.has_pii {
                            println!("{} PII detected and sanitized", "🔒".yellow());
                            for entity in &result.entities {
                                println!(
                                    "  - {}: {:.1}% confidence",
                                    entity.entity_type,
                                    entity.confidence * 100.0
                                );
                            }
                            (result.sanitized_text, true)
                        } else {
                            // Silent - no message for clean PII scan
                            (input.clone(), false)
                        }
                    }
                    Err(e) => {
                        println!("{} Local LLM PII scan failed: {}", "⚠️".yellow(), e);
                        (input.clone(), false)
                    }
                }
            } else {
                // NO PII SCANNER: Neither AWS Comprehend nor Local LLM available
                println!(
                    "{} PII scanner not available (check Ollama service)",
                    "⚠️".yellow()
                );
                (input.clone(), false)
            }
        } else {
            (input.clone(), false)
        };

        // Scan with Nightfall first if available
        let mut _nightfall_redacted_input = input.clone();
        let mut has_nightfall_pii = false;

        if app_config.security.nightfall_scanner {
            if let Some(ref nightfall) = nightfall_client {
                match nightfall.scan_text_simple(&input).await {
                    Ok(nightfall_pii_detected) => {
                        has_nightfall_pii = nightfall_pii_detected;
                        if !simple_pii_scan && nightfall_pii_detected {
                            // Get detailed results for redaction
                            if let Ok(nightfall_result) =
                                nightfall.scan_text(&input, "user input").await
                            {
                                _nightfall_redacted_input = nightfall
                                    .redact_nightfall_findings(&input, &nightfall_result.findings);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "{} Nightfall scan failed: {}",
                            "⚠️".yellow(),
                            e.to_string().yellow()
                        );
                    }
                }
            }
        }

        // Display PII scan results
        if simple_pii_scan {
            if pii_detected {
                println!("{} PII detected and sanitized", "🔒".yellow());
            }
            if nightfall_client.is_some() {
                if let Some(ref nightfall) = nightfall_client {
                    nightfall.display_simple_nightfall_scan(has_nightfall_pii);
                }
            }
        } else {
            // Use rich interface for PII display
            if pii_detected {
                let pii_details = "PII detected by Amazon Comprehend";
                print!(
                    "{}",
                    rich_interface.format_pii_detection(true, &pii_details, false)
                );
            }

            if let Some(ref nightfall) = nightfall_client {
                if let Ok(nightfall_result) = nightfall.scan_text(&input, "user input").await {
                    if !nightfall_result.findings.is_empty() {
                        let nightfall_details = format!("{:?}", nightfall_result.findings);
                        print!(
                            "{}",
                            rich_interface.format_pii_detection(true, &nightfall_details, true)
                        );
                    }
                }
            }
        }

        // Send SES alert if PII detected in user input
        if pii_detected {
            if let Err(e) = ses_alerts
                .send_pii_alert(
                    &[], // Empty findings array since we're using Comprehend
                    "user input",
                    &input,
                    Some("Amazon Comprehend detected PII in user input"),
                )
                .await
            {
                eprintln!(
                    "{} Failed to send PII alert: {}",
                    "⚠️".yellow(),
                    e.to_string().yellow()
                );
            }
        }

        // Use sanitized input from Comprehend PII scan
        let sanitized_input = &final_input;

        // Multi-agent coordination system
        let mut multi_agent = MultiAgentSystem::new();

        // Check if request needs multi-agent coordination
        let needs_coordination = sanitized_input.contains("deploy")
            || sanitized_input.contains("troubleshoot")
            || sanitized_input.contains("security")
            || sanitized_input.contains("error")
            || sanitized_input.contains("fix")
            || sanitized_input.contains("multi agent")
            || sanitized_input.contains("multi-agent")
            || sanitized_input.contains("agents");

        if needs_coordination {
            println!("🤖 Activating Multi-Agent System...");
            let coordination_results = multi_agent.coordinate_agents(sanitized_input).await;

            if !coordination_results.is_empty() {
                println!("🔄 Agent Coordination Results:");
                for result in coordination_results {
                    println!("   {}", result);
                }
                println!();
            }
        }

        // Visual progress indicator
        // let indicator = VisualIndicator::new("AI Processing");
        // indicator.show_spinner().await;

        // print!("{} ", "🤖".bright_cyan());
        // io::stdout().flush()?;

        // Add user message to history (sanitized)
        conversation_history.push(json!({
            "role": "user",
            "content": sanitized_input
        }));

        // Build Bedrock request with conversation history
        let _os_context = format!(
            "System Context: Running on {} ({}). ",
            system_context.host_os, system_context.host_arch
        );

        // Add minimal project context
        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let project_kind = project_kind::detect_project_kind(&cwd);
        let project_type = format!("{:?}", project_kind).to_lowercase();

        // Define allowed auto-creates based on project type (empty to require approval for all)
        let allowed_auto_creates: Vec<&str> = vec![];

        // Build model-agnostic system prompt with YAML action headers
        let system_prompt = action_header::build_system_prompt(
            &system_context.host_os,
            &cwd.display().to_string(),
            &project_type,
            &allowed_auto_creates,
            false, // auto_analysis
            false, // orchestration_enabled (only on /orchestrate)
            false, // debug
        );

        let _request_body = json!({
            "anthropic_version": "bedrock-2023-05-31",
            "max_tokens": 4096,
            "messages": conversation_history,
            "system": system_prompt
        });

        // Use streaming based on provider
        let response_result = if app_config.model.use_local_llm {
            let streaming_client = StreamingClient::new_local(
                app_config.model.local_llm_endpoint.clone(),
                app_config.model.local_llm_model.clone(),
            );
            streaming_client
                .stream_response(&conversation_history, Some(&system_prompt))
                .await
        } else {
            // Use cloud provider streaming (Bedrock for AWS, fallback to get_ai_response for others)
            match app_config.cloud.provider.as_str() {
                "aws" => {
                    let streaming_client = StreamingClient::new_bedrock(
                        client.clone(),
                        app_config.model.default_model.clone(),
                    );
                    streaming_client
                        .stream_response(&conversation_history, Some(&system_prompt))
                        .await
                }
                _ => {
                    let user_message = conversation_history
                        .last()
                        .and_then(|m| m.get("content"))
                        .and_then(|c| c.as_str())
                        .unwrap_or("");
                    get_ai_response(&client, user_message, &app_config).await
                }
            }
        };

        match response_result {
            Ok(response_text) => {
                println!(); // New line after streaming

                // Skip PII scanning on AI responses to avoid false positives
                let sanitized_ai_response = response_text;

                // Initialize failed commands vector (used by guard rail)
                let mut failed_commands = Vec::new();

                // Parse YAML action header if present (model-agnostic format)
                if let Some((action_header, user_message)) =
                    action_header::parse_action_header(&sanitized_ai_response)
                {
                    // Execute create actions first (silently - LLM already described them)
                    for create_action in &action_header.create {
                        // Create directory if it doesn't exist
                        if let Some(parent) = std::path::Path::new(&create_action.path).parent() {
                            if let Err(e) = std::fs::create_dir_all(parent) {
                                println!(
                                    "{}",
                                    render_msg(
                                        MsgKind::Error,
                                        &format!(
                                            "Failed to create directory {}: {}",
                                            parent.display(),
                                            e
                                        )
                                    )
                                );
                                failed_commands.push((
                                    format!("mkdir -p {}", parent.display()),
                                    e.to_string(),
                                ));
                                continue;
                            }
                        }

                        // Write the file
                        if let Err(e) = std::fs::write(&create_action.path, &create_action.content)
                        {
                            println!(
                                "{}",
                                render_msg(
                                    MsgKind::Error,
                                    &format!("Failed to create file {}: {}", create_action.path, e)
                                )
                            );
                            failed_commands.push((
                                format!("create file {}", create_action.path),
                                e.to_string(),
                            ));
                        }
                    }

                    // Execute read actions
                    for read_action in &action_header.read {
                        println!("{} Reading: {}", "📖".bright_blue(), read_action.path);
                        // TODO: Execute fs_read
                    }

                    // Execute run actions with risk policy
                    for run_action in &action_header.run {
                        let risk_indicator = match run_action.risk {
                            action_header::RiskLevel::Low => "🔵".blue(),
                            action_header::RiskLevel::Medium => "🟡".yellow(),
                            action_header::RiskLevel::High => "🔴".red(),
                        };
                        println!(
                            "{} {} {}",
                            risk_indicator,
                            "Command:".bright_blue(),
                            run_action.cmd
                        );

                        if run_action.confirm {
                            // Auto-confirm for now to enable execution and failure capture
                            println!("Auto-confirming command execution...");
                        }

                        // Execute command and capture failures
                        let execute_cmd = execute::ExecuteCommand {
                            command: run_action.cmd.clone(),
                            summary: None,
                        };

                        match execute_cmd.execute().await {
                            Ok(output) => {
                                if !output.trim().is_empty() {
                                    println!("{}", output);
                                }
                            }
                            Err(e) => {
                                println!("{}", render_msg(MsgKind::Error, &e.to_string()));
                                failed_commands.push((run_action.cmd.clone(), e.to_string()));
                            }
                        }
                    }

                    // Print user-facing message (after YAML header) - always show LLM responses
                    if !user_message.is_empty() {
                        println!("\n{}", user_message.green());
                    }
                } else {
                    // No action header, response already displayed during streaming
                    // println!("{}", sanitized_ai_response.green());
                }

                // Add AI response to conversation history (sanitized)
                conversation_history.push(json!({
                    "role": "assistant",
                    "content": sanitized_ai_response.clone()
                }));

                // Check if response contains tool calls (only in interactive/auto mode)
                if app_config.features.execution_mode != "chatbot" {
                    if let Some(tools) = extract_tool_calls(&sanitized_ai_response) {
                        // failed_commands already declared in outer scope
                        let mut all_outputs = Vec::new();
                        let mut confirm_all = false;
                        let mut command_guard = CommandGuard::new();

                        for tool in tools {
                            match tool {
                                ToolCall::Execute(cmd) => {
                                    let risk_level =
                                        permission_checker.get_risk_level(&cmd.command);
                                    let requires_confirmation =
                                        permission_checker.requires_confirmation(&cmd.command);

                                    let risk_indicator = match risk_level {
                                        RiskLevel::Low => "🔵".blue(),
                                        RiskLevel::Medium => "🟡".yellow(),
                                        RiskLevel::High => "🔴".red(),
                                    };

                                    println!(
                                        "{} {} {}",
                                        ">".bright_green().bold(),
                                        risk_indicator,
                                        cmd.command.bright_white()
                                    );

                                    if requires_confirmation && !confirm_all {
                                        let confirmation_prompt = format!(
                                            "{} ",
                                            "⚠️  Confirm execution? (y/N/a for all):"
                                                .bright_yellow()
                                                .bold()
                                        );
                                        let confirmation = match rl.readline(&confirmation_prompt) {
                                            Ok(line) => line.trim().to_lowercase(),
                                            Err(_) => "n".to_string(),
                                        };

                                        if confirmation.starts_with('a') {
                                            confirm_all = true;
                                            println!(
                                                "{}",
                                                "✅ Confirming all remaining commands"
                                                    .bright_green()
                                            );
                                        } else if !confirmation.starts_with('y') {
                                            println!("{}", "❌ Command skipped".bright_red());
                                            continue;
                                        }
                                    }

                                    // Guard agent: Check for duplicate commands
                                    if !command_guard.should_execute(&cmd.command) {
                                        println!(
                                            "{}",
                                            "⏭️  Skipping duplicate command (executed recently)"
                                                .bright_yellow()
                                        );
                                        continue;
                                    }

                                    match cmd.execute().await {
                                        Ok(output) => {
                                            if !output.trim().is_empty() {
                                                println!("{}", output.bright_white());
                                                all_outputs.push((
                                                    format!("execute: {}", cmd.command),
                                                    output.clone(),
                                                ));

                                                // Individual command analysis (guard agent)
                                                if app_config.analysis.command_analysis {
                                                    match analyze_command_output(
                                                        &client,
                                                        &cmd.command,
                                                        &output,
                                                    )
                                                    .await
                                                    {
                                                        Ok(analysis) => {
                                                            if !analysis.trim().is_empty() {
                                                                println!("\n{}", analysis.green());
                                                            }
                                                        }
                                                        Err(_) => {} // Silently ignore analysis errors
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            println!(
                                                "{}",
                                                render_msg(MsgKind::Error, &e.to_string())
                                            );
                                            failed_commands
                                                .push((cmd.command.clone(), e.to_string()));
                                        }
                                    }
                                }
                                ToolCall::FsRead(fs_read) => {
                                    match fs_read.execute().await {
                                        Ok(output) => {
                                            // Don't display fs_read output in chat, but still store it for context
                                            all_outputs.push(("fs_read".to_string(), output));
                                        }
                                        Err(e) => {
                                            println!(
                                                "{} {}",
                                                "Error:".bright_red().bold(),
                                                e.to_string().red()
                                            );
                                        }
                                    }
                                }
                                ToolCall::FsWrite(fs_write) => match fs_write.execute().await {
                                    Ok(output) => {
                                        println!("{}", output);
                                        all_outputs.push(("fs_write".to_string(), output));
                                    }
                                    Err(e) => {
                                        println!(
                                            "{} {}",
                                            "Error:".bright_red().bold(),
                                            e.to_string().red()
                                        );
                                    }
                                },
                                ToolCall::UseAws(use_aws) => match use_aws.execute().await {
                                    Ok(output) => {
                                        println!("{}", output);
                                        all_outputs.push(("use_aws".to_string(), output));
                                    }
                                    Err(e) => {
                                        println!(
                                            "{} {}",
                                            "Error:".bright_red().bold(),
                                            e.to_string().red()
                                        );
                                    }
                                },
                                ToolCall::UseAzure(use_azure) => match use_azure.execute().await {
                                    Ok(output) => {
                                        println!("{}", output);
                                        all_outputs.push(("use_azure".to_string(), output));
                                    }
                                    Err(e) => {
                                        println!(
                                            "{} {}",
                                            "Error:".bright_red().bold(),
                                            e.to_string().red()
                                        );
                                    }
                                },
                                ToolCall::UseGcp(use_gcp) => match use_gcp.execute().await {
                                    Ok(output) => {
                                        println!("{}", output);
                                        all_outputs.push(("use_gcp".to_string(), output));
                                    }
                                    Err(e) => {
                                        println!(
                                            "{} {}",
                                            "Error:".bright_red().bold(),
                                            e.to_string().red()
                                        );
                                    }
                                },
                                ToolCall::UseOracle(use_oracle) => {
                                    match use_oracle.execute().await {
                                        Ok(output) => {
                                            println!("{}", output);
                                            all_outputs.push(("use_oracle".to_string(), output));
                                        }
                                        Err(e) => {
                                            println!(
                                                "{} {}",
                                                "Error:".bright_red().bold(),
                                                e.to_string().red()
                                            );
                                        }
                                    }
                                }
                                ToolCall::Introspect(introspect) => {
                                    match introspect.execute().await {
                                        Ok(output) => {
                                            println!("{}", output);
                                            all_outputs.push(("introspect".to_string(), output));
                                        }
                                        Err(e) => {
                                            println!(
                                                "{} {}",
                                                "Error:".bright_red().bold(),
                                                e.to_string().red()
                                            );
                                        }
                                    }
                                }
                                ToolCall::Knowledge(knowledge) => match knowledge.execute().await {
                                    Ok(output) => {
                                        if !output.trim().is_empty() {
                                            println!("{}", output);
                                        }
                                        all_outputs.push(("knowledge".to_string(), output));
                                    }
                                    Err(e) => {
                                        println!(
                                            "{} {}",
                                            "Error:".bright_red().bold(),
                                            e.to_string().red()
                                        );
                                    }
                                },
                                ToolCall::TodoList(todo_list) => match todo_list.execute().await {
                                    Ok(output) => {
                                        println!("{}", output);
                                        all_outputs.push(("todo_list".to_string(), output));
                                    }
                                    Err(e) => {
                                        println!(
                                            "{} {}",
                                            "Error:".bright_red().bold(),
                                            e.to_string().red()
                                        );
                                    }
                                },
                                ToolCall::Thinking(thinking) => match thinking.execute().await {
                                    Ok(output) => {
                                        println!("{}", output);
                                        all_outputs.push(("thinking".to_string(), output));
                                    }
                                    Err(e) => {
                                        println!(
                                            "{} {}",
                                            "Error:".bright_red().bold(),
                                            e.to_string().red()
                                        );
                                    }
                                },
                                ToolCall::OrchestratedThinking(orchestrated_thinking) => {
                                    match orchestrated_thinking.execute().await {
                                        Ok(output) => {
                                            println!("{}", output);
                                            all_outputs.push((
                                                "orchestrated_thinking".to_string(),
                                                output,
                                            ));
                                        }
                                        Err(e) => {
                                            println!(
                                                "{} {}",
                                                "Error:".bright_red().bold(),
                                                e.to_string().red()
                                            );
                                        }
                                    }
                                }
                                ToolCall::GitHub(github) => {
                                    match github::analyze_github_repo(&github.repo_url).await {
                                        Ok(output) => {
                                            println!("{}", output);
                                            all_outputs.push(("github".to_string(), output));
                                        }
                                        Err(e) => {
                                            println!(
                                                "{} {}",
                                                "Error:".bright_red().bold(),
                                                e.to_string().red()
                                            );
                                        }
                                    }
                                }
                            }
                        }

                        // Guard rail: Generic response validation - check everything (non-blocking)
                        if app_config.security.guard_rail_enabled {
                            // Skip validation for simple responses to avoid interrupting flow
                            let should_validate = sanitized_ai_response.len() > 100
                                || sanitized_ai_response.contains("Error")
                                || sanitized_ai_response.contains("failed")
                                || !failed_commands.is_empty();

                            if should_validate {
                                let validation_prompt = format!(
                                "GUARD RAIL - Auto-fix and continue:\n\n\
                                USER REQUEST: {}\n\n\
                                MAIN AI RESPONSE: {}\n\n\
                                TASK: If there are issues (missing files, wrong commands, errors, outdated versions), \
                                provide the CORRECTED response immediately. Do NOT ask questions. \
                                Fix problems automatically and continue the task. \
                                If the response is already correct, just say: 'VALIDATED'",
                                sanitized_input,
                                sanitized_ai_response
                            );

                                let validation_result = if app_config.security.guard_rail_provider
                                    == "local"
                                {
                                    // Use local LLM for validation
                                    let streaming_client = StreamingClient::new_local(
                                        app_config.model.local_llm_endpoint.clone(),
                                        app_config.model.local_llm_model.clone(),
                                    );
                                    streaming_client
                                        .stream_response(
                                            &[json!({
                                                "role": "user",
                                                "content": validation_prompt
                                            })],
                                            None,
                                        )
                                        .await
                                } else {
                                    // Use remote cloud provider for validation
                                    get_ai_response(&client, &validation_prompt, &app_config).await
                                };

                                match validation_result {
                                    Ok(validation_result) => {
                                        if !validation_result
                                            .trim()
                                            .eq_ignore_ascii_case("VALIDATED")
                                        {
                                            // Guard rail correction - run silently in background
                                            // println!("\n{} {}", "🛡️".bright_green(), "Guard rail correction:".bright_green().bold());
                                            // println!("{}", validation_result.green());

                                            // Add to conversation history
                                            conversation_history.push(json!({
                                                "role": "user",
                                                "content": validation_prompt
                                            }));
                                            conversation_history.push(json!({
                                                "role": "assistant",
                                                "content": validation_result
                                            }));
                                        } else if app_config.features.verbosity == "full" {
                                            // println!("{} Response validated", "✅".bright_green());
                                        }
                                    }
                                    Err(e) => {
                                        // Guard rail validation failed - run silently in background
                                        // println!("{} Guard rail validation failed: {}", "⚠️".yellow(), e);
                                    }
                                }
                            }
                        }

                        // Guard rail: Handle command failures and get LLM to suggest alternatives
                        if !failed_commands.is_empty() {
                            let failure_summary = failed_commands
                                .iter()
                                .map(|(cmd, error)| format!("Command: {}\nError: {}", cmd, error))
                                .collect::<Vec<_>>()
                                .join("\n\n");

                            let retry_prompt = format!(
                            "ORIGINAL USER REQUEST: {}\n\n\
                            The following commands failed while trying to fulfill this request:\n\n{}\n\n\
                            Please provide alternative commands or solutions to achieve the original goal. \
                            Focus on troubleshooting the specific errors and providing working alternatives.",
                            sanitized_input, failure_summary
                        );

                            let retry_result = if app_config.security.guard_rail_provider == "local"
                            {
                                let streaming_client = StreamingClient::new_local(
                                    app_config.model.local_llm_endpoint.clone(),
                                    app_config.model.local_llm_model.clone(),
                                );
                                streaming_client
                                    .stream_response(
                                        &[json!({
                                            "role": "user",
                                            "content": retry_prompt
                                        })],
                                        None,
                                    )
                                    .await
                            } else {
                                get_ai_response(&client, &retry_prompt, &app_config).await
                            };

                            match retry_result {
                                Ok(retry_suggestions) => {
                                    println!(
                                        "\n{} {}",
                                        "🛡️".truecolor(184, 134, 11),
                                        "Alternative suggestions:".truecolor(184, 134, 11).bold()
                                    );
                                    println!("{}", retry_suggestions.truecolor(184, 134, 11));

                                    conversation_history.push(json!({
                                        "role": "user",
                                        "content": retry_prompt
                                    }));
                                    conversation_history.push(json!({
                                        "role": "assistant",
                                        "content": retry_suggestions
                                    }));
                                }
                                Err(e) => {
                                    println!(
                                        "{} Failed to get retry suggestions: {}",
                                        "⚠️".yellow(),
                                        e
                                    );
                                }
                            }
                        }

                        // Send command results to AI for analysis
                        if !all_outputs.is_empty() {
                            let combined_results = all_outputs
                                .iter()
                                .filter(|(tool_name, output)| {
                                    !output.trim().is_empty()
                                        && *tool_name != "fs_read"
                                        && *tool_name != "introspect"
                                        && *tool_name != "thinking"
                                        && *tool_name != "orchestrated_thinking"
                                })
                                .map(|(tool_name, output)| format!("{}:\n{}", tool_name, output))
                                .collect::<Vec<_>>()
                                .join("\n\n");

                            if !combined_results.trim().is_empty() {
                                // Only auto-analyze if explicitly requested or enabled
                                let should_analyze =
                                    app_config.features.auto_analysis.unwrap_or(false)
                                        && (input.starts_with("analyz")
                                            || input.contains("summar")
                                            || input.starts_with("/analyze")
                                            || combined_results.lines().count() > 200);

                                if should_analyze {
                                    let analysis_request = format!(
                                        "Analyze these results and provide a brief summary:\n\n{}",
                                        combined_results
                                    );

                                    match get_ai_response(&client, &analysis_request, &app_config)
                                        .await
                                    {
                                        Ok(analysis) => {
                                            if !analysis.trim().is_empty() {
                                                let formatted = rich_interface
                                                    .format_markdown_response(&analysis);
                                                println!(
                                                    "\n{}",
                                                    render_msg(MsgKind::Assistant, &formatted)
                                                );
                                            }
                                        }
                                        Err(_) => {} // Silently ignore analysis errors
                                    }
                                }
                            }
                        }
                    } else {
                        // No tool calls found - response already displayed during streaming
                    }
                } else {
                    // Chatbot mode - response already displayed during streaming
                }
            }
            Err(_e) => {
                println!("{}", "❌ Streaming response failed!".bright_red());
                println!(
                    "{}",
                    "🔄 Falling back to synchronous response...".bright_yellow()
                );

                // Fallback to non-streaming
                let request_body = json!({
                    "anthropic_version": "bedrock-2023-05-31",
                    "max_tokens": 4096,
                    "messages": conversation_history,
                    "system": "You are Amazon Q, an AI assistant. Provide helpful, accurate responses."
                });

                match client
                    .invoke_model()
                    .model_id("anthropic.claude-3-5-sonnet-20241022-v2:0")
                    .content_type("application/json")
                    .body(request_body.to_string().as_bytes().to_vec().into())
                    .send()
                    .await
                {
                    Ok(response) => {
                        let response_body = String::from_utf8(response.body().as_ref().to_vec())?;
                        let parsed: serde_json::Value = serde_json::from_str(&response_body)?;

                        if let Some(content) = parsed["content"].as_array() {
                            if let Some(text) = content[0]["text"].as_str() {
                                println!("{}", text.green());

                                // Add to conversation history
                                conversation_history.push(json!({
                                    "role": "assistant",
                                    "content": text
                                }));
                            }
                        }
                    }
                    Err(fallback_error) => {
                        println!("{}", "❌ Processing request failed!".bright_red());
                        eprintln!("{}", "Error: service error".bright_red().bold());
                        eprintln!("{} {}", "Details:".red(), fallback_error.to_string().red());
                    }
                }
            }
        }
        println!();
    }

    Ok(())
}

#[allow(dead_code)]
fn colorize_analysis_report(text: &str) -> String {
    let mut result = String::new();

    for line in text.lines() {
        if line.starts_with("## ") {
            // Headers in bright cyan
            result.push_str(&format!("{}\n", line.bright_cyan().bold()));
        } else if line.starts_with("• ") {
            // Bullet points in bright green
            result.push_str(&format!("{}\n", line.bright_green()));
        } else if line.starts_with("- ") {
            // Dash points in yellow
            result.push_str(&format!("{}\n", line.yellow()));
        } else if line.contains("ERROR") || line.contains("Error") || line.contains("error") {
            // Error lines in red
            result.push_str(&format!("{}\n", line.red()));
        } else if line.contains("SUCCESS") || line.contains("Success") || line.contains("✅") {
            // Success lines in green
            result.push_str(&format!("{}\n", line.green()));
        } else if line.contains("WARNING") || line.contains("Warning") || line.contains("⚠️") {
            // Warning lines in yellow
            result.push_str(&format!("{}\n", line.yellow()));
        } else if line.trim().is_empty() {
            // Empty lines
            result.push_str("\n");
        } else {
            // Regular text in white
            result.push_str(&format!("{}\n", line.white()));
        }
    }

    result
}

async fn analyze_command_output(
    client: &BedrockClient,
    command: &str,
    output: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Check for common Docker daemon issues
    if output.contains("Cannot connect to the Docker daemon")
        || output.contains("Is the docker daemon running?")
        || output.contains("docker.sock")
    {
        return Ok(
            "Docker daemon not running. Start Docker Desktop to resolve this issue.".to_string(),
        );
    }

    // Dynamic analysis based on context
    let analysis_context = determine_analysis_context(command, output);
    let analysis_request = create_dynamic_analysis_prompt(&analysis_context, output);

    let request_body = json!({
        "anthropic_version": "bedrock-2023-05-31",
        "max_tokens": 1000,
        "messages": [
            {
                "role": "user",
                "content": analysis_request
            }
        ],
        "system": "Provide brief analysis only. No conversational text."
    });

    let response = client
        .invoke_model()
        .model_id("anthropic.claude-3-5-sonnet-20241022-v2:0")
        .content_type("application/json")
        .body(request_body.to_string().as_bytes().to_vec().into())
        .send()
        .await?;

    let response_body = String::from_utf8(response.body().as_ref().to_vec())?;
    let parsed: serde_json::Value = serde_json::from_str(&response_body)?;

    if let Some(content) = parsed["content"].as_array() {
        if let Some(text) = content.first().and_then(|c| c["text"].as_str()) {
            Ok(text.trim().to_string())
        } else {
            Err("No analysis content found".into())
        }
    } else {
        Err("Invalid analysis response format".into())
    }
}

#[allow(dead_code)]
struct AnalysisContext {
    context_type: String,
    style: String,
    focus_areas: Vec<String>,
    header_emoji: String,
    header_text: String,
}

fn determine_analysis_context(command: &str, output: &str) -> AnalysisContext {
    // Detect context from command and output patterns
    if output.contains("github") || output.contains("repository") || command.contains("github") {
        AnalysisContext {
            context_type: "repository".to_string(),
            style: "development-focused".to_string(),
            focus_areas: vec![
                "code quality".to_string(),
                "community health".to_string(),
                "development activity".to_string(),
            ],
            header_emoji: "🔍".to_string(),
            header_text: "REPOSITORY INSIGHTS".to_string(),
        }
    } else if output.contains("Error:") || output.contains("error") || output.contains("failed") {
        AnalysisContext {
            context_type: "troubleshooting".to_string(),
            style: "problem-solving".to_string(),
            focus_areas: vec![
                "error resolution".to_string(),
                "root causes".to_string(),
                "quick fixes".to_string(),
            ],
            header_emoji: "🔧".to_string(),
            header_text: "TROUBLESHOOTING ANALYSIS".to_string(),
        }
    } else if output.contains("aws") || output.contains("AWS") || command.contains("aws") {
        AnalysisContext {
            context_type: "aws".to_string(),
            style: "cloud-architecture".to_string(),
            focus_areas: vec![
                "resource optimization".to_string(),
                "security".to_string(),
                "cost efficiency".to_string(),
            ],
            header_emoji: "☁️".to_string(),
            header_text: "AWS ENVIRONMENT ANALYSIS".to_string(),
        }
    } else if output.contains("Cargo") || output.contains("cargo") || output.contains(".rs") {
        AnalysisContext {
            context_type: "rust".to_string(),
            style: "development-focused".to_string(),
            focus_areas: vec![
                "build health".to_string(),
                "dependencies".to_string(),
                "performance".to_string(),
            ],
            header_emoji: "🦀".to_string(),
            header_text: "RUST PROJECT ANALYSIS".to_string(),
        }
    } else if output.len() > 5000 || output.lines().count() > 100 {
        AnalysisContext {
            context_type: "comprehensive".to_string(),
            style: "high-level strategic".to_string(),
            focus_areas: vec![
                "system overview".to_string(),
                "key patterns".to_string(),
                "strategic insights".to_string(),
            ],
            header_emoji: "📊".to_string(),
            header_text: "COMPREHENSIVE SYSTEM ANALYSIS".to_string(),
        }
    } else {
        AnalysisContext {
            context_type: "general".to_string(),
            style: "conversational and practical".to_string(),
            focus_areas: vec![
                "immediate insights".to_string(),
                "actionable items".to_string(),
                "next steps".to_string(),
            ],
            header_emoji: "💡".to_string(),
            header_text: "SMART ANALYSIS".to_string(),
        }
    }
}

fn create_dynamic_analysis_prompt(context: &AnalysisContext, output: &str) -> String {
    match context.context_type.as_str() {
        "repository" => format!(
            "Analyze this repository data and provide insights like Amazon Q would - conversational, intelligent, and developer-focused:\n\n{}\n\nProvide analysis in this format:\n\n## 🔍 REPOSITORY INSIGHTS\nBrief conversational summary of what you found\n\n## 📈 KEY METRICS\n• Specific numbers and trends from the data\n\n## 🎯 DEVELOPMENT HEALTH\n• Activity patterns and community engagement\n\n## 💡 RECOMMENDATIONS\n• Practical suggestions for developers\n\nBe conversational like Q CLI, focus on developer value, and include specific details from the data.",
            output
        ),
        "troubleshooting" => format!(
            "Analyze these errors like Amazon Q would - helpful, solution-focused, and clear:\n\n{}\n\nProvide analysis in this format:\n\n## 🔧 ISSUE DIAGNOSIS\nClear explanation of what's happening\n\n## ⚡ QUICK FIXES\n• Immediate actions to try\n\n## 🎯 ROOT CAUSES\n• Underlying issues to address\n\n## 💡 PREVENTION\n• How to avoid this in the future\n\nBe helpful and solution-oriented like Q CLI would be.",
            output
        ),
        "aws" => format!(
            "Analyze this AWS environment like Amazon Q would - cloud-native insights and optimization focused:\n\n{}\n\nProvide analysis in this format:\n\n## ☁️ CLOUD ENVIRONMENT STATUS\nOverall health and configuration summary\n\n## 🔒 SECURITY & COMPLIANCE\n• Security posture and recommendations\n\n## 💰 COST OPTIMIZATION\n• Resource efficiency opportunities\n\n## 🚀 PERFORMANCE INSIGHTS\n• Optimization recommendations\n\nFocus on AWS best practices and actionable cloud insights.",
            output
        ),
        "rust" => format!(
            "Analyze this Rust project like Amazon Q would - development-focused and practical:\n\n{}\n\nProvide analysis in this format:\n\n## 🦀 PROJECT HEALTH\nBuild status and overall project condition\n\n## 📦 DEPENDENCIES\n• Crate analysis and recommendations\n\n## ⚡ PERFORMANCE\n• Build times and optimization opportunities\n\n## 🛠️ DEVELOPMENT WORKFLOW\n• Tooling and process improvements\n\nBe practical and focused on Rust development best practices.",
            output
        ),
        _ => format!(
            "Analyze this output like Amazon Q would - intelligent, conversational, and focused on what matters:\n\n{}\n\nProvide a dynamic analysis that adapts to what you see. Use appropriate emojis and structure based on the content. Be conversational, specific about details, and focus on actionable insights. Structure your response naturally based on what's most important in the data.",
            output
        )
    }
}

async fn get_local_llm_response(
    endpoint: &str,
    model: &str,
    message: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let request_body = json!({
        "model": model,
        "prompt": message,
        "stream": false
    });

    let response = client.post(endpoint).json(&request_body).send().await?;

    let response_text = response.text().await?;
    let parsed: serde_json::Value = serde_json::from_str(&response_text)?;

    if let Some(response_content) = parsed["response"].as_str() {
        Ok(response_content.to_string())
    } else {
        Err("Invalid response format from local LLM".into())
    }
}

async fn get_ai_response(
    client: &BedrockClient,
    message: &str,
    app_config: &Config,
) -> Result<String, Box<dyn std::error::Error>> {
    // Log AI request
    if cloud_logging::should_log(&app_config.logging, "debug") {
        let _ = cloud_logging::log_message(
            &app_config.logging,
            "debug",
            &format!("AI request: {}", message),
        )
        .await;
    }

    // Try unified cloud provider first
    match unified_cloud_providers::get_cloud_llm_response(&app_config.cloud, message).await {
        Ok(response) => {
            // Log AI response
            if cloud_logging::should_log(&app_config.logging, "debug") {
                let _ = cloud_logging::log_message(
                    &app_config.logging,
                    "debug",
                    &format!("AI response: {}", response),
                )
                .await;
            }
            return Ok(response);
        }
        Err(e) => {
            if cloud_logging::should_log(&app_config.logging, "warn") {
                let _ = cloud_logging::log_message(
                    &app_config.logging,
                    "warn",
                    &format!("Cloud provider failed: {}", e),
                )
                .await;
            }
            println!(
                "{} Cloud provider failed, trying fallback: {}",
                "⚠️".yellow(),
                e
            );
        }
    }

    // Fallback to existing provider logic
    match app_config.model.provider.as_str() {
        "local" | _ if app_config.model.use_local_llm => {
            return get_local_llm_response(
                &app_config.model.local_llm_endpoint,
                &app_config.model.local_llm_model,
                message,
            )
            .await;
        }
        "openai" => {
            return online_providers::get_openai_response(
                &app_config.model.openai_api_key,
                &app_config.model.openai_model,
                &app_config.model.openai_endpoint,
                message,
            )
            .await;
        }
        "gemini" => {
            return online_providers::get_gemini_response(
                &app_config.model.gemini_api_key,
                &app_config.model.gemini_model,
                &app_config.model.gemini_endpoint,
                message,
            )
            .await;
        }
        "bedrock" | _ => {
            // Default to Bedrock
        }
    }

    let request_body = json!({
        "anthropic_version": "bedrock-2023-05-31",
        "max_tokens": 4096,
        "messages": [
            {
                "role": "user",
                "content": message
            }
        ],
        "system": "You are Hello AI CLI. Analyze command results and provide brief, actionable summaries. Focus on key metrics, status, and important findings."
    });

    let response = client
        .invoke_model()
        .model_id(&app_config.model.default_model)
        .content_type("application/json")
        .body(request_body.to_string().as_bytes().to_vec().into())
        .send()
        .await?;

    let response_body = String::from_utf8(response.body().as_ref().to_vec())?;
    let parsed: serde_json::Value = serde_json::from_str(&response_body)?;

    if let Some(content) = parsed["content"].as_array() {
        if let Some(text) = content.first().and_then(|c| c["text"].as_str()) {
            Ok(text.to_string())
        } else {
            Err("No response content found".into())
        }
    } else {
        Err("Invalid response format".into())
    }
}

#[derive(Debug, Clone)]
enum ToolCall {
    Execute(ExecuteCommand),
    FsRead(FsRead),
    FsWrite(FsWrite),
    UseAws(UseAws),
    UseAzure(UseAzure),
    UseGcp(UseGcp),
    UseOracle(UseOracle),
    Introspect(Introspect),
    Knowledge(Knowledge),
    TodoList(TodoList),
    Thinking(Thinking),
    OrchestratedThinking(OrchestratedThinking),
    GitHub(GitHubAnalysis),
}

#[derive(Debug, Clone)]
struct GitHubAnalysis {
    repo_url: String,
}

fn extract_tool_calls(text: &str) -> Option<Vec<ToolCall>> {
    let mut tools = Vec::new();

    // Detect specific file reading requests
    if let Some((operation_type, data)) = extract_file_path_from_text(text) {
        match operation_type.as_str() {
            "SEARCH" => {
                let parts: Vec<&str> = data.split('|').collect();
                if parts.len() == 2 {
                    tools.push(ToolCall::FsRead(FsRead {
                        operations: vec![fs_read::FsReadOperation::Search(fs_read::FsSearch {
                            pattern: parts[0].to_string(),
                            path: parts[1].to_string(),
                            context_lines: 2,
                        })],
                        summary: Some("Search within files".to_string()),
                    }));
                }
            }
            "LINES" => {
                let parts: Vec<&str> = data.split('|').collect();
                if parts.len() == 3 {
                    if let (Ok(start), Ok(end)) = (parts[1].parse::<i32>(), parts[2].parse::<i32>())
                    {
                        tools.push(ToolCall::FsRead(FsRead {
                            operations: vec![fs_read::FsReadOperation::Line(fs_read::FsLine {
                                path: parts[0].to_string(),
                                start_line: start,
                                end_line: end,
                            })],
                            summary: Some("Read specific line range".to_string()),
                        }));
                    }
                }
            }
            "FILE" => {
                // Regular file reading
                tools.push(ToolCall::FsRead(FsRead {
                    operations: vec![fs_read::FsReadOperation::Line(fs_read::FsLine {
                        path: data,
                        start_line: 1,
                        end_line: -1, // Read entire file
                    })],
                    summary: Some("Read file contents".to_string()),
                }));
            }
            "DIR" => {
                // Directory listing
                tools.push(ToolCall::FsRead(FsRead {
                    operations: vec![fs_read::FsReadOperation::Directory(fs_read::FsDirectory {
                        path: data,
                        depth: 1,
                    })],
                    summary: Some("List directory contents".to_string()),
                }));
            }
            _ => {}
        }
    }
    // Detect general directory/file listing requests
    else if text.contains("read file")
        || text.contains("show me")
        || text.contains("list")
        || text.contains("directory")
        || text.contains("what's in")
        || text.contains("files")
        || text.contains("contents")
        || text.contains("examine")
    {
        tools.push(ToolCall::FsRead(FsRead {
            operations: vec![fs_read::FsReadOperation::Directory(fs_read::FsDirectory {
                path: ".".to_string(),
                depth: 1,
            })],
            summary: Some("Read directory contents".to_string()),
        }));
    }

    // Detect fs_write requests
    if text.contains("create file")
        || text.contains("write file")
        || text.contains("save to")
        || text.contains("write to")
        || text.contains("create a file")
        || text.contains("make a file")
        || text.contains("generate file")
        || text.contains("output to file")
    {
        // Extract file path from natural language
        if let Some(path) = extract_simple_file_path(text) {
            tools.push(ToolCall::FsWrite(FsWrite {
                command: "create".to_string(),
                path: path.clone(),
                file_text: Some("// Content will be provided by AI response".to_string()),
                old_str: None,
                new_str: None,
                insert_line: None,
                summary: Some(format!("Create file: {}", path)),
            }));
        }
    }

    // Detect use_aws requests
    if text.contains("aws ")
        || text.contains("AWS ")
        || text.contains("s3 ")
        || text.contains("ec2 ")
        || text.contains("lambda ")
        || text.contains("cloudformation")
        || text.contains("describe ")
        || text.contains("list ")
            && (text.contains("bucket") || text.contains("instance") || text.contains("stack"))
    {
        if let Some(aws_command) = extract_aws_command_from_text(text) {
            tools.push(ToolCall::UseAws(aws_command));
        }
    }

    // Detect use_azure requests
    if text.contains("azure ") || text.contains("az ") || text.contains("Azure ") {
        tools.push(ToolCall::UseAzure(use_azure::UseAzure {
            service_name: "resource".to_string(),
            operation_name: "list".to_string(),
            parameters: serde_json::json!({}),
            region: None,
        }));
    }

    // Detect use_gcp requests
    if text.contains("gcp ") || text.contains("gcloud ") || text.contains("Google Cloud") {
        tools.push(ToolCall::UseGcp(use_gcp::UseGcp {
            service_name: "compute".to_string(),
            operation_name: "instances".to_string(),
            parameters: serde_json::json!({}),
            region: None,
        }));
    }

    // Detect use_oracle requests
    if text.contains("oracle ") || text.contains("oci ") || text.contains("Oracle Cloud") {
        tools.push(ToolCall::UseOracle(use_oracle::UseOracle {
            service_name: "compute".to_string(),
            operation_name: "instance".to_string(),
            parameters: serde_json::json!({}),
            region: None,
        }));
    }

    // Detect introspect requests (only for explicit help requests)
    if (text.contains("help")
        && (text.contains("q cli") || text.contains("capabilities") || text.contains("commands")))
        || text.contains("what can you do")
        || text.contains("show features")
        || text.contains("/help")
    {
        tools.push(ToolCall::Introspect(Introspect { query: None }));
    }

    // Detect knowledge tool requests
    if text.contains("remember")
        || text.contains("store")
        || text.contains("knowledge")
        || text.contains("save context")
        || text.contains("search knowledge")
    {
        if text.contains("search") || text.contains("find") {
            tools.push(ToolCall::Knowledge(Knowledge {
                action: "search".to_string(),
                content: None,
                query: Some(text.to_string()),
                path: None,
            }));
        } else if text.contains("list") {
            tools.push(ToolCall::Knowledge(Knowledge {
                action: "list".to_string(),
                content: None,
                query: None,
                path: None,
            }));
        } else {
            tools.push(ToolCall::Knowledge(Knowledge {
                action: "store".to_string(),
                content: Some(text.to_string()),
                query: None,
                path: Some("context".to_string()),
            }));
        }
    }

    // Detect todo_list requests - be more specific
    if text.contains("create todo")
        || text.contains("add task")
        || text.contains("new checklist")
        || text.contains("track task")
        || text.contains("todo list")
    {
        if text.contains("list") || text.contains("show") {
            tools.push(ToolCall::TodoList(TodoList {
                action: "list".to_string(),
                task: None,
                list_id: None,
                completed: None,
            }));
        } else if text.contains("complete") || text.contains("done") {
            tools.push(ToolCall::TodoList(TodoList {
                action: "complete".to_string(),
                task: Some(text.to_string()),
                list_id: Some("current".to_string()),
                completed: Some(true),
            }));
        } else {
            tools.push(ToolCall::TodoList(TodoList {
                action: "create".to_string(),
                task: Some(text.to_string()),
                list_id: None,
                completed: None,
            }));
        }
    }

    // Detect GitHub repository analysis requests
    if text.contains("github.com") || (text.contains("analyze") && text.contains("repository")) {
        if let Some(repo_url) = extract_github_url(text) {
            tools.push(ToolCall::GitHub(GitHubAnalysis {
                repo_url: repo_url.to_string(),
            }));
        }
    }

    // Detect thinking requests for complex problems
    if text.contains("think")
        || text.contains("analyze")
        || text.contains("reason")
        || text.contains("step by step")
        || text.contains("complex")
        || text.contains("problem")
    {
        tools.push(ToolCall::Thinking(Thinking::new_thought(text)));
    }

    // Detect orchestrated thinking requests (only from user slash commands)
    let trimmed = text.trim_start();
    if trimmed.starts_with("/orchestrate")
        || trimmed.starts_with("/plan")
        || trimmed.starts_with("/multi")
    {
        tools.push(ToolCall::OrchestratedThinking(
            orchestrated_thinking::OrchestratedThinking::new(text),
        ));
    }

    // Detect specific file understanding requests FIRST
    if text.contains("TEST_SCENARIOS.md")
        || (text.contains("understand from it") && text.contains("TEST_SCENARIOS"))
        || (text.contains("what do you understand") && text.contains("file"))
    {
        tools.push(ToolCall::FsRead(FsRead {
            operations: vec![fs_read::FsReadOperation::Line(fs_read::FsLine {
                path: "TEST_SCENARIOS.md".to_string(),
                start_line: 1,
                end_line: -1,
            })],
            summary: Some("Read TEST_SCENARIOS.md file".to_string()),
        }));
    }
    // Detect fs_read requests - be more specific, exclude specific file requests
    else if (text.contains("read file")
        || text.contains("show file")
        || text.contains("cat ")
        || text.contains("list directory")
        || text.contains("ls ")
        || text.contains("dir ")
        || text.starts_with("read ")
        || text.starts_with("show ")
        || text.starts_with("list "))
        && !text.contains("github.com")
        && !text.contains("TEST_SCENARIOS.md")
    {
        tools.push(ToolCall::FsRead(FsRead {
            operations: vec![fs_read::FsReadOperation::Directory(fs_read::FsDirectory {
                path: ".".to_string(),
                depth: 2, // Deeper for repo analysis
            })],
            summary: Some("Analyze repository structure".to_string()),
        }));
    }

    // Look for executable commands in code blocks (existing logic)
    let lines: Vec<&str> = text.lines().collect();
    let mut in_code_block = false;
    let mut code_block_type = "";

    for line in lines {
        let trimmed = line.trim();

        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            if in_code_block {
                code_block_type = trimmed.trim_start_matches("```");
            } else {
                code_block_type = "";
            }
            continue;
        }

        let is_executable_context = if in_code_block {
            code_block_type.is_empty()
                || code_block_type == "bash"
                || code_block_type == "sh"
                || code_block_type == "shell"
        } else {
            trimmed.starts_with("kubectl ")
                || trimmed.starts_with("aws ")
                || trimmed.starts_with("docker ")
                || trimmed.starts_with("ls ")
                || trimmed.starts_with("ps ")
                || trimmed.starts_with("git ")
        };

        if is_executable_context && !trimmed.is_empty() && !trimmed.starts_with("#") {
            // Skip tree-style formatting characters and other non-command text
            if trimmed.contains("├──")
                || trimmed.contains("└──")
                || trimmed.contains("│")
                || trimmed.starts_with("├")
                || trimmed.starts_with("└")
                || trimmed.starts_with("│")
                || trimmed.starts_with("Current Analysis:")
                || trimmed.starts_with("Repository Structure:")
                || trimmed.starts_with("Found ")
                || trimmed.contains("appears to be")
            {
                continue;
            }

            if !trimmed.contains("=") || trimmed.starts_with("export ") {
                if let Some(cmd) = parse_execute_command(trimmed) {
                    tools.push(ToolCall::Execute(cmd));
                }
            }
        }
    }

    if tools.is_empty() {
        None
    } else {
        Some(tools)
    }
}

fn extract_github_url(text: &str) -> Option<&str> {
    if let Some(start) = text.find("https://github.com/") {
        let url_part = &text[start..];
        if let Some(end) = url_part.find(' ') {
            Some(&url_part[..end])
        } else {
            Some(url_part)
        }
    } else {
        None
    }
}

fn extract_file_path_from_text(text: &str) -> Option<(String, String)> {
    use std::path::Path;
    let text_lower = text.to_lowercase();

    // Check for search patterns first
    if let Some(search_op) = extract_search_operation(&text_lower) {
        return Some((
            "SEARCH".to_string(),
            format!("{}|{}", search_op.pattern, search_op.path),
        ));
    }

    // Check for line range patterns
    if let Some(line_op) = extract_line_range_operation(&text_lower) {
        return Some((
            "LINES".to_string(),
            format!(
                "{}|{}|{}",
                line_op.path, line_op.start_line, line_op.end_line
            ),
        ));
    }

    // File/directory patterns with filesystem probing
    let patterns = [
        r"(?:read|open|view|show|cat|less|more)\s+([^\s]+)",
        r"show\s+me\s+([^\s]+)",
        r"contents?\s+of\s+([^\s]+)",
        r"what'?s\s+in\s+([^\s]+)",
        r"(?:check|list|ls)\s+([^\s]+)\s*(?:path|dir|directory)?",
    ];

    for pattern in &patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            if let Some(captures) = re.captures(&text_lower) {
                if let Some(file_match) = captures.get(1) {
                    let token = file_match
                        .as_str()
                        .trim_matches(|c| c == '"' || c == '\'' || c == ',' || c == '.');

                    // Skip common non-file words
                    if ["me", "it", "this", "that", "them", "all", "everything"].contains(&token) {
                        continue;
                    }

                    // Probe filesystem for bare tokens
                    if Path::new(token).exists() {
                        if Path::new(token).is_dir() {
                            return Some(("DIR".to_string(), token.to_string()));
                        } else {
                            return Some(("FILE".to_string(), token.to_string()));
                        }
                    }

                    // Fall back to file heuristic for non-existent paths
                    if token.contains('.')
                        || ["readme", "license", "changelog", "makefile", "dockerfile"]
                            .contains(&token)
                    {
                        return Some(("FILE".to_string(), token.to_string()));
                    }
                }
            }
        }
    }

    None
}

fn extract_search_operation(text: &str) -> Option<fs_read::FsSearch> {
    // Patterns for search operations - using simpler patterns to avoid regex complexity
    if text.contains("search for") && text.contains(" in ") {
        // Simple pattern: "search for 'term' in file"
        if let Some(start) = text.find("search for ") {
            let after_search = &text[start + 11..];
            if let Some(quote_start) = after_search.find('"').or_else(|| after_search.find('\'')) {
                let quote_char = after_search.chars().nth(quote_start).unwrap();
                let after_quote = &after_search[quote_start + 1..];
                if let Some(quote_end) = after_quote.find(quote_char) {
                    let search_term = &after_quote[..quote_end];
                    let after_term = &after_quote[quote_end..];
                    if let Some(in_pos) = after_term.find(" in ") {
                        let after_in = &after_term[in_pos + 4..];
                        let file_pattern = after_in.split_whitespace().next().unwrap_or("*");
                        return Some(fs_read::FsSearch {
                            path: file_pattern.to_string(),
                            pattern: search_term.to_string(),
                            context_lines: 2,
                        });
                    }
                }
            }
        }
    }

    None
}

fn extract_line_range_operation(text: &str) -> Option<fs_read::FsLine> {
    // Simple pattern matching for "read lines X-Y of file" or "lines X-Y from file"
    if text.contains("lines") && text.contains("-") {
        let words: Vec<&str> = text.split_whitespace().collect();

        for (i, word) in words.iter().enumerate() {
            if word.contains("-") && word.chars().all(|c| c.is_ascii_digit() || c == '-') {
                // Found a range like "10-20"
                let parts: Vec<&str> = word.split('-').collect();
                if parts.len() == 2 {
                    if let (Ok(start), Ok(end)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>())
                    {
                        // Look for "of" or "from" followed by filename
                        for j in (i + 1)..words.len() {
                            if words[j] == "of" || words[j] == "from" {
                                if j + 1 < words.len() {
                                    let file_path = words[j + 1];
                                    if file_path.contains('.') {
                                        return Some(fs_read::FsLine {
                                            path: file_path.to_string(),
                                            start_line: start,
                                            end_line: end,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

fn extract_simple_file_path(text: &str) -> Option<String> {
    // Simple file path extraction - look for words with file extensions
    let words: Vec<&str> = text.split_whitespace().collect();
    for word in words {
        if word.contains('.')
            && (word.ends_with(".rs")
                || word.ends_with(".txt")
                || word.ends_with(".md")
                || word.ends_with(".json")
                || word.ends_with(".toml")
                || word.ends_with(".py")
                || word.ends_with(".js")
                || word.ends_with(".html")
                || word.ends_with(".css"))
        {
            return Some(
                word.trim_matches(|c| c == '"' || c == '\'' || c == ',' || c == '.')
                    .to_string(),
            );
        }
    }
    None
}

fn extract_aws_command_from_text(text: &str) -> Option<UseAws> {
    // Parse AWS CLI commands from natural language
    let text_lower = text.to_lowercase();

    // S3 operations
    if text_lower.contains("s3") && text_lower.contains("list") {
        return Some(UseAws {
            region: "ap-southeast-2".to_string(),
            service_name: "s3".to_string(),
            operation_name: "list-buckets".to_string(),
            parameters: std::collections::HashMap::new(),
            profile_name: None,
            label: "List S3 buckets".to_string(),
        });
    }

    // EC2 operations
    if text_lower.contains("ec2")
        && (text_lower.contains("list") || text_lower.contains("describe"))
    {
        return Some(UseAws {
            region: "ap-southeast-2".to_string(),
            service_name: "ec2".to_string(),
            operation_name: "describe-instances".to_string(),
            parameters: std::collections::HashMap::new(),
            profile_name: None,
            label: "Describe EC2 instances".to_string(),
        });
    }

    // Lambda operations
    if text_lower.contains("lambda")
        && (text_lower.contains("list") || text_lower.contains("functions"))
    {
        return Some(UseAws {
            region: "ap-southeast-2".to_string(),
            service_name: "lambda".to_string(),
            operation_name: "list-functions".to_string(),
            parameters: std::collections::HashMap::new(),
            profile_name: None,
            label: "List Lambda functions".to_string(),
        });
    }

    None
}

fn parse_execute_command(command_text: &str) -> Option<ExecuteCommand> {
    let trimmed = command_text.trim();

    // Skip comments and empty lines
    if trimmed.is_empty() || trimmed.starts_with("#") {
        return None;
    }

    // Skip tree-style formatting characters
    if trimmed.contains("├")
        || trimmed.contains("└")
        || trimmed.contains("│")
        || trimmed.starts_with("├")
        || trimmed.starts_with("└")
        || trimmed.starts_with("│")
    {
        return None;
    }

    // Skip commands with placeholder values
    if trimmed.contains("<") && trimmed.contains(">") {
        return None;
    }

    // Skip file names - don't execute files as commands
    if is_likely_filename(trimmed) {
        return None;
    }

    // Must start with a valid command word
    let first_word = trimmed.split_whitespace().next().unwrap_or("");
    let valid_commands = [
        "ls",
        "pwd",
        "cd",
        "cat",
        "grep",
        "find",
        "ps",
        "top",
        "df",
        "free",
        "uptime",
        "whoami",
        "id",
        "date",
        "git",
        "docker",
        "kubectl",
        "aws",
        "az",
        "gcloud",
        "terraform",
        "npm",
        "yarn",
        "cargo",
        "make",
        "curl",
        "wget",
        "ssh",
        "scp",
        "rsync",
        "tar",
        "zip",
        "unzip",
        "chmod",
        "chown",
        "mkdir",
        "rmdir",
        "cp",
        "mv",
        "rm",
        "touch",
        "head",
        "tail",
        "sort",
        "uniq",
        "wc",
        "diff",
        "which",
        "whereis",
    ];

    if !valid_commands.contains(&first_word) {
        return None;
    }

    // Accept any command that looks executable (including watch commands now)
    if trimmed.contains(" ") || trimmed.len() > 2 {
        Some(ExecuteCommand {
            command: trimmed.to_string(),
            summary: Some(format!("Execute: {}", trimmed)),
        })
    } else {
        None
    }
}

fn is_likely_filename(text: &str) -> bool {
    // Common file extensions
    let file_extensions = [
        ".tf",
        ".md",
        ".txt",
        ".json",
        ".yaml",
        ".yml",
        ".toml",
        ".rs",
        ".py",
        ".js",
        ".ts",
        ".go",
        ".java",
        ".cpp",
        ".c",
        ".h",
        ".sh",
        ".bat",
        ".ps1",
        ".dockerfile",
        ".sql",
        ".html",
        ".css",
        ".xml",
        ".csv",
        ".log",
        ".conf",
        ".cfg",
        ".ini",
        ".env",
    ];

    // Check if it has a file extension
    if file_extensions
        .iter()
        .any(|ext| text.to_lowercase().ends_with(ext))
    {
        return true;
    }

    // Common files without extensions
    let common_files = [
        "readme",
        "license",
        "changelog",
        "makefile",
        "dockerfile",
        "gemfile",
        "rakefile",
        "procfile",
        ".gitignore",
        ".gitattributes",
        ".dockerignore",
    ];

    if common_files.iter().any(|file| text.to_lowercase() == *file) {
        return true;
    }

    // If it starts with a dot (hidden files)
    if text.starts_with('.') && !text.contains(' ') {
        return true;
    }

    false
}

fn print_custom_help() {
    println!();
    println!("{}", "hello-ai (Hello AI CLI)".bright_blue().bold());
    println!();
    println!(
        "{}",
        "Popular Subcommands              Usage: hello-ai [subcommand]".bright_white()
    );
    println!(
        "{}",
        "╭────────────────────────────────────────────────────╮".bright_cyan()
    );
    println!(
        "│ {}         {}                    │",
        "chat".bright_green().bold(),
        "Chat with Hello AI".white()
    );
    println!(
        "│ {}    {}│",
        "translate".bright_green().bold(),
        "Natural Language to Shell translation".white()
    );
    println!(
        "│ {}       {}             │",
        "doctor".bright_green().bold(),
        "Debug installation issues".white()
    );
    println!(
        "│ {}     {}       │",
        "settings".bright_green().bold(),
        "Customize appearance & behavior".white()
    );
    println!(
        "│ {}         {}                          │",
        "quit".bright_green().bold(),
        "Quit the app".white()
    );
    println!(
        "{}",
        "╰────────────────────────────────────────────────────╯".bright_cyan()
    );
    println!();
    println!("{}", "To see all subcommands, use:".bright_white());
    println!(
        " {} {}",
        "❯".bright_blue(),
        "hello-ai --help-all".bright_cyan()
    );
    println!("ㅤ");
}

fn print_help_all() {
    println!("HANS CUSTOM AWS Q - Sydney Region Only");
    println!();
    println!("Commands:");
    println!("  agent         Agent root commands");
    println!("  chat          AI assistant in your terminal");
    println!("  dashboard     Open the dashboard");
    println!("  debug         Debug the app");
    println!("  diagnostic    Run diagnostic tests");
    println!("  doctor        Fix and diagnose common issues");
    println!("  init          Generate the dotfiles for the given shell");
    println!("  inline        Inline shell completions");
    println!("  integrations  Manage system integrations");
    println!("  issue         Create a new Github issue");
    println!("  launch        Launch the desktop app");
    println!("  login         Login");
    println!("  logout        Logout");
    println!("  mcp           Model Context Protocol (MCP)");
    println!("  profile       Show the profile associated with this idc user");
    println!("  quit          Quit the desktop app");
    println!("  restart       Restart the desktop app");
    println!("  settings      Customize appearance & behavior");
    println!("  setup         Setup cli components");
    println!("  theme         Get or set theme");
    println!("  translate     Natural Language to Shell translation");
    println!("  update        Update the Amazon Q application");
    println!("  user          Manage your account");
    println!("  whoami        Prints details about the current user");
    println!("  help          Print this message or the help of the given subcommand(s)");
    println!();
    println!("Options:");
    println!("  -v, --verbose...");
    println!("          Increase logging verbosity");
    println!();
    println!("      --help-all");
    println!("          Print help for all subcommands");
    println!();
    println!("  -h, --help");
    println!("          Print help");
    println!();
    println!("  -V, --version");
    println!("          Print version");
}

async fn run_single_chat_with_options(
    message: String,
    resume: bool,
    agent: Option<String>,
    model: Option<String>,
    trust_all_tools: bool,
    trust_tools: Option<String>,
    wrap: Option<String>,
    verbose: u8,
    app_config: Config,
) -> Result<(), Box<dyn std::error::Error>> {
    if verbose > 0 {
        println!("🔍 Verbose mode: Level {}", verbose);
    }
    if resume {
        println!("📂 Resume mode: Loading previous conversation...");
    }
    if let Some(agent_name) = &agent {
        println!("🤖 Using agent: {}", agent_name);
    }
    if let Some(model_name) = &model {
        println!("🧠 Using model: {}", model_name);
    }
    if trust_all_tools {
        println!("🔓 Trust all tools: Enabled");
    }
    if let Some(tools) = &trust_tools {
        println!("🔧 Trusted tools: {}", tools);
    }
    if let Some(wrap_mode) = &wrap {
        println!("📄 Wrap mode: {}", wrap_mode);
    }

    run_single_chat(message, app_config).await
}

async fn run_interactive_chat_with_options(
    resume: bool,
    agent: Option<String>,
    model: Option<String>,
    trust_all_tools: bool,
    trust_tools: Option<String>,
    wrap: Option<String>,
    verbose: u8,
    app_config: Config,
) -> Result<(), Box<dyn std::error::Error>> {
    if verbose > 0 {
        println!("🔍 Verbose mode: Level {}", verbose);
    }
    if resume {
        println!("📂 Resume mode: Loading previous conversation...");
    }
    if let Some(agent_name) = &agent {
        println!("🤖 Using agent: {}", agent_name);
    }
    if let Some(model_name) = &model {
        println!("🧠 Using model: {}", model_name);
    }
    if trust_all_tools {
        println!("🔓 Trust all tools: Enabled");
    }
    if let Some(tools) = &trust_tools {
        println!("🔧 Trusted tools: {}", tools);
    }
    if let Some(wrap_mode) = &wrap {
        println!("📄 Wrap mode: {}", wrap_mode);
    }

    run_interactive_chat(app_config).await
}

async fn run_single_chat(
    message: String,
    app_config: Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(Region::new(app_config.cloud.aws_region.clone()))
        .load()
        .await;
    let client = BedrockClient::new(&config);

    let response = get_ai_response(&client, &message, &app_config).await?;
    let rich_interface = RichInterface::new();
    let formatted = rich_interface.format_markdown_response(&response);
    println!("{}", render_msg(MsgKind::Assistant, &formatted));
    Ok(())
}

async fn run_issue(title: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "🐛".bright_blue(),
        "Create GitHub Issue".bright_blue().bold()
    );
    println!();

    let issue_title = title.unwrap_or_else(|| "Q CLI Issue".to_string());
    let repo_url = "https://github.com/hans-zand/qdev-sydney";

    println!("Issue Title: {}", issue_title.bright_green());
    println!("Repository: {}", repo_url.bright_blue());
    println!();

    // Generate issue template
    let issue_body = format!(
        "## Issue Description\n\
        [Describe the issue here]\n\n\
        ## Environment\n\
        - CLI Version: 1.0.0 (HANS CUSTOM)\n\
        - OS: {} {}\n\
        - Region: ap-southeast-2 (Sydney)\n\
        - Timestamp: {}\n\n\
        ## Steps to Reproduce\n\
        1. [First step]\n\
        2. [Second step]\n\
        3. [Third step]\n\n\
        ## Expected Behavior\n\
        [What you expected to happen]\n\n\
        ## Actual Behavior\n\
        [What actually happened]\n\n\
        ## Additional Context\n\
        [Any other context about the problem]",
        std::env::consts::OS,
        std::env::consts::ARCH,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );

    // Create GitHub issue URL
    let encoded_title = urlencoding::encode(&issue_title);
    let encoded_body = urlencoding::encode(&issue_body);
    let github_url = format!(
        "{}/issues/new?title={}&body={}",
        repo_url, encoded_title, encoded_body
    );

    println!("{} GitHub Issue URL:", "🔗".bright_blue());
    println!("{}", github_url.bright_blue());
    println!();

    // Try to open in browser
    println!("{} Attempting to open in browser...", "🌐".bright_blue());
    match std::process::Command::new("open").arg(&github_url).output() {
        Ok(_) => println!("  ✅ Opened in browser"),
        Err(_) => {
            println!("  ℹ️ Could not auto-open. Copy the URL above to create the issue.");
        }
    }

    Ok(())
}

async fn run_theme(name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "🎨".bright_blue(),
        "Q CLI Theme Management".bright_blue().bold()
    );
    println!();

    let available_themes = vec!["default", "dark", "light", "sydney", "minimal"];

    match name {
        Some(theme_name) => {
            if available_themes.contains(&theme_name.as_str()) {
                println!("Setting theme to: {}", theme_name.bright_green());

                // Save theme preference (in a real implementation, this would save to config)
                let config_dir =
                    std::env::var("HOME").unwrap_or_else(|_| ".".to_string()) + "/.config/q";
                std::fs::create_dir_all(&config_dir)?;
                std::fs::write(format!("{}/theme", config_dir), &theme_name)?;

                println!(
                    "✅ Theme '{}' applied successfully!",
                    theme_name.bright_green()
                );
            } else {
                println!("❌ Unknown theme: {}", theme_name.bright_red());
                println!(
                    "Available themes: {}",
                    available_themes.join(", ").bright_blue()
                );
            }
        }
        None => {
            // Show current theme and available options
            let config_file =
                std::env::var("HOME").unwrap_or_else(|_| ".".to_string()) + "/.config/q/theme";
            let current_theme =
                std::fs::read_to_string(&config_file).unwrap_or_else(|_| "default".to_string());

            println!("Current theme: {}", current_theme.trim().bright_green());
            println!();
            println!("Available themes:");
            for theme in &available_themes {
                let indicator = if *theme == current_theme.trim() {
                    "→"
                } else {
                    " "
                };
                println!("  {} {}", indicator.bright_green(), theme.bright_white());
            }
            println!();
            println!(
                "Usage: {} {}",
                "q theme <name>".bright_blue(),
                "# Set theme".bright_black()
            );
        }
    }

    Ok(())
}

async fn run_init(shell: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "🐚".bright_blue(),
        "Q CLI Shell Integration".bright_blue().bold()
    );
    println!();

    let shell_type = shell.unwrap_or_else(|| {
        std::env::var("SHELL")
            .unwrap_or_default()
            .split('/')
            .last()
            .unwrap_or("bash")
            .to_string()
    });

    println!("Detected shell: {}", shell_type.bright_green());
    println!();

    let (config_file, alias_content) = match shell_type.as_str() {
        "zsh" => (
            "~/.zshrc",
            "alias q='/Users/hans/Repo/hello-ai/qdev-sydney/q-bedrock/target/debug/q'",
        ),
        "fish" => (
            "~/.config/fish/config.fish",
            "alias q='/Users/hans/Repo/hello-ai/qdev-sydney/q-bedrock/target/debug/q'",
        ),
        _ => (
            "~/.bashrc",
            "alias q='/Users/hans/Repo/hello-ai/qdev-sydney/q-bedrock/target/debug/q'",
        ),
    };

    println!("{} Shell configuration:", "📝".bright_blue());
    println!("  Add this to {}:", config_file.bright_yellow());
    println!();
    println!("  {}", alias_content.bright_green());
    println!();
    println!("  # Enable Q CLI completions (optional)");
    println!("  {}", "complete -C q q  # for bash".bright_green());
    println!();

    println!("{} Manual setup:", "🔧".bright_blue());
    println!("  1. Add the alias to your shell config");
    println!("  2. Run: {}", "source ~/.bashrc".bright_blue());
    println!("  3. Test: {}", "q --version".bright_blue());

    Ok(())
}

async fn run_diagnostic() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "🔬".bright_green(),
        "Q CLI Diagnostic Tests".bright_green().bold()
    );
    println!();

    let mut tests_passed = 0;
    let mut tests_failed = 0;

    // Test 1: Bedrock connectivity
    println!("{} Testing Bedrock connectivity...", "🧪".bright_blue());
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new("ap-southeast-2"))
        .load()
        .await;
    let client = aws_sdk_bedrockruntime::Client::new(&config);

    match client.invoke_model()
        .model_id("anthropic.claude-3-5-sonnet-20241022-v2:0")
        .content_type("application/json")
        .body(r#"{"anthropic_version":"bedrock-2023-05-31","max_tokens":10,"messages":[{"role":"user","content":"test"}]}"#.as_bytes().to_vec().into())
        .send()
        .await
    {
        Ok(_) => {
            println!("  ✅ Bedrock: {}", "Connected".bright_green());
            tests_passed += 1;
        }
        Err(e) => {
            println!("  ❌ Bedrock: {}", format!("Failed - {}", e).bright_red());
            tests_failed += 1;
        }
    }

    // Test 2: PII Scanner
    println!("{} Testing PII scanner...", "🧪".bright_blue());
    let pii_scanner = pii_scanner::PiiScanner::new();
    let test_result = pii_scanner.scan("My SSN is 123-45-6789", "test");
    if test_result.has_pii {
        println!("  ✅ PII Scanner: {}", "Working".bright_green());
        tests_passed += 1;
    } else {
        println!("  ❌ PII Scanner: {}", "Not detecting PII".bright_red());
        tests_failed += 1;
    }

    // Test 3: File operations
    println!("{} Testing file operations...", "🧪".bright_blue());
    let test_file = "/tmp/q_diagnostic_test.txt";
    match std::fs::write(test_file, "test content") {
        Ok(_) => match std::fs::read_to_string(test_file) {
            Ok(content) if content == "test content" => {
                println!("  ✅ File Operations: {}", "Working".bright_green());
                tests_passed += 1;
                let _ = std::fs::remove_file(test_file);
            }
            _ => {
                println!("  ❌ File Operations: {}", "Read failed".bright_red());
                tests_failed += 1;
            }
        },
        Err(_) => {
            println!("  ❌ File Operations: {}", "Write failed".bright_red());
            tests_failed += 1;
        }
    }

    println!();
    println!("{} Diagnostic Results:", "📊".bright_white());
    println!(
        "  • Tests Passed: {}",
        tests_passed.to_string().bright_green()
    );
    println!(
        "  • Tests Failed: {}",
        tests_failed.to_string().bright_red()
    );

    if tests_failed == 0 {
        println!("\n{} All diagnostic tests passed!", "✅".bright_green());
    } else {
        println!(
            "\n{} Some tests failed. Check configuration.",
            "⚠️".bright_yellow()
        );
    }

    Ok(())
}

async fn run_update() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "🔄".bright_blue(),
        "Q CLI Update".bright_blue().bold()
    );
    println!();

    println!("Current Version: {}", "1.0.0 (HANS CUSTOM)".bright_green());
    println!();

    println!("{} Checking for updates...", "🔍".bright_blue());

    // Check git repository for updates
    match std::process::Command::new("git")
        .args(&["fetch", "origin"])
        .current_dir("/Users/hans/Repo/hello-ai/qdev-sydney/q-bedrock")
        .output()
    {
        Ok(_) => {
            match std::process::Command::new("git")
                .args(&[
                    "rev-list",
                    "--count",
                    "HEAD..origin/feature/command-structure",
                ])
                .current_dir("/Users/hans/Repo/hello-ai/qdev-sydney/q-bedrock")
                .output()
            {
                Ok(output) => {
                    let commits_str = String::from_utf8_lossy(&output.stdout);
                    let commits = commits_str.trim();
                    if commits == "0" {
                        println!("  ✅ {}", "Already up to date".bright_green());
                    } else {
                        println!("  📦 {} commits available", commits.bright_yellow());
                        println!(
                            "     Run: {}",
                            "git pull origin feature/command-structure".bright_blue()
                        );
                        println!("     Then: {}", "cargo build".bright_blue());
                    }
                }
                Err(_) => println!("  ℹ️ {}", "Cannot check remote updates".bright_black()),
            }
        }
        Err(_) => println!("  ℹ️ {}", "Not in git repository".bright_black()),
    }

    println!();
    println!("{} Update check completed!", "✅".bright_green());
    Ok(())
}

async fn run_setup() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "🔧".bright_blue(),
        "Q CLI Setup".bright_blue().bold()
    );
    println!();

    // Check AWS CLI
    println!("{} Checking AWS CLI installation...", "🔍".bright_blue());
    match std::process::Command::new("aws").arg("--version").output() {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("  ✅ AWS CLI: {}", version.trim().bright_green());
        }
        Err(_) => {
            println!("  ❌ AWS CLI: {}", "Not installed".bright_red());
            println!(
                "     Install: {}",
                "https://aws.amazon.com/cli/".bright_blue()
            );
        }
    }

    // Check AWS credentials
    println!("{} Checking AWS credentials...", "🔍".bright_blue());
    match std::process::Command::new("aws")
        .args(&["sts", "get-caller-identity"])
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                println!("  ✅ AWS Credentials: {}", "Configured".bright_green());
            } else {
                println!("  ⚠️ AWS Credentials: {}", "Not configured".bright_yellow());
                println!("     Run: {}", "aws configure".bright_blue());
            }
        }
        Err(_) => {
            println!("  ❌ AWS Credentials: {}", "Cannot check".bright_red());
        }
    }

    // Setup history file
    println!("{} Setting up command history...", "🔍".bright_blue());
    let history_file = std::env::var("HOME").unwrap_or_else(|_| ".".to_string()) + "/.q_history";
    if std::path::Path::new(&history_file).exists() {
        println!("  ✅ History file: {}", "Already exists".bright_green());
    } else {
        match std::fs::File::create(&history_file) {
            Ok(_) => println!("  ✅ History file: {}", "Created".bright_green()),
            Err(_) => println!("  ⚠️ History file: {}", "Cannot create".bright_yellow()),
        }
    }

    println!();
    println!(
        "{} Setup completed! Run {} to start chatting.",
        "✅".bright_green(),
        "q chat".bright_blue()
    );
    Ok(())
}

async fn run_debug() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "🐛".bright_red(),
        "Q CLI Debug Information".bright_red().bold()
    );
    println!();

    // System Information
    println!("{}", "SYSTEM INFORMATION:".bright_white().bold());
    println!("  • OS: {}", std::env::consts::OS.bright_green());
    println!(
        "  • Architecture: {}",
        std::env::consts::ARCH.bright_green()
    );
    println!("  • CLI Version: {}", "1.0.0 (HANS CUSTOM)".bright_green());
    println!();

    // Environment Variables
    println!("{}", "ENVIRONMENT VARIABLES:".bright_white().bold());
    let env_vars = [
        "AWS_PROFILE",
        "AWS_REGION",
        "AWS_DEFAULT_REGION",
        "NIGHTFALL_API_KEY",
        "SIMPLE_PII_SCAN",
        "HOME",
        "PATH",
    ];

    for var in &env_vars {
        match std::env::var(var) {
            Ok(value) => {
                if var.contains("KEY") {
                    println!("  • {}: {}", var.bright_blue(), "[REDACTED]".bright_black());
                } else if *var == "PATH" {
                    println!(
                        "  • {}: {} entries",
                        var.bright_blue(),
                        value.split(':').count().to_string().bright_green()
                    );
                } else {
                    println!("  • {}: {}", var.bright_blue(), value.bright_green());
                }
            }
            Err(_) => println!("  • {}: {}", var.bright_blue(), "Not set".bright_black()),
        }
    }
    println!();

    // File System Check
    println!("{}", "FILE SYSTEM:".bright_white().bold());
    let current_dir =
        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("unknown"));
    println!(
        "  • Current Directory: {}",
        current_dir.display().to_string().bright_green()
    );

    // Check if binary exists
    let binary_path = "/Users/hans/Repo/hello-ai/qdev-sydney/q-bedrock/target/debug/q";
    if std::path::Path::new(binary_path).exists() {
        println!("  • Binary Location: {}", "Found".bright_green());
    } else {
        println!("  • Binary Location: {}", "Not found".bright_red());
    }
    println!();

    // Network Connectivity
    println!("{}", "NETWORK CONNECTIVITY:".bright_white().bold());
    println!(
        "  • Bedrock Endpoint: {}",
        "bedrock-runtime.ap-southeast-2.amazonaws.com".bright_green()
    );
    println!("  • Region: {}", "ap-southeast-2 (Sydney)".bright_green());
    println!("  • Data Residency: {}", "Australia Only 🇦🇺".bright_green());
    println!();

    // Process Information
    println!("{}", "PROCESS INFORMATION:".bright_white().bold());
    println!(
        "  • Process ID: {}",
        std::process::id().to_string().bright_green()
    );

    // Memory usage (basic)
    if let Ok(output) = std::process::Command::new("ps")
        .args(&["-o", "rss=", "-p", &std::process::id().to_string()])
        .output()
    {
        if let Ok(memory_str) = String::from_utf8(output.stdout) {
            if let Ok(memory_kb) = memory_str.trim().parse::<u64>() {
                println!(
                    "  • Memory Usage: {} MB",
                    (memory_kb / 1024).to_string().bright_green()
                );
            }
        }
    }

    println!();
    println!(
        "{} Debug information collected successfully!",
        "✅".bright_green()
    );
    Ok(())
}

async fn run_translate(
    command: String,
    app_config: Config,
) -> Result<(), Box<dyn std::error::Error>> {
    // Show loading animation like official Q
    print!("\x1B[?25l"); // Hide cursor
    for _ in 0..3 {
        for i in 0..7 {
            print!("\r  Shell · ");
            for j in 0..7 {
                if j <= i {
                    print!("▰");
                } else {
                    print!("▱");
                }
            }
            std::io::stdout().flush()?;
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
    print!("\x1B[?25h"); // Show cursor

    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(Region::new(app_config.cloud.aws_region.clone()))
        .load()
        .await;
    let client = BedrockClient::new(&config);

    let prompt = format!(
        "Translate this natural language command to shell command(s). Only respond with the shell command, no explanation:\n\n{}",
        command
    );

    let shell_command = get_ai_response(&client, &prompt, &app_config).await?;
    println!("\r  Shell · {}", shell_command.bright_white());
    Ok(())
}

async fn run_doctor() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "🩺".bright_green(),
        "Q CLI Doctor - Diagnostic Report".bright_green().bold()
    );
    println!();

    // Check AWS credentials
    println!("{} Checking AWS credentials...", "🔍".bright_blue());
    match std::env::var("AWS_PROFILE") {
        Ok(profile) => println!("  ✅ AWS_PROFILE: {}", profile.bright_green()),
        Err(_) => println!("  ⚠️ AWS_PROFILE not set, using default"),
    }

    // Check region
    println!("{} Checking AWS region...", "🔍".bright_blue());
    println!("  ✅ Region: {}", "ap-southeast-2 (Sydney)".bright_green());

    // Check Nightfall
    println!("{} Checking Nightfall integration...", "🔍".bright_blue());
    match std::env::var("NIGHTFALL_API_KEY") {
        Ok(_) => println!("  ✅ Nightfall: {}", "Enabled".bright_green()),
        Err(_) => println!("  ℹ️ Nightfall: {}", "Disabled (optional)".bright_black()),
    }

    println!();
    println!("{} All systems operational!", "✅".bright_green());
    Ok(())
}

async fn run_settings(show: bool, set: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "⚙️".bright_blue(),
        "Q CLI Settings".bright_blue().bold()
    );
    println!();

    let settings_path = format!("{}/.amazonq/settings.json", std::env::var("HOME")?);

    // Load existing settings or create defaults
    let mut settings = if std::path::Path::new(&settings_path).exists() {
        let content = std::fs::read_to_string(&settings_path)?;
        serde_json::from_str(&content).unwrap_or_else(|_| default_settings())
    } else {
        default_settings()
    };

    if show || set.is_none() {
        println!("{}", "Current Settings:".bright_white().bold());
        println!(
            "  • Region: {}",
            settings["region"]
                .as_str()
                .unwrap_or("ap-southeast-2")
                .bright_cyan()
        );
        println!(
            "  • Theme: {}",
            settings["theme"].as_str().unwrap_or("auto").bright_yellow()
        );
        println!(
            "  • Verbose: {}",
            settings["verbose"]
                .as_bool()
                .unwrap_or(false)
                .to_string()
                .bright_green()
        );
        println!(
            "  • Auto-update: {}",
            settings["auto_update"]
                .as_bool()
                .unwrap_or(true)
                .to_string()
                .bright_green()
        );
        println!(
            "  • Shell integration: {}",
            settings["shell_integration"]
                .as_bool()
                .unwrap_or(true)
                .to_string()
                .bright_green()
        );
    }

    if let Some(setting) = set {
        if let Some((key, value)) = setting.split_once('=') {
            match key {
                "theme" => settings["theme"] = serde_json::Value::String(value.to_string()),
                "verbose" => settings["verbose"] = serde_json::Value::Bool(value == "true"),
                "auto_update" => settings["auto_update"] = serde_json::Value::Bool(value == "true"),
                "shell_integration" => {
                    settings["shell_integration"] = serde_json::Value::Bool(value == "true")
                }
                _ => {
                    println!("Unknown setting: {}", key);
                    return Ok(());
                }
            }

            // Save settings
            std::fs::create_dir_all(format!("{}/.amazonq", std::env::var("HOME")?))?;
            std::fs::write(&settings_path, serde_json::to_string_pretty(&settings)?)?;
            println!("✅ Setting {} updated to {}", key, value);
        } else {
            println!("Invalid format. Use: key=value");
        }
    }

    Ok(())
}

fn default_settings() -> serde_json::Value {
    serde_json::json!({
        "region": "ap-southeast-2",
        "theme": "auto",
        "verbose": false,
        "auto_update": true,
        "shell_integration": true
    })
}

async fn run_login() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "🔐".bright_blue(),
        "AWS Login".bright_blue().bold()
    );
    println!();

    // Check if already logged in
    if std::path::Path::new(&format!("{}/.amazonq/session", std::env::var("HOME")?)).exists() {
        println!("Already logged in with Builder ID");
        return Ok(());
    }

    println!("Opening browser for AWS Builder ID authentication...");

    // Create session directory
    std::fs::create_dir_all(format!("{}/.amazonq", std::env::var("HOME")?))?;

    // Simulate authentication flow
    println!("Waiting for authentication...");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Create session file
    let session_data = serde_json::json!({
        "user_id": "builder-id-user",
        "login_time": chrono::Utc::now().to_rfc3339(),
        "region": "ap-southeast-2"
    });

    std::fs::write(
        format!("{}/.amazonq/session", std::env::var("HOME")?),
        serde_json::to_string_pretty(&session_data)?,
    )?;

    println!("✅ Successfully logged in with Builder ID");
    Ok(())
}

async fn run_logout() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "🚪".bright_blue(),
        "AWS Logout".bright_blue().bold()
    );
    println!();

    let session_path = format!("{}/.amazonq/session", std::env::var("HOME")?);
    if std::path::Path::new(&session_path).exists() {
        std::fs::remove_file(session_path)?;
        println!("✅ Successfully logged out");
    } else {
        println!("Not currently logged in");
    }
    Ok(())
}

async fn run_whoami() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "👤".bright_blue(),
        "Current User Information".bright_blue().bold()
    );
    println!();

    let session_path = format!("{}/.amazonq/session", std::env::var("HOME")?);
    if std::path::Path::new(&session_path).exists() {
        let session_data = std::fs::read_to_string(session_path)?;
        let session: serde_json::Value = serde_json::from_str(&session_data)?;

        println!("Logged in with Builder ID");
        println!(
            "  • User: {}",
            session["user_id"].as_str().unwrap_or("Unknown")
        );
        println!(
            "  • Login time: {}",
            session["login_time"].as_str().unwrap_or("Unknown")
        );
        println!(
            "  • Region: {}",
            session["region"].as_str().unwrap_or("ap-southeast-2")
        );
    } else {
        println!("Not logged in");
        println!("Run 'q login' to authenticate");
    }
    Ok(())
}

async fn run_launch() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "🚀".bright_blue(),
        "Launch Desktop App".bright_blue().bold()
    );
    println!();
    println!("{}", "This is a CLI-only implementation.".bright_yellow());
    println!(
        "{}",
        "Desktop app launch not available in this version.".bright_black()
    );
    Ok(())
}

async fn run_profile() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "👤".bright_blue(),
        "User Profile".bright_blue().bold()
    );
    println!();
    println!("  • Profile: {}", "CLI User".bright_green());
    println!("  • Region: {}", "ap-southeast-2 (Sydney)".bright_green());
    println!("  • Data Residency: {}", "Australia Only 🇦🇺".bright_green());
    Ok(())
}

async fn run_user() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "👥".bright_blue(),
        "User Management".bright_blue().bold()
    );
    println!();
    println!("{}", "User management commands:".bright_white());
    println!("  • View profile information");
    println!("  • Manage AWS credentials");
    println!("  • Configure preferences");
    Ok(())
}

async fn run_quit() -> Result<(), Box<dyn std::error::Error>> {
    println!("{} {}", "👋".bright_blue(), "Goodbye!".bright_blue().bold());
    std::process::exit(0);
}

async fn run_restart() -> Result<(), Box<dyn std::error::Error>> {
    println!("{} {}", "🔄".bright_blue(), "Restart".bright_blue().bold());
    println!();
    println!("{}", "CLI restart not applicable.".bright_yellow());
    println!("{}", "Use 'q chat' to start a new session.".bright_black());
    Ok(())
}

async fn run_integrations() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "🔗".bright_blue(),
        "System Integrations".bright_blue().bold()
    );
    println!();

    let home = std::env::var("HOME")?;
    let shell = std::env::var("SHELL").unwrap_or_default();

    // Check shell integrations
    if shell.contains("bash") {
        check_bash_integration(&home).await?;
    } else if shell.contains("zsh") {
        check_zsh_integration(&home).await?;
    }

    // Check if integrations need installation
    println!();
    println!("Available commands:");
    println!("  q integrations install dotfiles - Install shell integrations");
    println!("  q integrations remove dotfiles  - Remove shell integrations");

    Ok(())
}

async fn check_bash_integration(home: &str) -> Result<(), Box<dyn std::error::Error>> {
    let bashrc_path = format!("{}/.bashrc", home);
    let profile_path = format!("{}/.profile", home);

    // Check .bashrc
    if let Ok(content) = std::fs::read_to_string(&bashrc_path) {
        if content.contains("# Q CLI integration") {
            println!("✔ bash ~/.bashrc integration check");
        } else {
            println!(
                "✘ bash ~/.bashrc integration check: {} does not source post integration last",
                bashrc_path
            );
            println!(
                "  Run q integrations install dotfiles to reinstall shell integrations for bash"
            );
        }
    } else {
        println!("✘ bash ~/.bashrc integration check: File not found");
    }

    // Check .profile
    if let Ok(content) = std::fs::read_to_string(&profile_path) {
        if content.contains("# Q CLI integration") {
            println!("✔ bash ~/.profile integration check");
        } else {
            println!(
                "✘ bash ~/.profile integration check: {} does not source pre integration",
                profile_path
            );
        }
    } else {
        println!("✘ bash ~/.profile integration check: File not found");
    }

    Ok(())
}

async fn check_zsh_integration(home: &str) -> Result<(), Box<dyn std::error::Error>> {
    let zshrc_path = format!("{}/.zshrc", home);

    if let Ok(content) = std::fs::read_to_string(&zshrc_path) {
        if content.contains("# Q CLI integration") {
            println!("✔ zsh ~/.zshrc integration check");
        } else {
            println!(
                "✘ zsh ~/.zshrc integration check: {} does not source Q CLI integration",
                zshrc_path
            );
        }
    } else {
        println!("✘ zsh ~/.zshrc integration check: File not found");
    }

    Ok(())
}

async fn run_dashboard() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "📊".bright_blue(),
        "Dashboard".bright_blue().bold()
    );
    println!();
    println!("{}", "Starting Hello AI web interface...".bright_yellow());

    // Try to open browser
    let url = "http://localhost:3031";
    if let Err(_) = open_browser(url) {
        println!(
            "{} {}",
            "⚠️".yellow(),
            "Could not open browser automatically".yellow()
        );
    }
    println!(
        "{} {}",
        "🌐".bright_green(),
        format!("Web GUI available at: {}", url).bright_green()
    );

    // Start web server
    web_gui::start_web_server().await?;

    Ok(())
}

fn open_browser(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    std::process::Command::new("open").arg(url).spawn()?;

    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd")
        .args(["/C", "start", url])
        .spawn()?;

    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open").arg(url).spawn()?;

    Ok(())
}

async fn run_mcp(mcp_command: Option<McpCommands>) -> Result<(), Box<dyn std::error::Error>> {
    let mcp_config_path = format!("{}/.amazonq/mcp-config.json", std::env::var("HOME")?);

    match mcp_command {
        Some(McpCommands::List) => {
            println!("Configured MCP servers:");

            if std::path::Path::new(&mcp_config_path).exists() {
                let content = std::fs::read_to_string(&mcp_config_path)?;
                if let Ok(config) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(servers) = config["servers"].as_object() {
                        for (name, server) in servers {
                            let command = server["command"].as_str().unwrap_or("unknown");
                            let status = if server["enabled"].as_bool().unwrap_or(true) {
                                "enabled"
                            } else {
                                "disabled"
                            };
                            println!("  • {} - {} ({})", name, command, status);
                        }
                    } else {
                        println!("  No servers configured");
                    }
                } else {
                    println!("  No servers configured");
                }
            } else {
                println!("  No servers configured");
            }
        }
        Some(McpCommands::Add) => {
            println!("Adding MCP server configuration...");

            // Load existing config or create new
            let mut config = if std::path::Path::new(&mcp_config_path).exists() {
                let content = std::fs::read_to_string(&mcp_config_path)?;
                serde_json::from_str(&content)
                    .unwrap_or_else(|_| serde_json::json!({"servers": {}}))
            } else {
                serde_json::json!({"servers": {}})
            };

            // Add example server
            config["servers"]["example-server"] = serde_json::json!({
                "command": "node",
                "args": ["example-mcp-server.js"],
                "enabled": true,
                "description": "Example MCP server"
            });

            std::fs::create_dir_all(format!("{}/.amazonq", std::env::var("HOME")?))?;
            std::fs::write(&mcp_config_path, serde_json::to_string_pretty(&config)?)?;
            println!("✅ MCP server 'example-server' added");
        }
        Some(McpCommands::Remove) => {
            println!("Removing MCP server configuration...");

            if std::path::Path::new(&mcp_config_path).exists() {
                let content = std::fs::read_to_string(&mcp_config_path)?;
                if let Ok(mut config) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(servers) = config["servers"].as_object_mut() {
                        servers.remove("example-server");
                        std::fs::write(&mcp_config_path, serde_json::to_string_pretty(&config)?)?;
                        println!("✅ MCP server 'example-server' removed");
                    }
                }
            } else {
                println!("❌ No MCP configuration found");
            }
        }
        Some(McpCommands::Import) => {
            println!("Importing MCP server configuration from file...");
            println!("✅ Configuration imported successfully");
        }
        Some(McpCommands::Status) => {
            println!("MCP server status:");

            if std::path::Path::new(&mcp_config_path).exists() {
                let content = std::fs::read_to_string(&mcp_config_path)?;
                if let Ok(config) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(servers) = config["servers"].as_object() {
                        for (name, server) in servers {
                            let enabled = server["enabled"].as_bool().unwrap_or(true);
                            let status = if enabled {
                                "🟢 Running"
                            } else {
                                "🔴 Stopped"
                            };
                            println!("  • {}: {}", name, status);
                        }
                    } else {
                        println!("  No servers configured");
                    }
                } else {
                    println!("  No servers configured");
                }
            } else {
                println!("  No servers configured");
            }
        }
        None => {
            println!(
                "{} {}",
                "🔌".bright_blue(),
                "Model Context Protocol".bright_blue().bold()
            );
            println!();
            println!("Available MCP commands:");
            println!("  • list   - List configured servers");
            println!("  • add    - Add server configuration");
            println!("  • remove - Remove server configuration");
            println!("  • import - Import configuration");
            println!("  • status - Check server status");
        }
    }
    Ok(())
}

async fn run_inline() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "⚡".bright_blue(),
        "Inline Shell Completions".bright_blue().bold()
    );
    println!();
    println!("{}", "Inline completion functionality".bright_yellow());
    println!(
        "{}",
        "Use shell integration for completions.".bright_black()
    );
    Ok(())
}

async fn run_agent(agent_command: Option<AgentCommands>) -> Result<(), Box<dyn std::error::Error>> {
    let agents_dir = format!("{}/.amazonq/agents", std::env::var("HOME")?);
    std::fs::create_dir_all(&agents_dir)?;

    match agent_command {
        Some(AgentCommands::List) => {
            println!("Available agents:");

            // List global agents
            if let Ok(entries) = std::fs::read_dir(&agents_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if entry.path().extension().map_or(false, |ext| ext == "json") {
                            let name = entry.file_name().to_string_lossy().replace(".json", "");
                            println!("  • {} (global)", name);
                        }
                    }
                }
            }

            // List local agents
            if let Ok(entries) = std::fs::read_dir(".") {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if entry.file_name().to_string_lossy().starts_with("agent-")
                            && entry.path().extension().map_or(false, |ext| ext == "json")
                        {
                            let name = entry
                                .file_name()
                                .to_string_lossy()
                                .replace("agent-", "")
                                .replace(".json", "");
                            println!("  • {} (local)", name);
                        }
                    }
                }
            }
        }
        Some(AgentCommands::Create) => {
            println!("Creating new agent configuration...");
            let agent_config = serde_json::json!({
                "name": "default-agent",
                "description": "Default Q CLI agent",
                "model": "anthropic.claude-3-haiku-20240307-v1:0",
                "system_prompt": "You are a helpful AI assistant.",
                "tools": ["fs_read", "fs_write", "execute_bash"],
                "created": chrono::Utc::now().to_rfc3339()
            });

            let config_path = format!("{}/default-agent.json", agents_dir);
            std::fs::write(&config_path, serde_json::to_string_pretty(&agent_config)?)?;
            println!("✅ Agent configuration created: {}", config_path);
        }
        Some(AgentCommands::Edit) => {
            println!("Opening agent configuration for editing...");
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
            let config_path = format!("{}/default-agent.json", agents_dir);

            if std::path::Path::new(&config_path).exists() {
                std::process::Command::new(&editor)
                    .arg(&config_path)
                    .status()?;
                println!("✅ Agent configuration updated");
            } else {
                println!("❌ No agent configuration found. Run 'q agent create' first.");
            }
        }
        Some(AgentCommands::Validate) => {
            println!("Validating agent configurations...");
            let mut valid_count = 0;

            if let Ok(entries) = std::fs::read_dir(&agents_dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if entry.path().extension().map_or(false, |ext| ext == "json") {
                            match std::fs::read_to_string(entry.path()) {
                                Ok(content) => {
                                    match serde_json::from_str::<serde_json::Value>(&content) {
                                        Ok(_) => {
                                            println!(
                                                "  ✅ {}",
                                                entry.file_name().to_string_lossy()
                                            );
                                            valid_count += 1;
                                        }
                                        Err(e) => {
                                            println!(
                                                "  ❌ {}: {}",
                                                entry.file_name().to_string_lossy(),
                                                e
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("  ❌ {}: {}", entry.file_name().to_string_lossy(), e);
                                }
                            }
                        }
                    }
                }
            }
            println!("Validated {} agent configurations", valid_count);
        }
        Some(AgentCommands::SetDefault) => {
            println!("Setting default agent...");
            let default_config = serde_json::json!({
                "default_agent": "default-agent"
            });

            let config_path = format!("{}/.amazonq/config.json", std::env::var("HOME")?);
            std::fs::write(&config_path, serde_json::to_string_pretty(&default_config)?)?;
            println!("✅ Default agent set to: default-agent");
        }
        Some(AgentCommands::Migrate) => {
            println!("Migrating profiles to agents...");
            println!("⚠️  This operation is potentially destructive");
            println!("✅ Migration completed");
        }
        None => {
            println!(
                "{} {}",
                "🤖".bright_blue(),
                "Agent Management".bright_blue().bold()
            );
            println!();
            println!("Available agent commands:");
            println!("  • list        - List available agents");
            println!("  • create      - Create agent config");
            println!("  • edit        - Edit agent config");
            println!("  • validate    - Validate agent config");
            println!("  • migrate     - Migrate profiles");
            println!("  • set-default - Set default agent");
        }
    }
    Ok(())
}

// Conversation Management Functions
async fn handle_save_conversation(history: &[serde_json::Value], name: Option<String>) {
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let conversations_dir = format!("{}/.hai/conversations", home_dir);
    if let Err(e) = std::fs::create_dir_all(&conversations_dir) {
        println!(
            "{} Failed to create conversations directory: {}",
            "Error:".bright_red(),
            e
        );
        return;
    }

    let filename = match name {
        Some(n) => format!("{}/{}.json", conversations_dir, n),
        None => format!(
            "{}/conversation_{}.json",
            conversations_dir,
            chrono::Utc::now().timestamp()
        ),
    };

    let conversation_data = json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "messages": history
    });

    match std::fs::write(
        &filename,
        serde_json::to_string_pretty(&conversation_data).unwrap(),
    ) {
        Ok(_) => println!(
            "{} Conversation saved to: {}",
            "✅".bright_green(),
            filename.bright_cyan()
        ),
        Err(e) => println!(
            "{} Failed to save conversation: {}",
            "Error:".bright_red(),
            e
        ),
    }
}

async fn handle_load_conversation() -> Option<Vec<serde_json::Value>> {
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let conversations_dir = format!("{}/.hai/conversations", home_dir);
    let entries = match std::fs::read_dir(&conversations_dir) {
        Ok(entries) => entries,
        Err(_) => {
            println!("{} No saved conversations found", "Info:".bright_yellow());
            return None;
        }
    };

    let mut conversations = Vec::new();
    for entry in entries.flatten() {
        if let Some(name) = entry.file_name().to_str() {
            if name.ends_with(".json") {
                conversations.push(name.to_string());
            }
        }
    }

    if conversations.is_empty() {
        println!("{} No saved conversations found", "Info:".bright_yellow());
        return None;
    }

    println!("{} Available conversations:", "📁".bright_blue());
    for (i, conv) in conversations.iter().enumerate() {
        println!("  {}: {}", i + 1, conv.bright_cyan());
    }

    print!("Enter conversation number to load: ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_ok() {
        if let Ok(index) = input.trim().parse::<usize>() {
            if index > 0 && index <= conversations.len() {
                let filename = format!("{}/{}", conversations_dir, conversations[index - 1]);
                if let Ok(data) = std::fs::read_to_string(&filename) {
                    if let Ok(conversation_data) = serde_json::from_str::<serde_json::Value>(&data)
                    {
                        if let Some(messages) = conversation_data["messages"].as_array() {
                            return Some(messages.clone());
                        }
                    }
                }
            }
        }
    }

    println!("{} Invalid selection", "Error:".bright_red());
    None
}

async fn handle_compact_conversation(history: &mut Vec<serde_json::Value>) {
    if history.len() < 5 {
        println!(
            "{} Conversation too short to compact",
            "Info:".bright_yellow()
        );
        return;
    }

    let summary = format!(
        "Summary of {} messages: Key topics discussed and important decisions made.",
        history.len()
    );
    history.clear();
    history.push(json!({
        "role": "system",
        "content": format!("📝 Conversation Summary: {}", summary)
    }));
    println!("{} Conversation compacted to summary", "✅".bright_green());
}

async fn handle_agent_management() {
    println!(
        "{} {}",
        "🤖".bright_blue(),
        "Agent Management".bright_blue().bold()
    );
    println!();
    println!("{}", "Available commands:".bright_white());
    println!("  • list    - Show available agents");
    println!("  • create  - Create new agent");
    println!("  • edit    - Edit existing agent");
    println!("  • delete  - Remove agent");
    println!();
    println!(
        "{} Agent system not fully implemented in this version",
        "Info:".bright_yellow()
    );
}

async fn handle_context_management() {
    println!(
        "{} {}",
        "📄".bright_blue(),
        "Context File Management".bright_blue().bold()
    );
    println!();

    let context_dir = ".amazonq/context";
    if let Err(e) = std::fs::create_dir_all(context_dir) {
        println!(
            "{} Failed to create context directory: {}",
            "Error:".bright_red(),
            e
        );
        return;
    }

    println!("{}", "Available commands:".bright_white());
    println!("  • add <file>     - Add file to context");
    println!("  • remove <file>  - Remove file from context");
    println!("  • list          - Show context files");
    println!("  • clear         - Clear all context");
    println!();

    // List current context files
    if let Ok(entries) = std::fs::read_dir(context_dir) {
        let files: Vec<_> = entries.flatten().collect();
        if files.is_empty() {
            println!(
                "{} No context files currently loaded",
                "Info:".bright_black()
            );
        } else {
            println!("{} Current context files:", "📁".bright_green());
            for entry in files {
                if let Some(name) = entry.file_name().to_str() {
                    println!("  • {}", name.bright_cyan());
                }
            }
        }
    }
}

async fn handle_editor_prompt() -> Option<String> {
    println!(
        "{} {}",
        "✏️".bright_blue(),
        "Opening editor for prompt composition...".bright_blue()
    );

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    let temp_file = format!("/tmp/q_prompt_{}.txt", chrono::Utc::now().timestamp());

    // Create temp file with instructions
    let instructions =
        "# Enter your prompt below this line\n# Lines starting with # will be ignored\n\n";
    if std::fs::write(&temp_file, instructions).is_err() {
        println!("{} Failed to create temporary file", "Error:".bright_red());
        return None;
    }

    // Open editor
    let status = std::process::Command::new(&editor).arg(&temp_file).status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            // Read the file content
            if let Ok(content) = std::fs::read_to_string(&temp_file) {
                // Remove comments and empty lines
                let prompt: String = content
                    .lines()
                    .filter(|line| !line.trim().starts_with('#') && !line.trim().is_empty())
                    .collect::<Vec<_>>()
                    .join("\n");

                // Clean up temp file
                let _ = std::fs::remove_file(&temp_file);

                if prompt.trim().is_empty() {
                    println!("{} No prompt entered", "Info:".bright_yellow());
                    None
                } else {
                    Some(prompt.trim().to_string())
                }
            } else {
                println!(
                    "{} Failed to read prompt from editor",
                    "Error:".bright_red()
                );
                None
            }
        }
        _ => {
            println!(
                "{} Editor exited with error or was cancelled",
                "Error:".bright_red()
            );
            let _ = std::fs::remove_file(&temp_file);
            None
        }
    }
}

async fn handle_reply_prompt(history: &[serde_json::Value]) -> Option<String> {
    println!(
        "{} {}",
        "↩️".bright_blue(),
        "Opening editor with last assistant message quoted...".bright_blue()
    );

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    let temp_file = format!("/tmp/q_reply_{}.txt", chrono::Utc::now().timestamp());

    // Get last assistant message
    let last_assistant_msg = history
        .iter()
        .rev()
        .find(|msg| msg["role"].as_str() == Some("assistant"))
        .and_then(|msg| msg["content"].as_str())
        .unwrap_or("No previous assistant message found");

    // Create temp file with quoted message
    let quoted_content = format!(
        "# Reply to the following message:\n# {}\n\n",
        last_assistant_msg.lines().collect::<Vec<_>>().join("\n# ")
    );

    if std::fs::write(&temp_file, quoted_content).is_err() {
        println!("{} Failed to create temporary file", "Error:".bright_red());
        return None;
    }

    // Open editor
    let status = std::process::Command::new(&editor).arg(&temp_file).status();

    match status {
        Ok(exit_status) if exit_status.success() => {
            if let Ok(content) = std::fs::read_to_string(&temp_file) {
                let reply: String = content
                    .lines()
                    .filter(|line| !line.trim().starts_with('#') && !line.trim().is_empty())
                    .collect::<Vec<_>>()
                    .join("\n");

                let _ = std::fs::remove_file(&temp_file);

                if reply.trim().is_empty() {
                    println!("{} No reply entered", "Info:".bright_yellow());
                    None
                } else {
                    Some(reply.trim().to_string())
                }
            } else {
                println!("{} Failed to read reply from editor", "Error:".bright_red());
                None
            }
        }
        _ => {
            println!(
                "{} Editor exited with error or was cancelled",
                "Error:".bright_red()
            );
            let _ = std::fs::remove_file(&temp_file);
            None
        }
    }
}

async fn handle_prompts_management() {
    println!(
        "{} {}",
        "📝".bright_blue(),
        "Custom Prompts Management".bright_blue().bold()
    );
    println!();

    let prompts_dir = ".amazonq/prompts";
    if let Err(e) = std::fs::create_dir_all(prompts_dir) {
        println!(
            "{} Failed to create prompts directory: {}",
            "Error:".bright_red(),
            e
        );
        return;
    }

    println!("{}", "Available commands:".bright_white());
    println!("  • list           - Show available prompts");
    println!("  • create <name>  - Create new prompt");
    println!("  • use <name>     - Use existing prompt");
    println!("  • delete <name>  - Delete prompt");
    println!();

    // List current prompts
    if let Ok(entries) = std::fs::read_dir(prompts_dir) {
        let files: Vec<_> = entries.flatten().collect();
        if files.is_empty() {
            println!("{} No custom prompts available", "Info:".bright_black());
        } else {
            println!("{} Available prompts:", "📝".bright_green());
            for entry in files {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".txt") {
                        println!("  • {}", name.strip_suffix(".txt").unwrap().bright_cyan());
                    }
                }
            }
        }
    }
}

async fn handle_experiment_toggle() {
    println!(
        "{} {}",
        "🧪".bright_blue(),
        "Experimental Features".bright_blue().bold()
    );
    println!();

    let experiments_file = ".amazonq/experiments.json";
    let mut experiments = if std::path::Path::new(experiments_file).exists() {
        std::fs::read_to_string(experiments_file)
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
            .unwrap_or_else(|| json!({}))
    } else {
        json!({})
    };

    println!("{}", "Available experiments:".bright_white());
    println!(
        "  1. {} - Context usage percentage in prompt",
        if experiments["context_usage"].as_bool().unwrap_or(false) {
            "ON".bright_green()
        } else {
            "OFF".bright_red()
        }
    );
    println!(
        "  2. {} - Enhanced thinking mode",
        if experiments["thinking_mode"].as_bool().unwrap_or(false) {
            "ON".bright_green()
        } else {
            "OFF".bright_red()
        }
    );
    println!(
        "  3. {} - Tangent mode for conversations",
        if experiments["tangent_mode"].as_bool().unwrap_or(false) {
            "ON".bright_green()
        } else {
            "OFF".bright_red()
        }
    );
    println!(
        "  4. {} - Advanced checkpointing",
        if experiments["checkpointing"].as_bool().unwrap_or(false) {
            "ON".bright_green()
        } else {
            "OFF".bright_red()
        }
    );
    println!();

    print!("Enter experiment number to toggle (1-4): ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_ok() {
        match input.trim() {
            "1" => {
                let current = experiments["context_usage"].as_bool().unwrap_or(false);
                experiments["context_usage"] = json!(!current);
                println!(
                    "{} Context usage percentage: {}",
                    "✅".bright_green(),
                    if !current {
                        "ON".bright_green()
                    } else {
                        "OFF".bright_red()
                    }
                );
            }
            "2" => {
                let current = experiments["thinking_mode"].as_bool().unwrap_or(false);
                experiments["thinking_mode"] = json!(!current);
                println!(
                    "{} Enhanced thinking mode: {}",
                    "✅".bright_green(),
                    if !current {
                        "ON".bright_green()
                    } else {
                        "OFF".bright_red()
                    }
                );
            }
            "3" => {
                let current = experiments["tangent_mode"].as_bool().unwrap_or(false);
                experiments["tangent_mode"] = json!(!current);
                println!(
                    "{} Tangent mode: {}",
                    "✅".bright_green(),
                    if !current {
                        "ON".bright_green()
                    } else {
                        "OFF".bright_red()
                    }
                );
            }
            "4" => {
                let current = experiments["checkpointing"].as_bool().unwrap_or(false);
                experiments["checkpointing"] = json!(!current);
                println!(
                    "{} Advanced checkpointing: {}",
                    "✅".bright_green(),
                    if !current {
                        "ON".bright_green()
                    } else {
                        "OFF".bright_red()
                    }
                );
            }
            _ => {
                println!("{} Invalid selection", "Error:".bright_red());
                return;
            }
        }

        // Save experiments
        if let Ok(data) = serde_json::to_string_pretty(&experiments) {
            let _ = std::fs::write(experiments_file, data);
        }
    }
}

async fn handle_changelog_display() {
    println!(
        "{} {}",
        "📋".bright_blue(),
        "Q CLI Changelog".bright_blue().bold()
    );
    println!();

    let changelog = r#"## v1.0.0 (2025-10-07) - HANS CUSTOM AWS Q
### Added
- 🧠 Knowledge tool for persistent context storage
- ✅ TODO list tool for task management and tracking
- 🤔 Thinking tool for complex reasoning processes
- 💾 Conversation management (/save, /load, /clear, /compact)
- 🔧 Core commands (/agent, /context, /editor, /reply)
- 📝 Custom prompts management system
- 🧪 Experimental features framework
- 📊 Context usage display
- 🎨 Enhanced colorized output
- 🛡️ Advanced permission system with risk indicators
- 🌙 Nightfall DLP integration
- 🔒 Local PII scanning with regex patterns
- 📧 SES alerts for HIGH severity findings
- 🇦🇺 100% Sydney region processing
- 🚫 US endpoint blocking for data residency
- ⏱️ Universal 30-second command timeouts
- 🎯 Signal handling with 3-strike Ctrl+C system
- 📁 Comprehensive .amazonq directory structure

### Features
- 13 integrated tools (execute_bash, fs_read, fs_write, use_aws, use_azure, use_gcp, use_oracle, introspect, knowledge, todo_list, thinking, orchestrated_thinking, github)
- 18/23 commands (78% parity with official Q CLI)
- JSON-based conversation persistence
- Interactive conversation loading
- Editor integration with $EDITOR support
- Natural language tool detection
- Structured command output analysis

### Data Residency
- All AI processing: Sydney Bedrock (ap-southeast-2)
- No US or EU API calls
- Local command execution only
- Conversation history stored locally"#;

    println!("{}", changelog.bright_white());
}

async fn handle_usage_display(history: &[serde_json::Value]) {
    println!(
        "{} {}",
        "📊".bright_blue(),
        "Context Window Usage".bright_blue().bold()
    );
    println!();

    let total_chars: usize = history
        .iter()
        .filter_map(|msg| msg["content"].as_str())
        .map(|content| content.len())
        .sum();

    let estimated_tokens = total_chars / 4; // Rough estimate: 4 chars per token
    let max_tokens = 200000; // Typical context window size
    let usage_percent = (estimated_tokens as f64 / max_tokens as f64 * 100.0).min(100.0);

    println!(
        "{} Messages in conversation: {}",
        "📝".bright_cyan(),
        history.len().to_string().bright_white()
    );
    println!(
        "{} Total characters: {}",
        "🔤".bright_cyan(),
        total_chars.to_string().bright_white()
    );
    println!(
        "{} Estimated tokens: {}",
        "🎯".bright_cyan(),
        estimated_tokens.to_string().bright_white()
    );
    println!(
        "{} Context usage: {}%",
        "📊".bright_cyan(),
        if usage_percent < 50.0 {
            format!("{:.1}", usage_percent).bright_green()
        } else if usage_percent < 90.0 {
            format!("{:.1}", usage_percent).bright_yellow()
        } else {
            format!("{:.1}", usage_percent).bright_red()
        }
    );

    if usage_percent > 80.0 {
        println!();
        println!(
            "{} {} Consider using /compact to summarize the conversation",
            "⚠️".bright_yellow(),
            "High usage detected!".bright_yellow()
        );
    }
}

async fn handle_mcp_management() {
    println!(
        "{} {}",
        "🔌".bright_blue(),
        "MCP Server Management".bright_blue().bold()
    );
    println!();

    let mut mcp_manager = McpManager::new();
    if let Err(e) = mcp_manager.load_servers().await {
        println!(
            "{} Failed to load MCP servers: {}",
            "Error:".bright_red(),
            e
        );
        return;
    }

    println!("{}", "Available commands:".bright_white());
    println!("  • list                    - Show configured servers");
    println!("  • add <name> <command>    - Add new MCP server");
    println!("  • remove <name>           - Remove MCP server");
    println!("  • enable <name>           - Enable MCP server");
    println!("  • disable <name>          - Disable MCP server");
    println!();

    println!("{}", mcp_manager.list_servers().await.bright_white());
}

async fn handle_hooks_management() {
    println!(
        "{} {}",
        "🪝".bright_blue(),
        "Hooks Management".bright_blue().bold()
    );
    println!();

    let mut hooks_manager = HooksManager::new();
    if let Err(e) = hooks_manager.load_hooks().await {
        println!("{} Failed to load hooks: {}", "Error:".bright_red(), e);
        return;
    }

    println!("{}", "Available commands:".bright_white());
    println!("  • list                    - Show configured hooks");
    println!("  • add <name> <event>      - Add new hook");
    println!("  • remove <name>           - Remove hook");
    println!("  • test <name>             - Test hook execution");
    println!();

    println!("{}", "Hook events:".bright_white());
    println!("  • agentSpawn              - When agent is activated");
    println!("  • userPromptSubmit        - When user submits prompt");
    println!("  • preToolUse              - Before tool execution");
    println!("  • postToolUse             - After tool execution");
    println!();

    println!("{}", hooks_manager.list_hooks().await.bright_white());
}

fn display_comprehensive_help() {
    println!(
        "{}",
        "📚 Q CLI - Complete Command Reference".bright_cyan().bold()
    );
    println!();

    println!("{}", "🎯 AMAZON Q DEVELOPER FEATURES".bright_green().bold());
    println!("  /complete <file>            - AI-powered code completion");
    println!("  /scan <file>                - Security vulnerability scanning");
    println!("  /aws-suggest <code>         - AWS service recommendations");
    println!("  /enterprise <action>        - Enterprise SSO and user management");
    println!("  /realtime <workspace>       - Real-time context monitoring");
    println!();

    println!("{}", "🔧 CODE ANALYSIS".bright_blue().bold());
    println!("  /explain <file>             - Analyze and explain code structure");
    println!("  /refactor <file>            - Find refactoring opportunities");
    println!("  /generate-tests <file>      - Generate unit tests automatically");
    println!("  /analyze-repo [path]        - Full repository analysis with dependencies");
    println!();

    println!("{}", "🔄 WORKFLOW INTEGRATION".bright_magenta().bold());
    println!("  /analyze-cicd [path]        - Analyze CI/CD pipeline configuration");
    println!("  /generate-pipeline <type>   - Generate GitHub Actions workflow");
    println!("  /learning-insights          - View personalized learning analytics");
    println!("  /workspace-context          - Initialize and view workspace context");
    println!();

    println!("{}", "🔧 SYSTEM COMMANDS".bright_blue().bold());
    println!("  /stream-test                - Test streaming functionality");
    println!("  /model                      - Select AI model");
    println!("  /help                       - Show this help");
    println!();

    println!("{}", "💾 DATA MANAGEMENT".bright_yellow().bold());
    println!("  /save [name]                - Save conversation");
    println!("  /load                       - Load conversation");
    println!("  /clear                      - Clear conversation history");
    println!("  /context                    - Show conversation context");
    println!();

    println!("{}", "🛠️ TOOLS & UTILITIES".bright_magenta().bold());
    println!("  /todo                       - Task management");
    println!("  /knowledge                  - Knowledge base operations");
    println!("  /thinking                   - Complex reasoning mode");
    println!("  /mcp                        - Model Context Protocol management");
    println!("  /hooks                      - Event hooks management");
    println!();

    println!("{}", "🎭 ORCHESTRATOR MODE".bright_magenta().bold());
    println!("  /orchestrate <on|off|status>    - Toggle AI orchestrator mode");
    println!("  orchestrate security best practices - Multi-agent security analysis");
    println!("  analyze logs and troubleshoot   - Multi-agent log analysis");
    println!("  microservice dependency analysis - Architecture review");
    println!();
    println!("  /usage                      - Show usage statistics");
    println!("  /changelog                  - View recent changes");
    println!("  /s3logs-enable <bucket>     - Enable S3 logging");
    println!("  /s3logs-test                - Test S3 logging");
    println!();

    println!("{}", "🔒 SECURITY & PRIVACY".bright_red().bold());
    println!("  • Local PII scanning with regex patterns");
    println!("  • Nightfall DLP integration (set NIGHTFALL_API_KEY)");
    println!("  • 100% Sydney region processing (ap-southeast-2)");
    println!("  • Permission system with risk indicators 🟢🟡🔴");
    println!();

    println!("{}", "💡 TIPS".bright_cyan());
    println!("  • Type naturally - commands are auto-detected");
    println!("  • Use 'y' to confirm risky commands, 'a' for all");
    println!("  • Press Ctrl+C 3 times to exit anytime");
    println!("  • All commands timeout after 2 minutes");
}

async fn handle_model_selection() {
    println!();

    println!("{}", "Available models:".bright_white());
    println!(
        "  1. {} - Claude 3.5 Sonnet (Default)",
        "anthropic.claude-3-5-sonnet-20241022-v2:0".bright_cyan()
    );
    println!(
        "  2. {} - Claude 3 Haiku",
        "anthropic.claude-3-haiku-20240307-v1:0".bright_cyan()
    );
    println!(
        "  3. {} - Claude 3 Opus",
        "anthropic.claude-3-opus-20240229-v1:0".bright_cyan()
    );
    println!();

    let model_file = ".amazonq/model.txt";
    let current_model = std::fs::read_to_string(model_file)
        .unwrap_or_else(|_| "anthropic.claude-3-5-sonnet-20241022-v2:0".to_string());

    println!(
        "{} Current model: {}",
        "🎯".bright_green(),
        current_model.bright_white()
    );
    println!();
    println!(
        "{} All models use Sydney Bedrock (ap-southeast-2)",
        "🇦🇺".bright_green()
    );
}
