# Verona Rebrand Contract

> SSOT for the Verona Agent Toolkit full rebrand. Plan: `.mstar/plans/verona-rebrand.md` · Status: `.mstar/status.json` → `verona-rebrand`.

## Scope

- **In scope:** Product-layer rename (CLI, crate, repo, skills, config paths, env vars, error types, docs).
- **Out of scope:** Chain/protocol identifiers (see [Chain-layer KEEP](#chain-layer-keep)).
- **External docs:** [docs.verona.dev Toolkit page](https://docs.verona.dev/en/build-on-verona/tools/verona-toolkit.md) updates **after** this repository rebrand merges.

Reference: [Rebrand: XION → Verona](https://docs.verona.dev/en/others/rebrand-from-xion.md).

---

## Rename matrix

| Category | Before | After | Notes |
|----------|--------|-------|-------|
| Product name | Xion Agent Toolkit | **Verona Agent Toolkit** | Global narrative |
| Cargo package | `xion-agent-toolkit` | `verona-agent-toolkit` | `Cargo.toml` |
| Library crate | `xion_agent_toolkit` | `verona_agent_toolkit` | All `use` paths |
| CLI binary | `xion-toolkit` | `verona-toolkit` | clap `name` |
| Config directory | `~/.xion-toolkit/` | `~/.verona-toolkit/` | Startup migration required |
| Encryption salt prefix | `xion-toolkit-credentials-v1:` | `verona-toolkit-credentials-v1:` | Read old, write new |
| Error types | `XionError`, `XionErrorCode` | `VeronaError`, `VeronaErrorCode` | `src/shared/error.rs` |
| JSON field (credentials) | `xion_address` | `verona_address` | `#[serde(alias = "xion_address")]` |
| Schema format | `xion-address` | `verona-address` | Validation still accepts `xion1` prefix |
| Skill folders (7) | `xion-*` | `verona-*` | `git mv` under `skills/` |
| Skill env | `XION_SKILLS_DIR` | `VERONA_SKILLS_DIR` | |
| GitHub repository | `burnt-labs/xion-agent-toolkit` | `burnt-labs/verona-agent-toolkit` | Coordinate with org |
| Installer artifacts | `xion-agent-toolkit-installer.*` | `verona-agent-toolkit-installer.*` | |
| CI artifact | `xion-toolkit-binary` | `verona-toolkit-binary` | |

### Skill folder mapping

| Before | After |
|--------|-------|
| `skills/xion-dev/` | `skills/verona-dev/` |
| `skills/xion-toolkit-init/` | `skills/verona-toolkit-init/` |
| `skills/xion-oauth2/` | `skills/verona-oauth2/` |
| `skills/xion-oauth2-client/` | `skills/verona-oauth2-client/` |
| `skills/xion-treasury/` | `skills/verona-treasury/` |
| `skills/xion-faucet/` | `skills/verona-faucet/` |
| `skills/xion-asset/` | `skills/verona-asset/` |

---

## Environment variable mapping

| Before | After |
|--------|-------|
| `XION_NETWORK_OVERRIDE` | `VERONA_NETWORK_OVERRIDE` |
| `XION_CI_ENCRYPTION_KEY` | `VERONA_CI_ENCRYPTION_KEY` |
| `XION_TESTNET_OAUTH_CLIENT_ID` | `VERONA_TESTNET_OAUTH_CLIENT_ID` |
| `XION_MAINNET_OAUTH_CLIENT_ID` | `VERONA_MAINNET_OAUTH_CLIENT_ID` |
| `XION_TESTNET_OAUTH_API_URL` | `VERONA_TESTNET_OAUTH_API_URL` |
| `XION_MAINNET_OAUTH_API_URL` | `VERONA_MAINNET_OAUTH_API_URL` |
| `XION_SKILLS_DIR` | `VERONA_SKILLS_DIR` |
| `XION_AUDIT_LOG` | `VERONA_AUDIT_LOG` |
| `XION_AUDIT_LOG_FILE` | `VERONA_AUDIT_LOG_FILE` |
| `XION_SKIP_CONFIRM` | `VERONA_SKIP_CONFIRM` |

Build-time compile env in `build.rs` follows the same `XION_*` → `VERONA_*` pattern. Chain URLs embedded in `build.rs` are **not** renamed.

---

## Chain-layer KEEP

Do **not** rename these unless the blockchain itself changes identifiers:

| Identifier | Examples / locations |
|------------|---------------------|
| Bech32 prefix | `xion1...`, `xionvaloper1...` |
| Native denom | `uxion` |
| Chain IDs | `xion-local`, `xion-testnet-2`, `xion-mainnet-1` |
| OAuth scopes | `xion:identity:read`, `xion:blockchain:read`, `xion:transactions:submit`, `xion:mgr:read`, `xion:mgr:write` |
| Node CLI | `xiond` |
| RPC / REST / Indexer URLs | `rpc.xion-testnet-2.burnt.com`, indexer `/xion-testnet-2` paths |
| Indexer API segments | `/xion/account/treasuries` |
| On-chain contract addresses | Faucet, protected treasury in AGENTS.md |
| Upstream crate | `xion-types` (dependency name) |
| External repo | `burnt-labs/xion-skills` (`xiond-*` skills) — update cross-references only |

Document rule: **Verona in product prose; XION in chain/command examples where precision is required.**

---

## Compatibility strategy

1. **Config directory:** On startup, if `~/.verona-toolkit/` is missing and `~/.xion-toolkit/` exists → migrate (rename or copy + warn). Covered by migration tests in Batch 5.
2. **Environment variables:** First release accepts `XION_*` as fallback with deprecation warning; primary names are `VERONA_*`.
3. **CLI binary:** No long-lived `xion-toolkit` alias. Breaking change documented in CHANGELOG with migration guide.
4. **Encrypted credentials:** Support both encryption salt prefixes when reading; write with new prefix.
5. **JSON fields:** `verona_address` with serde alias for legacy `xion_address`.

---

## Batch index (implementation roadmap)

| Batch | Scope | Key paths |
|-------|-------|-----------|
| **0** | Plan registration (done) | `.mstar/knowledge/verona-rebrand-contract.md`, `.mstar/plans/verona-rebrand.md`, `.mstar/status.json` |
| **1** | Rust core | `Cargo.toml`, `build.rs`, `src/config/*`, `src/cli/*`, `src/shared/error.rs` |
| **2** | CI / release | `.github/workflows/*`, `release-please-config.json`, `.env.example` |
| **3** | Skills | `skills/verona-*/` (7 folders), scripts, schemas, routing, evals |
| **4** | Documentation | `README.md`, `INSTALL-FOR-AGENTS.md`, `docs/`, `AGENTS.md`, `CLAUDE.md` |
| **5** | Tests | `tests/`, `tests/skills/`, migration tests, E2E scripts |
| **6** | Verify & deliver | fmt / clippy / test, QC tri-review, PR `rebranding` → `main` |

Post-merge (outside repo):

- GitHub repo rename to `verona-agent-toolkit`
- docs.verona.dev Toolkit page update
- Notify `xion-skills` maintainers for cross-reference updates

---

## Full rebrand done criteria

- Zero product-layer references to `xion-toolkit`, `xion-agent-toolkit`, or `~/.xion-toolkit/` (`.mstar/plans/archived/` excluded).
- `verona-toolkit --version` works; all skill scripts invoke `verona-toolkit`.
- Seven skill folders named `verona-*`.
- Legacy `~/.xion-toolkit/` migrates automatically.
- Chain examples remain valid (`xion1...`, `1000000uxion`, `xion-testnet-2`).
- `cargo test --all-features` passes.

---

## Risks and mitigations

| Risk | Mitigation |
|------|------------|
| Credential directory migration failure | Startup migration + unit/integration tests; manual fallback in docs |
| Encryption salt change breaks old files | Dual salt read path |
| Global skill install breakage | CHANGELOG: `npx skills add burnt-labs/verona-agent-toolkit -g` |
| CI secrets rename | Sync GitHub repo secrets before Batch 2 merge |
| Bulk replace misses strings | Optional CI grep gate for product-layer `xion-toolkit` |
