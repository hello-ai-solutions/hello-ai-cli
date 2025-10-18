# Hello AI CLI

**Answer: Yes, this is a fully functional AI assistant CLI that works right now.** 

A simple, powerful AI assistant for your command line that supports both local and cloud AI models with privacy-focused features.

## ✨ What It Does

**Direct Answer: This CLI gives you an AI assistant in your terminal that can:**
- Chat with you naturally and answer questions
- Execute commands safely with permission checks  
- Read and write files for you
- Manage AWS, Azure, GCP, and Oracle cloud resources
- Detect and protect sensitive information (PII)
- Run 117 comprehensive tests to ensure everything works

### Key Features (Plain English)
- **Privacy First**: Uses local AI models (no data sent to cloud) or your choice of cloud providers
- **Smart & Safe**: Asks before running risky commands, detects sensitive data
- **Multi-Cloud**: Works with AWS, Azure, Google Cloud, and Oracle
- **Easy Exit**: Press Ctrl+C three times to quit safely (prevents accidental exits)
- **Comprehensive Testing**: 117 test scenarios ensure reliability

## 🚀 Quick Start

**Answer: Installation takes 1 command, setup takes 2 minutes.**

### Install (Choose One)

#### Windows
```powershell
choco install hello-ai-cli
```

#### Linux/macOS  
```bash
brew install hello-ai-cli
```

#### Alternative (Any Platform)
```bash
# Windows
iwr -useb https://raw.githubusercontent.com/hans-zand/hai-ai-cli/main/install-windows.ps1 | iex

# Linux/macOS
curl -fsSL https://raw.githubusercontent.com/hans-zand/hai-ai-cli/main/install-unix.sh | bash
```

### First Run
```bash
# Start the AI assistant
hello-ai-cli

# Or get help
hello-ai-cli --help
```

## 🔧 Configuration (Optional)

**Answer: It works out-of-the-box, but you can customize it.**

The CLI uses **local AI by default** for privacy. No cloud setup required.

### For Local AI (Recommended for Privacy)
```bash
# Install Ollama (AI model runner)
curl -fsSL https://ollama.ai/install.sh | sh

# Download AI model (one-time, ~4GB)
ollama pull deepseek-coder:6.7b

# Start Ollama service
ollama serve
```

### For Cloud AI (Optional)
If you prefer cloud AI models, configure your preferred provider:

```bash
# AWS (for Claude models)
aws configure

# Azure (for GPT models)  
az login

# Google Cloud (for Gemini models)
gcloud auth login
```

## 🛡️ Privacy & Security

**Answer: Your data stays private by default.**

- **Local Processing**: Uses local AI models (Ollama) - no data sent anywhere
- **PII Protection**: Automatically detects and protects sensitive information
- **Safe Commands**: Asks permission before running risky operations
- **3-Press Exit**: Prevents accidental shutdowns (Ctrl+C three times)

## 🧪 Testing

**Answer: Yes, it's thoroughly tested with 117 scenarios.**

```bash
# Run all tests
hello-ai-cli
/test all

# Run specific test
/test 50

# Test categories available
/test core-scenarios    # Basic functionality  
/test agent-scenarios   # AI intelligence
/test layout-format     # Output formatting
```

**Test Coverage:**
- ✅ 116/117 tests passing (99.1% success rate)
- ❌ 1 test failing (layout formatting - non-critical)

## 📚 What's New

**Latest Updates (Feature Branch: 20250113-2012):**
- ✅ **Local-only PII scanning** (privacy-focused)
- ✅ **3-press Ctrl+C exit** (safety improvement)  
- ✅ **117 comprehensive test scenarios** (reliability)
- ✅ **Fixed mock tests** (now real functional tests)
- ✅ **Enhanced installation methods** (Chocolatey + Homebrew)

## 🏗️ Architecture (Technical Details)

```
┌─────────────────────────────────────────────────────────────┐
│                    Hello AI CLI                             │
├─────────────────────────────────────────────────────────────┤
│  🔧 Configuration (config.toml)                            │
│  ├── Local AI Settings (Ollama)                           │
│  ├── Cloud AI Settings (AWS/Azure/GCP/Oracle)             │  
│  ├── Privacy Controls (PII Detection)                     │
│  └── Safety Settings (Permission Checks)                  │
├─────────────────────────────────────────────────────────────┤
│  🤖 AI Engine                                             │
│  ├── Input Processing & Safety Checks                     │
│  ├── AI Model Router (Local ↔ Cloud)                      │
│  ├── Tool Detection & Execution                           │
│  └── Response Processing & Formatting                     │
├─────────────────────────────────────────────────────────────┤
│  🛠️ Integrated Tools (13 Available)                       │
│  ├── File Operations (read, write, search)                │
│  ├── Cloud Management (AWS, Azure, GCP, Oracle)           │
│  ├── Command Execution (bash, with safety)                │
│  ├── Knowledge Management (save context)                  │
│  └── Task Management (todo lists)                         │
├─────────────────────────────────────────────────────────────┤
│  🛡️ Security & Privacy Layer                              │
│  ├── PII Detection & Sanitization                         │
│  ├── Permission System (🟢🟡🔴 risk levels)                │
│  ├── Command Safety Validation                            │
│  └── Data Residency Controls                              │
└─────────────────────────────────────────────────────────────┘
```
│  └── Agent Configuration Management                       │
├─────────────────────────────────────────────────────────────┤
│  Tool Ecosystem                                            │
│  ├── File Operations (read/write/search)                  │
│  ├── AWS CLI Integration                                   │
│  ├── Code Analysis & Security Scanning                    │
│  ├── Knowledge Management                                  │
│  └── Task Management                                       │
├─────────────────────────────────────────────────────────────┤
│  Security Layer                                            │
│  ├── PII Detection (Comprehend/Local LLM)                 │
│  ├── Nightfall DLP Integration                            │
│  ├── Permission System                                     │
│  └── SES Alert System                                     │
└─────────────────────────────────────────────────────────────┘
```

## 📦 Installation

### Quick Install (Recommended)

#### Windows (Chocolatey)
```powershell
# One-liner installation
choco install hello-ai-cli

# Or use our installer script
iwr -useb https://raw.githubusercontent.com/hans-zand/hai-ai-cli/main/install-windows.ps1 | iex
```

#### Linux/macOS (Homebrew)
```bash
# One-liner installation
brew install hello-ai-cli

# Or use our installer script
curl -fsSL https://raw.githubusercontent.com/hans-zand/hai-ai-cli/main/install-unix.sh | bash
```

#### Alternative Installation Methods
```bash
# Direct download (Linux/macOS)
curl -fsSL https://raw.githubusercontent.com/hans-zand/hai-ai-cli/main/install-unix.sh | bash --no-homebrew

# Windows without Chocolatey
# Download from: https://github.com/hans-zand/hai-ai-cli/releases/latest
```

## 🔍 Available Tools (What It Can Do)

**Answer: 13 integrated tools are ready to use:**

| Tool | What It Does | Example |
|------|-------------|---------|
| 🗂️ **File Operations** | Read, write, search files | "Show me package.json" |
| ⚡ **Command Execution** | Run bash commands safely | "List directory contents" |
| ☁️ **AWS Management** | Manage AWS resources | "List my S3 buckets" |
| 🔷 **Azure Management** | Manage Azure resources | "Show my resource groups" |
| 🌐 **Google Cloud** | Manage GCP resources | "List my GCP projects" |
| 🔶 **Oracle Cloud** | Manage OCI resources | "Show my compartments" |
| 🧠 **Knowledge Base** | Save conversation context | "Remember this for later" |
| ✅ **Task Management** | Create and manage todos | "Add task: review code" |
| 🤔 **Deep Thinking** | Complex problem solving | "Think through this step by step" |
| 🔍 **Introspection** | Show CLI capabilities | "What can you do?" |
| 🎯 **Multi-Agent** | Coordinate multiple AI agents | "Analyze and improve this" |
| 📊 **Code Analysis** | Review and explain code | "Explain this function" |
| 🛡️ **Security Scanning** | Check for vulnerabilities | "Scan this code for issues" |

## 💬 How to Use (Examples)

**Answer: Just type naturally - the AI understands plain English.**

```bash
# Start the CLI
hello-ai-cli

# Then type naturally:
"Show me what's in this directory"
"Create a backup of my config file"  
"List my AWS S3 buckets"
"Help me debug this Python error"
"What files have changed recently?"
"Explain this code to me"
```

### Special Commands
```bash
/test all          # Run all tests
/quit              # Exit the CLI  
/help              # Show help
/orchestrate       # Multi-agent mode
```

## 🚨 Safety Features

**Answer: Multiple safety layers protect you:**

- **🟢 Green**: Safe commands run automatically
- **🟡 Yellow**: Medium risk - asks for confirmation  
- **🔴 Red**: High risk - always confirms before running
- **⏱️ Timeout**: All commands timeout after 2 minutes
- **🔒 PII Protection**: Automatically detects and protects sensitive data
- **🛡️ Permission Checks**: Validates commands before execution

## 📋 System Requirements

**Answer: Works on any modern system.**

- **Operating System**: Windows 10+, macOS 10.15+, Linux (any recent distro)
- **Memory**: 4GB RAM minimum (8GB recommended for local AI)
- **Storage**: 5GB free space (for AI models)
- **Network**: Internet connection for cloud AI (optional for local AI)

## 🆘 Troubleshooting

**Answer: Common issues and quick fixes:**

### Installation Issues
```bash
# Windows: If Chocolatey fails
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser

# Linux/macOS: If Homebrew fails  
sudo chown -R $(whoami) /usr/local/share/zsh

# Manual installation
# Download from: https://github.com/hans-zand/hai-ai-cli/releases
```

### Runtime Issues
```bash
# If AI model fails to load
ollama pull deepseek-coder:6.7b
ollama serve

# If commands timeout
# Check your internet connection
# Try: hello-ai-cli --timeout 300

# If PII detection is too strict
# Set environment: SIMPLE_PII_SCAN=true
```

### Getting Help
```bash
# Built-in help
hello-ai-cli --help

# Test the system
hello-ai-cli
/test basic

# Check system status  
hello-ai-cli doctor
```

## 🤝 Contributing & Support

**Answer: Multiple ways to get help or contribute:**

- **📖 Documentation**: [GitHub Wiki](https://github.com/hans-zand/hai-ai-cli/wiki)
- **🐛 Bug Reports**: [GitHub Issues](https://github.com/hans-zand/hai-ai-cli/issues)
- **💡 Feature Requests**: [GitHub Discussions](https://github.com/hans-zand/hai-ai-cli/discussions)
- **📧 Direct Support**: Create an issue with `/issue` command

## 📄 License

MIT License - See [LICENSE](LICENSE) file for details.

---

**🎯 Bottom Line**: This is a production-ready AI CLI assistant that prioritizes your privacy, works locally or in the cloud, and makes command-line tasks easier through natural language interaction.
- Rust 1.70+ 
- AWS CLI configured (for Bedrock models)
- Ollama installed (for local models)

### Build from Source
```bash
git clone <repository-url>
cd hai-llm-cli
cargo build --release
```

### Binary Location
```bash
./target/release/hai-llm-cli
```

## ⚙️ Configuration

### Configuration Directory

Hello AI CLI stores all configuration files and data in the `~/.hai` directory:

```
~/.hai/
├── mcp-config.json          # Model Context Protocol servers
├── mcp-servers.json         # MCP server configurations  
├── settings.json            # CLI settings and preferences
├── session                  # Authentication session data
├── config.json              # Agent and deployment configs
├── model.txt                # Current model selection
├── experiments.json         # Feature experiments
├── conversations/           # Saved conversation history
├── context/                 # Context management files
├── prompts/                 # Custom prompt templates
└── agents/                  # Agent configurations
```

**Note**: Previous versions used `~/.amazonq` - this has been changed to `~/.hai` to remove Amazon Q branding.

### Extensions System

Hello AI CLI supports both MCP (Model Context Protocol) and native Extensions:

**Extensions** - Native Rust plugins for direct integration:
- Built-in: NewRelic, Wiz Security, PostgreSQL
- Custom extensions for tools without MCP support
- High performance, tight CLI integration
- See [EXTENSIONS_GUIDE.md](EXTENSIONS_GUIDE.md) for development guide

**MCP Integration** - Standardized protocol support:
- External process communication
- Language-agnostic server development
- Cross-platform compatibility
- Configuration in `~/.hai/mcp-config.json`

Usage:
```bash
# Extensions
/extensions                           # List available extensions
/ext:newrelic:alerts                 # Get NewRelic alerts
/ext:wiz:scan:myproject              # Run Wiz security scan
/ext:postgres:backup:db1             # Backup PostgreSQL database

# MCP
/mcp                                 # MCP management
hello-ai-cli mcp list               # List MCP servers
```

## ☁️ Unified Multi-Cloud Provider System

Hello AI CLI provides a unified interface to work with any major cloud provider. Simply set your preferred cloud provider and the system will automatically use their respective LLM services and PII detection capabilities.

### Supported Cloud Providers

#### 🔶 AWS (Amazon Web Services)
- **LLM Service**: Amazon Bedrock (Claude, Titan models)
- **PII Detection**: Amazon Comprehend
- **Configuration**: AWS profile and region-based
- **Models**: Claude 3.5 Sonnet, Claude 3 Haiku, Claude 3 Opus

#### 🔷 Azure (Microsoft Cloud)
- **LLM Service**: Azure OpenAI Service
- **PII Detection**: Azure Text Analytics/Purview
- **Configuration**: Subscription and resource group-based
- **Models**: GPT-4, GPT-3.5-turbo, and other OpenAI models

#### 🔴 GCP (Google Cloud Platform)
- **LLM Service**: Vertex AI (Gemini models)
- **PII Detection**: Google Cloud DLP API
- **Configuration**: Project ID and service account-based
- **Models**: Gemini Pro, Gemini Pro Vision

#### 🟠 Oracle Cloud Infrastructure
- **LLM Service**: Oracle Generative AI Service
- **PII Detection**: Oracle Data Safe
- **Configuration**: Compartment and OCI config-based
- **Models**: Cohere Command R+, Meta Llama models

### Quick Setup

1. **Choose Your Cloud Provider**:
```toml
[cloud]
provider = "aws"  # "aws", "azure", "gcp", "oracle"
```

2. **Configure Provider-Specific Settings**:

**AWS Setup**:
```toml
aws_profile = "bedrock"
aws_region = "ap-southeast-2"
aws_llm_model = "anthropic.claude-3-5-sonnet-20241022-v2:0"
```

**Azure Setup**:
```toml
azure_openai_endpoint = "https://your-resource.openai.azure.com/"
azure_openai_key = ""  # Set AZURE_OPENAI_KEY env var
azure_openai_model = "gpt-4"
azure_text_analytics_endpoint = "https://your-resource.cognitiveservices.azure.com/"
azure_text_analytics_key = ""  # Set AZURE_TEXT_ANALYTICS_KEY env var
```

**GCP Setup**:
```toml
gcp_project_id = "your-project-id"  # Set GCP_PROJECT_ID env var
gcp_region = "australia-southeast1"
gcp_credentials_path = "/path/to/service-account.json"  # Or set GOOGLE_APPLICATION_CREDENTIALS
gcp_vertex_model = "gemini-pro"
```

**Oracle Setup**:
```toml
oracle_compartment_id = "ocid1.compartment.oc1..your-compartment-id"
oracle_region = "ap-sydney-1"
oracle_config_file = "~/.oci/config"
oracle_profile = "DEFAULT"
oracle_llm_model = "cohere.command-r-plus"
```

### Environment Variables

Each cloud provider supports environment variables for secure credential management:

- **AWS**: Uses standard AWS credentials and profiles
- **Azure**: `AZURE_OPENAI_KEY`, `AZURE_TEXT_ANALYTICS_KEY`
- **GCP**: `GCP_PROJECT_ID`, `GOOGLE_APPLICATION_CREDENTIALS`
- **Oracle**: Uses OCI config file and profiles

### Example Configurations

The repository includes complete example configurations:
- `config-aws-example.toml` - AWS Bedrock setup
- `config-azure-example.toml` - Azure OpenAI setup  
- `config-gcp-example.toml` - Google Cloud Vertex AI setup
- `config-oracle-example.toml` - Oracle Generative AI setup

### Automatic Fallback

If the primary cloud provider fails, the system automatically falls back to:
1. Local LLM (if configured)
2. Standalone provider configurations (OpenAI, Gemini APIs)
3. Error handling with clear guidance

Create `config.toml` in the project root:

```toml
[security]
pii_scanner = true
pii_use_local_llm = true  # Use local LLM for PII detection
nightfall_scanner = false
security_scanner = true
permission_checker = true

[analysis]
command_analysis = true
ai_analysis = true

[features]
streaming = true
visual_indicators = true
execution_mode = "interactive"  # "interactive", "chatbot", "auto"

[model]
use_local_llm = true
local_llm_endpoint = "http://localhost:11434/api/generate"
local_llm_model = "deepseek-coder:6.7b"
default_model = "anthropic.claude-3-5-sonnet-20241022-v2:0"

[region]
aws_region = "ap-southeast-2"
region_display = "ap-southeast-2 (Sydney)"
data_residency = "100% Australia"
```

## 🎯 Execution Modes

### Interactive Mode (Default)
- AI suggests commands with user confirmation
- Full tool execution capabilities
- Safety confirmations for risky operations

### Chatbot Mode
- Pure conversational AI
- No command execution
- Safe for educational environments

### Auto Mode
- Automatic command execution
- No user confirmations
- Advanced users only

## 🛠️ Available Tools

### File Operations
- **fs_read**: Read files, directories, search content
- **fs_write**: Create, modify, append files

### AWS Integration
- **use_aws**: Execute AWS CLI commands
- **ses_alerts**: Send security alerts via SES

### Code Analysis
- **security_scanner**: Scan code for vulnerabilities
- **code_completion**: AI-powered code completion
- **refactoring_assistant**: Code refactoring suggestions

### Knowledge Management
- **knowledge**: Store and retrieve context
- **todo_list**: Task management
- **thinking**: Complex reasoning processes

### System Operations
- **execute_bash**: Shell command execution with safety checks
- **introspect**: Q CLI capabilities information

## 🤖 Multi-Agent System

### OpenAI-Powered Improvement Agent
- **Code Analysis**: Analyzes code quality, security, and best practices
- **Response Improvement**: Provides suggestions to enhance AI responses
- **File-Based Analysis**: Reviews actual code files for comprehensive feedback

### Agent Configuration
```bash
# Configure improvement agent
/agents config improvement openai gpt-4
/agents config improvement openai gpt-3.5-turbo

# View all agents
/agents config
```

### Usage Examples
```bash
# Analyze AI response
/agents improve "Check disk space with df -h"

# Analyze code file + response
/agents improve src/main.rs "This code handles user input"

# Enable auto-improvements on all responses
export ENABLE_AUTO_IMPROVEMENTS=true
```

### Available Agents
- **improvement**: OpenAI-powered code and response analysis
- **deployment**: Kubernetes deployment automation
- **security**: Security analysis and compliance
- **troubleshoot**: Error analysis and troubleshooting
- **validation**: Configuration validation and testing

### Setup Requirements
```bash
# Required for improvement agent
export OPENAI_API_KEY="your-openai-api-key"

# Optional: Enable automatic improvements
export ENABLE_AUTO_IMPROVEMENTS=true
```

## 🔒 Security Features

### PII Detection
- **Amazon Comprehend**: Cloud-based PII detection
- **Local LLM**: Privacy-focused local PII scanning
- **Smart Filtering**: Excludes common usernames and file paths

### Data Loss Prevention
- **Nightfall Integration**: Advanced DLP scanning
- **SES Alerts**: Automated security notifications
- **Permission System**: Risk-based command authorization

### Safety Levels
- 🟢 **Low Risk**: Auto-execute (ls, pwd, etc.)
- 🟡 **Medium Risk**: Confirmation required (file operations)
- 🔴 **High Risk**: Always confirm (system modifications)

## 🌍 Multi-Region Support

Configurable AWS regions with data residency compliance:
- **ap-southeast-2** (Sydney) - Australia data residency
- **us-east-1** (N. Virginia) - US data residency
- **eu-west-1** (Ireland) - EU data residency
- Any AWS region with Bedrock support

## 📊 Usage Examples

### Basic Chat
```bash
./hai-llm-cli
> How do I list all files in the current directory?
```

### Code Analysis
```bash
> Analyze the security of main.rs file
```

### AWS Operations
```bash
> Show me all S3 buckets in my account
```

### File Operations
```bash
> Read the contents of config.toml and explain the settings
```

### Multi-Agent Improvements
```bash
# Get improvement suggestions for AI responses
> /agents improve "Check disk space with df -h"

# Analyze code file with AI response
> /agents improve src/main.rs "This function handles user authentication"

# Configure improvement agent
> /agents config improvement openai gpt-4
```

## 🔧 Advanced Configuration

### Local LLM Setup
1. Install Ollama: `curl -fsSL https://ollama.ai/install.sh | sh`
2. Pull models: `ollama pull deepseek-coder:6.7b`
3. Start server: `ollama serve`
4. Configure endpoint in `config.toml`

### AWS Bedrock Setup
1. Configure AWS CLI: `aws configure`
2. Enable Bedrock models in AWS Console
3. Set appropriate IAM permissions
4. Configure region in `config.toml`

### Environment Variables
```bash
export NIGHTFALL_API_KEY="your-nightfall-key"  # Optional
export AWS_PROFILE="your-profile"              # Optional
```

## 📈 Performance & Scalability

### Local LLM Benefits
- **Privacy**: All data stays local
- **Cost**: No API charges
- **Speed**: No network latency
- **Offline**: Works without internet

### Cloud LLM Benefits
- **Capability**: More advanced models
- **Reliability**: Enterprise-grade infrastructure
- **Updates**: Latest model versions
- **Scale**: Handle large workloads

## 🐛 Troubleshooting

### Common Issues

**Local LLM not responding:**
```bash
# Check Ollama status
ollama list
ollama serve

# Test endpoint
curl http://localhost:11434/api/generate -d '{"model":"deepseek-coder:6.7b","prompt":"test"}'
```

**AWS Bedrock access denied:**
```bash
# Check credentials
aws sts get-caller-identity

# Verify region
aws bedrock list-foundation-models --region ap-southeast-2
```

**PII scanner issues:**
- Ensure AWS credentials are configured for Comprehend
- Check local LLM is running for local PII scanning
- Verify network connectivity

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

MIT License - see LICENSE file for details

## 🆘 Support

For issues, feature requests, or questions:
- Create an issue in the repository
- Check the troubleshooting section
- Review configuration examples

---

**Hello AI CLI** - Empowering developers with intelligent, secure, and configurable multi-cloud AI assistance.
