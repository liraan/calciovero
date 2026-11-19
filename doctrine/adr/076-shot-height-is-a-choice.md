# ADR-076 — Shot height is a choice (OI-100, radice rimasta)

- **Status:** Accepted (delegated, 2026-11-19) — to be reviewed by the maintainer
- **Date:** 2026-11-19
- **Decision:** DD-17, from OI-100
- **Depends on:** ADR-046, ADR-047, ADR-052

## Contesto

Il tiro era sempre basso (contact_height 0.0), tranne il pallonetto. Nel
vero il tiratore sceglie l'altezza in funzione del portiere e della scena.

## Decisione

1. Per ogni post e potenza, oltre al tiro basso si aggiunge un candidato
   alto (contact_height 1.0), più difficile da piazzare (geom 0.85) e più
   rischioso (risk 0.1). Vince solo se il tiratore legge il gioco e
   accetta il rischio.
2. Il pallonetto resta candidato separato.

## Conseguenze

- `SIM_VERSION` 159. Golden rigenerato.
- La taratura dell'altezza del tiro (OI-100) resta aperta: i tiri da vicino
  sono ancora troppo precisi (97 % contro 46 % reale).