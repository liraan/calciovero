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
| A8 | `Actor::first_time` valuta "controllare" con tiratore perfetto, piede destro fisso, `first_touch` vuoto (controlla sempre) → il gioco di prima è sottovalutato | actor.rs:206-207 | analista |
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
| B2 | **Il motore decide**: `MatchState::decide` è in ds-match (thought.rs) e chiama `resolve_pass/duel/shot`; scelte prese nel motore (piede migliore, uscita del portiere con soglia 0.5, posto da chiudere, cambi dell'allenatore in bench.rs). ADR-041 §4 dice che MatchState non chiama `resolve_*`; il test anti_leak cerca solo `.resolve_choice(` | analista |
| B3 | **Formule fuori dal calcolatore**: l'orchestratore chiama `latents_for` 38 volte e scrive formule e pesi suoi (duello, battitore, scansione); formule anche in ds-match/balance.rs (`KnockBalance::knocks`, `CoachBalance::minute`, `stoppage_for`) e in set_pieces (barriera) | analista |
| B4 | **Fisica della palla in tre crate con modelli diversi**: motore (con aria), orchestratore (tiro senza aria, lancio con aria), calcolatore (pallonetto a velocità costante "the same physics") → l'attore immagina un volo diverso da quello che il motore esegue | analista |
| B5 | **Il motore dipende dalla cronaca** (ds-match → ds-narrative, `NarrativeEntry` dentro `MatchResult`), contro STRUCTURE.md e ADR-063 | ✔ (Cargo) |
| B6 | **Cronaca scritta fuori da ds-narrative**: `why_conceded` in ds-demo, usato dalla GUI | analista |
| B7 | **Statistiche fuori da ds-stats**: `MatchStatistics` in ds-match, ds-reference riconta tiri/tiri in porta con un'altra definizione | analista |
| B8 | **Bilanciamento caricato in due crate**: ds-match/balance.rs carica 12 JSON; `ball.json` è deserializzato due volte con tipi diversi (i16 vs f64) | analista |
| B9 | **Plugin solo di nome**: 3 plugin su 11 implementano `run()`; l'ordine delle fasi dichiarato in ds-match-api (`Phase::ALL`) non è quello eseguito; le schede non dichiarano le letture vere (fatigue legge endurance; referee injury_proneness; "knocks: no code yet" ma esiste) | analista |
| B10 | ds-gui dipende da ds-demo (un tool da un altro tool): ds-demo è anche libreria condivisa | analista |

Tabella dei crate (dall'analista): rispettano la responsabilità ds-core, ds-model, ds-fixtures, ds-testkit, ds-test-runner, ds-gui (quasi); **parziale** ds-stats, ds-calculator, ds-orchestrator, ds-match-api, ds-narrative, ds-demo, ds-reference; **no** ds-db (caveau aperto) e ds-match (decide, legge il piano A, dipende dalla cronaca, carica balance e formule).

## C. Dati e balance: stessi numeri in più posti, numeri senza fonte

**Stato (2026-11-23):** C1 chiuso: la Legge 1 ha una sola fonte (`laws.json` → `ds_calculator::laws`); tolti i doppioni in obstacles.json, thought.json, offball.json e i 366/244/732/3400 nel codice; golden identico. **C2 chiuso** con nomi distinti, senza cambiare la partita: `choice_pressure_radius_cm` (300, quanto si sente pressato chi sceglie) e `pressure_radius_cm` (500, l'evento Pressure e la ricezione); `watch_mark_radius_cm` (800, chi marca guarda il suo uomo) e `mark_radius_cm` (300, chi si smarca). Unificarli a 5 m è stato provato (SIM 197, non committato): rompeva due guardiani di comportamento (la squadra più larga su richiesta, il pallone avanti nel dribbling); entrambi i valori sono senza fonte, restano distinti fino alla taratura. **C6 chiuso**: `TICKS_PER_SECOND` costante, il motore rifiuta un altro valore, gli strumenti la leggono; tolto `MatchConfig.duration_minutes` (mai letto). **Chiusi anche:** C3 in parte (tolte `reading_weight`, `risk_weight`, `clear_salience` e `ChoiceCandidate.risk`, mai letti; restano le sei chiavi di offball.json raggiungibili solo nei test, che vanno con B2), C4 (le quattro righe mancanti; il conteggio di SOURCES rifatto: 214 parametri, 0 senza riga), C7 (il custode legge `max_speed` e `maximal_force`; `Acceleration` resta senza radice e lo dice), C8 (i bilanci del pensiero, del senza palla, del tiro, del blocco e del colpo letti una volta: la suite d'integrazione 255 → 146 s). **Aperto:** C5 (numeri nel codice senza chiave né fonte).

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

**Secondo passaggio, golden identici:** la velocità del passaggio rasoterra pensato è `KickBalance::weighed_pass_speed` (era in quattro punti, anche in offball), i tick di rotolamento `KickBalance::rolling_ticks` (due), l'arrivo del pallonetto `lob_landing` (due). **Aperti:** le formule che divergono già fra loro (vedi sotto: "ci arriva in tempo", la pressione sul calcio, `choose_place`/`choose_closed_place`, i due `World`): unirle cambia la partita e va con la sezione B.

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

**Stato (2026-11-23):** fatti: conteggio dei test (607, più uno ignorato), `rules.rs` → `plugins/referee.rs` in CONTRIBUTING, ACTIONS e L0-match/README, intestazione e §4.1 di ACTIONS (ADR-042 accettato), ADR-002 superato nella tabella, plugin e balance descritti come sono (moduli con scheda; `.json`), _index senza crate inesistenti e con i quattro che mancavano, i nomi `pcfm-*` nei commenti del codice, le guardie anti-leak di ds-demo che cercavano un crate scomparso, il doc del custode. Secondo passaggio: i cross contati dal qualificatore del Pass (ds-reference, ds-demo), le note di duel.json (retry 50), laws.json (il portiere non è copertura), keeper.json (la regola del SIM 124), il formato v2 del record, l'esempio `--html` di ds-demo. Terzo passaggio: il doc di `move_agents` e di `check_shot_crossing`, i commenti sui "dadi" (outcome, context, save, ball, thought). **Aperti:** i corsivi di ACTIONS, le varianti di evento mai prodotte (`RandomEvent` e altre cinque: toglierle cambia il formato dei salvataggi, va con un ADR).

Conteggio dei test 589 contro 605 `#[test]`; `rules.rs` citato in CONTRIBUTING/ACTIONS/README ma non esiste (è plugins/referee.rs); ACTIONS.md fermo al SIM 76 con il duello vecchio e corsivi sbagliati; ADR-002 "accepted" e "superseded" nella stessa STRUCTURE.md; `sheet.toml` e balance ".toml" che non esistono; _index.md con crate inesistenti (ds-metrics, ds-data, ds-gen) e senza quelli veri; nomi `pcfm-*` in molti doc e in una guardia anti-leak che cerca un crate scomparso; doc di `move_agents`, keeper.json, laws.json (portiere come copertura), duel.json (retry 30 vs 50), `MatchConfig` (file toml inesistente), formato record v1 vs v2, `ds-demo --help` con un esempio `--html /tmp/…` che fallisce sempre; molti commenti su "die"/dadi contro ADR-035; `RandomEvent` e altre 5 varianti di evento mai prodotte; `EventType::Cross` mai emesso ma contato da ds-reference, ds-demo e GUI (la metrica è esclusa come "non confrontabile" con una motivazione falsa).

## Raccomandazione di ordine

1. **A1-A5, A7, A17, A18**: bug veri, piccoli, ognuno con il suo SIM bump e un test (A17/A18 senza bump).
2. **E1-E3**: rendere di nuovo capaci di fallire i test che proteggono i principi (determinismo, piano A).
3. **B1**: chiudere il caveau (nessuna latente intera fuori da ds-db/orchestratore; il motore chiede valori derivati) — è la violazione più seria del principio "un crate per responsabilità" e va con un ADR.
4. **C1, C2, C6**: una sola fonte per porta/area/raggi, e `ticks_per_second` o vero o costante.
5. B2-B4 e le ridondanze D: decisione architetturale (ADR) prima di toccare.
6. Documentazione F: un passaggio unico, solo doc.
