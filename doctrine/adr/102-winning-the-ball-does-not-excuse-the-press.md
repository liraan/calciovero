# ADR-102 — Vincere il pallone non assolve la pressione (DD-44)

- **Status:** Accepted (delegated, 2026-11-22) — da rivedere da parte del maintainer
- **Date:** 2026-11-22
- **Decision:** DD-44
- **Attua:** OI-114 (proposta di grado 2, approvata dal maintainer il 2026-11-22)
- **Depends on:** ADR-021 (l'arbitro applica le Leggi), ADR-035 (nessun dado), ADR-039 (read/choose/contact/band), ADR-042 §4 (la banda del fallo), ADR-043 (l'attore decide)

## Decisione

Un contatto a terra in cui il difensore *preme* il portatore può essere
fallo anche se il difensore vince il pallone. Le radici restano le stesse del
fallo di contrasto — aggressività, controllo emotivo, tecnica, velocità
d'arrivo, entrata alle spalle, scivolata — e la Legge 12 resta applicata
dall'arbitro. Cambia la soglia: la pressione ha una soglia sua, derivata
dalla quota StatsBomb del 6,6% delle pressioni che finiscono in fallo, non
dalla fascia del contrasto.

Con i due filtri: il **dito** è il contatto, la distanza e l'angolo
d'arrivo, il tick, l'evento `Duel` e il `Foul` che ne segue; la **luna** è lo
stesso difensore che ha scelto *come* pressare e l'arbitro che, senza
guardare l'etichetta dell'atto, applica la Legge al contatto. Il giocatore
non riceve un ordine né un limite: può pressare, scivolare, restare davanti;
il fallo è la conseguenza del suo modo di farlo.

La soglia è un numero nuovo di grado 2, con fonte: 6,6% delle pressioni
(StatsBomb, Liga 2020/21, 20 partite: 20,6 falli su 314 pressioni). Sulle
partite del motore i contatti-pressione sono circa 173 a partita; il 6,6%
sono circa 11 falli in più. Sui contatti vinti dal difensore il band
dell'ADR-042 li taglia a 675 milli: è il valore che lascia fuori il 93,4%
dei contatti, scritto in `balance/duel.json` con la derivazione.

## Conseguenze

- `DuelBalance::foul` prende `ChallengeFacts::pressing` e sceglie fra
  `foul_threshold_milli` (800, il contrasto non vinto) e
  `foul_press_threshold_milli` (675, la pressione che vince). L'orchestratore
  distingue i due casi nello stesso `resolve_duel`: il fallo di contrasto
  resta dov'era, quello di pressione si aggiunge solo su un contatto vinto
  dal difensore e non loose, senza doppio conteggio.
- `jockey_pressure` in `ds-match` non butta più via `foul_committed`: se la
  banda della pressione arriva alla soglia, emette il contatto e passa i fatti
  a `apply_foul`, che chiama il cartellino, il vantaggio e la punizione di
  Law 12. Il portiere non è toccato: la soglia è di terra.
- Il gemello `a_press_that_wins_the_ball_can_still_foul` ha lo stesso contatto
  vinto e cambia solo il controllo emotivo: il più impulsivo fa fallo, il
  più calmo no; letto come contrasto, lo stesso band resta sotto la soglia
  del contrasto.
- Se i falli salgono, `knock.json` resta tarato sul vecchio conteggio e va
  rifissato con la procedura di ADR-066, non con un numero adattato alla
  metrica giudicata (A5-bis).
- I cartellini gialli salgono come conseguenza dichiarata della nuova
  popolazione di falli (il maintainer li ha tenuti fuori dalla decisione):
  si osservano nella misura, non si tarano.

## Scartate

- Dare il fallo al `Jockey` con la fascia del contrasto (prova SIM 134): chi
  resta davanti va piano e la banda non arriva mai a 800; nessun fallo in
  più. Per questo la soglia è sua e derivata dalla quota vera.
- Il duello per il pallone sciolto nello stesso tick: provato e tolto, non
  spiegava i falli da pressione.
- Il 6,6% come soglia (66 milli): confonde una quota di eventi con una
  banda; la soglia va derivata dalla distribuzione della banda, non scritta
  come se fosse la percentuale.

## Note datate

- **2026-11-22 — Status: accepted (delegated, DD-44).** Il maintainer ha
  approvato "ok alla soglia derivata". Attuazione con SIM 186.
