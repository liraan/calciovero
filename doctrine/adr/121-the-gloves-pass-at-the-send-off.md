# ADR-121 — I guanti passano all'espulsione (DD-101)

- **Status:** Accepted (delegated, 2026-11-28) — da rivedere da parte del maintainer
- **Date:** 2026-11-28
- **Decision:** DD-101
- **Depends on:** Law 3, Law 12, ADR-063, ADR-120
- **Amends:** ADR-120 (chiude il residuo S8/T22 del portiere espulso a metà partita)

## Il problema

T22 dell'audit, residuo di S8: se il portiere veniva espulso (o portato fuori)
durante il gioco, i guanti restavano al vecchio numero fino alla sosta.
`bench.rs::keep_a_keeper` girava solo dentro `substitute_due`, quando
`self.stoppage`: fino al fischio nessuno era in porta, e i lettori in gioco
cercavano ancora il portiere nell'undici della formazione. Ogni tiro nello
specchio diventava gol, anche con un compagno a tre metri dalla propria linea.

## I filtri

- **partita vera:** il dito è il cartellino rosso e il portiere che esce; la
  luna è la squadra che, per la Legge 3, deve avere sempre un portiere. Il
  guanto passa quando l'uomo esce, non quando l'arbitro fischia una sosta
  qualsiasi: il tempo tra i due eventi è gioco vero, non un intervallo.
- **LUNA:** l'uomo che prende i guanti è già in campo e resta lo stesso
  giocatore: cambia il compito, non l'identità, la capacità o la scelta. Non
  gli viene imposto un gesto, né gli si tolgono le opzioni: la regola dice
  solo chi difende la porta, come già faceva `keep_a_keeper` alla sosta.

## Decisione

1. `SpatialState::send_off` passa i guanti subito quando esce il portiere di
   una squadra: lo prende il compagno in campo più vicino alla propria porta,
   con lo stesso criterio e lo stesso pareggio per slot già usati a
   stoppage. Nessun numero, soglia, ruolo o opzione nuova.
2. `SpatialState::goalkeeper(home)` è la verità del portiere corrente: il
   lettore in gioco (palla in porta, palla in arrivo, duello, colpo di testa,
   scena del portatore, ripresa, negazione di un'occasione) legge quello, non
   il primo della rosa. La rosa resta la formazione; il cambio di portiere in
   `bench.rs` resta il momento in cui si aggiorna l'undici dichiarato.
3. Il gemello `the_gloves_pass_at_the_send_off_not_at_the_stoppage` tiene
   fissi i ventidue e cambia solo l'uomo espulso: se esce il portiere i
   guanti vanno al compagno più vicino alla porta, se esce un altro la
   geometria resta invariata.

## Conseguenze

- `SIM_VERSION` +1 e voce in `CHANGELOG-SIM.md`; nessun evento, campo,
  parametro o gesto nuovo, nessun numero preso dalla banda (A5-bis).
- I cinque semi dei golden e della coppa non contengono un'espulsione di un
  portiere: gli hash e i semi restano identici, aggiornati solo i commenti
  che citano la versione. Misura `ds-reference 42..101` scritta in
  `L0-CALIBRATION.md` §228; ogni riga fuori banda conserva la sua causa.

## Scartate

- **Aspettare la sosta, poi cambiare l'undici.** Era il difetto: Law 3 non
  sospende il gioco per un portiere espulso, e un tiro immediato non poteva
  trovare un uomo in porta.
- **Far scegliere il compagno al portiere espulso.** Non è più in campo e non
  ha la scena: la scelta di chi prende i guanti è una regola della squadra,
  già scritta in `keep_a_keeper`; inventarne un'altra sarebbe un numero o una
  preferenza senza fonte.
- **Aggiungere un evento `KeeperChange`.** Nessun letto ha bisogno del
  evento: lo stato del portiere è già letto da `spatial`, e un evento nuovo
  cambierebbe il racconto senza correggere il gioco.
