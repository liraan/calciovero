# Decisioni del 15 novembre 2026 (piano del gioco intero)

Prese dal maintainer analizzando le fragilità del piano del gioco intero (doc: https://claude.ai/artifact/1Hf5mWgJUiJt2kCHZPvob7). La fonte da qui in poi è questo file; vanno scritte negli ADR indicati.

| # | Decisione | Dove va |
|---|---|---|
| 1 | Golden: a ogni commit un mondo piccolo (2 serie da 6 squadre, 2 stagioni); il golden pieno di dieci stagioni di notte e prima di ogni merge in main | ADR del contratto di stagione |
| 2 | Contratti in L1: alla scadenza si rinnovano da soli alle stesse condizioni; la scadenza resta un fatto visibile | ADR del contratto di stagione |
| 3 | Salvataggi: il salvataggio tiene lo stato; dalla partita successiva gioca il motore nuovo e il gioco lo dice; il replay identico vale dentro una `SIM_VERSION` | ADR del contratto di stagione |
| 4 | Plugin: gli ufficiali compilati dentro il gioco, gli esterni solo WebAssembly (isolati, deterministici, NaN canonicalizzati, carburante); un contratto solo. Ufficiale solo dopo la nostra revisione, il validatore e la prova di determinismo; un non ufficiale si carica col consenso dell'utente e il manifesto lo dichiara | emenda ADR-012, ADR-061 |
| 5 | Fonti: tre gradi in `_source` (1 fonte diretta, 2 misurato sull'effetto, 3 arbitrario dichiarato con motivo e test gemello sul verso); un grado 3 non decide da solo un risultato importante; registro visibile, conteggio in `STATE.md`; in L0 niente di nuovo di grado 3 | emenda RULES A5-bis |
| 6 | Ricaricare: niente di punitivo; le preferenze delle persone sono stato nascosto (latenti dal seme, coerenti, stimati dallo staff), mai un dado travestito; ogni risposta ha i suoi tempi | ADR prima del mercato (L2) |
| 7 | Betatester: 0.1 Stagione a L1 finito (pochi, scelti dal maintainer), 0.2 Mercato, 0.3 Carriera, poi 1.0; in L1 il file di segnalazione (seme, scelte datate, versione del motore), registro locale degli errori senza invii, binari Windows/Linux/macOS | piano L1, passo 10 |
| A | Fasi del tick: otto, con "decidere senza palla" prima di "decidere con palla" (il portatore vede i movimenti appena decisi, come oggi nel motore 112). Prima di chiudere L0 la scena passa solo dal cono e dalla memoria (oggi `rank`, `loose_rival_cm`, `cutters` leggono il campo vero): un OI, un test anti-fuga sulla scena, un `SIM_VERSION`. Poi la divisione in plugin a golden identico | OI nuovo; ADR della divisione (OI-94) |
| B | Plugin spenti: letture facoltative dichiarate nella scheda e controllate dal validatore; L1 tutte le 8 combinazioni sul mondo piccolo; L2 acceso, spento, uno alla volta, più le coppie dichiarate, di notte | ADR del contratto di stagione |
| C | Serie minori: un solo motore, le serie differiscono solo per i giocatori; B e C confrontate sui dati di stagione pubblici, citati | `SOURCES.md` |
| D | Allenatore avversario: piano suo, `context/plans/L1-COACH.md`, all'inizio del passo 5 di L1 | piano L1 |
| E | La fonte dei piani è la repo; i doc rimandano a `context/plans/` | — |
