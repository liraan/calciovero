# ADR-064 — The choice at the reception

- **Status:** Accepted (delegated, 2026-11-16) — to be reviewed by the maintainer
- **Date:** 2026-11-16
- **Decision:** DD-3 (the mandate of 2026-11-16, `context/STATE.md`)
- **Amends:** ADR-058 (the ball is received where it arrives)
- **Depends on:** ADR-035, ADR-049, ADR-052
- **Opened by:** OI-118 (the foot clearance almost never happens)

*Written in Italian as the project's working language; the decision is binding as written.*

## Contesto

Nel motore la ricezione è solo un controllo: chi raggiunge un pallone libero lo controlla, o lo tocca male e il pallone corre via (ADR-058). Scegliere viene dopo, con il pallone al piede. Nel calcio vero molti palloni non si controllano: si giocano di prima. Il difensore che nella sua area riceve un cross basso o un rimbalzo lo calcia via al volo. Le spazzate di piede sono 16,2 a partita in StatsBomb open data (Liga 2020/21), quelle di testa 13,8. Nel motore 133 sono 0,5 di piede e 27,7 di testa (OI-118). La spazzata di testa è una regola: nella propria metà campo il pallone alto vinto si allontana sempre. Quella di piede non esiste, perché non c'è il momento in cui scegliere.

## LUNA

Chi riceve un pallone sceglie già prima di toccarlo: fermarlo e poi giocare, oppure giocarlo subito. Lo decide da ciò che sa del proprio tocco, da quanto arriva forte il pallone e da chi ha addosso.

## Partita vera

- **Come lo riconoscono gli attori:** il pallone colpito al volo senza essere fermato (la spazzata, il passaggio di prima, il tiro di prima).
- **Da cosa dipende:** dalla velocità del pallone, dalla pressione dell'avversario più vicino, dal primo tocco di chi riceve, e da quanto vale ciascuna opzione dove arriva.
- **Chi lo produce:** chi riceve. Non il motore e non una regola.

## Decisione

1. **Prima del tocco, chi riceve sceglie** fra due cose:
   - **controllare**, e poi scegliere con il pallone al piede, come ora;
   - **giocare di prima** una delle opzioni che il suo pensiero gli dà dal punto dove il pallone arriva: la spazzata, il passaggio, il tiro.
2. **Lo stesso pensiero, nessun peso per tipo** (ADR-052). Il controllo vale quanto la migliore opzione che avrebbe con il pallone fermo, se lo controlla. Se il suo tocco non basta, vale la perdita del pallone lì. Lo sa, perché conosce il suo tocco e vede la velocità e l'avversario: ADR-058, la stessa difficoltà. Un'opzione di prima vale come ogni opzione, con l'errore del colpo su un pallone in movimento, cioè il termine della corsa del tiro (`kick.json` `run_error_rad`, ADR-051), già nel motore.
3. **Nessun numero nuovo e nessun dado.** La scelta è la migliore delle due (`resolve_choice`, ADR-052).
4. **La spazzata di testa diventa la stessa scelta** per il pallone alto, quando il duello aereo è vinto: fermarlo, passarlo di testa, spazzarlo. Si fa in un secondo passo, con il suo `SIM_VERSION`.

## Conseguenze

- Il catalogo dei gesti (`L0-match/design/ACTIONS.md`) guadagna il gioco di prima. Il test del repertorio deve vedere la spazzata di piede.
- La ricezione costa un pensiero in più per chi riceve un pallone libero: un costo da misurare sui tempi della partita (C2).
- Il SIM 134 (OI-117 (A), in `context/plans/patches/`) si applica dopo questo, perché la spazzata di piede torna a esistere come scelta.
