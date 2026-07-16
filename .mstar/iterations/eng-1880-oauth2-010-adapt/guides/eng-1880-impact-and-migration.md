Promoted to: `.mstar/knowledge/conventions/oauth2-api-010-client-ssot.md` (iteration-close compound)

# ENG-1880 → verona-agent-toolkit impact

**Upstream:** [ENG-1880](https://linear.app/burnt/issue/ENG-1880) / oauth2-api-service **0.1.0** (PR #49).  
**Reference tree:** `~/workspace/burnt-labs/oauth2-api-service`  
**Toolkit contract:** `specs/oauth2-api-010-client-ssot.md` (`locked`)

## Root cause (upstream)

Unauthenticated `POST /auth/sdk-session` could mint Session JWTs for arbitrary `xion1` addresses; session-mode mgr-api lacked mgr scopes; callers could write free-form `redirectUris` into OAUTH_KV; authorize accepted those URIs.

## Upstream fix summary (0.1.0)

| Area | Change |
|------|--------|
| Auth | `POST /auth/sdk-session` → **410** `SDK_SESSION_DISABLED` |
| Scopes | Session JWT embeds `grantedScopes`; mgr-api needs `xion:mgr:read` / `xion:mgr:write` |
| Create | No free-form redirect/logo/clientUri/owner; from treasury / caller |
| Update | PUT cannot write those fields; use `POST .../sync-from-treasury` |
| Authorize | Fail-closed if `redirect_uri` ≠ on-chain treasury `redirect_url` |
| GET client | Missing / no access → **403** `PERMISSION_DENIED_VIEW_CLIENT` (not 404) |

## Toolkit impact

| Area | Pre-0.1.0 toolkit | Required adapt |
|------|-------------------|----------------|
| Login / PKCE | Does not call sdk-session | No sdk-session work |
| `oauth2 client create` | Requires `--redirect-uris`; sends chain-mapped fields | Require `bindedTreasury` (`--treasury`); warn + discard chain-mapped fields |
| `oauth2 client update` | Can send `redirectUris` / `logoUri` / `clientUri` | Warn + discard; use `sync-from-treasury` to refresh from treasury |
| GET client | May conflate missing/inaccessible with not-found semantics | Map `PERMISSION_DENIED_VIEW_CLIENT` → `ClientAccessDenied` (`EOAUTHCLIENT020`) |
| Skills | Teach free-form redirect create | Treasury-first + sync workflow |
| Warnings | N/A | stderr `[WARNING]` via `print_warning`; success JSON on stdout unchanged |

## Before / after (agent-facing)

### Create client

**Before (deprecated teaching):**

```bash
verona-toolkit oauth2 client create \
  --redirect-uris "https://myapp.com/callback" \
  --treasury "xion1abc..." \
  --client-name "My DApp"
```

**After (0.1.0 SSOT):**

```bash
# 1. Set redirect on treasury first (verona-treasury)
verona-toolkit treasury update-params xion1abc... \
  --redirect-url "https://myapp.com/callback"

# 2. Create — treasury binding only
verona-toolkit oauth2 client create \
  --treasury "xion1abc..." \
  --client-name "My DApp"
```

If an agent passes legacy flags, expect stderr such as:

`[WARNING] --redirect-uris is ignored; set redirect_url on treasury and run sync-from-treasury.`

Exit code remains `0` when create otherwise succeeds.

### Change redirect later

**Before:** `oauth2 client update ... --redirect-uris ...` (no longer valid SSOT)

**After:**

```bash
verona-toolkit treasury update-params xion1abc... \
  --redirect-url "https://myapp.com/new-callback"
verona-toolkit oauth2 client sync-from-treasury client_abc123
```

## Agent migration checklist

1. Set treasury `redirect_url`, `icon_url`, and `display_url` on-chain (`treasury create` or `update-params`).
2. `oauth2 client create --treasury <addr> ...` — do **not** rely on `--redirect-uris` or other chain-mapped flags.
3. To change redirect/branding later: update treasury on-chain → `oauth2 client sync-from-treasury <client_id>`.
4. Ensure login authorize `redirect_uri` equals treasury `redirect_url`.
5. Re-auth if the session/token lacks `xion:mgr:*` scopes (`--dev-mode` or full OAuth with mgr scopes).
6. On client GET **403** `PERMISSION_DENIED_VIEW_CLIENT`: treat as access denial (client may not exist or you lack permission) — **not** proof the id is wrong. On **403** `INSUFFICIENT_SCOPE`: re-auth with mgr scopes.

## Troubleshooting (skills / agents)

| Symptom | Likely cause | Action |
|---------|--------------|--------|
| stderr `[WARNING]` about ignored redirect/logo flags | Legacy flags still passed | Update treasury + sync; remove deprecated flags from automation |
| Authorize fails redirect mismatch | `redirect_uri` ≠ treasury `redirect_url` | Align treasury param and sync client |
| GET client 403 `PERMISSION_DENIED_VIEW_CLIENT` | Missing client or no access (enumeration-safe) | Verify ownership; re-auth with `xion:mgr:read`; do not assume wrong client id |
| GET client 403 `INSUFFICIENT_SCOPE` | Token lacks mgr scope | Re-auth with `--dev-mode` or full OAuth including mgr scopes |
| Create succeeds but wrong callback | Redirect never set on treasury | Fix treasury `redirect_url`, then sync |
| Agent assumes redirect updated from CLI flags | Ignored fields after warn + discard | Read stderr; follow treasury → sync path |

## Out of scope for this migration

- Deploying oauth2-api-service Worker or running ops bulk sync.
- sdk-session re-enablement (toolkit never used it for login).
- Multi-redirect treasury support.
