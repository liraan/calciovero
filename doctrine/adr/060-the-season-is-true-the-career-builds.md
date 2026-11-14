# ADR-060 — The season is true, the career builds

- **Status:** accepted
- **Date:** 2026-11-14
- **Supersedes:** —
- **Amends:** ADR-016 (D4: one nation; D5: the match-day choices of every coach move from `L2-ai` to L1; a new L2 module for the manager), ADR-047 (the training that changes the latents is L2, not L1)
- **Depends on:** ADR-015, ADR-016, ADR-029 (keyed derivation), ADR-047, ADR-052, ADR-059
- **Blocks:** `L1-season/*`, `L1-world/*`, `L1-roster/*`, the L2 module specs

## Context

L0 is closing (`context/plans/L0-CLOSE.md`, step 10 and then 11). The maintainer, 2026-11-14, opened the discussion on L1 so that it is ready when L0 closes: no L1 code before L0 is closed (maintainer, 2026-11-13), but its doctrine can be written now.

The doctrine says what "the game runs" means at each level (`STRUCTURE.md`): at L1 *"you can play a league season. Standings, calendar, results"*; at L2 *"you can play a career"*. It does not say where the line between them is for the things that sit on it: training, the other coaches' choices, contracts, staff. Two accepted ADRs disagree with the line drawn here: ADR-047 says *"at L1 the training modulates the latents"*, and ADR-016 D5 gives the squad selection to `L2-ai`. ADR-016 D4 also fixed five nations.

The maintainer's answers, 2026-11-14:
- on the other matches of the season: they are played by the same engine (*"esatto"*), never by a quick simulation;
- on the nations: *"concentriamoci sul solo campionato italiano"*;
- on the format: *"formato reale (tutto derivabile da db) ma cambiando i nomi ovviamente"*;
- on L2: the list below, *"compresa la vita privata dell'allenatore dove può spendere il suo stipendio"*;
- on the proposals of this ADR (the L1/L2 line, the correction to ADR-047, the pyramid, the names, one nation, the manager's private life with caused effects): *"sì, tutte"*.

Measured on `main` (`SIM_VERSION` 103, release build, one core of the cloud workspace): one full match takes about 1.05 s (seeds 1-5: 1.02-1.06 s).

## Decision

### 1. The line

**L1 is a true season: the team is what it is, and the season puts it to the test.** **L2 is the career: the choices that change the club, the players and the manager over time.**

A thing is L1 if a season would not be true without it even for a manager who only picks his eleven. It is L2 if it exists to let someone build.

### 2. What L1 holds

- **Every match is played by the same engine.** No quick simulation, no result drawn from strengths: the other matches of the round are matches, with their own keyed seeds (ADR-029).
- **The bodies carry the season.** Fatigue and recovery between matches, injuries (OI-70), the days a player is out.
- **The rules carry the season.** Cards and suspensions, the calendar, standings with the real tie-breakers, promotions, relegations, play-offs, play-outs, the cup.
- **Every coach chooses for the match.** The eleven, the bench, the phase shapes and the in-match instructions (ADR-059), for the manager and for every other club. Without the other coaches choosing, the season is not true; this is not `L2-ai`, which keeps the club's long-term decisions.
- **The week prepares the match.** Training at L1 is recovery and preparation: it changes the state of the body (fatigue, sharpness), never a characteristic.
- **The world and the roster as data.** The nation, its real cities, the clubs, the stadiums, who plays for whom and until when (the contract end as a fact; its money is L2), the staff who exist (their judgement is L2).

### 3. What L2 holds

1. **Building the squad:** the market (transfers, loans, free agents, clauses, agents), contracts and wages, scouting (reports limited by the scout's competence), the youth sector (the under-19 side, the yearly intake, loans), the development of the players (the training that grows the characteristics, individual plans, age and decline, retirement).
2. **The club:** finances (budget, TV rights, sponsors, tickets, wage bill, economic rules), the board (objectives, trust, sacking), facilities (stadium, training ground, youth facilities), the staff as actors (assistant, fitness coaches, analysts, doctors, scouts — those who report in ADR-059).
3. **The people:** the dressing room (morale, relationships, hierarchy, leaders, promises), the press, the supporters (ADR-018 keeps the last two for v1.1).
4. **Tactics over time:** a team learns a shape week after week, so changing it costs; set-piece routines; the analysis of the opponent.
5. **The living world:** the other clubs' market and coaching changes, players who age, retire and are born, seasons that follow, history and records.
6. **The manager himself:** his career (reputation, offers, his contract, sacking, coaching licences) and his private life — his salary and how he spends it (house, car, holidays, investments), his family, his time and his energy. A new module, `L2-manager`.

### 4. The manager's private life has caused effects

The private life is not decoration, and it has no bonuses. Buying a car does not add morale: that would be a weight invented by the designer (ADR-052). Its effects are caused:
- **time:** the week has its hours; the hours at the training ground and the hours with the family are the same hours;
- **energy:** a tired manager reads the match worse;
- **reputation:** what he does is seen by the supporters and the press;
- **money:** a salary spent does not come back, and a bad investment weighs on whether to accept an offer.

How each of these reaches the game is decided in the `L2-manager` spec, under the same rules as everything else: a cause, a twin, no dice (ADR-035).

### 5. The world of v1: Italy only

One nation, Italy (amends ADR-016 D4). The format is the real one, held as data in the database and never written in the code:

| Competition | Real format (2025-26) |
|---|---|
| First tier | 20 clubs, 38 rounds |
| Second tier | 20 clubs, 38 rounds; play-offs from 3rd to 8th for the third promotion (none if 3rd leads 4th by 14 points or more); a play-out for the fourth relegation |
| Third tier | three groups of 20 (with the second teams of first-tier clubs); the group winners are promoted, the fourth promotion comes from a 28-club play-off |
| National cup | 44 clubs (20 + 20 + 4 from the third tier), a numbered draw from 1 to 44, single legs up to the semi-finals (two legs), a single final |

The groups, the play-off places, the 14-point rule and the draw are rows of data: a change of format is a change of data.

**The pyramid stops at the third tier.** The fourth tier is not played: it is a reservoir from which the promoted clubs are generated every season, and into which the relegated clubs leave.

**The names change, the cities stay.** Competitions, clubs, stadiums and players have fictional names (the competition names are trademarks, like the club names); the nation and its cities are real (ADR-016). Real names come only from the user's packs.

The European cups and the other nations are not in v1.

### 6. Speed

The same engine for every match makes speed a question of the engine, never of the football. At about 1.05 s a match on one core:
- **a round** of the three tiers is about 50 matches: under a minute on one core, a few seconds on several; it is played in the background while the manager lives his week;
- **a whole season at once** (a long skip, the 10-season golden) needs the parallel run and, later, a faster engine: the target of the foundations (a full season under 60 s) applies there.

The matches of a round are independent (each has its keyed seed), so they can run in parallel without touching determinism. An optimisation must never simplify the match.

### 7. Determinism

The same start and the same choices of the manager give the same season. A season is identified by its start (world seed and data), the manager's choices with their dates, and the in-match instructions with their ticks (ADR-059). The golden test of L1 is the one the foundations already name: 10 seasons, one state hash.

## Consequences

### Easier
- L1 can be specified now and coded as soon as L0 closes, without waiting for any L2 module.
- The season is true from the first L1 build: every result comes from a match.

### Harder
- L1 needs a match-day coach for every club, the manager's included, before any L2 AI exists.
- The engine's speed becomes a requirement: about 7 times faster on one core, or the parallel run, for a whole season under 60 s.

### Locked in
- No match of the world is ever decided without the engine.
- No training at L1 changes a characteristic.
- The format of the competitions is data.

## Amendments

- **ADR-047:** the sentence *"At L1 the training modulates the latents (the coach, the work), and the repertoire follows"* reads, from this ADR, at L2 (`L2-tactics`, the development of the players). At L1 the latents are as derived, and the training changes the body.
- **ADR-016 D4:** one nation, Italy, instead of five; the other four are not in v1.
- **ADR-016 D5:** the squad selection leaves `L2-ai` (it is the match-day coach of L1); `L2-ai` keeps the transfers and the long-term planning. A new module, `L2-manager` (the manager's career and private life), is listed; its v1 scope is decided with its spec.

## References

- `doctrine/STRUCTURE.md` (the levels), `doctrine/00-foundations.md` (the 10-season golden, the season under 60 s)
- ADR-015, ADR-016, ADR-018, ADR-029, ADR-035, ADR-047, ADR-052, ADR-059
- The 2025-26 formats: Serie B regulation summary (Virgilio Sport), Serie C 2025-26 promotions and play-offs (Sky Sport), Coppa Italia 2025-26 (Calcio e Finanza, the Lega Serie A regulation 2024-2027)

## Dated notes

- **2026-11-14 — accepted** by the maintainer (*"sì, tutte"*). Written while L0 is closing; no L1 code before L0 closes.
