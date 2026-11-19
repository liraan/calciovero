# ADR-079 — Le linee avversarie e lo spazio fra esse (ADR-052 §3 e §5)

- **Status:** Accepted (delegated, 2026-11-19) — to be reviewed by the maintainer
- **Date:** 2026-11-19
- **Decision:** DD-20, dal linguaggio del calcio moderno di ADR-052, `BEHAVIOURS.md` strato 2
- **Depends on:** ADR-039, ADR-040, ADR-051, ADR-052, ADR-071, ADR-075

## Contesto

Il giocatore leggeva una sola linea avversaria: l'ultima, e solo per
non farsi trovare in fuorigioco. Il linguaggio di ADR-052 e del glossario
(`GLOSSARY.md` §0) dice che gli uomini professionali leggono la prima linea
di pressione, quella di centrocampo e l'ultima, e gli spazi fra esse: là si
riceve girati, rompendo una linea. Il passaggio avanti era quindi valutato
solo per la distanza e la qualità dell'occasione dello spazio raggiunto, non
per quello che lasciava alle spalle.

## Decisione

1. `ds_orchestrator::read::lines` legge le linee avversarie dalla profondità
   degli uomini che il giocatore vede (il portiere escluso): entro
   `same_line_cm` (`thought.json`) sono una linea; se i reparti sono più di
   tre, i due più vicini diventano uno. L'ordine, nella direzione
   d'attacco, è prima pressione, centrocampo, ultima.
2. `lines_broken` conta le linee che un passaggio supera **verso la porta
   avversaria**: un passaggio indietro non rompe niente. `line_break` ne fa
   un fattore del valore dello spazio raggiunto (`line_break_milli`). È la
   misura di ADR-052 §3: una linea passata è una linea che gli avversari
   devono recuperare. Nessun dado, nessun peso per kind di opzione.
3. Lo stesso fattore vale per il posto che si prende senza palla:
   `TeamView::mate_ball` lo applica a `hold`, `offer` e `run`, e
   `choose_place` lo applica agli spazi della propria zona. Fra le profondità
   che considera ci sono ora anche i punti medi fra le linee avversarie,
   purché dentro la zona del ruolo: la disciplina di ADR-071 §4 resta.
4. Il mezzo spazio non ha un bonus fisso per corridoio: è il corridoio
   dove chi ha il ruolo (mezzala, seconda punta) prende lo spazio fra le
   linee. Un bonus a tutti darebbe al difensore centrale la voglia di
   uscire dalla colonna vertebrale, che non è calcio.

## Conseguenze

- `SIM_VERSION` 162. Golden rigenerato.
- I numeri di `thought.json` sono dichiarati, da tarare a L3; cambia la
  causa, non una scorciatoia per kind di giocata. `line_break_milli` è 60:
  a 70 il singolo pressatore della scena di ADR-050 diventava una linea da
  rompere e la progressione sotto pressione pesava più, non meno, rompendo
  quel gemello più vecchio. Con 60 valgono entrambe le cause: la pressione
  costa, la linea rotta paga.
- Le linee si leggono una volta per scena, non una per candidato di
  passaggio: lo stesso dato, senza lavoro ripetuto.
- Misura su 60 semi con i filtri della partita vera: passaggi
  avanti/laterali/indietro 24/53/23 % -> 27/50/23 % (reali 31/53/16 %);
  palla nella propria metà 73.8 % -> 71.0 % (reali 52.9 %). I salti più
  lunghi delle gambe sono 0.0 a partita (prima 0.1).
- Test gemelli in `ds-orchestrator::read` e in `ds-match::state`: lo stesso
  passo vale di più se rompe una linea; lo stesso uomo prende lo spazio fra
  le linee solo se le linee ci sono.
