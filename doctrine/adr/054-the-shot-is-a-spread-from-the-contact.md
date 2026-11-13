# ADR-054 — The shot is a spread, and the contact chooses where in it

- **Status:** accepted
- **Date:** 2026-11-13
- **Supersedes:** —
- **Amends:** ADR-046 §1 (the shot's miss), ADR-051 (the family "the kick": the error's direction, OI-79's placeholder)
- **Depends on:** ADR-035 (no dice), ADR-052 (every player thinks), ADR-053 (the touch)

## Context

Up to `SIM_VERSION` 90 a shot's miss was one offset of known size and fixed side (towards the post he aimed at, or the way his body moved). The shooter reckoned it and aimed inside it, so from 12-18 m 98% of the shots were on target (30 seeds; real about a third of all shots). In a real match the same player striking the same ball never puts it in the same place: from 11 m, ball at rest and no defender, players with ten years of football spread it by about 0.57 m per axis (Shimotashiro and Shinya 2024). The maintainer approved the model on 2026-11-13 (*"go this way"*).

Partita vera: *how do the actors recognise it?* The shot that runs away wide, the one that flies over, the one dragged back across. *What does it depend on?* How wide his miss can be (his finishing, his foot, the power, the pressure, the run, fatigue) and the moment his foot meets the ball. *Who produces it?* The shooter, at the contact.

## Decision

1. **The width** of the miss is the angle the engine already computes (ADR-051), with its base `error_ref_rad` derived from the source (0.062; before 0.08, arbitrary).
2. **Where in it the ball goes is the contact:** the phase of his stride when his foot meets the ball. Every player's stride moves on with the ground he covers (a step is the fastest top speed over the running cadence). In the middle of the step the strike is clean; late the foot opens, the ball runs away from the post he chose and rises; early it is pulled back across and kept low. The run's part of the miss drags the ball the way his body moves.
3. **Up and down** it spreads as much as across, as in the source: `vertical_share` 2 (before 0.5, arbitrary).
4. **He cannot see the contact as he decides.** He knows how wide his miss can be, as well as he reads the game, not which way it will go; he aims so that his widest miss, as he reckons it, still meets the inside of the post.

No die (ADR-035): the phase is a fact of how he has moved, the same in every replay.

## Consequences

- Measured at 91 (30 seeds, not a target): shots on target, goals plus saves, 14.6 -> 10.6 of 20.7 a match (51%; real about 34%), goals 3.8 -> 2.2.
- P1: `the_contact_spreads_the_shot`.
- Open: the shooter does not yet choose the height of a shot (every shot is aimed low, but for the lob), nor weigh going over the bar in his chance; the pass's precision keeps its own declared reference; the step does not yet scale with a player's height.
