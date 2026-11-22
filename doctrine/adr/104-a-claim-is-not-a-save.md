# ADR-104 — La presa del portiere non è una parata (DD-46)

- **Status:** Accepted (delegated, 2026-11-22) — da rivedere da parte del maintainer
- **Date:** 2026-11-22
- **Decision:** DD-46
- **Attua:** la contraddizione interna della misura SIM 186 (parate 20,80 > tiri in porta 17,45)
- **Depends on:** ADR-021 (l'arbitro non è fisico), ADR-039 (read/choose/contact/band), ADR-045 (il portiere è un attore), ADR-052 (il valore di uno spazio), ADR-065 (il registro della partita è un contratto)

## Decisione

Nel registro della partita il portiere che prende un cross area emette
`Save` con `claim: true`: è un'azione del portiere sull'area, non una parata
di un tiro. `ds-reference` conta come **parate** solo gli eventi `Save` che
non portano quel flag. Nessun evento viene cambiato, rimosso o rinominato, e
nessun numero di `balance/` viene toccato.

Con i due filtri: il **dito** è l'evento `Save`, il suo flag `claim`, il tick
e l'evento `Shot` che lo precede (o non lo precede); la **luna** è lo stesso
portiere che sceglie di uscire e prendere il pallone invece di pararlo — la
sua lettura del cross e la sua tecnica non cambiano. Cambia soltanto cosa il
registro *dice* di quell'atto: non una parata, una presa.

La ragione non è la banda della metrica, ma un'impossibilità interna: ogni
parata segue un tiro in porta, quindi le parate non possono superare i tiri
in porta. Al SIM 186 le parate 20,80 superano i tiri in porta 17,45. Sul seed
42 il registro ha 21 eventi `Save`, dei quali 9 con `claim: true` e 11 tiri in
porta: tolte le prese restano 12 parate, coerenti con gli 11 tiri.

## Conseguenze

- Nessun SIM bump, nessun golden o seme rigenerato: il motore non cambia.
  Cambia la lettura del registro in `tools/ds-reference/src/engine.rs`.
- Il gemello `a_keepers_claim_is_not_a_save` usa la stessa partita (seed 42):
  cambia solo il flag `claim`. Senza la distinzione le parate sono 21; con la
  distinzione sono 12, e la differenza è esattamente il numero di prese.
- La misura `ds-reference --seeds 42..101` si rilegge senza adattare numeri
  (A5-bis). Il conteggio delle metriche in banda può salire di una unità; non
  è un obiettivo di taratura, è la conseguenza di aver contato solo le parate.

## Scartate

- Cambiare l'evento del motore da `Save` a `Claim`: introduce un tipo nuovo
  e cambia il contratto del registro (ADR-065) e i golden, senza guadagnare
  nulla nella lettura: il flag `claim` c'è già.
- Ritoccare la banda delle parate: la banda è il dato reale, non un peso.
- Ignorare la contraddizione perché «tanto le parate seguono i tiri»: è il
  difetto di conteggio che questa decisione rimuove.

## Note datate

- **2026-11-22 — Status: accepted (delegated, DD-46).** Nessun numero nuovo,
  nessun comportamento in campo cambiato: solo il significato del conteggio.
