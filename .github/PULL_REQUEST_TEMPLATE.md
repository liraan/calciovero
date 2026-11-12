# Pull Request

Thank you for contributing. Before submitting, please fill in the sections below.

---

## What does this PR do?

<!--
  One or two sentences. What changes, and why?

  Examples:
  - "Adds L1-season/00-overview.md (first cell of L1)."
  - "Fixes a broken link in L0-match/specs/07-save-minimal.md."
  - "Proposes ADR-008 on the calendar model."
-->



## Type of change

<!-- Check one. -->

- [ ] **New cell** (one module file, with frontmatter)
- [ ] **Revised cell** (version bump in the frontmatter, minor or major)
- [ ] **New ADR** (proposes a decision, `status: proposed`)
- [ ] **ADR ratification** (`status: proposed` → `status: accepted`)
- [ ] **Fix** (typo, broken link, factual correction — no semantic change)
- [ ] **Structure change** (README, STRUCTURE, CONTRIBUTING, CHANGELOG)
- [ ] **Other** (explain below)

If "Other", describe:



## Scope check

<!--
  One PR = one change. If this PR touches more than one of the following,
  split it. See CONTRIBUTING.md.
-->

- [ ] This PR touches **one cell** (or one ADR, or one fix).
- [ ] I have **not** bundled unrelated changes.
- [ ] If this PR touches an open ADR (003, 005, 006, 007), I have checked that the ADR is accepted, or this PR **is** the ADR ratification.

---

## Doctrine compliance

<!--
  The non-negotiable rules from doctrine/00-foundations.md.
  Tick every box. If a box cannot be ticked, explain in "Doctrine exceptions".
-->

- [ ] **No CA/PA.** No `current_ability`, `potential_ability`, or equivalent scalar introduced.
- [ ] **No truth leakage.** No module outside `pcfm-hidden` and `pcfm-match` (or `pcfm-gen`) reads Plane A.
- [ ] **No ratings from latents.** Any rating formula introduced takes `&[MatchEvent]`, not latent fields.
- [ ] **No Plane C → Plane A feedback.**
- [ ] **No hardcoded rules, names, colours, or balance values.** Everything lives in files.
- [ ] **No invented real data.**
- [ ] **No commercial data, no scraping.**
- [ ] **No unlicensed assets.**
- [ ] **Determinism preserved.** Named RNG streams, no `HashMap` in simulation paths, no `f32` in the critical path.
- [ ] **No entities deleted from the registry.** Deprecated, never deleted.
- [ ] **No save-breaking change without a migration.**
- [ ] **No circular dependency between modules.**
- [ ] **Statistics are append-only.**
- [ ] **Confidence intervals are not hidden.**

### Doctrine exceptions

<!--
  If any box above is not ticked, explain why here. A doctrine exception
  requires a doctrine ADR. Do not merge without one.
-->



---

## Level compliance

<!--
  The specification is concentric (L0 → L1 → L2 → L3).
  A level N cell may read the OUTPUT of a level < N cell, never its internals.
  A level N cell may NOT depend on a level > N cell.
  See doctrine/STRUCTURE.md §2.
-->

- [ ] If this PR adds a cell, the cell is at the **correct level** (L0, L1, L2, or L3).
- [ ] If this PR adds a cell, its `depends_on` only lists cells at the **same level or below**.
- [ ] If this PR adds a cell, its `referenced_by` is updated in downstream cells.
- [ ] If this PR adds a cell, `doctrine/_index.md` is updated.

If any box cannot be ticked, explain:



---

## Frontmatter (for new or revised cells)

<!--
  Every cell has a frontmatter. See doctrine/STRUCTURE.md §4.
  Fill in the values for the cell this PR adds or revises.
-->

```yaml
---
title: "L{level}.{NN} — {title}"
level: {0|1|2|3}
cell: "L{level}-{name}"
module_id: "L{level}.{NN}"
version: "{x.y}"
status: "{draft|review|ratified}"
last_updated: "YYYY-MM-DD"
supersedes: {null | "previous-id"}
depends_on: [...]
referenced_by: [...]
conforms_to: "00-foundations (v1.1)"
---
```

---

## Sources

<!--
  Every factual claim needs a source. If you can't find one, say "unverified".
  Do not invent. See CONTRIBUTING.md.
-->

- [ ] I have cited sources for every factual claim.
- [ ] If a claim is unverified, I have said so explicitly.

**Sources:**



---

## Review checklist (for the reviewer)

<!--
  The reviewer ticks these, not the author.
-->

- [ ] Frontmatter is present and correct.
- [ ] Doctrine is not contradicted (or a doctrine ADR is linked).
- [ ] Level dependency rules are not violated.
- [ ] Sources are cited for factual claims.
- [ ] The WHY is explained, not just the what.
- [ ] One change per PR.
- [ ] If the PR touches an open ADR, the ADR is accepted first (or this PR is the ratification).
- [ ] `doctrine/_index.md` is updated if a cell was added.
- [ ] `CHANGELOG.md` has an entry if the change is user-visible.
- [ ] No contradiction with existing cells.
- [ ] Language is English (or a translation in `locales/`).
- [ ] No `License: Proprietary` string anywhere.
- [ ] No `TBD` left in a field that should have a value.

---

## Related issues

<!--
  Link related issues with `Closes #123` or `Related to #123`.
-->



## Notes for the reviewer

<!--
  Anything the reviewer should know: a tricky decision, a known limitation,
  a question you want to discuss before merge.
-->



---

<!--
  Thank you. If this PR is small and clear, it will be merged quickly.
  If it is large and unclear, it will be sent back. That is by design.
  See CONTRIBUTING.md for the full process.
-->
