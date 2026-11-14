# ADR-058 — The ball is received where it arrives

- **Status:** accepted (implemented in `SIM_VERSION` 103)
- **Date:** 2026-11-14
- **Supersedes:** —
- **Amends:** ADR-051 (the act "receive": its roots and where it happens), ADR-053 (the ball is touched, not carried: now also when it arrives)
- **Depends on:** ADR-035 (no dice), ADR-049 (the act is the player's), ADR-052 (every player thinks), ADR-053, OI-98

## Context

OI-98: since `SIM_VERSION` 100, when Law 10 counts every ball, about 1.7 goals a match (seeds 42-71) are own goals, against about 0.1 in real football. Followed tick by tick (seed 42, 29'04"), the cause is the reception. A pass resolves at `due_tick`, a fixed flight time. At that tick the receiver's first touch is judged wherever he is, even metres from the ball. The pass is told complete, nobody has the ball, it rolls on with nobody going for it, often into the passer's own goal. The judgement itself almost never fails: it reads the receiver's freedom at the launch and the length of the pass, not the ball and the pressure at the moment it arrives, and a professional's first touch plus the freedom bonus clears the threshold nearly always.

Making the reception only spatial (tried, 2026-11-14, not kept) removes the own goals but leaves the game with no way to lose the ball at a reception: one seed had no shot. So how a reception fails is a model decision. The maintainer approved the recommendation the same day (*"si procedi"*).

Partita vera. *How do the actors recognise it?* A pass is complete when a teammate has the ball under control, not when a time has passed. A ball that runs past him is a lost pass, and whoever is nearest goes for it. A receiver fails his first touch when the ball comes fast, when an opponent is on him, or both; a good first touch survives more of both. *What does it depend on?* At the moment the ball reaches him: its speed, the nearest opponent, and his first touch. *Who produces it?* The receiver, with his foot; the pressure is the opponent's, the speed is the passer's.

## Decision

1. **The reception happens where the ball reaches a player**, never at a fixed time. A pass on its way is received by the first player within his reach of it (`control_loose_ball`, as since `SIM_VERSION` 52). The clock (`due_tick`) no longer judges a receiver who is not on the ball.
2. **A pass nobody reaches is lost.** When the ball has gone past its receiver (it is farther from him than the closest it came, by more than his reach) or has stopped, the pass is incomplete (`Pass { complete: false }`) and the ball is loose: anyone may go for it, the goalkeeper too when it is in his area. The possession changes only when someone controls it.
3. **The difficulty is born at the arrival**, like the keeper's catch (`save.json`): `difficulty = (speed / speed_ref + pressure) / 2`. `speed` is the ball's speed as it reaches him; `speed_ref` is the kick's reference speed (`kick.json` `speed_ref_cm_per_tick`: the hardest ball a kick produces); `pressure = 1 - distance / reach` of the nearest opponent to the ball, clamped to `[0, 1]`, where `reach` is the farthest an opponent can contest the ball, his slide (`duel.json` `reach_cm` x `slide_reach_milli`, 1.68 m). Beyond it he cannot play the ball; nearer, a contest with the body is the duel's, with its fouls and cards (ADR-042). The maintainer, 2026-11-14: *"un conto è la protezione del pallone, un'altra è sbracciare per prendere il controllo della palla: o danno fallo o addirittura cartellino; parametra tutto sulla partita reale"*. (The first draft used the 5 m pressure radius of `influence.json`, which describes when a carrier feels pressed, not who can touch the ball.) He controls it when his `first_touch` reaches the difficulty. Together, at their references (the hardest ball, an opponent on the ball), the two causes demand a perfect first touch: derived from the definitions, not chosen by looking at a statistic (RULES A5-bis). No new number.
4. **A failed touch is a loose ball near him** (as since `SIM_VERSION` 93): it runs on, slower, and whoever is nearest contests it.
5. **Declared, not read yet:** the side of the body the ball arrives on and his orientation (OI-64), the height of the ball (a ball in the air is met by the header and the chest, not here).

## Consequences

- `reception.json` loses `freedom_ref_cm`, `freedom_bonus_max_milli`, `distance_ref_cm`, `distance_penalty_max_milli` and `control_threshold_milli`: the freedom at the launch and the length of the pass were stand-ins for what the ball and the opponent are at the arrival.
- `ReceptionContext` carries the ball's speed at the arrival and the nearest opponent's distance, not the freedom and the distance at the launch.
- A pass resolved at `due_tick` keeps only the path of an opponent the passer did not see on the line (F.4-d), declared as it is.
- The twin test: two receivers equal but for `first_touch`, the same ball, the same opponent: one controls it, the other does not.
- `SIM_VERSION` changes; the golden is regenerated.

## Alternatives considered

- **Keep the clock and fix the formula only.** Rejected: the receiver would still be judged away from the ball, and the ball would still roll on with nobody going for it.
- **A probability of control.** Rejected: ADR-035.
- **A threshold chosen to reach a pass completion of about 82%.** Rejected: RULES A5-bis; the completion is measured after, never targeted.

## Dated notes

- **2026-11-14 — implemented in `SIM_VERSION` 103.** Three things the first implementation lacked, each found by measure, none by a number chosen (RULES A5-bis). (1) Without the clock nobody but the receiver went for a pass on its way, and the game stayed in its own third: a pass on its way is now anyone's to go for (ADR-052 note, the first aim is the ball). (2) A ball is met where its path this tick passes a player below the band of the header (1 m), not only where it is at the end of the tick: a lofted pass coming down at 15 m/s crossed a receiver's reach between two ticks; and a pass won in the air tells its outcome (complete if the passer's side wins it). (3) The passer imagines the reception with the same difficulty (`ds_calculator::control::reception_difficulty`), against his teammate's first touch, which a poor reader underrates (as the shooter his own miss, `SIM_VERSION` 90). (4) No clock ends a shot either: a shot is over when it meets the woodwork or stops short of the line, and the ball is anyone's (without it a shot back into play off the post stayed a shot nobody might go for, and the match froze for the rest of the half). Measured (30 seeds 42-71, a report): goals 4.67 (own goals 1.50), shots 32.7, crosses 15.0, passes 1116, completed 90%; at 102 goals 4.8 (own goals 1.70), shots 22.8, crosses 14.5.
