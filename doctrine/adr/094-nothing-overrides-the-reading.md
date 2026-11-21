# ADR-094 — Il passaggio filtrante è una lettura, non una geometria (DD-35)

- **Status:** Accepted (delegated, 2026-11-21) — da rivedere da parte del maintainer
- **Date:** 2026-11-21
- **Decision:** DD-35
- **Estende:** ADR-047 (l'istinto è riconoscimento), ADR-052 (ogni giocatore pensa), ADR-050 (il progresso del passaggio)
- **Depends on:** ADR-035 (nessun dado), ADR-043 (l'attore decide), ADR-048 §4 (numeri dichiarati), ADR-083 (`BallState` resta etichetta)

## Contesto

Il maintainer, 2026-11-21: «si procedi e ripristina». La bozza iniziale di
questo ADR chiedeva di rimuovere cinque vincoli insieme (il rango fisso nella
corsa alla palla libera, l'esclusione del portatore dalla marcatura, il ritiro
di `Press`, la griglia 3×3 del posto, il calcolo di `brings()` gettato via).
L'autorizzazione data copre solo la **modulazione del passaggio filtrante** e il
**ripristino dell'istinto** di `goes_for_loose_ball`: gli altri quattro vincoli
restano sospesi, non attuati e non dichiarati risolti.

Il difetto era nel filtrante. `through_candidates` (`ds-orchestrator/src/read.rs`)
spostava il punto d'incontro in avanti di un passo **generico** dell'uomo
che corre, preso da `ThoughtBalance::believed_speed_cm_per_tick`. Due coppie con
intesa diversa, due portatori con lettura diversa e due compagni con passo
diverso mandavano il pallone esattamente nello stesso punto: la geometria
decideva per loro.

Filtrando con LUNA: il **dito** è la corsa che il portatore vede e la velocità
che legge da quella corsa, entro il cono; la **luna** è lo stesso portatore che
decide quanto fidarsi di quella lettura — con l'intesa che ha con il compagno e
con la propria `game_reading` — e sceglie dove mandare il pallone. Cambia il
come esegue il passaggio, non chi prende il pallone, non la fisica della palla
(ADR-083) e non il diritto del compagno a correre.

## Decisione

1. In `thought.rs`, per ogni compagno che corre e che il portatore vede,
   `scene.run_pace` riceve il passo **visto** (`scene.motion`, cioè la norma di
   `agent.velocity` attraverso `dmath::hypot`), ponderato fra il passo generico
   già dichiarato e quello visto:

   `believed = (intesa * game_reading).mul_add(seen_pace - generic_pace, generic_pace)`

   `intesa` è la già esistente memoria di coppia, `game_reading` la latente del
   portatore già letta per il filtrante. Con intesa o lettura a zero si resta al
   passo generico di prima: nessun comportamento nuovo per una coppia che non si
   conosce. Nessun numero nuovo, nessun dado (ADR-035), nessun latente nuovo
   (ADR-047 §4): l'intesa e la lettura sono la forma in cui il portatore
   riconosce la corsa del compagno.

2. In `read.rs`, `through_candidates` usa `scene.run_pace` del singolo compagno
   (default: il passo generico, minima 1 cm/tick) al posto del passo generico.
   Il punto d'incontro resta calcolato dalla stessa formula, con il tempo di
   volo della palla e il lead già limitato a 2000 cm: cambia solo di quanto il
   portatore crede che quel compagno corra.

3. `offball.rs` `goes_for_loose_ball`: il rank 0 torna a essere istinto (`true`
   a qualsiasi distanza), come prima della bozza. La corsa alla palla libera non
   è un premio al più vicino per regola, è il riconoscimento del giocatore che
   vede quella palla (ADR-052, ADR-084); il gate `seen.contains(id)` resta,
   nessuna lettura onnisciente del target altrui.

## Verifica sulla partita vera

Confronto A/B, stessi semi, stessi input, SIM 178 contro SIM 179:

| seed 42 | SIM 178 | SIM 179 |
| --- | --- | --- |
| passaggi | 1255 | 1195 |
| filtranti | 20 | 22 |
| passaggi completati | 1130 | 1087 |
| gol | 9 | 5 |
| carrier oltre 90 / 200 / 500 cm | 1398 / 160 / 0 | 1460 / 225 / 0 |
| picco del gap | 373 cm | 370 cm |

| seed 53 | SIM 178 | SIM 179 |
| --- | --- | --- |
| passaggi | 1205 | 1251 |
| filtranti | 20 | 24 |
| passaggi completati | 1113 | 1140 |
| gol | 3 | 18 |
| carrier oltre 90 / 200 / 500 cm | 1456 / 152 / 0 | 1393 / 173 / 0 |
| picco del gap | 352 cm | 377 cm |

I filtranti aumentano su entrambi i semi A/B e, sui 12 semi 42-53, salgono in
5 casi, restano uguali in 3 e calano in 4: il resto è divergenza caotica
legittima di una simulazione deterministica (un passo diverso cambia gli
antecedenti di tutto il seguito). Il salto di gol del seed 53
e l'episodio di 1141 cm del seed 46 (34 tick oltre 500 cm, tick 38857-38891,
inizialmente un ricevitore che corre via dal primo tocco) sono **non risolti**:
rientrano nel caso del primo tocco già aperto in OI-123 e non in questa
decisione. Nessuna statistica è stata usata per scegliere un numero: gli unici
fattori sono intesa, lettura e passo già visti.

Guardiano gemello: `the_pace_he_believes_of_the_runner_moves_the_through_ball`
— stessa corsa, cambia solo l'intesa/lettura del portatore e il passo del
compagno; il punto del passaggio cambia. Verdi anche
`a_man_sent_running_stops_at_his_place` e `no_restart_stops_the_match`, che
bloccano le riprese: il primo modello, che usava la velocità latente del
compagno, lanciava il pallone troppo lungo e faceva morire il pallone 1213 tick
su seed 50 contro la soglia di 1212; leggere il passo **visto** riporta il
guardiano verde senza toccare la soglia.

## Scartate

- **Usare `hidden.top_speed_now` come passo del compagno:** è una lettura
  onnisciente del latente altrui e fa scadere il guardiano delle riprese; il
  portatore può fidarsi solo del passo che vede.
- **Cancellare il passo generico:** la memoria di coppia e la lettura sono un
  peso sulla percezione, non una sostituzione: chi non conosce il compagno resta
  al passo dichiarato, non indovina.
- **Attuare gli altri quattro vincoli della bozza:** non autorizzati in DD-35;
  restano aperti come scelte separate, con la loro partita vera e il loro SIM.

## Conseguenze

`SIM_VERSION` 179, golden e semi rigenerati (coppa 48 → 64, pareggio al 90'
44 → 49, decisivo 42 → 50, shoot-out di soli tiri 86 → 64). La festa dopo il
gol resta fuori: non è una correzione di una lettura, è un nodo di decisione che
non esiste e si affronta con SIM 180. Le decisioni delle Leggi del gioco restano
in `plugins/referee.rs` (DD-31).
