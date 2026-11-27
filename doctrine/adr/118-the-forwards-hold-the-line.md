# ADR-118 — The forwards hold the line (DD-98)

- **Status:** Accepted (delegated, decision of the maintainer, 2026-11-27)
- **Date:** 2026-11-27
- **Decision:** DD-98
- **Depends on:** ADR-051, ADR-059 §4, ADR-070 §2, ADR-071, ADR-019, Rule 11, ADR-062
- **Amends:** ADR-070 §2 (the onside place of a forward is no longer only for a man given the "attack the depth" task)

## Il problema

T23 dell'audit (`context/plans/CODE-AUDIT.md:187`): `shape.rs::place` mette
le punte a `line + depth`, con `depth_in_possession_cm` = 45 m, quindi
abitualmente oltre il penultimo avversario. ADR-070 §2 corregge quel posto,
ma soltanto per chi ha il compito individuale `Task::Run`. La lavagna
predefinita (4-4-2, `ds-model/src/squad.rs:349`) ha `Task::Free` in tutti gli
slot: le punte di ogni squadra senza istruzioni restano oltre la linea e
possono essere servite solo se il fischio dell'assistente le trova in
fuorigioco. Il motore le mette in una posizione che la Regola 11 vieta.

Non è una distanza da colmare scegliendo un margine guardando la banda
(A5-bis): il margine esiste già (`offball.json`, `onside_margin_cm`) e la
linea è già letta dallo stesso helper usato da ADR-070.

## I filtri

- **partita vera:** il dito è il ruolo sulla lavagna (la linea d'attacco,
  `depth` 1000 in `shape.rs`) e il penultimo avversario (Regola 11); la luna
  è lo stesso attaccante che aspetta sulla linea e poi sceglie COME giocare:
  offre, parte, torna. Un attaccante vero aspetta sulla spalla del difensore,
  non a 45 m oltre.
- **LUNA:** la regola fissa *dove sta chi aspetta*, non *cosa fa*: non toglie
  l'offerta, la corsa né alcuna opzione, e non impone l'esito del passaggio.
  La scelta resta nel COME (ADR-049, ADR-051, ADR-070 §1). Il gate è la
  Regola 11, non un veto sulla scelta (ADR-062).

## Decisione

Con la palla, un uomo disegnato sulla linea delle punte
(`role.depth == 1000`, estremo già dichiarato della lavagna) tiene il posto
sulla linea del penultimo avversario, corta dello stesso
`onside_margin_cm` che ADR-070 §2 e la corsa in profondità già usano. Vale
anche per chi ha `Task::Run`: la regola di ADR-070 resta, ma non è più
necessario un ordine perché valga per le punte.

Nessun numero, opzione, stato o gesto nuovo. Il punto di arrivo di una corsa
scelta resta quello di `offball.rs` e `state.rs::act_target`, già sulla
stessa linea; cambia soltanto il posto di chi aspetta senza aver scelto
ancora.

La linea è letta come oggi (`spatial.rs::second_last`, dalle posizioni dei
corpi). Il difetto di percezione segnalato da OI-98 / ADR-062 (c) — la linea
si legge in modo perfetto, senza errore funzione della vista — resta una
domanda aperta e non viene risolta qui: non esiste una fonte per l'errore,
e inventarne la grandezza per avvicinare una banda sarebbe A5-bis. Questa
decisione non aggiunge una nuova lettura occulta: estende ai senza-istruzioni
la stessa lettura che i con-istruzioni usano dal SIM 148.

## Conseguenze

- `block_target` applica la correzione a ogni punta in possesso, senza
  guardare i compiti della lavagna; il ramo `Task::Run` resta per gli uomini
  che la lavagna disegna più indietro.
- Un gemello a scena identica cambia solo il ruolo del giocatore (punta contro
  centrocampista) e mostra che la punta aspetta sulla linea, il centrocampista
  no. Nessuna opzione di passaggio o di corsa viene tolta.
- Il gemello storico di ADR-070 passa dalla punta a un centrocampista: con la
  punta già sulla linea per questa regola, l'ordine di attaccare la profondità
  non può più produrre il metro di differenza che misurava. ADR-070 resta
  verificato per chi è disegnato sotto la linea delle punte.
- `SIM_VERSION` +1, golden e semi rigenerati, voce in `CHANGELOG-SIM.md`,
  misura `ds-reference 42..101` in `L0-CALIBRATION.md`. Il numero di fuorigioco
  è un controllo, non un bersaglio: nessun valore viene scelto guardandolo.

## Scartate

- **Aggiungere un gate sul passaggio verso uomini oltre la linea.** Sarebbe
  un clamp sul COME del portatore (preferences-010): la scelta resta sua e il
  fuorigioco lo fischia l'arbitro (Regola 11).
- **Arretrare l'offerta in `shape.rs` con un nuovo margine o peso.** Il
  margine c'è già e nessun numero nuovo è giustificato da una fonte (A5-bis).
- **Arretrare tutto il blocco.** Impone la posizione a chi non è punta e
  distrugge la larghezza e le corsie lette da ADR-052; il compito del blocco
  resta in `shape.rs`.
- **Leggere la linea da posizioni percepite con soglia nuova.** La soglia
  dell'occhio (ADR-062 (b)) riguarda il fischio del passatore, non il posto
  che aspetta; una nuova freschezza o acuità resta senza produttore (DD-77,
  A5-bis).
