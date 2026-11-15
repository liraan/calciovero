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
| 4 | **fatto** — Un plugin per volta, dal più isolato: fatica, poi percezione, arbitro, palle inattive, panchina, portiere, duelli, gesti tecnici, pensiero, corpo, pallone | per ognuno: il modulo dietro il contratto, i suoi test spostati, golden identico |
| 5 | **fatto** — La cronaca fuori dal motore: `narrative.rs` legge gli eventi, non scrive nella partita | il golden identico; `ds-match` non dipende più dalla cronaca |
| 6 | **fatto per la psicologia; ambiente e colpi con la sola scheda** — I tre plugin che si spengono (psicologia, ambiente, colpi): la scheda dichiara le letture facoltative, il validatore le controlla (decisione B) | spento ognuno, la partita gira e i test lo provano |

**Passo 1 (2026-11-15):** l'unica dipendenza dall'ordine trovata nel substrato era la separazione minima dei corpi, corretta nel 120. Il passo di ogni giocatore si calcola dalla fotografia, e il pensiero senza palla legge ciò che era deciso prima del suo giro.

**Passo 2 (2026-11-15):** `L0-match/crates/ds-match-api`: `Phase` (le otto fasi, in ordine), `PluginCard` e `Weight` (la scheda di ADR-012), `MatchPlugin` (per ora solo la scheda: i metodi di fase arrivano con il primo plugin spostato, RULES D3), i nomi degli undici obbligatori e dei tre facoltativi.

**Passo 3 (2026-11-15):** `ds-match/src/registry.rs` tiene le schede degli undici obbligatori, lette dal codice di oggi (le fasi, cosa leggono e scrivono, i testi della cronaca), con un test che sono gli undici del contratto. Il tick le chiamerà per fase quando il primo plugin sarà dietro il contratto: un metodo entra solo con il suo consumatore (RULES D3).

**Passo 4, la fatica (2026-11-15):** `ds-match/src/plugins/fatigue.rs`, il primo plugin dietro il contratto: la sua scheda dal registro, il suo `run` (legge le velocità, scrive la fatica dei corpi), chiamato dal tick dove prima c'era `tire_players`; golden identico. La chiamata uniforme per fase (un metodo di `MatchPlugin` per fase) aspetta il tipo dello stato osservabile in `ds-match-api`, che si scrive quando due plugin lo condividono.

**Passo 4, la percezione (2026-11-15):** `plugins/perception.rs`: la memoria dell'ultimo sguardo (`Memory`), uscita da `state.rs`, con la sua scheda; la geometria del cono e dello sguardo resta in `crate::perception` (pubblico, letto dagli esempi). Golden identico.

**Passo 4, arbitro, palle inattive e panchina (2026-11-15):** `plugins/referee.rs` (era `rules.rs`; `crate::rules` resta lo stesso modulo), `plugins/set_pieces.rs` (`apply_restart`, `restart_taker`, `clear_the_spot`, la serie dei rigori), `plugins/bench.rs` (i cambi pianificati e decisi, le istruzioni). Sono metodi di `MatchState` in file loro; i campi di `MatchState` sono `pub(crate)`. Golden identico.

**Passo 4, completo (2026-11-15):** gli undici obbligatori sono moduli di `ds-match/src/plugins/`, ognuno con il suo tipo e la sua scheda: pallone, corpo, percezione, pensiero, gesti tecnici, duelli, portiere, arbitro, palle inattive, panchina, fatica. Un test (`every_mandatory_plugin_is_behind_the_contract`) controlla che siano gli undici del contratto. `state.rs` è sceso da 4.074 a circa 1.585 righe: resta il nucleo (orologio, stato, eventi, risultato) e un `prelude` che i plugin condividono. Il corpo ha la sua scheda, ma il movimento resta in `spatial.rs`, perché agisce sullo stato spaziale. Golden identico.

**Passo 5 (2026-11-15):** la cronaca è il crate `ds-narrative`, che dipende solo da `ds-core`, `ds-model` e `ds-stats`. In `ds-match` resta ciò che la unisce al tick record (`NarratedTick`, `narrate_ticks`), e `ds_match::narrate` è lo stesso di prima. Golden identico.

**Passo 6 (2026-11-15, SIM 121):** la psicologia si spegne con `Match.psychology`: il coraggio non trattiene più nessuno e la compostezza lascia la pressione com'è. Ambiente e colpi hanno la loro scheda (`registry::OPTIONAL_CARDS`) e non ancora codice; i colpi aspettano L1 (OI-110).

**La divisione, a che punto è:**
- fatto: il contratto, gli undici plugin obbligatori come moduli con la loro scheda, la cronaca fuori dal motore, un plugin che si spegne;
- resta, per portare il contratto da "dichiarato" a "chiamato": il tipo dello stato osservabile in `ds-match-api` e un metodo di `MatchPlugin` per fase, perché il tick chiami i plugin attraverso il registro invece che per nome. È un passo che tocca ogni plugin, da fare a golden identico.

**Il contratto chiamato (2026-11-15):** `ds-match-api` ha `World`, lo stato osservabile come lo vede un plugin (oggi: i giocatori, la velocità, la fatica, una caratteristica per nome), e `MatchPlugin::run(phase, world)`. La fatica è il primo plugin che agisce solo attraverso il contratto: il tick la chiama nella fase `Move`; un test la fa girare su un campo finto di due giocatori, senza motore. Golden identico. `World` cresce con il plugin che ne ha bisogno (RULES D3).

**La percezione attraverso il contratto (2026-11-15):** la memoria è lo stato del plugin (`Perception.memory`, per `PlayerKey`); il plugin legge giocatori, posizioni e sguardi da `World`; il tick lo chiama nella fase `Perceive`. Il mondo del motore è uno solo per tutti i plugin (`plugins::EngineWorld`). Golden identico.

**La panchina attraverso il contratto (2026-11-15):** il plugin decide i cambi (pianificati e dell'allenatore che reagisce) leggendo solo da `World` (il minuto, il portatore, le rose, i ruoli, la fatica, la velocità possibile, i cambi già fatti) e li fa con `World::substitute`; il motore li esegue (`MatchState::make_substitution`). Lo schema per tutti: il motore presta al plugin un `StateWorld`, che vede lo stato intero ma solo attraverso il contratto, togliendo il plugin dallo stato per la durata della chiamata. Golden identico.

**Una regola del contratto (decisione di Claude, 2026-11-15):** `run` serve ai plugin che agiscono in una fase loro. Un plugin che lavora dentro il passo di un altro (il portiere nel pallone che attraversa la linea, le palle inattive nella ripresa dell'arbitro, i gesti tecnici nelle scelte del pensiero) resta un servizio, chiamato da quel passo. Come un plugin esterno chiami il servizio di un altro lo deciderà un ADR, quando ci saranno plugin esterni (decisione 4).

**Il confine del contratto per L0 (decisione di Claude, 2026-11-15, con i filtri luna e partita vera):** passano attraverso `World` i plugin che leggono e scrivono cose che il contratto sa dire senza i tipi del motore: fatica, percezione, i cambi della panchina (fatti). Duelli, pensiero, gesti tecnici, portiere, arbitro e pallone decidono con i tipi del motore (`DuelContext` e l'esito del contatto, la scena percepita, la parata), che vivono in `ds-orchestrator`; portarli nel contratto vorrebbe dire rifare quei tipi in `ds-match-api`, un secondo modello del calcio da tenere allineato al primo. Restano plugin ufficiali compilati dentro (decisione 4), ognuno nel suo modulo con la sua scheda, chiamati dal tick nell'ordine delle fasi. Quali tipi vedrà un plugin esterno lo deciderà l'ADR dei plugin esterni, con il primo plugin esterno.

**La divisione di L0 è chiusa:** il contratto, i quattordici plugin (undici obbligatori come moduli, tre facoltativi con la scheda, uno già spegnibile), la cronaca fuori dal motore, la separazione dei corpi senza dipendenza dall'ordine, tre plugin chiamati attraverso il contratto, il golden identico a ogni passo.
