# ADR-123 — La spazzata legge i guanti, non lo slot (DD-103)

- **Status:** Accepted (delegated, 2026-11-28) — da rivedere da parte del maintainer
- **Date:** 2026-11-28
- **Decision:** DD-103
- **Depends on:** Law 3, ADR-121, ADR-051, ADR-063
- **Amends:** ADR-121 (chiude l'ultimo residuo T22 del portiere letto per slot)

## Il problema

T22 dell'audit: `technique.rs::play_clearance` calcolava la pressione sul
calciatore saltando `players.iter().skip(1)`, cioè il primo uomo della rosa.
Finché la rosa coincide con l'undici in campo quello è il portiere, che sulla
propria linea non pressa il calciatore. Dopo il SIM 228 i guanti passano subito
all'uomo più vicino alla porta quando il portiere esce in gioco, mentre la rosa
viene aggiornata solo alla sosta: fino al fischio il primo della rosa è un
difensore di movimento e il vero portiere è un altro. La spazzata continuava a
non sentire la pressione del primo e a sentire quella del portiere, il contrario
di ciò che è in campo.

Le altre due voci T22 (`denial` e muro) sono già chiuse dall'ADR-121:
`referee.rs::denial` e `set_pieces.rs::form_wall` leggono `spatial.goalkeeper`.
Questa ADR chiude la terza.

## I filtri

- **partita vera:** il dito è l'uomo che in quel momento ha i guanti e si è
  avvicinato al calciatore; la luna è lo stesso calciatore che, con la stessa
  tecnica, sente la pressione di chi ha addosso e sceglie come spazzare. Il
  fatto che il portiere non pressi quando sta sulla sua linea viene da dove
  sta, non dal numero di maglia.
- **LUNA:** nessun gesto, opzione o capacità viene tolta al calciatore: cambia
  solo quale avversario entra nel calcolo della pressione che il suo calcio
  già usa. L'uomo che ha preso i guanti resta lo stesso giocatore, con la sua
  identità e le sue scelte.

## Decisione

1. `play_clearance` esclude dal calcolo della pressione
   `self.spatial.goalkeeper(...)` — il portiere corrente — invece del primo
   uomo della rosa. Nessun numero, raggio, soglia o parametro nuovo.
2. Il gemello `the_clearance_pressure_follows_the_gloves_not_the_slot` tiene
   fissi calciatore, pallone e avversari nelle stesse posizioni: cambia solo
   chi ha i guanti. Con il vecchio `skip(1)` i due calci sono identici; con la
   lettura del portiere corrente la pressione cambia e il calcio esce diverso.

## Conseguenze

- `SIM_VERSION` 230, voce in `CHANGELOG-SIM.md`. Nessun evento, campo, numero
  o gesto nuovo; nessun valore preso guardando la banda (A5-bis).
- Nessun seme dei golden e della coppa contiene un cambio di portiere in gioco:
  gli hash e i semi restano identici, aggiornati solo i commenti di versione.
  Misura `ds-reference 42..101` scritta in `L0-CALIBRATION.md` §230; ogni riga
  fuori banda conserva la sua causa.
