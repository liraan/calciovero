# Sources of every calibration

**What this is.** One row for every number in `L0-match/balance/`: its value, its status, and what the file itself says about where it comes from. The maintainer, 2026-11-14: *cite always the sources and the studies of every calibration*. It is the companion of RULES A5-bis (the statistics are derived, never adjusted): a number has a source, is computed from other numbers, or is declared arbitrary and left alone.

**Status**, read from each file's own `_note` and `_source`:
- **sourced**: the text names a source (a study, a data provider, the Laws of the Game, a measurement).
- **derived**: computed from other numbers or from a definition (no value of its own).
- **arbitrary, declared**: a choice, declared as one, waiting for a source (calibration, L3).
- **explained, no source**: the file explains what it does, without a source or a derivation.
- **undocumented**: the file says nothing about this parameter. These come first in the work to do.

**Today:** 225 parameters in 21 files: 17 sourced, 5 derived, 9 arbitrary and declared, 89 explained without a source, 105 undocumented.

**The rule from now on.** A change to a number in `balance/` updates its row here in the same commit, with the source (author, year, title, and the page or table) or the derivation, or the word *arbitrary*. A number chosen by looking at the statistic it produces is never a source. This file was generated from the JSON files on 2026-11-14 (the classification is read from their text and is a first pass: a row marked sourced still needs its full reference written here when the file gives only a name).

## The references cited so far

To be filled as each row is completed: full reference, what it measures, which parameters read it.

| Reference | What it gives | Parameters |
|---|---|---|
| IFAB, *Laws of the Game* | distances and rules of restarts, the goal and the pitch | `laws.json` |
| [to complete] | | |

## Parameter by parameter

### `ball.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `gravity_cm_per_tick2` | 10 | undocumented | — |
| `air_friction_milli` | 990 | undocumented | — |
| `ground_friction_milli` | 957 | undocumented | — |
| `stop_threshold_cm_per_tick` | 1 | undocumented | — |

### `block.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `depth_in_possession_cm` | 4500 | undocumented | — |
| `depth_out_of_possession_cm` | 3200 | undocumented | — |
| `width_in_possession_cm` | 6000 | undocumented | — |
| `width_out_of_possession_cm` | 4000 | undocumented | — |
| `line_behind_ball_in_possession_cm` | 3000 | undocumented | — |
| `line_behind_ball_out_of_possession_cm` | 1500 | undocumented | — |
| `line_min_cm` | 1000 | undocumented | — |
| `line_max_in_possession_cm` | 5800 | undocumented | — |
| `line_max_out_of_possession_cm` | 4500 | undocumented | — |
| `ball_side_shift_milli` | 600 | undocumented | — |
| `high_block_line_cm` | 1200 | explained, no source | A high block pushes the line high_block_line_cm up and closer to the ball out of possession (the line behind the ball shrinks by half as much); a low block drops it low_block_line_cm and sits further behind the ball; a wide team stretches its width with the ball by width_change_milli, a narrow one shrinks it. |
| `low_block_line_cm` | 1500 | explained, no source | A high block pushes the line high_block_line_cm up and closer to the ball out of possession (the line behind the ball shrinks by half as much); a low block drops it low_block_line_cm and sits further behind the ball; a wide team stretches its width with the ball by width_change_milli, a narrow one shrinks it. |
| `width_change_milli` | 200 | explained, no source | A high block pushes the line high_block_line_cm up and closer to the ball out of possession (the line behind the ball shrinks by half as much); a low block drops it low_block_line_cm and sits further behind the ball; a wide team stretches its width with the ball by width_change_milli, a narrow one shrinks it. |

### `carry.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `carry_distance_cm` | 1000 | undocumented | — |
| `carry_stop_from_goal_cm` | 1650 | sourced | To be calibrated at L3. carry_stop_from_goal_cm 1650: the edge of the penalty area (Law 1, 16.5 m); a carrier stops there and chooses, he does not dribble onto the goal line. |

### `choice.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `reading_weight` | 0.5 | undocumented | — |
| `risk_weight` | 0.5 | undocumented | — |
| `decision_bias` | 0.2 | arbitrary, declared | Arbitrary and declared until calibration (D6). game_reading and risk_management enter the formula (`ds-calculator/src/choice.rs`); decision_making does not (it is a threshold on the *choice*, ADR-039 read/choose split): in `resolve_choice` a poorly deciding actor accepts the second-best candidate if `second.score * (1 + (1 - decision_making) * decision_bias) > best.score`. A strong decider (decision_making = 1) falls |
| `floor` | 0.5 | undocumented | — |
| `clarity_threshold` | 0.25 | undocumented | — |
| `perception_floor` | 0.1 | undocumented | — |
| `perception_range` | 0.7 | undocumented | — |

### `duel.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `reach_cm` | 120 | explained, no source | An opponent within reach_cm of the ball challenges the carrier, at most once every retry_ticks (his own pace); the contact is deterministic: the defender's force is a weighted sum of his tackling and explosive force, the carrier's of his first touch (keeping the ball, a proxy until dribbling exists) and explosive force; the stronger contact wins, ties to the carrier. In the air (2026-11-13, plan L0-CLOSE step 4): a b |
| `retry_ticks` | 50 | explained, no source | An opponent within reach_cm of the ball challenges the carrier, at most once every retry_ticks (his own pace); the contact is deterministic: the defender's force is a weighted sum of his tackling and explosive force, the carrier's of his first touch (keeping the ball, a proxy until dribbling exists) and explosive force; the stronger contact wins, ties to the carrier. |
| `defender_tackling_milli` | 450 | undocumented | — |
| `defender_explosive_milli` | 200 | undocumented | — |
| `defender_force_milli` | 200 | undocumented | — |
| `defender_body_milli` | 100 | undocumented | — |
| `carrier_first_touch_milli` | 450 | undocumented | — |
| `carrier_explosive_milli` | 200 | undocumented | — |
| `carrier_force_milli` | 200 | undocumented | — |
| `carrier_body_milli` | 200 | undocumented | — |
| `loose_margin_milli` | 40 | undocumented | — |
| `loose_speed_cm_per_tick` | 50 | undocumented | — |
| `slide_reach_milli` | 1400 | explained, no source | Standing (standing_salience x (0.5 + 0.5 x tackling)), slide (slide_salience x aggressiveness; it reaches slide_reach_milli of the standing reach and adds slide_bonus to his force, but if he does not win the ball he is on the ground for slide_recovery_ticks), or jockey (jockey_salience x (1 - aggressiveness) x (0.5 + 0.5 x game_reading): he stays in front and waits, no contact). |
| `slide_bonus_milli` | 80 | undocumented | — |
| `slide_recovery_ticks` | 60 | explained, no source | Standing (standing_salience x (0.5 + 0.5 x tackling)), slide (slide_salience x aggressiveness; it reaches slide_reach_milli of the standing reach and adds slide_bonus to his force, but if he does not win the ball he is on the ground for slide_recovery_ticks), or jockey (jockey_salience x (1 - aggressiveness) x (0.5 + 0.5 x game_reading): he stays in front and waits, no contact). |
| `shield_force_milli` | 150 | undocumented | — |
| `shield_body_milli` | 100 | undocumented | — |
| `dribble_dribbling_milli` | 500 | undocumented | — |
| `dribble_explosive_milli` | 250 | undocumented | — |
| `dribble_body_milli` | 200 | undocumented | — |
| `beaten_ticks` | 80 | explained, no source | A man beaten by a dribble is out of the play for beaten_ticks. |
| `answer_trust_milli` | 250 | explained, no source | The carrier answers (2026-11-13, plan L0-CLOSE step 5, ADR-042 step c): when a defender goes in, the carrier shields or takes him on, by what he imagines (ADR-052): how sure he is to keep the ball (his own force for that answer against an average defender, answer_trust_milli the margin that makes him sure), what the space he keeps is worth, what the space past the man is worth, and what the opponents would have if he |
| `standing_salience_milli` | 600 | undocumented | — |
| `slide_salience_milli` | 900 | undocumented | — |
| `jockey_salience_milli` | 500 | undocumented | — |
| `foul_threshold_milli` | 800 | undocumented | — |
| `foul_slide_milli` | 300 | undocumented | — |
| `foul_behind_milli` | 350 | undocumented | — |
| `foul_temper_milli` | 500 | undocumented | — |
| `foul_speed_milli` | 200 | undocumented | — |
| `foul_technique_milli` | 200 | undocumented | — |
| `speed_ref_cm_per_tick` | 90 | undocumented | — |
| `area_slide_caution_milli` | 250 | explained, no source | In his own penalty area a defender is careful: the slide's salience is multiplied by area_slide_caution_milli (a foul there is a penalty). |
| `booked_slide_caution_milli` | 150 | explained, no source | A player already cautioned is careful too: the slide's salience is multiplied by booked_slide_caution_milli (another yellow is a red). |
| `jump_ref_cm` | 55 | explained, no source | In the air (2026-11-13, plan L0-CLOSE step 4): a ball in the air is met by the players it passes within reach of, horizontally (their reach for a passing ball) and in height (their height plus their jump, explosive_force x jump_ref_cm; a keeper in his area adds his arms, hands_reach_cm). |
| `hands_reach_cm` | 70 | explained, no source | In the air (2026-11-13, plan L0-CLOSE step 4): a ball in the air is met by the players it passes within reach of, horizontally (their reach for a passing ball) and in height (their height plus their jump, explosive_force x jump_ref_cm; a keeper in his area adds his arms, hands_reach_cm). |
| `header_speed_cm_per_tick` | 130 | undocumented | — |
| `aerial_heading_milli` | 400 | undocumented | — |
| `aerial_jump_milli` | 300 | undocumented | — |
| `aerial_force_milli` | 150 | undocumented | — |
| `aerial_height_milli` | 150 | undocumented | — |
| `keeper_hands_bonus_milli` | 300 | undocumented | — |

### `fatigue.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `effort_ref_cm_per_tick` | 90.0 | undocumented | — |
| `rest_effort` | 0.02 | explained, no source | Each tick a player's fatigue (0 to 1) changes by gain_per_tick x ((1.5 - endurance) x effort - (0.5 + endurance) x rest_effort), where effort = (his speed / effort_ref)^2: a sprint costs about nine times a jog, and a slow tick lets him recover, the more the higher his endurance. Whatever the engine's players then run, more or less than real ones, shows in their fatigue: it is a finding, not a number to tune. match_pr |
| `half_time_recovery` | 0.3 | sourced | At half-time he recovers half_time_recovery of it. Whatever the engine's players then run, more or less than real ones, shows in their fatigue: it is a finding, not a number to tune. match_profile: a professional match's activity, about 60% of the time walking or standing (1.5 m/s), 25% jogging (3.5 m/s), 13% running (5 m/s), 2% sprinting (7.5 m/s), about 10-11 km in all (Bangsbo, Mohr and Krustrup 2006, J Sports Sci |
| `match_profile` | … | derived | Derived, not adjusted (2026-11-13, the maintainer: the statistics are derived from the players, never fixed): gain_per_tick and v_max_coefficient are not in this file; they are computed (FatigueBalance::load) so that an average player who runs a real match (match_profile) ends it with the fatigue that dims his high-intensity running by late_high_intensity_decline and his top speed by late_top_speed_decline. Whatever  |
| `match_ticks` | 54000.0 | undocumented | — |
| `late_high_intensity_decline` | 0.3 | derived | Derived, not adjusted (2026-11-13, the maintainer: the statistics are derived from the players, never fixed): gain_per_tick and v_max_coefficient are not in this file; they are computed (FatigueBalance::load) so that an average player who runs a real match (match_profile) ends it with the fatigue that dims his high-intensity running by late_high_intensity_decline and his top speed by late_top_speed_decline. Whatever  |
| `late_top_speed_decline` | 0.04 | derived | Derived, not adjusted (2026-11-13, the maintainer: the statistics are derived from the players, never fixed): gain_per_tick and v_max_coefficient are not in this file; they are computed (FatigueBalance::load) so that an average player who runs a real match (match_profile) ends it with the fatigue that dims his high-intensity running by late_high_intensity_decline and his top speed by late_top_speed_decline. Whatever  |

### `influence.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `w_space` | 100 | undocumented | — |
| `w_skill` | 40 | undocumented | — |
| `w_cost` | 20 | undocumented | — |
| `w_red` | 50 | undocumented | — |
| `space_threshold_cm` | 2000 | explained, no source | Before, the radius was space_threshold_cm (20 m) and there was no hysteresis: a match had 2,000-9,000 Pressure events (a real one has a few hundred), an opponent oscillating on the radius 'entered' again every tick. |
| `pressure_radius_cm` | 500 | explained, no source | See L0-match/specs/03-entities-minimal.md §2.9. pressure_radius_cm and pressure_exit_milli (2026-11-12): an opponent puts the carrier under pressure when he comes within 5 m of the ball, and stays so until he is 30% farther (hysteresis). |
| `pressure_exit_milli` | 1300 | explained, no source | See L0-match/specs/03-entities-minimal.md §2.9. pressure_radius_cm and pressure_exit_milli (2026-11-12): an opponent puts the carrier under pressure when he comes within 5 m of the ball, and stays so until he is 30% farther (hysteresis). |

### `keeper.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `positioning_range_cm` | 3000 | undocumented | — |
| `advance_cm` | 400 | undocumented | — |
| `support_gap_cm` | 2500 | explained, no source | Support (2026-11-12, maintainer: the keeper steps up and is an outlet when his side has the ball): with the ball, the keeper follows his defensive line at support_gap_cm behind it, never farther than support_max_cm from his goal line; he is then a teammate the carrier can pass back to. No cited figure for the two values. support_gap_cm 2500 and support_max_cm 2200: order of magnitude of a modern keeper's position whe |
| `support_max_cm` | 2200 | explained, no source | Support (2026-11-12, maintainer: the keeper steps up and is an outlet when his side has the ball): with the ball, the keeper follows his defensive line at support_gap_cm behind it, never farther than support_max_cm from his goal line; he is then a teammate the carrier can pass back to. No cited figure for the two values. support_gap_cm 2500 and support_max_cm 2200: order of magnitude of a modern keeper's position whe |
| `depth_share_milli` | 500 | explained, no source | Without the ball the rule above applies. depth_share_milli (2026-11-12, ADR-051): without the ball the keeper is never deeper than this share of the ball's distance from his goal line, so he is always between the ball and the goal; before, with the ball close to the line at a side he advanced 4 m and stood behind the shooter, who scored from beside the post. |

### `kick.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `speed_ref_cm_per_tick` | 340.0 | undocumented | — |
| `speed_power_base` | 0.5 | undocumented | — |
| `speed_explosive_base` | 0.6 | undocumented | — |
| `error_ref_rad` | 0.062 | arbitrary, declared | 2026-11-13 (plan L0-CLOSE step 8): the pass has its own reference error, pass_error_ref_rad (a pass is aimed at a man who comes to meet it), today equal to the shot's. Arbitrary, declared, to be calibrated at L3 against real shot speed, placement and on-target distributions. throw_speed (18 m/s) and throw_max (35 m): order of magnitude of a keeper's overarm throw, declared, to be sourced at L3. run_error_rad (about 9 |
| `pass_error_ref_rad` | 0.08 | derived | 2026-11-13 (plan L0-CLOSE step 8): the pass has its own reference error, pass_error_ref_rad (a pass is aimed at a man who comes to meet it), today equal to the shot's. Before, 0.08, arbitrary. pass_error_ref_rad stays declared at 0.08 (the source is for the instep shot). vertical_share (2026-11-13, SIM_VERSION 91): derived from the same source, whose ellipse spreads about as much up and down as across (its area is re |
| `throw_speed_cm_per_tick` | 180.0 | explained, no source | The keeper's throw (2026-11-13, plan L0-CLOSE 10d): from his hands, to a teammate within throw_max_cm, at most throw_speed_cm_per_tick, its precision his hands' (goalkeeping) with both hands; farther he kicks it. |
| `throw_max_cm` | 3500 | explained, no source | The keeper's throw (2026-11-13, plan L0-CLOSE 10d): from his hands, to a teammate within throw_max_cm, at most throw_speed_cm_per_tick, its precision his hands' (goalkeeping) with both hands; farther he kicks it. |
| `error_power_gain` | 1.0 | undocumented | — |
| `error_pressure_rad` | 0.08 | undocumented | — |
| `run_error_rad` | 0.15 | arbitrary, declared | On the run (2026-11-13, plan L0-CLOSE 11): a shot struck running adds run_error_rad x (his speed / run_speed_ref, at most 1) x (1 - body_control) to the error: the contact is less clean the faster he runs, the less so the better his balance. Arbitrary, declared, to be calibrated at L3 against real shot speed, placement and on-target distributions. throw_speed (18 m/s) and throw_max (35 m): order of magnitude of a kee |
| `run_speed_ref_cm_per_tick` | 80.0 | undocumented | — |
| `vertical_share` | 2.0 | explained, no source | Precision is an angle: error = error_ref x (1 - skill) x (0.5 + power_gain x power) + error_pressure x pressure, in radians, with skill = shooting x (0.5 + 0.5 x the quality of the foot he kicks with); the miss is the distance times the angle, so the same player misses by more from farther away, and a strong shot is less precise than a placed one. vertical_share is the share of the error that goes up (a miscued strik |
| `aim_low_cm` | 40 | explained, no source | Precision is an angle: error = error_ref x (1 - skill) x (0.5 + power_gain x power) + error_pressure x pressure, in radians, with skill = shooting x (0.5 + 0.5 x the quality of the foot he kicks with); the miss is the distance times the angle, so the same player misses by more from farther away, and a strong shot is less precise than a placed one. vertical_share is the share of the error that goes up (a miscued strik |
| `aim_high_cm` | 200 | explained, no source | Precision is an angle: error = error_ref x (1 - skill) x (0.5 + power_gain x power) + error_pressure x pressure, in radians, with skill = shooting x (0.5 + 0.5 x the quality of the foot he kicks with); the miss is the distance times the angle, so the same player misses by more from farther away, and a strong shot is less precise than a placed one. vertical_share is the share of the error that goes up (a miscued strik |
| `pass_arrive_cm_per_tick` | 60.0 | explained, no source | The pass is weighed (2026-11-13): the passer gives it the speed that brings it to the receiver still rolling at pass_arrive_cm_per_tick (6 m/s), knowing the ground friction (ball.json): v0 = arrive + distance x (1 - friction), capped by his strongest kick; the error angle (from passing and the foot) makes it drift across the line by distance x angle and overhit by the same share. |
| `clear_length_cm` | 4000 | explained, no source | A clearance (2026-11-13): his strongest kick, towards a point clear_length_cm up the pitch on the wing of his side, lifted (the vertical speed is clear_loft of the horizontal one). |
| `clear_loft` | 0.35 | explained, no source | A clearance (2026-11-13): his strongest kick, towards a point clear_length_cm up the pitch on the wing of his side, lifted (the vertical speed is clear_loft of the horizontal one). |
| `loft_ticks_per_10m` | 6.0 | explained, no source | A lofted pass (2026-11-13) flies loft_ticks_per_10m ticks for every ten metres (0.6 s: a 30 m ball is in the air about 1.8 s) and is lifted to stay up that long; its error widens the distance as for a ground pass. |

### `latency.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `base_ticks` | 7 | sourced | ADR-044 §6, R.1-relation). latency = base_ticks * (1 + (1 - decision_making) * decision_bias) * (1 + crowd_weight * crowding + threat_weight * nearest_threat). Rescaled 2026-11-12 (OI-97): base_ticks was 1334, derived to reproduce the prototype's mean wait between actions (1/0.0005 = 2000 ticks, the density of the old die), not the time a footballer takes to choose; measured latencies were 142-394 s and the match sto |
| `decision_bias` | 1.0 | explained, no source | ADR-044 §6, R.1-relation). latency = base_ticks * (1 + (1 - decision_making) * decision_bias) * (1 + crowd_weight * crowding + threat_weight * nearest_threat). A third term, plausible_options, is tracked as OI-96. decision_bias 0.0 means decision_making does not change the pace; 1.0 means a weak decider (0.0) waits twice as long as a strong one (1.0). |
| `crowd_weight` | 0.5 | explained, no source | ADR-044 §6, R.1-relation). latency = base_ticks * (1 + (1 - decision_making) * decision_bias) * (1 + crowd_weight * crowding + threat_weight * nearest_threat). With base_ticks 7 a strong decider in space chooses in about 0.7 s, a weak decider under pressure in about 2-3 s, which is the order of magnitude of a real match. crowd_weight and threat_weight are the weights of the two scene terms. |
| `threat_weight` | 0.5 | explained, no source | ADR-044 §6, R.1-relation). latency = base_ticks * (1 + (1 - decision_making) * decision_bias) * (1 + crowd_weight * crowding + threat_weight * nearest_threat). With base_ticks 7 a strong decider in space chooses in about 0.7 s, a weak decider under pressure in about 2-3 s, which is the order of magnitude of a real match. crowd_weight and threat_weight are the weights of the two scene terms. |

### `laws.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `reckless_speed_milli` | 600 | explained, no source | A foul is the fact of a contact (the duel's band); its sanction is read from the facts: a slide from behind at an approach speed of at least reckless_speed_milli of the reference speed is reckless (yellow card); a challenge on his feet is careless (no card); a slide from behind at excessive_speed_milli or more is excessive force (red card); a foul that denies an obvious goal-scoring opportunity (the carrier within do |
| `excessive_speed_milli` | 1150 | explained, no source | A foul is the fact of a contact (the duel's band); its sanction is read from the facts: a slide from behind at an approach speed of at least reckless_speed_milli of the reference speed is reckless (yellow card); a challenge on his feet is careless (no card); a slide from behind at excessive_speed_milli or more is excessive force (red card); a foul that denies an obvious goal-scoring opportunity (the carrier within do |
| `dogso_distance_cm` | 2500 | explained, no source | A foul is the fact of a contact (the duel's band); its sanction is read from the facts: a slide from behind at an approach speed of at least reckless_speed_milli of the reference speed is reckless (yellow card); a challenge on his feet is careless (no card); a slide from behind at excessive_speed_milli or more is excessive force (red card); a foul that denies an obvious goal-scoring opportunity (the carrier within do |
| `penalty_spot_cm` | 1100 | explained, no source | Inside the penalty area the free kick is a penalty, taken from penalty_spot_cm off the goal line. |
| `penalty_area_depth_cm` | 1650 | undocumented | — |
| `penalty_area_half_width_cm` | 2016 | undocumented | — |
| `goal_area_depth_cm` | 550 | sourced | The speed thresholds and dogso_distance_cm are arbitrary, declared, to be calibrated at L3 against real foul and card counts (about 20 fouls, 4 yellow cards and 0.1-0.2 red cards a match). goal_area_depth_cm, goal_area_half_width_cm, centre_circle_radius_cm, goal_half_width_cm (2026-11-13, ADR-055, read by the GUI to draw the pitch): Law 1, the goal area 5.5 m deep and 18.32 m wide, the centre circle 9.15 m, the goal |
| `goal_area_half_width_cm` | 916 | sourced | The speed thresholds and dogso_distance_cm are arbitrary, declared, to be calibrated at L3 against real foul and card counts (about 20 fouls, 4 yellow cards and 0.1-0.2 red cards a match). goal_area_depth_cm, goal_area_half_width_cm, centre_circle_radius_cm, goal_half_width_cm (2026-11-13, ADR-055, read by the GUI to draw the pitch): Law 1, the goal area 5.5 m deep and 18.32 m wide, the centre circle 9.15 m, the goal |
| `centre_circle_radius_cm` | 915 | sourced | The speed thresholds and dogso_distance_cm are arbitrary, declared, to be calibrated at L3 against real foul and card counts (about 20 fouls, 4 yellow cards and 0.1-0.2 red cards a match). goal_area_depth_cm, goal_area_half_width_cm, centre_circle_radius_cm, goal_half_width_cm (2026-11-13, ADR-055, read by the GUI to draw the pitch): Law 1, the goal area 5.5 m deep and 18.32 m wide, the centre circle 9.15 m, the goal |
| `goal_half_width_cm` | 366 | sourced | The speed thresholds and dogso_distance_cm are arbitrary, declared, to be calibrated at L3 against real foul and card counts (about 20 fouls, 4 yellow cards and 0.1-0.2 red cards a match). goal_area_depth_cm, goal_area_half_width_cm, centre_circle_radius_cm, goal_half_width_cm (2026-11-13, ADR-055, read by the GUI to draw the pitch): Law 1, the goal area 5.5 m deep and 18.32 m wide, the centre circle 9.15 m, the goal |

### `movement.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `accel_min_cm_per_tick2` | 2.5 | undocumented | — |
| `accel_max_cm_per_tick2` | 5.0 | undocumented | — |
| `top_speed_min_cm_per_tick` | 75.0 | undocumented | — |
| `top_speed_max_cm_per_tick` | 100.0 | undocumented | — |
| `decel_min_cm_per_tick2` | 3.5 | undocumented | — |
| `decel_max_cm_per_tick2` | 6.0 | undocumented | — |
| `turn_share_min` | 0.6 | explained, no source | Turning (2026-11-13, plan L0-CLOSE 10e): changing direction by more than 60 degrees he brakes and turns with turn_share_min + (turn_share_max - turn_share_min) x joint_mobility of his braking: the stiff player needs longer to turn. Professional footballers' peak sprint speeds of about 7.5-10 m/s (27-36 km/h, top-league tracking data); maximal accelerations of about 2.5-5 m/s2 and decelerations larger than acceleratio |
| `turn_share_max` | 1.0 | explained, no source | Turning (2026-11-13, plan L0-CLOSE 10e): changing direction by more than 60 degrees he brakes and turns with turn_share_min + (turn_share_max - turn_share_min) x joint_mobility of his braking: the stiff player needs longer to turn. |
| `with_ball_share_min` | 0.75 | explained, no source | With the ball (2026-11-13, plan L0-CLOSE 10a): his top speed is with_ball_share_min + (with_ball_share_max - with_ball_share_min) x dribbling of his sprint: the good dribbler runs almost as fast with it. Professional footballers' peak sprint speeds of about 7.5-10 m/s (27-36 km/h, top-league tracking data); maximal accelerations of about 2.5-5 m/s2 and decelerations larger than accelerations, up to 5-6 m/s2 and more  |
| `with_ball_share_max` | 0.95 | explained, no source | With the ball (2026-11-13, plan L0-CLOSE 10a): his top speed is with_ball_share_min + (with_ball_share_max - with_ball_share_min) x dribbling of his sprint: the good dribbler runs almost as fast with it. |
| `touch_steps_max` | 6.0 | explained, no source | He touches it every 1 + (touch_steps_max - 1) x (his speed / top_speed_max) x (1 - dribbling) steps, at step_hz steps a second: walking, or the best dribbler, every step; the worst dribbler at full speed every touch_steps_max. Professional footballers' peak sprint speeds of about 7.5-10 m/s (27-36 km/h, top-league tracking data); maximal accelerations of about 2.5-5 m/s2 and decelerations larger than accelerations, u |
| `step_hz` | 4.0 | explained, no source | He touches it every 1 + (touch_steps_max - 1) x (his speed / top_speed_max) x (1 - dribbling) steps, at step_hz steps a second: walking, or the best dribbler, every step; the worst dribbler at full speed every touch_steps_max. Professional footballers' peak sprint speeds of about 7.5-10 m/s (27-36 km/h, top-league tracking data); maximal accelerations of about 2.5-5 m/s2 and decelerations larger than accelerations, u |
| `reposition_ticks` | 30.0 | explained, no source | His pace is his: when he contests something (pressing, going in, going for the ball, running in behind, carrying it) he runs as fast as he can; when he takes up his place he goes at the speed that gets him there in about reposition_ticks. Professional footballers' peak sprint speeds of about 7.5-10 m/s (27-36 km/h, top-league tracking data); maximal accelerations of about 2.5-5 m/s2 and decelerations larger than acce |
| `work_rate_floor` | 0.6 | sourced | Work rate (2026-11-13, plan L0-CLOSE 10c): without the ball, out of a contest, he gives work_rate_floor + (1 - work_rate_floor) x work_rate of the pace his side needs. Professional footballers' peak sprint speeds of about 7.5-10 m/s (27-36 km/h, top-league tracking data); maximal accelerations of about 2.5-5 m/s2 and decelerations larger than accelerations, up to 5-6 m/s2 and more (Harper, Carling and Kiely 2019, Spo |
| `weight_ref_kg` | 78 | undocumented | — |
| `height_ref_cm` | 180 | undocumented | — |
| `fatigue_coefficient` | 0.5 | sourced | Acceleration: (accel_min + (accel_max - accel_min) x explosive_force) x weight_ref / weight, dimmed by fatigue (ADR-030: 1 - fatigue_coefficient x fatigue, floored). Professional footballers' peak sprint speeds of about 7.5-10 m/s (27-36 km/h, top-league tracking data); maximal accelerations of about 2.5-5 m/s2 and decelerations larger than accelerations, up to 5-6 m/s2 and more (Harper, Carling and Kiely 2019, Sport |
| `fatigue_floor` | 0.5 | sourced | Professional footballers' peak sprint speeds of about 7.5-10 m/s (27-36 km/h, top-league tracking data); maximal accelerations of about 2.5-5 m/s2 and decelerations larger than accelerations, up to 5-6 m/s2 and more (Harper, Carling and Kiely 2019, Sports Medicine 49:1923-1947). weight_ref and height_ref: median professional (78 kg, 180 cm). reposition_ticks (3 s), fatigue_coefficient and fatigue_floor: arbitrary, de |

### `obstacles.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `goal_half_width_cm` | 366 | undocumented | — |
| `goal_height_cm` | 244 | undocumented | — |
| `post_radius_cm` | 6 | undocumented | — |
| `crossbar_thickness_cm` | 6 | undocumented | — |
| `elasticity_num` | 8 | undocumented | — |
| `elasticity_den` | 10 | undocumented | — |

### `offball.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `hold_salience_milli` | 300 | undocumented | — |
| `proximity_ref_cm` | 1500 | undocumented | — |
| `second_reader_milli` | 600 | undocumented | — |
| `challenge_salience_milli` | 1300 | undocumented | — |
| `keeper_box_depth_cm` | 1650 | sourced | Arbitrary, declared, to be calibrated at L3, except keeper_box_depth_cm (16.5 m) and keeper_box_half_width_cm (20.16 m), Law 1. cover_depth_cm (7 m) and lane_step_cm (3 m): arbitrary, declared, coaching order of magnitude, to be sourced at L3. |
| `keeper_box_half_width_cm` | 2016 | sourced | Arbitrary, declared, to be calibrated at L3, except keeper_box_depth_cm (16.5 m) and keeper_box_half_width_cm (20.16 m), Law 1. cover_depth_cm (7 m) and lane_step_cm (3 m): arbitrary, declared, coaching order of magnitude, to be sourced at L3. |
| `mark_radius_cm` | 300 | explained, no source | With a teammate on the ball (2026-11-13, ADR-051, the family "without the ball"): offer for a pass = offer_salience x proximity to the carrier x (0.5 + 0.5 x game_reading), only when an opponent is within mark_radius_cm of him; he moves offer_step_cm away from that opponent. |
| `offer_step_cm` | 600 | explained, no source | With a teammate on the ball (2026-11-13, ADR-051, the family "without the ball"): offer for a pass = offer_salience x proximity to the carrier x (0.5 + 0.5 x game_reading), only when an opponent is within mark_radius_cm of him; he moves offer_step_cm away from that opponent. The offer and run weights are removed; offer_step_cm and onside_margin_cm remain (where he goes). |
| `onside_margin_cm` | 50 | explained, no source | Run in behind = run_salience x proximity to the last defender's line (run_reach_cm halves it) x the room behind the line (room / run_room_ref_cm, at most 1) x max_speed x (0.5 + 0.5 x game_reading), only when he is ahead of the ball: he runs to the last defender's line, onside_margin_cm short of it. The offer and run weights are removed; offer_step_cm and onside_margin_cm remain (where he goes). |
| `mark_salience_milli` | 800 | undocumented | — |
| `mark_zone_cm` | 1200 | explained, no source | Marking (2026-11-13, ADR-051): when the opponents have the ball, an outfield player sees the nearest opponent other than the carrier and how many teammates are closer to him; mark = mark_salience x 1 / (1 + d / mark_zone_cm) x the rank factor x (0.5 + 0.5 x game_reading): the nearest defender who reads the game marks him, standing mark_gap_cm goal-side of him (between him and the goal). |
| `mark_gap_cm` | 150 | explained, no source | Marking (2026-11-13, ADR-051): when the opponents have the ball, an outfield player sees the nearest opponent other than the carrier and how many teammates are closer to him; mark = mark_salience x 1 / (1 + d / mark_zone_cm) x the rank factor x (0.5 + 0.5 x game_reading): the nearest defender who reads the game marks him, standing mark_gap_cm goal-side of him (between him and the goal). |
| `cover_depth_cm` | 700 | explained, no source | Cover and intercept (2026-11-13, plan L0-CLOSE 10c): with a teammate pressing the carrier, a defender may cover the space cover_depth_cm behind him on the carrier's line to our goal, worth half its danger (if the presser is beaten: he cannot know, half), if he gets there in time; or step into the passing lane lane_step_cm in front of his man, towards the carrier, worth half the man's danger (it cuts out the ground pa |
| `lane_step_cm` | 300 | explained, no source | Cover and intercept (2026-11-13, plan L0-CLOSE 10c): with a teammate pressing the carrier, a defender may cover the space cover_depth_cm behind him on the carrier's line to our goal, worth half its danger (if the presser is beaten: he cannot know, half), if he gets there in time; or step into the passing lane lane_step_cm in front of his man, towards the carrier, worth half the man's danger (it cuts out the ground pa |
| `discipline_floor_milli` | 500 | explained, no source | Discipline (plan L0-CLOSE 10c): the place he holds in the team's shape weighs discipline_floor_milli of its danger for the least disciplined player, all of it for the most. |
| `ball_reach_base_milli` | 600 | undocumented | — |
| `ball_reach_gain_milli` | 800 | undocumented | — |
| `block_height_cm` | 180 | explained, no source | Block (2026-11-13, ADR-051): a shot that passes within an outfield defender's reach for a passing ball (his reactivity) and below block_height_cm is blocked: it comes back at block_rebound_milli of its speed, loose; out over the goal line it is a corner. 2026-11-13 (ADR-052, plan L0-CLOSE step 2): offering and attacking the depth are no longer weighted: the attacker without the ball occupies the space that gives his  |
| `block_rebound_milli` | 300 | explained, no source | Block (2026-11-13, ADR-051): a shot that passes within an outfield defender's reach for a passing ball (his reactivity) and below block_height_cm is blocked: it comes back at block_rebound_milli of its speed, loose; out over the goal line it is a corner. 2026-11-13 (ADR-052, plan L0-CLOSE step 2): offering and attacking the depth are no longer weighted: the attacker without the ball occupies the space that gives his  |

### `profiles.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `left_footed_share` | 0.25 | explained, no source | The strong foot follows the passing profile; the weak foot is a fraction of it (weak_foot_ratio); left_footed_share of the players are left-footed. |
| `weak_foot_ratio` | [0.6, 0.15] | explained, no source | The strong foot follows the passing profile; the weak foot is a fraction of it (weak_foot_ratio); left_footed_share of the players are left-footed. |
| `family_weight` | 0.4 | derived | A characteristic is derived by key, so the same world gives the same player: value = mean + spread * z, clamped to [0.02, 0.98], with z in [-1, 1] mixing the player's own key (1 - family_weight) and a key shared by his family of characteristics (family_weight), so that what goes together goes together (an explosive player is usually fast). |
| `roles` | … | undocumented | — |

### `reception.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `speed_base_cm_per_tick` | 150 | explained, no source | Since SIM_VERSION 52 (ADR-051) speed_base_cm_per_tick and speed_passing_bonus_cm_per_tick are no longer read: the pass is weighed by the kick (kick.json). |
| `speed_passing_bonus_cm_per_tick` | 150 | explained, no source | Since SIM_VERSION 52 (ADR-051) speed_base_cm_per_tick and speed_passing_bonus_cm_per_tick are no longer read: the pass is weighed by the kick (kick.json). |
| `freedom_ref_cm` | 800 | arbitrary, declared | Speed base 150 and passing bonus 150 reproduce the old prototype's flight range (~150-300 cm/tick). freedom_ref_cm matches PASS_FREEDOM_CM in state.rs. distance_ref_cm is a typical pass length (~30 m). control_threshold_milli 400/1000: a player with first_touch >= 0.4 controls in neutral situations; higher first_touch or more freedom gives margin; a very long or contested reception fails. loose_control_radius_cm 100  |
| `freedom_bonus_max_milli` | 250 | undocumented | — |
| `distance_ref_cm` | 3000 | arbitrary, declared | Speed base 150 and passing bonus 150 reproduce the old prototype's flight range (~150-300 cm/tick). freedom_ref_cm matches PASS_FREEDOM_CM in state.rs. distance_ref_cm is a typical pass length (~30 m). control_threshold_milli 400/1000: a player with first_touch >= 0.4 controls in neutral situations; higher first_touch or more freedom gives margin; a very long or contested reception fails. loose_control_radius_cm 100  |
| `distance_penalty_max_milli` | 300 | undocumented | — |
| `control_threshold_milli` | 400 | arbitrary, declared | Speed base 150 and passing bonus 150 reproduce the old prototype's flight range (~150-300 cm/tick). freedom_ref_cm matches PASS_FREEDOM_CM in state.rs. distance_ref_cm is a typical pass length (~30 m). control_threshold_milli 400/1000: a player with first_touch >= 0.4 controls in neutral situations; higher first_touch or more freedom gives margin; a very long or contested reception fails. loose_control_radius_cm 100  |
| `loose_control_radius_cm` | 100 | arbitrary, declared | Speed base 150 and passing bonus 150 reproduce the old prototype's flight range (~150-300 cm/tick). freedom_ref_cm matches PASS_FREEDOM_CM in state.rs. distance_ref_cm is a typical pass length (~30 m). control_threshold_milli 400/1000: a player with first_touch >= 0.4 controls in neutral situations; higher first_touch or more freedom gives margin; a very long or contested reception fails. loose_control_radius_cm 100  |
| `first_touch_keep_max_milli` | 150 | arbitrary, declared | Speed base 150 and passing bonus 150 reproduce the old prototype's flight range (~150-300 cm/tick). freedom_ref_cm matches PASS_FREEDOM_CM in state.rs. distance_ref_cm is a typical pass length (~30 m). control_threshold_milli 400/1000: a player with first_touch >= 0.4 controls in neutral situations; higher first_touch or more freedom gives margin; a very long or contested reception fails. loose_control_radius_cm 100  |

### `save.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `reaction_fast_ticks` | 3.0 | undocumented | — |
| `reaction_slow_ticks` | 6.0 | undocumented | — |
| `dive_speed_cm_per_tick` | 35.0 | undocumented | — |
| `dive_max_cm` | 220.0 | undocumented | — |
| `body_centre_cm` | 100.0 | explained, no source | His reach is his standing reach (half his height) plus the dive, measured from the centre of his body (body_centre_cm above the ground) to the point where the ball crosses the plane. |
| `catch_speed_ref_cm_per_tick` | 250.0 | undocumented | — |
| `parry_rebound_milli` | 300 | arbitrary, declared | If the ball is within reach he stops it; he holds it if his hands (goalkeeping, the prototype of handling, OI-92) are at least the difficulty (offset / reach) x (ball speed / catch_speed_ref), otherwise he parries it: the ball comes back at parry_rebound_milli of its speed, pushed away from the centre of the goal. |
| `tip_share_milli` | 750 | explained, no source | A ball he reaches only at the edge of his reach (offset beyond tip_share_milli of it) is tipped: his touch turns it but does not stop it, and it goes on past the post (a corner when it crosses the goal line); nearer, a parry pushes it back into play (2026-11-13). |
| `standing_reach_share` | 0.75 | explained, no source | Standing reach (2026-11-13, plan L0-CLOSE): his reach before any dive is standing_reach_share of his height from the centre of his body (his arm span is about his height, plus a step): it was half his height, a keeper with his arms down. |
| `reference_keeper_height_cm` | 190 | explained, no source | The shooter judging a chance imagines a keeper of reference_keeper_height_cm. |

### `scanning.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `scan_rate_min_per_s` | 0.2 | explained, no source | How often a player looks around (2026-11-13, plan L0-CLOSE 10b, ADR-052 e): his vision places him between scan_rate_min_per_s and scan_rate_max_per_s glances a second, staggered by player; between glances the gaze is on the ball. |
| `scan_rate_max_per_s` | 0.8 | explained, no source | How often a player looks around (2026-11-13, plan L0-CLOSE 10b, ADR-052 e): his vision places him between scan_rate_min_per_s and scan_rate_max_per_s glances a second, staggered by player; between glances the gaze is on the ball. |
| `scan_reach_cm` | 3000 | undocumented | — |
| `mark_radius_cm` | 800 | explained, no source | Marking beats the ball when the mark is within `mark_radius_cm` (ADR-034 cascade, not a weight). |
| `weight_ball` | 100 | undocumented | — |
| `weight_opponent_on_ball` | 80 | undocumented | — |
| `weight_mark` | 60 | undocumented | — |

### `shot_choice.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `power_strong` | 0.85 | explained, no source | The actor does not choose 'shoot' and then where and how strong: he chooses one of these complete actions. power_strong and power_placed are the two powers the scene makes visible; the read does not compute the optimum. aim_edge_fraction is how near the post the aim sits (1.0 = the post). reading_weight and risk_weight modulate the salience of each action via game_reading and risk_management (L0.11 section 4.5: produ |
| `power_placed` | 0.35 | explained, no source | The actor does not choose 'shoot' and then where and how strong: he chooses one of these complete actions. power_strong and power_placed are the two powers the scene makes visible; the read does not compute the optimum. aim_edge_fraction is how near the post the aim sits (1.0 = the post). reading_weight and risk_weight modulate the salience of each action via game_reading and risk_management (L0.11 section 4.5: produ |
| `aim_edge_fraction` | 0.8 | explained, no source | The actor does not choose 'shoot' and then where and how strong: he chooses one of these complete actions. power_strong and power_placed are the two powers the scene makes visible; the read does not compute the optimum. aim_edge_fraction is how near the post the aim sits (1.0 = the post). reading_weight and risk_weight modulate the salience of each action via game_reading and risk_management (L0.11 section 4.5: produ |
| `reading_weight` | 0.5 | explained, no source | The actor does not choose 'shoot' and then where and how strong: he chooses one of these complete actions. power_strong and power_placed are the two powers the scene makes visible; the read does not compute the optimum. aim_edge_fraction is how near the post the aim sits (1.0 = the post). reading_weight and risk_weight modulate the salience of each action via game_reading and risk_management (L0.11 section 4.5: produ |
| `risk_weight` | 0.5 | explained, no source | The actor does not choose 'shoot' and then where and how strong: he chooses one of these complete actions. power_strong and power_placed are the two powers the scene makes visible; the read does not compute the optimum. aim_edge_fraction is how near the post the aim sits (1.0 = the post). reading_weight and risk_weight modulate the salience of each action via game_reading and risk_management (L0.11 section 4.5: produ |
| `shot_range_cm` | 3500 | explained, no source | The shot is an option within shot_range_cm, and its distance factor is 1 / (1 + (d / shot_distance_ref_cm)^2): high in the box, small but never zero from distance (it used to fall linearly to zero at 25 m). 2026-11-13 (ADR-051, measured: from 10-30 m the carrier passed back 83 times and shot 4): the goal cone is read against how the goal looks from the penalty spot (shot_cone_ref 0.6, the sine of the angle of a 7.32  |
| `shot_distance_ref_cm` | 1800 | explained, no source | The shot is an option within shot_range_cm, and its distance factor is 1 / (1 + (d / shot_distance_ref_cm)^2): high in the box, small but never zero from distance (it used to fall linearly to zero at 25 m). 2026-11-13 (ADR-051, measured: from 10-30 m the carrier passed back 83 times and shot 4): the goal cone is read against how the goal looks from the penalty spot (shot_cone_ref 0.6, the sine of the angle of a 7.32  |
| `progression_gain` | 1.5 | explained, no source | ADR-050 (2026-11-12), the carrier reads what an option brings: a pass's salience is multiplied by max(progression_floor, 1 + progression_gain x appetite x (1 - pressure) x progression), where progression in [-1, 1] is how much nearer to the goal the receiver is than the carrier, appetite is his risk_management, and pressure (0..1) grows as the nearest opponent comes within pressure_radius_cm. |
| `progression_floor` | 0.4 | explained, no source | ADR-050 (2026-11-12), the carrier reads what an option brings: a pass's salience is multiplied by max(progression_floor, 1 + progression_gain x appetite x (1 - pressure) x progression), where progression in [-1, 1] is how much nearer to the goal the receiver is than the carrier, appetite is his risk_management, and pressure (0..1) grows as the nearest opponent comes within pressure_radius_cm. |
| `pressure_radius_cm` | 300 | explained, no source | kick_pressure_radius_cm (2026-11-13, plan L0-CLOSE step 8): an outfield opponent within it hurries the kick, linearly less the farther he is (a defender at 5 m still makes him rush); the engine's error and the shooter's own reckoning read the same value. ADR-050 (2026-11-12), the carrier reads what an option brings: a pass's salience is multiplied by max(progression_floor, 1 + progression_gain x appetite x (1 - press |
| `kick_pressure_radius_cm` | 1500 | explained, no source | kick_pressure_radius_cm (2026-11-13, plan L0-CLOSE step 8): an outfield opponent within it hurries the kick, linearly less the farther he is (a defender at 5 m still makes him rush); the engine's error and the shooter's own reckoning read the same value. |
| `shot_cone_ref` | 0.6 | explained, no source | The shot is an option within shot_range_cm, and its distance factor is 1 / (1 + (d / shot_distance_ref_cm)^2): high in the box, small but never zero from distance (it used to fall linearly to zero at 25 m). 2026-11-13 (ADR-051, measured: from 10-30 m the carrier passed back 83 times and shot 4): the goal cone is read against how the goal looks from the penalty spot (shot_cone_ref 0.6, the sine of the angle of a 7.32  |
| `carry_salience` | 0.6 | explained, no source | Carry (2026-11-13, ADR-051): with room in front he may carry the ball, an option like the others: carry_salience x (room / carry_room_ref_cm, at most 1) x brings to the point the room lets him reach; the room is the distance to the nearest opponent ahead of him towards the goal. |
| `carry_room_ref_cm` | 1000 | explained, no source | Carry (2026-11-13, ADR-051): with room in front he may carry the ball, an option like the others: carry_salience x (room / carry_room_ref_cm, at most 1) x brings to the point the room lets him reach; the room is the distance to the nearest opponent ahead of him towards the goal. |
| `clear_salience` | 2.5 | explained, no source | Clear (2026-11-13, ADR-051): within clear_zone_cm of his own goal and under pressure, a carrier may kick the ball away: clear_salience x pressure x (1 - distance / clear_zone) x (1 - 0.5 x appetite): the more he is pressed and the nearer his goal, the more he clears; a cautious player clears more than a bold one. From a wide position into the box it is a cross. 2026-11-13 (plan L0-CLOSE step 3): the post is chosen by |
| `clear_zone_cm` | 2500 | explained, no source | Clear (2026-11-13, ADR-051): within clear_zone_cm of his own goal and under pressure, a carrier may kick the ball away: clear_salience x pressure x (1 - distance / clear_zone) x (1 - 0.5 x appetite): the more he is pressed and the nearer his goal, the more he clears; a cautious player clears more than a bold one. From a wide position into the box it is a cross. 2026-11-13 (plan L0-CLOSE step 3): the post is chosen by |
| `other_side_factor` | 0.8 | explained, no source | Both sides (2026-11-13): the shot is offered to both posts, the side of his other foot at other_side_factor of the salience; the keeper hides the side he covers, so the shooter can pick the far post. From a wide position into the box it is a cross. 2026-11-13 (plan L0-CLOSE step 3): the post is chosen by the chance each gives with the keeper where the shooter sees him (other_side_factor is no longer read); the lob (p |
| `loft_salience` | 1.2 | explained, no source | The lofted pass (2026-11-13, ADR-051): to a free teammate from loft_min_cm away, over the line (the lane does not count), fading out at loft_max_cm; at loft_salience of a ground pass, because it is less precise and harder to control. |
| `loft_min_cm` | 1800 | explained, no source | The lofted pass (2026-11-13, ADR-051): to a free teammate from loft_min_cm away, over the line (the lane does not count), fading out at loft_max_cm; at loft_salience of a ground pass, because it is less precise and harder to control. |
| `loft_max_cm` | 5000 | explained, no source | The lofted pass (2026-11-13, ADR-051): to a free teammate from loft_min_cm away, over the line (the lane does not count), fading out at loft_max_cm; at loft_salience of a ground pass, because it is less precise and harder to control. |
| `power_lob` | 0.2 | explained, no source | From a wide position into the box it is a cross. 2026-11-13 (plan L0-CLOSE step 3): the post is chosen by the chance each gives with the keeper where the shooter sees him (other_side_factor is no longer read); the lob (power_lob, high contact) is an option over a keeper off his line; the clearance is worth the contested space where it lands (clear_salience is no longer read; clear_zone_cm remains: where he thinks of  |

### `stoppage_time.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `base` | 0 | undocumented | — |
| `goal` | 60 | explained, no source | Seconds of stoppage per event (ADR-026). duel 10 -> 0 (2026-11-12): a clean duel does not stop play; the time is lost to fouls, goals, injuries. |
| `foul` | 20 | explained, no source | Seconds of stoppage per event (ADR-026). duel 10 -> 0 (2026-11-12): a clean duel does not stop play; the time is lost to fouls, goals, injuries. |
| `duel` | 0 | explained, no source | Seconds of stoppage per event (ADR-026). duel 10 -> 0 (2026-11-12): a clean duel does not stop play; the time is lost to fouls, goals, injuries. With the duel in the match (SIM_VERSION 47) thousands of duels made a 328-minute match. |

### `thought.json`

| Parameter | Value | Status | What the file says about it |
|---|---|---|---|
| `goal_width_cm` | 732 | sourced | Two of football's own values and one resolution, nothing per kind of option. goal_width_cm is Law 1. possession_value is what keeping the ball is worth against a goal (1.0): a side with the ball has a chance to score that a side without it has not; declared, to be derived at L3 from the engine's own matches. |
| `possession_value` | 0.02 | sourced | Two of football's own values and one resolution, nothing per kind of option. goal_width_cm is Law 1. possession_value is what keeping the ball is worth against a goal (1.0): a side with the ball has a chance to score that a side without it has not; declared, to be derived at L3 from the engine's own matches. The value of a space, in units of keeping the ball: (1 + how near the goal it is, as a share of the pitch) + i |
| `forecast_step_cm` | 100 | sourced | The forecast of who reaches a ball first samples its path every forecast_step_cm. Law 1 for the goal width. possession_value and forecast_step_cm: declared, arbitrary until L3; the chance a position gives is not a number of this file: it is computed from the shot and the save. horizon_min_cm and horizon_max_cm: arbitrary, declared, to be sourced at L3. |
| `believed_speed_cm_per_tick` | 65.0 | undocumented | — |
| `believed_reaction_ticks` | 3.0 | undocumented | — |
| `trust_ticks` | 10.0 | explained, no source | A forecast with trust_ticks (1 s) to spare is sure; one with none is a coin he does not toss but weighs as half. |
| `horizon_min_cm` | 1500.0 | sourced | Anticipation (2026-11-13, plan L0-CLOSE 10b): how far ahead he imagines, between horizon_min_cm and horizon_max_cm; an option that ends farther than his horizon he cannot see through, and his confidence in it falls towards a coin he does not toss (half), the more the farther. Law 1 for the goal width. possession_value and forecast_step_cm: declared, arbitrary until L3; the chance a position gives is not a number of t |
| `horizon_max_cm` | 4500.0 | sourced | Anticipation (2026-11-13, plan L0-CLOSE 10b): how far ahead he imagines, between horizon_min_cm and horizon_max_cm; an option that ends farther than his horizon he cannot see through, and his confidence in it falls towards a coin he does not toss (half), the more the farther. Law 1 for the goal width. possession_value and forecast_step_cm: declared, arbitrary until L3; the chance a position gives is not a number of t |
