# Piano — la partita divisa in plugin (ADR-063)

**Obiettivo:** dividere `ds-match` in un nucleo e quattordici plugin, con il contratto che avrà anche L1, senza cambiare il calcio. Il maintainer ha dato il via il 2026-11-15 (*"si inizia"*, *"sempre in autonomia"*).

**Regola di ogni passo:**
- i tre golden identici, bit per bit, su glibc e musl;
- un commit per passo, C2 verde, segnalibro E4b;
- se un passo cambia anche un solo risultato, non è una divisione: diventa un cambio di calcio, con il suo `SIM_VERSION`.

| # | Passo | Fatto quando |
|---|---|---|
| 1 | Il controllo dell'ordine nel substrato (`MatchState::tick`): due giocatori che si muovono o si contrastano nello stesso tick dipendono dall'ordine in cui sono visitati? | un test che scambia l'ordine e trova lo stesso tick record, oppure la dipendenza scritta come OI con la sua correzione (un `SIM_VERSION`) |
| 2 | Il crate `ds-match-api`: le otto fasi del tick (`Phase`), la scheda del plugin di ADR-012 (`PluginCard`: nome, versione, cosa legge, cosa scrive, peso, testi della cronaca), il contratto `MatchPlugin`; senza dipendenze dagli altri `ds-*` | il crate compila, ha i suoi test, `ds-match` non lo usa ancora |
| 3 | Il registro nel nucleo: `ds-match` elenca i suoi plugin (ancora codice interno) con la loro scheda, e il tick li chiama per fase | il golden identico; un test che il registro ha gli undici obbligatori |
| 4 | Un plugin per volta, dal più isolato: fatica, poi percezione, arbitro, palle inattive, panchina, portiere, duelli, gesti tecnici, pensiero, corpo, pallone | per ognuno: il modulo dietro il contratto, i suoi test spostati, golden identico |
| 5 | La cronaca fuori dal motore: `narrative.rs` legge gli eventi, non scrive nella partita | il golden identico; `ds-match` non dipende più dalla cronaca |
| 6 | I tre plugin che si spengono (psicologia, ambiente, colpi): la scheda dichiara le letture facoltative, il validatore le controlla (decisione B) | spento ognuno, la partita gira e i test lo provano |

**Il prossimo:** il passo 1.
