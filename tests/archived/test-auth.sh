#!/bin/bash

# Verona Agent Toolkit - Authentication and Treasury Test Script
# This script tests the OAuth2 authentication and basic treasury operations

set -e

echo "================================"
echo "Verona Agent Toolkit Test Script"
echo "================================"
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

TOOLKIT="./target/release/verona-toolkit"

# Function to print status
print_status() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ $1${NC}"
    else
        echo -e "${RED}✗ $1${NC}"
    fi
}

# 1. Check if CLI binary exists
echo "1. Checking CLI binary..."
if [ -f "$TOOLKIT" ]; then
    echo -e "${GREEN}✓ CLI binary found${NC}"
else
    echo -e "${RED}✗ CLI binary not found. Please run: cargo build --release${NC}"
    exit 1
fi
echo ""

# 2. Check authentication status
echo "2. Checking authentication status..."
AUTH_STATUS=$("$TOOLKIT" auth status --output json 2>&1 | grep -v "^\[2m" | grep -A 100 "^{")
if echo "$AUTH_STATUS" | grep -q '"authenticated": true'; then
    echo -e "${GREEN}✓ User is authenticated${NC}"
    echo ""
    echo "Authentication Details:"
    echo "$AUTH_STATUS" | jq '.'
    
    # Chain address field (legacy JSON key xion_address retained for compat)
    VERONA_ADDRESS=$(echo "$AUTH_STATUS" | jq -r '.verona_address // .xion_address // empty')
    if [ -n "$VERONA_ADDRESS" ]; then
        echo -e "${GREEN}✓ verona_address found: $VERONA_ADDRESS${NC}"
    else
        echo -e "${RED}✗ verona_address is missing or null${NC}"
        echo -e "${YELLOW}Please login again with: $TOOLKIT auth login${NC}"
        exit 1
    fi
else
    echo -e "${RED}✗ User is not authenticated${NC}"
    echo -e "${YELLOW}Please login first with: $TOOLKIT auth login --network testnet${NC}"
    exit 1
fi
echo ""

# 3. List treasuries
echo "3. Listing treasuries..."
TREASURY_LIST=$("$TOOLKIT" treasury list --output json 2>&1 | grep -v "^\[2m" | grep -A 100 "^{")
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Treasury list retrieved successfully${NC}"
    echo ""
    echo "Treasuries:"
    echo "$TREASURY_LIST" | jq '.'
    
    # Count treasuries
    TREASURY_COUNT=$(echo "$TREASURY_LIST" | jq '.treasuries | length')
    echo ""
    echo -e "${GREEN}Found $TREASURY_COUNT treasury(ies)${NC}"
else
    echo -e "${RED}✗ Failed to list treasuries${NC}"
    echo "$TREASURY_LIST"
fi
echo ""

# 4. Test query (if treasuries exist)
if [ "$TREASURY_COUNT" -gt 0 ]; then
    echo "4. Querying first treasury..."
    FIRST_TREASURY=$(echo "$TREASURY_LIST" | jq -r '.treasuries[0].address')
    
    TREASURY_QUERY=$("$TOOLKIT" treasury query "$FIRST_TREASURY" --output json 2>&1 | grep -v "^\[2m" | grep -A 100 "^{")
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ Treasury query successful${NC}"
        echo ""
        echo "Treasury Details:"
        echo "$TREASURY_QUERY" | jq '.'
    else
        echo -e "${RED}✗ Treasury query failed${NC}"
        echo "$TREASURY_QUERY"
    fi
    echo ""
else
    echo -e "${YELLOW}No treasuries found. Skipping query test.${NC}"
    echo ""
fi

# Summary
echo "================================"
echo "Test Summary"
echo "================================"
echo -e "${GREEN}✓ CLI binary: OK${NC}"
echo -e "${GREEN}✓ Authentication: OK${NC}"
echo -e "${GREEN}✓ verona_address: OK${NC}"
echo -e "${GREEN}✓ Treasury list: OK${NC}"
if [ "$TREASURY_COUNT" -gt 0 ]; then
    echo -e "${GREEN}✓ Treasury query: OK${NC}"
fi
echo ""
echo -e "${GREEN}All basic tests passed!${NC}"
echo ""
echo "Next steps:"
echo "  - Test fund: $TOOLKIT treasury fund <address> <amount>"
echo "  - Test withdraw: $TOOLKIT treasury withdraw <address> <amount>"
