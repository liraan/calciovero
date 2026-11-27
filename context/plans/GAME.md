# Il piano del gioco intero

**Cos'è.** La mappa di Calcio Vero dall'inizio alla 1.0 e oltre: i livelli, l'ordine di lavoro con le sue prove, il contenuto della 0.1, le regole di lavoro e dove sta ogni cosa. È la fonte: i doc fuori dalla repo servono solo per discutere (decisione E). Aggiornato al 16 novembre 2026.

## Il principio: prima completo, poi perfetto

**La priorità è completare il gioco** (maintainer, 16 novembre 2026). L'aderenza alla realtà è una tendenza e uno degli scopi del progetto, non l'unico: la perfezione non esiste. Le correzioni arrivano dopo, a gioco intero, con i betatester. Restano ferme le regole che fanno il gioco: niente dadi, ogni scelta del giocatore, le statistiche ricavate e mai ritoccate (RULES A5, A5-bis).

## I livelli

| Livello | Cos'è | Stato | Piano |
|---|---|---|---|
| L0 · La partita | Ventidue giocatori che pensano, il pallone con la fisica, le Leggi, la panchina | quasi chiuso, in taratura; si riprende con il 2.5D (decisione S) | [`L0-CLOSE`](L0-CLOSE.md), [`L0-CALIBRATION`](L0-CALIBRATION.md), [`L0-AUDIT`](L0-AUDIT.md) |
| L0 · Plugin | La partita divisa in plugin dietro un contratto (ADR-063) | fatto | [`PLUGINS`](PLUGINS.md) |
| Vista 2.5D | La lavagna inclinata, la vista della GUI su desktop e Android | adesso, prima della chiusura di L0 (decisione S) | [`DESIGN`](GUI-DESIGN.md) |
| L1 · La stagione | Tre serie e la coppa nel formato reale, ogni partita dal motore | dopo il 2.5D | [`L1`](L1.md) |
| L2 · La carriera | Procuratori, osservatori, vivaio, crescita, bilancio completo, società, spogliatoio, la vita dell'allenatore (contratti e mercato sono in L1, decisione AR) | dopo la 0.1 | ADR-060; il piano in [`L2.md`](L2.md), un blocco alla volta |
| L3 · Il gioco che si modifica | Editor (il giocatore nasce dai suoi dati), pacchetti, importatore locale, plugin esterni in WebAssembly, accessibilità, tutorial | dopo L2 | [`L3.md`](L3.md); decisioni 4, F–K, BE |
| 3D | Solo desktop, con Bevy se maturo | dopo la 1.0 | [`DESIGN`](GUI-DESIGN.md) |

## L'ordine di lavoro

Ogni passo si chiude con la sua prova, e il successivo non parte prima.

1. **La vista 2.5D** (sezione "Il lavoro del 2.5D" di `DESIGN.md`; prima della chiusura di L0, decisione S: le metriche più lontane dal vero sono in altezza, e il 2.5D serve a vederne le cause). Prova: la finestra mostra la partita inclinata, con le figurine, il pallone con l'ombra e l'interpolazione alla frequenza dello schermo; il golden identico; il README con l'animazione del 2.5D (WebP a 60 fotogrammi, o MP4 se GitHub non la anima).
2. **La panchina in partita** (decisione T, dentro L0, è completezza): i cambi decisi dal vivo dalla GUI; ADR-059 passo 3 (istruzioni rapide, transizioni, cambio di modulo, istruzioni individuali); l'allenatore avversario che legge la partita, con il momento in cui reagisce fissato come grado 2 sui cambi veri del riferimento Wyscout; il colpo che costringe a un cambio (OI-110); con le decisioni del 16 novembre: lo stato di partenza di ogni giocatore come input (AB), i disegni liberi al posto della griglia del 4-4-2 e la lavagna che si cambia in partita (AL, AG), l'intervallo, i richiami e le pause idriche come input (AM, AO), l'ADR dei canali dalla panchina che sostituisce ADR-059 §2 (AP); i passi piccoli, si pensa a 10 tick al secondo e ci si muove a 50, con il loro ADR, prima di finire la taratura (BJ). Prova: un cambio e un'istruzione dati dal vivo si rigiocano identici dal replay; i test gemelli dell'allenatore avversario (sotto di due gol all'85' contro in vantaggio); il golden su glibc e musl.
3. **Chiudere L0**, con il 2.5D come strumento: prima OI-102 (il gioco aereo), poi i tiri da vicino (OI-100), poi il resto. Prova: almeno **22 metriche su 22 in banda**, e ogni metrica fuori banda con la sua causa scritta e il suo OI (decisione L). Poi il golden su glibc, musl e Windows, la durata di una partita in `STATE.md` e la partita di prova preparata per il maintainer.
4. **L1, i dieci passi di [`L1.md`](L1.md).** Prova: dieci stagioni con lo stesso hash, il formato reale, i plugin spenti.
5. **La versione 0.1 · Stagione.** Prova: la lista congelata qui sotto, completa.
6. **La 0.2 · Mercato e panchina:** il mercato di [`L1-MARKET.md`](L1-MARKET.md) (contratti, cassa, finestre, trasferimenti, prestiti, il posto di direttore sportivo, decisione AR) e i moduli consigliati dell'allenatore di [`L1-COACH.md`](L1-COACH.md) (canali, discorsi, piani "se, allora", riunione tattica, leader, decisione AQ) e le **conferenze stampa** (anticipate dalla 1.1, maintainer, 2026-11-24: lo stesso meccanismo a pezzi dei discorsi, `L2.md` decisione BC), uno alla volta, ognuno spento senza rompere nulla. Prova: dieci anni di mercato con lo stesso hash; ogni modulo acceso e spento lascia il golden deterministico.
7. **L2, prima metà** ([`L2.md`](L2.md)): la crescita, lo staff, il mercato che cresce (procuratori, contratti moderni, il resto del mondo, i campionati stranieri come moduli), la società e il bilancio; il posto giocabile di presidente (decisioni AK, BD). Prova: dieci anni di crescita, mercato e bilanci deterministici. Poi la **0.3 · Società**.
8. **L2, seconda metà** ([`L2.md`](L2.md)): il vivaio e le accademie, le licenze, la carriera, lo spogliatoio, le strutture, i posti giocabili dello staff. Prova: una carriera di dieci anni con lo stesso hash. Poi la **0.4 · Carriera**.
9. **La vita dell'allenatore**, modulo opzionale dopo la 0.4 ([`L2.md`](L2.md), decisione BA): la casa, i soldi personali, le persone.
10. **L3.** Prova: un dato modificato dall'editor e un pacchetto esterno lasciano la partita deterministica. Poi la **1.0** sul desktop.
11. **Android**, poi la **1.1** (tifosi e il resto della stampa: voci, pagelle; le conferenze sono nella 0.2), poi il **3D** sul desktop, se Bevy è maturo.

## La 0.1 · Stagione, contenuto congelato

Decisione N: quello che non è in questa lista va dopo la 0.1, salvo decisione esplicita del maintainer.

- [ ] La vista 2.5D nella GUI (passo 1)
- [ ] La panchina in partita: cambi dal vivo, istruzioni rapide, l'allenatore avversario che legge la partita (passo 2)
- [ ] L0 chiuso (passo 2)
- [ ] L1 completo: tre serie, la coppa, il calendario, le classifiche, l'allenatore, il corpo, la disciplina, la settimana
- [ ] Le fondamenta del 16 novembre (decisione AQ): il modello di attore per ogni ruolo (AH), i ruoli in dati con il posto di allenatore (AI, AK), il database (AD, AE), la lavagna libera che si cambia in partita (AL, AG)
- [ ] La GUI della stagione: la lavagna, i post-it a linguette, il calendario con l'agenda, la rosa, la classifica, la scheda giocatore, la partita
- [ ] Il file di segnalazione (seme, scelte datate, versione del motore) con il pulsante "segnala un problema"; il registro locale degli errori, senza invii
- [ ] I binari per Windows, Linux e macOS
- [ ] Il golden verificato anche su ARM64 nella CI (decisione P)
- [ ] **Compito del maintainer:** un legale rivede le decisioni F–K su dati reali e asset (decisione Q). Promemoria quando L1 arriva al passo 9.

## Le versioni (decisione BF)

Ogni versione: cosa porta, la prova che la chiude, a chi va, cosa serve prima di uscire. Il contenuto di dettaglio è nei piani dei livelli; questa tabella è la fonte delle versioni (decisione E: niente `RELEASES.md` a parte).

| Versione | Cosa porta | La prova | A chi | Prima di uscire |
|---|---|---|---|---|
| **0.1 · Stagione** | la stagione intera, la lavagna libera, le fondamenta (attore, ruoli, database), la 2.5D; la lista congelata qui sopra | dieci stagioni con lo stesso hash; L0 chiuso (decisione L); la stagione si gioca dalla GUI | pochi tester | la revisione legale delle decisioni F–K (decisione Q); il golden su glibc, musl e ARM64 (decisione P); il file di segnalazione |
| **0.2 · Mercato e panchina** | il mercato di [`L1-MARKET.md`](L1-MARKET.md), il direttore sportivo giocabile, i moduli consigliati dell'allenatore di [`L1-COACH.md`](L1-COACH.md), le conferenze stampa | dieci anni di mercato con lo stesso hash; ogni modulo acceso e spento lascia il golden deterministico | i tester | l'ADR che modifica l'ADR-060 (decisione AR) |
| **0.3 · Società** | la crescita, lo staff, il mercato che cresce, i campionati stranieri come moduli, la società e il bilancio, il presidente giocabile ([`L2.md`](L2.md)) | dieci anni di crescita, mercato e bilanci con lo stesso hash; le curve dell'età tarate (decisione AS) | i tester | le regole economiche vere come dati, con le fonti in `SOURCES.md` (decisione AY) |
| **0.4 · Carriera** | il vivaio e le accademie, le licenze, la carriera, lo spogliatoio, le strutture, i posti giocabili dello staff ([`L2.md`](L2.md)) | una carriera di dieci anni con lo stesso hash | tutti | il costo dei moduli misurato su un computer modesto (decisione AX) |
| **La vita dell'allenatore** | moduli opzionali: la vita dell'allenatore, le vite private dei giocatori (decisione BA) | lo stesso hash con i moduli accesi e spenti | tutti | — |
| **1.0** | L3: l'editor (il giocatore nasce dalle sue statistiche), i pacchetti, l'importatore, i plugin esterni, l'accessibilità, il tutorial, la cura ([`L3.md`](L3.md)) | un dato modificato dall'editor e un pacchetto esterno lasciano la partita deterministica; il ritorno dei dati; ogni schermata da tastiera e dal lettore di schermo | tutti, sul desktop | la revisione legale di pacchetti e importatore (decisioni G–K); la documentazione dei formati |
| **1.1** | Android (ADR-061), poi tifosi e stampa (decisione BC; le conferenze già nella 0.2) | lo stesso golden su Android; dieci anni di tifosi e stampa con lo stesso hash | tutti, anche sul telefono | i profili leggeri dei moduli misurati sul telefono |
| **Dopo** | il 3D sul desktop, se Bevy è maturo | — | — | — |
| **Dopo il 3D** | la carriera da calciatore, con il pad o la tastiera, solo desktop (decisione BI, [`L2.md`](L2.md)), con molta calma | lo stesso hash con e senza un umano in campo (i passi piccoli sono già in L0, decisione BJ) | tutti, sul desktop | — |

**Come esce una versione:** il tag `v*` lo mette il maintainer (un agente non può pushare tag su GitHub); il workflow costruisce i binari per Linux, Windows e macOS; su Codeberg la release si crea con la sua API; il registro delle modifiche e la tabella del README si aggiornano nello stesso giro.

## Le regole di lavoro

- **Un solo scrittore su main** (decisione O, RULES C1): main lo scrive solo Claude Code; chi lavora in parallelo usa un ramo e gli lascia la riga di merge.
- **Le decisioni delegate** (DD) restano da rivedere senza un tetto (decisione M): il maintainer le rivede quando vuole; ognuna si annulla con un revert.
- **Il mandato di autonomia** del 16 novembre: Code continua da solo dentro RULES e VISION, con la LUNA e la partita vera per ogni decisione (`STATE.md`, "Mandato attivo").

## Dove sta cosa

| Cosa | Dove |
|---|---|
| Il segnalibro: dove siamo, il prossimo passo, gli OI | [`context/STATE.md`](../STATE.md) |
| Le regole di lavoro | [`context/RULES.md`](../RULES.md) |
| La visione e i fondamenti | [`doctrine/VISION.md`](../../doctrine/VISION.md), [`doctrine/00-foundations.md`](../../doctrine/00-foundations.md) |
| Le decisioni prese con il maintainer, 15 e 16 novembre | [`DECISIONS-2026-11-15.md`](DECISIONS-2026-11-15.md) |
| Le decisioni di forma del modello | [`doctrine/adr/`](../../doctrine/adr/) |
| L0: chiusura, taratura, ricontrollo, confronto con il piano ideale, plugin | [`L0-CLOSE`](L0-CLOSE.md), [`L0-CALIBRATION`](L0-CALIBRATION.md), [`L0-AUDIT`](L0-AUDIT.md), [`L0-IDEAL-COMPARISON`](L0-IDEAL-COMPARISON.md), [`PLUGINS`](PLUGINS.md) |
| L1 | [`L1.md`](L1.md) |
| La GUI, le viste, la scheda giocatore, Android | [`context/plans/GUI-DESIGN.md`](GUI-DESIGN.md) |
| Le fonti di ogni numero | [`L0-match/balance/SOURCES.md`](../../L0-match/balance/SOURCES.md) |
