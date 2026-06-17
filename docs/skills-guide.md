# Skills Guide

> **For AI Agents**: Skills wrap `verona-toolkit` CLI for structured JSON output. For command reference, see [QUICK-REFERENCE.md](./QUICK-REFERENCE.md).

## Overview

Skills are bash scripts that provide AI Agents with structured, JSON-output capabilities:

- Output JSON to stdout (machine-readable)
- Progress messages to stderr (non-blocking)
- Consistent error codes with remediation hints
- Follow [Agent Skills](https://agentskills.io/) format

### Frontmatter (Codex / Cursor / Claude)

Loaders reject invalid `SKILL.md` files at startup. Keep frontmatter valid:

| Rule | Requirement |
|------|-------------|
| Delimiters | Start and end YAML with `---` on its own line |
| `description` | Quoted string (single or double quotes); **max 1024 characters** |
| Long triggers | Put keyword lists in a `## Triggers` section in the body, not in `description` |
| Extra docs | Command details, refresh-first notes, and examples belong in the body |

Repository layout: `skills/` contains only skill packages (`verona-dev/`, `verona-oauth2/`, `verona-treasury/`, …). Each package has `SKILL.md`, `schemas/`, and optional `scripts/`. There is no `skills/scripts/` directory.

| Script | Location | Purpose |
|--------|----------|---------|
| `validate-skill-frontmatter.sh` | repo `scripts/` (dev/CI only) | Codex-compatible `SKILL.md` checks (runs `cargo run --bin validate-skill-frontmatter`; requires Rust toolchain) |
| `validate-params.sh` | `skills/verona-dev/scripts/` | Validate parameters against any skill’s JSON schemas (sibling skills under the same install root) |
| `security-utils.sh` | `skills/verona-treasury/scripts/` | Confirmations and validation for fund/withdraw scripts |

Parameter validation requires **`verona-dev`** installed alongside the target skill (same parent directory as `~/.agents/skills/`). Treasury fund/withdraw scripts source `security-utils.sh` locally.

Validate locally before publishing or reinstalling:

```bash
./scripts/validate-skill-frontmatter.sh
```

After changing skills in this repo, reinstall global copies for Codex:

```bash
npx skills add burnt-labs/verona-agent-toolkit -g -y -a codex
```

## Installation

Install to global skills directory for all common agents (Cursor, Claude Code, Codex, OpenClaw):

```bash
# Install all verona-agent-toolkit skills
npx skills add burnt-labs/verona-agent-toolkit -g -y -a cursor -a claude-code -a codex -a openclaw

# Optional: xion-skills for xiond CLI operations
npx skills add burnt-labs/xion-skills -g -y -a cursor -a claude-code -a codex -a openclaw
```

## Available Skills

### verona-dev (Entry Point)

Unified entry point for Verona development. Routes to correct skill based on user needs.

**Decision Matrix:**

| User Needs | Skill | Why |
|------------|-------|-----|
| Login / Authentication | `verona-oauth2` | MetaAccount, gasless |
| Create / Manage Treasury | `verona-treasury` | Core functionality |
| Fund / Withdraw | `verona-treasury` | Gasless transactions |
| Authz / Fee Grant | `verona-treasury` | Specialized feature |
| Get testnet tokens | `verona-faucet` | Testnet development |
| Query chain data | `xiond-usage` | More powerful queries |
| Deploy CosmWasm | `xiond-wasm` | Contract developer tool |
| OAuth2 client management | `verona-oauth2-client` | Manager API operations |

### verona-toolkit-init

Install verona-toolkit CLI when not present.

```bash
bash /path/to/verona-toolkit-init/scripts/install.sh
```

### verona-oauth2

OAuth2 authentication commands.

| Script | Command |
|--------|---------|
| login | `verona-toolkit auth login` |
| status | `verona-toolkit auth status` |
| logout | `verona-toolkit auth logout` |
| refresh | `verona-toolkit auth refresh` |

### verona-treasury

Treasury management commands.

| Script | Command |
|--------|---------|
| list | `verona-toolkit treasury list` |
| query | `verona-toolkit treasury query <ADDR>` |
| create | `verona-toolkit treasury create --name "..." --redirect-url "..."` |
| fund | `verona-toolkit treasury fund <ADDR> --amount 1000000uxion` |
| withdraw | `verona-toolkit treasury withdraw <ADDR> --amount 500000uxion` |
| grant-config | `verona-toolkit treasury grant-config add/remove/list` |
| fee-config | `verona-toolkit treasury fee-config set/remove/query` |
| admin | `verona-toolkit treasury admin propose/accept/cancel` |
| export | `verona-toolkit treasury export <ADDR> --output backup.json` |
| import | `verona-toolkit treasury import <ADDR> --from-file backup.json` |

### verona-asset

NFT operations.

| Script | Command |
|--------|---------|
| types | `verona-toolkit asset types` |
| create | `verona-toolkit asset create --type cw721-base --name "..." --symbol "..."` |
| mint | `verona-toolkit asset mint --contract <ADDR> --token-id "1" --owner <ADDR>` |
| predict | `verona-toolkit asset predict --type cw721-base --name "..." --symbol "..." --salt "..."` |
| batch-mint | `verona-toolkit asset batch-mint --contract <ADDR> --tokens-file tokens.json` |
| query | `verona-toolkit asset query --contract <ADDR> --msg '{"...": {}}'` |

### verona-faucet

Claim testnet XION tokens from the faucet.

| Script | Command |
|--------|---------|
| claim | `verona-toolkit faucet claim` |
| claim (delegate) | `verona-toolkit faucet claim --receiver xion1...` |
| status | `verona-toolkit faucet status [--address xion1...]` |
| info | `verona-toolkit faucet info` |

**Details:**
- Amount: 1 XION (1,000,000 uxion) per claim
- Cooldown: 24 hours
- Balance gate: Receiver must have < 1 XION
- Network: Testnet only

### verona-oauth2-client

OAuth2 client lifecycle management via the Manager API.

| Script | Command |
|--------|---------|
| list | `verona-toolkit oauth2 client list` |
| create | `verona-toolkit oauth2 client create --redirect-uris "..." --treasury "..."` |
| get | `verona-toolkit oauth2 client get <CLIENT_ID>` |
| update | `verona-toolkit oauth2 client update <CLIENT_ID>` |
| delete | `verona-toolkit oauth2 client delete <CLIENT_ID> --force` |
| extension | `verona-toolkit oauth2 client extension get/update <CLIENT_ID>` |
| managers | `verona-toolkit oauth2 client managers add/remove <CLIENT_ID>` |
| transfer-ownership | `verona-toolkit oauth2 client transfer-ownership <CLIENT_ID>` |
| rotate-secret | `verona-toolkit oauth2 client rotate-secret <CLIENT_ID>` |

**Note:** Requires `--dev-mode` authentication for Manager API scopes.

## Output Format

**Success:**
```json
{"success": true, "data": "...", "tx_hash": "..."}
```

**Error:**
```json
{"success": false, "error": "...", "error_code": "...", "hint": "..."}
```

## Common Error Codes

| Code | Fix |
|------|-----|
| `CLI_NOT_FOUND` | Install CLI: `curl ... | sh` |
| `NOT_AUTHENTICATED` | Run `verona-toolkit auth login` |
| `TOKEN_EXPIRED` | Run `verona-toolkit auth refresh` |
| `TREASURY_NOT_FOUND` | Verify address and network |
| `INSUFFICIENT_BALANCE` | Fund the treasury/account |
| `PORT_IN_USE` | Use `--port` flag |

> Full error reference: [ERROR-CODES.md](./ERROR-CODES.md)

## Best Practices

### 1. Check Authentication First
```bash
AUTH_STATUS=$(verona-toolkit auth status --output json)
if [[ $(echo "$AUTH_STATUS" | jq -r '.authenticated') != "true" ]]; then
    verona-toolkit auth login
fi
```

### 2. Parse JSON Output
```bash
RESULT=$(verona-toolkit treasury list --output json)
if [[ $(echo "$RESULT" | jq -r '.success') == "true" ]]; then
    echo "$RESULT" | jq '.treasuries'
fi
```

### 3. Use Network Flag
```bash
verona-toolkit treasury list --network mainnet
```

## xion-skills vs verona-agent-toolkit

| Feature | verona-agent-toolkit | xion-skills |
|---------|-------------------|-------------|
| **Target CLI** | verona-toolkit | xiond |
| **Auth Method** | OAuth2 (gasless) | Mnemonic/Keyring |
| **Use Case** | MetaAccount, Treasury | Chain queries, CosmWasm |

**Recommendation**: Use `verona-agent-toolkit` for most Verona development. Use `xion-skills` for advanced chain operations.

## Resources

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/burnt-labs/verona-agent-toolkit |
| Agent Skills | https://agentskills.io/ |
| Verona Docs | https://docs.verona.dev |
| Developer Portal | https://dev.testnet2.burnt.com |

---

*Document Version: 2.0.0*
*Last Updated: 2026-03-14*
