# ADR-109 — Il pericolo che lasciano i compagni più vicini (DD-51)

- **Status:** Accepted (delegated, 2026-11-23) — da rivedere da parte del maintainer
- **Date:** 2026-11-23
- **Decision:** DD-51
- **Attua:** fase 3 di `PITCH-AUDIT.md` (senza palla); ADR-052 §5 (la difesa pensa)
- **Depends on:** ADR-049 (l'atto senza palla è del giocatore), ADR-052 §5, ADR-100 (i pericoli letti)

## Decisione

In `defend()` il valore di pressare il portatore e di marcare un uomo era moltiplicato per un rango: 1 al più vicino, `lettura × 0,5` al secondo, 0 dal terzo in poi. Il taglio era una regola imposta: dal terzo uomo nessuna di quelle azioni esisteva.

Ora il valore è moltiplicato per la parte di pericolo che i compagni più vicini gli lasciano, come lui la crede: per ognuno, la probabilità che non arrivi in tempo, con la stessa fiducia e lo stesso passo creduto della sua corsa (`ThoughtBalance::confidence`); il prodotto su di loro, perché ciascuno può prenderlo.

    lasciato = Π_k (1 − in_time(d_k))

Nessun numero nuovo, spariscono il taglio e lo 0,5. Con LUNA: il dito è chi è più vicino al portatore o all'uomo; la luna è che ognuno legge quanto pericolo gli altri lasciano davvero. Partita vera: il secondo raddoppia quando il primo rischia di non arrivare, e lascia fare quando il primo è già addosso.

## Conseguenze

- SIM_VERSION 190 → 191. Golden rigenerati; semi coppa 83, supplementari 54 (pari) e 42 (deciso), rigori 83.
- `Target.left`, `offball::left_by_nearer`; `ds-match` lo calcola dagli stessi compagni che conta `rank`.
- Gemello `the_second_man_doubles_up_only_when_the_first_may_not_get_there`.
- Test GUI `the_whistles_follow_every_stop_and_the_restart`: l'ultima interruzione della partita può non avere ripresa (Legge 7), emerso con la nuova partita del seme 42.
- Misura 42..101: 6 su 22, gol 10.98 → 9.78. Sonda: pressing +7%, il secondo e il terzo uomo pressano (prima mai il terzo); il terzo resta al posto nel 97% dei casi, lettura corretta della zona.
- Il rango resta per il pallone libero (`goes_for_loose_ball`) e il ramo senza portatore: non toccati qui.

## Scartate

- Lasciare il taglio: un uomo che non può scegliere non è un attore (ADR-049).
- Un rango graduato con un fattore nuovo: un numero senza fonte (A5-bis).
