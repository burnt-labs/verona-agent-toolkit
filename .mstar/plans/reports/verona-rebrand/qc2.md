---
report_kind: qc
reviewer: qc-specialist-2
reviewer_index: 2
plan_id: verona-rebrand
verdict: Approve
generated_at: "2026-06-16"
---

# Code Review Report

## Reviewer Metadata
- Reviewer: @qc-specialist-2
- Runtime Agent ID: qc-specialist-2
- Runtime Model: composer-2.5-fast
- Review Perspective: security & correctness (credential migration, encryption, env fallback, chain KEEP boundaries)
- Report Timestamp: 2026-06-16T00:00:00Z

## Scope
- plan_id: verona-rebrand
- Review range / Diff basis: 893c9f03724fa2e69adae81d4ee419432fba3b57..c6d93a86a8e53624c69c052a00a419bac87ac83f
- Working branch (verified): rebranding
- Review cwd (verified): /Users/bibi/workspace/burnt-labs/agent-toolkit
- Files reviewed: 12 primary (`src/config/*`, `build.rs`, contract cross-checks in `src/oauth/client.rs`)
- Commit range (if not identical to Review range line, explain): same as Review range
- Tools run: `git diff`, `git rev-parse`, `git branch`, `cargo test 'config::'`

## Findings

### 🔴 Critical
- None.

### 🟡 Warning
- None.

### 🟢 Suggestion
- **F-001 — Dual-dir migration edge case:** `migrate_legacy_config_dir` skips when `~/.verona-toolkit/` already exists, even if credentials remain only under `~/.xion-toolkit/` (`paths.rs` L44–46, test `test_migrate_skips_when_new_dir_exists`). Behavior matches the contract (“missing → migrate”), but users who pre-created an empty Verona dir could lose CLI access until manual merge. → Document in CHANGELOG/migration guide; optional future enhancement: merge `credentials/` when new dir lacks `.enc` files.
- **F-002 — Copy-fallback path untested:** Rename migration is covered; the `copy_dir_all` + `remove_dir_all` fallback (L54–57) has no unit test simulating cross-device rename failure. Contract Batch 5 calls for migration tests. → Add test with mock or temp setup where rename is forced to fail.
- **F-003 — `Box::leak` in network override:** `ConfigManager::get_current_network` leaks an owned override string on every call when `VERONA_NETWORK_OVERRIDE` / legacy fallback is set (`manager.rs` L58–62). Harmless for short-lived CLI, but avoidable. → Cache override in `ConfigManager` or use `once_cell`/`LazyLock` for the leaked str.
- **F-004 — Empty primary env blocks legacy fallback:** `env_var_with_legacy` returns `Ok("")` if `VERONA_*` is set to an empty string, preventing `XION_*` fallback (`env_compat.rs` L7–8). Same pattern in `build_env` (`build.rs` L17–18). → Treat empty/whitespace as unset during transition window.
- **F-005 — Skill-layer env fallback gap (contract note):** `.env.example` documents `XION_AUDIT_LOG`, `XION_SKILLS_DIR`, `XION_SKIP_CONFIRM` fallbacks, but `skills/verona-treasury/scripts/security-utils.sh` reads only `VERONA_*`. Rust core fallbacks are correct; shell scripts are not yet aligned with contract § Compatibility strategy item 2. → Track for skills batch or add `${VERONA_*:-${XION_*:-default}}` pattern.

## Source Trace
- Finding ID: F-001
- Source Type: manual-reasoning
- Source Reference: `src/config/paths.rs` L44–46, L99–113
- Confidence: High

- Finding ID: F-002
- Source Type: git-diff
- Source Reference: `src/config/paths.rs` L54–57; contract Batch 5 index
- Confidence: High

- Finding ID: F-003
- Source Type: manual-reasoning
- Source Reference: `src/config/manager.rs` L58–62
- Confidence: High

- Finding ID: F-004
- Source Type: manual-reasoning
- Source Reference: `src/config/env_compat.rs` L6–14; `build.rs` L16–24
- Confidence: Medium

- Finding ID: F-005
- Source Type: doc-rule
- Source Reference: `.mstar/knowledge/verona-rebrand-contract.md` L91; `skills/verona-treasury/scripts/security-utils.sh` L23–29
- Confidence: High

## Security & Correctness Assessment

### Credential migration (`paths.rs`)
- **Correct:** `config_dir()` is SSOT; triggers migration before `create_dir_all`. Prefer `rename`, fall back to recursive copy, remove legacy only after successful copy. No credential file deletion outside intentional migration/logout paths.
- **Correct:** `CredentialsManager::new` routes through `config_dir()`, so encrypted `.enc` files move with the directory tree without re-encryption.
- **AGENTS.md “NEVER delete credentials” rule:** Applies to agent/test behavior; product migration intentionally relocates (not destroys) legacy config. No violation.

### Dual encryption salt (`encryption.rs`)
- **Correct:** `NEW_SALT` / `LEGACY_SALT` constants match contract. `decrypt` retries legacy machine-derived key when primary fails and no CI env key is set (L160–171). `encrypt` always uses current key (new salt for machine-derived keys).
- **Correct:** `XION_CI_ENCRYPTION_KEY` fallback via `env_var_with_legacy`; covered by `test_legacy_env_key_fallback` and `test_decrypt_legacy_salt_ciphertext`.
- **Correct:** When CI env key is set, legacy salt retry is skipped—prevents ambiguous decryption in CI.

### Env fallback (`env_compat.rs`, `build.rs`, `manager.rs`)
- **Correct:** Runtime fallbacks for `VERONA_CI_ENCRYPTION_KEY` / `XION_CI_ENCRYPTION_KEY` and `VERONA_NETWORK_OVERRIDE` / `XION_NETWORK_OVERRIDE`.
- **Correct:** Build-time OAuth client IDs and API URLs use `build_env` with deprecation warnings; chain URLs in generated config unchanged (`xion-testnet-2`, `xion-mainnet-1`, rpc/indexer paths).
- **Correct:** `cargo:rerun-if-env-changed` lists both VERONA and XION variants.

### Serde alias `xion_address` (`schema.rs`)
- **Correct:** `#[serde(default, alias = "xion_address")]` on `verona_address`; deserialize test passes. Serialize writes `verona_address` (expected forward migration).

### Chain KEEP boundaries
- **Correct:** OAuth scopes unchanged in `src/oauth/client.rs` (`xion:identity:read`, `xion:blockchain:read`, `xion:transactions:submit`, dev `xion:mgr:*`).
- **Correct:** `build.rs` preserves chain IDs, RPC/REST/indexer URLs, `uxion` context in docs; no scope or OAuth API path renames.
- **Correct:** Address field still stores `xion1…` bech32; tests use `xion1` prefix.

### Tests
- **31/31** `config::` unit tests pass, including migration rename, legacy salt decrypt, env fallback, and `xion_address` alias.

## Summary
| Severity | Count |
|----------|-------|
| 🔴 Critical | 0 |
| 🟡 Warning | 0 |
| 🟢 Suggestion | 5 |

**Verdict**: Approve
