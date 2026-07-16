# eng-1880-oauth2-010-adapt — OAuth2 0.1.0 Client SSOT Adapt

> **For agentic workers:** REQUIRED SUB-SKILL: Use `mstar-sdd`. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Adapt verona-agent-toolkit CLI, mgr API client, skills, and docs to oauth2-api-service **0.1.0** client SSOT (ENG-1880) so agents cannot teach or send free-form redirect injection paths.

**User outcome:** Operators and agents use treasury on-chain params + `sync-from-treasury`; legacy redirect/logo flags may still appear in old scripts but are visibly ignored (stderr warning) and never sent to the API.

**Architecture:** Strip/warn chain-mapped fields in CLI + request builders before HTTP; add `sync-from-treasury` end-to-end; align error mapping for GET 403; update `verona-oauth2-client` / `verona-treasury` skills to treasury-first workflow.

**Tech Stack:** Rust (`clap`, `reqwest`, `serde`), bash skills + JSON schemas, existing mock test harness under `tests/`.

**Execution:** mstar-sdd

**Prepare gates:** specify ✅ · clarify ✅ (grill-me A/R′/P1/K1/V1 + branch + Warning UX) · plan ✅ (this file)

**Primary spec:** `specs/oauth2-api-010-client-ssot.md` (`locked`)  
**Iteration compass:** `.mstar/iterations/eng-1880-oauth2-010-adapt/delivery-compass.md`  
**Migration guide:** `.mstar/iterations/eng-1880-oauth2-010-adapt/guides/eng-1880-impact-and-migration.md`  
**Linear:** ENG-1880

## Global Constraints

- Never call `/auth/sdk-session`.
- Chain-mapped fields (`redirectUris`, `logoUri`, `clientUri`, create `owner`): **warn + discard** per spec **Warning UX** (`print_warning` → stderr `[WARNING]`; success JSON on stdout unchanged; exit 0 when otherwise valid).
- **Pipeline order (AD-1):** parse → `strip_chain_mapped_*` → `print_warning` → `MgrApiClient` serialize/send. Warn+discard **before** `serde_json::to_value` inside API client.
- **Double-send risk:** If `--json-input` is parsed into a struct and passed to HTTP **without** calling the same strip helper as the flag path, chain-mapped keys leak into POST/PUT and API may reject. **Mitigation:** single strip functions in `mgr_api.rs`; both `handle_create` and `handle_update` call them; no post-strip merge of raw JSON. Add unit test: json_input with `redirectUris` → stripped body omits key.
- clap: deprecated flags **stay visible** (AD-2); `--redirect-uris` optional, not required.
- Do not hard-fail solely for presence of deprecated fields.
- Do not delete `~/.verona-toolkit/credentials/*.enc`; do not run `auth logout` unless user asks.
- Protected treasury address in tests/scripts remains write-protected per root `AGENTS.md`.
- English comments/docs in code; conventional commits.
- CI: `cargo fmt -- --check`, `clippy -D warnings`, `cargo test --all-features`.
- GitNexus: run impact before editing symbols; `detect_changes` before commit.

## Requirement → task map

| Spec | Task |
|------|------|
| R1, R2, R4 (partial) | Task 1 (serialization, sync HTTP, 403 mapper) |
| R1, R2, R3, Warning UX | Task 2 (CLI) |
| R3, R6 (skills) | Task 3 |
| R6 (docs), grep hygiene | Task 4 |
| R5 | Tasks 1–4 (grep + no new sdk-session code) |

---

### Task 1: Mgr API types + sync-from-treasury + error mapping

**Covers:** R1, R2, R3 (HTTP), R4, R5 (API layer)

**Files:**
- Modify: `src/api/mgr_api.rs`
- Modify: `src/shared/error.rs` (`OAuthClientError::ClientAccessDenied`, `VeronaErrorCode::EOAUTHCLIENT020`)
- Test: unit tests in `src/api/mgr_api.rs` (or adjacent test module)

**Interfaces:**

```rust
// src/api/mgr_api.rs — new helpers
pub fn strip_chain_mapped_create(req: &mut CreateClientRequest) -> Vec<&'static str>;
pub fn strip_chain_mapped_update(req: &mut UpdateClientRequest) -> Vec<&'static str>;

// src/api/mgr_api.rs — new method
impl MgrApiClient {
    pub async fn sync_from_treasury(
        &self,
        access_token: &str,
        client_id: &str,
    ) -> VeronaResult<ClientResponse>; // POST /mgr-api/clients/{client_id}/sync-from-treasury, body: None
}

// src/api/mgr_api.rs — map_error addition
(403, "PERMISSION_DENIED_VIEW_CLIENT") => OAuthClientError::ClientAccessDenied { message }
```

**Done when:**
- `strip_chain_mapped_*` remove chain-mapped fields; unit tests prove serialized JSON omits keys.
- Create/update serializers omit chain-mapped fields when stripped (JSON unit tests).
- `sync_from_treasury(client_id)` → `POST /mgr-api/clients/{id}/sync-from-treasury` (no body).
- `PERMISSION_DENIED_VIEW_CLIENT` → `ClientAccessDenied` / `EOAUTHCLIENT020`; `INSUFFICIENT_SCOPE` unchanged; no `ClientNotFound` for 403 GET semantics.

**Steps:**

- [x] **Step 1:** Add failing unit tests for: strip helpers + create/update JSON omitting discarded fields; sync path/method/body; `PERMISSION_DENIED_VIEW_CLIENT` vs `INSUFFICIENT_SCOPE` vs `CLIENT_NOT_FOUND` mapping per spec AD-4.
- [x] **Step 2:** Run targeted tests — expect FAIL.
- [x] **Step 3:** Implement `strip_chain_mapped_*`, `sync_from_treasury`, `ClientAccessDenied` + mapper arm; ensure create/update only serialize post-strip structs from callers.
- [x] **Step 4:** Run targeted tests — expect PASS.
- [x] **Step 5:** Commit `feat(api): add sync-from-treasury and harden client SSOT requests`

---

### Task 2: CLI create/update warn + discard + sync-from-treasury command

**Covers:** R1, R2, R3 (CLI), Warning UX

**Files:**
- Modify: `src/cli/oauth2_client.rs` (and command enum wiring if split)
- Modify: related handler module(s) under `src/cli/` if create/update handlers live elsewhere
- Test: CLI/handler unit tests in the same modules

**Interfaces:**

```rust
// src/cli/oauth2_client.rs
pub enum OAuth2ClientCommands {
    // ... existing ...
    SyncFromTreasury { client_id: String }, // new
}

// handle_create / handle_update pipeline (AD-1):
// 1. build CreateClientRequest | UpdateClientRequest (flags or --json-input)
// 2. let discarded = strip_chain_mapped_*(&mut request)
// 3. for label in discarded { print_warning(&format!(...)) }
// 4. mgr_client.create_client | update_client(request)

// CreateClientArgs: --treasury -> binded_treasury; --redirect-uris optional (visible, not hidden)
```

**Done when:**
- Create works without required `--redirect-uris`.
- Deprecated flags / `--json-input` keys → stderr `[WARNING]` via `print_warning`, exit 0 if otherwise valid, stripped **before** HTTP (same strip helpers as Task 1).
- `--json-input` path uses strip helpers (no double-send).
- `verona-toolkit oauth2 client sync-from-treasury <client_id>` wired.

**Steps:**

- [x] **Step 1:** Failing tests for: create without redirect-uris; stderr warning when deprecated flags/JSON keys present; success stdout JSON unchanged; json_input strip; sync subcommand wiring.
- [x] **Step 2:** Run — expect FAIL.
- [x] **Step 3:** Implement warn + discard in handlers (call `strip_chain_mapped_*` + `print_warning`); update clap args (`redirect_uris` optional, visible, help text); wire `SyncFromTreasury`; remove `bail!("--redirect-uris is required")`.
- [x] **Step 4:** Run — expect PASS.
- [x] **Step 5:** Commit `feat(cli): treasury SSOT for oauth2 client create/update + sync`

---

### Task 3: Skills + schemas (`verona-oauth2-client`, `verona-treasury`)

**Covers:** R3, R6, Warning UX (skill guidance)

**Files:**
- Modify: `skills/verona-oauth2-client/SKILL.md`
- Modify: `skills/verona-oauth2-client/schemas/create.json`
- Modify: `skills/verona-oauth2-client/schemas/update.json`
- Modify: other oauth2-client schemas if they mention redirect create/update
- Modify: `skills/verona-treasury/SKILL.md` (cross-link sync workflow)
- Modify: treasury references only if they teach old client redirect create coupling
- Test: existing skills mock tests under `tests/` if applicable; schema validation scripts if present

**Done when:**
- Schemas: no required `redirect-uris`; deprecated keys documented as ignored (stderr warning).
- Examples: treasury param update → create → sync workflow; no free-form redirect SSOT teaching.
- Treasury skill links sync when `redirect_url` / branding params change.
- Skill text tells agents stderr warnings mean redirect did **not** change via CLI flags.

**Steps:**

- [x] **Step 1:** Update schemas per R6; add sync-from-treasury schema/script entry if skill pattern requires it.
- [x] **Step 2:** Rewrite SKILL examples to treasury-first + sync; add troubleshooting table aligned with migration guide.
- [x] **Step 3:** Add treasury cross-link section for redirect/branding param changes.
- [x] **Step 4:** Run skill-related tests / dry-run schema checks.
- [x] **Step 5:** Commit `docs(skill): align oauth2 client + treasury with API 0.1.0 SSOT`

---

### Task 4: CLI reference docs + residual grep hygiene

**Covers:** R6, V1 verification

**Files:**
- Modify: `docs/cli-reference.md` (oauth2 client sections)
- Optionally: README snippets if they show create with `--redirect-uris` as required
- Grep sweep: skills + docs for stale “required redirectUris on create” teaching

**Done when:**
- `docs/cli-reference` documents warn + discard (stderr), sync command, and treasury-first create.
- In-scope grep hits for stale redirect SSOT teaching are fixed (K1 scope).
- Full local CI gate passes.

**Steps:**

- [x] **Step 1:** Update cli-reference create/update/sync sections (include Warning UX).
- [x] **Step 2:** Repo grep for stale patterns; fix remaining in-scope hits (K1).
- [x] **Step 3:** `cargo fmt` + `clippy -D warnings` + `cargo test --all-features`.
- [x] **Step 4:** Commit `docs(cli): document oauth2 client treasury SSOT and sync`

---

## Durable Roadmap Gate

| Deferred | Owner | Trigger | Done when |
|----------|-------|---------|-----------|
| Optional Testnet-2 live smoke (create → sync → authorize URI match) | PM / ops | After merge or user request | Smoke note in iteration close or follow-up issue |
| Multi-URI treasury support | Product | Product demand | Separate iteration; out of ENG-1880 |

## Plan self-review (PM + architect + writing)

1. **Spec coverage:** R1–R6 + Warning UX + AD-1–AD-5 → Tasks 1–4 ✅
2. **Spec status:** `specs/oauth2-api-010-client-ssot.md` locked ✅
3. **Placeholder scan:** no TBD in task steps ✅
4. **Type consistency:** `sync-from-treasury` naming aligned CLI/API/skills ✅
5. **403 mapping:** `PERMISSION_DENIED_VIEW_CLIENT` distinct from `INSUFFICIENT_SCOPE` ✅
6. **Double-send:** Global Constraints + Task 2 `json_input` strip ✅

## SDD runtime (ephemeral)

When using `mstar-sdd`, artifacts live under `{SDD_DIR}` (`.mstar/sdd/eng-1880-oauth2-010-adapt/`). Do not duplicate briefs/reports in this file.

## Review gate summary

- **QA:** Accept with residuals (005f30bc)


- **QC (tri):** Approve with residuals after fix `36c2de6` (qc1/qc2/qc3 Approve)
- **Residuals:** R1 accepted_risk (CLI-only strip), R2 deferred (sync script polish) — SSOT `status.json` `residual_findings[eng-1880-oauth2-010-adapt]`
- **Review range:** `1167507..36c2de6`
- **Bundle:** `.mstar/sdd/eng-1880-oauth2-010-adapt/review/` (ephemeral)

