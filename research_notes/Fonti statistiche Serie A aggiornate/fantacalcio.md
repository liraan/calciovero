# Fantacalcio come fonte di statistiche Serie A aggiornate (al 27 novembre 2026)

Verifica eseguita il 27/11/2026. Etichette: **[VERIFICATO]** = letto direttamente sulla pagina citata; **[DEDUZIONE]** = inferenza mia, non affermazione della fonte. Lo strumento di web search interno ha risposto `HTTP 402` a tutte le query, quindi le pagine sono state verificate con fetch/curl diretti. Le pagine non raggiungibili o con risposta 404/soft-404 sono segnalate. Nessun file di codice del repo è stato modificato.

## 1. Quali statistiche pubblica Fantacalcio, a che livello e con quali definizioni?

### Takeaway
**[VERIFICATO]** La pagina che si chiama «Statistiche» è una classifica **fantasy cumulativa per calciatore**, non un referto di atti di partita: PV, MV, Fantamedia, gol, gol subiti, rigori, rigori parati, assist, ammonizioni ed espulsioni. **[VERIFICATO]** Non espone tiri, passaggi, tiri in porta, angoli, fuorigioco, falli, parate, duelli, possesso, spazzate, rimesse o coordinate. **[VERIFICATO]** Le uniche definizioni visibili nella pagina sono i `title` delle colonne; non c'è un glossario collegato. L'assist ha invece un regolamento editoriale Fantacalcio, non la definizione Wyscout.

### Cited Findings
- **[VERIFICATO]** Il titolo della pagina è «Statistiche Fantacalcio Serie A - Stagione 2026/27»; la tabella ha colonne `Calciatore`, `Sq`, `PV` (title: «Partite a voto»), `MV` («Media voto»), `FM` («Fantamedia»), `Gol`, `GS` («Gol subiti»), `Rig` («Rigori segnati / tirati»), `RP` («Rigori parati»), `Ass`, `Amm`, `Esp`. Il modal «Scegli visualizzazione» offre **solo queste colonne**, con minimo 1 e massimo 8 visibili — [Source](https://www.fantacalcio.it/statistiche-serie-a)
- **[VERIFICATO]** Le viste `/riepilogo`, `/gol`, `/assist`, `/rigori`, `/medie` hanno **le stesse colonne**; cambia l'ordinamento/il sottoinsieme, non il vocabolario dei dati — [Source](https://www.fantacalcio.it/statistiche-serie-a/2026-27/fantacalcio/riepilogo); [Source](https://www.fantacalcio.it/statistiche-serie-a/2026-27/fantacalcio/gol); [Source](https://www.fantacalcio.it/statistiche-serie-a/2026-27/fantacalcio/assist); [Source](https://www.fantacalcio.it/statistiche-serie-a/2026-27/fantacalcio/rigori); [Source](https://www.fantacalcio.it/statistiche-serie-a/2026-27/fantacalcio/medie)
- **[VERIFICATO]** I filtri sono: ricerca, stagione (2015/16–2026/27), redazione, presenze, ruolo (classic/mantra) e squadra. **Non c'è un filtro per giornata** — [Source](https://www.fantacalcio.it/statistiche-serie-a)
- **[VERIFICATO]** L'opzione `statistico` è una **redazione di voto**, non una fonte statistica alternativa: nell'HTML il radio commentato dice «Voto Statistico». Confrontando la stessa prima riga (Malen), passando da `fantacalcio` a `statistico` cambiano solo `MV` 6,82 → 6,73 e `FM` 10,14 → 10,05; gol 12, rig 0/0, ass 2, amm 1, esp 1 restano identici — [Source](https://www.fantacalcio.it/statistiche-serie-a); [Source](https://www.fantacalcio.it/statistiche-serie-a/2026-27/statistico)
- **[VERIFICATO]** La pagina squadra mostra «Marcatori», «Assist», «Tiratori» (con «Dati non disponibili» sotto «Rigori») e «Disciplina» con «Cartellini gialli» e «Cartellini rossi»: sono aggregati di squadra, non conteggi di tiri/passaggi — [Source](https://www.fantacalcio.it/serie-a/squadre/atalanta)
- **[VERIFICATO]** La pagina partita Atalanta-Inter (12ª giornata) ha un timeline con etichette «Calcio d'inizio», «Primo tempo», «Match terminato» e tipi di evento `Gol segnato`, `Gol subito`, `Rigore segnato`, `Ammonizione`, `Subentrato`. Non compaiono tiri, passaggi, parate, angoli, duelli o coordinate — [Source](https://www.fantacalcio.it/serie-a/calendario/12/2026-27/atalanta-inter/18065)
- **[VERIFICATO]** Il regolamento ufficiale Fantacalcio definisce l'assist come «un passaggio che contribuisce alla realizzazione di un gol», richiede la «volontarietà», tratta le deviazioni con regole proprie (2a–2e) e classifica gli assist in `Soft`, `Standard`, `Gold` (Quality Assist) — [Source](https://www.fantacalcio.it/regolamenti/assist)

### Inferences
- **[DEDUZIONE]** Le colonne `Gol`, `Rig`, `Ass`, `Amm`, `Esp` sono **conteggi di fatti di partita**, ma attribuiti secondo regole Fantacalcio; `MV` e `FM` sono invece quantità **derivate e ponderate dal fantacalcio**. `GS` e `RP` sono conteggi derivati da eventi, non event-level.
- **[DEDUZIONE]** «Statistico» non risolve il problema: è una redazione che cambia le valutazioni, non un feed di statistiche reali. Usarlo come se fossero metriche Wyscout sarebbe un errore di semantica.
- **[DEDUZIONE]** Con LUNA e partita vera: i conteggi dicono *quale atto è stato registrato*, non *come* il giocatore ha letto la scena; Fantacalcio non dà percezione, posizioni, scelta o latenza. Le regole di assist sono una convenzione editoriale, non una misura dell'atto.
- **[DEDUZIONE]** Per Calcio Vero la fonte copre al massimo 4–5 delle 22 metriche (gol, rigori, assist, cartellini, parate/rigori parati solo per i portieri), e con definizioni proprie.

### Gaps
- Fantacalcio non documenta il calcolo di `MV`/`FM`, la soglia per «partita a voto», né le regole di conteggio di `Rig`, `RP`, `Amm`, `Esp` nella pagina Statistiche; ho trovato solo i `title` delle colonne.
- Non è pubblicata una tabella di mappatura «Fantacalcio → Wyscout 2017-18»; l'assist è documentato, ma con regole Fantacalcio.
- Non verificabile se i conteggi dei gol includano autogol o rigori in modo diverso da Wyscout: la pagina non lo dichiara.

## 2. Sono aggiornate all'ultima giornata? A che granularità?

### Takeaway
**[VERIFICATO]** Sì, al momento del fetch i dati sono alla **12ª giornata**, che è l'ultima completata: la 13ª giornata era in programma il 27/11 alle 20:45. **[VERIFICATO]** Però la pagina Statistiche è **cumulativa di stagione**, non per giornata e non per partita; il dettaglio partita è solo un tabellino di eventi sintetici.

### Cited Findings
- **[VERIFICATO]** La homepage del 27/11/2026 mostra «Marcia di avvicinamento alle gare della 13a giornata» e l'evento «Serie A 2026-27 - 13° giornata - venezia-bologna» alle `27/11 20:45` — [Source](https://www.fantacalcio.it/)
- **[VERIFICATO]** «Cronache ultima giornata» rimanda al calendario della giornata 12; la pagina dei voti ha titolo «Voti Fantacalcio Serie A 12 giornata - stagione 2026/27» — [Source](https://www.fantacalcio.it/serie-a/cronache-e-tabellini); [Source](https://www.fantacalcio.it/voti-fantacalcio-serie-a)
- **[VERIFICATO]** Nella tabella Statistiche il massimo di `PV` è 12 (69 calciatori); non c'è una data di «ultimo aggiornamento» nella pagina — [Source](https://www.fantacalcio.it/statistiche-serie-a)
- **[VERIFICATO]** La tabella è per **calciatore** con la squadra; la pagina squadra dà alcuni aggregati di **squadra**; la pagina partita dà una lista di **eventi con minuto**, non statistiche per squadra — [Source](https://www.fantacalcio.it/statistiche-serie-a); [Source](https://www.fantacalcio.it/serie-a/squadre/atalanta); [Source](https://www.fantacalcio.it/serie-a/calendario/12/2026-27/atalanta-inter/18065)

### Inferences
- **[DEDUZIONE]** L'aggiornamento «all'ultima giornata» è verificabile solo osservando che `PV` arriva a 12; non è una garanzia contrattuale di freschezza, perché la pagina non dichiara orari o ritardi di pubblicazione.
- **[DEDUZIONE]** Per un confronto con Wyscout serve una fonte con evento per partita; Fantacalcio dà solo il minuto e l'etichetta dell'atto nel tabellino, senza posizioni, senza sequenza completa e senza ripartizione per tempo.

### Gaps
- Orari di pubblicazione, ritardi e policy di correzione dei dati non sono dichiarati nelle pagine verificate.
- Non è chiaro se e quando la tabella venga aggiornata durante una giornata in corso: al fetch la 13ª non era ancora iniziata.

## 3. Estrazione tecnica e legalità per un gioco distribuito

### Takeaway
**[VERIFICATO]** L'HTML è server-rendered e leggibile, ma **non c'è un'API pubblica utilizzabile in anonimo**: i link Excel presenti nelle pagine rispondono `401`, e `/api`, `/api/v1`, `/rss` sono soft-404. **[VERIFICATO]** I Termini d'Uso vietano esplicitamente scraping e copia/riproduzione/elaborazione/divulgazione dei «Contenuti», che includono «statistiche, tabelle», senza autorizzazione scritta ed espressa. **[DEDUZIONE]** Per un gioco distribuito serve una licenza scritta; non ho trovato listino o contratto dati pubblico.

### Cited Findings
- **[VERIFICATO]** La pagina Statistiche contiene il link `/api/v1/Excel/stats/21/1`; la pagina Voti contiene `/api/v1/Excel/votes/21/12`. Con richiesta anonima, con cookie di sessione e con header browser-like, entrambi rispondono `401` e corpo vuoto — [Source](https://www.fantacalcio.it/statistiche-serie-a); [Source](https://www.fantacalcio.it/api/v1/Excel/stats/21/1); [Source](https://www.fantacalcio.it/api/v1/Excel/votes/21/12)
- **[VERIFICATO]** `/api`, `/api/v1` e `/rss` reindirizzano a `/404` — [Source](https://www.fantacalcio.it/api); [Source](https://www.fantacalcio.it/rss)
- **[VERIFICATO]** `robots.txt` consente la navigazione tranne alcune sezioni (`/ricerca`, `/probabiliformazioniseriea`, `/homesheet`, `/test/`); non dichiara un feed dati pubblico — [Source](https://www.fantacalcio.it/robots.txt)
- **[VERIFICATO]** Termini d'Uso, «Versione Settembre 2026», sezione **NAVIGAZIONE**: «La Navigazione nell'Ecosistema è consentita liberamente a chiunque.» Subito dopo, però: «Nell'accedere e utilizzare i Servizi dell'Ecosistema – inclusi la visualizzazione delle pagine [...] e il download dei contenuti (come articoli di approfondimento, articoli di analisi e valutazione, editoriali, **statistiche, tabelle**, vignette, – congiuntamente, i "Contenuti") – l'Utente deve rispettare le normative vigente nonché i diritti di Fantacalcio così come quelli dei terzi». E ancora: «Fantacalcio si riserva di perseguire qualsiasi uso non autorizzato dell'Ecosistema e dei suoi Contenuti» — [Source](https://www.fantacalcio.it/termini-e-condizioni)
- **[VERIFICATO]** Sempre nella sezione NAVIGAZIONE: «usare programmi software o altri meccanismi automatici o manuali per copiare o accedere alle pagine della Piattaforma o al loro Contenuto (ivi compresi sistemi atti a effettuare il c.d. scraping) senza una autorizzazione scritta ed espressa di Fantacalcio o del terzo titolare»; e «copiare, riprodurre, alterare, modificare, elaborare, vendere o comunque divulgare i Contenuti senza espressa autorizzazione scritta di Fantacalcio o del terzo titolare dei diritti di proprietà intellettuale [...]. Resta inteso che tali divieti si estendono anche a qualsiasi elaborazione dei dati (sia essa lecita - ossia precedentemente espressamente autorizzata dalla Società, o illecita)» — [Source](https://www.fantacalcio.it/termini-e-condizioni)
- **[VERIFICATO]** L'informativa privacy è «Versione ottobre 2026» e riguarda i dati personali dell'utente («Informativa sul trattamento dei dati personali - Ecosistema Fantacalcio»), non una licenza sui dati sportivi; gli indirizzi ivi pubblicati sono `privacy@fantacalcio.it` e `dpo@fantacalcio.it` — [Source](https://www.fantacalcio.it/privacy)
- **[VERIFICATO]** Il footer dichiara «© Fantacalcio S.R.L. P.IVA 10938501219 - Tutti i diritti riservati» e «Fantacalcio è un marchio registrato da Fantacalcio S.r.l.» — [Source](https://www.fantacalcio.it/)
- **[VERIFICATO]** `https://www.fantacalcio.com/` reindirizza in modo permanente (`301`) a `https://www.fantacalcio.it/`; non è una fonte distinta — [Source](https://www.fantacalcio.com/)
- **[VERIFICATO]** Non ho trovato un «Fantacalcio ufficiale» separato di Lega Serie A/Gazzetta tra gli URL verificati: `fantacalcio.gazzetta.it` non risolve, `gazzetta.it/Fantacalcio` risponde 404, `legaseriea.it/it/fantacalcio` reindirizza alla home della Lega — [Source](https://fantacalcio.gazzetta.it/); [Source](https://www.gazzetta.it/Fantacalcio/); [Source](https://www.legaseriea.it/it/fantacalcio)

### Inferences
- **[DEDUZIONE]** Anche se l'HTML è leggibile, un crawler che estrae e conserva le tabelle rientra nella fattispecie «meccanismi automatici [...] per copiare o accedere»: serve autorizzazione scritta. Trascrivere i numeri in un gioco distribuito rientra in «elaborare [...] o comunque divulgare i Contenuti».
- **[DEDUZIONE]** Il link Excel non è un'API pubblica: è un endpoint protetto, probabilmente legato a un account; la sua presenza nell'HTML non costituisce licenza d'uso.
- **[DEDUZIONE]** Il costo economico non è determinabile dalle pagine pubbliche: non c'è listino, contratto di data licensing o documentazione API. Va negoziato, oppure la fonte va scartata.
- **[DEDUZIONE]** La privacy policy non copre l'uso dei dati sportivi in un prodotto terzo: citarla come «permesso» sarebbe un errore.

### Gaps
- Non verificato se un account Fantacalcio abiliti l'export Excel e a quali condizioni; test solo anonimo.
- Non trovata alcuna pagina di licensing/commerciale dati o prezzo.
- Non chiarito chi sia il «terzo titolare» citato nei Termini (Lega, provider statistico o altri); la clausola non lo specifica.
- Non trovata una documentazione API/JSON/RSS pubblica.

## 4. Confrontabilità con Wyscout e alternative pratiche

### Takeaway
**[VERIFICATO]** Le definizioni Wyscout sono specifiche e documentate; quelle Fantacalcio non lo sono, tranne l'assist che ha una regola editoriale propria. **[DEDUZIONE]** Fantacalcio non è sostituibile a Wyscout 2017-18 per Calcio Vero: manca quasi tutta la granularità e la semantica non è allineata. L'alternativa gratuita più immediata per il 2026-27 resta il CSV aggregato di football-data.co.uk, con 7 metriche.

### Cited Findings
- **[VERIFICATO]** Wyscout documenta eventId/subEventId/tag e regole di conteggio (es. `Pass` eventId 8/subEventId 85, `Successful` = tag 1801, esclusione di goal kick/corner/throw-in dalle statistiche passaggi) — [Source](https://dataglossary.wyscout.com/pass/)
- **[VERIFICATO]** Fantacalcio documenta l'assist con volontarietà, deviazioni e qualità (`Soft`/`Standard`/`Gold`), cioè una regola di redazione — [Source](https://www.fantacalcio.it/regolamenti/assist)
- **[VERIFICATO]** football-data.co.uk pubblica `mmz4281/2627/I1.csv` con `HS, AS, HST, AST, HF, AF, HC, AC, HY, AY, HR, AR` oltre a gol e xG; al 27/11/2026 ha 120 righe, ultima partita `23/11/2026 Monza-Fiorentina` — [Source](https://www.football-data.co.uk/mmz4281/2627/I1.csv); [Source](https://www.football-data.co.uk/data.php)
- **[VERIFICATO]** StatsBomb open data distribuisce JSON event-level con attribuzione richiesta, ma non contiene la Serie A maschile 2026-27; la Serie A maschile presente è 2015/16 e 1986/87 — [Source](https://github.com/hudl/open-data); [Source](https://raw.githubusercontent.com/statsbomb/open-data/master/data/competitions.json)
- **[VERIFICATO]** openfootball/football.json ha la cartella 2026-27 con solo risultati e licenza CC0 1.0 — [Source](https://raw.githubusercontent.com/openfootball/football.json/master/2026-27/it.1.json); [Source](https://raw.githubusercontent.com/openfootball/football.json/master/LICENSE.md)

### Inferences
- **[DEDUZIONE]** Fantacalcio può essere usato al più come **controllo editoriale** su gol/assist/cartellini, con una DD esplicita: non può tarare le 22 metriche di `ds-reference` e non fornisce gli atti mancanti (tiri, passaggi, parate, duelli, fuorigioco, spazzate, rimesse, recupero per tempo).
- **[DEDUZIONE]** La strada che mantiene lo schema del referto attuale resta Wyscout API con licenza; il CSV football-data.co.uk è un confronto parziale su 7 metriche, con definizioni da verificare una per una e senza tarature guardando la metrica (A5-bis).
- **[DEDUZIONE]** Usare Fantacalcio in un gioco distribuito senza autorizzazione scritta è un rischio legale concreto, non solo tecnico: i Termini vietano sia l'acquisizione automatica sia l'elaborazione/divulgazione.

### Gaps
- Nessuna mappatura ufficiale Fantacalcio ↔ Wyscout per le metriche comuni.
- Nessuna licenza d'uso dati per un gioco distribuito trovata o negoziata.
- Non verificabile se i conteggi Fantacalcio siano corretti sugli eventi base: la fonte non pubblica definizioni né metodologia di verifica post-partita.
