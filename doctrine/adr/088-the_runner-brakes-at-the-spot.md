# ADR-088 — Chi corre a una ripresa si ferma sulla palla (DD-28)

- **Status:** Accepted (delegated, 2026-11-20) — da rivedere da parte del maintainer
- **Date:** 2026-11-20
- **Decision:** DD-28
- **Amends:** SIM 150 (`send(id, spot, true)`, SIM 150: non teletrasportare 22 uomini in un tick)
- **Depends on:** ADR-035 (nessun dado), ADR-052 (la scena letta), ADR-069 (passi piccoli, corpo continuo)

## Contesto

Il maintainer in 2.5D (seed 42): «prima di controllare la palla si avvicina e
passa oltre poi torna sui suoi passi e la controlla» — non un caso, ma una cosa
che si vede «in continuazione in partita». Filtrando con LUNA, il gesto non è
quello di DD-22/ADR-082: la palla è in attesa di ripresa, fuori gioco, e chi la
supera è il **battitore designato** a ogni punizione, angolo e calcio d'inizio.

`call_restart` manda il battitore al punto con `send(taker, spot, true)`. Il
`run` fu introdotto in SIM 150 per dire «vacci di corsa, ma al tuo passo, senza
saltare 10-27 m in un tick come prima». In `move_agents`, però, `run` diventava
`contests`, e a chi contesta il motore dà `top_s` senza frenata: il battitore
arrivava a tutta velocità (75-100 cm/tick, `movement.json`), superava il punto
anche di 6 m, girava e ci riprovava finché l'arbitro (`within`) non lo trovava
abbastanza vicino per riprendere. La sosta si allungava e il gesto si vedeva a
ogni ripresa.

## LUNA

Il dito: la posizione del battitore, il punto della ripresa, la distanza in
centimetri, il contatore di tick, il `send(id, spot, true)`.

La luna: lo stesso giocatore, mandato a battere la ripresa, con la stessa
velocità, la stessa accelerazione e lo stesso freno che ha in ogni altro istante
della partita. Non cambia identità davanti alla barriera: cambia solo che il
suo corpo riceve «corri» senza ricevere «frena quando arrivi». L'agency è
nell'esecuzione del gesto — come arriva — non in quale gesto sceglie. La tattica
non entra: nessun ordine dalla panchina dice dove fermarsi.

## Decisione

1. **Chi è mandato a un punto frena per arrivarci.** In `move_agents`, quando
   `sent` contiene il giocatore, la velocità desiderata è
   `top_s.min((2.0 * decel_s * (dist - arrive).max(0.0)).sqrt())`, con
   `arrive = BODY_RADIUS_CM` (45 cm, SIM 173): la stessa frenata già usata per
   chi arriva in un punto qualsiasi, ma che si esaurisce appena **prima**
   dell'arrivo, non sul punto. «Run» resta il modo in cui va (non un
   teletrasporto), non il permesso di portare momentum nel calcio.
2. **Chi contesta un pallone vivo resta a tutta velocità.** La frenata vale
   solo per `sent.is_some()`: pressing, contrasti e corse dietro al pallone non
   cambiano. Nessun numero nuovo: la decelerazione è quella già dichiarata in
   `movement.json`.
3. **Il gemello del burattino regge.** Cambia l'esecuzione, non la scelta: a
   scena uguale il giocatore sceglie lo stesso atto. Il guardiano
   `a_man_sent_running_stops_at_his_place` in `spatial.rs` falliva prima (superava
   il punto al tick 14, dopo esserci arrivato all'11) e passa ora.

## La rifinitura del 173

Vista la 2.5D del 172, il maintainer dice «mooooolto meglio», ma «succede ancora
in misura minore»: «è come se avessero troppo momentum». La traccia del
calcio d'inizio del seme 42 lo mostra: il battitore entra nell'attesa a 106 cm
con 34 cm/tick, scorre fino a 15 cm oltre il punto, torna indietro. Il 172
frenava verso d=0, ma l'arbitro rilascia a 90 cm (`reach = 2*BODY_RADIUS_CM`):
il freno non era finito quando il piede era ancora in moto. Anticipando la fine
della frenata a un raggio corpo (45 cm), al rilascio il battitore è a 41 cm dal
pallone e non lo supera. Stesso filtro LUNA: il dito è la distanza dal pallone
al rilascio, la luna è lo stesso giocatore che arriva con il piede fermo.
Nessun numero nuovo — 45 e 90 cm sono il raggio del corpo e la portata
dell'arbitro già usati da `resume_restart` e `all_arrived`.
Provando a frenare esattamente a 90 cm il corpo si fermava a 96 cm, appena
fuori portata: l'arbitro non lo rilasciava più e la partita si bloccava
(25.000 tick). Per questo il freno finisce prima dell'attesa, non sull'attesa.

## Scartate

- **Rimetterlo sul punto con un salto:** è il teletrasporto che SIM 150 ha
  tolto (74 salti di 10-27 m a partita).
- **Abbassare `top_s` per tutti:** rallenterebbe ogni contrasto, non solo
  l'arrivo a una ripresa.
- **Aumentare la tolleranza dell'arbitro:** allungherebbe ancora la sosta e
  lascerebbe il gesto visibile.

## Conseguenze

- `SIM_VERSION` 173 (172: prima frenata; 173: freno esaurito entro il raggio
  corpo), golden rigenerati (primo seme di coppa con rigori 44, shootout pulito
  e riusato dal test degli eventi), voce in `CHANGELOG-SIM.md`, P1 in
  `spatial.rs` aggiornato; corretto anche l'off-by-one del guardiano del
  kick-off in `golden_scenes.rs` (il ritardo si misura dal tick del gol, non
  dal successivo): il 172 lo mascherava con l'overshoot.
- La ripresa non si allunga più e il battitore arriva con il piede fermo mentre
  si dispone la barriera. Nessun merge in `main` senza autorizzazione.
