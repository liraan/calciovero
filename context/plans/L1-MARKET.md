# L1 · Il mercato (`ds-contracts`, `ds-market`, `ds-budget`)

Il piano del mercato di L1: contratti, trasferimenti, prestiti e il minimo di soldi che li regge. Decisione AR (maintainer, 16 novembre 2026): il mercato si fa subito, dentro L1 e dentro la 0.1. Con i filtri partita vera, LUNA e futuribilità; come per l'allenatore, si scrive adesso e si prova giocando (decisione M).

Il quadro è in [`L1.md`](L1.md); gli attori e i ruoli in [`L1-COACH.md`](L1-COACH.md) (decisioni AH, AI).

## Il principio

Nessun prezzo è scritto da qualche parte. Ogni club ha un direttore sportivo attore, ogni giocatore è un attore: il prezzo nasce dalla trattativa fra chi compra e chi vende, e il trasferimento si fa solo se anche il giocatore lo accetta. Tutti scelgono con le loro caratteristiche, sulle stime, senza dadi. L'ADR-060 metteva il mercato nella carriera: serve un ADR che lo modifichi, prima del passo del mercato.

## Cosa entra

1. **I contratti** (`ds-contracts`): durata, ingaggio, scadenza, rinnovo, la clausola di rescissione dove c'è. Il rinnovo è una trattativa come un acquisto. A scadenza il giocatore è libero (parametro zero).
2. **Le finestre** come regole modulari con la loro stagione di validità: l'estiva e l'invernale, con le loro date; la lista dei giocatori si aggiorna a ogni finestra, con le regole della lista come dati.
3. **I trasferimenti** (`ds-market`): acquisto a titolo definitivo, cessione, scambio; **i prestiti**, secco, con diritto o con obbligo di riscatto (e l'obbligo che scatta a una condizione, come succede in Italia); i parametri zero.
4. **Il minimo di soldi** (`ds-budget`): ogni club ha una cassa, all'inizio dal suo livello (grado 3 in L1, dichiarato), un budget per gli acquisti e uno per gli stipendi. Le cessioni liberano budget; gli stipendi si pagano ogni mese. Il bilancio vero (incassi, diritti, sponsor, premi) è L2.

## Gli attori del mercato

**Il direttore sportivo** è un attore, costruito come l'allenatore (decisione AH). L'albero è una proposta da correggere ramo per ramo:

| Famiglia | Caratteristiche |
|---|---|
| **Valutazione** | l'occhio per il giocatore (la larghezza delle sue stime), la lettura del mercato (quanto sa cosa vale un giocatore per gli altri) |
| **Trattativa** | la fermezza, la pazienza, la capacità di chiudere |
| **Progetto** | quanto compra per l'idea del suo allenatore, quanto per il futuro, quanto per l'occasione |
| **Carattere** | la propensione al rischio, la fedeltà ai giocatori del club |

Le abilità (formule): valutare un giocatore, trattare un acquisto, trattare una cessione, trattare un rinnovo, costruire la rosa.

**Il giocatore** decide con le sue caratteristiche: l'ambizione, il legame con il club e con la città, la voglia di giocare, quanto pesa l'ingaggio. Accetta, rifiuta o chiede di più; sceglie fra le offerte che ha. Il procuratore come attore è L2: in L1 il giocatore tratta per sé.

**L'allenatore** dice al direttore sportivo cosa gli serve: il ruolo, il profilo, un nome. Il direttore sportivo decide con le sue caratteristiche e la cassa.

## Il valore è una stima

Ognuno si fa la sua stima di quanto vale un giocatore, da quello che vede: il rendimento nelle partite (piano B), l'età, il contratto, il ruolo, chi altro lo vuole. Nessuno legge il piano A. Le stime sui giocatori degli altri club sono larghe (decisione 4), quanto dice l'occhio del direttore sportivo: in L1 si compra un po' alla cieca, come prima dei dati; gli osservatori come persone, che stringono le stime, sono L2. Sui giocatori che hanno giocato contro di te, la stima è più stretta: li hai visti.

## La trattativa, senza dadi

Una trattativa è una serie di offerte e risposte datate, una al giorno nei momenti della giornata: l'offerta, la controfferta, l'accordo o la rottura. Ogni risposta viene dalle caratteristiche di chi risponde e dalla sua stima. Il risultato è deterministico: lo stesso mondo, le stesse offerte negli stessi giorni danno la stessa trattativa. Si tratta con il club e poi con il giocatore; il trasferimento si chiude solo con tutti e due.

## Chi lo fa: il posto giocabile

Il posto di direttore sportivo è giocabile già in L1 (aggiorna la decisione AK). Due modi di giocare il mercato:

- **da allenatore:** chiedi al direttore sportivo attore, che decide lui;
- **da direttore sportivo:** fai tu le trattative, con la cassa che ti dà la società; l'allenatore attore ti dice cosa gli serve, e se gli compri un giocatore che non vuole, lo legge con le sue caratteristiche.

Gli altri club hanno tutti il loro direttore sportivo attore.

## I test

- **Gemelli:** due giocatori uguali tranne il legame con il club, la stessa offerta, e quello più legato resta; due direttori sportivi uguali tranne la fermezza, e il più fermo vende a più; lo stesso giocatore con un contratto lungo e uno in scadenza, e quello in scadenza costa meno.
- **Determinismo:** dieci anni di mercato con lo stesso hash; il mercato spento lascia la stagione di oggi (rose fisse, rinnovi automatici).
- **Le regole:** nessun acquisto fuori finestra, nessuna lista fuori regola, nessun club sotto zero oltre quello che le regole permettono.
- **Taratura (grado 2):** quanti trasferimenti per finestra, quanti prestiti, la distribuzione dei prezzi e degli ingaggi per livello del club, contro le stagioni vere di riferimento.

## Fuori da L1

I procuratori come attori; gli osservatori come persone; il bilancio completo (incassi, diritti, sponsor, premi), il presidente giocabile; le giovanili e i giocatori del vivaio; il fair play finanziario e le regole economiche delle leghe. Tutto L2.

## Domande aperte

- [ ] Le regole vere della lista (quanti giocatori, quanti cresciuti nel club e nel paese) e delle finestre: dai regolamenti ufficiali, citati in `SOURCES.md`, prima del passo del mercato.
- [ ] Le fonti per la taratura: i trasferimenti veri della stagione di riferimento (Wikidata per i movimenti, decisione H); per prezzi e ingaggi, una fonte con licenza pulita, da trovare.
- [ ] La cassa iniziale per livello: grado 3 dichiarato in L1, con il suo registro.
