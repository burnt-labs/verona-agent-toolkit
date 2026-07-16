---
title: OAuth2 API 0.1.0 — Client SSOT contract (toolkit)
status: locked
created_at: 2026-07-16
updated_at: 2026-07-16
source: ENG-1880 / oauth2-api-service 0.1.0
product_review: 2026-07-16
architect_review: 2026-07-16
writing_review: 2026-07-16
---

# Spec: OAuth2 API 0.1.0 Client SSOT (verona-agent-toolkit)

## Purpose

Define the **locked** client-management contract for verona-agent-toolkit against oauth2-api-service **0.1.0**. CLI, HTTP client, skills, and docs share one SSOT so agents cannot teach or automate free-form redirect injection.

## User value

| Actor | Before 0.1.0 | After adapt |
|-------|--------------|-------------|
| **Agent / skill author** | Examples showed `--redirect-uris` as required; easy to teach a path the API no longer accepts | Treasury-first create + `sync-from-treasury`; schemas and skills match API SSOT |
| **Human operator** | Could believe the CLI updated redirect/logo without touching treasury | stderr warnings make ignored fields explicit; `sync-from-treasury` is the documented refresh path |
| **Security posture** | Toolkit could send chain-mapped fields that implied client-side redirect control | Outbound bodies omit discarded fields; authorize `redirect_uri` must match on-chain treasury |

## Normative references

- Linear [ENG-1880](https://linear.app/burnt/issue/ENG-1880)
- oauth2-api-service `CHANGELOG.md` § **0.1.0** (breaking items 1–6; added `sync-from-treasury`; GET client → `403 PERMISSION_DENIED_VIEW_CLIENT`)
- Upstream endpoints: `POST/PUT/GET /mgr-api/clients`, `POST /mgr-api/clients/{id}/sync-from-treasury`

## Architecture decisions (architect lock)

### AD-1 — Warn + discard pipeline order

Chain-mapped discard and stderr warnings **MUST** occur **before** HTTP body serialization and **before** `MgrApiClient::create_client` / `update_client` invoke `serde_json::to_value`.

```
CLI flags / --json-input
  → parse into CreateClientRequest | UpdateClientRequest
  → strip_chain_mapped_* (mutate struct; collect discarded field labels)
  → print_warning per discarded group (stderr only)
  → MgrApiClient::create_client | update_client (serialize already-stripped struct)
```

**Invariant:** Both the flag path and `--json-input` path call the **same** strip helpers. No code path may merge raw JSON after strip or pass an unstripped struct to the API client.

**Strip helper location:** `src/api/mgr_api.rs` — `strip_chain_mapped_create(&mut CreateClientRequest) -> Vec<&'static str>` and `strip_chain_mapped_update(&mut UpdateClientRequest) -> Vec<&'static str>`. CLI (`src/cli/oauth2_client.rs`) owns warning text + `print_warning`; API module owns field SSOT.

### AD-2 — clap deprecation UX (confirmed)

Product choice **confirmed** architecturally:

| Aspect | Decision |
|--------|----------|
| Flag visibility | **Keep visible** — do **not** set `hide = true` on deprecated flags (`--redirect-uris`, `--logo-uri`, `--client-uri`, `--owner` on create) |
| Requiredness | `--redirect-uris` becomes **optional**; remove hard-fail when empty |
| Help text | Add `help` noting field is ignored; treasury + sync is SSOT |
| Warning channel | `print_warning()` → stderr `[WARNING]` **after** strip detects discarded values |
| Exit code | `0` when otherwise valid (per **Warning UX**) |

Rationale: legacy scripts keep working; operators see deprecation without breaking `--help` discovery.

### AD-3 — `bindedTreasury` wire format vs CLI `--treasury`

| Layer | Field name | Notes |
|-------|------------|-------|
| HTTP JSON (API SSOT) | `bindedTreasury` | camelCase per mgr-api OpenAPI |
| Rust struct | `CreateClientRequest::binded_treasury` | `#[serde(rename_all = "camelCase")]` — already aligned |
| CLI flag | `--treasury` | CLI-only alias; maps to `binded_treasury` when building struct from flags |
| `--json-input` | `bindedTreasury` key | Deserializes via same struct; **required** on create unless provided via flags |

**Serializer alignment:** Existing `CreateClientRequest` / `ClientExtension` serde uses `camelCase`; no rename needed. Implementers **MUST NOT** introduce a separate JSON key for the CLI flag.

### AD-4 — GET 403 error mapping (no collision with `INSUFFICIENT_SCOPE`)

Upstream 0.1.0 returns `403` with distinct `code` values. Toolkit **MUST** map each explicitly; **MUST NOT** map inaccessible/missing client GET to `ClientNotFound` (`EOAUTHCLIENT012`).

| HTTP | Upstream `code` | `OAuthClientError` variant | `VeronaErrorCode` | Agent-facing hint (summary) |
|------|-----------------|----------------------------|-------------------|------------------------------|
| 403 | `INSUFFICIENT_SCOPE` | `InsufficientScope` | `EOAUTHCLIENT010` | Re-auth with mgr scopes (`--dev-mode` or full OAuth) |
| 403 | `ONLY_OWNER_ALLOWED` | `OnlyOwnerAllowed` | `EOAUTHCLIENT011` | Only owner may perform this action |
| 403 | `PERMISSION_DENIED_VIEW_CLIENT` | **`ClientAccessDenied`** (new) | **`EOAUTHCLIENT020`** (new) | Client missing or no access — **not** “check ID” 404 semantics; verify id + ownership + mgr read scope |
| 403 | *(other / fallback)* | `InsufficientScope` | `EOAUTHCLIENT010` | Preserve existing `(403, _)` fallback in `map_error` |
| 404 | `CLIENT_NOT_FOUND` | `ClientNotFound` | `EOAUTHCLIENT012` | Reserved for responses that **actually** return 404 (e.g. post-access internal paths per upstream); **not** the 0.1.0 GET-client enumeration case |

**New variant:** Add `OAuthClientError::ClientAccessDenied { message: String }` in `src/shared/error.rs` with hint: *"Client may not exist or you lack access. Re-auth with xion:mgr:read; do not assume the client ID is wrong from 'not found' wording."*

**Unit tests:** `test_map_error_permission_denied_view_client` — assert variant is `ClientAccessDenied`, **not** `ClientNotFound` or conflated message with `INSUFFICIENT_SCOPE`.

### AD-5 — `sync-from-treasury` contract

| Item | Value |
|------|-------|
| Method + path | `POST /mgr-api/clients/{clientId}/sync-from-treasury` |
| Request body | **None** (empty POST) |
| Rust method | `MgrApiClient::sync_from_treasury(&self, access_token: &str, client_id: &str) -> VeronaResult<ClientResponse>` |
| CLI | `OAuth2ClientCommands::SyncFromTreasury { client_id }` → `verona-toolkit oauth2 client sync-from-treasury <client_id>` |
| Auth | Bearer token with `xion:mgr:write` (same as other mutating mgr routes) |
| 403 on sync | Same access-denial semantics as GET (no clientId enumeration via 404) — map via AD-4 table |

## Definitions

- **Chain-mapped fields:** `redirectUris`, `logoUri`, `clientUri`, and create-time `owner` (API forces owner = caller).
- **Treasury SSOT:** on-chain treasury `redirect_url`, `icon_url`, `display_url` are authoritative for those mappings.
- **Warn + discard:** the user may pass deprecated flags or JSON keys; the toolkit logs a stderr warning, strips fields before HTTP, and continues when no other validation fails.

## Warning UX (product decision)

Deprecated chain-mapped input uses the existing CLI convention:

| Channel | Behavior |
|---------|----------|
| **stderr** | One `[WARNING]` line per discarded field group via `print_warning()` (human-readable). Example: `[WARNING] --redirect-uris is ignored; set redirect_url on treasury and run sync-from-treasury.` |
| **stdout (success JSON)** | Unchanged success payload; warnings **MUST NOT** be mixed into success JSON. Agents parse stdout for results; stderr is for operator visibility and log capture. |
| **stdout (failure JSON)** | Structured `ErrorResponse` on stderr-independent failure paths per existing CLI error handling. |
| **Exit code** | `0` on successful create/update/sync after warn + discard unless another validation or API error occurs. |

Skills **MUST** tell agents: treat stderr warnings as actionable (do not assume redirect/branding changed) and prefer treasury update + sync over deprecated flags.

## Requirements

### R1 — Create client

- Toolkit **MUST** require `bindedTreasury` (CLI `--treasury` / JSON equivalent).
- Toolkit **MUST NOT** require caller-supplied `redirectUris` for successful create.
- If the user supplies chain-mapped fields via flags or `--json-input`, the toolkit **MUST** warn + discard per **Warning UX** and proceed (no hard-fail solely for their presence).
- Outbound create body **MUST NOT** include discarded chain-mapped fields (verify via unit test on serialized JSON).

**Agent-testable checks:**

- `oauth2 client create --treasury <addr> --client-name X` succeeds without `--redirect-uris`.
- `oauth2 client create ... --redirect-uris https://x` exits 0, prints `[WARNING]` on stderr, and POST body has no `redirectUris` key.

### R2 — Update client

- Toolkit **MUST NOT** send chain-mapped fields on `PUT /mgr-api/clients/{id}`.
- If supplied, **MUST** warn + discard (same as R1).
- Non-mapped fields (`clientName`, `policyUri`, `tosUri`, `jwksUri`, `contacts`, etc.) remain updatable per API.

**Agent-testable checks:**

- `oauth2 client update <id> --logo-uri https://x` exits 0 with stderr warning; PUT body omits `logoUri`.

### R3 — Sync from treasury

- Toolkit **MUST** expose `POST /mgr-api/clients/{clientId}/sync-from-treasury` as:
  - `MgrApiClient::sync_from_treasury` (or equivalent)
  - CLI: `verona-toolkit oauth2 client sync-from-treasury <client_id>`
- Skills **MUST** document: after changing `redirect_url` / `icon_url` / `display_url` on-chain → run sync.
- `verona-treasury` **MUST** cross-link this command when teaching param updates that affect OAuth redirect/branding.

**Agent-testable checks:**

- CLI invokes `POST /mgr-api/clients/{id}/sync-from-treasury` with no chain-mapped body fields.
- Skill quick-start shows treasury param update → `sync-from-treasury` (not `--redirect-uris` on update).

### R4 — GET / errors

- When API returns **403** with `PERMISSION_DENIED_VIEW_CLIENT` (upstream 0.1.0 GET client for missing/inaccessible client), toolkit **MUST** map to `ClientAccessDenied` / `EOAUTHCLIENT020` per **AD-4** — **not** `ClientNotFound`.
- When API returns **403** with `INSUFFICIENT_SCOPE`, toolkit **MUST** keep existing `InsufficientScope` mapping (distinct from access-denied).
- Toolkit **MUST NOT** map 403 access-denial to a “client not found (404)”-style success narrative for agents.

**Agent-testable checks:**

- Unit test: `map_error(403, { code: "PERMISSION_DENIED_VIEW_CLIENT" })` → `ClientAccessDenied`, not `ClientNotFound`.
- Unit test: `map_error(403, { code: "INSUFFICIENT_SCOPE" })` → `InsufficientScope` (unchanged).
- Skill remediation table includes 403 `PERMISSION_DENIED_VIEW_CLIENT` → verify access + mgr scopes; 403 `INSUFFICIENT_SCOPE` → re-auth scopes.

### R5 — Auth / scopes

- Mgr operations continue to require `xion:mgr:read` / `xion:mgr:write` as today.
- Toolkit **MUST NOT** call `/auth/sdk-session` (no shims, no docs teaching it).

### R6 — Skills & docs

- `verona-oauth2-client` schemas/examples **MUST** match R1–R3 and **Warning UX** (deprecated keys documented as ignored with stderr warning).
- `verona-treasury` **MUST** cross-link sync workflow when redirect/params change.
- `docs/cli-reference` oauth2 client sections **MUST** match CLI behavior, including warn + discard and the sync command.

## Agent migration story

1. **Stop** requiring or documenting `--redirect-uris` on create.
2. **Ensure** treasury on-chain params are correct before create (`redirect_url`, `icon_url`, `display_url`).
3. **Create** with `--treasury` only; read stderr for any ignored legacy flags.
4. **After** treasury param changes, run `oauth2 client sync-from-treasury <client_id>`.
5. **Login / authorize:** `redirect_uri` in authorize flow must equal treasury `redirect_url`.
6. **On 403** from client GET: refresh auth with mgr scopes; do not assume client id is wrong solely from “not found” wording.

## Non-requirements

- oauth2-api-service Worker deploy, mainnet ops, or bulk sync (`OPS_BULK_SYNC_SECRET`, ops runbooks).
- Multi-redirect URI on treasury.
- sdk-session compatibility shims or re-enablement docs.
- Changing PKCE callback host binding in toolkit code (document-only: authorize URI must match treasury).

## Definition of done (product)

- R1–R6 satisfied with agent-testable checks above.
- No in-repo skill/doc example presents free-form redirect create as SSOT.
- Local CI gate green (`fmt`, `clippy -D warnings`, `cargo test --all-features`).
- Testnet-2 live smoke optional; not merge-blocking (V1).

## Acceptance mapping

| Requirement | Primary verification |
|-------------|----------------------|
| R1 | Unit: create JSON omits chain-mapped fields; CLI test: create without `--redirect-uris`; stderr warning when deprecated flags present |
| R2 | Unit: update JSON omits chain-mapped fields; CLI test: warn on `--logo-uri` etc. |
| R3 | Unit/integration: sync HTTP path; CLI subcommand; skill grep for `sync-from-treasury` |
| R4 | Unit: `PERMISSION_DENIED_VIEW_CLIENT` → `ClientAccessDenied`; `INSUFFICIENT_SCOPE` unchanged; no false 404 narrative |
| R5 | Static grep: no `sdk-session`; mgr scope checks unchanged |
| R6 | Skill schema review; `docs/cli-reference` section review; repo grep hygiene (plan Task 4) |
| Warning UX | CLI tests assert `[WARNING]` on stderr, exit 0, clean success JSON on stdout |

## Interfaces (implementer SSOT)

### Types (`src/api/mgr_api.rs`)

| Symbol | Role |
|--------|------|
| `CreateClientRequest` | Create body; `binded_treasury` → JSON `bindedTreasury` |
| `UpdateClientRequest` | Update body; chain-mapped fields stripped before send |
| `ClientResponse` | Sync + get/update success envelope |
| `strip_chain_mapped_create` | **New** — mutates create request; returns discarded labels |
| `strip_chain_mapped_update` | **New** — mutates update request; returns discarded labels |
| `MgrApiClient::sync_from_treasury` | **New** — `POST .../sync-from-treasury`, no body |
| `MgrApiClient::map_error` | Add `(403, "PERMISSION_DENIED_VIEW_CLIENT")` arm per **AD-4** |

### CLI (`src/cli/oauth2_client.rs`)

| Symbol | Role |
|--------|------|
| `CreateClientArgs::treasury` | Maps to `binded_treasury`; required when not using `--json-input` |
| `CreateClientArgs::redirect_uris` | Optional after adapt; triggers warn + discard when non-empty |
| `handle_create` / `handle_update` | Parse → strip → `print_warning` → API call (order per **AD-1**) |
| `OAuth2ClientCommands::SyncFromTreasury` | **New** subcommand variant |
| `print_warning` | `src/utils/output.rs` — `eprintln!("[WARNING] {}", message)` |

### Chain-mapped field set (strip targets)

**Create:** `redirect_uris`, `logo_uri`, `client_uri`, `owner`  
**Update:** `redirect_uris`, `logo_uri`, `client_uri`

## Status

**`locked`** — product-reviewed 2026-07-16; architect technical pass 2026-07-16; writing review 2026-07-16. Implementers proceed from **Architecture decisions** and **Interfaces**; changes require a new spec revision.
