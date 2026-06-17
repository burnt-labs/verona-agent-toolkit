---
report_kind: qc-review
reviewer: qc-specialist-3
reviewer_index: 3
plan_id: verona-rebrand
verdict: Approve with residuals
generated_at: 2026-06-16T12:00:00Z
---

# Code Review Report

## Reviewer Metadata
- Reviewer: @qc-specialist-3
- Runtime Agent ID: qc-specialist-3
- Runtime Model: composer-2.5-fast
- Review Perspective: Performance, reliability, test coverage, CI/docs consistency, breaking-change migration UX
- Report Timestamp: 2026-06-16T12:00:00Z

## Scope
- plan_id: verona-rebrand
- Review range / Diff basis: 893c9f03724fa2e69adae81d4ee419432fba3b57..c6d93a86a8e53624c69c052a00a419bac87ac83f
- Working branch (verified): rebranding
- Review cwd (verified): /Users/bibi/workspace/burnt-labs/agent-toolkit
- Files reviewed: 182 (per diff stat); focused pass on `src/config/*`, `.github/workflows/ci.yml`, `tests/skills/`, `tests/e2e_*.sh`, `CHANGELOG.md`, `AGENTS.md`, `.env.example`, `docs/configuration.md`
- Commit range (if not identical to Review range line, explain): identical — 10 commits; local HEAD `c2c5f90` is one commit ahead (qc1 report only); CI gates run on current tree which includes review range
- Tools run: `git branch --show-current`, `git diff --stat 893c9f0..c6d93a8`, `cargo fmt -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-features`, `cargo build --release`, `MOCK_ENABLED=true CI=true tests/skills/run_all.sh`, product-layer grep

## Findings

### 🔴 Critical
- None.

### 🟡 Warning
- **W1 — Skill scripts lack legacy env fallback (migration UX / reliability)** → Contract § Compatibility strategy item 2 requires `XION_*` fallback with deprecation warning for mapped vars. Rust core (`env_compat.rs`, `build.rs`) and `.env.example` document fallbacks, but `skills/verona-treasury/scripts/security-utils.sh` (and related skill scripts) read only `VERONA_AUDIT_LOG`, `VERONA_AUDIT_LOG_FILE`, `VERONA_SKIP_CONFIRM`. CI pipelines or users still exporting `XION_SKIP_CONFIRM` / `XION_AUDIT_LOG` will silently lose behavior after rebrand. **Fix:** adopt `${VERONA_SKIP_CONFIRM:-${XION_SKIP_CONFIRM:-false}}` pattern or shared bash helper; align with `.env.example` comments.

- **W2 — User docs omit config-directory migration guidance** → `CHANGELOG.md` [Unreleased] states automatic migration from `~/.xion-toolkit/`, but `docs/configuration.md` describes only `~/.verona-toolkit/` with no migration section, dual-dir edge case, or manual merge steps. Breaking-change migration UX is incomplete for operators upgrading from Xion toolkit. **Fix:** add a "Migrating from Xion Agent Toolkit" subsection to `docs/configuration.md` (auto-rename, legacy env vars, skill reinstall, empty-dir edge case).

- **W3 — CI clippy gate narrower than AGENTS.md / local dev** → `AGENTS.md` and assignment workflow specify `cargo clippy --all-targets --all-features -- -D warnings`. `.github/workflows/ci.yml` lint job runs `cargo clippy --all-features -- -D warnings` (no `--all-targets`). Local review passes with `--all-targets`; CI could miss target-only warnings (benches, tests). **Fix:** add `--all-targets` to CI lint step for parity.

### 🟢 Suggestion
- **S1 — `Box::leak` on every network override read** → `ConfigManager::get_current_network` leaks override string on each call when env override is set (`manager.rs` L57–64). Harmless for short-lived CLI; avoidable with cached override or `OnceLock`. Minor hot-path concern.
- **S2 — Copy-fallback migration path untested** → `paths.rs` rename migration is tested; `copy_dir_all` + `remove_dir_all` fallback (L54–57) has no unit test simulating cross-device rename failure. Contract Batch 5 calls for migration tests.
- **S3 — Empty primary env blocks legacy fallback** → `env_var_with_legacy` returns `Some("")` when `VERONA_*` is set empty, preventing `XION_*` fallback during transition. Edge case for misconfigured CI; treat whitespace-only as unset if desired.
- **S4 — Skills shellcheck non-blocking in CI** → `skills-lint` job runs `shellcheck "$script" || true`, so skill script lint regressions won't fail CI. Pre-existing pattern; consider failing on error post-rebrand stabilization.
- **S5 — Optional product-layer grep gate absent** → Contract risk table mentions optional CI grep for residual `xion-toolkit` product strings; not added. Low priority post-merge hardening (grep confirms clean `src/`, `skills/`, `tests/`, `docs/` except intentional legacy constants).
- **S6 — E2E secret name still `XION_TESTNET_CREDENTIALS`** → CI workflow correctly uses `~/.verona-toolkit/credentials/` but secret name unchanged. Intentional to avoid GitHub secret rotation mid-rebrand; document in `docs/release.md` or CI comments for ops clarity.

## Performance, Reliability & Test Coverage Assessment

### CI / static analysis (positive)
- `cargo fmt -- --check`: **pass**
- `cargo clippy --all-targets --all-features -- -D warnings`: **pass**
- `cargo test --all-features`: **pass** — 618 tests (521 lib + 29 integration + 19 other + 49 doc tests; 7 doc tests ignored)
- Skills mock suite (`MOCK_ENABLED=true tests/skills/run_all.sh`): **pass** — 4 files, 49 cases
- Release artifact renamed: `verona-toolkit-binary` → `target/release/verona-toolkit`
- CI env primary key updated to `VERONA_CI_ENCRYPTION_KEY` with comment noting legacy fallback
- `release-please-config.json` package name aligned to `verona-agent-toolkit`

### Migration & compat tests (positive)
- Config dir rename migration: `test_migrate_legacy_config_dir_renames`, `test_migrate_skips_when_new_dir_exists`
- Env fallback: `test_env_var_with_legacy_*` (3 cases)
- Credential schema: `test_verona_address_deserializes_legacy_xion_address_field`
- Encryption: `test_legacy_env_key_fallback`, `test_decrypt_legacy_salt_ciphertext`
- E2E scripts renamed and invoke `verona-toolkit`; protected treasury unchanged
- Skills tests renamed to `test_verona_*.sh`; mocks use `verona_address`

### Reliability observations
- `CredentialsManager::new` routes through `config_dir()` SSOT — migration runs before credential access (good)
- Dual salt read path preserves existing encrypted credentials without re-encryption
- Product-layer grep in `src/`, `skills/`, `tests/`, `docs/`, `.github/` shows only intentional legacy constants (`LEGACY_CONFIG_DIR_NAME`, `LEGACY_SALT`)

### Docs / AGENTS.md consistency (mixed)
- `AGENTS.md` updated: product name, `VERONA_CI_ENCRYPTION_KEY`, `~/.verona-toolkit/`, test count **618** (matches run)
- `CHANGELOG.md` [Unreleased] covers breaking changes adequately at high level
- Gap: user-facing migration runbook in `docs/configuration.md` (see W2)
- Harness knowledge files (`.mstar/knowledge/feature-roadmap.md`, etc.) still reference `xion-toolkit` — out of done-criteria scope but may confuse agents (noted for PM residual tracking)

## Source Trace
| Finding ID | Source Type | Source Reference | Confidence |
|------------|-------------|------------------|------------|
| W1 | git-diff + doc-rule | `skills/verona-treasury/scripts/security-utils.sh` L23–29; contract § Compatibility | High |
| W2 | doc-rule | `CHANGELOG.md` L12–14 vs `docs/configuration.md` (no migration section) | High |
| W3 | doc-rule | `.github/workflows/ci.yml` L70–71 vs `AGENTS.md` clippy command | High |
| S1 | manual-reasoning | `src/config/manager.rs` L57–64 | High |
| S2 | git-diff | `src/config/paths.rs` L54–57 | High |
| S3 | manual-reasoning | `src/config/env_compat.rs` L6–14 | Medium |
| S4 | git-diff | `.github/workflows/ci.yml` L177–181 | High |
| S5 | doc-rule | contract Risks table | Medium |
| S6 | git-diff | `.github/workflows/ci.yml` L202, L231 | High |

## Summary
| Severity | Count |
|----------|-------|
| 🔴 Critical | 0 |
| 🟡 Warning | 3 |
| 🟢 Suggestion | 6 |

## Verdict

**Approve with residuals**

All mandatory CI gates pass locally. Rebrand delivers solid Rust-core migration tests, dual salt/env compat, and updated skills/E2E harness. Residual warnings are migration-UX and CI-parity gaps (skill bash env fallback, user docs, clippy `--all-targets`) that should be tracked for pre-release or fast-follow; none block merge from a performance/reliability perspective given Rust-layer compat is correct and 618 tests green.
