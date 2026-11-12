---
name: Bug report
about: Something is wrong — a broken link, a contradictory statement, an outdated reference, a violation of the doctrine.
title: "[bug] "
labels: ["bug"]
assignees: []
---

# Bug report

Thank you for taking the time to report an issue. This project is documentation-first, so a "bug" can be:

- a **broken link** (a link to a file that does not exist),
- a **contradiction** between two documents,
- a **doctrine violation** (a document that contradicts `doctrine/00-foundations.md`),
- a **stale reference** (a file mentioned as existing but not yet created, without the `(planned)` marker),
- a **factual error** (a claim that is wrong, unsupported, or invented),
- a **formatting problem** (broken frontmatter, broken table, broken code block).

If what you found is a code bug, this is not the right template: there is no code yet. If what you found is a design problem, open a "module revision" issue instead (see the other template).

---

## Where is the problem?

<!--
  File path and, if possible, the line number or the section heading.
  Examples:
    - L0-match/specs/04-match-engine.md, §2.6
    - README.md, "Repository Structure"
    - doctrine/adr/003-plane-a-enforcement.md, "Decision"
-->



## What is wrong?

<!--
  A clear description of the problem. One or two sentences are enough.
-->



## What did you expect?

<!--
  What should be there instead? If you don't know the correct answer,
  say "I don't know, but this is wrong" — that's still useful.
-->



## Type of bug

<!-- Check all that apply. -->

- [ ] **Broken link** — a link points to a file that does not exist
- [ ] **Broken internal link** — a link between two files in the repo is wrong
- [ ] **Contradiction** — two documents disagree
- [ ] **Doctrine violation** — a document contradicts `doctrine/00-foundations.md`
- [ ] **Stale reference** — a file is cited as if it exists, but it doesn't (and isn't marked `(planned)`)
- [ ] **Factual error** — a claim is wrong or unsupported
- [ ] **Formatting** — frontmatter, table, code block, or heading is broken
- [ ] **Typo** — spelling, grammar, or punctuation
- [ ] **Other** — describe below

If "Other", describe:



## Can you fix it?

<!--
  If you can, open a PR instead of an issue. See CONTRIBUTING.md.
  If you can't, that's fine — this issue is enough.
-->

- [ ] I can open a PR to fix this.
- [ ] I can describe the fix, but I can't open a PR.
- [ ] I don't know how to fix it, I'm just reporting it.

---

## Additional context

<!--
  Anything else that helps: a screenshot, a quote from the document,
  a link to a related issue or PR, a reference to a standard or source.
-->



---

## Checklist

- [ ] I have read `doctrine/00-foundations.md` to check that my report is not itself a doctrine violation.
- [ ] I have searched existing issues to make sure this is not a duplicate.
- [ ] I have checked the `(planned)` markers — if the file is planned and not yet created, it is not a bug.
- [ ] I have not invented a fix; I have either described one or asked for help.

---

<!--
  This template is for documentation bugs. For design issues, use
  module-revision.md. For questions, use the "question" label directly.
-->
