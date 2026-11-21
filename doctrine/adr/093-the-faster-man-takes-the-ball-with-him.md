# ADR-093 — L'uomo più veloce della palla se la porta con sé (DD-34)

- **Status:** Accepted (delegated, 2026-11-21) — da rivedere da parte del maintainer
- **Date:** 2026-11-21
- **Decision:** DD-34
- **Estende:** ADR-092 / SIM 177 (il primo tocco è del ricevitore), OI-123
- **Depends on:** ADR-035 (nessun dado), ADR-048 §4 (numeri dichiarati),
  ADR-058 §5 (lato del corpo e orientamento), ADR-083 (la palla resta fisica
  qualunque sia l'etichetta), ADR-088/089 (la frenata del portatore)

## Contesto

Dopo SIM 177 il maintainer chiede di risolvere i problemi in partita «nei
limiti della visione del giocatore decidente». Ricostruendo i carrier-gap
ancora presenti su seed 42 (portatore a oltre 90 cm dal pallone che ha appena
vinto) restano quattro episodi lunghi: in tutti il giocatore vince una palla
che rotola via a pochi cm/tick e corre via a 50-60 cm/tick, superandola di
3-5 m, frenando e tornando indietro. Esempio: tick 665, away slot 8 prende la
palla a (4276, 3475) e corre a (22, 53) cm/tick mentre il pallone resta quasi
fermo; al tick 684 è a 466 cm, poi torna.

SIM 177 copriva solo il caso in cui il ricevitore corre **contro** la palla in
arrivo (`run·ball < 0`). Qui corre nella sua stessa direzione, ma è molto più
veloce del residuo di rotolamento: il tocco del SIM 93 manda la palla avanti a
`spill` cm/tick e l'uomo le passa comunque davanti. La causa è la stessa di
OI-123: nel primo tocco si leggeva la velocità della palla, non il rapporto tra
quella e la propria corsa.

Filtrando con LUNA: il dito è il residuo `spill` della palla e la velocità
dell'uomo, due fatti che il giocatore vede entrambi nel proprio cono; la luna è
lo stesso giocatore che, appena prende la palla, sceglie di portarla nel proprio
passo invece di rincorrerla. Cambia il come esegue il controllo, non chi prende
il pallone, né la regola del controllo, né la fisica della palla libera.

## Decisione

Nella branca `won` di `SpatialState::follow_carrier`, il primo tocco prende la
direzione della corsa dell'uomo quando corre contro il passaggio (`run·ball < 0`,
già SIM 177) **oppure** quando è più veloce del residuo di rotolamento
(`spill < pace`), con `pace >= 1.0` in entrambi i casi. La formula è la stessa
del SIM 177 e del tocco successivo: `period_of(carrier, pace)`,
`touch_speed(pace, period, keep)` e la frenata già dichiarati. Il confronto è
tra due velocità in cm/tick (`control_spill` e la norma di `agent.velocity`):
nessuna unità, soglia o costante nuova. Negli altri casi (uomo più lento della
palla che rotola nella sua direzione, palla ferma, uomo fermo) il tocco resta
quello del SIM 93. Nessun gate su `BallState`, nessun dado (ADR-035), fisica
libera invariata (ADR-083).

## Verifica sulla partita vera

Confronto A/B, stessi semi e stessi input, con e senza la condizione in più:

| metro (seed 42) | SIM 177 | SIM 178 |
| --- | --- | --- |
| tick con portatore oltre 90 cm | 2814 | 1398 |
| oltre 200 cm | 1180 | 160 |
| oltre 500 cm | 257 | 0 |
| picco del gap | 675 cm | 373 cm |
| passaggi | 1224 | 1255 |
| gol | 8 | 9 |

Seed 53: oltre 90 cm 2390 → 1456, oltre 200 cm 879 → 152, oltre 500 cm 213 →
0, picco 723 → 352 cm; passaggi 1212 → 1205, gol 10 → 3. I tre episodi lunghi
di seed 42 spariscono; resta un solo episodio, dovuto a un tocco pesante da
fermo e non all'uomo che supera la palla, fuori dal perimetro di questa
decisione.

Guardiani: `a_man_faster_than_the_spilled_ball_takes_it_with_him` (gemello:
stessa palla in arrivo, cambia solo la velocità di corsa dell'uomo) e, verdi,
`a_man_running_into_the_pass_takes_it_with_him` e
`a_ball_won_is_taken_where_it_is`, che bloccano i casi del SIM 177 e del
ricevitore fermo del SIM 93.

## Scartate

- **Generalizzare a «nessun tocco in avanti se l'uomo corre contro»:** la
  versione drastica provata per SIM 177 (mettere la palla sotto il piede)
  dimezzava passaggi e gol e faceva sparire una pausa idrica; qui si cambia solo
  il rapporto tra due velocità già lette, non l'atto del controllo.
- **Cambiare `first_touch_keep_max_milli` o `touch_steps_max`:** sono dati
  dichiarati; il difetto è chi sceglie la direzione, non quanto residuo resta.
- **Aggiungere un gate su `BallState` o leggere la posizione vera di un uomo non
  visto:** le etichette non comandano la fisica e la decisione usa solo ciò che
  il portatore vede di sé e della palla.

## Conseguenze

`SIM_VERSION` 178, golden e semi rigenerati (coppa 49 → 48, pareggio al 90'
42 → 48, decisivo 43 → 42, shoot-out di soli tiri 49 → 48). Le decisioni delle
Leggi del gioco restano in `plugins/referee.rs` (DD-31).
