# Piano — chiudere L0

**Obiettivo (maintainer, 2026-11-13):** completare L0 in questa sessione, con una base solida e testata, prima di L1.
**Metodo (maintainer):** (1) lo scheletro del task; (2) passo per passo, ogni passo tarato e filtrato attraverso *partita vera* e *luna*; (3) alla fine i test, poi la taratura fine con il maintainer; (4) task completato, si passa al prossimo.

**Definizione di L0 finito** (`doctrine/_index.md`): un'amichevole giocabile, salvabile, ricaricabile, deterministica, in cui ogni evento ha una causa leggibile.

**Ogni passo:** SIM_VERSION, golden, CHANGELOG-SIM, P1 e test dei gemelli, gate verdi, segnalibro, push. Misura su 30 seed (42-71) confrontata con il reale; il filtro *partita vera*: un tifoso lo riconoscerebbe? il filtro *luna*: che cosa indica, oltre la lettera?

## Lo stato vero e ciò che resta (riscritto il 2026-11-15, dopo il ricontrollo `L0-AUDIT.md`)

**Mandato del maintainer (2026-11-15):** completare L0 in cinque fasi: ricontrollo, aggiornamento, il calcio che manca, la taratura fine (compito di Claude Code da ora), la chiusura con la partita di prova. L1 non si apre prima.

**Fatto:** passi 1-9; nel 10, le famiglie 10a, 10b, 10c, 10g; il 10d, 10e e 10f in parte; il ricontrollo (fase 1) e l'aggiornamento dei documenti (fase 2). Il motore è al `SIM_VERSION` 114 (448 test).

**Fase 3, il calcio che manca, in quest'ordine** (ogni voce con il suo `SIM_VERSION`, i gemelli, il golden su glibc e musl; la misura sulla finestra 42..101 solo quando serve una decisione sulle metriche, non come rito di chiusura — RULES C2, 2026-11-20):

| # | Voce | Dipende da | Prima azione |
|---|---|---|---|
| 1 | **fatto (115)** — La memoria minima di ciò che si è visto: l'ultimo sguardo (ADR-034, MD-40) | — | per ogni giocatore, dove ha visto ogni altro al suo ultimo sguardo; nel salvataggio e nell'hash |
| 2 | **fatto (115)** — La scena solo dal cono e dalla memoria (OI-106) | 1 | il test anti-fuga, rosso; poi `rank`, `loose_rival_cm`, `cutters` dalla memoria |
| 3 | **(c) fatto (116, dalla memoria); (a) aperto, OI-108** — ADR-062 (a) e (c): l'ampiezza dell'attenzione come `vision`, la linea letta dall'ultimo sguardo (fonti in OI-104, `SOURCES.md`); rimisurare fuorigioco e palloni che escono (OI-98, OI-100) | 1, 2 | il gemello su `vision` |
| 4 | **fatto (117)** — Supplementari e rigori finali (Regole 7 e 10, testo 2026/27), quando la competizione li chiede; la monetina come input della partita (approvata il 2026-11-15) | — | un `Match` di coppa finito pari: i supplementari, poi la serie |
| 5 | **fatto (122); il coraggio nella barriera resta, OI-109** — La barriera: la somma di corpi, ognuno sceglie se entrarci e dove, resta o si gira secondo il coraggio (OI-105) | 8 (il coraggio) | la distanza della Regola 13 e il metro dalla barriera |
| 6 | **cambi fatti (118); istruzioni in partita e ADR-059 passo 3 aperti** — L'allenatore avversario che reagisce in partita (ADR-059 passo 3, poi cambi e istruzioni dallo stato) | ADR-059 passo 3 | il passo 3: istruzioni rapide e transizioni |
| 7 | **bloccato, OI-110** — Il colpo che fa uscire un giocatore: `Injury` emesso dalla forza del contatto, `injury_proneness` col suo consumatore | — | la fonte (studio UEFA, Ekstrand) e la misura del contatto nel duello |
| 8 | **coraggio e compostezza fatti (119)** — Gli appannatori: lucidità, coraggio, aggressività (OI-70, ADR-057) | — | il coraggio come radice fissa, col gemello |
| 9 | Le cause aperte del realismo: OI-102 (il secondo uomo in aria), OI-103 (protezione e dribbling), OI-64 (il corpo rispetto alla palla), 10d (il portiere che si piazza, esce), 10e (il tempo del salto), la tabella di ADR-057 | — | una per volta, la causa nel calcio vero prima del codice |

**Fase 4, la taratura fine:** prima due proposte di dottrina al maintainer (RULES A6, la taratura a Claude Code; RULES A5-bis con i tre gradi della decisione 5), e si aspetta il suo ok; poi ogni parametro di grado 3 (356 su 381, `L0-AUDIT.md` §7) verso il grado 1 o 2, la misura sulla finestra di taratura con le bande scritte prima, ogni metrica fuori banda con la sua causa o il suo OI; la tabella in `L0-CALIBRATION.md`.

**Stato al 2026-11-15:** fasi 1-5 svolte; L0 chiuso, in attesa della prova del maintainer (`STATE.md`, "Per il maintainer"); ciò che resta è negli OI-64, OI-100-OI-112.

**Fase 5, la chiusura:** la misura finale (passo 11), il golden su glibc, musl e dove si può Windows, macOS, ARM64 (OI-99; ARM64 anche nella CI, decisione P), la durata di una partita su un core, la partita di prova del maintainer. **Quando L0 si chiude (decisione L del 16 novembre):** almeno 16 metriche su 22 in banda (`L0-CALIBRATION.md`), e ogni metrica fuori banda con la sua causa scritta e il suo OI; il resto del realismo continua dopo, senza bloccare (decisione M: prima completo, poi perfetto). Poi la partita di prova del maintainer; L0 è chiuso dopo il suo ok. **Dopo L0:** la vista 2.5D (decisione R, `context/plans/GUI-DESIGN.md`), poi L1 (`GAME.md`).

---

*Sotto, la storia del piano com'era, per riferimento.*

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
| 10 | **Il motore completo** (maintainer, 2026-11-13: *"prima completa il motore, poi il realismo; i numeri sempre derivati dalle abilità, mai inventati"*) | ogni gesto del catalogo con la sua forma piena, nelle famiglie qui sotto (10a-10h); nessuna statistica guardata | ogni riga di `ACTIONS.md` ✓, il test di repertorio senza assenze dichiarate, e ogni caratteristica con il suo consumatore, il suo gemello e la sua riga nella tabella del ritorno dal calcio reale (ADR-057, `CHARACTERISTICS-AND-ABILITIES.md` §8) | prossimo |
| 11 | **Il realismo derivato** — in corso (87: il tiro in corsa, di traverso, stanco: in porta 95% → 64%; 88: il tiratore mette in conto il suo tiro e mira meglio, 80%: la causa che resta è una che non può prevedere, la palla non incollata al piede; 89: la palla toccata, non portata (ADR-053): contrasti vinti 8 → 23, gol 3.3; il tiro in porta resta 70%, gol + parate, la sua causa è altrove; 90: il tiratore stima il suo errore di base come legge la partita; 91: il tiro è una dispersione, il contatto sceglie dove, ADR-054: in porta 51%; 92: il cross anche basso, come nella definizione reale; 93: la palla vinta presa dove è, dal primo controllo; 94: il portatore ritocca la palla entro il suo allungo; 95: ogni scelta ha un quando, tenere palla per il momento migliore, ADR-056; 96: il passaggio sulla corsa, il portiere che legge il cross; 97: la palla in aria letta dove scende, dal calcio; 98: la spazzata dove la sua squadra arriva prima; 99: la palla che rotola presa sulla sua traiettoria; 100: la Regola 10 per ogni pallone) | solo ora: 30 seed contro il reale, e per ogni distanza la causa nelle abilità dei giocatori, mai un numero aggiustato (ADR-052 nota) | il maintainer gioca e approva, L0 chiuso | |

### Passo 10, le famiglie che mancano (da `ACTIONS.md`)

| | Famiglia | Gesti |
|---|---|---|
| 10a | **Il passaggio nei suoi tipi** — fatto (79, 80) | il filtrante (nello spazio davanti a chi attacca la profondità), il cambio di gioco, il cross come scelta; la conduzione con la sua velocità (dribbling) e la sua direzione |
| 10b | **Il pensiero intero** (ADR-052 e) — fatto (81, 82) | visione (quanti e quanto lontano vede), anticipazione (quanto avanti immagina), creatività (le opzioni che pochi vedrebbero), concentrazione (l'appannatore che cala) |
| 10c | **Senza palla** — fatto (83, 84) | la copertura e la corsa all'indietro, l'intercetto come scelta (entrare nella linea), il passaggio respinto, lo smarcamento che legge la linea libera; disciplina e generosità (work rate) |
| 10d | **In porta** — in parte (85: le mani e il rinvio con le mani; aperti con la causa mancante: il piazzarsi al momento del tiro, la profondità letta, l'uscita sul passaggio) | la posizione letta (non una regola), pronto o no quando parte il tiro (concentrazione), l'uscita come scelta anche sul filtrante e sul cross, il rinvio con le mani e con i piedi |
| 10e | **Il corpo** — girarsi fatto (86); il tempo del salto aperto | girarsi (mobilità articolare), il tempo del salto |
| 10f | **L'allenatore in partita** — le sostituzioni fatte (pianificate prima, fatte a palla ferma); cambiare il piano durante la partita resta per L1 | le sostituzioni |
| 10g | **L'arbitro disegnato** — fatto | presente nella vista 2D, senza occlusione |
| 10h | **I pezzi che L1 userà subito** (maintainer, 2026-11-15), dopo il resto del passo 10 | (a) supplementari e rigori finali (Regole 7 e 10 dell'IFAB): chi batte e in che ordine lo sceglie la squadra, il portiere legge come in partita, nessun dado; un golden con una partita di coppa finita pari; (b) la barriera sulle punizioni (Regola 13: la distanza; i tre attaccanti fuori dalla barriera se il testo in vigore lo prevede); quanti uomini e dove li ricava la difesa, proposto al maintainer prima di scriverlo; (c) l'allenatore avversario che reagisce in partita (cambi e istruzioni dallo stato: fatica, cartellini, risultato, minuto), un ADR proposto, dopo il passo 3 dell'ADR-059; (d) il colpo che fa uscire un giocatore (l'evento `Injury` senza produttore), un ADR proposto sul fatto misurabile che lo porta, con la fonte; lo stop e il rientro sono L1 |


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
