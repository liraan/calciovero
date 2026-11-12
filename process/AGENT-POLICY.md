# Agent policy — PCFM

**Purpose:** define what an agent (Warp, DeepSeek-TUI, or a future custom bot) may and may not do on this repository, and how its actions are approved. This is a **policy**, not a specification: it defines the guardrails, not the implementation.

**Last updated:** 2026-10-29 (commit `08eb158`)

## 1. Principles

- The agent **proposes**, the human **approves**. No action on the remote repo happens without explicit confirmation (exception: the audit-report push in §8).
- **Phase 2 (ruleset active):** every change goes through a **branch and a pull request**. No direct push to `main` by anyone. **Phase 1 (ruleset disabled, single maintainer):** direct push to `main` is allowed only to the maintainer (audit-report exception: see §8).
- The agent **does not merge** pull requests. Only the human merges.
- The agent **does not force-push** `main`, **does not delete** `main`, **does not create tags** without explicit confirmation.
- The agent works on `agent/<task>-<date>` branches. These branches are intentionally **not protected**: they are the agent's workspace. `main` is protected by a GitHub ruleset **when active** (server-side).

## 2. What the agent may do autonomously

- Read any file in the repository.
- Run `cargo test`, `cargo clippy`, `cargo fmt --check`.
- Run `git status`, `git diff`, `git log`, `git branch`.
- Propose a diff or a patch in the chat.
- Create a local branch (`agent/<task>-<date>`).
- Commit on that branch (the local pre-commit hook enforces fmt, clippy, test on Rust changes).
- Push that branch to `origin` (the branch is not protected; `main` is).

## 3. What requires explicit confirmation

- Opening a pull request.
- Modifying any file under `doctrine/`, `L0-match/specs/`, `process/`.
- Modifying `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`.
- Modifying `LICENSE*`, `.github/`, `.githooks/`.
- Any operation not listed in §2.

## 4. What the agent must never do

- `git push --force` on `main` (blocked by the ruleset, not just by policy).
- `git push` directly to `main` (blocked by the ruleset).
- Merge a pull request.
- `git tag`, `git reset --hard`, `git clean -fd` on the remote.
- Modify files outside the repository.
- Disable or bypass the ruleset on `main`.

## 5. How approval works

- The agent shows the exact command it wants to run, and the diff it will produce.
- The human reviews and confirms (or rejects).
- Only then the agent executes.
- If the agent is uncertain, it asks. It never guesses.

## 6. Defences in place

| Defence | Where | What it blocks |
|---|---|---|
| **GitHub ruleset on `main`** | Server-side (GitHub) | Direct push, force-push, deletion, merge without PR + 1 approval |
| **Pre-commit hook** (`.githooks/pre-commit`) | Local | Commit with Rust code that fails fmt, clippy, or test |
| **Branch naming** (`agent/<task>-<date>`) | Convention | Not a security control: traceability |
| **PR + merge by human** | Process | Unreviewed changes reaching `main` |

The ruleset is the **primary** defence. The hook is a local safety net. The branch naming is traceability, not security.

**Current status (2026-11-02):** the ruleset is **disabled**. Only the
maintainer pushes. It is reactivated when one of these holds: (a) a second
reviewer exists — Phase 2 is "branch + PR + independent review", and a PR
without a reviewer is a commit with two steps; (b) the project ships, and
the ruleset protects a released artefact; (c) a write token is granted to
an agent. Until then, Phase 1 rules apply, and the pre-commit hook remains
active. `Evaluate` (log without blocking) is not available: it requires an
organisation or Enterprise, and this is a personal repository.

### Two remotes (2026-11-08)

The project has **two remotes**, and both are first-class:

| Remote | URL | Role |
|---|---|---|
| `origin` | `github.com:liraan/calciovero` | **Primary.** CI (Actions) runs here on every push; the pre-push gates run locally, before the remote. |
| `codeberg` | `codeberg.org:liraan/calciovero` | **Backup.** Codeberg is not a second home; it is the copy that survives if the primary does not. No hosted CI on this side. |

**Rule.** Every push goes to **both**. The command is `git push && git push codeberg main`, one block, so the backup cannot drift. The upstream is `origin` (`git status -sb` shows `## main...origin/main`); Codeberg is a tracking *branch*, not the upstream, and must not replace it.

**Verifiable artefact.** `git remote -v` lists both; `git status -sb` after a push shows `## main...origin/main`, without `[ahead]` or `[behind]`; `git log --oneline -1 codeberg/main` matches `origin/main` for the same HEAD. If one of the two diverges, the push was incomplete: fix it before continuing.

**Migration (future).** The migration to Codeberg as primary is `git remote set-url origin <codeberg-url>` + `git remote rename github backup-github` + `git push -u origin main`, and is decided by the maintainer. It is not on the plan; ADR-032 (crate rename) is a candidate moment because it already changes the repository layout, but the two decisions are independent.
