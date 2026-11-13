---
title: "Vision — football as human behaviour, readable at every scale"
version: "0.2"
status: "draft"
last_updated: "2026-11-12"
conforms_to: "doctrine/00-foundations.md (v4.0) §2, §6, §9"
---

# Vision — football as human behaviour, readable at every scale

**What this is.** The direction of the project, in the maintainer's terms:
what we are building and the one idea that holds it together. The
doctrine (`00-foundations.md`) says what the engine must be; this page
says what it is *for*.

**What it is not.** Not a contract and not a plan. It traces a direction
("the wise man points at the moon"): it is read for where it points, not
word by word. If it contradicts the doctrine, the doctrine wins and this
page is corrected.

**Status.** Draft 0.2, written on 2026-11-12 from the maintainer's own
statement of the vision, expanded with the maintainer in conversation. It changes
step by step, as each part is discussed.

---

## 1. What we are building

A **deep football simulator**, written in Rust, with the structure of a
football manager. We start from the match, and only when the match holds
do we expand outward, to the season, the career, the world, until the
whole is a football manager. Not a copy of one: **better studied**.

Every aspect is refined one at a time. Nothing is added because a
commercial game has it; it is added when the simulation needs it and
when it can be explained.

## 2. Why determinism and a phenomenological engine

They are not the goal. They are the means to **clarity**.

- **Deterministic**: the same situation gives the same outcome. Nothing
  happens "because of a roll". What looks like luck is a cause not yet
  seen.
- **Phenomenological**: the engine models the *effect* of a decision on
  the pitch (a pass that is late, a defender who arrives a step behind),
  not the biomechanics of the body. The ball, which does not decide, has
  declared physics.

Together they make the game **understandable**: by the person who
builds it, and by the person who plays it. A player of Calcio Vero
learns football by playing, because every event has a reason that can
be found.

## 3. The one idea: the actor with his limits

**The action belongs to the actor, never to a deus ex machina.** A
footballer reads the situation from where he stands, with his own
perception, his own sight, his own mind, and he chooses. The engine
applies what he chose; it never chooses for him. The goalkeeper is the
same kind of actor as everyone else.

What the actor brings onto the pitch is a chain:

```
characteristics            innate, and developed in training
      |
      v
abilities                  what the characteristics let him do in a situation
      |
      v
the response               read, choose, act, from his point of view
      ^
      |  dimmed by
states                     fatigue, morale, concentration: the moment
```

The **coach gives every player a role**: an intention, not a script.
Zonal or man-to-man does not change the principle. The player carries
the role out through two different filters:

- his **psychology** shapes *how* he interprets it: the disciplined
  full-back holds his line, the restless one goes forward and leaves
  the gap;
- his **body** limits *how well* he carries it out: the slow one
  arrives late, whatever he intended.

This is how it happens in a real match, and it is the test every model
choice must pass: *would a fan recognise it?*

## 4. The same logic at every scale

The actor with his limits is not a rule of the match. **It is the unit of
the whole game**, and it repeats at every level of the nest:

| Scale | The actor | What he perceives, and his limits |
|---|---|---|
| The pitch | the footballer, the goalkeeper | the scene from his position; his sight, his reading, his state |
| The bench | the coach (you) | the match from the touchline; what he gives is an intention, interpreted by others |
| The scouting | the scout | a few matches, from one point of view, with his own bias: the estimate *is* someone's perception |
| The academy | the young player | the same chain as on the pitch: what is innate, what training develops, what the states allow |
| The world | clubs, agents, the board, the press | each with its own goals and its own limited view, you included |

So the uncertainty of scouting is the same thing as the limited sight of
a full-back, at another scale. The growth of a kid is explained by the
same reasons that explain a late pass. One logic, top to bottom: that is
what "better studied" means.

## 5. Every scale has its clock

The states that dim the abilities live on different times, one inside
the other:

| Clock | What moves on it |
|---|---|
| the tick | attention, concentration, the reading of the scene |
| the match | fatigue, morale that swings with the events |
| the week | training, recovery |
| the season | form, growth |
| the career | age, decline |

The structure of the repository is concentric in space (the levels);
the game is concentric in time as well.

## 6. What the manager does

A consequence of the above, not its starting point. The manager does
not set numbers: he **shapes intentions and develops people**, and he
does it with his own limits.

```
observe  ->  decide  ->  watch  ->  understand  ->  adjust
```

He observes what his players did (ratings from events, never from hidden
numbers), decides roles, line-ups, training and signings, watches the
match, understands why it went as it went, and adjusts because he
understood. He never sees the truth directly: no CA/PA, no star rating
that is the engine's input. He sees estimates, with how sure they are.

The soul is the one of the classic Italian managers, with the depth of
the modern ones: dense, readable screens, the radio commentary, a world
that reacts.

## 7. The question every new part must answer

Before anything is designed, at any level:

1. **Who is the actor here?**
2. **What does he perceive, and from where?**
3. **With which limits** (characteristics, abilities, states)?
4. **On which clock?**
5. **Would a fan recognise the result?**

A part that cannot answer is not ready to be built.

## 8. Where this points the work now

- **The match first**, until it reads as football: it is the smallest
  scale, and every other scale reuses its logic.
- **The role enters early**: a minimal form of the coach's intention is
  needed for the match itself, because it gives the players a purpose
  off the ball. It is not a career feature.
- **The states are the next great site of the engine**, once the match
  can be watched: fatigue, then concentration, then morale.
- **Characteristics are not fixed numbers forever**: the data model
  must allow the innate and the trained part, even before training
  exists.

## 9. What we do not compete on

Licensed data (players are fictional, the football is real), a 3D engine,
breadth at v1.0, a physics engine for the bodies. Not a delay: a choice.
See foundations §7.
