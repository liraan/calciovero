# Stato — Calcio Vero

**Cos'è.** Il segnalibro: dove siamo, cosa viene dopo, cosa resta aperto. Non è un diario: la storia è in `git log` (anche quella del vecchio `process/SESSION-STATE.md`). Le regole per aggiornarlo sono in `RULES.md` §E.

**Last commit verified:** `00bb935` - "refactor(balance): the statistics are derived, never adjusted"

## Dove siamo (2026-11-13)

- **Motore L0, `SIM_VERSION` 78.** **Metodo (maintainer, 2026-11-13): le statistiche si derivano dalle abilità, mai si aggiustano** (ADR-052 nota, RULES A5-bis); tre valori scelti guardando le statistiche sono stati rifondati (77). Ogni giocatore sceglie con il suo pensiero (ADR-052): immagina le opzioni (chi arriva prima, la qualità dello spazio raggiunto, cosa si perde) e le giudica con una sola misura; nessun peso per tipo di opzione, nessun dado (ADR-035). Ogni gesto ha le sue radici nelle caratteristiche ed è riconoscibile (ADR-051, catalogo `L0-match/design/ACTIONS.md`).
- **Fatto in L0-CLOSE** (`plans/L0-CLOSE.md`): la difesa pensa (68), l'attacco senza palla pensa (69), spazzata, pallonetto e altro palo (70), il gioco aereo (71), il portatore che protegge palla o punta l'uomo, con la nuova `dribbling` (72), la fatica che cresce con lo sforzo contro la nuova `endurance` e appanna accelerazione e velocità (73), le palle inattive vere (chi batte lo sceglie la squadra, le distanze delle Leggi, il rigore solo tiro) e tre bug trovati per strada, tra cui il portiere avversario che la squadra di casa non riconosceva (74). Nessuna caratteristica composta salvata: tolte `aerial` e `set_pieces`, ogni valore deriva dalle sue radici (75, richiesta del maintainer). Il test di repertorio (76): ogni gesto del catalogo compare nei seed 42-51; per farlo verde, cinque pensieri allineati al motore (parata trattenuta o respinta, pallonetto, pressione sul calcio, mira dentro il palo, palla alta contesa). L'amichevole giocabile (passo 9): `ds-demo --xi`, `--block`, `--width`, `--save`, `--load`, `--why`; l'intenzione dell'allenatore modula il blocco (OI-27, prima forma). Prima: portiere che legge e si tuffa, tiro con forza e piede, contrasto in tre forme con quattro esiti (ADR-042), le Leggi (fuorigioco, falli, cartellini, vantaggio, rigore), palla fuori e ripartenze.
- **Misura 30 seed (42-71), per partita:** il corpo sui valori dei professionisti e il passo scelto (78): 9,5 km, il tempo per fascia di velocità come nel reale, tiri 29 (reale 25). Restano: 86% dei tiri nello specchio (reale 34%), quindi 7,8 gol. Le cause in `plans/L0-CLOSE.md` passo 10.
- **Dottrina recente:** ogni attore ha i suoi tre piani (`doctrine/00-foundations.md` 4.1); i due cicli annidati del manager (README, `VISION.md` §6); l'arbitro sono le Leggi applicate, presenti ma senza occlusione (ADR-021, nota).

## Plan

| Pr. | # | Cosa | Dipende da | Prima azione |
|---|---|---|---|---|
| [next] | L0-CLOSE | **Chiudere L0 prima di L1** (maintainer, 2026-11-13). Passi 1-9 fatti; il 10 è cercare le cause delle statistiche lontane dal reale, senza aggiustarle (la prima, la corsa, fatta in 78). | ADR-051, ADR-052, ADR-042 | La fonte dell'errore del tiro: gli studi sulla precisione del calcio (oggi 86% nello specchio contro 34%). |
| [open] | 10 | **L1, il mondo** (`L1-world/`). Si apre solo dopo L0 chiuso. | L0-CLOSE | Scrivere `L1-world/00-overview.md` (L1.04) prima di L1.05. |
| [deferred] | 11 | **Ruleset GitHub (fase 2).** Disattivo; si riattiva quando c'è un secondo revisore, il progetto esce, o un agente riceve un token di scrittura. | — | Rileggere `RULES.md` §C quando una delle tre condizioni si presenta. |

## Open items

| # | Cosa | Prima azione |
|---|------|--------------|
| OI-7 | **Scouting L2: manca il calcolo di `ProjectionProfile`** (la previsione di crescita con la sua incertezza). Il tipo esiste, la formula no. | Nessuna ora: si decide quando si apre L2-scouting. |
| OI-8 | **Scouting L2: manca la formula del bias dichiarato dello `ScoutReport`.** È anche il prerequisito della cantera (IDEA-YOUTH-001): senza, si fingerebbe che tutti vedano tutto. | Nessuna ora: si decide con L2-scouting. |
| OI-27 | **L'intenzione dell'allenatore, prima forma (passo 9).** Blocco alto, medio, basso e ampiezza modulano la struttura di default (`block.json`); oggi spostano la linea e l'ampiezza, non cambiano ancora il pensiero di ogni giocatore (pressare più alto, attaccare la profondità). Idee: IDEA-INFRA-014, IDEA-INFRA-016, IDEA-TACT-001. | A L1: l'intenzione come categoria del pensiero di ogni giocatore (ADR-052 nota, punto 4), e le altre istruzioni (ritmo, pressing, marcature). |
| OI-31 | **La specifica degli eventi e il codice divergono.** `L0-match/specs/05-match-events.md` §2.2 non elenca `KickOff`/`SecondHalf`, che il codice ha; `Slip`/`ControlError` sono previsti e non esistono. | Allineare la specifica al codice nel passo 8 (test di repertorio); `Slip`/`ControlError` restano idea finché un gesto non li produce. |
| OI-38 | **Il ruleset ha tre decisioni rinviate:** per partita o per stagione; lo sceglie la configurazione (L0) o la competizione (L1); i recuperi sono spiegati (trasparenza) o no. | Decidere (1) e (2) all'apertura di L1; (3) con il primo calcolo dei recuperi. |
| OI-40 | **Il bus eventi di `ds-core` non ha consumatori**, e le `ObservationEvent` si accumulano senza essere svuotate in partita. | Decidere: usarlo (per le Leggi o il narratore) o rimuoverlo. Default: rimuoverlo se a fine L0 nessuno lo usa. |
| OI-49 | **I documenti di navigazione invecchiano** (`ARCHITECTURE`, `CODE_TOUR`, `GLOSSARY`): il codice cambia con `SIM_VERSION`, loro no, e nessun gate se ne accorge. | Rileggerli contro il codice a L0 chiuso (passo 10) e correggerli in un commit. |
| OI-55 | **`Referee.strictness` e `Referee.consistency` vanno rimossi.** L'arbitro sono le Leggi applicate in modo oggettivo (ADR-021, nota 2026-11-13): una severità variabile è un errore sistematico. | Rimuovere i due campi e i loro lettori, con golden invariato. |
| OI-64 | **Il corpo rispetto alla palla.** L'angolo del passaggio rispetto all'orientamento del passatore, il lato di ricezione e il giro del corpo (ADR-031, IDEA-PM-005 chiusa) non sono letti; il lato del piede nel tiro è ancora un segnaposto. | L0-CLOSE: nel passo 5 (girarsi con la palla) e nel 6 (il corpo); ogni termine con il suo gemello. |
| OI-69 | **Il pulsante "donate" e il fisco** (Spagna, Canarie, ISD gruppo IV). Pulsante pubblicato; la consulenza fiscale è a carico del maintainer. | Del maintainer: consulenza fiscale prima di promuovere le donazioni. |
| OI-70 | **Gli altri appannatori.** La fatica ha un produttore (73); restano la concentrazione che cala (ADR-052), la fiducia e la compostezza del momento (IDEA-PM-003/004), e il pensiero che non sa ancora di essere stanco (la corsa in profondità legge `max_speed` fresca). Gli infortuni sono L1. | Dopo L0-CLOSE: la concentrazione come secondo appannatore, con il suo gemello; poi il ritmo creduto che legge la fatica. |
| OI-85 | **`balance/choice.json` sopravvive a ADR-052.** La soglia di `decision_making` e i residui della vecchia formula di scelta sono ancora applicati, dove il pensiero del giocatore li ha sostituiti. | Leggere chi lo usa ancora; togliere ciò che ADR-052 sostituisce, con golden rigenerato. |
| OI-90 | **Campi morti.** `PassContext.receiver_freedom_cm` non è letto da nessuno; restano campi di `offball.json` e `shot_choice.json` che il pensiero ha reso inutili. | Una passata con `grep` per ogni campo di `balance/` e dei contesti: chi non ha lettore va tolto. |
| OI-94 | **`ds-match` ha più motivi di cambiamento** (tick, movimento, percezione, eventi, Leggi, salvataggio, narrazione): ADR-041 lo dichiara e rinvia lo split. | A L0 chiuso: una tabella modulo → motivo, poi decidere lo split con un ADR. |
