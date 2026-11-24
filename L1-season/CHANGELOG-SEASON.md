# Changelog — SEASON_VERSION

This file records every bump of `SEASON_VERSION` (`L1-season/crates/ds-season/src/lib.rs`), the version of the **season's behaviour** (ADR-113 §8). It is separate from `L0-match/CHANGELOG-SIM.md`: a change of the match engine is a `SIM_VERSION`, a change of a season plugin that makes the same seed and choices give another season is a `SEASON_VERSION`.

**Rule:** if a season comes out different for the same seed, the same plugins and the same choices, `SEASON_VERSION` is bumped and an entry is added here before the change is merged.

---

## 7 — the coaches told, the bench from the rule, the tie-break (decision BM)

`ds-coach` (card version 2): on the first morning every club's coach is told (`CoachAppointed`: the name from the pack's new `coaches.json`, or from the seed; born and nationality if the pack says). The bench is the series' rule (`nation.json` `bench`: Serie A 15, Lega Serie A norms 2024-25, IFAB Law 3 "from three to a maximum of fifteen") up to what the engine can play (`MAX_BENCH`, 7): a keeper first, then the outfield roles in turn, each the best man left. At an equal estimate, the man who started least recently, then the lowest identity (ADR-114 §7). The small golden regenerated.

## 6 — discipline on the text of the CGS (decision BM)

The thresholds read on the FIGC Codice di Giustizia Sportiva (art. 9 c. 5, the text of 2 July 2024): a ban at the 5th, 10th, 14th, 17th and 19th caution, then at every further one. A yellow and a red to the same man in the same match is a second caution: its yellow does not count (read in the press, not on the text; declared). Tested with matches made by hand.

## 5 — the human seat and the coach who adapts (DD-70)

`ds-coach`: a club's coach may be the human player (`Coach::with_human`): his line-up for a day is played as given; an invalid one (a banned man, a man not in the squad, a man twice, a plan out of range) is refused with `LineupRefused` and why, and the vice's proposal is played; every `LineupAnnounced` says `by` (human, vice, actor). The AI coaches adapt: the block of their idea moves by a new root, `coach_adaptability`, against the opponent's strength (the points a match gap from the public table, over three); with none they keep their idea. The small golden regenerated.

## 4 — discipline: cautions, bans, who may not play (DD-69)

`ds-discipline` (recommended, may be switched off): the matchday tells every card of a match by the man's identity (`MatchPlayed`, field `cards`); the plugin counts the cautions over the season, bans for one match at each threshold of `L1-season/balance/discipline.json` (5 and 10 from the Giudice Sportivo's communiqués; 14 and 17 to verify on the text in force of the CGS) and for one match after a red card, serves a ban when the man's club plays, and each morning tells who may not play (`Unavailable`); the coach leaves them out. Tested: no banned man lines up. The small golden regenerated.

## 3 — the body between matches (DD-67)

`ds-body` (recommended, may be switched off): every man's fatigue and muscle damage at the final whistle (the engine's new observed `MatchResult.bodies`) are carried to the next kick-off, where the matchday gives them to the engine as his starting state (decision AB, ADR-067); each morning they recover by the share of what is left that the sources give (`L1-season/balance/body.json`: Nédélec et al. 2012, 72 h; Ispirlidis et al. 2008, 120 h; "recovered" read as 90%). Switched off, every man starts fresh. The small golden regenerated.

## 2 — the coach lines up his side (DD-66)

ADR-114: every club's coach is an actor; before the day's matches he announces the line-up (`LineupAnnounced`): for every place of the 4-4-2 and of the bench the man of his squad who fits the role best (first what he lacks against the role's profile, then what he has beyond it), and the plan from his game idea (the block, the width), which the matchday gives the engine as the coach's intention from the kick-off. The small golden regenerated.

## 1 — the season core (DD-64)

The core `ds-season` (ADR-113): a season advances a day at a time and calls its plugins at `DayStarted`, `BeforeMatches`, `AfterMatch` (once per `MatchPlayed` of the day, in the order told), `DayEnded`, and once at `SeasonEnded`; the plugins in the order of the shared validator (`ds-plugin-api`); the save with its manifest and every plugin's state under its name; the hash of the events as the season's fingerprint. Then the first football plugins, still version 1 (nothing released before them): `ds-world` and `ds-roster` (the invented clubs and their squads), `ds-calendar` (a double round robin, a matchday a week), `ds-matchday` (every match played by the engine through `simulate_keyed`), `ds-standings` (3-1-0, goal difference, goals scored). The small golden: a league of 4 clubs, 12 matches.
