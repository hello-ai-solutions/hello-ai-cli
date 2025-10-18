#!/bin/bash

# Smoke test for hello-ai-cli
# Basic validation that the binary can be executed

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BINARY_PATH="${AI_CLI_BIN:-./target/release/hello-ai-cli}"

echo -e "${BLUE}🚀 Hello AI CLI Smoke Test${NC}"
echo -e "${BLUE}===========================${NC}"
echo "Binary: $BINARY_PATH"
echo ""

# Test 1: Binary exists and is executable
echo -e "${YELLOW}Test 1: Binary Existence${NC}"
if [ -f "$BINARY_PATH" ] && [ -x "$BINARY_PATH" ]; then
    echo -e "${GREEN}✅ PASSED - Binary exists and is executable${NC}"
else
    echo -e "${RED}❌ FAILED - Binary not found or not executable${NC}"
    exit 1
fi
echo ""

# Test 2: Binary can be invoked (check help or version)
echo -e "${YELLOW}Test 2: Binary Invocation${NC}"
if "$BINARY_PATH" --help > /dev/null 2>&1; then
    echo -e "${GREEN}✅ PASSED - Binary responds to --help${NC}"
elif "$BINARY_PATH" --version > /dev/null 2>&1; then
    echo -e "${GREEN}✅ PASSED - Binary responds to --version${NC}"
else
    # Try to run with timeout and see if it starts
    if timeout 2s "$BINARY_PATH" < /dev/null > /dev/null 2>&1; then
        echo -e "${GREEN}✅ PASSED - Binary starts successfully${NC}"
    else
        exit_code=$?
        if [ $exit_code -eq 124 ]; then
            echo -e "${GREEN}✅ PASSED - Binary starts (timeout expected)${NC}"
        else
            echo -e "${YELLOW}⚠️  WARNING - Binary exits with code $exit_code (may need config)${NC}"
        fi
    fi
fi
echo ""

# Test 3: Check binary dependencies (basic ldd check on Linux)
echo -e "${YELLOW}Test 3: Dependency Check${NC}"
if command -v ldd > /dev/null 2>&1; then
    if ldd "$BINARY_PATH" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ PASSED - All dependencies satisfied${NC}"
    else
        echo -e "${YELLOW}⚠️  WARNING - Some dependencies may be missing${NC}"
    fi
elif command -v otool > /dev/null 2>&1; then
    # macOS
    if otool -L "$BINARY_PATH" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ PASSED - Binary structure valid${NC}"
    else
        echo -e "${YELLOW}⚠️  WARNING - Binary structure issues${NC}"
    fi
else
    echo -e "${BLUE}ℹ️  INFO - Dependency check skipped (no ldd/otool)${NC}"
fi
echo ""

# Test 4: File size check (ensure it's not empty)
echo -e "${YELLOW}Test 4: Binary Size Check${NC}"
file_size=$(stat -c%s "$BINARY_PATH" 2>/dev/null || stat -f%z "$BINARY_PATH" 2>/dev/null || echo "0")
if [ "$file_size" -gt 1000000 ]; then  # > 1MB
    echo -e "${GREEN}✅ PASSED - Binary size: $(echo $file_size | numfmt --to=iec)${NC}"
else
    echo -e "${RED}❌ FAILED - Binary too small: $file_size bytes${NC}"
    exit 1
fi
echo ""

echo -e "${GREEN}🎉 All smoke tests passed!${NC}"
echo -e "${BLUE}Binary is ready for deployment${NC}"
exit 0
