# ADR-075 — The offside line is read with an error (OI-98)

- **Status:** Accepted (delegated, 2026-11-19) — to be reviewed by the maintainer
- **Date:** 2026-11-19
- **Decision:** DD-16, from OI-98
- **Depends on:** ADR-062 (he sees what he looked at), ADR-071

## Contesto

La linea del fuorigioco era letta perfettamente (OI-98): i giocatori sapevano
esattamente dove stava l'ultimo difensore. Il vero non è così: un giocatore
legge la linea con un errore che dipende dalla sua lettura del gioco, e il
fuorigioco nasce da lì (1,13 contro 4,3 a partita).

## Decisione

1. Nel posto con la palla (ADR-071 §2-3), la linea del fuorigioco che il
   giocatore usa per non oltrepassarla è letta con un errore: fino a ~2 m
   per il lettore peggiore, zero per il migliore.
2. Il controllo vero del fuorigioco (Legge 11) usa le posizioni reali:
   l'errore è nella percezione, non nella legge.
3. La soglia è una costante dichiarata in `state.rs`.

## Conseguenze

- `SIM_VERSION` 158. Golden rigenerato.
- Il gemello è da aggiungere: due giocatori uguali tranne la lettura del
  gioco, la stessa offerta in profondità: il migliore resta in linea, il
  peggiore si fa prendere in fuorigioco.