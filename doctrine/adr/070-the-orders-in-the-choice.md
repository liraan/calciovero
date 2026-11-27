# ADR-070 — The orders in the choice

- **Status:** Accepted (delegated, 2026-11-17) — to be reviewed by the maintainer
- **Date:** 2026-11-17
- **Decision:** DD-12, from the maintainer's answer of 17 November ("Sì, adesso": the instructions in each player's choice, before the calibration)
- **Amends:** ADR-059 §3 and §4 (the quick instructions, the transitions and the tasks of the board)
- **Depends on:** ADR-049 (every choice is the player's), ADR-052 (the reading), ADR-059, ADR-068 (the channels)

*Written in Italian as the project's working language; the decision is binding as written.*

## Contesto

Il maintainer ha cambiato le impostazioni tattiche e non ha visto i giocatori cambiare atteggiamento. Aveva ragione. Le istruzioni rapide ("più pressing", "tutti avanti") e le transizioni spostavano soltanto il punto del blocco in cui un giocatore si mette quando tiene la posizione (`shape.rs`). Non entravano nella sua scelta: se pressare, se partire, se venire incontro. Il compito individuale sulla lavagna (ADR-059 passo 4) entrava nella scelta (`obey`), ma valeva solo se l'azione chiesta era già fra le sue opzioni del momento. Un attaccante dietro il pallone non ha mai l'opzione di attaccare la profondità, quindi il compito "attacca la profondità" lo spostava di 1 cm: il test che lo misurava passava per caso, e al SIM 148 ha smesso di passare.

## LUNA

Un'istruzione cambia quello che il giocatore vuole fare, non dove il motore lo mette: chi riceve l'ordine di pressare va a pressare, se la sua disciplina accetta quello che lascia per farlo.

## Partita vera

- **Come lo riconoscono gli attori:** dopo l'ordine di pressare, i due più vicini al portatore gli vanno addosso; l'attaccante a cui si chiede la profondità aspetta sulla spalla dell'ultimo difensore; dopo il pallone perso con il contropressing la squadra non scappa indietro.
- **Da cosa dipende:** dall'ordine dell'allenatore, arrivato per il suo canale (ADR-068); dalla disciplina di chi lo riceve; dalla scena che vede.
- **Chi lo produce:** il giocatore, con la sua scelta (ADR-049); l'allenatore chiede soltanto.

## Decisione

1. **Gli ordini di squadra sono richieste nella scelta di ogni giocatore** (`SpatialState::team_asked`), come i compiti della lavagna, e passano per la stessa regola della disciplina (`obey`, ADR-059 §4). Il compito individuale viene prima; se non c'è, vale l'ordine della squadra:
   - **senza palla**, con "più pressing", oppure nei secondi dopo il pallone perso con il contropressing: ai due più vicini al portatore si chiede di pressare (gli stessi due che il pensiero del difensore già considera, ADR-052 §5);
   - nei secondi dopo il pallone perso con "indietro": a tutti si chiede di tenere il posto;
   - **con la palla**, nei secondi dopo il pallone vinto: il contropiede chiede di attaccare la profondità, "tenere palla" chiede di venire incontro.

   Lo spostamento del blocco di ADR-059 §3 resta com'era: è dove si mette chi tiene il posto.
2. **Il posto di chi deve attaccare la profondità è sulla spalla dell'ultimo difensore.** Con la palla alla sua squadra, chi ha il compito "attacca la profondità" tiene il posto sulla linea del penultimo avversario (Regola 11), corta del margine di `offball.json` che la corsa già rispetta, nella sua corsia. Da lì la corsa diventa una delle sue opzioni.
3. **Nessun numero nuovo:** la regola della disciplina, i due più vicini e il margine del fuorigioco ci sono già.
4. **Gemelli con un effetto che si vede, non di un centimetro** (`tests/instructions.rs`), su sei partite dopo l'ordine e con la stessa partita fino all'ordine:
   - con "più pressing" il più vicino al portatore gli sta almeno mezzo metro più vicino (8,5 m invece di 9,7);
   - con "attacca la profondità" l'attaccante sta almeno due metri più avanti del pallone (35,9 m invece di 31,3).

## Conseguenze

- Le partite senza istruzioni non cambiano: i tre golden e la misura della taratura sono gli stessi con e senza questo ADR. Il `SIM_VERSION` 148 lo porta insieme ai contatti a ogni passo piccolo (ADR-069).
- Le regole tattiche di base che ogni squadra ha senza istruzioni (gli inneschi del pressing, la copertura e il raddoppio, la compattezza fra i reparti, scelte dal maintainer il 17 novembre) sono il passo dopo, con il loro ADR, prima della taratura.

## Alternative scartate

- **Ordini più forti spostando di più il blocco:** il giocatore resterebbe un punto spostato, non uno che sceglie, e i numeri dello spostamento sono già arbitrari fino a L3.
- **L'ordine eseguito sempre:** toglierebbe la disciplina del giocatore (ADR-059 §4). Il giocatore indisciplinato che non pressa è calcio vero.

## Note datate

- **2026-11-17 — il gemello della corsa in profondità misura la linea (`SIM_VERSION` 149).** Il gemello misurava quanto l'attaccante sta davanti al pallone, una misura che si muove anche con dove va il pallone. Al 149 il compito lo porta ancora 2,4 m più vicino alla linea del penultimo avversario (3,9 m invece di 6,3), ma solo 1,2 m più avanti del pallone. Ora il gemello misura quello che il compito chiede, la distanza dalla linea, con almeno un metro di differenza (`a_striker_asked_to_run_waits_on_the_last_line`). **SIM 225 (ADR-118):** il gemello misura ora un centrocampista (`a_midfielder_asked_to_run_waits_on_the_last_line`): la punta aspetta sulla linea anche senza ordine, quindi l'ordine non può più spostarla di un metro.
