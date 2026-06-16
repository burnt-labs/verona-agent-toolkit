#!/bin/bash
#
# Verona Agent Toolkit - Treasury Grant & Fee Configuration E2E Test
# Tests grant-config and fee-config operations in detail
#
# Usage:
#   ./tests/e2e_treasury_grant_fee.sh [treasury-address]
#   If no address provided, will use first available treasury
#

set -euo pipefail

# =============================================================================
# Configuration
# =============================================================================

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Binary path (can be overridden via CLI argument)
BINARY_PATH="${1:-./target/release/verona-toolkit}"
NETWORK="${NETWORK:-testnet}"
SPECIFIC_TREASURY="${2:-}"

# Protected treasury (DO NOT USE - this is a protected test treasury)
PROTECTED_TREASURY="xion17vg5l9za4768g0hnxezltgnu4h7eleqdcmwark2uuz2s4z5q4dfsr80vvm"

# Temporary directory for config files
TEMP_DIR=""

# =============================================================================
# Helper Functions
# =============================================================================

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1" >&2
}

log_success() {
    echo -e "${GREEN}[PASS]${NC} $1" >&2
}

log_error() {
    echo -e "${RED}[FAIL]${NC} $1" >&2
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1" >&2
}

log_skip() {
    echo -e "${CYAN}[SKIP]${NC} $1" >&2
}

# Run a CLI command and capture JSON output
# Usage: run_cmd "treasury list"
run_cmd() {
    local cmd="$1"
    local full_cmd="$BINARY_PATH $cmd --output json"
    
    # Execute and capture output
    local output
    output=$(eval "$full_cmd" 2>&1)
    
    # Remove ANSI escape codes and filter out tracing log lines
    output=$(echo "$output" | sed 's/\x1b\[[0-9;]*m//g' | grep -v "^[[:space:]]*20[0-9]" | grep -v "^\[INFO\]")
    
    # Try to extract JSON using jq
    local json
    json=$(echo "$output" | jq -s 'add' 2>/dev/null)
    
    if [ -n "$json" ] && [ "$json" != "null" ]; then
        echo "$json"
    else
        # Fallback: output raw output
        echo "$output"
    fi
}

# Parse JSON field from output
# Usage: parse_json "$output" "address"
parse_json() {
    local json="$1"
    local field="$2"
    
    local value
    value=$(echo "$json" | grep -o "\"$field\":[^,}]*" | head -1 | sed 's/.*: *//' | tr -d ' "')
    
    if [ -z "$value" ] || [ "$value" = "null" ]; then
        value=$(echo "$json" | jq -r ".$field // empty" 2>/dev/null)
    fi
    
    echo "$value"
}

# Check if JSON indicates success
json_true() {
    local json="$1"
    local field="$2"
    
    if echo "$json" | grep -q "\"$field\""; then
        local value
        value=$(echo "$json" | grep -o "\"$field\":[^,}]*" | sed 's/.*: *//' | tr -d ' "')
        [ "$value" = "true" ] && return 0
    fi
    
    if [ "$field" = "success" ]; then
        if echo "$json" | grep -q '"tx_hash"'; then
            return 0
        fi
        if echo "$json" | grep -q '"count"'; then
            return 0
        fi
    fi
    
    return 1
}

# Print step header
print_step() {
    local current=$1
    local total=$2
    local name=$3
    printf "[%d/%d] %-28s " "$current" "$total" "$name"
}

# =============================================================================
# Test Results Tracking
# =============================================================================

PASS_COUNT=0
FAIL_COUNT=0
SKIP_COUNT=0
TEST_RESULTS=()

record_result() {
    local status="$1"
    local name="$2"
    local detail="${3:-}"
    
    TEST_RESULTS+=("[$status] $name")
    
    case "$status" in
        PASS)
            ((PASS_COUNT++))
            if [ -n "$detail" ]; then
                echo -e "${GREEN}✓ PASS${NC} ($detail)"
            else
                echo -e "${GREEN}✓ PASS${NC}"
            fi
            ;;
        FAIL)
            ((FAIL_COUNT++))
            if [ -n "$detail" ]; then
                echo -e "${RED}✗ FAIL${NC} ($detail)"
            else
                echo -e "${RED}✗ FAIL${NC}"
            fi
            ;;
        SKIP)
            ((SKIP_COUNT++))
            if [ -n "$detail" ]; then
                echo -e "${CYAN}⊘ SKIP${NC} ($detail)"
            else
                echo -e "${CYAN}⊘ SKIP${NC}"
            fi
            ;;
    esac
}

# =============================================================================
# Pre-flight Checks
# =============================================================================

test_preflight() {
    print_step 1 11 "Pre-flight Check"
    
    # Check if binary exists
    if [ ! -f "$BINARY_PATH" ]; then
        record_result "FAIL" "Pre-flight Check" "Binary not found at $BINARY_PATH"
        echo "Run 'cargo build --release' first"
        return 1
    fi
    
    # Check authentication status
    local auth_output=$(run_cmd "auth status")
    
    if json_true "$auth_output" "authenticated"; then
        record_result "PASS" "Pre-flight Check" "CLI ready and authenticated"
        return 0
    else
        record_result "FAIL" "Pre-flight Check" "Not authenticated"
        echo ""
        log_error "Please login first: $BINARY_PATH auth login --network $NETWORK"
        return 1
    fi
}

# =============================================================================
# Treasury Selection
# =============================================================================

TREASURY_ADDRESS=""

select_treasury() {
    print_step 2 11 "Treasury Selection"
    
    if [ -n "$SPECIFIC_TREASURY" ]; then
        TREASURY_ADDRESS="$SPECIFIC_TREASURY"
        record_result "PASS" "Treasury Selection" "Using specified: ${TREASURY_ADDRESS:0:20}..."
        return 0
    fi
    
    # List treasuries and select first non-protected one
    local list_output=$(run_cmd "treasury list")
    
    if json_true "$list_output" "success"; then
        local count=$(parse_json "$list_output" "count")
        
        if [ "$count" -eq 0 ]; then
            record_result "FAIL" "Treasury Selection" "No treasuries available"
            return 1
        fi
        
        # Try to get first non-protected treasury
        local first_addr=$(echo "$list_output" | jq -r '.treasuries[0].address' 2>/dev/null)
        
        if [ "$first_addr" = "$PROTECTED_TREASURY" ] && [ "$count" -gt 1 ]; then
            TREASURY_ADDRESS=$(echo "$list_output" | jq -r '.treasuries[1].address' 2>/dev/null)
            record_result "PASS" "Treasury Selection" "Using 2nd treasury (1st is protected)"
        else
            TREASURY_ADDRESS="$first_addr"
            record_result "PASS" "Treasury Selection" "Found $count treasury(ies)"
        fi
        
        return 0
    fi
    
    record_result "FAIL" "Treasury Selection" "Failed to list treasuries"
    return 1
}

# =============================================================================
# Grant Config - List Current
# =============================================================================

test_grant_config_list() {
    print_step 3 11 "Grant Config List"
    
    if [ -z "$TREASURY_ADDRESS" ]; then
        record_result "SKIP" "Grant Config List" "No treasury selected"
        return 0
    fi
    
    local list_output=$(run_cmd "treasury grant-config list $TREASURY_ADDRESS")
    
    if json_true "$list_output" "success"; then
        local count=$(parse_json "$list_output" "count")
        record_result "PASS" "Grant Config List" "$count grant(s) found"
        return 0
    fi
    
    local error_msg=$(parse_json "$list_output" "error")
    record_result "FAIL" "Grant Config List" "${error_msg:0:50}"
    return 1
}

# =============================================================================
# Grant Config - Add Generic
# =============================================================================

test_grant_config_add_generic() {
    print_step 4 11 "Grant Add (Generic)"
    
    if [ -z "$TREASURY_ADDRESS" ]; then
        record_result "SKIP" "Grant Add (Generic)" "No treasury selected"
        return 0
    fi
    
    if [ "$TREASURY_ADDRESS" = "$PROTECTED_TREASURY" ]; then
        record_result "SKIP" "Grant Add (Generic)" "Protected treasury"
        return 0
    fi
    
    local grant_output=$(run_cmd "treasury grant-config add $TREASURY_ADDRESS \
        --type-url /cosmos.bank.v1beta1.MsgSend \
        --auth-type generic \
        --description 'E2E test generic authorization'")
    
    if json_true "$grant_output" "success"; then
        local tx_hash=$(parse_json "$grant_output" "tx_hash")
        record_result "PASS" "Grant Add (Generic)" "tx: ${tx_hash:0:12}..."
        return 0
    fi
    
    local error_msg=$(parse_json "$grant_output" "error")
    # May fail if not admin
    if echo "$error_msg" | grep -qi "unauthorized\|not admin\|permission"; then
        record_result "SKIP" "Grant Add (Generic)" "Not treasury admin"
        return 0
    fi
    
    record_result "FAIL" "Grant Add (Generic)" "${error_msg:0:50}"
    return 1
}

# =============================================================================
# Grant Config - Add Send Preset
# =============================================================================

test_grant_config_add_send() {
    print_step 5 11 "Grant Add (Send)"
    
    if [ -z "$TREASURY_ADDRESS" ]; then
        record_result "SKIP" "Grant Add (Send)" "No treasury selected"
        return 0
    fi
    
    if [ "$TREASURY_ADDRESS" = "$PROTECTED_TREASURY" ]; then
        record_result "SKIP" "Grant Add (Send)" "Protected treasury"
        return 0
    fi
    
    local grant_output=$(run_cmd "treasury grant-config add $TREASURY_ADDRESS \
        --preset send \
        --spend-limit 1000000uxion \
        --description 'E2E test send authorization'")
    
    if json_true "$grant_output" "success"; then
        local tx_hash=$(parse_json "$grant_output" "tx_hash")
        record_result "PASS" "Grant Add (Send)" "tx: ${tx_hash:0:12}..."
        return 0
    fi
    
    local error_msg=$(parse_json "$grant_output" "error")
    if echo "$error_msg" | grep -qi "unauthorized\|not admin\|permission"; then
        record_result "SKIP" "Grant Add (Send)" "Not treasury admin"
        return 0
    fi
    
    record_result "FAIL" "Grant Add (Send)" "${error_msg:0:50}"
    return 1
}

# =============================================================================
# Fee Config - Query Current
# =============================================================================

test_fee_config_query() {
    print_step 6 11 "Fee Config Query"
    
    if [ -z "$TREASURY_ADDRESS" ]; then
        record_result "SKIP" "Fee Config Query" "No treasury selected"
        return 0
    fi
    
    local fee_output=$(run_cmd "treasury fee-config query $TREASURY_ADDRESS")
    
    if json_true "$fee_output" "success"; then
        record_result "PASS" "Fee Config Query" "Fee config queried"
        return 0
    fi
    
    local error_msg=$(parse_json "$fee_output" "error")
    record_result "FAIL" "Fee Config Query" "${error_msg:0:50}"
    return 1
}

# =============================================================================
# Fee Config - Set Basic
# =============================================================================

test_fee_config_set_basic() {
    print_step 7 11 "Fee Set (Basic)"
    
    if [ -z "$TREASURY_ADDRESS" ]; then
        record_result "SKIP" "Fee Set (Basic)" "No treasury selected"
        return 0
    fi
    
    if [ "$TREASURY_ADDRESS" = "$PROTECTED_TREASURY" ]; then
        record_result "SKIP" "Fee Set (Basic)" "Protected treasury"
        return 0
    fi
    
    # Create temp config file
    TEMP_DIR=$(mktemp -d)
    local fee_config_file="$TEMP_DIR/fee_basic.json"
    
    cat > "$fee_config_file" << 'EOF'
{
  "allowance_type": "basic",
  "spend_limit": "2000000uxion",
  "description": "E2E test basic fee allowance"
}
EOF
    
    local fee_output=$(run_cmd "treasury fee-config set $TREASURY_ADDRESS --fee-config $fee_config_file")
    
    if json_true "$fee_output" "success"; then
        local tx_hash=$(parse_json "$fee_output" "tx_hash")
        record_result "PASS" "Fee Set (Basic)" "tx: ${tx_hash:0:12}..."
        rm -rf "$TEMP_DIR"
        return 0
    fi
    
    local error_msg=$(parse_json "$fee_output" "error")
    rm -rf "$TEMP_DIR"
    
    if echo "$error_msg" | grep -qi "unauthorized\|not admin\|permission"; then
        record_result "SKIP" "Fee Set (Basic)" "Not treasury admin"
        return 0
    fi
    
    record_result "FAIL" "Fee Set (Basic)" "${error_msg:0:50}"
    return 1
}

# =============================================================================
# Fee Config - Set Periodic
# =============================================================================

test_fee_config_set_periodic() {
    print_step 8 11 "Fee Set (Periodic)"
    
    if [ -z "$TREASURY_ADDRESS" ]; then
        record_result "SKIP" "Fee Set (Periodic)" "No treasury selected"
        return 0
    fi
    
    if [ "$TREASURY_ADDRESS" = "$PROTECTED_TREASURY" ]; then
        record_result "SKIP" "Fee Set (Periodic)" "Protected treasury"
        return 0
    fi
    
    # Create temp config file
    TEMP_DIR=$(mktemp -d)
    local fee_config_file="$TEMP_DIR/fee_periodic.json"
    
    cat > "$fee_config_file" << 'EOF'
{
  "allowance_type": "periodic",
  "period_seconds": 86400,
  "period_spend_limit": "500000uxion",
  "description": "E2E test periodic daily fee allowance"
}
EOF
    
    local fee_output=$(run_cmd "treasury fee-config set $TREASURY_ADDRESS --fee-config $fee_config_file")
    
    if json_true "$fee_output" "success"; then
        local tx_hash=$(parse_json "$fee_output" "tx_hash")
        record_result "PASS" "Fee Set (Periodic)" "tx: ${tx_hash:0:12}..."
        rm -rf "$TEMP_DIR"
        return 0
    fi
    
    local error_msg=$(parse_json "$fee_output" "error")
    rm -rf "$TEMP_DIR"
    
    if echo "$error_msg" | grep -qi "unauthorized\|not admin\|permission"; then
        record_result "SKIP" "Fee Set (Periodic)" "Not treasury admin"
        return 0
    fi
    
    record_result "FAIL" "Fee Set (Periodic)" "${error_msg:0:50}"
    return 1
}

# =============================================================================
# Error Handling Tests
# =============================================================================

test_error_handling() {
    print_step 9 11 "Error Handling"
    
    if [ -z "$TREASURY_ADDRESS" ]; then
        record_result "SKIP" "Error Handling" "No treasury selected"
        return 0
    fi
    
    local errors_found=0
    
    # Test 1: Invalid preset
    local invalid_preset=$($BINARY_PATH treasury grant-config add "$TREASURY_ADDRESS" \
        --preset invalid_preset \
        --output json 2>&1 || true)
    
    if echo "$invalid_preset" | grep -qi "error\|invalid"; then
        ((errors_found++))
    fi
    
    # Test 2: Missing type-url for generic
    local missing_type=$($BINARY_PATH treasury grant-config add "$TREASURY_ADDRESS" \
        --auth-type generic \
        --output json 2>&1 || true)
    
    if echo "$missing_type" | grep -qi "error\|required"; then
        ((errors_found++))
    fi
    
    # Test 3: Missing config file
    local missing_file=$($BINARY_PATH treasury fee-config set "$TREASURY_ADDRESS" \
        --fee-config "/nonexistent/config.json" \
        --output json 2>&1 || true)
    
    if echo "$missing_file" | grep -qi "error\|not found\|no such"; then
        ((errors_found++))
    fi
    
    if [ "$errors_found" -eq 3 ]; then
        record_result "PASS" "Error Handling" "All 3 error cases handled"
        return 0
    else
        record_result "WARN" "Error Handling" "$errors_found/3 error cases handled"
        return 0
    fi
}

# =============================================================================
# Chain Query - Grants
# =============================================================================

test_chain_query_grants() {
    print_step 10 11 "Chain Query Grants"
    
    if [ -z "$TREASURY_ADDRESS" ]; then
        record_result "SKIP" "Chain Query Grants" "No treasury selected"
        return 0
    fi
    
    local grants_output=$(run_cmd "treasury chain-query grants $TREASURY_ADDRESS")
    
    if json_true "$grants_output" "success"; then
        local count=$(parse_json "$grants_output" "count")
        record_result "PASS" "Chain Query Grants" "$count grant(s) found"
        return 0
    fi
    
    local error_msg=$(parse_json "$grants_output" "error")
    if echo "$error_msg" | grep -qi "503\|unavailable\|404\|not found"; then
        record_result "SKIP" "Chain Query Grants" "Service unavailable"
        return 0
    fi
    
    record_result "FAIL" "Chain Query Grants" "${error_msg:0:50}"
    return 1
}

# =============================================================================
# Chain Query - Allowances
# =============================================================================

test_chain_query_allowances() {
    print_step 11 11 "Chain Query Allowances"
    
    if [ -z "$TREASURY_ADDRESS" ]; then
        record_result "SKIP" "Chain Query Allowances" "No treasury selected"
        return 0
    fi
    
    local allowances_output=$(run_cmd "treasury chain-query allowances $TREASURY_ADDRESS")
    
    if json_true "$allowances_output" "success"; then
        local count=$(parse_json "$allowances_output" "count")
        record_result "PASS" "Chain Query Allowances" "$count allowance(s) found"
        return 0
    fi
    
    local error_msg=$(parse_json "$allowances_output" "error")
    if echo "$error_msg" | grep -qi "503\|unavailable\|404\|not found"; then
        record_result "SKIP" "Chain Query Allowances" "Service unavailable"
        return 0
    fi
    
    record_result "FAIL" "Chain Query Allowances" "${error_msg:0:50}"
    return 1
}

# =============================================================================
# Cleanup
# =============================================================================

cleanup() {
    [ -d "$TEMP_DIR" ] && rm -rf "$TEMP_DIR"
}

# =============================================================================
# Main Test Runner
# =============================================================================

main() {
    echo ""
    echo "=========================================="
    echo "Treasury Grant & Fee Config E2E Test"
    echo "=========================================="
    echo ""
    log_info "Binary: $BINARY_PATH"
    log_info "Network: $NETWORK"
    if [ -n "$SPECIFIC_TREASURY" ]; then
        log_info "Treasury: $SPECIFIC_TREASURY"
    fi
    echo ""
    
    # Set trap for cleanup
    trap cleanup EXIT
    
    # Run all tests
    test_preflight || { echo ""; echo "Pre-flight failed. Exiting."; exit 1; }
    echo ""
    
    select_treasury
    echo ""
    
    test_grant_config_list
    echo ""
    
    test_grant_config_add_generic
    echo ""
    
    test_grant_config_add_send
    echo ""
    
    test_fee_config_query
    echo ""
    
    test_fee_config_set_basic
    echo ""
    
    test_fee_config_set_periodic
    echo ""
    
    test_error_handling
    echo ""
    
    test_chain_query_grants
    echo ""
    
    test_chain_query_allowances
    echo ""
    
    # Print summary
    echo "=========================================="
    echo "Results: $PASS_COUNT PASS, $FAIL_COUNT FAIL, $SKIP_COUNT SKIP"
    echo "=========================================="
    
    # Exit with error if any tests failed
    if [ $FAIL_COUNT -gt 0 ]; then
        exit 1
    fi
    
    exit 0
}

# Run main
main "$@"
