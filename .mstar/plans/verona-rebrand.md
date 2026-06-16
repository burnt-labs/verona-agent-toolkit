---
status: Todo
created_at: 2026-06-15
updated_at: 2026-06-15
priority: P0
complexity: High
effort: XL (6 batches)
---

# Verona Agent Toolkit — Full Rebrand

## Background

The product renames from **Xion Agent Toolkit** to **Verona Agent Toolkit** as part of the network rebrand to Verona. This repository must complete the full product-layer rename before [docs.verona.dev](https://docs.verona.dev/en/build-on-verona/tools/verona-toolkit.md) updates its Toolkit documentation (currently still references `xion-toolkit`).

Chain-level identifiers remain unchanged per [Rebrand: XION → Verona](https://docs.verona.dev/en/others/rebrand-from-xion.md): `xion1` addresses, `uxion` denom, chain IDs, OAuth `xion:*` scopes, `xiond`, RPC/indexer URLs, and contract addresses.

**Branch policy:** All work on `rebranding`; merge target `main` via PR after Batch 6.

**Naming SSOT:** [verona-rebrand-contract.md](../knowledge/verona-rebrand-contract.md)

---

## Scope decision

| Layer | Action |
|-------|--------|
| Product (CLI, crate, repo, skills, config, env, types, docs) | **Full rename** to Verona |
| Chain (addresses, denom, chain ID, OAuth scopes, xiond, RPC) | **Keep** XION identifiers |

Estimated impact: ~220+ files across Rust, skills, docs, CI, and tests.

---

## Batch 0 — Plan registration (this iteration)

- [x] Create `verona-rebrand-contract.md`
- [x] Create this plan file
- [x] Register `verona-rebrand` in `status.json`
- [x] Update `knowledge/README.md` index

No product code changes in Batch 0.

---

## Batch 1 — Rust core

**Manifest & build**

- `Cargo.toml`: package name, bin, lib, authors, description, keywords, repository URL
- `build.rs`: `VERONA_*` compile-time env (chain URLs unchanged)
- `release-please-config.json`: package name

**Config & credentials (highest risk)**

- `src/config/manager.rs`: `~/.verona-toolkit/` as SSOT; startup migration from `~/.xion-toolkit/`
- `src/config/credentials.rs`, `oauth_discovery.rs`: path updates
- `src/config/encryption.rs`: new salt prefix; dual-read for legacy files
- `src/config/schema.rs`: `verona_address` + serde alias

**CLI & errors**

- `src/cli/mod.rs`: `name = "verona-toolkit"`
- `src/shared/error.rs`: `VeronaError`, `VeronaErrorCode`, hint strings
- `src/utils/output.rs`, `src/main.rs`, all CLI handlers

**Verify:** `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-features`

---

## Batch 2 — CI / release / dist

- `.github/workflows/ci.yml`: binary name, `VERONA_CI_ENCRYPTION_KEY`, artifact `verona-toolkit-binary`
- `.github/workflows/release.yml`, `dist/build-setup.yml`
- `.env.example`: env var rename + comments
- Installer artifact names (dist templates if present in repo)

---

## Batch 3 — Skills

**Folder renames (7):** `xion-*` → `verona-*` via `git mv`

Per skill:

- `SKILL.md` frontmatter `name`, `metadata.requires/recommends`, triggers (Verona-primary)
- 24 scripts: `command -v verona-toolkit`, `VERONA_*` env
- 40 schemas: `verona-address` format, remediation text, field names
- `verona-dev/schemas/routing.json`: all skill targets
- 8 evals JSON files
- `.agents/AGENTS.md`, `validate-skill-frontmatter.sh` if hardcoded names

---

## Batch 4 — Documentation

- `README.md`, `INSTALL-FOR-AGENTS.md`: `verona-toolkit`, repo URL `verona-agent-toolkit`
- `AGENTS.md`, `CLAUDE.md`, `CONTRIBUTING.md`
- `docs/*.md` (7 files): mechanical replace + spot-check
- `examples/README.md`
- `CHANGELOG.md`: new breaking-change entry (do not rewrite history)

Doc rule: product/commands use Verona; chain examples keep `xion1...`, `uxion`, `xion-testnet-2` with "chain identifier" note.

---

## Batch 5 — Tests & migration

- `tests/skills/`: rename `test_xion_*.sh` → `test_verona_*.sh`; update mocks
- E2E scripts: binary invocations
- Migration tests: config dir, `xion_address` deserialization, `XION_*` env fallback
- Update `#[serial(encryption_key)]` tests for `VERONA_CI_ENCRYPTION_KEY`

---

## Batch 6 — Verify & deliver

```bash
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
tests/skills/run_all.sh
```

QC tri-review → PR `rebranding` → `main`

**Post-merge coordination:**

1. GitHub repo rename `xion-agent-toolkit` → `verona-agent-toolkit`
2. docs.verona.dev Toolkit page update
3. Notify `xion-skills` maintainers (non-blocking)

---

## Out of scope

- Chain protocol rename (`xion1`, `uxion`, chain ID, OAuth scopes, RPC, contract addresses)
- Upstream `xion-types` crate rename
- `burnt-labs/xion-skills` internal `xiond-*` skill renames (cross-reference text only)
- `.mstar/plans/archived/` historical plan bodies (optional sed, non-blocking)
- CHANGELOG historical entry rewrites

---

## Done criteria (full rebrand)

- [ ] Zero product-layer `xion-toolkit` / `xion-agent-toolkit` / `~/.xion-toolkit/` (archived plans excluded)
- [ ] `verona-toolkit --version` works; skills call `verona-toolkit`
- [ ] Seven `verona-*` skill folders
- [ ] `~/.xion-toolkit/` auto-migrates
- [ ] Chain examples valid (`xion1...`, `uxion`)
- [ ] `cargo test --all-features` green
- [ ] Contract doc available for docs team

---

## Risks

| Risk | Mitigation |
|------|------------|
| Credential migration failure | Startup migration + tests; manual fallback in docs |
| Encryption salt change | Dual salt read path |
| Skill global install breakage | CHANGELOG + reinstall guide |
| CI secrets rename | Sync GitHub secrets before Batch 2 |
| Bulk replace misses | Optional CI grep gate |
