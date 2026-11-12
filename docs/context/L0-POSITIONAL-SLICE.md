# L0 positional slice — design note

## For the new agent (read before §0)

This note assumes you have run the startup ritual in
`docs/context/NEW-SESSION-PROMPT.md` and read the bookmark. It does not
repeat the operating rules. It tells you what this note is, what it is
not, and where to find what you need.

### The note and the spec

`specs/L0-match/` is the **contract**, ratified. This note is a **plan to
implement it**. If they disagree, the spec wins: the note is wrong and
must be corrected. `04-match-engine.md` §2.2 (the tick loop) and
`05-match-events.md` §2.1-§2.6 (the event, the two-phase pattern) are the
two sections the note leans on most.

### Before writing code, read (in this order)

1. `docs/context/SESSION-STATE.md` — the bookmark: where we are
2. this note, §0 (vision), §2 (the slices), §3 (the decisions), §6-ter (the
   event map), §7 (what is next)
3. `specs/L0-match/04-match-engine.md` §2.2 and `10-ground-truth.md` §2.2
4. `crates/pcfm-match/src/spatial.rs` (slice A) and `perception.rs`
   (slice B part 1): what exists
5. `crates/pcfm-match/src/state.rs`, `pick_players` at `:228`: what B2
   replaces

### Where things are

| What | Where |
|---|---|
| `SIM_VERSION` | `crates/pcfm-match/src/save.rs:74` |
| Golden hash | `crates/pcfm-match/tests/golden_l0.rs:21` (`EXPECTED`) |
| `EventType` enum | `crates/pcfm-stats/src/event.rs` |
| Exhaustive `EventType` map (the board) | `crates/pcfm-demo/src/viewmodel.rs:178` — a new variant stops the build until mapped |
| `MatchEvent.fields` type | `crates/pcfm-stats/src/event.rs:117` (`serde_json::Value`) |
| `resolve_*` API | `crates/pcfm-hidden/src/lib.rs` |
| Balance parameters | `balance/` (JSON, embedded with `include_str!`) |
| The board | `cargo run -p pcfm-demo -- --seed 42 --board` |
| The demo, JSON | `cargo run -p pcfm-demo -- --seed 42 --json` |

### The DoD of a slice

A slice is done when **all** hold:
1. `cargo build --release`, `cargo clippy --workspace --all-targets -- -D
   warnings`, `cargo test --workspace`, `cargo fmt --check`, `cargo run
   --release --bin pcfm-test-runner -- --check --seed 42`: all green (C2).
2. `scripts/gates.sh closing`: green.
3. If the behaviour changed: `SIM_VERSION` bumped, golden hash regenerated,
   `CHANGELOG-SIM` entry.
4. Commit pushed, `git status -sb` shows `## main...origin/main`.
5. The board still renders: `cargo run -p pcfm-demo -- --seed 42 --board`.

"Compiles" is not a DoD. "Tests pass" is not a DoD. The five above are.

### The "observed field" rule

An event carries **observable** data: who, where, what happened, what
condition. **Never a latent.** If a decision needs a latent (finishing,
passing, tackling), it goes through `resolve_*`; the outcome is observable,
the latent stays in `pcfm-hidden` (ADR-003, `00-foundations` §5). Adding a
latent name to `fields` is a D2 violation, and the anti-leak test
(`crates/pcfm-match/tests/anti_leak.rs`) will catch it in the save.

### A ten-line glossary

- **Slice**: a unit of work with an observable result; one session at most.
- **Board**: the textual renderer in `pcfm-demo`; it reads `MatchResult`,
  it is not the product (§0).
- **Golden hash**: the concatenated state hashes of 10 matches, seed
  42..51, compared with `EXPECTED`. Regenerated on every `SIM_VERSION`
  bump.
- **`resolve_*`**: the only way the engine reads Plane A. It returns an
  outcome, never a latent.
- **`fields`**: the variant-dependent payload of a `MatchEvent`,
  `serde_json::Value`, observable only.
- **`carrier`**: `SpatialState.carrier`, the agent with the ball, if any.
- **Phenomenological**: effects are modelled, not simulated (§0).
- **`pick_players`**: the current random picker (`state.rs:228`), the thing
  slice B2 replaces.
- **`pick_from_space`**: its replacement: the agent nearest the ball acts.
- **`SIM_VERSION`**: the simulation behaviour version; bumped when the
  output changes for the same seed.

## 0. Vision (read this first)

**What this project is building:** a realistic simulation of a football
match. Not a video game with a renderer: a **match engine** that
understands space, interactions between agents, and the conditions that
make an action succeed or fail. The renderer is downstream; the engine is
the product.

**The paradigm: phenomenological simulation.** The engine does not
simulate physics (trajectories, contact forces, biomechanics). It models
the **observable effects** of conditions and decisions, tick by tick. The
effects are modelled, not simulated: a slip is an event with a cause, not
a computed friction coefficient.

> **Note (2026-11-12) — the paradigm restated: the actors decide, the ball has declared physics.** The paragraph above is the 2026-11-01 wording, and one word in it is no longer true: "trajectories". The maintainer's statement of the vision (2026-11-12): a deterministic engine that simulates a football match in depth, with everything that happens on the pitch; **phenomenological**, with **the ball with physics** and **the players with the power of choice** — thinking, evaluating, deciding actors, not puppets, the goalkeepers included. Accepted sources already say each half: ADR-043 (the actor decides) and ADR-045 (the goalkeeper is an actor); ADR-044 §3-§5 and ADR-048 (the ball answers a chain of declared physical properties — gravity, friction, elasticity — applied by a formula, in integers). What stays out is the physics **of the bodies**: contact forces and biomechanics are not simulated; a gesture is read, chosen, executed with a band (ADR-039). Wherever this note and `NEW-SESSION-PROMPT.md` say "not physics", read *no physics of the bodies*.

### The scope: eight categories, not one

The engine models eight kinds of phenomena. **Weather is an input to two
of them, not the theme.**

| # | Category | What is modelled | Inputs (examples) |
|---|---|---|---|
| 1 | **Perception** | what an agent sees, with what error and delay | position, fatigue, concentration, crowd, *weather* |
| 2 | **Decision** | the choice among *perceived* options | perception, psychology, tactical clarity, situation |
| 3 | **Execution** | pass, shot, tackle, with error | skill (latent, via `resolve_*`), pressure, fatigue, *surface* |
| 4 | **Interaction** | duels, pressing, marking as contests | position, timing, intensity |
| 5 | **Collective shape** | lines compressing, transitions | individual decisions + tactical intents (L2) |
| 6 | **State over time** | fatigue, condition, concentration | minutes, intensity, score |
| 7 | **Psychology** | confidence, pressure, morale | events, context (L2) |
| 8 | **Referee** | fouls, cards, advantage | perception, strictness, consistency |

**Weather is #1 (perception: rain makes the estimate noisy) and #3
(execution: wet grass raises the control error).** It is an input, not a
category. A slip is an execution event that depends on a surface input; a
noisy pass under rain is a perception event. Neither is "the engine".

**The heart is #1–#4:** perception, decision, execution, interaction. A
match is mostly these four, tick after tick. #5–#8 are the context that
shapes them.

### Why phenomenological

It is deterministic (integer coordinates, named RNG streams, no
platform-dependent floating point), calibratable (each model has
parameters in `balance/`), and it produces events a fan recognises. A
physical engine would require fixed-point physics, months of calibration,
and data that does not exist publicly. That is a different project, not a
later step of this one.

### The success criterion

The board shows events a fan recognises, and **every event carries the
fields to understand why it happened**: who, where, under what condition,
with what outcome. "He slipped" is not enough; "he slipped at the 67th
minute on the wet grass and lost the ball" is the target. Same for "he
passed", "he was pressed", "he anticipated".

### The existing work

The L0 specification (`specs/L0-match/`) is ratified and describes an
agent-first, positional engine (`04-match-engine.md` §2.2). The code is a
probability-per-tick prototype that does not implement it: `pick_players`
(`state.rs:228`) picks an attacker and a defender **at random from the
whole squad, with the same roll**. `MatchEvent` has 31 variants, the
engine emits 8. The gap is declared; this note is the plan to close it.

- **Slice A is done** (`39c444b`): `spatial.rs`, `AgentState`,
  `SpatialState`, 4-4-2 grid, 6 tests.
- **Slice B part 1 is done** (`c231c98`): `perception.rs` (4 tests),
  `SpatialState.carrier`.

Neither is used by the engine yet. See §7.

> **Note (2026-11-12) — four statements in §0 that are prototype.** (1) "named RNG streams" (*Why phenomenological*): no die is drawn in a simulation path (ADR-035); determinism is by construction, not by seeded stream. (2) Weather as an input to categories 1 and 3, and "surface" in category 3: weather and wear are out (T5); the stable properties of the surface enter as a term (ADR-044 §4). (3) Referee "strictness, consistency" (category 8): the referee applies the rules (OI-55). (4) *The existing work* describes the code of 2026-11-01 (`pick_players` at random, slices A and B unused): none of it is true today. The actor is elected from space and decides through `Actor::decide`; the state is `SESSION-STATE.md`. The text above stays as history (D7).

## 1. The gap this note closes

engine. The code is a prototype that does not implement it. This is declared,
not a discovery:

- `04-match-engine.md` §2.2: `for player in all_on_pitch() { perception →
  decision → execution }`. The code does: one global roll, then
  `pick_players` picks an attacker and a defender **at random from the whole
  squad, with the same roll** (`state.rs:228-247`).
- `05-match-events.md` §2.1: `MatchEvent` has `position_snapshot` and
  `attempt_event_id`. The code has neither.
- `05-match-events.md` §2.2: four "attempt" event types
  (`PassAttempted`, `ShotAttempted`, `DuelStarted`, `FoulCommitted`). The code
  emits zero.
- `10-ground-truth.md` §2.2: `DuelContext` has `pitch_position: (f32, f32)`.
  The code's `DuelContext` has no position.
- `09-context-modifiers.md`: `ContextModifiers` exists in the spec. The code
  has no such type.
- `02-event-bus.md`: a deterministic bus. The code has one, the engine does
  not use it.

`04-match-engine.md` carries a maintainer note: *"This module describes the
current prototype. Revision expected after ADR-019."* This note is that
revision, in its first slice.

## 2. What the slices are, and what they are not

The work is split into slices, each one observable. The slices build the
**base** of the phenomenological engine; they are not the engine.

**Slice A — spatial state. DONE (`39c444b`).**
`crates/pcfm-match/src/spatial.rs`: `AgentState` (integer position,
velocity, heading), `SpatialState` (`BTreeMap<PlayerId, AgentState>`, ball,
carrier), `from_squads` with the 4-4-2 grid. 6 tests. Unused by the
engine: `pick_players` still runs.

**Slice B part 1 — perception and carrier. DONE (`c231c98`).**
`crates/pcfm-match/src/perception.rs`: `distance_squared`,
`nearest_teammates`, `nearest_opponents`, `nearest_to_point`. 4 tests.
`SpatialState.carrier: Option<PlayerId>`. Unused by the engine.

**Slice B part 2 — who acts comes from space, and conquest is an event. NEXT.**
Two changes, one bump:
- `pick_players` (random, same roll for attacker and defender) becomes
  `pick_from_space`: the attacker is the agent of the possessing team
  nearest the ball; the defender is the nearest opponent to the attacker.
- The conquest becomes an event: when a duel is won by the defender, emit
  `Tackle` with `fields: { "from": <previous carrier slot>, "to":
  <defender slot> }`. When a pass is incomplete, `Pass` already carries
  `complete: false`; the engine also clears the carrier.
- `SpatialState` is built by `MatchState` (it does not exist outside the
  match engine).
- `SIM_VERSION` 7 → 8. The golden hash is regenerated. The save schema
  does not change.

**Why the conquest event and the space together:** a change in *who acts*
that nobody can see is not a slice, it is a refactor. The event makes the
change observable: the board shows who lost the ball and who took it. This
is the success criterion of §0 applied to the first slice.

**Slice C — the first phenomenological events.** After B part 2, the engine
can decide from space and record a conquest. Slice C adds the first
**modelled conditions**:
- a `Slip` event, driven by `pitch_wear` and `weather` (the parameters

> **Note (2026-11-08) — the Slice C inputs are prototype.** T5 (2026-11-03) excludes weather and pitch wear from the new engine (`WORLD-MODEL` §5 note 2026-11-05), so `Slip` and `ControlError` cannot be driven by them. The event types stay in the vocabulary; the causes are re-derived when Slice C opens. This is the same correction recorded in OI-56.
  live in `balance/`, declared arbitrary until calibration);
- a `ControlError` event, driven by the same conditions plus pressure;
- a `Pressure` event, already in the vocabulary, now emitted.

Each event carries the fields to understand why (position, condition,
intensity). The parameters are arbitrary and declared: the model exists,
the calibration is a later step with data.

**What none of the slices is:**
- Not a physics engine. No trajectories, no contact forces, no
  biomechanics. The effects are modelled, not simulated (§0).
- Not a renderer. The board is a consumer; it is not the product.
- Not L1 or L2. No season, no ratings, no market. Those consume the events;
  they do not produce them.
- Not `PositionSnapshot` in events. That is a save-breaking change with no
  consumer yet. It returns when a view needs it (slice D, not planned).
## 3. Decisions


### 3.1 `SpatialState` lives in `pcfm-match`, not `ds-model`

The position is **match state**, not player identity. `Player` (in
`ds-model`) is shared with L1, where a player has a contract and a squad,
not a position on the pitch. If `position` goes into `Player`, every level
inherits it and overwrites it.

    // pcfm-match/src/spatial.rs (slice A, merged 39c444b)
    pub struct AgentState {
        pub position: (i32, i32),   // centimetres, integer (ADR-019 D1)
        pub velocity: (i16, i16),   // cm per tick, signed
        pub heading: u16,           // 0..=359, degrees, 0 = north
    }

    pub struct SpatialState {
        agents: BTreeMap<PlayerId, AgentState>,
        ball: (i32, i32),
        carrier: Option<PlayerId>,  // slice B: who has the ball, if any
    }

**Why integer coordinates:** ADR-019 D1 (determinism). Floating-point
positions would make the tick loop depend on platform FP behaviour. Integer
centimetres give a 10500 × 6800 grid, which is enough for a prototype and
exact across platforms.

**Why `BTreeMap`:** D2 bans `HashMap` in simulation paths.

**Why `carrier`:** the first draft of §3.4 said "the agent with the ball",
but the ball is a point, not a possession. Without a carrier the decision is
degenerate (the agent nearest the centre always acts). `carrier` is the
minimum: `None` (ball loose), `Some(id)` (the ball follows `id`). A
completed pass or a tackle changes it.

### 3.2 Initial position from the formation

`Squad.formation` exists (`4-4-2` in the fixture) and the engine never reads
it. The first slice places the 11 starters on a 4-4-2 grid:

    GK    (5%, 50%)
    LB    (25%, 15%)   CB (25%, 38%)   CB (25%, 62%)   RB (25%, 85%)
    LM    (50%, 15%)   CM (50%, 38%)   CM (50%, 62%)   RM (50%, 85%)
    ST    (75%, 38%)   ST (75%, 62%)

Coordinates as percentages of pitch length/width, converted to integer
centimetres at kickoff. The away team mirrors on the x axis. Hardcoded for
`4-4-2`; any other formation falls back to a 4-4-2 grid and logs a warning
(the formation parser is a later slice, D3: no content, no structure).

### 3.3 Perception: squared distance, no map

An agent perceives:
- the ball position (always, it is Plane B: the ball is visible);
- the distance to the 3 nearest opponents (squared, integer);
- the distance to the 3 nearest teammates (squared, integer).

It does **not** perceive the full map. This is the Plane B rule: an agent
sees what it sees, not where everyone is. The perception is a local
computation from `SpatialState`, and the result is not stored in the event
(a later slice adds `PositionSnapshot`).

Slice B implements it in `perception.rs`: `distance_squared`,
`nearest_teammates`, `nearest_opponents`, `nearest_to_point`. Four tests.

### 3.4 Who acts: the agent nearest the ball (conservative method)

**The first draft of this section said: "the agent with the ball passes to
the nearest free teammate, or moves toward the goal". That would have
removed shots, goals, duels and fouls: the demo would have shown 0-0 and
no shot events. A regression, not a slice.**

**The slice-B method is conservative.** The action split stays exactly as it
is (`action_probability` for *when*, `action_kind_roll` for *what*: pass,
shot, duel, foul). What changes is **who acts**:

    before:  pick_players = random attacker + random defender (same roll)
    after:   attacker = the agent nearest the ball
             defender = the nearest opponent to the attacker

`SpatialState.carrier` starts at `None` (ball at the centre); the first
action sets it to the attacker. A completed pass keeps it (simplification:
the receiver is not modelled); a failed pass sets it to `None`; a duel won
by the defender sets it to the defender; a foul sets it to `None`.

**Why conservative:** it proves the thesis ("the decision comes from space,
not from a global roll") with zero regression on the demo, the event types,
or `MatchResult`. The conquest event (a duel won by the defender emits
`Tackle`) is added in the same slice, because a change in *who acts* that
nobody can see is not observable. The "pass to the nearest free teammate" rule and the movement are not in this slice. **Nor are they in slice C** (§2, §7: C is `Slip` + `ControlError` + `Pressure`). Movement is a **precondition of the ball slice**, not a slice: a slip is recognised because someone was moving and fell. The design is in `docs/context/MOVEMENT-NOTE.md` (2026-11-03, closes OI-28).
**What stays out:** the goalkeeper, the pressing, the formation-aware
movement, `PositionSnapshot`, attempt/outcome events. Those are §5.

**This is deliberately minimal.** It proves the loop: position → perception →
decision → movement. Everything else is vocabulary.
### 3.5 OI-22 (RNG streams without `match_id`) closes here

The streams today are `"match:actions"`, `"match:action_kind"`,
`"match:player_pick"` (`state.rs:60-64`). The spec (`01-rng.md:66-68`) wants
`match:{id}:...`. The positional slice changes the RNG consumption anyway, so
`SIM_VERSION` bumps regardless. Closing OI-22 now avoids a second bump.

**Action:** `MatchState::new` takes `match_id` and builds
`format!("match:{}:actions", id.slot())`. Same for the other streams.

### 3.6 OI-24 (getters on `GroundTruth`) — deferred, with a recommendation

`world_seed()` and `match_id()` are public getters. ADR-003 says "no public
getters on latents". These are not latents (they are identifiers), but the
contract is strict. Recommendation: **amend ADR-003** to declare them
non-latent, rather than remove them (they are useful for the save
reconstruction test). This is a decision for the maintainer, not for the
slice. **Not blocking.**

### 3.7 `duration_minutes` duplication — not in this slice

`MatchRules.duration_minutes` and `MatchConfig.duration_minutes` both exist.
The engine uses `MatchConfig`. The duplication is real but harmless today.
Recorded here so it is not lost; fixed when a config file
(`balance/match_engine.toml`) exists.

## 4. Files the slice touches

    crates/pcfm-match/src/spatial.rs      NEW — AgentState, SpatialState
    crates/pcfm-match/src/perception.rs   NEW — what an agent sees
    crates/pcfm-match/src/state.rs        SpatialState field, pick from space
    crates/pcfm-match/src/orchestrator.rs  no change (the tick loop stays)
    crates/pcfm-match/src/context.rs       no change
    crates/pcfm-match/src/lib.rs           export the new modules
    crates/pcfm-match/tests/golden_l0.rs   hash regenerated (SIM_VERSION 8)
    CHANGELOG-SIM.md                       SIM_VERSION 7 -> 8

Not touched: `ds-model`, `pcfm-hidden`, `pcfm-stats`, `pcfm-demo`.
The board keeps working: `MatchResult` does not change.

## 5. What is out, and when it comes back

| Piece | Why out | When |
|---|---|---|
| `PositionSnapshot` in events | Breaking change on the save; no consumer yet | Slice 2, when the board or a debug view needs it |
| Attempt/outcome two-phase events | Requires the rules plugin contract (ADR-019 D11) | Slice 3 |
| `ContextModifiers` | Requires L2 producers | With L2-psyche |
| Event bus in the tick loop | The engine emits directly today; the bus is for subscribers | When a second subscriber exists |
| Referee as perceptual agent | ADR-021; independent of space | Separate slice |
| Formation parser | D3: no content, no structure. 4-4-2 hardcoded | When L2-tactics exists |
| `MatchResult` changes | Not needed for the slice | Never, if the slice works |

## 6. Open points, with a hypothesis

1. **Does the ball have a position?** Yes: `SpatialState.ball`. It moves when
   a pass completes (from passer to receiver), stays with the carrier
   otherwise. Hypothesis: a simple interpolation over 3 ticks, integer.
   If it is too much for slice 1, the ball can teleport; the slice still
   proves the decision loop.
2. **What is "significant action" after the slice?** Today it is
   `action_probability` (0.0005 per tick). After the slice, the same roll
   selects **which agent** acts (the one nearest the ball), not **whether**
   an action happens. Hypothesis: keep the probability for now; the space
   decides *who*, the probability decides *when*.

> **Note (2026-11-08) — `action_probability` is prototype.** ADR-035 (2026-11-07) forbids a die in simulation paths: "when" an action happens is not drawn. `action_roll` is tracked as Plan J (OI-78 point 4), default removal. The sentence above stays as history of the design at the time.
3. **How is possession represented?** Today `Possession::Home | Away`. After
   the slice, the ball position plus the nearest agent gives the carrier.
   Hypothesis: keep `Possession` for the event stream (compatibility), add
   `carrier: Option<PlayerId>` in `SpatialState`.
4. **Does the slice change the score distribution?** Almost certainly yes:
   the decision is now spatial, and the number of passes/shots will differ.
   This is why `SIM_VERSION` bumps and the golden hash is regenerated. The
   new distribution is not calibrated; it is the first spatial one.

## 6-bis. State at 2026-11-01, end of session

**Slice A is done** (commit `39c444b`). What exists now:

- `crates/pcfm-match/src/spatial.rs` (204 lines): `AgentState`
  (`position: (i32, i32)` cm, `velocity: (i16, i16)`, `heading: u16`),
  `SpatialState` (`BTreeMap<PlayerId, AgentState>` + `ball: (i32, i32)`),
  `from_squads(pitch, home, away)` with the 4-4-2 grid, away mirrored, ball
  at the centre.
- 6 tests, all green: 22 agents, determinism (same input → same output),
  ball at the centre, goalkeepers at the extremes, every position inside
  the pitch, away mirrored on x.
- Exported from `pcfm-match`: `pub use spatial::{AgentState, SpatialState};`
- 129 tests total (was 123). Badge and the four docs updated. A4 green.

**What has NOT changed** (important for the next agent):

- `state.rs` still has `pick_players` (random pick, no space).
- `SpatialState` exists but **nobody uses it yet**. The engine does not
  build it, does not read it. It is dead code until slice B.
- `SIM_VERSION` is still 7. The golden hash is unchanged.
- `MatchEvent` has no `position_snapshot`. `MatchResult` is unchanged.
- The board still works: `cargo run -p pcfm-demo -- --seed 42 --board`.

**The next concrete step (slice B part 2):** in `state.rs`,
`MatchState` builds (or receives) the `SpatialState`; `pick_players` is
replaced by `pick_from_space`: the attacker is the agent of the
possessing team nearest the ball, the defender the nearest opponent to
the attacker. The conquest becomes an event: a duel won by the defender
emits `Tackle` with `fields: { "from": ..., "to": ... }`; an incomplete
pass clears the carrier. `SIM_VERSION` 7 → 8, golden hash regenerated.
See §2 and §7.
**Reading order for the next agent, before writing any code:**

1. `docs/context/SESSION-STATE.md` (the bookmark: where we are)
2. this note, §1 (the gap), §3 (the decisions), §6-bis (what exists)
3. `crates/pcfm-match/src/spatial.rs` (what slice A produced)
4. `crates/pcfm-match/src/state.rs`, `pick_players` at `:228` (what slice B
   replaces)
5. `specs/L0-match/04-match-engine.md` §2.2 (the target tick loop)
## 6-ter. The phenomenological event vocabulary (the map)

The slices above produce **structure** (space, perception, conquest). This
section is the **map of the events** the engine should eventually produce.
It is a map, not a commitment: each event is implemented when its model
exists, not before (D3).

### The principle

An event is worth adding when:
1. a fan recognises it;
2. it has a **cause** the engine can model (a condition, a decision, an
   interaction), not a random draw;
3. it carries the **fields** to understand why (position, condition,
   intensity, outcome);
4. it has a **consumer** today (the board, the narrative) or a declared
   one in the next slice.

An event that fails any of the four is an idea, not a design (D3).

### The candidates, by category (see §0)

The category numbers refer to the eight categories in §0. **The heart is
#1–#4**; the rest is context. Weather appears in #1 and #3 as an input,
never as a category.

**#1 Perception** — what an agent sees, with error. *(No event yet: a
perception is not an observation. Its effects are events #3 and #4.)*

**#2 Decision** — the choice among perceived options.

| Event | Cause (modelled) | Fields | Consumer |
|---|---|---|---|
| `Pass` (already) | decision + perception | complete, from, to | board, narrative |

**#3 Execution** — pass, shot, tackle, with error. Weather and surface are
inputs here.

| Event | Cause (modelled) | Fields | Consumer |
|---|---|---|---|
| `Shot` (already) | decision + skill (via `resolve_*`) | goal, on_target, position | board, narrative |
| `ControlError` | surface × pressure × skill | position, pressure, lost_ball | board, narrative |
| `Slip` | surface × velocity × balance | position, condition, lost_ball | board, narrative |

**#4 Interaction** — duels, pressing, marking as contests. **This is the
largest category and the first one slice B2 implements.**

| Event | Cause (modelled) | Fields | Consumer |
|---|---|---|---|
| `Tackle` | defender wins a duel | from, to, position | board, narrative |
| `Interception` | defender anticipates a pass | from, to, position | board, narrative |
| `Duel` (already) | two agents contest the ball | winner_slot | board, narrative |
| `Foul` (already) | illegal contest | position | board, narrative |
| `Pressure` (in enum) | opponent within R, intent | position, intensity | board, narrative |
| `Push` | contact, intensity, body part | position, intensity, outcome | **deferred**: no public dataset for "which part of the body"; inventing it violates `00-foundations` §5 |

**#5 Collective shape** — lines compressing, transitions. *(No event yet:
a shape is a relation, not an observation. L2 territory.)*

**#6 State over time** — fatigue, condition, concentration. *(Inputs to
#1–#4, not events of their own at L0. L2 territory.)*

**#7 Psychology** — confidence, pressure, morale. *(L2 producers, via
`ContextModifiers`. Not L0.)*

**#8 Referee** — fouls, cards, advantage. *(`Foul` and `Card` exist; the
referee as a perceptual agent is ADR-021, a separate slice.)*

### What slice B2 implements

Only **#4**: `Tackle` when a duel is won by the defender. `Interception`
and `Pressure` follow in slice C, with #3 (`Slip`, `ControlError`). The
order is deliberate: interaction first (the heart), execution after
(weather as an input, not the theme).


### The consumer gap (2026-11-02)

An event with the right fields is not enough: someone must read them.
`crates/pcfm-match/src/narrative.rs` ignores `fields` for every event:
`Tackle` does not say who lost the ball (`from`/`to`), `Shot` does not say
whether it was a goal (`goal`), `Duel` does not say against whom
(`winner_slot`). The map above says "fields to understand why"; the
narrative does not use them.

This is **step 19** in the bookmark. It is not part of slice C, and it
does not need the new engine: the fields exist today. It is the first
consumer fix, and the pattern to follow: every new field added in slice C
must have a consumer in the same slice, or it is an idea, not a design
(the four criteria above, applied to the fields).
### The parameters

Every model has parameters (how much a surface raises the control error,
how much pressure raises it further). They live in `balance/`, they are
**arbitrary and declared** until calibration. The `CHANGELOG-SIM` entry
records them as "prototype, not calibrated". Calibration is a later step
with data; it does not block the model.

### What is not in the map

- Physics (trajectories, forces, biomechanics): §0.
- "Which part of the body": no data, deferred.
- Events with no cause the engine can model: a random draw is not a cause.
- Events with no consumer: an idea, not a design.
- L2 phenomena (#5–#7): they shape the events, they are not L0 events.
## 7. Where we are, and the next sessions

**Stato reale al 2026-11-05 (nota datata; il testo sotto resta il record del 2026-11-01).**
L0 ha 172 test verdi, 9 crate `pcfm-*`, `SIM_VERSION` 18, C2 e gates verdi. Le foglie del
Plan chiuse dopo questa nota: F.1 (primitivi spaziali), F.2 (bench movimento), F.3-a/b1/b2/b3
(fact layer, BallState 3D, Pass/Goal geometrico, portiere e gol da attraversamento), F.4-a/b/c1/c2/d
(movimento ogni tick, cono, pendenti + volo, decisione situata, intercettazione), F.5 (Pressure
come relazione di copertura via `Fact::RelationEnterExit`). Aperte: F.2-bis (`resolve_*` micro-bench),
F.6 (movimento off-ball, l'agente percepisce la palla), OI-52 (rimbalzo pali/traversa), OI-53
(decisione tick record / IDEA-INFRA-018 e IDEA-INFRA-021). **Fatto dichiarato che sorprende:** con
seed 42–51 il motore emette **0 `Shot`** (19 `Pass`, 19 `Pressure`): il golden non esercita lo
schema di `Shot`, la copertura è nei test `shot_forced.rs` (3 test su `resolve_shot` chiamato
diretto). Il filtro partita vera ha guidato F.5: `Pressure` è la relazione di copertura che cambia,
non un evento "accanto all'azione". `PositionChange`/`HeadingChange` non sono `Fact`: sono campi
del tick record (IDEA-INFRA-018), sfondo per 2D/3D. `RelationEnterExit` è l'unico fatto di
movimento con un consumatore-evento (T1-bis emendato il 2026-11-05).


**Done:**
- Slice A (`39c444b`): `spatial.rs`, `AgentState`, `SpatialState`, 4-4-2
  grid, 6 tests.
- Slice B part 1 (`c231c98`): `perception.rs` (4 tests),
  `SpatialState.carrier`.
- 133 tests, C2 green, CI green on `main`, gates green.

**The next session (slice B part 2).** Two changes, one `SIM_VERSION` bump:
1. `MatchState` owns a `SpatialState` (built from `ctx.match_` in
   `MatchState::new`, or in `simulate_match` and passed in — the second is
   simpler, decide in the session).
2. `pick_players` becomes `pick_from_space`: attacker = the agent of the
   possessing team nearest the ball; defender = the nearest opponent to
   the attacker.
3. The conquest is an event: a duel won by the defender emits `Tackle`
   with `fields: { "from": ..., "to": ... }`; an incomplete pass clears
   the carrier (`Pass` already carries `complete: false`).
4. `SIM_VERSION` 7 → 8, golden hash regenerated, `CHANGELOG-SIM`.

Tests: the attacker is the nearest to the ball; the defender is the
nearest opponent to the attacker; a won duel emits `Tackle` with both
slots; determinism (same seed → same hash).

**The session after (slice C — first phenomenological events).** `Slip`
and `ControlError`, driven by `pitch_wear` and `weather` from

> **Note (2026-11-08) — see the note at §6-bis.** `pitch_wear` and `weather` are out of the new engine (T5); the Slice C inputs as written are prototype. Slice C stays in the plan; its inputs are re-derived when it opens.
`balance/` (parameters arbitrary, declared). `Pressure` emitted. See
§6-ter for the map.

**Open, not for the next session:**
- OI-22 (stream names with `match_id`) — closes in slice B part 2, because
  the RNG consumption changes anyway.
- OI-24 (getters on `GroundTruth`) — a maintainer decision (remove or
  amend ADR-003). Not blocking.
- `PositionSnapshot` in events — slice D, not planned; needs a consumer.
- `ContextModifiers` — needs L2 producers.
## 8. How to verify the slice worked

Not "the match looks better". Concrete:
- `pick_players` no longer exists; `grep -rn "pick_players" crates/` is empty.
- The agent that acts is the one nearest the ball: a test with two fixed
  positions asserts it.
- The 4-4-2 grid is deterministic: two `SpatialState::from_squad` calls with
  the same squad produce the same map.
- `SIM_VERSION` is 8, the golden test passes with the new hash.
- The board still renders: `cargo run -p pcfm-demo -- --seed 42 --board`.

## 9. What the next agent must not do

- Do not add `PositionSnapshot` to `MatchEvent`. It is a save-breaking
  change, and there is no consumer. It returns when a view needs it.
- Do not touch `pcfm-hidden`. Perception reads `SpatialState`, not latents.
  If a decision needs a latent (finishing, passing), it goes through
  `resolve_*` as today.
- Do not introduce `f32` coordinates. Integer, per ADR-019 D1.
- Do not add `ContextModifiers`. It has no producer (L2 does not exist).
- Do not add a formation parser. 4-4-2 hardcoded, warning for the rest.
- Do not add an event that has no consumer and no cause the engine can
  model. §6-ter lists the candidates; an event outside the map is an idea,
  not a design.
- Do not calibrate. The parameters in `balance/` are arbitrary and
  declared. Calibration is a later step with data.
