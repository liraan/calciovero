# ADR-100 — Il pericolo è il valore dell'attore (DD-42)

- **Status:** Accepted (delegated, 2026-11-22) — da rivedere da parte del maintainer
- **Date:** 2026-11-22
- **Decision:** DD-42
- **Attua:** la nota "da fare separatamente" di ADR-099 (D5)
- **Depends on:** ADR-035 (nessun dado), ADR-039 (read/choose/contact/band), ADR-043 (l'attore decide), ADR-052 §5 (il valore degli spazi e la corsa del passaggio), ADR-071 §5 (la scena senza palla appartiene all'orchestratore), ADR-099 §4 (l'orchestratore possiede lo stato della lettura)

## Decisione

Il valore dei tre pericoli che un difensore pesa — lo spazio del portatore,
quello dell'uomo che marcerebbe, quello che guarda — è calcolato
dall'orchestratore, nello stesso crate che poi sceglie l'atto. Il motore
fornisce solo i fatti della scena (`ThreatFacts`: dove l'uomo sta, dove vede
gli altri, la dimensione del campo, l'attrito del terreno). Nessun valore
arriva già calcolato dal motore, e nessuna regola è duplicata: i punti di
copertura e di corsia, l'uomo da marcare e il marcatore più vicino restano
nel motore, che li passa come fatti.

Con i due filtri: il **dito** è la posizione del portatore, dell'uomo da
marcare e dei compagni, le dimensioni del campo, il tick; la **luna** è lo stesso
difensore che legge quelle posizioni e pesa quanto vale per lui ogni spazio,
prima di scegliere come chiuderlo. Cambia chi possiede il valore, non
l'etichetta dell'atto, non la Legge, non la fisica del passaggio.

## Conseguenze

- `TeamView::threats` e `TeamView::reaches` escono da
  `ds-match/src/state.rs`; `offball::threats` e `offball::reaches` li
  sostituiscono in `ds-orchestrator`, con la stessa aritmetica. Il gemello
  `a_man_no_pass_can_reach_is_less_of_a_danger` (OI-117 (A), SIM 134) si
  sposta con la regola; il nuovo gemello `the_danger_is_the_actors_own_reading`
  mostra che il valore dipende da ciò che l'attore vede ed è deterministico.
- Nessun numero nuovo in `balance/`, nessuna formula cambiata, nessun
  comportamento nuovo: i golden e i semi restano identici. Il SIM bump è
  dovuto al cambio di confine tra crate, non a una taratura.
- La scena passa ancora le posizioni vere, come prima: rendere il valore
  dipendente dalla percezione è un passo successivo, con la sua misura e il
  suo SIM bump, non parte di questa decisione.

## Scartate

- Calcolare i pericoli in `ds-match` e passare `Threats` già fatti
  all'orchestratore: lascerebbe il valore dell'attore in mano al motore,
  che è proprio il difetto che ADR-099 §4 chiude per la latenza.
- Duplicare `step_towards`, `marker` e `man_to_mark` nell'orchestratore:
  sarebbero due regole per gli stessi punti (RULES A5); il motore li calcola
  una volta e li passa come fatti.
