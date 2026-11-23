# ADR-111 — Il corpo e l'errore: lo scatto vero, la frenata, il lancio lungo (DD-53)

- **Status:** Accepted (delegated, 2026-11-23) — da rivedere da parte del maintainer
- **Date:** 2026-11-23
- **Decision:** DD-53
- **Attua:** fase 4 di `PITCH-AUDIT.md` (il corpo); ADR-052 (il giocatore conosce il proprio piede); Legge 12
- **Depends on:** ADR-051, ADR-054, ADR-058, ADR-069, ADR-110

## Il problema

Gli autogol erano 1.95 a partita contro 0.10 reali. Sonde sui record (60 partite): più della metà sono lanci di 30-45 m all'indietro al proprio portiere, giocati sotto pressione, che arrivano lunghi e gli passano sopra la testa dentro la porta; gli altri, passaggi rasoterra al portiere che gli scorrono a 1-2 m.

## Decisione

1. **L'intercetto conta la velocità solo verso il punto.** `SpatialState::intercept` calcolava dove un uomo incontra la palla con tutta la sua velocità, in qualunque direzione corresse. Un portiere che scivolava lungo la linea sceglieva un punto lontano sulla traiettoria, non lo raggiungeva, e la palla gli passava accanto. Ora conta la sola componente della sua velocità verso ogni punto; chi corre di lato o all'indietro parte da zero in quella direzione. Gemello `a_man_running_away_meets_the_ball_no_sooner_than_one_standing`.
2. **Lo scatto ha il profilo vero.** `movement.json` dava 2.5-5 m/s² costanti fino alla velocità massima, letti da Harper, Carling e Kiely 2019, che però sono le soglie con cui il GPS conta un'accelerazione in partita, non la capacità. Ora: il profilo accelerazione-velocità misurato in partita su professionisti (Alonso-Callejo et al. 2022, *Scientific Reports* 12:19266, doi 10.1038/s41598-022-23790-w): A0 8.26-8.68 m/s² per ruolo (DS 0.36-0.81), S0 9.22-9.50 m/s, accelerazione che cala in modo lineare con la velocità. `accel_min` e `accel_max` = media ± 2 DS (7.2 e 9.6 m/s²), da fermo; correndo, ridotta della quota della sua velocità di punta già raggiunta. Lo stesso profilo in `run_in` (l'intercetto). Velocità massima invariata (7.5-10 m/s, coerente con S0).
3. **Correndo a un punto si frena per arrivarci.** In un contrasto il motore faceva correre a tutta velocità fino al bersaglio, senza frenare. Con lo scatto vero il pressatore superava l'uomo e tornava indietro, 11 m per parte (il test `the_presser_takes_the_goal_side_of_the_carrier` lo ha visto). Ora chi va su un punto (l'uomo che pressa, la palla che incontra) frena per arrivarci, con la sua decelerazione, come già chi va a battere un calcio piazzato; chi entra in tackle entra a velocità.
4. **Il passatore sa che il suo lancio va lungo.** Il lancio porta la palla a `distanza × (1 + errore)` (`resolve_pass`). Il passatore ora conosce il proprio errore come il tiratore (ADR-052): il suo passaggio, il piede, la lettura del gioco, la pressione sul calcio come la legge il motore (`kick_pressure`), la sua compostezza. Se il lungo porta la palla oltre la sua linea di porta fra i pali, vede la palla nella sua rete: quel lancio non vale nulla. Per il rasoterra la stessa regola sulla deviazione laterale (`distanza × errore`) contro la portata del compagno più il tratto che copre mentre la palla rotola: a queste velocità non cambia nulla, è tenuta per coerenza.
5. **Legge 12, chiara occasione in area.** Dal 2016 la chiara occasione negata con un fallo in area, rigore concesso, con un tentativo di giocare il pallone, è un'ammonizione, non un'espulsione. Il motore la puniva sempre col rosso. Ogni fallo del motore è un intervento sul pallone (tackle, pressing): in area è giallo. `FoulFacts.denial` (`Denial::No | Chance | ChanceForTheBall`); gemello nel test di `card_for`.

Con LUNA: il dito è la velocità, la palla, la porta alle spalle del portiere; la luna è che un giocatore conosce il proprio corpo e il proprio piede, e non gioca un pallone che sa di non poter mettere dove vuole. Partita vera: nessuno lancia di 40 m al proprio portiere sotto pressione; chi pressa rallenta prima dell'uomo; il primo passo di un professionista è esplosivo.

## Conseguenze

- `SIM_VERSION` 192 → 194 (il 193 è riservato al branch `sim-193-law7-stoppage`, non su main). Golden rigenerati; semi coppa e rigori 90, supplementari 44 (pari) e 42; il primo infortunio del test `knock` passa al seme 109 (intervallo 42..121).
- `ReceptionReckoning.pass_miss_rad`, `control_radius_cm`, `ball_reach`; `read::own_pass_spread`; `actor::own_pass_miss`, `actor::believed_reaches`; `intercept` prende la velocità come vettore.
- Misura 42..101 in `L0-CALIBRATION.md`: **4 su 22** (main 192: 6). Autogol 1.95 → 1.05, fuorigioco 13.4 → 9.98, duelli aerei 7.6 → 16.6, gol 10.8 → 9.2, rinvii in banda. Falli 27.5 → 35.2 e rossi fuori banda (0.93, tutti scivolate da dietro fuori area entro 25 m: troppe occasioni, non la Legge); gialli 1.38 (il cartellino conosce solo la scivolata da dietro veloce: manca il fallo tattico che ferma un'azione promettente, OI da aprire); passaggi 1838, tiri 48.

## Scartate

- Tenere 2.5-5 m/s²: non è la capacità di un giocatore, è una soglia di conteggio.
- Fermarsi al passo 3 (6 su 22, il conteggio migliore): il pressatore che oscilla di 11 m attorno all'uomo è fisica sbagliata; A5-bis vieta di scegliere guardando la metrica.
- Toccare le soglie del cartellino (`reckless_speed_milli`, `excessive_speed_milli`) per i gialli e i rossi: sono di grado 3 e si tarerebbero guardando la metrica.
