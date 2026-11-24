# Changelog — SEASON_VERSION

This file records every bump of `SEASON_VERSION` (`L1-season/crates/ds-season/src/lib.rs`), the version of the **season's behaviour** (ADR-113 §8). It is separate from `L0-match/CHANGELOG-SIM.md`: a change of the match engine is a `SIM_VERSION`, a change of a season plugin that makes the same seed and choices give another season is a `SEASON_VERSION`.

**Rule:** if a season comes out different for the same seed, the same plugins and the same choices, `SEASON_VERSION` is bumped and an entry is added here before the change is merged.

---

## 1 — the season core (DD-64)

The core `ds-season` (ADR-113): a season advances a day at a time and calls its plugins at `DayStarted`, `BeforeMatches`, `AfterMatch` (once per `MatchPlayed` of the day, in the order told), `DayEnded`, and once at `SeasonEnded`; the plugins in the order of the shared validator (`ds-plugin-api`); the save with its manifest and every plugin's state under its name; the hash of the events as the season's fingerprint. No football plugin yet.
