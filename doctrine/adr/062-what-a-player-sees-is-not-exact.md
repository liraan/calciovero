# ADR-062 — What a player sees is not exact

- **Status:** accepted (the form; the size of the error waits for its source, see §5)
- **Date:** 2026-11-15
- **Supersedes:** —
- **Amends:** ADR-034 (perception: the cone and the scan, now also the error of what is seen), ADR-052 (he imagines from what he sees: now from what he sees with its error)
- **Depends on:** `00-foundations` §4 ("perception is derived, not noisy by hash: the error of perception is a function of the characteristics, the distance, and the angle"), ADR-035 (no dice), IDEA-PERCEPTION-004

## Context

The calibration of 2026-11-15 (SIM 107-111, ds-reference, seeds 42-101) found one root under two distances from real football: offsides 1.1 against 4.3 a match (the passer reads the line perfectly) and throw-ins 5.6 against 42.7 (the players foresee every flight and meet nearly every ball before it goes out). Every player sees every position, and every ball's path, exactly: the engine has the cone and the scan (ADR-033, ADR-034) and no error of what is seen (`perception.rs`; the perception delay MD-40 was never applied). The doctrine prescribes that error. The maintainer approved the recommendation of IDEA-PERCEPTION-004 on 2026-11-15 (*"approvo"*).

Partita vera. *How do the actors recognise it?* A passer plays the runner who is half a yard off; a defender misjudges where a long ball comes down and it bounces over him; an assistant referee errs on about a fifth to a quarter of the offside situations he judges, more often flagging a player who was onside (Oudejans et al. 2000, *Nature*: 40 errors in 200; Helsen, Gilis and Weston 2006, *J. Sports Sci.*: 26.2% of 337 at the 2002 World Cup). *What does it depend on?* How far, how much to the side of his gaze, how well he sees (vision), how tired his attention (concentration). *Who produces it?* The player, perceiving.

## Decision

1. **Every player perceives positions and flights with an error**, deterministic, never a die (ADR-035): the same scene gives the same misreading.
2. **Its size grows with the distance and the angle from his gaze**, and shrinks with his vision; his concentration, as it wears down, widens it (ADR-052 §4).
3. **Depth is read worse than the side**: along his line of sight the error is larger than across it, as the eye judges distance less well than direction.
4. **Its direction comes from the geometry of the view** (the line of sight), not from a draw; how a moving object is extrapolated (the flash-lag effect, Nijhawan 1994) is part of the open question of §5.
5. **The size is not chosen.** No value enters `balance/` until it is read from a source: the tables of Oudejans et al. 2000 and Helsen et al. 2006 (errors by the observer's position and the players' separation), the visual delay of the flash-lag literature (of the order of 70 ms in Nijhawan's account, to be verified in the original), and the outdoor egocentric distance studies (Loomis et al. 1992; Durgin and colleagues). Choosing it by looking at the offsides or the throw-ins is forbidden (RULES A5-bis).

## Consequences

- The offside line and the drop of a ball become misjudged by the players' own sight; the distances of offsides and throw-ins are re-measured after, never targeted.
- Twin test: two players equal but for vision, the same scene, the poorer one misjudges a far position by more.
- Implementation waits for §5; until then the engine keeps the exact perception, declared as the cause still open (STATE: OI-98, OI-102).

## Alternatives considered

- **An arbitrary size now.** Rejected: any value chosen without a source would be chosen by what it does to the statistics.
- **A margin on the offside line only.** Rejected: a rule for one statistic, not the sight that causes several.

## Dated notes

- **2026-11-15 — the first part implemented** (`SIM_VERSION` 113), from the source read (Oudejans et al. 2000, Nature 404, 33, Table 1 and text): the offside line is read as the eye projects it, through the second-last defender along the passer's line of sight. The source gives a model, not a size: two men look in line when they stand on the same line of sight; level with the line the reading is right, off it the error grows with how far apart across the pitch they are; and it holds 'regardless of the quality' of the observer, so no twin on vision for this part. Looking along the depth the projection says nothing: the depth misjudged, the flights and the effect of vision and concentration (§2) remain open, waiting for their source.
