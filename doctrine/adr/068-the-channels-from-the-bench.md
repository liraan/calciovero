# ADR-068 — The channels from the bench

- **Status:** Accepted (delegated, 2026-11-16) — to be reviewed by the maintainer
- **Date:** 2026-11-16
- **Decision:** DD-10, from decision AP, and the pauses of decisions AM and AO (`context/plans/DECISIONS-2026-11-15.md`; `context/plans/L1-COACH.md`)
- **Supersedes:** ADR-059 §2 (every instruction at the next tick, for everyone)
- **Depends on:** ADR-059, ADR-062 (what a player sees), ADR-013 and decision X (the recommended modules)

*Written in Italian as the project's working language; the decision is binding as written.*

## Contesto

L'ADR-059 §2 fa arrivare ogni istruzione a tutti dal tick successivo. In campo non è così: dalla panchina si urla una parola, si fa un gesto a chi guarda, si passa un foglietto quando il gioco si ferma. La decisione AP vuole i canali in L0, come modulo consigliato: acceso di base, e spento torna l'arrivo immediato di oggi. Le decisioni AM e AO mettono in L0 l'intervallo e le pause idriche, che sono le pause in cui passa tutto.

## LUNA

Un'istruzione non è un ordine al computer: deve arrivare a chi gioca, per una strada che ha il suo tempo e i suoi limiti.

## Partita vera

- **Come lo riconoscono gli attori:** l'allenatore che urla "più alti!", il terzino che si gira verso la panchina e vede il gesto, il capitano che riceve il foglietto durante un infortunio, la squadra intorno all'allenatore nella pausa idrica.
- **Da cosa dipende:** da quanto c'è da dire e da quando il gioco si ferma.
- **Chi lo produce:** l'allenatore che sceglie il canale, il giocatore che guarda la panchina, l'arbitro che ferma il gioco.

## Decisione

1. **Ogni istruzione arriva per un canale** (`Instruction.channel`):
   - **la voce:** un messaggio breve, a tutti, dal tick successivo. Senza volume né distanza: scelta del maintainer, sarebbe frustrante.
   - **il gesto:** arriva quando uno dei suoi giocatori in campo ha la panchina nel cono di vista (la percezione di L0), e lui lo passa ai compagni.
   - **la pausa:** qualunque cosa, anche una lavagna nuova, alla prima pausa dopo che è stata data. Le pause sono il colpo che ferma il gioco, l'intervallo, la pausa idrica e il cambio, perché chi entra porta il foglietto.
2. **Il canale lo sceglie il gioco** se il manager non lo sceglie, da quanto c'è da dire. Le istruzioni rapide, le transizioni, l'intenzione e i cambi vanno a voce. Una lavagna che sposta al massimo due posti va a voce (il richiamo a un giocatore, lo scambio di due); una che ne sposta di più, o che cambia i disegni per terzi, va alla pausa. Il manager può scegliere lui il canale.
3. **Le pause idriche** sono una regola della competizione (`MatchRules.water_breaks`, i minuti). L'arbitro ferma il gioco alla prima interruzione dopo ciascun minuto, ed è una pausa. Ogni pausa idrica allunga il tempo di tre minuti, la durata della pausa di raffreddamento della FIFA.
4. **Il fatto dell'istruzione si racconta quando arriva alla squadra**, non quando la si dà.
5. **È un modulo consigliato** (`Match.channels`, acceso di base). Spento, ogni istruzione arriva a tutti dal tick successivo, come prima.

## Conseguenze

- Le istruzioni a voce arrivano come prima: i golden, anche quello con le istruzioni, restano identici.
- **Non ancora:**
  - i segnali preparati in settimana che fanno scattare i piani "se, allora", e l'allenatore espulso con il vice: sono moduli della 0.2;
  - nella GUI la scelta del canale e la vista di chi ha ricevuto l'istruzione;
  - l'effetto psicologico dei discorsi, all'intervallo e nei richiami. Arriva con gli stati psicologici e i tratti che li leggono (L1, decisione AM), come scritto per il maintainer (strada A).
- Il compagno che ripete è oggi immediato: chi vede il gesto lo dice a tutti a voce, e la voce non ha distanza.
