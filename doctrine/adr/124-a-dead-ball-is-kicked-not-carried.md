# ADR-124 — La palla morta si calcia, non si conduce (DD-104)

- **Status:** Accepted (delegated, 2026-11-28) — da rivedere da parte del maintainer
- **Date:** 2026-11-28
- **Decision:** DD-104
- **Depends on:** Laws 8, 13-17, ADR-080, ADR-119, ADR-052, ADR-062
- **Amends:** ADR-080 §Decisione e ADR-119 §Decisione (il primo atto del battitore delle altre riprese)

## Il problema

T17 dell'audit chiudeva la rimessa (SIM 226 / ADR-119) e la punizione
indiretta (SIM 229 / ADR-122), ma lasciava aperto il primo atto delle altre
riprese. `restart_pass` dice al motore che la palla è morta e a `OffBallScene`
(`dead_ball`, ADR-080) che nessuno la pressa; la scena del battitore, però,
non lo sapeva. `PerceivedScene` aveva solo `penalty_kick` e `throw_in`.
Il battitore di rinvio, angolo, punizione diretta e calcio d'inizio aveva
quindi le stesse opzioni di un portatore a palla viva: `Advance`, `Carry`,
`Hold` e `Shift`. Con `hold = restart_pass.is_some()` il pallone resta sotto il
corpo e segue l'uomo (`spatial.rs`): il calcio d'inizio veniva condotto, non
calciato.

Sonda temporanea in `thought.rs` (rimossa, nessun file del motore toccato nel
commit) sui semi 42..101: 1291 prime azioni non-calcio, 21,5 a partita; il
calcio d'inizio era `Carry` in 60 partite su 60, le punizioni dirette 50
`Advance` e 32 `Hold`, i rinvii 23 `Hold` e 19 `Advance`. Il conteggio non era
in difetto: era una regola del gioco che la scena non conosceva.

## I filtri

- **partita vera:** una ripresa comincia con un calcio o un lancio; nessuno
  può correre con la palla ferma, proteggerla o fermarla prima del primo
  tocco. Il calcio d'inizio, l'angolo e la punizione diretta possono invece
  entrare direttamente in porta: la Legge 10 vieta solo il gol dal primo calcio
  della punizione indiretta, già chiuso dall'ADR-122. Le righe reali dei passaggi
  delle riprese non sono una regola e non diventano un obbligo di passaggio.
- **LUNA:** il dito è il pallone tenuto per il calcio e il compagno che il
  battitore vede; la luna è lo stesso battitore che, con le sue qualità e la
  sua lettura, sceglie COME calciare — a chi, in profondità, in area o in
  spazzata. Cambia solo il fatto che la palla è morta: non si tolgono scelte
  legittime, si escludono gesti che la regola del gioco non permette.

## Decisione

1. `PerceivedScene` ha `dead_ball: bool`; `thought.rs` lo deriva da
   `restart_pass.is_some()`, lo stesso fatto che già deriva `penalty_kick` e
   `throw_in`. È una percezione, non un ordine: il battitore vede la palla
   pronta per il calcio.
2. Con `dead_ball`, `build_choice` non offre `Advance`, `Carry`, `Hold` né
   `Shift`. Restano passaggi, tiri e spazzate, con le loro salienze di sempre:
   il battitore sceglie fra le opzioni che la Legge gli lascia.
3. Se non vede nessuna opzione, il fallback non è più `Advance`: calcia lungo
   verso il campo avversario con la distanza `clear_length_cm` già dichiarata
   in `kick.json`. Nessun numero, soglia, opzione od evento nuovo (A5-bis).
4. Il gemello `a_restart_taker_does_not_run_with_the_ball` mette lo stesso
   uomo nella stessa scena, con compagno in arrivo e scelta di appoggio sul
   piede forte: a palla viva ha conduzione, attesa e `Shift`; a palla morta ha
   solo i calci e il passaggio al compagno.

## Conseguenze

- `SIM_VERSION` 231, voce in `CHANGELOG-SIM.md`. La sonda dopo il gate non
  trova più conduzioni, attese o `Shift` su alcuna ripresa dei 60 semi.
- Golden L0 base/istruito/coppa e golden L1 rigenerati; semi (coppa, pareggio
  al 90′, primo calcio della serie di rigori) ritrovati con le scansioni
  one-off e aggiornati. Misura `ds-reference 42..101` in `L0-CALIBRATION.md`
  §231: 12 su 20 in banda, nessun valore scelto guardando la banda.
- T17 resta aperto solo per la frequenza dei passaggi nelle riprese: i gesti
  rimasti sono tutti legali e restano scelte del giocatore; una regola che
  obblighi al passaggio avrebbe bisogno di una fonte propria, non di una banda.
