# ADR-119 — The throw-in is thrown (DD-99)

- **Status:** Accepted (delegated, 2026-11-27) — da rivedere da parte del maintainer
- **Date:** 2026-11-27
- **Decision:** DD-99
- **Depends on:** ADR-039, ADR-040, ADR-051, ADR-062, ADR-080, Law 15, Law 10
- **Amends:** ADR-080 §Decisione (il primo atto del battitore di una rimessa è il lancio, non un calcio qualsiasi)

## Il problema

T17 dell'audit (`context/plans/CODE-AUDIT.md:179`) elenca fra i difetti che
cambiano la partita: «la rimessa è calciata; gol diretto da rimessa».
`resume_restart` consegna la palla al battitore (`set_pieces.rs:208-226`) ed
egli entra nel normale ciclo decisionale: `build_choice` gli offre anche
`Shoot`, `Clear`, `Carry` e `Hold`; il passaggio usa il piede, perché `throw`
nasceva solo da `from_hands`, cioè solo dal portiere (`thought.rs:474`). La
rimessa veniva quindi calciata e poteva entrare direttamente: la Regola 15
vieta entrambe le cose.

## I filtri

- **partita vera:** il dito è il gesto del lancio, con le due mani, desde la
  linea laterale; la luna è lo stesso battitore che legge i compagni e sceglie
  **a chi** lanciare. Nessun giocatore vero tira in porta da una rimessa, né
  la calcia.
- **LUNA:** la Regola 15 fissa *quale gesto* è lecito al primo tocco, non
  *quale compagno* scegliere: l'agency resta tutta nella lettura del ricevitore
  e del suo movimento (ADR-040, ADR-052). Non è un clamp sul COME: è una
  regola del gioco, come il calcio di rigore di ADR-070 e la punizione
  indiretta quando ci sarà. preferences-010 vieta le scelte imposte, non le
  Leggi.

## Decisione

1. `PerceivedScene` porta `throw_in`, letta da `restart_pass` come già fa
   `penalty_kick` per la Regola 14. In `build_choice`, con `throw_in` il
   battitore vede solo le opzioni di passaggio (`Pass`, `Loft`, `Through`):
   può scegliere chi riceve, non può tirare, spazzare, condurre né fermare
   la palla.
2. Il passaggio della rimessa usa `throw = true` in `resolve_pass`: la
   velocità e l'abilità delle mani sono già dichiarate in `kick.json`
   (`throw_speed_cm_per_tick`, `throw_max_cm`, il gesto del portiere), senza
   numero nuovo e senza taratura (A5-bis). La distanza di 2 m resta fuori,
   come da T18/ADR già chiuso.
3. `Pending` porta `throw_in`: un pallone lanciato dentro la porta senza che
   un altro giocatore lo tocchi non è gol (Regola 15) ma uscita di fondo —
   rinvio o angolo secondo l'ultimo tocco, già calcolato da
   `rules::out_of_play` (Regola 10). Nessun evento o banda nuova.

Le altre riprese restano come sono: ADR-080 governa la palla morta e la
scelta del battitore, SIM 218 racconta una sola volta il calcio di ripresa,
la distanza del muro e il fuorigioco non cambiano.

## Conseguenze

- Gemello in `read.rs` (`a_throw_in_is_only_a_throw_to_a_mate`): stessa scena,
  stesso attore; a palla viva ha anche il tiro, a rimessa solo il lancio a un
  compagno.
- Gemello in `ball.rs` (`a_direct_throw_into_the_goal_is_not_a_goal`): lo
  stesso attraversamento del piano conta gol per un passaggio, rinvio per un
  lancio.
- `SIM_VERSION` +1, golden L0/L1 e semi rigenerati, voce in
  `CHANGELOG-SIM.md`, misura `ds-reference 42..101` in `L0-CALIBRATION.md`.
  Le righe fuori banda si indagano, non si tarano (A5-bis).

## Scartate

- **Interpretare T17 come "rendi le riprese palla in movimento".** Il testo
  dell'audit è nell'elenco dei difetti: sono da correggere, non da
  introdurre. Il comportamento opposto contraddice le Leggi 13-15.
- **Vietare il gol con un secondo controllo post-hoc in `ball.rs` senza
  distinguere il lancio.** Sarebbe un filtro sull'etichetta, non una regola
  letta dall'atto: il flag vive nel `Pending` del lancio.
- **Aggiungere una soglia o una finestra "lancio corto/lungo".** Nessuna
  fonte la dà e non serve: `kick.json` ha già la velocità del lancio.
- **Introdurre la punizione indiretta in questo cantiere.** L'ABI degli
  eventi è chiusa (`event.rs`):serve una decisione dottrinale separata, non
  un effetto collaterale della rimessa. T17 resta parzialmente aperto su
  quel punto e sul primo calcio delle altre riprese.
