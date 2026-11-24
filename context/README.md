# context/ — the memory of the work

> **Contributors: you can ignore this directory.** It is the working memory of the maintainer and of the AIs that work with him. Start from [`CONTRIBUTING.md`](../CONTRIBUTING.md).

It is outside the rings: nothing in `doctrine/`, `L0-match/` or `tools/` reads it. The repository is the source of truth; what is here is checked against it (`git log`, `cargo test`) before it is trusted.

| File | What it is |
|---|---|
| [`NEW-AI-PROMPT.md`](NEW-AI-PROMPT.md) | the prompt to paste into a new chat with an AI that has never opened the repository: what to read, how to answer, the direction, the technical procedure |
| [`HANDOFF.md`](HANDOFF.md) | for an operator (opencode or another AI) who carries on when Claude Code is away: who writes on `main`, the work queue in order, the proven procedures (a SIM bump, a season change), the traps, a log |
| [`STATE.md`](STATE.md) | the bookmark: last commit verified, where we are, the plan, the open items (the history archived in `captures/`) |
| [`RULES.md`](RULES.md) | how we work: the pact, the filters (luna, partita vera), honesty, git, the sessions |
| [`IDEAS.md`](IDEAS.md) | ideas still alive, not decisions; the rejected ones, so they are not discussed again |
| [`plans/`](plans/) | the whole game, level by level, and where everything is (`GAME.md`, start here); the bugs that keep L0's metrics out of band (`CODE-AUDIT.md`, "Terza passata"); the season being built (`L1.md`, `L1-COACH.md`); the ideal L0 compared with today's (`L0-IDEAL-COMPARISON.md`); the decisions on the whole-game plan (`DECISIONS-2026-11-15.md`) |
| [`captures/`](captures/) | what was said at each pause or task boundary, written by `session.sh capture` |
| [`scripts/gates.sh`](scripts/gates.sh) | the mechanical checks (docs coherence, bookmark, platform-free engine); run by CI and the pre-push hook |
| [`scripts/session.sh`](scripts/session.sh) | `start`, `close`, `capture`: the rituals of a session, in one script |

**Read order for a new session:** `RULES.md`, `STATE.md`, then `bash context/scripts/session.sh start`. For an AI new to the repository, paste `NEW-AI-PROMPT.md`.

Files here are rewritten or removed when their function is served; what was removed is in `git log` (until 2026-11-13 this directory was `process/`).
