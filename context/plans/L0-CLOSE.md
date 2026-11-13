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
| 2 | **L'attacco senza palla pensa** | smarcarsi e attaccare la profondità valgono lo spazio che aprono (linee, mezzi spazi, ampiezza), non un peso | i pesi `offer`/`run` spariscono, la manovra resta | **fatto** (69) |
| 3 | **Spazzata, pallonetto, altro palo** | la spazzata, il pallonetto (sul portiere fuori dai pali) e il tiro sull'altro palo giudicati con la stessa misura | via `clear_salience` e `other_side_factor` | **fatto** (70) |
| 4 | **In aria** | il colpo di testa sul cross e sul lancio (in area è un tiro, dietro è un rinvio), il duello aereo, il portiere che esce sul cross; radici `heading`, salto | i cross diventano occasioni e angoli | **fatto** (71) |
| 5 | **Il portatore risponde** (ADR-042 c) | proteggere palla, dribblare; radice `dribbling` | dribbling e protezione riconoscibili (gemelli) | **fatto** (72) |
| 6 | **La fatica** | si consuma con la corsa e gli sprint, rallenta e attenua, recupera in parte all'intervallo; radice `endurance` | il secondo tempo si riconosce | **fatto** (73) |
| 7 | **Palle inattive vere** | chi batte lo sceglie la squadra (il miglior calciatore vicino), punizione e rigore calciati come tiri, angolo come cross | i calci piazzati producono occasioni | **fatto** (74) |
| 8 | **Test di repertorio** | ogni gesto del catalogo compare almeno una volta nei 30 seed | test verde, ogni assenza spiegata | **fatto** (76) |
| 9 | **L'amichevole giocabile** | da riga di comando: scegli gli undici e l'intenzione (blocco alto/medio/basso, ampiezza), guardi la partita, salvi, ricarichi, e alla fine leggi *perché* hai subito gol | il maintainer gioca un'amichevole | **fatto** (prima forma: da giocare) |
| 10 | **Misura finale e consegna** | 30 seed contro il reale, resoconto per la taratura fine | il maintainer tara, L0 chiuso | prossimo |

## Passo 10 — la misura finale, per la taratura con il maintainer (SIM_VERSION 76)

Trenta partite, seed 42-71, la squadra di default (blocco medio, ampiezza normale).

| per partita, entrambe le squadre | Calcio Vero (seed 42-71) | partita vera (ordine di grandezza) |
|---|---|---|
| gol | 7.1 | 2.7 |
| tiri | 73.7 | 25 |
| tiri in porta | 55.4 | 8.5 |
| parate | 56.4 | 6 |
| angoli | 16.4 | 10 |
| falli | 22.1 | 24 |
| gialli | 8.6 | 4.5 |
| rossi | 0.5 | 0.15 |
| fuorigioco | 2.5 | 4 |
| passaggi | 1372.5 | 900 |
| dribbling riusciti | 22.8 | 15 |
| contrasti vinti | 7.4 | 30 |
| cross | 12.7 | 35 |
| duelli aerei | 3.3 | 35 |
| rigori | 0.2 | 0.3 |
| rimesse laterali | 50.7 | 45 |
| spazzate | 75.4 | 40 |
| tiri respinti | 8.5 | 7 |
| intercetti | 153.4 | 20 |
| passaggi riusciti % | 90 | 82 |
| recupero (minuti) | 15.8 | 7 |

**Cosa è già vicino al reale:** falli, rigori, rimesse laterali, tiri respinti, dribbling, la percentuale di passaggi riusciti (un po' alta), il 10% di conversione dei tiri.

**Cosa non lo è, e la leva che lo muove** (nessuna è un peso per tipo di opzione: ADR-052):

| # | Che cosa | Perché, per come lo leggo | La leva proposta |
|---|---|---|---|
| 1 | **Tiri troppi (74 contro 25), quindi gol troppi (7 contro 2,7)** | tenere palla vale poco rispetto a un tiro: un tiro al 5% batte un passaggio laterale | `thought.json` `possession_value` (0,02): quanto vale tenere palla contro un gol; il reale (xT) dice 0,03-0,1 vicino all'area |
| 2 | **Tiri in porta troppi (75% contro 34%)** | l'errore del tiro è piccolo (`kick.json` `error_ref_rad` 0,08); a 0,2 i tiri in porta erano il 38% | `error_ref_rad`, insieme alla leva 1 (da sola faceva più gol) |
| 3 | **Contrasti vinti pochi (7 contro 30), intercetti tanti (153 contro 20)** | il difensore temporeggia più che entrare; molti passaggi persi sono contati come intercetti | il pensiero del difensore (passo 1): quanto vale entrare; e la definizione di intercetto |
| 4 | **Cross e duelli aerei pochi (13 e 3 contro 35 e 35)** | il cross vale poco nel pensiero del portatore; un duello aereo vuole due giocatori sotto la palla nello stesso istante | il valore dello spazio in area (segue la leva 1) e chi va sulla palla alta |
| 5 | **Recupero lungo (16 contro 7 minuti)** | `stoppage_time.json`: 60 s a gol, 20 s a fallo, e i gol sono troppi | segue la leva 1; poi i secondi per evento |
| 6 | **Gialli troppi (8,6 contro 4,5)** | la scivolata da dietro è spesso imprudente | `laws.json` `reckless_speed_milli` |

**L'ordine che propongo:** 1, poi 2, misurando ogni volta i 30 seed; poi 3-6. Ogni passo è un numero in `balance/`, con la fonte o "arbitrario fino a L3".

**Prima della taratura:** giocare un'amichevole (`README`, *Try the engine*) e dire che cosa non sembra calcio.
