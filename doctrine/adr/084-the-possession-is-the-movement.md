# ADR-084 — The possession is the movement (OI-128)

- **Status:** Accepted — DD-24, maintainer gave the go-ahead on 2026-11-20
- **Date:** 2026-11-20
- **Decision:** how a player obtains the ball: the same movement, now with the ball
- **Emends:** ADR-082 (DD-22) and ADR-083 (DD-23) in their diagnosis, not in their accepted facts

## LUNA

Il maintainer guarda la 2.5D (seed 42, tick 233) e corregge la diagnosi: non
si tratta di "trattare la palla come propria", e non basta che la palla rotoli.
«Ottenere il possesso del pallone è quello che i giocatori fanno, ognuno con le
proprie caratteristiche e abilità. Ed essendo azione non è un qualcosa di
diverso da sé: invece di fare movimenti senza palla fa movimenti con la palla,
quindi controllandola e tentando di tenerla nella propria disponibilità.»

Nel motore il controllo sta fuori dal movimento. `control_loose_ball` è un
evento che assegna `carrier = Some(id)`; `move_agents` muove chi non ha la palla
con `OffBallAct`, e chi ce l'ha con `follow_carrier`. Due sistemi, due nature:
prima il giocatore fa movimenti senza palla, poi uno stato diverso sostituisce
la sua azione. È il burattino: il possesso arriva al giocatore invece di essere
qualcosa che lui fa.

## Partita vera

Un calciatore non smette di muoversi quando prende la palla. Continua a
muoversi, e i movimenti che faceva nello spazio ora li fa con la palla: la
controlla e cerca di tenerla nella propria disponibilità. Non esiste un
"prima senza palla" e un "dopo con palla" come due nature del giocatore: c'è un
movimento solo, che ora include la palla. Chi ottiene il possesso lo ottiene
con le proprie caratteristiche — lettura, reattività, tecnica, velocità — non
perché una soglia geometrica si è verificata.

Dal primo all'ultimo minuto i giocatori restano **solo giocatori**: cambia la
stanchezza e cambia il fatto di avere palla o meno, non cambia chi sono come
atleti e come calciatori. Le loro caratteristiche non si sostituiscono con uno
stato quando la palla arriva: sono sempre le stesse che leggono, corrono e
controllano.

Con il filtro LUNA: il **dito** è la stanchezza che cresce e il pallone che va
e viene; la **luna** è l'identità stabile dell'atleta, che in ogni minuto fa le
stesse cose con le sue stesse qualità, con o senza palla. Il possesso non è un
cambio di natura: è il suo stesso movimento, ora con la palla.

Il maintainer lo dice ancora: «non vogliamo burattini ma vogliamo giocatori
evoluti cognitivamente che scelgono di fare la cosa giusta in base alla loro
posizione in campo, la disposizione degli avversari e dei compagni (o no)».
Con LUNA: il **dito** è la porzione di campo, gli avversari, i compagni — la
scena che cambia a ogni istante; la **luna** è che la scelta resta del
giocatore, che la legge con le sue qualità. Ogni atto — correre, controllare,
passare, filtrare, tirare — dipende sempre e solo dalle sue capacità cognitive,
psicologiche, fisiche e tecniche, lette nella scena: mai da una regola che lo
sostituisce. La zona non è una tabella di
comportamenti: è un fattore della scena. Se a centrocampo copre lo spazio e in
area difende la porta, non è perché un confine gli impone l'atto, ma perché in
quel punto vede un pericolo, uno spazio, un compagno diversi. Nel motore c'è già
in nuce: `Place::Keeper { ball_in_area }`, `Threats`, `SpaceValues`, l'`asked`
di fase. La tendenza per zona, se mai servirà, andrà cercata in ciò che il
giocatore percepisce in quella zona, non assegnata come regola al posto della
lettura — e sarà causa separata, con ADR e misura sua.

## Decisione

1. Per una palla libera (nessun passaggio in arrivo), il controllo nasce
   dall'atto con cui il giocatore si muove verso di essa: se ha scelto
   `GoToBall` — atto che `choose_off_ball` ricava dalla sua lettura, dalla sua
   intesa e dalle sue gambe — la palla è sua entro il `loose_control_radius_cm`
   già dichiarato. Il movimento include il controllo; non serve un sistema di
   controllo separato.
2. Chi non si muove verso la palla fa movimenti senza palla e non la prende per
   caso: la sua portata resta `ball_reach_cm`, dalla sua reattività. Nessun
   numero nuovo, nessun dado, nessun bonus, nessun copione.
3. Il gemello del burattino regge sulla catena intera: cambiando la lettura
   (`game_reading`) cambia l'atto e cambia chi controlla la palla; se cambiasse
   solo l'atto senza cambiare lettura o abilità, la causa sarebbe rigettata.
4. ADR-082 resta valido nella scelta A (la palla libera è di chi legge di
   poterci arrivare) e ADR-083 nel suo fatto (la palla libera rotola); la loro
   diagnosi — "nessuno agisce per averla", "la palla è congelata" — è superata
   da questa: il difetto è la separazione fra muoversi e giocare la palla.
5. La zona di campo, gli avversari e i compagni restano fattori della scena
   letta dal giocatore (`OffBallScene`), non regole che scelgono al posto suo:
   nessuna tendenza per zona va codificata come copione.

## Conseguenze

Una causa alla volta: `SIM_VERSION` +1, voce in `CHANGELOG-SIM`, golden, P1 e
gemello, `fmt`, `clippy`, `cargo test --workspace`, gate `closing`, misura
42..101 e tabella autorevole aggiornata. Nessun merge in `main` senza
autorizzazione.
