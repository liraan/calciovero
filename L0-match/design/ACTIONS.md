---
title: "The acts on the pitch — what each one reads, and what the engine does today"
version: "0.1"
status: "draft"
last_updated: "2026-11-12"
conforms_to: "doctrine/adr/051-every-act-has-its-roots.md (proposed); doctrine/VISION.md §3; L0-match/design/CHARACTERISTICS-AND-ABILITIES.md"
---

# The acts on the pitch

**What this is.** The catalogue of ADR-051: every act a footballer or a
goalkeeper can perform, the quantities of the act, and the roots of each
quantity (the abilities of
[`CHARACTERISTICS-AND-ABILITIES.md`](CHARACTERISTICS-AND-ABILITIES.md)
and their characteristics). The last column is the honest state of the
engine at `SIM_VERSION` 50.

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
| Turn, change direction | time to turn, speed kept | *joint mobility*, *body control*, weight | — the heading changes instantly for everyone |
| Stop, decelerate | braking | explosive force, *body control* | — |
| Jump | height, timing | explosive force, height, *joint mobility* | — |
| Endure the 90 minutes | how fast fatigue rises, recovers | *endurance* | — fatigue has no producer |

## 2. The eyes and the mind

| Act | Quantities | Roots | Engine today |
|---|---|---|---|
| Scan, look around | width of the cone, how often | *vision*, game reading | ◐ fixed cone (`balance/scanning.json`), no vision |
| Read the play | what he notices | game reading | ✓ the visibility filter of the choice |
| Decide | how long he takes | decision making; crowding, threat | ✓ `resolve_latency` (`balance/latency.json`) |

## 3. With the ball

| Act | Quantities | Roots | Engine today |
|---|---|---|---|
| Receive, control | clean or loose, the touch's length | first touch, foot, *body control* | ✓ reception reads first touch; foot ✗ |
| Carry | speed with the ball, direction | *dribbling*, run | ◐ speed is the run's; step length constant; direction is a rule (to goal, or to the open pitch when blocked) |
| Dribble past a man | beat or lose | *dribbling*, acceleration, *joint mobility*, *body control* | — |
| Shield | keep the ball under a challenge | *maximal force*, *body control*, weight | — (the carrier's side of the duel reads first touch) |
| Short pass | choice, speed, precision | passing, foot, vision, game reading, risk | ◐ choice ✓, speed from passing ✓, precision and foot ✗ |
| Long pass, switch, through ball | same, with height | passing, foot, explosive force, game reading | — one kind of pass only |
| Cross | same | passing, foot | — |
| Shoot | choice, direction, power, precision | *shooting* (`finishing` today), foot, explosive force, composure | ◐ choice and direction ✓, deviation from finishing ✓; **speed ✗ fixed** (30 or 15 m/s, set by whether it is on target); power chosen but unread; foot ✗ |
| Head (attack) | reach, direction | *heading*, jump, courage | — |
| Clear | distance, direction | passing, explosive force | — |

## 4. Without the ball

| Act | Quantities | Roots | Engine today |
|---|---|---|---|
| Hold the place | where, how strictly | game reading, *discipline* | ◐ the choice ✓ (ADR-049); the place is the block rule (`balance/block.json`) |
| Press | whether, how hard | game reading; *aggressiveness*, *work rate* | ◐ the choice reads proximity, rank and game reading |
| Challenge, tackle | whether, win or lose | tackling, explosive force vs first touch | ◐ ✓ roots; the same two players give the same result in every scene |
| Intercept | reach on the line | game reading, *reactivity*, sprint | ✗ fixed 2 m radius |
| Block a shot or pass | reach | *reactivity*, *courage* | — |
| Mark | whom, how close | game reading, *concentration*, *discipline* | — |
| Cover, recovery run | when, where | game reading, *work rate*, run | — |
| Offer for a pass | where, when | game reading, decision making | — |
| Run in behind | when | game reading, sprint | — |
| Aerial duel (defend) | win or lose | *heading*, jump, *maximal force* | — |

## 5. In goal

| Act | Quantities | Roots | Engine today |
|---|---|---|---|
| Position (the angles) | where, relative to ball and goal | game reading, *concentration* | ✗ a rule (`balance/keeper.json`) |
| Get set | ready or not when the shot leaves | *reactivity*, *concentration* | — |
| Dive, stop the shot | reach in the time the ball takes | *reactivity*, explosive force, *joint mobility*, height | **✗ standing reach only** (half his height × `goalkeeping`): about every shot on target is a goal |
| Catch or parry | held, parried, spilled | *handling* (`goalkeeping` today), *body control* | — a stopped shot is always held |
| Come out | whether | decision making, *courage* | ◐ the choice exists (ADR-049), only inside his area |
| Claim a cross | whether, win | decision making, jump, *handling* | — |
| Distribute | hands, feet | passing, foot, vision | ◐ as an outfield pass |

## 6. Restarts

| Act | Quantities | Roots | Engine today |
|---|---|---|---|
| Kick-off, throw-in, goal kick, corner | who takes it, where it goes | role, passing, *set pieces* split | ✗ the taker is placed by the rules, the ball plays on as an open pass |
| Free kick, penalty | — | *shooting*, foot, composure | — (no fouls yet) |

---

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
