# Fonti statistiche Serie A aggiornate: dati event-level e aggregati (al 27 novembre 2026)

Verifica eseguita il 27/11/2026. Etichette: **[VERIFICATO]** = letto direttamente sulla fonte citata (o, dove indicato, solo nello snippet di ricerca); **[DEDUZIONE]** = inferenza mia, non affermazione della fonte. Lo strumento di web search interno ha risposto HTTP 402, quindi le ricerche sono state fatte con DuckDuckGo Lite e le pagine sono state verificate con fetch diretto. Fetch non riusciti sono segnalati esplicitamente.

## 1. Esiste una fonte event-level (posizioni, tag, timestamp) per Serie A 2026-27 con licenza accettabile per un gioco?

### Takeaway
**[VERIFICATO]** Non ho trovato nessuna fonte event-level *aperta* per la Serie A 2026-27. Le fonti event-level con coordinate e tag (Wyscout, Opta/Stats Perform, StatsBomb) sono prodotti commerciali con licenza da negoziare; lo StatsBomb open data è fermo alle stagioni 2023/24 e non contiene Serie A maschile recente; Understat non è raggiungibile il 27/11/2026.

### Cited Findings
- **[VERIFICATO]** StatsBomb open data distribuisce JSON esportati dalla StatsBomb Data API: `competitions.json`, `matches/`, `events/`, `lineups/`, `three-sixty/` e documentazione in `doc/` — [Source](https://github.com/hudl/open-data)
- **[VERIFICATO]** In `competitions.json` la Serie A maschile (competition_id 12) ha solo `2015/2016` (season_id 27) e `1986/1987` (season_id 86); la Serie A Women ha `2023/2024`; la stagione più recente presente è `2023/2024` — [Source](https://raw.githubusercontent.com/statsbomb/open-data/master/data/competitions.json)
- **[VERIFICATO]** L'ultimo commit del repo open data è del 2026-09-07, quindi il dataset è manutenuto ma non contiene la stagione 2026-27 — [Source](https://api.github.com/repos/hudl/open-data/commits/master)
- **[VERIFICATO]** I termini di StatsBomb open data chiedono di citare la fonte e usare il logo se si pubblicano research/analisi basate sui dati; esiste un `LICENSE.pdf` nel repo — [Source](https://github.com/hudl/open-data)
- **[VERIFICATO]** Wyscout pubblica un glossary che descrive "the current state of the Wyscout data model, used across the Wyscout Platform, reports and the API", con eventi, tag API e metriche — [Source](https://dataglossary.wyscout.com/)
- **[VERIFICATO]** La documentazione prodotto Wyscout (mirror GitHub del README ufficiale, che rimanda a `footballdata.wyscout.com/resources/`) definisce le coordinate evento come **(x,y) in percentuale**, con x=0% porta da difendere e x=100% porta da attaccare — [Source](https://raw.githubusercontent.com/mbochicchio/wyscout-openapi/main/README.md)
- **[VERIFICATO]** La documentazione API ufficiale è `https://apidocs.wyscout.com/` (pagina JS, testo minimo via fetch) e la pagina di supporto Hudl indica che si può scaricare la specifica OpenAPI — [Source](https://apidocs.wyscout.com/); [Source](https://support.hudl.com/s/article/wyscout-api?topic=Scouting_and_Transfer)
- **[VERIFICATO]** Mirror non ufficiale ma dichiaratamente estratto dalle OpenAPI di apidocs.wyscout.com: v3 ha 46 endpoint, v4 50; l'endpoint eventi è `GET /matches/{wyId}/events` con `details=tag` e `exclude=...positions...`; v4 aggiunge tracking, physical e direction data — [Source](https://raw.githubusercontent.com/withqwerty/football-docs/main/docs/wyscout/api-endpoints.md)
- **[VERIFICATO]** Wyscout è un prodotto Hudl a pagamento ("world's biggest football library", 1000+ competizioni); i termini d'uso sono un contratto di subscription — [Source](https://www.hudl.com/en_gb/products/wyscout); [Source](https://static.hudl.com/craft/legal/Wyscout-Platform-Individual-Terms-and-Conditions_2023-08-21.pdf?mtime=20230821115836)
- **[VERIFICATO solo via snippet]** Opta/Stats Perform pubblica "Opta Event Definitions" e dichiara definizioni globalmente consistenti; la pagina è protetta (403 al fetch) — [Source](https://www.statsperform.com/opta-event-definitions/)
- **[VERIFICATO]** Understat non è utilizzabile: `/League/Serie-A/2025` e la home rispondono con la pagina "We will come back soon" — [Source](https://understat.com/League/Serie-A/2025); [Source](https://understat.com/)
- **[VERIFICATO]** `openfootball/football.json` ha una cartella `2026-27`, ma `it.1.json` contiene solo `round`, `date`, `time`, `team1`, `team2`, `score.ht`, `score.ft` — nessun evento, coordinata o statistica — [Source](https://raw.githubusercontent.com/openfootball/football.json/master/2026-27/it.1.json); licenza CC0 1.0 — [Source](https://raw.githubusercontent.com/openfootball/football.json/master/LICENSE.md)

### Inferences
- **[DEDUZIONE]** L'unica opzione che manterrebbe *identico* lo schema del dump attuale (eventId/subEventId/tag/positions) è Wyscout API con licenza: gli endpoint e le convenzioni di coordinate sono documentati e coincidono con quelle del referto storico. Opta e StatsBomb commerciale darebbero event-level, ma con schema diverso e quindi con converter.
- **[DEDUZIONE]** "Licenza accettabile per uso in un gioco" non è dimostrabile per le fonti commerciali senza leggere il contratto: la documentazione pubblica parla di subscription/termini, non di diritto di redistribuzione in un prodotto.
- **[DEDUZIONE]** Le fonti gratuite e aperte verificabili oggi (StatsBomb open, openfootball) non possono aggiornare il confronto alla stagione 2026-27.

### Gaps
- Quali eventi/tag Wyscout siano effettivamente disponibili per Serie A 2026-27 (serve account/API key); nessuna fonte pubblica verificata.
- Se Opta o StatsBomb commerciale coprano Serie A 2026-27 e a quali condizioni di licenza per un videogioco.
- Ultima stagione coperta da Understat: il sito è down, quindi non verificabile; non assume alcuna data.
- API-Football e Sofascore non sono riusciti via fetch (403); la documentazione degli endpoint è nota solo via snippet di ricerca.

## 2. Quali fonti danno solo statistiche aggregate e quali danno eventi con coordinate? Quanto sono aggiornate?

### Takeaway
**[VERIFICATO]** Event-level con coordinate: Wyscout, Opta, StatsBomb (tutti commerciali per il presente). Agregati: Sportmonks, API-Football, Sofascore, FBref, football-data.co.uk. **football-data.co.uk ha un CSV gratuito con 120 partite di Serie A 2026-27 fino al 23/11/2026**, ma solo 7 delle 22 metriche del gioco.

### Cited Findings
| Fonte | Livello | Coordinate | Aggiornamento verificato | Licenza | Fonte |
|---|---|---|---|---|---|
| Wyscout (Hudl) | event-level + tag | sì, (x,y) % | API v3/v4 correnti; copertura 1000+ competizioni, 2026-27 non verificabile senza account | commerciale | [Source](https://apidocs.wyscout.com/); [Source](https://raw.githubusercontent.com/mbochicchio/wyscout-openapi/main/README.md); [Source](https://www.hudl.com/en_gb/products/wyscout) |
| Opta / Stats Perform | event-level | documentazione eventi dedicata | prodotto corrente, 3900+ competizioni (solo snippet) | commerciale | [Source](https://www.statsperform.com/products/opta-data/); [Source](https://www.statsperform.com/opta-event-definitions/) |
| StatsBomb open data | event-level + 360 | sì | max 2023/24; Serie A maschile solo 2015/16 e 1986/87 | aperta con attribuzione | [Source](https://github.com/hudl/open-data); [Source](https://raw.githubusercontent.com/statsbomb/open-data/master/data/competitions.json) |
| StatsBomb commerciale | event-level | sì | prodotto attivo; stagione 2026-27 non verificata | commerciale | [Source](https://www.statsbomb.com/football-data/) |
| Sportmonks API 3.0 | aggregati + timeline eventi | **no** sugli eventi: "No location data on events"; `ballCoordinates` è posizione palla non attribuita a un'azione | API corrente; statistiche match/team/player in tutti i piani, xG/Pressure Index add-on | commerciale | [Source](https://docs.sportmonks.com/v3/tutorials-and-guides/tutorials/includes/events); [Source](https://docs.sportmonks.com/v3/tutorials-and-guides/tutorials/statistics/fixture-statistics) |
| API-Football | aggregati + endpoint `events` | non verificato | documentazione elenca Fixtures, Events, statistics; fetch 403 | commerciale | [Source](https://www.api-football.com/documentation-v3) |
| Sofascore | aggregati (classifica, risultati, statistiche, capocannonieri) | non documentato | pagina Serie A **2026/2027** verificata | sito/API non ufficialmente documentata | [Source](https://www.sofascore.com/football/tournament/italy/serie-a/23) |
| FBref | aggregati | no event-level | pagina 2025-2026 esiste (fetch 403 Cloudflare); metriche avanzate rimosse nel gennaio 2026 | gratuito ma senza feed Opta avanzato | [Source](https://fbref.com/en/comps/11/2025-2026/2025-2026-Serie-A-M-Stats); [Source](https://www.sports-reference.com/blog/2026/01/fbref-stathead-data-update/) |
| football-data.co.uk | CSV aggregato per partita | no | `mmz4281/2627/I1.csv`: 120 righe, 40 date, dal 22/08/2026 al **23/11/2026**, tutte con statistiche compilate | gratuito | [Source](https://www.football-data.co.uk/mmz4281/2627/I1.csv); [Source](https://www.football-data.co.uk/) |
| openfootball/football.json | risultati | no | cartella 2026-27 presente | CC0 | [Source](https://raw.githubusercontent.com/openfootball/football.json/master/2026-27/it.1.json) |
| Understat | tiri/xG con coordinate (noto storicamente) | sì per i tiri | sito down il 27/11/2026 | gratuito, non documentato | [Source](https://understat.com/) |

- **[VERIFICATO]** Le colonne del CSV football-data.co.uk sono: `FTHG, FTAG, HTHG, HTAG, HxG, AxG, HS, AS, HST, AST, HF, AF, HC, AC, HY, AY, HR, AR` (più quote scommessa): copre gol, tiri, tiri in porta, falli, angoli, gialli, rossi e xG — [Source](https://www.football-data.co.uk/mmz4281/2627/I1.csv)
- **[VERIFICATO solo via snippet]** Sports-Reference ha annunciato che il provider dei dati avanzati di calcio ha terminato l'accesso e ha richiesto la cancellazione dei dati da FBref nel gennaio 2026; il New York Times Athletic collega l'interruzione a Stats Perform/Opta — [Source](https://www.sports-reference.com/blog/2026/01/fbref-stathead-data-update/); [Source](https://www.nytimes.com/athletic/7002196/2026/01/28/fbref-opta-football-data-soccer-analytics/)

### Inferences
- **[DEDUZIONE]** Solo Wyscout/Opta/StatsBomb commerciali preservano eventi + coordinate; Sportmonks esclude esplicitamente coordinate attribuite al giocatore e API-Football/Sofascore non le documentano.
- **[DEDUZIONE]** Il CSV football-data.co.uk è la fonte corrente gratuita più immediata, ma copre 7/22 metriche: gol, tiri, tiri in porta, falli, gialli, rossi, angoli. Mancano autogol, passaggi/accuratezza, cross, duelli, dribbling, rimesse, punizioni, fuorigioco, spazzate, parate, recuperi per tempo e gol su rigore.
- **[DEDUZIONE]** FBref non è più un sostituto affidabile dei dati Opta avanzati per il 2026-27, anche se la pagina storica 2025-26 esiste.

### Gaps
- API-Football: campi esatti dell'endpoint `events` e presenza/assenza di coordinate (documentazione 403).
- Sofascore: se esponga eventi con posizione; API bloccata 403 da questo ambiente.
- Verifica di una fonte event-level aperta alternativa non presente nell'elenco suggerito (non ho trovato riscontri credibili per il 2026-27).

## 3. Le definizioni delle metriche combinate sono confrontabili con Wyscout? Esiste documentazione?

### Takeaway
**[VERIFICATO]** Wyscout documenta ogni evento con eventId/subEventId/tag e regole di conteggio: sono regole specifiche, non universali. **[DEDUZIONE]** Il confronto 1:1 con altre fonti non è automatico: ogni metrica va mappata con una decisione esplicita, e la documentazione Wyscout è dichiaratamente riferita allo stato *corrente* del modello, non a quello del 2017-18.

### Cited Findings
- **[VERIFICATO]** `Pass`: eventId 8, subEventId 85. Cross, head pass, hand pass, smart pass e long pass sono inclusi nelle statistiche passaggi; **goal kick, corner kick, free kick cross e throw-in non sono inclusi**. `Successful` = tag 1801 (next touch by a teammate); pass accuracy = % di passaggi riusciti — [Source](https://dataglossary.wyscout.com/pass/)
- **[VERIFICATO]** `Aerial duel`: eventId 1, subEventId 10, sottotipo di Duel; vinto chi tocca per primo (tag 703 win / 701 loss), anche in caso di fallo subito; metrica "Aerial duels won (%)" — [Source](https://dataglossary.wyscout.com/aerial_duel/)
- **[VERIFICATO]** `Offensive duel`: esito con algoritmo dedicato e tag 703/701/702 (clearly won/lost/neutral); la definizione elenca esplicitamente i casi in cui l'attaccante vince il duello — [Source](https://dataglossary.wyscout.com/offensive_duel/)
- **[VERIFICATO]** `Save`: sottotipo di "Shot against (Save attempt)"; è "a successful attempt from the goalkeeper to prevent a shot from being scored", con attributi xG e xCG — [Source](https://dataglossary.wyscout.com/save/)
- **[VERIFICATO]** `Recovery`: punto in cui ricomincia il possesso; **non** sono considerate recoveries le riprese dopo uscita del pallone o fallo; tipicamente duelli (44%) e intercetti (16%) — [Source](https://dataglossary.wyscout.com/recovery/)
- **[VERIFICATO]** `Cross`, `Corner kick`, `Throw in`, `Penalty kick`, `Own goal`, `Offside`, `Yellow card`, `Red card`, `Foul`, `Clearance` sono voci separate del glossary — [Source](https://dataglossary.wyscout.com/)
- **[VERIFICATO]** Sportmonks dichiara statistiche match con "shots, possession, passes, etc." in tutti i piani e xG/Pressure Index come add-on separato — [Source](https://docs.sportmonks.com/v3/tutorials-and-guides/tutorials/statistics/fixture-statistics)
- **[VERIFICATO]** Opta ha una documentazione dedicata alle definizioni degli eventi, con controllo indipendente post-partita (solo snippet: pagina 403 al fetch) — [Source](https://www.statsperform.com/opta-event-definitions/)
- **[VERIFICATO solo via snippet]** FBref ha rimosso le metriche avanzate del provider nel gennaio 2026 — [Source](https://www.sports-reference.com/blog/2026/01/fbref-stathead-data-update/)

### Inferences
- **[DEDUZIONE]** Il rischio concreto non è solo il formato del file, ma la semantica: "passaggi accurati" Wyscout usa il next-touch del compagno e esclude i restart; un provider che conta anche rimesse/corner produce numeri non confrontabili senza DD.
- **[DEDUZIONE]** "Parate" è un sottoinsieme di "save attempt": chi fornisce solo "saves" o solo "shots on target" non equivale automaticamente al conteggio Wyscout.
- **[DEDUZIONE]** "Duelli aerei vinti" e "duelli a terra vinti" usano regole Wyscout distinte (first touch vs algoritmo su azione successiva); aggregati generici tipo "duels won" non distinguono le due popolazioni.
- **[DEDUZIONE]** "Recupero 1º/2º tempo" richiede il periodo dell'evento: il CSV gratuito non lo dà; serve un feed event-level o una fonte che spezzi le statistiche per tempo.

### Gaps
- Confronto puntuale Wyscout 2017-18 vs Wyscout 2026-27: il glossary descrive lo stato corrente, non garantisce che le regole siano identiche a quelle del dump storico.
- Definizioni ufficiali API-Football/Sofascore/FBref dopo il 2026 non recuperate (403/JS).
- Nessuna fonte verificata dia metriche aggregate per "recuperi per tempo", "spazzate" e "gol su rigore" nella stagione 2026-27.

## 4. Qual è il percorso minimo per passare dal referto 2017-18 a una fonte corrente senza riscrivere il motore?

### Takeaway
**[VERIFICATO]** Esistono tre strade documentate: (A) Wyscout API con licenza → JSON event-level nello stesso schema; (B) CSV aggregato gratuito → solo 7 metriche, confronto parziale; (C) open data aperti → non coprono la stagione. **[DEDUZIONE]** La strada A è l'unica che non tocca il reader event-level; la B richiede un nuovo formato di referto aggregato, non una riscrittura del motore.

### Cited Findings
- **[VERIFICATO]** Endpoint eventi Wyscout `GET /matches/{wyId}/events`, con `details=tag` e possibilità di escludere `positions`; risposta JSON; auth HTTP Basic — [Source](https://raw.githubusercontent.com/withqwerty/football-docs/main/docs/wyscout/api-endpoints.md); [Source](https://apidocs.wyscout.com/)
- **[VERIFICATO]** Convenzione coordinate Wyscout: (x,y) percentuale, x=0 porta difesa, x=100 porta attaccata — [Source](https://raw.githubusercontent.com/mbochicchio/wyscout-openapi/main/README.md)
- **[VERIFICATO]** StatsBomb open data è già organizzato come `events/<match_id>.json` separati per partita, con `matches/` e `competitions.json` — [Source](https://github.com/hudl/open-data)
- **[VERIFICATO]** football-data.co.uk offre CSV per stagione con URL prevedibile `mmz4281/2627/I1.csv`, aggiornato al 23/11/2026 (120 partite) — [Source](https://www.football-data.co.uk/mmz4281/2627/I1.csv)
- **[VERIFICATO]** openfootball 2026-27 è scaricabile come singolo JSON di risultati con licenza CC0 — [Source](https://raw.githubusercontent.com/openfootball/football.json/master/2026-27/it.1.json); [Source](https://raw.githubusercontent.com/openfootball/football.json/master/LICENSE.md)
- **[VERIFICATO]** Il glossary Wyscout definisce i tag delle metriche usate dal gioco (1801 pass riuscito, 703/701 duelli, 1801/1802 cross, ecc.) — [Source](https://dataglossary.wyscout.com/)

### Inferences
- **[DEDUZIONE] Percorso A (nessuna riscrittura del motore):** ottenere per ogni partita 2026-27 il JSON di `GET /matches/{wyId}/events` con `details=tag`, salvarlo con la stessa forma del dump locale (eventId, subEventId, tag, positions, timestamp) e usare l'reader esistente. Restano da rifare golden, semi e misure `ds-reference`, e da registrare una DD se le regole del glossary corrente differiscono dal 2017-18.
- **[DEDUZIONE] Percorso B (confronto parziale, zero eventi):** scaricare `I1.csv` a ogni giornata e costruire un referto aggregato con le 7 metriche dirette; tenere il dump 2017-18 per le 22 metriche complete. Serve una decisione esplicita (DD) sul fatto che i due referti non misurano lo stesso insieme; non si tara guardando la metrica (A5-bis).
- **[DEDUZIONE] Percorso C (da scartare per il 2026-27):** StatsBomb open data e openfootball non contengono statistiche della stagione target; Understat è offline.
- **[DEDUZIONE]** Se si sceglie Opta/Sportmonks/API-Football invece di Wyscout, il motore andrà adattato: gli schemi e le definizioni non coincidono (Sportmonks dichiara esplicitamente l'assenza di coordinate sugli eventi).

### Gaps
- Non verificato se il reader locale richieda esattamente lo schema Wyscout v3 o tolleri v4; serve confronto dei campi con un evento reale 2026-27.
- Non verificati costi, limiti di chiamata e diritti di uso in un gioco per Wyscout API; decisione che spetta al maintainer.
- Non verificato se il CSV football-data.co.uk resti aggiornato dopo il 23/11/2026 con la stessa cadenza (la home riporta "Updated: 27/11/26").
