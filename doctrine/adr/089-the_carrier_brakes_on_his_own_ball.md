# ADR-089 — Anche il portatore frena quando il suo tocco è scaduto (DD-29)

- **Status:** Accepted (delegated, 2026-11-20) — da rivedere da parte del maintainer
- **Date:** 2026-11-20
- **Decision:** DD-29
- **Amends:** ADR-088 (DD-28, SIM 172/173: la frenata era solo per `sent`)
- **Depends on:** ADR-035 (nessun dado), ADR-051 (la scena letta), ADR-069 (passi piccoli), ADR-082/DD-22 (palla libera)

## Contesto

Il maintainer ha autorizzato il fix del 12'30" e chiesto il confronto sulla
partita vera. Filtrando con LUNA, l'atto non è una ripresa: in open play il
portatore ha giocato il pallone avanti, il suo `Touch` ha il periodo scaduto e
lui rilegge la scena con il pallone come target. AIR-088/DD-28 aveva insegnato a
frenare a chi è mandato a un punto (`sent`), ma `move_agents` dava ancora
`top_s` a chi contesta: il portatore rincorreva il proprio pallone a tutta
velocità, gli arrivava a 9 cm a t=7490, lo superava di 255 cm e tornava
indietro. Stesso gesto del calcio d'inizio, in un altro atto della partita.

## LUNA

Il dito: il `Touch` scaduto, il target che torna al pallone, la distanza in
centimetri, il contatore di tick, la velocità `top_s`.

La luna: lo stesso giocatore, con la stessa velocità, lo stesso freno e la
stessa lettura della scena; cambia solo che il suo tocco è finito e lui rilegge
dove il pallone è ora. Non gli viene insegnato un nuovo gesto e non gli viene
detto chi deve prendere il pallone: gli viene tolto il permesso di portare
momentum oltre il punto che ha scelto. L'agency resta nell'esecuzione, non nel
quale atto sceglie; la tattica non entra.

## Decisione

1. **La frenata di ADR-088 vale anche per il portatore che torna al proprio
   pallone.** In `move_agents`, il ramo `contests` usa già
   `top_s.min((2.0 * decel_s * (dist - BODY_RADIUS_CM).max(0.0)).sqrt())` per
   `sent.is_some()`; ora vale anche quando `overdue && Some(id) == carrier`.
   Nessun numero nuovo: 45 cm è il raggio del corpo, la decelerazione è quella
   dichiarata in `movement.json`.
2. **Gli altri contrasti non cambiano.** Pressing, challenge e corse dietro a
   un pallone vivo restano a `top_s`: la frenata è solo per chi arriva al
   pallone che ha appena giocato.
3. **Il gemello del burattino regge.** A scena uguale il giocatore sceglie lo
   stesso atto; cambia solo come arriva. Il guardiano
   `the_carrier_brakes_when_his_touch_runs_out` falliva prima (122 cm di
   superamento al tick 18) e passa ora, insieme a
   `a_man_sent_running_stops_at_his_place`.

## Verifica sulla partita vera

Confrontate le partite reali del seed 42 prima e dopo: il tick 7470 non è più
lo stesso stato (il fix agisce anche sugli antecedenti), quindi il confronto è
per atto, non per posizione coincidente. Nel nuovo 12'28"-12'30" il portatore
110 arriva al pallone con la distanza che scende a 5 cm e resta entro il raggio
di arrivo, senza il viaggio avanti e indietro di 255 cm. I golden SIM 174 sono
rigenerati; il seme 65 è il primo pareggio al 90', il 111 il primo shoot-out i
cui calci sono tutti tiri (il 77, usato dal golden, ha un `Pass` del portiere
dopo una parata: atto da tenere d'occhio, non corretto in questo ADR).

## Scartate

- **Agganciare la palla al piede:** rende il pallone una parte del corpo,
  contraddice ADR-053 e sposta il controllo fuori dal giocatore.
- **Abbassare `top_s` per tutti i portatori:** rallenterebbe anche chi sta solo
  occupando il proprio spazio; il difetto è nell'arrivo, non nella velocità.
- **Ritoccare la statistica dei possessi:** decisions-009 — il referto non è il
  bersaglio, si corregge la logica dell'atto.

## Conseguenze

- `SIM_VERSION` 174, golden rigenerati, voce in `CHANGELOG-SIM.md`, P1 in
  `spatial.rs`, semi dei test `extra_time` aggiornati. Nessun merge in `main`
  senza autorizzazione.
- Resta fuori dal fix il `Pass` nei rigori del seed 77: è un atto diverso e
  richiederebbe un'altra decisione con il suo `SIM_VERSION`.
