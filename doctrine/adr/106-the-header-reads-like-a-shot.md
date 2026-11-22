# ADR-106 — Il colpo di testa in area è letto come un tiro (DD-48)

- **Status:** Accepted (delegated, 2026-11-22) — da rivedere da parte del maintainer
- **Date:** 2026-11-22
- **Decision:** DD-48
- **Attua:** fase 2 di `PITCH-AUDIT.md`: testata in area, tiro sotto pressione
- **Depends on:** ADR-042 (duello aereo), ADR-046 (frase del tiro), ADR-052 (valore delle opzioni), ADR-054 (contatto), ADR-057 (composure), ADR-087 (distanza minima di errore)

## Decisione

Quando il vincitore di un duello aereo è nell'area avversaria, sceglie il colpo
di testa leggendo la stessa scena del tiro: palo, portiere, pressione del
contatto, propria composure. La fisica del colpo usa `resolve_shot` con
`is_header=true`: l'errore deriva da `heading` (non dal piede), non c'è corsa,
attraversamento del corpo o fase del passo; potenza e altezza restano scelte
del giocatore. Nessun numero nuovo, nessun clamp, nessuna opzione rimossa.

Con LUNA: il dito è la palla alta, il contatto, la posizione del portiere, la
distanza e il tick; la luna è lo stesso giocatore che legge la porta e sceglie
dove e come colpire di testa, con le proprie abilità. La regola e l'evento
restano gli stessi (`Shot`/`Header`).

## Conseguenze

- SIM_VERSION 187 → 188. Golden e semi rigenerati.
- Gemello `a_header_in_front_of_goal_is_a_shot_choice`: il read restituisce un
  candidato di testa mirato davanti alla porta.
- `ShotContext.is_header` distingue il colpo di testa dal tiro di piede;
  `ShotType::Header` resta derivato.

## Scartate

- Lasciare il colpo di testa fisso al centro: ignora la lettura del giocatore.
- Introdurre una formula di header separata: duplicherebbe il modello del tiro
  senza causa nuova.
