---
title: "Vision — what the manager does, and why they choose this game"
version: "0.1"
status: "draft"
last_updated: "2026-11-12"
conforms_to: "doctrine/00-foundations.md (v4.0) §2, §6, §7"
---

# Vision — what the manager does, and why they choose this game

**What this is.** One page on the *experience*: who the player is, what
they do in a week of play, and the moments that make someone choose Calcio
Vero over a commercial football manager. The doctrine
(`doctrine/00-foundations.md`) says what the engine must be; this page says
what the engine is *for*.

**What it is not.** Not a contract and not a plan. If it contradicts the
doctrine, the doctrine wins and this page is corrected. The milestones are
in the roadmap (a proposal of 2026-11-12, not yet in the repository); the steps are in `process/SESSION-STATE.md`.

**Status.** Draft written by an agent on 2026-11-12 for the maintainer to
correct. Every line marked **[?]** is a guess that needs a decision.

---

## 1. In one sentence

You manage a football club in a world whose truth you never see: you watch,
you estimate, you decide, and when it goes wrong the game tells you *why*,
honestly.

## 2. Who you are

A manager, not a god. You do not know your players' numbers. You know what
you have seen them do, what your staff tell you, and how sure they are.
You are allowed to be wrong, and the game does not hide that you were.

## 3. The loop

A week of play, at the scope of v1.0:

```
observe  ->  decide  ->  watch  ->  understand  ->  adjust
   ^                                                   |
   +---------------------------------------------------+
```

| Step | What the manager does | Source |
|------|-----------------------|--------|
| **Observe** | Reads match events, ratings from events, scout reports with their band | pillars 7, 9, 10 |
| **Decide** | Picks the eleven, sets the tactic (a modulation of the shape, never a script), trains, buys, sells, promotes a kid | OI-27, ADR-016 |
| **Watch** | Follows the match in the 2D or the radio text; pauses, changes instructions, substitutes **[?]** | IDEA-INFRA-007, ADR-022 |
| **Understand** | Asks "why": why the goal was conceded, why the kid is not growing, why the valuation changed | pillar 5 |
| **Adjust** | Changes a decision because of what was understood, not because a number went up | — |

The loop is the game. A feature that does not feed one of the five steps
does not belong in v1.0.

## 4. Three moments that are ours

The things a commercial manager does not give, and the reason to choose
this one. Each one is a test the build must pass, not a slogan.

1. **"Why did I lose?" has a real answer.** After a match the game shows
   the chain: the characteristic, the relation, the act. "Their 8 pressed
   your 6 from his blind side four times; he lost the ball twice; the
   second was the goal." Not "your team played badly". Pillar 5, ADR-044.
2. **The scout can be wrong, and says how much.** A report is an estimate
   of an observation of the truth, with a band. You sign a player on a
   narrow band and he is what you were told; you sign one on a wide band
   and you gambled. The gamble was visible. Pillars 7, 9.
3. **The academy is a game inside the game.** A kid grows, or does not,
   for reasons you can read: minutes, training, the people around him.
   Pillar 6. **[?]** Is this the third moment, or is it the match itself
   ("a match that reads as football")?

## 5. The soul

From foundations §6: the depth of FM24, the soul of PC Calcio. Concretely
**[?]**:

- Text first. The radio commentary and the dense, readable screens are
  the primary presentation (ADR-022), not a fallback.
- The world reacts: the weekly sports show, real sackings, a crowd with a
  mood (v1.1, ADR-018).
- Owned: the formulas are public, the save replays from its seed, a
  number in `L0-match/balance/` can be changed and the change is visible in the
  next match.

## 6. What the manager never gets

- **The truth.** No CA/PA, no hidden scalar shown as a star rating, no
  attribute screen that is the engine's input. Plane A stays hidden.
- **Certainty without a cost.** Knowing more costs scouts, time, money.
- **A die to blame.** Nothing in the match is luck in the sense of a
  draw; what looks like luck is a cause the manager did not see yet.

The risk of this list is frustration. The answer is §4.1: a hidden truth
is fun only if the consequences are explained.

## 7. What we do not compete on

Licensed data, a 3D engine, breadth at v1.0 (20 roles, 10 formations,
5 nations: ADR-016), crowd and media at v1.0 (ADR-018). Not a delay: a
choice. See foundations §7.

## 8. How to use this page

- Every milestone is checked against §3 and §4, besides its own exit
  criteria: "which step of the loop does this feed?"
- A proposal that makes the engine more faithful but feeds no step of the
  loop waits.
- The page changes only when the maintainer changes it.

## 9. Open questions for the maintainer

1. Is the manager in the loop **during** the match (pause, instructions,
   hints: IDEA-INFRA-007), or only before and at the substitutions?
2. The third moment of §4: the academy, or the match itself?
3. How long is a season meant to take a player, in real hours? It decides
   how much is delegated (the assistant and skip-match: IDEA-STAFF-001).
4. Who is the first audience at M3: football-manager players, people who
   like simulations, modders, or contributors? The README speaks to all
   four.
5. Is the post-match "why" a screen, a narration (the radio, after the
   whistle), or both?
