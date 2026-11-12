---
title: "Tool evaluation — 2026-11-06"
version: "1.0"
status: "analysis"
last_updated: "2026-11-06"
conforms_to: "B1, B1-bis, OI-74"
---

# Tool evaluation — 2026-11-06

**Why this file exists:** OI-74 declared that B1 and B1-bis contradicted each other ("use `sed`/`awk`" vs "`perl` first, `git apply` second") and that the evaluation lived only in chat. This is the record. It does not add a rule: B1 and B1-bis are the rules, this is the reasoning behind them.

## Adopted

- **`perl -CSD -Mutf8 -i -pe`** — first choice for single-line edits that may contain 4-byte UTF-8. Tested on a sandbox file (P1: a case that can fail). Results: `perl -CSD -i -pe 's/🔵/🟢/'` **silently does nothing** (the script is read as bytes, the same trap as `sed`); `perl -CSD -Mutf8 -i -pe 's/🔵/🟢/'` works; the codepoint form `\x{1F535}` works too. **`-Mutf8` is mandatory.** Adopted in B1-bis (commit `a5449ec`).
- **`git apply --check` + `--stat`** — second choice for multi-line or multi-file edits already expressed as a diff. Three proofs (commit `a5449ec`): `--check` rejects a double application (safety); `--stat` shows the preview before the mutation (D5 artefact); `--check -R` confirms internal coherence (positive-application proof). Adopted in B1-bis.

## Rejected (evaluated, not adopted)

- **`sd`** (Rust) — same slot as `perl`, no advantage, adds a dependency. Rejected: no slot.
- **`ex`** (Vim batch mode) — an editor, not a stream editor; same capability as `perl` for our use, no added value. Rejected: no added capability.
- **`ruplacer`** (Rust) — recursive multi-file replace; would fit the `ds-*` refactor, but `ast-grep` is more precise on Rust. Rejected: superseded before adoption.
- **`node -e`**, **`python3`**, **PowerShell `WriteAllText`** — present but no advantage over `perl`; the policy keeps one toolchain (Bash/Git Bash + `perl`). The `python3` presence was verified on 2026-11-06 (the previous "Python is not installed" line was false; corrected in `a5449ec`).

## Candidates (installed or tracked, not adopted today)

- **`ast-grep`** (installed 2026-11-06, 0.51.1) — exact patterns work (`use pcfm_core::ids::PlayerId;` matches). Metavariables (`$A`, `$$$`) do **not** match a `use_tree` with multi-segment paths: the `ds-*` refactor needs a YAML rule with `kind: use_declaration`, not a `--pattern`. Tracked as **OI-73**.
- **`yq`** — for the YAML frontmatter of the reports in `analysis/`. No case today (no report changes status in this session). Tracked as **OI-75**.

## Finding: line endings cross-platform

The repo is **already correct**, verified on 2026-11-06. `.gitattributes` exists, is tracked, contains `* text=auto eol=lf`, per-extension `eol=lf` for source files, `eol=crlf` only for `.bat/.cmd/.ps1`, `binary` declarations for images and archives, and `Cargo.lock merge=binary`. `git check-attr` confirms the rules are active (`*.rs` → `text: set, eol: lf`; `*.png` → `binary: set`). Zero **textual** files with CRLF in the blob: the three "CRLF" hits were binary assets already declared binary (a false positive of the first check, not of the repo). `core.autocrlf=true` on Windows is the recommended setting and is overridden by `.gitattributes`. **No change needed, no OI.**

## Traps paid

- **A check that does not exclude binaries reports false CRLF.** `git cat-file -p | grep $'\r'` finds `0x0D` inside PNGs and PDFs. The filter must skip files declared `binary` before counting. Cost: one wrong count, corrected in the next block.
- **`grep -c "\`pattern\`"` eats the backticks.** Unquoted backticks in a double-quoted shell string are command substitution: the pattern becomes empty and the count is meaningless. The verification must use single quotes or an anchor without backticks. Cost: one failed verification, corrected in the next block.
- **A `grep` anchor written from memory (em-dash, bold markdown) does not match.** `grep -nE "^\*\*B1 —"` returned nothing because the line begins with `- **B1`. The anchor must be the literal prefix that exists, verified with `grep -n "B1 — Tools"` first. This is P3 applied to `grep`, not only to patterns.

## Extended evaluation (2026-11-06, evening)

The original evaluation rejected `sd`, `ex`, `ruplacer`, `node`, `python`, PowerShell by argument, not by test. That was a leak (D5): "no slot" is a judgment, "tested and failed" is a fact. The candidates were installed (`sd` 1.0.0, `ruplacer` 0.10.0, `ast-grep` 0.51.1 via cargo, `yq` 4.54.2) and tested on six groups. Timing is process startup on Git Bash/Windows, not operation cost.

### Group 1 — single-line, 4-byte UTF-8 (blue circle to green circle)

| Tool | Result | Time |
|---|---|---|
| `perl -CSD -Mutf8 -i -pe` | correct | 84 ms |
| `perl -CSD` (no `-Mutf8`) | silent failure | 78 ms |
| `sed` | silent failure | 52 ms |
| `sd` | correct | 97 ms |
| `node -e` | fails (argument eaten by shell, see note) | 133 ms |
| `python3 -c` | fails (same cause) | 247 ms |
| PowerShell `WriteAllText` | fails (same cause) | 695 ms |

Note (D1): `node`/`python3`/PowerShell failures are argument-passing, not language: the emoji inside `-c`/`-e` is consumed by Git Bash before reaching the process. Not tested with a script file. Declared as a false negative.

Verdict: `perl -CSD -Mutf8` and `sd` correct; `perl` already installed, `sd` no slot.

### Group 2 — multi-line / multi-file patch

`perl` with `^(\s*)use ` and `git apply` both correct (3 `use` changed, one `// use` comment untouched). `git apply` wins on atomicity and preview (`--check`, `--stat`); `perl` wins on setup speed. Both retained.

### Group 3 — whole-file rewrite

`cat > file`, `ex -s`, `vim -es`: all correct (UTF-8 preserved, no BOM, LF). `cat > file` wins on simplicity; `ex`/`vim` no added value.

### Group 4 — YAML frontmatter

`yq --front-matter=process`, `sed`, `perl`: all correct (body intact, UTF-8 preserved, idempotent for `yq`). `ex` failed because `-c 's/.../.../'` without `%` acts on the current line (my error, not the tool). `yq` wins on structure-awareness.

### Group 6 — large refactor (`use pcfm_` to `use ds_`)

| Tool | Result |
|---|---|
| `perl '^(\s*)use pcfm_/$1use ds_'` | 11/11 correct, indentation preserved, doc comments untouched |
| `ast-grep` scan (YAML rule) | 123 matches (correct, more precise than grep: excludes `/// use`) |
| `ast-grep` fix (`$$$` in `fix:`) | broken: writes the literal `use ds_$$$` |
| `ruplacer "use pcfm_" "use ds_" --go` | 13/13 correct, dry-run native, 5 files, doc comments untouched |

Verdict: `ruplacer` wins for `ds-*` (ADR32): dry-run + apply, no regex, Rust, respects `.gitignore`. `ast-grep` remains a scan tool (precise on code, not on comments) but is not usable for rewriting on 0.51.1.

### Finding: shell CWD destroyed by `cd /tmp/x && ... && rm -rf /tmp/x`

While testing Group 2, the shell was `cd`'d into `/tmp/gt` and then the directory was deleted with `rm -rf`. Every subsequent command ran from a deleted CWD: `crates/` not found, `git` "Unable to read current working directory", `ast-grep` native binary ENOENT (false alarm, the binary existed). Five commands were wasted and one group (locate) was invalidated before the cause was found.

Rule: `cd /tmp/x` must be followed by `cd -` (or an absolute path) before `rm -rf /tmp/x`. Symptom: every relative path fails, `git` reports the CWD error. Remedy: `cd "$HOME"` (or an absolute path) and retry; it is not a tool problem.
