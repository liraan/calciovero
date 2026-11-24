# L0 — la taratura fine (2026-11-15)

**Cos'è.** La fase 4 del mandato del maintainer del 2026-11-15: il motore misurato contro il calcio reale, metrica per metrica, con la sua banda e, per ogni metrica fuori banda, la causa nel modello o l'OI che la cerca. Nessun numero è stato ritoccato per rientrare in una banda (RULES A5-bis).

## Come si misura

- **Strumento:**
  ```bash
  cargo run --release -p ds-reference -- compare --reference L0-match/reference/serie-a-2017-18.json --seeds 42..101
  ```
  Finestra di taratura 42..101 (60 partite), squadre di prova, blocco medio e ampiezza normale. Non è un numero statistico: la banda è già la dispersione della partita vera (decisione Y), la finestra serve solo a rileggere la stessa partita più volte e a confrontare misure diverse. **Non fa parte dei controlli di chiusura** (RULES C2, 2026-11-20): si esegue quando serve una decisione sulle metriche, non a ogni intervento.
- **Riferimento:** Serie A 2017-18, 380 partite, Wyscout (Pappalardo et al. 2019, *Scientific Data* 6, 236; CC BY 4.0), per partita ed entrambe le squadre.
- **Motore:** `SIM_VERSION` 195 (in codice, DD-54); ultima misura del 195: **4 su 22** (sezione "La misura del 195"). Prima: `SIM_VERSION` 192 (DD-52/ADR-110). L'ultima misura, del 2026-11-23 su 60 partite 42..101, è del **192**: **6 su 22** (sezione "La misura del 192" qui sotto). Il conteggio "10 su 22" / "9 su 22" scritto prima per il 188 era sbagliato (tre righe segnate in banda fuori banda). Il SIM 189 (DD-49) è accantonato sul branch `sim-189-first-time`. Il 187 (DD-47) era 6/22 con parate 14.65. Il 185 ha spostato i `Threats` da `ds-match` all'orchestratore con aritmetica identica e nessuna misura nuova. La tabella storica sotto è l'ultima misura riga per riga del **171** (6 su 22), lasciata come confronto: le sue cause e i suoi OI valgono ancora dove non sono stati chiusi. SIM 167 (il possesso è il movimento) e SIM 168 (solo osservatori, golden identico) non cambiano il conteggio. Storia delle misure: al 168, dopo ADR-084 e ADR-085 (gol 4.80, passaggi 1237, duelli aerei 10.82, rimesse 17.87, 6 su 22); al 166, la palla libera conserva la velocità e continua a rotolare (gol 5.22, passaggi 1254, duelli aerei 10.62, rimesse 18.08, 6 su 22); al 165, l'emendamento ADR-071 sul posto senza palla letto dal giocatore (gol 5.22, duelli aerei 10.85, rimesse 18.27, 6 su 22); al 164, la scelta C di OI-102 (ADR-081): gol 5.25, duelli aerei 11.02, 6 su 22; al 163, i calci morti e il battitore scelto per ruolo (DD-21, ADR-080): gol 5.20, duelli aerei 10.82, 6 su 22; al 154, la posizione senza palla (ADR-071): 9 su 22; al 151, 11 su 22; al 147, i passi piccoli (ADR-069): gol 2.95, rimesse 25.9, 11 su 22; al 141, il colpo che costringe al cambio (ADR-066): 12 su 22. La taratura è ferma finché i comportamenti non sono giusti (decisione BK). La misura del 116, del 117 e del 118 è identica riga per riga: il 117 e il 118 cambiano solo le partite di coppa e le squadre con l'allenatore che reagisce. **SIM 170** (la pressione rimette alla prova il controllo del portatore, ADR-086) non è stato misurato separatamente: la misura 42..101 è stata fatta al 171, quando il difetto angolare di OI-100 chiedeva una decisione sulle metriche, non un rito di chiusura (RULES C2).
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

## La misura del 201 (2026-11-24, DD-61)

Stesso comando e banda. **5 su 22** (penalties, ground_duels, fouls, red_cards, goal_kicks); gol 7.78, autogol 1.13, tiri 43.58, in porta 21.17, passaggi 1830.32, falli 30.63, gialli 1.38, rossi 0.48, parate 13.3, fuorigioco 14.1, recupero 9.33/8.66. Nessun numero adattato.

## La misura del 200 (2026-11-24, DD-60)

Stesso comando e banda. **7 su 22** (penalties, ground_duels, take_ons_won, fouls, red_cards, goal_kicks, saves); gol 6.53, autogol 0.82, tiri 42.43, in porta 19.7, passaggi 1854.2, falli 31.4, gialli 1.45, rossi 0.55, parate 12.47, fuorigioco 14.73, recupero 8.79/8.21. Nessun numero adattato.

## La misura del 199 (2026-11-24, DD-59)

Stesso comando e banda. **5 su 22** (penalties, ground_duels, fouls, red_cards, goal_kicks); gol 7.17, autogol 0.87, tiri 45.03, in porta 20.78, passaggi 1843.65, falli 32.33, gialli 1.22, rossi 0.67, parate 13.48, fuorigioco 12.52, recupero 8.87/9.08. Nessun numero adattato.

## La misura del 198 (2026-11-24, DD-58)

Stesso comando e banda. **7 su 22** (penalties, ground_duels, take_ons_won, fouls, red_cards, goal_kicks, saves); gol 6.95, autogol 0.83, tiri 43.63, in porta 20.15, passaggi 1829.75, falli 30.68, gialli 1.42, rossi 0.7, parate 12.72, fuorigioco 12.78, recupero 8.4/8.78. Nessun numero adattato.

## La misura del 192 (2026-11-23, DD-52/ADR-110)

Stesso comando e tranche (60 partite), stessa banda. Nessun numero adattato.

| Metrica | Reale | Motore 191 | Dentro? | Motore 192 | Dentro? |
|---|---|---|---|---|---|
| goals | 2.68 | 9.78 | no | 10.82 | no |
| own_goals | 0.1 | 2.03 | no | 1.95 | no |
| shots | 24.6 | 36.82 | no | 39.48 | no |
| shots_on_target | 8.69 | 19.37 | no | 21.1 | no |
| penalties | 0.33 | 0.32 | sì | 0.28 | sì |
| passes | 887.68 | 1750 | no | 1701.87 | no |
| passes_accurate | 739.17 | 1443.07 | no | 1383.77 | no |
| aerial_duels | 38.14 | 9.13 | no | 7.58 | no |
| ground_duels | 182.63 | 147.82 | no | 172.07 | sì |
| take_ons_won | 16.03 | 19.5 | sì | 20.7 | sì |
| fouls | 26 | 23.83 | sì | 27.5 | sì |
| yellow_cards | 3.97 | 7.33 | no | 7.58 | no |
| red_cards | 0.24 | 0.5 | sì | 0.5 | sì |
| corners | 10.3 | 21.35 | no | 21.87 | no |
| throw_ins | 42.72 | 21.95 | no | 27.02 | no |
| goal_kicks | 17.15 | 18.45 | sì | 19.78 | sì |
| free_kicks | 29.45 | 31.45 | sì | 39.08 | no |
| offsides | 4.26 | 9.55 | no | 13.43 | no |
| clearances | 29.53 | 43.87 | no | 43.5 | no |
| saves | 9.37 | 13.1 | no | 14.25 | no |
| stoppage_first_half_min | 1.49 | 9.07 | no | 10.06 | no |
| stoppage_second_half_min | 4.08 | 8.66 | no | 9.93 | no |

**In banda: 192 → 6 su 22** (rigori, duelli a terra, dribbling, falli, rossi, rinvii). Entrano i duelli a terra, escono le punizioni (fuorigioco 13.4 + falli 27.5). Varianti misurate: solo difesa sul passaggio 6/22 (fuorigioco 9.0); parte 1 completa senza memoria della corsa 5/22 (fuorigioco 17.0).

## La misura del 191 (2026-11-23, DD-51/ADR-109)

Stesso comando e stesse tre tranche del 190 (42..61, 62..81, 82..101; 60 partite), stessa banda. Nessun numero adattato (A5-bis).

| Metrica | Reale | Motore 190 | Dentro? | Motore 191 | Dentro? |
|---|---|---|---|---|---|
| goals | 2.68 | 10.98 | no | 9.78 | no |
| own_goals | 0.1 | 2.32 | no | 2.03 | no |
| shots | 24.6 | 37.02 | no | 36.82 | no |
| shots_on_target | 8.69 | 19.6 | no | 19.37 | no |
| penalties | 0.33 | 0.42 | sì | 0.32 | sì |
| passes | 887.68 | 1743.95 | no | 1750 | no |
| passes_accurate | 739.17 | 1437.62 | no | 1443.07 | no |
| aerial_duels | 38.14 | 7.9 | no | 9.13 | no |
| ground_duels | 182.63 | 146.42 | no | 147.82 | no |
| take_ons_won | 16.03 | 18.7 | sì | 19.5 | sì |
| fouls | 26 | 24.47 | sì | 23.83 | sì |
| yellow_cards | 3.97 | 7.27 | no | 7.33 | no |
| red_cards | 0.24 | 0.67 | sì | 0.5 | sì |
| corners | 10.3 | 21.5 | no | 21.35 | no |
| throw_ins | 42.72 | 23.5 | no | 21.95 | no |
| goal_kicks | 17.15 | 17.27 | sì | 18.45 | sì |
| free_kicks | 29.45 | 32.72 | sì | 31.45 | sì |
| offsides | 4.26 | 10.27 | no | 9.55 | no |
| clearances | 29.53 | 42.27 | no | 43.87 | no |
| saves | 9.37 | 13.05 | no | 13.1 | no |
| stoppage_first_half_min | 1.49 | 10.01 | no | 9.07 | no |
| stoppage_second_half_min | 4.08 | 9.13 | no | 8.66 | no |

**In banda: 191 → 6 su 22**, come 190 e 188. Gol 10.98 → 9.78, fuori banda. Sonda temporanea su 10 partite (42..51): in difesa "restare" 74% → 72%, pressing 7.923 → 8.491 decisioni (il secondo uomo 1.289, il terzo 469: prima 0), marcatura 32.749 → 38.141. Il terzo uomo e oltre restano al posto nel 97% dei casi: con lo stesso passo creduto, chi è più vicino arriva prima, quindi il pericolo che gli resta è piccolo; è la lettura giusta della zona, non un taglio. La leva di OI-100 non è qui: resta il 42% di decisioni con la palla in volo, dove l'unica opzione è restare (prossimo, SIM 192).

## La misura del 197 (2026-11-24, DD-57)

Stesso comando e banda. **5 su 22** (penalties, ground_duels, fouls, red_cards, goal_kicks); gol 7.75, autogol 0.78, tiri 47.83, passaggi 1833.95, falli 31.75, gialli 1.28, rossi 0.65, fuorigioco 12.15, recupero 9.12/9.22. Nessun numero adattato.

## La misura del 196 (2026-11-23, DD-55)

Stesso comando e banda. I difetti della sezione A dell'audit del codice, nessun numero nuovo. **In banda: 196 → 6 su 22** (rigori, duelli a terra, dribbling riusciti, falli 30.08, rossi 0.73, rinvii). Gol 9.28 → 7.65, autogol 1.25 → 0.95, tiri 46.95 → 44.57, fuorigioco 10.55 → 12.03 (ora anche di testa), gialli 1.23, recupero 9.18 / 8.49.

## La misura del 195 (2026-11-23, DD-54)

Stesso comando (`compare --seeds 42..101`, 60 partite in una passata), stessa banda (media reale ± una deviazione standard). Nessun numero adattato guardando la metrica: la frenata viene da Harper et al. 2020, la soglia del colpo è rifissata con il suo metodo di grado 2 (semi pari, verifica sui dispari) e non entra in nessuna delle 22 metriche.

| Metrica | Reale ± | 194 | 195 |
|---|---|---|---|
| gol | 2.68 ± 1.65 | 9.2 | 9.28 |
| autogol | 0.10 ± 0.31 | 1.05 | 1.25 |
| tiri | 24.60 ± 5.46 | 48 | 46.95 |
| passaggi | 887.7 ± 122.8 | 1838 | 1843 |
| duelli aerei | 38.1 ± 13.0 | 16.6 | 15.95 |
| duelli a terra | 182.6 ± 27.5 | — | 207.5 (in banda) |
| falli | 26.0 ± 6.7 | 35.2 | 35.93 |
| gialli | 3.97 ± 1.96 | 1.38 | 1.70 |
| rossi | 0.24 ± 0.49 | 0.93 | 0.98 |
| fuorigioco | 4.26 ± 2.40 | 9.98 | 10.55 |
| recupero 1°/2° tempo | 1.49 / 4.08 | — | 10.39 / 10.87 |

**In banda: 195 → 4 su 22** (rigori, duelli a terra, dribbling riusciti, rinvii), come il 194. La frenata più dolce di 0.3 m/s² non sposta nessuna metrica di banda; il recupero (10 minuti per tempo) resta la leva del branch `sim-193-law7-stoppage`.

## La misura del 194 (2026-11-23, DD-53/ADR-111)

Stesso comando del 190, tre tranche 42..61, 62..81, 82..101 (60 partite), media semplice; banda = media reale ± una deviazione standard. Ogni colonna aggiunge un passo al precedente; l'ultima è il SIM 194 committato. Nessun numero adattato (A5-bis): ogni passo è una correzione della fisica, della lettura o della Legge, tenuto anche quando il conteggio scende.

| Metrica | Reale | Intercetto | + accelerazione | + lancio lungo | + frenata e Legge 12 (194) |
|---|---|---|---|---|---|
| goals | 2.68 | 10.32 no | 9.88 no | 9.3 no | 9.2 no |
| own_goals | 0.1 | 2.07 no | 1.95 no | 1.38 no | 1.05 no |
| shots | 24.6 | 39.83 no | 39.67 no | 39.07 no | 48.47 no |
| shots_on_target | 8.69 | 19.93 no | 20.23 no | 20.03 no | 23.18 no |
| penalties | 0.33 | 0.33 sì | 0.47 sì | 0.38 sì | 0.32 sì |
| passes | 887.68 | 1702.22 no | 1685.02 no | 1677.2 no | 1837.58 no |
| passes_accurate | 739.17 | 1388.83 no | 1378.57 no | 1371.43 no | 1509.7 no |
| aerial_duels | 38.14 | 7.42 no | 7.2 no | 7.07 no | 16.6 no |
| ground_duels | 182.63 | 169.47 sì | 168.02 sì | 172.78 sì | 201.62 sì |
| take_ons_won | 16.03 | 20.98 sì | 19.55 sì | 19.98 sì | 20.37 sì |
| fouls | 26 | 27.42 sì | 26.97 sì | 27.05 sì | 35.15 no |
| yellow_cards | 3.97 | 7.68 no | 7.93 no | 8.6 no | 1.38 no |
| red_cards | 0.24 | 0.63 sì | 0.63 sì | 0.62 sì | 0.93 no |
| corners | 10.3 | 21 no | 20.95 no | 19.62 no | 20.48 no |
| throw_ins | 42.72 | 26.23 no | 25.53 no | 25.72 no | 17.6 no |
| goal_kicks | 17.15 | 21.98 no | 21.77 no | 20.9 sì | 18.77 sì |
| free_kicks | 29.45 | 39.67 no | 40.07 no | 41.23 no | 41.75 no |
| offsides | 4.26 | 14.02 no | 15.13 no | 15.6 no | 9.98 no |
| clearances | 29.53 | 42.32 no | 44.98 no | 45.32 no | 54.58 no |
| saves | 9.37 | 13.57 no | 14.43 no | 14.25 no | 16.72 no |
| stoppage_first_half_min | 1.49 | 10 no | 10.1 no | 9.66 no | 10.57 no |
| stoppage_second_half_min | 4.08 | 9.46 no | 8.77 no | 8.66 no | 10.35 no |

**In banda: intercetto 5, + accelerazione 5, + lancio lungo 6, SIM 194 finale 4 su 22** (main al 192: 6). Migliorano autogol (192: 1.95 → 1.05), fuorigioco (13.4 → 9.98), duelli aerei (7.6 → 16.6), gol (10.8 → 9.2), rinvii dal fondo in banda. Peggiorano, con la frenata: falli 27.5 → 35.2, rossi fuori banda (0.93: tutti scivolate da dietro entro 25 m fuori area, chiara occasione negata — la Legge è giusta, le occasioni sono troppe), gialli 1.38 (il modello del cartellino conosce solo la scivolata da dietro veloce), passaggi 1702 → 1838, tiri 39 → 48. La regola sul rasoterra (la deviazione laterale contro la portata più la corsa del compagno) a queste velocità non cambia nulla: identica al passo precedente, numero per numero.

## La misura del 188 rifatta e del 190 (2026-11-22, DD-50/ADR-108)

Stesso comando, tre tranche 42..61, 62..81, 82..101 (60 partite), aggregate con media semplice; banda = media reale ± una deviazione standard (decisione Y). **Correzione:** la tabella "La misura del 188" qui sotto segnava "in banda" tiri (36.16, banda 19.1–30.1), duelli a terra (147.53, banda 155–210) e rimesse (22.66, banda 33.8–51.6), che non lo sono: il 188 era **6 su 22**, non 9 (né 10). Rimisurato qui con lo stesso metodo del 190. Nessun numero adattato (A5-bis).

| Metrica | Reale | Motore 188 | Dentro? | Motore 190 | Dentro? |
|---|---|---|---|---|---|
| goals | 2.68 | 10.18 | no | 10.98 | no |
| own_goals | 0.1 | 1.95 | no | 2.32 | no |
| shots | 24.6 | 36.5 | no | 37.02 | no |
| shots_on_target | 8.69 | 20.02 | no | 19.6 | no |
| penalties | 0.33 | 0.22 | sì | 0.42 | sì |
| passes | 887.68 | 1758.52 | no | 1743.95 | no |
| passes_accurate | 739.17 | 1447.9 | no | 1437.62 | no |
| aerial_duels | 38.14 | 8.97 | no | 7.9 | no |
| ground_duels | 182.63 | 147.82 | no | 146.42 | no |
| take_ons_won | 16.03 | 18.83 | sì | 18.7 | sì |
| fouls | 26 | 24.3 | sì | 24.47 | sì |
| yellow_cards | 3.97 | 6.87 | no | 7.27 | no |
| red_cards | 0.24 | 0.5 | sì | 0.67 | sì |
| corners | 10.3 | 22.12 | no | 21.5 | no |
| throw_ins | 42.72 | 22.55 | no | 23.5 | no |
| goal_kicks | 17.15 | 16.93 | sì | 17.27 | sì |
| free_kicks | 29.45 | 32.13 | sì | 32.72 | sì |
| offsides | 4.26 | 9.47 | no | 10.27 | no |
| clearances | 29.53 | 41.55 | no | 42.27 | no |
| saves | 9.37 | 13.42 | no | 13.05 | no |
| stoppage_first_half_min | 1.49 | 9.5 | no | 10.01 | no |
| stoppage_second_half_min | 4.08 | 8.78 | no | 9.13 | no |

**In banda: 188 → 6 su 22; 190 → 6 su 22** (rigori, dribbling riusciti, falli, rossi, rinvii dal fondo, punizioni). Il 190 non cambia banda a nessuna metrica: cambia chi sa che cosa (la lettura dei compagni), non la radice di OI-100. Il SIM 189 (DD-49, il gioco di prima) misurava 5 su 22, e 4 su 22 con la sua correzione: è accantonato sul branch `sim-189-first-time`, non è su main.

## La misura del 188 (2026-11-22, DD-48/ADR-106)

`cargo run --release -p ds-reference -- compare --reference L0-match/reference/serie-a-2017-18.json --seeds 42..101` spezzato in tre tranche (42..62, 62..81, 81..101; 62 partite), motore 188, aggregato manualmente. La banda è la stessa (media reale ± una deviazione standard); le due voci non confrontabili (cross, intercetti) restano fuori. Nessun numero è stato adattato per rientrare in banda (A5-bis).

| Metrica | Reale | Motore 188 | Banda | Dentro? | Causa o OI |
|---|---|---|---|---|---|
| gol | 2.68 | 10.10 | 1.0–4.3 | no | OI-100: catena dei troppi tiri e delle ripartenze |
| autogol | 0.10 | 1.97 | 0–0.4 | no | OI-100: spazzata sotto pressione deviata |
| tiri | 24.60 | 36.16 | 19.1–30.1 | sì | DD-48: header letto come tiro riduce il rumore, ma la catena resta aperta |
| tiri in porta | 8.69 | 19.87 | 5.5–11.9 | no | OI-100: troppi tiri in porta |
| rigori | 0.33 | 0.21 | 0–0.9 | sì | — |
| passaggi | 887.68 | 1758.55 | 765–1011 | no | OI-100: troppi passaggi |
| passaggi riusciti | 739.17 | 1448.81 | 614–864 | no | OI-100: troppi passaggi in tutto |
| duelli aerei | 38.14 | 8.84 | 25.2–51.1 | no | OI-102: causa nota (ADR-081), divario aperto |
| duelli a terra | 182.63 | 147.53 | 155–210 | sì | DD-48/contesto partita; resta da leggere col filtro LUNA |
| dribbling riusciti | 16.03 | 18.95 | 10.4–21.7 | sì | — |
| falli | 26.00 | 24.13 | 19.3–32.7 | sì | OI-114 chiusa |
| gialli | 3.97 | 6.90 | 2.0–5.9 | no | Conseguenza dichiarata della popolazione di falli; non tarata |
| rossi | 0.24 | 0.50 | 0–0.73 | sì | — |
| angoli | 10.30 | 21.92 | 6.7–13.9 | no | OI-100: catena dei troppi tiri |
| rimesse laterali | 42.72 | 22.66 | 33.8–51.6 | sì | Direzione migliorata; causa da confermare |
| rinvii dal fondo | 17.15 | 16.84 | 13.0–21.4 | sì | — |
| punizioni | 29.45 | 32.07 | 22.6–36.3 | sì | Effetto collaterale dei più falli; OI-112 non toccata |
| fuorigioco | 4.26 | 9.52 | 1.9–6.7 | no | OI-098 / OI-104: errore di percezione non costruito |
| spazzate | 29.53 | 41.29 | 20.9–38.2 | no | OI-100: troppe spazzate, regola ADR-064 |
| parate | 9.37 | 13.34 | 6.0–12.7 | no | DD-46 mantiene la lettura corretta; resta fuori banda per OI-100 |
| recupero, primo tempo (min) | 1.49 | 9.42 | 0.1–2.8 | no | OI-101: il motore addiziona tutto il tempo perso |
| recupero, secondo tempo (min) | 4.08 | 8.73 | 2.5–5.7 | no | OI-101: stessa causa del primo tempo |

**In banda (SIM 188): 9 su 22 — conteggio sbagliato, in realtà 6 su 22 (vedi "La misura del 188 rifatta e del 190")** (tiri, rigori, duelli a terra, dribbling riusciti, falli, rossi, rimesse, rinvii dal fondo, punizioni). DD-48 fa entrare tiri, duelli a terra e rimesse; parate restano fuori per 0.64 (13.34, banda 6.0–12.7). Nessun numero adattato (A5-bis).

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

## La misura del 186 (2026-11-22)

Stesso comando, motore 186, stessa banda (media reale ± una deviazione standard). Le due voci non confrontabili (cross, intercetti) restano fuori. Nessun numero è stato adattato per rientrare in banda (A5-bis).

| Metrica | Reale | Motore 186 | Banda | Dentro? | Causa o OI |
|---|---|---|---|---|---|
| gol | 2.68 | 9.53 | 1.0–4.3 | no | OI-100: catena dei troppi tiri e delle ripartenze |
| autogol | 0.10 | 1.97 | 0–0.4 | no | OI-100: spazzata sotto pressione deviata |
| tiri | 24.60 | 38.07 | 19.1–30.1 | no | OI-100: troppi tiri (34.10 con la sola OI-116) |
| tiri in porta | 8.69 | 17.45 | 5.5–11.9 | no | OI-100; ADR-087 invariata per decisione del maintainer |
| rigori | 0.33 | 0.30 | 0–0.9 | sì | — |
| passaggi | 887.68 | 1753.28 | 765–1011 | no | OI-100: troppi passaggi; un passo di lettura in più non li riduce |
| passaggi riusciti | 739.17 | 1444.53 | 614–864 | no | OI-100: troppi passaggi in tutto |
| duelli aerei | 38.14 | 9.08 | 25.2–51.1 | no | OI-102: causa nota (ADR-081), divario aperto |
| duelli a terra | 182.63 | 147.38 | 155–210 | no | OI-114: più contatti vinti, meno duelli contati; era 163.63 al 184 |
| dribbling riusciti | 16.03 | 18.07 | 10.4–21.7 | sì | — |
| falli | 26.00 | 23.93 | 19.3–32.7 | sì | OI-114 chiusa: soglia derivata 675, 11,37 → 23,93 |
| gialli | 3.97 | 6.85 | 2.0–5.9 | no | Conseguenza dichiarata della nuova popolazione di falli; non tarata |
| rossi | 0.24 | 0.43 | 0–0.73 | sì | — |
| angoli | 10.30 | 23.40 | 6.7–13.9 | no | OI-100: catena dei troppi tiri |
| rimesse laterali | 42.72 | 20.73 | 33.8–51.6 | no | OI-100: un duello manda poco spesso la palla fuori |
| rinvii dal fondo | 17.15 | 17.65 | 13.0–21.4 | sì | — |
| punizioni | 29.45 | 31.33 | 22.6–36.3 | sì | Effetto collaterale dei più falli: OI-112 non è stata toccata |
| fuorigioco | 4.26 | 9.33 | 1.9–6.7 | no | OI-098 / OI-104: errore di percezione non costruito |
| spazzate | 29.53 | 42.85 | 20.9–38.2 | no | OI-100: troppe spazzate, regola ADR-064 |
| parate | 9.37 | 20.80 | 6.0–12.7 | no | OI-100: troppe parate seguono i troppi tiri in porta; riletto dopo DD-46 (vedi sotto) |
| recupero, primo tempo (min) | 1.49 | 9.01 | 0.1–2.8 | no | OI-101: il motore addiziona tutto il tempo perso |
| recupero, secondo tempo (min) | 4.08 | 8.50 | 2.5–5.7 | no | OI-101: stessa causa del primo tempo |

### La rilettura del 186 (2026-11-22, DD-46)

Stesso motore, stessi 60 semi 42..101, stessa banda: cambia solo il conteggio delle parate in `ds-reference`. Il portiere che prende un cross emette `Save` con `claim: true`; quelle prese non sono parate di un tiro (ADR-104). Sul seed 42, 9 dei 21 `Save` sono prese.

| Metrica | Reale | Motore 186 riletto | Banda | Dentro? |
|---|---|---|---|---|
| parate | 9.37 | 15.22 | 6.0–12.7 | no |

Le parate scendono 20.80 → 15.22 e tornano sotto i 17,45 tiri in porta: l'impossibilità interna è rimossa senza toccare l'evento, i numeri o il SIM. La metrica resta **fuori banda** e il conteggio resta **6/22**: la causa aperta è ancora OI-100 (troppi tiri in porta), non il conteggio. Nessun numero è stato adattato per rientrare in banda (A5-bis). Gemello: `a_keepers_claim_is_not_a_save`.

**In banda (SIM 186): 6 metriche su 22 confrontabili** (rigori, dribbling riusciti, falli, rossi, rinvii dal fondo, punizioni). **Fuori banda: 16.** Rispetto al 184 entrano falli, rinvii e punizioni; escono duelli a terra e gialli. I gialli sono la conseguenza dichiarata della soglia OI-114: il maintainer li ha tenuti fuori dalla decisione e non vanno ritoccati per rientrare nella banda. **`STATE.md` e la decisione L (16/22) non sono soddisfatti: la taratura continua.** OI-112 resta aperta: le punizioni sono entrate in banda per effetto dei più falli, non perché il vantaggio sia stato corretto.

**Decisione del maintainer (2026-11-22):** ADR-087 resta come è: lo scarto assoluto sotto gli 11 m è la forma corretta e l'errore residuo (54,6% contro 46% nello specchio) resta una causa documentata di OI-100, non un numero da ritoccare. Non si riaprono né `error_pressure_rad` né il tiro di prima di SIM 166.

**Decisioni del maintainer (2026-11-22), attuate insieme al SIM 186:** OI-116 (A): il passaggio vale la migliore opzione che il ricevente avrà dove arriva, un solo passo in più dello stesso pensiero, nessun numero nuovo (DD-43/ADR-101). OI-114: via libera alla soglia derivata per il fallo da pressione, con le stesse radici del fallo di contrasto e la fonte StatsBomb del 6,6% delle pressioni che finiscono in fallo; i gialli restano fuori dalla decisione, da osservare come conseguenza e non da tarare (DD-44/ADR-102). La misura si rilegge senza adattare numeri (A5-bis).

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
