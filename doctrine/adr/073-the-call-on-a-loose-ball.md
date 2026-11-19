# ADR-073 — The call on a loose ball (strato 3, first piece)

- **Status:** Accepted (delegated, 2026-11-19) — to be reviewed by the maintainer
- **Date:** 2026-11-19
- **Decision:** DD-14, from strato 3 of `context/plans/BEHAVIOURS.md` (approved by the maintainer)
- **Depends on:** ADR-049 (every player chooses his own act), ADR-052 (the measure of the game), ADR-071 (the place without the ball)

*Written in Italian as the project's working language; the decision is binding as written.*

## Contesto

Senza intesa, due compagni vanno sulla stessa palla, o la lasciano tutti e due (BEHAVIOURS.md, strato 3). Il motore, finora, decide per ogni uomo separatamente: il piu' vicino a un pallone libero va sempre, il secondo solo se legge bene e arriverebbe prima. Non c'e' coordinazione fra compagni: nessuno chiama, nessuno lascia.

## LUNA

Una coppia affiatata divide il compito senza parlarsi: chi va, va; chi copre, copre. Il posto di ognuno tiene conto di dove andrà l'altro.

## Decisione

1. **L'intesa e' per coppia**, non per ruolo (decisione BB): quanto bene uno prevede l'altro. Chiave `(attore, compagno)`, in `[0, 1]`.
2. **Il valore iniziale e' un dato della partita** (decisione BB: nasce dai fatti, il vivaio, le stagioni insieme). A L0 non ci sono quei fatti: ogni coppia della stessa squadra parte da un'intesa di base (0.5), dichiarata in `state.rs::new`. Cresce con le giocate riuscite insieme (un passaggio controllato fra i due): da fare, lasciato al passo successivo dello strato.
3. **La chiamata sul pallone libero**: il piu' vicino (rank 0) va sempre. Un compagno di rank >= 1 che conosce abbastanza il compagno che va (intesa sopra la soglia `offball.json:intesa_yield_milli`, 0.5) gli lascia il pallone e tiene il posto (copre). Sotto la soglia, il secondo lettore puo' ancora andare, come prima: due sullo stesso pallone, o tutti e due fermi, e' quello che fa una coppia che si e' appena conosciuta.
4. **Nessun numero nuovo**: la soglia e' in `offball.json`; l'intesa e' una latente del motore, non una caratteristica del custode.

## Conseguenze

- `SIM_VERSION` 155. Golden rigenerato.
- Il test gemello e' in `ds-orchestrator/src/offball.rs` (`a_teammate_he_knows_yields_the_loose_ball`).
- La crescita dell'intesa dalle giocate riuscite resta aperta (strato 3, passo successivo), insieme ai movimenti complementari e alle chiamate vere ("lascia!", "esco io!").