#!/bin/bash
#
# verona-oauth2-client/sync-from-treasury.sh - Sync OAuth client from treasury
#
# Refreshes OAuth client redirect/branding from the bound on-chain treasury.
# Run after treasury update-params changes redirect_url or icon_url.
#
# Usage:
#   ./sync-from-treasury.sh --client-id <CLIENT_ID>
#
# Output:
#   JSON to stdout with updated client metadata

set -e

output_json() {
    echo "$1"
}

log_info() {
    echo "[INFO] $1" >&2
}

log_error() {
    echo "[ERROR] $1" >&2
}

handle_error() {
    local message="$1"
    local code="${2:-UNKNOWN_ERROR}"
    output_json "{\"success\": false, \"error\": \"$message\", \"error_code\": \"$code\"}"
    exit 1
}

CLIENT_ID=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --client-id)
            CLIENT_ID="$2"
            shift 2
            ;;
        --help|-h)
            echo "Usage: $0 --client-id <CLIENT_ID>" >&2
            echo "" >&2
            echo "Sync OAuth client metadata from the bound on-chain treasury." >&2
            echo "Run after treasury update-params changes redirect or branding." >&2
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            handle_error "Unknown option: $1" "INVALID_ARGUMENT"
            ;;
    esac
done

if [[ -z "$CLIENT_ID" ]]; then
    handle_error "Missing required argument: --client-id" "MISSING_CLIENT_ID"
fi

if ! command -v verona-toolkit &> /dev/null; then
    handle_error "verona-toolkit CLI not found in PATH." "CLI_NOT_FOUND"
fi

log_info "Syncing OAuth client from treasury: $CLIENT_ID"

STDERR_FILE=$(mktemp)
trap 'rm -f "$STDERR_FILE"' EXIT

set +e
RESULT=$(verona-toolkit --no-interactive oauth2 client sync-from-treasury "$CLIENT_ID" --output json 2>"$STDERR_FILE")
EXIT_CODE=$?
set -e

STDERR_CONTENT=""
if [[ -s "$STDERR_FILE" ]]; then
    STDERR_CONTENT=$(cat "$STDERR_FILE")
    echo "$STDERR_CONTENT" >&2
fi

if [ $EXIT_CODE -eq 0 ]; then
    output_json "$RESULT"
else
    ERROR_MSG="$RESULT"
    if [[ -z "$ERROR_MSG" ]]; then
        ERROR_MSG="$STDERR_CONTENT"
    fi
    log_error "Failed to sync client from treasury: $ERROR_MSG"
    if echo "$ERROR_MSG" | grep -qi "not authenticated"; then
        handle_error "Not authenticated. Run 'verona-toolkit auth login' first." "EOAUTHCLIENT008"
    elif echo "$ERROR_MSG" | grep -qi "insufficient scope"; then
        handle_error "Insufficient scope. Re-login with mgr scopes." "EOAUTHCLIENT010"
    elif echo "$ERROR_MSG" | grep -qi "PERMISSION_DENIED_VIEW_CLIENT\|access denied"; then
        handle_error "Client access denied. Verify client ID and mgr scopes." "EOAUTHCLIENT020"
    else
        handle_error "Failed to sync client: $ERROR_MSG" "SYNC_FROM_TREASURY_FAILED"
    fi
fi
