---
title: "Characteristics and abilities — the tree, from the real match"
version: "0.2"
status: "draft"
last_updated: "2026-11-12"
conforms_to: "doctrine/VISION.md (0.2) §3; L0-match/specs/11-player-characteristics.md (L0.11) §2, §7"
---

# Characteristics and abilities — the tree, from the real match

**What this is.** A design note: the chain of `VISION.md` §3 made
concrete. It starts from what a fan sees in a real match and builds a
tree: the **characteristics** are the roots, the **abilities** are the
branches, the **dimmers** (the states of the moment) dim the branches.

**What it is not.** Not the contract. The contract is L0.11, ratified.
Where this note differs from L0.11 (marked **★**), an ADR amends L0.11
before any code changes. Until then this note is a proposal, corrected by
the maintainer branch by branch.

**Decided with the maintainer (2026-11-12).**

- An ability is **a formula, not a datum**: characteristics combined in a
  kind of situation, dimmed by the states, never stored.
- **Innate and trained**: the innate part is the base (the body, the
  predisposition); the trained part grows on top of it, from age,
  minutes played and the quality of training. No number fixes where the
  growth ends: that would be the potential (PA) under another name.
- The characteristics keep **the four families** of L0.11.

---

## 1. The roots: characteristics, in four families

A characteristic is hidden, stable over months, and it is the player
(L0.11 §2.1). The goalkeeper has no family of his own: he uses the same
roots, plus handling.

| Family | Characteristics |
|---|---|
| **Physical** | explosive force, maximal force, acceleration, max speed, endurance, joint mobility, reactivity, body control (balance and coordination). Visible body data, measured, not hidden: height, weight |
| **Technical** | first touch, passing, shooting, dribbling, heading, tackling, left foot, right foot, **handling** (the goalkeeper's hands; replaces the composite `goalkeeping`, OI-92) |
| **Cognitive** | vision, game reading, decision making, risk management, ★ **concentration** (how long and how well he holds his attention) |
| **Psychological** | *modulating traits:* consistency, resilience, emotional control, composure (threshold). *Dispositions:* aggressiveness, courage, ★ **discipline** (how closely he keeps to the role), ★ **work rate** (how much he works without the ball) |

## 2. The branches: abilities, by situation of play

An ability is what the characteristics let a player do in a kind of
situation. Its **sources** are one or more characteristics, from any
family; a characteristic can feed many abilities. The main source is in
**bold**. An ability can itself be the source of another (marked →).

### 2.1 Base movements (everyone, and the source of other abilities)

| Ability | Sources |
|---|---|
| Sprint | **acceleration**, explosive force, weight |
| Long run | **max speed**, endurance |
| Jump | **explosive force**, height, joint mobility |
| Sustained intensity (the 90 minutes) | **endurance** |
| Recovery between sprints | **endurance**, explosive force |

### 2.2 With the ball

| Ability | Sources |
|---|---|
| Receiving | **first touch**, foot, body control, vision (looking around before) |
| Short pass | **passing**, foot, vision, game reading, decision making |
| Long pass / switch / cross | **passing**, foot, explosive force, game reading, vision |
| Through ball | **game reading**, vision, passing, decision making, risk management |
| Dribbling | **dribbling**, acceleration, joint mobility, body control, courage |
| Carrying the ball | **dribbling**, → long run, body control |
| Shooting | **shooting**, foot, explosive force, body control, composure, decision making |
| Shielding the ball | **maximal force**, body control, weight |
| Attacking header | **heading**, → jump, game reading (timing), courage |

### 2.3 Without the ball

| Ability | Sources |
|---|---|
| Positioning / marking | **game reading**, concentration, ★ discipline, → sprint |
| Pressing | **aggressiveness**, ★ work rate, → sustained intensity, game reading |
| Tackling | **tackling**, game reading (timing), body control, aggressiveness, emotional control |
| Interception | **game reading**, vision, reactivity, → sprint |
| Aerial duel | **heading**, → jump, maximal force, courage |
| Runs off the ball | **game reading**, decision making, → sprint, → long run |
| Recovery runs / covering | **→ long run**, ★ work rate, game reading |

### 2.4 In goal

| Ability | Sources |
|---|---|
| Shot stopping (reflex, dive) | **reactivity**, joint mobility, explosive force, handling, height (reach) |
| Catching / parrying | **handling**, concentration, body control |
| Positioning (the angles) | **game reading**, concentration |
| One-on-one | **courage**, reactivity, decision making, explosive force |
| Coming for crosses | **decision making**, → jump, handling, courage |
| Distribution (hands and feet) | **passing**, foot, vision, game reading |

**The roots that are everywhere.** Game reading, body control,
explosive force and acceleration feed many branches. That is what
happens on a real pitch: "he reads the game" shows everywhere.

## 3. The psychological characteristics work on the other branches

Most of them produce no gesture of their own: they decide how much a
gesture holds (L0.11's distinction between producing and modulating an
effect). Composure does not make a pass; it decides how much the pass
holds when the pressing arrives. Discipline does not make a run; it
decides whether the player keeps to the role or leaves it.

## 4. The dimmers: a category of their own

The states of the moment are a separate category (maintainer,
2026-11-12, as football managers treat them): they are not the player,
they are **how the player is right now**, and they dim the abilities.
They dim the **branches**, never the roots. **Every dimmer has a root:
the characteristic that governs how fast or how wide it moves.** The
dimmers are of two kinds, because in a real match they move in two
different ways.

### 4.1 Dimmers that wear down with time: the match's duration

| Dimmer | How it moves | Governed by (root) | What it dims |
|---|---|---|---|
| Fatigue | rises minute by minute, faster after sprints and duels; part of it recovers at half-time | endurance (and recovery between sprints) | the **body** (arrives later, jumps less) and the **quality** (misplaces the easy pass); each formula reads it once (L0.11 §4.8) |
| Concentration of the moment | falls minute by minute, faster when tired; partly restored at half-time | concentration (cognitive) | reading and perception: lapses, late reads, positioning errors |

### 4.2 Dimmers that swing with events: the match's story

| Dimmer | How it moves | Governed by (root) | What it dims |
|---|---|---|---|
| Morale | up after a goal scored, down after a goal conceded or an injustice | resilience; consistency (how wide) | intensity and commitment |
| Confidence | up after a gesture that worked, down after a mistake | resilience; consistency (how wide) | how much he risks |
| Composure of the moment | falls under sustained pressure, after provocation, in decisive moments | composure (threshold), emotional control; consistency (how wide) | precision under pressure |

The first kind tells the **duration** of the match, the second its
**story**: the team that scores at the 60th and comes alive, the defender
who after the error does not win another ball. Both kinds are needed.

## 5. Consistency without a die ★

L0.11 §4.7 applies the trait modulators by **sampling** from a
distribution with a deterministic random stream. That is a die, even if a
reproducible one, and it contradicts ADR-035.

From the real match: an inconsistent player does not draw a 9 or a 4. He
is one who **loses his composure more**: after a mistake, under
pressure, when tired. So consistency is the player's **sensitivity to
the dimmers**: how wide the dimmers of §4.2 swing with the events. It is
the root that governs their amplitude. The consistent player crosses the match almost
unchanged; the inconsistent one rises and falls, and every time there is
a cause that can be read. The phenomenon stays; the die goes.

## 6. The two filters of the role, on this tree

The coach's role says *what* to do. The **psychology** (discipline,
aggressiveness, courage, risk management) decides *how* the player
interprets it. The **body** (the physical branches) decides *how well*
he manages it. This is why discipline and work rate are needed: without
them the first filter has no roots.

## 7. What differs from L0.11 (to be amended by an ADR)

| ★ | Change | Why |
|---|---|---|
| 1 | New psychological characteristic: **discipline** | the role's first filter (VISION §3) |
| 2 | New psychological characteristic: **work rate** | pressing, covering, the work without the ball |
| 3 | **Handling** replaces `goalkeeping` | `goalkeeping` is a composite; already planned (OI-92) |
| 4 | **Consistency** as sensitivity to the dimmers, not sampling | no die (ADR-035) |
| 5 | **Concentration** is cognitive (attention), **composure** psychological; the psychological family is split in modulating traits and dispositions | they fail for different reasons on the pitch: the lapse at the 70th is not the trembling at the penalty |
| 6 | "Ability" named as the formula layer | L0.11 has characteristics and formulas, but no name for the branch |
| 7 | The **dimmers** are a category of their own, in two kinds (wear down with time, swing with events), each governed by a root | L0.11 §7.6 has the states; here they get their dynamics and their root |

## 8. The way back: from real football to the characteristics (ADR-057)

Every characteristic must be read back from what a real player is seen to
do: one row per characteristic, with the acts that read it (`ACTIONS.md`),
its twin test, the real datum that reveals it and how directly
(**measured**: a body datum or what tracking data give almost directly;
**counted**: from event data, through a model; **judged**: seen only
through its effects). A source is cited, never invented; a row without
one says so. Reading back is an estimate (Plane C), never an extraction.

| Characteristic | Acts that read it | Twin test | The real datum that reveals it | Uncertainty |
|---|---|---|---|---|

*To be filled, one row per characteristic of §1, before step 10 of
L0-CLOSE closes (first action: the consumers and the twins from the code
with `grep`; then the sources).*
