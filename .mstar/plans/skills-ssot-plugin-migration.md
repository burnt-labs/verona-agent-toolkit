# Skills SSOT → verona-dev-plugin Migration

> **For agentic workers:** REQUIRED SUB-SKILL: Use `mstar-sdd`. Steps use checkbox (`- [x]`) syntax.

**Goal:** Make [burnt-labs/verona-dev-plugin](https://github.com/burnt-labs/verona-dev-plugin) the sole skills SSOT for agents; remove bundled `skills/` and skill-only CI/tests from this CLI repo; fully align install/routing docs with the plugin (including `verona-bin` instead of `xion-skills`).

**Architecture:** This repository remains the **verona-toolkit CLI** source. Agent skills live only in `verona-dev-plugin`. Install guidance must point at that repo’s [INSTALL.md](https://github.com/burnt-labs/verona-dev-plugin/blob/main/INSTALL.md) (manual host plugin / clone — **no** `npx skills add burnt-labs/verona-agent-toolkit`). Local skill mock tests did not exercise skill scripts (fixture/CLI JSON only) and are retired here.

**Tech Stack:** Markdown docs, GitHub Actions, Cargo bin cleanup, bash CI.

**Execution:** mstar-sdd

**plan_id:** `skills-ssot-plugin-migration`  
**Working branch:** `plan/skills-ssot-plugin-migration` (from `main`)  
**Merge target / PR target:** `main`  
**QA gate:** `mandatory` (CI graph + docs surface change)  
**Locked clarify:** B1 delete `skills/` tree + short pointer docs; full routing/install alignment; retire skill-package tests/CI.

## Global Constraints

- Skills SSOT URL: `https://github.com/burnt-labs/verona-dev-plugin`
- Install SSOT: plugin `INSTALL.md` (Cursor / Codex / Claude / Kimi). Do **not** document `npx skills add burnt-labs/verona-agent-toolkit` or `burnt-labs/xion-skills` as the agent install path.
- Chain / CosmWasm / `xiond` path in docs: **`verona-bin`** (inside the plugin), not a separate `xion-skills` install.
- Do **not** delete `~/.verona-toolkit/credentials` or run `auth logout`.
- Do **not** rewrite historical `.mstar/plans/archived/*` or old CHANGELOG entries for this migration (optional one-line note in active docs only).
- CLI binary install (curl installer / cargo) stays in this repo’s docs; only **skills/plugin** install moves.
- After removing `skills/`, leave either no `skills/` directory **or** a single `skills/README.md` pointer file — prefer **no package tree**; pointer primarily in `.agents/AGENTS.md` + user-facing install docs.
- Deferred (residual, not this plan): recreate CLI JSON fixture tests under `tests/cli/` or in the plugin — **out of scope**.

## Intent gate

| Item | Statement |
|------|-----------|
| Real goal | Agents and humans install Verona skills from `verona-dev-plugin`; this repo no longer hosts or CI-tests skill packages |
| Success | No `skills/verona-*` packages; no skills-mock/skills-lint CI; grep-clean of `npx skills add burnt-labs/verona-agent-toolkit` in active docs; install/routing point to plugin + `verona-bin` |
| Non-goals | Moving skill content into the plugin (already done); rewriting archived harness plans; migrating mock fixtures this round |

---

### Task 1: Remove skill packages and skill-only test/CI tooling

**Files:**
- Delete: entire `skills/` tree (`verona-*` packages)
- Delete: `tests/skills/` (framework, mocks, suites)
- Delete: `scripts/validate-skill-frontmatter.sh`, `src/bin/validate_skill_frontmatter.rs` (and Cargo.toml `[[bin]]` / deps used only by that bin)
- Modify: `.github/workflows/ci.yml` — remove `skills-mock-tests`, `skills-lint`, `skills-tests-status`; fix `needs:` of downstream jobs (`ci-status` / e2e coupling) so the pipeline still gates on remaining jobs
- Modify: any path references in active non-archived docs that assume local `skills/*/scripts` (handled fully in Task 2 if missed)

**Interfaces:**
- Consumes: locked clarify (B1)
- Produces: repo without skill packages; CI green without skills jobs

- [x] **Step 1:** Inventory and delete `skills/` packages (git rm)
- [x] **Step 2:** Delete `tests/skills/` and frontmatter validator bin/script; clean `Cargo.toml` / lockfile if needed
- [x] **Step 3:** Remove skills CI jobs and repair workflow `needs` graph
- [x] **Step 4:** Sanity — `rg` for `skills-mock-tests|validate-skill-frontmatter|tests/skills` in active CI/scripts; `cargo check` still passes
- [x] **Step 5:** Commit on `plan/skills-ssot-plugin-migration`

---

### Task 2: Full docs / agent-guide alignment to verona-dev-plugin

**Files (active surfaces — update all that still teach old install):**
- Modify: `README.md`, `INSTALL-FOR-AGENTS.md`, `docs/skills-guide.md`, `docs/configuration.md` (skills reinstall snippets), `.agents/AGENTS.md`, root `AGENTS.md` / `CLAUDE.md` (skills layer description + project structure), any other **non-archived** user/agent doc that still says `npx skills add burnt-labs/verona-agent-toolkit` or points agents to install from this repo / `xion-skills`
- Rewrite `docs/skills-guide.md` as a **pointer + CLI-consumer note**: skills live in the plugin; this guide no longer describes authoring/publishing packages from this repo (or shrink to “how toolkit CLI relates to plugin skills”)
- Align routing language: Development → `verona-dev` / `verona-bin`; Toolkit → leaf `verona-*` skills; install via plugin `INSTALL.md`

**Interfaces:**
- Consumes: Task 1 (no local skill packages)
- Produces: install/routing SSOT text matching plugin README + INSTALL.md

- [x] **Step 1:** Replace skill install sections with plugin clone / host steps (summarize + link to raw/blob INSTALL.md)
- [x] **Step 2:** Replace `xion-skills` recommendations with `verona-bin` inside the plugin
- [x] **Step 3:** Update `.agents/AGENTS.md` + root `AGENTS.md` / `CLAUDE.md` project structure so they no longer claim bundled `skills/` SSOT
- [x] **Step 4:** Grep-clean active docs for `npx skills add burnt-labs/verona-agent-toolkit`, `burnt-labs/xion-skills` (as install source), and “bundled skills live at skills/”
- [x] **Step 5:** Commit

---

## Plan self-review (PM)

1. **Spec coverage:** B1 delete + retire skill tests/CI + full install/routing alignment — covered by T1–T2
2. **Placeholder scan:** no TBD
3. **Type consistency:** `plan_id` / branch names consistent
4. **Roadmap:** fixture migration deferred → residual after QC if needed

## SDD runtime (ephemeral)

`{SDD_DIR}` = `.mstar/sdd/skills-ssot-plugin-migration/`

## QC gate summary

- **Verdict:** Approve with residuals (`665aaab..9464122`)
- **Tri:** qc1/qc2 Approve; qc3 Approve with residuals (W2/W4 fixed in `9464122`)
- **Open residuals:** R1 knowledge paths, R2 fixture migration, R3 optional polish — see `status.json` `residual_findings.skills-ssot-plugin-migration`
- **Bundle:** `.mstar/sdd/skills-ssot-plugin-migration/review/` (gitignored)


## QA gate summary

- **Verdict:** Accept with residuals
- **Report:** `.mstar/sdd/skills-ssot-plugin-migration/review/qa.md` (gitignored)
- **Open residuals:** R1–R3 in `status.json`
