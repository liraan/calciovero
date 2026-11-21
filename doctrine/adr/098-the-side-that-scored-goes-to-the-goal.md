# ADR-098 — La squadra che ha segnato va alla porta (DD-40)

- **Status:** Accepted (delegated, 2026-11-21) — da rivedere da parte del maintainer
- **Date:** 2026-11-21
- **Decision:** DD-40
- **Estende:** ADR-037 (DD-37: la festa è della squadra), ADR-094 (SIM 180 riservata al nodo della festa)
- **Depends on:** ADR-035 (nessun dado), ADR-043 (l'attore decide), ADR-048 §4 (numeri dichiarati), ADR-065 §5 (il motore non si cambia per la vista), ADR-088 (uomini mandati a un punto, con frenata), SIM 150 (nessun teletrasporto)

## Contesto

Dopo SIM 181/182 il maintainer guarda la GUI e segnala: «i giocatori non vanno
a festeggiare» e «sono molto disordinati tatticamente». Interrogato sul momento,
indica la **sosta dopo il gol**, non il gioco aperto.

L'osservazione è vera sul setup esatto della GUI (`friendly::apply`, seed 42).
DD-37 aveva dato `Gesture::Celebrate` a tutti gli uomini della squadra che ha
segnato, ma solo come posa e salto: i frame registrati al tick 6137 mostrano i
centroidi quasi fermi. Nella TUI, al minuto 11 (tick 6600, dopo il gol del
tick 6137), la squadra che ha segnato è già sparsa verso le posizioni del calcio
d'inizio: nessuno si avvicina al marcatore o alla porta. Il motore, appena il
pallone varca la linea, manda tutti ai propri posti di ripartenza; la festa è
solo un'animazione.

Qui il **dito** è l'evento `Goal`, il tick, il punto di varco della porta, la
sosta dichiarata `goal_to_kick_off_s`, le posizioni e le destinazioni `sent`.
La **luna** è la stessa squadra che ha segnato: legge il gol e sceglie di
raggiungere insieme la porta, poi di rientrare nel blocco per il calcio
d'inizio. Cambia il come esegue durante la sosta, non l'etichetta dell'evento,
non la fisica e non la regola del calcio d'inizio.

La correzione non può stare nella vista (ADR-065 §5): spostare i corpi solo
nell'immagine inventerebbe posizioni che non esistono nel record della partita.
Il movimento è calcio, quindi va nel motore, con un SIM bump. La riserva SIM
180 dell'ADR-094 è così chiusa.

## Decisione

1. Dopo un gol, `call_restart` riceve `GoalParty { side_home, point }` dal
   punto reale del varco (`ball.rs`). La squadra che ha segnato è mandata a
   correre verso quel punto con lo stesso `spatial.send` già usato per le
   ripartenze (SIM 150): nessun teletrasporto, nessun punto inventato.
   La squadra che ha concesso il gol va ai suoi posti di calcio d'inizio
   esattamente come prima.
2. Il portiere della squadra che ha segnato resta alla sua porta e il battitore
   del calcio d'inizio (della squadra che ha concesso) va al pallone: nessun
   uomo deve attraversare il campo per poi tornare.
3. La festa dura metà della sosta già dichiarata da `goal_to_kick_off_s`:
   a quel tick `resume_restart` rimanda la squadra che ha segnato nel proprio
   blocco con lo stesso `block_target` e lo stesso andamento a piedi usato per
   il calcio d'inizio. La seconda metà della sosta serve a rientrare; il
   calcio d'inizio resta subordinato alla Law 8, non a un timer nuovo.
4. Nessun numero nuovo, nessun dado, nessun behavioural latente: l'unica
   divisione è a metà di un tempo già dichiarato nei balance e già letto dalla
   vista per la posa di DD-37. SIM_VERSION 183.

## Verifica sulla partita vera

- P1 `the_side_that_scores_goes_to_the_goal_before_the_kick_off` (gemello:
  la squadra che segna è fuori casa, l'altra va ai posti): allo scattare della
  sosta gli undici di chi ha segnato hanno come destinazione il punto del gol;
  prima della metà della sosta la festa è ancora in corso; dopo, nessuno ha
  più quel punto come destinazione. I portieri non partono.
- TUI seed 42, gol al tick 1384 (ospiti in porta a `x = 0, y = 3657 cm`):
  la distanza media degli otto uomini di movimento ospiti dal punto scende da
  35,1 m (tick 1390) a 6,4 m (tick 1480) e resta tale fino alla fine della
  festa; al tick 1720 sono già a 16,9 m e al tick 1785 a 61,9 m, diretti ai
  posti del calcio d'inizio. Simmetria verificata sul gol di casa al tick
  14426 (`x = 10500 cm`): 35,0 m → 2,5 m, poi rientro. La ripartenza dopo il
  gol avviene al tick 2007, 623 tick dopo il gol (606 dichiarati più 17 per
  la Law 8): nessun pallone morto aggiunto.
- Golden e semi rigenerati: hashing SIM 183, coppa 81 → 117, pareggio al 90'
  53 → 44, decisivo 42 → 42, shoot-out di soli tiri 81 → 117.
- `ds-reference compare --seeds 42..51`: gol 11.90 → 11.30, tiri 49.30 →
  47.60, in porta 23.70 → 21.50, passaggi 1374.70 → 1384.00. La variazione è
  l'effetto atteso del cambio delle posizioni di ripartenza, non una taratura.

## Scartate

- **Spostare i corpi solo nella vista:** ADR-065 §5 — il record della partita
  direbbe una cosa e lo schermo un'altra.
- **Mandare tutti dal marcatore:** il marcatore è un'etichetta dell'evento e il
  suo corpo può essere lontano dal varco; il punto del gol è un fatto osservato
  e serve a tutti.
- **Festa per tutta la sosta:** toglierebbe il tempo per rientrare e sposterebbe
  il calcio d'inizio, che deve restare quando tutti sono nella propria metà.
- **Un nuovo numero in `balance` (`party_s`):** la sosta c'è già e la vista la
  usa già; un secondo numero darebbe una seconda regola per lo stesso fatto.
- **Far correre anche i portieri:** dovrebbero attraversare il campo e tornare
  prima del fischio, senza alcun guadagno per la lettura della scena.
