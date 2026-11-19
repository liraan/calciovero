# ADR-080 — La palla morta e il battitore (ADR-052, `BEHAVIOURS.md` 25-26)

- **Status:** Accepted (delegated, 2026-11-19) — to be reviewed by the maintainer
- **Date:** 2026-11-19
- **Decision:** DD-21, dal linguaggio del calcio moderno di ADR-052 e dal catalogo `BEHAVIOURS.md`
- **Depends on:** ADR-039, ADR-051, ADR-052, ADR-063, ADR-071, ADR-079

## Contesto

Nelle ripartenze il giocatore si comportava come se la palla fosse ancora in
gioco: gli avversari pressavano il battitore e correvano sulla palla ferma,
e il battitore era semplicemente l'uomo più vicino. Le Leggi 13-17 dicono
l'opposto: finché il calcio non è battuto la palla è morta, gli avversari
devono rispettare la distanza e la squadra che riprende sceglie chi ha il
piede adatto. Il piano `BEHAVIOURS.md` 25-26 dice che il battitore ci va e
gli altri prendono posto. Non era una tattica di squadra mancante: mancava
un fatto nella percezione del singolo e una scelta sua.

## Decisione

1. `OffBallScene` ha `dead_ball: bool`. `thought.rs` lo deriva da
   `self.restart_pass.is_some()`: è ciò che il giocatore vede — il pallone
   tenuto per il calcio — non un ordine dell'allenatore né una regola che
   muove i corpi.
2. Con `dead_ball`, `choose_off_ball` non offre più `Press`, `Challenge`,
   `Intercept` o `GoToBall`. Restano `Mark`, `Cover` e `Hold`, con le salienze
   di sempre: ognuno interpreta la palla morta con la propria lettura, la
   propria disciplina e il proprio intesa, non con uno schema comune. Il fatto
   è di tutti; la decisione resta del singolo, come vuole ADR-052.
3. `restart_taker` sceglie per ruolo e per piede: rinvio dal fondo al
   portiere; rimessa laterale all'ala o al terzino dalla parte giusta (il più
   vicino dei due); angolo e punizione al miglior piede fra gli uomini di
   movimento entro il raggio già dichiarato; rigore al miglior finalizzatore;
   calcio d'inizio al più vicino. La scelta passa da `set_piece_taker`, quindi
   dalle qualità del candidato e dal suo piede migliore, non dal caso e non
   da una lista di squadra. Il shootout resta com'era. Nessun nuovo numero.
4. Un colpo di testa che allontana la palla dalla propria porta ha lo stesso
   `retry_ticks` di una liberazione al piede: dopo il rimbalzo sul corpo di
   chi ha crossato, lo stesso difensore non può colpire di testa due tick di
   fila. È la stessa regola che valeva per il primo tocco (SIM 135), applicata
   al gesto che l'aveva aggirata.

## Conseguenze

- `SIM_VERSION` 163. I tre golden sono rigenerati.
- Il gemello in `ds-orchestrator::offball` mette lo stesso uomo davanti alla
  stessa palla: pressa e contrasta a palla viva, tiene il posto a palla morta
  anche se l'allenatore chiede pressione; lo vale anche per la palla libera.
  Il test in `set_pieces` controlla portiere, rimesse dalle due parti, e
  angolo/punizione dati al piede migliore anziché al più vicino.
- Misura su 60 semi con i filtri della partita vera (42-101): passaggi
  1282,9 -> 1250,1 a partita (reali 887,7); duelli a terra 179,5 -> 172,1
  (reali 182,6); rigori 0,00 -> 0,02 (reali 0,33); pallone libero preso subito
  69,3 % -> 69,1 %. I gol salgono 4,82 -> 5,20 (reali 2,68): ripartenze senza
  contrasto e un piede migliore creano più conclusioni. È un effetto da
  tarare a L3, non un motivo per far pressare il battitore.
- Il giocatore non diventa un esecutore di copioni: anche la scelta del
  battitore nasce dalle sue qualità e dal suo piede, e il resto della squadra
  continua a decidere con le proprie salienze cognitive.
