# ADR-024 — Remove `CauseTrace` at L0

- **Status:** accepted
- **Date:** 2026-10-31
- **Supersedes:** ADR-023
- **Superseded by:** none
- **Blocks:** none (the change is applied in the same session)

## Context

ADR-023 (accepted 2026-10-31, morning) chose Option 2: keep `CauseTrace`, have the engine build it from observed `Outcome` fields. It declared "at L0 the observed cause is poor" as an accepted cost.

Later the same day, removing all latent-derived factors (`finishing`, `passing`, `injury_proneness`, `goalkeeping`, `set_pieces`, `attacker_score`, `defender_score`) and `xg_value` left the outcomes with **only boolean results**: `goal`, `on_target`, `complete`, `success`, `saved`, `injured`, `winner`. None of these is a *cause*: they are the *outcome*.

Result: `CauseTrace { contributing_factors: Vec<Factor> }` could only be **empty**. ADR-023 said "poor"; the reality is "empty". A serialised, hash-contributing, always-empty field is ceremony, not transparency.

## Decision

**Option 1b: remove `CauseTrace` and `Factor` from L0.**

- `pcfm-model::cause` is deleted.
- `MatchEvent` loses `cause_trace`.
- The `resolve_*` outcomes lose `cause`.
- `MatchEvent.event_type` + `MatchEvent.fields` are the cause: the event type is what happened, the fields are the observable result.
- At L1+, when observable features exist (distance, pressure, body part), a cause trace is reintroduced **with content**. It is tracked as IDEA-INFRA-009, not implemented now (D3).

## Consequences

### Easier
- No empty field in the save, in `chain_hash`, in the event model.
- Fewer types: `CauseTrace`, `Factor`, `EmptyCauseTrace` gone.
- The Plane A leak surface (R-01) is removed at the root: no cause object can carry a latent.

### Harder
- The specification `05-match-events.md` §2.3 ("every event must have a cause trace") was written for a world with causes. It is amended: at L0 the cause is the event itself.
- At L1 a new ADR will reintroduce `CauseTrace`. That is a future decision, not a removed capability.

### Locked in
- No latent value in an event. `event_type` + `fields` are observed only.
- If a continuous observed value is needed in a cause in future, it goes through Plane C with declared uncertainty.

## Implementation notes

1. `crates/pcfm-model/src/cause.rs` deleted; `pub mod cause` and `pub use cause::{...}` removed from `crates/pcfm-model/src/lib.rs`.
2. `MatchEvent.cause_trace` removed from `pcfm-stats::event`; `validate()` checks only `timestamp_minute > 120`; `StatsError::EmptyCauseTrace` removed.
3. The seven `resolve_*` outcomes lose `cause`, `xg_value`, `injury_risk_modifier`, `temperament_modifier`, `risk_modifier`.
4. `state.rs` builds no cause; `emit_event` has no `cause` parameter.
5. Anti-leak test `crates/pcfm-match/tests/anti_leak.rs`: it failed pre-fix (`finishing` in `Factor.name`), it passes post-fix. Failing output committed as evidence.
6. `SIM_VERSION` bumped (1 → 2): event payload changed.
7. `L0-match/specs/05-match-events.md` §2.3 amended; `L0-match/specs/10-ground-truth.md` §2.2 amended.
8. `IDEAS.md`: IDEA-INFRA-009 "CauseTrace at L1" (sketch, prerequisite: observable features in the engine).

## References

- ADR-023 (superseded by this ADR)
- `doctrine/00-foundations.md` §3 (Plane A), §5 (no truth leakage)
- `L0-match/specs/05-match-events.md` §2.3 (amended)
- `L0-match/specs/10-ground-truth.md` §2.2 (amended)
- `process/analysis/2026-10-30-13-39-review.md` (R-01)
