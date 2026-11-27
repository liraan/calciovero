# Fonti API per statistiche Serie A aggiornate (lettura 27/11/2026)

## Qual è l'ultima giornata Serie A giocata al 27/11/2026?

### Takeaway
L'ultima giornata **completamente giocata** è la **12ª**, disputata tra il 21 e il 23 novembre 2026; la 13ª giornata è già calendarizzata ma non era ancora iniziata al momento della lettura.

### Cited Findings
- L'endpoint Sofascore della stagione 26/27 (`uniqueTournament` 23, `season` 95836) restituisce per il round 12 dieci partite `finished`, dal 21/11/2026 (Como–Cagliari 2-0) al 23/11/2026 (Monza–Fiorentina 3-0) — [Sofascore, giornata 12](https://www.sofascore.com/api/v1/unique-tournament/23/season/95836/events/round/12).
- Il primo evento successivo è Venezia–Bologna del 27/11/2026 alle 19:45, round 13, con stato `notstarted`; gli altri eventi `next` sono del 28–30 novembre — [Sofascore, prossimi eventi](https://www.sofascore.com/api/v1/unique-tournament/23/season/95836/events/next/0).
- La pagina della stagione riporta "All statistics correct as of 23 November 2026", classifica e risultati "Updated to match(es) played on 23 November 2026", 120 partite e 379 gol (3,16 a partita), con 12 partite giocate per squadra — [Wikipedia, 2026–27 Serie A](https://en.wikipedia.org/wiki/2026%E2%80%9327_Serie_A).
- Il sito ufficiale pubblica in data 27 novembre 2026 un articolo intitolato "Tra speranze e incroci pericolosi: spazio alla 13ª giornata" e espone le sezioni Calendario e Risultati, Classifica e Statistiche — [Lega Serie A](https://www.legaseriea.it/serie-a).

### Inferences
- Per una calibrazione "a giornata giocata" il taglio corretto è fine giornata 12 (23/11/2026); usare la 13ª significherebbe mescolare partite non giocate.
- La fonte ufficiale e Wikipedia concordano sulla freschezza al 23 novembre; Sofascore dà il dettaglio delle partite e del numero di giornata.

### Gaps
- Non ho trovato una pagina ufficiale Lega con l'etichetta esplicita "12ª giornata" leggibile senza JavaScript: l'URL del calendario ritorna solo la navigazione nel mio fetch — [Lega Serie A, calendario](https://www.legaseriea.it/serie-a/calendario-risultati).
- L'orario esatto di lettura non è noto: se la lettura avviene dopo le 19:45 del 27/11, Venezia–Bologna potrebbe essere iniziata o finita.

## Quali API/servizi coprono la Serie A, con quali statistiche, latenza, formato e piano?

### Takeaway
Le opzioni verificabili sono: **football-data.org** (l'unica con Serie A esplicita nel piano gratuito e prezzi pubblici), **Sportradar** (coverage ufficiale dichiarata per la Serie A, ma prezzi su contatto), **Stats Perform/Opta** (prodotto premium, nessun prezzo pubblico), **Sofascore** (JSON pubblico non ufficiale), plus **FotMob** non ufficiale con avvertenze legali; **API-Football/RapidAPI** non è verificabile dalle mie letture.

### Cited Findings
- football-data.org: il piano Free (€0/mese) comprende 12 competizioni tra cui Serie A, "Scores delayed", fixture, schedule, league table, 10 chiamate/minuto; "Free w/ Livescores" è €12/mese con 20 chiamate/minuto; Standard €49/mese (30 competizioni, 60/minuto) e Advanced €99/mese (50 competizioni, 100/minuto) — [Prezzi football-data.org](https://www.football-data.org/pricing), [policy di throttling](https://docs.football-data.org/general/v4/policies.html).
- Lo Statistic Add-On (€15/mese) copre corners, free-kick, goal-kick, fuorigioco, falli, possesso, parate, rimesse, tiri in/out e cartellini; lo Stats Pro Add-On (€30/mese) aggiunge xG, xA, xGOT, passaggi, tiri, duelli, tackle, cross e intercetti — [Prezzi football-data.org](https://www.football-data.org/pricing).
- La pagina coverage elenca "Italy Serie A LEAGUE" e anche Serie B, Coppa Italia, Supercoppa e Serie C gironi A/B/C; il piano "ML Pack Light" (€29/mese) dichiara 10 stagioni di storia e 20 chiamate/minuto — [Coverage](https://www.football-data.org/coverage), [Prezzi](https://www.football-data.org/pricing).
- Sofascore espone JSON senza chiave: l'ID Serie A è 23 (ricerca "Serie A", categoria Italy) e la stagione 26/27 ha ID 95836; le statistiche partita includono xG, possesso, tiri, parate, passaggi, duelli e rating — [Ricerca Sofascore](https://www.sofascore.com/api/v1/search/all?q=Serie%20A), [Stagioni](https://www.sofascore.com/api/v1/unique-tournament/23/seasons), [Statistiche evento 16285093](https://www.sofascore.com/api/v1/event/16285093/statistics).
- Sportradar dichiara statistiche "official" per Italy Serie A nel feed League Timeline; la Season Schedule ha cache TTL di 300 secondi, la Sport Event Summary di 1 secondo; la documentazione ammette accessi `trial` e `production` e al massimo 3 stagioni per competizione (2 passate + corrente) — [League Timeline](https://developer.sportradar.com/soccer/reference/soccer-league-timeline.md), [Season Schedule](https://developer.sportradar.com/soccer/reference/soccer-season-schedule.md), [Sport Event Summary](https://developer.sportradar.com/soccer/reference/soccer-sport-event-summary.md).
- Il portale Sportradar offre "Sign Up For Free Trials", ma non pubblica prezzi nella documentazione consultata — [Getting Started](https://developer.sportradar.com/getting-started/docs/get-started).
- Stats Perform/Opta descrive feed real-time con analisi dettagliate e "Opta Vision" (tracking XY continuo dei 22 giocatori, sincronizzato con i dati evento); le FAQ indicano REST, websocket, push/pull e S3, JSON e XML, con API key; per prezzi e accesso rimanda al team commerciale — [Opta Data Feeds](https://www.statsperform.com/products/opta-data-feeds/), [FAQ API e data delivery](https://www.statsperform.com/faqs/stats-perform-faqs-apis-data-delivery/).
- La pagina RapidAPI di API-Football v3 è raggiungibile come HTML ma i dettagli di piani e documentazione non sono estraibili dal contenuto servito; la documentazione ufficiale api-football.com risulta protetta da verifica bot (HTTP 403) — [RapidAPI API-Football v3](https://rapidapi.com/api-sports/api/api-football-v3), [Documentazione API-Football](https://www.api-football.com/documentation-v3).
- FotMob è una documentazione non ufficiale di endpoint pubblici non autenticati (JSON) per `matchDetails`, `leagues`, `teams`, ma il README segnala che i termini d'uso vietano retrieval automatico e che una richiesta di takedown è irrisolta — [README Public-FotMob-API](https://raw.githubusercontent.com/pseudo-r/Public-FotMob-API/main/README.md).

### Inferences
- Per un uso immediato e legale, football-data.org è la via più semplice: Serie A già nel piano Free, ma per statistiche complete servono gli add-on e la latenza resta "delayed" nel piano gratuito.
- Per dati evento ricchi (xG, duelli, posizioni) Sportradar e Opta sono più adatti, ma richiedono negoziazione commerciale; Sofascore è il sostituto rapido non ufficiale per esplorazione, non per una dipendenza di produzione senza verifica dei termini.
- Le tre stagioni massime di Sportradar e le 10 stagioni dell'ML Pack Light di football-data.org vanno tenute presenti se si vuole ricalibrare su 2017-18: la stagione vecchia non è garantita da Sportradar.

### Gaps
- Prezzi, limiti e endpoint API-Football/API-Sports non verificati: documentazione e pricing bloccati da Cloudflare (HTTP 403) — [api-sports.io](https://api-sports.io/documentation/football/v3), [api-football.com](https://www.api-football.com/pricing).
- Per Stats Perform/Opta non ho trovato prezzi, piano free, endpoint pubblici o una dichiarazione esplicita di copertura Serie A nelle pagine accessibili — [FAQ](https://www.statsperform.com/faqs/stats-perform-faqs-apis-data-delivery/).
- Non ho trovato un'API pubblica documentata della Lega Serie A: il sito ufficiale offre pagine web, non un endpoint commerciale — [Lega Serie A](https://www.legaseriea.it/serie-a).
- Il README FotMob non dà l'ID Serie A negli esempi (usa la Premier League = 47), quindi la copertura Serie A di quella fonte non è stata verificata — [README](https://raw.githubusercontent.com/pseudo-r/Public-FotMob-API/main/README.md).

## Quali endpoint documentati coprono fixture per giornata/lega/stagione e statistiche partita?

### Takeaway
football-data.org ha filtri ufficiali `matchday` e `season` e uno schema Match con statistiche; Sofascore ha endpoint JSON empirici per round e statistiche; Sportradar offre `season schedule` e `sport event summary`, ma non un filtro "round" documentato nel feed consultato.

### Cited Findings
- football-data.org: la documentazione mostra `GET https://api.football-data.org/v4/competitions/DED/matches?matchday=23` e la pagina Filtri definisce `matchday` (intero) e `season` (4 cifre, default anno corrente); il codice Serie A è `SA`, usato negli esempi `/v4/competitions/SA/scorers` — [Competition](https://docs.football-data.org/general/v4/competition.html), [Filtri](https://docs.football-data.org/general/v4/filters.html).
- La risorsa Match è `GET https://api.football-data.org/v4/matches/{id}`; l'esempio documentato contiene `matchday`, `lastUpdated`, `status` e un oggetto `statistics` con corner, free kick, goal kick, fuorigioco, falli, possesso, parate, rimesse, tiri, tiri in porta, cartellini — [Match](https://docs.football-data.org/general/v4/match.html).
- La documentazione non elenca un sotto-endpoint `/statistics`: le statistiche sono campi della risorsa Match — [Match](https://docs.football-data.org/general/v4/match.html).
- Sofascore, verificato il 27/11/2026: `GET /api/v1/unique-tournament/23/season/95836/events/round/12` restituisce le 10 partite della giornata con `hasNextPage`; `GET /api/v1/event/16285093/statistics` restituisce periodi `ALL`, `1ST`, `2ND` e gruppi Match overview, Shots, Attack, Passes, Duels, Defending, Goalkeeping — [Giornata 12](https://www.sofascore.com/api/v1/unique-tournament/23/season/95836/events/round/12), [Statistiche](https://www.sofascore.com/api/v1/event/16285093/statistics).
- Sportradar: `GET https://api.sportradar.com/soccer/{access_level}/v4/{language_code}/seasons/{season_id}/schedules.{format}` (access_level `trial` o `production`, format `json`/`xml`, parametri `start`/`offset`) e `GET https://api.sportradar.com/soccer/{access_level}/v4/{language_code}/sport_events/{sport_event_id}/summary.{format}` — [Season Schedule](https://developer.sportradar.com/soccer/reference/soccer-season-schedule.md), [Sport Event Summary](https://developer.sportradar.com/soccer/reference/soccer-sport-event-summary.md).
- Sempre Sportradar: `league_timeline.{format}` fornisce statistiche allineate ai siti ufficiali delle leghe, inclusa Italy Serie A, con TTL 300 secondi — [League Timeline](https://developer.sportradar.com/soccer/reference/soccer-league-timeline.md).

### Inferences
- Lo schema minimo per Calcio Vero può essere: `/v4/competitions/SA/matches?season=2026&matchday=12` per le partite, poi `/v4/matches/{id}` per il dettaglio e le statistiche, pagando gli add-on se servono xG/passaggi/duelli.
- Sofascore permette lo stesso flusso senza token (`events/round/{n}` → `event/{id}/statistics`), ma è un'API non documentata e non ufficiale.
- Sportradar copre lega+stagione e singolo evento, non ho trovato un parametro round nella documentazione del Season Schedule: il filtro giornata va ricostruito dal payload.

### Gaps
- Non ho verificato con una chiamata autenticata che `/v4/competitions/SA/matches?season=2026&matchday=12` risponda con i dati attesi: ho verificato la forma dell'endpoint dalla documentazione, non la risposta live — [Competition](https://docs.football-data.org/general/v4/competition.html).
- Endpoint e parametri API-Football per `fixtures?league=...&season=...&round=...` non sono confermati da fonte accessibile — [Documentazione](https://www.api-football.com/documentation-v3).

## Quali fonti non coprono la Serie A o non risultano aggiornate, e cosa resta da verificare?

### Takeaway
La fonte "Sofascore uniqueTournament 8" NON è Serie A ma LaLiga; le fonti ufficiali premium non offrono verifiche pubbliche complete; la Lega Serie A non espone un'API pubblica documentata.

### Cited Findings
- `GET https://www.sofascore.com/api/v1/unique-tournament/8/seasons` restituisce "LaLiga 26/27" (id 97268) e stagioni della Liga spagnola, non della Serie A — [Sofascore tournament 8](https://www.sofascore.com/api/v1/unique-tournament/8/seasons).
- L'ID corretto della Serie A è 23, con stagione 2026/27 id 95836 — [Ricerca](https://www.sofascore.com/api/v1/search/all?q=Serie%20A), [Stagioni](https://www.sofascore.com/api/v1/unique-tournament/23/seasons).
- L'URL `/it/serie-a/calendario-risultati` della Lega ritorna un redirect 307 al sito senza contenuto utile nel fetch diretto; tramite lettura testuale si arriva alla pagina `/serie-a` con sola navigazione — [Lega Serie A](https://www.legaseriea.it/it/serie-a/calendario-risultati).
- La documentazione API-Football e api-sports.io non è leggibile senza superare la verifica bot (HTTP 403), quindi non ci sono numeri di piano o copertura da citare — [api-football.com](https://www.api-football.com/documentation-v3), [api-sports.io](https://api-sports.io/documentation/football/v3).

### Inferences
- Qualsiasi pipeline che usi l'ID 8 di Sofascore per la Serie A sarebbe structuralmente sbagliata: l'ID corretto è 23.
- Le affermazioni su API-Football, Opta e Lega Serie A vanno trattate come incomplete finché non si ha accesso a documentazione o contratto.

### Gaps
- Non verificato se esista un piano API-Football gratuito sufficiente per 380 partite e statistiche, né il costo del piano utile — [RapidAPI](https://rapidapi.com/api-sports/api/api-football-v3).
- Non verificata la copertura della stagione 2017-18 nelle fonti correnti: Sportradar dichiara al massimo 3 stagioni per competizione, football-data.org vende 10 stagioni solo in ML Pack Light — [Season Schedule](https://developer.sportradar.com/soccer/reference/soccer-season-schedule.md), [Prezzi](https://www.football-data.org/pricing).
- Non misurata la latenza numerica di football-data.org, Sofascore, Stats Perform o Lega: solo Sportradar pubblica TTL espliciti — [Prezzi](https://www.football-data.org/pricing), [Sport Event Summary](https://developer.sportradar.com/soccer/reference/soccer-sport-event-summary.md).
