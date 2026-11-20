# ADR-083 — One ball, always physical (OI-127)

- **Status:** Accepted — DD-23, maintainer chose "fai prima la palla" on 2026-11-20
- **Date:** 2026-11-20 (attuazione prima di OI-100, per scelta del maintainer)
- **Decision:** a free ball keeps its physics; `Loose` is not a frozen state
- **Depends on:** ADR-044, ADR-048, ADR-058, ADR-069, ADR-080
- **Supersedes:** nothing; emenda la definizione di `BallState::Loose` in `spatial.rs`

## LUNA

Il dito: nel 2.5D e nel TUI il pallino copre la palla e sembra che i giocatori
le passino sopra senza spostarla. La luna: il motore non ha una palla sola, ha
tre oggetti. `Possessed` è attaccata al piede e senza fisica, `InFlight` ha
gravità e attrito, `Loose` — appena la velocità scende sotto 1 cm/tick — ha la
velocità azzerata e la fisica spenta. La palla cambia natura per decreto, non
perché è successo qualcosa in campo.

## Partita vera

C'è una palla sola, sempre fisica: rotola, rallenta, sbatte. Il possesso è una
relazione che dura finché un corpo la controlla, non un interruttore che
sostituisce la palla. Quando un giocatore la perde, la palla continua con la
velocità che aveva. Un pallone che rotola non si congela a 10 cm/s: arriva
fino in fondo al suo rotolamento. L'unica palla davvero ferma è quella
fuori gioco in attesa di una ripresa, e la Legge lo dice (ADR-080).

## Decisione

1. `advance_ball` applica la fisica a ogni palla non in attesa di ripresa,
   compresa `Loose` e `Possessed`. Per quest'ultima è di fatto un no-op: il
   controllore la tiene a velocità zero e il passo successivo la lascia dov'è.
2. Quando la velocità scende sotto la soglia dichiarata, lo stato passa da
   `InFlight` a `Loose`, ma **la velocità residua non viene azzerata**: la palla
   continua a rotolare finché l'attrito dichiarato la esaurisce. Nessun numero
   nuovo: la soglia, l'attrito e il rimbalzo restano quelli di `ball.json`.
3. `Loose` significa "libera in questo tick", non "ferma". `waiting` resta
   l'unica eccezione: la palla è fuori gioco e il passo della partita non la
   muove (SIM 150).
4. Il gemello del burattino regge: cambiando solo la reattività o la lettura di
   un giocatore cambia chi prende la palla, non la fisica della palla; la
   portata resta `ball_reach_cm` e `loose_control_radius_cm`.

## Conseguenze

Una causa alla volta: `SIM_VERSION` +1, voce in `CHANGELOG-SIM`, goldens, P1
`a_free_ball_keeps_rolling_after_it_slows_below_the_stop_threshold`, `fmt`,
`clippy`, `cargo test --workspace`, gate `closing`, misura 42..101 e tabella
autorevole aggiornata. Nessun merge in `main` senza autorizzazione.
