---
iteration_id: eng-1880-oauth2-010-adapt
start_date: 2026-07-16
end_date: 2026-07-16
status: completed
iteration_base_branch: main
target_branch: main
plans:
  - eng-1880-oauth2-010-adapt
---

# eng-1880-oauth2-010-adapt Delivery Compass

## Problem & user value

**Problem:** oauth2-api-service **0.1.0** (ENG-1880) removes free-form redirect/logo/client URI writes from mgr-api. The toolkit still requires `--redirect-uris` on create and teaches that flow in skills—misleading agents and humans after deploy.

**User value:** One treasury-first workflow: set redirect/branding on-chain, create client bound to treasury, refresh mgr-api view via `sync-from-treasury`. Deprecated CLI flags warn and are stripped so legacy scripts degrade safely without false success on redirect changes.

## Scope

Align **verona-agent-toolkit** (Rust CLI + bundled skills) with **oauth2-api-service 0.1.0** security/breaking changes from [ENG-1880](https://linear.app/burnt/issue/ENG-1880) (sdk-session impersonation → unauthorized `redirect_uri`).

Locked spec points:

- Mgr client **create/update** no longer treat free-form `redirectUris` / `logoUri` / `clientUri` / `owner` as writable SSOT; treasury on-chain params are SSOT.
- Add **`sync-from-treasury`** client command + API client method.
- CLI: deprecated chain-mapped flags / JSON fields → **warning + discard** (not hard-fail); warnings on **stderr** via `[WARNING]`, success JSON on stdout unchanged (see spec **Warning UX**).
- Skills: `verona-oauth2-client` + `verona-treasury` cross-links; schemas/examples updated.
- Error mapping: unknown/inaccessible GET client → **403** `PERMISSION_DENIED_VIEW_CLIENT` → toolkit `ClientAccessDenied` (`EOAUTHCLIENT020`); distinct from **403** `INSUFFICIENT_SCOPE` (see spec AD-4).
- This toolkit does **not** use `/auth/sdk-session`; no sdk-session migration needed for login.

## Plans

| plan_id | Name | Status | Notes |
|---------|------|--------|-------|
| eng-1880-oauth2-010-adapt | OAuth2 0.1.0 client SSOT adapt (CLI + skills) | Done | Single plan; SDD multi-task |

Status values: `Todo` | `InProgress` | `InReview` | `Done` | `Blocked`

## Milestones

| Milestone | Target date | Status |
|-----------|-------------|--------|
| Spec freeze (Phase 1 lock) | 2026-07-16 | done |
| Dev complete | 2026-07-16 | done |
| QC complete | 2026-07-16 | done |
| Iteration close | 2026-07-16 | done |

## Acceptance Criteria

All items are **agent-testable** (automated test, CLI invocation, or repo grep unless noted).

### Create & update (R1–R2)

- [x] `oauth2 client create --treasury <addr> --client-name X` succeeds **without** `--redirect-uris`.
- [x] When chain-mapped flags or `--json-input` keys are present, CLI exits **0** (if otherwise valid), emits **`[WARNING]` on stderr** per field group, and outbound POST/PUT JSON **omits** those keys (unit-tested serialization).
- [x] Non-mapped metadata fields (`clientName`, `policyUri`, etc.) still update via `oauth2 client update`.

### Sync (R3)

- [x] `verona-toolkit oauth2 client sync-from-treasury <client_id>` calls `POST /mgr-api/clients/{id}/sync-from-treasury`.
- [x] Skills document treasury param change → sync (not redirect flags on update).

### Errors (R4–R5)

- [x] GET client (and related mgr reads) map API **403** `PERMISSION_DENIED_VIEW_CLIENT` to `ClientAccessDenied` / `EOAUTHCLIENT020` — **not** `ClientNotFound` / 404 narrative.
- [x] **403** `INSUFFICIENT_SCOPE` still maps to `InsufficientScope` / `EOAUTHCLIENT010` (unchanged).
- [x] No code, skill, or doc references `/auth/sdk-session` as supported.

### Skills & docs (R6)

- [x] `skills/verona-oauth2-client` (+ schemas): no required `redirect-uris`; deprecated keys marked ignored/warned; sync workflow in examples.
- [x] `skills/verona-treasury`: cross-link to sync when `redirect_url` / branding params change.
- [x] `docs/cli-reference` oauth2 client sections match warn + discard and sync behavior.

### Verification (V1)

- [x] Local gate: `cargo fmt -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-features`.
- [x] Unit tests cover discard serialization, sync path, 403 mapping, stderr warning path.
- [x] Testnet-2 live smoke (create → sync → authorize URI match): **optional**, not merge-blocking.

## Non-Goals

- **Worker / ops:** oauth2-api-service deploy, mainnet rollout, bulk sync, `OPS_BULK_SYNC_SECRET`, or ops-only scripts in this repo.
- **sdk-session:** Re-introducing, wrapping, or documenting `/auth/sdk-session`.
- **Multi-URI treasury:** Multiple redirect URLs on-chain (deferred product iteration).
- **PKCE host changes:** Altering local callback binding in toolkit code—only document that authorize `redirect_uri` must match treasury `redirect_url`.
- **Unrelated skills:** Full rewrite of `verona-asset`, `verona-faucet`, `verona-dev`, `verona-toolkit-init` unless grep shows they still teach free-form client redirect create/update.

## Roadmap position

- **Current iteration（eng-1880-oauth2-010-adapt）**：`delivered` — toolkit + skills aligned to oauth2-api-service 0.1.0 treasury SSOT (ENG-1880).
- **Next iteration**：Optional Testnet-2/mainnet live smoke after API mainnet deploy; multi-URI treasury if product demands. Trigger: ENG-1880 mainnet complete or user request. Owner: project-manager.
- **Final goal**：Agents manage OAuth clients with treasury as redirect/branding SSOT.

## Delivery Branch Policy

> Mirror of frontmatter; keep in sync with `{HARNESS_DIR}/status.json` `metadata`.

| Field | Value |
|-------|-------|
| `iteration_base_branch` | `main` |
| `spec_integration_branch` | `iteration/eng-1880-oauth2-010-adapt` |
| `target_branch` | `main` |

## Direction lock decisions (grill-me)

| Decision | Choice |
|----------|--------|
| Scope | **A** — CLI + mgr API + skills + docs |
| Deprecated fields UX | **R′** — warning + discard (not hard-fail) |
| Warning visibility | **stderr `[WARNING]`** via `print_warning`; success JSON unchanged (spec **Warning UX**) |
| clap deprecation | **Flags stay visible**; `--redirect-uris` optional (spec AD-2) |
| 403 mapping | **`PERMISSION_DENIED_VIEW_CLIENT`** → `ClientAccessDenied` (`EOAUTHCLIENT020`); distinct from `INSUFFICIENT_SCOPE` (spec AD-4) |
| Plan packaging | **P1** — single plan, multi-task SDD |
| Branch policy | `main` → `iteration/eng-1880-oauth2-010-adapt` → `main` |
| Skills surface | **K1** — `verona-oauth2-client` + `verona-treasury` cross-links |
| Verification | **V1** — local CI/unit/skills; Testnet-2 optional |

## Risk Register

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| API still rejecting bodies that include discarded fields if we strip too late | Med | High | Strip in CLI via shared `strip_chain_mapped_*` **before** `MgrApiClient` serialize; unit-test both flag and `--json-input` paths (plan Global Constraints) |
| `--json-input` bypasses strip → double-send of chain-mapped keys | Med | High | Same strip helpers for flag + JSON paths; test json_input with `redirectUris` present |
| Agents ignore stderr warnings and assume redirect updated | Med | Med | Skills: stderr warnings are actionable; examples omit deprecated flags; treasury → sync workflow |
| GET 403 vs 404 breaks existing scripts | Low | Med | Map 403 distinctly; skill remediation table |
| Testnet vs local API version skew | Low | Low | V1 does not block on live; document optional smoke |

## Iteration package

| Path | Purpose |
|------|---------|
| `guides/` | ENG-1880 impact notes, migration checklist |
| `specs/` | Iteration-scoped drafts (if any beyond `{SPECS_DIR}` = `.mstar/specs/`) |
| `README.md` | Package index |

## Quality Gate Summary

| plan_id | QC decision | QA gate | Residuals | Durable summary |
|---------|-------------|---------|-----------|-----------------|
| eng-1880-oauth2-010-adapt | Approve with residuals | Accept with residuals (mandatory) | R1 accepted_risk, R2 deferred | plan `#review-gate-summary`; open SSOT `status.json` residual_findings |

## Compound round summary

- Documents crystallized: 1 (`.mstar/knowledge/conventions/oauth2-api-010-client-ssot.md`)
- Package triage: **Promote** `guides/eng-1880-impact-and-migration.md` → conventions knowledge; **Keep** compass/README as iteration history; **Skip** empty iteration `specs/` dir; locked contract lives in `.mstar/specs/`
- New `CONCEPTS.md` entries: none (file not present; no new domain terms beyond existing OAuth/treasury vocabulary)
- `compound-refresh` triggered: no

## Iteration retrospective (minimal)

- What went well: Clear grill-me lock (R′ warn+discard); SDD sticky implementer kept context; QC warnings fixed in one wave before QA.
- What to improve: Skill docs claimed `display_url` on update-params early — catch CLI capability drift in Task 3 self-review.
- Suggested next iteration: Optional Testnet-2 smoke; consider strip at `MgrApiClient` boundary (R1).
