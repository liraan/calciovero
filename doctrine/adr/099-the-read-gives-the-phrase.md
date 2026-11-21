# ADR-099 — Il riconoscimento chiude OI-96 (DD-41)

- **Status:** Accepted (delegated, 2026-11-21) — Form B scelta e §4 confermata dal maintainer il 21 nov; da rivedere da parte del maintainer
- **Date:** 2026-11-21
- **Decision:** DD-41
- **Chiude:** OI-96 (il terzo termine della latenza, `plausible_options`)
- **Amends:** ADR-047 §2 e §5 (la forma del riconoscimento è decisa; qui si chiude il buco dell'attuazione); ADR-044 §6 (la durata della catena sta nella relazione *percepita*)
- **Depends on:** ADR-035 (nessun dado), ADR-039 (read/choose/contact/band), ADR-043 (l'attore decide), ADR-044 (la catena), ADR-047 (la scelta è riconoscimento), ADR-062 (la percezione non è esatta), ADR-059 (l'intenzione del coach), `BEHAVIOURS.md` 83-91 (primo uomo istintivo, secondo uomo lettura)

## Contesto

Nella lista delle fasi in cui il giocatore agisce (ricostruita il 21 novembre dai due filtri), la catena reale è:

    Legge → corpo si muove → percezione → riconoscimento → scelta → tempo → esecuzione

Il riconoscimento non era una fase: `choose_off_ball` (`ds-orchestrator/src/offball.rs:240`) offre sempre tutte le opzioni, e `latency.rs:21-26` dichiara che un terzo termine della latenza (`plausible_options`) non si può scrivere senza duplicare la plausibilità del choose o senza circolarità — OI-96 resta aperto dal SIM 31.

La distinzione non è un nuovo peso. Il maintainer ha scelto esplicitamente la **forma B** (21 novembre): il +1 tick va sulla **disponibilità della frase nella lettura**, non su un ritardo di esecuzione dopo l'argmax (forma A, respinta perché ADR-044 §6 la vieta e la rende circolare). L'atto resta scelto dal giocatore; la frase pensata diventa disponibile un tick dopo.

Due ulteriori difetti sono emersi dalla stessa verifica, con gli stessi due filtri:

- `latency_scene` (`thought.rs:1149`) calcolava `crowding` e `nearest_threat` da **posizioni vere** (`self.spatial`), mentre la scena dell'attore è filtrata con `as_seen_by` (`:144`). Il tempo della catena dipendeva da un avversario che il giocatore non percepisce.
- `pick_from_space` (`thought.rs:973`) eleggeva l'attore come il giocatore in possesso **più vicino alla palla**, e subito dopo `decide()` verificava `carrier() == attacker` (`:53`). Il selettore per distanza è l'avanzo del modello vecchio: nei tick in cui il portatore non è il più vicino, nessuno decideva.

Il **dito** è il tick, la frase disponibile, la posizione percepita, l'id del portatore. La **luna** è lo stesso giocatore: legge con i propri occhi, riconosce con il proprio bagaglio, sceglie fra ciò che ha in mano ed esegue con il proprio corpo. Cambia il *come* e il *quando* della catena, non l'etichetta dell'atto, non la fisica, non la Legge.

## Decisione

### 1. Il riconoscimento è una fase, non un termine

Il read produce le frasi **disponibili**, il choose sceglie fra esse (ADR-047 §2). Non si aggiunge un peso "istinto" e non si scrive una lista nuova di atti pensati: la frontiera è già `BEHAVIOURS.md:83-91` e `game_reading` dice quante frasi il giocatore riconosce (ADR-047:47).

- **Frase istintiva** (primo uomo: `rank(0)`, pallone libero più vicino, sfida a portata): disponibile nel tick in cui la scena la mostra.
- **Frase pensata** (secondo uomo: i termini `rank(1)` — raddoppio, secondo uomo sul pallone libero): disponibile un tick dopo che la scena l'ha mostrata.

Il +1 sta nella disponibilità, quindi prima dell'argmax. Non c'è un `plausible_options` nella formula della latenza: OI-96 si chiude spostando il conteggio delle opzioni dal tempo della catena al riconoscimento, dove ADR-047 lo mette. Nessun numero nuovo, nessun dado.

### 2. La durata si legge da ciò che il giocatore percepisce

`latency_scene` usa `self.perceived(actor, opp)` invece di `self.spatial.agent(opp)`. Un avversario fuori dal cono, mai visto, non rallenta la catena: la durata è una relazione (ADR-044 §6) e la relazione è quella percepita (ADR-062). È una riga, non un cambio di dottrina.

### 3. L'attore è chi ha il pallone

`pick_from_space` è cancellato. `decide()` prende `self.spatial.carrier()`: chi ha il pallone è un fatto fisico, non un ruolo assegnato dal motore per distanza. Il `defender` resta soltanto nel ramo `ChosenAction::Duel`, oggi irraggiungibile (`build_choice` non emette Duel): quando Plan E lo riattiverà, andrà preso dagli avversari che il portatore *percepisce*, non dalle posizioni vere. `nearest_in_squad` resta come helper della Legge (`set_pieces.rs:257`) e della copertura (`thought.rs:1222`).

### 4. Plumbing (scelta di attuazione, non di modello)

Il riconoscimento vive nell'orchestratore, che è il read dell'attore; il motore resta applicazione (ADR-041). L'orchestratore restituisce al motore il tick in cui la frase è diventata disponibile; il motore lo scrive in `offball_due`. Il motore non vale e non sceglie nulla. **Confermata dal maintainer il 21 novembre 2026.**

### 5. Il portatore fermo non aspetta il vecchio orologio

Dalla 2.5D (seed 42) il maintainer ha visto il portatore fermarsi. Il **dito**: un ricevitore con palla al piede, velocity zero e `carry_target = None` restava fermo fino a scadere il `latencies` del suo possesso precedente. La **luna**: è lo stesso giocatore che ha già la palla e non ha niente da eseguire; il vecchio timer non è una sua lettura. Se è fermo e senza destinazione, legge una volta subito; poi la sua scelta (anche `Hold`) resta valida per tutto il suo tempo di decisione. Un portatore in movimento conserva il proprio ritmo: non si cambia la velocità, né si anticipa la scelta di chi sta correndo. Sullo stesso seed le finestre di sosta scendono da 83/1132 tick a 39/519; `no_match_diverges` resta verde (il reset anticipato per tutti dava 23 gol sul seed 4 ed è scartato).

## I due filtri

| | Partita vera (D6, attori) | Luna |
|---|---|---|
| Riconoscimento | Si vede come esitazione del secondo uomo: il primo agisce, il secondo ci pensa. Non come un peso diverso sulla scelta. | È lo stesso giocatore con la sua storia: il bagaglio decide *cosa ha in mano*, non *cosa sceglie*. |
| Latenza percepita | Un giocatore pressato da un uomo fuori dal suo cono non esita. | Il dito è il tick in più; la luna è lo stesso giocatore che non può misurare una minaccia che non ha visto. |
| Attore | Il tifoso vede il portatore decidere, non un arbitro che nomina il più vicino. | Il dito è una distanza e un id; la luna è il portatore che decide, senza alternative sottratte. |

Nessun filtro giudica il crate layout (NOTA §9): i tre punti non spostano responsabilità, tolgono un selettore e correggono una misura.

## Verifica sulla partita vera

Tre gemelli, uno per punto:

1. **Disponibilità**: stessa scena, stessa frase; istantanea per il primo uomo, disponibile al tick+1 per il secondo. Nessun secondo salto dopo l'argmax.
2. **Latenza percepita**: stesso portatore, avversario visto contro avversario mai visto → latenza diversa.
3. **Attore = portatore**: portatore più vicino contro portatore non più vicino → decide comunque.

`SIM_VERSION` 183 → 184; golden e semi rigenerati (coppa 117 → 45, pareggio 44 → 44, decisivo 42 → 42, shoot-out 117 → 45); `the_match_never_freezes` rivalutato e verde. `ds-reference compare --seeds 42..51`: gol 11.30 → 12.80, tiri 47.60 → 49.20, in porta 21.50 → 20.90, passaggi 1384.00 → 1526.10 — variazione caotica, non tarata. TUI seed 42 e 2.5D girati con il maintainer.

## Conseguenze

- OI-96 chiuso: la latenza resta `base × bias(decision_making) × scene(crowding, threat)`; il terzo termine non entra perché il suo contenuto è ora una fase del read.
- Nessun behavioural latente nuovo, nessun numero nuovo in `balance/`, nessuna lista di atti pensati.
- Il test `the_match_never_freezes` e i golden devono essere rivalutati dopo la rigenerazione: il selettore per distanza tolto può cambiare il momento della ripartenza.
- Da fare separatamente, **dopo** questo cantiere: D5, i `Threats` di `TeamView::threats` (`state.rs:547`) calcolati dall'orchestratore, così i valori che l'attore pesa sono suoi. Cambia i valori per giocatore: SIM bump a parte.

## Scartate

- **Forma A** (+1 dopo l'argmax): ADR-044 §6 la vieta e `latency.rs:21-26` la dichiara circolare.
- **Un peso "istinto"**: ADR-031 vieta il terzo termine; `instinct(0.5, 0.5, 0)` garantisce già la presenza.
- **Una lista di atti pensati**: la frontiera è `BEHAVIOURS.md:83-91`, non un nuovo catalogo.
- **Un `plausible_options` nella formula**: duplica la plausibilità del choose (D7) e rende la latenza dipendente dall'esito della scelta.
- **Tenere `pick_from_space` come protezione**: il gate `carrier() == attacker` la rende inutile in tutti i casi tranne quello in cui blocca il portatore.

## Note datate

- **2026-11-21.** Scritto dopo la lista in ordine reale e l'armonizzazione a due filtri approvata dal maintainer. I tre punti di codice sono stati letti prima della stesura (ADR-026). Le modifiche a `latency_scene` e a `decide()` sono applicate e compilano; golden, semi e SIM bump seguono la chiusura del cantiere.
- **2026-11-21 (tardo).** §4 confermata; attuata la Form B con stato del riconoscimento in `GroundTruth` e gemello `the_second_man_gets_the_phrase_one_tick_later`. Aggiunto §5 dopo l'osservazione dalla 2.5D; gemello `a_new_carrier_decides_without_waiting_for_the_old_clock`, con controllo che `Hold` non venga disfatto. GUI ricompilata con l'indicatore dello sguardo a freccia.
- **2026-11-21 (chiusura).** SIM bump 184, golden e semi rigenerati, pulizia delle ridondanze di codice e della documentazione; ADR-071 §5 emendata con la nota sullo spostamento di `choose_closed_place` in `ds-orchestrator/src/offball.rs`. `choose_closed_place` resta regola dell'orchestratore, nessun comportamento cambiato.
