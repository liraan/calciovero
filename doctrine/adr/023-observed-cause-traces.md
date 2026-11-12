# ADR-023 — Observed cause traces, not latent-derived causes

- **Status:** superseded by ADR-024
- **Date:** 2026-10-31
- **Supersedes:** none
- **Superseded by:** ADR-024 (Remove `CauseTrace` at L0, 2026-10-31)
- **Blocks:** L0 match engine (event emission), `pcfm-hidden` outcome types, `pcfm-stats` event validation, `pcfm-model::cause`, the R-01 fix and its anti-leak test

## Context

The 2026-10-30 audits (13-39 and 20-24) found a critical Plane A disclosure: `pcfm-hidden` writes raw latent values into `Factor.value`, which is `Serialize`, is carried by `MatchEvent.cause_trace`, enters `chain_hash`, and is written to save JSON. The chain is end-to-end and verified on file:line.

Verified during the 2026-10-30/31 session (not taken from the audit on trust):

- 7 leak sites, not 5: duel (`attacker_score`, `defender_score`), shot (`finishing`), pass (`passing`), injury (`injury_proneness`), save (`goalkeeping`), set piece (`set_pieces`).
- `primary_cause` has 3 real consumers (`state.rs:249` workaround, `event.rs:122` validation, `test-runner:240`), all artefacts of duplication with `EventType`.
- `explanation_template`: zero consumers, and it duplicates `EventType` (it is `"explanation.{event_type}"`).
- `injury_risk_modifier` / `temperament_modifier`: constants `1.0`, zero consumers. `risk_modifier`: exact linear function of `injury_proneness`, zero consumers.
- The specification requires a cause trace: `L0-match/specs/05-match-events.md:117` says "an event without a cause trace is a bug".
- The specification requires observed values in the cause trace: `05-match-events.md:117-124` and `10-ground-truth.md:96-111` say the cause trace references observed values, never latents.
- Plane A has no observations. The resolver knows only latents. It cannot produce observed causes.

The conflict: the spec asks for an observed cause trace; the resolver cannot produce one; the current code produces a latent one. The previously saved decision ("remove `CauseTrace` and `Factor` entirely", option 1b, commit `73f7ad1`) was based on incomplete premises and would violate `05-match-events.md:117`. It is superseded by this ADR.

## Options considered

### Option 1 — Remove `CauseTrace` and `Factor` entirely (1b)

- **Pro:** eliminates the leak at the root; no new type; ~2 sessions.
- **Con:** violates `05-match-events.md:117` (cause trace mandatory); loses the pillar #5 (transparency) vehicle at event level; at L1/L2 the type must be reinvented with another migration and another `SIM_VERSION` bump. Work today is thrown away.

### Option 2 — The engine builds the cause trace from observed outcomes

- **Pro:** satisfies the written specification; eliminates the leak at the root (Plane A stops producing causes); keeps pillar #5; durable — the type survives to L1/L2 and is enriched, not replaced.
- **Con:** ~4-5 sessions; at L0 the observed cause is poor (few factors); D3 risk if the structure outruns its content.

### Option 3 — Defer

- **Pro:** no decision without content.
- **Con:** the verified doctrinal breach stays; every session pays the cost of reconstructing the context; a failing anti-leak test is better than no test.

## Decision

**Option 2.** The engine builds the cause trace from the `Outcome` returned by `resolve_*`, not from `GroundTruth` and not from `ObservationEvent`. Plane A stops producing causes. The type `CauseTrace` stays and is enriched over time.

Why the `Outcome` and not `ObservationEvent`: the engine already receives the outcome from `resolve_*`; the outcome (after the removal of `cause`) contains only observable fields (`goal`, `on_target`, `winner`, `complete`, `saved`, `success`, `injured`). Using it requires no drain and no matching between observation and event. `ObservationEvent` is Plane B's input (statistics), not the cause's.

Ten clauses, decided with the maintainer on 2026-10-31:

1. The resolver loses `cause`; the engine builds the cause trace from the `Outcome`.
2. Neither `Outcome` nor `ObservationEvent` exposes a continuous value derived directly from a latent (e.g. `xg_value`, `goal_probability`) **to the cause**. `xg_value` is **removed from `ShotOutcome`** (amended 2026-10-31: it was a function of `finishing`, and L0 has no observable features to recompute it from; xG becomes a Plane C / `L3-statistics` concern, see `06-match-output.md` after the amendment). If a continuous observed value is needed in a cause in future, it is a Plane C estimate (with declared uncertainty), not an exact function of the latent.
3. The cause includes only factors whose metric is active; absent factors are omitted, not represented.
4. A measured zero (`Some(0.0)`) is included; a zero from absence (`None`) is omitted.
5. `Factor` has private fields and a single constructor `observed_factor(name, Option<f32>, weight) -> Option<Factor>`. `contributing_factors: Vec<Factor>` is unchanged and may be empty. No placeholder values.
6. At L1/L2, each factor declares its source (`source: ModuleId`). Not today (D3), but the single constructor is the extension point.
7. `primary_cause` is removed. `MatchEvent.event_type` is the primary cause.
8. `explanation_template` is removed. It duplicates `EventType` and cannot represent a parameterised template. The narrative (future) will read `MatchEvent` (event_type + fields + contributing_factors) and resolve a localised template from `fields`; it does not need a fixed string field on the event. See IDEA-INFRA-008 (explanation/narration as a plugin).
9. `CauseTrace` becomes `{ contributing_factors: Vec<Factor> }`. `EmptyCauseTrace` is removed; `MatchEvent::validate()` checks only `timestamp_minute`.
10. The order of `contributing_factors` is deterministic: factors are pushed in the declaration order of the outcome's observable fields, never from a `HashMap` or an unordered iteration. This keeps `chain_hash` stable across runs.

Additional decisions:

- `SIM_VERSION` is bumped (the event payload enters `chain_hash`). `SCHEMA_VERSION` is not bumped (serde ignores unknown fields).
- Old saves remain contaminated: the fix does not retroactively clean JSON written before the bump. Declared, not hidden.
- `xg_value` in `ShotOutcome` **is removed** (amended 2026-10-31). It was a function of `finishing` (latent) and L0 has no observable features to recompute it. Tracked originally as OI-9 "out of scope"; the maintainer decided to bring it **in scope** (choice C, 2026-10-31): `xg_value` removed from `ShotOutcome`, `xg` removed from `MatchStatistics`, `06-match-output.md` amended (xG becomes `L3-statistics` / Plane C).
- The false doc comments `outcome.rs:4` and `cause.rs:14` are corrected as part of the fix.

## Consequences

### Easier
- Plane A no longer produces a serialisable object; the leak surface at the resolver is removed.
- The specification is satisfied, not amended: `05-match-events.md:117` stays true.
- The type survives to L1/L2; the cause trace is enriched with observed factors without migration.
- `CauseTrace` is minimal: one field, no duplicated strings.

### Harder
- The engine must build causes for every event: more code in `state.rs`, more tests.
- The anti-leak test must cover the engine path, not only the resolver path.
- At L0 the cause trace is poor; accepted as the cost of durability.

### Locked in
- Observations must be the only input to causes. Continuous values derived from latents cannot enter the cause; they must go through Plane C (estimate with uncertainty) or be computed from observable features (when they exist).
- `Factor` cannot be constructed outside `observed_factor`.
- Factor order is deterministic.

## Implementation notes

1. **Test first, committed together with the fix.** Write the anti-leak test and run it locally on the pre-fix code; capture the failing output. Commit the test and the fix **in the same commit**, with the failing output in the commit message as evidence. Do not commit the test alone: the pre-commit hook would block it (`cargo test` fails by design). The test is structural: (a) no latent name in `Factor.name` after serialising a save; (b) no latent in `MatchEvent.fields`; (c) no `Outcome` field that is an exact function of a latent is exposed to the cause — best effort, covers known linear/simple cases, not exhaustive.
2. Remove `cause` from the 7 outcomes in `pcfm-hidden/src/outcome.rs`; remove the 7 `CauseTrace` constructions in `pcfm-hidden/src/lib.rs`.
3. Remove `injury_risk_modifier`, `temperament_modifier` (constants), `risk_modifier` (exact function of a latent): dead code, zero consumers.
4. In `pcfm-match/src/state.rs`, build the cause trace from the `Outcome` when the event is emitted. The builder pushes `observed_factor(name, Some(value), weight)` per observable field, in declaration order; `None` is skipped. `primary_cause` is dropped; `event_type` is the primary cause.
5. In `pcfm-stats/src/event.rs`, remove `primary_cause` and `explanation_template` from `CauseTrace`; simplify `validate()` to check only `timestamp_minute`; remove `EmptyCauseTrace`.
6. In `pcfm-model/src/cause.rs`, make `Factor` fields private; add `observed_factor(name, Option<f32>, weight) -> Option<Factor>`; `CauseTrace` keeps only `contributing_factors`. Update all construction sites. Verified today: 7 in `pcfm-hidden/src/lib.rs`, 1 in `pcfm-stats/src/chain.rs` (test), 1 in `pcfm-stats/src/event.rs` (test), 1 in `pcfm-match/src/narrative.rs` (test fixture), 1 in `pcfm-match/src/state.rs` (engine). Nine sites total; recount before editing (line numbers move).
7. Update tests: `pcfm-stats/src/chain.rs`, `pcfm-match/src/narrative.rs`, `pcfm-match/src/engine.rs:160` (`assert_ne!(ev.cause_trace.primary_cause, "")` → remove or replace), `pcfm-test-runner/src/features/l0_match.rs:240`, `pcfm-stats/src/event.rs`.
8. Bump `SIM_VERSION` in `crates/pcfm-match/src/save.rs`; add the `L0-match/CHANGELOG-SIM.md` entry (five-step procedure, including the "no code exists yet" stale text at lines 17-19, 41 — OI-5).
9. Update `L0-match/specs/10-ground-truth.md` (outcomes no longer carry `cause`); verify `05-match-events.md` stays coherent (it already asks for observed causes; the `Factor` example at `:117-124` may need a note that `value` is `f32` but the constructor enforces presence).
10. Fix the false doc comments: `outcome.rs:4` ("They carry no latent value") and `cause.rs:14` ("The value of the factor (observed)").

## References

- `doctrine/00-foundations.md` §3 (Three Planes), §5 (banned: no truth leakage, no ratings from latents)
- `L0-match/specs/05-match-events.md:117-124` (cause trace mandatory, observed values)
- `L0-match/specs/10-ground-truth.md:96-111` (outcome contract, observed values)
- `process/analysis/2026-10-30-13-39-review.md` (R-01)
- `process/analysis/2026-10-30-20-24-review.md` (R-01 confirmation)
- ADR-003 (Plane A enforcement, option B)
- ADR-012 (plugin contract), ADR-013 (local determinism) — for clause 6
- IDEA-INFRA-008 (explanation/narration as a plugin) — for clause 8
- Commit `73f7ad1` — the superseded decision (option 1b, incomplete premises)
