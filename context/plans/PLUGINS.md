# Piano — la partita divisa in plugin (ADR-063)

**Obiettivo:** dividere `ds-match` in un nucleo e quattordici plugin, con il contratto che avrà anche L1, senza cambiare il calcio. Il maintainer ha dato il via il 2026-11-15 (*"si inizia"*, *"sempre in autonomia"*).

**Regola di ogni passo:**
- i tre golden identici, bit per bit, su glibc e musl;
- un commit per passo, C2 verde, segnalibro E4b;
- se un passo cambia anche un solo risultato, non è una divisione: diventa un cambio di calcio, con il suo `SIM_VERSION`.

| # | Passo | Fatto quando |
|---|---|---|
| 1 | **fatto (120)** — Il controllo dell'ordine nel substrato (`MatchState::tick`): due giocatori che si muovono o si contrastano nello stesso tick dipendono dall'ordine in cui sono visitati? | un test che scambia l'ordine e trova lo stesso tick record, oppure la dipendenza scritta come OI con la sua correzione (un `SIM_VERSION`) |
| 2 | **fatto** — Il crate `ds-match-api`: le otto fasi del tick (`Phase`), la scheda del plugin di ADR-012 (`PluginCard`: nome, versione, cosa legge, cosa scrive, peso, testi della cronaca), il contratto `MatchPlugin`; senza dipendenze dagli altri `ds-*` | il crate compila, ha i suoi test, `ds-match` non lo usa ancora |
| 3 | **le schede fatte; la chiamata per fase arriva con il passo 4** — Il registro nel nucleo: `ds-match` elenca i suoi plugin (ancora codice interno) con la loro scheda, e il tick li chiama per fase | il golden identico; un test che il registro ha gli undici obbligatori |
| 4 | Un plugin per volta, dal più isolato: fatica, poi percezione, arbitro, palle inattive, panchina, portiere, duelli, gesti tecnici, pensiero, corpo, pallone | per ognuno: il modulo dietro il contratto, i suoi test spostati, golden identico |
| 5 | La cronaca fuori dal motore: `narrative.rs` legge gli eventi, non scrive nella partita | il golden identico; `ds-match` non dipende più dalla cronaca |
| 6 | I tre plugin che si spengono (psicologia, ambiente, colpi): la scheda dichiara le letture facoltative, il validatore le controlla (decisione B) | spento ognuno, la partita gira e i test lo provano |

**Passo 1 (2026-11-15):** l'unica dipendenza dall'ordine trovata nel substrato era la separazione minima dei corpi, corretta nel 120. Il passo di ogni giocatore si calcola dalla fotografia, e il pensiero senza palla legge ciò che era deciso prima del suo giro.

**Passo 2 (2026-11-15):** `L0-match/crates/ds-match-api`: `Phase` (le otto fasi, in ordine), `PluginCard` e `Weight` (la scheda di ADR-012), `MatchPlugin` (per ora solo la scheda: i metodi di fase arrivano con il primo plugin spostato, RULES D3), i nomi degli undici obbligatori e dei tre facoltativi.

**Passo 3 (2026-11-15):** `ds-match/src/registry.rs` tiene le schede degli undici obbligatori, lette dal codice di oggi (le fasi, cosa leggono e scrivono, i testi della cronaca), con un test che sono gli undici del contratto. Il tick le chiamerà per fase quando il primo plugin sarà dietro il contratto: un metodo entra solo con il suo consumatore (RULES D3).

**Passo 4, la fatica (2026-11-15):** `ds-match/src/plugins/fatigue.rs`, il primo plugin dietro il contratto: la sua scheda dal registro, il suo `run` (legge le velocità, scrive la fatica dei corpi), chiamato dal tick dove prima c'era `tire_players`; golden identico. La chiamata uniforme per fase (un metodo di `MatchPlugin` per fase) aspetta il tipo dello stato osservabile in `ds-match-api`, che si scrive quando due plugin lo condividono.

**Passo 4, la percezione (2026-11-15):** `plugins/perception.rs`: la memoria dell'ultimo sguardo (`Memory`), uscita da `state.rs`, con la sua scheda; la geometria del cono e dello sguardo resta in `crate::perception` (pubblico, letto dagli esempi). Golden identico.

**Il prossimo:** il passo 4, l'arbitro (`rules.rs`, i falli e i cartellini).
