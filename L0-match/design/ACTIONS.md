---
title: "The acts on the pitch — what each one reads, and what the engine does today"
version: "0.1"
status: "draft"
last_updated: "2026-11-12"
conforms_to: "doctrine/adr/051-every-act-has-its-roots.md (accepted); doctrine/VISION.md §3; L0-match/design/CHARACTERISTICS-AND-ABILITIES.md"
---

# The acts on the pitch

**What this is.** The catalogue of ADR-051: every act a footballer or a
goalkeeper can perform, the quantities of the act, and the roots of each
quantity (the abilities of
[`CHARACTERISTICS-AND-ABILITIES.md`](CHARACTERISTICS-AND-ABILITIES.md)
and their characteristics). The last column is the honest state of the
engine (updated at `SIM_VERSION` 76; the repertoire test, `ds-match/tests/repertoire.rs`, checks that every act shows).

**How to read the state.**

| Mark | Meaning |
|---|---|
| ✓ | the act exists and its quantities read the actor's characteristics |
| ◐ | the act exists, some quantity is a constant or a rule of the engine |
| ✗ | the act exists but its main quantity ignores the actor |
| — | the act does not exist yet |

**For contributors.** A ✗ or — row is a task. Read ADR-051 first: every
change comes with a twin test (two actors identical but for one
characteristic: the difference must show).

Characteristics in *italics* do not exist in the engine yet; they enter
with their first consumer (ADR-051 §4).

---

## 1. The body (everyone)

| Act | Quantities | Roots | Engine today |
|---|---|---|---|
| Run, sprint | top speed, acceleration | max speed, explosive force, weight; fatigue dims | ✓ `resolve_movement` (`balance/movement.json`); measured: speed follows `max_speed` |
| Turn, change direction | time to turn, speed kept | *joint mobility*, body control, weight | ◐ since 78: he turns by braking (body control); the heading follows the velocity; joint mobility ✗ |
| Stop, decelerate | braking | body control | ✓ since 78: decel 3.5-6 m/s² by body control; he brakes to stop at his place |
| Jump | height, timing | explosive force, height, *joint mobility* | ◐ since 71: in the aerial duel, height plus explosive force; timing and joint mobility ✗ |
| Endure the 90 minutes | how fast fatigue rises, recovers | endurance | ✓ since 73: fatigue grows with the effort (speed squared) against endurance, recovers walking and at half-time (`balance/fatigue.json`); it dims acceleration and top speed; seen in the tick record; his thought still believes his pace fresh |

## 2. The eyes and the mind

| Act | Quantities | Roots | Engine today |
|---|---|---|---|
| Scan, look around | width of the cone, how often | vision, game reading | ✓ since 81: how often he glances, 0.2-0.8 a second by vision (Jordet et al. 2013); the cone is fixed |
| Read the play | what he notices | game reading | ✓ the visibility filter of the choice |
| Decide | how long he takes | decision making, concentration; crowding, threat, fatigue | ✓ `resolve_latency` (`balance/latency.json`); since 82 concentration slows it as he tires |

## 3. With the ball

| Act | Quantities | Roots | Engine today |
|---|---|---|---|
| Receive, control | clean or loose, the touch's length | first touch, foot, *body control* | ✓ since 55 he goes to meet the pass, takes it within his reach (reactivity), controls it by first touch, freedom and distance, and decides after his own time; foot ✗ |
| Carry | speed with the ball, direction | dribbling, run | ✓ since 80: the direction is his, towards goal or 35/70 degrees to either side into the space beside a man, each imagined (ADR-052); with the ball he keeps 75-95% of his sprint by dribbling |
| Dribble past a man | beat or lose | dribbling, explosive force, *joint mobility*, body control | ✓ since 72: when a defender goes in, the carrier may take him on, by what he imagines (ADR-052); won, the man is left behind (`beaten_ticks`), a `Dribble`; joint mobility ✗ |
| Shield | keep the ball under a challenge | maximal force, body control, first touch, *weight* | ✓ since 72: the carrier's other answer to a challenge, surer for the strong and balanced; weight ✗ |
| Short pass | choice, speed, precision | passing, foot, vision, game reading, risk | ✓ since 52: choice; weighed to arrive rolling (`kick.json`), capped by his kick; drift and overhit from passing and the foot |
| Long pass, switch, through ball | same, with height | passing, foot, explosive force, game reading | ✓ the long ball and the switch are the lofted pass (since 54); since 79 the through ball: into the space ahead of a teammate he sees attacking the depth, where they arrive together, worth who gets there first; offside judged at the pass |
| Cross | same | passing, foot | ✓ since 54: a lofted pass from a wide position into the area; met in the air (71) |
| Shoot | choice, direction, power, precision | *shooting* (`finishing` today), foot, explosive force, composure | ✓ since 52: speed from power and explosive force; the miss an angle from finishing, the foot, power and pressure; low or high by his choice, struck to reach that height (gravity); either post, the keeper hiding the side he covers (60); the miss's side from his body's momentum, standing from his foot's side (placeholder); composure ✗ |
| Head (attack) | reach, direction | heading, jump, *courage* | ✓ since 71: in the opponents' area the winner of the ball in the air heads at goal (shot type `header`, 76); courage ✗ |
| Clear | distance, direction | passing, explosive force | ◐ since 59 his choice near his own goal under pressure (pressure, distance, appetite); his strongest kick (explosive force), lifted, towards his wing; no error yet |

## 4. Without the ball

| Act | Quantities | Roots | Engine today |
|---|---|---|---|
| Hold the place | where, how strictly | game reading, discipline | ✓ the choice (ADR-049); since 83 how much his place weighs reads discipline; the place is the block (`balance/block.json`) and the coach's intention |
| Press | whether, how hard | game reading; *aggressiveness*, *work rate* | ◐ the choice reads proximity, rank and game reading |
| Challenge, tackle | whether, how, win, lose, loose | tackling, explosive force, maximal force, body control, aggressiveness vs first touch, explosive force, maximal force, body control | ◐ since 63 (ADR-042): standing, slide or jockey by his choice; won, kept or loose by the bodies; the foul (Law 12) is next; the approach angle is not read yet |
| Intercept | reach on the line | game reading, reactivity, sprint | ✓ his reach for a passing ball reads reactivity (55); since 83 stepping into the passing lane in front of his man is his choice |
| Block a shot or pass | reach | reactivity, first touch, *courage* | ✓ a shot is blocked by an outfield defender it passes within reach of (58); since 84 a pass reaching an opponent is controlled by his first touch or only deflected; stepping into the line is his choice (83); courage ✗ |
| Mark | whom, how close | game reading, *concentration*, *discipline* | ◐ since 55: the nearest defender who reads the game marks the nearest other opponent, goal-side; concentration and discipline ✗ |
| Cover, recovery run | when, where | game reading, work rate, run | ✓ since 83: with a teammate pressing, he covers the space behind him on the carrier's line to goal; out of possession he runs back at the pace his side needs, by his work rate |
| Offer for a pass | where, when | game reading, decision making | ✓ chosen when marked near the ball (53); since 84 he moves away from his marker to where the line from the carrier is most open |
| Run in behind | when | game reading, sprint | ◐ since 53: chosen ahead of the ball near the last defender's line with room behind (max speed, game reading); he stops on the line (no offside law yet, issue #13); the through ball is not a pass kind yet |
| Aerial duel (defend) | win or lose | heading, jump, maximal force, height | ✓ since 71; since 76 a pass in the air is contested by the opponent nearest where it comes down, and players meet a high ball at its drop point; about 3 a match (real: many more, step 10) |

### 4.1 The duel: the target form (maintainer, 2026-11-12)

Today the duel is an instant comparison of two weighted sums (`balance/duel.json`): the same two players give the same result in every scene, the carrier chooses nothing, and the outcome is only "kept" or "won". The target form:

- **two choices that meet.** The defender chooses how to go in (standing tackle, slide, jockey, wait for the mistake), each with its risk: a slide wins more often, and if it misses he is out of the play and may foul. The carrier chooses how to answer (shield with the body, take him on, release the ball first);
- **the contact is decided by the bodies in the scene**: approach speed and angle, who is between the ball and the opponent, *maximal force*, *body control*, weight, the fatigue of the moment;
- **four outcomes**: ball kept, ball won, ball loose, foul.

The variation comes from the situation, never from a die. Twin tests: the better shielder keeps the ball more, the better tackler wins it more, and it shows in the 2D. Where this differs from ADR-042 (proposed), ADR-042 is amended first.

## 5. In goal

| Act | Quantities | Roots | Engine today |
|---|---|---|---|
| Position (the angles) | where, relative to ball and goal | game reading, *concentration* | ✗ a rule (`balance/keeper.json`); since 51 always between the ball and the goal; game reading not read yet |
| Get set | ready or not when the shot leaves | *reactivity*, *concentration* | — |
| Dive, stop the shot | reach in the time the ball takes | reactivity, explosive force, *joint mobility*, height | ✓ since 51: reaction time from reactivity, dive from explosive force, met where the ball passes him (`balance/save.json`); joint mobility not yet |
| Catch or parry | held, parried, spilled | *handling* (`goalkeeping` today), *body control* | ✓ since 76: held when his hands reach the difficulty, the reach margin and the ball\'s speed added (before they multiplied and every ball was held); about 60% held; parried nearer him, tipped at the edge; no spill yet |
| Come out | whether | decision making, *courage* | ◐ the choice exists (ADR-049), only inside his area |
| Claim a cross | whether, win | decision making, jump, *handling* | ◐ since 71: the keeper in his area catches the ball in the air within his reach (arms); the choice to come is not his yet |
| Distribute | hands, feet | passing, foot, vision | ◐ as an outfield pass |

## 6. Restarts

| Act | Quantities | Roots | Engine today |
|---|---|---|---|
| Kick-off, throw-in, goal kick, corner | who takes it, where it goes | role, passing, set pieces | ✓ since 74: the keeper takes his goal kick, the best deliverer near the ball the corner, the nearest the throw-in; the opponents 9.15 m away; the ball is then the taker's choice, unchallenged until he plays it |
| Free kick, penalty | who takes it, shot or pass | finishing, set pieces, foot, *composure* | ✓ since 74: the best kicker near the ball takes it; a penalty is only a shot, everyone else outside the area and behind the mark (Law 14), the shooter aiming inside the post by the miss he expects of himself; composure ✗ |

---

### 6.1 The referee

**The referee is not an actor** (maintainer, 2026-11-13; ADR-021 note): he is the Laws, applied always objectively, to the facts of the match. He is **present** (shown on the pitch, a projection for the renderer) but **without occlusion** (not a body: the ball and the players pass through him, he hides nothing). No perception, no strictness, no error.

| Law | Engine today |
|---|---|
| Kick-off, throw-in, goal kick, corner (Laws 8, 15-17) | ✓ `rules.rs`, exact |
| Goal (Law 10) | ✓ the ball over the line between the posts |
| Offside (Law 11) | ✓ since 62, exact: `rules::offside_position` at the pass, the offence when he plays the ball, a free kick |
| Fouls (Law 12) | ✓ since 64: the contact's band (slide, from behind, speed, temper, technique); free kick, or a penalty in the area |
| Advantage | ✓ since 64: kept ball in the opponents' half, outside the area |
| Cards | ✓ since 64: reckless yellow, excessive force and denial of an obvious chance red, second yellow red; the sent-off player leaves the pitch |
| The referee on the pitch | — not drawn yet |

## 7. The order of work (ADR-051 §5)

**Every act is in every match, in potential.** The work does not add acts
one at a time: a first pass gives every row of this catalogue its
minimal real form (a candidate in its scenes, quantities that read their
roots), family by family, until the repertoire test is green (every act
appears at least once in the golden range of seeds). The families of the
first pass:

1. **In goal** — position by reading, get set, dive, catch / parry / spill, come out, claim a cross, punch, distribute (today's biggest error: the save).
2. **The kick** — power and the foot for every kick; the pass in its kinds (short, long, through ball, switch, cross), the shot, the clearance.
3. **The body** — turn and change of direction, stop, jump; fatigue, the first dimmer.
4. **In the air** — header in attack and in defence, the aerial duel.
5. **With the ball under pressure** — shield, dribble past a man, the duel in its situation.
6. **Without the ball** — intercept by reach, block, mark, cover, offer for a pass, run in behind.
7. **Restarts** — the taker chosen, throw-in, corner and goal kick played as kicks.

Then the depth, act by act, by what a fan sees wrong first.
