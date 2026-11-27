# Audit del codice — Calcio Vero (2026-11-23, HEAD 1f5da3e)

**Cos'è.** Il piano di correzione nato dall'audit del codice chiesto dal maintainer il 2026-11-23 ("analizza tutto il codice cercando incongruenze e ridondanze"; "uno dei preconcetti è un crate per responsabilità"). Mandato: partire dal punto 1 e proseguire in autonomia fino alla risoluzione di tutti i problemi. Ogni voce chiusa riporta qui il commit che la chiude.

Analisi in sola lettura di ~44.000 righe Rust (15 crate) da 8 analisti paralleli (spazio, motore, regole, orchestratore, dati, strumenti, test/doc, confini dei crate). Circa 160 rilievi grezzi, qui accorpati e senza doppioni. ✔ = verificato a mano da me sul codice; gli altri sono verificati dall'analista leggendo le righe.

## A. Difetti che cambiano la partita (bug)

**Stato (2026-11-23):** chiusi nel SIM 196 (DD-55) A1-A7 e A9-A19; A8 spostato a B (è la scelta dell'attore, insieme al branch accantonato `sim-189-first-time`). Il test GUI delle linee del fuorigioco ha rivelato un difetto in più: le linee erano ricostruite dall'ultimo passaggio *ricevuto*; ora la chiamata dice il tick in cui il pallone è stato giocato (`played_tick`).

| # | Rilievo | Dove | Verifica |
|---|---|---|---|
| A1 | `ball_at_line`: con la palla verso x=0 `dx` è negativo e `.max(1)` lo trasforma in 1 → y/z all'incrocio estrapolati con segno sbagliato. Sbaglia la parata deviata sul palo e il "colpo di testa nello specchio" verso la porta di casa | spatial.rs:1287-1290; usato da keeper.rs:122, technique.rs:162 | ✔ |
| A2 | Il portiere che deve parare è identificato con id fissi (slot 0 / 100): dopo un cambio o un'espulsione del portiere `keeper_meets_the_shot` non trova nessuno → ogni tiro nello specchio è gol | plugins/ball.rs:170-173 (anche `players.first()` in duels/referee/set_pieces/technique) | ✔ |
| A3 | Fuorigioco mai fischiato su una palla vinta in aria (cross di testa): `offside_at_pass` viene svuotato senza controllo | technique.rs:387-389 (il controllo esiste solo a 1065) | ✔ |
| A4 | Ripresa bloccata: il sostituto del battitore si sceglie con `spatial.iter()`, che include gli espulsi/usciti (in `off_pitch` a (metà, -300)); se è lui il più vicino la ripresa non parte mai | set_pieces.rs:169-183, spatial.rs:2284 | ✔ |
| A5 | Rigori e calcio d'inizio dopo i cambi: `on_pitch_count`, `shootout_order` e `kick_off` leggono la rosa iniziale (`ctx.match_`), non quella viva: dopo 3 cambi i tiratori possibili scendono a 8, i subentrati non calciano | simulate.rs:135,154,200-206; set_pieces.rs:248,414,437 | ✔ (lettura) |
| A6 | Cambi durante la serie dei rigori (Legge 10 lo vieta) e tiratore sostituibile dopo che l'ordine è fissato | set_pieces.rs:36/467 + simulate.rs:97 | plausibile |
| A7 | I cambi pianificati ignorano `subs_allowed` (solo quelli ordinati hanno `take(left)`); e il limite esiste due volte (`MAX_SUBSTITUTIONS=5` e `MatchRules.subs_allowed`) | bench.rs:52-60 vs 63-72; squad.rs:412 | ✔ |
| A8 | ✅ SIM 197: `first_time` usa `own_scene`, la stessa testa di `decide`. Prima: `Actor::first_time` valuta "controllare" con tiratore perfetto, piede destro fisso, `first_touch` vuoto (controlla sempre) → il gioco di prima è sottovalutato | actor.rs:206-207 | analista |
| A9 | `tick_steps` dichiara la palla ferma al primo rimbalzo di una palla quasi verticale: le previsioni (`intercept`, `drop_point`) smettono lì, la fisica vera continua | spatial.rs:322-336 | analista |
| A10 | Gol: due regole di superamento della linea (centro palla per un tiro, palla intera per il resto) | ball.rs:112-114 vs 234-236 | analista |
| A11 | Vantaggio + colpo: il commento dice "l'uomo a terra ferma il gioco", il codice conta la pausa e lascia giocare | referee.rs:559-607 | analista |
| A12 | `play_advantage(…, half_length)` riceve la lunghezza intera; il test passa metà lunghezza → il test protegge un'altra soglia | referee.rs:346, 605, 719 | analista |
| A13 | `shootout_kick_event_done`: un evento senza attore (Instruction, WaterBreak) chiude il rigore come sbagliato | referee.rs:390-400 | plausibile |
| A14 | Il rinvio sceglie il piede dal lato assoluto del campo (il bug già corretto per il passaggio al SIM 55) | orchestrator lib.rs:848-852 vs 667-673 | analista |
| A15 | Separazione dei corpi per assi (spinta a 45°, √2 troppo forte; due corpi sullo stesso punto mai separati) | spatial.rs:2324-2328 | analista |
| A16 | `kicker_nerves` ignora l'interruttore della psicologia: stima ≠ esecuzione con psicologia spenta | orchestrator lib.rs:1021 vs 376; technique.rs:236 | analista |
| A17 | Il narratore cerca il campo `goal` sul tiro, che non esiste (c'è `on_target`): "shoots wide" non appare mai | ds-narrative lib.rs:232 | ✔ |
| A18 | `EventChain::verify` non ricalcola nulla: confronta `current` salvato con l'hash salvato → un salvataggio manomesso passa; il campo `hashes` non è mai letto | ds-stats chain.rs:87-95 | ✔ |
| A19 | `is_home` in ds-demo ignora i subentrati (e `ball.rs:282` per l'autogol ha lo stesso schema: da verificare) | friendly.rs:243 | analista |

## B. Un crate per responsabilità (il principio è rispettato solo nel grafo delle dipendenze)

Grafo: nessun ciclo, i tool non dipendono da orchestratore/db/calcolatore. Ma dentro:

| # | Violazione | Dove |
|---|---|---|
| B1 | ✅ **Chiuso (ADR-112, DD-56)**: `latents_for` privata, il motore chiede all'attore le letture, un plugin legge solo ciò che la scheda dichiara, test strutturale. Prima: **Il piano A esce dal caveau**: `pub fn latents_for` restituisce tutte le latenti ("Read-only access for match plugins"), `World::characteristic` le apre al contratto dei plugin; il motore le legge in technique.rs:236, thought.rs:234/383-387/889, set_pieces.rs:520, referee.rs:563, fatigue.rs:40. Contro ADR-003, ADR-041 §3, il doc dell'orchestratore ("never exposes a latent value") e context.rs:35 | ✔ |
| B2 | 🟡 **In parte (2026-11-24)**: la scelta del posto con la palla (`choose_place`) passa all'attore, `ds_orchestrator::offball::choose_offer_place` da una `OfferPlaceScene` costruita dal motore, con la sua lettura della linea (`GroundTruth::choose_offer_place`); come già `choose_closed_place`. Golden identici. Anche (2026-11-24): l'uscita del portiere (`GroundTruth::keeper_comes_for`), il coraggio sul pallone alto (`goes_up_against`, formula in `ds_calculator::duel::goes_up`), la profondità del portiere (`keeper_line_depth_cm`), il colpo di testa (`choose_header_shot`), la lettura del gioco messa nella scena dall'attore stesso (`Actor::decide`); `courage`, `foresight` e `risk_appetite`, che restituivano caratteristiche grezze, sono privati (seguito di ADR-112). Restano: i cambi dell'allenatore in `bench.rs`, `decide` in `thought.rs` come regista delle chiamate. Prima: **Il motore decide**: `MatchState::decide` è in ds-match (thought.rs) e chiama `resolve_pass/duel/shot`; scelte prese nel motore (piede migliore, uscita del portiere con soglia 0.5, posto da chiudere, cambi dell'allenatore in bench.rs). ADR-041 §4 dice che MatchState non chiama `resolve_*`; il test anti_leak cerca solo `.resolve_choice(` | analista |
| B3 | 🟡 **In parte (2026-11-23)**: in `ds-calculator` l'intervallo delle scansioni (`latency::scan_interval_ticks`), l'orizzonte (`ThoughtBalance::horizon_cm`), la qualità del battitore (`kick::set_piece_quality`, usata anche dal test che la copiava), la forza del colpo (ADR-112), il peso del passaggio (D). Resta: le formule del duello composte nell'orchestratore, `CoachBalance::minute` (va con B2). Prima: **Formule fuori dal calcolatore**: l'orchestratore chiama `latents_for` 38 volte e scrive formule e pesi suoi (duello, battitore, scansione); formule anche in ds-match/balance.rs (`KnockBalance::knocks`, `CoachBalance::minute`, `stoppage_for`) e in set_pieces (barriera) | analista |
| B4 | 🟡 **SIM 198**: il tempo di volo del tiro (motore) e dei tiri pensati (calcolatore) con l'aria, come il passaggio alzato. Resta che le tre implementazioni vivano in tre crate (una sola funzione di volo in `ds-calculator` sarebbe il passo successivo). Prima: **Fisica della palla in tre crate con modelli diversi**: motore (con aria), orchestratore (tiro senza aria, lancio con aria), calcolatore (pallonetto a velocità costante "the same physics") → l'attore immagina un volo diverso da quello che il motore esegue | analista |
| B5 | ✅ **Chiuso (2026-11-23)**: `MatchResult` senza cronaca, `ds-match` senza `ds-narrative` (solo come dev-dependency dei test dei campi), `narrate_ticks` in `ds_demo::narration`, gli strumenti leggono la cronaca dagli eventi; `SCHEMA_VERSION` 4. Prima: **Il motore dipende dalla cronaca** (ds-match → ds-narrative, `NarrativeEntry` dentro `MatchResult`), contro STRUCTURE.md e ADR-063 | ✔ (Cargo) |
| B6 | ✅ **Chiuso (2026-11-23)**: `why_conceded` e i suoi aiuti in `ds_narrative::why`; ds-demo lo riesporta. Prima: **Cronaca scritta fuori da ds-narrative**: `why_conceded` in ds-demo, usato dalla GUI | analista |
| B7 | 🟡 **Dichiarato**: STRUCTURE dice che ds-stats tiene gli eventi e le statistiche sono del motore e dei lettori. Resta da verificare che le definizioni coincidano. Prima: **Statistiche fuori da ds-stats**: `MatchStatistics` in ds-match, ds-reference riconta tiri/tiri in porta con un'altra definizione | analista |
| B8 | **Bilanciamento caricato in due crate**: ds-match/balance.rs carica 12 JSON; `ball.json` è deserializzato due volte con tipi diversi (i16 vs f64) | analista |
| B9 | 🟡 **Dichiarato (2026-11-23)**: `ds_match_api::phase` dice l'ordine che il core esegue oggi accanto a quello del contratto; seguire le fasi cambia la partita e aspetta un ADR. Prima: **Plugin solo di nome**: 3 plugin su 11 implementano `run()`; l'ordine delle fasi dichiarato in ds-match-api (`Phase::ALL`) non è quello eseguito; le schede non dichiarano le letture vere (fatigue legge endurance; referee injury_proneness; "knocks: no code yet" ma esiste) | analista |
| B10 | 🟡 **Dichiarato**: STRUCTURE dice che ds-demo è anche la libreria delle viste. Prima: ds-gui dipende da ds-demo (un tool da un altro tool): ds-demo è anche libreria condivisa | analista |

Tabella dei crate (dall'analista): rispettano la responsabilità ds-core, ds-model, ds-fixtures, ds-testkit, ds-test-runner, ds-gui (quasi); **parziale** ds-stats, ds-calculator, ds-orchestrator, ds-match-api, ds-narrative, ds-demo, ds-reference; **no** ds-db (caveau aperto) e ds-match (decide, legge il piano A, dipende dalla cronaca, carica balance e formule).

## C. Dati e balance: stessi numeri in più posti, numeri senza fonte

**Stato (2026-11-23):** C1 chiuso: la Legge 1 ha una sola fonte (`laws.json` → `ds_calculator::laws`); tolti i doppioni in obstacles.json, thought.json, offball.json e i 366/244/732/3400 nel codice; golden identico. **C2 chiuso** con nomi distinti, senza cambiare la partita: `choice_pressure_radius_cm` (300, quanto si sente pressato chi sceglie) e `pressure_radius_cm` (500, l'evento Pressure e la ricezione); `watch_mark_radius_cm` (800, chi marca guarda il suo uomo) e `mark_radius_cm` (300, chi si smarca). Unificarli a 5 m è stato provato (SIM 197, non committato): rompeva due guardiani di comportamento (la squadra più larga su richiesta, il pallone avanti nel dribbling); entrambi i valori sono senza fonte, restano distinti fino alla taratura. **C6 chiuso**: `TICKS_PER_SECOND` costante, il motore rifiuta un altro valore, gli strumenti la leggono; tolto `MatchConfig.duration_minutes` (mai letto). **Chiusi anche:** C3 in parte (tolte `reading_weight`, `risk_weight`, `clear_salience` e `ChoiceCandidate.risk`, mai letti; restano le sei chiavi di offball.json raggiungibili solo nei test, che vanno con B2), C4 (le quattro righe mancanti; il conteggio di SOURCES rifatto: 214 parametri, 0 senza riga), C7 (il custode legge `max_speed` e `maximal_force`; `Acceleration` resta senza radice e lo dice), C8 (i bilanci del pensiero, del senza palla, del tiro, del blocco e del colpo letti una volta: la suite d'integrazione 255 → 146 s). **C5 in parte:** 915 e 550 dalla Legge 1 (`laws.json`), il portiere fuori dalla linea (`keeper.json` `line_offset_cm`), la base e il passo dell'intesa (`offball.json`), il prato di `TeamView` dalla fisica della palla; golden identici. **C5, SIM 199:** la testa di ciascuno per il pallone alto. **C5, anche:** `taker_radius_cm` in offball.json. **Aperti di C5:** `TAKER_RADIUS_CM`, lo `0.8` di `read.rs`, gli orizzonti di previsione di `spatial.rs`.

| # | Rilievo |
|---|---|
| C1 | Porta e area in 3 JSON e 4 punti del codice: `goal_half_width_cm` in laws.json **e** obstacles.json (letti da moduli diversi), `goal_width_cm` in thought.json, `366`/`244` scritti in orchestrator lib.rs:490, `GOAL_WIDTH_CM` in read.rs:285, `(…, 3400, 244)` in thought.rs:93/1144 e technique.rs:707 (centro porta fisso: sbaglia su un campo non largo 68 m); `keeper_box_*` (offball.json) duplica `penalty_area_*` (laws.json), `is_cross` e technique riscrivono il predicato |
| C2 | Stessa chiave, valori diversi: `pressure_radius_cm` 500 (influence.json) e 300 (shot_choice.json); `mark_radius_cm` 800 (scanning) e 300 (offball) |
| C3 | Chiavi caricate e mai usate (pesi per opzione che ADR-052 dice rimossi): `reading_weight`, `risk_weight`, `clear_salience` (silenziata con `let _`), e in offball.json sei chiavi (`hold_salience`, `challenge_salience`, `mark_salience`, …) raggiungibili solo nei test; `ChoiceCandidate.risk` scritto 7 volte e mai letto |
| C4 | Chiavi senza riga in SOURCES.md: `intesa_yield_milli`, `water_break`, `same_line_cm`, `line_break_milli` (il "0 undocumented" è falso); righe SOURCES che descrivono chiavi rimosse; stato "sourced" su valori dichiarati arbitrari |
| C5 | Numeri nel codice senza chiave né fonte: `HEAD_HEIGHT_CM=180` (invece dell'altezza del giocatore), `GK_LINE_OFFSET_CM=200`, 915/550 della Legge 1 (laws.json li ha ma "li legge solo la GUI"), `TAKER_RADIUS_CM`, intesa 0.5/0.1/0.6, `ground_keep 0.957`, `0.8` in read.rs:799, default piede forte [0.5, 0.15], `kick.speed(0.85,…)` copia di `power_strong` |
| C6 | **10 tick al secondo dato per scontato** ovunque (`ticks_per_second` configurabile ma fisica, KICK_TICKS, scan, GUI, ds-demo `×600`, test channels.rs `tps=10`); `duration_minutes`, `half_time_minute` e `Match.seed` mai letti |
| C7 | Il custode restituisce 0.0 per `MaxSpeed` e `MaximalForce` anche se le latenti esistono |
| C8 | ThoughtBalance (e altri) riparsati dal JSON in 16 punti caldi (`perceived` per ogni coppia di giocatori, ogni tick), mentre altri balance sono in cache |

## D. Logica duplicata (ridondanze)

**Stato (2026-11-23), primo passaggio, golden identici:** `contact`/`drag` calcolati una volta in `resolve_shot`; `stoppage_for` usa `StoppageTime::total`; `goal_side_of` usa `step_towards`; `flip_possession` usa `other`; il blocco doppio di `tick()` è `move_bodies` + `count_possession`; `visible_to` usa `in_cone`; ds-demo ha un solo `hex` e un solo `importance_kind`; il board formatta il recupero con la durata del periodo (prima `> 45`, sbagliato al 45° e nei supplementari).

**Secondo passaggio, golden identici:** la velocità del passaggio rasoterra pensato è `KickBalance::weighed_pass_speed` (era in quattro punti, anche in offball), i tick di rotolamento `KickBalance::rolling_ticks` (due), l'arrivo del pallonetto `lob_landing` (due). **Terzo passaggio, golden identici:** `MatchSave` non scrive più la catena due volte (verifica quella del risultato, SCHEMA 5); `apply_restart` e `call_restart` condividono `stop_for_restart`; i tre test di determinismo di `engine.rs` sono uno. **Aperti:** le formule che divergono già fra loro (vedi sotto: "ci arriva in tempo", la pressione sul calcio, `choose_place`/`choose_closed_place`, i due `World`): unirle cambia la partita e va con la sezione B.

Cono visivo scritto 3 volte (perception.rs); `kick_pressure` e `difficulty` con lo stesso calcolo su due punti diversi; formula del tiro che ricopia `ThoughtBalance::expected` senza il clamp; fisica del pallone immaginato copiata in 4-5 punti di read.rs/offball.rs/lib.rs; "ci arriva in tempo" con tre formule (con e senza tempo di reazione); `choose_closed_place` (orchestratore) e `choose_place` (motore) quasi identici e già divergenti; due implementazioni di `World` (EngineWorld/StateWorld) con comportamenti diversi su fatica e id; blocco duplicato in `tick()`; `apply_restart`/`call_restart` e `header_at_goal`/`header_at_goal_aimed` copiati; `contact`/`drag` calcolati due volte in `resolve_shot`; verso d'attacco ricavato in 4 modi; tre convenzioni per la "panchina"; `MatchSave` serializza gli eventi due volte; helper duplicati negli strumenti (hex, importanza, recupero formattato con un off-by-one fra board e friendly).

## E. Test che non possono fallire

**Stato (2026-11-23):** chiusi. E1: i quattro test di determinismo fanno la seconda corsa con il motore vero (`::ds_match::…`), non con la cache. E2: il repertorio ha i rilevatori di "turn" e "stop" (fatti dal SIM 86 e 78); gli atti senza rilevatore sono un elenco per chi legge, non un'asserzione impossibile. E3: anti_leak legge anche plugins/mod.rs, psychology.rs, simulate.rs, driver.rs, engine.rs. E4: ds-test-runner rigioca davvero il salvataggio, la catena è ricalcolata, nessun xG nominato e non letto, un panic diventa uno scenario fallito (con il suo test). E5: tolti `2+2==4` e i quattro `version_is_not_empty`; "minuto ≤ 120" e "possesso somma a 1" sostituiti da "l'orologio non torna indietro" e "entrambe le squadre hanno la palla"; tolti il match esaustivo di viewmodel e quello di `ShotType`. In più: `channels.rs` leggeva il tick dal minuto con la convenzione sbagliata, ora dal registro dei tick.

| # | Rilievo |
|---|---|
| E1 | **La cache della suite rende banali i test di determinismo**: `the_same_instructions_give_the_same_match`, `recording_does_not_change_the_match`, `the_same_seed_gives_the_same_frames`, il "no die" del colpo confrontano un risultato con la sua copia in cache (✔). Il commento di support/mod.rs dice il contrario. Il determinismo vero lo controllano ancora i golden e i test in engine.rs |
| E2 | `repertoire`: il controllo `NOT_YET` non può fallire (gli atti non sono mai inseriti) e le sue ragioni contraddicono ACTIONS.md (Turn, Stop, Cover, Shield fatti) |
| E3 | anti_leak non legge psychology.rs, mod.rs, simulate.rs, driver.rs, engine.rs; cerca solo `.resolve_choice(` |
| E4 | ds-test-runner: `SaveLoadScenario` non fa replay, "every event has a cause" controlla solo il minuto, "xG ≥ 0" non legge xG, i panic non sono catturati |
| E5 | Asserzioni sempre vere (`2+2==4`, minuto ≤ 120 dopo un clamp, match esaustivi, possesso `1 - home`) |

## F. Documentazione che non dice il vero (sintesi)

**Stato (2026-11-23):** fatti: conteggio dei test (607, più uno ignorato), `rules.rs` → `plugins/referee.rs` in CONTRIBUTING, ACTIONS e L0-match/README, intestazione e §4.1 di ACTIONS (ADR-042 accettato), ADR-002 superato nella tabella, plugin e balance descritti come sono (moduli con scheda; `.json`), _index senza crate inesistenti e con i quattro che mancavano, i nomi `pcfm-*` nei commenti del codice, le guardie anti-leak di ds-demo che cercavano un crate scomparso, il doc del custode. Secondo passaggio: i cross contati dal qualificatore del Pass (ds-reference, ds-demo), le note di duel.json (retry 50), laws.json (il portiere non è copertura), keeper.json (la regola del SIM 124), il formato v2 del record, l'esempio `--html` di ds-demo. Terzo passaggio: il doc di `move_agents` e di `check_shot_crossing`, i commenti sui "dadi" (outcome, context, save, ball, thought). Quarto passaggio: i corsivi e la compostezza di ACTIONS, la radice "set pieces" tolta. **Aperti:** le varianti di evento mai prodotte (`RandomEvent` e altre cinque: toglierle cambia il formato dei salvataggi, va con un ADR).

Conteggio dei test 589 contro 605 `#[test]`; `rules.rs` citato in CONTRIBUTING/ACTIONS/README ma non esiste (è plugins/referee.rs); ACTIONS.md fermo al SIM 76 con il duello vecchio e corsivi sbagliati; ADR-002 "accepted" e "superseded" nella stessa STRUCTURE.md; `sheet.toml` e balance ".toml" che non esistono; _index.md con crate inesistenti (ds-metrics, ds-data, ds-gen) e senza quelli veri; nomi `pcfm-*` in molti doc e in una guardia anti-leak che cerca un crate scomparso; doc di `move_agents`, keeper.json, laws.json (portiere come copertura), duel.json (retry 30 vs 50), `MatchConfig` (file toml inesistente), formato record v1 vs v2, `ds-demo --help` con un esempio `--html /tmp/…` che fallisce sempre; molti commenti su "die"/dadi contro ADR-035; `RandomEvent` e altre 5 varianti di evento mai prodotte; `EventType::Cross` mai emesso ma contato da ds-reference, ds-demo e GUI (la metrica è esclusa come "non confrontabile" con una motivazione falsa).

## Raccomandazione di ordine

1. **A1-A5, A7, A17, A18**: bug veri, piccoli, ognuno con il suo SIM bump e un test (A17/A18 senza bump).
2. **E1-E3**: rendere di nuovo capaci di fallire i test che proteggono i principi (determinismo, piano A).
3. **B1**: chiudere il caveau (nessuna latente intera fuori da ds-db/orchestratore; il motore chiede valori derivati) — è la violazione più seria del principio "un crate per responsabilità" e va con un ADR.
4. **C1, C2, C6**: una sola fonte per porta/area/raggi, e `ticks_per_second` o vero o costante.
5. B2-B4 e le ridondanze D: decisione architetturale (ADR) prima di toccare.
6. Documentazione F: un passaggio unico, solo doc.


## Seconda passata (2026-11-24, notte)

Rilanciata l'analisi con gli stessi 8 analisti, a partire da questo file. Nuovi rilievi e voci chiuse ma non del tutto, in ordine di priorità.

**Alta (cambiano la partita o aprono il piano A)**
- S1. ✅ (SIM 200) Gol perso con la palla appena oltre la linea (0-11 cm) a fine tick: il gol vuole la palla intera, `out_of_play` il centro, e assegna rimessa dal fondo (ball.rs:121, 250; referee.rs:156). Anche la rimessa laterale va a palla intera (Legge 9).
- S2. ✅ (SIM 200) Il lancio alto si valuta verso il punto d'anticipo della corsa, il motore lo calcia dove il compagno è adesso (read.rs:1445, thought.rs:438).
- S3. ✅ (SIM 200) Il colpo di testa pensato: `closing = i64::MAX` dà sempre pressione piena; la potenza contata due volte (lib.rs:637, read.rs:457, 1060).
- S4. ✅ (SIM 200) Peso fisso 0.85 sul tiro alto (read.rs:921): un peso per opzione contro ADR-052.
- S5. Passaggi durante la serie dei rigori (Legge 10): il test usa un seme scelto perché non succede (extra_time.rs).
- S6. ✅ (SIM 200-201: `World` filtra per scheda; il test legge tutto il motore con nome letterale; `PlayerLatents` senza `Debug`; la porta pubblica documentata) `GroundTruth::characteristic(nome)` resta pubblica e generica; i `World` la inoltrano senza guardare la scheda; il test di ADR-112 non legge plugins/mod.rs; `World::top_speed` morto; `PlayerLatents` con campi pubblici e `Debug` in ds-db.
- S7. ✅ (SIM 201) Il motore mette ancora nella scena letture dell'attore (`horizon_cm`, `kicker_nerves`).

**Media**
- S8. ✅ (SIM 201) Portiere espulso o portato fuori: nessuno prende i guanti (Legge 3), ogni tiro nello specchio è gol.
- S9. ✅ (SIM 201) `deflect_ball` non respinge una palla Loose o portata sul palo.
- S10. ✅ (SIM 200) `visible_to`/`nearest_to_point` vedono gli espulsi; il record li elenca tra i ventidue e gli strumenti li contano.
- S11. ✅ (SIM 200) Pressure nomina il compagno più vicino, non il portatore.
- S12. ✅ (SIM 200) Il fuorigioco si cancella se il passaggio manca il destinatario o è deviato (Legge 11).
- S13. ✅ (SIM 201) "Ultimo tocco" diverso per fuori e porta (`Touches` contro possesso).
- S14. ✅ (SIM 200) Chiara occasione negata più vantaggio: rosso e si gioca (la Legge 12 vuole il giallo).
- S15. ✅ (SIM 200) Un cambio impossibile (uomo espulso) occupa per sempre un posto della Legge 3.
- S16. ✅ (SIM 200) Deriva di passaggio e rinvio dal segno assoluto di `dy`, non dal piede.
- S17. ✅ (SIM 201) `Hold` senza la perdita (due misure del tenere palla); `brings()` calcolato e buttato, `progression_*` morte.
- S18. ✅ (SIM 201) Lancio e rinvio pensati con un calciatore medio; il portiere immaginato con 3000/400/200 scritti a mano; `POST_RADIUS_CM`.
- S19. ✅ (SIM 200) Intesa di un subentrato letta con tre ripieghi (0.0, 0.5, base).
- S20. ✅ (SIM 200) Il possesso conta i tick della serie dei rigori.
- S21. Gli strumenti: tabellone numerato da 0; passaggi persi da ds-reference (deviati, giocati di prima, in fuorigioco); portiere per slot in GUI e ds-reference; `--instruct` sul tick assoluto; post-it dei subentrati; arbitro del TUI.
- S22. `MatchRules.duration_minutes`, `half_time_minute`, `Match.seed` mai letti; `MAX_SUBSTITUTIONS` doppio di `subs_allowed`; 10 tick/s ancora in calcolatore, orchestratore e test.
- S23. ✅ (SIM 201) Chiavi morte: `influence.json` (cinque), `reception.json` (due), `decision_bias` con due significati, `speed_ref_cm_per_tick` con due significati.

**Bassa (documentazione e ridondanze)**: doc di B5 nel motore (lib, engine, simulate, save, driver), `sheet.toml` in STRUCTURE, ACTIONS (coraggio e uscita già fatti), README (motore 184), SOURCES (stati "sourced" su valori arbitrari, formule vecchie), `MatchEvent` campi temporali, `why.rs` con 5250/10500/3400 e un minuto diverso dalla cronaca, `LawsBalance` e `Law1` due parser, `MovementBalance`/`KickBalance` non in cache, copie (cono, `check_shot_crossing`/`check_goal_line`, piede, composure, tired), test (knock con "portato fuori", repertorio incompleto, test doppi, id fissi nei test, `reach_has_one_value`), metadati del workspace.


## Terza fase: un motore corretto prima della taratura (concordata il 2026-11-24 con il maintainer)

Obiettivo del maintainer: "partire con un motore stabile e corretto"; togliere la taratura dove lo scarto viene da un bug.

1. **Controlli di regola su molte partite** (`tests/cases/laws_hold.rs`): su molti semi, a ogni tick del record, le Leggi come asserzioni: al più undici in campo e sempre un portiere (Legge 3); nessun espulso in gioco; gol solo con la palla intera oltre la linea, fra i pali e sotto la traversa (Leggi 1, 10); ogni ripresa alla squadra e del tipo giusti (Leggi 15-17); cambi entro il limite; fuorigioco solo su un compagno che riceve (Legge 11). Un seme leggero nel gate, molti semi `#[ignore]`.
2. **Ogni violazione corretta**, una SIM per blocco, con guardia; rilancio fino a controlli puliti.
3. **A ogni giro (anche il punto 4, chiesto dal maintainer):** misura delle 22 metriche; ogni metrica fuori banda è un indizio: si cerca negli eventi delle partite più lontane la catena causale, si decide **bug** (si corregge, SIM + guardia) o **taratura** (meccanismo giusto, parametro arbitrario: si annota, non si tocca guardando la metrica, A5-bis). Esito in una tabella "metrica | fuori banda | causa trovata | bug o taratura" nella misura di ogni SIM in `L0-CALIBRATION.md`.
4. Poi le voci rimaste della seconda passata (S5, S21, S22, ridondanze di H) e la chiusura di L0 (≥16/22 in banda, ogni fuori banda spiegato), poi L1.

Stato al momento della scrittura: SIM 200 e 201 su main; SIM 202 (punti 1-2) chiuso, sotto.

**SIM 202 (2026-11-24, DD-62).** Punto 1 fatto: `tests/cases/laws_hold.rs` (seme 42 nel gate, 42..142 con `--ignored`, `LAWS_FROM`/`LAWS_TO`). Due violazioni trovate e corrette: (a) la porta senza la misura della palla (assi di pali e traversa a un raggio fuori, il raggio della palla conta, `in_the_goal_mouth` unico giudice, `crossbar_thickness_cm` tolto; guardia `the_ball_has_a_size_at_the_goal`); (b) il fuorigioco sulla palla sciolta con il tick del passaggio (`MatchState::offside_played`). Dopo: 100 partite su 100 pulite. Misura: **6 su 22**, con la tabella degli indizi in `L0-CALIBRATION.md` ("La misura del 202"). **Prossimo:** punto 3, seguire gli indizi della tabella, a partire dagli autogol (×10) e dai gialli (candidati bug).

5. **Test più veloci** (chiesto dal maintainer il 2026-11-24): misurare il tempo di ogni test e trovare i più lenti; riusare le partite simulate (cache `tests/support`); partite più corte dove la regola lo consente; raggruppare golden e semi in una corsa release; `opt-level` delle dipendenze nel profilo di test. Vincolo: nessuna copertura persa.


## Terza passata (2026-11-24, mattina)

Stessi 8 analisti in sola lettura, a HEAD `5eafc14` (SIM 202), con la tabella degli scarti del 202 come indizi: ogni metrica fuori banda trattata come bug da trovare, mai come numero da ritoccare (maintainer, 2026-11-24). Accorpati i doppioni; fra parentesi le aree che l'hanno trovato. Le sonde sugli autogol (62 autogol su 42..101): in 42 il portiere era entro 2 m dalla palla negli ultimi 15 tick, in 61 entro 5 m; quasi sempre un pallone rasoterra lento (4-6 m/s).

**Blocco 1 — la porta e i palloni verso la porta (autogol ×10, gol, angoli)**
- T1. ✅ (SIM 203) `intercept` prevede l'incontro con la velocità massima piena e senza frenata; il movimento corre al 75/90 (`contest_pace_share`) e frena per arrivare: chi va sulla palla punta un punto troppo avanti, arriva tardi, ripunta più avanti; il portiere finisce sulla linea mentre la palla entra (seme 49, tick 44309-44325). ADR-052 (spazio; sonda).
- T2. ✅ (SIM 203) Il controllo fallito lascia la palla attraversare il corpo a metà velocità (`receives_loose`, tocco pesante, `deflects_the_pass`); solo `off_the_body` la fa rimbalzare: un difensore che sbaglia la accompagna verso la sua porta (spazio, motore).
- T3. ✅ (SIM 203) Il portiere non incontra mai una palla che non sia un tiro: `check_goal_line` dà il gol senza `keeper_meets_the_shot` (motore; sessione interrotta).
- T4. ✅ (SIM 203) Gol perso con la palla lenta: l'incrocio vuole `prev.0 <= length` ma la palla intera è oltre solo a `length + r`; fra i due il gol sparisce e diventa fondo o angolo (S1 chiusa a metà) (regole, spazio, motore).
- T5. ✅ (SIM 203) La parata trattenuta lascia la palla oltre la linea: nello stesso tick `restart_if_out` dà l'angolo (regole).
- T6. ✅ (SIM 203) Il muro sul tiro guarda solo la palla a fine tick, non i piccoli passi: un tiro a 25 m/s attraversa il difensore (confini, motore).
- T7. ✅ (SIM 203) `play_clearance` (spazzata di prima) non registra chi calcia: ultimo tocco stantio per fondo, laterale, gol/autogol (motore; sessione interrotta).
- T8. ✅ (SIM 203) "Tiro in porta" giudicato sul punto mirato e senza il raggio della palla (`resolve_shot` `on_target`, `header_at_goal`); `in_the_goal_mouth` non è l'unico giudice come il 202 dichiara (tutti).
- T9. Il primo tocco "nella corsa" può portare la palla verso la propria porta e lui non decide finché non la raggiunge (spazio).
- T10. ✅ (SIM 223, ADR-115) Nel tick porta e linee sono giudicate prima dei contatti: nell'ultimo tratto nessun salvataggio sulla linea (confini). Tocca l'ordine delle fasi (B9): con un ADR.

**Blocco 2 — la misura (ds-reference)**
- T11. ✅ (RUN 36) La banda era gonfiata: 1017 tentativi Wyscout 9 con tag 101 su 380 partite (2.676/m = i gol); esclusi, saves 6.695 ± 2.823 e il motore 5.27 è in banda (strumenti).
- T12. Passaggi: il motore conta come passaggio anche rimesse, punizioni, angoli e rinvii, Wyscout no (circa 90 a partita) (strumenti).
- T13. Falli e gialli reali contano offese che il motore non conosce (simulazione, perdite di tempo, proteste) (strumenti).
- T14. Gli eventi della serie dei rigori entrano nei conteggi (motore).
- T14-bis. Recupero: la banda reale è l'ultimo evento Wyscout (380 partite, 1.488/4.083 min); il fischio non è tra i suoi eventi, quindi è un limite inferiore, non il tempo aggiunto che il motore misura al `HalfTime`. `compare` marcava le due righe come confrontabili (`lib.rs:138`). Corretto: `comparable()` le esclude; la banda resta come limite inferiore, nessun secondo ritoccato (strumenti).

**Blocco 3 — le Leggi**
- T15. ✅ (SIM 203) Legge 7: il fallo con il vantaggio aggiunge 20 s; cambi e infortuni 0 (tutti).
- T16. Legge 12: giallo solo per la scivolata da dietro e la chiara occasione; mancano l'azione promettente interrotta, l'imprudenza in piedi, la reiterazione (tutti). Il rosso per vigoria non può scattare (soglia 103.5 cm/tick contro una corsa massima di circa 83 in contesa) (dati).
- T17. Leggi 13, 15-17: le riprese si giocano come palla in movimento (il battitore conduce, spazza, tira; la rimessa è calciata; gol diretto da rimessa); nessuna punizione indiretta (regole, motore).
- T18. ✅ (SIM 203) Legge 16: al rinvio gli avversari restano in area; Legge 15: niente 2 m alla rimessa (regole, motore).
- T19. ✅ (SIM 203) Legge 8: il calcio d'inizio del secondo tempo e dei supplementari con i giocatori dove sono; non si cambia campo (regole, motore, strumenti, test).
- T20. ✅ (SIM 203) L'intercetto "a orologio" (`FLIGHT_TICKS`): `Interception` senza tocco, possesso girato, segni del fuorigioco che restano (regole, motore).
- T21. ✅ (SIM 203) Legge 11: al tiro non si marca il fuorigioco (la ribattuta di chi era oltre non è punita) (test).
- T22. Minori: lato della ripresa vicino alla bandierina (y prima di x); retropassaggio preso con le mani; portiere espulso fino alla sosta (S8 a metà); `denial` e muro e spazzata col portiere per slot; il secondo giallo non contato fra i gialli.

**Blocco 4 — il fuorigioco nelle posizioni**
- T23. `shape.rs`: le punte del blocco alla pari della linea per costruzione (+1500 da entrambe le parti), oltre la linea negli ultimi 25 m e con il blocco alto; a ogni cambio di possesso tutti tornano al blocco (spazio).
- T24. ✅ (SIM 203) `Hold`: il posto del ruolo (`anchor`) non è riportato in gioco; l'errore di lettura della linea sempre in avanti (200 cm scritti) contro un margine di 50 (orchestratore).
- T25. ✅ (SIM 203) `foresee` rialza verso 0.5 una fiducia zero che viene da un fatto certo (fuorigioco, palla nella propria porta) (orchestratore).
- T26. Il filtrante non è limitato al campo e non ha controllo né errore (orchestratore).

**Blocco 5 — quello che il giocatore prevede è quello che il motore esegue (ADR-052)**
- T27. La spazzata pensata scende a 40 m in un volo lineare; quella eseguita è sempre a potenza piena (circa 50 m in volo, poi rimbalzi) (orchestratore, dati).
- T28. ✅ (SIM 203) Il tiro pensato ha sempre la velocità del tiro forte (`speed(0.85, 0.5)`): il piazzato sopravvalutato (orchestratore, dati).
- T29. ✅ (SIM 204: la spazzata di testa solo sul pallone loro; nessun tiro di testa d'ufficio; SIM 205: contende chi arriva nel tick) Il colpo di testa a velocità di tiro di piede; ogni pallone alto vinto in area avversaria diventa un tiro (nessuna sponda); nella propria metà ogni pallone alto è spazzato, anche il passaggio di un compagno (orchestratore, motore).
- T30. `first_time` sceglie su una scena più povera di `decide` (niente conduzione, `Hold`, filtranti, altezze): A8 chiuso a metà (orchestratore).
- T31. ✅ (SIM 203) Con tutte le opzioni filtrate vince lo slot più basso (il portiere): `NEG_INFINITY` e tie-break per slot (orchestratore).
- T32. ✅ (SIM 203) Il ricevitore fermo decide subito, senza il suo tempo di decisione (motore).
- T33. Il resto: solo il passaggio guarda un passo avanti (non-bug per ADR-101, DD-76: il lancio resta fuori da OI-116); il minimo di 11 m della dispersione non è nel pensiero; `Advance` pensato oltre il limite dell'area; il lancio con la velocità del rasoterra; `Hold` senza la perdita (S17 a metà); spazzata e lancio pensati con un calciatore medio e `keeper_off` 3000/400/200 (S18 a metà); il passaggio nella corsa di un compagno non visto; tiro dal corpo, non dalla palla; pressione della spazzata dalla palla; discretizzazione di rotolamento e volo diversa da quella pensata (2,6%) (orchestratore, spazio, motore).

**Blocco 6 — guardie e documenti**
- T34. `laws_hold` non controlla ciò che dichiara (portiere per tick, chi batte la rimessa, angolo contro ultimo tocco, fuorigioco su chi riceve), salta supplementari e rigori (S5 ancora coperto da un seme scelto), manca il controllo inverso "ogni palla intera fra i pali è gol" (test, regole).
- T35. `anti_leak` taglia `balance.rs` al primo `#[cfg(test)]` (righe 357-725 non lette); cerca solo `.resolve_choice(`; liste di file a mano (confini, test).
- T36. Test che non possono fallire (seme diverso con fixture diversa nel test-runner, `verify` tautologico, `functional_l0` e altri); commenti dei test e README fermi (test, strumenti).
- T37. Ridondanze: `ball.json`, `laws.json`, `movement.json`, `profiles.json` letti due volte (B8); il raggio della palla preso da `ds_orchestrator::read`; `World::characteristic` silenzioso in release; schede che non dicono chi scrive punteggio e fuorigioco (confini, dati).

**Stato (SIM 203):** fatte T1-T8, T15 (vantaggio; cambi e infortuni a 0 s restano: serve una fonte per i secondi), T18 (rinvio; i 2 m della rimessa no), T19, T20, T21, T24, T25, T28, T31, T32. Aperte le altre. **Ordine:** blocco 1 (SIM 203), poi il 2 (misura onesta, senza SIM), poi 3, 5, 4, 6; una SIM per blocco o per gruppo coerente, ognuna con la sua guardia, C2 e misura con la tabella. Poi una quarta passata.
