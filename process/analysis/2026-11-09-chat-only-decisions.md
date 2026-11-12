---
title: "Chat-only decisions — 2026-11-09"
version: "1.3"
status: "analysis"
last_updated: "2026-11-09"
conforms_to: "SESSION-STATE.md, NEW-SESSION-PROMPT.md (D1, D5, D6, E3c), NOTA-PER-NUOVA-AI.md §3, ADR-041, ADR-042"
---

# Chat-only decisions — 2026-11-09

**What this is.** Decisioni prese in chat il 2026-11-09 (e poco prima) che non avevano altra sede. Scritto su richiesta del maintainer, dopo la chiusura della sessione (E3c). Il report che ha aperto il file era prosa; questo file rende lo stesso contenuto verificabile, con uno **stato di verifica** per punto e una **prima azione** per ciascuno. Due filtri applicati, dichiarati per punto: **partita vera** (D6) e **luna** (filtro di scoperta). Nessun dado nel modello, nemmeno nelle formule dei filtri (ADR-035: derivazione da chiave non è dado).

**What this is not.** Non è un handoff (quello è `2026-11-09-session-handoff-v7.md`). Non è il bookmark (§F: `SESSION-STATE.md`). Non è un ADR: niente qui è ratificato. È una lista di finding, scritta per essere letta da un agente che non c'era e chiusa un punto alla volta.

## 0. I due filtri — la mappa

**Partita vera** (D6): *come si riconosce / da cosa dipende / chi lo produce.* Ammissione: si applica alle decisioni di modello, non a quelle di architettura o di dato (un tifoso non vede il layout di un crate).
**Luna** (filtro di scoperta): *qual è la proprietà più grande che questo punto rivela?* Priorità: si applica a ogni punto; se non c'è, il punto lo dichiara (`Luna: nessuna`).
**Precedenza (dichiarata).** In caso di disaccordo fra i due, **vince partita vera come ammissione** (un punto che mente sulla partita non entra, anche se rivela qualcosa); vince **luna come priorità** (a parità di ammissione, il punto che rivela di più si chiude prima).

| # | Punto | Natura | Partita vera | Luna — cosa rivela |
|---|---|---|---|---|
| 1.1 | ds-selector a L2 | architettura | non si applica | un ADR accepted può contenere una riga superata; A1 conta gli ADR, non le righe interne |
| 1.2 | resolve_movement separato | architettura | non si applica | contatto/effetto non è una partizione, è una parte: il substrato va nominato o cade fuori |
| 1.3 | from_key genera, non riempie | modello | si applica | la derivazione da chiave non è neutra: ha una semantica (provenienza) |
| 1.4 | vault eterogeneo | dato | non si applica | la provenienza è già nel modello (1.3); il vault la materializza |
| 1.5 | il mondo genera giocatori? | prodotto | informa | una decisione di prodotto non presa condiziona la dottrina (1.3, 1.4) |
| 1.6 | struttura di fattori cantera | modello (L3) | si applicherà | un prerequisito dichiarato senza proprietario è un'idea, non un prerequisito |
| 2.1 | tre liste non allineate | dato | non si applica | quattro numeri, nessuno coerente: la specifica non è una fonte, è un insieme di fonti |
| 2.2 | Shot legge finishing, spec dice shooting | modello | si applica | vince la parola del tifoso: il prototipo non è una spec |
| 2.3 | consistency in §7.3 ma non in §4.6 | spec interna | non si applica | due liste nella stessa specifica non si parlano: il catalogo non è indicizzato |
| 2.4 | Pressure non legge una caratteristica | modello | si applica | "ogni evento ha un campo" è falso: la copertura non si misura in latenti |
| 2.5 | MaximalForce/Acceleration senza campo | dato | non si applica | OI-92 dice "rimuovere o dare un valore", ma il dato non esiste: è da togliere |

## 1. Decisioni prese in chat e mai scritte

### 1.1 ds-selector rinviato a L2

**Natura:** architettura. **Partita vera:** non si applica (D3). **Luna:** si applica.
**Luna — cosa rivela.** Un ADR accepted può contenere una riga superata: A1 conta gli ADR, non le righe interne. La coerenza di un ADR non è verificata dal gate.
**Stato:** `verificato`. ADR-041 §2 riga 37 elenca `pcfm-selector (new)`; riga 26 elenca `ds-selector` tra i nomi `ds-*`. La conclusione in chat ("a L0 i 22 sono in campo, la selezione è input, i cambi sono L2, quindi `ds-selector` a L0 è struttura senza contenuto") non è in ADR-041. Check: `grep -nE 'ds-selector|pcfm-selector' doctrine/adr/041*.md`.
**Prima azione.** Nota datata in ADR-041 §2: la riga `pcfm-selector (new)` è `Removed-by: L2`. Il nome `ds-selector` resta per il crate L2; lo *split* è rinviato, non il nome (D7).

### 1.2 resolve_movement è un caso separato

**Natura:** architettura. **Partita vera:** non si applica. **Luna:** si applica.
**Luna — cosa rivela.** Contatto/effetto non è una partizione, è una parte: il substrato (il corpo che si muove) va nominato, o cade fuori dalla mappa di ADR-041.
**Stato:** `verificato`. ADR-041 §4 riga 20 raggruppa `resolve_movement` con i gesti di contatto (`resolve_pass`, `resolve_shot`, `resolve_duel`, `resolve_reception`); il movimento non è contatto (non tocca la palla) né effetto (è il corpo che si muove). Check: `grep -nE 'resolve_movement' doctrine/adr/041*.md`.
**Prima azione.** Nota datata in ADR-041 §4: `resolve_movement` è il movimento del corpo (substrato, formula F.7), non contatto, non effetto; resta nel modulo movimento. Nota additiva (D7); l'ADR è accepted, la nota è la forma corretta per un chiarimento.

### 1.3 from_key genera, non riempie

**Natura:** modello + provenienza. **Partita vera:** si applica. **Luna:** si applica.
**Partita vera.** In partita vera un tifoso riconosce la differenza fra un giovane all'esordio (nessuna storia: la derivazione da chiave è l'unico modo per esistere) e un giocatore con 200 partite (la storia c'è: riempire i buchi con una derivazione da chiave è mentire sulla provenienza).
**Luna — cosa rivela.** La derivazione da chiave **non è neutra**: ha una semantica (provenienza). "Derivato" e "generato" sono due cose diverse, non due sinonimi; la regola vale per ogni derivazione futura, non solo per i giocatori.
**Stato:** `assente`. `grep -niE 'from_key|provenance' doctrine/adr/029*.md` restituisce solo `generated` in senso generico; nessuna distinzione provenance.
**Prima azione.** Nota datata in ADR-029: `from_key` **genera** (il giocatore non ha storia, la derivazione è il suo inizio); un uso improprio **riempie** (il giocatore ha storia, la derivazione mente). La distinzione diventa un campo (`provenance: Derived | Generated`) nel vault quando il vault esiste (1.4).

### 1.4 Il vault futuro è eterogeneo

**Natura:** dato. **Partita vera:** non si applica (il tifoso non vede il vault). **Luna:** si applica.
**Luna — cosa rivela.** La provenienza è già nel modello (1.3); il vault la materializza. Un vault che mescola `Derived` e `Generated` senza distinguerli rende **impossibile la calibrazione** (L3): non si può tarare un parametro su un campione di cui non si sa la provenienza.
**Stato:** `assente`. `grep -rniE 'provenance' docs/ specs/ crates/` restituisce solo due hit (ADR-015 §128 "legal/provenance question", `MIGRATION-01-04.md` §101 "DataProvenance" del vecchio schema), nessuno pertinente.
**Prima azione.** Dipende da 1.3 (prima la semantica, poi il campo). Nessuna azione ora.

### 1.5 "Il mondo genera giocatori?" è una domanda aperta

**Natura:** prodotto. **Partita vera:** informa, non decide (P6: la scelta è del maintainer). **Luna:** si applica.
**Partita vera.** In partita vera il mondo è **finito**: i giocatori esistono, non si generano. La domanda è se il *prodotto* debba simulare un mondo finito (dati reali) o infinito (generazione).
**Luna — cosa rivela.** Una decisione di prodotto non presa **condiziona la dottrina** (1.3, 1.4): il significato di `from_key` e la struttura del vault dipendono da questa risposta.
**Stato:** `assente`. Non è in `IDEAS.md`, non è un OI. La prima azione è eseguibile: mancava solo il proprietario.
**Prima azione.** Aprire un OI: "leggere IDEA-YOUTH-001 e ADR-013 §worlds; decidere se il mondo è finito o infinito; la risposta è input di 1.3 e 1.4".

### 1.6 La struttura di fattori della cantera

**Natura:** modello (L3). **Partita vera:** si applicherà quando L3 esiste. **Luna:** si applica.
**Partita vera.** In partita vera la selezione di un giovane dipende da fattori riconoscibili (l'accademia che lo ha formato, la nazione, l'età, il ruolo, lo sviluppo fisico). IDEA-YOUTH-001 li nomina, non li pesa.
**Luna — cosa rivela.** Un prerequisito dichiarato **senza proprietario** è un'idea, non un prerequisito. IDEA-YOUTH-001 dice "struttura di fattori (accademia, nazione, età, ruolo, sviluppo)" e si ferma: nessun file dice quali fattori, con che peso, chi li produce.
**Stato:** `parziale`. La parola "struttura" è in IDEA-YOUTH-001; i fattori no; nessun OI.
**Prima azione.** Nota in IDEA-YOUTH-001 con `Removed-by: L3`. Nessuna azione ora: L3 non esiste.

## 2. Findings che vivono solo in COVERAGE.md (WIP) o nei commit

### 2.1 Le tre liste non allineate

**Natura:** dato. **Partita vera:** non si applica. **Luna:** si applica.
**Luna — cosa rivela.** Quattro numeri (25 catalogo L0.11 §4.6, 7 `Characteristic`, ~15 `PlayerLatents`, 7 letti dai `resolve_*`), nessuno coerente: **la specifica non è una fonte, è un insieme di fonti**. Un termine può vivere in tre posti e mancare nel quarto senza che nessun gate lo veda.
**Stato:** `parziale`. In `L0-match/design/COVERAGE.md` (WIP, 9 eventi su 26), non in un OI. Il `grep` che renderebbe la tabella meccanica è la prima azione di IDEA-DESIGN-012.
**Prima azione.** Scrivere il `grep` di IDEA-DESIGN-012 (per gesto, caratteristiche lette) e farne il test meccanico di OI-92.

### 2.2 Shot legge `finishing`, il modello dice `shooting`

**Natura:** modello, nome. **Partita vera:** si applica. **Luna:** si applica.
**Partita vera.** In partita vera il tifoso dice "che tiro", non "che finishing": `shooting` è la parola del gioco. `finishing` è il nome del prototipo (`pcfm-hidden/src/lib.rs:158, 223, 239`).
**Luna — cosa rivela.** **Il prototipo non è una spec.** Quando i due divergono, vince la spec (L0.11 §7.2), non il codice che gira. La stessa trappola può esistere su ogni altro nome (`passing`, `tackling`).
**Stato:** `verificato`. `grep -nE 'finishing|shooting' crates/pcfm-hidden/src/lib.rs L0-match/design/COVERAGE.md L0-match/specs/11-player-characteristics.md` mostra `finishing` in codice e `shooting` in §7.2/§4.6. Nota 2026-11-08 §532 dichiara "prototype form" per `passing`/`shooting`/`tackling`, non per `finishing`.
**Prima azione.** Decidere se rinominare `finishing` → `shooting` (bump `SIM_VERSION` solo se cambia il comportamento; rename = no) o dichiarare `finishing` un alias dichiarato. Model choice (P6).

### 2.3 `consistency` in §7.3 ma non in §4.6

**Natura:** spec interna. **Partita vera:** non si applica. **Luna:** si applica.
**Luna — cosa rivela.** Due liste nella stessa specifica non si parlano: **il catalogo non è indicizzato**. Un termine può stare in §7.3 (trait) e mancare in §4.6 (catalogo) senza che nessun gate lo veda.
**Stato:** `parziale`. In `L0-match/design/COVERAGE.md`, non in un OI.
**Prima azione.** Un `grep` incrociato che elenca, per ogni termine di §7.3, se è anche in §4.6. Stessa prima azione di 2.1.

### 2.4 Pressure non legge nessuna caratteristica

**Natura:** modello. **Partita vera:** si applica. **Luna:** si applica.
**Partita vera.** La pressione in partita vera è una **relazione** fra attaccante, portatore e spazio (chi arriva, da dove, con che velocità), non un tratto individuale. Che `Pressure` non legga una caratteristica è **corretto**.
**Luna — cosa rivela.** "Ogni evento ha un campo che lo spiega" è un'affermazione da raffinare: **la copertura non si misura in latenti**. Un evento può essere spiegato da una geometria e da nessuna caratteristica. Chi misura la copertura contando le caratteristiche lette misura la cosa sbagliata.
**Stato:** `verificato`. `Pressure` è emessa in `state.rs` e non legge `PlayerLatents` per costruzione (relazione pura, ADR-036).
**Prima azione.** Nota in `L0-match/design/DECISION-TABLE.md` §4: la copertura si misura in "eventi con una causa dichiarata", non in "eventi con una caratteristica letta". Nessuna azione sul codice.

### 2.5 MaximalForce e Acceleration non hanno un campo dietro

**Natura:** dato. **Partita vera:** non si applica. **Luna:** si applica.
**Luna — cosa rivela.** OI-92 dice "rimuovere o dare un valore ai 3 termini a 0.0", ma **il dato non esiste**: `PhysicalLatents` ha `max_speed` e non ha `maximal_force` né `acceleration`. Non è "da materializzare": è **da togliere o da dichiarare**. La differenza conta: materializzare aggiunge struttura senza contenuto (D3); togliere chiude il bug.
**Stato:** `verificato`. `grep -nE 'maximal_force|acceleration|max_speed' crates/pcfm-hidden/src/latents.rs` mostra solo `max_speed`. OI-92 va corretta.
**Prima azione.** Nota in OI-92: due termini su tre non hanno un campo dietro; vanno tolti da `Characteristic` o dichiarati `Removed-by:` (D3).

## 3. Dove vivono §3 e §4 del report sorgente

Il report che ha aperto questo file aveva altre due sezioni. **Non** stanno qui, per non duplicare (D7):

- **§3 (trappole di metodo non ancora in B1-bis).** Sono andate in `NEW-SESSION-PROMPT.md` B1-bis con lo stesso commit che ha scritto questo file (task successivo della sessione).
- **§4 (il pattern del maintainer).** Va in `NOTA-PER-NUOVA-AI.md` con lo stesso commit, filtrato da `dup-check` e dalla regola 200 righe.

**§5 del report (cosa fa un'AI nuova) non va scritto**: è già l'E1 + §0 di questo file. Un elenco che ripete E1 è rumore.
