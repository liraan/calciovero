# Context directory — agent workspace

This directory is an **agent workspace**, not human documentation. The documentation for collaborators is `README.md` at the repo root and `doctrine/STRUCTURE.md` (the normative source).

## Rule

The repository itself remains the source of truth. The files in `process/` are session memory, snapshots, policies, and review notes. Before treating any item here as live fact, validate it against the current repo state (`git status`, `git log`, `git ls-files`, `cargo test`).
Review notes under `analysis/` are evaluative material, not source-of-truth repository state: they are not the project specification and not a substitute for the live code and live git state.

## Functional directory (exception to the repo rules)

This directory contains **functional working documents** (session prompts, policy, bookmark, analysis reports), not historical records. Unlike the rest of the repository:

- Files here **can be removed, restructured, or rewritten** freely when their function has been served.
- **Analysis reports** in `analysis/` are working documents. A report is **closed** when it has been **examined** and its findings are **entered in `SESSION-STATE.md`** — as steps to do, or as explicitly rejected/deferred items. Resolution is not required: the report is a transport for findings, and its function is served once the findings are in the plan. Closed reports are moved to `analysis/archive/` at the closing ritual (history preserved in git, active directory clean).
- At the startup ritual, read **all reports in `analysis/`** (not `analysis/archive/`). No fixed count.

This exception applies **only to `process/`**. Everywhere else, the standard rules apply.

## Files in this directory

**Entry points (read in this order):**

1. `SESSION-STATE.md` — current session bookmark: last commit, next steps, files touched. Read this first when resuming after a pause.
2. `NEW-SESSION-PROMPT.md` — prompt for the chat agent. Contains the operating rules and the codebase-specific rules.
3. `AGENT-POLICY.md` — policy for any agent (chat or GitHub). What it may do, what requires confirmation, what it must never do, how approval works, how revocation works.

**Audit agent entry point:**

- `GITHUB-SESSION-PROMPT.md` — prompt for the audit agent (analysis reports, not code changes).

**Supporting material (not normative):**

- `L0-POSITIONAL-SLICE.md` — design note for the current L0 work: §0 the vision (phenomenological, eight categories), §6-ter the event map, §7 where we are. **Not normative** (the spec is), but it is the entry point before touching the engine: a commit that touches `state.rs`, `spatial.rs`, `perception.rs`, `orchestrator.rs`, `save.rs` or `golden_l0.rs` carries `Note-Read: L0-POSITIONAL-SLICE §0, §6-ter, §7`.
- `PROJECT-IDEA-LOG.md` — **archived** to `archive/PROJECT-IDEA-LOG.md` (historical record, frozen at 2026-10-27). Decisions now live in ADRs and `SESSION-STATE.md` Open items.
- `IDEAS.md` — open and rejected ideas. Do not implement from it.

**Subfolders:**

- `analysis/` — current reports and the newest handoff, dated files. Superseded ones are deleted, not archived (2026-11-12): recover them with `git log -- <path>` and `git show <commit>:<path>`.

## Two modes, one policy

- **Chat agent** (interactive): reads `NEW-SESSION-PROMPT.md`, follows the operating rules, edits files, commits, and pushes on approval.
- **Audit agent** (asynchronous): reads `GITHUB-SESSION-PROMPT.md`, produces one Markdown report in `analysis/`, does not modify code. Role and limits are in `AGENT-POLICY.md` §8.
- **Both** follow the same `AGENT-POLICY.md`. The two prompts cite it; they do not duplicate it.

## Workflow

1. The audit agent produces `analysis/YYYY-MM-DD-HH-MM-review.md` (findings table: ID, severity, file:line, issue, recommendation).
2. The chat agent triages: for each finding, propose **apply** (with commit + verification), **reject** (with reason), or **backlog** (add to `IDEAS.md`).
3. The maintainer approves or rejects. Nothing is applied because a report says so.
4. The maintainer may authorize the audit agent to apply a scoped modification as an exception (see `AGENT-POLICY.md` §8). For synchronous emergencies, use the chat agent or the maintainer directly.

## Read order for a new chat

1. `SESSION-STATE.md`
2. `NEW-SESSION-PROMPT.md`
3. `AGENT-POLICY.md`
4. Only then the normative docs (`doctrine/STRUCTURE.md`, `L0-match/design/ARCHITECTURE.md`, `L0-match/design/CODE_TOUR.md`, `L0-match/specs/...`).

## In short

`process/` is a continuity layer for agents. It helps with context retention, not with defining the project's authoritative state.
