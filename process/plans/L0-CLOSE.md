# Piano — chiudere L0

**Obiettivo (maintainer, 2026-11-13):** completare L0 in questa sessione, con una base solida e testata, prima di L1.
**Metodo (maintainer):** (1) lo scheletro del task; (2) passo per passo, ogni passo tarato e filtrato attraverso *partita vera* e *luna*; (3) alla fine i test, poi la taratura fine con il maintainer; (4) task completato, si passa al prossimo.

**Definizione di L0 finito** (`doctrine/_index.md`): un'amichevole giocabile, salvabile, ricaricabile, deterministica, in cui ogni evento ha una causa leggibile.

**Ogni passo:** SIM_VERSION, golden, CHANGELOG-SIM, P1 e test dei gemelli, gate verdi, segnalibro, push. Misura su 30 seed (42-71) confrontata con il reale; il filtro *partita vera*: un tifoso lo riconoscerebbe? il filtro *luna*: che cosa indica, oltre la lettera?

## Riferimenti reali (per partita, entrambe le squadre)

Gol ~2,7 · tiri ~25 · nello specchio ~9 · angoli ~10 · falli ~22 · gialli ~4 · rossi ~0,15 · rigori ~0,3 · passaggi ~900-1.000 · rimesse laterali ~40 · cross ~35 · fuorigioco ~4.

## I passi

| # | Passo | Cosa | Fatto quando | Stato |
|---|---|---|---|---|
| 1 | **La difesa pensa** (ADR-052 d) | ogni difensore sceglie l'atto che toglie più pericolo: chiudere sul portatore, marcare l'uomo, tenere lo spazio della struttura; il pericolo è la qualità dell'occasione per gli avversari | gol e tiri tornano verso il reale senza pesi per atto | **fatto** (68) |
| 2 | **L'attacco senza palla pensa** | smarcarsi e attaccare la profondità valgono lo spazio che aprono (linee, mezzi spazi, ampiezza), non un peso | i pesi `offer`/`run` spariscono, la manovra resta | prossimo |
| 3 | **Spazzata, pallonetto, altro palo** | la spazzata, il pallonetto (sul portiere fuori dai pali) e il tiro sull'altro palo giudicati con la stessa misura | via `clear_salience` e `other_side_factor` | |
| 4 | **In aria** | il colpo di testa sul cross e sul lancio (in area è un tiro, dietro è un rinvio), il duello aereo, il portiere che esce sul cross; radici `heading`, salto | i cross diventano occasioni e angoli | |
| 5 | **Il portatore risponde** (ADR-042 c) | proteggere palla, dribblare; radice `dribbling` | dribbling e protezione riconoscibili (gemelli) | |
| 6 | **La fatica** | si consuma con la corsa e gli sprint, rallenta e attenua, recupera in parte all'intervallo; radice `endurance` | il secondo tempo si riconosce | |
| 7 | **Palle inattive vere** | chi batte lo sceglie la squadra (il miglior calciatore vicino), punizione e rigore calciati come tiri, angolo come cross | i calci piazzati producono occasioni | |
| 8 | **Test di repertorio** | ogni gesto del catalogo compare almeno una volta nei 30 seed | test verde, ogni assenza spiegata | |
| 9 | **L'amichevole giocabile** | da riga di comando: scegli gli undici e l'intenzione (blocco alto/medio/basso, ampiezza), guardi la partita, salvi, ricarichi, e alla fine leggi *perché* hai subito gol | il maintainer gioca un'amichevole | |
| 10 | **Misura finale e consegna** | 30 seed contro il reale, resoconto per la taratura fine | il maintainer tara, L0 chiuso | |
