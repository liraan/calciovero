# ADR-078 — The keeper's depth is read, not fixed (L0-CLOSE 10d)

- **Status:** Accepted (delegated, 2026-11-19) — to be reviewed by the maintainer
- **Date:** 2026-11-19
- **Decision:** DD-19, from L0-CLOSE 10d
- **Depends on:** ADR-045, ADR-051, ADR-063

## Contesto

L'Audit L0 (voce 10d) segnalava il portiere "senza causa": il piazzarsi al
tiro, la profondità letta e l'uscita sul passaggio erano geometria fissa.
Ogni portiere avanzava dello stesso `advance_cm * t`, qualunque fosse la sua
lettura del gioco.

## Decisione

1. La profondità off-line è `advance_cm * t * read`, dove `read` è la lettura
   del gioco del portiere (`game_reading`), `t` la vicinanza del pallone alla
   porta e `advance_cm` il budget di `keeper.json`.
2. Chi legge il gioco esce a restringere l'angolo e a coprire lo spazio dietro
   la difesa; chi non lo legge resta sulla linea, pronto a reagire al tiro.
3. La scelta resta del portiere: nessun copione, nessun dado. La profondità
   è ancora limitata da `depth_share_milli` (ADR-051).

## Conseguenze

- `SIM_VERSION` 161. Golden rigenerato.
- La taratura dei numeri di `keeper.json` resta di L3; qui cambia la causa,
   non il numero.
