# ADR-051 — Every act has its roots, and you can see them

- **Status:** proposed
- **Date:** 2026-11-12
- **Supersedes:** —
- **Amends:** L0.11 (`L0-match/specs/11-player-characteristics.md`), with the changes of `L0-match/design/CHARACTERISTICS-AND-ABILITIES.md` §7, each when its first consumer enters
- **Depends on:** VISION §3, ADR-035 (no dice), ADR-040 (salience), ADR-049 (every act is the player's)

## Context

The maintainer, 2026-11-12: *"every action that can be performed on the pitch must be coded, each one derived from the abilities and characteristics of its actor, and they must be recognisable: a very fast player will really go faster in the 2D simulation."*

VISION §3 gives the chain: characteristics → abilities → the response, dimmed by the states. The tree of that chain is drafted in `CHARACTERISTICS-AND-ABILITIES.md`. What is missing is the list of the **acts** the tree must feed, and a rule that tells, act by act, whether the engine honours the chain.

Measured on the engine at `SIM_VERSION` 50 (the full audit is in `L0-match/design/ACTIONS.md`):

- **Honoured:** running. A player's speed follows his `max_speed` and his acceleration his `explosive_force` and weight (seed 42: from 39 cm/tick at `max_speed` 0.30 to 96 at 0.79). The ground duel, the reception, the latency of a decision, the choice of the pass and of the off-ball act read characteristics.
- **Not honoured:** the save is a standing reach (half the keeper's height × `goalkeeping`), with no dive and no reaction, and today about every shot on target is a goal (3 to 17 goals a match). The shot leaves at a fixed speed (30 m/s if it is on target, 15 m/s if not: the cause after the effect), whoever kicks it. The interception is a fixed 2 m radius. The turn is instant for everyone. The keeper's position and the carry's direction are rules of the engine. Weak and strong foot are generated and read by nobody.

## Decision

**Every act on the pitch is in a catalogue; every quantity of an act comes from its actor; and the difference between two actors is visible.**

1. **The catalogue.** `L0-match/design/ACTIONS.md` lists every act a footballer or a goalkeeper can perform: the body (run, turn, stop, jump), the eyes and the mind (scan, read, decide), with the ball (receive, carry, dribble, shield, pass in its kinds, cross, shoot, head, clear), without it (hold, mark, press, challenge, intercept, block, cover, offer, run in behind), in goal (position, set, dive, catch or parry, come out, claim, distribute), and the restarts. An act the engine performs and the catalogue does not list is a defect.
2. **Every quantity has roots.** For each act the catalogue names its output quantities (speed, reach, timing, power, precision, the choice) and, for each quantity, the ability of the tree and the characteristics it reads, with the body data and the dimmers. A constant is allowed only as a **reference** in `balance/` (the speed of a reference player, the size of the goal), multiplied by the actor's characteristics; never as the value of the act itself.
3. **Recognisable, by test.** Every act that enters or changes comes with a **twin test**: two actors identical except for one characteristic, in the same scene; the record shows the difference in the expected direction and large enough for a viewer to see it in the 2D (the faster twin arrives first; the stronger kicker's shot is faster; the more reactive keeper reaches the shot the other misses). Twins are made with `ds-testkit`.
4. **Characteristics enter with their first consumer** (D3), with the amendment of L0.11 that `CHARACTERISTICS-AND-ABILITIES.md` §7 names (handling for `goalkeeping`, reactivity, joint mobility, body control, maximal force, endurance, dribbling, heading, shooting for `finishing`, and so on). No characteristic is generated without an act that reads it; the feet are read by the pass and the shot or removed.
5. **The order of work** is by what a fan sees wrong first, and is kept in the catalogue: (1) the save, with reaction and dive; (2) kick power, so the shot and the long ball leave at the kicker's speed; (3) the foot; (4) the turn and the change of direction; (5) the duel read in its situation (approach, angle, shielding), so the same two players do not give the same result forever; (6) the interception; (7) fatigue, the first dimmer; then the off-ball candidates of ADR-049 (offer, run in behind, mark, cover).

Each step of point 5 is a normal change: `SIM_VERSION` bump, golden, P1 and twin test, the catalogue row updated. A step that needs a new model form (not only new roots for a quantity already decided) gets its own ADR.

## Consequences

- The catalogue becomes the map of the engine for a contributor: each row says what an act reads and whether the engine already does it, and a row marked ✗ is a task.
- The amendment of L0.11 happens one characteristic at a time, with its consumer, not in one rewrite.
- Calibration (how fast is fast, how far a dive reaches) stays L3; until then the references in `balance/` are sourced or declared arbitrary.

## Alternatives considered

- **Fix the save alone and go on case by case.** Rejected: the same gap (a constant where the actor should be) is in the shot, the interception, the turn and the keeper's place; without a catalogue each is found only when a match goes wrong.
- **Ratings per act (a "pace" number, a "diving" number).** Rejected: an ability is a formula over characteristics, never a stored datum (`CHARACTERISTICS-AND-ABILITIES.md`, decided 2026-11-12); a rating per act would be a second, hidden copy of the player.

## Dated notes
