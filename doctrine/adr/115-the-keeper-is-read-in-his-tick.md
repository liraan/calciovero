# ADR-115 — Il portiere è letto nel suo tick (DD-95)

- **Status:** Accepted (delegated, 2026-11-27) — da rivedere da parte del maintainer
- **Date:** 2026-11-27
- **Decision:** DD-95
- **Attua:** `context/plans/CODE-AUDIT.md` T10; `context/RULES.md` C3
- **Depends on:** ADR-035, ADR-051, ADR-062, ADR-069

## Il problema

In `advance_ball` la palla avanzava, veniva giudicato l'attraversamento della
linea (`check_shot_crossing`) e solo dopo si muovevano i corpi
(`move_bodies`). Il portiere che parava era quindi quello del tick precedente:
con LUNA, il dito è la palla che passa il piano della porta e la luna è
l'uomo che legge quel volo e sceglie COME andargli incontro; un corpo fermo
al tick prima non è la scena che ha letto. Era T10 in `CODE-AUDIT.md`, noto
dal 2026-11-16 e rimasto aperto «con un ADR».

## Decisione

`move_bodies` corre prima di `check_shot_crossing`, subito dopo
`spatial.advance_ball()`. La parata usa la posizione del portiere in quello
stesso tick, come già fa per ogni altro contatto nello stesso piccolo passo
(ADR-069). L'ordine delle altre fasi resta invariato.

La guardia `the_keeper_moves_before_the_shot_crosses` legge il sorgente di
`state.rs` e fallisce se i due passi si invertono: non è una nuova regola, è
l'invariante dell'ordine. Non è un gemello di scena perché non nasce né
scompare un atto: cambia soltanto a quale istante dello stesso tick si legge
un corpo già presente.

Con LUNA: non si toglie al portiere la scelta di uscire, restare o tuffarsi;
si smette di giudicare la sua scelta con il corpo di un istante fa.

## Conseguenze

- `SIM_VERSION` 222 → 223; golden L0 base/istruito/coppa, golden L1 e
  fixture rigenerati; `CHANGELOG-SIM.md` 223 e misura `ds-reference 42..101`
  in `L0-CALIBRATION.md`.
- La misura resta a 15 su 20 confrontabili in banda: gol 5,28 → 5,02
  (ancora fuori), parate 5,08 → 5,83, tiri in porta 9,85 → 10,23,
  rimesse 22,75 → 20,57 (peggiorano), duelli aerei 8,22 → 8,10. Non chiude
  nessuna delle cinque righe fuori banda: è correttezza di stato, non una
  taratura (A5-bis).
- `resolve_save`, la portata in `save.json`, la geometria dei pali e il
  giudizio dell'evento `on_target` non cambiano.

## Scartate

- **Tarare la portata del portiere per alzare le parate.** `save.json`
  dichiara i parametri arbitrari e la taratura rimandata a L3; sceglierli
  guardando i gol violerebbe A5-bis e la riga `saves` è già in banda (T11).
- **Spostare tutto l'aggiornamento dei corpi prima dell'avanzamento della
  palla.** Cambierebbe anche chi può toccare il pallone in volo, fuori da
  T10: il minimo necessario è spostare una sola chiamata.
- **Una nuova opzione di parata.** Il portiere ha già tutte le scelte; il
  difetto era nell'istante in cui veniva letto, non in quello che poteva fare.
