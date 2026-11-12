# ADR-034 — Scanning: when and where the gaze turns

- **Status:** accepted
- **Date:** 2026-11-07
- **Supersedes:** — (specifies the behaviour §7.7 of L0.11 already declares; does not supersede ADR-033)
- **Blocks:** OI-71 (implementation); the F.6 step 2 test deferred to OI-71
- **Depends on:** ADR-033 (gaze is separate from heading), ADR-030 (fatigue as a modulator, cognitive mechanism declared), L0.11 §7.7 (scanning is a behaviour, not a datum), MD-40 (perception delay), OI-70 (fatigue producer, not implemented)

## Context

ADR-033 separates the body (`heading`, the direction of motion) from the gaze (`gaze`, where the player looks), and makes the cone read `gaze`. It also declares the L0 default: "an agent looks at the ball". That default makes `perceived_ball` return `Some` for every agent, so F.6 step 2's filter is identity and the "real match" filter (an agent does not react to a ball he does not see) does not exist yet.

L0.11 §7.7 already declares the behaviour: **scanning (turning the head before receiving) uses vision and concentration, and lives in the decision code (F.6)**. This ADR specifies that behaviour: when the gaze turns, towards what, with what latency, and which characteristics modulate it.

The behaviour is not a datum (not a latent, not a state, not saved). It is recomputed every tick from the situation and the characteristics, as §7.7 requires.

## Decision

**The gaze turns towards the most salient candidate, chosen by a deterministic geometric rule. The characteristics modulate the behaviour; they do not choose the target.**

### The five relational candidates

The salience is relational and situational, not categorical: the role does not decide what the player looks at. At any tick the candidates are:

1. **The ball.** Always a candidate. The default, not the only one. Its score grows with proximity to the player and with the ball being in flight towards him.
2. **The opponent nearest the ball.** The immediate threat: who can play it, who is about to receive. Scored by the opponent's distance to the ball and by the player's own distance to that opponent.
3. **The opponent nearest the player.** The mark. Salient even when the ball is elsewhere. Scored by the opponent's distance to the player.
4. **The receiving space.** The point where the ball will be, if the ball is in flight towards the player. Scored by the flight time and the player's distance to the landing point.
5. **The line.** The team's line (teammates and opponents between the player and his own goal), when the ball is far and the team is defending. Scored by the distance from the player to the line and by the team's phase.

The score of each candidate is a geometric function of positions and the phase (possession, ball in flight, defending). The rule is deterministic: the highest score wins. **No die enters the choice.** Ties are broken by `PlayerId` order, as ADR-019 already requires for every agent decision.

**Priority cascade (declared, not a weight).** If the player is marking (candidate 3 within a declared radius), the mark beats the ball. This is the only case where "the man" beats "the ball": it is the marking situation, and it is a rule, not a preference. Every other candidate competes on score.

### The characteristics modulate the behaviour

- **`vision`** (trait, Plane A) is the width of the visual field. It does not move the gaze; it changes what the gaze captures once it turns. Wide vision sees more per look, so it reduces the need to re-scan: `vision` sets the **re-scan interval**, not the direction.
- **`concentration_level`** (state, Plane B) modulates the **frequency of the scan**: a concentrated player re-checks more often; a distracted one less. The producer of `concentration_level` is not in place today, so the term is declared and its consumer is an open item, not a hidden default (D3).
- **`game_reading`** (trait) is **not** in the scanning rule. §7.7 assigns it to positioning, not scanning. Game reading changes *which option the player then chooses* (MD-43), not where he looks. Adding it here would duplicate the decision code's job.
- **Cognitive fatigue** (ADR-030) lengthens the latency of the turn when OI-70 gives fatigue a producer. Declared, not implemented.

### The latency

The gaze does not turn instantly. The turn has a latency of **N ticks**, where N is a declared default in `L0-match/balance/` (prototype, not calibrated) and is reduced by `vision` (a wide field needs less turning) and increased by cognitive fatigue (when it exists). The latency is deterministic: the same situation produces the same N.

### What does not change

- `gaze` stays Plane B (observable): a spectator sees where a player looks.
- `heading` stays the body's direction of motion (F.7 step 9).
- The cone (`visible_to`, `perceived_ball`) reads `gaze` (ADR-033).
- No die enters the gaze (MD-43).

## Consequences

- The scanning rule lives in the decision code (`move_agents` or a helper it calls), not in `pcfm-hidden`: it reads no latent. The characteristics it uses (`vision`, `concentration_level`, cognitive fatigue) are read through the custodian when the consumer exists.
- `L0-match/balance/` gains a file for the scanning parameters (the five candidate weights, the marking radius, the latency N). They are arbitrary and declared until calibration (D6: no number without a source).
- `SIM_VERSION` bumps when the rule enters the code, because `gaze` changes, and with it who is seen, and with it what happens. The bump is declared in `CHANGELOG-SIM` with the behaviour change, not promised by this ADR (the implementation session decides after the golden).
- The F.6 step 2 test ("an agent running east, looking west, does not move towards a ball to the east") becomes executable: `gaze` can differ from `heading` for more than one tick.
- OI-71 closes when the rule is implemented, not when this ADR is accepted.

## Alternatives considered

- **The gaze always towards the ball (the ADR-033 default).** Rejected as the final rule: it makes `perceived_ball` always `Some`, and the "real match" filter does not exist. It stays as the starting default of the prototype, not as the model.
- **Role decides the target (defender -> the man, midfielder -> the space, striker -> the line).** Rejected: it is a categorical shortcut that fails on real cases (a defender in build-up looks at the free teammate, not at the man; a pressing striker looks at the passing lane). The role modulates the weights at L3; it does not choose the target.
- **A stochastic gaze (a die inside the choice).** Rejected: a noisy gaze is a puppet with a coin (MD-43). The variation comes from the situation and the characteristics, not from a random draw.
- **Derive `gaze` from `heading` with a head-yaw offset.** Rejected at L0 (ADR-033): it ties gaze to motion. The head yaw belongs to L2 (shoulder check).

## References

- ADR-033 (body heading and gaze are two things)
- ADR-030 (fatigue as a modulator, cognitive mechanism declared)
- L0.11 §7.4 (cognitive family: vision, game reading, decision making, risk management), §7.7 (behaviours: scanning, positioning, pressing)
- MD-40 (perception delay), MD-43 (the actor decides from his point of view)
- OI-71 (the finding that triggered this ADR), OI-70 (fatigue producer)
- F.6 (the first consumer: `perceived_ball` in `move_agents`)
