# GitHub Audit Session Prompt — PCFM

**Purpose:** prompt to paste as the first message of a new chat with a **read-only audit agent** (GitHub Copilot, Hermes Agent, or any AI agent with analysis capabilities). The agent produces one comprehensive Markdown report and does not modify code.

**Last updated:** 2026-10-30 (commit `55a932a`)
**Last updated:** 2026-10-31 (added: dimension 7 "Gate integrity" — audit `scripts/gates.sh` as code: checks that lie, format-dependent checks, declared vs undeclared holes. Not run as part of C2: A2 duplicates cargo test. Earlier: pure/gap classification with `maintainer_override` column)
---

## How to use

1. Open a new chat with the audit agent (GitHub Copilot, Hermes, or any agent with analysis capabilities).
2. Copy the content of the block below (from `# Audit Agent Session Prompt` to the last line, without the 4 backticks).
3. Paste it as the first message.
4. If the agent has a write token, it commits the report to `docs/context/analysis/YYYY-MM-DD-HH-MM-review.md`. Otherwise, it outputs the report in chat: save it and commit it yourself.
5. **Copilot Coding Agent (cloud):** assign an issue rather than pasting a chat prompt — the agent may open an empty branch and deliver the report in chat instead of committing it.

---

## Prompt

````
# Audit Agent Session Prompt — PCFM Repository Audit

**Purpose:** this prompt starts a session with a **read-only audit agent** (GitHub Copilot, Hermes Agent, or any AI agent with analysis capabilities). The agent produces **one comprehensive** Markdown report. It does not modify code, configuration, or any other file.

**This prompt is agent-agnostic.** It works for human-driven agents (Copilot) and autonomous agents (Hermes). The difference is only in the approval channel: see "Rules (non-negotiable)" below.

## What to read first (in this order)

1. `docs/context/SESSION-STATE.md` — current bookmark, next steps, open items
2. `docs/context/AGENT-POLICY.md` §1 and §8 — your role and limits
3. `docs/context/NEW-SESSION-PROMPT.md` — operating rules, **C2 and C2-bis**
4. `STRUCTURE.md` §5 — single source of truth for ADR status
5. `docs/ARCHITECTURE.md` — how the crates interact
6. `specs/00-foundations.md` — the doctrine (Three Planes, determinism, bans)
7. **All** reports in `docs/context/analysis/*.md` (not `analysis/archive/`) — for continuity

## What to test — mandatory baseline (C2)

Run these commands and report the **actual output**, not the numbers claimed in docs:

    cargo build --release
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace
    cargo fmt --check
    cargo run --release --bin pcfm-test-runner -- --check --seed 42

If any is red, report it as a finding. Do not fix it: your job is to report, not to modify.

## What to test — beyond baseline (C2-bis, encouraged)

The goal is a clean, optimized, secure repository, not merely passing the baseline. If your toolchain or model capabilities allow additional checks — e.g. `cargo-audit`, `cargo-deny`, `miri`, coverage (`cargo-tarpaulin`), mutation testing, fuzzing, manual review of hot paths — run them and report the result. **Do not limit yourself to the baseline.**

**Caveat (D2 applies):** "optimized" means *verified* optimization: reduce allocations, profile, remove redundancy, **without changing behaviour**. Do not propose changes to the simulation model without an ADR and a `SIM_VERSION` bump.

## Test plan (required)

Before running any test beyond the C2 baseline, write a short plan: which additional tests you consider necessary **for this repository**, and why. Include the plan in the report. At the end, report for each planned test: **ran** (with result) or **skipped** (with reason). A test never mentioned in the plan is a gap the maintainer cannot see.

## What to check — six dimensions

Every dimension must be addressed explicitly. A dimension with no findings must state what was checked and why it is clean — "no issues found" alone is not acceptable.

### 1. Build and test (real tool output, not claims)

Report the actual numbers (tests passed, scenarios, coverage), not the numbers claimed in docs. Include the baseline (C2) and any C2-bis checks you ran.

### 2. Security (static scan)

- Hardcoded secrets, API keys, tokens
- `unsafe` blocks (should be zero outside `pcfm-bridge`, which does not exist yet)
- Network access (`std::net`)
- Shell/command execution (`std::process::Command`)
- File I/O with unsanitized paths (path traversal)
- Unsafe deserialization patterns
- Dependencies: `cargo audit` / `cargo deny` if available; if not, say so and list transitive dependencies by hand

### 3. Coherence: spec ↔ code

For every claim in `specs/L0-match/*`, `STRUCTURE.md`, `docs/ARCHITECTURE.md`, `docs/CODE_TOUR.md`: is it true in the code? Cite file:line for both the claim and the code that contradicts or confirms it.

**3b. Text-level ambiguities:** contradictions between documents, terms used with two meanings, instructions that can be read two ways, cross-references that disagree. Cite both sides (file:line + file:line).

### 4. Doctrine: Plane A / B / C

- Does any crate outside `pcfm-match` / `pcfm-gen` read `pcfm-hidden` directly?
- Does `pcfm-hidden` expose any getter on latent values (other than `resolve_*` and `drain_observations`)?
- Does any `Debug`, `Serialize`, or `Display` on a Plane A type leak latent values?
- Does any stats code read latents instead of events?
- Are there hardcoded rules, thresholds, or balance values that should live in `balance/*.toml`?

### 5. Determinism

- Is there any `HashMap`/`HashSet` in a simulation path?
- Is there any `unwrap`/`expect`/`panic!` in a simulation path (other than documented compile-time constants)?
- Are RNG streams named and derived from `world_seed`?
- Are `SIM_VERSION` bumps documented in `CHANGELOG-SIM.md` before the change?
- Does the current `SIM_VERSION` in code match the last entry in `CHANGELOG-SIM.md`?

### 6. Correctness observations

Design notes that are not bugs but that the maintainer should know about. Example: the `pick_players` correlation in `pcfm-match/src/state.rs:222-226` (single RNG roll for attacker and defender). Each observation with file:line and one paragraph of context.

### 7. Gate integrity

`scripts/gates.sh` is the project's mechanical gate. It is read, not run as part of C2 (its A2 check duplicates `cargo test`). Audit it as code:

- Does each check verify what its label says, or does it report `ok` without a real assertion?
- Does any check depend on a format that can change — the bookmark's tables, the README badge, the ADR status line at `docs/adr/*.md:3`? If a format changes, the check must fail loudly, not silently.
- Are there known holes? `Files touched` is not enforced mechanically (E4 declares it). `B2`, `B4`, `B5` are `manual` on purpose. A hole that is declared is acceptable; a hole that is not, is a finding.
- A gate that lies is worse than no gate: it makes an agent believe it is compliant when it is not.

Each finding with `file:line` in `scripts/gates.sh` or in the ritual step that invokes it.

## Output format

Write to: `docs/context/analysis/YYYY-MM-DD-HH-MM-review.md` (ISO date, UTC).

Structure: a header with `title`, `version`, `status`, `last_updated`, `auditor`, `conforms_to`; then sections 1-8 as in the existing reports (the earlier reviews, e.g. `docs/context/analysis/archive/2026-10-29-18-18-review.md`, were removed on 2026-11-12 and are in git history). Number every finding `R-01`, `R-02`, … so the maintainer can verify each one with `grep`/`sed`.

**Classify every finding.** Each finding carries a class per the exit clause of `NEW-SESSION-PROMPT.md` D4: **pure** if it exposes a latent (D2), violates an invariant (`SIM_VERSION`, `chain_hash`, schema), touches a doctrine ADR, or has no executable first action; **gap** otherwise. Present the class in the finding's row: `R-NN | class | ... | maintainer_override`. A `maintainer_override` is written by the maintainer (e.g. `gap (reason)`) and **must not be re-classified** by a later audit: read it, do not overwrite it. Default when in doubt: pure.

## Rules (non-negotiable)

**Some rules are hard stops: violating them invalidates the report. They are marked 🛑.**

- 🛑 **Cite file:line** for every finding. If you have not read the file, say so — do not invent.
- 🛑 **Do not modify code, configuration, or any other file.** Your output is the report, nothing else.
- 🛑 **The report is the only file you may push, and only if you have a write token** (see `AGENT-POLICY.md` §8). One file, one commit, canonical name, commit message `docs(analysis): ...`. If you do not have a write token, output the report in chat: the maintainer will commit it.
- **Language:** the report is in **English** (it will be committed to the repo). Chat with the maintainer is in the maintainer's language.
- **One report per run.** If a second topic is needed, a second run will be requested.
- 🛑 **Number your findings** (`R-01`, `R-02`, …) so the maintainer can verify each one with `grep`/`sed`.

## Audit lifecycle

A report is **open** until all its findings are either fixed, explicitly deferred in `SESSION-STATE.md`, or rejected with a reason. At the closing ritual, the chat agent proposes to move closed reports to `docs/context/analysis/archive/`. See `docs/context/README.md` for the functional-directory policy.

## What this prompt is not

It is not a specification of the project (`STRUCTURE.md`, `specs/`, ADRs are). It is not a substitute for the chat agent's judgment (see `docs/context/NEW-SESSION-PROMPT.md`). The maintainer triages every finding with `grep`/`sed` before anything is applied. An audit that says "Pass" without verified findings is a draft, not a fact.
````
