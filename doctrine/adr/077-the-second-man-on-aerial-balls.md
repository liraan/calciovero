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

1. Su ogni palla alta, da entrambi i lati, anche il secondo uomo (rank 1)
   legge il punto di caduta e contesta se può raggiungerlo.
2. Il più vicino (rank 0) va sempre; il secondo solo se può arrivare. Il
   raggiungimento non è un punto fisso: il suo `Target` porta il rank vero,
   così la corsa con il proprio passo (`offball.rs::goes_for_loose_ball`,
   `ThoughtBalance::race`) e la sua lettura del gioco decidono se ci va.
   Nessun numero nuovo.
3. Emendata il 2026-11-20 da ADR-081 (scelta C): prima la lettura era
   limitata al lato che difende e al pass in volo, e il `Target` aereo
   hardcodificava `rank: 0`, quindi la corsa del secondo non scattava mai.

## Conseguenze

- `SIM_VERSION` 160. Golden rigenerato.