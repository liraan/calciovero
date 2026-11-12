# ADR-033 — Body heading and gaze are two things

- **Status:** accepted
- **Date:** 2026-11-07
- **Supersedes:** — (resolves IDEA-PERCEPTION-002; amends MD-34's "orientamento = sguardo (collasso dichiarato)")
- **Blocks:** F.6 step 2 (use `perceived_ball` in `move_agents`); the "turn to face play" decision (OI-71)
- **Depends on:** MD-43 (the actor decides from his point of view), MD-34 (the cone, `visible_to`), F.7 step 9 (`heading_from_velocity`), IDEA-PERCEPTION-002 (the sketch this decision resolves)

## Context

Today `AgentState` has a single `heading: u16`. MD-34 declared the collapse explicitly: "orientamento = sguardo (collasso dichiarato): testa/corpo e saccadi sono sketch in IDEA, non campi di stato". The collapse worked while the only consumer was `visible_to` called for the attacker alone, whose heading was overwritten toward the goal before the read (`state.rs:361`).

F.6 changes that. The step "the agent perceives the ball, not the engine" needs the ball to be filtered by the cone for every agent that moves, not only for the chaser. Two forms were considered in OI-71, and both fail:

- **Form A (filter only the chaser).** Leaves the main shortcut intact: 21 of 22 agents still react to the global ball through the translated slot (`dx/dy`). A simplification the "real match" filter condemns.
- **Form B (filter all movement with the single `heading`).** Freezes agents, because `heading` is derived from velocity (F.7 step 9) and velocity from movement: no see, no move, no turn, no see.

The root cause is not F.6. It is the collapse: a defender running toward his own goal while looking at the ball is not representable. In a real match, a player turns because he decides to look, not because he moves. Body and gaze are two things.

## Decision

**`AgentState` gains a second field, `gaze: u16` (degrees, 0 = north, clockwise). `heading` stays the body's direction of motion, derived from `velocity` as today (F.7 step 9). The cone (`visible_to`, `perceived_ball`) reads `gaze`, not `heading`.**

- **`heading`** = where the body is going. Updated by the movement pass (`heading_from_velocity`), never set by a decision. Zero velocity = heading unchanged.
- **`gaze`** = where the player is looking. Set by a decision, not by the movement pass. At L0, a declared default: an agent looks at the ball. At L2, the manager's instructions and the player's role modulate it (a defender checking his shoulder, a striker scanning the line).
- **The cone reads `gaze`.** `visible_to` and `perceived_ball` take the observer's gaze vector instead of the heading vector. `heading_vector` keeps its meaning; the call sites change.
- **`gaze` is Plane B (observable).** A spectator sees where a player looks. No latent, no hidden value.
- **`gaze` is state, not a decision in the sense of MD-43.** The decision is "where to look", not "what to do". MD-43 says the actor decides from his point of view; the gaze is the point of view, and it is a state the actor carries, not a die.

## Consequences

- `AgentState` gains `gaze: u16`; `from_squads` initialises it (the ball is at the centre, everyone looks at it).
- The `SIM_VERSION` bump happens when the field enters the state and the cone reads it (implementation session, not this ADR). The state is in the hash through the events that depend on it (a different gaze changes who is seen, which changes what happens).
- `visible_to` and `perceived_ball` change their input (gaze vector, not heading vector). The tests are updated accordingly.
- F.6 step 2 becomes executable: each agent moves toward its slot, the chaser toward the ball, and each agent's perception is filtered by its own gaze. The "turn to face play" rule at L0 is declared: **gaze points at the ball** (prototype; L2 modulates).
- IDEA-PERCEPTION-002 moves from `sketch` to `resolved by ADR-033`. The head yaw / shoulder check remain L2 refinements; the field exists at L0 with a declared default.
- MD-34's sentence "orientamento = sguardo (collasso dichiarato)" is superseded: the collapse is removed. A dated note in MD-34 points here.

## Alternatives considered

- **Keep the collapse, turn the body toward the ball before perception (form A).** Rejected: a defender running back to his goal would face the ball and run backwards. The "real match" filter condemns it, and the shortcut it leaves (21 agents reading the global ball) is exactly what F.6 exists to remove.
- **Keep the collapse, accept the freeze (form B).** Rejected: a motor with half the players still is not football.
- **Derive `gaze` from `heading` with an offset (head yaw).** Rejected at L0: it still ties gaze to motion. The head yaw belongs to L2 (shoulder check), as IDEA-PERCEPTION-002 already says.
- **A separate `gaze` decision with its own die.** Rejected: a noisy gaze is a puppet with a coin (MD-43). At L0 the gaze is a declared default (the ball); L2 modulates.

## References

- MD-34 (the cone), MD-43 (the actor decides from his point of view)
- IDEA-PERCEPTION-002 (head/body separation, this decision resolves the L0 part)
- OI-71 (the finding that triggered this ADR)
- F.6 (the first consumer), F.7 step 9 (`heading_from_velocity`)
- `crates/pcfm-match/src/spatial.rs`, `crates/pcfm-match/src/perception.rs`

## Amendment (2026-11-07) — gaze as a decision

The original text says "gaze is state, not a decision in the sense of MD-43" and rejects "a separate gaze decision with its own die". The second half stands: a noisy gaze is a puppet with a coin. The first half is amended: **gaze is a decision without a die**. The actor decides where to look, the decision is deterministic (geometry, context, role), and the variation comes from the situation, not from a random draw.

The declared L0 default ("an agent looks at the ball") is the first behaviour, not the only one. **The "turn to face play" mechanic — when to turn, towards what, with what delay — is open** and is tracked as OI-71, not resolved by this ADR. The default stays as the starting rule; it is not a prohibition.

What this amendment does not change: `gaze` is Plane B (observable), the cone reads `gaze`, `heading` is the body's direction of motion, and no die enters the gaze.
