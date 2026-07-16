---
title: OAuth2 mgr client treasury SSOT (API 0.1.0)
category: conventions
status: Active
created_at: 2026-07-16
updated_at: 2026-07-16
source: iteration:eng-1880-oauth2-010-adapt/guides/eng-1880-impact-and-migration.md
linear: ENG-1880
promoted_from: .mstar/iterations/eng-1880-oauth2-010-adapt/guides/eng-1880-impact-and-migration.md
normative_spec: .mstar/specs/oauth2-api-010-client-ssot.md
---

# Convention: OAuth2 mgr client — treasury SSOT (0.1.0)

> **Compound crystallization** (iteration-close). Normative locked contract: [`.mstar/specs/oauth2-api-010-client-ssot.md`](../../specs/oauth2-api-010-client-ssot.md). This file is the how-we-work summary for agents; change behavior only via the locked spec + code.

## Problem

oauth2-api-service **0.1.0** (ENG-1880) closed free-form `redirectUris` writes on mgr-api. Toolkit create/update and skills previously taught required `--redirect-uris`, which breaks agents after deploy and re-opens a false mental model of redirect injection via CLI.

## Decision

1. **Treasury on-chain params are SSOT** for redirect / logo / display mappings (`redirect_url`, `icon_url`, `display_url`).
2. **Create** binds `bindedTreasury` only for those fields; CLI does not require `--redirect-uris`.
3. **Deprecated chain-mapped CLI/JSON fields** → **warn + discard** (stderr `[WARNING]` via `print_warning`), then HTTP without those keys.
4. **Change redirect later** → update treasury on-chain → `oauth2 client sync-from-treasury <client_id>`.
5. **GET inaccessible/missing client** may return **403** `PERMISSION_DENIED_VIEW_CLIENT` → toolkit `EOAUTHCLIENT020` (`ClientAccessDenied`), not `EOAUTHCLIENT012` (404).

## What did not work / avoid

- Hard-failing solely because deprecated flags are present (rejected in direction lock — breaks legacy scripts unnecessarily).
- Teaching `display_url` via `treasury update-params` when CLI has no `--display-url` (set at create / on-chain outside that CLI surface).
- Calling `/auth/sdk-session` (removed upstream; toolkit never used it for login).

## Implementation anchors

- Locked spec: `.mstar/specs/oauth2-api-010-client-ssot.md`
- Strip helpers: `strip_chain_mapped_*` in `src/api/mgr_api.rs`
- CLI pipeline: `prepare_*_request` in `src/cli/oauth2_client.rs`
- Skills: `skills/verona-oauth2-client/`, treasury cross-link in `skills/verona-treasury/`

## Residual

- Strip is enforced at CLI boundary today (`MgrApiClient` trusts stripped structs) — accepted risk while CLI is the sole caller.
