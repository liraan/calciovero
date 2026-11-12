---
name: Module revision
about: Propose a revision to an existing module (version bump, invariant change, or clarification)
title: "[revision] L{level}.{NN} — short description"
labels: ["module-revision"]
assignees: []
---

# Module revision

This template is for **proposing a revision to an existing module** (a file under `specs/`). It is not for:

- Creating a new module — use a regular PR with the "New cell" type, see `.github/PULL_REQUEST_TEMPLATE.md`.
- Reporting a bug — use `bug_report.md`.
- Proposing an architectural decision — write an ADR in `doctrine/adr/` instead.

A revision can be **minor** (clarification, typo, added example) or **major** (changes an invariant, affects downstream modules). The distinction is in `doctrine/STRUCTURE.md` §4 ("Version policy").

---

## 1. Module to revise

<!--
  Which module? Give the file path and the module_id.
  Example:
    - File: L0-match/specs/04-match-engine.md
    - module_id: L0.04
    - Current version: 1.2
-->

- **File:**
- **module_id:**
- **Current version:**
- **Current status** (draft / review / ratified):

---

## 2. Type of revision

<!-- Check one. -->

- [ ] **Minor** — clarification, typo, added example, no semantic change. No ADR needed.
- [ ] **Major** — changes an invariant, affects downstream modules. **Requires an ADR** (see §4).
- [ ] **Status transition** — e.g. `draft` → `review`, `review` → `ratified`.
- [ ] **Superseding** — this module is replaced by a new one. Requires a new module and an ADR.

---

## 3. What is being changed

<!--
  Describe the change. Be concrete: which section, which paragraph, which
  table row. If it is a semantic change, explain the old semantics and the
  new semantics.
-->

### Current content

<!--
  Quote or summarise the current text/specification that is changing.
  A link to the section is not enough: the reviewer needs to see the
  current state without opening another tab.
-->



### Proposed content

<!--
  What should it say instead? Be explicit. If the change is structural,
  describe the new structure.
-->



---

## 4. Why (the motivation)

<!--
  The WHY is mandatory. A revision without a why is a draft, not a proposal.
  See CONTRIBUTING.md: "Explain the WHY."
-->

### What problem does this solve?



### What happens if we don't do this?



---

## 5. ADR (only for major revisions)

<!--
  A major revision changes an invariant. Invariants are protected by the
  doctrine. To change one, you need an ADR.

  If this is a minor revision, write "N/A" here.
-->

- **ADR (proposed or accepted):** <!-- link to the ADR file, e.g. doctrine/adr/016-xyz.md -->
- **ADR status:** <!-- proposed / accepted -->
- **If the ADR is still proposed:** this revision cannot be merged until the ADR is accepted. See CONTRIBUTING.md §"Open ADRs block code".

---

## 6. Downstream impact

<!--
  Which other modules read from this one? A major revision may break them.
  Check the `referenced_by` field in the module's frontmatter.
  Check whether the change affects the Three Planes, the determinism
  rules, or the banned things (see 00-foundations §3, §4, §5).
-->

### Modules that depend on this one

<!--
  List them, with the reason. Example:
    - L0.05 (match events) reads the MatchEvent type
    - L0.07 (save) reads the MatchResult type
-->



### Effect on the doctrine

<!--
  Does this revision affect the Three Planes, the determinism rules, or
  the banned things? If yes, an ADR is mandatory (see §5).
-->

- [ ] This revision does not affect the doctrine.
- [ ] This revision affects the doctrine. ADR linked in §5.

### Effect on the save format

<!--
  Does this revision change what is saved? If yes, a migration is needed.
  See L0-match/specs/07-save-minimal.md §2.3.
-->

- [ ] This revision does not change the save format.
- [ ] This revision changes the save format. Migration plan: <!-- link or description -->

---

## 7. Version bump

<!--
  Per doctrine/STRUCTURE.md §4: minor for clarifications, major for invariant changes.
  If this is a major bump, a doctrine ADR is required (see §5).
-->

- **Current version:**
- **Proposed version:**
- **Bump type:** <!-- minor / major -->

---

## 8. Checklist

- [ ] I have read `doctrine/00-foundations.md` to check that my proposal does not contradict the doctrine.
- [ ] If this is a major revision, an ADR is linked in §5 and it is accepted (or this revision is the ADR ratification).
- [ ] If this revision affects `referenced_by`, I have listed the downstream modules in §6.
- [ ] If this revision changes the save format, a migration plan is described in §6.
- [ ] I have bumped the version in the frontmatter (see §7).
- [ ] I have updated `last_updated` in the frontmatter.
- [ ] I have updated `doctrine/_index.md` if the module's status changed.
- [ ] I have not bundled unrelated changes (one revision per PR).

---

## 9. Notes for the reviewer

<!--
  Anything the reviewer should know: a tricky trade-off, an open question,
  a part of the module you are unsure about.
-->



---

<!--
  This template is for revisions to existing modules. For new modules,
  see .github/PULL_REQUEST_TEMPLATE.md. For bugs, see bug_report.md.
  For architectural decisions, write an ADR.
-->
