<!-- Thank you! Keep it short. See CONTRIBUTING.md for the paths and the rules. -->

## What and why

<!-- One change. What it does, and why: the football reason, if there is one. -->

Closes #<!-- issue number, if any -->

## What kind of change

- [ ] Balance (a file in `L0-match/balance/`)
- [ ] A law or the team shape (`rules.rs`, `shape.rs`)
- [ ] Engine (needs an accepted ADR for a model change)
- [ ] Tools, tests or the 2D view
- [ ] Documentation or a decision (ADR)
- [ ] Fix

## How I checked it

<!-- For balance or behaviour: seed(s) and minute(s) you watched in the 2D
     view, what you saw before and after. For code: the commands you ran. -->

- [ ] `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo test --workspace` pass
- [ ] If the match changes: `SIM_VERSION` bumped, golden regenerated, entry in `L0-match/CHANGELOG-SIM.md`
- [ ] A test that fails without this change (for a behaviour change)

## The rules (CONTRIBUTING.md §4)

- [ ] No dice: nothing on the pitch is decided by a random draw
- [ ] Every choice on the pitch is the player's, not the engine's
- [ ] No hardcoded numbers: parameters live in `L0-match/balance/`, with a source or "arbitrary"
- [ ] The hidden characteristics stay hidden (not shown, logged or saved outside the engine)
- [ ] Sources cited for factual claims; no licensed or scraped data

<!-- AI helped? Say so here: it is welcome, and you own the result. -->
