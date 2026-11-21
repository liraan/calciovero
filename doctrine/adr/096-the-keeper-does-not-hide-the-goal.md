# ADR-096 — Il portiere cambia il valore del tiro, non la vista della porta (DD-38)

- **Status:** Accepted (delegated, 2026-11-21) — da rivedere da parte del maintainer
- **Date:** 2026-11-21
- **Decision:** DD-38
- **Estende:** ADR-046 (l'attore sceglie il palo), ADR-047 (percezione senza onniscenza), ADR-052 (il tiro è una scelta)
- **Depends on:** ADR-035 (nessun dado), ADR-043 (l'attore decide), ADR-048 §4 (numeri dichiarati), ADR-094 (niente sostituisce la lettura)

## Contesto

Dopo SIM 181 il maintainer guarda la GUI e segnala: «invece di tirare in porta
passano ai compagni». La verifica sul setup esatto della GUI (`friendly::apply`,
seed 42) trova due situazioni concrete: tick 32760, slot 10 a 314 cm dalla porta,
e tick 60021, slot 105 a 761 cm. In entrambi i casi il portatore passa.

Con una sonda temporanea sul decisione (rimossa) si legge il perché: lo slot 10
in (9999, 4336) ha un tiro con `geometric = 15.526` e `visibility = 0.083`,
mentre i passaggi hanno visibilità 0.333..1.0; lo slot 5 in (9729, 3069) ha un
tiro valido con `geometric = 28.855` e `visibility = 0.705`, ma il tiro verso il
palo coperto ha visibilità 0.000. Il filtro della lettura butta via il tiro
proprio quando il portiere è sulla linea di tiro.

La causa è `keeper_visibility`: una rampa sulla distanza dal portiere al segmento
tiro-palo che azzerava la visibilità del tiro coperto. Ma la copertura del
portiere è **già** nel valore del tiro (`chance`, poi `geometric`, ADR-046): lo
stesso fatto fisico veniva contato due volte, una come valore e una come
percezione. Il risultato è un errore di lettura, non una lettura del gioco: il
giocatore non vede più la porta perché un uomo ci sta davanti.

Qui il **dito** è l'apertura della porta dal punto in cui si sta, le linee dei
pali, la posizione del portiere, il tick. La **luna** è lo stesso attaccante che
riconosce che la porta è lì e sceglie quanto vale ciascun palo: cambia il come
legge la scena, non il gesto deciso, non la fisica e non l'etichetta
dell'evento. Un passaggio resta un passaggio anche quando è sbagliato.

## Decisione

1. In `shot_candidates` la visibilità di ogni tiro è `goal_cone(pos, goal)`,
   la stessa apertura geometrica già usata per le altre letture. Nessuna rampa
   sul portiere nel filtro della percezione.

2. Il helper `keeper_visibility` è cancellato, non mantenuto con un pavimento:
   nessuna regola ridondante, nessun numero nuovo, nessun dado (ADR-035).

3. Il portiere resta nella scena e continua a cambiare il **valore** dei tiri
   attraverso `chance`/`geometric`: la lettura del gioco resta al portiere, la
   vista della porta resta al tiratore.

## Verifica sulla partita vera

- Gemello `the_keeper_changes_the_shots_worth_not_his_sight_of_the_goal`: stesso
  tiratore in area, solo il portiere si sposta dal palo coperto all'altro lato.
  La visibilità di ogni tiro resta uguale al cono della porta; il valore del
  miglior tiro cambia. È l'invariante che la vecchia rampa violava: con il
  portiere sulla linea la visibilità crollava a 0.000.
- Il vecchio test `the_keeper_hides_the_shot_he_covers_and_only_that_shot`
  affermava esattamente la confusione rimossa ed è stato tolto; il modulo di
  test prende il nome `shot_perception_tests`.
- `ds-reference compare --reference L0-match/reference/serie-a-2017-18.json
  --seeds 42..51`: gol 9.20 → 11.90, tiri 35.80 → 49.30, tiri in porta
  15.90 → 23.70, passaggi 1361 → 1374.70 (il valore 181 era la base di
  confronto). La conversione resta dello stesso ordine (26% → 24% dei tiri in
  gol): più tiri sono l'effetto atteso della lettura corretta, non una nuova
  patologia tarata. I valori assoluti restano lontani dal reference e la
  divergenza tra semi è caotica, come già dichiarato per SIM 179.
- Golden e semi rigenerati: hashing SIM 182, coppa 55 → 81, pareggio al 90'
  46 → 53, decisivo 50 → 42, shoot-out di soli tiri 55 → 81.

## Scartate

- **Tenere la rampa con un pavimento più alto:** lascerebbe la percezione
  dipendere dalla copertura, cioè il doppio conteggio che causa il difetto.
- **Togliere il portiere dal valore del tiro:** il portiere deve incidere su
  quanto il tiro è buono (ADR-046), non su se la porta si vede.
- **Aggiungere una soglia di distanza minima prima di tirare:** un numero nuovo
  e una regola che non spiega perché il tiro era invisibile.
