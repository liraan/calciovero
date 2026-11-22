# ADR-105 — Il tiratore legge il proprio nervosismo (DD-47)

- **Status:** Accepted (delegated, 2026-11-22) — da rivedere da parte del maintainer
- **Date:** 2026-11-22
- **Decision:** DD-47
- **Attua:** la shortlist della fase 2 di `PITCH-AUDIT.md`: la compostezza nel read del tiro
- **Depends on:** ADR-046 (la frase del tiro), ADR-052 (il valore di un options), ADR-057 (la compostezza modula la pressione del calcio), ADR-087 (non si riapre `error_pressure_rad`)

## Decisione

Il portatore che sceglie il tiro legge quanto la pressione disturberebbe
**il proprio** calcio, con la stessa compostezza che il motore applicherà
quando lo esegue (ADR-057). La formula è una sola, in
`ds_calculator::kick::KickBalance::pressure_nerves`: `2 (1 - composure)`.
Il fattore passa nella scena percepita come `own_nerves` e modula soltanto il
termine di pressione dell'errore che il tiratore si aspetta, sia quando
sceglie quanto aprire l'angolo (`aim_inside`) sia quando valuta la propria
dispersione (`own_spread`).

Con i due filtri: il **dito** è la pressione dell'avversario più vicino, la
distanza dalla porta, l'angolo, il piede, la potenza, il tick; la **luna** è lo
stesso giocatore che sa quanto è calmo e che, con quella conoscenza di sé,
sceglie fra le stesse opzioni di prima. Cambia il *quanto* vale il tiro per
lui, non *quali* opzioni ha, non l'etichetta dell'atto, non la fisica del
calcio e non la regola. Chi è nervoso con un uomo addosso legge un tiro più
difficile e può scegliere diversamente; chi è calmo lo legge più facile. Nessun
clamp sulla scelta, nessun peso nuovo, nessun numero di `balance/` toccato
(A5-bis).

La ragione non è la banda di una metrica, ma una coerenza interna: il motore
già fa pagare la compostezza al momento del calcio (ADR-057, SIM 132), mentre
il giocatore, un istante prima, sceglieva come se fosse sempre di compostezza
media. La sua lettura e la sua esecuzione devono essere dello stesso uomo.

## Conseguenze

- SIM_VERSION 186 → 187: a parità di seed e input il comportamento cambia.
  Golden e semi rigenerati una volta sola, con voce in `CHANGELOG-SIM.md`.
- `own_nerves = 1.0` (compostezza 0,5) riproduce esattamente la lettura
  precedente: la correzione non è un ritocco della taratura.
- Gemello `the_nervous_shooter_reads_his_own_shot_worse_under_pressure`:
  due tiratori identici, diversi solo per la compostezza che conoscono; con
  un uomo addosso il nervoso attribuisce al tiro un valore minore, senza
  pressione i due valori restano uguali.
- Nessun evento, cartellino o numero del registro cambiato: `ds-reference`
  rilegge la stessa partita senza adattare bande.

## Scartate

- Cambiare `error_pressure_rad` in `balance/kick.json`: è il numero dichiarato
  dell'ADR-057 e ADR-087 vieta di riaprirlo per tarare le metriche.
- Aggiungere un peso "nervosismo" in `shot_choice.json`: la compostezza è già
  un latente del giocatore e la sua regola già esiste in ADR-057; un peso
  nuovo sarebbe una seconda sorgente per lo stesso fatto.
- Far leggere al tiratore la compostezza dell'avversario o del compagno:
  resta una lettura di sé, come la fatica e il piede.

## Note datate

- **2026-11-22 — Status: accepted (delegated, DD-47).** Nessun numero nuovo,
  nessuna scelta imposta: il fattore che il motore già applica al calcio entra
  nella lettura che lo precede.
