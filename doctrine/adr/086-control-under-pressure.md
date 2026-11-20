# ADR-086 — Il controllo sotto pressione (OI-132)

- **Status:** Accepted — DD-26, direttive del maintainer del 2026-11-20
- **Date:** 2026-11-20
- **Decision:** la pressione di un avversario che aspetta mette alla prova la
  risposta del portatore, con la stessa lettura deterministica del contrasto
- **Builds on:** ADR-042 (il contatto senza dado), ADR-052 (la scena letta),
  ADR-084 (il possesso è il movimento), ADR-085 (repertorio, famiglia 3)

## Il metodo, non l'architettura

Il maintainer: «il contrasto è uno degli atti della partita, se è giusto il suo
modo di leggere l'atto usalo per tutto», «vale per tutti gli atti». Questo è un
**criterio di analisi**: per ogni atto si chiede se ha un riscontro nella
partita vera; se non ce l'ha, si cerca l'errore nella logica della simulazione,
non la si avvicina statisticamente. Non è una regola di codice: non tutti gli
atti devono passare dalla stessa funzione. Trasformarla in un'architettura
sarebbe un'invenzione, non una decisione del maintainer. Il workflow va portato
fino in fondo nell'ordine deciso e non si interrompe per una battuta.

## LUNA

Il dito: il piede dell'avversario che aspetta (`Jockey`), la palla che rotola
fra due tocchi, il contatore dei tick. La luna: lo stesso portatore, con il suo
`first_touch`, il suo dribbling, il suo corpo, che legge l'uomo vicino e
sceglie se e come tenere la palla. Cambia la scena e il suo stato; non cambia
chi è.

## Partita vera

Un giocatore pressato non perde il pallone solo quando gli entra un tackle:
spesso sbaglia il controllo, la palla gli scappa via rotolando ed è di chi
arriva. Prima il motore non lo sapeva: `Jockey` era «no contact this time», il
portatore restava `Possessed` finché non arrivava un `Challenge`, per quanto
l'avversario gli fosse addosso. Mancava l'atto, non la statistica.

## Decisione

1. Quando un avversario entro `duel.reach_cm` sceglie di `Jockey`, la risposta
   del portatore è letta da `resolve_duel`, lo stesso resolutor deterministico
   del contrasto: le forze dei corpi, il `first_touch`, il dribbling, lo spazio
   e i numeri già dichiarati in `duel.json`. Nessun dado, nessun bonus, nessun
   numero nuovo, nessun copione dalla panchina.
2. Il resolutor sa già se la palla è sotto il piede (`carrier_on_ball`): fra due
   tocchi il portatore non ha una risposta da dare. È la differenza che il
   gemello mette alla prova, non una soglia nuova.
3. Se il portatore non risponde, la palla **si scioglie dal suo tocco** con
   `loose_from_contact`: non è un tackle di chi ha solo jockeyato, e l'evento
   resta un duello senza vincitore. Il cooldown è il `retry_ticks` dichiarato.
4. Una causa alla volta: nessun altro atto è toccato, nessun altro numero
   entra. Non è un gesto nuovo: la famiglia 3 di ADR-085 rinvia qui la nota del
   controllo sotto pressione, che è la logica dell'atto che già c'è.

## Conseguenze

`SIM_VERSION` +1, voce in `CHANGELOG-SIM`, golden rigenerati, gemello
`pressure_tests_the_carrier_answer_not_the_mans_tackle` (stesso portatore,
stesso pressatore fermo, cambia solo il posto della palla), `fmt`, `clippy`,
`cargo test --workspace`, gate `closing`, giudizio visivo del maintainer. I semi
non si riducono e le asserzioni non si indeboliscono. Nessun merge in `main`
senza autorizzazione.
