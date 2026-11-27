# ADR-117 — The air goes to the man who gets there (DD-97)

- **Status:** Accepted (decision of the maintainer, 2026-11-27)
- **Date:** 2026-11-27
- **Decision:** DD-97
- **Depends on:** ADR-042 §3, ADR-051, ADR-077, ADR-081 (choice C), ADR-040
- **Amends:** ADR-042 §3 for aerial contacts only

## Il problema

`resolve_aerial` (`L0-match/crates/ds-orchestrator/src/lib.rs:700-719`) sceglie
il vincitore prendendo il massimo di `aerial_force`: testata, forza esplosiva,
forza massima, altezza e mani del portiere. Chi arriva prima al pallone e chi
è effettivamente alla sua portata non contano. Nella misura del 223
(`context/plans/L0-CALIBRATION.md:315`) i duelli aerei sono 8,10 a partita
contro 38,14 ± 12,98 della partita vera; ADR-081 (C) ha già dato il secondo
uomo a ogni palla alta, ma l'esito resta una gara di forze astratta.

Questa non è una distanza da colmare scegliendo un peso guardando la banda:
sarebbe A5-bis. È una domanda di modello: chi incontra il pallone in aria.

## I filtri

- **partita vera:** il dito è il passo del tick in cui il pallone passa
  all'altezza del corpo e la distanza dal punto; la luna è lo stesso uomo che
  arriva lì e sceglie COME contendere. Un referto registra l'esito del duello,
  non la forza con cui è stato vinto.
- **LUNA:** la regola non toglie nessuna opzione al giocatore e non fissa un
  esito voluto: dice soltanto quali corpi sono in contatto con il pallone e in
  che ordine lo incontrano (ADR-051 §1-3, ADR-081 C). La scelta resta nel COME
  giocare quell'arrivo.

## Decisione

L'esito di un duello aereo si legge nell'ordine seguente, senza numeri nuovi:

1. **Portata.** Un uomo dentro il raggio di presa vince sempre su chi può
   soltanto contendere spalla a spalla (la distinzione già presente in
   `technique.rs::aerial_contenders`).
2. **Arrivo.** A parità di portata, vince chi raggiunge il pallone nel passo
   più presto dello stesso tick. Il passo è già calcolato dal percorso del
   pallone, non è una nuova soglia.
3. **Forza.** Se due uomini lo raggiungono nello stesso passo, decide
   `aerial_force` come oggi; in caso di pareggio resta il primo della lista
   (ADR-019).

Questo emenda ADR-042 §3 soltanto per il contatto aereo: il contatto decide
ancora l'esito, ma il contatto include l'ordine in cui i corpi raggiungono il
pallone. Nessun parametro, campo di stato, opzione o gesto nuovo: il calcio
dietro il pallone non diventa una scelta «manda fuori» (DD-94).

## Conseguenze

- `resolve_aerial` legge l'arrivo e la portata insieme alla forza; l'unico
  chiamante in `technique.rs` passa i dati già disponibili (passo del percorso
  e distanza dalla portata).
- Un gemello a scena identica mostra che chi arriva prima vince anche se è
  più debole in aria, e che a parità di arrivo la forza decide ancora.
- `SIM_VERSION` +1, golden e semi rigenerati, voce in `CHANGELOG-SIM.md`,
  misura `ds-reference 42..101` in `L0-CALIBRATION.md`. Nessun numero scelto
  guardando il risultato della misura.

## Scartate

- **Tenere solo `aerial_force`.** È lo stato attuale: ignora l'arrivo e la
  posizione che ADR-051 e ADR-081 C chiedono di leggere.
- **Aggiungere un peso per arrivo/posizione tarato sulla banda.** A5-bis.
- **Riscrivere il duello a terra con la stessa regola.** L'ordine di arrivo
  conta nel volo; il duello a terra ha già la sua geometria (ADR-042). Fuori
  da questa decisione.
