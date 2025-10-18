# CI/CD Pipeline Documentation

## Overview

The Hello AI CLI project uses GitHub Actions for continuous integration and deployment. The pipeline automatically builds, tests, and releases the application.

## Pipeline Stages

### 1. Build and Test (`build-and-test`)
- **Triggers**: Push to `main`/`develop`, Pull Requests to `main`
- **Actions**:
  - Code formatting check (`cargo fmt`)
  - Linting with Clippy (`cargo clippy`)
  - Release build (`cargo build --release`)
  - Unit tests (`cargo test`)
  - Integration tests (simulates `/test all`)
  - Binary artifact upload

### 2. Security Scan (`security-scan`)
- **Dependencies**: Requires `build-and-test` to pass
- **Actions**:
  - Security audit (`cargo audit`)
  - Dependency vulnerability scan (`cargo deny`)

### 3. Release (`release`)
- **Triggers**: Push to `main` branch only
- **Dependencies**: Requires both previous stages to pass
- **Actions**:
  - Creates GitHub release with version tag
  - Uploads release binary
  - Generates release notes

## Local Development

### Quick Commands
```bash
# Build and test everything
make all

# Just build
make build

# Run tests (includes integration tests)
make test

# Run specific test scenario
make test-scenario

# Format and lint
make format lint
```

### Manual Testing
```bash
# Build the project
cargo build --release

# Run integration tests
AI_CLI_BIN=./target/release/hello-ai-cli ./scripts/run-tests.sh

# Test specific functionality
echo "your test input" | ./target/release/hello-ai-cli
```

## Test Scenarios

The integration tests cover:

1. **Basic CLI Response** - Ensures basic functionality works
2. **Command Execution** - Tests command execution capabilities  
3. **Error Handling** - Validates troubleshooting system
4. **File Operations** - Tests YAML action header file creation
5. **Multi-step Operations** - Complex workflows with troubleshooting
6. **Tool Integration** - kubectl, docker, terraform, aws cli
7. **Troubleshooting System** - Advanced error recovery scenarios

## Environment Variables

- `AI_CLI_BIN` - Path to the binary for testing (default: `./target/release/hello-ai-cli`)
- `CARGO_TERM_COLOR` - Enable colored output in CI

## Artifacts

- **Binary**: `hello-ai-cli-{commit-sha}` (retained for 30 days)
- **Releases**: Tagged releases with binaries attached

## Security

- Automated security audits on every build
- Dependency vulnerability scanning
- No secrets or credentials stored in repository
- All builds run in isolated GitHub Actions runners

## Troubleshooting CI/CD

### Common Issues

1. **Build Failures**: Check Rust version compatibility
2. **Test Timeouts**: Integration tests have 30s timeout per scenario
3. **Security Audit Failures**: Update dependencies with `cargo update`
4. **Clippy Warnings**: Fix with `cargo clippy --fix`

### Debug Commands
```bash
# Check what the CI sees
cargo fmt -- --check
cargo clippy -- -D warnings
cargo build --release --verbose
cargo test --verbose
```

## Release Process

1. Update version in `Cargo.toml`
2. Commit changes to `main` branch
3. CI automatically creates release with format: `v{version}-{build-number}`
4. Binary is attached to GitHub release
5. Release notes are auto-generated

## Monitoring

- Build status: Check GitHub Actions tab
- Test results: View in Actions logs
- Security alerts: GitHub Security tab
- Release history: GitHub Releases page
