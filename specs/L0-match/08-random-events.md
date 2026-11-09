---
title: "L0.08 — Random Events (in-match)"
level: 0
cell: "L0-match"
module_id: "L0.08"
version: "1.2"
status: "ratified"
last_updated: "2026-10-25"
supersedes: null
depends_on: ["00-foundations", "L0.01", "L0.03", "L0.04", "L0.05", "L0.10"]
referenced_by: ["L0.06", "L2-psyche"]
conforms_to: "00-foundations (v2.0)"
---

<!--
  MAINTAINER NOTE: ADR-003 is accepted (option B). This cell is now `ratified`.

  "random" here means "unpredictable to the player", not "non-deterministic
  to the system". Every event is sampled from a dedicated stream. Same seed +
  same state = same event. If someone says "but it's random, it can't be
  deterministic", they've misunderstood the design. Point them to this comment.

  Also: the probability function reads OBSERVED state (fatigue, condition,
  morale, confidence) and, when a latent is needed (injury_proneness), it is
  accessed via GroundTruth::resolve_injury_risk (API in L0.10), NOT by reading
  a latent field.
-->

> **Note (2026-11-08) — the paradigm has changed; this cell documents the prototype.** ADR-035 (2026-11-07) replaces the doctrine this cell was written for: outcomes emerge from the local transitions of the actors, not from a stream draw. Every "probability function" and every `rng.stream(...)` in this file is prototype residual, not the target. Weather and pitch wear (`pitch.condition`, `weather`) are out of the new engine (T5, 2026-11-03; `WORLD-MODEL` §5 note 2026-11-05). The catalogue, the stream isolation, and the "unpredictable to the player" framing stay as prototype records; the target form is written in `04-match-engine.md` and in ADR-035.

# L0.08 — Random Events (in-match)

## 0. Purpose

The catalogue of **discrete, in-match events** that are not part of the normal tick-by-tick flow: injuries, cards, brawls, weather interruptions, crowd incidents, individual mistakes, moments of genius.

**"Random" here means unpredictable to the player, not non-deterministic to the system.** Every event is sampled from a dedicated RNG stream, with a probability function of the current state. Same seed + same state → same event.

**Status: `ratified`.** ADR-003 (Plane A enforcement, option B) is accepted. The engine calls `resolve_*` for the underlying checks, and the API is defined in [`10-ground-truth.md`](10-ground-truth.md). The implementation can proceed.

## 1. Context and dependencies

- Depends on `L0.01` (RNG), `L0.03` (entities), `L0.04` (engine), `L0.05` (events), **`L0.10` (`GroundTruth` API)**
- Triggered by the engine at each tick, with a probability function
- Produces `MatchEvent`s of type `RandomEvent`
- Reads `ContextModifiers` from `L0.09`
- Reads latents **only** via `GroundTruth::resolve_*` (ADR-003, option B). The API is in [`10-ground-truth.md`](10-ground-truth.md) §2.1.

## 2. Specification

### 2.1 Event catalogue

| Family | Examples | Triggered by | Effects |
|---|---|---|---|
| **Physical** | Muscle injury, cramp, concussion, hard knock | `fatigue`, `condition`, `pitch.condition`, `injury_risk_modifier` (from `resolve_injury_risk`) | Player leaves pitch or continues with penalty |
| **Technical** | Glaring miss, brilliant solo goal, keeper howler, own goal | execution outcome from `resolve_shot` / `resolve_pass`, `pressure_load`, `confidence` | Goal or missed chance; narrative |
| **Psychological** | Loss of concentration, panic after goal conceded, on-field argument | `confidence`, `morale`, `pressure_load`, `acute_arousal` | Temporary penalty to decisions; possible card |
| **Relational** | Brawl between teammates, argument with manager, dissent | `morale`, `squad.chemistry`, `temperament_modifier` (from `resolve_duel` with `DuelType::Brawl`) | Card, substitution, narrative |
| **Environmental** | Pitch invasion, flare, referee controversy, weather change | `crowd.hostility`, `weather`, `referee.strictness` | Match pause, morale shift, card |
| **Set-piece** | Penalty, direct free kick, corner routine | `referee.strictness`, `foul` events | Goal opportunity |

**Rule:** every "triggered by" that references a latent does so through a `resolve_*` outcome, never through a direct field access. The `injury_risk_modifier` is an `InjuryOutcome` field (see `L0.10` §2.2); the `temperament_modifier` is a `DuelOutcome` field. Neither is a raw latent.

### 2.2 Event definition

> **Note (2026-11-08) — the TOML probability function is prototype.** ADR-035 replaces it with local transitions; do not extend this schema.


Each event is a `RandomEventDef` in `balance/random_events.toml` *(planned — the `balance/` directory does not exist yet)*:

```toml
[[event]]
id = "muscle_injury_mild"
family = "physical"
description = "Mild muscle strain, player can continue with reduced condition"

trigger_conditions = [
    "player.fatigue > 0.6",
    "player.condition < 0.8",
    "pitch.condition < 0.7",
]

# The probability function is a deterministic expression of OBSERVED state.
# Latents (e.g. injury_proneness) are NOT read here; they are folded into
# the outcome by GroundTruth::resolve_injury_risk (API in L0.10), which
# returns a modifier in [0.0, +inf). See §2.3.
probability_function = """
base = 0.0001
* (1.0 + player.fatigue * 2.0)
* (1.0 + (1.0 - player.condition) * 1.5)
* (1.0 + (1.0 - pitch.condition) * 0.5)
* injury_risk_modifier   # from resolve_injury_risk(...)
"""

effects = [
    "player.condition -= 0.15",
    "player.fatigue += 0.10",
    "emit_event:Injury { severity: Mild }",
]

narrative_template = "narrative.injury.mild"
explanation_template = "explanation.injury.mild"
```

**Rule:** the `probability_function` is a deterministic expression of the current state. It is not a constant.

**Rule:** latents (e.g. `injury_proneness`) are never referenced directly in the TOML. The engine calls `GroundTruth::resolve_injury_risk(ctx)` once per player per tick, which returns an `InjuryOutcome` (see `L0.10` §2.2). The `injury_risk_modifier` field of that outcome is the only latent-derived input to the probability function.

### 2.3 Sampling

> **Note (2026-11-08) — the `rng.stream(...)` draw is the prototype form.** Under ADR-035 no stream is consumed to decide an outcome (a key derivation is not a die; a stream draw is). `00-foundations` §4.


At each tick, for each player, for each eligible event:

```
outcome = hidden.resolve_<family>(player_context)   // e.g. resolve_injury_risk
risk_modifier = outcome.<modifier_field>            // e.g. injury_risk_modifier
p = evaluate(probability_function, state, risk_modifier)
if rng.stream("match:{id}:random-events").gen::<f32>() < p {
    apply(effects)
    emit_event(...)
}
```

The stream `match:{id}:random-events` is **isolated** from `match:{id}:micro-decisions`. Adding a new random event never shifts the micro-decisions.

The `resolve_*` calls are made once per player per tick, not once per event, to avoid redundant computation. The `outcome` is cached for the tick.

### 2.4 Cause trace

**No `CauseTrace` at L0 (ADR-024, 2026-10-31).** A random event carries its cause in `event_type` + `fields`: the event type says what happened, `fields` says the observed result. A `CauseTrace` that could only ever be empty would be ceremony, not transparency. See `L0.05` §2.3 for the rule.

### 2.5 Distribution target

> **Note (2026-11-08) — the distribution targets are prototype targets.** They measure the prototype calibration; the new engine calibration belongs to L3.


The catalogue is calibrated so that:

- **Mild injuries**: ~1.5 per match (across both teams)
- **Severe injuries**: ~0.05 per match (once every 20 matches)
- **Red cards**: ~0.1 per match
- **Yellow cards**: ~4 per match
- **Penalties**: ~0.3 per match
- **On-field brawls**: ~0.02 per match (rare)

These targets are in `balance/random_events.toml` *(planned)* and tested by `L3-quality` *(planned)*. Until `L3-quality` exists, they are tested by the CI job that runs the `L0.04` golden tests with distribution assertions.

### 2.6 What is **not** here

- **Pre-match events** (player had a fight with his wife) → `L2-psyche/05-life-events.md` *(planned)*
- **Training events** (injury in training) → `L2-psyche/05-life-events.md` *(planned)* (training events are psyche-adjacent at L0; a dedicated training module comes with L2)
- **Season events** (transfer request) → `L2-market` *(planned)*
- **Crowd events outside the stadium** → `L2-crowd` *(planned)*

This cell is **only** what happens *during* the match, *in* the match context.

## 3. Data model

`RandomEventDef`, `RandomEventFamily`. Event kinds are the `RandomEvent` variant of `MatchEvent` (see `L0.05`). No `CauseTrace`, no `Factor` (ADR-024).

The `resolve_*` contexts and outcomes (`InjuryContext`, `InjuryOutcome`, etc.) are defined in `L0.10` §2.2, not here.

## 4. Edge cases

- **Same player, two events in one tick**: second event is deferred to the next tick
- **Injury on a player already injured**: prevented (condition check)
- **Brawl between teammates**: both get cards, manager may substitute
- **Pitch invasion**: match paused, morale shift, resumed
- **Penalty in stoppage time**: allowed
- **Event probability > 1.0**: clamped to 1.0, logged (calibration bug)
- **Event probability < 0.0**: error at load
- **`resolve_*` returns an error**: the event is skipped for this tick, logged; the match continues
- **`resolve_*` called more than once per player per tick**: forbidden. The outcome must be cached and reused within the tick (see §2.3)

## 5. Definition of Done

- [x] ADR-003 accepted (option B)
- [ ] Event catalogue in `balance/random_events.toml` with ≥ 30 events
- [ ] Each event has trigger, probability, effects, narrative, explanation
- [ ] Sampling uses dedicated stream
- [ ] Latents accessed only via `resolve_*` (API in `L0.10`), never read directly
- [ ] `resolve_*` outcome cached per player per tick (not recomputed per event)
- [ ] Every event's cause is observable (`event_type` + `fields`; no `CauseTrace`, ADR-024)
- [ ] Distribution targets tested (10,000 matches)
- [ ] No event decides the match on its own (statistical check)
- [ ] Events are deterministic (same seed → same event)

## 6. Risks and Mitigations

| Risk | Severity | Mitigation |
|---|---|---|
| Events feel "cheap" | High | Cause trace mandatory; distribution calibrated |
| Event decides match | High | Statistical check: max event impact on win probability < 0.1 |
| Probability overflow | Medium | Clamp + log |
| Catalogue too small | Medium | ≥ 30 events at v1.0 |
| Stream shared with micro-decisions | Critical | Dedicated stream, tested |
| Latent leak via `probability_function` | Critical | TOML cannot reference latents; only `resolve_*` modifiers (API in `L0.10`) |
| `resolve_*` call cost dominates the tick | Medium | Cache the result per player per tick; call once, not per event (see §2.3) |
| The TOML references a modifier that does not exist | Medium | Schema validation at load; fail loud, not silent |

## 7. References

- [`../00-foundations.md`](../00-foundations.md) §4 — determinism
- [`00-overview.md`](00-overview.md) — L0 cell map
- [`01-rng.md`](01-rng.md) — stream isolation
- [`03-entities-minimal.md`](03-entities-minimal.md) — entities
- [`04-match-engine.md`](04-match-engine.md) — caller
- [`05-match-events.md`](05-match-events.md) — event type
- [`09-context-modifiers.md`](09-context-modifiers.md) — external state
- [`10-ground-truth.md`](10-ground-truth.md) — `GroundTruth` API (contexts, outcomes, modifiers)
- [`../../docs/adr/003-plane-a-enforcement.md`](../../docs/adr/003-plane-a-enforcement.md) — no latent access (accepted)
- `../../archive/Prompt_Master_v7.md` §Psiche — original event families *(planned — the archive/ directory does not exist yet)*
- [`../../docs/GLOSSARY.md`](../../docs/GLOSSARY.md) — terms used in this cell

## 8. Dated notes

- **2026-11-09 — `resolve_injury_risk` and `resolve_set_piece` are removed.** Both functions were dormant (no caller outside `pcfm-hidden`) and drew from `ChaCha20Rng` streams; ADR-035 forbids a die in simulation paths and D3 forbids structure without a consumer. They are removed together with their streams (`injury`, `set_piece`), their `InjuryContext`/`InjuryOutcome`/`SetPieceContext`/`SetPieceOutcome` types, and their `ObservationEvent` variants. The §2 text above that references `GroundTruth::resolve_injury_risk` and the `injury_risk_modifier` from its outcome is stale: the injury phenomenon returns with the injury model (OI-60), and the set-piece phenomenon returns with the set-piece model (OI-48), each with an ADR and a formula in `balance/`. `EventType::Injury` and `EventType::SetPiece` in `pcfm-core` are untouched (OI-31 tracks the spec/code gap). The latent `injury_proneness` is declared `#[allow(dead_code)]` in `pcfm-hidden/src/latents.rs` until the injury model lands. The text above stays as history (D7).

