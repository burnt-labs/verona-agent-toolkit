# Skills Guide

> **For AI Agents**: Agent skills live in [burnt-labs/verona-dev-plugin](https://github.com/burnt-labs/verona-dev-plugin). This repository ships only the `verona-toolkit` CLI. For command reference, see [QUICK-REFERENCE.md](./QUICK-REFERENCE.md).

## Where skills live

| Item | Location |
|------|----------|
| Skills SSOT | [github.com/burnt-labs/verona-dev-plugin](https://github.com/burnt-labs/verona-dev-plugin) |
| Install SSOT | [INSTALL.md](https://github.com/burnt-labs/verona-dev-plugin/blob/main/INSTALL.md) (Cursor, Codex, Claude Code, Kimi) |
| Session entry | `verona-dev` skill (auto-loaded when plugin hooks are active) |
| CLI in this repo | `verona-toolkit` — consumed by toolkit leaf skills |

There is **no** `npx skills add burnt-labs/verona-agent-toolkit` path and **no** bundled `skills/` tree in this repository.

## Install (summary)

Clone or install the plugin per your host. Example for **Cursor** (local path):

```bash
mkdir -p ~/.cursor/plugins/local
git clone https://github.com/burnt-labs/verona-dev-plugin.git ~/.cursor/plugins/local/verona-dev-plugin
```

Reload Cursor after install. For Codex, Kimi, Claude Code, and marketplace flows, follow the host-specific steps in the plugin [INSTALL.md](https://github.com/burnt-labs/verona-dev-plugin/blob/main/INSTALL.md) — do not invent install steps from this page.

Install the **CLI** separately (curl installer or `cargo install` from this repo). Skills wrap `verona-toolkit`; the CLI must be on `PATH` before toolkit workflows run.

## Routing (Development vs Toolkit)

The plugin README defines two lanes. Load **`verona-dev`** first; it routes into the correct skill.

### Development

Use when building or integrating apps, or when you need chain / CosmWasm / `xiond` help.

| Skill | Role |
|-------|------|
| **`verona-dev`** | Entry router + application guides in [verona-dev-plugin](https://github.com/burnt-labs/verona-dev-plugin): [`skills/verona-dev/references/`](https://github.com/burnt-labs/verona-dev-plugin/tree/main/skills/verona-dev/references) (xion.js / Abstraxion, mob, oauth2-api-service) |
| **`verona-bin`** | `xiond` install, chain queries/txs, CosmWasm — use when the task needs the binary, not the app SDK |

For chain-level queries, contract deployment, and mnemonic-wallet work, use **`verona-bin`** inside the plugin. Do **not** install a separate `xion-skills` package.

### Toolkit

Use when operating MetaAccount flows through the **verona-toolkit** CLI.

| Skill | CLI area |
|-------|----------|
| `verona-toolkit-init` | Install / set up the toolkit CLI |
| `verona-oauth2` | Login / MetaAccount auth |
| `verona-oauth2-client` | OAuth app / client CRUD (Manager API) |
| `verona-treasury` | Treasury create / fund / grants |
| `verona-asset` | NFT / asset operations |
| `verona-faucet` | Testnet tokens |

Do not mix lanes: app SDK setup stays in `verona-dev` references; toolkit command workflows stay in the leaf skills above.

## How skills relate to this CLI

Toolkit leaf skills are bash wrappers around `verona-toolkit` commands. They:

- Emit JSON to stdout (machine-readable)
- Send progress to stderr
- Return consistent error codes with remediation hints

You can use the CLI directly without skills. Skills add routing, parameter schemas, and agent-friendly scripts on top of the same commands documented in [cli-reference.md](./cli-reference.md).

**Authentication first:** Before treasury, faucet, or NFT operations, complete `verona-oauth2` / `verona-toolkit-init` flows (or run `verona-toolkit auth login`).

## Output format

**Success:**

```json
{"success": true, "data": "...", "tx_hash": "..."}
```

**Error:**

```json
{"success": false, "error": "...", "error_code": "...", "hint": "..."}
```

> Full error reference: [ERROR-CODES.md](./ERROR-CODES.md)

## Resources

| Resource | URL |
|----------|-----|
| verona-dev-plugin | https://github.com/burnt-labs/verona-dev-plugin |
| Plugin INSTALL.md | https://github.com/burnt-labs/verona-dev-plugin/blob/main/INSTALL.md |
| verona-toolkit CLI (this repo) | https://github.com/burnt-labs/verona-agent-toolkit |
| Agent install guide | [INSTALL-FOR-AGENTS.md](../INSTALL-FOR-AGENTS.md) |
| Verona Docs | https://docs.verona.dev |

---

*Document Version: 3.0.0*  
*Last Updated: 2026-07-21*
