# AGENTS.md — harness (`.mstar/`)

Rules for the harness subtree only. Rust CLI build, test, and security rules live in the repository root `AGENTS.md`.

## Path symbols (this repository)

| Symbol | Path |
|--------|------|
| `{HARNESS_DIR}` | `.mstar/` |
| `{PLAN_DIR}` | `.mstar/plans/` |
| `{KNOWLEDGE_DIR}` | `.mstar/knowledge/` |
| `{ITERATION_DIR}` | `.mstar/iterations/` |
| `{SPECS_DIR}` | `.mstar/specs/` |

> **Project override:** This repo keeps frozen specs/ADRs under `.mstar/specs/`, not a repository-root `specs/` or `designs/` directory. Do not create root-level `specs/`.

Human-facing product docs stay in `docs/`. Harness state, plans, knowledge, specs, and QC reports stay under `{HARNESS_DIR}`.

## Layout

| Location | Purpose |
|----------|---------|
| `docs/` | Install, CLI reference, error codes, release process |
| `{HARNESS_DIR}/status.json` | Plan rows, roadmap pointers, `residual_findings` SSOT |
| `{PLAN_DIR}/` | Active implementation plans |
| `{PLAN_DIR}/archived/` | Completed plans (`archived/README.md` index) |
| `{PLAN_DIR}/reports/<plan-id>/` | QC / review reports (not `docs/`) |
| `{PLAN_DIR}/residuals/<plan-id>/` | Optional long-form open residual notes |
| `{SPECS_DIR}/` | Locked normative specs / ADRs (`primary_spec` / `spec_refs`) |
| `{KNOWLEDGE_DIR}/` | Durable implementation notes from **compound** (`README.md` index) |
| `{ITERATION_DIR}/` | Iteration packages (`README.md` index); per-iteration `specs/` = drafts only |

### Specs vs knowledge

| Tree | Who writes | Contents |
|------|------------|----------|
| `{SPECS_DIR}/` | Prepare / Review & Edit (product, architect) | Locked contracts agents implement against |
| `{KNOWLEDGE_DIR}/` | **`mstar-compound`** @ iteration-close | Crystallized conventions, how-we-work notes (may *link* to a locked spec) |

Do not store dynamic plan progress in root `AGENTS.md` or `CLAUDE.md`.

## Status and roadmap

- Read and update `{HARNESS_DIR}/status.json` for active `plans[]` and open `residual_findings`.
- Index pointers: `status.json` → `metadata.active_roadmap`, `metadata.knowledge_index`, `metadata.archived_plans_index`.
- Per-plan assignment templates: colocate with the plan; archive alongside it when the plan is `Done` (see `{PLAN_DIR}/archived/`).
- Register new knowledge in `{KNOWLEDGE_DIR}/README.md`.

## Working on harness artifacts

1. Confirm plan status in `status.json` before large edits.
2. Put QC conclusions under `{PLAN_DIR}/reports/<plan-id>/`.
3. Move completed main plans to `{PLAN_DIR}/archived/` and update `archived/README.md`.
4. Open residual findings: root `residual_findings[<plan-id>]` only (no dual-write to plan metadata).

## Git

Commit harness changes with the plan or docs scope (for example `docs(plan): ...`). Do not git-ignore the whole `{HARNESS_DIR}`; clones need handoff artifacts.
