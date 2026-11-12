# ADR-048 — The ball can be deflected, and the deflection has a cause

- **Status:** accepted
- **Date:** 2026-11-12
- **Supersedes:** —
- **Amends:** ADR-044 §3 (the ball's chain: this ADR specifies the deflection as a case of the chain, not an addition); ADR-044 §5 (the integer path: this ADR gives the ball's formula a door); OI-52 (the "Removed-by: never" was the wrong answer: the bounce is a case of the chain, not a debt)
- **Blocks:** OI-81-step-c (the read in flight presupposes a ball that can change; a read of a trajectory that cannot change is not a read); every future read (the shooter's and the keeper's, ADR-045 §1, ADR-047 §1); the flight's duration (`FLIGHT_TICKS` fixed is not the duration)
- **Depends on:** ADR-035 (no dice), ADR-044 §3-§5 (the chain, the ball, the surface, the calculator integer), ADR-028 §3 (the formula declares its terms), ADR-041 (one crate, one responsibility)

## Context

The ball, today, cannot be deflected. `advance_ball` has no collision:
the ball crosses the goal plane and `check_shot_crossing` is the only
place where it meets something. A shot that hits the post enters as if
it were in the goal (OI-52). `FLIGHT_TICKS = 15` is fixed (`state.rs:26`):
the flight's duration does not depend on the trajectory. The ball's
physical properties are hardcoded (`BALL_GRAVITY`, `BALL_FRICTION_NUM/DEN`,
`BALL_STOP_THRESHOLD`, `spatial.rs:92-102`): they violate
`00-foundations` §5 ("No hardcoded balance values").

The maintainer, 2026-11-12: *"la palla deve poter essere deviata"*. This
is not a feature request. It is the consequence of ADR-044 §3, which
says the ball answers a chain -- `physical properties -> relations ->
effects` -- and a chain in which the ball meets nothing is not a chain,
it is a straight line to a plane. A read of a trajectory that cannot
change is not a read: it is a lookup, and the keeper who reads it is a
calculator (the 2026-11-12 note in the keeper cantiere, §1-§2).

The two directions of the defect are the same hole from opposite sides:
the keeper does not read because the ball cannot change (the read in
flight); the ball cannot be deflected because nothing meets it (this
ADR). The order is declared in the cantiere (§3): deflection first,
then the shape of the read, then the read in flight.

## Decision

**The ball can be deflected, and the deflection has a cause.** Three kinds
of cause, and each one is a relation:

### 1. The static obstacle (post, crossbar)

The ball's trajectory meets the obstacle's geometry. The obstacle is a
declared property of the pitch (position, size, elasticity), not a
collision mesh. The effect is a declared reflection: the velocity's
component normal to the obstacle is reversed, scaled by the combined
elasticity of ball and obstacle. The form is geometric and integer.

This closes OI-52. "Removed-by: never" was the wrong answer: the bounce
is a case of the chain (ADR-044 §3), not a debt. The consumer is the
read (the keeper who sees the ball change direction) and the board (a
shot that hits the post is an event a fan recognises).

### 2. The actor's contact (the keeper's hand, the defender on the line)

The contact is the actor's (ADR-039 step 3): the body meets the ball,
and the effect is a new trajectory. The contact already exists as a
concept (`resolve_save` is the keeper's contact, `resolve_duel` the
field actor's), but today the contact is a terminal outcome ("saved" or
"not saved"), not a touch that changes the ball's path. This ADR
declares the shape: **a contact that touches the ball produces a new
trajectory**, not a boolean. The magnitude of the deflection is the
contact's band (ADR-046 §2): a strong `goalkeeping` deflects less, a
weak one more.

This is not implemented here. The contact that touches the ball is
OI-81-step-d (the contact and the band). This ADR declares the shape
so that step-d is not invented around a ball that cannot move.

### 3. The surface (the ground)

The bounce (ADR-044 §4). The surface is a term with properties,
symmetric to the ball in the relation: the bounce depends on both
elasticities. Today `if z < 0 { z = 0; vz = 0 }` (`spatial.rs:316-318`):
the ball stops dead, it does not bounce.

**The condition stays out (T5).** No weather, no wear, no
`Pitch.condition`. What enters is a stable property of the surface,
declared per pitch (L1/L3), read by the engine (L0). The distinction is
already in ADR-044 §4; this ADR does not re-open it.

### 4. The properties are declared data, not latents

The ball's properties (radius, mass, elasticity, friction), the
obstacles' properties (position, size, elasticity), the surface's
properties (elasticity): they are **declared data**, not latents. A
second door, declared (ADR-044 §3): the custodian answers for the
actors, an open structure answers for the ball and the surface. The
numbers live in `balance/` files with `_source`, as every declared
parameter.

**The properties move out of the code.** `BALL_GRAVITY`,
`BALL_FRICTION_NUM/DEN`, `BALL_STOP_THRESHOLD` today are constants in
`spatial.rs`: they violate `00-foundations` §5. They move to a declared
data structure, read by the formula. The move is mechanical, and it is
part of this ADR's implementation, not a separate cantiere.

### 5. The formula lives in the calculator, integer

The relation ball-obstacle and ball-surface combines properties into an
effect. A combination is a formula: it lives in the calculator
(ADR-028 §3), not in the data module and not in a new crate. A separate
"physics crate" would duplicate the formula logic -- same shape, same
`balance/`, same `_source` -- against D7.

**Integer, not `f64`.** The ball's physics stays integer. Sources, in
precedence order: `00-foundations` §4 pillar 1 (local determinism, never
compromised; float in the simulation path is the classic way to break it,
through libm and FMA); L0.11 §4.10 (no trig, no float in the critical
path; `advance_ball` uses no `f64` today); the CI step `golden_l0 on
Linux (OI-58)`, whose comment says a red result "means a libm or FMA
divergence". The formula's arithmetic is integer: the properties are
integers (milli-units where needed), the combination is integer.

**The `Situation` trait changes meaning.** It is no longer only the
context of an actor (height, weight, fatigue); it is the context of a
formula. The trait is generic and holds; the meaning is declared, or a
reader believes the ball is a player (ADR-044 §5).

**Declared cost.** Two paths (float for the actors, integer for the
ball), and the form -- one path with a case, or two paths each with its
door -- is not decided here. The default is two paths, each with its
door, because the two regimes already exist (ADR-044 §3: two doors,
declared). The cost is the reader's, not the engine's: the declaration
must be visible.

### 6. The flight's duration follows the trajectory

`FLIGHT_TICKS = 15` fixed (`state.rs:26`) is not the duration: it is a
constant that today stands in for a duration. With the ball's physics
declared and the deflection possible, the flight ends when the ball
stops, is deflected out, or is caught -- not when a constant expires.
`Pending.due_tick` becomes a **computed** expiry, not a fixed one.

The consumer is immediate: `check_shot_crossing` fires when the ball
crosses a goal plane, not at a declared tick; the keeper's read (step-c)
updates every tick until the ball stops or is caught.

### 7. What is not decided (do not assume)

- **The list of the surfaces' properties.** ADR-044 §5 already declares
  the form, not the list. This ADR does not add the list: it belongs to
  the bounce formula, calibrated at L3.
- **The two paths' form** (one path with a case, two paths with doors):
  declared as a cost, not resolved.
- **The contact that touches the ball.** The shape is declared (§2), the
  implementation is OI-81-step-d.
- **The goalkeeper's deflection.** It is a case of §2 (the keeper's
  contact), and it lands with step-d, not here.
- **The ball's `f64` legacy.** `advance_ball` is integer today; the
  formula stays integer. No determinism bump.

## Consequences

- `BALL_GRAVITY`, `BALL_FRICTION_NUM/DEN`, `BALL_STOP_THRESHOLD` move from
  `spatial.rs` to a declared data structure in `balance/` (with
  `_source`), read by the formula. Foundations §5 is satisfied.
- `advance_ball` consults the formula at every tick (the ground, and any
  obstacle on the trajectory). The integer path is the same shape as
  today's arithmetic.
- `Pending.due_tick` becomes computed: the flight ends when the ball
  stops or is deflected out, not at a fixed tick.
- `check_shot_crossing` becomes one of the ball's effects, not the only
  one.
- OI-52 closes when the obstacle deflection lands (a separate commit, but
  the same ADR).
- OI-81-step-c (the read in flight) is unblocked **after** this ADR's
  implementation: a read of a trajectory that can change is a read.
- `SIM_VERSION` bump: the ball's behaviour changes. The golden regenerated.

## The filters

**Partita vera (D6, attori: il portiere e il tiratore, L0).** *Come si
riconosce?* Una palla che sbatte sul palo e torna indietro; un portiere
che la sfiora e le cambia strada; una palla che rimbalza. *Da cosa
dipende?* Dalla traiettoria, dalla geometria dell'ostacolo, dal contatto
del corpo, dalle proprietà della palla e della superficie. *Chi lo
produce?* La relazione palla-ostacolo, palla-attore, palla-superficie:
non un dado, non un evento casuale. Il filtro passa.

**Luna.** La proprietà rivelata: la palla non è un punto che viaggia su
una retta fino a un piano -- è un oggetto la cui traiettoria è il
prodotto di relazioni. La deviazione non è un'aggiunta: è la prova che la
catena di ADR-044 §3 vale anche per la palla. Senza, la "catena corta"
era una catena vuota.

**Cosa i filtri non giudicano.** L'architettura (dove vive la formula,
quante porte) è di ADR-041 e ADR-028, non dei filtri (NOTA §9). Il
`Situation` che cambia significato è un'architettura dichiarata, non un
modello da filtrare.

## References

- `specs/00-foundations.md` §4 (local determinism), §5 (no hardcoded
  values, coverage first)
- ADR-044 §3-§5 (the ball's chain, the surface as a term with properties,
  the calculator integer)
- ADR-028 §3 (the formula declares its terms)
- ADR-035 (no dice: the deflection is deterministic, not a draw)
- ADR-041 (one crate, one responsibility: no physics crate)
- L0.11 §4.10 (no trig, no float in the critical path)
- OI-52 (the obstacle's deflection: "Removed-by: never" was wrong)
- OI-81-step-c (the read in flight, unblocked by this ADR)
- `docs/context/analysis/2026-11-11-goalkeeper-cantiere.md` §1-§4 (the
  2026-11-12 note: the ball that can be deflected, the read as a stima)
- `crates/ds-match/src/spatial.rs:92-102` (the hardcoded `BALL_*`),
  `:316-318` (`if z < 0 { z = 0; vz = 0 }`), `state.rs:26` (`FLIGHT_TICKS`)

## Dated notes

- **2026-11-12.** Written before any code, after the 2026-11-12 note in
  the keeper cantiere (§1-§4) corrected the `[next]`: OI-81-step-c
  presupposed a ball that cannot change, and a read of a trajectory that
  cannot change is a lookup. Status `proposed`: the code does not move
  until the maintainer accepts. The claims about the code (no collision in
  `advance_ball`, `FLIGHT_TICKS` fixed, `BALL_*` hardcoded, `z < 0` no
  bounce) are verified: see ADR-044's 2026-11-10 verification for the
  same three, plus `advance_ball`'s body read in this session.

**2026-11-12 — Status: accepted.** Accepted by the derivable default (P6, maintainer directive 2026-11-07): the direction already answers it. ADR-044 §3 (accepted) says the ball answers the chain and that the bounce is a case of the chain; this ADR specifies it, the filters confirmed it (partita vera passes: post, hand, bounce are three things a fan recognises; no contradiction with ADR-035, ADR-044 §3-§5, ADR-041, foundations §5), and the code confirms it. **Four code claims, verified one by one before acceptance:** (1) `advance_ball` has no collision (`grep collision|bounce` in `spatial.rs` returns 0); (2) `FLIGHT_TICKS = 15` fixed (`state.rs:26`, used at `:536`); (3) four hardcoded `BALL_*` constants (`spatial.rs`); (4) `if z < 0 { z = 0; vz = 0 }` at `spatial.rs:319`: the ball stops dead, it does not bounce. No code change, `SIM_VERSION` unchanged. The implementation is a cantiere: it starts from the static obstacle (OI-52) because that is the smallest case with a visible effect, then the surface, then the actor's contact (which lands with OI-81-step-d).
