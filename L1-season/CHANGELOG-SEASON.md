# Changelog — SEASON_VERSION

This file records every bump of `SEASON_VERSION` (`L1-season/crates/ds-season/src/lib.rs`), the version of the **season's behaviour** (ADR-113 §8). It is separate from `L0-match/CHANGELOG-SIM.md`: a change of the match engine is a `SIM_VERSION`, a change of a season plugin that makes the same seed and choices give another season is a `SEASON_VERSION`.

**Rule:** if a season comes out different for the same seed, the same plugins and the same choices, `SEASON_VERSION` is bumped and an entry is added here before the change is merged.

---

## 3 — the body between matches (DD-67)

`ds-body` (recommended, may be switched off): every man's fatigue and muscle damage at the final whistle (the engine's new observed `MatchResult.bodies`) are carried to the next kick-off, where the matchday gives them to the engine as his starting state (decision AB, ADR-067); each morning they recover by the share of what is left that the sources give (`L1-season/balance/body.json`: Nédélec et al. 2012, 72 h; Ispirlidis et al. 2008, 120 h; "recovered" read as 90%). Switched off, every man starts fresh. The small golden regenerated.

## 2 — the coach lines up his side (DD-66)

ADR-114: every club's coach is an actor; before the day's matches he announces the line-up (`LineupAnnounced`): for every place of the 4-4-2 and of the bench the man of his squad who fits the role best (first what he lacks against the role's profile, then what he has beyond it), and the plan from his game idea (the block, the width), which the matchday gives the engine as the coach's intention from the kick-off. The small golden regenerated.

## 1 — the season core (DD-64)

The core `ds-season` (ADR-113): a season advances a day at a time and calls its plugins at `DayStarted`, `BeforeMatches`, `AfterMatch` (once per `MatchPlayed` of the day, in the order told), `DayEnded`, and once at `SeasonEnded`; the plugins in the order of the shared validator (`ds-plugin-api`); the save with its manifest and every plugin's state under its name; the hash of the events as the season's fingerprint. Then the first football plugins, still version 1 (nothing released before them): `ds-world` and `ds-roster` (the invented clubs and their squads), `ds-calendar` (a double round robin, a matchday a week), `ds-matchday` (every match played by the engine through `simulate_keyed`), `ds-standings` (3-1-0, goal difference, goals scored). The small golden: a league of 4 clubs, 12 matches.
