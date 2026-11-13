# ADR-052 — Every player thinks: the choice is his own imagined football

- **Status:** accepted
- **Date:** 2026-11-13
- **Supersedes:** —
- **Amends:** ADR-040 (salience as hand-weighted terms per kind of option), ADR-050 (progression as a separate term), ADR-049 (the off-ball weights); keeps ADR-047 (recognition) and ADR-035 (no dice)
- **Depends on:** VISION §3, ADR-034 (perception from where he stands), ADR-047, ADR-051 (every act has its roots)

## Context

The maintainer, 2026-11-13: *"the cognitive abilities are fundamental on the pitch, as well as the physical and psychological ones"*; *"I would not want all the choices to be artificially imposed by percentages: I really want each player to have his own AI."*

Measured on the engine at `SIM_VERSION` 66: no choice is a die, but every option's worth is a formula with a weight of its own, set by hand (`loft_salience` 1.2, `clear_salience` 2.5, `carry_salience` 0.6, `challenge_salience_milli`, `offer_salience_milli`, `run_salience_milli`, ...). In the night of 2026-11-12/13 those weights were tuned by looking at aggregate statistics (too many corners: lower it; too few shots: raise it). The numbers came close to a real match, but the player did not think: the designer chose for him, one weight at a time. It is the deus ex machina VISION §3 forbids, moved from the dice to the weights.

## Decision

**A player chooses by imagining what each of his options would lead to, from what he perceives, and by judging the outcomes with the one measure football has. His cognitive characteristics are the quality of that thought.**

1. **He recognises his options** (ADR-047): the repertoire of acts (ADR-051) offers the phrases available in the scene he sees: the passes, the lofted passes, the shot to either post, the carry, the clearance, the challenge, the run, the offer, the mark.
2. **He imagines each one.** For every option he forecasts what would happen, with the same physics and the same Laws as the engine, but from *his* scene: where the ball would go and how fast, who would reach it first (his teammates and the opponents he sees, at the speed he believes they have), whether it would arrive, where it would leave him. The forecast is deterministic and limited by cause, never by a die: he imagines only the players he perceives (vision, ADR-034), as far ahead as he can (anticipation), and he misjudges others in proportion to how poorly he reads the game.
3. **He judges the outcomes with one measure**: how much nearer it brings his side to scoring, and how much nearer to conceding, if the ball is lost there. The measure is the same for every option and every player; it is a fact of football derived from the engine's own Laws and geometry (the chance a position gives, read from the goal's angle, the distance and the defenders between), not a weight per kind of option. His **risk management** decides how he weighs what he could gain against what he could lose.
4. **The cognitive characteristics drive the choice**, as the physical ones drive the body:

   | Characteristic | What it does in his thought |
   |---|---|
   | vision | how many of the players around him he sees, and how far |
   | game reading | how well he forecasts what the others will do |
   | anticipation | how far ahead he imagines |
   | decision making | how many options he compares, and how fast he decides |
   | risk management | how he weighs gain against loss |
   | creativity | whether he considers the options few would see |
   | concentration | how well he thinks as the match wears on (a dimmer that wears down) |

   `game_reading`, `decision_making` and `risk_management` exist; `vision`, `anticipation`, `creativity` and `concentration` enter with this ADR, each with its first consumer (ADR-051 §4). Body and psychology remain the two filters (VISION §3): the body limits what he can do, the psychology colours how he interprets it (aggressiveness, emotional control, discipline).
5. **The same for the off-ball acts** (ADR-049): pressing, holding, running in behind, offering, marking are options he imagines and judges with the same measure, from his place in the team.
6. **What stays in `balance/`**: physical facts (the ball, the bodies, the reference speeds), the Laws, and the very few values that define football itself (what a goal is worth against keeping the ball), each sourced or declared. The weights per kind of option are removed.
7. **Statistics are the test, not the target.** Shots, corners, goals and passes per match are measured to find what is wrong in how the players think or move; they are never matched by tuning a choice weight. Calibration moves to the characteristics' distributions (L3).

## Consequences

- The choice becomes recognisable at the level a fan sees it: the playmaker who sees the whole pitch and plays the pass nobody saw, the defender who only reads one move ahead, the striker who imagines the run before the ball is played. Twin tests compare two players identical but for one cognitive characteristic, in the same scene.
- Cost: every decision imagines a handful of options a few seconds ahead, for the carrier at his pace and for the others at theirs. The forecast must be coarse (arrival times along lines, not a full simulation) and bounded by `decision_making` and `anticipation`; it is measured before it is accepted.
- The order of the implementation, after acceptance: (a) the forecast (who reaches the ball first, along a pass, a shot, a carry, from his scene); (b) the measure (the chance a position gives, from the shot's own geometry and the defenders); (c) the carrier's choice migrated, the per-option weights of `shot_choice.json` removed; (d) the off-ball choice migrated, `offball.json`'s weights removed; (e) `vision`, `anticipation`, `creativity` and `concentration` enter; each step with its twin tests and the 30-seed measure.
- L0 closes after this ADR: it is the base L1 builds on.

## Alternatives considered

- **Keep the weights and tune them better.** Rejected: whatever the tuning, the designer chooses for the player.
- **A learned model (a neural network) per player.** Rejected: not readable (the manager could not be told why), and a trained model is a hidden weight table; determinism and transparency are the project's means (VISION §2).
- **Utility from a fixed table of positions (an external xG/xT grid).** Rejected as the source: the measure is derived from the engine's own geometry and Laws, so that what a player believes and what the match does are the same football.

## Dated notes

- **2026-11-13 — accepted** by the maintainer ("approvo, poi se ci sarà qualcosa da limare limeremo"). The implementation follows the order of the Consequences, (a) first.
- **2026-11-13 — the statistics are derived, never adjusted** (the maintainer: *"the statistics must not be fixed artificially, because they must be derived from the abilities of the footballers"*). §7 restated as a method. (1) A value in `balance/` is a physical or physiological fact with its source (the sports-science literature, the Laws), or a consequence of other values (computed, not repeated), or it is declared *arbitrary until L3* and left alone. (2) No value is chosen by looking at what statistic it produces in the engine: not a choice weight, not a physical constant. (3) What a player believes about football is derived from the engine's own football (the value of keeping the ball: how often a possession from there ends in a goal in the engine's matches). (4) When a statistic is far from the real one, the work is to find the cause that is missing or wrong in how the players perceive, think or move, and to derive it from their abilities; the number that came out wrong is not touched. (5) Calibration against real football happens at L3, on the distributions of the characteristics (IDEA-DESIGN-007), not on the engine's coefficients.
- **2026-11-13 — the player thinks in the categories of modern football** (the maintainer: *"use the terms of modern football: not position but occupation of space"*; *"always use the moon to widen the horizon"*). The words of `GLOSSARY.md` §0 are not only names: they are what the player perceives and what he judges. (1) **He reads the pitch in lines and spaces**: the opponents' lines (the last line, the midfield line, the first line of pressure), the spaces between them, the corridors (the wings, the half-spaces, the centre). (2) **The measure (§3) values space, not points**: what an option is worth is the chance quality of the space it reaches, and what it breaks on the way (a line passed is a line the opponents must recover); the expected threat of a zone is the same football measure read before the shot. (3) **Off the ball he occupies space** (§5): not a place in a grid but a relation, to the ball, to his teammates (width, depth, support, spacing: not two in the same corridor and line) and to the opponents' lines (between them, behind them, on their shoulder); without the ball, to close the spaces (compactness, covering, the diagonal). (4) **The match has phases** the players recognise from the scene, not states the engine switches: build-up, progression, finishing; the defensive block high, mid or low; the transitions just after the ball changes side. The team's structure, given by the coach as an intention, is read by each player through these categories (VISION §3: psychology interprets, the body limits). Each enters with the step that consumes it: (1)-(2) with the carrier's choice (c), (3) with the off-ball choice (d), (4) with the coach's intention (the friendly you can play).

