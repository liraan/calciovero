# ADR-114 — Un attore di ogni tipo, i ruoli come dati: l'allenatore sceglie l'undici (DD-66)

- **Status:** Accepted (delegated, 2026-11-24); emendato dal maintainer il 2026-11-24 (decisione BM, §7)
- **Date:** 2026-11-24
- **Decision:** DD-66
- **Attua:** `context/plans/L1-COACH.md` ("Un attore come i giocatori", "Il giocatore umano sceglie il suo ruolo", "Come sceglie, senza dadi"); decisioni AH, AI; `L1.md` passo 5, primo anello
- **Depends on:** ADR-003 (piano A), ADR-035 (niente dadi), ADR-041, ADR-043 (la scelta dell'attore), ADR-057 (le stime), ADR-059 (le istruzioni), ADR-108 (il compagno conosciuto quanto ci si fida), ADR-112 (il caveau resta chiuso), ADR-113 (il contratto della stagione)

## Il problema

Al passo 4 ogni squadra scende in campo con i primi della rosa per ruolo: nessuno sceglie. L1 vuole che ogni club scelga undici e piano con il suo allenatore, e che l'allenatore sia un attore come i giocatori (decisione AH), in un posto che un giorno potrà prendere il giocatore umano (decisione AI).

## Decisione

1. **Un attore di ogni tipo.** Giocatore e allenatore (poi staff e dirigenza, L2) sono attori con la stessa porta verso il caveau: le loro caratteristiche nascoste stanno in `ds-db`, ricavate dal seme del mondo e dalla loro identità (ADR-029); le abilità sono formule (`ds-calculator`); la scelta la fa l'attore in `ds-orchestrator`, e nessun plugin legge una caratteristica (ADR-112). L'identità di un allenatore è nello spazio delle identità del caveau, in un intervallo suo (`COACH_KEY_BASE + club`), finché le persone non hanno un tipo d'identità comune.

2. **Le caratteristiche dell'allenatore, primo anello** (l'albero di `L1-COACH.md`, grado 3 dichiarato finché non nascono dal livello del club e dalle squadre vere): l'idea di gioco (l'altezza del blocco, l'ampiezza preferite) e la mente tattica (la conoscenza tattica). Ricavate per chiave in `[0, 1)`, come le caratteristiche dei giocatori prima dei profili.

3. **L'undici, senza dadi.** Per ogni posto del piano (oggi i ruoli del 4-4-2) l'allenatore sceglie il giocatore con la **stima migliore per quel ruolo**, fra quelli non ancora scelti, nell'ordine dei posti; a parità, l'identità più bassa (in un anello successivo: chi ha giocato meno di recente). Cinque in panchina, uno per reparto, con lo stesso criterio.
   - **L'adattamento al ruolo** viene dal profilo del ruolo (`balance/profiles.json`: la media e la dispersione di ogni caratteristica in quel ruolo): prima quanto gli manca rispetto a ciò che il ruolo chiede, in dispersioni (meno è meglio), poi quanto lo supera. Nessun peso nuovo. Scartata la media pesata dal profilo: premiava il giocatore forte in tutto, e il primo test mise un centrocampista in porta.
   - **La stima** dell'allenatore sui suoi giocatori è stretta (`L1-COACH.md`: li allena ogni giorno) e ne conserva l'ordine: in questo anello è l'adattamento stesso. La conoscenza tattica peserà dove la stima è larga (gli avversari, il piano gara, gli anelli successivi); una forma lineare uguale per tutti i suoi giocatori, come `0,5 + k(vero − 0,5)`, non cambierebbe nessuna scelta, e per questo non si usa qui.

4. **Il piano, primo anello.** L'altezza del blocco e l'ampiezza vengono dall'idea di gioco dell'allenatore (in tre terzi: basso, medio, alto; stretto, normale, largo) ed entrano nella partita come l'istruzione di ADR-059 al tick 0, che il motore legge già. Il modulo resta il 4-4-2 finché la lavagna libera (decisione AL) non c'è.

5. **I ruoli sono dati** (decisione AI). Un ruolo è una scheda: le scelte che gli spettano, cosa vede e con che precisione, a chi risponde. Il primo ruolo è `coach`, a cui spettano l'undici e il piano; ogni scelta di ogni plugin dichiara il ruolo a cui spetta. Il posto giocabile in L1 è l'allenatore: quando il giocatore umano lo prende, le sue scelte entrano come input datati (ADR-060) al posto di quelle dell'attore; la proposta del vice (`L1-COACH.md`) è la scelta dell'attore stesso.

6. **`LineupAnnounced`.** Il plugin `ds-coach` annuncia la formazione a `BeforeMatches`, prima della Giornata (che dipende da lui): l'identità di ogni posto, la panchina, il blocco e l'ampiezza. La Giornata la legge e la gioca; senza annuncio la partita non si gioca e la stagione si ferma con un errore che lo dice.

7. **Emendamento (decisione BM, maintainer, 2026-11-24; `L1-COACH-STUDY.md` §4.2).**
   - **Lo spareggio:** a stima pari sceglie chi ha giocato meno di recente (il giorno dell'ultima formazione che lo schierava), poi l'identità più bassa. Con stime continue la parità è rara: conta soprattutto quando la stima include lo stato del giorno (la rotazione).
   - **La panchina:** quanti ne permette la regola della serie (`nation.json`), fino a quanti il motore sa giocarne (`MAX_BENCH`, oggi 7); lo scarto si dichiara. Il primo posto è un portiere; gli altri, a turno sui ruoli del campo, il migliore rimasto per ciascuno (non "uno per reparto").
   - **Il blocco continuo:** l'altezza del blocco e l'ampiezza sono valori continui nella lavagna libera (decisione AL); fino a quando il piano del motore li prende continui (un cambiamento di L0) restano in tre terzi, la radice continua.
   - **La rotazione** (anello 2): la stima per il ruolo legge il corpo della mattina come lo farà correre il motore (la fatica è la maggiore fra fatica e danno portati, la velocità di punta ridotta da `v_max_factor` e riportata sulla scala di `max_speed`), quanto dice la radice `coach_rotation`. Il rischio di infortunio del carico ravvicinato (Dupont et al. 2010) entra con l'ADR degli infortuni.
   - **Le radici:** le disposizioni dell'allenatore (idea, adattabilità, lettura del gioco, stile con le persone, rotazione, giovani, rischio) non hanno un'unità: sono percentili del mondo in `[0, 1)`, che è la forma che la decisione AE chiede per il livello; le code sono quelle delle grandezze con un'unità (i giocatori), non di un percentile.

## Con LUNA

Dito: un undici e due istruzioni. Luna: in campo vanno gli uomini che qualcuno ha scelto, con il suo modo di vedere il gioco e il suo modo di sbagliare; la stessa scelta un giorno sarà di chi gioca.

## Alternative scartate

- **Un'IA dell'allenatore a parte**, con le sue regole: contro AH, e un secondo modello di scelta da tenere coerente con il primo.
- **L'allenatore che legge le caratteristiche dei giocatori**: contro ADR-003 e ADR-112; sceglie sulle stime.
- **Pesi nuovi per l'adattamento al ruolo**: i profili dei ruoli dicono già in cosa ogni ruolo è forte (A5-bis: nessun numero scelto guardando l'effetto).

## Conseguenze

- `ds-calculator`: l'adattamento al ruolo; `ds-orchestrator`: le caratteristiche dell'allenatore e la sua stima, la scelta dell'undici e del piano; `ds-db`: le chiavi delle persone (già c'è il mappaggio delle identità, ADR-113).
- Il golden della stagione cambia (`SEASON_VERSION` 2); il motore della partita no.
- I prossimi anelli: la condizione e la rotazione (con il corpo, passo 6), il piano gara sull'avversario (dalle sue partite), il modulo (la lavagna), i rigoristi e i battitori.
