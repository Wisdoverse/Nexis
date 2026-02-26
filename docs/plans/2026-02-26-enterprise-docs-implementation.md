# Enterprise Documentation Standard Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Upgrade project documentation to enterprise-grade bilingual standards, with complete governance docs and validated links.

**Architecture:** Documentation is split between root governance docs and `docs/` technical docs. We will standardize structure, format, and cross-linking incrementally with category-level commits for traceability.

**Tech Stack:** Markdown, Git, shell tooling (`rg`, `find`, `git`).

---

### Task 1: README modernization

**Files:**
- Modify: `README.md`
- Create: `docs/assets/nexis-overview.svg`

**Step 1: Write target sections**
- Add bilingual title nav, badges, feature matrix, install paths, screenshot/GIF section.

**Step 2: Verify commands exist**
Run: `rg -n "name = \"nexis-(cli|gateway)\"" crates/*/Cargo.toml`
Expected: matching crate names found.

**Step 3: Add image asset**
- Create local SVG visual preview for README embedding.

**Step 4: Validate links**
Run: `rg -n "\]\(" README.md`
Expected: all links are intentional and resolvable.

**Step 5: Commit**
`git add README.md docs/assets/nexis-overview.svg && git commit -m "docs(readme): upgrade to enterprise bilingual standard"`

### Task 2: CHANGELOG standardization

**Files:**
- Modify: `CHANGELOG.md`

**Steps:**
1. Ensure Keep a Changelog sections (`Added`, `Changed`, `Fixed`, `Security`).
2. Add release link refs and Chinese notes.
3. Commit with docs(changelog) message.

### Task 3: Code of Conduct creation

**Files:**
- Create: `CODE_OF_CONDUCT.md`

**Steps:**
1. Add Contributor Covenant 2.1 canonical English text.
2. Add Chinese quick summary and reporting channels.
3. Commit.

### Task 4: Contribution guide update

**Files:**
- Create/Modify: `CONTRIBUTING.md`
- Modify: `docs/CONTRIBUTING.md`

**Steps:**
1. Add setup, coding standards, PR workflow in bilingual format.
2. Keep docs/ copy as short pointer page to root guide.
3. Commit.

### Task 5: Security policy update

**Files:**
- Modify: `SECURITY.md`

**Steps:**
1. Add bilingual policy with scope, supported versions, SLA, disclosure process.
2. Keep profile references in `docs/security/*`.
3. Commit.

### Task 6: docs/index and structure completion

**Files:**
- Modify: `docs/index.md`
- Create: `docs/development/guide.md`

**Steps:**
1. Build nav hub with quick links and bilingual sections.
2. Ensure required directories have at least one discoverable doc.
3. Commit.

### Task 7: Link verification

**Files:**
- Modify as needed from findings

**Steps:**
1. Run local internal-link checker for markdown files.
2. Fix broken links.
3. Commit results.
