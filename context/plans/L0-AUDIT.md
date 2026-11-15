# L0 — il ricontrollo (2026-11-15)

**Cos'è.** La fase 1 del mandato del maintainer del 2026-11-15 (*"completare L0"*): ogni voce dichiarata nei documenti confrontata con il codice e i test di `main` a `a343b05` (`SIM_VERSION` 114). Nessun cambio di calcio. Le correzioni sono nella fase 2; il calcio che manca nella fase 3 (`plans/L0-CLOSE.md`).

## 1. Il segnalibro, la versione, i test

| Voce | Stato dichiarato | Stato reale (prova) | Cosa fare |
|---|---|---|---|
| Segnalibro E4b | `STATE.md`: LCV `de71d3e` | `git log`: HEAD `a343b05` è il segnalibro, il commit prima è `de71d3e` | niente |
| `SIM_VERSION` | `STATE.md`, `CHANGELOG-SIM.md`: 114 | `ds-match/src/save.rs`: 114; i due golden passano su glibc, musl e Windows (113) | niente |
| Numero dei test | `README.md` (badge e testo), `doctrine/_index.md`: 448 | `cargo test --workspace`: 448 passati (SIM 114, 2026-11-15); 440 dopo la fase 2, che ha tolto i test del bus eventi e dell'arbitro | `STRUCTURE.md` e `CONTRIBUTING.md` non riportano il numero: niente da allineare |
| Numero dei crate | `_index.md`: 13 | `Cargo.toml` `members`: 13 | niente |

## 2. Gli OI aperti (23)

| OI | Stato dichiarato | Stato reale (prova) | Cosa fare |
|---|---|---|---|
| OI-7, OI-8 | scouting L2 | non tocca L0 | restano, per L2 |
| OI-27 | l'intenzione dell'allenatore, prima forma | superato in parte: ADR-059 passi 1 e 2 fatti (104: la forma con la palla, pesata dalla disciplina); resta il passo 3 (istruzioni rapide e transizioni) | chiudere OI-27 rimandando ad ADR-059 passo 3, che va nel piano |
| OI-31 | la specifica degli eventi diverge | vero: `specs/05-match-events.md` non nomina `KickOff`, `SecondHalf`, `Instruction` (33 tipi nell'enum di `ds-stats/src/event.rs`) | fase 2: allineare la specifica all'enum |
| OI-38 | tre decisioni del ruleset rinviate a L1 | vero | resta, per L1 |
| OI-40 | il bus eventi di `ds-core` senza consumatori | vero: `event_bus` non è usato fuori da `ds-core`; le `ObservationEvent` hanno `drain_observations` ma nessuno le svuota in partita | fase 2: togliere il bus di `ds-core` (default dichiarato nell'OI), golden identico |
| OI-49 | i documenti di navigazione invecchiano | vero: `design/ARCHITECTURE.md` (71 righe con `rng`/`dice`/`pcfm`/`Streams.`), `design/CODE_TOUR.md` (30), `doctrine/GLOSSARY.md` (22) | fase 2: rileggerli contro il codice |
| OI-55 | `Referee.strictness`/`consistency` da togliere | vero: `ds-model/src/referee.rs` li ha e li valida | fase 2: toglierli, golden identico (se cambia, fase 3) |
| OI-64 | il corpo rispetto alla palla | vero | fase 3, voce 9 |
| OI-69 | il fisco delle donazioni | del maintainer | resta |
| OI-70 | gli altri appannatori | vero: nessun lettore per la concentrazione che cala, la compostezza, il coraggio | fase 3, voce 8 |
| OI-85 | `choice.json` sopravvive ad ADR-052 | vero in parte: `perception_floor`, `perception_range` non hanno lettori fuori dai commenti | fase 2: togliere i campi senza lettore |
| OI-90 | campi morti | vero: `PassContext.receiver_freedom_cm` scritto e mai letto; `shot_choice.json` `shot_distance_ref_cm`, `shot_cone_ref`, `carry_salience`, `other_side_factor` (L0-CLOSE passo 3 dice tolto: non lo è), `loft_salience`, `loft_max_cm` senza lettore | fase 2: toglierli |
| OI-94 | `ds-match` ha più motivi di cambiamento | vero (ADR-041 rinvia lo split) | fase 5, dopo l'ok del maintainer |
| OI-98 | autogol e linea del fuorigioco | in parte superato: 113 e 114 fondano la lettura della linea (fuorigioco 2.97 contro 4.26); autogol 0.32 contro 0.10 | resta, con la prima azione: un autogol del 114 tick per tick |
| OI-99 | il golden su più sistemi | Linux glibc e musl, Windows UCRT64 (113) fatti | resta per macOS e ARM64; comando nella fase 5 |
| OI-100 | il gioco troppo preciso | vero (114: passaggi riusciti 96%, tiri in porta 56%) | fase 3, voce 3 e fase 4 |
| OI-101 | il recupero lungo per dottrina | vero (114: 5.5 e 5.1 min) | fase 4 |
| OI-102 | il secondo uomo sui palloni alti | vero (114: duelli aerei 11.3 contro 38.1) | fase 3, voce 9 |
| OI-103 | protezione e dribbling | vero (114: dribbling riusciti 8.7 contro 16.0) | fase 3, voce 9 |
| OI-104 | le fonti dell'errore di percezione | superato in parte: approvato, (b) fatto nel 114 | aggiornare: restano (a) e (c), dopo la memoria |
| OI-105 | le risposte sul 10h | vero; la monetina approvata (*"si procedi"*, 2026-11-15) come input della partita | aggiornare |
| OI-106 | la scena solo dal cono e dalla memoria | vero: `state.rs` `decide_off_ball` `rank`, `loose_rival_cm`, `cutters` leggono tutta la squadra | fase 3, voci 1 e 2 |

## 3. I passi di L0-CLOSE

| Passo | Stato dichiarato | Stato reale (prova) | Cosa fare |
|---|---|---|---|
| 1-9 | fatti | i loro test sono nella suite verde (448); il repertorio (`tests/repertoire.rs`) verde, il rigore dichiarato raro | niente; passo 3: `other_side_factor` non è stato tolto (OI-90) |
| 10a-10c, 10g | fatti | gemelli presenti (catalogo `design/ACTIONS.md`) | niente |
| 10d il portiere | in parte | il piazzarsi al tiro, la profondità letta, l'uscita sul passaggio: senza causa | fase 3, voce 9 |
| 10e il corpo | il tempo del salto aperto | vero | fase 3, voce 9 |
| 10f l'allenatore in partita | le sostituzioni pianificate | vero: nessun cambio deciso in partita | fase 3, voce 6 |
| 10h | da fare | supplementari e rigori mai giocati (`MatchRules.extra_time`, `penalty_shootout` senza lettore); la barriera assente; `Injury` mai emesso | fase 3, voci 4-7 |
| ADR-057 (il passo 10 si chiude anche qui) | da fare | `CHARACTERISTICS-AND-ABILITIES.md` §8 da verificare riga per riga; `injury_proneness` generato senza consumatore | fase 3, voce 7 (il suo consumatore) e fase 4 |
| 11 | in corso | ultima misura 114 (tabella in `CHANGELOG-SIM.md`) | fase 4 e fase 5 |

## 4. Gli ADR che toccano L0

| ADR | Stato | Il codice lo rispetta? | Cosa fare |
|---|---|---|---|
| Tutti | 55 accettati; nessun proposto (gate B6) | — | — |
| ADR-035 (niente dadi) | accettato | sì: nessuno stream di numeri casuali nella simulazione (gate B9, `clippy.toml`) | niente |
| ADR-041 (lo split di `ds-match` rinviato) | accettato | sì, lo split non è fatto (OI-94) | fase 5 |
| ADR-052 (ogni giocatore pensa) | accettato | in parte: tre letture del pensiero senza palla vedono il campo vero (OI-106) | fase 3, voce 2 |
| ADR-059 (l'allenatore parla) | accettato | passi 1-2 sì; passo 3 da fare | fase 3, voce 6 (prima il passo 3) |
| ADR-062 (non si vede esatto) | accettato, grandezza approvata | (b) sì (114); (a) e (c) no: manca la memoria (MD-40) | fase 3, voci 1 e 3 |

## 5. I documenti contro il codice

| Documento | Divergenza | Cosa fare |
|---|---|---|
| `specs/05-match-events.md` (OI-31) | dei 33 tipi dell'enum mancano `KickOff`, `SecondHalf`, `Instruction`; `Slip` e `ControlError` non sono più nella specifica (la seconda metà dell'OI è superata) | fase 2 |
| `design/ARCHITECTURE.md`, `design/CODE_TOUR.md`, `doctrine/GLOSSARY.md` (OI-49) | parlano di flussi casuali, dadi e nomi `pcfm` superati da ADR-035 e ADR-032 | fase 2 |
| `specs/01-rng.md`, `specs/08-random-events.md` | descrivono il caso che ADR-035 ha tolto | fase 2: segnarli superati, con il rimando |

## 6. Codice morto e residui

| Voce | Prova | Cosa fare |
|---|---|---|
| `Referee.strictness`, `consistency` (OI-55) | solo dichiarati e validati in `ds-model/src/referee.rs` | togliere |
| `choice.json` `perception_floor`, `perception_range` (OI-85) | nessun lettore | togliere |
| `shot_choice.json` sei campi (OI-90) | nessun lettore | togliere |
| `PassContext.receiver_freedom_cm` (OI-90) | scritto, mai letto | togliere |
| il bus eventi di `ds-core` (OI-40) | nessun consumatore | togliere |
| `injury_proneness` | generato, nessun consumatore in partita | non morto: aspetta il suo consumatore (fase 3, voce 7) |

## 7. I parametri di `balance/` e il loro grado (decisione 5)

Parametri di `SOURCES.md`: 213 (dopo la fase 2, che ha tolto otto parametri senza lettore); foglie dei profili dei ruoli: 168, tutte grado 3. **Totale: 381 parametri; grado 1: 25; grado 2: 0; grado 3: 356.**

| file | parametro | valore | stato in SOURCES.md | grado |
|---|---|---|---|---|
| `ball.json` | `gravity_cm_per_tick2` | 10 | undocumented | 3 |
| `ball.json` | `air_friction_milli` | 990 | undocumented | 3 |
| `ball.json` | `ground_friction_milli` | 957 | undocumented | 3 |
| `ball.json` | `stop_threshold_cm_per_tick` | 1 | undocumented | 3 |
| `block.json` | `depth_in_possession_cm` | 4500 | undocumented | 3 |
| `block.json` | `depth_out_of_possession_cm` | 3200 | undocumented | 3 |
| `block.json` | `width_in_possession_cm` | 6000 | undocumented | 3 |
| `block.json` | `width_out_of_possession_cm` | 4000 | undocumented | 3 |
| `block.json` | `line_behind_ball_in_possession_cm` | 3000 | undocumented | 3 |
| `block.json` | `line_behind_ball_out_of_possession_cm` | 1500 | undocumented | 3 |
| `block.json` | `line_min_cm` | 1000 | undocumented | 3 |
| `block.json` | `line_max_in_possession_cm` | 5800 | undocumented | 3 |
| `block.json` | `line_max_out_of_possession_cm` | 4500 | undocumented | 3 |
| `block.json` | `ball_side_shift_milli` | 600 | undocumented | 3 |
| `block.json` | `high_block_line_cm` | 1200 | explained, no source | 3 |
| `block.json` | `low_block_line_cm` | 1500 | explained, no source | 3 |
| `block.json` | `width_change_milli` | 200 | explained, no source | 3 |
| `carry.json` | `carry_distance_cm` | 1000 | undocumented | 3 |
| `carry.json` | `carry_stop_from_goal_cm` | 1650 | sourced | 1 |
| `choice.json` | `reading_weight` | 0.5 | undocumented | 3 |
| `choice.json` | `risk_weight` | 0.5 | undocumented | 3 |
| `choice.json` | `decision_bias` | 0.2 | arbitrary, declared | 3 |
| `choice.json` | `floor` | 0.5 | undocumented | 3 |
| `choice.json` | `clarity_threshold` | 0.25 | undocumented | 3 |
| `duel.json` | `reach_cm` | 120 | explained, no source | 3 |
| `duel.json` | `retry_ticks` | 50 | explained, no source | 3 |
| `duel.json` | `defender_tackling_milli` | 450 | undocumented | 3 |
| `duel.json` | `defender_explosive_milli` | 200 | undocumented | 3 |
| `duel.json` | `defender_force_milli` | 200 | undocumented | 3 |
| `duel.json` | `defender_body_milli` | 100 | undocumented | 3 |
| `duel.json` | `carrier_first_touch_milli` | 450 | undocumented | 3 |
| `duel.json` | `carrier_explosive_milli` | 200 | undocumented | 3 |
| `duel.json` | `carrier_force_milli` | 200 | undocumented | 3 |
| `duel.json` | `carrier_body_milli` | 200 | undocumented | 3 |
| `duel.json` | `loose_margin_milli` | 40 | undocumented | 3 |
| `duel.json` | `loose_speed_cm_per_tick` | 50 | undocumented | 3 |
| `duel.json` | `slide_reach_milli` | 1400 | explained, no source | 3 |
| `duel.json` | `slide_bonus_milli` | 80 | undocumented | 3 |
| `duel.json` | `slide_recovery_ticks` | 60 | explained, no source | 3 |
| `duel.json` | `shield_force_milli` | 150 | undocumented | 3 |
| `duel.json` | `shield_body_milli` | 100 | undocumented | 3 |
| `duel.json` | `dribble_dribbling_milli` | 500 | undocumented | 3 |
| `duel.json` | `dribble_explosive_milli` | 250 | undocumented | 3 |
| `duel.json` | `dribble_body_milli` | 200 | undocumented | 3 |
| `duel.json` | `beaten_ticks` | 80 | explained, no source | 3 |
| `duel.json` | `answer_trust_milli` | 250 | explained, no source | 3 |
| `duel.json` | `standing_salience_milli` | 600 | undocumented | 3 |
| `duel.json` | `slide_salience_milli` | 900 | undocumented | 3 |
| `duel.json` | `jockey_salience_milli` | 500 | undocumented | 3 |
| `duel.json` | `foul_threshold_milli` | 800 | undocumented | 3 |
| `duel.json` | `foul_slide_milli` | 300 | undocumented | 3 |
| `duel.json` | `foul_behind_milli` | 350 | undocumented | 3 |
| `duel.json` | `foul_temper_milli` | 500 | undocumented | 3 |
| `duel.json` | `foul_speed_milli` | 200 | undocumented | 3 |
| `duel.json` | `foul_technique_milli` | 200 | undocumented | 3 |
| `duel.json` | `speed_ref_cm_per_tick` | 90 | undocumented | 3 |
| `duel.json` | `area_slide_caution_milli` | 250 | explained, no source | 3 |
| `duel.json` | `booked_slide_caution_milli` | 150 | explained, no source | 3 |
| `duel.json` | `jump_ref_cm` | 55 | explained, no source | 3 |
| `duel.json` | `hands_reach_cm` | 70 | explained, no source | 3 |
| `duel.json` | `header_speed_cm_per_tick` | 130 | undocumented | 3 |
| `duel.json` | `aerial_heading_milli` | 400 | undocumented | 3 |
| `duel.json` | `aerial_jump_milli` | 300 | undocumented | 3 |
| `duel.json` | `aerial_force_milli` | 150 | undocumented | 3 |
| `duel.json` | `aerial_height_milli` | 150 | undocumented | 3 |
| `duel.json` | `keeper_hands_bonus_milli` | 300 | undocumented | 3 |
| `fatigue.json` | `effort_ref_cm_per_tick` | 90.0 | undocumented | 3 |
| `fatigue.json` | `rest_effort` | 0.02 | explained, no source | 3 |
| `fatigue.json` | `half_time_recovery` | 0.3 | sourced | 1 |
| `fatigue.json` | `match_profile` | … | derived | 1 |
| `fatigue.json` | `match_ticks` | 54000.0 | undocumented | 3 |
| `fatigue.json` | `late_high_intensity_decline` | 0.3 | derived | 1 |
| `fatigue.json` | `late_top_speed_decline` | 0.04 | derived | 1 |
| `influence.json` | `w_space` | 100 | undocumented | 3 |
| `influence.json` | `w_skill` | 40 | undocumented | 3 |
| `influence.json` | `w_cost` | 20 | undocumented | 3 |
| `influence.json` | `w_red` | 50 | undocumented | 3 |
| `influence.json` | `space_threshold_cm` | 2000 | explained, no source | 3 |
| `influence.json` | `pressure_radius_cm` | 500 | explained, no source | 3 |
| `influence.json` | `pressure_exit_milli` | 1300 | explained, no source | 3 |
| `keeper.json` | `positioning_range_cm` | 3000 | undocumented | 3 |
| `keeper.json` | `advance_cm` | 400 | undocumented | 3 |
| `keeper.json` | `support_gap_cm` | 2500 | explained, no source | 3 |
| `keeper.json` | `support_max_cm` | 2200 | explained, no source | 3 |
| `keeper.json` | `depth_share_milli` | 500 | explained, no source | 3 |
| `kick.json` | `speed_ref_cm_per_tick` | 340.0 | undocumented | 3 |
| `kick.json` | `speed_power_base` | 0.5 | undocumented | 3 |
| `kick.json` | `speed_explosive_base` | 0.6 | undocumented | 3 |
| `kick.json` | `error_ref_rad` | 0.062 | sourced | 1 |
| `kick.json` | `pass_error_ref_rad` | 0.08 | derived | 1 |
| `kick.json` | `throw_speed_cm_per_tick` | 180.0 | explained, no source | 3 |
| `kick.json` | `throw_max_cm` | 3500 | explained, no source | 3 |
| `kick.json` | `error_power_gain` | 1.0 | undocumented | 3 |
| `kick.json` | `error_pressure_rad` | 0.08 | undocumented | 3 |
| `kick.json` | `run_error_rad` | 0.15 | arbitrary, declared | 3 |
| `kick.json` | `run_speed_ref_cm_per_tick` | 80.0 | undocumented | 3 |
| `kick.json` | `vertical_share` | 2.0 | explained, no source | 3 |
| `kick.json` | `aim_low_cm` | 40 | explained, no source | 3 |
| `kick.json` | `aim_high_cm` | 200 | explained, no source | 3 |
| `kick.json` | `pass_arrive_cm_per_tick` | 60.0 | explained, no source | 3 |
| `kick.json` | `clear_length_cm` | 4000 | explained, no source | 3 |
| `kick.json` | `clear_loft` | 0.35 | explained, no source | 3 |
| `kick.json` | `loft_ticks_per_10m` | 6.0 | explained, no source | 3 |
| `latency.json` | `base_ticks` | 7 | sourced | 1 |
| `latency.json` | `decision_bias` | 1.0 | explained, no source | 3 |
| `latency.json` | `crowd_weight` | 0.5 | explained, no source | 3 |
| `latency.json` | `threat_weight` | 0.5 | explained, no source | 3 |
| `laws.json` | `reckless_speed_milli` | 600 | explained, no source | 3 |
| `laws.json` | `excessive_speed_milli` | 1150 | explained, no source | 3 |
| `laws.json` | `dogso_distance_cm` | 2500 | explained, no source | 3 |
| `laws.json` | `penalty_spot_cm` | 1100 | explained, no source | 3 |
| `laws.json` | `penalty_area_depth_cm` | 1650 | undocumented | 3 |
| `laws.json` | `penalty_area_half_width_cm` | 2016 | undocumented | 3 |
| `laws.json` | `goal_area_depth_cm` | 550 | sourced | 1 |
| `laws.json` | `goal_area_half_width_cm` | 916 | sourced | 1 |
| `laws.json` | `centre_circle_radius_cm` | 915 | sourced | 1 |
| `laws.json` | `goal_half_width_cm` | 366 | sourced | 1 |
| `movement.json` | `accel_min_cm_per_tick2` | 2.5 | undocumented | 3 |
| `movement.json` | `accel_max_cm_per_tick2` | 5.0 | undocumented | 3 |
| `movement.json` | `top_speed_min_cm_per_tick` | 75.0 | undocumented | 3 |
| `movement.json` | `top_speed_max_cm_per_tick` | 100.0 | undocumented | 3 |
| `movement.json` | `decel_min_cm_per_tick2` | 3.5 | undocumented | 3 |
| `movement.json` | `decel_max_cm_per_tick2` | 6.0 | undocumented | 3 |
| `movement.json` | `turn_share_min` | 0.6 | explained, no source | 3 |
| `movement.json` | `turn_share_max` | 1.0 | explained, no source | 3 |
| `movement.json` | `with_ball_share_min` | 0.75 | explained, no source | 3 |
| `movement.json` | `with_ball_share_max` | 0.95 | explained, no source | 3 |
| `movement.json` | `touch_steps_max` | 6.0 | explained, no source | 3 |
| `movement.json` | `step_hz` | 4.0 | explained, no source | 3 |
| `movement.json` | `reposition_ticks` | 30.0 | explained, no source | 3 |
| `movement.json` | `work_rate_floor` | 0.6 | sourced | 1 |
| `movement.json` | `weight_ref_kg` | 78 | undocumented | 3 |
| `movement.json` | `height_ref_cm` | 180 | undocumented | 3 |
| `movement.json` | `fatigue_coefficient` | 0.5 | sourced | 1 |
| `movement.json` | `fatigue_floor` | 0.5 | sourced | 1 |
| `obstacles.json` | `goal_half_width_cm` | 366 | undocumented | 3 |
| `obstacles.json` | `goal_height_cm` | 244 | undocumented | 3 |
| `obstacles.json` | `post_radius_cm` | 6 | undocumented | 3 |
| `obstacles.json` | `crossbar_thickness_cm` | 6 | undocumented | 3 |
| `obstacles.json` | `elasticity_num` | 8 | undocumented | 3 |
| `obstacles.json` | `elasticity_den` | 10 | undocumented | 3 |
| `offball.json` | `hold_salience_milli` | 300 | undocumented | 3 |
| `offball.json` | `proximity_ref_cm` | 1500 | undocumented | 3 |
| `offball.json` | `second_reader_milli` | 600 | undocumented | 3 |
| `offball.json` | `challenge_salience_milli` | 1300 | undocumented | 3 |
| `offball.json` | `keeper_box_depth_cm` | 1650 | sourced | 1 |
| `offball.json` | `keeper_box_half_width_cm` | 2016 | sourced | 1 |
| `offball.json` | `mark_radius_cm` | 300 | explained, no source | 3 |
| `offball.json` | `offer_step_cm` | 600 | explained, no source | 3 |
| `offball.json` | `onside_margin_cm` | 50 | explained, no source | 3 |
| `offball.json` | `mark_salience_milli` | 800 | undocumented | 3 |
| `offball.json` | `mark_zone_cm` | 1200 | explained, no source | 3 |
| `offball.json` | `mark_gap_cm` | 150 | explained, no source | 3 |
| `offball.json` | `cover_depth_cm` | 700 | explained, no source | 3 |
| `offball.json` | `lane_step_cm` | 300 | explained, no source | 3 |
| `offball.json` | `discipline_floor_milli` | 500 | explained, no source | 3 |
| `offball.json` | `ball_reach_base_milli` | 600 | undocumented | 3 |
| `offball.json` | `ball_reach_gain_milli` | 800 | undocumented | 3 |
| `offball.json` | `block_height_cm` | 180 | explained, no source | 3 |
| `offball.json` | `block_rebound_milli` | 300 | explained, no source | 3 |
| `profiles.json` | `left_footed_share` | 0.25 | explained, no source | 3 |
| `profiles.json` | `weak_foot_ratio` | [0.6, 0.15] | explained, no source | 3 |
| `profiles.json` | `family_weight` | 0.4 | derived | 1 |
| `profiles.json` | `roles` | … | undocumented | 3 |
| `reception.json` | `speed_base_cm_per_tick` | 150 | explained, no source | 3 |
| `reception.json` | `speed_passing_bonus_cm_per_tick` | 150 | explained, no source | 3 |
| `reception.json` | `loose_control_radius_cm` | 100 | arbitrary, declared | 3 |
| `reception.json` | `first_touch_keep_max_milli` | 150 | arbitrary, declared | 3 |
| `save.json` | `reaction_fast_ticks` | 3.0 | undocumented | 3 |
| `save.json` | `reaction_slow_ticks` | 6.0 | undocumented | 3 |
| `save.json` | `dive_speed_cm_per_tick` | 35.0 | undocumented | 3 |
| `save.json` | `dive_max_cm` | 220.0 | undocumented | 3 |
| `save.json` | `body_centre_cm` | 100.0 | explained, no source | 3 |
| `save.json` | `catch_speed_ref_cm_per_tick` | 250.0 | undocumented | 3 |
| `save.json` | `parry_rebound_milli` | 300 | arbitrary, declared | 3 |
| `save.json` | `tip_share_milli` | 750 | explained, no source | 3 |
| `save.json` | `standing_reach_share` | 0.75 | explained, no source | 3 |
| `save.json` | `reference_keeper_height_cm` | 190 | explained, no source | 3 |
| `scanning.json` | `scan_rate_min_per_s` | 0.2 | explained, no source | 3 |
| `scanning.json` | `scan_rate_max_per_s` | 0.8 | explained, no source | 3 |
| `scanning.json` | `scan_reach_cm` | 3000 | undocumented | 3 |
| `scanning.json` | `mark_radius_cm` | 800 | explained, no source | 3 |
| `scanning.json` | `weight_ball` | 100 | undocumented | 3 |
| `scanning.json` | `weight_opponent_on_ball` | 80 | undocumented | 3 |
| `scanning.json` | `weight_mark` | 60 | undocumented | 3 |
| `scanning.json` | `level_threshold_mrad` | 8 | sourced (derived) | 1 |
| `shot_choice.json` | `power_strong` | 0.85 | explained, no source | 3 |
| `shot_choice.json` | `power_placed` | 0.35 | explained, no source | 3 |
| `shot_choice.json` | `aim_edge_fraction` | 0.8 | explained, no source | 3 |
| `shot_choice.json` | `reading_weight` | 0.5 | explained, no source | 3 |
| `shot_choice.json` | `risk_weight` | 0.5 | explained, no source | 3 |
| `shot_choice.json` | `shot_range_cm` | 3500 | explained, no source | 3 |
| `shot_choice.json` | `progression_gain` | 1.5 | explained, no source | 3 |
| `shot_choice.json` | `progression_floor` | 0.4 | explained, no source | 3 |
| `shot_choice.json` | `pressure_radius_cm` | 300 | explained, no source | 3 |
| `shot_choice.json` | `kick_pressure_radius_cm` | 1500 | explained, no source | 3 |
| `shot_choice.json` | `carry_room_ref_cm` | 1000 | explained, no source | 3 |
| `shot_choice.json` | `clear_salience` | 2.5 | explained, no source | 3 |
| `shot_choice.json` | `clear_zone_cm` | 2500 | explained, no source | 3 |
| `shot_choice.json` | `loft_min_cm` | 1800 | explained, no source | 3 |
| `shot_choice.json` | `power_lob` | 0.2 | explained, no source | 3 |
| `stoppage_time.json` | `base` | 0 | undocumented | 3 |
| `stoppage_time.json` | `goal` | 60 | sourced | 1 |
| `stoppage_time.json` | `foul` | 20 | explained, no source | 3 |
| `stoppage_time.json` | `duel` | 0 | explained, no source | 3 |
| `thought.json` | `goal_width_cm` | 732 | sourced | 1 |
| `thought.json` | `possession_value` | 0.02 | sourced | 1 |
| `thought.json` | `forecast_step_cm` | 100 | sourced | 1 |
| `thought.json` | `believed_speed_cm_per_tick` | 65.0 | undocumented | 3 |
| `thought.json` | `believed_reaction_ticks` | 3.0 | undocumented | 3 |
| `thought.json` | `trust_ticks` | 10.0 | explained, no source | 3 |
| `thought.json` | `horizon_min_cm` | 1500.0 | sourced | 1 |
| `thought.json` | `horizon_max_cm` | 4500.0 | sourced | 1 |
| `profiles.json` | `roles.goalkeeper.height_cm` | [190, 5] | coperto da `roles` (undocumented); `_source`: ordini di grandezza | 3 |
| `profiles.json` | `roles.goalkeeper.weight_kg` | [85, 6] | coperto da `roles` (undocumented); `_source`: ordini di grandezza | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.explosive_force` | [0.62, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.max_speed` | [0.35, 0.1] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.injury_proneness` | [0.3, 0.15] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.reactivity` | [0.7, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.finishing` | [0.1, 0.05] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.passing` | [0.45, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.first_touch` | [0.45, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.tackling` | [0.15, 0.08] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.goalkeeping` | [0.7, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.game_reading` | [0.6, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.decision_making` | [0.6, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.risk_management` | [0.4, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.vision` | [0.45, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.anticipation` | [0.55, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.creativity` | [0.2, 0.1] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.concentration` | [0.6, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.work_rate` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.discipline` | [0.65, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.maximal_force` | [0.6, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.body_control` | [0.6, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.aggressiveness` | [0.4, 0.15] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.emotional_control` | [0.55, 0.15] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.heading` | [0.2, 0.1] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.dribbling` | [0.15, 0.08] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.endurance` | [0.4, 0.1] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.goalkeeper.characteristics.joint_mobility` | [0.6, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.height_cm` | [187, 5] | coperto da `roles` (undocumented); `_source`: ordini di grandezza | 3 |
| `profiles.json` | `roles.centre_back.weight_kg` | [82, 5] | coperto da `roles` (undocumented); `_source`: ordini di grandezza | 3 |
| `profiles.json` | `roles.centre_back.characteristics.explosive_force` | [0.6, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.max_speed` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.injury_proneness` | [0.3, 0.15] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.reactivity` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.finishing` | [0.2, 0.1] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.passing` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.first_touch` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.tackling` | [0.72, 0.1] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.goalkeeping` | [0.05, 0.03] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.game_reading` | [0.65, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.decision_making` | [0.55, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.risk_management` | [0.35, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.vision` | [0.45, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.anticipation` | [0.55, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.creativity` | [0.3, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.concentration` | [0.55, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.work_rate` | [0.55, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.discipline` | [0.65, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.maximal_force` | [0.72, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.body_control` | [0.55, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.aggressiveness` | [0.62, 0.15] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.emotional_control` | [0.55, 0.15] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.heading` | [0.68, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.dribbling` | [0.3, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.endurance` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.centre_back.characteristics.joint_mobility` | [0.45, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.height_cm` | [176, 5] | coperto da `roles` (undocumented); `_source`: ordini di grandezza | 3 |
| `profiles.json` | `roles.full_back.weight_kg` | [72, 5] | coperto da `roles` (undocumented); `_source`: ordini di grandezza | 3 |
| `profiles.json` | `roles.full_back.characteristics.explosive_force` | [0.6, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.max_speed` | [0.7, 0.1] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.injury_proneness` | [0.3, 0.15] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.reactivity` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.finishing` | [0.3, 0.1] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.passing` | [0.55, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.first_touch` | [0.55, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.tackling` | [0.62, 0.1] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.goalkeeping` | [0.05, 0.03] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.game_reading` | [0.55, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.decision_making` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.risk_management` | [0.45, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.vision` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.anticipation` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.creativity` | [0.4, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.concentration` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.work_rate` | [0.65, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.discipline` | [0.6, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.maximal_force` | [0.55, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.body_control` | [0.6, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.aggressiveness` | [0.55, 0.15] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.emotional_control` | [0.55, 0.15] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.heading` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.dribbling` | [0.5, 0.13] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.endurance` | [0.62, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.full_back.characteristics.joint_mobility` | [0.6, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.height_cm` | [180, 6] | coperto da `roles` (undocumented); `_source`: ordini di grandezza | 3 |
| `profiles.json` | `roles.central_midfielder.weight_kg` | [75, 5] | coperto da `roles` (undocumented); `_source`: ordini di grandezza | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.explosive_force` | [0.55, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.max_speed` | [0.55, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.injury_proneness` | [0.3, 0.15] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.reactivity` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.finishing` | [0.45, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.passing` | [0.72, 0.1] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.first_touch` | [0.7, 0.1] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.tackling` | [0.55, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.goalkeeping` | [0.05, 0.03] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.game_reading` | [0.7, 0.1] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.decision_making` | [0.68, 0.1] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.risk_management` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.vision` | [0.65, 0.13] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.anticipation` | [0.6, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.creativity` | [0.6, 0.14] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.concentration` | [0.55, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.work_rate` | [0.65, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.discipline` | [0.6, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.maximal_force` | [0.58, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.body_control` | [0.62, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.aggressiveness` | [0.55, 0.15] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.emotional_control` | [0.55, 0.15] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.heading` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.dribbling` | [0.55, 0.13] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.endurance` | [0.66, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.central_midfielder.characteristics.joint_mobility` | [0.6, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.height_cm` | [174, 6] | coperto da `roles` (undocumented); `_source`: ordini di grandezza | 3 |
| `profiles.json` | `roles.wide_midfielder.weight_kg` | [70, 5] | coperto da `roles` (undocumented); `_source`: ordini di grandezza | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.explosive_force` | [0.65, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.max_speed` | [0.78, 0.1] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.injury_proneness` | [0.3, 0.15] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.reactivity` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.finishing` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.passing` | [0.62, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.first_touch` | [0.68, 0.1] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.tackling` | [0.4, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.goalkeeping` | [0.05, 0.03] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.game_reading` | [0.58, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.decision_making` | [0.55, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.risk_management` | [0.6, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.vision` | [0.55, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.anticipation` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.creativity` | [0.55, 0.14] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.concentration` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.work_rate` | [0.6, 0.13] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.discipline` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.maximal_force` | [0.48, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.body_control` | [0.65, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.aggressiveness` | [0.42, 0.15] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.emotional_control` | [0.55, 0.15] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.heading` | [0.42, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.dribbling` | [0.68, 0.13] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.endurance` | [0.6, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.wide_midfielder.characteristics.joint_mobility` | [0.68, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.height_cm` | [182, 7] | coperto da `roles` (undocumented); `_source`: ordini di grandezza | 3 |
| `profiles.json` | `roles.striker.weight_kg` | [78, 6] | coperto da `roles` (undocumented); `_source`: ordini di grandezza | 3 |
| `profiles.json` | `roles.striker.characteristics.explosive_force` | [0.7, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.max_speed` | [0.7, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.injury_proneness` | [0.3, 0.15] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.reactivity` | [0.55, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.finishing` | [0.75, 0.1] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.passing` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.first_touch` | [0.68, 0.1] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.tackling` | [0.25, 0.1] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.goalkeeping` | [0.05, 0.03] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.game_reading` | [0.62, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.decision_making` | [0.6, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.risk_management` | [0.65, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.vision` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.anticipation` | [0.6, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.creativity` | [0.55, 0.14] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.concentration` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.work_rate` | [0.5, 0.13] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.discipline` | [0.45, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.maximal_force` | [0.62, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.body_control` | [0.6, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.aggressiveness` | [0.5, 0.15] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.emotional_control` | [0.55, 0.15] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.heading` | [0.66, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.dribbling` | [0.62, 0.14] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.endurance` | [0.5, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
| `profiles.json` | `roles.striker.characteristics.joint_mobility` | [0.6, 0.12] | coperto da `roles` (undocumented); prototipo del generatore | 3 |
