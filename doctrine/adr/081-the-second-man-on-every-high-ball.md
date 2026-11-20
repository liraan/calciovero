# ADR-081 — The second man on every high ball (OI-102)

- **Status:** Accepted — DD-19, maintainer chose C on 2026-11-19
- **Date:** 2026-11-19 (accepted 2026-11-20, applied in SIM 164)
- **Decision:** which high balls a second man reads, and when
- **Depends on:** ADR-077, ADR-049, ADR-051
- **Supersedes:** nothing; it would amend ADR-077 §Decision

## Contesto

ADR-077 gave the second man (rank 1) a contest on a high ball, but its
implementation in SIM 160 restricted the reading to the **defending** side and
to a **pass in flight**. On 60 real-match-filtered seeds (42..101) at SIM 163
the engine still reports 10.82 aerial duels a match against 38.14 in Serie A
2017-18; the audit of SIM 149 found 66 aerial meetings a match, 61 won by one
man alone. The kicker's side stops reading its own lofted ball, and a
clearance, a header away or a loose ball never offers the second man at all.

An uncommitted trial extended the same perception-based rank test to every
high ball read by either side, including the fast recheck on the kick. It was
measured on the same 60 seeds: aerial duels 10.82 -> 12.12, goals 5.20 ->
4.52, shots 33.58 -> 31.75, but **still 6 of 22 metrics in band**. It also
removed every shot and cross by the two wide men in seeds 42..49 after the
30th minute, so the existing twin test `swapped_wide_men_strike_with_the_feet_they_have`
no longer has a sample. The trial was reverted; SIM 163 is intact.

## The model question

This is not a number to fit. It is a choice of what a player perceives:

1. **A — Kicker's side too, only aerial passes.** Remove ADR-077's
   `home != passing_home`, keep the pass-only trigger and the original
   recheck. Smallest change; leaves clearances and loose high balls to one
   man.
2. **B — Every high ball, both sides.** Treat the drop point of any ball a
   man sees coming down as contested by the first two men he sees near it,
   as in the trial. Explains the reference count better, but changes which
   side keeps the ball and starves the wide men of the sample that guards
   the foot decision (ADR-031).
3. **C — Every high ball, but the second man must reach it.** B with the
   reach test ADR-077 already states in words but the code never implemented:
   he goes only if his own pace can get him to the drop point before it
   arrives. No new parameter — the race already exists in `thought.rs` and
   `movement.json`. This may restore the attacking sample without weakening
   the duel.

## Recommendation (agent)

**C**, with a twin that proves the second man reaches a near ball and holds
for a far one, and an A/B on the same 60 seeds. If C still removes the foot
sample, the choice is whether the foot test should be moved to a scene the
behaviour still produces (not weakened) or whether the model is wrong.

The maintainer chose **C** on 2026-11-19. Applied in SIM 164.

## Consequences of acceptance

- `SIM_VERSION` bump, golden regenerated, `CHANGELOG-SIM` entry.
- ADR-077 §Decision amended; this ADR moves to Accepted with its DD trailer.
- `L0-CALIBRATION.md` table and the OI-102 row updated from the new measure.
- The foot test stays at the same seeds and the same assertion; if it fails,
  the cause is reported, not the assertion relaxed.

---

## DD-19 — Chiusura (2026-11-20)

- **Scelta:** C. Ogni palla alta, entrambi i lati; il secondo va solo se può
  arrivarci, con la corsa e la lettura già esistenti. Nessun numero nuovo.
- **Attuazione:** `thought.rs` legge il drop point da entrambi i lati e il
  `Target` aereo porta il rank vero (portiere 0); `offball.rs` decide con la
  race sul passo del giocatore. SIM 164, golden rigenerati.
- **Prova:** gemello `the_second_reaches_the_near_ball_and_holds_for_the_far_one`
  (vicino 300 cm → `GoToBall`; lontano 8000 cm → `Hold`); test del piede
  `swapped_wide_men_strike_with_the_feet_they_have` preservato.
- **Esito:** nella tabella `L0-CALIBRATION.md`, aggiornata con la misura
  SIM 164 sulle stesse 60 partite 42..101.
- **Interazione (SIM 165):** da solo, SIM 164 abbassava la garanzia di
  pressione ADR-070 sotto la soglia (46 cm di scarto medio contro i 50
  richiesti da `more_pressing_closes_the_carrier_down`). L'attuazione
  dell'emendamento ADR-071 in SIM 165 l'ha ripristinata senza indebolire
  l'asserzione né accorciare i venti semi del test: le due cause sono
  accoppiate nel golden 165.
