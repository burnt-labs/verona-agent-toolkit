---
name: verona-oauth2-client
description: "OAuth2 client lifecycle on Verona/Xion via Manager API (create, list, update, delete, managers, transfer). Use when registering OAuth apps or managing OAuth2 clients. Requires verona-oauth2 (dev-mode for Manager scopes)."
metadata:
  author: burnt-labs
  version: "1.1.0"
  requires:
    - verona-toolkit-init
    - verona-oauth2
compatibility: Requires verona-toolkit CLI >=0.9.0 and OAuth2 authentication
---

# verona-oauth2-client

OAuth2 client management skill for Xion blockchain. Enables dApp registration and lifecycle management through the MGR API.

## Triggers

OAuth2 client, OAuth client, client management, create OAuth app, register client, MGR API, client ID, sync-from-treasury, treasury SSOT, oauth2 client list/create/get/update/delete, managers, transfer-ownership, OAuth2 客户端, 客户端管理, 创建 OAuth 应用.

## Overview

OAuth2 clients are the bridge between dApps and Xion's MetaAccount system. Each client is bound to a Treasury and can have multiple managers for team collaboration.

**Key concepts:**
- **Client** — OAuth2 application registered with Xion
- **Treasury binding** — Every client must be bound to an existing Treasury; **treasury on-chain params are SSOT** for redirect and branding (API 0.1.0)
- **sync-from-treasury** — Refreshes mgr-api client view after treasury `redirect_url` / `icon_url` changes (and `display_url` read from chain at sync time)
- **Managers** — Team members who can manage the client (but not delete it)
- **Owner** — The sole user who can delete or transfer the client

## Treasury SSOT (API 0.1.0)

Redirect URIs, logo, and client homepage are **not** set via `oauth2 client create` or `update`:

1. Set `redirect_url` and `icon_url` via `verona-treasury` → `update-params` (`--redirect-url`, `--icon-url`). Set `display_url` at treasury **create** (`--config` JSON) if needed — update-params has no `--display-url` flag.
2. Create the OAuth client with `--treasury` only.
3. After treasury `redirect_url` or `icon_url` changes, run `oauth2 client sync-from-treasury <client_id>`.

**Warning UX:** If legacy flags (`--redirect-uris`, `--logo-uri`, etc.) or deprecated JSON keys are passed, the CLI prints `[WARNING]` on **stderr** and **discards** those fields before the API call. Exit code stays `0` when otherwise valid. **Stdout success JSON is unchanged** — agents must read stderr; redirect did **not** change via CLI flags.

## Relationship to Treasuries

OAuth clients are tightly coupled with Treasuries:
- Each client must specify a Treasury at creation time
- The Treasury handles fee payment for the client's gasless transactions
- The Treasury address cannot be changed after creation
- You must have admin access to the Treasury to create clients for it

> **Tip:** Create a Treasury first using the `verona-treasury` skill, then register your OAuth client.

## Prerequisites

1. `verona-toolkit` CLI installed (use `verona-toolkit-init` if not present)
2. **Authenticated** with `verona-oauth2` skill (required for all operations)
3. **Treasury exists** — Create one with `verona-toolkit treasury create` before creating clients

> **Known Limitation:** The toolkit's `auth login` access token does NOT include `xion:mgr:read`/`xion:mgr:write` scopes against the production testnet server. For local development, point at a local oauth2-api-service via `VERONA_TESTNET_OAUTH_API_URL` env var. Production scope additions require backend team coordination.

## Quick Start

```bash
# 1. Authenticate (required!)
verona-toolkit auth login

# 2. Set redirect on treasury (SSOT) — see verona-treasury
verona-toolkit treasury update-params xion1abc... \
  --redirect-url "https://myapp.com/callback"

# 3. Create OAuth client (treasury binding only — no --redirect-uris required)
verona-toolkit oauth2 client create \
  --treasury "xion1abc..." \
  --client-name "My DApp"

# 4. After treasury param changes, sync client view from chain
verona-toolkit oauth2 client sync-from-treasury client_abc123

# 5. Get client details
verona-toolkit oauth2 client get client_abc123
```

## Multi-Step Workflows

### Register a New DApp

Complete workflow for registering a new dApp with OAuth2 (treasury-first):

1. **Create a Treasury** (if you don't have one) with redirect URL on-chain
   ```bash
   verona-toolkit treasury create \
     --name "My DApp Treasury" \
     --redirect-url "https://myapp.com/callback"
   ```

2. **Register the OAuth client** (treasury binding only)
   ```bash
   verona-toolkit oauth2 client create \
     --treasury "xion1treasury..." \
     --client-name "My DApp" \
     --contacts "admin@myapp.com"
   ```

3. **Store the client secret securely** (shown once at creation)

4. **After changing treasury redirect/branding**, sync the client
   ```bash
   verona-toolkit treasury update-params xion1treasury... \
     --redirect-url "https://myapp.com/new-callback"
   verona-toolkit oauth2 client sync-from-treasury client_abc123
   ```

5. **Add managers** (optional)
   ```bash
   verona-toolkit oauth2 client managers add client_abc123 --manager-id user_456
   ```

### Manage Client Team

Add and remove managers for collaborative access:

```bash
# Add a manager
verona-toolkit oauth2 client managers add client_abc123 --manager-id user_456

# Remove a manager
verona-toolkit oauth2 client managers remove client_abc123 --manager-id user_456

# Update extension (managers list via extension)
verona-toolkit oauth2 client extension update client_abc123 --managers "user_a,user_b"
```

### Transfer Ownership

Transfer client ownership when team roles change:

```bash
verona-toolkit oauth2 client transfer-ownership client_abc123 --new-owner user_789 --force
```

> Only the current owner can transfer. The new owner must already have a Xion MetaAccount.
> The `--force` flag is required to confirm the ownership transfer.

### Rotate Client Secret

Rotate the client secret for confidential clients:

```bash
verona-toolkit oauth2 client rotate-secret client_abc123
```

> **Owner-only operation.** Only confidential clients (auth_method: `client_secret_basic` or `client_secret_post`) support secrets.
> The new secret is returned **only once** — store it securely immediately.
> Requires `--dev-mode` authentication for `xion:mgr:write` scope.

## Common Operations

### List OAuth Clients

```bash
verona-toolkit oauth2 client list
```

Output:
```json
{
  "success": true,
  "items": [
    {
      "clientId": "client_abc123",
      "clientName": "My App",
      "redirectUris": ["https://example.com/callback"],
      "bindedTreasury": "xion1treasury...",
      "owner": "user_123",
      "managers": ["user_456"]
    }
  ],
  "cursor": null,
  "count": 1
}
```

With pagination:
```bash
verona-toolkit oauth2 client list --limit 10 --cursor "next_page_cursor"
```

### Create OAuth Client

```bash
# Recommended: treasury already has redirect_url set
verona-toolkit oauth2 client create \
  --treasury "xion1abc..." \
  --client-name "My App"

# Non-chain-mapped metadata still supported
verona-toolkit oauth2 client create \
  --treasury "xion1treasury..." \
  --client-name "My App" \
  --managers "user_456,user_789" \
  --auth-method client_secret_basic \
  --contacts "admin@example.com" \
  --policy-uri "https://myapp.com/privacy"
```

> **Do not** teach `--redirect-uris`, `--logo-uri`, or `--client-uri` as SSOT. If passed, stderr shows `[WARNING]` and fields are discarded.

Output:
```json
{
  "success": true,
  "client": {
    "clientId": "client_abc123",
    "clientName": "My App",
    "redirectUris": ["https://myapp.com/callback"]
  },
  "clientSecret": "********"
}
```

Using JSON input (`bindedTreasury` required; deprecated keys stripped with stderr warning):
```bash
verona-toolkit oauth2 client create --json-input request.json
```

### Sync from Treasury

Refresh client redirect/branding from the bound on-chain treasury:

```bash
verona-toolkit oauth2 client sync-from-treasury client_abc123
```

Run after `treasury update-params` changes `redirect_url` or `icon_url`.

### Get OAuth Client

```bash
verona-toolkit oauth2 client get client_abc123
```

### Update OAuth Client

Update **non-chain-mapped** metadata only (`clientName`, `policyUri`, `tosUri`, `jwksUri`, `contacts`):

```bash
verona-toolkit oauth2 client update client_abc123 \
  --client-name "Updated Name" \
  --policy-uri "https://example.com/privacy"
```

To change redirect or branding: update treasury on-chain, then `sync-from-treasury` — not `--redirect-uris` / `--logo-uri` on update.

### Delete OAuth Client

```bash
verona-toolkit oauth2 client delete client_abc123 --force
```

> Only the owner can delete a client. This action is irreversible.
> The `--force` flag is required to confirm deletion.

### Extension Operations

```bash
# Get extension data
verona-toolkit oauth2 client extension get client_abc123

# Update managers via extension
verona-toolkit oauth2 client extension update client_abc123 --managers "user_a,user_b"
```

### Manager Operations

```bash
# Add manager
verona-toolkit oauth2 client managers add client_abc123 --manager-id user_456

# Remove manager
verona-toolkit oauth2 client managers remove client_abc123 --manager-id user_456
```

### Transfer Ownership

```bash
verona-toolkit oauth2 client transfer-ownership client_abc123 --new-owner user_789 --force
```

### Rotate Client Secret

```bash
verona-toolkit oauth2 client rotate-secret client_abc123
```

> **Note:** Only works for confidential clients (auth_method: `client_secret_basic` or `client_secret_post`).
> Owner-only operation. The new secret is returned **once** — save it immediately.

## Secret Redaction Policy

The `clientSecret` is **redacted by default** in all output (`"********"`).

To reveal the secret during creation:
```bash
verona-toolkit oauth2 client create ... --show-secret
```

**Security reminders:**
- Store the client secret securely immediately after creation (it's shown only once)
- Never commit secrets to version control
- The secret is required for `client_secret_basic` and `client_secret_post` auth methods

## Error Handling

All commands return JSON with a `success` field:

**Success:**
```json
{"success": true, "client": {...}}
```

**Error:**
```json
{
  "success": false,
  "error": {
    "code": "EOAUTHCLIENT012",
    "message": "Client not found",
    "remediation": "Check the client ID and try again"
  }
}
```

**Common Error Codes:**

| Code | Description | Remediation |
|------|-------------|-------------|
| `EOAUTHCLIENT008` | Not authenticated | Run `verona-toolkit auth login` first |
| `EOAUTHCLIENT010` | Insufficient scope (403 `INSUFFICIENT_SCOPE`) | Token lacks mgr scopes — re-auth with `xion:mgr:read` / `xion:mgr:write`; **not** the same as access denial |
| `EOAUTHCLIENT011` | Only owner allowed | Only the client owner can perform this action |
| `EOAUTHCLIENT020` | Client access denied (GET 403 `PERMISSION_DENIED_VIEW_CLIENT`) | Client may not exist **or** you lack access — **not** proof the ID is wrong; re-auth with `xion:mgr:read` |
| `EOAUTHCLIENT012` | Client not found (404 `CLIENT_NOT_FOUND`) | API confirms the client ID does not exist — distinct from `EOAUTHCLIENT020` enumeration-safe 403 on GET |
| `EOAUTHCLIENT014` | Treasury not found | Verify the treasury address is correct |
| `EOAUTHCLIENT015` | Internal server error | Retry later or contact support |

**Other Error Codes:**

| Code | Description |
|------|-------------|
| `EOAUTHCLIENT001` | Bad request — invalid parameters |
| `EOAUTHCLIENT002` | Client ID is required |
| `EOAUTHCLIENT003` | Redirect URIs required (upstream API only) | Set `redirect_url` on treasury before create; toolkit create does not require `--redirect-uris` |
| `EOAUTHCLIENT004` | Binded treasury is required |
| `EOAUTHCLIENT005` | Owner is required |
| `EOAUTHCLIENT006` | Invalid grant type |
| `EOAUTHCLIENT007` | Manager user ID is required |
| `EOAUTHCLIENT013` | Client extension not found |
| `EOAUTHCLIENT016` | Treasury fetch error |
| `EOAUTHCLIENT017` | Treasury query error |
| `EOAUTHCLIENT018` | Unknown network |

## Parameter Collection Workflow

Before executing any command, ensure all required parameters are collected.

### Step 1: Identify Operation

Determine which operation the user wants to perform (list, create, get, update, delete, etc.).

### Step 2: Check Parameter Schema

Refer to the `schemas/` directory for detailed parameter definitions.

### Step 3: Collect Missing Parameters

Collect ALL missing required parameters in a SINGLE interaction:

> Example for create:
> "I need the following to create the OAuth client:
> - Treasury address (must already exist with redirect_url set on-chain)
> - Client name (optional but recommended)
> - Auth method (optional, defaults to 'none')
>
> Do **not** collect `--redirect-uris` as required — treasury + sync is SSOT."

### Step 4: Confirm Before Execution

Present the parameters in a tree format and ask for confirmation:

```
Will execute: oauth2 client create
├─ Treasury: xion1abc...
├─ Client Name: My DApp
└─ Auth Method: none (default)
Confirm? [y/n]
```

## Parameter Schemas

See `schemas/` directory for detailed parameter definitions:

| Schema File | Command | Description |
|-------------|---------|-------------|
| `list.json` | `oauth2 client list` | List OAuth clients with pagination |
| `create.json` | `oauth2 client create` | Create new OAuth client (treasury SSOT) |
| `sync-from-treasury.json` | `oauth2 client sync-from-treasury` | Sync redirect/branding from treasury |
| `get.json` | `oauth2 client get` | Get client by ID |
| `update.json` | `oauth2 client update` | Update client metadata |
| `delete.json` | `oauth2 client delete` | Delete a client |
| `extension.json` | `oauth2 client extension get` | Get client extension data |
| `extension-update.json` | `oauth2 client extension update` | Update extension managers |
| `managers-add.json` | `oauth2 client managers add` | Add a manager |
| `managers-remove.json` | `oauth2 client managers remove` | Remove a manager |
| `transfer-ownership.json` | `oauth2 client transfer-ownership` | Transfer ownership |
| `rotate-secret.json` | `oauth2 client rotate-secret` | Rotate client secret |

## Troubleshooting

| Symptom | Likely cause | Action |
|---------|--------------|--------|
| stderr `[WARNING]` about ignored redirect/logo flags | Legacy flags still passed | Update treasury + `sync-from-treasury`; remove deprecated flags from automation |
| Authorize fails redirect mismatch | `redirect_uri` ≠ treasury `redirect_url` | Align treasury param and sync client |
| GET client 403 / `EOAUTHCLIENT020` | Missing client or no access | Verify ownership; re-auth with `xion:mgr:read`; do not assume wrong client id |
| GET client 403 / `EOAUTHCLIENT010` | Token lacks mgr scope | Re-auth with `--dev-mode` or full OAuth including mgr scopes |
| Create succeeds but wrong callback | Redirect never set on treasury | Fix treasury `redirect_url`, then sync |
| Agent assumes redirect updated from CLI flags | Ignored fields after warn + discard | Read stderr; follow treasury → sync path |

### Not Authenticated
```bash
verona-toolkit auth login
```

### Client Not Found
```bash
# List your clients first to verify the ID
verona-toolkit oauth2 client list
```

### Treasury Not Found
```bash
# Verify the treasury exists
verona-toolkit treasury list
```

### Insufficient Scope (EOAUTHCLIENT010)

The toolkit's standard auth token lacks MGR API scopes. For local development:
```bash
# Point to local oauth2-api-service
export VERONA_TESTNET_OAUTH_API_URL=http://localhost:8787
cargo build
verona-toolkit auth login
```

### GET 403 vs 404: EOAUTHCLIENT020 vs EOAUTHCLIENT012 vs EOAUTHCLIENT010

| Code | When | Action |
|------|------|--------|
| `EOAUTHCLIENT010` | 403 `INSUFFICIENT_SCOPE` | Re-auth with mgr read/write scopes — token problem, not client ID |
| `EOAUTHCLIENT020` | 403 `PERMISSION_DENIED_VIEW_CLIENT` on GET | Client may not exist or you lack access — list clients; re-auth with `xion:mgr:read`; **do not** assume wrong ID |
| `EOAUTHCLIENT012` | 404 `CLIENT_NOT_FOUND` | Client ID confirmed absent — verify spelling via `oauth2 client list` |

### Treasury Binding Error

You cannot change a client's treasury after creation. Ensure the treasury exists and you have admin access before creating the client.

## Related Skills

- **verona-dev** — Unified entry point for Xion development
- **verona-oauth2** — Authentication (use before this skill)
- **verona-treasury** — Treasury management (required for client creation)
- **verona-toolkit-init** — CLI installation (use if CLI not found)
- **xiond-usage** (xion-skills) — Chain-level queries

## Version

- Skill Version: 1.1.0
- Compatible CLI Version: >=0.13.0 (oauth2-api 0.1.0 client SSOT)
