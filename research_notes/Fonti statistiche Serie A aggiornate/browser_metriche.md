# Fonti browser per le 22 metriche — Serie A 2026-27 al 27/11/2026

Priorità: **Sofascore**. Il confronto con le altre fonti è ridotto al minimo necessario. Ogni dato qui sotto è stato verificato con fetch diretto il 27/11/2026; `[Fatto]` = letto nella fonte, `[Deduzione]` = interpretazione mia.

**Nota di correzione.** La versione precedente di queste note mappava erroneamente "gol su rigore" e "cross" tra le 22 metriche. Le 22 righe confrontate da `ds-reference` sono quelle di [L0-CALIBRATION.md](../../context/plans/L0-CALIBRATION.md): gol, autogol, tiri, tiri in porta, **rigori (taken)**, passaggi, passaggi riusciti, duelli aerei, duelli a terra, dribbling riusciti, falli, gialli, rossi, angoli, rimesse, **rinvii dal fondo**, punizioni, fuorigioco, spazzate, parate, recupero 1º tempo, recupero 2º tempo. I cross sono raccolti ma *non confrontati* (definizioni non allineate) — [reference JSON](../../L0-match/reference/serie-a-2017-18.json); [Metric enum](../../tools/ds-reference/src/lib.rs).

## Q1. La pagina Serie A 2026/2027 e una partita della 12ª giornata: quali delle 22 metriche sono realmente visibili per singola partita?

### Takeaway
**[Fatto]** La pagina del torneo e la pagina partita caricano (HTTP 200). **[Fatto]** Gli endpoint che alimentano la pagina partita espongono tutte e 22 le metriche per squadra: 15 in `statistics` (con split `ALL`/`1ST`/`2ND`) e 7 in `incidents`/`event`; la limitazione vera è che sono **conteggi aggregati**, non righe evento. **[Deduzione]** È la fonte browser più completa trovata, ma non è una fonte event-level per passaggi, cross, duelli, rimesse o spazzate.

### Cited Findings
- **[Fatto]** Pagina torneo: `https://www.sofascore.com/football/tournament/italy/serie-a/23` → 200, titolo "Serie A 2026/2027 table, schedule & stats". L'API dichiara `uniqueTournament` id `23` e la stagione `Serie A 26/27` id `95836` — [Sofascore Serie A](https://www.sofascore.com/football/tournament/italy/serie-a/23); [seasons API](https://www.sofascore.com/api/v1/unique-tournament/23/seasons).
- **[Fatto]** La 12ª giornata ha 10 partite chiuse (21–22/11/2026). Esempio usato: **Juventus–Venezia 4-0**, event id `16285091`, 22/11/2026 11:30 UTC, `roundInfo.round = 12` — [event API](https://www.sofascore.com/api/v1/event/16285091); [pagina partita](https://www.sofascore.com/football/match/venezia-juventus/MdbsNdb).
- **[Fatto]** `statistics` ha tre periodi (`ALL`, `1ST`, `2ND`) con 7 gruppi (Match overview, Shots, Attack, Passes, Duels, Defending, Goalkeeping); 15 chiavi del gruppo statistiche coprono 15 delle 22 metriche e sono presenti in tutti e tre i periodi (le altre 7 — gol, autogol, rigori, gialli, rossi e i due recuperi — vengono da `incidents`/`event`) — [statistics API](https://www.sofascore.com/api/v1/event/16285091/statistics).
- **[Fatto]** Mappatura verificata su Juventus–Venezia (valori `home/away`, somma tra squadre tra parentesi quando utile al confronto con `ds-reference`, che conteggia entrambe le squadre):

| Metrica (22) | Chiave / evento Sofascore | Valore JUV–VEN |
|---|---|---|
| gol | incidenti `goal` / punteggio | 4 |
| autogol | `goal` + `incidentClass: ownGoal` | 0 (es. in 12ª giornata: Calvani 4' in [Milan–Frosinone](https://www.sofascore.com/api/v1/event/16285098/incidents)) |
| tiri | `totalShotsOnGoal` | 12/13 (25) |
| tiri in porta | `shotsOnGoal` | 8/1 (9) |
| rigori (taken) | `goal|penalty` + `inGamePenalty|missed` + `shotmap.situation = penalty` | 1 (Kolo Muani 71') |
| passaggi | `passes` | 578/417 (995) |
| passaggi riusciti | `accuratePasses` | 507/347 (854) |
| duelli aerei | `aerialDuelsPercentage` | 9/15 + 6/15 (15 contest) |
| duelli a terra | `groundDuelsPercentage` | 37/70 + 33/70 (70 contest) |
| dribbling riusciti | `dribblesPercentage` | 7/12 + 6/17 (13 riusciti) |
| falli | `fouls` | 11/11 (22) |
| gialli | incidenti `card|yellow` | 0 (es. 4 in [Bologna–Udinese](https://www.sofascore.com/api/v1/event/16285090/incidents)) |
| rossi | incidenti `card|red`, `card|yellowRed` | 0 (presenti altrove nella stagione, es. [Lazio–Cagliari](https://www.sofascore.com/api/v1/event/16285073/incidents)) |
| angoli | `cornerKicks` | 5/2 (7) |
| rimesse | `throwIns` | 19/26 (45) |
| rinvii dal fondo | `goalKicks` | 4/7 (11) |
| punizioni | `freeKicks` | 10/11 (21) |
| fuorigioco | `offsides` | 2/1 (3) |
| spazzate | `totalClearance` | 12/17 (29) |
| parate | `goalkeeperSaves` | 1/4 (5) |
| recupero 1º tempo | `injuryTime` con `length` al 45', o `time.injuryTime1` | 1 min |
| recupero 2º tempo | `injuryTime` con `length` al 90', o `time.injuryTime2` | 3 min |

Fonti dei valori: [statistics](https://www.sofascore.com/api/v1/event/16285091/statistics), [incidents](https://www.sofascore.com/api/v1/event/16285091/incidents), [event/time](https://www.sofascore.com/api/v1/event/16285091), [shotmap](https://www.sofascore.com/api/v1/event/16285091/shotmap).
- **[Fatto]** I cross non sono tra le 22 confrontate ma sono visibili: `accurateCross` = `4/14 (29%)` e `1/6 (17%)` nella stessa partita — [statistics API](https://www.sofascore.com/api/v1/event/16285091/statistics).
- **[Fatto]** I due recuperi **non** sono `ballRecovery`: `ballRecovery` è la voce "Recoveries" del gruppo Defending (50/40 in JUV–VEN), mentre i minuti di recupero sono `time.injuryTime1/2` e le voci `injuryTime` con `length` — [statistics](https://www.sofascore.com/api/v1/event/16285091/statistics); [incidents](https://www.sofascore.com/api/v1/event/16285091/incidents); [event](https://www.sofascore.com/api/v1/event/16285091).
- **[Fatto]** L'evento `inGamePenalty|missed` distingue il rigore non segnato dal gol (Bologna–Monza, 81'); `varDecision|penaltyAwarded` e `varDecision|penaltyNotAwarded` sono decisioni VAR, non tentativi (entrambe in Parma–Bologna) — [incidents Bologna–Monza](https://www.sofascore.com/api/v1/event/16285066/incidents); [incidents Parma–Bologna](https://www.sofascore.com/api/v1/event/16285084/incidents).

### Inferences
- **[Deduzione]** Per il confronto L0, Sofascore copre 22/22 *conteggi per partita*, ma la copertura "evento con minuto" è reale solo per gol, cartellini, rigori, sostituzioni, injury time e tiri (shotmap). Le altre 14 metriche restano totali di squadra.
- **[Deduzione]** Non sommare i numeri Sofascore a quelli Wyscout senza una mappatura: i duelli aerei (15 contest contro 38,14 reali), i duelli a terra (70 contro 182,6) e i passaggi (995 contro 887,7) hanno definizioni diverse — [reference JSON](../../L0-match/reference/serie-a-2017-18.json).
- **[Deduzione]** "Recupero 1º/2º tempo" va letto come minuti (il progetto misura `stoppage_first_half_min` / `stoppage_second_half_min`), non come recuperi di possesso — [L0-CALIBRATION](../../context/plans/L0-CALIBRATION.md).
- **[Deduzione]** Il nome `totalShotsOnGoal` è fuorviante: l'etichetta UI è "Total shots" e il sottoinsieme in porta è `shotsOnGoal` — [statistics API](https://www.sofascore.com/api/v1/event/16285091/statistics); [locale_labels](https://www.sofascore.com/static/locales/messages.en.fbc91c1b1873.js).

### Gaps
- Le pagine sono client-rendered: l'HTML iniziale della partita non contiene le etichette statistiche (verificato: "Ball possession", "Aerial duels", "Clearances" assenti nel sorgente); la verifica è quindi sull'endpoint che la pagina consume, non su uno screenshot del tab "Statistics".
- Nessun endpoint singolo dà "rigori tentati" come numero: va ricostruito da incidenti + shotmap.
- Non verificato se il tab "Statistics" della pagina mostri anche i valori del 1º/2º tempo per ogni voce (l'API li dà; l'UI non è stata ispezionata visivamente).

## Q2. Statistiche per squadra/giocatore e timeline/eventi con minuto?

### Takeaway
**[Fatto]** Le statistiche aggregate sono per squadra; esiste un blocco statistiche per giocatore dentro `lineups`, ma è incompleto (nessun cartellino, angolo, rimessa, recupero-possesso per giocatore). **[Fatto]** La "timeline" è fatta di `incidents` (minuto + periodo) e `shotmap` (minuto + `timeSeconds`); non esiste un feed di tutti gli eventi di gioco.

### Cited Findings
- **[Fatto]** `lineups` contiene per ogni giocatore 71 chiavi: `totalPass`, `accuratePass`, `totalCross`, `accurateCross`, `aerialWon/Lost`, `duelWon/Lost`, `totalContest/wonContest`, `goals`, `ownGoals`, `totalClearance`, `ballRecovery`, `fouls`, `totalOffside`, `saves`, `minutesPlayed`, `rating`, `xg`-like (`keeperSaveValue`), ecc. Mancano `yellowCards`, `redCards`, `cornerKicks`, `throwIns`, `freeKicks`, `goalKicks` e un contatore esplicito di dribbling riusciti — [lineups API](https://www.sofascore.com/api/v1/event/16285091/lineups).
- **[Fatto]** `incidents` dà `time`, `addedTime`, `reversedPeriodTime`, `incidentType`, `incidentClass`, `reason`, `player`, `isHome`; esempi: giallo "Foul" al 49' in AC Milan–Inter (10ª giornata), `card|yellowRed` in Lazio–Cagliari, `injuryTime` con `length` — [incidents AC Milan–Inter](https://www.sofascore.com/api/v1/event/16285075/incidents); [incidents Lazio–Cagliari](https://www.sofascore.com/api/v1/event/16285073/incidents).
- **[Fatto]** `shotmap` dà 26 tiri per AC Milan–Inter con `time`, `addedTime`, `timeSeconds`, `periodTimeSeconds`, `situation`, `goalMouthLocation`, `xg`; per Juventus–Venezia il rigore segnato da Randal Kolo Muani al 71' ha `situation: "penalty"` e `goalType: "penalty"` — [shotmap 16285075](https://www.sofascore.com/api/v1/event/16285075/shotmap); [shotmap 16285091](https://www.sofascore.com/api/v1/event/16285091/shotmap).
- **[Fatto]** L'autogol di Calvani include `footballPassingNetworkAction` con 7 azioni di passaggio precedenti (con `eventType`, `time`, coordinate) — quindi una cronaca parziale, solo legata al gol — [incidents Milan–Frosinone](https://www.sofascore.com/api/v1/event/16285098/incidents).
- **[Fatto]** Il bundle JavaScript del sito chiama `/event/${id}/lineups`, `/statistics`, `/incidents`, `/shotmap`, `/graph`, `/best-players`; le mie probe di `/timeline`, `/events`, `/player-statistics`, `/statistics/events` hanno dato 404 — [match page](https://www.sofascore.com/football/match/venezia-juventus/MdbsNdb); [incidents API](https://www.sofascore.com/api/v1/event/16285091/incidents).

### Inferences
- **[Deduzione]** Per il reader locale servono `eventId/subEventId/tag/periodo/tempo`: Sofascore fornisce `time`/`period` per incidenti e tiri, ma **`eventId` e `subEventId` sono del proprio dataset, non del dump Wyscout**; il vocabolario (`ownGoal`, `penalty`, `yellowRed`) è mappings, non equivalenza provata di definizioni.
- **[Deduzione]** Il livello giocatore può integrare i totali di squadra, ma non sostituisce un feed event-level: per 22 metriche serve comunque l'aggregato di squadra.

### Gaps
- Non verificato alcun endpoint che dia tutti i passaggi di una partita con minuto e giocatore (fuori dalle reti).
- Non verificato se Sofascore Pro mostri timeline più ricche: la pagina pubblica non le espone negli endpoint provati.
- Non è stato possibile ispezionare l'UI in un browser con JS (nessuno strumento browser disponibile in questa sessione).

## Q3. API interna: documentazione, accesso senza key, Termini d'Uso e definizioni?

### Takeaway
**[Fatto]** Gli endpoint `/api/v1/...` funzionano senza key dal contesto browser, ma **non ho trovato documentazione ufficiale**: non sono API pubbliche dichiarate. **[Fatto]** I Termini vietano espressamente scraping, aggregazione, embedding, riproduzione e riutilizzo del database senza consenso scritto, e le richieste automatizzate che sovraccaricano i server. **[Fatto]** Non esiste un glossario delle definizioni: solo etichette. **[Deduzione]** Per ricerca locale il dato è comodo; per un prodotto terzo serve consenso o altra fonte.

### Cited Findings
- **[Fatto]** Endpoint senza key, HTTP 200: [stagioni](https://www.sofascore.com/api/v1/unique-tournament/23/seasons), [eventi](https://www.sofascore.com/api/v1/unique-tournament/23/season/95836/events/last/0), [statistiche](https://www.sofascore.com/api/v1/event/16285091/statistics), [incidents](https://www.sofascore.com/api/v1/event/16285091/incidents). **[Fatto]** Con `curl` da shell risposta 403 Varnish `{"error":{"code":403,"reason":"Forbidden"}}`; con `fetch` dal runtime/browser risposta 200 — accesso non uniforme per client.
- **[Fatto]** Nessuna documentazione trovata: `/api/v1` → 404 HTML, `/api/v1/swagger.json` → 404 JSON, `/api/v1/docs` → 404 JSON, `developer.sofascore.com` → DNS failure — [API root](https://www.sofascore.com/api/v1); [swagger](https://www.sofascore.com/api/v1/swagger.json); [docs](https://www.sofascore.com/api/v1/docs). **[Fatto]** Le URL sono scoperte leggendo il bundle JS del sito (§Q2), non da documentazione.
- **[Fatto — testuale, art. 2.9 "Unauthorised interference"]** *"Overloading Sofascore’s servers with automated requests or assisting others in doing so is strictly prohibited. Modifying, disassembling or reverse engineering the website is not permitted unless permitted by law. Using website content by embedding, aggregating, scraping or reproducing it without express consent is prohibited, unless permitted by law."* — [Sofascore Terms & Conditions](https://www.sofascore.com/terms-and-conditions); [testi caricati dinamicamente](https://www.sofascore.com/_next/static/chunks/66834.d2b5cd218b4b812a.js).
- **[Fatto — testuale, art. 2.8 "Database protection"]** *"The materials within the Platform’s database (“Database Content”) are protected by a sui generis right granted to the database maker. Without prior written agreement with Sofascore, Database Content may be lawfully used only to the extent permitted by law. In particular, no extraction (copying) or re-utilisation (making available to the public) of Database Content, or of a substantial part thereof, is permitted without Sofascore’s express consent."* — [Terms](https://www.sofascore.com/terms-and-conditions); [bundle](https://www.sofascore.com/_next/static/chunks/66834.d2b5cd218b4b812a.js).
- **[Fatto — testuale, art. 2.6 "Intellectual property"]** *"In particular, reproduction (copying) of Copyright Content for direct or indirect economic gain, as well as its distribution, rental, display or communication to the public (including online communication), is prohibited without Sofascore’s express consent."* — [Terms](https://www.sofascore.com/terms-and-conditions); [bundle](https://www.sofascore.com/_next/static/chunks/66834.d2b5cd218b4b812a.js).
- **[Fatto — testuale, art. 2.2]** *"The results and statistical data displayed on the Platform originate from various independent sources (third parties), internal efforts or official applications. Sofascore strives to keep the content on the Platform up to date and accurate but cannot guarantee its absolute accuracy. The User is advised to verify information from the Platform against original and alternative sources."* — [Terms](https://www.sofascore.com/terms-and-conditions); [bundle](https://www.sofascore.com/_next/static/chunks/66834.d2b5cd218b4b812a.js).
- **[Fatto — testuale, art. 1.1]** *"By accessing or using the Platform, the User agrees to be bound by these Terms, regardless of whether the User is a registered or unregistered user."* La pagina dice inoltre *"These Terms were last updated on: {publish_date}."*, ma nel bundle il segnaposto `{publish_date}` non è risolto e l'HTML iniziale non contiene la data: la data effettiva non è stata verificata — [Terms](https://www.sofascore.com/terms-and-conditions); [bundle pagina](https://www.sofascore.com/_next/static/chunks/pages/terms-and-conditions-3c765062ad6ee81c.js); [bundle testi](https://www.sofascore.com/_next/static/chunks/66834.d2b5cd218b4b812a.js).
- **[Fatto]** `robots.txt` nega tutto a `Bytespider`, vieta le stagioni 2017–2025 e le pagine classifica, ma **non contiene una regola per `/api/`** — [robots.txt](https://www.sofascore.com/robots.txt).
- **[Fatto]** Nessun glossario: `/glossary`, `/football/glossary`, `/football/statistics`, `/statistics`, `/football/statistics-definitions` → 404. Il bundle locale contiene solo etichette (`football_aerial_duels = "Aerial duels"`, `football_dribbles = "Dribbles"`, `penaltiesTaken = "Penalties taken"`, `football_goal_kicks = "Goal kicks"`, `football_recoveries = "Recoveries"`), senza definizioni — [glossary 404](https://www.sofascore.com/glossary); [locale labels](https://www.sofascore.com/static/locales/messages.en.fbc91c1b1873.js).

### Inferences
- **[Deduzione]** "Senza key" è un fatto tecnico, non una licenza: l'art. 2.9 vieta scraping/aggregazione senza consenso e l'art. 2.8 vieta estrazione/riutilizzo del database; usare questi numeri in un prodotto terzo richiede accordo scritto (o il passaggio a fonte con licenza esplicita).
- **[Deduzione]** L'assenza di glossary significa che il confronto con Wyscout resta l'unica via per definire le unità; in particolare `ballRecovery` (Sofascore) non è il recupero di tempo, e `aerialDuelsPercentage` conta contest diversi da Wyscout.
- **[Deduzione]** Il 403 con `curl` e il 200 con `fetch` indicano un filtro lato edge sensibile al client: un eventuale accesso automatizzato non è affidabile nemmeno a livello tecnico.

### Gaps
- Non verificato se Sofascore venda una licenza dati (il sito corporate non espone un pricing leggibile nelle pagine provate).
- Non verificato il testo completo dei Termini in italiano; le citazioni sono dalla versione inglese del bundle.
- Non trovata documentazione ufficiale dell'API: le chiavi usate qui sono dedotte dai bundle e dai payload, quindi possono cambiare senza preavviso.

## Q4. Confronto minimo con le altre fonti e freschezza

### Takeaway
**[Fatto]** Nessun'altra fonte verificata copre 22/22; WhoScored e FBref hanno limiti d'uso/blocchi, football-data copre ~13 colonne, Understat è offline. **[Fatto]** Al 27/11 l'ultimo turno chiuso è il 23/11 e la partita del 27/11 non è iniziata: coerenza temporale, non ritardo.

### Cited Findings
- **[Fatto]** Stagione Sofascore `95836`: 120 eventi, ultimo terminato il 23/11/2026; primo evento successivo `Venezia–Bologna` 27/11 19:45 UTC, `Not started` — [events](https://www.sofascore.com/api/v1/unique-tournament/23/season/95836/events/last/0); [next](https://www.sofascore.com/api/v1/unique-tournament/23/season/95836/events/next/0).
- **[Fatto]** ESPN ha la season `2026-27 Italian Serie A` e statistiche di squadra (falli, cartellini, fuorigioco, angoli, parate, tiri, rigori, passaggi, cross, tackle, intercetti, spazzate), ma non duelli aerei, duelli a terra, dribbling, rimesse, punizioni e recuperi — [ESPN summary](https://site.api.espn.com/apis/site/v2/sports/soccer/ITA.1/summary?event=401875002); [scoreboard](https://site.api.espn.com/apis/site/v2/sports/soccer/ITA.1/scoreboard?dates=20261101).
- **[Fatto]** football-data.co.uk (aggiornato 27/11/26) dà CSV con `HxG, AxG, HS, AS, HST, AST, HF, AF, HC, AC, HY, AY, HR, AR` e risultati FT/HT; non dà passaggi, cross, duelli, rimesse, punizioni, fuorigioco, spazzate, parate, rigori, autogol come colonne — [Italia](https://football-data.co.uk/italym.php); [CSV I1 2627](https://football-data.co.uk/mmz4281/2627/I1.csv).
- **[Fatto]** WhoScored ha pagina Serie A 2026/27 e glossario pubblico, ma la pagina partita risponde 403 al fetch e i Termini vietano copia/download/riproduzione/trasmissione delle statistiche senza licenza — [WhoScored Serie A](https://www.whoscored.com/regions/108/tournaments/5/seasons/11126/stages/25518/show/italy-serie-a-2026-2027); [glossario](https://www.whoscored.com/glossary); [ToS](https://www.whoscored.com/termsofuse).
- **[Fatto]** FBref 403; Sports Reference ha annunciato il 20/01/2026 la rimozione dei dati avanzati dopo la fine dell'accesso del fornitore; BeSoccer 406, Playmakerstats 403, Statbunker 404, Understat offline — [FBref](https://fbref.com/en/comps/11/Serie-A-Stats); [blog SR](https://www.sports-reference.com/blog/2026/01/fbref-stathead-data-update/); [Understat](https://understat.com/).
- **[Fatto]** URL da non riusare: WhoScored `/Regions/74/Tournaments/22/Italy-Serie-A` ridirige a Ligue 1; Sofascore `/football/competition/serie-a/23` → 404 (percorso corretto `/football/tournament/italy/serie-a/23`) — [WhoScored errato](https://www.whoscored.com/Regions/74/Tournaments/22/Italy-Serie-A); [Sofascore errato](https://www.sofascore.com/football/competition/serie-a/23).

### Inferences
- **[Deduzione]** Per calibrazione L0: usare Sofascore per freschezza e conteggi per partita/tempo; mantenere il grezzo Wyscout come definizione di riferimento; ESPN e football-data solo come controllo incrociato parziale.
- **[Deduzione]** Nessuna fonte browser gratuita verificata fornisce eventi di gioco completi (passaggi, duelli, spazzate con minuto): per il reader event-level serve tracking o piattaforma a pagamento.

### Gaps
- Ricerca web automatica non disponibile (HTTP 402 su tutte le query): verifica fatta con fetch diretto e alcune query Bing; può esserci una fonte non trovata.
- Non verificato se Understat tornerà online, né se esista una pagina Lega Serie A con statistiche per partita leggibili senza account.
