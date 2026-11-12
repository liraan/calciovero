# Decision patterns — PCFM

**Purpose:** observed defaults for recurring judgments. Not rules. A rule says "do X". A DP says "when the case is exactly X, the default is Y; the maintainer may override". A DP is read at the startup ritual (`grep -nE '^## DP-'`) and cited in the closing commit (`DPs-Applied:` trailer).

**Application rule (read first):** a DP applies **only to an identical case** — same category, same options, same source. A variant (different category, extra constraint, or unfamiliar context) requires asking. Do not stretch a DP.

**Governance:**
- Every DP cites a real session as `Source:` (date + commit hash). No source → no DP.
- Max 10 active. A DP not mentioned in **two consecutive closing commits** (`git log --format='%(trailers:key=DPs-Applied,valueonly)'`) is proposed for archival.
- A DP is a default, not a rule. The maintainer can contradict it. A contradiction is recorded in the session's `SESSION-STATE.md` Open items, not in this file.

---

## DP-01 — Defer over decide, when uncertain

- **Source:** 2026-10-31, session with OI-14 (golden test 10 seasons).
- **Rule:** when a decision requires data not yet in hand, do not pick an option. Write an OI with the exact verification needed.
- **Anti-pattern:** ask "a / b / c?" when the honest answer is "verify first".

## DP-02 — Session-scoped authorization

- **Source:** 2026-10-31, session with OI-16 (ADR / foundations edit permission).
- **Rule:** authorization to touch protected paths (`doctrine/adr/`, `00-foundations`) is session-scoped by default. Never assume permanent.
- **Anti-pattern:** propose a permanent rule for a temporary need.

## DP-03 — Split by nature

- **Source:** 2026-10-31, session with step 6 → 6a / 6b.
- **Rule:** when a step mixes mechanical fixes and content decisions, split. Mechanical first; content after its dependencies are settled.
- **Anti-pattern:** bundle a decision with a typo fix in one commit.

