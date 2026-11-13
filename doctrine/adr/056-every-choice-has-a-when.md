# ADR-056 — Every choice has a when

- **Status:** accepted
- **Date:** 2026-11-13
- **Supersedes:** —
- **Amends:** ADR-052 (the options a player imagines: now, and at his next decision)
- **Depends on:** ADR-052 (every player thinks), ADR-042 (the shield), ADR-034 (he sees from where he stands)

## Context

The maintainer, 2026-11-13, watching the GUI: *"non vedo protezione della palla per far alzare la squadra"*; then *"cerca di pensare a cosa pensa ed immagina un giocatore in una partita reale e parti da li"*, and *"filtro luna"* on the first version (an option "hold the ball" that imagined only the passes later). The maintainer approved the wider form (*"si"*).

Until `SIM_VERSION` 94 a carrier chose what to do now; the shield existed only as his answer to a challenge already coming in. A centre forward could not hold a long ball up for his midfielders, nor a centre back wait for a teammate to come free.

Partita vera: *how do the actors recognise it?* The forward with his back to goal, his body between the ball and his man, laying it off to the midfielder who arrives; the playmaker who waits a beat for the run. *What does it depend on?* What he sees coming (his vision, the players' motion), how far he trusts what he foresees (his game reading), how long his body holds the man off (strength, balance, first touch), what he risks (his risk management). *Who produces it?* The carrier, and the teammates whose runs he waits for.

## Decision

1. **Every choice has a when.** Beside his options now, the carrier weighs keeping the ball and playing at his next decision (`ChoiceKind::Hold`).
2. **He imagines that moment from what he sees:** everyone he sees (ADR-034), where they will be by his next decision (his latency), going as they are going: a teammate coming up, an opponent getting back, a space opening or closing. Not the team's ideal shape: the motion in front of him.
3. **He judges it with the same thought** (ADR-052): every option of that scene, the best of them.
4. **Waiting costs what it risks:** he gets there if he keeps the ball, which the pressure on him puts at risk as far as his body cannot hold the man (his shield, ADR-042); and what he foresees is worth as much as he trusts his reading of the others (his game reading). He waits only for a moment clearly better than this one.
5. **He waits only for what he would recognise:** the waiting takes the visibility of the option he waits for (ADR-039 §2: a poor reader who would not see that shot does not wait for it).
6. **Holding, he stays on the ball**, his body on it, until his next decision; a challenge meets his shield as before.

No weight of its own and no new number: the same passes, shots and carries, the same measure.

## Consequences

- Measured at 95 (30 seeds, not a target): a carrier waits about fifty to seventy times a match; passes 911 -> 891, tackles won 34 -> 35, goals 1.6 -> 2.1. A first form that weighed the later moment without the doubt of foresight, or with the visibility of an option he would not see, waited five hundred to fifteen hundred times: both were causes in the thought, found and corrected, not numbers adjusted.
- Twin test: `the_stronger_carrier_is_surer_to_shield_the_ball`. P1: `he_weighs_holding_the_ball_while_his_team_comes_up` (body, foresight).
- Open: the pass still goes where the teammate is, not where he is going (only the through ball reads the run); the teammates do not yet choose to come up because he holds (their acts read the team's block, which follows the ball); the GUI does not mark a player holding the ball.
