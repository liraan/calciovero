# ADR-053 — The ball is touched, not carried

- **Status:** accepted
- **Date:** 2026-11-13
- **Supersedes:** —
- **Amends:** ADR-042 (the duel: the carrier answers only with the ball at his foot), IDEA-DESIGN-010 (carrying is a choice: now a run of touches)
- **Depends on:** ADR-051 (every act has its roots), ADR-052 (every player thinks), ADR-048 (the ball's declared physics)

## Context

Up to `SIM_VERSION` 88 the carried ball was at the carrier's position on every tick (`follow_carrier`): it went where he went, glued to his feet. In a real match a player running with the ball plays it ahead and meets it again a few steps later; between two touches it rolls on its own, and it belongs to whoever reaches it first. Measured at 88 (30 seeds): tackles won 7.6 a match against about 30 in a real one, because a defender could only take the ball through a duel against a carrier who was always on it. The maintainer approved the model on 2026-11-13 (*"procedi"*).

Partita vera: *how do the actors recognise it?* The ball a step, a metre ahead of a man at a sprint; the defender who nicks it between two touches. *What does it depend on?* His speed, which sets how far each touch goes, and his `dribbling`, which sets how often he touches it. *Who produces it?* The carrier, with each touch.

## Decision

1. **The touch.** A carrier who chose to take the ball somewhere (a carry target) plays it that way at each touch, weighed so that, slowed by the ground (`ball.json` ground friction), it comes back to his foot after the touch period at the pace he runs that way: `u = pace x T x (1 - k) / (1 - k^T)`. How far ahead it goes in between follows from the friction, nothing else. Holding the ball (no target), standing, or with it in his hands or at a restart, the ball is at his feet.
2. **The period.** `1 + (touch_steps_max - 1) x pace / top_speed_max x (1 - dribbling)` steps, at `step_hz` steps a second (`movement.json`): walking, or the best dribbler, every step; the worst at full speed every `touch_steps_max`. Two declared numbers, the only new ones.
3. **Between two touches** the ball rolls on its own: if he has not reached it when the touch is due (he turned, he was held up) it rolls on and he goes to it. It can roll out of play (Laws 15-17).
4. **He plays it at his foot.** A pass, a shot, a clearance wait for the ball to be within his reach (`BODY_RADIUS_CM`), as a real player waits for his stride to meet it.
5. **The duel.** A defender who reaches the ball while it rolls ahead plays it with nobody to answer: the carrier's force in the contact is zero (ADR-042, amended).

No die (ADR-035). The ball's position between touches is computed from the touch, never stored as a draw.

## Consequences

- Measured at 89 (30 seeds, not a target): tackles won 7.6 -> 23.4, goals 6.2 -> 3.3, corners 25.7 -> 21.2, passes 972 -> 938. Shots on target are unchanged: 14.4 of 20.8 a match by the real definition (goals plus saves, 69%; real about 34%), corrected on 2026-11-13 from a first count that also took in blocked shots (about 90%). The shooter strikes the ball at his foot, as a real one does; the cause of that gap is elsewhere.
- Twin test: `the_better_dribbler_touches_the_ball_more_often`. P1: `a_touch_comes_back_to_his_foot`, `a_ball_between_two_touches_is_the_defenders`.
- Open: the touch is not yet in the carrier's thought (he does not imagine the defender nicking the ball between touches, nor choose a closer touch under pressure); the receiver's first touch (where he puts the ball as he controls it) is not yet a touch of this kind.
