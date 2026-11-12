# Archive

This folder contains **superseded documents**. Nothing here is maintained. Nothing here is deleted.

The archive exists for traceability: so that "why did we decide X?" can be answered years later, even after the decision has been replaced.

---

## What goes here

| File | What it is | Superseded by |
|---|---|---|
| `Prompt_Master_v5.md` | The v5 doctrine (pre-statistics) | `Prompt_Master_v6.md` |
| `Prompt_Master_v6.md` | The v6 doctrine (statistics added) | `Prompt_Master_v7.md` |
| `Prompt_Master_v7.md` | The v7 doctrine (CA/PA removed, Three Planes explicit) | `specs/00-foundations.md` (v1.0) |
| `00-foundations-vX.Y.md` | Previous versions of the doctrine | current `specs/00-foundations.md` |
| `01-architecture-vX.Y.md` | Previous versions | current module |

*(The rows above are the planned contents. Some of them do not exist yet. See "Current contents" below.)*

---

## Current contents

As of 2026-10-25, the archive is **empty**. The three Prompt Master versions and the four legacy modules were never committed and are not recoverable.

Once recovered, they will be placed here. Until then, the files are recoverable from git history:

```bash
# Recover a legacy module
git log --all --full-history -- modules/01-architecture.md
git show <commit>:modules/01-architecture.md > archive/legacy/01-architecture.md
```

---

## Rules

1. **Read-only.** Do not edit archived files. They are snapshots, not living documents.
2. **Never deleted.** An archived file stays in the archive forever. If it becomes irrelevant, it stays anyway: history is not a curation exercise.
3. **Naming.** `original-name-vX.Y.md`, matching the version in the frontmatter (if the archived file has a frontmatter). For files without a frontmatter, use the version in the filename, or the commit date.
4. **One subfolder per concept.** `legacy/` for the four pre-concentric modules. `prompts/` for the Prompt Master versions. `doctrine/` for previous `00-foundations` versions. Do not mix.
5. **Update this README when you add a file.** The table at the top must reflect the actual contents. A stale table is worse than no table.

---

## How to archive a document

When a document is superseded by a newer version:

1. **Copy** (do not move) the old version to `archive/<subfolder>/<name>-vX.Y.md`.
2. **Move** the old version out of its original location (delete it from `specs/` or `docs/`).
3. **Update this README** with the new row in the table at the top.
4. **Add an entry to `CHANGELOG.md`** under "Removed" or "Changed", referencing the archive file.
5. **In the new document**, add `supersedes: "<archived-id>"` in its frontmatter (if it has one).

For the doctrine specifically (`specs/00-foundations.md`), the archive subfolder is `archive/doctrine/`. For the prompt masters, it is `archive/prompts/`. For the legacy flat modules, it is `archive/legacy/`.

---

## Planned subfolders

```
archive/
├── README.md                      ← this file
├── prompts/                       ← *(planned)* Prompt Master v5, v6, v7
├── doctrine/                      ← *(planned)* previous 00-foundations versions
└── legacy/                        ← *(planned)* the four Italian flat modules
```

Each subfolder gets its own `README.md` when it is created, listing its contents and the reason each file was archived.

---

## Why archive instead of delete

Deleting is irreversible and loses context. Archiving:

- **Preserves the "why"** — a decision made three years ago can be explained by reading what was known at the time.
- **Supports audits** — if a user reports a bug that was "fixed" years ago and then reintroduced, the archive shows the original fix.
- **Avoids rewriting history** — git history is linear and lossy; the archive is explicit and readable.

The archive is not a graveyard. It is the project's memory.

---

<!--
  NOTES FOR FUTURE EDITORS:

  - Keep the "Current contents" section accurate. If the archive is empty, say
    it is empty. If it contains files, list them. A wrong table is a broken
    promise to the reader.
  - Do not add files to the archive without updating the table at the top.
  - Do not delete files from the archive. If a file is truly irrelevant, it
    still stays; the point is traceability, not cleanliness.
  - When you create a subfolder, create its README.md at the same time.
  - This file is linked from:
      - specs/00-foundations.md §8 (as the historical reference, once it
        exists)
    If you move or rename this file, update those links.
-->
