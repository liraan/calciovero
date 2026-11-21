# ADR-097 — Il tocco che non ha raggiunto torna sotto il piede (DD-39)

- **Status:** Accepted (delegated, 2026-11-21) — da rivedere da parte del maintainer
- **Date:** 2026-11-21
- **Decision:** DD-39
- **Estende:** ADR-089 (il portatore frena sul proprio pallone), ADR-092 (il primo tocco è del ricevitore), ADR-093 (l'uomo più veloce della palla se la porta con sé)
- **Depends on:** ADR-035 (nessun dado), ADR-043 (l'attore decide), ADR-048 §4 (numeri dichiarati), ADR-083 (`BallState` resta etichetta)

## Contesto

Dopo la correzione DD-38, SIM 182 faceva fallire il guardiano
`a_carrier_with_nothing_on_does_not_stand_still`: su seed 42 un portatore restava
fermo per 979 tick. La sonda temporanea (rimossa) descrive l'episodio:
tick 50689..51668, slot 9 di casa in (9965, 3540), pallone in (10122, 3588),
164 cm davanti al piede, `BallState::Possessed`, `ball_at_feet = false`,
azione letta `Press`, punteggio 1-3. L'ultimo tocco era del tick 50575, da
(9895, 2254), direzione (0.167, 0.986), velocità 58.6 cm/tick, periodo 5.0 tick,
`held_cm = 0`: un pallone giocato avanti, non uno tenuto sotto la suola.

Il tocco restava `due` ma mai `reached`: i corpi avversari tenevano il portatore
fermo a 164 cm dal punto, mentre il pallone si era fermato fuori portata.
`thought.rs` ritorna prima della scelta quando `!ball_at_feet()`, dunque nessuna
nuova decisione veniva presa; `follow_carrier` aspettava un incontro che non
sarebbe mai avvenuto. Un minuto e mezzo di partita su un solo uomo: è il
«comportamento molto strano» segnalato in GUI.

Qui il **dito** è il pallone fermo oltre la portata, il periodo del tocco scaduto
da due volte, il corpo dell'uomo trattenuto: fatti della scena. La **luna** è lo
stesso portatore che, non essendo riuscito a raggiungere il tocco che aveva
giocato, se lo riprende sotto il piede come farebbe con qualsiasi pallone che
non è riuscito a prendere. Cambia il come esegue il controllo, non la decisione
già presa (`carry_target` resta la stessa) e non la fisica del pallone.

## Decisione

In `follow_carrier`, quando il tocco è scaduto, non è stato raggiunto e sono
passati almeno due suoi periodi (`period > 0`), il pallone torna sotto il piede
con lo stesso `under_foot` usato per ogni altro pallone non raggiunto. Nessun
numero nuovo: il periodo è quello dichiarato dal tocco e la distanza è quella
già letta tra pallone e piede. Le due branchi vicine restano intatte: un pallone
morto preso dove arriva (`held_cm > 0`) e un tocco scarso che rotola via
continuano a essere controllati dal rispettivo ramo.

## Verifica sulla partita vera

- `an_unreached_touch_comes_back_under_his_foot`: lo stesso tocco giocato avanti,
  l'uomo fermo dai corpi; prima della correzione il pallone resta a 80+ cm
  (`(2501, 1550)`), dopo torna sotto il piede. Il test controlla anche che il
  recupero non avvenga prima che il tocco abbia avuto il suo tempo.
- I due guardiani del primo tocco, `a_ball_won_is_taken_where_it_is` e
  `a_man_running_into_the_pass_takes_it_with_him`, restano verdi: sono loro ad
  aver imposto la finestra di due periodi invece di «qualunque tocco scaduto».
- `a_carrier_with_nothing_on_does_not_stand_still` torna verde: i 979 tick di
  fermo su seed 42 spariscono.
- TUI seed 42, frame 75/76/77: il pallone passa dal portatore H a C a E, senza
  la sosta di un minuto e mezzo. Frame GUI registrati al tick 50689 senza errori
  (il controllo visivo delle immagini resta al maintainer).
- Golden e semi rigenerati insieme a DD-38 (hashing SIM 182, coppa 81,
  pareggio al 90' 53, decisivo 42, shoot-out di soli tiri 81).

## Scartate

- **Aspettare che la fisica segnali il pallone fermo:** un pallone morto sotto
  il piede ha velocità zero tanto quanto uno fuori portata: la condizione
  faceva scattare il recupero su un tocco appena preso, rompendo i due
  guardiani del primo controllo.
- **Recuperare a qualunque scadenza del tocco:** lo stesso errore, con la
  distanza di 80 cm che veniva accorciata prima del tempo.
- **Allargare la portata del portatore:** cambierebbe la geometria del duello e
  un numero dichiarato, non il solo recupero del proprio pallone.
