# ADR-092 — Il primo tocco è del ricevitore (DD-33)

- **Status:** Accepted (delegated, 2026-11-21) — da rivedere da parte del maintainer
- **Date:** 2026-11-21
- **Decision:** DD-33
- **Closes:** OI-123 (ADR-053 §5, «The ball won at full speed and run past is the reception, open»)
- **Amends:** ADR-058 §5 (lato del corpo e orientamento «Declared, not read yet»)
- **Depends on:** ADR-035 (nessun dado), ADR-048 §4 (numeri dichiarati),
  ADR-083 (la palla resta fisica qualunque sia l'etichetta), ADR-088/089 (la
  frenata del portatore)

## Contesto

Il maintainer, guardando seed 42 in 2.5D: «non puoi fixarle da solo questi
comportamenti? sono cose chiaramente che non succedono nella partita reale …
guardati la partita tick per tick, poi la cosa più importante è capire perché
succede, non il fatto nello specifico». Il caso indicato è il «9 rosso» al
second 16 (home slot 8, etichetta GUI `slot + 1`).

Tick per tick: al tick 152 controlla a 2 cm con velocità −67 cm/tick; al tick
153 il primo tocco manda la palla a 4,84 cm/tick in direzione
`(0.7775, 0.6289)`, indietro verso chi l'ha passata, con `period = 0` e
`held_cm = 0`; dal tick 154 al 188 l'uomo ha la palla come bersaglio (`overdue`)
e frena secondo SIM 173/174, ma l'inerzia lo porta 651 cm oltre il pallone al
tick 170, mentre il pallone si muove di circa 70 cm; al tick 189 è di nuovo a
0 cm e la gioca lunga (tick 190, `z = 127`). 37 tick, 3,7 s.

La causa è architetturale, non il caso: in `SpatialState::follow_carrier`,
branca `won`, il primo tocco prende `dir` e `speed` solo da `won` (la velocità
con cui la palla è arrivata) e da `spill_of`. La velocità del ricevitore, che
`move_agents` ha già aggiornato nello stesso tick, non viene letta. La
direzione del giocatore entra solo al tocco **successivo**, via `carry_target`
e `agent.velocity`. Il controllo è dunque un atto della palla, non dell'uomo.

Filtrando con LUNA: il dito è il vettore in arrivo e il residuo di
`first_touch`; la luna è lo stesso giocatore che arriva, con la propria corsa e
la propria capacità di controllo, e sceglie dove mettere il pallone appena lo
prende. Cambia il modo in cui esegue, non chi prende il pallone, né la regola
del controllo, né la fisica della palla libera.

## Decisione

Nella branca `won`, se il ricevitore corre contro il passaggio
(`run·ball < 0` e `pace >= 1`), il primo tocco usa la **sua** direzione di
corsa, con `period_of(carrier, pace)`, `touch_speed(pace, period, keep)` e la
frenata in arrivo: la stessa formula del tocco successivo, gli stessi numeri
dichiarati. Negli altri casi (palla ferma, ricevitore fermo, corsa nella stessa
direzione della palla) il tocco resta quello del SIM 93: la palla ha il residuo
di velocità dichiarato da `reception.json` e chi la prende le va dietro.
Nessuna soglia nuova, nessun `BallState` nuovo, nessun dado (ADR-035).

## Verifica sulla partita vera

Seed 42, SIM 176 → SIM 177: i tick con portatore oltre 90 cm dal pallone
scendono da 8296 a 2864 (−65%), quelli con palla ferma e portatore distante da
7558 a 1766 (−77%); gli episodi da 609 a 513 e il picco da 839 a 675 cm, in
65.000 tick invece di 63.200. Su seed 53 i passaggi restano 1302 (1196 prima
del fix) e le due pause idriche restano due.

Guardiani: `a_man_running_into_the_pass_takes_it_with_him` (gemello: stessa
palla in arrivo, cambia solo la direzione di corsa del ricevitore) e
`a_ball_won_is_taken_where_it_is`, che resta verde e blocca il caso del
ricevitore fermo del SIM 93. L'audit degli stati della palla conferma che
`advance_ball` non legge `BallState`: il confine InFlight → Loose avviene dopo
la fisica e conserva la velocità residua (ADR-083).

## Scartate

- **Mettere la palla sotto il piede a ogni ricezione:** provato, dimezzava i
  passaggi su seed 53 (1196 → 630), i gol (6 → 2) e faceva sparire una pausa
  idrica. Cambiava l'atto, non solo l'esecuzione.
- **Cambiare `first_touch_keep_max_milli` o `touch_steps_max`:** sono dati
  dichiarati; il difetto era chi sceglieva la direzione, non quanto residuo
  resta.
- **Aggiungere un gate su `BallState`:** le etichette non comandano la fisica;
  il tocco del portatore è un atto di gioco, non un cambio di legge della palla.
- **Togliere la frenata SIM 173/174:** è già la correzione del portatore che
  supera il proprio pallone; con il tocco nella direzione giusta l'inerzia non
  lo porta più 651 cm oltre.

## Conseguenze

`SIM_VERSION` 177, golden e semi rigenerati (coppa 58 → 49, pareggio al 90'
49 → 42, decisivo 42 → 43). Il guardiano
`nobody_moves_faster_than_his_legs` esclude ora il posizionamento del cartellino
rosso: è una regola di Legge, non una corsa. Le decisioni delle Leggi del gioco
restano in `plugins/referee.rs` (DD-31).
