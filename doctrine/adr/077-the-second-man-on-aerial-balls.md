# ADR-077 — The second man on aerial balls (OI-102)

- **Status:** Accepted (delegated, 2026-11-19) — to be reviewed by the maintainer
- **Date:** 2026-11-19
- **Decision:** DD-18, from OI-102
- **Depends on:** ADR-049, ADR-051

## Contesto

Sulle palle alte, solo il più vicino contestava (rank 0): i duelli aerei
erano 10,4 a partita contro 38,1 del vero, perché il secondo uomo non
arrivava mai.

## Decisione

1. Su una palla aerea (lato che difende), anche il secondo uomo (rank 1)
   contesta se può raggiungere il punto di caduta.
2. Il resto resta: il più vicino (rank 0) va sempre; il secondo solo se
   può arrivare.

## Conseguenze

- `SIM_VERSION` 160. Golden rigenerato.