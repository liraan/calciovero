# ADR-019 — Match engine contract

**Status:** accepted
**Date:** 2026-10-27
**Supersedes:** none (clarifies, does not replace, ADR-013)
**Superseded by:** ADR-021 (D9 only)
**Blocks:** positional match engine implementation; first L1 module (`L1-world/01-nation.md`)
**Depends on:** ADR-012 (plugin contract), ADR-013 (LOD and local determinism)

## Context

The current L0 prototype (`MatchState::tick()`) is a per-tick probability model
(60/25/12/3). The declared goal is a positional simulation of 22 players, with a
path to 2D without rewriting the architecture. Discussion has settled on an
**agent-first** direction: state lives in players, the grid is only a
quantisation and an adjacency graph. Explicit decisions are needed on state,
rules, position, and resolution, otherwise the implementation will take them
implicitly.

## Options

**A. Grid-centric** — state lives in cells, players are derived. Simple, but
closes the door to 2D and makes defensive blocks artificial.
**B. Agent-first** — state lives in players; grid = quantisation + graph. More
expressive, compatible with 2D, requires discipline to avoid duplicating state.
**C. Pure event-driven** — events only, no persistent position. Consistent with
the prototype, but does not represent retreats or blocks.

## Decision

**D1. Agent-first (option B).** State belongs to players. The grid is a
quantisation function of `Position` and an adjacency graph; it is not an entity
with its own state.

The engine simulates **N agents** (22 players + referee + peripheral agents
when active), not a fixed set of 22. Entities with their own state (ball,
referee) are allowed: D1 forbids state in the grid, not in entities.

**D2. Player functional state: closed enum.** Built-in variants only for now
(`InPossesso`, `InPressione`, `InCopertura`, `InRipiegamento`, `InCorsa`,
`InAttesa`). Extension is decided by the plugin contract (ADR-012), not by this
contract. The engine interprets only built-in variants.

The distinction from D3 is deliberate: functional state is **structural**
(its built-in repertoire), while psychological axes are **parameters**
(the supported set is implementation-defined, see D3).

**D3. Shared rules + per-player parameters.** A single rule set; individual
behaviour emerges from parameters (observable attributes, functional state,
tactical instructions, and psychological axes, passed as named parameters). The
set of active axes is configuration-defined; the set the engine understands is
implementation-defined. The interaction between tactical instructions and
psychological axes (e.g. how psychological state modulates adherence to
instructions) is deferred to L2-tactics and L2-psyche. Not independent
per-player rules: evaluation order would become semantics, with testing cost and
determinism risk.

**D4. Position: integer coordinates.** `Position { x: u16, y: u16 }` in integer
metres on a reference pitch of 105×68 (real standard dimensions). The grid is a
quantisation of `Position`, not the primary type.

`Position` applies to players, the ball, and the referee. Assistant
referees have no in-pitch position in v1.0 (see D10).

**Note (2026-11-03) — D4 amended.** `Position` is in integer centimetres
(`i32`, signed), not metres `u16`: this confirms `spatial.rs:8-10,20`
(`AgentState.position: (i32, i32)`, "Coordinates are integer centimetres").
The named type `Position` does not exist in the code yet; when introduced,
it uses cm `i32`. The ball's 3D position (MD-17) lives in a separate
`BallState` (MD-25), not in `Position`: this amendment covers the unit and
the type, not the ball (see the deferred-3 note below).

**D5. Resolution: engine parameter, fixed per match, saved.**
`Resolution { width: u16, height: u16 }`, mandatory in `MatchSave`, included in
`state_hash`, verified on load and replay. Default: 105×68. The engine does not
know the platform: it receives the value and respects it.

**D6. LOD is not saved in `MatchSave`.** LOD is derivable from the module
manifest (ADR-013); saving it would duplicate state. Verifying that the manifest
exists and is coherent remains mandatory.

**D7. Two distinct parameters, single user experience (deferred).** LOD and
resolution are separate concepts (content vs granularity). The user chooses the
**modules** (from which LOD is computed) and the **resolution** freely. The
granular interface (separate choice of LOD and resolution) is **deferred**.

**D8. Zone of influence (derived, not stored).** A player's zone of influence
is a **derived relation** (player + observable attributes + functional state +
context), computed with integers or fixed-point, **not stored** as state. The
engine uses it to determine who can intervene on a position; it is not an entity
with its own state, like the grid. The observable attributes referenced here are
the same as those in D3. The specific attributes and formula are
deferred (see deferred 8).


**Note (2026-11-02) — D8 reinterpreted for the influence model.** The formula lives in `03-entities-minimal.md` §2.9; the weights in `L0-match/balance/influence.json` (arbitrary, declared, "prototype, not calibrated"). "Observable attributes" in D8 is reinterpreted as **"computed inside `pcfm-hidden` with the latents, exposed to the engine as an observable order (`Vec<PlayerId>`)"**: the engine never sees a latent, and never sees a continuous influence value (anti-CA/PA: no scalar in Plane B). D8's "derived, not stored" is unchanged: the influence is computed per query, never saved. The rest of D8 (engine uses it to determine who can intervene on a position) is unchanged.
**D9. Referee is a normative agent.** In-pitch, with `Position` and
functional state, but not competitive: it does not play for the ball. Its
behaviour (positioning, avoidance, whistling) is deferred to the 2D
implementation. In v1.0: position and whistling, no full physics.

**D10. Assistant referees are peripheral normative agents.** No in-pitch
position in v1.0: they signal offside and out-of-play abstractly. Their
position along the touchlines is deferred to the 2D implementation.

**D11. Self-contained temporal events, two synchronous phases.** Temporal
events (`PassAttempted`, `ShotAttempted`, `DuelStarted`, `FoulCommitted`)
are **self-contained**: each carries a snapshot of the relevant positions
at the moment it occurs. Rule plugins read the snapshot. Rules produce the
outcome in a second, synchronous phase (e.g. `PassAttempted` -> `Pass` or
`Offside`). No VAR in v1.0: a future `VARReview` event may delay the
decision when uncertain (see `WORLD-MODEL.md`).

## Deferred decisions

The following are explicitly outside this ADR and must be closed by later ADRs
or dedicated notes before implementation:

1. **Player iteration order** (determinism): **spatial order** (squared
   integer distance from the ball), with `EntityId` as tie-break. All 22
   players are iterated each tick. The set of "active" players (radius,
   zone) is deferred to deferred 2.
2. **Off-ball pre-emption** (preventive foul, defensive block): an off-ball
   action may pre-empt an on-ball action if its tick cost is strictly lower
   and the defender is within a radius R. Equal cost: spatial order (deferred
   1) decides. Preventive foul pre-empts passes and dribbles, not shots
   inside the box (that would be a penalty). Costs, R, and the pre-emption
   matrix live in `docs/calibration/` (currently empty).
3. **Possession representation (resolved).** `possession: Option<PlayerId>`
   (the carrier, or none) plus `ball_state: Possessed | InFlight | Contested |
   Free`. The ball is an entity with its own state (compatible with D1: the
   grid is not an entity, the ball is). The state machine must prevent invalid
   combinations (e.g. `InFlight` requires `possession = None`). The ball's
   `Position` is deferred to the 2D implementation.

   **Note (2026-11-03) — deferred 3 amended for F.3.** The ball's `Position`
   is no longer deferred to the 2D implementation: MD-17/25 put the ball's
   continuous 3D state (`InFlight` with position) in F.3, at v1.0. The
   decision is the ball has an intermediate position with height, saved in
   the state hash and reconstructed by replay (MD-41), not a field in
   `MatchSave`. `Possessed | InFlight | Contested | Free` (above) is
   superseded by MD-25's `Possessed | InFlight | Loose`, declared in F.3.
4. **Granular UX**: separate choice of LOD and resolution, readable presets,
   per-platform defaults.
5. **Resolution values per platform** (mobile, ADR-014): not before mobile is
   real.
6. **Adaptive resolution** (during the match): not recommended due to
   determinism risk; to be revisited only with a dedicated ADR.
7. **LOD override** beyond the sum of weights: would require an amendment to
   ADR-013.
8. **Zone of influence formula and attributes.** Exact formula, required
   attributes, and their definition in `pcfm-model`. Deferred to a later ADR or
   a revision of `03-entities-minimal`. Formula calibration lives in
   `docs/calibration/`.
9. **Role → tactic → parameter chain.** How ADR-016 role mechanics become
   engine parameters. Deferred to `L2-tactics`.

## Consequences

- **`SIM_VERSION`**: bump when the contract enters into force (simulation model
  change).
- **`SCHEMA_VERSION`**: bump when `MatchSave` receives `resolution`.
- **Tests**: tests that build `MatchSave` must be updated; new test for the
  default and round-trip of `resolution`.
- **No conflict with ADR-013**: LOD remains defined as the sum of weights;
  resolution is an additional, orthogonal parameter.
- **No code changes in this phase**: this ADR is `accepted`; implementation
  follows ratification and is tracked in deferred 1–9.

## Implementation notes

- The first real consumer of `resolution` will be the positional engine; until
  then the field stays in `MatchSave` with default and tests, without game logic
  using it.
- `Position` and `Resolution` are distinct types: `Position` is a player's
  position, `Resolution` is the pitch granularity. They must not be confused or
  unified.
- The grid, when it arrives, will be a type derived from `Position` +
  `Resolution`, not a primary type of the contract.
- **Current status**: the code does not yet implement D4/D5; the gap is
  intentional and tracked in deferred 1–9. Implementation follows
  ratification.

## References

- ADR-012 (Plugins: brick contract)
- ADR-013 (Worlds: LOD and local determinism)
- ADR-015 / ADR-016 (v1.0 scope)
- `doctrine/00-foundations.md` §3 (Three Planes), §9 (Concentric Structure)
- `L0-match/specs/04-match-engine.md` (current engine specification)
- `process/REPO-SNAPSHOT.md` §6.5 (`pcfm-match` state)
