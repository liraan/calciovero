# Changelog — SIM_VERSION

This file records every bump of `SIM_VERSION`, the version identifier of the **simulation behaviour**.

It is separate from [`CHANGELOG.md`](CHANGELOG.md) (which tracks documentation and structure) and from [`CHANGELOG-REGISTRY.md`](CHANGELOG-REGISTRY.md) (which tracks registry deprecations and migrations).

**Rule:** if the outcome of a simulation changes for the same seed and the same inputs, `SIM_VERSION` is bumped, and an entry is added here **before** the change is merged.

---

## Current SIM_VERSION

```
SIM_VERSION = 26
```

`SIM_VERSION = 1` was the initial baseline (no code). 2 removed `CauseTrace` and `xg` (ADR-024). 3 removed `template_key`/`params` from `NarrativeEntry`. 4 removed four gusci fields from `MatchEvent`/`MatchSave`. 5 replaced `player_id` with `actor: Option<ActorId>` and added `KickOff`/`SecondHalf` (ADR-026). 6: two halves with separate clocks, `Half` field, four-phase loop, fixed-point stoppage time, boundary events emitted (ADR-026 phase 2). 7: stoppage contributions calibrated (goal 60s, foul 20s, duel 10s). 8: slice B2 — the acting agent comes from space (`pick_from_space`), `Tackle` emitted when a duel is won by the defender, `SpatialState.carrier` tracked, stream names with `match_id` (OI-22). OI-22 closed: RNG stream names now carry `match_id` (`match:{id}:actions`, `match:{id}:action_kind`), so two matches with the same world seed no longer share the engine streams. The current value is the reference against which all future bumps are measured.

9: C.1 — `Pressure` emitted beside an action (Pass, Shot, Duel), calculated with `resolve_influence` (03-entities-minimal §2.9) on the ball's point; the most influential opponent within `space_threshold_cm` presses. A step toward the outcome (ControlError, later). Golden hash regenerated.

10: A1 — one RNG stream per noise consumer in `pcfm-hidden` (`hidden:{match_id}:{latents,duel,shot,pass,save,injury,set_piece,pressure}`), replacing the single shared stream. Adding a new consumer no longer shifts the outcomes of the others (`00-foundations` §4, `01-rng.md` §2.3). Closes OI-39. Golden hash regenerated.

11: F.3-b2 — the ball crosses the goal line as a geometric fact: `Shot` carries `goal` and `on_target` (was: `goal` only, on the shot). `Goal` is emitted only from a `Fact::LineCrossing { kind: GoalLine }`, not from the shot. `Pass` is emitted for the first time (was: `resolve_pass` called, no event). `Fact::Trajectory` carries `kind` and `success`; `Fact::LineCrossing` carries `actor` and `point`. `classify` extended; six unit tests. Golden hash regenerated. Behavior change: a `Pass` event appears in the stream, and `Shot.goal` now comes from the line crossing. F.3-b1 (BallState 3D) did not bump: no behaviour change (golden identical).

12: F.4-a — agents move every tick (MD-09, MD-10, MD-12). `move_agents` reads the ball's offset from the pitch centre, translates every formation slot by a declared fraction (30%), deviates the nearest agent to the ball, steps each agent by at most `MAX_STEP_CM` (120), and enforces a minimal separation (`BODY_RADIUS_CM` 45). No RNG in the movement (S1): the variation comes from the inputs. Golden hash regenerated; the match now moves, so the hash changes. The separation pass is a `Vec` snapshot (same operations, same order, no per-pair map lookup): same behaviour, 28 s -> 5.8 s on the golden.
13: F.4-c — the ball flies and the possession resolves at landing, not at launch. `Pending` (MD-37) with a mandatory expiry (`FLIGHT_TICKS` = 15) and the expected possession; `other()` computes the opposite without mutating; the check at the top of `tick` applies the possession when the pending is due. In the pass and shot branches the carrier is released at launch, `set_ball_in_flight` is no longer a no-op, `advance_ball` runs. Declared simplification: no interception yet (OI-37), the ball lands at the destination or out; the goal stays the die's outcome (MD-20). Golden hash regenerated (behaviour changed: the ball moves between ticks).
14: F.4-c2 — the situated decision replaces the global die (MD-01, MD-43). `choose_action` is geometric and reads `visible_to`: shoot if in range, pass to a visible teammate with space, duel otherwise. `action_kind_roll` and `action_kind_stream` are removed: no die in the choice, the variation stays in `resolve_*` and in the degraded image. The pass aims at the actual receiver, not a fixed point. The separate foul branch is removed (the foul is a duel outcome, OI-48). `action_roll` (whether an action happens in the tick) stays as declared pacing; finding tracked, density is MD-16. Audit test (MD-43): a pass receiver must be visible from the attacker. Golden regenerated.
15: F.4-c2b — the cone does not filter the pass target ("filtro partita vera", A, 2026-11-04). In `choose_action` the passer considers every teammate with their freedom; the cone (`visible_to`) is removed from the choice. A pass target is a *choice*, not passive perception: a player looks at the teammate he serves. The cone's real consumer is the interception of a pass by an opponent the passer did not see, which is OI-37. Declared: the cone is not read by the tick today (audit test: a mate behind the passer, outside the cone, is still chosen). Audit test replaced: the previous tautology (`!x || x`) was a D5 leak and is removed. Golden regenerated.
16: F.4-d — interception: the cone filters the opponents, not the target. At launch the passer saves the opponents outside his cone; `find_interceptor` (integer point-to-segment distance, `isqrt`) checks if one is within `INTERCEPT_RADIUS_CM` (200) of the pass line. The pending carries `interceptor` and `passer`; at resolution `Interception` is emitted with `from`/`to` and the possession flips. Shot passes `None` (a shot is not intercepted in this model, declared). The cone now has its real consumer. Golden regenerated; measured on the demo: seeds 44-46 produce 8/23/17 interceptions, seed 42 none (geometry-dependent, not a bug).

17: F.3-b3 — shot schema, goalkeeper, goal from the crossing. `ShotOutcome` loses `goal` and gains `to` (3D arrival after the deviation), `initial_velocity`, `on_target` (geometric, from the goal mouth), `shot_type` (choice, derived geometrically). `ShotContext` gains `difficulty` (pressure, computed in `state.rs`); the die widens its deviation bidirectionally, so a strong finisher deviates less. `SaveContext` gains `point` (the crossing point); `resolve_save` unchanged in formula, but now called from `check_shot_crossing` when the ball crosses the goal plane inside `GK_REACH_CM` of the goalkeeper. `Fact::Trajectory` is a struct variant with the new shot payload; `Fact::GoalLineCrossing` is emitted via `classify`. The goalkeeper (slot 0) is anchored (`GK_LINE_OFFSET_CM` 200 cm, `GK_ANCHOR_CM` 250 cm): `from_squads` places it on the goal line, `move_agents` skips it. Declared derogation MD-38 stays: ball out of the goal mouth → possession flips, no restart. **Not observable in seeds 42–51** (0 shots in the golden range); the behaviour is exercised by the new `functional_l0` test that forces a shot. Bump declared for hygiene (D5: a bump too many beats a silent schema change). Golden hash regenerated (identical, 0 shots in the range): the bump declares the schema change, the golden cannot see it.

18: F.5 — Pressure is a coverage relation, not a tick event. `Fact::RelationEnterExit` is emitted when an opponent enters (or exits) the coverage of the carrier (`space_threshold_cm` around the ball). The narrator classifies `entered: true` as `Pressure` with `from`/`to`; `entered: false` emits nothing (the pressure ended, nothing new started). This removes the "Pressure beside the action" bridge (decision 2026-11-02): `Pressure` is now driven by the coverage change, not by the action. `update_coverage` is called once per tick after `move_agents`; the set is persistent (`coverage`, `coverage_possession`), discarded on possession change. **T1-bis amended:** `PositionChange` and `HeadingChange` are not `Fact`; they are tick-record fields (IDEA-INFRA-018) read as background by 2D/3D, not classified into events. The only movement fact with an event consumer is `RelationEnterExit`. **Arbitrary parameters of F.3-b3 declared here, as the Plan requires:** `GK_LINE_OFFSET_CM` 200, `GK_ANCHOR_CM` 250, `GK_REACH_CM` 200, shot deviation `spread_base` 400 × (1 − finishing), `spread_diff` 2 × difficulty, goal mouth `goal_half_width_cm` 366, `goal_height_cm` 244, `INTERCEPT_RADIUS_CM` 200, `PASS_FREEDOM_CM` 800, `SHOOT_RANGE_CM` 2500, `MAX_STEP_CM` 120, `BODY_RADIUS_CM` 45, `FLIGHT_TICKS` 15, `action_probability` 0.0005. Golden hash regenerated (Pressure now depends on coverage changes, not on actions).

28: E step 3 -- `resolve_save` loses the die, the hold is geometric (ADR-035, OI-50). The `next_unit(&mut self.streams.save)` draw is removed. `SaveContext` gains `offset_cm` (lateral distance from the keeper to the crossing point, computed by `check_shot_crossing`) and `reach_cm` (standing reach = height/2, already computed); `point` stays for the `Save` event payload, not for the formula. Formula: `saved = offset_cm < goalkeeping * reach_cm` -- the effective reach is `goalkeeping * reach`, so a keeper with skill near 0 reaches almost nothing and one with skill 1 covers the full arm span. The `goalkeeping` latent is consumed (was declared in L0.11 §7.2 as the save consumer, now actually consumed). `Streams::save` removed (no consumer). **Not observable in seeds 42-51** (0 shots in the range, so 0 saves, same as entries 17, 22, 27): the behaviour is exercised by the new `save_forced.rs` (4 tests: centre always saved, far outside never saved, deterministic, reach widens the window). Test count 211 -> 215 in README, STRUCTURE, CONTRIBUTING, specs/_index. `SIM_VERSION` 27 -> 28, golden hash identical. Balance note: with real goalkeeping at L0 most shots in the reach are saved; the calibration (which fraction) is L3. Related: ADR-035 Consequences, OI-50, E step 2 (entry 27), OI-62 (the dive extends reach, deferred).
27: E step 2 -- `resolve_shot` loses the die, the deviation is geometric (ADR-035, OI-50). The two `next_unit` draws (`streams.shot` for `y_rand` and `z_rand`) are removed. The deviation is now: `spread = (1 - finishing) * 400 + difficulty * 2` (magnitude, unchanged from before); lateral side `dy_sign = +1 if from.y < to.y else -1` (a real shooter misses toward the side he is on, geometric from positions); vertical `dz_sign = +1 if difficulty > 50 else -1` (up under pressure, down otherwise, threshold declared); amplitude per axis `spread / 4` (the average magnitude of the old uniform `(rand - 0.5) * spread`, same scale); `to = (ctx.to.x, ctx.to.y + dy_sign*dy_abs, ctx.to.z + dz_sign*dz_abs)`. `on_target` and `shot_type` stay geometric (unchanged). `Streams::shot` removed from `pcfm-hidden` (no consumer). `speed_from_success` renamed `speed_from_on_target` (`success` param -> `on_target`; the function is a function of on-target, not of the die). **Not observable in seeds 42-51** (0 shots in the golden range, same as entries 17 and 22): the behaviour is exercised by `shot_forced.rs` (5 tests now: schema, determinism, miss-toward-shooter-side, high/low pressure vertical, pressure widens lateral deviation; two of the old tests removed because they tested the die). Test count 209 -> 211 in README, STRUCTURE, CONTRIBUTING, specs/_index. `SIM_VERSION` 26 -> 27, golden hash identical (0 shots in the range). Balance note: with real-world `finishing` values at L0 the shot never goes over the bar nor wide; it is a calibration issue (L3), declared here, not a silent limit. Related: ADR-035 Consequences, OI-50, REC (entry 26). **Note 2026-11-08 (OI-79):** the lateral rule stated above ("a real shooter misses toward the side he is on") is **false in a real match**: the direction depends on the foot (ADR-031), the point of impact and the posture, not on the shooter's position. `lateral_sign` (named `dy_sign` at the time of this entry) is a **placeholder**, the foot is not read here yet (`Removed-by: ADR-031 + OI-65 step 3+`). No behaviour change, no new bump: this note corrects the record, it does not change the simulation.
26: REC -- reception of the pass, the first step of E (ADR-035). `resolve_pass` loses the die: `complete` is no longer in `PassOutcome`; the flight velocity is a function of the passer's `passing` (`speed_from_passing`, `balance/reception.json`). `resolve_reception` (geometric: `first_touch` + freedom - distance, no die) runs at the pending's expiry: if an interceptor was on the line, the possession flips as before; otherwise the receiver tries to control, and the `Pass` event is emitted at landing (`Fact::Trajectory::Pass`, `success = controlled`), not at launch. `PassContext` gains `receiver_freedom_cm`; `Pending` carries `receiver`, `freedom_cm`, `distance_cm`. `TechnicalLatents` gains `first_touch` (from_key, no shift thanks to OI-53). Behaviour change: seeds 42-51 all change; passes now fail only by interception or by a failed reception, not by a die. `SIM_VERSION` 25 -> 26.

25: OI-71 -- scanning: the gaze turns towards the most salient candidate (ADR-034). `perception::scan` chooses among the three relational candidates the L0 state supports: the ball, the opponent nearest the ball, the opponent nearest the player (the mark). Candidates 4 (receiving space) and 5 (line) are declared, not implemented (`Removed-by: OI-71`). The cascade is the declared one: the mark beats the ball when within `mark_radius_cm`; otherwise the highest salience wins, ties by `PlayerId::slot()`. `move_agents` takes `tick` and a `ScanningBalance`; `write_back` runs the scan every `scan_interval_ticks` (5), in a second pass so the movement loop keeps `&mut`. `AgentState::gaze` is no longer "always the ball": the F.6 step 2 filter (`perceived_ball` in `move_agents`) is now real -- an agent that does not see the ball does not chase it. Parameters in `balance/scanning.json`, embedded at compile time (`include_str!`), no RNG, no float. Behaviour change: seeds 42-51 all change, because the gaze now differs from "towards the ball" for most of a match and the chaser is chosen from those who perceive the ball. `SIM_VERSION` 24 -> 25.

24: OI-72 -- gaze separated from heading (ADR-033). AgentState gains gaze: u16, the direction the agent looks at, distinct from heading (the body direction of motion). At kickoff both squads look at the ball (gaze = 90 home, 270 away); move_agents updates gaze every tick via heading_towards(agent.position, ball), a simplification declared Removed-by: OI-71. visible_to and perceived_ball now read gaze, not heading: the cone filters what the agent sees, not where he runs. Behaviour change. Not visible in seeds 43-46 and 49-51 (identical hashes); seeds 47-48 change because gaze differs from heading changes the interceptor visibility in a pass. Seed 42 coincides with SIM_VERSION 19 by RNG artefact, verified not a regression (toggle visible_to back to heading restores 9c632da4, 73/73 lib tests green). SIM_VERSION 23 -> 24.
23: F.7 step 10 — the movement formula reads fatigue. `balance/movement.json` gains `fatigue_coefficient` (0.5) and `fatigue_floor` (0.5); `MovementBalance::formula()` multiplies acceleration by `Clamp(Sum(1, -c*fatigue), floor, 1)`. The grammar `Formula` gains `Sum(Vec<Self>)`, the minimum needed to express `1 - x` (a `Datum(FatigueModifier)` was rejected: it would hide the coefficient in Rust, against ADR-028). **Not observable in seeds 42-51:** no producer writes `fatigue` during a match (`PlayerBody.fatigue = Player.physical.fatigue = 0.0` always), so the modifier is identity there. The behaviour is exercised by `crates/pcfm-hidden/tests/movement.rs` (3 tests: stride by height, acceleration by weight, fatigue reduces acceleration). `SIM_VERSION` 22 -> 23: the bump declares the behaviour change (possible, not observed), same as entry 20.
22: F.7 step 8 — goalkeeper reach is geometry, not a constant. `GK_REACH_CM` (200 cm, arbitrary) is removed. The goalkeeper now saves if the crossing point is within `reach_cm = height_cm / 2` (arm span ≈ height, an anthropometric fact: Plane B, no new number) of his current position, and `resolve_save` decides whether he holds it. **Geometry decides "does he arrive", the die decides "does he hold"** — the same separation as MD-20 (the goal is geometric, the save is a precondition plus a die). **Not observable in seeds 42–51** (0 shots in the golden range): the behaviour is exercised by `shot_forced.rs` and the functional test, not by the golden. The dive is **deferred**: `dive_ref_cm` and `dive_time_s` need a cited sports-science source, and a term with no derivation does not enter the model (F.7 principle). The standing reach `height_cm / 2` is the fallback, and `Removed-by: F.gk-dive` is declared. `GK_ANCHOR_CM` stays: the keeper does not run after the ball; lateral repositioning for crosses is a separate phenomenon, out of F.7. `SIM_VERSION` 21 → 22, golden hash **identical** (0 shots in the range): the bump declares the behaviour change, the golden cannot see it. Same as entry 17.
21: F.7 step 7b — `move_agents` applies per-agent movement. Each agent's displacement now comes from `resolve_movement` (the movement formula in `balance/movement.json`, `pcfm-calculator`), not from a fixed step. The formula `a = accel_ref * explosive_force * (weight_ref / weight)` is a *force* (cm/tick²); `v_max` and `stride` scale it to displacement (`v_max * stride`, `a * stride`). `PlayerBody { height_cm, weight_kg, fatigue }` is built once in `MatchState::new` from the squads and passed to `move_agents`. `velocity` stores the previous displacement; its L-infinity norm is the accumulated speed (no reader today; the tick record will read displacement, IDEA-INFRA-018). `MAX_STEP_CM` is removed (the formula replaces the clamp). The model is per-axis (L-infinity): a diagonal step has the same per-axis magnitude as a cardinal one, matching the previous rule (MD-09). `GK_ANCHOR_CM` still excludes the goalkeeper from the pass (declared simplification; step 8 replaces it with the same model). Golden hash regenerated; seeds 42-51 change, as expected: every agent's displacement is now a function of the player's body and the formula, not a clamp. **Note on `v_max`:** it is a per-axis clamp (`v_max * stride` per component), not a Euclidean norm; the field's doc says so. `SIM_VERSION` 20 → 21.

20: OI-58 — `next_unit` never returns 1.0. The function took `(next_u32 as f32) / (u32::MAX as f32)`: `u32::MAX as f32` rounds to 2^32, so a `next_u32` near the maximum gave exactly `1.0` in f32, outside the declared `[0, 1)`. A 1000-draw test did not catch it (P1: a test that cannot fail). The fix takes the top 24 bits of the `u32` (f32 has a 24-bit mantissa) and divides by 2^24: every value is exactly representable, the maximum is `1 - 2^-24`, strictly below 1.0. A test forces the edge case. **The golden hash is identical**: in seeds 42-51 the edge case does not occur, and the change does not alter the values that do occur. The bump is declared for the behaviour change (possible, not observed), and for the new `pcfm-core` test. `SIM_VERSION` 19 → 20.

19: OI-53 — latents derived by key, not by sequence (ADR-029). `PlayerLatents::from_rng` is removed; a player's latents are derived from `hash(world_seed, player_id, characteristic)` (`PlayerLatents::from_key`), with **no `match_id`**: a player's strength is the same in every match of his world. The `latents` stream disappears from `Streams`. Adding a characteristic no longer shifts the others: each value is derived from its own purpose string. **Every match changes**, because every latent changes: the golden hash is regenerated once (this bump), and then it is stable. The distribution is still uniform in `[0,1)` (prototype; the factor structure is L3's job, L0.11 §11.1); this bump fixes the derivation, not the distribution. **The noise half of ADR-029 is decided but not implemented**: it waits for the per-agent loop (OI-44), where the iteration order would otherwise matter. `SIM_VERSION` 18 → 19.
**Known gap (OI-14):** the 10-season golden test (an L1 artefact) is not implemented. The L0 golden test (N matches, seeds 42..51) exists and is regenerated on every bump.


`SIM_VERSION` is defined in `pcfm-match` (`crates/pcfm-match/src/save.rs`) and embedded in every save (see `L0.07` §2.1).

---

## When to bump

`SIM_VERSION` is bumped when **any** of the following is true:

1. **RNG behaviour changes** — algorithm, seed derivation, or stream naming (see `L0.01` §2.6).
2. **Event emission changes** — an event type is added, removed, or its emission rules change in a way that affects the event stream (see `L0.05`).
3. **Resolution logic changes** — a `resolve_*` function in `pcfm-hidden` returns a different outcome for the same inputs (see `L0.03` §3.2).
4. **Tick loop changes** — the order of operations within a tick changes, or the tick granularity changes (see `L0.04` §2.2).
5. **Random event catalogue changes** — a `RandomEventDef` is added, removed, or its probability function changes in a way that affects the distribution (see `L0.08` §2.5).

If none of the above is true, `SIM_VERSION` is **not** bumped. A refactor that preserves behaviour does not bump `SIM_VERSION`. A new feature that does not affect the match does not bump `SIM_VERSION`.

---

## Bump procedure

A `SIM_VERSION` bump requires **all five steps**, in order. In phase 1 (ruleset disabled, no CI, single maintainer) the local C2 verification replaces the CI merge gate; see `AGENT-POLICY.md` §1 and §9.

1. **Bump the value** in `pcfm-match` (`crates/pcfm-match/src/save.rs`). The value is a monotonic integer. It never goes down.
2. **Regenerate the golden hash** — run `cargo test -p pcfm-match --test golden_l0`, copy the `left:` value from the failure, replace `EXPECTED` in `golden_l0.rs`. At L0 the test is N matches (not 10 seasons: L1 does not exist). The L1 form is a future artefact.
3. **Write the entry** in the "History" section below, using the template.
4. **Verify save compatibility** — every existing save fixture must still load. If a save cannot be reconstructed under the new `SIM_VERSION`, the entry must say so explicitly and a migration must be provided (see `L0.07` §2.3).
5. **Review by a second pair of eyes.** In phase 1 there is one maintainer: the post-push review (`AGENT-POLICY.md` §9) replaces the second reviewer. In phase 2 (ruleset active, branch + PR), the reviewer confirms that the golden hash change is expected and that the entry is honest.

The full migration procedure for **saves** lives in [`docs/MIGRATION-01-04.md`](docs/MIGRATION-01-04.md). That document points back to this changelog for the bump side and forward to the save side.

---

## Entry template

Every entry has the same shape. Copy this template and fill it in.

```markdown
## [SIM_VERSION N] — YYYY-MM-DD

### Why the bump

One paragraph. What changed and why it could not be done without changing
simulation behaviour.

### What changes for the same seed

Concrete. "Match X (seed 12345) now ends 2-1 instead of 1-1 because ...".
If the change is statistical (affects distributions but not any single
match), say so and give the distribution delta.

### Saves affected

- Can existing saves still be loaded? (yes / no / with migration)
- Can existing saves be replayed? (yes / no / with warning)
- If a migration is required, where does it live? (link)

### Golden hash

- Previous: `0x...`
- New: `0x...`
- Verified by: (reviewer name or handle)

### Related changes

- `CHANGELOG.md` entry: (link or "none")
- `CHANGELOG-REGISTRY.md` entry: (link or "none")
- ADRs affected: (list or "none")
```

---

## History

## [SIM_VERSION 8] — 2026-11-01

### Why the bump

Slice B2: the acting agent comes from space, not from a random roll.
`pick_players` (random attacker + random defender, same roll) is replaced
by `pick_from_space`: the attacker is the agent of the possessing team
nearest the ball, the defender the nearest opponent to the attacker.
`SpatialState.carrier` tracks who has the ball. A duel won by the
defender emits `Tackle` with `from`/`to`.

### What changes for the same seed

The attacker and defender are no longer random: they are the agents
nearest the ball. The event stream changes (who shoots, who passes, who
duels), so the score and the chain hash change. Seed 42 produces a
different match.

### Saves affected

No schema change. The `SIM_VERSION` bump is for the simulation behaviour,
not the save format. A v7 save loads under v8, but re-simulation differs.

### Golden hash

- Before: `d8470ee8...`
- After: the new value in `crates/pcfm-match/tests/golden_l0.rs:21`
  (regenerated from the failure, as the procedure says).

### Related changes

- `crates/pcfm-match/src/state.rs`: `SpatialState` field, `pick_from_space`,
  `Tackle` on a won duel, carrier set/cleared.
- `crates/pcfm-match/src/orchestrator.rs`: `MatchState::new(ctx.match_)`.
- `crates/pcfm-match/src/save.rs`: `SIM_VERSION` 7 -> 8.

## [SIM_VERSION 7] — 2026-11-01

### Why the bump

ADR-026 phase 2 calibration. The initial stoppage-time contributions
(goal 30s, foul 30s) were a first proposal, not a calibrated value. The
match recovery was ~2'50" for a half with 4 goals, low against the real
game. New values: goal 60s, foul 20s, duel 10s, base 0.

### What changes for the same seed

The recovery (and therefore the timestamp of the boundary events) changes.
Seed 42: first half 45'+2' -> 45'+4', second half 45'+1' -> 45'+2'.

### Saves affected

No schema change. The `SIM_VERSION` bump is for the simulation behaviour,
not the save format. A v6 save loads under v7, but the timestamps of a
re-simulation differ.

### Golden hash

- Before: not recorded. OI-14 tracks the missing golden test.
- After: not recorded. OI-14 still open.

### Related changes

- `balance/stoppage_time.json`: goal 60, foul 20, duel 10, base 0.
- `pcfm-match::balance` tests updated.

## [SIM_VERSION 6] — 2026-11-01

### Why the bump

ADR-026 phase 2: two halves with separate clocks, `Half` field on every
event, four-phase tick loop, fixed-point stoppage time, boundary events
(`KickOff`, `HalfTime`, `SecondHalf`, `FullTime`) emitted.

### What changes for the same seed

Events carry `half: Half`; `timestamp_minute`/`timestamp_second` are
relative to the half. The match runs two halves of 45' plus a fixed-point
stoppage time each. `FullTime` is at `(Second, 45+rec2)`.

### Saves affected

Breaking: `half` added to every event. A v5 save does not load under v6.

### Golden hash

- Before: not recorded. OI-14 tracks the missing golden test.
- After: not recorded. OI-14 still open.

### Related changes

- ADR-026 (Match boundaries and stoppage time), amended 2026-11-01.
- `balance/stoppage_time.json` (new, embedded with `include_str!`).
- `pcfm-match::orchestrator`: four-phase loop.
- `pcfm-demo`: board shows `1T`/`2T` and `45'+X'`.

## [SIM_VERSION 5] — 2026-11-01

### Why the bump

ADR-026 (accepted 2026-11-01): match boundaries and stoppage time.
`MatchEvent.actor: Option<ActorId>` replaces `player_id: PlayerId`; new
`EventType::KickOff` and `EventType::SecondHalf`; `HalfTime` and `FullTime`
emitted; two halves with separate clocks and separate stoppage time.

### What changes for the same seed

Events carry `actor` instead of `player_id`. Boundaries (`KickOff`,
`HalfTime`, `SecondHalf`, `FullTime`) have `actor: None`. Match end time is
`45+rec1` for the first half and `45+rec2` for the second, with `rec`
computed from the events of each half (fixed point, no cap; guard at 45').

### Saves affected

Breaking: `player_id` removed, `actor` added. A v4 save does not load under
v5. Migration is not provided: no v4 save exists outside test fixtures, and
the fixture is regenerated. The `docs/MIGRATION-01-04.md` procedure is the
template; a `MIGRATION-04-05.md` is not needed for phase 1.

### Golden hash

- Before: not recorded. OI-14 tracks the missing golden test.
- After: not recorded. OI-14 still open.

### Related changes

- ADR-026 (Match boundaries and stoppage time) — accepted 2026-11-01.
- `pcfm-core`: `ActorId` (`Player | Referee`).
- `pcfm-stats::MatchEvent`: `player_id` -> `actor`.
- `pcfm-stats::EventType`: `KickOff`, `SecondHalf` added.
- `pcfm-match::narrative`: handles `actor` and the new event types.
- `pcfm-demo::viewmodel`: `EventRow.team` gains `Match` for boundary events.

## [SIM_VERSION 4] — 2026-10-31

### Why the bump

`MatchEvent` lost three fields: `career_minutes_cumulative`, `season_label`, `observed_attrs_snapshot_hash`. All three were always `None` or a duplicate of another field: `career_minutes_cumulative` equalled `minutes_played_cumulative`, `season_label` was `None` in production, `observed_attrs_snapshot_hash` was `None`. The event payload enters `chain_hash`, so the bump is mandatory. `MatchSave` also lost `config_versions`, always empty: that alone would not require a bump, but it is included in the same change.

### What changes for the same seed

The `chain_hash` differs from SIM_VERSION 3 for the same seed: the serialised `MatchEvent` has three fewer fields. Score, possession, shots and the events themselves (type, minute, player) are identical.

### Saves affected

- Load: yes, unknown fields are ignored.
- Replay: no — `can_replay()` returns false for SIM_VERSION < 4.
- Migration: none. Old saves are read-only.

### Golden hash

- Not recorded. The 10-season golden test does not exist yet (OI-14).
- Verified by: C2, locally, 2026-10-31.

### Related changes

- `crates/pcfm-stats/src/event.rs`: three fields removed from `MatchEvent`; `Hash` import removed.
- `crates/pcfm-match/src/save.rs`: `config_versions` removed; `SIM_VERSION` 3 -> 4.
- Specs: `05-match-events.md` §2.4/§2.5 (removed fields), `07-save-minimal.md` (`config_versions`).
- IDEAS: IDEA-INFRA-012 (career/season data at L1), IDEA-INFRA-013 (observed-attrs snapshot at Plane C).

---

## [SIM_VERSION 3] — 2026-10-31

### Why the bump

`NarrativeEntry` lost `template_key` and `params`. Both were produced by `narrate()` and serialised in the save, but had no consumer: `render.rs` reads only `minute` and `text`, and `locales/` does not exist. The payload is smaller; the save shape changed, so the bump is mandatory.

### What changes for the same seed

The `chain_hash` is unchanged (narrative is not part of the chain). The save JSON differs: `result.narrative[*]` has four fields instead of six. Score, stats, events and `state_hash` are identical. Old saves written at SIM_VERSION 2 load (serde ignores unknown fields) but cannot be replayed under SIM_VERSION 3.

### Saves affected

- Load: yes, unknown fields are ignored.
- Replay: no — the sim version differs, `can_replay()` returns false.
- Migration: none.

### Golden hash

- Not recorded. The 10-season golden test does not exist yet (OI-14).
- Verified by: C2, locally, 2026-10-31.

### Related changes

- `crates/pcfm-match/src/narrative.rs`: `NarrativeEntry` reduced to `{minute, event_id, text, importance}`. `template_for` → `importance_for`; `render_text` takes `EventType`.
- `crates/pcfm-demo/src/render.rs`: test fixture updated.
- IDEAS: IDEA-INFRA-011 (narrative localisation at L1/L3).

---

## [SIM_VERSION 2] — 2026-10-31

### Why the bump

ADR-024 removed `CauseTrace` and `Factor` from L0 (supersedes ADR-023). The event payload changed: `MatchEvent.cause_trace` is gone, `ShotOutcome.xg_value` is gone, the latent-derived modifiers (`injury_risk_modifier`, `temperament_modifier`, `risk_modifier`) are gone. Because the event payload enters `chain_hash`, the same seed now produces a different chain hash. A bump was mandatory.

### What changes for the same seed

Every match: the `chain_hash` differs from SIM_VERSION 1 for the same seed, because the serialised `MatchEvent` has fewer fields. The score, possession and shots are **not** expected to change (the resolution logic is identical; only the serialised payload is smaller). Verified by the test suite and the test-runner (seed 42): no match result changed, only the hash.

### Saves affected

- Can existing saves still be loaded? **Yes** — `serde_json` ignores unknown fields, and the removed fields simply do not deserialise.
- Can existing saves be replayed? **No.** Replay re-simulates under the new rules; the chain hash will not match the saved one. Old saves are therefore **read-only**. The contamination is declared, not hidden.
- Migration: none. The fix does not retroactively clean JSON written before the bump.

### Golden hash

- Previous: not recorded (no golden test existed at SIM_VERSION 1 either).
- New: not recorded. OI-14 tracks the missing golden test.
- Verified by: C2 (build, clippy, 102/102 tests, fmt, test-runner 6/6), locally, 2026-10-31.

### Related changes

- ADR-024 (Remove `CauseTrace` at L0) — accepted, supersedes ADR-023.
- `specs/L0-match/05-match-events.md` §2.3 amended.
- `specs/L0-match/10-ground-truth.md` §2.2 amended.
- `specs/L0-match/06-match-output.md` §2.3 amended (xG removed).
- Anti-leak test: `crates/pcfm-match/tests/anti_leak.rs` (fails pre-fix, passes post-fix).
- IDEAS: IDEA-INFRA-009 (CauseTrace at L1).

---

<!--
  NOTES FOR FUTURE EDITORS:

  - Do not edit a past entry. If a past entry was wrong, add a correction
    entry and reference the one it corrects. The history is append-only.
  - Do not bump SIM_VERSION "to be safe". A bump is a promise to the user
    that their saves and replays are affected. Bump only when necessary.
  - If you are unsure whether a change requires a bump, ask in an issue
    before merging. The cost of a missed bump is a silent determinism
    regression, which is the worst kind of bug this project can have.
  - This file is linked from:
      - specs/00-foundations.md §4 (determinism)
      - specs/L0-match/01-rng.md §2.6
      - specs/L0-match/07-save-minimal.md §2.3
    If you move or rename this file, update those links.
-->
