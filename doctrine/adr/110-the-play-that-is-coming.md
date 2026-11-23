# ADR-110 — Sul passaggio ognuno legge il gioco che arriva (DD-52)

- **Status:** Accepted (delegated, 2026-11-23) — da rivedere da parte del maintainer
- **Date:** 2026-11-23
- **Decision:** DD-52
- **Attua:** fase 3 di `PITCH-AUDIT.md` (senza palla); ADR-034 (la memoria dello sguardo); OI-098
- **Depends on:** ADR-049, ADR-052 §5, ADR-058, ADR-109

## Decisione

1. **Il passaggio in viaggio è una scena.** Finché un passaggio viaggia, chi vede l'uomo a cui va (o è lui) legge quel compagno o avversario, dove incontra la palla, come il portatore che arriva: i difensori scelgono fra accorciare, marcare, coprire, intercettare (`defend()`); i compagni fra smarcarsi, attaccare la profondità, tenere. Chi va sulla palla ci va prima, con la stessa corsa del pallone libero. Prima, chi non andava sulla palla aveva una sola opzione: restare (sonda: 227.000 decisioni su 300.000 durante i passaggi, 10 partite).
2. **La memoria tiene la corsa.** Lo sguardo registra dove e come si muoveva un uomo e da quanti tick non lo si guarda; chi lo ricorda lo immagina proseguire alla corsa vista, per il tempo in cui si fida di una previsione (`trust_ticks`, il limite di ogni previsione del pensiero, già nel motore). Prima lo congelava all'ultimo sguardo: nell'80% dei passaggi in fuorigioco il passatore vedeva il ricevente 2-10 m più indietro di dov'era.
3. **La linea letta dove sarà.** Durante un passaggio l'ultima linea è letta dove la portano i difensori nel tempo che la palla impiega ad arrivare.
4. **Correzione:** `OFFSIDE_PERCEPTION_ERROR_CM` era 2000 (20 m) contro il proprio commento (~2 m); ora 200. Resta arbitrario dichiarato (OI-098).

Nessun numero nuovo. Con LUNA: il dito è la palla che viaggia e l'uomo che le va incontro; la luna è che ognuno legge dove andrà il gioco e decide prima che arrivi. Partita vera: il passaggio è il segnale del pressing; i compagni si muovono mentre la palla viaggia; il regista sa che il suo attaccante non si è fermato.

## Conseguenze

- SIM_VERSION 191 → 192. Golden rigenerati; semi coppa e rigori 52, supplementari 52 (pari) e 42.
- `OffBallScene.carrier_to_come`; `Memory::see_moving`, `Memory::believed`; `TeamView::new(.., ahead_ticks)`; `GroundTruth::first_to_it`.
- Gemelli `on_the_pass_the_defender_steps_to_the_man_it_goes_to`, `a_man_seen_running_is_imagined_gone_on`.
- Test `no_more_substitutions_than_the_laws_allow` conta le sostituzioni raccontate (Legge 3), non chi è in campo alla fine (seme 47: tre rossi in casa). Resta da capire perché l'ordine (5, 4) non è stato eseguito: OI aperto.
- Misura 42..101: 6 su 22; duelli a terra 147.8 → 172.1 (in banda), falli 23.8 → 27.5, passaggi 1750 → 1702; fuorigioco 9.6 → 13.4 (con la sola parte 1 erano 17), punizioni 31.5 → 39.1 fuori banda. La causa che resta dei fuorigioco: la lettura della linea per proiezione lungo lo sguardo (OI-104, OI-108).

## Scartate

- Lasciare "restare" come unica opzione sul passaggio: un uomo senza scelta non è un attore (ADR-049).
- Togliere ai compagni il movimento sul passaggio per contenere i fuorigioco: misurato (6 su 22, fuorigioco 9), ma toglie una scelta che nel calcio vero c'è; la causa è nella lettura, non nella scelta.
