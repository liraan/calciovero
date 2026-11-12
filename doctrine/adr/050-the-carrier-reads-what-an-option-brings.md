# ADR-050 — The carrier reads what an option brings

- **Status:** proposed
- **Date:** 2026-11-12
- **Supersedes:** —
- **Amends:** ADR-040 (the salience of the pass and of the shot: a term each)
- **Depends on:** VISION §3, ADR-035 (no dice), ADR-040 (salience, not probability), ADR-047 (the choice is recognition), ADR-049 (every act is the player's)

## Context

A footballer with the ball plays to bring his side closer to a goal. Every option is read for what it brings (getting closer to the goal) and for what it risks (losing the ball); the shot is not a separate kind of option, it is the end of the same intention, more attractive as the goal gets closer and opens to the eye.

ADR-040 reads only how *easy* an option is. The pass is `freedom × line × distance`: a pass back to the centre back is as salient as a pass to the striker. The shot is `distance_factor × goal_cone`, with a distance factor falling linearly to zero at 25 m. Measured on seeds 42-51 at `SIM_VERSION` 48: 900-2,500 passes and almost no shots a match; in the 2D view a carrier at the edge of the box passes back.

## Decision

**Every option carries a term for what it brings, and the carrier's characteristics decide how much he weighs it.**

1. **The pass reads its progression.** `progression = (d_goal(carrier) - d_goal(receiver)) / d_goal(carrier)`, in `[-1, 1]`: positive forward, negative back. The pass's salience is multiplied by `1 + w × progression`, clamped above a floor so that a pass back stays a real option (the safe pass exists in football). `w` is the carrier's appetite for risk: `risk_management` (ADR-047: the choice is recognition, the player's own reading), scaled by `balance/choice.json`.
2. **Under pressure, safety counts more.** When an opponent is close (the scene's `nearest_threat`, already read for the latency), `w` falls: a pressed player keeps the ball, a free one plays forward.
3. **The shot does not vanish at 25 m.** `distance_factor` becomes a smooth decreasing function of the distance (high in the box, small but not zero from distance), with its scale in `balance/shot_choice.json`; `goal_cone` and the keeper's cover (ADR-047 §3) stay as they are.

All terms are deterministic and read only the carrier's scene and his characteristics: no die, no global decider.

## Consequences

- `SIM_VERSION` bump, golden regenerated, P1 tests: a forward pass to a free teammate is more salient than the same pass backwards; a pressed carrier weighs progression less; a shot from 20 m in front of goal is more salient than one from 30 m, and neither is zero.
- Measured afterwards on 10 seeds and in the 2D view: the target is the order of magnitude of a real match (about 20-30 shots, 900-1,000 passes); calibration is L3.
- Out of this ADR: the off-ball candidates that make a forward pass possible (offering for a pass, runs in behind: ADR-049's next set), the long ball and the cross as options of their own.

## Alternatives considered

- **Tune the clarity threshold or the shot range only.** Rejected: it would raise the number of shots without giving the carrier a reason; the backward pass would stay as salient as the forward one.
- **A global "attack" mode for the side in possession.** Rejected: a global decider, and the choice would be the team's, not the player's (ADR-049).
