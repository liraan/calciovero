---
title: "Session handoff - 2026-11-06"
version: "1.0"
status: "analysis"
last_updated: "2026-11-06"
conforms_to: "SESSION-STATE.md, ADR-033, OI-72, OI-73, OI-74, OI-75"
---

# Session handoff - 2026-11-06

**Who this is for:** an agent that has never opened the repo. Every claim is verifiable with `git log`, `git show`, or a `grep`; nothing is from memory (D1).

## 1. State of the repo (verified)

- Path: `/c/Users/liraan/calciovero`; remote `github.com:liraan/calciovero.git`.
- HEAD = origin/main = `b0b9042`; working tree clean.
- 205 tests, 10 `pcfm-*` crates (moving to `ds-*`, ADR-032, not executed), 30 ADRs (28 accepted, 2 superseded), `SIM_VERSION` 23.
- Bookmark `SESSION-STATE.md` is ~148 lines (was 227 at the start of the session: the two handoff Notes moved verbatim to `analysis/2026-11-07-session-notes-log.md`, one-line pointer).
- 54 OIs; new today: OI-72 (gaze), OI-73 (ast-grep YAML rule), OI-74 (tool table), OI-75 (yq).
- No Rust code changed today. Last full C2 green is from this session; every commit after it is docs/rules/hooks and declares `Verified: C2-not-run`.

## 2. What was done (10 commits, all pushed)

1. `dc4f93e` - six pause items traced: ADR-027 note ("by Liraan" is implicit in the maintainer field), L0.11 section 3 note (the four families are fixed; add content, not structure), IDEA-DESIGN-005 (more data is not more capability), IDEA-DESIGN-006 (FM-gap map as a pointer index), REPO-DESCRIPTION (Ko-fi feed rejected, shields.io badge chosen), OI-72..75; bookmark compaction (227 -> 147 lines, Notes relocated verbatim).
2. `a5449ec` - rules: `perl -CSD -Mutf8 -i -pe` first for UTF-8, `git apply --check` second for patch-shaped edits; "Python is not installed" corrected (it is installed; the one-toolchain policy stays).
3. `f6f9dad` - LCV (E4b, pre-push).
4. `16b1e3d` - OI-74 executed: B1 no longer says "use sed/awk"; Tool choice table at the top of B1-bis; `analysis/2026-11-06-tool-evaluation.md`.
5. `feca5ee` - OI-75: `yq` installed and tested (frontmatter only, body intact, idempotent); not adopted yet (D3: adopt on the first commit that changes a report status).
6. `f7eb364` - extended tool evaluation with real data (groups 1/2/3/4/6) and the CWD finding.
7. `2e9d043` - B3: output sentinel in bright red (`\033[91m`) instead of bold, on maintainer request.
8. `9363150` - LCV.
9. `fc37e5d` - pre-commit shebang `#!/usr/bin/env bash` -> `#!/bin/sh` (Niubash has no env.exe); B1-bis bullet on Niubash paste.
10. `b0b9042` - LCV.

## 3. Tools: what to use and what not to use

Decided today, tested on real cases, not by argument.

| Case | Tool | Mandatory flag |
|---|---|---|
| Single line, may contain 4-byte UTF-8 | `perl -CSD -Mutf8 -i -pe` | `-Mutf8` (without it, emoji substitutions fail silently) |
| Multi-line or multi-file, already a diff | `git apply --check` then `git apply` | `--check` before; `--check -R` for positive proof |
| Whole-file rewrite | `cat > file << 'EOF'` | no code fence inside the heredoc |
| YAML frontmatter of the reports | `yq --front-matter=process -i` | - |
| `ds-*` refactor (ADR32) | `ruplacer --dry-run` / `--go` | - |
| Structural scan (code vs comments) | `ast-grep`, scan only | - |
| Locate | `grep -n`, `sed -n 'Np'` | - |

**Rejected with data:** `sd` (correct, no slot: `perl` is already there); `ex`/`vim` (correct, no advantage); `node`/`python3`/PowerShell (argument eaten by Git Bash, declared as a false negative); `ast-grep` fix (on 0.51.1 writes the literal `$$$`, unusable for rewriting; scan-only, tracked as OI-73); `gitoxide` (not tested, no demonstrated slot).

**Honest note on gain:** in machine time it is about 0 (52-695 ms is fork/exec cost, not operation cost). The gain is in errors avoided: `perl -Mutf8` against the silent failure of `sed`; `--dry-run`/`--check` against irreversible writes; `yq` against silently invalid YAML. The cost paid today is a longer rule (B1-bis) and new traps (section 6).

## 4. Environment: Niubash instead of Git Bash

Niubash is a native Bash-compatible shell (Rust), no MSYS. Installed and verified today on the project.

**Works:** `git` (native, `push --dry-run` green), `ssh` (Windows native OpenSSH, `Hi liraan!`), `grep`/`sed`/`awk` (winuxcmd), `perl` 5.42.3 native MSWin32, `cargo`, `ruplacer`, `sd`, `yq`, `ast-grep` (scan), `scripts/gates.sh` identical to Git Bash.

**Three things to know before touching anything:**

1. **The hooks wanted `#!/usr/bin/env bash`.** Niubash has no `env.exe` (the Git for Windows one is MSYS and needs `msys-2.0.dll`; copying it alone opens an error dialog). Fix: `pre-commit` is now `#!/bin/sh`, like `pre-push` and `commit-msg` which already worked. The pre-commit is POSIX-safe (verified by reading it): on Linux CI (`dash`) it will not break.
2. **Niubash loses 4-byte UTF-8 on paste** and mangles **non-ASCII inside multi-line shell commands** (observed twice today: emoji in paste, em-dash inside a `perl`). Rule: **ASCII in commands, UTF-8 in files**. For substitutions, codepoint form: `s/\x{1F535}/\x{1F7E2}/` always works, because it is ASCII in the command.
3. **`GIT_PAGER=cat`** in commands that may open the pager: otherwise the terminal stays in `less` and the output does not arrive.

**Git Bash: replaceable.** Kept installed as a fallback until the maintainer decides to remove it. The SSH key is the same in `~/.ssh/`; the `config` path must be Windows-style (`C:/Users/...`), not `/c/Users/...`.

## 5. What remains open, and how to start the next session

**The engine was not touched today.** Rust code is unchanged since `f7bb13a`.

**The Plan's next step is OI-72 - `AgentState.gaze` (ADR-033).** Read and discussed today, not implemented. The blocker is a **model choice**, not a technical one: ADR-033 says "an agent looks at the ball" but does not say *when* the gaze updates. Four forms:

- **(A)** on possession change - setup toward the centre, then toward the ball on every change. Makes perception meaningful with one rule. **Recommended.**
- **(B)** every tick with a rate limit - the rate would be a number with no derivation (F.7 forbids it).
- **(C)** setup only - after the first pass nobody looks at the ball; the goalkeeper does not see the shot.
- **(D)** every tick without a rate - tautological: `perceived_ball` always `Some`, F.6 step 2 becomes a no-op.

The implementation touches `spatial.rs` (`gaze: u16` in `AgentState`, initialised in `from_squads`), `perception.rs` (`visible_to` and `perceived_ball` read `gaze`), `state.rs` (remove the `am.heading = h` override before the decision; ADR-033: "heading... never set by a decision"), `save.rs` (`SIM_VERSION` 23 to 24), golden regenerated, `CHANGELOG-SIM`, full C2. The `Note-Read: L0-POSITIONAL-SLICE section 0, 6-ter, 7` requirement is already satisfied in this session.

**Other open, minor:** the "locate" group (group 5) was invalidated by a CWD crash and must be redone; `gitoxide` not tested; the B1-bis bullet on paste must be extended to "non-ASCII inside multi-line shell commands" (second finding today, not yet written).

## 6. Traps paid today, for whoever comes next

- `cd /tmp/x && ... && rm -rf /tmp/x` destroys the shell's CWD: every relative path fails and a broken tool is blamed unfairly. `cd -` before the `rm`, always.
- `perl -i -pe` with `.=` appends to **every** line that matches: three copies of a bullet without a `$done` flag.
- `grep -c` counts lines, not occurrences.
- A `grep` anchor written from memory does not match: verify the literal prefix that exists, first.
- A test that passes for the wrong reason (emoji lost in paste -> file without emoji -> "substitution" at zero) is worse than no test.

**How to start:** startup ritual (`NEW-SESSION-PROMPT.md`) - repo alignment, bookmark, IDEAS, reports, `gates.sh startup`, `gates.sh resume`, foundations, numbers, report to the maintainer. Then choose form A/B/C/D on `gaze`, and open. One focus only.
