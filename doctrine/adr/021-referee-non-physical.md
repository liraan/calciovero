# ADR-021 — Referee is a non-physical perceptual agent

- **Status:** accepted
- **Date:** 2026-10-29
- **Supersedes:** ADR-019, decision D9 only (all other decisions of ADR-019 remain in force)
- **Superseded by:** —
- **Blocks:** 2D implementation of referee positioning behaviour; any engine rule that relies on the referee as a collision body.
- **Depends on:** ADR-019 (match engine contract, D1 and D9).


**Note (2026-11-03) — DEFERRED for the new engine.** The new engine has an **omniscient arbiter** (SESSION-STATE T1.3): no vision cone, no obstructed-perception errors, the match is fair. The perception model described below belongs to the prototype; it is not the model of the new engine. Do not implement from this ADR without reading T1.3 (and T1.3-bis: "fairness over realism" is a declared design value). The parts of this ADR that still hold in the new engine: the arbiter is non-physical, not a collision body, its position is informational for the renderer.


**Note (2026-11-13) — the maintainer's decision for the new engine.** *"The referee is not an actor: the referee is the rules, applied always objectively; present, but without occlusion."* So: (1) no perception, no position-dependent error, no strictness: every decision is the exact application of the Laws to the facts of the match (T1.3, T1.3-bis confirmed); (2) **present**: the referee is part of the match the manager watches, shown on the pitch as a projection computed for the renderer; (3) **without occlusion**: he is not a body, the ball and the players pass through him, he blocks no line and no view. The VISION's "actor with his limits" applies to the players and the staff, not to the Laws. The laws still missing (offside, fouls as an outcome of contact, advantage, cards) enter as exact rules; the catalogue is `L0-match/design/ACTIONS.md` §6.

## Context

ADR-019 D9 currently says:

> **D9. Referee is a normative agent.** In-pitch, with `Position` and
> functional state, but not competitive: it does not play for the ball. Its
> behaviour (positioning, avoidance, whistling) is deferred to the 2D
> implementation. In v1.0: position and whistling, no full physics.

Two things in D9 are in tension with the 2026-10-29 discussion:

1. **`Position` and “avoidance”** imply that the referee is a physical body
   that players must route around. If so, the referee influences the
   simulation (a player who would have reached the ball is delayed by an
   obstacle), and its position becomes part of the state that must be
   deterministic and calibrated.
2. **“Normative agent”** is accurate (the referee enforces the rules), but the
   phrase does not say what the referee’s *inputs* are. A referee who sees
   everything makes no mistakes; a referee who sees nothing is random. The
   interesting middle — a referee whose decisions depend on what they could
   see — is not described.

The 2026-10-29 discussion agreed on a different model: **the referee is
visible and follows play, but is not a physical obstacle**. Decisions derive
from **perception** (what the referee sees or does not see) plus the rules,
not from being at a specific `Position`. Errors emerge from obstructed vision,
not from physical position.

## Options considered

### Option A — Status quo (physical agent)

Referee has `Position`, players avoid it, positioning is simulated and
calibrated. Referee position is part of the deterministic state.

**Pro:** matches the letter of D9; easy to reason about.
**Con:** the referee adds a collision constraint that costs simulation cycles
and yields no gameplay value — an obstacle that is neither fun nor
instructive. Position must be calibrated and is a hidden source of edge cases
(players stuck behind the referee, pathing around it). Determinism surface
grows without a corresponding gain.

### Option B — Non-physical perceptual agent (chosen)

Referee is **visible** (rendered, follows play) but **not a collision body**.
It has a **perception** (a visual field, or an equivalent abstracted input)
that determines what it can and cannot see. Decisions (fouls, offside,
cards) are computed from perception + rules. Errors are possible when the
visual field is obstructed.

**Pro:** removes an obstacle that adds no gameplay; makes refereeing errors
explicable (“the referee was unsighted”); keeps the referee visible for the
renderer without coupling the simulation to its position; perception is a
**derived** property (like D8 zone of influence), not stored state.
**Con:** requires a perception model (what does the referee see?), which is
non-trivial and must be calibrated. The visual field is a new concept that
needs its own definitions.

### Option C — Omniscient referee

Referee sees everything, never errs.

**Pro:** simplest.
**Con:** removes refereeing uncertainty, which is part of the game. Also
inconsistent with the Three Planes: an omniscient referee is a Plane A
observer, which contradicts the observation/estimation split the project is
built on.

## Decision

**Option B is proposed.**

- The referee is a **normative but non-physical** agent. It is visible to the
  renderer, it follows play, it emits whistles and cards. It is **not** a
  collision body: no player is delayed by it, no path is altered by it, its
  `Position` does not influence player outcomes.
- Referee decisions are a function of **perception + rules**. Perception is a
  **derived** property (like D8), computed per decision, not stored as state.
- Referee **errors** emerge from obstructed perception, not from being at a
  particular position. An unsighted referee can miss a foul; a well-placed
  one is more likely to see it. The specific perception model is deferred
  (see Open questions).
- **`Position` for the referee remains, but is informational for the
  renderer**: it is used to draw the referee on screen, and (optionally) to
  compute what the referee can see. It does not participate in
  `state_hash` as a physical constraint, and it does not influence player
  movement.

### What this changes in ADR-019

Only D9. The rest of ADR-019 (D1–D8, D10, D11, deferred 1–9) remains in force.
In particular:

- **D1** (“entities with their own state are allowed”) still holds: the
  referee is an entity with its own state (perception, current decision
  context), it is just not a **physical** one.
- **D4** (“`Position` applies to players, the ball, and the referee”) still
  holds for the **type**, but the referee’s `Position` is informational, not
  a physical constraint.
- **D10** (assistant referees peripheral, no in-pitch position) is unchanged.

## Consequences

### Easier

- No collision coupling: the referee can be rendered without affecting
  `state_hash` as a physical body.
- Refereeing errors have a **cause** (obstructed perception), consistent
  with pillar 5 (radical transparency).
- No calibration of referee positioning: the position is cosmetic or
  perception-input, not a physics parameter.

### Harder

- The **perception model** must be designed: what the referee sees, how it is
  computed, how it interacts with player positions and the ball. This is a
  new concept and needs a dedicated ADR or module (probably
  `L0-match/12-referee.md`).
- The renderer must draw the referee as a visible marker without implying
  physical presence. UI decision.
- Tests for refereeing errors must be expressed in terms of perception, not
  position: “the referee was unsighted” becomes a test case.

### Locked in

- The referee is **not** a collision body. It does not influence player
  outcomes.
- Referee decisions derive from perception + rules, not from `Position` as a
  physical quantity.
- Refereeing errors are possible and explicable (obstructed perception).
- The perception model itself is **not** locked in by this ADR — it is left
  to a later decision.

## Open questions

1. **Perception model.** What does the referee see? A cone? A set of relevant
   entities? Is perception computed from `Position` + facing direction, or
   abstracted (e.g. a per-decision check)?
2. **Where does perception live?** In `pcfm-hidden` (Plane A, since it
   determines decisions), or in a new plane-B computation, or in the referee
   module?
3. **Does referee positioning matter at all in v1.0?** If the perception
   model is deferred, the referee can be purely cosmetic in v1.0 and gain
   perception later. Or v1.0 already includes a minimal perception.
4. **What errors are possible?** Missing a foul, misjudging offside,
   missing a card. Which of these is in scope for v1.0, and which is
   deferred?
5. **Does the assistant referee (D10) have perception too?** If yes, it is a
   second perceptual agent, and the model must apply to both. If no, it
   remains abstract.
6. **Interaction with `VARReview` (D11).** If VAR exists later, does it
   replace the referee’s perception or augment it?

## References

- ADR-019 — match engine contract (D1, D4, D9, D10, D11)
- ADR-003 — Plane A enforcement (relevant only if perception is later decided to live in Plane A; see Open question 2).
- `doctrine/00-foundations.md` §3 — Three Planes (observation vs truth)
- conversation 2026-10-29 (this ADR is its first written record); related sketches in
  `process/IDEAS.md` (IDEA-TACT-001, IDEA-INFRA-007, IDEA-STAFF-001)
