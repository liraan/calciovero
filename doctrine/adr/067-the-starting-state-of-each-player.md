# ADR-067 — The starting state of each player

- **Status:** Accepted (delegated, 2026-11-16) — to be reviewed by the maintainer
- **Date:** 2026-11-16
- **Decision:** DD-9, from decision AB (`context/plans/DECISIONS-2026-11-15.md`; `context/plans/L1.md`, "L'allenamento", point 7)
- **Depends on:** ADR-030 (fatigue), ADR-060 (states, not characteristics)

*Written in Italian as the project's working language; the decision is binding as written.*

## Contesto

La decisione AB vuole che la partita riceva lo stato di partenza di ogni giocatore come input: freschezza fisica e cognitiva, condizione, danno. È un input del motore, non un cambio di calcio, da mettere in L0 con il suo test. L'allenamento di L1 scriverà questi stati; la partita li legge. Oggi il motore legge solo la fatica di partenza. `condition` esiste nel modello ma nessuno la legge.

## LUNA

Una partita non comincia da zero: ognuno arriva con il suo corpo e la sua testa di quella settimana.

## Partita vera

- **Come lo riconoscono gli attori:** chi rientra da un infortunio e cala dopo un'ora; chi ha giocato tre giorni prima e non ha lo scatto; chi arriva da un viaggio lungo e sbaglia le scelte semplici.
- **Da cosa dipende:** dall'allenamento, dalle partite ravvicinate, dal viaggio, dal riposo; tutto prima del calcio d'inizio.
- **Chi lo produce:** la settimana (L1); la partita lo riceve.

## Decisione

Ogni stato è letto da un consumatore che il motore ha già. Nessun numero nuovo.

1. **La freschezza fisica** è il complemento della fatica di partenza (`PhysicalStateMinimal.fatigue`), come prima.
2. **La freschezza cognitiva** è il complemento di `cognitive_fatigue`. La stanchezza mentale di partenza si somma a quella fisica, al massimo 1, nel rallentare la decisione. È la forma che il motore già usa dalla concentrazione (Smith et al. 2016: la fatica mentale rallenta la decisione).
3. **La condizione** pesa la resistenza: nel plugin della fatica la sua `endurance` conta per la quota della sua `condition`. Fuori condizione si stanca prima.
4. **Il danno muscolare** (`damage`) è una fatica che la partita non restituisce. Il giocatore parte almeno così stanco, e né il riposo in campo né l'intervallo lo portano sotto il danno.
5. **I valori di base** (fresco, in condizione, senza danno) lasciano la partita com'era: i golden restano identici. Il contratto dei plugin ha `condition`.

## Conseguenze

- L1 scrive questi stati con l'allenamento e il viaggio; nella scheda prima della partita il manager li vede.
- `match_sharpness` (il ritmo partita) resta senza consumatore: L1 deciderà se è uno stato distinto dalla condizione o la stessa cosa. Annotato, non usato.
- La forma della freschezza cognitiva (somma con la fisica) è una scelta: sarà verificata con le fonti sulla fatica mentale nel calcio quando L1 la scrive.
