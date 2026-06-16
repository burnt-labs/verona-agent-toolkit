---
report_kind: qc-review
reviewer: qc-specialist
reviewer_index: 1
plan_id: verona-rebrand
verdict: Approve with residuals
generated_at: 2026-06-16T00:00:00Z
---

# Code Review Report

## Reviewer Metadata
- Reviewer: @qc-specialist
- Runtime Agent ID: qc-specialist
- Runtime Model: composer-2.5-fast
- Review Perspective: Architecture & maintainability (crate rename ripple, config module split, migration design, error rename consistency, scope vs contract)
- Report Timestamp: 2026-06-16T00:00:00Z

## Scope
- plan_id: verona-rebrand
- Review range / Diff basis: 893c9f03724fa2e69adae81d4ee419432fba3b57..c6d93a86a8e53624c69c052a00a419bac87ac83f
- Working branch (verified): rebranding
- Review cwd (verified): /Users/bibi/workspace/burnt-labs/agent-toolkit
- Files reviewed: 182 (per diff stat)
- Commit range (if not identical to Review range line, explain): identical — 10 commits in range
- Tools run: `git branch --show-current`, `git diff --stat`, `cargo fmt -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --lib` (migration/compat sample)

## Findings

### 🔴 Critical
- None.

### 🟡 Warning
- **W1 — Skill-layer env compat incomplete vs contract** → Contract § Compatibility strategy requires `XION_*` fallback with deprecation warning for all mapped env vars (including `VERONA_SKILLS_DIR`, `VERONA_AUDIT_LOG`, `VERONA_SKIP_CONFIRM`). Rust core covers network override and CI encryption via `env_compat.rs`; `build.rs` covers OAuth build-time vars. Skill bash scripts (`skills/verona-dev/scripts/validate-params.sh`, `skills/verona-treasury/scripts/security-utils.sh`, fund/withdraw scripts) read only `VERONA_*` with no `XION_*` fallback. Users with existing CI/shell exports of `XION_SKIP_CONFIRM` / `XION_AUDIT_LOG` will silently lose behavior. **Fix:** add a small shared bash helper (e.g. `read_env_with_legacy PRIMARY LEGACY`) or inline `${VERONA_SKIP_CONFIRM:-${XION_SKIP_CONFIRM:-false}}` pattern in affected scripts; document in CHANGELOG if deferred.

- **W2 — Config migration skips when target already exists** → `paths.rs::migrate_legacy_config_dir` no-ops when `~/.verona-toolkit/` exists even if legacy `~/.xion-toolkit/` also exists (test `test_migrate_skips_when_new_dir_exists` encodes this). A user who created an empty new dir before first run will not auto-merge legacy credentials. **Fix:** document manual merge in migration docs, or detect empty new dir + non-empty legacy and merge/warn (follow-up).

### 🟢 Suggestion
- **S1 — Duplicated env-compat logic** → Runtime uses `env_var_with_legacy` in `src/config/env_compat.rs`; build-time uses parallel `build_env()` in `build.rs`. Consider extracting a shared convention doc or macro list to avoid drift when adding env vars.
- **S2 — Stale local identifiers after rename** → Test code still names variables `xion_err` while typing `VeronaError` (`src/cli/oauth2_client.rs` ~972, `src/utils/output.rs` ~451). Cosmetic but reduces grep signal for incomplete renames.
- **S3 — Harness knowledge docs still product-stale** → `.mstar/knowledge/feature-roadmap.md`, `faucet-contract-research.md`, `treasury-create-design.md`, `xion-skills-update-plan.md` still reference `xion-toolkit` / `xion-agent-toolkit`. Plan excludes `.mstar/plans/archived/` from done criteria but not knowledge; agents reading harness knowledge may get wrong CLI names.
- **S4 — Optional CI grep gate not added** → Contract risk table mentions optional CI grep for product-layer `xion-toolkit`; not present in `.github/workflows/ci.yml`. Low priority post-merge hardening.
- **S5 — `get_current_network` uses `Box::leak`** → Pre-existing pattern retained in `manager.rs` for override lifetime. Consider `OnceLock` or storing override on `ConfigManager` to avoid intentional leak (non-blocking).

## Source Trace
| Finding ID | Source Type | Source Reference | Confidence |
|------------|-------------|------------------|------------|
| W1 | git-diff + manual-reasoning | `skills/verona-treasury/scripts/security-utils.sh`, contract § Environment variable mapping | High |
| W2 | git-diff + manual-reasoning | `src/config/paths.rs:44-46`, `test_migrate_skips_when_new_dir_exists` | High |
| S1 | manual-reasoning | `src/config/env_compat.rs`, `build.rs:15-25` | High |
| S2 | git-diff | `src/cli/oauth2_client.rs`, `src/utils/output.rs` | High |
| S3 | grep | `.mstar/knowledge/*.md` | High |
| S4 | doc-rule | contract Risks table | Medium |
| S5 | git-diff | `src/config/manager.rs:57-64` | Medium |

## Architecture Assessment

### Plan alignment (positive)
- **Crate rename ripple:** `Cargo.toml` → `verona-agent-toolkit` / `verona_agent_toolkit` / `verona-toolkit`; no remaining `XionError`, `xion_agent_toolkit`, or product-layer `xion-toolkit` in `src/`, `skills/`, `tests/` (legacy strings confined to intentional compat constants).
- **Config module split:** Clean extraction — `paths.rs` owns directory SSOT + migration; `env_compat.rs` owns runtime legacy env reads; `manager.rs` delegates to `config_dir()` instead of inline path logic. Module surface in `mod.rs` is minimal and coherent.
- **Migration design:** Rename-first with copy+delete fallback; dual salt decrypt in `encryption.rs`; serde alias for `xion_address` → `verona_address`. Unit tests cover rename, legacy field, legacy salt, and env fallback.
- **Error rename:** `VeronaError` / `VeronaErrorCode` / `anyhow_to_verona_error`; remediation hints updated to `verona-toolkit` commands.
- **Chain-layer KEEP:** Bech32 `xion1`, `uxion`, OAuth `xion:*` scopes preserved in code and docs per contract.
- **Skills:** Seven `verona-*` folders present; scripts invoke `verona-toolkit`.

### Static analysis / sample tests
- `cargo fmt -- --check`: pass
- `cargo clippy --all-targets --all-features -- -D warnings`: pass
- Sample tests: `test_migrate_legacy_config_dir_renames`, `test_env_var_with_legacy_*`, `test_verona_address_deserializes_legacy_xion_address_field`, `test_decrypt_legacy_salt_ciphertext` — all pass

## Summary
| Severity | Count |
|----------|-------|
| 🔴 Critical | 0 |
| 🟡 Warning | 2 |
| 🟢 Suggestion | 5 |

## Verdict

**Approve with residuals**

Core architecture matches the rebrand plan and contract: config split, migration paths, encryption dual-read, and error/crate rename are well-structured and tested. No Critical issues; fmt/clippy green. Two Warnings should be tracked as residuals (skill env fallback gap is the higher-impact item for migration UX). Suggestions are non-blocking maintainability follow-ups.

Recommended PM residual IDs (for `status.json`):
- **R1 (Warning):** Skill scripts missing `XION_*` env fallback per contract
- **R2 (Warning):** Dual config-dir edge case when empty `~/.verona-toolkit/` coexists with legacy dir
