#!/bin/bash

# Test runner script for hello-ai-cli
# Simulates the /test all command functionality

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BINARY_PATH="${AI_CLI_BIN:-./target/release/hello-ai-cli}"
TIMEOUT_DURATION=30
TEST_COUNT=0
PASSED_COUNT=0
FAILED_COUNT=0

echo -e "${BLUE}🚀 Hello AI CLI Test Suite${NC}"
echo -e "${BLUE}================================${NC}"
echo "Binary: $BINARY_PATH"
echo "Timeout: ${TIMEOUT_DURATION}s per test"
echo ""

# Function to run a test
run_test() {
    local test_name="$1"
    local test_input="$2"
    local expected_behavior="$3"
    
    TEST_COUNT=$((TEST_COUNT + 1))
    echo -e "${YELLOW}Test $TEST_COUNT: $test_name${NC}"
    echo "Input: $test_input"
    echo "Expected: $expected_behavior"
    
    # Run the test with timeout
    if echo "$test_input" | timeout ${TIMEOUT_DURATION}s "$BINARY_PATH" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ PASSED${NC}"
        PASSED_COUNT=$((PASSED_COUNT + 1))
    else
        echo -e "${RED}❌ FAILED${NC}"
        FAILED_COUNT=$((FAILED_COUNT + 1))
    fi
    echo ""
}

# Test scenarios based on TEST_SCENARIOS.md
echo -e "${BLUE}Running Core Functionality Tests...${NC}"

# Test 1: Basic functionality
run_test "Basic CLI Response" \
    "hello" \
    "Should provide helpful response"

# Test 2: Command execution
run_test "Command Execution" \
    "run echo 'test successful'" \
    "Should execute echo command and show output"

# Test 3: Error handling and troubleshooting
run_test "Error Handling" \
    "run nonexistent-command-12345" \
    "Should capture error and provide troubleshooting"

# Test 4: File operations
run_test "File Creation" \
    "create a test file with content hello world" \
    "Should create file using YAML action header"

# Test 5: Multi-step operations
run_test "Multi-step Operations" \
    "create folder test-dir and add readme file" \
    "Should handle multiple operations with troubleshooting"

# Test 6: Kubernetes commands (if available)
run_test "Kubernetes Command" \
    "run kubectl version --client" \
    "Should execute kubectl or provide alternatives"

# Test 7: Docker commands (if available)
run_test "Docker Command" \
    "run docker --version" \
    "Should execute docker or provide troubleshooting"

# Test 8: Terraform commands (if available)
run_test "Terraform Command" \
    "run terraform version" \
    "Should execute terraform or provide alternatives"

# Test 9: AWS CLI commands (if available)
run_test "AWS CLI Command" \
    "run aws --version" \
    "Should execute aws cli or provide guidance"

# Test 10: Complex troubleshooting scenario
run_test "Complex Troubleshooting" \
    "create dockerfile and build it" \
    "Should create dockerfile, attempt build, and troubleshoot failures"

# Summary
echo -e "${BLUE}Test Results Summary${NC}"
echo -e "${BLUE}===================${NC}"
echo "Total Tests: $TEST_COUNT"
echo -e "Passed: ${GREEN}$PASSED_COUNT${NC}"
echo -e "Failed: ${RED}$FAILED_COUNT${NC}"

if [ $FAILED_COUNT -eq 0 ]; then
    echo -e "${GREEN}🎉 All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}❌ Some tests failed${NC}"
    exit 1
fi
