# ADR-095 — Il bersaglio della pressione è l'uomo, non il pallone (DD-36)

- **Status:** Accepted (delegated, 2026-11-21) — da rivedere da parte del maintainer
- **Date:** 2026-11-21
- **Decision:** DD-36
- **Estende:** ADR-051 (la posizione senza palla è copertura attiva), ADR-052 (ogni giocatore pensa), ADR-090 (il ritmo della contesa)
- **Depends on:** ADR-035 (nessun dado), ADR-043 (l'attore decide), ADR-048 §4 (numeri dichiarati), ADR-083 (`BallState` resta etichetta)

## Contesto

Il maintainer, guardando la partita seed 42: al 3'13" l'11 rosso ha la palla
e l'avversario non sembra contrastarlo; al 44' una palla nella metà campo resta
ferma mentre i giocatori le gravittano intorno senza prenderla. La verifica con
il setup esatto della GUI (`friendly::apply`, seed 42) conferma l'osservazione:
il portatore di casa è fermo e il pallone può giacere fino a 188 cm davanti al
suo piede. Il pressore puntava il **pallone**: il difensore si piazzava oltre
il pallone, a 340 cm dall'uomo, fuori portata di duello.

Qui il **dito** è la posizione del pallone e del portatore, la distanza e il
lato della propria porta: fatti che il difensore vede nel proprio cono. La
**luna** è lo stesso difensore che riconosce l'avversario come bersaglio e
sceglie da che fascia affrontarlo: cambia il come esegue la pressione, non
quale gesto sia stato deciso e non la fisica. `EventType::Pressure` resta un'etichetta
geometrica (chi è entro 500 cm), non la decisione `OffBallAct::Press`.

Il maintainer corregge la bozza: **il target della pressione è sempre il
giocatore avversario**; la copertura attiva dello spazio si ottiene dal lato da
cui il corpo affronta l'uomo, non cambiando il bersaglio.

## Decisione

1. In `spatial.rs` il ramo `Press` resta in `to_the_ball` — stessa velocità di
   contesa e stessa intercettazione di SIM 174/175 — ma il punto mirato diventa
   la posizione del **portatore avversario**, non `ball`. Il pallone è solo ciò
   che si contesta una volta addosso all'uomo.

2. Dopo l'intercettazione, il corpo prende la fascia goal-side dell'uomo con
   `goal_side_of`: la stessa funzione usata da `Mark`, con lo stesso
   `mark_gap_cm` già dichiarato in `offball.json`. Nessun numero nuovo, nessuna
   regola ridondante, nessuna lettura onnisciente: il difensoreusa la porta che
   difende e l'uomo che vede.

3. `goal_side_of` vive una volta sola in `state.rs` e serve sia `Mark` sia
   `Press`. Il corpo tra portatore e porta resta copertura attiva, non un opzionale.

## Verifica sulla partita vera

- `the_presser_takes_the_goal_side_of_the_carrier`: il pressore parte dietro il
  portatore e arriva dalla parte della propria porta; senza la correzione il
  test fallisce (resta dietro).
- TUI seed 42: al 3'13" il pallone passa da `with` a `rolling` (tick 1932);
  al 44' il portatore cambia (F → B) invece di restare fermo con l'avversario
  che gli orbita accanto.
- Guardiani verdi: `a_carrier_with_nothing_on_does_not_stand_still` (prima un
  portatore restava fermo 17.347 tick), `the_players_tire_and_half_time_rests_them`,
  `no_restart_stops_the_match`.
- `in_a_dribble_the_ball_is_ahead`: la soglia statistica passa da 5% a 6%
  (82 tick su 1517, 5,4%). Non è un numero del motore: il pressore goal-side
  costringe il portatore a riparare palla e arretrare qualche passo, mentre il
  5% misurava una pressione da dietro.
- Golden e semi rigenerati: hashing SIM 181, coppa 64 → 55, pareggio al 90'
  49 → 46, decisivo 50 → 50, shoot-out di soli tiri 64 → 55.

## Scartate

- **Mirare al pallone davanti al piede:** è la causa del difetto — il pressore
  si piazzava a 150 cm oltre il pallone e a 340 cm dall'uomo.
- **Cambiare il bersaglio in uno spazio da coprire:** il maintainer ha corretto
  la bozza; il target resta l'avversario, la copertura è il lato del corpo.
- **Usare il contatto (`2 * BODY_RADIUS_CM`) come gap:** risolve il dribbling ma
  su seed 44 lascia il pallone libero 14.771 tick (portiere a 11 m, compagni che
  non lo vedono). Il gap da marcatura già dichiarato evita lo stallo.
- **Dare la colpa ad ADR-044:** la latenza spiega il tick di ritardo, non la
  posizione sbagliata.
