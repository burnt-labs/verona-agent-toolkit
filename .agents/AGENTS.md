# AGENTS.md — Agent skill context (`.agents/`)

Rules for how Codex, Cursor, and similar coding agents use **Verona skills** alongside this repository.

## What this directory is for

The repo keeps `.agents/` as a **conventional anchor** next to global skill install layouts (`~/.agents/skills/`, host plugin trees). It holds **no** source code, plans, or review artifacts—only this guide for skill discovery.

**Agent skills do not live in this repository.** The skills SSOT is [burnt-labs/verona-dev-plugin](https://github.com/burnt-labs/verona-dev-plugin). Install and update skills only via that plugin’s [INSTALL.md](https://github.com/burnt-labs/verona-dev-plugin/blob/main/INSTALL.md).

## Install and discovery

| Step | Location |
|------|----------|
| Plugin install (SSOT) | [verona-dev-plugin INSTALL.md](https://github.com/burnt-labs/verona-dev-plugin/blob/main/INSTALL.md) |
| CLI + agent workflow | `INSTALL-FOR-AGENTS.md` (repository root) |
| Skills routing overview | `docs/skills-guide.md` |
| Plugin README (Development vs Toolkit) | [verona-dev-plugin README](https://github.com/burnt-labs/verona-dev-plugin/blob/main/README.md) |

Do **not** use `npx skills add burnt-labs/verona-agent-toolkit` or install `burnt-labs/xion-skills` for Verona agent workflows. Chain / CosmWasm / `xiond` tasks route to **`verona-bin`** inside the plugin.

## Bundled skills (plugin)

Session entry: **`verona-dev`**. Toolkit leaf skills: `verona-toolkit-init`, `verona-oauth2`, `verona-oauth2-client`, `verona-treasury`, `verona-faucet`, `verona-asset`. Development lane: `verona-dev` references + **`verona-bin`**.

Before treasury, faucet, or NFT operations: authenticate per `verona-oauth2` / `verona-toolkit-init` (or `verona-toolkit auth login`).

## Skill conventions (plugin)

- Skill packages live under `skills/` in **verona-dev-plugin**; each has `SKILL.md`, optional `scripts/`, and `schemas/`.
- Scripts invoked by skills should use `--output json` and structured stderr on failure (see root `AGENTS.md` CLI output rules).
- Skill authoring and frontmatter validation happen in the plugin repository, not here.

## Rust / CLI work in this repo

Use repository root **`AGENTS.md`** for `cargo` workflows, error codes, test serialization, and credential safety.
