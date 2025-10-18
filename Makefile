# Makefile for Hello AI CLI

.PHONY: build test clean install format lint check all help

# Default target
all: format lint build test

# Build the project
build:
	@echo "🔨 Building hello-ai-cli..."
	cargo build --release

# Run tests
test: build
	@echo "🧪 Running tests..."
	cargo test
	@echo "🚀 Running integration tests..."
	AI_CLI_BIN=./target/release/hello-ai-cli ./scripts/run-tests.sh

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

# Install locally
install: build
	@echo "📦 Installing hello-ai-cli..."
	cargo install --path .

# Format code
format:
	@echo "🎨 Formatting code..."
	cargo fmt

# Run linter
lint:
	@echo "🔍 Running linter..."
	cargo clippy -- -D warnings

# Check without building
check:
	@echo "✅ Checking code..."
	cargo check

# Run security audit
audit:
	@echo "🔒 Running security audit..."
	cargo audit

# Development build (debug)
dev:
	@echo "🔨 Building debug version..."
	cargo build

# Run specific test scenario
test-scenario:
	@echo "🧪 Running specific test scenario..."
	@read -p "Enter test input: " input; \
	echo "$$input" | ./target/release/hello-ai-cli

# Docker build
docker-build:
	@echo "🐳 Building Docker image..."
	docker build -t hello-ai-cli .

# Docker run
docker-run: docker-build
	@echo "🐳 Running Docker container..."
	docker run -it --rm hello-ai-cli

# Release build with optimizations
release:
	@echo "🚀 Building optimized release..."
	RUSTFLAGS="-C target-cpu=native" cargo build --release

# Help
help:
	@echo "Hello AI CLI - Available commands:"
	@echo ""
	@echo "  build         - Build the project in release mode"
	@echo "  test          - Run all tests (unit + integration)"
	@echo "  dev           - Build debug version"
	@echo "  clean         - Clean build artifacts"
	@echo "  install       - Install locally"
	@echo "  format        - Format code with rustfmt"
	@echo "  lint          - Run clippy linter"
	@echo "  check         - Check code without building"
	@echo "  audit         - Run security audit"
	@echo "  test-scenario - Run interactive test scenario"
	@echo "  docker-build  - Build Docker image"
	@echo "  docker-run    - Build and run Docker container"
	@echo "  release       - Build optimized release"
	@echo "  all           - Run format, lint, build, and test"
	@echo "  help          - Show this help message"
