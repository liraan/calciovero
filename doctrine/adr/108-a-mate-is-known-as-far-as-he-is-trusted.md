# ADR-108 — Il compagno è conosciuto quanto ci si fida di lui (DD-50)

- **Status:** Accepted (delegated, 2026-11-22) — da rivedere da parte del maintainer
- **Date:** 2026-11-22
- **Decision:** DD-50
- **Attua:** foundations §3, "ogni attore ha i suoi tre piani"; VISION §3
- **Depends on:** ADR-052 (ogni giocatore pensa), ADR-058 (la ricezione letta dal passatore), ADR-094 / SIM 179 (il passo del compagno creduto per intesa e lettura), OI-102 / SIM 149 (il valore in aria creduto)

## Decisione

Il passatore che valuta un compagno (il suo primo controllo, quanto vale in aria) non legge più la verità del compagno: legge ciò che crede di lui. La credenza è la verità per quanto si fida di ciò che ha visto, il resto un uomo medio:

    creduto = 0.5 + fiducia × (vero − 0.5),   fiducia = intesa × lettura del gioco

È la stessa fiducia con cui dal SIM 179 crede al passo di un compagno che attacca la profondità, e lo stesso uomo medio (0.5) con cui vede ogni avversario. Nessun numero nuovo: il centro della scala, l'intesa della coppia (già nello stato della partita) e la sua `game_reading`.

Con LUNA: il dito è il primo controllo e il gioco aereo del compagno; la luna è che ognuno conosce gli altri solo per ciò che ha visto di loro. Una coppia affiatata e un buon lettore si conoscono come sono; uno sconosciuto, o chi legge male il gioco, vede un uomo medio. Partita vera: il regista che conosce i piedi del suo attaccante gli dà la palla difficile; il nuovo arrivato la riceve facile.

## Conseguenze

- SIM_VERSION 188 → 190 (il 189 resta del branch accantonato `sim-189-first-time`). Golden rigenerati; i semi della coppia e dei supplementari passano a 49 (pari, rigori) e 42 (deciso al 90').
- `ReceptionReckoning.intesa`, riempita da `ds-match` accanto a `run_pace`; `ds_orchestrator::actor::believed_of_mate`.
- Gemello `a_mate_is_known_as_far_as_he_is_trusted`.
- Misura 42..101: 6 su 22, come il 188 rimisurato; nessuna metrica cambia banda.

## Scartate

- Lasciare la verità: un oracolo nella lettura del passatore, contro i tre piani di ogni attore.
- Una soglia d'intesa (come lo `intesa >= 0.5` della previsione): un salto senza causa; la fiducia graduata è già nel motore.
