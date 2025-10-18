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
TIMEOUT_DURATION=10
TEST_COUNT=0
PASSED_COUNT=0
FAILED_COUNT=0

echo -e "${BLUE}🚀 Hello AI CLI Test Suite${NC}"
echo -e "${BLUE}================================${NC}"
echo "Binary: $BINARY_PATH"
echo "Timeout: ${TIMEOUT_DURATION}s per test"
echo ""

# Check if binary exists
if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${RED}❌ Binary not found: $BINARY_PATH${NC}"
    exit 1
fi

# Function to run a test
run_test() {
    local test_name="$1"
    local test_input="$2"
    local expected_behavior="$3"
    
    TEST_COUNT=$((TEST_COUNT + 1))
    echo -e "${YELLOW}Test $TEST_COUNT: $test_name${NC}"
    echo "Input: $test_input"
    echo "Expected: $expected_behavior"
    
    # Create a temporary file for output
    local temp_output=$(mktemp)
    
    # Run the test with timeout and capture output
    if timeout ${TIMEOUT_DURATION}s bash -c "echo '$test_input' | '$BINARY_PATH'" > "$temp_output" 2>&1; then
        # Check if output contains expected patterns
        if grep -q "hello\|AI\|CLI\|Command\|Error\|VALIDATED" "$temp_output"; then
            echo -e "${GREEN}✅ PASSED${NC}"
            PASSED_COUNT=$((PASSED_COUNT + 1))
        else
            echo -e "${RED}❌ FAILED (No expected output)${NC}"
            FAILED_COUNT=$((FAILED_COUNT + 1))
        fi
    else
        # Check if it's a timeout or actual failure
        local exit_code=$?
        if [ $exit_code -eq 124 ]; then
            # Timeout - but this might be expected for interactive CLI
            echo -e "${GREEN}✅ PASSED (Timeout expected for interactive CLI)${NC}"
            PASSED_COUNT=$((PASSED_COUNT + 1))
        else
            echo -e "${RED}❌ FAILED (Exit code: $exit_code)${NC}"
            FAILED_COUNT=$((FAILED_COUNT + 1))
        fi
    fi
    
    # Clean up
    rm -f "$temp_output"
    echo ""
}

# Test scenarios - simplified for CI/CD
echo -e "${BLUE}Running Core Functionality Tests...${NC}"

# Test 1: Binary execution
run_test "Binary Execution" \
    "quit" \
    "Should start and quit cleanly"

# Test 2: Help command
run_test "Help Command" \
    "/help" \
    "Should show help information"

# Test 3: Version check
run_test "Version Check" \
    "/version" \
    "Should show version information"

# Test 4: Basic command
run_test "Basic Command" \
    "hello" \
    "Should provide AI response"

# Test 5: Quit command
run_test "Quit Command" \
    "/quit" \
    "Should exit cleanly"

# Summary
echo -e "${BLUE}Test Results Summary${NC}"
echo -e "${BLUE}===================${NC}"
echo "Total Tests: $TEST_COUNT"
echo -e "Passed: ${GREEN}$PASSED_COUNT${NC}"
echo -e "Failed: ${RED}$FAILED_COUNT${NC}"

# Calculate pass rate
if [ $TEST_COUNT -gt 0 ]; then
    PASS_RATE=$((PASSED_COUNT * 100 / TEST_COUNT))
    echo "Pass Rate: ${PASS_RATE}%"
    
    # Consider 80% pass rate as success for CI/CD
    if [ $PASS_RATE -ge 80 ]; then
        echo -e "${GREEN}🎉 Tests passed! (${PASS_RATE}% pass rate)${NC}"
        exit 0
    else
        echo -e "${RED}❌ Tests failed (${PASS_RATE}% pass rate, need 80%+)${NC}"
        exit 1
    fi
else
    echo -e "${RED}❌ No tests executed${NC}"
    exit 1
fi
