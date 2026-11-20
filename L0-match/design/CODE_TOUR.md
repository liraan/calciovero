# Code Tour — Calcio Vero (Deep Simulator)

**Version:** 2.0
**Status:** active
**Last updated:** 2026-11-15 (rewritten against the code at `SIM_VERSION` 114, OI-49)

This is the **reading path** for the code, for someone who wants to contribute and does not know where to start. It follows the code as it is, not as it was planned. The rules of the game are in `context/RULES.md`; the doctrine in `doctrine/`.

Three ideas hold the engine together:

- **No dice** (ADR-035). Every outcome is a cause: the players' characteristics, what each one sees, the physics of the ball, the Laws of the Game. The same match from the same inputs is the same, bit for bit, on every machine (`ds_core::dmath`, the golden test).
- **Every player thinks** (ADR-052). The engine does not decide for anybody: it gives each player the scene he perceives, he imagines his options and picks one with a single measure (closer to scoring, closer to conceding).
- **Three planes** (`00-foundations` §3). Plane A is what a player is (hidden values, `ds-db`); Plane B is what can be seen (positions, events, `ds-stats`); Plane C is what someone believes. Nothing of Plane A leaves to the events.

## Before you start

```bash
cargo build --release
cargo test --workspace
cargo run --release -p ds-demo -- --seed 42
cargo run --release -p ds-gui
```

The quality gate (RULES C2) is in `README.md`.

## Stage 1 — The primitives: `ds-core`

| File | What to look at |
|---|---|
| `dmath.rs` | Why `hypot`, `ln`, `powf` do not come from `std`: one bit of the system's libm changes a match (SIM 101). |
| `geometry.rs`, `ids.rs`, `time.rs`, `hash.rs` | Integer geometry, generational ids, the clock, the BLAKE3 hash of the state. |
| `identity.rs` | The two facts that identify a match in a world. |
| `rng.rs`, `event_bus.rs` | Residues: the engine draws no random number (ADR-035) and nobody listens to the bus (OI-40). Do not build on them. |

## Stage 2 — The entities: `ds-model` and `ds-db`

| File | What to look at |
|---|---|
| `ds-model/src/player.rs`, `squad.rs`, `match_.rs`, `pitch.rs` | The visible entities. `Squad.instructions` carries the coach's orders during the match (ADR-059). |
| `ds-db/src/latents.rs` | Plane A: every characteristic of a player, derived by key from the world seed, grouped by family. |
| `ds-db/src/profiles.rs` | How the test squads are generated from `balance/profiles.json`. |
| `ds-db/src/custodian.rs` | The only door to Plane A, open to the calculator only. |

## Stage 3 — How a player acts: `ds-calculator` and `ds-orchestrator`

| File | What to look at |
|---|---|
| `ds-calculator/src/thought.rs` | The thought (ADR-052): what a player forecasts and how he judges it. |
| `ds-calculator/src/kick.rs`, `save.rs`, `duel.rs`, `control.rs`, `fatigue.rs`, `movement.rs` | The formulas of the gestures, each reading its characteristics and its `balance/` file. |
| `ds-orchestrator/src/read.rs` | How the player reads the scene he perceives (`PerceivedScene`): passes, shots, carries, the offside line (`offside_to`, SIM 113-114). |
| `ds-orchestrator/src/offball.rs` | What a player without the ball does (ADR-049). |
| `ds-orchestrator/src/actor.rs`, `choice.rs` | The decision belongs to the actor, not to the engine (ADR-043). |
| `ds-orchestrator/src/lib.rs` | The `resolve_*` functions: a pass, a shot, a duel, a save, applied to the world. |

## Stage 4 — The match: `ds-match`

| File | What to look at |
|---|---|
| `simulate.rs`, `driver.rs` | One tick of the match, and the entry point `simulate_from_seed`. |
| `state.rs` | The match in progress: who decides when, the scene each player gets (`decide_off_ball`, the carrier's choice), the restarts, the substitutions. The largest file (OI-94: it has too many reasons to change). |
| `spatial.rs` | Positions, the ball's flight through the air and on the grass, interceptions. |
| `perception.rs` | The vision cone and the scan (ADR-033, ADR-034). |
| `rules.rs` | The Laws: restarts, offside, the obvious chance, the penalty. |
| `fact.rs` | Facts become events (Plane B). |
| `save.rs`, `record.rs` | The save, the replay, `SIM_VERSION`, the tick record the 2D view plays back. |
| `narrative.rs`, `engine.rs` | The commentary and the text engine. |
| `balance.rs` | The loaders of `L0-match/balance/*.json`; every number has its row in `balance/SOURCES.md`. |

The tests to read first: `tests/cases/golden_l0.rs` (the golden), `tests/cases/repertoire.rs` (every gesture appears), `tests/cases/instructions.rs` (the coach speaks); all three are modules of `tests/suite.rs` and share one match cache.

## Stage 5 — The tools

| Tool | What it is |
|---|---|
| `tools/ds-demo` | The match on the command line; the friendly you can play (`--xi`, `--block`, `--save`, `--why`). |
| `tools/ds-gui` | The friendly in a window, the 2D pitch from the tick record (ADR-055). |
| `tools/ds-reference` | The engine beside real football (Wyscout Serie A 2017-18). |
| `tools/ds-test-runner`, `tools/ds-testkit` | The scenarios and the health check (`--check`). |
| `tools/ds-fixtures` | The test squads. |

## Where to start contributing

Read `context/STATE.md` (where we are) and `context/plans/L0-CLOSE.md` (what is left). Every change to the football bumps `SIM_VERSION`, regenerates the golden and comes with a twin test (RULES C3); every number comes with its source (`balance/SOURCES.md`).
