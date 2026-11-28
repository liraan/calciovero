# L0 — la taratura fine (2026-11-15)

**Cos'è.** La fase 4 del mandato del maintainer del 2026-11-15: il motore misurato contro il calcio reale, metrica per metrica, con la sua banda e, per ogni metrica fuori banda, la causa nel modello o l'OI che la cerca. Nessun numero è stato ritoccato per rientrare in una banda (RULES A5-bis).

## Come si misura

- **Strumento:**
  ```bash
  cargo run --release -p ds-reference -- compare --reference L0-match/reference/serie-a-2017-18.json --seeds 42..101
  ```
  Finestra di taratura 42..101 (60 partite), squadre di prova, blocco medio e ampiezza normale. Non è un numero statistico: la banda è già la dispersione della partita vera (decisione Y), la finestra serve solo a rileggere la stessa partita più volte e a confrontare misure diverse. **Non fa parte dei controlli di chiusura** (RULES C2, 2026-11-20): si esegue quando serve una decisione sulle metriche, non a ogni intervento.
- **Riferimento:** Serie A 2017-18, 380 partite, Wyscout (Pappalardo et al. 2019, *Scientific Data* 6, 236; CC BY 4.0), per partita ed entrambe le squadre.
- **Motore:** `SIM_VERSION` 226 (in codice, DD-99); ultima misura: **13 su 20 confrontabili in banda** (sezione "La misura del 226"). Prima: `SIM_VERSION` 225 (DD-98), 13 su 20 (sezione "La misura del 225"); 224 (DD-97), 14 su 20 (sezione "La misura del 224"); 223 (DD-95), 15 su 20 (sezione "La misura del 223"). Storia: il 195 era 4 su 22, il 192 (DD-52/ADR-110) 6 su 22. Il conteggio "10 su 22" / "9 su 22" scritto prima per il 188 era sbagliato (tre righe segnate in banda fuori banda). Il SIM 189 (DD-49) è accantonato sul branch `sim-189-first-time`. Il 187 (DD-47) era 6/22 con parate 14.65. Il 185 ha spostato i `Threats` da `ds-match` all'orchestratore con aritmetica identica e nessuna misura nuova. La tabella storica sotto è l'ultima misura riga per riga del **171** (6 su 22), lasciata come confronto: le sue cause e i suoi OI valgono ancora dove non sono stati chiusi. SIM 167 (il possesso è il movimento) e SIM 168 (solo osservatori, golden identico) non cambiano il conteggio. Storia delle misure: al 168, dopo ADR-084 e ADR-085 (gol 4.80, passaggi 1237, duelli aerei 10.82, rimesse 17.87, 6 su 22); al 166, la palla libera conserva la velocità e continua a rotolare (gol 5.22, passaggi 1254, duelli aerei 10.62, rimesse 18.08, 6 su 22); al 165, l'emendamento ADR-071 sul posto senza palla letto dal giocatore (gol 5.22, duelli aerei 10.85, rimesse 18.27, 6 su 22); al 164, la scelta C di OI-102 (ADR-081): gol 5.25, duelli aerei 11.02, 6 su 22; al 163, i calci morti e il battitore scelto per ruolo (DD-21, ADR-080): gol 5.20, duelli aerei 10.82, 6 su 22; al 154, la posizione senza palla (ADR-071): 9 su 22; al 151, 11 su 22; al 147, i passi piccoli (ADR-069): gol 2.95, rimesse 25.9, 11 su 22; al 141, il colpo che costringe al cambio (ADR-066): 12 su 22. La taratura è ferma finché i comportamenti non sono giusti (decisione BK). La misura del 116, del 117 e del 118 è identica riga per riga: il 117 e il 118 cambiano solo le partite di coppa e le squadre con l'allenatore che reagisce. **SIM 170** (la pressione rimette alla prova il controllo del portatore, ADR-086) non è stato misurato separatamente: la misura 42..101 è stata fatta al 171, quando il difetto angolare di OI-100 chiedeva una decisione sulle metriche, non un rito di chiusura (RULES C2).
- **La banda:** media reale ± una deviazione standard delle 380 partite reali. **Il suo significato (decisione Y, maintainer, 2026-11-16):** L0 si chiude sulla credibilità della singola partita: una partita vera può finire 0-0 o 4-3, e la dispersione delle partite vere dice cosa è credibile. L1 si chiuderà sulle medie del campionato, con la banda sull'errore della media su una stagione intera, perché i totali di stagione (gol, capocannoniere) mostrano ogni scarto nelle medie. La fonte è lo stesso riferimento. È una regola sola per tutte le metriche, presa dalla dispersione del calcio vero e non dai valori del motore. Il caos del motore (circa 1.5 gol fra due esecuzioni che differiscono di un bit, SIM 101) sta dentro questa larghezza.
- **Non confrontabili:** cross, intercetti e i due recuperi (T14-bis). Le definizioni Wyscout e del motore differiscono (`ds-reference`, nota della tabella).

## Il fascicolo delle fonti (DD-96, ADR-116)

Il riferimento non è legato a un solo fornitore: `L0-match/reference/bundle.json`
dichiara, metrica per metrica, quale fonte reale la fornisce. Una nuova fonte
entra come dati (e, per un formato nuovo, un lettore in `ds-reference`), senza
un nuovo ADR e senza toccare il motore. Il filtro resta partita vera + LUNA:
un totale di squadra può essere un cross-check, non la verità su una scelta.
`ds-reference bundle` appiattisce il fascicolo e `compare` legge `comparable`
e `note` dal reference con fallback all'enum. Il fascicolo attuale contiene
solo Wyscout 2017-18: numeri, bande e conteggio in banda non cambiano.

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
  e va respinta. La soglia (22 su 22 dal 2026-11-24, `CLAUDE.md`) resta la misura; il giudizio finale è lo
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
soglia (22 su 22) misurano quella sensazione; il test del burattino la garantisce.
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
| rimesse laterali | 42.72 | 8.92 | 19.20 | 33.8–51.6 | no | OI-100: raddoppiate dal 127 (3.85) con il rimbalzo; nel calcio vero il 21,5% segue una spazzata, il 19,7% un tocco, il 31% un duello, il 24% un passaggio sbagliato (Wyscout); nel motore un duello non manda quasi mai la palla fuori e il passaggio sbaglia poco. DD-83 (RUN 76): nessuna modifica; il 31,2% è prossimità temporale, non causalità, e non si confronta con il 2,6% del motore; la causa resta ignota |
| rinvii dal fondo | 17.15 | 4.20 | 20.07 | 13.0–21.4 | sì | — |
| punizioni | 29.45 | 6.81 | 20.48 | 22.6–36.3 | no | OI-112: probabilmente di conteggio |
| fuorigioco | 4.26 | 2.40 | 8.10 | 1.9–6.7 | no | OI-098 / OI-104: la linea è letta con posizioni esatte, l'errore di percezione non è ancora costruito |
| spazzate | 29.53 | 8.65 | 55.92 | 20.9–38.2 | no | OI-100: la spazzata di testa resta una regola (ADR-064, passo 2) |
| parate | 9.37 | 3.37 | 18.85 | 6.0–12.7 | no | OI-100: troppe parate seguono i troppi tiri in porta |
| recupero, primo tempo (min) | 1.49 | 1.35 | 5.54 | 0.1–2.8 | no | OI-101: il motore aggiunge tutto il tempo perso, gli arbitri circa metà (fonte nell'OI) |
| recupero, secondo tempo (min) | 4.08 | 1.61 | 4.51 | 2.5–5.7 | sì | — |

## La misura del 208 (2026-11-25)

Il recupero della Regola 7.3 (OI-101).

| metrica | 207 | 208 | reale | causa | bug o taratura |
|---|---|---|---|---|---|
| stoppage_first_half_min | 6.86 | 3.75 | 1.49 ± 1.35 | 20 s a ogni fallo, che non è tempo perso (corretto); restano cartellini e perdite di tempo senza tolleranza misurata | bug corretto; il resto aperto |
| stoppage_second_half_min | 6.75 | 5.40 | 4.08 ± 1.61 | idem | in banda |
| ground_duels | 211.8 | 201.2 | 182.6 ± 27.5 | tempi più corti | in banda |

**In banda: 208 → 14 su 22.**

## La misura del 222 (2026-11-27, opencode)

Stesso comando e bande del 221. Un solo bump: DD?-88 toglie il gate
`line_in_sight` sul passaggio. Grezzo Wyscout: 1.620 offside su 380 partite
(4,263 ± 2,397, la banda); 1.327 (81,9%) hanno un pass della stessa squadra
entro 5 s e nel 93,1% di essi l'evento di offside cade esattamente
sull'endpoint del pass. Il dump non ha le posizioni dei difensori, quindi non
misura la linea: la storia "passaggio al ricevitore, poi bandierina" è
sostenuta, la geometria della linea no.

`offside_to` leggeva già Law 11 dalle posizioni percepite (ultimo sguardo
estrapolato) con la soglia dell'occhio di 8 mrad (ADR-062 (b)). Il gate
aggiungeva che i due difensori più profondi dovessero essere in `motion` in
quel tick, altrimenti il pass valeva zero: un secondo veto sulla scelta, non
un gate di percezione — il fischio arriva da posizioni reali
(`referee.rs:466-509`). La (c) di ADR-062 (linea letta dall'ultimo sguardo
fuori dall'attenzione) non è attuata e chiederebbe una soglia di freschezza
che nessuna fonte dà: non l'ho aggiunta (A5-bis).

**15 su 20 confrontabili in banda** (erano 11). Entrano in banda `offsides`,
`clearances`, `goal_kicks` e `free_kicks`; esce `goals`. `aerial_duels` 8,22
contro 38,14 ± 12,98 resta fuori e non è questa la sua causa (OI-100).

| metrica | 222 | 221 | reale | nota |
|---|---|---|---|---|
| goals | 5.28 | 4.73 | 2.68 ± 1.65 | fuori di 0,95, non tarato |
| own_goals | 0.30 | 0.27 | 0.10 ± 0.31 | in banda |
| shots | 29.10 | 27.27 | 24.60 ± 5.46 | in banda |
| shots_on_target | 9.85 | 8.55 | 8.69 ± 3.22 | in banda |
| penalties | 0.15 | 0.20 | 0.33 ± 0.56 | in banda |
| passes | 1193.28 | 1143.52 | 887.68 ± 122.81 | fuori, volume aperto (DD-82) |
| passes_accurate | 1079.88 | 1064.23 | 739.17 ± 124.79 | fuori, volume aperto (DD-82) |
| crosses | 8.68 | 6.90 | 33.83 ± 8.78 | non confrontabile |
| aerial_duels | 8.22 | 4.75 | 38.14 ± 12.98 | fuori, OI-100 |
| ground_duels | 159.70 | 156.70 | 182.63 ± 27.54 | in banda |
| take_ons_won | 19.03 | 18.55 | 16.03 ± 5.67 | in banda |
| fouls | 22.03 | 21.05 | 26.00 ± 6.69 | in banda |
| yellow_cards | 2.52 | 2.28 | 3.97 ± 1.96 | in banda |
| red_cards | 0.42 | 0.22 | 0.24 ± 0.49 | in banda |
| corners | 7.67 | 8.85 | 10.30 ± 3.58 | in banda |
| throw_ins | 22.75 | 21.40 | 42.72 ± 8.92 | fuori, OI-100 (RUN 76) |
| goal_kicks | 16.38 | 10.85 | 17.15 ± 4.20 | in banda |
| free_kicks | 26.77 | 21.32 | 29.45 ± 6.81 | in banda |
| offsides | 6.17 | 1.52 | 4.26 ± 2.40 | in banda, DD?-88 |
| clearances | 25.67 | 19.82 | 29.53 ± 8.65 | in banda |
| interceptions | 70.18 | 49.13 | 89.86 ± 13.97 | non confrontabile |
| saves | 5.08 | 4.35 | 6.70 ± 2.82 | in banda con T11 |
| stoppage_first_half_min | 2.73 | 2.57 | ≥1.49 ± 1.35 | non confrontabile (T14-bis) |
| stoppage_second_half_min | 6.21 | 5.60 | ≥4.08 ± 1.61 | non confrontabile (T14-bis) |

DD?-89 chiuso senza modifica (RUN 83): `clearances` 25,67 e `corners`
7,67 sono in banda sul 222, il grezzo riproduce 11.222 spazzate Wyscout
(29,531579 ± 8,650923, `eventId 7 / subEventId 71`) e il 78,6% dei precedenti
entro 5 s è dell'altra squadra. Il dump non registra pressione né opzioni di
passaggio: non autorizza a cambiare DD-5/DD-8/DD-10. All'epoca restavano
aperti DD?-91 (passaggi), DD?-92 (gol), DD?-93 (duelli aerei) e DD?-94
(rimesse): DD-93 chiuso in RUN 91 come unità Wyscout non confrontabile,
senza toccare ADR-081; DD-94 in RUN 92 con emissione completa e nessun gesto
«fuori». RUN 84:
la fonte riproduce 887,676 passaggi/m e 739,168 accurati/m (tag 1801,
83,27%); il motore conta l'esito del controllo del ricevitore (`complete`,
intercetti sempre inaccurati), quindi l'accuratezza è almeno in parte una
definizione diversa; le catene event-based del grezzo (334,89/m, 2,65
pass/catena) non sono confrontabili con quelle tick-based del motore. RUN 85:
i gol non hanno bug di conteggio (0 mismatch), ma la conversione è quasi
raddoppiata (0,1814 contro 0,1088 gol/tiro) mentre il volume dei tiri è in
banda; le etichette che precedono i gol precedono anche i tiri non segnati.
RUN 86: i duelli aerei non hanno uno slot perso (contenders in `Vec`,
`Pending` consumato nello stesso tick) e correggere il cross-tick darebbe al
massimo +0,43/partita; il grezzo dimezza le righe per giocatore e la frazione
«contesa» dipende dalla finestra di matching (19% a 1 secondo, 85% entro 1 s).
RUN 87: ogni uscita ha il suo `ThrowIn`, le 16.235 rimesse grezze danno
42,724 ± 8,921 con 0 duplicati e il 99,6% segue l'`Interruption`; il dump non
registra l'azione che ha mandato la palla fuori. Chiusura: DD-91 e DD-92
definizioni diverse (tag dell'accuratezza, placement e portata del portiere),
DD-93 unità Wyscout non confrontabile, DD-94 emissione completa senza gesto
«fuori»: nessun numero, soglia, finestra o golden toccato (A5-bis).

RUN 96 aggiunge una sonda sulle origini del duello aereo, senza modifica al
modello: delle 28.985 righe aeree del grezzo, il 45,3% segue un'altra riga
aerea entro 5 s (la seconda riga dello stesso duello); fra le prime righe,
passaggi alti e cross 31,1%, rinvii dal fondo 14,3%, punizioni 10,4%, angoli
9,6%, rimesse 4,6%. Lato motore (493 duelli su 60 semi, 8,22/partita):
`Pressure` 40,8%, passaggi 21,3%, spazzate 9,3%, angoli 8,7%, rimesse 7,7%,
punizioni 5,3%, **rinvii dal fondo 0%**. La ripartizione è una misura nuova,
non una soglia: il motore non contesta mai il rinvio e il 40,8% dei suoi
duelli nasce da una pressione, categoria assente nel referto. Non autorizza
a cambiare ADR-081 (DD-93 chiuso) né a tarare raggi o finestre (A5-bis); è
l'evidenza da portare a una nuova DD dottrinale, se si apre.

RUN 97 aggiunge una fonte nuova, sempre senza modifica al modello. Non esiste
Serie A maschile in StatsBomb open data: le 426 partite con 360 sono World Cup,
Euro, La Liga, Bundesliga, Ligue 1, MLS e tornei femminili. Ho letto gli eventi
delle 35 partite maschili di La Liga 2020/21 che hanno 360: 839 tiri (23,97 a
partita), 111 gol, 227 parati, 203 bloccati, 256 fuori, 15 pali, 4 parati fuori,
3 parati sul palo, 20 wayward; 616/839 end_location hanno la quota z.
Goal/(Goal+Saved) = 111/338 = **32,8%** (111/356 = 31,2% includendo pali e
parati sul palo); tiri in porta 40,3%, gol/tiro 13,2%. Confronti: Wyscout
Serie A 2017-18 30,8% di conversione, 35,3% di tiri in porta, 0,1088 gol/tiro;
motore SIM 222 53,6% di conversione, 33,9% di tiri in porta, 0,1814 gol/tiro.
La selezione dei tiri non spiega il divario: è la risoluzione del tiro in porta
da auditare come possibile bug (geometria di portiere e pali), non una soglia da
tarare (A5-bis). Il campione è un'altra lega e un'altra stagione, quindi non
confronta lo stesso evento del referto: non dà una banda nuova né una taratura.
Il match femminile `3835328` esaminato prima (22 tiri, 2 gol, conversione 20%)
aveva i free frame con portiere nominato e quota z, ma non basta a separare
geometria da definizione: due gol soli, istante del frame non dichiarato.

RUN 98 chiude l'audit del tiro senza trovare un bug di conteggio. Gol dal
punteggio (`engine.rs:18-21`), `on_target` geometrico
(`ds-orchestrator/src/lib.rs:592-601`), gol solo dall'attraversamento reale
(`ball.rs:220-230`), nessun doppio conteggio; `resolve_save`
(`ds-orchestrator/src/lib.rs:949-978`) è una soglia deterministica (ADR-035).
`L0-match/balance/save.json` dichiara i parametri «arbitrary, declared, to be
calibrated at L3 against real save rates (about seven shots on target in ten
are saved)»: la taratura non è mai stata fatta, ma scegliere oggi la portata
guardando i gol violerebbe A5-bis; la riga `saves` del resto è già in banda
(T11: 6,695 ± 2,823 contro 5,08 del motore). Ho misurato in un worktree
temporaneo, senza toccare l'albero principale, l'unico sospetto di stato rimasto
(T10 in `CODE-AUDIT.md`: il portiere è letto un tick prima di muoversi): spostare
`move_bodies` prima di `check_shot_crossing` (`state.rs:1013-1027`) lascia
**15 su 20** in banda; gol 5,28 → 5,02 (ancora fuori, banda fino a 4,33),
conversione 53,6% → 49,1%, parate 5,08 → 5,83, tiri in porta 9,85 → 10,23,
rimesse 22,75 → 20,57 (peggiora), duelli aerei 8,22 → 8,10, passaggi +5.
T10 è un effetto di ordine reale, ma non è la causa delle righe fuori banda:
è stato applicato dopo, con DD-95/ADR-115 e SIM 223, nella misura qui sotto.
Le cinque righe restano fuori banda con la loro causa, senza tarature.

Golden L0 base/istruito/coppa rigenerati (seed coppa 62 -> 78, hash
`78:24f14890…`); golden L1 `SMALL_GOLDEN` `0b00a6ad…`. Fixtures spostati:
header in fuorigioco 79 -> 48, livello al 90' 53 -> 49, knock 42, rigori
solo-tiri 62 -> 78. Il gemello riscritto
`an_unseen_offside_line_does_not_take_the_pass_away` legge la stessa scena
con e senza i due uomini in `motion` e pretende lo stesso valore del pass.

## La misura del 223 (2026-11-27, opencode, DD-95/ADR-115)

Stesso comando e bande del 222. DD-95 sposta `move_bodies` prima di
`check_shot_crossing` in `advance_ball` (`state.rs`): il portiere è letto nel
tick in cui la palla passa il piano, non in quello prima. Nessun numero,
soglia o portata cambiati (A5-bis).

**15 su 20 confrontabili in banda**, come nel 222. Fuori banda restano le
stesse cinque righe, con le cause già archiviate: `goals` 5,02 contro
2,68 ± 1,65, `passes` 1198,93 contro 887,68 ± 122,81, `passes_accurate`
1085,60 contro 739,17 ± 124,79, `aerial_duels` 8,10 contro 38,14 ± 12,98,
`throw_ins` 20,57 contro 42,72 ± 8,92 (peggiora di 2,18 rispetto al 222).

| metrica | 223 | 222 | reale | nota |
|---|---|---|---|---|
| goals | 5.02 | 5.28 | 2.68 ± 1.65 | fuori di 0,69, non tarato |
| own_goals | 0.27 | 0.30 | 0.10 ± 0.31 | in banda |
| shots | 29.35 | 29.10 | 24.60 ± 5.46 | in banda |
| shots_on_target | 10.23 | 9.85 | 8.69 ± 3.22 | in banda |
| penalties | 0.12 | 0.15 | 0.33 ± 0.56 | in banda |
| passes | 1198.93 | 1193.28 | 887.68 ± 122.81 | fuori, volume aperto (DD-82) |
| passes_accurate | 1085.60 | 1079.88 | 739.17 ± 124.79 | fuori, volume aperto (DD-82) |
| crosses | 8.37 | 8.68 | 33.83 ± 8.78 | non confrontabile |
| aerial_duels | 8.10 | 8.22 | 38.14 ± 12.98 | fuori, OI-100 |
| ground_duels | 159.18 | 159.70 | 182.63 ± 27.54 | in banda |
| take_ons_won | 19.38 | 19.03 | 16.03 ± 5.67 | in banda |
| fouls | 22.15 | 22.03 | 26.00 ± 6.69 | in banda |
| yellow_cards | 2.77 | 2.52 | 3.97 ± 1.96 | in banda |
| red_cards | 0.27 | 0.42 | 0.24 ± 0.49 | in banda |
| corners | 8.00 | 7.67 | 10.30 ± 3.58 | in banda |
| throw_ins | 20.57 | 22.75 | 42.72 ± 8.92 | fuori, DD-94 |
| goal_kicks | 15.78 | 16.38 | 17.15 ± 4.20 | in banda |
| free_kicks | 26.45 | 26.77 | 29.45 ± 6.81 | in banda |
| offsides | 5.65 | 6.17 | 4.26 ± 2.40 | in banda |
| clearances | 24.75 | 25.67 | 29.53 ± 8.65 | in banda |
| interceptions | 71.77 | 70.18 | 89.86 ± 13.97 | non confrontabile |
| saves | 5.83 | 5.08 | 6.70 ± 2.82 | in banda (T11) |
| stoppage_first_half_min | 3.12 | 2.73 | ≥1.49 ± 1.35 | non confrontabile (T14-bis) |
| stoppage_second_half_min | 5.47 | 6.21 | ≥4.08 ± 1.61 | non confrontabile (T14-bis) |

Parate 5,08 → 5,83, conversione dei tiri in porta 53,6% → 49,1%: il divario
con Wyscout (30,8%) e StatsBomb (32,8%) resta, coerentemente con i parametri
di `save.json` dichiarati arbitrari e da tarare a L3, mai tarati. Golden L0 e
L1 rigenerati, semi spostati (coppa 78 → 47, header 48 → 52, livello 49 → 47).

## La misura del 224 (2026-11-27, opencode, DD-97/ADR-117)

Stesso comando e bande del 223. DD-97 emenda ADR-042 §3 per il solo
contatto aereo: `resolve_aerial` legge portata, passo di arrivo e poi la
forza aerea già esistente, invece della sola forza. Nessun peso, soglia,
opzione o gesto nuovo: l'ordine arriva dai dati che `aerial_contenders`
calcolava già e nessun numero è stato scelto guardando la banda (A5-bis).

**14 su 20 confrontabili in banda** (15 nel 223). Fuori banda: `goals`
5,13 contro 2,68 ± 1,65 (peggiora di 0,11), `shots` 31,02 contro
24,60 ± 5,46 (29,35 nel 223, oltre il limite superiore di 30,06),
`passes` 1191,65 contro 887,68 ± 122,81, `passes_accurate` 1080,05 contro
739,17 ± 124,79, `aerial_duels` 8,55 contro 38,14 ± 12,98 (8,10 nel 223:
migliora di 0,45 restando sotto il limite inferiore di 25,16),
`throw_ins` 20,22 contro 42,72 ± 8,92. Le cause archiviate restano le
stesse: volume dei passaggi (DD-82), uscita laterale non attribuita dalla
fonte (DD-94), duello aereo (OI-100), tiro in porta (RUN 98). Nessuna di
queste righe è stata tarata.

| metrica | 224 | 223 | reale | nota |
|---|---|---|---|---|
| goals | 5.13 | 5.02 | 2.68 ± 1.65 | fuori, RUN 98 |
| own_goals | 0.28 | 0.27 | 0.10 ± 0.31 | in banda |
| shots | 31.02 | 29.35 | 24.60 ± 5.46 | fuori di 0,96, non tarato |
| shots_on_target | 10.33 | 10.23 | 8.69 ± 3.22 | in banda |
| penalties | 0.25 | 0.12 | 0.33 ± 0.56 | in banda |
| passes | 1191.65 | 1198.93 | 887.68 ± 122.81 | fuori, volume aperto (DD-82) |
| passes_accurate | 1080.05 | 1085.60 | 739.17 ± 124.79 | fuori, volume aperto (DD-82) |
| crosses | 8.63 | 8.37 | 33.83 ± 8.78 | non confrontabile |
| aerial_duels | 8.55 | 8.10 | 38.14 ± 12.98 | fuori, OI-100 |
| ground_duels | 161.18 | 159.18 | 182.63 ± 27.54 | in banda |
| take_ons_won | 20.00 | 19.38 | 16.03 ± 5.67 | in banda |
| fouls | 22.17 | 22.15 | 26.00 ± 6.69 | in banda |
| yellow_cards | 2.68 | 2.77 | 3.97 ± 1.96 | in banda |
| red_cards | 0.22 | 0.27 | 0.24 ± 0.49 | in banda |
| corners | 8.30 | 8.00 | 10.30 ± 3.58 | in banda |
| throw_ins | 20.22 | 20.57 | 42.72 ± 8.92 | fuori, DD-94 |
| goal_kicks | 16.13 | 15.78 | 17.15 ± 4.20 | in banda |
| free_kicks | 26.68 | 26.45 | 29.45 ± 6.81 | in banda |
| offsides | 6.15 | 5.65 | 4.26 ± 2.40 | in banda |
| clearances | 26.42 | 24.75 | 29.53 ± 8.65 | in banda |
| interceptions | 70.10 | 71.77 | 89.86 ± 13.97 | non confrontabile |
| saves | 5.87 | 5.83 | 6.70 ± 2.82 | in banda |
| stoppage_first_half_min | 2.98 | 3.12 | ≥1.49 ± 1.35 | non confrontabile (T14-bis) |
| stoppage_second_half_min | 5.76 | 5.47 | ≥4.08 ± 1.61 | non confrontabile (T14-bis) |

Golden L0 e L1 rigenerati, semi spostati (coppa e rigori solo-tiri 47 → 115,
header in fuorigioco 52 → 45, livello al 90' 47 → 44). Il gemello
`the_man_who_gets_there_first_wins_the_air` tiene ferma la scena e cambia
solo arrivo e portata. La riga `shots` è uscita di banda per effetto della
nuova risoluzione dei duelli aerei, non per una taratura: va indagata come
possibile bug prima di qualunque aggiustamento (A5-bis).

## La misura del 226 (2026-11-28, opencode, DD-99/ADR-119)

Stesso comando e bande del 225. Prima parte di T17: la rimessa è un lancio con
le mani a un compagno scelto dal battitore, non un calcio, e non segna
direttamente (Leggi 15 e 10). Il fatto vive nella scena del lanciatore e nel
`Pending` del suo passaggio; non c'è numero, soglia o gesto nuovo, e niente è
stato scelto guardando la banda (A5-bis). I due gemelli cambiano un solo fatto
a scena identica.

**13 su 20 confrontabili in banda** (13 nel 225). Fuori banda: `goals` 4,97
contro 2,68 ± 1,65, `shots` 30,70 contro 24,60 ± 5,46, `passes` 1188,43 contro
887,68 ± 122,81, `passes_accurate` 1077,40 contro 739,17 ± 124,79,
`aerial_duels` 8,87 contro 38,14 ± 12,98, `throw_ins` 20,53 contro 42,72 ± 8,92
e `offsides` 6,73 contro 4,26 ± 2,40 (6,95 nel 225: esce di 0,07 dal limite
alto). Le sei righe vecchie restano con le cause archiviate: volume dei
passaggi (DD-82), uscita laterale non attribuita dalla fonte (DD-94), duello
aereo (OI-100), tiro in porta (RUN 98). La settima è stata indagata in RUN 101:
il fischio non ha bug geometrico e il 73% dei casi nasce dalla linea difensiva
che arretra dopo l'ultima decisione dell'attaccante; i 50 cm restano invariati.
`ground_duels` 155,77 è appena dentro il limite basso (155,09): si tiene
d'occhio, non si tarà. Nessun numero è stato adattato per rientrare in banda.

| metrica | 226 | 225 | reale | nota |
|---|---|---|---|---|
| goals | 4.97 | 5.47 | 2.68 ± 1.65 | fuori, RUN 98 |
| own_goals | 0.33 | 0.23 | 0.10 ± 0.31 | in banda |
| shots | 30.70 | 31.90 | 24.60 ± 5.46 | fuori di 0,64, non tarato |
| shots_on_target | 10.87 | 10.90 | 8.69 ± 3.22 | in banda |
| penalties | 0.17 | 0.18 | 0.33 ± 0.56 | in banda |
| passes | 1188.43 | 1182.80 | 887.68 ± 122.81 | fuori, volume aperto (DD-82) |
| passes_accurate | 1077.40 | 1067.63 | 739.17 ± 124.79 | fuori, volume aperto (DD-82) |
| crosses | 7.97 | 9.55 | 33.83 ± 8.78 | non confrontabile |
| aerial_duels | 8.87 | 8.40 | 38.14 ± 12.98 | fuori, OI-100 |
| ground_duels | 155.77 | 160.78 | 182.63 ± 27.54 | in banda, 0,68 dal limite basso |
| take_ons_won | 18.25 | 19.52 | 16.03 ± 5.67 | in banda |
| fouls | 21.77 | 21.70 | 26.00 ± 6.69 | in banda |
| yellow_cards | 2.83 | 2.52 | 3.97 ± 1.96 | in banda |
| red_cards | 0.17 | 0.33 | 0.24 ± 0.49 | in banda |
| corners | 8.60 | 8.52 | 10.30 ± 3.58 | in banda |
| throw_ins | 20.53 | 21.05 | 42.72 ± 8.92 | fuori, DD-94 |
| goal_kicks | 17.62 | 18.05 | 17.15 ± 4.20 | in banda |
| free_kicks | 27.30 | 27.37 | 29.45 ± 6.81 | in banda |
| offsides | 6.73 | 6.95 | 4.26 ± 2.40 | fuori di 0,07, indagata RUN 101: non bug |
| clearances | 26.02 | 25.42 | 29.53 ± 8.65 | in banda |
| interceptions | 72.90 | 72.37 | 89.86 ± 13.97 | non confrontabile |
| saves | 6.53 | 6.15 | 6.70 ± 2.82 | in banda |
| stoppage_first_half_min | 2.95 | 3.45 | ≥1.49 ± 1.35 | non confrontabile (T14-bis) |
| stoppage_second_half_min | 5.55 | 5.64 | ≥4.08 ± 1.61 | non confrontabile (T14-bis) |

Golden L0 e L1 rigenerati, semi spostati: coppa e rigori solo-tiri 55 → 58,
livello al 90' 55 → 45, header in fuorigioco 42 invariato. Il golden coppa è
`58:2e8d8dd470fe1cac8b4563f1f525bfbc94f0885bb928c052bb5fb916fd71d36b`.
Gol, tiri e fuorigioco si muovono un po' verso la banda senza tarature: restano
indizi, non numeri da ricalibrare. La parte restante di T17 (primo atto delle
altre riprese, punizione indiretta) non è stata toccata.

## RUN 101 — il fischio di fuorigioco letto sul passaggio (2026-11-28, sola lettura)

Domanda aperta dal 225: `offsides` 6,73 contro 4,26 ± 2,40 — bug del fischio o
differenza dichiarata? Sonda temporanea su `tools/ds-reference/examples/` (poi
rimossa): per ogni `FreeKick` con `reason: offside` dei semi 42..101, ricostruisce
dal tick record la linea del secondo avversario **vera** al `played_tick`, il
posto dell'autore e la stessa linea tre secondi prima. Nessun numero, soglia o
gesto del modello è stato toccato (A5-bis).

**Il fischio non ha bug geometrico: 404 su 404** (6,73 ± 2,36 a partita) hanno
l'autore oltre la linea del secondo avversario quando il pallone è stato
giocato; nessuno è stato flagrato in posizione regolare. La deviazione del
motore, 2,36, è quasi la stessa del referto, 2,40: la media è 1,03 DS sopra
quella reale, appena fuori banda.

La premessa scritta nel 226 era sbagliata e qui si corregge: il fischio **non**
legge posizioni percepite. `mark_offside_at_pass` (`referee.rs:499-505`) usa
`self.spatial.agent`, cioè i corpi veri; `offside_to` (`read.rs:182`) serve solo
alla lettura del passatore, che può credere onside un compagno che l'arbitro
vede oltre la linea (ADR-062 (b)). Le due letture non sono lo stesso atto.

**Causa dei flag:** 361 su 404 hanno visto la linea arretrare verso la propria
porta nei tre secondi precedenti; in 295 (73% del totale) l'autore era ancora
regolare tre secondi prima ed è la linea ad averlo superato, mentre in 109 era
già oltre. Distanza oltre la linea al passaggio: 130 entro 50 cm, 105 tra 51 e
150, 100 tra 151 e 400, 69 oltre 400. Il margine di 50 cm usato per il posto
delle punte (ADR-118) non c'entra con il fischio e non va cambiato guardando il
conteggio.

**Conclusione:** non è un bug da correggere né una taratura da fare. La riga
resta fuori di 0,07 con la causa dichiarata: la linea difensiva si arretra
nell'intervallo tra due decisioni dell'attaccante, in un motore che gioca più
passaggi del referto (DD-82). Nessun cambio di motore, nessun `SIM_VERSION` nuovo.

## La misura del 225 (2026-11-27, opencode, DD-98/ADR-118)

Stesso comando e bande del 224. T23: con la palla le punte aspettano sulla
linea del penultimo avversario, corta dello stesso `onside_margin_cm` già usato
dalla corsa di ADR-070, anche senza compito sulla lavagna. Nessun gate sul
passaggio, nessun margine, peso o gesto nuovo, nessun numero preso guardando
la banda (A5-bis). Il gemello `the_forward_waits_on_the_line_without_an_order`
cambia solo il ruolo nella stessa scena.

**13 su 20 confrontabili in banda** (14 nel 224). Fuori banda: `goals` 5,47
contro 2,68 ± 1,65, `shots` 31,90 contro 24,60 ± 5,46, `passes` 1182,80 contro
887,68 ± 122,81, `passes_accurate` 1067,63 contro 739,17 ± 124,79,
`aerial_duels` 8,40 contro 38,14 ± 12,98, `throw_ins` 21,05 contro 42,72 ± 8,92
e `offsides` 6,95 contro 4,26 ± 2,40 (6,15 nel 224: esce di 0,29 dal limite
alto). Le sei righe vecchie restano con le cause archiviate: volume dei
passaggi (DD-82), uscita laterale non attribuita dalla fonte (DD-94), duello
aereo (OI-100), tiro in porta (RUN 98). La settima è nuova e va indagata come
possibile bug, non tarata: il posto della punta legge `second_last` da posizioni
vere, il fischio legge `offside_to` da posizioni percepite con la soglia
dell'occhio (ADR-062 (b)); i 50 cm di margine non sono una soglia di vista scritta
e non vanno cambiati guardando il conteggio (A5-bis).

| metrica | 225 | 224 | reale | nota |
|---|---|---|---|---|
| goals | 5.47 | 5.13 | 2.68 ± 1.65 | fuori, RUN 98 |
| own_goals | 0.23 | 0.28 | 0.10 ± 0.31 | in banda |
| shots | 31.90 | 31.02 | 24.60 ± 5.46 | fuori di 1,84, non tarato |
| shots_on_target | 10.90 | 10.33 | 8.69 ± 3.22 | in banda |
| penalties | 0.18 | 0.25 | 0.33 ± 0.56 | in banda |
| passes | 1182.80 | 1191.65 | 887.68 ± 122.81 | fuori, volume aperto (DD-82) |
| passes_accurate | 1067.63 | 1080.05 | 739.17 ± 124.79 | fuori, volume aperto (DD-82) |
| crosses | 9.55 | 8.63 | 33.83 ± 8.78 | non confrontabile |
| aerial_duels | 8.40 | 8.55 | 38.14 ± 12.98 | fuori, OI-100 |
| ground_duels | 160.78 | 161.18 | 182.63 ± 27.54 | in banda |
| take_ons_won | 19.52 | 20.00 | 16.03 ± 5.67 | in banda |
| fouls | 21.70 | 22.17 | 26.00 ± 6.69 | in banda |
| yellow_cards | 2.52 | 2.68 | 3.97 ± 1.96 | in banda |
| red_cards | 0.33 | 0.22 | 0.24 ± 0.49 | in banda |
| corners | 8.52 | 8.30 | 10.30 ± 3.58 | in banda |
| throw_ins | 21.05 | 20.22 | 42.72 ± 8.92 | fuori, DD-94 |
| goal_kicks | 18.05 | 16.13 | 17.15 ± 4.20 | in banda |
| free_kicks | 27.37 | 26.68 | 29.45 ± 6.81 | in banda |
| offsides | 6.95 | 6.15 | 4.26 ± 2.40 | fuori di 0,29, da indagare |
| clearances | 25.42 | 26.42 | 29.53 ± 8.65 | in banda |
| interceptions | 72.37 | 70.10 | 89.86 ± 13.97 | non confrontabile |
| saves | 6.15 | 5.87 | 6.70 ± 2.82 | in banda |
| stoppage_first_half_min | 3.45 | 2.98 | ≥1.49 ± 1.35 | non confrontabile (T14-bis) |
| stoppage_second_half_min | 5.64 | 5.76 | ≥4.08 ± 1.61 | non confrontabile (T14-bis) |

Golden L0 e L1 rigenerati, semi spostati: coppa e rigori solo-tiri 115 → 55,
livello al 90' 44 → 55, header in fuorigioco 45 → 42. Il golden coppa è
`55:68dff084d68b080c174315186aff2ae676179e462659ace9e975a5b12549fc86`. Gol e
tiri salgono insieme alle punte che ora restano in posizione valida: sono
indizi di una causa a valle, non numeri da ricalibrare.

## RUN 100 — un campione 360: cosa dice e cosa non dice (2026-11-27, sola lettura)

Fuori dal repository, da `statsbomb/open-data/data/three-sixty/3773377.json`
(La Liga 2020/21, Barcellona–Valencia, la partita con più tiri delle 35 del
RUN 97): 3.512 frame, tutti con UUID presente nei 3.717 eventi già scaricati.
Nessun file tracciato del motore toccato.

Al tiro: 35/35 frame, 34 con un solo portiere nominato, 25 con quota
z dell'endpoint, 24 con entrambi. Distanza x–y portiere → endpoint (coordinate
StatsBomb 120×80 yard): mediana 1,79 yard nei 8 `Saved`, 4,27 nei 4 `Goal`,
6,45 nei 12 `Off T`, 16,07 nei 9 `Blocked`. Con LUNA il dito è la posizione
del portiere nel frame dell'evento e la luna è lo stesso uomo che ha letto il
volo e scelto COME parare: il frame registra dove era, non la scelta, la quota
del corpo o l'istante dell'attraversamento. La fonte dà quindi una geometria
vera al tiro, ma non la portata in aria: non chiude DD-92 e non autorizza a
tarare `save.json` (A5-bis). Un solo campione, 4 gol: nessuna banda nuova.

Ai duelli aerei: 19 eventi `Duel` di tipo `Aerial` nella partita, 18 con frame.
I frame dei duelli hanno le coordinate ruotate di 180° rispetto a `location`
(`(x, y) → (120−x, 80−y)`); allineati sull'attore, tutti e 18 hanno almeno 2
giocatori entro 2 yard dal punto del duello (mediana 2, massimo 3). Con LUNA il
dito sono i corpi presenti e l'etichetta dell'evento, la luna è chi sceglie
COME contendere: il frame conferma la coppia del duello già etichettato, non
quanti voli alti diventano duelli. La causa del volume (38,14 reali contro
8,10 del motore) resta come nel RUN 96: DD-93 resta chiuso, nessun raggio o
finestra nuova.

Esito: nessun bump, golden, seme, parametro o test toccato. 15 su 20
confrontabili in banda, invariato; `goals` e `aerial_duels` restano fuori
banda con le cause archiviate.

## La misura del 221 (2026-11-27, opencode)

Stesso comando e bande del 220. Un solo bump: DD-86 toglie il veto
sull'esito immaginato del loft conteso. Il 39,26% dei duelli reali con un
passaggio alto/cross nei 5 s precedenti è vinto da chi ha calciato: il
passatore sceglie il loft anche perdendo l'aria, quindi `sure = 0` era un
clamp sul COME, non percezione (ADR-062) né regola. SIM 221 gradua con la
forza aerea immaginata solo il ramo che il veto azzerava; il ramo già vinto
resta bit-identico al 220. Una prima versione che graduava anche il ramo
vinto aveva portato `aerial_duels` a 3,48 e indebolito il guard del caso
forte: corretta come bug, non ricalibrata guardando la tabella (A5-bis).

**11 su 20 confrontabili in banda**, come al 220; nessuna riga entra o esce
dalla banda. `aerial_duels` 4,75 contro 38,14 ± 12,98 resta fuori: il veto
era falso, ma non era il serbatoio dei ~33 duelli mancanti (RUN 79: la
percezione attuale ne può recuperare +0,62/match; il 40,0% dei duelli reali
segue una ripresa, causa separata). `goals` 4,73 esce di 0,40 dalla banda
alta (era 4,33, fuori di 0,008): il cambiamento delle scelte di passaggio si
propaga, come già accaduto al 219; non si tarato per riportarlo dentro.

| metrica | 221 | 220 | reale | nota |
|---|---|---|---|---|
| goals | 4.73 | 4.33 | 2.68 ± 1.65 | fuori di 0,40, non tarato |
| own_goals | 0.27 | 0.20 | 0.10 ± 0.31 | in banda |
| shots | 27.27 | 26.55 | 24.60 ± 5.46 | in banda |
| shots_on_target | 8.55 | 8.17 | 8.69 ± 3.22 | in banda |
| penalties | 0.20 | 0.13 | 0.33 ± 0.56 | in banda |
| passes | 1143.52 | 1137.20 | 887.68 ± 122.81 | fuori, volume aperto (DD-82) |
| passes_accurate | 1064.23 | 1056.60 | 739.17 ± 124.79 | fuori, volume aperto (DD-82) |
| crosses | 6.90 | 7.45 | 33.83 ± 8.78 | non confrontabile |
| aerial_duels | 4.75 | 4.87 | 38.14 ± 12.98 | fuori, DD-86: veto falso, non il volume |
| ground_duels | 156.70 | 158.58 | 182.63 ± 27.54 | in banda |
| take_ons_won | 18.55 | 18.80 | 16.03 ± 5.67 | in banda |
| fouls | 21.05 | 21.42 | 26.00 ± 6.69 | in banda |
| yellow_cards | 2.28 | 2.15 | 3.97 ± 1.96 | in banda |
| red_cards | 0.22 | 0.28 | 0.24 ± 0.49 | in banda |
| corners | 8.85 | 9.17 | 10.30 ± 3.58 | in banda |
| throw_ins | 21.40 | 22.57 | 42.72 ± 8.92 | fuori, OI-100 (RUN 76) |
| goal_kicks | 10.85 | 10.35 | 17.15 ± 4.20 | fuori, DD-85 senza modifica |
| free_kicks | 21.32 | 21.77 | 29.45 ± 6.81 | fuori di 1,32 |
| offsides | 1.52 | 1.60 | 4.26 ± 2.40 | fuori, DD?-88 |
| clearances | 19.82 | 20.13 | 29.53 ± 8.65 | fuori, DD?-89 |
| interceptions | 49.13 | 49.75 | 89.86 ± 13.97 | non confrontabile |
| saves | 4.35 | 4.27 | 6.70 ± 2.82 | in banda con T11 |
| stoppage_first_half_min | 2.57 | 2.48 | ≥1.49 ± 1.35 | non confrontabile (T14-bis) |
| stoppage_second_half_min | 5.60 | 5.27 | ≥4.08 ± 1.61 | non confrontabile (T14-bis) |

Golden L0 base/istruito/coppa rigenerati (seed coppa 166 -> 62); golden L1
`3e8babad…`. Fixtures spostati dalla nuova-play: header in fuorigioco 50 ->
79, livello al 90' 44 -> 53, knock 43 -> 42, rigori solo-tiri 166 -> 62. Il
guard del movimento ora esclude il corpo mandato in panchina dopo un rosso.

## La misura del 220 (2026-11-26, opencode)

Stesso comando e bande del 219. Un solo bump: DD-84 emenda DD-71. La fonte
dice che `take_ons_won` è il duello d'attacco a terra vinto con il tag
take-on (`eventId 1`, `subEventId 11`, tag 503/504 + 703), non il superamento
fisico dell'uomo: sul grezzo locale quei duelli sono 16,03/partita, esattamente
la media del referto, mentre i duelli difensivi vinti con lo stesso tag
(6,97/partita) non sono conteggiati dalla fonte. Il motore raccontava
l'etichetta `Dribble` solo dopo il superamento, con un confronto con la porta
che non era né percezione né regola del gioco; ora la racconta al contatto
vinto, punto comune dei rami contrasto e jockey. Nessun numero scelto guardando
la tabella (A5-bis). **11 su 20 confrontabili** (era 10): entra in banda solo
`take_ons_won`, tutto il resto è invariato.

| metrica | 220 | 219 | reale | nota |
|---|---|---|---|---|
| goals | 4.33 | 4.33 | 2.68 ± 1.65 | fuori di 0,008, non tarato |
| own_goals | 0.20 | 0.20 | 0.10 ± 0.31 | in banda |
| shots | 26.55 | 26.55 | 24.60 ± 5.46 | in banda |
| shots_on_target | 8.17 | 8.17 | 8.69 ± 3.22 | in banda |
| penalties | 0.13 | 0.13 | 0.33 ± 0.56 | in banda |
| passes | 1137.20 | 1137.20 | 887.68 ± 122.81 | fuori, volume aperto (DD-82) |
| passes_accurate | 1056.60 | 1056.60 | 739.17 ± 124.79 | fuori, volume aperto (DD-82) |
| crosses | 7.45 | 7.45 | 33.83 ± 8.78 | non confrontabile |
| aerial_duels | 4.87 | 4.87 | 38.14 ± 12.98 | fuori, DD?-86 |
| ground_duels | 158.58 | 158.58 | 182.63 ± 27.54 | in banda |
| take_ons_won | 18.80 | 7.98 | 16.03 ± 5.67 | in banda, DD-84 chiude la definizione |
| fouls | 21.42 | 21.42 | 26.00 ± 6.69 | in banda |
| yellow_cards | 2.15 | 2.15 | 3.97 ± 1.96 | in banda |
| red_cards | 0.28 | 0.28 | 0.24 ± 0.49 | in banda |
| corners | 9.17 | 9.17 | 10.30 ± 3.58 | in banda |
| throw_ins | 22.57 | 22.57 | 42.72 ± 8.92 | fuori, OI-100 (RUN 76) |
| goal_kicks | 10.35 | 10.35 | 17.15 ± 4.20 | fuori, DD-85 senza modifica: uscite dalla linea, non scelta del portiere |
| free_kicks | 21.77 | 21.77 | 29.45 ± 6.81 | fuori di 0,87 |
| offsides | 1.60 | 1.60 | 4.26 ± 2.40 | fuori, DD?-88 |
| clearances | 20.13 | 20.13 | 29.53 ± 8.65 | fuori, DD?-89 |
| interceptions | 49.75 | 49.75 | 89.86 ± 13.97 | non confrontabile |
| saves | 4.27 | 4.27 | 6.70 ± 2.82 | in banda con T11 |
| stoppage_first_half_min | 2.48 | 2.48 | ≥1.49 ± 1.35 | non confrontabile (T14-bis) |
| stoppage_second_half_min | 5.27 | 5.27 | ≥4.08 ± 1.61 | non confrontabile (T14-bis) |

Golden L0 base/istruito/coppa rigenerati (seed coppa 166 invariato); golden L1
e tutti gli scan dei semi (44, 42, 166, 43) invariati. La cronaca cambia, non
il gioco: tutte le altre righe sono bit-identiche al 219.

## La misura del 219 (2026-11-26, opencode)

Stesso comando e bande del 218. Un solo bump: DD-81 toglie il gate duro
`PASS_FREEDOM_CM` dalle opzioni di passaggio a terra. Il compagno marcato
resta una scelta e lo spazio lo gradua con il `freedom_share` che ADR-040 già
usava; nessun numero, peso o soglia è stato scelto guardando la tabella
(A5-bis). Il gemello `a_marked_mate_is_still_a_pass_option` è rosso con il
gate e verde senza. **10 su 20 confrontabili** (era 11): il volume dei
passaggi non si chiude con questa correzione, e `goals` esce di 0,008 dalla
banda alta (4,330 contro 4,322). Nessuna di queste due righe è stata
"aggiustata": la decisione era sul modello della scelta, non sulla misura.

| metrica | 219 | 218 | reale | nota |
|---|---|---|---|---|
| goals | 4.33 | 4.00 | 2.68 ± 1.65 | fuori di 0,008, non tarato |
| own_goals | 0.20 | 0.20 | 0.10 ± 0.31 | in banda |
| shots | 26.55 | 26.77 | 24.60 ± 5.46 | in banda |
| shots_on_target | 8.17 | 8.12 | 8.69 ± 3.22 | in banda |
| penalties | 0.13 | 0.17 | 0.33 ± 0.56 | in banda |
| passes | 1137.20 | 1138.15 | 887.68 ± 122.81 | fuori, volume aperto (DD-82) |
| passes_accurate | 1056.60 | 1060.28 | 739.17 ± 124.79 | fuori, volume aperto (DD-82) |
| crosses | 7.45 | 7.13 | 33.83 ± 8.78 | non confrontabile |
| aerial_duels | 4.87 | 4.38 | 38.14 ± 12.98 | fuori, DD?-86 |
| ground_duels | 158.58 | 156.50 | 182.63 ± 27.54 | in banda |
| take_ons_won | 7.98 | 8.48 | 16.03 ± 5.67 | fuori, definizione chiusa da DD-84 al 220 |
| fouls | 21.42 | 20.65 | 26.00 ± 6.69 | in banda |
| yellow_cards | 2.15 | 2.38 | 3.97 ± 1.96 | in banda |
| red_cards | 0.28 | 0.20 | 0.24 ± 0.49 | in banda |
| corners | 9.17 | 9.23 | 10.30 ± 3.58 | in banda |
| throw_ins | 22.57 | 22.37 | 42.72 ± 8.92 | fuori, DD?-83 |
| goal_kicks | 10.35 | 11.07 | 17.15 ± 4.20 | fuori, DD?-85 |
| free_kicks | 21.77 | 21.00 | 29.45 ± 6.81 | fuori, DD?-88 |
| offsides | 1.60 | 1.57 | 4.26 ± 2.40 | fuori, DD?-88 |
| clearances | 20.13 | 20.20 | 29.53 ± 8.65 | fuori, DD?-89 |
| interceptions | 49.75 | 47.87 | 89.86 ± 13.97 | non confrontabile |
| saves | 4.27 | 4.52 | 6.70 ± 2.82 | in banda con T11 |
| stoppage_first_half_min | 2.48 | 2.27 | ≥1.49 ± 1.35 | non confrontabile (T14-bis) |
| stoppage_second_half_min | 5.27 | 5.15 | ≥4.08 ± 1.61 | non confrontabile (T14-bis) |

Golden L0 base/istruito/coppa rigenerati (primo seed di coppa 43 → 166),
golden L1 aggiornato, seed della GUI 42 invariato. La scoperta economica è
che il gate duro quasi non incideva sul conteggio: il modello era sbagliato
per la dottrina (una scelta tolta al giocatore), non perché producesse i
passaggi in eccesso. Il volume residuo resta una questione di definizione e
di tempo di palla viva, non di una soglia da abbassare.

## DD-86: il loft conteso resta una scelta (2026-11-26, opencode)

La riga `aerial_duels` (4,87 contro 38,14 ± 12,98) non chiude una differenza
di definizione e non si taratura: la fonte è riprodotta esattamente (28.985
record Wyscout `eventId 1 / subEventId 10` su 380 partite, dimezzati perché
il referto registra entrambi i contendenti), e i tag dicono solo esito e
accuratezza, non il contatto fisico. Il motore emette il duello solo con due
corpi in portata nello stesso sottopasso del tick; con la percezione attuale
il serbatoio recuperabile è +0,62 duelli/partita (SIM 216, meccanica
invariata al 220), non i ~33 mancanti. Riaprire l'opzione B di ADR-081 darebbe
12,12 duelli e distruggerebbe il guard del piede dei laterali.

Con LUNA il passatore sceglie COME giocare, non se vincerà l'aria. Il dato di
partita vera toglie la premessa del veto: su 6.639 contese con un pass
alto/cross nei cinque secondi precedenti vince la squadra che ha calciato
2.384/6.073 = 39,26% degli esiti decisi (robusta: 39,22% pesando i record
singoli, 39,28%/39,41% con finestre di accoppiamento 0,5/1 s, 39,55% col
primo pass). Il giocatore lofta anche quando perde la contesa, quindi il
`sure = 0` di `read.rs:1691-1698` non era percezione (ADR-062) né regola, ma
un clamp sul COME: DD-81 aveva già rimosso lo stesso tipo di gate sul pass a
terra.

SIM 221 conserva l'opzione: il ramo che `wins()` vietava è graduato dalla
quota della forza aerea immaginata del proprio uomo su quella del miglior
avversario che arriva all'atterraggio; il ramo già vinto resta invariato.
Nessuna costante nuova (schema di `freedom_share`, ADR-040). ADR-081 opzione C, il drop point letto esattamente e i gate di
percezione restano invariati. Il 39,26% non è entrato nel modello (A5-bis).
La misura 42..101 è sotto; il 40,0% dei duelli reali che seguono una ripresa
resta una causa separata, registrata qui e non risolta da questo bump.

## DD-85: il rinvio è raccontato al fischio, non dopo la scelta del portiere (2026-11-26, opencode)

La domanda aperta su `goal_kicks` (10,35 contro 17,15 ± 4,20) chiedeva se
correggere il modello del portiere che giocherebbe corto in 96/111 rinvii,
oppure se chiudere la riga come differenza di definizione. **Nessuna delle due
è stata attuata.**

La fonte è riprodotta esattamente: `tools/ds-reference/src/wyscout.rs:91,185-192`
conta `subEventId=34` senza filtri su `eventId` o tag; sul grezzo locale
(647.372 eventi, 380 partite) sono 6.518 rinvii, 6.518/380 = 17,153, la media
del referto. L'emissione del motore è completa (111/111 assegnati ed emessi,
RUN 66) e ha un solo punto, `plugins/referee.rs:707-724`, al fischio: la
scelta del portiere (`set_pieces.rs:241-303`, `read.rs:1903-1939`) arriva
dopo, quindi non può cambiare il conteggio dei rinvii. Con LUNA il portiere
sceglie COME battere, ma il rinvio è la Legge 16: è il fatto che la palla è
uscita dalla linea di porta dopo un tocco d'attaccante, non una sua decisione.

La premessa dei 96/111 passaggi corti era il doppio racconto già tolto da
DD-90/SIM 218: il `Pass` 41 tick dopo il `GoalKick` era la seconda etichetta
dello stesso calcio. Anche il "10,4% reale entro 10 s" del RUN 66 non è
riproducibile sul dump: con definizione esplicita i Pass dello stesso
battitore entro 10 s sono 498/6.518 = 7,64% (3,71% se `sub=85`, 1,15% se è il
primo evento). Il referto non registra GoalKick e Pass per lo stesso calcio:
il passaggio successivo parte da un'altra posizione ed è un'azione nuova.
Chiudere come "differenza di definizione" è quindi falso: fonte e motore
contano lo stesso atto nominale e la Legge 16 è attuata (`referee.rs:179-201`).

Resta una differenza di dinamica, non di registro: 19,52 uscite dalla linea di
porta per partita nel motore (10,35 rinvii + 9,17 angoli) contro 27,45 reali
(17,15 + 10,30), con quota rinvio 53,0% contro 62,5%. Gli angoli sono in banda:
non è un difetto generico delle uscite. Va indagato con una sonda sull'ultimo
tocco e sulle uscite (OI-100), non con una soglia letta dalla tabella
(A5-bis). **Nessuna modifica al modello, nessun bump di `SIM_VERSION`, nessun
golden, seme o guard da rigenerare.** Rapporto completo in
`target/opencode-log.md`, RUN 78.

## DD-83: le rimesse non si chiudono né con il lookahead né con DD-76 (2026-11-26, opencode)

La domanda aperta su `throw_ins` (22,57 contro 42,72 ± 8,92) proponeva di fare valere il lancio solo la migliore giocata successiva a terra del compagno, oppure di chiudere OI-100 come conseguenza dichiarata di DD-76. **Nessuna delle due è stata attuata.**

La prima reintroduce il lookahead nel ramo loftato che ADR-101 §Conseguenze esclude e DD-76 aveva già revocato dopo aver misurato angoli 18,97 e autogol 0,47 (RUN 17); inoltre non insegna l'uscita laterale, perché `lofted_missed_at` vede solo la propria porta, non la linea di touch. La seconda attribuisce male la causa: DD-76 spiega il calo 33,17 → 19,37, ma il divario esisteva già prima, 42,72 − 33,17 = 9,55 rimesse/match, quando i duelli che mandavano fuori erano già il 2,4% (RUN 20).

La seconda inoltre si appoggia su un confronto invalido: il 31,2% di duelli prima della rimessa, citato in questa stessa tabella e in RUN 65 come "Wyscout" (non StatsBomb), è la distribuzione di un evento qualunque in una finestra temporale con gap mediano 10,11 s, non l'attribuzione dell'azione che ha mandato la palla fuori; il 2,6% del motore è invece l'ultimo vero tocco. Le due percentuali non vanno confrontate e da lì non si può derivare una regola. DD-76 copre solo il calo storico 33,17 → 19,37, mentre il divario strutturale (42,72 − 33,17 = 9,55/match) esisteva già prima. L'emissione è completa (RUN 20, 37, 65: ogni uscita ha il suo `ThrowIn`) e l'opzione di calciare in fascia esiste ed è verde (`pressed_with_nothing_on_he_puts_it_into_touch`): con LUNA non manca una scelta. La causa del residuo resta ignota: una regola sui duelli richiederebbe un dataset che registri l'uscita stessa, non la prossimità degli eventi. OI-100 resta aperto con questa correzione di fonte; nessun comportamento o `SIM_VERSION` cambia. Rapporto completo in `target/opencode-log.md`, RUN 76.

## DD-82: il tempo morto non spiega il volume dei passaggi (2026-11-26, opencode)

La domanda aperta su `passes` e `passes_accurate` era se aggiungere fermate
dopo falli, infortuni e cambi per portare il tempo vivo del motore ai ~57′
citati in RUN 35. I timestamp del grezzo Wyscout locale (647.372 eventi, 380
partite) smentiscono la premessa: il ritardo mediano ripresa → prima azione è
2,85 s per una rimessa, 3,70 s per un calcio libero, 4,69 s per un angolo e
3,72 s per un rinvio dal fondo, già vicino ai 4-7 s del motore. Sommandogli i
gol e unendo gli intervalli sovrapposti, il morto reale è **8,70 min/partita**
su 95,56 minuti trascorsi, quindi **86,86 minuti di palla viva**; con la
definizione più stretta (fino al passaggio successivo) il morto sale a circa
13,9 minuti. Il motore ha già 14,72 minuti morti e 84,25 vivi (RUN 35, SIM
precedente): aggiungere pause non avvicina la misura, la allontana. Il "~57′"
non ha fonte in `L0-match/reference/` ed è contraddetto dai dati locali.

La decisione delegata è quindi di **non modificare il modello**: nessun
secondo inventato, nessuna soglia, nessun bump di `SIM_VERSION`. `passes`
resta confrontabile, perché referto e motore contano entrambi i tentativi di
passaggio per partita, e resta fuori banda con causa nota nel ritmo di scelta:
337,58 catene contro circa 200 possessi per squadra, con 4,45 passaggi per
catena già corretti (RUN 35, sonda B). Il tasso è 13,50 pass/minuto vivo nel
motore contro 10,22 reali. La causa va presa con un DD separato sul numero di
possessi, non sul cronometro. Nessun numero è stato scelto guardando la tabella
(A5-bis); il rapporto completo è in `target/opencode-log.md`, RUN 75.

## La misura del 218 (2026-11-26, opencode)

Stesso comando e bande del 217. Un solo bump: il calcio di una ripresa giocato a un compagno era raccontato due volte (`GoalKick`/`Corner`/`ThrowIn`/`FreeKick` al fischio e `Pass` all'atterraggio); il grezzo Wyscout locale non ha nessuna coppia con le stesse posizioni su 37.861 riprese, quindi il secondo evento era un doppio conteggio, non una definizione diversa. Nessun numero scelto guardando la tabella (A5-bis). **11 su 20 confrontabili, invariato rispetto al 217.**

| metrica | 218 | 217 | reale | nota |
|---|---|---|---|---|
| passes | 1138,15 | 1175,30 | 887,68 ± 122,81 | fuori, DD?-81/82 (era +287,62, ora +250,47) |
| passes_accurate | 1060,28 | 1092,35 | 739,17 ± 124,79 | fuori, DD?-81 (era +353,18, ora +321,11) |
| crosses | 7,13 | 8,40 | 33,83 ± 8,78 | non confrontabile; cala con i passaggi di ripresa |
| tutte le altre 19 righe | invariate | vedi 217 | — | 11 in banda, 9 fuori |

Il doppio racconto era una delle voci del vecchio OI-100 ("troppi passaggi"): la si chiude qui come errore di evento, non come taratura. Le otto domande aperte (DD?-81..86, 88..89) restano tali.

## La misura del 217 (2026-11-26, opencode)

Stesso comando e bande del 216. Un solo bump: il ramo jockey non registrava il `TakeOn` vinto dal portatore, mentre lo fa il ramo contrasto; ADR-086 chiede la stessa lettura per ogni atto. Nessun numero adattato (A5-bis). **11 su 20 confrontabili, invariato rispetto al 216.**

| metrica | 217 | 216 | reale | nota |
|---|---|---|---|---|
| goals | 4.00 | 4.00 | 2.68 ± 1.65 | in banda |
| own_goals | 0.20 | 0.23 | 0.10 ± 0.31 | in banda |
| shots | 26.77 | 26.53 | 24.60 ± 5.46 | in banda |
| shots_on_target | 8.12 | 7.97 | 8.69 ± 3.22 | in banda |
| penalties | 0.17 | 0.15 | 0.33 ± 0.56 | in banda |
| passes | 1175.30 | 1175.82 | 887.68 ± 122.81 | fuori, DD?-81/82 |
| passes_accurate | 1092.35 | 1091.70 | 739.17 ± 124.79 | fuori, DD?-81 |
| crosses | 8.40 | 8.25 | 33.83 ± 8.78 | non confrontabile |
| aerial_duels | 4.38 | 4.67 | 38.14 ± 12.98 | fuori, DD?-86 |
| ground_duels | 156.50 | 157.10 | 182.63 ± 27.54 | in banda |
| take_ons_won | 8.48 | 8.35 | 16.03 ± 5.67 | fuori, DD?-84 (definizione) |
| fouls | 20.65 | 21.53 | 26.00 ± 6.69 | in banda |
| yellow_cards | 2.38 | 2.37 | 3.97 ± 1.96 | in banda |
| red_cards | 0.20 | 0.18 | 0.24 ± 0.49 | in banda |
| corners | 9.23 | 9.43 | 10.30 ± 3.58 | in banda |
| throw_ins | 22.37 | 22.75 | 42.72 ± 8.92 | fuori, DD?-83 |
| goal_kicks | 11.07 | 10.65 | 17.15 ± 4.20 | fuori, DD?-85 |
| free_kicks | 21.00 | 21.75 | 29.45 ± 6.81 | fuori di 1,64 (era 0,89) |
| offsides | 1.57 | 1.55 | 4.26 ± 2.40 | fuori di 0,29 |
| clearances | 20.20 | 20.77 | 29.53 ± 8.65 | fuori di 0,68 (era 0,11) |
| interceptions | 47.87 | 49.00 | 89.86 ± 13.97 | non confrontabile |
| saves | 4.52 | 4.47 | 6.70 ± 2.82 | in banda con T11 |
| stoppage_first_half_min | 2.27 | 2.38 | ≥1.49 ± 1.35 | non confrontabile (T14-bis) |
| stoppage_second_half_min | 5.15 | 5.03 | ≥4.08 ± 1.61 | non confrontabile (T14-bis) |

Il tanto piccolo spostamento delle righe non è una taratura: nessun numero è stato scelto guardando la tabella, e le due righe che si allontanano (punizioni, spazzate) restano affidate ai DD? già aperti.

## La misura del 216 (2026-11-26, opencode)

Stesso comando (`cargo run --release -p ds-reference -- compare --reference L0-match/reference/serie-a-2017-18.json --seeds 42..101`), bande corrette da T11 (`saves`) e T14-bis (recupero: limite inferiore, non confrontabile). Un solo bump per tre decisioni: DD-8 (il calcio dietro la propria porta è un corner, non un atterraggio libero), DD-9 (l'attesa vale solo se l'uomo che rende disponibile l'opzione si muove verso di essa) e DD-10 (il rischio del corner si legge dove arriva il cross, alla linea dell'area, non come tiro dalla bandierina). DD-9 è emersa perché DD-8 ha tolto al portiere del seed 42 l'uscita facile e ha scoperto 343 tick fermi in `Hold`; DD-10 è emersa perché il valore alla bandierina rendeva il corner più sicuro di una rimessa a 50 m. Nessuna delle tre è stata tarata guardando la misura.

| metrica | 216 | 215 | reale | nota |
|---|---|---|---|---|
| goals | 4.00 | 7.28 | 2.68 ± 1.65 | in banda |
| own_goals | 0.23 | 0.52 | 0.10 ± 0.31 | in banda |
| shots | 26.53 | 34.70 | 24.60 ± 5.46 | in banda |
| shots_on_target | 7.97 | 12.08 | 8.69 ± 3.22 | in banda |
| penalties | 0.15 | 0.23 | 0.33 ± 0.56 | in banda |
| passes | 1175.82 | 1132.78 | 887.68 ± 122.81 | fuori, OI aperto (RUN 35/66) |
| passes_accurate | 1091.70 | 1043.28 | 739.17 ± 124.79 | fuori, stesso indizio |
| crosses | 8.25 | 16.58 | 33.83 ± 8.78 | non confrontabile |
| aerial_duels | 4.67 | 6.93 | 38.14 ± 12.98 | fuori, secondaria di ADR-081 (scelta C) |
| ground_duels | 157.10 | 161.62 | 182.63 ± 27.54 | in banda |
| take_ons_won | 8.35 | 8.37 | 16.03 ± 5.67 | fuori, definizione DD/ADR, non soglia da ritoccare |
| fouls | 21.53 | 22.10 | 26.00 ± 6.69 | in banda |
| yellow_cards | 2.37 | 2.52 | 3.97 ± 1.96 | in banda |
| red_cards | 0.18 | 0.32 | 0.24 ± 0.49 | in banda |
| corners | 9.43 | 27.72 | 10.30 ± 3.58 | in banda: DD-8/DD-10 chiudono l'indizio del 215 |
| throw_ins | 22.75 | 19.18 | 42.72 ± 8.92 | fuori, invariato nella causa aperta |
| goal_kicks | 10.65 | 10.70 | 17.15 ± 4.20 | fuori |
| free_kicks | 21.75 | 22.55 | 29.45 ± 6.81 | fuori di 0,89 |
| offsides | 1.55 | 1.93 | 4.26 ± 2.40 | fuori di 0,31 |
| clearances | 20.77 | 42.35 | 29.53 ± 8.65 | fuori di 0,11 |
| interceptions | 49.00 | 51.37 | 89.86 ± 13.97 | non confrontabile |
| saves | 4.47 | 5.60 | 6.70 ± 2.82 | in banda con T11 |
| stoppage_first_half_min | 2.38 | 4.27 | ≥1.49 ± 1.35 | non confrontabile (T14-bis) |
| stoppage_second_half_min | 5.03 | 6.53 | ≥4.08 ± 1.61 | non confrontabile (T14-bis) |

**In banda con le bande corrette: 11 su 20 confrontabili** (9 fuori: passaggi, passaggi accurati, duelli aerei, take-on vinti, rimesse, calci di porta, calci liberi, offside, spazzate; 4 righe non confrontabili). Rispetto al 215 sono entrate in banda gol, autogol, tiri e tiri in porta; angoli e falli erano già dentro. Sono uscite le offside (1,93 → 1,55, fuori di 0,31) e le spazzate (42,35 → 20,77, fuori di 0,11). Golden base, instruito e coppa rigenerati; semi coppa 59 → 43, pareggio al 90′ 56 → 42, vittoria al 90′ 42 → 45, rigori solo tiri 59 → 43, colpo di testa in fuorigioco 97 → 50; golden della piccola stagione L1 cambiato. Il seed della finestra GUI è passato da 42 a 43 perché il 42 non produce più fuorigioco con le scelte predefinite.

## La misura del 215 (2026-11-26, opencode)

Stesso comando (`cargo run --release -p ds-reference -- compare --reference L0-match/reference/serie-a-2017-18.json --seeds 42..101`), bande corrette da T11 (`saves`) e T14-bis (recupero non confrontabile). Un solo bump per sei decisioni applicate insieme: DD 1 (la scansione staggered è un pavimento della latenza, non un termine additivo), DD 2 (lo spazio del ricevitore gradua il guadagno del passaggio e del lancio), DD 3 (la linea del fuorigioco va letta, non solo proiettata), DD 4 (nessun emendamento ad ADR-081: la scelta C resta, il residuo è a monte), DD 5 (il difensore può liberare anche oltre la propria linea di porta) e DD-6 (l'attesa esiste solo se la scena immaginata cambia). Le sei non sono state misurate una per una: la riga di una metrica non va attribuita a una singola DD senza una bisecazione delle misure.

| metrica | 215 | 214 | reale | nota |
|---|---|---|---|---|
| goals | 7.28 | 4.07 | 2.68 ± 1.65 | fuori e quasi raddoppiato: da indagare come possibile bug, non da tarare |
| own_goals | 0.52 | 0.27 | 0.10 ± 0.31 | fuori e raddoppiato: da indagare come possibile bug |
| shots | 34.70 | 27.60 | 24.60 ± 5.46 | fuori |
| shots_on_target | 12.08 | 9.35 | 8.69 ± 3.22 | fuori di 0,17 |
| penalties | 0.23 | 0.22 | 0.33 ± 0.56 | in banda |
| passes | 1132.78 | 1524.25 | 887.68 ± 122.81 | fuori, ma 391 in meno: DD 2 ha ridotto il guadagno immaginato, la causa del residuo resta aperta (RUN 35/66) |
| passes_accurate | 1043.28 | 1383.45 | 739.17 ± 124.79 | fuori, stesso indizio dei passaggi |
| crosses | 16.58 | 7.50 | 33.83 ± 8.78 | non confrontabile |
| aerial_duels | 6.93 | 5.93 | 38.14 ± 12.98 | fuori, secondaria del DD 4: DD 5 non l'ha mossa; la scelta C di ADR-081 resta |
| ground_duels | 161.62 | 201.03 | 182.63 ± 27.54 | in banda |
| take_ons_won | 8.37 | 8.80 | 16.03 ± 5.67 | fuori, definizione DD/ADR, non soglia da ritoccare |
| fouls | 22.10 | 26.92 | 26.00 ± 6.69 | in banda |
| yellow_cards | 2.52 | 2.95 | 3.97 ± 1.96 | in banda |
| red_cards | 0.32 | 0.30 | 0.24 ± 0.49 | in banda |
| corners | 27.72 | 6.52 | 10.30 ± 3.58 | fuori, peggiorato dal 214: DD 5 ha aggiunto l'uscita dietro la porta ma la misura non è migliorata; da indagare prima come bug |
| throw_ins | 19.18 | 19.37 | 42.72 ± 8.92 | fuori, invariato: la graduazione di DD 2 non ha agito sul gioco largo come sperato |
| goal_kicks | 10.70 | 13.40 | 17.15 ± 4.20 | fuori |
| free_kicks | 22.55 | 32.72 | 29.45 ± 6.81 | fuori di 0,09 |
| offsides | 1.93 | 7.63 | 4.26 ± 2.40 | in banda: il gate di percezione di DD 3 ha chiuso l'indizio del 214 |
| clearances | 42.35 | 23.93 | 29.53 ± 8.65 | fuori |
| interceptions | 51.37 | 93.97 | 89.86 ± 13.97 | non confrontabile |
| saves | 5.60 | 5.72 | 6.70 ± 2.82 | in banda con T11 |
| stoppage_first_half_min | 4.27 | 2.90 | ≥1.49 ± 1.35 | non confrontabile (T14-bis) |
| stoppage_second_half_min | 6.53 | 4.91 | ≥4.08 ± 1.61 | non confrontabile (T14-bis) |

**In banda con le bande corrette: 7 su 20 confrontabili** (13 fuori: gol, autogol, tiri, tiri in porta, passaggi, passaggi accurati, duelli aerei, take-on vinti, angoli, rimesse, calci di porta, calci liberi, spazzate); 4 righe non confrontabili (cross, intercettazioni, due recuperi). Rispetto al 214 sono entrate in banda solo le offside; sono uscite gol, autogol, tiri, tiri in porta, calci di porta, calci liberi e spazzate. Nessun numero è stato scelto guardando la misura (A5-bis); ogni riga fuori banda resta un indizio da indagare come possibile bug prima di ogni taratura. Golden e semi rigenerati: base, instruito e coppa cambiati; semi coppa 57 → 59, pareggio al 90′ 50 → 56, rigori solo tiri 57 → 59, colpo di testa in fuorigioco 45 → 97; golden della piccola stagione L1 cambiato.

## La misura del 214 (2026-11-26, opencode)

Stesso comando (`cargo run --release -p ds-reference -- compare --reference L0-match/reference/serie-a-2017-18.json --seeds 42..101`), bande corrette da T11 (`saves`) e T14-bis (recupero: limite inferiore, non confrontabile). Due fix indipendenti: il primo tocco riconosce la propria linea dall'identità del ricevitore, e un secondo uomo battuto non cancella il primo take-on.

| metrica | 214 | reale | nota |
|---|---|---|---|
| goals | 4.07 | 2.68 ± 1.65 | in banda |
| own_goals | 0.27 | 0.10 ± 0.31 | in banda, invariato dal 213: l'attribuzione non cambia il conteggio in questi 60 semi |
| shots | 27.60 | 24.60 ± 5.46 | in banda |
| shots_on_target | 9.35 | 8.69 ± 3.22 | in banda |
| penalties | 0.22 | 0.33 ± 0.56 | in banda |
| passes | 1524.25 | 887.68 ± 122.81 | fuori, OI aperto |
| passes_accurate | 1383.45 | 739.17 ± 124.79 | fuori, OI aperto |
| crosses | 7.50 | 33.83 ± 8.78 | non confrontabile |
| aerial_duels | 5.93 | 38.14 ± 12.98 | fuori, OI-102 aperto |
| ground_duels | 201.03 | 182.63 ± 27.54 | in banda |
| take_ons_won | 8.80 | 16.03 ± 5.67 | fuori: 8.72 → 8.80, il guard chiude la perdita del primo dribble ma non chiude la banda (definizione da DD) |
| fouls | 26.92 | 26.00 ± 6.69 | in banda |
| yellow_cards | 2.95 | 3.97 ± 1.96 | in banda |
| red_cards | 0.30 | 0.24 ± 0.49 | in banda |
| corners | 6.52 | 10.30 ± 3.58 | fuori, invariato dal 213 |
| throw_ins | 19.37 | 42.72 ± 8.92 | fuori, OI aperto |
| goal_kicks | 13.40 | 17.15 ± 4.20 | in banda |
| free_kicks | 32.72 | 29.45 ± 6.81 | in banda |
| offsides | 7.63 | 4.26 ± 2.40 | fuori, OI aperto |
| clearances | 23.93 | 29.53 ± 8.65 | in banda |
| interceptions | 93.97 | 89.86 ± 13.97 | non confrontabile |
| saves | 5.72 | 6.70 ± 2.82 | in banda con T11 (prima 9.37 ± 3.37, banda gonfiata dal tag GOAL) |
| stoppage_first_half_min | 2.90 | ≥1.49 ± 1.35 | non confrontabile (T14-bis): reale = ultimo evento Wyscout, motore = fischio |
| stoppage_second_half_min | 4.91 | ≥4.08 ± 1.61 | non confrontabile (T14-bis): idem |

**In banda con le bande corrette: 13 su 20 confrontabili** (7 fuori: passaggi, passaggi accurati, duelli aerei, take-on vinti, angoli, rimesse, fuorigioco); 4 righe non confrontabili (cross, intercettazioni, due recuperi). Il 14 su 22 del 213 non è uno scenario comparabile: T11 ha cambiato la banda delle parate e T14-bis ha tolto dal confronto le due righe di recupero. Nessun numero è stato scelto guardando la misura. Il solo golden cambiato è `EXPECTED_INSTRUCTED`; golden base, coppa, semi (coppa 57, pari al 90′ 50, rigori 57, colpo di testa in fuorigioco 45) e golden L1 sono bit-identici al 213 (`cargo test --release --workspace` e `cup_seed_scan` verdi).

## La misura del 213 (2026-11-25, DD-76)

Stesso comando, stessa banda; confronto con la misura del 212.

| metrica | 213 | reale | nota |
|---|---|---|---|
| goals | 4.07 | 2.68 ± 1.65 | in banda |
| own_goals | 0.27 | 0.10 ± 0.31 | in banda |
| shots | 27.60 | 24.60 ± 5.46 | in banda |
| shots_on_target | 9.35 | 8.69 ± 3.22 | in banda |
| penalties | 0.22 | 0.33 ± 0.56 | in banda |
| passes | 1524.25 | 887.68 ± 122.81 | fuori, OI aperto |
| passes_accurate | 1383.45 | 739.17 ± 124.79 | fuori, OI aperto |
| crosses | 7.50 | 33.83 ± 8.78 | non confrontabile |
| aerial_duels | 5.93 | 38.14 ± 12.98 | fuori, OI-102 aperto |
| ground_duels | 201.03 | 182.63 ± 27.54 | in banda |
| take_ons_won | 8.72 | 16.03 ± 5.67 | fuori, OI aperto |
| fouls | 26.92 | 26.00 ± 6.69 | in banda |
| yellow_cards | 2.95 | 3.97 ± 1.96 | in banda |
| red_cards | 0.30 | 0.24 ± 0.49 | in banda |
| corners | 6.52 | 10.30 ± 3.58 | fuori, ma 18.02 → 6.52: la revoca ha chiuso l'indizio del 210, ora mancano angoli |
| throw_ins | 19.37 | 42.72 ± 8.92 | fuori, OI aperto |
| goal_kicks | 13.40 | 17.15 ± 4.20 | in banda |
| free_kicks | 32.72 | 29.45 ± 6.81 | in banda |
| offsides | 7.63 | 4.26 ± 2.40 | fuori, OI aperto |
| clearances | 23.93 | 29.53 ± 8.65 | in banda |
| interceptions | 93.97 | 89.86 ± 13.97 | non confrontabile |
| saves | 5.72 | 6.70 ± 2.82 | in banda dopo T11: il tag GOAL sui tentativi del portiere non è una parata |
| stoppage_first_half_min | 2.90 | ≥1.49 ± 1.35 | non confrontabile (T14-bis): reale = ultimo evento Wyscout, motore = fischio |
| stoppage_second_half_min | 4.91 | ≥4.08 ± 1.61 | non confrontabile (T14-bis): idem |

**In banda: 213 → 14 su 22** (212 era 9 su 22). La revoca di DD-73 ha portato gli angoli da 18.02 a 6.52. Con T11 `saves` rientra (5.72 contro 6.70 ± 2.82); con T14-bis le due righe di recupero non sono più confrontabili (banda reale = limite inferiore dell'ultimo evento Wyscout, motore = fischio) e OI-101 resta aperto come modello dell'arbitro, non come scarto di banda. Ogni metrica fuori banda resta un indizio con la sua causa, non una soglia ritoccata.

**Causa:** non è una taratura ma una revoca di DD-73. Il ramo loftato era stato aggiunto al lookahead di OI-116 benché ADR-101 §Conseguenze lo escluda, e il guadagno immaginato comprendeva una giocata successiva che può non avvenire: `lofted_missed_at` vede solo la propria porta, non l'uscita dal campo. Il ricevitore conserva tutte le sue opzioni; cambia solo il guadagno che il passatore immagina. Il guard `a_long_ball_does_not_read_past_the_landing` chiude il caso con scena identica.

## La misura del 212 (2026-11-25, DD-75)

Stesso comando, stessa banda; confronto con la misura del 211.

| metrica | 211 | 212 | reale | nota |
|---|---|---|---|---|
| stoppage_first_half_min | 3.45 | 3.17 | ≥1.49 ± 1.35 | non confrontabile: reale = ultimo evento Wyscout, motore = fischio |
| stoppage_second_half_min | 5.92 | 5.81 | ≥4.08 ± 1.61 | non confrontabile: idem |
| goals | 5.23 | 5.38 | 2.68 ± 1.65 | resta fuori |
| corners | 18.08 | 18.02 | 10.30 ± 3.58 | resta fuori, indizio aperto |
| saves | 5.08 | 5.27 | 9.37 ± 3.37 | resta fuori |

**In banda: 212 → 9 su 22**, come al 211 (10 dopo T11: la banda `saves` era gonfiata dal tag GOAL sui tentativi del portiere, non dal motore). La causa è una regola (Law 7.3), non una taratura; il guard `the_knock_and_his_change_are_one_stoppage` chiude il doppio conteggio. Le due righe di recupero non sono più lette come scarto di banda: la banda reale è il limite inferiore dell'ultimo evento Wyscout (1.488/4.083 min su 380 partite), il motore misura il fischio; OI-101 resta aperto come modello del comportamento dell'arbitro, non come differenza tra due misure dello stesso fatto.

## La misura del 211 (2026-11-25, DD-74)

Stesso comando, stessa banda; il confronto è con la misura del 209 (`target/opencode/run23-g-base209.txt`), perché il 210 non era stato misurato a sé prima della correzione del rimbalzo.

| metrica | 209 | 211 | reale | nota |
|---|---|---|---|---|
| shots | 38.73 | 30.50 | 24.60 ± 5.46 | appena fuori (30.06 il limite) |
| throw_ins | 17.77 | 33.32 | 42.72 ± 8.92 | appena fuori (33.80 il limite) |
| corners | 7.73 | 18.08 | 10.30 ± 3.58 | l'indizio degli angoli resta aperto |
| saves | 6.75 | 5.08 | 9.37 ± 3.37 | esce dalla banda (6.00 il limite) |
| goals | 4.87 | 5.23 | 2.68 ± 1.65 | resta fuori |
| aerial_duels | 6.82 | 8.97 | 38.14 ± 12.98 | resta fuori |

**In banda: 211 → 9 su 22** (le due metriche che il riferimento dichiara non confrontabili, cross e intercetti, restano escluse). La causa di SIM 211 non è una taratura ma una regola: il blocco dà il possesso ai difensori; il guard sulle registrazioni (`the_shooter_waits_after_a_block`) chiude il doppio possesso. L'indizio degli angoli e delle parate resta per il passo successivo.

## La misura del 210 (2026-11-25, DD-71)

Lo stesso comando e la stessa banda; la misura è quella presa sopra il 207 (HANDOFF §4 A0, opencode RUN 17), sul motore 210.

| metrica | 207 | 210 | reale | causa | bug o taratura |
|---|---|---|---|---|---|
| corners | 7.23 | 18.97 | 10.31 ± 3.58 | il lancio guarda un passo avanti: più palloni alti finiscono sul fondo; chi li manda e chi li tocca per ultimo è l'indizio aperto (RUN 17: `receiver_best_pass` conta anche i lanci del ricevitore) | bug a valle, da indagare |
| goals | 4.98 | 6.93 | 2.68 ± 1.65 | ogni pallone alto in area diventa un tiro (T29 chiuso, resta la scelta) | resta fuori |
| own_goals | 0.23 | 0.47 | 0.10 ± 0.31 | più palloni alti in area propria | resta fuori |
| shots | 42.7 | 49.4 | 24.60 ± 5.46 | come sopra | resta fuori |
| aerial_duels | 6.6 | 11.0 | 38.14 ± 12.98 | più palloni alti giocati, ma la contesa resta nel tick | resta fuori |
| throw_ins | 19.1 | 33.4 | 42.72 ± 8.92 | più palloni che escono sul fondo | ai limiti della banda |

**In banda: 210 → 10 su 22** (12 al 207). Nessun numero scelto guardando la metrica: il lookahead del lancio usa la stessa misura del passaggio a terra (ADR-052, OI-116 A). L'indizio successivo è l'angolo: troppi palloni alti finiscono fuori, e la catena va cercata nelle registrazioni prima di toccare il motore.

## La misura del 207 (2026-11-25)

Nella propria area il primo controllo va di lato (T9, opencode RUN 10).

| metrica | 206 | 207 | reale | causa | bug o taratura |
|---|---|---|---|---|---|
| own_goals | 0.90 | 0.23 | 0.10 ± 0.31 | il pallone non smorzato correva verso la propria linea (corretto) | bug, corretto: in banda |
| goals | 5.48 | 4.98 | 2.68 ± 1.65 | una parte erano autogol; il resto: tiri ×1.7 (T30, OI-100) | resta fuori |
| corners | 10.88 | 7.23 | 10.30 ± 3.58 | meno palloni giocati sul fondo dai propri difensori | in banda |

**In banda: 207 → 12 su 22** (contate come al 206, intercettazioni comprese: 11 al 206).

## La misura del 206 (2026-11-25)

Il dribbling è superare l'uomo (opencode RUN 15). Su 42..101 cambia solo il conteggio dei dribbling riusciti, ogni altra metrica è identica alla cifra.

| metrica | 205 | 206 | reale | causa | bug o taratura |
|---|---|---|---|---|---|
| take_ons_won | 25.98 | 9.73 | 16.03 ± 5.67 | contato al contatto, non al superamento (corretto); resta quante volte si prova a saltare l'uomo | bug, corretto; il resto da indagare |

**In banda: 206 → 10 su 22**, come il 205.

## La misura del 205 (2026-11-24, DD-68)

Stesso comando e banda. **10 su 22** (shots_on_target, penalties, fouls, yellow_cards, red_cards, corners, goal_kicks, free_kicks, clearances, saves); gol 5.48, autogol 0.9, tiri 42.92, in porta 11.13, passaggi 1628.97, duelli aerei 7.92, duelli a terra 210.83, dribbling 25.98, spazzate 22.77, rimesse 17.82, fuorigioco 9.25, recupero 7.22/6.75. Nessun numero adattato. I duelli aerei 6.15 → 7.92: la contesa nel tick c'è, ma i palloni alti giocati sono pochi (i cross 8.7 contro 33.8 reali, la metrica che `ds-reference` non confronta): **indizio nuovo, la scelta del pallone alto** (lancio, cross) va cercata come bug nel pensiero di chi passa.

## La misura del 204 (2026-11-24, DD-65)

Stesso comando e banda. **10 su 22** (shots_on_target, penalties, fouls, yellow_cards, red_cards, corners, goal_kicks, free_kicks, clearances, saves); gol 5.43, autogol 0.83, tiri 42.37, in porta 11.32, passaggi 1626.17, duelli aerei 6.15, duelli a terra 213.62, dribbling 27.23, spazzate 22.5, rimesse 17.73, fuorigioco 9.2, recupero 7.17/6.74. Nessun numero adattato. Rientrano i tiri in porta; le spazzate scendono da 36.5 a 22.5 (in banda). Restano fuori, con le cause della misura del 203: gol, autogol, tiri, passaggi, duelli aerei (la contesa solo nello stesso piccolo passo), duelli a terra e dribbling, rimesse, fuorigioco, recupero (taratura dichiarata). Nuovo sintomo dei gol in eccesso: in coppa i rigori arrivano una partita su circa 150 (il calcio vero circa una su otto).

## La misura del 203 (2026-11-24, DD-63)

Stesso comando e banda. **9 su 22** (penalties, fouls, yellow_cards, red_cards, corners, goal_kicks, free_kicks, clearances, saves); gol 5.5, autogol 0.78, tiri 45.02, in porta 11.92, passaggi 1583.57, duelli aerei 6.08, duelli a terra 214.63, dribbling 26.53, falli 27.27, gialli 3.1, rossi 0.45, angoli 8.67, rimesse 18.37, punizioni 34.9, fuorigioco 9.77, spazzate 36.53, parate 7.47, recupero 7.16/6.83. Nessun numero adattato. Le parate restano "in banda" con la riserva di T11 (la banda reale conta anche i tentativi sui gol).

| Metrica | Motore 203 | Fuori banda | Causa | Bug o taratura |
|---|---|---|---|---|
| gol, tiri, in porta | 5.5, 45.02, 11.92 | sopra | T29 (ogni pallone alto vinto in area avversaria è un tiro), T30 (la scelta di prima su una scena più povera) | bug aperti |
| autogol | 0.78 | sopra (×7) | dopo T1-T3 resta da cercare: controllo fallito e conduzione verso la propria porta (T9) | bug da indagare |
| passaggi, riusciti | 1583.57, 1434.38 | sopra (×1.8) | T17 (le riprese giocate come palla in movimento), T12 (la misura conta i passaggi delle riprese) | bug aperti |
| duelli aerei | 6.08 | sotto (×0.14) | T29 (la spazzata di testa imposta, anche sul passaggio del compagno), il duello solo nello stesso piccolo passo | bug aperti |
| duelli a terra, dribbling | 214.63, 26.53 | sopra | dal 203 (T32) il portatore resta sulla palla il suo tempo di decisione e subisce più contrasti | da indagare |
| rimesse laterali | 18.37 | sotto (×0.46) | i corpi fermi dentro il campo, il rinvio di testa sempre dritto | bug da indagare |
| fuorigioco | 9.77 | sopra (×2.5) | T23 (le punte del blocco sulla linea per costruzione) | bug aperto |
| recupero | 7.16/6.83 | sopra | T15 corretto; resta il modello: il motore recupera tutto il tempo perso, l'arbitro circa metà (OI-101) | taratura dichiarata |

## La misura del 202 (2026-11-24, DD-62)

Stesso comando e banda. **6 su 22** (penalties, ground_duels, fouls, red_cards, goal_kicks, saves); gol 7.38, autogol 1.03, tiri 43.45, in porta 20.3, passaggi 1832.7, falli 29.87, gialli 1.37, rossi 0.53, parate 11.62, fuorigioco 15.38, recupero 8.86/8.48. Nessun numero adattato. Rispetto al 201 la porta ha la misura della palla (pali e traversa a un raggio fuori, Legge 1): gol 7.78 → 7.38, parate 13.3 → 11.62 (di nuovo in banda). I controlli delle Leggi (`laws_hold`, 100 partite) sono puliti.

Ogni riga fuori banda è un indizio da seguire negli eventi delle partite più lontane (Terza fase, punto 3). Dove la colonna dice "da indagare", la catena causale non è ancora stata cercata su questo motore: la causa scritta è l'ipotesi o l'OI che la copre, non una conclusione.

| Metrica | Reale ± | Motore 202 | Fuori banda | Causa | Bug o taratura |
|---|---|---|---|---|---|
| gol | 2.68 ± 1.65 | 7.38 | sopra (×2.8) | la catena dei troppi tiri e il tiro troppo preciso (OI-100) | da indagare |
| autogol | 0.10 ± 0.31 | 1.03 | sopra (×10) | la spazzata sotto pressione e le deviazioni verso la propria porta (OI-100) | audit 2026-11-24: bug (a) `check_goal_line` non chiama `keeper_meets_the_shot` per palle non-tiro; bug (b) `play_clearance` senza `played_by` attribuisce male ultimo tocco. Fix (a) candidato principale; occhio a `saves` limite alto |
| tiri | 24.6 ± 5.46 | 43.45 | sopra | troppi tiri dalla catena dei passaggi e delle ripartenze (OI-100) | da indagare |
| tiri in porta | 8.69 ± 3.22 | 20.3 | sopra | segue i tiri; quota nello specchio 47% (reale 35%) | da indagare |
| passaggi | 887.7 ± 122.8 | 1832.7 | sopra (×2.1) | troppi passaggi e troppi intercettati (OI-100) | da indagare |
| passaggi riusciti | 739.2 ± 124.8 | 1512.2 | sopra | segue i passaggi (quota 82.5%, reale 83.3%) | segue i passaggi |
| duelli aerei | 38.14 ± 12.98 | 13.2 | sotto | pochi palloni alti contesi (OI-102, causa chiusa, divario aperto) | da indagare |
| dribbling riusciti | 16.03 ± 5.67 | 22.02 | sopra, al bordo | il portatore punta l'uomo troppo spesso (OI-103) | da indagare |
| gialli | 3.97 ± 1.96 | 1.37 | sotto | il fallo tattico che ferma un'azione promettente non è ammonito (Legge 12) | da indagare, candidato bug |
| angoli | 10.3 ± 3.58 | 15.77 | sopra | la catena dei troppi tiri (OI-100) | da indagare |
| rimesse laterali | 42.72 ± 8.92 | 13.83 | sotto (×0.3) | duelli e passaggi sbagliati che non mandano la palla fuori; dal 200 fuori solo a palla intera | da indagare |
| punizioni | 29.45 ± 6.81 | 42.87 | sopra | falli + fuorigioco: i 15 fuorigioco diventano punizioni (OI-112) | segue i fuorigioco |
| fuorigioco | 4.26 ± 2.40 | 15.38 | sopra (×3.6) | la linea letta senza l'errore di percezione (OI-098/104) | da indagare |
| spazzate | 29.53 ± 8.65 | 46.63 | sopra | la spazzata di testa resta una regola (ADR-064, passo 2) | da indagare |
| recupero, primo tempo | 1.49 ± 1.35 | 8.86 | sopra | il motore aggiunge tutto il tempo perso, gli arbitri circa metà (OI-101) | da indagare |
| recupero, secondo tempo | 4.08 ± 1.61 | 8.48 | sopra | come sopra (OI-101) | da indagare |

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
