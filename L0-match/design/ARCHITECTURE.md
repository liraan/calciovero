# Architecture — Calcio Vero (Deep Simulator)

**Version:** 2.0
**Status:** active
**Last updated:** 2026-11-21 (crate map checked against the workspace; reading path in [`CODE_TOUR.md`](CODE_TOUR.md))

How the crates of L0 work together, how a match flows through them, and the two patterns that hold (the simulation/rendering split, the testkit contract). The architectural counterpart of [`doctrine/STRUCTURE.md`](../../doctrine/STRUCTURE.md); the reading path is [`CODE_TOUR.md`](CODE_TOUR.md).

## 1. The fifteen crates

| Crate | Role | Depends on |
|---|---|---|
| `ds-core` | primitives: deterministic math (`dmath`), integer geometry, ids, clock, hash | — |
| `ds-model` | the visible entities: player, squad (with the coach's instructions), match, pitch, referee | `ds-core` |
| `ds-db` | Plane A: the hidden characteristics derived by key from the world seed, the role profiles, the custodian door | `ds-core`, `ds-calculator` |
| `ds-calculator` | the formulas of the gestures and of the thought, each reading its `balance/` file | `ds-core` |
| `ds-orchestrator` | the decision: the scene a player perceives, how he reads it, what he chooses; the `resolve_*` functions | `ds-core`, `ds-model`, `ds-db`, `ds-calculator` |
| `ds-stats` | Plane B: `MatchEvent`, the append-only `EventChain` | `ds-core`, `ds-model` |
| `ds-narrative` | the commentary, the events told as a story, outside the engine (ADR-063) | `ds-core`, `ds-model`, `ds-stats` |
| `ds-match-api` | the contract of a match plugin: the eight tick phases, the plugin card, the plugin (ADR-063) | — |
| `ds-match` | the match engine: tick, movement, perception, Laws, events, save, replay, record, commentary | `ds-core`, `ds-model`, `ds-calculator`, `ds-orchestrator`, `ds-stats` |
| `ds-fixtures` | the test squads | `ds-core`, `ds-model` |
| `ds-testkit` | the scenario contract, free of every `ds-*` crate | — |
| `ds-test-runner` | the scenarios and the health check | `ds-testkit`, `ds-match`, `ds-fixtures`, `ds-core`, `ds-model`, `ds-stats` |
| `ds-demo` | the match and the friendly on the command line | `ds-match`, `ds-fixtures`, `ds-core`, `ds-model`, `ds-stats` |
| `ds-gui` | the friendly in a window (egui, ADR-055) | `ds-demo`, `ds-match`, `ds-fixtures`, `ds-core`, `ds-model`, `ds-stats` |
| `ds-reference` | the engine beside real football (Wyscout, CC BY 4.0) | `ds-match`, `ds-stats`, `ds-fixtures` |

ADR-041 split the old `pcfm-hidden` into `ds-db`, `ds-calculator` and `ds-orchestrator`, and leaves `ds-match` whole for now: it has more than one reason to change (OI-94). The split into plugins is proposed at L0 closed.

## 2. The three planes, in code

- **Plane A** (what a player is): `ds-db` only, behind the custodian door, visible to the calculator only (`custodian.rs`); nothing of it is in an event, a save field or a commentary line (the anti-leak test, `ds-match/tests/anti_leak.rs`).
- **Plane B** (what can be seen): positions and velocities (`spatial.rs`), the events (`ds-stats`), the tick record (`record.rs`).
- **Plane C** (what someone believes): each player's scene (`ds_orchestrator::PerceivedScene`), built from what his cone and scan let him see (`perception.rs`). Three readings of the off-ball thought still see the whole pitch instead (OI-106), and the memory of what was seen does not exist yet (MD-40).

## 3. The flow of a match

`simulate_from_seed` (`driver.rs`) builds the context and calls `simulate_match` (`simulate.rs`): two halves with their stoppage time (`stoppage_time.json`), each a loop of ticks, ten a second. Every tick (`step`):

1. **substitutions due** (Law 3, planned before the kick-off);
2. **`MatchState::tick`**, the substrate: the coach's instructions due (ADR-059), a pass that missed its man, the ball crossing a line (goal, goal kick, corner, throw-in), the ball carried, fatigue, a ball in the air met, a loose ball controlled, a challenge on the carrier, the coverage;
3. **`MatchState::decide`**: first every player without the ball decides (`decide_off_ball`), then the carrier, when his latency is due, chooses from his scene (`ds-orchestrator`), and the engine applies the outcome (`resolve_*`);
4. **the record**: the tick as the 2D view will play it back.

The order matters and is part of the behaviour: the carrier sees the movements just decided. Whether every player decides on the world of the previous tick is checked in the plugin ADR (OI-94).

## 4. The simulation/rendering split

The engine produces data: events, the tick record, the narrative as entries (`narrative.rs`). Nothing in the simulation path knows a screen or a platform (gate B9, ADR-038). `ds-demo` and `ds-gui` read the record and draw it.

## 5. The testkit contract

`ds-testkit` defines `Scenario`, `TestRegistry` and the feature ids without depending on any engine crate, so that L1 and the plugins can register scenarios against the same contract. `ds-test-runner --check --seed 42` is part of the quality gate (RULES C2).

## 6. What is not yet implemented

Tracked in `context/plans/L0-CLOSE.md` and `context/STATE.md`: the memory of what was seen (MD-40) and the scene through the cone only (OI-106); the size of the perception error, parts (a) and (c) (ADR-062); extra time and the shoot-out; the wall; the opposing coach reacting in the match; the knock that takes a player off; the dimmers (composure, courage, aggressiveness). The two old residues are gone: the RNG registry is no longer carried in `MatchContext` (the engine draws no die, ADR-035 and ADR-042), and `ds-core` has no event bus — events live in `ds-stats` (OI-40).

## 7. References

`doctrine/00-foundations.md`; ADR-035 (no dice), ADR-041 (the split), ADR-043 (the actor decides), ADR-052 (every player thinks), ADR-058 (the reception), ADR-059 (the coach speaks), ADR-062 (perception with error); `L0-match/balance/SOURCES.md`.
