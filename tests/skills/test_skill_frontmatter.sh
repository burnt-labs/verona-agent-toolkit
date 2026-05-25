#!/usr/bin/env bash
# Shell test wrapper for skill frontmatter validation (Codex-compatible).
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
VALIDATE="$SCRIPT_DIR/../../scripts/validate-skill-frontmatter.sh"

if [[ ! -x "$VALIDATE" ]]; then
  chmod +x "$VALIDATE"
fi

echo "=== Skill frontmatter validation ==="
"$VALIDATE"
