# ADR-074 — Complementary movements (strato 3, second piece)

- **Status:** Accepted (delegated, 2026-11-19) — to be reviewed by the maintainer
- **Date:** 2026-11-19
- **Decision:** DD-15, from strato 3 of `context/plans/BEHAVIOURS.md`
- **Depends on:** ADR-049, ADR-052, ADR-071, ADR-073

## Contesto

Una coppia affiatata fa movimenti complementari: se il compagno viene corto,
io vado lungo, perché so che lo farà (BEHAVIOURS.md, strato 3). Finora il
posto senza palla (ADR-071) tiene conto solo di dove stanno i compagni ora,
non di dove andranno.

## Decisione

1. Il posto con la palla (ADR-071 §2-3) pesa anche dove i compagni stanno
   andando: la loro velocità, pochi tick avanti, entra nel controllo di
   affollamento, ma solo per chi conosce abbastanza i compagni (intesa ≥ 0.5).
2. Una coppia che si è appena conosciuta ignora la previsione: solo le
   posizioni attuali, come prima.
3. Nessun numero nuovo: l'orizzonte di previsione (5 tick) è una costante
   dichiarata in `state.rs`.

## Conseguenze

- `SIM_VERSION` 156. Golden rigenerato.
- Il test gemello è da aggiungere in `state.rs::marking_tests` (due uomini
  della stessa squadra, stessa zona, stessa velocità verso la fascia: con
  intesa alta non finiscono nello stesso corridoio).