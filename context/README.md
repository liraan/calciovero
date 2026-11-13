# context/ — the memory of the work

> **Contributors: you can ignore this directory.** It is the working memory of the maintainer and of the AIs that work with him. Start from [`CONTRIBUTING.md`](../CONTRIBUTING.md).

It is outside the rings: nothing in `doctrine/`, `L0-match/` or `tools/` reads it. The repository is the source of truth; what is here is checked against it (`git log`, `cargo test`) before it is trusted.

| File | What it is |
|---|---|
| [`STATE.md`](STATE.md) | the bookmark: last commit verified, where we are, the plan, the open items |
| [`RULES.md`](RULES.md) | how we work: the pact, the filters (luna, partita vera), honesty, git, the sessions |
| [`IDEAS.md`](IDEAS.md) | ideas still alive, not decisions; the rejected ones, so they are not discussed again |
| [`plans/`](plans/) | the plan of the current task (`L0-CLOSE.md`) |
| [`captures/`](captures/) | what was said at each pause or task boundary, written by `session.sh capture` |
| [`scripts/gates.sh`](scripts/gates.sh) | the mechanical checks (docs coherence, bookmark, platform-free engine); run by CI and the pre-push hook |
| [`scripts/session.sh`](scripts/session.sh) | `start`, `close`, `capture`: the rituals of a session, in one script |

**Read order for a new session:** `RULES.md`, `STATE.md`, then `bash context/scripts/session.sh start`.

Files here are rewritten or removed when their function is served; what was removed is in `git log` (until 2026-11-13 this directory was `process/`).
