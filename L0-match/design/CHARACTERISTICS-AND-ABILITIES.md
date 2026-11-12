---
title: "Characteristics and abilities — the tree, from the real match"
version: "0.1"
status: "draft"
last_updated: "2026-11-12"
conforms_to: "doctrine/VISION.md (0.2) §3; L0-match/specs/11-player-characteristics.md (L0.11) §2, §7"
---

# Characteristics and abilities — the tree, from the real match

**What this is.** A design note: the chain of `VISION.md` §3 made
concrete. It starts from what a fan sees in a real match and builds a
tree: the **characteristics** are the roots, the **abilities** are the
branches, the **states** dim the branches.

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
| **Cognitive** | vision, game reading, decision making, risk management |
| **Psychological** | consistency, resilience, emotional control, aggressiveness, courage, concentration (threshold), composure (threshold), ★ **discipline** (how closely he keeps to the role), ★ **work rate** (how much he works without the ball) |

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

## 4. The states: what dims the branches

The states dim the **branches**, never the roots: the player is the same,
his moment is not.

| State | What it dims |
|---|---|
| Fatigue | the **body** (arrives later, jumps less) and the **quality** (misplaces the easy pass); each formula reads it once (L0.11 §4.8) |
| Concentration of the moment | reading and perception: lapses, positioning errors |
| Composure of the moment | precision under pressure |
| Confidence | how much he risks |
| Morale | intensity and commitment |

## 5. Consistency without a die ★

L0.11 §4.7 applies the trait modulators by **sampling** from a
distribution with a deterministic random stream. That is a die, even if a
reproducible one, and it contradicts ADR-035.

From the real match: an inconsistent player does not draw a 9 or a 4. He
is one who **loses his composure more**: after a mistake, under
pressure, when tired. So consistency is the player's **sensitivity to
the states**: how much concentration and composure swing with the
events and with fatigue. The consistent player crosses the match almost
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
| 4 | **Consistency** as sensitivity to the states, not sampling | no die (ADR-035) |
| 5 | Concentration and composure listed with the psychological characteristics | L0.11 §7.3 already says their threshold is a trait; here they are explicit |
| 6 | "Ability" named as the formula layer | L0.11 has characteristics and formulas, but no name for the branch |
