# ADR-120 — The card says why it is red (DD-100)

- **Status:** Accepted (delegated, 2026-11-28) — da rivedere da parte del maintainer
- **Date:** 2026-11-28
- **Decision:** DD-100
- **Depends on:** Law 12, ADR-114 (`LineupAnnounced` and the season's events), ADR-116
- **Amends:** none

## Il problema

T22 dell'audit e la voce 9 della coda `HANDOFF.md` §4: il motore scrive
solo `{"color": "red"}` per ogni espulsione. Che il rosso sia un secondo
giallo lo si può dedurre solo incrociando due eventi della stessa partita,
e il fascicolo di una partita non dovrebbe obbligare chi lo legge a rifare
la decisione dell'arbitro (`ds-discipline` lo fa oggi, e lo dichiara in
`CHANGELOG-SEASON.md` §6 con la fonte del CGS).

## I filtri

- **partita vera:** il dito è il cartellino mostrato in quel momento; la
  luna è l'arbitro che ha già visto il primo giallo e sa perché estrae il
  rosso. Il motivo è parte dell'atto, non un'inferenza del referto.
- **LUNA:** la scelta del giocatore non cambia: il flag è scritto dove la
  decisione è già presa, nello stesso punto in cui `card_after` trasforma
  il secondo giallo in rosso. Non aggiunge né toglie opzioni.

## Decisione

1. Un rosso porta `second_yellow: bool`: `true` solo quando Law 12 ha
   trasformato una seconda ammonizione, `false` per un rosso diretto. Il
   giallo resta `{"color": "yellow"}` senza campi nuovi.
2. La disciplina di stagione continua a dedurre il secondo giallo dagli
   eventi: nessun cambio di contratto L1, nessun valore letto dal conteggio
   dei cartellini (A5-bis).
3. Il gemello in `laws_hold.rs` chiede che ogni rosso del record concordi
   con le ammonizioni dello stesso uomo; la guarda unitaria in `referee.rs`
   copre le tre forme del campo.

## Conseguenze

- `SIM_VERSION` +1 e voce in `CHANGELOG-SIM.md`; nessun evento, soglia o
  scelta nuova, nessun numero preso dalla banda (A5-bis).
- L'hash di stato non cambia: golden L0/L1 e semi restano validi, aggiornati
  solo i commenti che citano la versione. Misura `ds-reference 42..101`
  scritta in `L0-CALIBRATION.md` §227; le righe fuori banda restano con le
  cause già dichiarate.

## Scartate

- **Far leggere il flag a `ds-discipline`.** Sarebbe un cambio di contratto
  L1 senza un difetto: la deduzione attuale è corretta e dichiarata.
- **Aggiungere il campo anche al giallo con `false`.** Cambierebbe il record
  di ogni ammonizione per dire una cosa che il colore già dice.
- **Ricalcolare i golden o cercare un nuovo seme.** Il comportamento
  simulato è identico; cambia solo ciò che il cartellino racconta.
