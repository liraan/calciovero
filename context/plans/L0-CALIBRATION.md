# L0 — la taratura fine (2026-11-15)

**Cos'è.** La fase 4 del mandato del maintainer del 2026-11-15: il motore misurato contro il calcio reale, metrica per metrica, con la sua banda e, per ogni metrica fuori banda, la causa nel modello o l'OI che la cerca. Nessun numero è stato ritoccato per rientrare in una banda (RULES A5-bis).

## Come si misura

- **Strumento:**
  ```bash
  cargo run --release -p ds-reference -- compare --reference L0-match/reference/serie-a-2017-18.json --seeds 42..101
  ```
  Finestra di taratura 42..101 (60 partite), squadre di prova, blocco medio e ampiezza normale. Non è un numero statistico: la banda è già la dispersione della partita vera (decisione Y), la finestra serve solo a rileggere la stessa partita più volte e a confrontare misure diverse. **Non fa parte dei controlli di chiusura** (RULES C2, 2026-11-20): si esegue quando serve una decisione sulle metriche, non a ogni intervento.
- **Riferimento:** Serie A 2017-18, 380 partite, Wyscout (Pappalardo et al. 2019, *Scientific Data* 6, 236; CC BY 4.0), per partita ed entrambe le squadre.
- **Motore:** `SIM_VERSION` 185 (in codice; il 185 sposta i `Threats` da `ds-match` all'orchestratore con aritmetica identica, nessuna misura nuova). L'ultima misura, del 2026-11-21 su 60 partite 42..101, è del **184** (dopo DD-41/ADR-099: gol 13.20, passaggi 1526.27, duelli aerei 15.77, rimesse 30.62; **5 su 22**), ed è nella sezione "La misura del 184" qui sotto. La tabella storica sotto è l'ultima misura riga per riga del **171** (6 su 22), lasciata come confronto: le sue cause e i suoi OI valgono ancora dove non sono stati chiusi. SIM 167 (il possesso è il movimento) e SIM 168 (solo osservatori, golden identico) non cambiano il conteggio. Storia delle misure: al 168, dopo ADR-084 e ADR-085 (gol 4.80, passaggi 1237, duelli aerei 10.82, rimesse 17.87, 6 su 22); al 166, la palla libera conserva la velocità e continua a rotolare (gol 5.22, passaggi 1254, duelli aerei 10.62, rimesse 18.08, 6 su 22); al 165, l'emendamento ADR-071 sul posto senza palla letto dal giocatore (gol 5.22, duelli aerei 10.85, rimesse 18.27, 6 su 22); al 164, la scelta C di OI-102 (ADR-081): gol 5.25, duelli aerei 11.02, 6 su 22; al 163, i calci morti e il battitore scelto per ruolo (DD-21, ADR-080): gol 5.20, duelli aerei 10.82, 6 su 22; al 154, la posizione senza palla (ADR-071): 9 su 22; al 151, 11 su 22; al 147, i passi piccoli (ADR-069): gol 2.95, rimesse 25.9, 11 su 22; al 141, il colpo che costringe al cambio (ADR-066): 12 su 22. La taratura è ferma finché i comportamenti non sono giusti (decisione BK). La misura del 116, del 117 e del 118 è identica riga per riga: il 117 e il 118 cambiano solo le partite di coppa e le squadre con l'allenatore che reagisce. **SIM 170** (la pressione rimette alla prova il controllo del portatore, ADR-086) non è stato misurato separatamente: la misura 42..101 è stata fatta al 171, quando il difetto angolare di OI-100 chiedeva una decisione sulle metriche, non un rito di chiusura (RULES C2).
- **La banda:** media reale ± una deviazione standard delle 380 partite reali. **Il suo significato (decisione Y, maintainer, 2026-11-16):** L0 si chiude sulla credibilità della singola partita: una partita vera può finire 0-0 o 4-3, e la dispersione delle partite vere dice cosa è credibile. L1 si chiuderà sulle medie del campionato, con la banda sull'errore della media su una stagione intera, perché i totali di stagione (gol, capocannoniere) mostrano ogni scarto nelle medie. La fonte è lo stesso riferimento. È una regola sola per tutte le metriche, presa dalla dispersione del calcio vero e non dai valori del motore. Il caos del motore (circa 1.5 gol fra due esecuzioni che differiscono di un bit, SIM 101) sta dentro questa larghezza.
- **Non confrontabili:** cross e intercetti. Le definizioni Wyscout e del motore differiscono (`ds-reference`, nota della tabella).

## I due filtri e il test del burattino (maintainer, 2026-11-19)

- **Partita vera** = confrontare ciò che si vede nel gioco con ciò che si vedrebbe
  in una partita reale, annotare le differenze e da lì capire cosa migliorare.
- **LUNA** = non fermarsi al dito: espandere la prospettiva e guardare la bigger
  picture a cui i fatti puntano (il saggio indica la luna, lo stolto guarda il dito).
- **Obiettivo (maintainer, 2026-11-19):** non 22 burattini belli e ordinati in campo,
  ma 22 giocatori belli e ordinati **perché loro stessi sanno cosa fare in ogni
  situazione di campo**: ognuno con la sua storia di formazione, le sue abilità e
  la sua cognizione calcistica evoluta. L'ordine va letto come **effetto** della
  cognizione, non imposto dall'esterno come causa. Per questo ADR-071 (il posto
  senza palla pesato dalla disciplina) è il primo salto negativo della bisection:
  più ordine esterno, meno calcio.
- **Test del burattino — criterio di accettazione di ogni causa.** Nella stessa
  scena, cambiare solo la lettura (o l'intesa, o l'abilità) del giocatore: se
  l'atto non cambia, il giocatore non stava decidendo, stava obbedendo a una
  forma; se cambia, stava giocando a calcio. I twin test già presenti
  (`swapped_wide_men_strike_with_the_feet_they_have`,
  `a_teammate_he_knows_yields_the_loose_ball`) sono questo criterio applicato al
  piede e all'intesa; valgono come gate dell'obiettivo, non solo come regressione.
- **Regola di sequenza:** ogni causa deve passare **sia** il test del burattino
  **sia** la misura sulle stesse 60 partite. Una causa che alza le metriche ma
  lascia l'atto invariato al cambiare della lettura è un burattino più ordinato
  e va respinta. La soglia 16/22 resta la misura; il giudizio finale è lo
  sguardo sul campo: *si vede un giocatore che ha giocato migliaia di ore?*

## La cipolla (maintainer, 2026-11-19)

La partita è una cipolla, e il filtro LUNA è il modo di guardarla: **ciò che si
vede è la cipolla intera** (la *partita vera*: il confronto con una partita
reale si fa sull'insieme, non su una riga); dentro ci sono gli strati, ognuno
un fattore che la forma. Fermarsi alla superficie è guardare il dito; seguire
gli strati fino al nocciolo è guardare la luna.

Gli strati sono quelli che il progetto ha già, dal più interno al più esterno
(`BEHAVIOURS.md`, `STRUCTURE.md`: le dipendenze fluiscono verso l'interno — non
è una gerarchia nuova):

| Strato della cipolla | Fattore | Dove vive già |
|---|---|---|
| **Nocciolo** — formazione calcistica cognitiva del giocatore | come si gioca a pallone: scopo, misura del gioco, pallone/spazio/tempo, lettura dell'avversario | Strato 1, ADR-052; abilità e anni di formazione nei latenti |
| **Ruoli e coppie** | chi occupa quale spazio; intesa, movimenti complementari, divisione dei compiti | Strati 2 e 3, ADR-071, ADR-073, ADR-074 |
| **Tattica** | la lavagna che coordina i movimenti di squadra, sposta le zone, aggiunge responsabilità | Strato 4 (AL, AG, AO, AP), ADR-070 |
| **Il mondo** | allenatore, staff tecnico, allenamento, società, tifosi, media | L1-L3 (season, career, meta): il contesto che nutre la cipolla da fuori |

**I tre criteri letti insieme con LUNA.** La cipolla si costruisce dal nocciolo
verso l'esterno, ma ogni strato esterno è **letto** dal nocciolo, non imposto ad
esso: la forma dà la zona da cui il giocatore guarda (ADR-071 §1), l'allenatore
dà responsabilità che il giocatore pesa con la sua disciplina, l'intesa fa
prevedere il compagno senza copione. È la differenza fra ordine-imposto
(burattino) e ordine-effetto-della-cognizione (obiettivo). Uno strato che
sostituisce la lettura del nocciolo con un punto o un copione è un burattino,
anche se la cipolla è ordinata.

**Regola di sequenza delle cause — dal nocciolo verso l'esterno.** Si aggiusta
prima il nocciolo (il giocatore che sa cosa fare), poi via via gli strati che
lo avvolgono; non si mette tattica o disciplina a mascherare un nocciolo che
non legge. È la luna della bisection: ADR-071 (posto senza palla, uno strato
esterno che imponeva il punto) è il primo salto negativo 12/22 → 9/22; OI-102
(il secondo uomo che non legge la palla alta) è un danno al nocciolo e viene
prima. Ogni causa va localizzata nello strato giusto e passa il test del
burattino (il nocciolo decide ancora?) **e** la misura sulle 60 partite.

## La demo, la simulazione e la sensazione di partita reale (maintainer, 2026-11-19)

Alla fine è un **gioco simulativo**, ed è questa la luna sotto la demo. La demo
ha parti abbozzate, perché interi strati della cipolla non sono ancora
sviluppati nel gioco (il mondo L1-L3 — carriera, società, tifosi — è contesto,
non richiesto in L0). Ma essere una demo non cambia la natura del prodotto: la
**sensazione di una partita reale** non è un effetto scenico da rifinire, è
l'esito del modello che gira. La dottrina lo dice (`00-foundations.md` §4-5:
determinismo per costruzione, nessun dado, esiti dalle transizioni locali ed
emergenti degli attori): qui non si recita una partita, la si simula.

Il vincolo non è la completezza degli strati né la lucidatura, è la qualità del
nocciolo. Uno strato esterno può essere grezzo — una lavagna semplice,
un'intesa iniziale, una forma di base — senza che la partita sembri falsa,
purché il nocciolo legga: 22 giocatori che scelgono da ciò che vedono, con le
loro abilità, e da cui l'ordine in campo **emerge**. Se il nocciolo è un
burattino, nessuna rifinitura degli strati esterni restituisce la sensazione di
simulazione; se il nocciolo legge, anche una demo con strati abbozzati si guarda
come calcio vero.

È il giudizio finale già scritto sopra (*si vede un giocatore che ha giocato
migliaia di ore?*) detto in termini di simulazione: **la misura è la sensazione
che il modello produce, non la completezza né la vetrina**. Le 60 partite e la
soglia 16/22 misurano quella sensazione; il test del burattino la garantisce.
Non si giustifica un comportamento da burattino dicendo "è solo una demo": in
un gioco simulativo è proprio quello che non può succedere.

## La tabella

| Metrica | Reale | ± | Motore 171 | Banda | Dentro? | Causa o OI |
|---|---|---|---|---|---|---|
| gol | 2.68 | 1.65 | 5.07 | 1.03–4.33 | no | OI-100: dal 163 le ripartenze libere e i piedi migliori (da tarare a L3); il tiro troppo preciso |
| autogol | 0.10 | 0.31 | 0.85 | 0–0.41 | no | OI-100: dal 132 la spazzata sotto pressione devia di più; da guardare |
| tiri | 24.60 | 5.46 | 32.35 | 19.1–30.1 | no | OI-100: troppi tiri, dalla catena dei passaggi e delle ripartenze |
| tiri in porta | 8.69 | 3.22 | 13.20 | 5.5–11.9 | no | ADR-087/DD-27 ha portato lo specchio sotto gli 11 m dal 96% al 54,6% (reale 46%) senza ritoccare il conteggio; restano troppi tiri in tutto. `ds-reference` conta i tiri respinti fuori dallo specchio, come Wyscout e StatsBomb |
| rigori | 0.33 | 0.56 | 0.03 | 0–0.89 | sì | — |
| passaggi | 887.7 | 122.8 | 1254.63 | 765–1011 | no | OI-100: troppi passaggi, e troppi intercettati (157 a partita); `ds-reference` conta anche i passaggi intercettati, che il motore racconta solo come `Interception` |
| passaggi riusciti | 739.2 | 124.8 | 1009.67 | 614–864 | no | OI-100: troppi passaggi in tutto, con una quota di riuscita inferiore all'83,3% vero |
| duelli aerei | 38.14 | 12.98 | 10.52 | 25.2–51.1 | no | OI-102: causa trovata e attuata in SIM 164 (ADR-081, scelta C), rimisurata al 166: il drop point letto da entrambi i lati e il rank vero; il conteggio resta basso (10.82 → 11.02 → 10.62 → 10.52), la causa è chiusa ma l'OI resta aperta sul divario |
| duelli a terra | 182.6 | 27.5 | 207.30 | 155–210 | sì | — |
| dribbling riusciti | 16.03 | 5.67 | 27.92 | 10.4–21.7 | no | OI-103: dal 137 il portatore punta l'uomo nel 35% dei contatti (Wyscout 18,6%) |
| falli | 26.00 | 6.69 | 14.93 | 19.3–32.7 | no | OI-114: dal 131 il difensore vince più contrasti, e un contrasto vinto non è fallo (ADR-042 §4) |
| gialli | 3.97 | 1.96 | 5.87 | 2.0–5.9 | sì | — |
| rossi | 0.24 | 0.49 | 0.07 | 0–0.73 | sì | — |
| angoli | 10.30 | 3.58 | 18.05 | 6.7–13.9 | no | OI-100: la catena dei troppi tiri |
| rimesse laterali | 42.72 | 8.92 | 19.20 | 33.8–51.6 | no | OI-100: raddoppiate dal 127 (3.85) con il rimbalzo; nel calcio vero il 21,5% segue una spazzata, il 19,7% un tocco, il 31% un duello, il 24% un passaggio sbagliato (Wyscout); nel motore un duello non manda quasi mai la palla fuori e il passaggio sbaglia poco |
| rinvii dal fondo | 17.15 | 4.20 | 20.07 | 13.0–21.4 | sì | — |
| punizioni | 29.45 | 6.81 | 20.48 | 22.6–36.3 | no | OI-112: probabilmente di conteggio |
| fuorigioco | 4.26 | 2.40 | 8.10 | 1.9–6.7 | no | OI-098 / OI-104: la linea è letta con posizioni esatte, l'errore di percezione non è ancora costruito |
| spazzate | 29.53 | 8.65 | 55.92 | 20.9–38.2 | no | OI-100: la spazzata di testa resta una regola (ADR-064, passo 2) |
| parate | 9.37 | 3.37 | 18.85 | 6.0–12.7 | no | OI-100: troppe parate seguono i troppi tiri in porta |
| recupero, primo tempo (min) | 1.49 | 1.35 | 5.54 | 0.1–2.8 | no | OI-101: il motore aggiunge tutto il tempo perso, gli arbitri circa metà (fonte nell'OI) |
| recupero, secondo tempo (min) | 4.08 | 1.61 | 4.51 | 2.5–5.7 | sì | — |

## La misura del 184 (2026-11-21)

`cargo run --release -p ds-reference -- compare --reference L0-match/reference/serie-a-2017-18.json --seeds 42..101`, motore 184. La banda è la stessa della tabella storica (media reale ± una deviazione standard); le due voci non confrontabili (cross, intercetti) restano fuori.

| Metrica | Reale | Motore 184 | Banda | Dentro? | Causa o OI |
|---|---|---|---|---|---|
| gol | 2.68 | 13.20 | 1.0–4.3 | no | OI-100: catena dei troppi tiri e delle ripartenze; tiro troppo preciso |
| autogol | 0.10 | 1.92 | 0–0.4 | no | OI-100: spazzata sotto pressione deviata |
| tiri | 24.60 | 52.70 | 19.1–30.1 | no | OI-100: troppi tiri |
| tiri in porta | 8.69 | 24.75 | 5.5–11.9 | no | OI-100: tiri in eccesso; ADR-087 già attuata sullo specchio |
| rigori | 0.33 | 0.02 | 0–0.9 | sì | — |
| passaggi | 887.68 | 1526.27 | 765–1011 | no | OI-100: troppi passaggi e troppi intercettati |
| passaggi riusciti | 739.17 | 1207.85 | 614–864 | no | OI-100: troppi passaggi in tutto, quota di riuscita da leggere |
| duelli aerei | 38.14 | 15.77 | 25.2–51.1 | no | OI-102: causa nota (ADR-081), divario ancora aperto |
| duelli a terra | 182.63 | 163.63 | 155–210 | sì | — |
| dribbling riusciti | 16.03 | 19.97 | 10.4–21.7 | sì | — |
| falli | 26.00 | 11.37 | 19.3–32.7 | no | OI-114: troppi contrasti vinti, e un contrasto vinto non è fallo |
| gialli | 3.97 | 4.20 | 2.0–5.9 | sì | — |
| rossi | 0.24 | 0.13 | 0–0.7 | sì | — |
| angoli | 10.30 | 28.87 | 6.7–13.9 | no | OI-100: catena dei troppi tiri |
| rimesse laterali | 42.72 | 30.62 | 33.8–51.6 | no | OI-100: un duello manda poco spesso la palla fuori |
| rinvii dal fondo | 17.15 | 26.77 | 13.0–21.4 | no | OI-100: seguono le troppe spazzate |
| punizioni | 29.45 | 20.35 | 22.6–36.3 | no | OI-112: probabilmente di conteggio |
| fuorigioco | 4.26 | 10.72 | 1.9–6.7 | no | OI-098 / OI-104: errore di percezione non costruito |
| spazzate | 29.53 | 73.37 | 20.9–38.2 | no | OI-100: troppe spazzate, regola ADR-064 |
| parate | 9.37 | 29.95 | 6.0–12.7 | no | OI-100: troppe parate seguono i troppi tiri in porta |
| recupero, primo tempo (min) | 1.49 | 8.93 | 0.1–2.8 | no | OI-101: il motore addiziona tutto il tempo perso |
| recupero, secondo tempo (min) | 4.08 | 8.06 | 2.5–5.7 | no | OI-101: stessa causa del primo tempo |

**In banda (SIM 184): 5 metriche su 22 confrontabili** (rigori, duelli a terra, dribbling riusciti, gialli, rossi). **Fuori banda: 17**, ognuna con la sua causa o il suo OI aperto (OI-098, OI-100, OI-101, OI-102, OI-103, OI-104, OI-112, OI-114). La caduta da 6 a 5 rispetto al 171 non è un peggioramento dei comportamenti: il DD-41 ha tolto l'ultimo termine aperto della latenza e ha cambiato i tempi di lettura; la misura si rilegge dopo ogni SIM senza fittere numeri.

**Prova scartata (2026-11-21, DD-42 non aperto):** allineare la credenza del tiro al pavimento di ADR-087 — usare `kick.shot_error_distance_cm(meet)` invece di `meet` in `thought::chance_with` — con gemello rosso/verde in `ds-calculator`. Su 42..101 i gol scendono 13.20 → 11.37 e i tiri in porta 24.75 → 24.15, ma le metriche in banda restano **5**: tiri 52.45, passaggi 1532.10, parate 30.68, spazzate 70.75. La misura dell'ADR-052 è riservata al maintainer (OI-100), quindi la prova è stata riportata indietro senza SIM bump: il difetto di credenza sotto gli 11 m resta aperto, con la sua misura. **Prova scartata (stessa data):** togliere il clamp della destinazione della spazzata in `technique::play_clearance`, così il punto fuori campo scelto dal giocatore resta fuori. Le rimesse salgono solo 30.62 → 30.83 (banda 33.8–51.6) e le spazzate 73.37 → 73.80: il clamp non era la causa del divario, e la prova è stata riportata indietro.

### La tabella storica (SIM 171)

**In banda (SIM 171): 6 metriche su 22 confrontabili** (rigori, duelli a terra, gialli, rossi, rinvii dal fondo, recupero del secondo tempo). **Fuori banda: 16**, ognuna con la sua causa o il suo OI aperto (OI-098, OI-100, OI-101, OI-102, OI-103, OI-104, OI-112, OI-114). Storico: SIM 170 **6/22**; SIM 169 **6/22**; SIM 168 **6/22**; SIM 167 **6/22**; SIM 166 **6/22**; SIM 165 **6/22**; SIM 164 **6/22**; SIM 163 **6/22**; SIM 154 **9/22**; SIM 151 **11/22**; SIM 147 **11/22**; SIM 141 **12/22**. SIM 166 (ADR-083, la palla libera rotola) non cambia il conteggio: la fisica della palla era un prerequisito, non la causa del comportamento che si vede in area. Il maintainer conferma che in 2.5D la scena è identica: i giocatori fanno movimenti, ma nessuno ottiene il possesso con le proprie abilità. La caduta da 12 a 6 non è un peggioramento dei numeri ma l'effetto di togliere i burattini: più ordine imposto (ADR-071, SIM 154; calci morti e piedi migliori, SIM 163), meno calcio; la rincorsa è dal nocciolo verso l'esterno, non il contrario.

## I parametri di grado 3 (fase 4.1)

L'inventario completo è in `L0-AUDIT.md` §7: 381 parametri, di cui 25 di grado 1, 0 di grado 2 e 356 di grado 3. Nessuno è stato portato a grado 1 o 2 in questa fase:
- **le fonti:** quelle che servirebbero sono negli OI qui sopra (lo StatsBomb degli expected completion per la precisione dei passaggi; gli esiti dei duelli a terra per `duel.json`), e non sono ancora lette;
- **il grado 2:** chiede una metrica reale tenuta da parte, e i fuori banda sono legati fra loro (tiri, tiri in porta, parate, angoli, gol). Fissarne uno guardando un altro della stessa catena non sarebbe una verifica indipendente.

Restano quindi tutti nel registro, con il conteggio in `STATE.md`.

## L'ampiezza del gioco (2026-11-15)

- **Reale, Wyscout Serie A 2017-18** (337.317 passaggi): il 20,3% parte dal decimo esterno della larghezza, da un lato o dall'altro.
- **Reale, Metrica Sports** (partite campione 1 e 2): la palla sta nel decimo esterno il 7,5-9,8% del tempo per lato; con la palla l'uomo più vicino a una linea laterale ne dista 4 m (mediana).
- **Motore 126** (semi 42-51, dal registro dei tick): l'1,6% dei passaggi parte dal decimo esterno; le ali non andavano oltre 9 m dalla linea (la lettura della griglia, corretta al 127).
- **Dal centro verso la fascia:** dai decimi 2-4 della larghezza il 7,5-17,9% dei passaggi veri arriva nel decimo esterno del proprio lato (Wyscout); nel motore 128 lo 0,5-1,6%. Dalla fascia invece il motore è vicino al vero (dal decimo esterno ci resta il 39% contro il 29%). Manca il passaggio verso la linea, e il compagno che la tiene: con la palla in gioco e al centro, il più largo vero sta a 9-10 m dalla linea, quello del motore a circa 13 m (al 128; con la palla dovunque, mediana 10,6 m).

## Auditoria delle tarature storiche (2026-11-19)

Richiesta dal maintainer: distinguere i numeri scelti per spiegare il calcio
da quelli scelti per far tornare un conteggio. Esito, letto su
`CHANGELOG-SIM.md`, `balance/SOURCES.md` e `context/RULES.md` A5-bis:

- **11 episodi sospetti o border-line**, di cui **7 ancora vivi** nel
  comportamento e **4 morti o superati**.
- I vivi più rilevanti: reach/reaction/aim del portiere (`save.json`,
  SIM 61 e 71), soglie fallo/cartellino (`duel.json`, `laws.json`, SIM 64),
  retry/slide dei duelli (`duel.json`, SIM 63), `progression_gain`
  (`shot_choice.json`, SIM 57), velocità di calcio (`kick.json`, SIM 55),
  contributi al recupero (`stoppage_time.json`, SIM 7).
- Sono legittimi, con fonte o verifica fuori campione: reazione allenatore,
  soglia knock, errore base del tiro, goal→kick-off, soglia offside,
  larghezza del blocco, `line_break_milli` come scelta dichiarata.
- Alcune documentazioni sono vecchie e contraddicono il valore attuale
  (`save.json`, `duel.json`, `SOURCES.md`): va segnalato, non usato per
  ricalibrare.

**Regola operativa:** nessuno di questi numeri è una causa. Se una metrica
fuori banda sembra richiederlo, si prepara un DD/ADR e ci si ferma, come per
ogni scelta di modello.

## Bisection delle regressioni (2026-11-19)

Misura sugli stessi 60 semi, in worktree separati: SIM 141 **12/22**, SIM 147
**11/22**, SIM 151 **11/22**, SIM 154 **9/22**, SIM 163 **6/22**, SIM 164
**6/22**, SIM 165 **6/22**. Il primo salto negativo è con la posizione senza
palla (SIM 154, ADR-071); il secondo con i calci morti e i piedi migliori
(SIM 163). SIM 164 non aggiunge metriche in banda: la causa del duello aereo
è tolta (11.02 da 10.82), ma il conteggio resta 6/22. SIM 165 attua
l'emendamento del burattino: la lettura del giocatore sceglie la linea nel suo
corridoio quando nessuno arriva, le fasce accanto restano pesate dal gioco.
La garanzia di pressione ADR-070 e il test del piede restano verdi, e il
conteggio resta 6/22: i gialli rientrano in banda (5.00), i duelli aerei
tornano a 10.85.

## OI-102: causa trovata, scelta C applicata (SIM 164)

L'agente ha trovato la causa dell'isolamento del duello aereo: SIM 160 manda
il secondo uomo solo sul lato che difende e solo su un passaggio in volo, e
il `Target` aereo hardcodifica `rank: 0`, così la race del secondo uomo in
`offball.rs` non scatta mai. Il trial B (ogni palla alta, entrambi i lati)
portò i duelli aerei da 10,82 a 12,12 e i gol da 5,20 a 4,52 senza aggiungere
metriche in banda, e fece sparire dal campione dei semi 42-49 i tiri e i cross
laterali su cui si regge il test del piede (`feet.rs`).

Il maintainer ha scelto **C** (DD-19, ADR-081 `Accepted`): ogni palla alta,
entrambi i lati, ma il secondo va solo se può arrivarci, con la corsa e la
lettura già esistenti, nessun numero nuovo. In SIM 164 `thought.rs` legge il
drop point da entrambi i lati e passa il rank vero; `offball.rs` decide con la
race sul passo del giocatore. Il gemello
`the_second_reaches_the_near_ball_and_holds_for_the_far_one` e il test del
piede `swapped_wide_men_strike_with_the_feet_they_have` sono verdi. La misura
sulle stesse 60 partite 42..101 (2026-11-20) dà i numeri della tabella qui
sopra: i duelli aerei salgono da 10.82 a 11.02, i gol da 5.20 a 5.25, e le
metriche in banda restano 6 su 22. La causa di OI-102 è chiusa; il divario
del conteggio resta e va cercato in un'altra causa del nocciolo. In SIM 165
l'emendamento ADR-071 (posto senza palla letto dal giocatore, non imposto)
rimette il gemello di pressione ADR-070 verde senza toccare semi né asserzioni,
e il conteggio resta 6/22: le due cause sono accoppiate nel golden 165. In SIM 166 ADR-083 restituisce il rotolamento alla palla libera senza numeri nuovi; golden e conteggio restano 6/22, e il maintainer in 2.5D vede la stessa scena: il difetto non è che la palla sia ferma, ma che l'ottenimento del possesso non è ancora un'azione del giocatore con le sue abilità.

## La prossima misura

Dopo ogni causa corretta: la stessa misura, la tabella aggiornata qui, il `SIM_VERSION` con la sua voce in `CHANGELOG-SIM.md`.
