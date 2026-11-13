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
| 10 | **Il motore completo** (maintainer, 2026-11-13: *"prima completa il motore, poi il realismo; i numeri sempre derivati dalle abilità, mai inventati"*) | ogni gesto del catalogo con la sua forma piena, nelle famiglie qui sotto (10a-10g); nessuna statistica guardata | ogni riga di `ACTIONS.md` ✓, il test di repertorio senza assenze dichiarate | prossimo |
| 11 | **Il realismo derivato** | solo ora: 30 seed contro il reale, e per ogni distanza la causa nelle abilità dei giocatori, mai un numero aggiustato (ADR-052 nota) | il maintainer gioca e approva, L0 chiuso | |

### Passo 10, le famiglie che mancano (da `ACTIONS.md`)

| | Famiglia | Gesti |
|---|---|---|
| 10a | **Il passaggio nei suoi tipi** — fatto (79, 80) | il filtrante (nello spazio davanti a chi attacca la profondità), il cambio di gioco, il cross come scelta; la conduzione con la sua velocità (dribbling) e la sua direzione |
| 10b | **Il pensiero intero** (ADR-052 e) | visione (quanti e quanto lontano vede), anticipazione (quanto avanti immagina), creatività (le opzioni che pochi vedrebbero), concentrazione (l'appannatore che cala) |
| 10c | **Senza palla** | la copertura e la corsa all'indietro, l'intercetto come scelta (entrare nella linea), il passaggio respinto, lo smarcamento che legge la linea libera; disciplina e generosità (work rate) |
| 10d | **In porta** | la posizione letta (non una regola), pronto o no quando parte il tiro (concentrazione), l'uscita come scelta anche sul filtrante e sul cross, il rinvio con le mani e con i piedi |
| 10e | **Il corpo** | girarsi (mobilità articolare), il tempo del salto |
| 10f | **L'allenatore in partita** | le sostituzioni |
| 10g | **L'arbitro disegnato** | presente nella vista 2D, senza occlusione |


## Passo 11 — la misura (SIM_VERSION 76), da riprendere a motore completo

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

**Cosa non lo è, e la causa da cercare** (metodo del maintainer, 2026-11-13, ADR-052 nota, RULES A5-bis: le statistiche si derivano dalle abilità, non si aggiustano; nessun numero si sceglie guardando la statistica che produce):

| # | Che cosa | La causa da cercare (nel modo in cui i giocatori percepiscono, pensano, si muovono) |
|---|---|---|
| 1 | **Corrono troppo** (13,2 km a partita, un terzo del tempo sopra 4 m/s, contro 10-11 km e 15%): finiscono stanchi a 0,9 — **fatto (78)**: il corpo sui valori dei professionisti, la diagonale, la frenata, il passo scelto; ora 9,5 km, 60/26/9/3/1%, tiri 29 | nessuno sceglie il passo: ogni corsa è a tutta velocità. Il giocatore deve scegliere quanto correre come sceglie dove (ADR-052): di corsa se arrivare prima vale, al passo se no |
| 2 | **Tiri troppi** (64 contro 25), gol troppi (8 contro 2,7) | il valore del tenere palla non è ancora derivato dal motore (ADR-052 nota, punto 3: quanto spesso un possesso da lì finisce in gol nelle partite del motore); poi, se restano troppi, che cosa il tiratore non vede |
| 3 | **Tiri in porta troppi** (91% contro 34%) | l'errore del tiro (`kick.json` `error_ref_rad`) è arbitrario: va fondato sugli studi della precisione del calcio |
| 4 | **Contrasti vinti pochi** (7 contro 30), intercetti tanti | il difensore entra poco; un passaggio perso è contato come intercetto (definizione da allineare al reale) |
| 5 | **Cross e duelli aerei pochi** | il valore dello spazio in area (segue il 2); chi va sulla palla alta |
| 6 | **Recupero lungo, gialli tanti** | seguono il 2; i secondi per evento e la soglia dell'imprudenza vanno fondati sul regolamento e sui dati arbitrali |

**L'ordine:** 1 fatto (78); poi la fonte dell'errore del tiro (86% in porta contro 34%), poi il valore del possesso derivato; misurando dopo ognuno, senza ritoccare.
