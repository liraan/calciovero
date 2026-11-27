# Fonti browser per le 22 metriche — Serie A 2026-27 al 27/11/2026

## Q1. Quale fonte consultabile da browser copre le 22 metriche richieste, per partita e senza API key?

### Takeaway
Nessuna fonte verificata fornisce le 22 metriche come righe evento con `eventId/subEventId/tag/periodo/tempo`; **Sofascore** è l'unica che copre tutte e 22 come conteggi per partita (e per primo/secondo tempo) con endpoint pubblici raggiungibili dal browser senza chiave. Le altre coprono solo sottoinsiemi oppure non sono verificabili.

### Cited Findings
- La pagina Serie A 2026/2027 di Sofascore carica (HTTP 200) e l'endpoint pubblico `unique-tournament/23/seasons` dichiara `Serie A 26/27` con id `95836` — [Sofascore Serie A](https://www.sofascore.com/football/tournament/italy/serie-a/23); [Sofascore seasons API](https://www.sofascore.com/api/v1/unique-tournament/23/seasons).
- L'endpoint `statistics` di una partita conclusa (Milan-Inter 3-1, id `16285075`) espone tre periodi (`ALL`, `1ST`, `2ND`) e i gruppi Match overview / Shots / Attack / Passes / Duels / Defending / Goalkeeping — [Sofascore match statistics](https://www.sofascore.com/api/v1/event/16285075/statistics).
- Mappatura verificata delle 22 metriche sui campi Sofascore (stessi valori compaiono nei periodi `ALL`, `1ST`, `2ND`): gol = punteggio + incidenti `goal`; autogol = `goal|ownGoal`; tiri = `totalShotsOnGoal`; tiri in porta = `shotsOnGoal`; passaggi = `passes`; passaggi accurati = `accuratePasses`; cross = `accurateCross` (es. `6/11 (55%)` = 11 tentativi, 6 riusciti); duelli aerei = `aerialDuelsPercentage`; duelli a terra = `groundDuelsPercentage`; dribbling/duelli offensivi = `dribblesPercentage`; falli = `fouls`; gialli = `yellowCards`; rossi = incidenti `card|red` e `card|yellowRed`; angoli = `cornerKicks`; rimesse = `throwIns`; punizioni = `freeKicks`; fuorigioco = `offsides`; spazzate = `totalClearance`; parate = `goalkeeperSaves`; gol su rigore = incidente `goal|penalty`. — [Sofascore match statistics](https://www.sofascore.com/api/v1/event/16285075/statistics); [Sofascore incidents 16285075](https://www.sofascore.com/api/v1/event/16285075/incidents).
- I due "recuperi" non sono `ballRecovery` (che è una statistica di difesa distinta), ma il tempo aggiunto: ogni evento ha `time.injuryTime1` e `time.injuryTime2` (es. Bologna-Monza `1/6`) e gli incidenti hanno voci `injuryTime` con `length` (es. `3` al 45' e `2` al 90' in Atalanta-Parma) — [Sofascore season events](https://www.sofascore.com/api/v1/unique-tournament/23/season/95836/events/last/0); [Sofascore incidents 16285071](https://www.sofascore.com/api/v1/event/16285071/incidents).
- Gli incidenti distinguono tipi e classi con minuto e tempo di periodo: su 120 partite scansionate sono presenti `goal|ownGoal` (15), `goal|penalty` (20), `card|red`, `card|yellowRed`, `inGamePenalty|missed` e `injuryTime`; esempio di autogol: Zappacosta 47', `from:"owngoal"`, `goalType:"own"`, `isHome:false` nella cronaca di Atalanta-Parma 0-3 — [Sofascore incidents 16285071](https://www.sofascore.com/api/v1/event/16285071/incidents); [Sofascore incidents 16285073](https://www.sofascore.com/api/v1/event/16285073/incidents).
- Per i soli tiri esiste anche il livello evento: `shotmap` restituisce 26 voci per Milan-Inter (come `7+19` delle statistiche), con `time`, `addedTime`, `timeSeconds`, `reversedPeriodTime`, `periodTimeSeconds`, `incidentType` e `situation` — [Sofascore shotmap 16285075](https://www.sofascore.com/api/v1/event/16285075/shotmap).
- La pagina browser della partita esiste e risponde 200 dopo redirect (`/event/16285075/overview` → `/football/match/inter-ac-milan/RdbsXdb`) — [Sofascore match page](https://www.sofascore.com/football/match/inter-ac-milan/RdbsXdb).
- ESPN copre molte ma non tutte le metriche: nella summary di Como-Venezia le statistiche di squadra sono falli, gialli, rossi, fuorigioco, angoli, parate, possesso, tiri, tiri in porta, rigori (gol/tentativi), passaggi/accurati, cross/accurati, lanci lunghi, tiri bloccati, tackle, intercetti e spazzate; mancano duelli aerei, duelli a terra, dribbling, rimesse, punizioni e i due recuperi — [ESPN summary API](https://site.api.espn.com/apis/site/v2/sports/soccer/ITA.1/summary?event=401875002); [ESPN scoreboard API](https://site.api.espn.com/apis/site/v2/sports/soccer/ITA.1/scoreboard?dates=20261101).
- football-data.co.uk fornisce il CSV gratuito 2026/27 con `HxG, AxG, HS, AS, HST, AST, HF, AF, HC, AC, HY, AY, HR, AR` e risultati FT/HT, ma non passaggi, cross, duelli, rimesse, punizioni, fuorigioco, spazzate, parate, recuperi, rigori o autogol come colonne — [football-data Italia](https://football-data.co.uk/italym.php); [CSV I1 2627](https://football-data.co.uk/mmz4281/2627/I1.csv).
- WhoScored ha il glossario pubblico delle definizioni (Aerial Duel Won, Clearance, Cross, Dribble, Foul, Offside, Pass success, Shot, Shot on target, Tackle, Through Ball, Set Piece Goal, Open Play Goal), ma la pagina singola partita risponde 403 al fetch e i termini vietano copia/download/riproduzione/trasmissione delle statistiche senza licenza — [WhoScored Glossary](https://www.whoscored.com/glossary); [WhoScored match 1980904](https://www.whoscored.com/matches/1980904/show/italy-serie-a-2026-2027-juventus-venezia); [WhoScored ToS](https://www.whoscored.com/termsofuse).
- Wyscout è una piattaforma Hudl a pagamento con richiesta demo, non una fonte browser gratuita — [Wyscout platform](https://www.wyscout.com/platform/); [Wyscout pricing](https://www.wyscout.com/en_gb/products/wyscout/pricing).
- Transfermarkt mostra lo spielbericht (es. Inter-Monza 22/08/2026) ma la pagina non espone le categorie statistiche richieste (nessuna occorrenza di possession, shots, corners, fouls, offside, passes, tackles) — [Transfermarkt match sheet](https://www.transfermarkt.com/inter-milan_ac-monza/index/spielbericht/4893347).

### Inferences
- Per il confronto L0 con i numeri del grezzo Wyscout, Sofascore è l'unica fonte web verificata che permette di coprire 22/22 conteggi per partita e per tempo senza API key; le altre vanno usate come controllo incrociato parziale.
- Il filtro "senza coordinate" è soddisfatto: le statistiche aggregate non richiedono posizioni; gli incidenti e lo shotmap contengono coordinate ma esse non sono necessarie per le 22 metriche.
- `recupero 1º/2º tempo` va letto come minuti di recupero (`injuryTime1/2`), non come `ballRecovery`: l'ordine di grandezza atteso nel progetto è minuti, non conteggi.

### Gaps
- Nessuna fonte verificata espone come evento separately datato ogni passaggio, cross, duello, rimessa o spazzata: per quelle metriche Sofascore dà solo totali di squadra per `ALL`/`1ST`/`2ND`, non righe con `eventId/subEventId/tag/tempo`. La mappatura al reader locale resta quindi parziale.
- Non è stato possibile verificare le statistiche dentro le pagine WhoScored (403), quindi non si può confermare se il loro timeline offra timestamp per tutte le 22 metriche.
- Non è stata trovata una fonte browser gratuita con eventi di gioco completi per Serie A 26/27 (tranne tiri e incidenti major su Sofascore); serve tracciamento o piattaforma a pagamento per event-level completo.

## Q2. I dati sono aggiornati al 27/11/2026 e sono verificabili per partita?

### Takeaway
Sofascore risulta aggiornato all'ultimo turno giocato (23/11/2026) e mostra la partita del 27/11 come non ancora iniziata; WhoScored e football-data hanno coperture coerenti, Understat è offline.

### Cited Findings
- La stagione Sofascore `95836` contiene 120 eventi, l'ultimo dei quali terminato il 23/11/2026 — [Sofascore season events](https://www.sofascore.com/api/v1/unique-tournament/23/season/95836/events/last/0).
- Il primo evento successivo è `Venezia-Bologna`, 27/11/2026 19:45 UTC, stato `Not started` — [Sofascore next events](https://www.sofascore.com/api/v1/unique-tournament/23/season/95836/events/next/0).
- WhoScored mostra gli stessi incontri del 27/11 (Venezia-Bologna non ancora giocata) e risultati a tutto il 23/11 — [WhoScored Serie A 2026-2027](https://www.whoscored.com/regions/108/tournaments/5/seasons/11126/stages/25518/show/italy-serie-a-2026-2027).
- football-data.co.uk indica "last updated 27/11/26" per la pagina Italia e "26/11/26" per i dati; il CSV contiene righe di campionato 2026/27 (es. Inter-Monza 4-1 del 22/08/2026) — [football-data Italia](https://football-data.co.uk/italym.php); [CSV I1 2627](https://football-data.co.uk/mmz4281/2627/I1.csv).
- ESPN espone la stagione `2026-27 Italian Serie A` e partite `Full Time` nella scoreboard del 1/11/2026 — [ESPN scoreboard API](https://site.api.espn.com/apis/site/v2/sports/soccer/ITA.1/scoreboard?dates=20261101).
- Understat non èconsultabile: la homepage mostra solo `We will come back soon` e la pagina Serie A dà transport error — [Understat](https://understat.com/); [Understat Serie A](https://www.understat.com/League/Serie-A).
- La pagina ufficiale Lega Serie A statistics reindirizza (307) senza fornire statistiche leggibili — [Lega Serie A statistics](https://www.legaseriea.it/en/serie-a/statistics).

### Inferences
- Al 27/11/2026 non esiste un turno completo successivo al 23/11: l'assenza del match Venezia-Bologna nei dati Sofascore è coerenza temporale, non ritardo di aggiornamento.
- La granularità utile per il progetto è: totali per partita + totali per tempo (Sofascore), incidenti datati per gol/cartelli/infortuni/rigori, shotmap datato per tiri; per il resto serve aggregazione.

### Gaps
- Non è stato verificato se e quando Understat tornerà online; nessuna data di ritorno è dichiarata nella pagina.
- Non è stata trovata una pagina Lega Serie A con statistiche per partita liberamente leggibile senza account.
- La ricerca web automatica non era disponibile (HTTP 402); la verifica è stata fatta con fetch diretto delle pagine e degli endpoint citati.

## Q3. Quali sono i limiti tecnici/legali e quali fonti sono da escludere?

### Takeaway
I limiti principali sono il blocco bot su FBref/BeSoccer/Playmakerstats, la rimozione ufficiale dei dati avanzati da FBref a gennaio 2026, i termini restrittivi di WhoScored e la natura a pagamento di Wyscout; Sofascore non dichiara una licenza scraping verificabile nei termini letti.

### Cited Findings
- FBref risponde 403 sia al fetch diretto sia tramite proxy con verifica bot — [FBref Serie A](https://fbref.com/en/comps/11/Serie-A-Stats).
- Sports Reference ha annunciato il 20/01/2026 che il fornitore dei dati avanzati ha terminato l'accesso e ne ha chiesto la cancellazione immediata; i dati avanzati sono stati rimossi da FBref e Stathead — [Sports Reference blog](https://www.sports-reference.com/blog/2026/01/fbref-stathead-data-update/).
- BeSoccer e Playmakerstats non sono leggibili (rispettivamente 406/403 e 403); Statbunker dà 404/502 e Soccerway reindirizza a Flashscore senza statistiche verificabili — [BeSoccer](https://www.besoccer.com/competition/serie-a); [Playmakerstats](https://www.playmakerstats.com/competicao/serie-a/233?season=2026); [Statbunker](https://www.statbunker.com/competitions/Serie-A/); [Soccerway](https://www.soccerway.com/national/italy/serie-a/20262027/regular-season/).
- WhoScored vieta espressamente copia, download, riproduzione e trasmissione delle statistiche senza licenza ufficiale — [WhoScored ToS](https://www.whoscored.com/termsofuse).
- I termini Sofascore sono raggiungibili ma il corpo non è esposto nel testo statico della pagina; non ho quindi potuto verificare una clausola specifica su scraping o riutilizzo dei dati — [Sofascore Terms & Conditions](https://www.sofascore.com/terms-and-conditions).
- Wyscout richiede demo/contratto e non offre accesso browser gratuito ai dati — [Wyscout platform](https://www.wyscout.com/platform/); [Wyscout pricing](https://www.wyscout.com/en_gb/products/wyscout/pricing).
- URL errati da non riusare: `/Regions/74/Tournaments/22/Italy-Serie-A` di WhoScored ridirige a Ligue 1; `sofascore.com/football/competition/serie-a/17` e `/23` danno 404 (il percorso corretto è `/football/tournament/italy/serie-a/23`) — [WhoScored URL errato](https://www.whoscored.com/Regions/74/Tournaments/22/Italy-Serie-A); [Sofascore URL errato](https://www.sofascore.com/football/competition/serie-a/23); [Sofascore Serie A corretto](https://www.sofascore.com/football/tournament/italy/serie-a/23).

### Inferences
- Per un uso locale di ricerca/calibrazione, Sofascore è la via più breve; per una pubblicazione o redistribuzione dei numeri serve prima chiarire i termini di riutilizzo con il provider.
- FBref non è più una fonte affidabile per metriche avanzate nemmeno superando il 403: la rimozione è ufficiale e datata.
- I confronti con il grezzo Wyscout rimangono il riferimento per definizioni/unità; le fonti browser vanno usate per confermare freschezza e conteggi, non per cambiare tarature.

### Gaps
- Non verificato il contenuto legale completo dei termini Sofascore (testo non presente nel fetch statico): resta aperta la questione del riutilizzo dei dati.
- Non verificato se esista un accordo o un piano Sofascore/Hudl che conceda accesso dati a basso costo per uso di ricerca.
- Non è stato possibile consultare FBref per controllare direttamente quali statistiche basiche restino dopo la rimozione del gennaio 2026.
