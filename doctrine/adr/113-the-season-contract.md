# ADR-113 — Il contratto della stagione: il nucleo, i cinque momenti, la scheda condivisa (DD-64)

- **Status:** Accepted (delegated, 2026-11-24) — da rivedere da parte del maintainer
- **Date:** 2026-11-24
- **Decision:** DD-64
- **Attua:** `context/plans/L1.md`, passo 1 ("L'ADR del contratto di stagione"); decisioni 1, 2, 3, 4, 6, B, J, X, AC, AD, AQ di `DECISIONS-2026-11-15.md`
- **Depends on:** ADR-003 (piano A), ADR-012 (cronaca), ADR-013 (manifesto), ADR-029 (seme), ADR-035 (niente dadi), ADR-041 (un crate per responsabilità), ADR-055 (view model), ADR-059, ADR-060, ADR-063 (contratto dei plugin della partita), ADR-065 (registrazione)

## Il problema

L1 è una stagione intera giocata dal motore di L0. `L1.md` la descrive come un nucleo piccolo e tredici plugin; le decisioni del 16 novembre ne hanno fissato i pezzi (i plugin esterni solo in WebAssembly, le letture facoltative dichiarate, le fasce dei moduli, i risultati che arrivano con il tempo del gioco, il database SQLite, la scheda condivisa). Manca il contratto scritto che un plugin deve rispettare, e da cui partono il nucleo, il primo plugin e i test: senza, ogni plugin inventerebbe la sua forma, e la stagione non sarebbe né componibile né riproducibile.

## Decisione

1. **Il nucleo, `ds-season`, tiene solo il tempo, il registro, gli eventi e il salvataggio.** Non conosce il calcio. Avanza **un giorno alla volta** e in ogni giorno chiama i plugin in **cinque momenti fissi**, nell'ordine:
   - `DayStarted` (inizio giorno: recuperi, sedute, scadenze);
   - `BeforeMatches` (formazioni annunciate, squalifiche applicate);
   - `AfterMatch` (per ogni partita giocata quel giorno, nell'ordine dell'id della partita: risultato, cartellini, colpi);
   - `DayEnded` (classifiche, fine giornata);
   - `SeasonEnded` (promozioni, retrocessioni, rinnovi, età), una volta.
   È lo stesso disegno della partita (ADR-063): il giorno al posto del tick, i momenti al posto delle fasi.

2. **`ds-season-api` è il crate pubblico, gemello di `ds-match-api`.** Un plugin è un tipo che implementa
   ```rust
   pub trait SeasonPlugin {
       fn card(&self) -> PluginCard;
       fn run(&mut self, moment: Moment, world: &mut SeasonWorld<'_>) -> Result<(), PluginError>;
   }
   ```
   `SeasonWorld` è lo stato osservabile della stagione: la data, gli eventi già nati, le viste pubbliche degli altri plugin dichiarate nella scheda, la connessione al database limitata alle sue tabelle, e il canale per emettere eventi. Nessun campo pubblico dello stato: si legge e si scrive dalla porta (ADR-003, ADR-041).

3. **La scheda è una sola, in un crate condiviso** (`ds-plugin-api`, decisione AQ), usata da `ds-match-api` e da `ds-season-api`, con **un solo validatore**. Contiene: il nome (unico), la versione, i momenti o le fasi in cui agisce, le dipendenze, le letture (ognuna obbligatoria o **facoltativa con cosa fa senza**, decisione B), le scritture (le sue tabelle, con il suo prefisso), la **fascia** (decisione X: `Core`, `Recommended`, `Optional`; sostituisce `Weight::Core/Detail` della partita) e il **costo** dichiarato (tempo per partita o per giornata, per la GUI sul telefono). Il validatore rifiuta una lettura non dichiarata, una scrittura fuori dal prefisso, un ciclo di dipendenze, un plugin `Core` che dipende da uno spegnibile.

4. **L'ordine dei plugin è deciso dalle dipendenze, a parità dal nome**: un ordinamento topologico deterministico, lo stesso a ogni partenza. Spegnere un plugin `Recommended` o `Optional` non cambia l'ordine degli altri.

5. **I plugin si parlano solo con eventi e dati.** Gli eventi della stagione hanno nomi inglesi (ADR-007), i loro campi e la loro causa, e ognuno il suo testo di cronaca (ADR-012 §5-6): `DayStarted`, `LineupAnnounced`, `MatchPlayed`, `Injury`, `Recovered`, `Booking`, `Suspension`, `SuspensionServed`, `SessionHeld`, `StandingsUpdated`, `StageCompleted`, `Promoted`, `Relegated`, `Qualified`, `ContractRenewed`, `SeasonEnded`. Un plugin nuovo aggiunge i suoi eventi nel suo spazio di nomi, senza toccare il nucleo.

6. **Solo il plugin Giornata (`ds-matchday`) chiama il motore della partita**, dalla porta di `ds-match` (`simulate_from_seed`, e `simulate_from_seed_recorded` per le partite della nostra squadra). Nessun risultato si estrae dalla forza delle squadre. **Il seme di ogni partita** è una funzione pura del seme del mondo, della competizione, della giornata e della partita (ADR-029), in `ds-core`: la stessa partita ha lo stesso seme qualunque sia l'ordine in cui viene giocata.

7. **I risultati arrivano con il tempo del gioco** (decisione AC). Le partite di un giorno leggono lo stato com'è a `BeforeMatches` e non si leggono fra loro: si possono giocare **in parallelo e in anticipo** sull'orologio della GUI. Il loro effetto entra nella stagione solo in `AfterMatch`, nell'ordine dell'id della partita: il risultato non dipende da quanti core ci sono né da quando lo si guarda.

8. **Le versioni.** `SEASON_VERSION` (in `ds-season`) è separata da `SIM_VERSION`: si alza quando, a parità di seme e di scelte, la stagione esce diversa per un cambio di un plugin di stagione; un cambio del motore alza solo `SIM_VERSION`. Ogni bump ha la sua voce in `CHANGELOG-SEASON.md`.

9. **Il salvataggio** tiene lo stato del mondo, lo stato di ogni plugin sotto il suo nome, e il manifesto (ADR-013): plugin attivi con versione e fascia, `SIM_VERSION`, `SEASON_VERSION`, il seme, l'impronta del database del mondo da cui la carriera è partita (decisione J). Le partite della nostra squadra si salvano con la loro registrazione compressa in un file suo (ADR-065), citata per nome; delle altre restano risultato ed eventi. Con un motore nuovo la stagione continua dalla partita successiva e il gioco lo dice; le partite giocate restano com'erano (decisione 3). La scelta dei moduli si fa all'inizio della carriera e cambia solo fra due stagioni, come scelta datata nel manifesto (decisione X).

10. **Il database** (decisione AD) è SQLite incluso nel gioco, e **solo `ds-db` parla con SQLite**: oggi `ds-db` è il caveau del piano A, e lo resta (ADR-112); L1 lo allarga al mondo e alla stagione. Ogni plugin descrive le sue tabelle come dati nella sua scheda, con il prefisso del suo nome (`roster.*`, `discipline.*`); `ds-db` le crea, applica le migrazioni dichiarate e dà a ogni plugin una connessione che scrive solo nel suo prefisso e legge solo ciò che ha dichiarato (l'authorizer di SQLite). Le caratteristiche nascoste dei giocatori restano dietro la porta: un plugin di stagione chiede letture all'attore come il motore (ADR-112), mai le latenti.

11. **I plugin ufficiali sono compilati dentro il gioco; gli esterni solo WebAssembly** (decisione 4), deterministici, con i NaN canonicalizzati e un limite di carburante per giornata, dalla stessa porta e con la stessa scheda.

12. **Le prove** (`L1.md`, "Test, golden e gate"):
    - il **golden piccolo**: 2 serie da 6 squadre, 2 stagioni, lo stesso hash, a ogni commit (C2);
    - le **combinazioni dei plugin spegnibili** sul mondo piccolo, a ogni commit;
    - le **invarianti** (ognuno incontra ognuno, nessuno gioca due volte lo stesso giorno, i punti tornano, nessuno squalificato in campo, nessuna rosa vuota), a ogni commit;
    - il **golden pieno** (dieci stagioni del mondo intero), di notte e prima di un merge del motore della stagione;
    - un plugin d'esempio, creato dal **modello per i contributor**, passa i test senza toccare il nucleo (chiusura del passo 2).

## Con LUNA

Dito: un giorno, cinque momenti, una scheda. Luna: la stagione è la partita vista da lontano. Lo stesso disegno (un nucleo che non sa il calcio, plugin che lo sanno, una porta sola per i dati e per il motore) rende la stagione componibile da chi gioca e da chi contribuisce, e riproducibile bit per bit come la partita.

## Alternative scartate

- **Un nucleo che conosce il calendario e le classifiche:** più semplice all'inizio, ma ogni formato nuovo (play-off, gironi, coppe) cambierebbe il nucleo; il formato è dati (ADR-060).
- **Due schede, una per la partita e una per la stagione:** due validatori che divergono (decisione AQ).
- **Giocare le partite di un giorno in sequenza, ognuna sullo stato lasciato dalla precedente:** il risultato dipenderebbe dall'ordine e non si potrebbe giocare in parallelo e in anticipo (decisione AC).
- **Il risultato estratto dalla forza delle squadre per le partite degli altri:** contro "ogni partita giocata dallo stesso motore" (`L1.md`) e contro ADR-035.

## Conseguenze

- Il passo 2 di `L1.md` parte da qui: `ds-plugin-api` (la scheda e il validatore, spostati da `ds-match-api` a golden identico), `ds-season-api`, `ds-season` con una stagione vuota che avanza giorno per giorno e si salva, il plugin d'esempio e il modello.
- `Weight` della partita diventa la fascia di decisione X; le schede dei plugin di L0 dichiarano la loro (`Core` o `Recommended`) senza cambiare il comportamento.
- `ds-db` prende `rusqlite` con SQLite compilato dentro: una dipendenza nuova, da verificare su Android e WebAssembly prima del passo 3.
