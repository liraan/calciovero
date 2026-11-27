# Changelog — SIM_VERSION

This file records every bump of `SIM_VERSION`, the version identifier of the **simulation behaviour**.

It is separate from [`CHANGELOG.md`](../CHANGELOG.md) (which tracks documentation and structure) and from `CHANGELOG-REGISTRY.md` (removed 2026-11-12 until the registry exists, L3) (which tracks registry deprecations and migrations).

**Rule:** if the outcome of a simulation changes for the same seed and the same inputs, `SIM_VERSION` is bumped, and an entry is added here **before** the change is merged.

---


## 224 — the air goes to the man who gets there (opencode RUN 99, DD-97)

ADR-117: `resolve_aerial` chose the winner by `aerial_force` alone, so a man
who reached the ball a step later could win it against one already at it.
The contact now reads what `aerial_contenders` already calculates: a man
inside his reach beats one who can only contest him shoulder to shoulder;
then the earlier step of the same tick wins; `aerial_force` decides only at
the same step, with the first listed winning a tie (ADR-019). No weight,
parameter, option or gesture was added, and nothing was chosen by looking at
the band (A5-bis). The twin test `the_man_who_gets_there_first_wins_the_air`
holds the scene fixed and changes only arrival and reach. Golden L0/L1 and
the fixtures are regenerated; the measurement is in `L0-CALIBRATION.md` §224.

## 223 — the keeper is read where he is in the tick (opencode RUN 99, DD-95)

`CODE-AUDIT.md` T10: `advance_ball` judged the crossing of a shot — and
therefore the keeper's save — before `move_bodies`. The parry used the
position of the previous tick: the keeper was one tick behind the ball he
was facing. With LUNA the dito is the ball crossing the goal plane and the
luna is the keeper reading the flight and choosing how to meet it; a
position from the tick before is not what he saw. `move_bodies` now runs
before `check_shot_crossing`, with the guard
`the_keeper_moves_before_the_shot_crosses` on the order of the two calls.

It is an order of phases, not a new rule, a clamp or a tuned number:
`resolve_save`, the reach in `save.json` and the geometry of the goal are
untouched. Golden L0/L1 and the fixtures are regenerated (cup seed 78 → 47,
first offside header 48 → 52, first level friendly 49 → 47). `ds-reference
42..101` stays at **15 of 20 comparable metrics in band**: goals 5.28 →
5.02 (still out of the 2.68 ± 1.65 band), saves 5.08 → 5.83, shots on
target 9.85 → 10.23, conversion 53.6% → 49.1%, throw-ins 22.75 → 20.57
(worse), aerial duels 8.22 → 8.10. T10 is a real state correction, not the
cause of the five out-of-band rows; no parameter was fitted to them
(A5-bis). The measurement is in `L0-CALIBRATION.md` §223.

## 222 — a remembered offside line keeps the pass (opencode RUN 82, DD?-88)

On the local Wyscout dump 1,620 offside events give 4.263 ± 2.397 a match,
the reference band. The pass-first story holds: 1,327 of them (81.9%) have a
pass of the same team within five seconds and in 93.1% of those pairs the
offside event sits exactly on the pass endpoint. The dump has no positions
for the defenders, so it cannot measure the line itself.

`offside_to` already reads Law 11 from the positions the passer perceives,
remembered positions included, with the eye's 8 mrad threshold (ADR-062 (b)).
`line_in_sight` added a second gate: when the two deepest defenders he
perceives were not both in `motion` that tick, the ground pass and the loft
lost all worth. The tracked player had a line and could not play from it; the
whistle, meanwhile, is raised from real positions (`referee.rs`). That is a
veto on the choice, of the same kind as DD-81 and DD-86, not an extra
perception gate. ADR-062 (c) — reading the line from the last look outside
the spread of attention — would need a freshness threshold no source gives,
so none is added (A5-bis).

The gate and its `beyond_ball` helper are gone; the perceived line and the
eye threshold stay. The twin `an_unseen_offside_line_does_not_take_the_pass_away`
reads the same scene with and without the two men in `motion` and asserts the
same pass worth. Golden L0/L1 and the fixtures are regenerated;
`ds-reference 42..101` is recorded in `L0-CALIBRATION.md` §222.

## 221 — a contested loft is graded, not vetoed (opencode RUN 80, DD-86)

Wyscout counts 38.14 aerial duels a match; the local dump reproduces it
exactly (28,985 `eventId 1 / subEventId 10` records, halved because both
contenders are recorded, over 380 matches). No tag says there was contact:
the source counts a contest in the air. In 6,639 contests with a high pass
or cross in the previous five seconds, the kicking side wins 39.26% of the
decided ones: in a real match the kicker lofts even when he loses the
header. `read.rs` used to set `sure = 0` when `wins()` imagined a loss,
taking the loft away before the player chose. That was a veto on the *how*,
not a perception gate (ADR-062) nor a Law; DD-81 had already removed the
same kind of gate on the ground pass.

A contested loft now keeps the option: the cases `wins()` used to veto keep
their old `confidence(-spread)`; the imagined losses are graded by
`confidence(-spread) * mine / (mine + rival)` over the aerial forces the
passer imagines for his man and for the opponents who arrive before it
lands. Only the vetoed branch changes — grading the won branch as well had
weakened a choice that was already correct. No new number or threshold: it
is the `freedom_share` pattern of ADR-040. ADR-081's second man (option C),
the flight's drop point, the perception cones and the Laws are untouched.

Twin `the_long_ball_to_the_man_who_wins_it_in_the_air` now reads three
scenes with the same geometry — stronger, even, weaker in the air. It is red
before the change (even and weaker had identical confidence) and green
after. Golden L0 base/instructed/cup regenerated (cup shoot-out seed 166 ->
62); the fixtures that follow the new play moved: first offside header 50 ->
79, first level friendly at 90' 44 -> 53, first knock with a substitution
43 -> 42, first kick-only shoot-out 166 -> 62; the movement guard now
ignores a man placed on the bench by a sending off. `ds-reference 42..101` is recorded in `L0-CALIBRATION.md` §221. A5-bis:
the 39.26% is evidence that the veto is false, not a number fitted to the
band.

## 220 — a won take-on is told at the contact (opencode RUN 77, DD-84)

Wyscout's `take_ons_won` is the **attacking ground duel won with the take-on
tag** (`eventId 1`, `subEventId 11`, tags 503/504 and 703): in the local dump
those are exactly 16.03 per match, the reference's mean. Run 77 measured the
same dump and found 6.97 more won take-ons from defending duels (`subEventId
12`), which the reference excludes. DD-71/SIM 206 had told a `Dribble` only
after the carrier got past the man and was nearer the goal; that premise about
the source was wrong, and the post-hoc `past` gate on the label was neither a
perception gate nor a Law.

The won contact is now told in `beaten_by_dribble`, the point shared by the
tackle and jockey branches (SIM 217/DD-87). A `Dribble` carries `won` (the man
beaten), not `past`. The `taking_on` pending list and its `settle_take_on` pass
are gone: they existed only to defer that label. The player still chooses
`TakeOn` versus `Shield` before the contact and still decides what to do with
the ball after it; only the chronicle of the act changes.

Twin `a_take_on_won_is_told_at_the_contact`, plus the existing
`a_take_on_against_a_jockey_is_remembered` and
`a_second_man_beaten_does_not_erase_the_first` updated to read the new event.
Golden L0 base/instructed/cup regenerated (cup seed 166 unchanged); the L1
golden and every seed scan (44, 42, 166, 43) still hold. `ds-reference 42..101`:
**11/20 comparable metrics in band** (was 10); take-ons 7.98 -> 18.80 against
16.03 ± 5.67. No number was chosen from a metric (A5-bis); DD-71 is emended on
source evidence, not tuned to the band.

## 219 — a marked mate is still a pass option (opencode RUN 74, DD-81)

`pass_candidates` had the prototype's hard gate: a teammate with less than
`PASS_FREEDOM_CM` (300 cm) of room was not offered as a ground pass at all.
ADR-040 already grades that same room as salience (`freedom_share`), and
ADR-052's note had left the gate only because the prototype had it. With LUNA
the gate took away a choice the player may need — the wall pass, the lay-off,
the ball played into a man who is marked but moving away. The option is now
always present and the room grades its worth; the lookahead to the receiver's
next pass follows the same rule. No number, weight or threshold was added or
changed, and the lofted branch is untouched (ADR-101).

Twin `a_marked_mate_is_still_a_pass_option`: same passer, same receiver, same
ball; only the marker 50 cm away instead of 25 m. Before, the marked scene had
no ground-pass candidate; now it has one, worth less. Recorded in
`context/plans/BEHAVIOURS.md`, tendency 9, read with the partita-vera and LUNA
filters. Golden L0 base/instructions/cup regenerated (cup seed 43 -> 166),
small-season L1 golden updated, GUI seed 42 unchanged. `ds-reference 42..101`:
**10/20 comparable metrics in band** (was 11); passes 1138.15 -> 1137.20,
accurate 1060.28 -> 1056.60, goals 4.00 -> 4.33 (0.008 outside the band).
The gate was nearly inert on the count: the fix is doctrinal, not a tune.
No number was chosen from a metric (A5-bis).

## 218 — a restart kick is told once (opencode RUN 73, DD-90)

A goal kick, corner, throw-in or free kick played to a mate was told twice:
the restart at the whistle and a `Pass` when the ball landed. In the local
Wyscout file none of the 37,861 restart events has a `Pass` with the same
start and end positions; only 354 have any pass by the same player within
two seconds, and none is the same kick. With LUNA the kicker makes one
choice and strikes once, so the second label was a counting bug, not a
different definition. `Pending` now carries whether it is the restart kick
and the landing tells no second `Pass`; an intercepted restart still emits
`Interception`, which `ds-reference` no longer counts as a pass too. No
choice, number or threshold was added, and the ball still flies the same
way.

Twin `a_restart_kick_is_not_told_as_a_second_pass`: same receiver, same
ball, same tick; a normal pass is told once, a restart kick is not told
again. Golden L0 base/instructions/cup (seed 43 unchanged), the L1 golden
and the GUI seed all still hold. `ds-reference 42..101`: **11/20 comparable
metrics in band, unchanged**; passes 1175.30 -> 1138.15, accurate
1092.35 -> 1060.28. No number was chosen from a metric (A5-bis).

## 217 — a won answer is remembered under pressure too (opencode RUN 66, DD-87)

`resolve_duel` gives the carrier the same `TakeOn` answer whether the opponent
tackles or only jockeys him (SIM 170, ADR-086), and in both cases he can keep
the ball. Only the tackle branch called `beaten_by_dribble` when he did: a man
who chose to take his marker on and won under near pressure was forgotten, so
the take-on could never become a dribble when he went past him. The jockey
branch now remembers the same answer with the same `duel.beaten_ticks` already
used there for the cooldown. No threshold, weight or choice was added, and
`resolve_duel` is untouched.

Twin `a_take_on_against_a_jockey_is_remembered`: same carrier, same jockey,
same scene; red before, green after. Golden, seeds, the L1 golden and the
GUI seed 43 all still hold. `ds-reference 42..101`: **11/20 comparable metrics in band, unchanged from SIM 216** (take-ons 8.35 -> 8.48, still outside; free kicks 21.75 -> 21.00 and clearances 20.77 -> 20.20 move slightly further out). No number was chosen from a metric.
## 216 — a corner is a loss, a wait needs a man arriving (loop DD-8, DD-9, DD-10; DD-78, DD-79, DD-80 in `STATE.md`)

DD 5 gave the pressed defender the choice to knock it behind his own goal (Law 17), but the landing was valued like any other: nobody contests a ball that is out, so its confidence was 0.70 against 0.15 for a landing up the pitch and 84% of clearances ended behind the kicker's goal. With LUNA the man still chooses where to kick; what was wrong was the *value he read* — the opponents receive a corner, not a free ball at the goal line. The kick behind now carries the same `expected(0, 0, loss, risk)` already used for a ball into touch; no threshold, weight or removed choice. DD-10 fixes where the loss is read: not a shot from the flag (`space_value` there gave a corner *less* danger than a throw-in 50 m up the touchline), but the cross into the area, read at the penalty area line — a distance the Laws already declare, so no new number. Guards: `a_corner_is_not_a_free_landing`, the same defender, pressure and three opponents, only his mate 15 m clear of the near landing: before he put it behind his own goal, now he keeps it on the wing; `a_corner_is_a_loss_even_with_a_mate_behind`, the DD 5 twin, now keeps it in touch even with a mate waiting behind. The choice to put it behind remains in `clear_candidate`.

Removing that easy escape uncovered DD-9: the keeper in seed 42 chose `Hold` 21 times in a row from the 114th minute and stood still for 343 ticks. He foresaw a through ball to a mate who was not going anywhere, while other men wandered in `scene.soon`; DD-6 only asked that *someone* move, so the same unrealised scene was re-projected every tick. Now the awaited option must be a pass, loft or through to a man who actually moves towards it. No new number, no choice removed: if nobody arrives, he weighs only the options he has now. Guards: `the_wait_is_for_the_man_who_brings_the_option`, the same scene with only the mover changing (fails before), and the existing `a_carrier_with_nothing_on_does_not_stand_still`, which goes green again.

## 215 — the glance, the room, the line seen, the ball behind the goal, and the wait that ends (DD 1-6)

Six decisions from the same L0 match, one bump. (1) `LatencyContext` carries `ticks_to_scan`, the ticks to the next staggered glance from the very interval `move_agents` uses (`ADR-034`); `resolve_latency` takes it as a **floor**, not a second term, because `ADR-044 §6` puts perceiving inside the chain. (2) The freedom of the receiver, until now a yes/no gate at `PASS_FREEDOM_CM`, grades the imagined gain of the ground pass and of the loft at its landing point, using that same declared threshold (`ADR-040`; no lookahead past the landing, `ADR-101`). On the loft the grade applies only when he must control the landing: in a contested air ball the marker is already read by `wins()` (OI-102), so grading him again would count the same man twice. (3) A receiver beyond the ball is not clean unless the two men of the offside line are in the passer's cone **this tick** (`scene.motion`), a gate of perception (`ADR-062`), not a new threshold: the eye's 8 mrad and the offer of the runner are untouched. (4) `clear_candidate` may aim behind the kicker's own goal (Law 17), valued like its other landings with the same one measure of ADR-052, and `play_clearance` no longer clamps the chosen point back inside the pitch: a deliberate kick out stayed in play before. Guard: `a_pressed_defender_may_put_it_behind_his_own_goal`, the same defender and the same three contested landings, only a mate behind him changing. (5) `hold_candidate` only imagines a wait when someone he sees is actually going somewhere: with everyone standing still, the foreseen scene is this one and `Hold` used to be chosen again and again (the keeper held from the 59' to the final whistle, no events, no stoppage, the planned substitution never made). Guard: `he_does_not_wait_for_a_scene_that_does_not_change`, the same man, the same pressure, the same man foreseen — only his destination equal to where he stands. No choice or number was taken away: waiting for a moment that cannot arrive is not a moment. Guards: `the_chain_waits_for_the_next_glance_without_adding_to_it`, `room_at_the_landing_grades_the_pass`, `an_unseen_offside_line_is_not_a_clean_pass`. The aerial duel stays on the choice C of `ADR-081` (DD-19): no amendment, no extra contest imposed.

## 214 — the touch belongs to the receiver, and a second man does not erase the first (opencode)

Two independent attribution fixes, one bump. `first_touch_across` used to decide which goal line was the receiver's own from `home_in_possession`; with LUNA the touch belongs to the man who takes it, so the line now comes from `team_of(receiver)` (`PlayerId`), the identity already carried into `follow_carrier`. Guard: `the_first_touch_knows_the_receivers_own_goal_not_the_possessions`, two men in their own boxes with the same possession flag (fails before, passes after). Separately, `taking_on` was a single slot: beating a second man before getting past the first overwrote the first take-on and lost his dribble. It is now one pending entry per pair, settled in its own order; no choice, gate or number was added. Guard: `a_second_man_beaten_does_not_erase_the_first` (one man → one dribble, two men → two). Only the instructed golden changed (`EXPECTED_INSTRUCTED`); the base and cup golden, the seeds (cup 57, level at 90' 50, shoot-out 57, header offside 45), the L1 golden and both guards were re-verified green. No number was chosen from a metric.

## 213 — the loft reads its landing again (ADR-101, DD-76)

SIM 210 made the lofted pass worth the receiver's next pass, with the same lookahead as the ground pass. ADR-101 §Conseguenze explicitly leaves the lofted and filtered branches out of OI-116, and the ball can leave the pitch before the receiver can choose: `lofted_missed_at` only sees his own goal, not the touchline. The receiver's options stay all available; the passer's imagined gain is what changes. Guard: `a_long_ball_does_not_read_past_the_landing`, the same scene with the next mate free or marked now has the same loft salience. Golden and seeds regenerated. `ds-reference 42..101`: corners 18.02 → 6.52, second-half stoppage 5.81 → 4.91 min, 9 → 14 of 22 in band. No number was chosen from the metric.

## 212 — a knock and his forced change are one stoppage (opencode RUN 31, DD-75)

`stoppage_for` added an injury (60 s) and the substitution that follows it one tick later (30 s) as if they were two interruptions. Law 7.3 recovers time lost, not events: the knocked man leaving is the same stoppage. Now a substitution whose `out` is the player injured at the previous tick adds no time; a tactical change still does. Guard: `the_knock_and_his_change_are_one_stoppage` (RUN 31: 56 of 56 injury→substitution pairs had the same slot and a one-tick gap). Golden and seeds regenerated. `ds-reference 42..101`: first-half stoppage 3.45 → 3.17 min, second-half 5.92 → 5.81 min (still above the real bands); 9 of 22 in band, unchanged.

## 211 — the man who played the ball waits for his rebound (opencode RUN 32, DD-74)

A pass blocked by a defender, a pass that comes off a man's body, and a shot blocked on its way all leave the ball loose. Only the man who met it had to wait `retry_ticks` before he could take it back; the man who played it could be the carrier again one or two ticks later (RUN 32: 645 of 1084 blocks had him back on the ball at +1, 728 at +2; 37 shots repeated within two ticks). Now both men wait with the same clock already used after a failed touch and a clearance — no new number. Guard: `the_shooter_waits_after_a_block`, on the recorded ticks of seed 42. Golden and seeds regenerated. `ds-reference 42..101`: shots 38.73 → 30.50, throw-ins 17.77 → 33.32, corners 7.73 → 18.08, saves 6.75 → 5.08; 9 of 22 in band (the two metrics the reference declares not comparable stay excluded). No number was chosen from these measures.

## 210 — the lofted pass looks one step ahead (ADR-052, OI-116 A, T33; DD-73)

A lofted pass was worth only the space where the ball came down, while a ground pass was worth the best pass the receiver would then have. Now the loft uses the same measure (`receiver_best_pass` through `worth_of`) as the ground pass, so a man he reaches in the air is worth his next option too. No number was chosen from a metric. Twin test: `a_long_ball_to_a_man_with_a_free_pass_is_worth_more`. Golden and seeds regenerated. `ds-reference 42..101`: goals 4.98 → 6.93, own goals 0.23 → 0.47, corners 7.23 → 18.97, shots 42.7 → 49.4, crosses 7.8 → 14.2, aerial duels 6.6 → 11.0, throw-ins 19.1 → 33.4; 12 → 10 of 22 in band. Open lead: the corners (too many high balls go out for a corner) and the own goals; `receiver_best_pass` also counts the receiver's own lofts (opencode RUN 17).

## 209 — a header worth less than nothing is no header at goal (opencode RUN 18)

`header_shot` played the best of the headers at goal he read even when every one was worth less than nothing: every high ball won in the opponents' box was a shot. Now a header worth less than zero is not played; he brings the ball down, the branch the engine already had. Zero is the line every other choice of this reading uses. `ds-reference 42..101`: shots 38.85 → 38.73, the rest within a decimal; 14 of 22. What makes shots ×1.6 is elsewhere (RUN 18: rebounds, 685 shots within 6 s of another; the 35 m range, which needs a source and a choice). The lofted pass's look ahead is now the patch for 210.

## 208 — the added time of Law 7.3 (OI-101; first tried on the branch sim-193, DD-53)

The referee added 20 s for every foul. Law 7.3 lists the causes of time lost (substitutions, injuries, wasting time, disciplinary sanctions, medical and drinks breaks, VAR, any significant delay to a restart); a foul as such is not among them: foul 20 → 0. Substitutions and injuries now add the traditional allowance of the reference season: 30 s and one minute (Collina, FIFA 2022; an IFAB board member to AFP, 2018). The SIM 203 guard that read a stopped foul as time lost is replaced by `a_foul_adds_no_stoppage_an_injury_does`. `ds-reference 42..101`: added time 6.86 / 6.75 → 3.75 / 5.40 minutes (real 1.49 / 4.08; the second half now in the band), and with the shorter halves ground duels 211.8 → 201.2 (in the band); 14 of 22 in the band (12 at 207). Open: cards and wasting time have no measured allowance (OI-101). Seeds: level at 90' 49, the cup's shoot-out 129. The lofted pass's look ahead is now the patch for 209 (opencode RUN 17: it needs a choice, see HANDOFF).

## 207 — in his own area the first touch goes across (code audit T9, opencode RUN 10)

A receiver whose first touch does not kill the ball let what was left run on the way it came: on a pass played back to him in his own area, on towards his own goal line, and he chased it into his net (opencode RUN 10, seed 42: the keeper received 1.9 m from the line and the ball crossed it). In his own penalty area (Law 1's) the first touch never runs on towards his goal line: he takes it across, to the side the ball was going or away from the goal's middle, as a keeper or a defender opens his body to a pass back. The same rule as the carry's (nobody dribbles into a goal); out of the area nothing changes. Twin test: `in_his_own_area_the_first_touch_goes_across`. `ds-reference 42..101`: own goals 0.90 → 0.23 (real 0.10 ± 0.31, now in the band), goals 5.48 → 4.98, corners 10.88 → 7.23; 12 of 22 in the band (11 at 206, counted the same way). Goldens regenerated; seeds: level at 90' and the cup's shoot-out 52, the header from offside 42. The lofted pass's look ahead is now the patch for 208.

## 206 — the dribble is getting past the man (opencode RUN 15)

A take-on won at the contact is no longer a dribble at once: the man taken on is down for his `down` ticks, and the dribble is told when the carrier, the ball still his, is past him, the man no longer goal-side (the carrier nearer the goal he attacks). If he loses the ball first, or the man is up again before he is past, there is no dribble. Before, the dribble was told at the contact, the two still a step apart (opencode RUN 15: 1483 of 1559 within 150 cm); the take-on won of the Wyscout data the metric reads is the dribbler getting past the opponent. Only the event moves: every other metric of 42..101 is the same to the decimal. Guard: `a_dribble_is_told_once_the_man_is_past` (at every dribble, the man is past). `ds-reference 42..101`: take-ons won 25.98 → 9.73 (real 16.03 ± 5.67): out of the band on the other side; what is left is how often a man is taken on, not how it is counted. 10 of 22 in the band, as at 205. The lofted pass's look ahead, ready as SIM 206, is now `context/plans/patches/sim207-loft-lookahead.patch`.

## 205 — il pallone alto lo contende chi ci arriva nel tick (DD-68)

T29, seconda metà (`CODE-AUDIT.md`): contendono il pallone alto tutti quelli che lo raggiungono lungo il suo percorso nel tick, al punto in cui lo incontra il primo; prima solo chi arrivava nello stesso piccolo passo (un cinquantesimo di secondo). Nessun numero cambiato. I duelli aerei salgono poco (6.15 → 7.92): la causa grande è altrove, i palloni alti giocati sono pochi (i cross 8.7 contro 33.8), un indizio nuovo.

Golden rigenerati; semi coppa 127, pari al 90' 49, decisa 42, colpo di testa in fuorigioco 46. ds-reference 42..101: **10 su 22** (shots_on_target, penalties, fouls, yellow_cards, red_cards, corners, goal_kicks, free_kicks, clearances, saves); gol 5.48, autogol 0.9, tiri 42.92, in porta 11.13, passaggi 1628.97, duelli aerei 7.92, duelli a terra 210.83, dribbling 25.98, spazzate 22.77, rimesse 17.82, fuorigioco 9.25, recupero 7.22/6.75.

## 204 — il pallone alto è una scelta del giocatore (DD-65)

T29, prima metà (`CODE-AUDIT.md`): nella propria metà il pallone alto del passaggio di un compagno si stoppa, non si spazza d'ufficio (prima anche il rinvio corto del portiere finiva lontano); nell'area avversaria, se la sua scelta di testa non trova un tiro, il pallone gli scende ai piedi (prima il motore tirava d'ufficio al centro della porta; `header_at_goal` tolto). Nessun numero cambiato. Aperta l'altra metà: il duello aereo nasce solo se due uomini arrivano nello stesso piccolo passo.

Golden rigenerati; semi coppa 194 (nessuna delle prime cento partite di coppa pari dopo i supplementari: un sintomo dei gol in eccesso), pari al 90' 47, decisa 42, colpo di testa in fuorigioco 46. ds-reference 42..101: **10 su 22** (shots_on_target, penalties, fouls, yellow_cards, red_cards, corners, goal_kicks, free_kicks, clearances, saves); gol 5.43, autogol 0.83, tiri 42.37, in porta 11.32, passaggi 1626.17, duelli aerei 6.15, duelli a terra 213.62, dribbling 27.23, spazzate 22.5, rimesse 17.73, fuorigioco 9.2, recupero 7.17/6.74.

## 203 — la terza passata dell'audit: la porta, le Leggi, la previsione (DD-63)

Terza passata dell'audit (`CODE-AUDIT.md`, T1-T32), le metriche fuori banda trattate come bug. La porta: il portiere incontra ogni palla diretta nella sua porta, non solo il tiro, con il tempo che la palla ha fatto dal suo ultimo tocco; non è un tiro e conta come presa (T3); chi va su un pallone in movimento lo prevede con l'andatura che tiene in contesa e lo incontra correndo, mai oltre la linea di porta (T1); il controllo fallito fa rimbalzare la palla sul corpo invece di attraversarlo (T2); il gol della palla lenta non si perde (T4); la presa resta in mano al portiere, non oltre la linea (T5); il muro guarda tutto il percorso del tiro e il difensore più vicino (T6); la spazzata di prima dice chi l'ha calciata (T7); "in porta" con lo stesso giudice del gol (T8). Le Leggi: il fallo con il vantaggio non dà recupero (T15, Legge 7); al rinvio gli avversari fuori dall'area (T18, Legge 16); ogni tempo comincia con tutti nella propria metà (T19, Legge 8); nessun intercetto a orologio senza tocco (T20); fuorigioco anche al tiro (T21, Legge 11). La previsione: il posto del ruolo resta in gioco (T24); una fiducia zero per un fatto certo resta zero oltre l'orizzonte (T25); il tiro pensato alla velocità della sua potenza (T28); se non legge nessuna opzione gioca quella che vede meglio, non il portiere per slot (T31); il ricevitore decide dopo il suo tempo di decisione (T32). La larghezza chiesta dall'allenatore: il posto del ruolo vale contro la scelta propria del giocatore con la regola di `obey` (ADR-059 §4, la disciplina sua); prima solo un pari lo teneva, e con il posto rimesso in gioco (T24) una squadra chiesta larga giocava più stretta di una chiesta stretta (`the_team_plays_wider_when_the_coach_asks`). Nessun numero cambiato.

Golden rigenerati; semi coppa 43, pari al 90' 43, decisa 42. ds-reference 42..101: **9 su 22** (penalties, fouls, yellow_cards, red_cards, corners, goal_kicks, free_kicks, clearances, saves); gol 5.5, autogol 0.78, tiri 45.02, in porta 11.92, passaggi 1583.57, duelli aerei 6.08, duelli a terra 214.63, dribbling 26.53, falli 27.27, gialli 3.1, rossi 0.45, angoli 8.67, rimesse 18.37, punizioni 34.9, fuorigioco 9.77, spazzate 36.53, parate 7.47, recupero 7.16/6.83.

## 202 — le Leggi su molte partite; la porta con la misura della palla (DD-62)

Terza fase dell'audit (`CODE-AUDIT.md`): `tests/cases/laws_hold.rs` controlla le Leggi a ogni tick del record su molti semi (Legge 3: al più undici e sempre un portiere; nessun espulso in gioco; Leggi 1 e 10: gol solo a palla intera fra i pali e sotto la traversa; Leggi 15-17: ogni ripresa alla squadra e del tipo giusti; cambi entro il limite; Legge 11: fuorigioco solo su chi riceve). Seme 42 nel gate, 42..142 con `--ignored`. Due violazioni trovate e corrette: (a) la porta senza la misura della palla: `obstacle_hit` metteva il palo sul bordo interno e la palla era un punto (seme 48: gol con il centro a 356 cm, il fianco sul palo); ora gli assi di pali e traversa stanno a un raggio fuori dalla luce della porta, il raggio della palla conta, `in_the_goal_mouth` è l'unico giudice per il tiro e per ogni palla, `crossbar_thickness_cm` tolto (Legge 1: la traversa come i pali); guardia `the_ball_has_a_size_at_the_goal`; (b) il fuorigioco sulla palla sciolta (S12) non diceva il tick del passaggio: `MatchState::offside_played`. Dopo: 100 partite su 100 pulite.

Golden rigenerati; semi coppa 58, pari al 90' 43, decisa 42. ds-reference 42..101: **6 su 22** (penalties, ground_duels, fouls, red_cards, goal_kicks, saves); gol 7.38, autogol 1.03, tiri 43.45, in porta 20.3, passaggi 1832.7, falli 29.87, gialli 1.37, rossi 0.53, parate 11.62, fuorigioco 15.38, recupero 8.86/8.48.

## 201 — resto della seconda passata (DD-61)

Resto della seconda passata dell'audit (`CODE-AUDIT.md`): un compagno prende i guanti quando il portiere esce o è espulso (S8, Legge 3); la palla che rotola rimbalza sul palo come quella in volo (S9); fuori e gol leggono lo stesso ultimo tocco, deviazioni comprese (S13); l'attore legge da sé orizzonte e nervi (S7); il test di ADR-112 legge tutti i file del motore e vuole un nome letterale, `PlayerLatents` senza `Debug` (S6); un solo parser della Legge 1 e il raggio del palo in `laws.json` (S18, S22); nove chiavi mai lette tolte (S17, S23). I gol salgono e le parate escono dalla banda: da indagare come indizio (Terza fase), nessun numero toccato.

Golden rigenerati; semi coppa 122, pari al 90' 50, decisa 42, colpo 43. ds-reference 42..101: **5 su 22** (penalties, ground_duels, fouls, red_cards, goal_kicks); gol 7.78, autogol 1.13, tiri 43.58, in porta 21.17, passaggi 1830.32, falli 30.63, gialli 1.38, rossi 0.48, parate 13.3, fuorigioco 14.1, recupero 9.33/8.66.

## 200 — seconda passata dell'audit (DD-60)

Seconda passata dell'audit (S1-S20, `CODE-AUDIT.md`): la palla è fuori solo intera oltre la linea, come per il gol (S1); il tiro alto vale la stessa probabilità per la parte che resta sotto la traversa, non un peso fisso di 0.85 (S4, ADR-052); il colpo di testa pensato usa il proprio errore stimato, senza la pressione infinita e la potenza contata due volte (S3); il lancio è valutato dove il compagno è, come il motore lo calcia (S2); il cono non vede gli espulsi (S10); la pressione nomina il portatore (S11); il fuorigioco resta segnato se il passaggio è sbagliato o deviato (S12, Legge 11); vantaggio su chiara occasione: giallo, non rosso (S14, Legge 12); un cambio impossibile non occupa un posto (S15); la deriva del passaggio e del rinvio dal piede, non dal segno di dy (S16); il subentrato parte dall'intesa di base (S19); i tiri dei rigori non contano nel possesso (S20); `World` dà solo le caratteristiche che la scheda del plugin dichiara, `World::top_speed` tolto (S6).

Golden rigenerati; semi coppa 110, pari al 90' 48, decisa 42, colpo 43. ds-reference 42..101: **7 su 22** (penalties, ground_duels, take_ons_won, fouls, red_cards, goal_kicks, saves); gol 6.53, autogol 0.82, tiri 42.43, in porta 19.7, passaggi 1854.2, falli 31.4, gialli 1.45, rossi 0.55, parate 12.47, fuorigioco 14.73, recupero 8.79/8.21.

## 199 — Ognuno incontra il pallone alto con la sua testa; la panchina legge l'orologio della trasmissione (DD-59)

Audit del codice C5: il pallone alto si incontrava a 180 cm per tutti; ora all'altezza di ciascuno (`intercept`, `drop_point`), e dove serve un uomo medio con `movement.json` `height_ref`. Trovato durante il lavoro: la panchina confrontava il minuto di un cambio pianificato e delle pause per l'acqua con l'orologio che corre nel recupero; un primo tempo con un recupero lungo arrivava al "60'" e faceva lì il cambio del secondo tempo. Ora nel recupero l'orologio resta a "45'+".

Con LUNA: dito = 180 cm, un minuto; luna = il corpo di ciascuno e l'orologio della partita.

Golden rigenerati; semi coppa 96, pari al 90' 43, decisa 42, colpo 48. ds-reference 42..101: **5 su 22** (penalties, ground_duels, fouls, red_cards, goal_kicks); gol 7.17, autogol 0.87, tiri 45.03, in porta 20.78, passaggi 1843.65, falli 32.33, gialli 1.22, rossi 0.67, parate 13.48, fuorigioco 12.52, recupero 8.87/9.08.

## 198 — Il pallone vola nella stessa aria per chi calcia e per chi pensa (DD-58)

Audit del codice B4: il tiro alzava il pallone calcolando il volo senza l'attrito dell'aria (la palla saliva meno del previsto), la probabilità del tiro e del pallonetto immaginati lo facevano volare a velocità costante. Ora `resolve_shot` e `ThoughtBalance::flight_ticks` usano il tempo di volo con l'aria di `ball.json`, come il passaggio alzato dal SIM 110.

Con LUNA: dito = un tempo di volo; luna = quello che prevede è quello che succede (ADR-052).

Golden rigenerati; semi coppa 48, pari al 90' 48, decisa 42, colpo 45. ds-reference 42..101: **7 su 22** (penalties, ground_duels, take_ons_won, fouls, red_cards, goal_kicks, saves); gol 6.95, autogol 0.83, tiri 43.63, in porta 20.15, passaggi 1829.75, falli 30.68, gialli 1.42, rossi 0.7, parate 12.72, fuorigioco 12.78, recupero 8.4/8.78.

## 197 — Una misura per "in tempo", la pressione dal tiratore, la stessa testa alla ricezione (DD-57)

Dall'audit del codice: (1) "ci arriva in tempo" è una sola misura, `ThoughtBalance::in_time`: per un altro uomo con il tempo di reazione che gli si crede, per sé no; `left_by_nearer` leggeva i compagni senza reazione. (2) La pressione sul calcio si misura dal tiratore, come la legge lui e come la legge la difficoltà del tiro; il motore la misurava dalla palla. (3) A8: la scelta alla ricezione (`Actor::first_time`) pensa con la stessa testa di `decide` (`own_scene`): il suo errore, il piede migliore, il tocco dei compagni come li conosce; prima un tiro perfetto di destro e ogni controllo sicuro.

Con LUNA: dito = una formula, un punto, una scena; luna = lo stesso giocatore pensa con la stessa testa in ogni scelta. Golden rigenerati; semi coppa 122, pari al 90' 44, decisa 42, colpo 48. ds-reference 42..101: **5 su 22** (penalties, ground_duels, fouls, red_cards, goal_kicks); gol 7.75, autogol 0.78, tiri 47.83, passaggi 1833.95, falli 31.75, gialli 1.28, rossi 0.65, fuorigioco 12.15, recupero 9.12/9.22.

## 196 — I difetti dell'audit del codice (DD-55)

I bug della sezione A di `context/plans/CODE-AUDIT.md`, ognuno con il suo guardiano:
- la palla prevista verso la porta di casa (`ball_at_line`: lo spostamento negativo reso 1 da `.max(1)`) — gemello `the_line_at_the_home_goal_is_foreseen_like_the_other`, fallisce sul codice vecchio;
- il portiere che para è il primo della rosa viva, non l'id del titolare — `the_keeper_who_came_on_meets_the_shots`;
- Legge 11: il fuorigioco su una palla vinta di testa è fischiato; la chiamata dice il tick in cui il pallone è stato giocato (`played_tick`) e la GUI legge le linee lì — `a_header_from_offside_is_called`;
- la ripresa: il sostituto del battitore solo fra chi è in campo;
- calcio d'inizio e rigori sulla rosa viva (i subentrati calciano);
- Legge 10: nessun cambio durante la serie dei rigori — `no_substitution_during_the_shootout`;
- Legge 3: i cambi pianificati contano nel limite — `the_planned_substitutions_count_against_the_limit`;
- la palla non è "ferma" al rimbalzo mentre risale (le previsioni finivano lì);
- Legge 10: anche il tiro entra solo con la palla intera oltre la linea;
- Legge 5: con un uomo a terra niente vantaggio; `play_advantage` con il nome e il test giusti;
- il rigore della serie non si chiude su un evento senza attore;
- il rinvio con il piede più forte, non con quello del lato del campo;
- `kicker_nerves` rispetta l'interruttore della psicologia, e il colpo di testa lo legge invece delle latenti grezze;
- la separazione dei corpi lungo la linea fra i due (prima per assi, a 45°).
Fuori dal motore, senza effetto sull'hash: la cronaca "shoots wide" legge `on_target`; `EventChain::verify` ricalcola la catena; ds-demo conta i subentrati di casa come di casa.

Con LUNA: dito = id, segni, liste; luna = la regola o la fisica che valgono uguali per tutti e in ogni momento della partita. Golden rigenerati; semi coppa 50, pari al 90' 42, decisa 43, colpo 46. ds-reference 42..101: **6 su 22** (rigori, duelli a terra, dribbling, falli 30.1, rossi 0.73, rinvii); gol 7.65, autogol 0.95, fuorigioco 12.0.


La frenata da una fonte: Harper, Morin, Carling e Kiely 2020 (Sports Biomechanics, radar, Tabella 1), decelerazione media sull'arresto dopo uno sprint di 20 m 4.45 ± 0.61 m/s²; il motore frena a ritmo costante, quindi la media dell'arresto ± due deviazioni: 3.2-5.7 m/s² (prima 3.5-6.0, spiegati senza fonte). Atleti universitari, non professionisti: dichiarato. Con la nuova frenata, al seme 42 un portatore che aveva vinto la palla lontano dai piedi, senza un tocco in corso, restava fermo 39 minuti (decide solo con la palla al piede): ora va sulla palla, come chi l'ha giocata avanti e non l'ha raggiunta; lo ha trovato il guardiano `a_carrier_with_nothing_on_does_not_stand_still`. Il colpo che costringe al cambio (`knock.json`, grado 2): dal 194 i falli arrivano più lenti e 850 non toccava nessuno in 200 partite; rifissato con lo stesso metodo, 630: semi pari 42-281 0.183 colpi nel primo tempo a partita, dispari tenuti da parte 0.150, reale 0.25. Dichiarato: i falli del motore (36) sono sopra i reali (26), la soglia si sposterà quando scendono.

Con LUNA: dito = metri al secondo quadrato, il tocco, la soglia; luna = il corpo frena come quello di un atleta misurato, e chi ha la palla vicino la va a prendere. Golden rigenerati; semi coppa 88, pari al 90' 50, colpo 49. ds-reference 42..101: 4 su 22 (come il 194: rigori, duelli a terra, dribbling, rinvii); gol 9.28, autogol 1.25, falli 35.9, rossi 0.98, gialli 1.70, fuorigioco 10.55.

## 194 — Il corpo e l'errore (DD-53/ADR-111)

Il 193 è riservato al branch `sim-193-law7-stoppage` (recupero dalla Legge 7), non su main. L'intercetto conta la velocità solo verso il punto (un portiere che scivolava lungo la linea sceglieva un punto che non raggiungeva). L'accelerazione segue il profilo accelerazione-velocità misurato in partita (Alonso-Callejo et al. 2022): 7.2-9.6 m/s² da fermo, che cala in modo lineare fino a zero alla velocità di punta; prima 2.5-5 m/s² costanti, soglie GPS e non capacità. Chi corre a un punto frena per arrivarci (prima il pressatore superava l'uomo e tornava, 11 m per parte); il tackle entra a velocità. Il passatore sa che il lancio va lungo del proprio errore (`resolve_pass`) e non lancia nella sua porta. Legge 12: la chiara occasione negata in area, rigore, tentando il pallone, è giallo.

Con LUNA: dito = velocità, palla, porta alle spalle; luna = un giocatore conosce il proprio corpo e il proprio piede. Gemelli `a_man_running_away_meets_the_ball_no_sooner_than_one_standing`, chiara occasione in area in `card_for`. Golden rigenerati; semi 90/44/42, knock 109. ds-reference 42..101: 4 su 22 (main 192: 6); autogol 1.95 → 1.05, fuorigioco 9.98, duelli aerei 16.6, gol 9.2; falli 35.2, rossi 0.93, gialli 1.38 fuori.

## 192 — Sul passaggio ognuno legge il gioco che arriva (DD-52/ADR-110)

Sul passaggio in viaggio chi vede l'uomo a cui va legge il gioco che arriva: i difensori accorciano, marcano, coprono; i compagni si smarcano o attaccano la profondità; chi va sulla palla ci va prima. La memoria dello sguardo tiene la corsa vista e chi ricorda un uomo lo immagina proseguire (per trust_ticks); l'ultima linea durante il passaggio è letta dove la portano i difensori. Corretto OFFSIDE_PERCEPTION_ERROR_CM 2000 → 200 (il commento diceva ~2 m).

Con LUNA: dito = la palla che viaggia e l'uomo che le va incontro; luna = ognuno decide prima che arrivi. Gemelli on_the_pass_the_defender_steps_to_the_man_it_goes_to, a_man_seen_running_is_imagined_gone_on. Golden rigenerati; semi 52/52/42. ds-reference 42..101: 6 su 22; duelli a terra in banda (172.1); fuorigioco 13.4.

## 191 — Il pericolo che lasciano i compagni più vicini (DD-51/ADR-109)

In difesa pressare e marcare erano moltiplicati per un rango (1 al più vicino,
`lettura × 0,5` al secondo, 0 dal terzo): dal terzo uomo quelle azioni non
esistevano. Ora valgono la parte di pericolo che i compagni più vicini lasciano,
`Π(1 − in_time(d_k))`, con la fiducia e il passo creduto della sua corsa.
Nessun numero nuovo.

Con LUNA: dito = chi è più vicino; luna = ognuno legge quanto pericolo gli altri
lasciano davvero. Gemello
`the_second_man_doubles_up_only_when_the_first_may_not_get_there`. Golden
rigenerati; semi coppa 83, supplementari 54/42, rigori 83. `ds-reference
42..101`: 6 su 22, gol 10.98 → 9.78; pressing +7% (sonda, 10 partite).

## 190 — Il compagno è conosciuto quanto ci si fida di lui (DD-50/ADR-108)

Il passatore leggeva il primo controllo e il valore aereo dei compagni dalla
verità (i latenti): un piccolo oracolo, contro i tre piani di ogni attore. Ora
crede a ciò che ha visto di loro: `0.5 + intesa × lettura × (vero − 0.5)`,
la stessa fiducia del passo creduto del SIM 179, lo stesso uomo medio con cui
vede gli avversari. Nessun numero nuovo.

Con LUNA: dito = primo controllo e gioco aereo del compagno; luna = ognuno
conosce gli altri per ciò che ha visto di loro. Gemello
`a_mate_is_known_as_far_as_he_is_trusted`. Golden rigenerati; semi coppa 49,
supplementari 49/42, rigori 49. `ds-reference 42..101`: 6 su 22, come il 188
rimisurato (il "9 su 22" del 188 era un conteggio sbagliato). Il 189 (DD-49) è
accantonato sul branch `sim-189-first-time`.

## 188 — Il colpo di testa in area è letto come un tiro (DD-48/ADR-106)

Quando il vincitore di un duello aereo colpiva di testa in area, mirava al
centro con un errore fisso da `heading`, ignorando portiere, palo e propria
composure. Ora legge la stessa scena del tiro (aim/power/contact) e il colpo
viene risolto da `resolve_shot` con `is_header=true`: errore da heading, senza
piede/corsa/stride/attraversamento; stesse opzioni, nessun numero nuovo.

Con LUNA: dito = palla alta, contatto, portiere, distanza, tick; luna = lo
stesso giocatore che legge la porta e sceglie come colpire di testa. Gemello
`a_header_in_front_of_goal_is_a_shot_choice`. Golden e semi rigenerati.

## 187 — Il tiratore legge il proprio nervosismo (DD-47/ADR-105)

Il fattore con cui la compostezza modula la pressione di un calcio (ADR-057:
`2 (1 - composure)`) era applicato dal motore solo al momento del tiro. Il
portatore che sceglieva il tiro leggeva invece la pressione come se avesse
compostezza media: la sua lettura e la sua esecuzione non erano dello stesso
uomo. Ora lo stesso fattore passa nella scena percepita come `own_nerves` e
modula il termine di pressione dell'errore atteso in `aim_inside` e
`own_spread`. Nessun numero di `balance/`, nessun dado, nessuna scelta imposta:
il tiratore ha le stesse opzioni, le valuta con la propria compostezza.

Con LUNA: il dito è la pressione dell'avversario più vicino, la distanza,
l'angolo, il piede, la potenza e il tick; la luna è lo stesso giocatore che sa
quanto è calmo e sceglie con quella conoscenza di sé. Cambia il come legge e
sceglie, non l'etichetta dell'atto, non la fisica del calcio e non la Legge.
Gemello `the_nervous_shooter_reads_his_own_shot_worse_under_pressure`. Golden
e semi rigenerati.

## 186 — Il passo in più e la pressione che fa fallo (DD-43/ADR-101, DD-44/ADR-102)

Due decisioni del maintainer del 2026-11-22 in un solo bump: il comportamento
cambia per la stessa partita e i golden vanno rigenerati una volta sola.

- **OI-116 (A, DD-43):** il valore di un passaggio è almeno la migliore scelta
  che il ricevente avrà dove arriva, letta con lo stesso pensiero di chi passa,
  un solo livello più in là. Nessun peso nuovo, nessuna scelta imposta: nelle
  due scene del gemello `a_pass_to_a_man_with_a_free_pass_is_worth_more`
  cambia solo l'opzione successiva del compagno.
- **OI-114 (DD-44):** un contatto a terra vinto dal difensore può essere fallo
  se la banda delle stesse radici dell'ADR-042 arriva alla soglia derivata
  `foul_press_threshold_milli = 675` (StatsBomb, Liga 2020/21: il 6,6% delle
  pressioni finisce in fallo; sul band dei contatti vinti dal difensore, quella
  frazione dà circa 11 falli in più a partita). Gemello
  `a_press_that_wins_the_ball_can_still_foul`: stesso contatto vinto, cambia
  il controllo emotivo; letto come contrasto, lo stesso band resta sotto la
  soglia del contrasto. I cartellini seguono la Legge 12 dai fatti del contatto:
  i gialli salgono e restano una conseguenza dichiarata, non un numero tarato.

Con LUNA: il dito è il punto d'arrivo del pallone e le posizioni lette, il
contatto, la distanza, l'angolo, il tick; la luna è lo stesso passatore che
immagina un passo della lettura del compagno e lo stesso difensore che sceglie
come pressare, con l'arbitro che applica la Legge al contatto. Nessuna scelta
imposta, nessuna geometria forzata, nessuna regola del gioco cambiata.
Golden e semi rigenerati.

## 185 — Il pericolo è il valore dell'attore (DD-42, ADR-100)

D5, la nota "da fare separatamente" di ADR-099: `TeamView::threats` e
`TeamView::reaches` passano da `ds-match/src/state.rs` a
`ds-orchestrator/src/offball.rs`. Il dito è la posizione del portatore e
dell'uomo da marcare, il tick, la scena; la luna è lo stesso difensore che
legge quelle posizioni e pesa il valore dei tre spazi prima di scegliere
come chiuderli. Il motore fornisce i fatti (`ThreatFacts`), l'orchestratore
possiede il valore e la scelta, come già possiede la disponibilità della
frase dal SIM 184.

L'aritmetica è identica: nessun numero nuovo, nessuna formula cambiata,
nessun comportamento nuovo. Il gemello
`a_man_no_pass_can_reach_is_less_of_a_danger` migra con la regola e il nuovo
`the_danger_is_the_actors_own_reading` mostra che il valore dipende da ciò
che l'attore legge ed è deterministico. Golden e semi restano identici;
il bump segna il confine tra crate, non una taratura. `ds-reference 42..101`
non è ripetuto perché la misura non cambia.

## 184 — Il riconoscimento chiude la catena (DD-41, ADR-099)

Il maintainer, rilette le fasi in ordine reale con la luna del giocatore,
segnala che l'ultimo termine della latenza (`plausible_options`) è aperto dal
SIM 31 e che certe soste in TUI/2.5D non sono una sua lettura. Il dito è il
tick in cui una frase diventa disponibile, la posizione percepita dell'avversario,
l'id del portatore, la velocity zero del ricevitore; la luna è lo stesso uomo
che legge con i propri occhi, riconosce con il proprio bagaglio, sceglie fra ció
che ha in mano ed esegue con il proprio corpo. Cambia il quando e il come della
catena, non l'etichetta dell'atto, non la fisica, non la Legge.

**Forma B (scelta del maintainer, 21 nov).** Il +1 tick va sulla disponibilità
della frase nella lettura, prima dell'argmax: istantanea per il primo uomo
(istinto), un tick dopo per il secondo (pensiero). Non è un ritardo dopo la
scelta (forma A, respinta: ADR-044 §6 la vieta). OI-96 si chiude spostando il
conteggio delle opzioni dal tempo della latenza al riconoscimento, dove ADR-047
lo mette: nessun peso "istinto", nessuna lista di atti pensati, nessun numero
nuovo. Il riconoscimento sta nell'orchestratore (ADR-041/043, ADR-099 §4), che
restituisce al motore il tick di disponibilità e il motore scrive in
`offball_due`. Gemello `the_second_man_gets_the_phrase_one_tick_later`.

**Latenza da ciò che si vede.** `latency_scene` leggeva `crowding` e
`nearest_threat` da posizioni vere: un avversario fuori dal cono, mai visto,
rallentava la catena. Ora usa `self.perceived(actor, opp)`; la durata è la
relazione percepita (ADR-044 §6, ADR-062). Gemello
`an_unseen_opponent_does_not_extend_the_chain`.

**L'attore è chi ha il pallone.** `pick_from_space` eleggeva il portatore più
vicino e `decide()` subito dopo esigeva `carrier() == attacker`: nei tick in cui
il portatore non era il più vicino nessuno decideva. Ora `decide()` prende
`self.spatial.carrier()`, un fatto fisico, non un ruolo assegnato dal motore per
distanza. `nearest_in_squad` resta alla Legge e alla copertura.

**Il portatore fermo non aspetta il vecchio orologio.** Un ricevitore con palla
al piede, velocity zero e `carry_target = None` restava fermo fino a scadere il
`latencies` del possesso precedente. Se è fermo e senza destinazione, legge una
volta subito e la sua scelta (anche `Hold`) resta valida per il suo tempo; chi
corre conserva il ritmo. Sul seed 42 le finestre di sosta scendono da 83/1132
tick a 39/519; il reset per tutti dava 23 gol sul seed 4 ed è scartato.
Gemello `a_new_carrier_decides_without_waiting_for_the_old_clock`.

**Casa della regola del posto.** `choose_closed_place` e `ClosedPlaceScene`
si spostano da `ds-match/src/state.rs` a `ds-orchestrator/src/offball.rs`, il
read dell'attore (ADR-071 §5 nota 2026-11-21): nessuna formula o soglia cambia.

Golden e semi rigenerati (coppa 117 → 45, pareggio al 90' 44 → 44, decisivo
42 → 42, shoot-out 117 → 45); `the_match_never_freezes` rivalutato e verde.
`ds-reference compare --seeds 42..51`: gol 11.30 → 12.80, tiri 47.60 → 49.20,
in porta 21.50 → 20.90, passaggi 1384.00 → 1526.10; su 42..101, 5 misure su 22
in banda. Lo spostamento è caotico e non tarato: nessun numero di `balance/`
è statofitto a un bersaglio.

## 183 — La squadra che ha segnato va alla porta (DD-40)

Il maintainer guarda la GUI/TUI dopo SIM 181/182 e segnala «i giocatori non
vanno a festeggiare» e «sono molto disordinati tatticamente»; interrogato sul
momento, indica la **sosta dopo il gol**. Con LUNA: il dito è l'evento `Goal`,
il tick, il punto del varco, la sosta dichiarata e le destinazioni `sent`;
la luna è la stessa squadra che ha segnato che legge il gol, va insieme alla
porta e poi rientra nel blocco. Cambia il come esegue durante la sosta, non
l'etichetta dell'evento, non la fisica e non la regola del calcio d'inizio.

DD-37 aveva dato a tutti la posa, ma i frame GUI al tick 6137 mostravano i
centroidi fermi e, in TUI al minuto 11, la squadra era già sparsa verso i posti
del calcio d'inizio. La festa era solo un'animazione: spostare i corpi nella
vista inventerebbe posizioni assenti dal record (ADR-065 §5), quindi il
movimento va nel motore e la riserva SIM 180 dell'ADR-094 si chiude qui.

Ora `call_restart` riceve il punto reale del varco e manda a correre verso la
porta gli uomini di movimento della squadra che ha segnato, con lo stesso
`spatial.send` di SIM 150; il portiere resta alla sua porta e il battitore
avversario va al pallone. A metà della sosta dichiarata `goal_to_kick_off_s`
(nessun numero nuovo) `resume_restart` rimanda tutti nel blocco con lo stesso
`block_target`. P1 `the_side_that_scores_goes_to_the_goal_before_the_kick_off`
con il gemello della squadra che segna fuori casa.

TUI seed 42: gol ospite al tick 1384, distanza media dal punto 35,1 m → 6,4 m
al tick 1480, rientro a 61,9 m al tick 1785, ripartenza al tick 2007 (623
tick, 606 dichiarati più 17 per la Law 8). Gol di casa al tick 14426: 35,0 m →
2,5 m. Golden e semi rigenerati (coppa 81 → 117, pareggio al 90' 53 → 44,
decisivo 42 → 42, shoot-out 81 → 117). `ds-reference 42..51`: gol 11.90 →
11.30, tiri 49.30 → 47.60, in porta 23.70 → 21.50, passaggi 1374.70 →
1384.00.

## 182 — La porta si vede, il proprio pallone si riprende (DD-38, DD-39)

Dopo SIM 181 il maintainer guarda la GUI e segnala due cose: «invece di tirare
in porta passano ai compagni» e «alcuni behaviours veramente strani». Due fatti
veri sul setup esatto della GUI (`friendly::apply`, seed 42).

Con LUNA: il dito è l'apertura della porta, la copertura del portiere, il tocco
giocato avanti, la distanza dal piede, il tick; la luna sono gli stessi uomi che
leggono se la porta è lì e quanto vale ogni palo, e che, non avendo raggiunto il
proprio tocco, se lo riprendono sotto la suola. Cambia il come leggono e
eseguono, non il gesto già deciso, non l'etichetta dell'evento e non la fisica.

**DD-38 (ADR-096).** `keeper_visibility` azzerava la visibilità del tiro coperto
mentre la copertura era già nel suo valore (`chance`/`geometric`): lo stesso
fatto contato due volte. Un attaccante a 314 cm dalla porta (tick 32760) passava
perché il tiro migliore aveva visibilità 0.083 e i passaggi 0.333..1.0. Ora la
visibilità di ogni tiro è `goal_cone(pos, goal)` e l'helper è cancellato: il
portiere cambia il valore del tiro, non la vista della porta. Gemello
`the_keeper_changes_the_shots_worth_not_his_sight_of_the_goal`.

**DD-39 (ADR-097).** Un tocco giocato avanti (`held_cm = 0`, periodo 5.0 tick)
che i corpi avversari impediscono di raggiungere restava `due` per sempre:
pallone fermo 164 cm davanti al piede, `ball_at_feet = false`, nessuna nuova
scelta, portatore fermo 979 tick (50689..51668 su seed 42). Dopo due periodi
del proprio tocco, lo stesso `under_foot` di ogni pallone non raggiunto lo
riporta sotto il piede. Nessun numero nuovo, nessuna finestra generica: il
guardiano dei 979 tick e i due guardiani del primo tocco restano verdi.

`ds-reference compare --seeds 42..51`: gol 9.20 → 11.90, tiri 35.80 → 49.30,
tiri in porta 15.90 → 23.70, passaggi 1361 → 1374.70; la conversione resta
dello stesso ordine, l'aumento è caotico e non tarato. Golden e semi rigenerati
(coppa 55 → 81, pareggio al 90' 46 → 53, decisivo 50 → 42, shoot-out di soli
tiri 55 → 81). TUI seed 42 frame 75/76/77: il pallone cambia portatore senza
soste; frame GUI registrati al tick 50689.

## 181 — Il bersaglio della pressione è l'uomo, non il pallone (DD-36)

Il maintainer, guardando seed 42 in GUI/TUI: al 3'13" l'11 rosso ha la palla
e nessuno sembra contrastarlo; al 44' una palla nella metà campo resta ferma
mentre i giocatori le gravittano intorno. Con LUNA: il dito è la posizione del
pallone e del portatore, la distanza, il lato della propria porta; la luna è lo
stesso difensore che sceglie di affrontare l'avversario dalla fascia goal-side.
Cambia il come esegue la pressione, non il bersaglio (sempre l'uomo) e non la
fisica. `EventType::Pressure` resta geometria, non `OffBallAct::Press`.

Prima il pressore mirava `ball`: il pallone può giacere 188 cm davanti al piede
del portatore fermo, e il difensore si piazzava 150 cm oltre il pallone, a 340 cm
dall'uomo, senza entrare in duello. Ora `Press` resta in `to_the_ball` (stessa
contesa e intercettazione) ma punta la posizione del portatore e, dopo
l'intercettazione, il corpo prende la fascia goal-side con `goal_side_of`, la
stessa funzione di `Mark` e lo stesso `mark_gap_cm` già dichiarato. Nessun numero,
dado o latente nuovo; nessuna regola ridondante. Un portatore fermo per 17.347 tick
su seed 42 torna a muoversi.

Provare a usare il contatto (`2 * BODY_RADIUS_CM`) come gap faceva passare il
guardiano del dribbling ma lasciava il pallone libero 14.771 tick su seed 44:
scartato. Con il gap da marcatura il guardiano del dribbling passa da 5% a 6%
(82/1517 tick, 5,4%): il pressore goal-side fa riparare palla e arretrare, il
5% misurava una pressione da dietro. Gemello
`the_presser_takes_the_goal_side_of_the_carrier`; golden e semi rigenerati
(coppa 64 → 55, pareggio al 90' 49 → 46, decisivo 50 → 50, shoot-out di soli
tiri 64 → 55).

## 179 — Il filtrante va dove il portatore crede che il compagno arrivi (DD-35)

Il maintainer autorizza la sola modulazione del passaggio filtrante e il
ripristino dell'istinto di `goes_for_loose_ball`; gli altri quattro vincoli
della bozza ADR-094 restano sospesi (ADR-094, DD-35). Prima il punto
d'incontro del filtrante era spinto in avanti da un passo generico: due coppie
con intesa diversa, due letture e due passi diversi mandavano il pallone nello
stesso punto. Con LUNA: il dito è la corsa vista e il passo che se ne legge;
la luna è lo stesso portatore che decide quanto fidarsi di quel passo, con
l'intesa di coppia e la propria `game_reading`. Cambia il come esegue il
passaggio, non chi prende il pallone né la fisica (ADR-083).

`believed = (intesa * reading).mul_add(seen_pace - generic_pace, generic_pace)`
usa solo numeri già dichiarati: il passo visto (`scene.motion`), il passo
generico di `ThoughtBalance`, l'intesa e la lettura. Nessun dado (ADR-035),
nessun latente nuovo (ADR-047), nessuna lettura del latente altrui. Il primo
modello provato (`hidden.top_speed_now`) lanciava il pallone troppo lungo: su
seed 50 il pallone restava morto 1213 tick contro la soglia 1212 di
`no_restart_stops_the_match`; leggere il passo visto riporta il guardiano verde
senza toccare la soglia. In `offball.rs` il rank 0 torna a essere istinto; il
gate della vista resta.

A/B su partita vera, stessi semi: seed 42 filtranti 20 → 22, passaggi 1255 →
1195, gol 9 → 5, carrier oltre 90/200/500 cm 1398/160/0 → 1460/225/0, picco
373 → 370 cm; seed 53 filtranti 20 → 24, passaggi 1205 → 1251, gol 3 → 18,
carrier 1456/152/0 → 1393/173/0, picco 352 → 377 cm. La divergenza dei gol è
caotica, non tarata; l'episodio di 1141 cm del seed 46 (tick 38857, primo
tocco del ricevitore che corre via) resta aperto in OI-123, fuori da DD-35.
Gemello `the_pace_he_believes_of_the_runner_moves_the_through_ball`: stessa
corsa, cambia solo l'intesa/lettura e il passo visto. Golden e semi rigenerati
(coppa 48 → 64, pareggio al 90' 44 → 49, decisivo 42 → 50, shoot-out di soli
tiri 86 → 64).

## 178 — L'uomo più veloce della palla se la porta con sé (DD-34)

Il maintainer, dopo SIM 177: «risolvi i problemi in partita se é possibile
farlo nei limiti della visione del giocatore decidente». Ricostruendo i
carrier-gap ancora su seed 42 restano quattro episodi lunghi: il giocatore
prende una palla che rotola via a pochi cm/tick e corre a 50-60 cm/tick,
superandola di 3-5 m prima di frenare e tornare (tick 665-684, away slot 8,
gap 466 cm). SIM 177 copriva solo la corsa **contro** la palla in arrivo.

Con LUNA: il dito è il residuo `spill` della palla e la velocità dell'uomo, due
fatti che vede nel proprio cono; la luna è lo stesso giocatore che, appena
prende la palla, sceglie di portarla nel proprio passo invece di rincorrerla.
Cambia il come esegue il controllo, non chi prende il pallone né la fisica.

Nella branca `won` di `follow_carrier`, il primo tocco prende la direzione
della corsa se l'uomo corre contro il passaggio (`run·ball < 0`, SIM 177) **o**
se è più veloce del residuo di rotolamento (`spill < pace`), con `pace >= 1.0`.
Stessa formula del SIM 177 e del tocco successivo (`period_of`, `touch_speed`,
frenata dichiarata), lo confronto è tra due velocità in cm/tick: nessun numero
nuovo, nessun gate su `BallState`, nessun dado. Negli altri casi il tocco resta
quello del SIM 93.

A/B su partita vera, stessi semi: seed 42, portatore oltre 90 cm 2814 → 1398,
oltre 200 cm 1180 → 160, oltre 500 cm 257 → 0, picco 675 → 373 cm; passaggi
1224 → 1255, gol 8 → 9. Seed 53, 2390 → 1456 / 879 → 152 / 213 → 0, picco
723 → 352 cm; passaggi 1212 → 1205, gol 10 → 3. I tre episodi lunghi di seed
42 spariscono; resta un tocco pesante da fermo, fuori da questa decisione.
Guardiano `a_man_faster_than_the_spilled_ball_takes_it_with_him` (gemello:
stessa palla, cambia solo la velocità di corsa); verdi i due guardiani del SIM
177. Golden e semi rigenerati (coppa 49 → 48, pareggio al 90' 42 → 48,
decisivo 43 → 42, shoot-out di soli tiri 49 → 48).

## 177 — Il primo tocco è del ricevitore (DD-33)

Il maintainer, guardando seed 42 in 2.5D: «non puoi fixarle da solo questi
comportamenti? sono cose chiaramente che non succedono nella partita reale …
la cosa più importante è capire perché succede, non il fatto nello specifico».
Il caso indicato («secondo 16, 9 rosso») è stato seguito tick per tick: tick 152
il home slot 8 controlla a 2 cm con velocità −67 cm/tick; tick 153 il primo
tocco lancia la palla a 4,84 cm/tick in direzione `(0,7775, 0,6289)`, cioè
**indietro, da dove era venuta**, con `period = 0` e `held_cm = 0`; dal tick 154
al 188 l'uomo ha già la palla come bersaglio (`overdue`) e frena come in SIM
173/174, ma l'inerzia lo porta 651 cm oltre il pallone al tick 170, mentre la
palla si è mossa di circa 70 cm; tick 189 torna a 0 cm e la gioca lunga (tick
190, `z = 127`). 37 tick, 3,7 s, per un controllo che in partita vera non
esiste.

La causa non è il caso singolo: nella branca `won` di `follow_carrier` il primo
tocco usava solo la velocità con cui la palla era arrivata (`won`, `spill_of`)
e mai la velocità del ricevitore. Era un atto della palla, non dell'uomo:
ADR-058 §5 lo dichiarava «Declared, not read yet» e ADR-053 lasciava aperto
OI-123. Filtrando con LUNA: il dito è il vettore in arrivo e il residuo di
`first_touch`; la luna è lo stesso giocatore che arriva, con la propria corsa,
e che sceglie dove mettere il primo controllo. La tattica dà solo la tendenza;
l'agency è nel come esegue.

Quando corre **contro** il passaggio (`dot(velocità_uomo, velocità_palla) < 0`)
il tocco va ora nella sua direzione, con la stessa formula del tocco successivo:
`period_of`, `touch_speed` e la frenata già dichiarati. Chi corre insieme alla
palla la gioca ancora avanti, come dal SIM 93 (guardiano
`a_ball_won_is_taken_where_it_is`). Nessun numero nuovo, nessun gate di stato,
nessuna regola ridondante: la fisica libera resta identica (audit di
`advance_ball`, che non legge mai `BallState`).

La prima versione, che metteva la palla sotto il piede, è stata provata e
scartata: su seed 53 dimezzava i passaggi (1196 → 630) e i gol (6 → 2) e
perdeva la seconda pausa idrica. Quella finale su seed 42 porta i tick con
portatore oltre 90 cm da 8296 a 2864 (−65%) e quelli con palla ferma da 7558 a
1766 (−77%), in 65.000 tick invece di 63.200; su seed 53 i passaggi restano
1302 e le due pause ci sono ancora. Guardiano
`a_man_running_into_the_pass_takes_it_with_him` (gemello: stessa palla, sola
la direzione di corsa cambia). Il guardiano
`nobody_moves_faster_than_his_legs` ora esclude il cartellino rosso, che è un
posizionamento di Legge (`referee.rs` manda l'espulso a `y = -300`), non una
corsa: SIM 177 ha spostato un rosso sul seed 44 al tick 33883. Golden e semi
rigenerati (coppa 58 → 49, pareggio al 90' 49 → 42, decisivo 42 → 43).

## 176 — Il posto scelto continua a guidare chi è senza palla (DD-32)

Il maintainer, dopo SIM 175: «continuano ogni tanto a perdersi la palla e
soprattutto la cosa più strana è il movimento senza palla … sembra facciano solo
movimenti dondolanti come automi», «è piuttosto comune, lo vedo quasi sempre».
Prima di scrivere codice ho misurato la partita vera (seed 42, 61.800 tick).
Le 7 corse di 607 tick con pallone fermo e nessun portatore finiscono tutte con
un `Goal`: 607 = 606 + 1, i 60,6 s dichiarati di `goal_to_kick_off_s`. Le corse
di 49-65 tick finiscono con `ThrowIn`, `Corner` o `FreeKick`. Il pallone piantato
è la sosta legale, già ridotta in SIM 175 da 1510 a 606 tick; il rimettitore è
già a 41-45 cm dal punto, dentro i 90 cm di `at_ball`. **Non c'era un secondo
bug: nessun tempo di sosta è stato toccato.** Il dondolio invece era reale: il
peggior giocatore di movimento invertiva la marcia 92 volte in gioco aperto
(61.800 tick), con passi di 2-3 cm; i due portieri, 135 e 131, seguono il
pallone per mestiere.

Filtrando con LUNA: il dito è l'argmax senza memoria, il `hold_at` buttato via
appena il pallone non ha portatore e la frena sul punto esatto; la luna è lo
stesso giocatore che ha già letto la scena e scelto uno spazio, con la propria
fatica e le proprie gambe. Non cambia l'atto, né chi prende il pallone: cambia
il modo in cui esegue e il modo in cui tiene il posto.

Quattro modifiche, tutte con numeri già dichiarati:

1. `spatial::move_agents` usa il `hold_at` anche quando `carrier` è `None`: nel
   passaggio in volo il posto letto resta, invece della geometria della squadra.
   Il cambio di possesso lo cancella ancora (`set_home_in_possession`).
2. `set_hold_at` ignora una rilettura entro `BODY_RADIUS_CM` (45 cm): lo stesso
   spazio, non un nuovo bersaglio a cui frenare e ripartire.
3. `choose_closed_place` riceve la fatica (dichiarata in `[0, 1]`) e sconta il
   valore degli spazi lontani in linee del blocco (`depth_out_of_possession_cm /
   3`): il giocatore stanco copre ciò che può ancora raggiungere. La fatica non
   era prima nella scelta, solo nella velocità e nella latenza (ADR-030).
4. Chi arriva frena entro un corpo dal posto, non sul punto esatto: la
   separazione non può più spingerlo fuori e riportarlo dentro ogni tick. Il
   portiere è escluso perché deve seguire l'angolo con la palla; contese e
   `sent` alle riprese mantengono la loro frenata (ADR-088/089).

Guardiani: `fatigue_keeps_the_tired_man_in_the_space_he_can_reach` (gemello con
la sola fatica che cambia) e `a_near_reread_keeps_the_place_he_holds`. Su seed 42
le inversioni del peggiore giocatore di movimento scendono da 92 (61.800 tick)
a 60 (63.200 tick); i due portieri vanno da 135 e 131 a 141 e 137, perché devono
seguire il pallone. I 175 test di libreria restano verdi, compreso
`move_agents_moves_the_goalkeeper_toward_the_ball_segment`, che ha imposto
l'esclusione del portiere. Golden e semi dei test rigenerati con
`SIM_VERSION = 176`.

## 175 — Il passo della contesa è quello della partita vera (DD-30)

Il maintainer, guardando il seed 42 in 2.5D: «i giocatori non corrono quasi mai
alla loro massima velocità», «se ci sono statistiche in tal senso implementale».
La statistica è dichiarata in `fatigue.json`: `match_profile` arriva a
75 cm/tick per il 2% del tempo, contro un `effort_ref_cm_per_tick` di 90. Filtrando
con LUNA: il dito è `top_s`, lo stride, la distanza dal pallone e il picco
dichiarato; la luna è lo stesso giocatore, con la propria velocità e la propria
lettura della scena, a cui si toglie solo il passo che in partita vera non tiene
quasi mai. `contest_pace_share = 75/90` deriva dal dato, non è un numero nuovo.

In `spatial::move_agents`, ramo `contests`, la velocità è `top_s *
contest_pace_share`, anche per la frenata di SIM 172/173/174; `carrying` e
`in_time` non cambiano, nessuna scelta e nessun vincitore del pallone cambiano.
Il cap non vale per chi è `sent` al punto di una rimessa: si sta posizionando,
non contendendo una palla; il freno di SIM 173 lo ferma da sé. Con il cap anche
lì il battitore si fermava a 97 cm dal punto (fuori dai 90 cm di `at_ball`) e
il pallone restava morto 1510 tick su seed 42, oltre la soglia di 1212 del test
`no_restart_stops_the_match`. Guardiano:
`a_contest_runs_at_the_real_match_sprint_share` (prima 95 cm/tick contro 80).
Su partita vera seed 42: picco sopra palla ferma 90 → 74 cm/tick, campioni
oltre 60 cm/tick 13 → 7. Golden SIM 175 rigenerati; seme di coppa 63 → 54,
pareggio al 90' 63 → 46, shoot-out di soli tiri 63 → 54 (133 invariato).

Le decisioni delle Leggi del gioco giocato (kick-off, area, goal, fallo,
vantaggio, cambi, pause, shoot-out) sono ora tutte in `plugins/referee.rs`,
con `simulate.rs` che le applica: refactoring bit-identico, golden invariati
(DD-31).

## 174 — Anche il portatore frena sul proprio pallone (DD-29)

Il maintainer autorizza il fix del 12'30" e chiede il confronto sulla partita
vera. Filtrando con LUNA: il dito è il `Touch` scaduto in open play, il target
che torna al pallone e il portatore che arriva a 9 cm, lo supera di 255 cm e
torna indietro; la luna è lo stesso giocatore, con la stessa velocità e lo
stesso freno di ADR-088, che rilegge la scena dopo aver giocato il pallone. La
frenata di SIM 172/173 era solo per `sent` (le riprese): ora vale anche per
`overdue && carrier`, con la stessa formula e `BODY_RADIUS_CM = 45`, senza
numeri nuovi e senza cambiare chi prende il pallone. Pressing e contrasti su
palloni vivi restano a `top_s`. Il guardiano
`the_carrier_brakes_when_his_touch_runs_out` superava il pallone di 122 cm al
tick 18 prima del fix.

Confronto su partite vere seed 42: il 12'28"-12'30" non mostra più l'andata e
il ritorno; il golden cambia e i semi dei test vanno rifatti (65 primo pareggio
al 90', 111 primo shoot-out con soli tiri; il 77 del golden ha un `Pass` del
portiere, atto diverso lasciato aperto). Nessun dado, nessuna scelta cambiata.

## 173 — Il freno finisce prima dell'attesa

Il maintainer, vista la 2.5D del 172: «mooooolto meglio», ma «succede ancora
in misura minore», «è come se avessero troppo momentum». Filtrando con LUNA:
il dito è il battitore che, al calcio d'inizio del seme 42, entra nell'attesa
a 106 cm dal punto con 34 cm/tick ancora addosso, scorre fino a 15 cm oltre e
torna indietro; la luna è lo stesso giocatore, con la stessa velocità e lo
stesso freno, a cui il 172 aveva insegnato a frenare **verso il punto** (d=0),
mentre l'arbitro lo rilascia **all'arrivo** (90 cm = due raggi corpo,
`2*BODY_RADIUS_CM`). Il freno arrivava quindi a destinazione con il piede ancora
in moto: il resto lo faceva il momentum. Ora chi è mandato a una ripresa frena
fino a fermarsi **entro** un raggio corpo dal punto (`BODY_RADIUS_CM = 45` cm):
sotto radice c'è `(dist - BODY_RADIUS_CM)`, invece di `dist`; al rilascio il
battitore è a 41 cm dal pallone e non lo supera. Fermarsi esattamente sulla
soglia dei 90 cm era peggiorato: il corpo si fermava a 96 cm, appena fuori
portata, e la ripresa non partiva più. Nessun numero nuovo, nessuna scelta
cambiata: il gemello del burattino regge ancora. Il guardiano
`a_man_sent_running_stops_at_his_place` ora chiede che si fermi sul pallone e
non lo corra oltre.

## 172 — Chi corre a una ripresa si ferma sulla palla

Filtrando con LUNA: il dito è il battitore che si avvicina al pallone, lo
supera, torna indietro e lo controlla — davanti alla barriera, a ogni
punizione. La luna è lo stesso giocatore, mandato al punto del calcio
d'inizio: il suo corpo ha la stessa velocità, la stessa accelerazione e lo
stesso freno; cambia solo che nessuno gli dice di frenare. `send(id, spot,
true)` voleva dire «vacci correndo» (SIM 150: non teletrasportare più 22 uomini
in un tick), ma `move_agents` leggeva il `run` come `contests` e gli dava
`top_s` senza frenata: arrivava a tutta velocità (75-100 cm/tick,
`movement.json`), superava il punto anche di 6 m, rigirava e ci riprovava
finché l'arbitro non lo trovava entro 90 cm. Non è una
scelta sbagliata di chi prende il pallone: è l'esecuzione del gesto, quindi
regge il gemello del burattino. Ora chi è mandato a un punto frena per
arrivarci; chi contesta un pallone vivo resta a tutta velocità. Nessun numero
nuovo. Il guardiano è `a_man_sent_running_stops_at_his_place`, che prima
falliva (superava il punto al tick 14, dopo esserci arrivato all'11).

## 171 — Il tiro non diventa preciso perché è vicino (OI-100)

ADR-087/DD-27, dopo il tiro di prima tentato e tolto in SIM 166 di prova: la
distanza usata per lo scarto angolare del tiro non scende sotto gli 11 m della
misura di Shimotashiro e Shinya 2024. Non è un numero nuovo: è la distanza già
dichiarata della fonte di `error_ref_rad`. La mira del calciatore resta
`distanza × angolo` — ADR-054 dice che non può vedere la fase del contatto — e
la scelta del tiro non cambia; cambia solo dove arriva il pallone quando il
contatto avviene vicino alla porta. Valgono tiro e testata, non i passaggi.

Su 60 partite 42..101, sotto gli 11 m il tiro in porta passa dal 96% al 54,6%
(131/240; StatsBomb Liga 2020/21 46%); 11-18 m 52,2%, 18-25 m 43,9%, oltre
25 m 15,6%. Tiri 32,35 (30,65 al 169), in porta 13,20 (12,43), parate 18,85
(18,38), angoli 18,05 (15,90), rinvii dal fondo 20,07 ancora in banda: restano
6/22 metriche in banda. Il conteggio dei tiri non è stato ritoccato per far
tornare le metriche.

## 170 — La pressione mette alla prova il controllo del portatore (OI-132)

Il maintainer, guardando la partita e poi correggendomi: «il contrasto è uno
degli atti della partita, se è giusto il suo modo di leggere l'atto usalo per
tutto», «vale per tutti gli atti». La frase è una **regola di metodo**: il
criterio con cui ho giudicato il contrasto — ogni atto deve avere un riscontro
nella partita vera e, se manca, si corregge la logica della simulazione, non la
si avvicina statisticamente — vale per ogni atto. Non è un'architettura del
codice: non tutti gli atti passano da `resolve_duel`, e infatti qui si tocca un
solo atto.

Filtrando con LUNA: il dito è il piede che aspetta (`Jockey`), la palla che
rotola fra due tocchi, il contatore dei tick; la luna è lo stesso portatore,
con il suo `first_touch`, il suo dribbling e il suo corpo, che legge
l'avversario vicino e sceglie se e come tenere la palla. Prima il `Jockey` era
esplicitamente «no contact this time»: per quanto l'avversario pressasse, il
portatore non perdeva mai il pallone e `BallState::Possessed` restava stabile
finché non arrivava un `Challenge`. In partita vera non è così: un giocatore
sotto pressione sbaglia il controllo e la palla gli scappa via.

Ora la pressione di chi aspetta mette alla prova la risposta del portatore con
la stessa lettura deterministica del contrasto — `resolve_duel`, le forze e i
numeri già dichiarati in `duel.json`, senza dado, senza bonus e senza ordine
dalla panchina. Se il portatore non risponde, la palla **si scioglie dal suo
tocco** (`loose_from_contact`): non è un tackle di chi ha solo jockeyato. Il
cooldown è il `retry_ticks` già dichiarato. Cambia una causa sola.

Gemello: stesso portatore, stesso pressatore fermo nella stessa posizione,
stesse qualità; cambia solo il posto della palla — sotto il piede risponde e
tiene, rotolando davanti fra due tocchi non risponde e la perde. Se cambiasse
solo la vicinanza dell'avversario la causa sarebbe rigettata. Golden
rigenerati. `SIM_VERSION` 169 -> 170.

## 169 — Il portatore vicino sveglia chi correva alla palla (OI-131)

Il maintainer, guardando la partita in 2.5D: «continuano a orbitare intorno
alla palla», «chi arriva prima la prende e chi arriva dopo cerca di toglierla
all'avversario». Filtrando con LUNA: la luna è lo stesso giocatore, con le sue
capacità cognitive, psicologiche, fisiche e tecniche, che legge la scena e
sceglie; il dito sono l'atto, la distanza e il contatore dei tick. Il giocatore
che ha scelto `GoToBall` e trova un avversario con la palla a portata di contrasto
ha già la scena cambiata davanti, ma resta fermo dentro `offball_due` per la sua
latenza (0,7-3 s): corre verso la palla senza contrastare. Non gli manca la
lettura — `defend()` sa già scegliere pressare, entrare o tenere il posto con la
disciplina e il tackling — gli arriva tardi.

Lo sveglia esistente (`arrived`, il pressatore a portata rilegge subito) vale ora
anche per chi correva alla palla: stessa geometria già usata, `duel.reach_cm`,
nessun numero nuovo, nessun ordine dalla panchina. Cambia quando il giocatore
rilegge, non cosa gli viene detto di fare: dopo, sceglie da sé con le proprie
capacità. La tattica resta fuori da questo intervento: con un portatore,
`team_asked` e `obey` la fanno già arrivare.

Gemello: stessa squadra, stessa scena, cambia solo la portata del portatore —
dentro, il giocatore passa da `GoToBall` a `Press`/`Challenge`; fuori, resta alla
palla. Golden aggiornato. `SIM_VERSION` 168 -> 169.

## 168 — The through ball is seen (ADR-085, OI-129, DD-25, pass 2)

Il maintainer non vede mai il passaggio filtrante: il motore lo sceglie dal SIM
79, l'evento `Pass` porta il qualificatore `through: true`, ma nessun osservatore
lo nomina. Il censimento di `context/plans/L0-REPERTOIRE-CENSUS.md` ha mostrato
la catena intera: `read.rs::through_candidates` → `ChoiceKind::Through` →
`thought.rs` → `fact.rs` → `state.rs`, e nessun lettore in `ds-narrative` né in
`ds-gui`.

Ora il commento dice «threads it through» invece di «passes», il filtrante ha
importanza `Medium` e quindi appare nel commento anche senza «every touch», e la
2.5D gli dà un gesto proprio: la stessa spinta del passaggio, più lunga e con
un'inclinazione, mossa soltanto dal qualificatore già presente nell'evento.
Nessuna scelta, nessuna quantità, nessun numero nuovo: la luna resta lo stesso
giocatore che legge lo spazio e sceglie; cambia solo il dito che l'osservatore
vede. Nessun evento speciale con una regola propria.

Lo stato della partita non cambia: i tre golden restano identici, il bump è per
il comportamento dell'osservatore e per la versione del record. Test: P1/twin
`a_through_ball_is_told_and_shown` (narratore) e
`the_through_ball_is_drawn_differently_from_the_same_pass` (GUI, lo stesso
evento con e senza il qualificatore). `SIM_VERSION` 167 -> 168.

## 167 - The possession is the movement (ADR-084, OI-128, DD-24)

Il maintainer ha corretto la diagnosi guardando la partita: non si tratta di
"trattare la palla come propria" (DD-22) né di palla congelata (DD-23).
«Ottenere il possesso è quello che i giocatori fanno, ognuno con le proprie
caratteristiche e abilità; ed essendo azione non è qualcosa di diverso da sé:
invece di fare movimenti senza palla fa movimenti con la palla, controllandola
e tentando di tenerla nella propria disponibilità.»

Nel motore il controllo era un evento separato: `control_loose_ball` assegnava
`carrier = Some(id)` per una soglia di distanza, mentre `move_agents` muoveva
chi non aveva la palla con `OffBallAct` e chi ce l'avea con `follow_carrier`.
Due nature, due sistemi: il possesso arrivava al giocatore invece di essere
qualcosa che lui fa.

Ora, per una palla libera, chi ha scelto `GoToBall` — atto che
`choose_off_ball` ricava dalla sua lettura, dalla sua intesa e dalle sue gambe —
controlla la palla entro il `loose_control_radius_cm` già dichiarato. Il
movimento include il controllo; non serve un sistema di controllo a parte. I
giocatori restano solo giocatori dal primo all'ultimo minuto: cambia la
stanchezza e il fatto di avere palla o meno, non chi sono come atleti. Non
vogliamo burattini ma giocatori evoluti cognitivamente, che scelgono la cosa
giusta in base alla posizione in campo, alla disposizione di avversari e
compagni (o no): la zona è un fattore della scena letta, non una regola che
sceglie al posto del giocatore. Chi
non si muove verso la palla fa movimenti senza palla e la sua portata resta
`ball_reach_cm`, dalla sua reattività. Nessun numero nuovo, nessun dado, nessun
bonus, nessun copione. Il P1
`moving_to_a_free_ball_takes_it_without_a_separate_event` mostra il controllo
nato dal movimento; il gemello `the_reader_of_the_race_takes_it_and_the_other_holds`
cambia solo ciò che il giocatore vede della corsa e richiede che cambino l'atto
e il possesso. Golden e test mirati aggiornati.

## 166 - A free ball keeps rolling (ADR-083, OI-127, DD-23)

Il maintainer ha chiesto di correggere prima la palla: una palla è una palla,
rotola, rimbalza e rallenta; sono i corpi dei giocatori — piedi, teste,
ginocchia — a spostarla. Prima di SIM 166 il motore aveva tre palle: `Possessed`
senza fisica, `InFlight` con gravità e attrito, e `Loose` che, scesa sotto
1 cm/tick, aveva la velocità azzerata e la fisica spenta. La palla cambiava
natura per decreto, non per quello che succedeva in campo, e restava ferma
sull'erba con un giocatore che le passava accanto.

Ora `advance_ball` applica la fisica a ogni palla non in attesa di ripresa:
quella sotto un piede ha velocità zero e il passo la lascia dov'è; quella
libera, superata la soglia, passa da `InFlight` a `Loose` **senza perdere la
velocità residua** e continua a rotolare finché l'attrito dichiarato in
`ball.json` la esaurisce. Nessun numero nuovo, nessun dado, nessun copione: la
soglia, l'attrito e il rimbalzo restano quelli di sempre. Il gemello
`the_roll_is_the_balls_whatever_the_players_act` cambia solo l'atto di un
giocatore (GoToBall contro Hold) e richiede la stessa traiettoria: il rotolamento
è della palla, non di chi la guarda. Il P1
`a_free_ball_keeps_rolling_after_it_slows_below_the_stop_threshold` verifica la
discontinuità tolta. Il golden con istruzioni è stato rigenerato; quello
semplice e la tabella delle 22 metriche non cambiano.

## 165 - The place without the ball is read, not imposed

ADR-071 §5 was emended on 2026-11-19 (test del burattino): far from the ball
`read::soon` is zero at every candidate, so `choose_closed_place` weighed every
space zero and put the man on the geometric point of the block. Changing his
game reading, his intesa or his ability changed nothing — the place was imposed
from outside the player. Now, when no opponent can arrive soon in his zone,
his `game_reading` weighs the spaces of HIS OWN lane: he chooses the line,
while the lanes beside it keep being weighed by the game (`read::soon`). The
zone is still his discipline, so a wide man keeps his flank instead of running
inland; where in his lane he stands is his choice. The same reading already
drives where he offers with the ball; one man, one reading, two acts. The twin
`the_reader_leaves_the_given_place_the_blind_man_keeps_it` changes only
`game_reading` on the same scene and requires a different point. The ADR-070
pressing guarantee that SIM 164 (choice C) had lowered below its half-metre
threshold is green again, and the foot test `swapped_wide_men_strike_with_the_feet_they_have`
keeps its outside-foot strikes: neither assertion nor seed range was touched.

## 164 - The second man reads every high ball, if his legs can get there

ADR-077 gave a second man a contest on an opponent's aerial pass, but
`thought.rs` read the drop point only on the defending side, only for a pass in
flight, and gave the aerial `Target` a hardcoded `rank: 0`. The second man's
own race in `offball.rs::goes_for_loose_ball` — reading, first touch, and his
own pace — therefore never started for him. ADR-081 (DD-19, choice C) removes
the side and pass-only restriction and carries the real rank: the nearest man
still goes, the second goes only when he can reach it, both sides, every high
ball, no new number. The kick and the clearance are facts in what each player
perceives; who chases is still each player's own reading of the ball and his
legs. The twin `the_second_reaches_the_near_ball_and_holds_for_the_far_one`
proves the reach test; `swapped_wide_men_strike_with_the_feet_they_have`
keeps the sample that guards the foot decision.

## 163 - The dead ball is seen, and the kicker is chosen by role

A restart is now a fact in what each player perceives: `OffBallScene.dead_ball`
is derived by `thought.rs` from the ball being held for the kick, not from an
order to press or to stand. While it is true nobody presses, challenges,
intercepts or runs to the ball: the kick is unchallenged until it is taken
(Laws 13-17). Marking, covering and holding stay in the choice, each still
weighed by that player's own saliences, reading and discipline: the fact is
shared, the interpretation is his.

`restart_taker` no longer gives every restart to the nearest man. A goal kick
belongs to the keeper, a throw-in to the wide man or full back on that side
(nearest of the two), a corner or a free kick to the best deliverer among the
outfield men within the already-declared `TAKER_RADIUS_CM`, a penalty to the
best finisher; a kick-off still goes to the nearest. The choice uses the
candidate's own technique and better foot through `set_piece_taker`. No new
numeric parameter, no team script, no die. The kicker still walks to the
ball (BEHAVIOURS 25).

A header away from his own goal now carries the same retry as a first-time
clearance: a ball that came back off the crosser's body had let the same
defender head it twice in two ticks, breaking `one_clearance_per_act`.

Measured on 60 real-match-filtered seeds (42..101), engine 162 -> 163:
passes 1282.9 -> 1250.1 a match (real 887.7), ground duels 179.5 -> 172.1
(real 182.6), penalties 0.00 -> 0.02 (real 0.33). Scene counts: passes
forward/lateral/back 27/50/23 % -> 26/51/23 %, ball in the attacking side's
own half 71.0 % -> 71.2 %, free slow ball taken at once 69.3 % -> 69.1 %,
widest two 40.3 -> 40.2 m, two deepest 31.2 -> 31.3 m, ball behind the
carrier 25.5 % -> 25.7 %. Goals went 4.82 -> 5.20 (real 2.68): unchallenged
restarts and a better foot produce more attempts, which is a calibration
matter for L3, not a reason to make the kicker foul the Laws. Twin tests in
`ds-orchestrator::offball` and a role/foot test in `set_pieces`.

SIM_VERSION 162 -> 163. **Golden changed** (all three).

## 162 - The opponents' lines and the space between them

Players now read the opponents' three lines — the first line of pressure,
the midfield and the last — from the depths of the men they see
(`read::lines`, `thought.json`'s `same_line_cm`). Four repartitions read as
three lines: the two nearest merge. A pass is worth more for every
opposing line it goes past towards their goal (`line_break`,
`line_break_milli`): a line broken is a line the opponents must recover.
A backward pass breaks nothing. Off the ball, the place a man holds is
valued the same way (`TeamView::mate_ball`, `choose_place`), and the spaces
between their lines are now among the depths he considers, inside his
zone: the pocket in front of their last line is a place to receive and
turn, not a point in a grid. The half-space is where the men whose role
starts there (the mezzala, the second striker) take that pocket; no flat
bonus is given to a corridor, so a centre back is not pulled out of the
spine. Twin tests in `ds-orchestrator::read` and `ds-match::state`.

`line_break_milli` is declared at 60, not larger: at 70 the single presser
in ADR-050's scene became a line worth breaking and progression under
pressure weighed more, not less, breaking that older twin. The smaller
value keeps both causes — pressure still costs, a broken line still pays.
The lines are read once per scene, not once per pass candidate.

Measured on 60 real-match-filtered seeds (42..101), engine 161 -> this
change, scene counts: passes forward/lateral/back 24/53/23 % -> 27/50/23 %
(real 31/53/16 %); ball in the attacking side's own half 73.8 % -> 71.0 %
(real passes from own half 52.9 %); free slow ball taken at once 73.8 % ->
69.3 %; block without the ball, widest two 40.4 -> 40.3 m (real 37.0),
two deepest 32.7 -> 31.2 m behind the ball; ball behind the carrier 25.1 %
-> 25.5 %. Jumps longer than the fastest legs went from 0.1 to 0.0 a match.
The four residual single-step jumps seen during the tuning were
substitutions entering at the halfway line, present before this change.

SIM_VERSION 161 -> 162. **Golden changed.**

## 161 - 10d: the keeper's depth is read, not fixed

The keeper's advance off his line is now his own game reading times the
geometric closeness of the ball (`keeper::line_depth_cm`). Before, every
keeper advanced the same `advance_cm * t`: the depth had no cause. A keeper
who reads the play claims space; one who does not stays ready on his line.
Twin test in `ds-calculator::keeper`.

SIM_VERSION 160 -> 161.

## 160 - OI-102: the second man on aerial balls

On an aerial ball (defending side), rank 1 players also contest if they
can reach the landing point, not only the nearest (rank 0). Before, only
the nearest contested, so aerial duels were rare (10.4 vs 38.1 real).

SIM_VERSION 159 -> 160.

## 159 - OI-100: shot height is now a choice (radice rimasta)

The shooter no longer only strikes low: a high shot (contact_height 1.0) is
now a candidate alongside the low one, for each post and power. The high
shot is harder to place (geom 0.85) and riskier (risk 0.1), so it wins only
for a shooter who reads the game and accepts the risk. The lob remains a
separate candidate.

SIM_VERSION 158 -> 159.

## 158 — OI-98: the offside line is read with an error (perception)

The offside line is no longer read perfectly: a player with poor game
reading sees it further forward (up to ~2 m of error at game_reading 0,
none at 1.0), so he may offer beyond the real line and be caught offside.
A good reader reads it as it is. The engine's actual offside check
(Law 11) still uses the real positions.

SIM_VERSION 157 -> 158.

## 157 — A3, strato 3 (third piece): intesa grows from completed plays

A completed pass between two teammates raises their mutual intesa (0.1, capped
at 1.0): a pair that plays together learns to predict each other, so the
complementary movements and the calls on a loose ball become more likely as
the match goes on. A pair that has just met keeps its baseline.

SIM_VERSION 156 -> 157.

## 156 — A3, strato 3 (second piece): complementary movements (the place with the ball)

With a teammate on the ball, the man who holds his place no longer weighs
only where his teammates stand now: if he knows them (intesa high enough),
he also weighs where they are going (their velocity, a few ticks ahead),
so a space a partner is about to occupy counts as crowded for him. A pair
that knows each other does not end up in the same corridor: one comes
short, the other stays long. A pair that has just met keeps the old
behaviour (current positions only).

SIM_VERSION 155 -> 156.

## 155 — A3, strato 3 (first piece): the call on a loose ball

With a loose ball between two teammates, the one who knows his partner
yields it: the nearest always goes, and a teammate whose intesa with him is
high enough trusts that he will get there and holds his place (he covers)
instead of going for the same ball. A pair that has just met may still have
two men on the same ball, or both holding back, as before: that is what a
pair without intesa does. The intesa is keyed by (actor, partner), starts
at a baseline for every pair on the same side, and is the measure of how
well one predicts the other (decision BB). This is the first piece of
strato 3 (BEHAVIOURS.md): the call that avoids two men on the same ball.

## 154 — A3, ADR-071 §5: the place without the ball (the space he closes)

With an opponent on the ball, the man who holds his place no longer goes to
the point the geometric block computes: the block stays as his role's zone
(ADR-071 §4), and inside it he chooses the space that takes away the most
danger, by the same reasoning as with the ball (§2-3). Candidates: his
corridor and the two beside it, his line and one line ahead or behind (a
line, a third of the block's depth without the ball). A space is dangerous
as much as it is worth to the opponents (`read::space_value` towards our
goal) and as soon as one of them can be there (`read::soon`, the carrier
excepted: his own space is pressing, another act); a space a teammate is
nearer is his to close, not mine. Far from the ball no opponent is in the
zone soon and he keeps the coach's place: the others close in, they do not
chase it (`BEHAVIOURS.md` 19); the danger is weighed goal-side (20), and
the coverage behind the presser stays its own act (`Cover`, 18).
Regenerated goldens. Measured on the ten seeds 42-51 (`ds-reference
scene`): the block without the ball, the two widest men 39.2 → 40.0 m
(real 37.0, Metrica sample games 1 and 2), the two deepest 33.3 → 32.7 m
behind the ball; the passes forward / lateral / back 23/53/24 → 25/52/23
(real 31/53/16); the two most advanced ahead of the ball 26.8 → 27.2 m;
nothing else moves (jumps 0, the ball in the own half 74.4 → 74.7%). The
width moves 0.8 m away from the real 37.0: the block's drawing is the
coach's, and this choice does not touch it.

## 153 — A3, ADR-072: one act, one event (the cross is a pass)

The same struck ball no longer emits two events (`Pass` + `Cross`):
`tell_cross` is gone, replaced by `compute_cross` (which only returns
the qualifier, it does not emit). `TrajectoryKind::Pass` carries
`cross: bool` and `foot: Option<ds_orchestrator::Side>`; `classify`
emits `"cross": true|false` on every `Pass`, and `"foot"` only when
cross is true. `ds-narrative` says "crosses" when `"cross": true`.
`EventType::Cross` stays in the enum, deprecated, not emitted from 153
on. Golden regenerated; no other behaviour changes.

## Current SIM_VERSION

```
SIM_VERSION = 154
```

`SIM_VERSION = 1` was the initial baseline (no code). 2 removed `CauseTrace` and `xg` (ADR-024). 3 removed `template_key`/`params` from `NarrativeEntry`. 4 removed four gusci fields from `MatchEvent`/`MatchSave`. 5 replaced `player_id` with `actor: Option<ActorId>` and added `KickOff`/`SecondHalf` (ADR-026). 6: two halves with separate clocks, `Half` field, four-phase loop, fixed-point stoppage time, boundary events emitted (ADR-026 phase 2). 7: stoppage contributions calibrated (goal 60s, foul 20s, duel 10s). 8: slice B2 — the acting agent comes from space (`pick_from_space`), `Tackle` emitted when a duel is won by the defender, `SpatialState.carrier` tracked, stream names with `match_id` (OI-22). OI-22 closed: RNG stream names now carry `match_id` (`match:{id}:actions`, `match:{id}:action_kind`), so two matches with the same world seed no longer share the engine streams. The current value is the reference against which all future bumps are measured.

9: C.1 — `Pressure` emitted beside an action (Pass, Shot, Duel), calculated with `resolve_influence` (03-entities-minimal §2.9) on the ball's point; the most influential opponent within `space_threshold_cm` presses. A step toward the outcome (ControlError, later). Golden hash regenerated.

10: A1 — one RNG stream per noise consumer in `pcfm-hidden` (`hidden:{match_id}:{latents,duel,shot,pass,save,injury,set_piece,pressure}`), replacing the single shared stream. Adding a new consumer no longer shifts the outcomes of the others (`00-foundations` §4, `01-rng.md` §2.3). Closes OI-39. Golden hash regenerated.

11: F.3-b2 — the ball crosses the goal line as a geometric fact: `Shot` carries `goal` and `on_target` (was: `goal` only, on the shot). `Goal` is emitted only from a `Fact::LineCrossing { kind: GoalLine }`, not from the shot. `Pass` is emitted for the first time (was: `resolve_pass` called, no event). `Fact::Trajectory` carries `kind` and `success`; `Fact::LineCrossing` carries `actor` and `point`. `classify` extended; six unit tests. Golden hash regenerated. Behavior change: a `Pass` event appears in the stream, and `Shot.goal` now comes from the line crossing. F.3-b1 (BallState 3D) did not bump: no behaviour change (golden identical).

12: F.4-a — agents move every tick (MD-09, MD-10, MD-12). `move_agents` reads the ball's offset from the pitch centre, translates every formation slot by a declared fraction (30%), deviates the nearest agent to the ball, steps each agent by at most `MAX_STEP_CM` (120), and enforces a minimal separation (`BODY_RADIUS_CM` 45). No RNG in the movement (S1): the variation comes from the inputs. Golden hash regenerated; the match now moves, so the hash changes. The separation pass is a `Vec` snapshot (same operations, same order, no per-pair map lookup): same behaviour, 28 s -> 5.8 s on the golden.
13: F.4-c — the ball flies and the possession resolves at landing, not at launch. `Pending` (MD-37) with a mandatory expiry (`FLIGHT_TICKS` = 15) and the expected possession; `other()` computes the opposite without mutating; the check at the top of `tick` applies the possession when the pending is due. In the pass and shot branches the carrier is released at launch, `set_ball_in_flight` is no longer a no-op, `advance_ball` runs. Declared simplification: no interception yet (OI-37), the ball lands at the destination or out; the goal stays the die's outcome (MD-20). Golden hash regenerated (behaviour changed: the ball moves between ticks).
14: F.4-c2 — the situated decision replaces the global die (MD-01, MD-43). `choose_action` is geometric and reads `visible_to`: shoot if in range, pass to a visible teammate with space, duel otherwise. `action_kind_roll` and `action_kind_stream` are removed: no die in the choice, the variation stays in `resolve_*` and in the degraded image. The pass aims at the actual receiver, not a fixed point. The separate foul branch is removed (the foul is a duel outcome, OI-48). `action_roll` (whether an action happens in the tick) stays as declared pacing; finding tracked, density is MD-16. Audit test (MD-43): a pass receiver must be visible from the attacker. Golden regenerated.
15: F.4-c2b — the cone does not filter the pass target ("filtro partita vera", A, 2026-11-04). In `choose_action` the passer considers every teammate with their freedom; the cone (`visible_to`) is removed from the choice. A pass target is a *choice*, not passive perception: a player looks at the teammate he serves. The cone's real consumer is the interception of a pass by an opponent the passer did not see, which is OI-37. Declared: the cone is not read by the tick today (audit test: a mate behind the passer, outside the cone, is still chosen). Audit test replaced: the previous tautology (`!x || x`) was a D5 leak and is removed. Golden regenerated.
16: F.4-d — interception: the cone filters the opponents, not the target. At launch the passer saves the opponents outside his cone; `find_interceptor` (integer point-to-segment distance, `isqrt`) checks if one is within `INTERCEPT_RADIUS_CM` (200) of the pass line. The pending carries `interceptor` and `passer`; at resolution `Interception` is emitted with `from`/`to` and the possession flips. Shot passes `None` (a shot is not intercepted in this model, declared). The cone now has its real consumer. Golden regenerated; measured on the demo: seeds 44-46 produce 8/23/17 interceptions, seed 42 none (geometry-dependent, not a bug).

17: F.3-b3 — shot schema, goalkeeper, goal from the crossing. `ShotOutcome` loses `goal` and gains `to` (3D arrival after the deviation), `initial_velocity`, `on_target` (geometric, from the goal mouth), `shot_type` (choice, derived geometrically). `ShotContext` gains `difficulty` (pressure, computed in `state.rs`); the die widens its deviation bidirectionally, so a strong finisher deviates less. `SaveContext` gains `point` (the crossing point); `resolve_save` unchanged in formula, but now called from `check_shot_crossing` when the ball crosses the goal plane inside `GK_REACH_CM` of the goalkeeper. `Fact::Trajectory` is a struct variant with the new shot payload; `Fact::GoalLineCrossing` is emitted via `classify`. The goalkeeper (slot 0) is anchored (`GK_LINE_OFFSET_CM` 200 cm, `GK_ANCHOR_CM` 250 cm): `from_squads` places it on the goal line, `move_agents` skips it. Declared derogation MD-38 stays: ball out of the goal mouth → possession flips, no restart. **Not observable in seeds 42–51** (0 shots in the golden range); the behaviour is exercised by the new `functional_l0` test that forces a shot. Bump declared for hygiene (D5: a bump too many beats a silent schema change). Golden hash regenerated (identical, 0 shots in the range): the bump declares the schema change, the golden cannot see it.

18: F.5 — Pressure is a coverage relation, not a tick event. `Fact::RelationEnterExit` is emitted when an opponent enters (or exits) the coverage of the carrier (`space_threshold_cm` around the ball). The narrator classifies `entered: true` as `Pressure` with `from`/`to`; `entered: false` emits nothing (the pressure ended, nothing new started). This removes the "Pressure beside the action" bridge (decision 2026-11-02): `Pressure` is now driven by the coverage change, not by the action. `update_coverage` is called once per tick after `move_agents`; the set is persistent (`coverage`, `coverage_possession`), discarded on possession change. **T1-bis amended:** `PositionChange` and `HeadingChange` are not `Fact`; they are tick-record fields (IDEA-INFRA-018) read as background by 2D/3D, not classified into events. The only movement fact with an event consumer is `RelationEnterExit`. **Arbitrary parameters of F.3-b3 declared here, as the Plan requires:** `GK_LINE_OFFSET_CM` 200, `GK_ANCHOR_CM` 250, `GK_REACH_CM` 200, shot deviation `spread_base` 400 × (1 − finishing), `spread_diff` 2 × difficulty, goal mouth `goal_half_width_cm` 366, `goal_height_cm` 244, `INTERCEPT_RADIUS_CM` 200, `PASS_FREEDOM_CM` 800, `SHOOT_RANGE_CM` 2500, `MAX_STEP_CM` 120, `BODY_RADIUS_CM` 45, `FLIGHT_TICKS` 15, `action_probability` 0.0005. Golden hash regenerated (Pressure now depends on coverage changes, not on actions).

137: past the man is on the other side of him (OI-103, delegated DD-5). Partita vera: *how do the actors recognise it?* A carrier takes on the man in front of him, between him and the goal, and shields the ball from one at his back. *What does it depend on?* Where the man stands: beating him takes the carrier to the other side of him, the space he was closing. No new number: a fixed 4 m nearer the goal, wherever the man stood, is removed. Before, beating the man was always worth the space 4 m ahead, and the carrier took him on in 51% of the contacts at his feet (Wyscout Serie A 2017-18: 18.6% of the ground attacking duels are take-ons). Now 35% (seeds 42-101). Test `a_man_in_front_is_worth_beating_one_behind_is_not`. ds-reference seed 42-101, 136 -> 137: take-ons won 32.9 -> 25.9 (still above its band, 21.7), goals 2.23 -> 2.55, shots 19.2 -> 23.1, on target 10.2 -> 12.1 (just above its band, 11.9), saves 11.2 -> 12.7 (on its edge), own goals 0.43 -> 0.33 (in band). Goldens regenerated, glibc and musl.
138: the bench in the match, step 1 (decision T). Partita vera: *how do the actors recognise it?* The manager on the touchline decides a change while the match is on; the fourth official raises the board at the next stoppage. *What does it depend on?* The minute he decides it and the ball going dead (Law 3). *Who produces it?* The manager. A substitution ordered during the match is an input with its tick, `Order::Substitute` in the coach's instructions (as an intention, ADR-059): told at its minute (`Instruction` with `off` and `on`), made at the first stoppage after it by the bench plugin, after the planned ones and before those the coach decides himself. Law 3 is now kept for all of them together: what is already due at a stoppage counts against the limit, and the coach does not change a man already in an order (before, planned changes and the coach's own in the same stoppage could go beyond five). Same inputs, same match for the three goldens (unchanged); the bump is for the Law 3 edge case. Tests `a_substitution_ordered_live_is_made_at_the_next_stoppage`, `no_more_substitutions_than_the_laws_allow`; in the GUI the bench post-it gives the order at the tick on the screen and the match goes on from there, the same up to it (`a_change_from_the_bench_keeps_the_match_up_to_now`).
139: the quick instructions and the transition choices (ADR-059 step 3, decision T step 2). Partita vera: *how do the actors recognise it?* The coach shouts "press!", "everyone up!", and the team's reaction when the ball changes side: the counter-press or the drop back, the counter-attack or the pause to keep the ball. *What does it depend on?* The coach's choice, at its minute; how each player follows it is his discipline, as for the intention (ADR-059 §4). *Who produces it?* The coach, through the block his players read. Two new orders with their tick, `Order::Quick` (more pressing, everyone forward, on or off) and `Order::Transitions` (on losing the ball: shape, counter-press, drop back; on winning it: shape, counter, keep), each told at its minute. They move the block on the board, never a player's number: more pressing engages 8 m nearer the ball, everyone forward stands 12 m higher with the ball, and for 6 s after the ball changes side the counter-press keeps the line up behind the ball, the drop back takes the low block, the counter pushes up, keeping holds back (steps arbitrary until L3, the 6 s from the counter-pressing rule of Guardiola's Barcelona; `balance/SOURCES.md`). Without these orders the match is unchanged: the three goldens identical. Twin test `the_quick_instructions_and_the_transitions_move_the_block`; P1 `quick_instructions_take_effect_from_their_tick` (the same match up to the order, told, replayed).
140: the coach who reads the match, on by default (decision Y; DD-7). Partita vera: *how do the actors recognise it?* A coach looks at the score and the clock and changes someone: losing, earlier and for an attacker; winning, later and for a defender. *What does it depend on?* The score, the minute, his men's fatigue. *Who produces it?* The coach. His first, second and third changes come at the minutes the real coaches of Serie A 2017-18 made them with that score (grade 2, `balance/coach.json`: losing 58', 67', 79'; level 59', 73', 84'; winning 66', 76', 84'; fitted on half the matches, held out on the other half), at a stoppage: the most tired outfield man off, for an attacker when losing, a defender when winning, a man of the same role when level; after the third, nobody of his own accord. It replaces the old rule (SIM 118: a tired man for a faster fresh one, off by default). On by default (`Squad.coach_reacts`); the friendly match switches it off for the manager's own side. The contract (`ds_match_api::World`) gains `goals` and `on_pitch`. Not yet: moving the block with the score (DD-7, OI-120): tried, pushed up when losing it took the goals from 2.60 to 3.95 a match. ds-reference seed 42-101, 139 -> 140: goals 2.55 -> 2.60, shots 23.1 -> 25.4, on target 9.2 -> 9.4, corners 13.4 -> 13.8, saves 12.7 -> 13.2; 11 of 22 in band. Goldens regenerated, glibc and musl.
141: the knock that forces a substitution (decision Y, ADR-066, DD-8; closes OI-110). Partita vera: *how do the actors recognise it?* The man stays down, the referee stops play, the bench must change him. *What does it depend on?* The force of the contact and the body of the man it hits. *Who produces it?* The challenge. A foul's approach speed (the fact Law 12 reads for the card) x (0.5 + the fouled man's injury_proneness), at or over `knock.json` knock_force_milli (850): he cannot go on; the `Injury` event at its tick; at the stoppage he leaves, replaced by a man of his role, or any outfield man, or carried off if no change is left (Law 3). No die. Grade 2 on the effect: the real first-half forced substitutions (Wyscout, 0.19 a match on the even ids), fixed on the engine's 120 even seeds, held out on the 120 odd (0.19 against the real 0.25; the two real halves differ by 0.06). injury_proneness has its consumer. Tests `a_knocked_man_leaves_the_pitch`, twin `the_same_contact_knocks_the_frailer_body`. ds-reference 42-101, 140 -> 141: goals 2.60 -> 2.67, saves 13.2 -> 12.6 (in band); 12 of 22 in band. Goldens regenerated, glibc and musl.
142: the free board (decisions AL and AG, decision T). Partita vera: *how do the actors recognise it?* The coach draws his team on the board, with the ball and without, and changes it during the match: two wide men swapped, a back three, a forward dropped deep. *What does it depend on?* The coach's drawing; each player follows it with his discipline (ADR-059 §4). *Who produces it?* The coach. The fixed 4-4-2 grid of `shape.rs` gives way to `ds_model::Board`: each slot's place in the block (depth and width, the representation the block already read), a drawing with the ball and one without, both required, and optionally one per third of the pitch between which the place moves continuously with the ball. The place belongs to the slot, so the man who comes on takes it. `Order::Board` changes the whole board from the tick after it (AG); the swap of two places is a case (`Board::swapped`). The default board is the 4-4-2 of before: the three goldens identical. Tests `the_default_board_is_the_four_four_two`, `between_the_thirds_the_place_moves_continuously`, `swapping_two_places_exchanges_them_everywhere`, P1 `the_board_changes_from_its_tick`. Not yet: the responsibilities in each drawing (ADR-059 step 4), the foot as the player's choice (ADR-031, the twin of the inverted wingers), the board in the GUI.
143: the foot is the player's choice (ADR-031 applied; the maintainer, 2026-11-16). Partita vera: *how do the actors recognise it?* The right-footer on the left who cuts inside to shoot; the back on the wrong side who crosses with his weak foot. *What does it depend on?* Where the ball is on his body, his two feet, the time a touch takes and the man on him. *Who produces it?* The player. The ball is on the foot of the side where it is (the outside foot in the wide thirds, in front of him in the middle third), and every kick is struck with that foot (`ShotContext.foot`, `PassContext.foot`): before, the shot took the foot of the side it aimed at and the pass the receiver's side on the pitch (one team had the other foot). With the ball on his weaker foot he also imagines a touch across his body (two body radii) onto the stronger one (`ChoiceKind::Shift`): the shot after it with his better foot, with the opponents that much nearer, weighed by whether he keeps it through the touch (the race of ADR-052 §5) against the ball lost there; the same measure as every option. His shot's worth now reads his own spread (the foot, the power, the pressure; ADR-052, he knows his own foot), not an average shooter's. Shot and cross events tell the foot. Twin `with_space_he_moves_it_onto_his_strong_foot_under_pressure_he_strikes`; match twin `swapped_wide_men_strike_with_the_feet_they_have` (the wide men swapped at the 30th minute: every kick with the foot on the ball's side or the stronger after a touch, never the weaker by choice). The touch is rare in these matches: in 6 matches it was an option 614 times and worth more than the best one 3 times (OI-121). ds-reference 42-101, 141 -> 143: goals 2.67 -> 2.18, shots 25.2 -> 20.9, corners 13.4 -> 9.4 (in band), saves 12.6 -> 10.5; own goals 0.48, just out; 11 of 22 in band. Goldens regenerated, glibc and musl.
144: the responsibilities on the board (ADR-059 step 4, decision T; the maintainer: the individual instructions are each player's responsibilities in the drawing, the change of formation is a new drawing). Partita vera: *how do the actors recognise it?* "You go out on their centre-back", "you stay back", "you attack the depth", "you come short". *What does it depend on?* The coach's ask and the player's discipline. *Who produces it?* The coach, through the player who follows it as he is. Each slot of the board has an optional responsibility with the ball (run, offer, stay) and without it (press, cover, mark, stay); `Free` is his own reading. He chooses among his acts as before, unless the act asked is among them and what he gives up for it is within what his discipline accepts: the asked act worth at least `1 - discipline` of his best (`offball::obey`; no number of its own). The fully disciplined always does it, the undisciplined only when it is his own choice too. The responsibilities change during the match with the board (`Order::Board`): no order of their own. Without responsibilities the choice is the same as before: the three goldens identical. Twin `the_disciplined_does_what_he_is_asked`; P1 `a_striker_asked_to_run_stands_farther_ahead` (over six matches). Not yet: the board in the GUI (the click on a token, ADR-059 step 5).
145: the starting state of each player (decision AB, ADR-067, DD-9). Partita vera: *how do the actors recognise it?* The man back from injury who fades after an hour, the one who played three days ago and has no burst, the one off a long trip who gets the simple choices wrong. *What does it depend on?* The week before the match. *Who produces it?* The week (L1); the match receives it. Each state read by a consumer the engine already has: the physical freshness as the starting fatigue; the cognitive freshness as a mental fatigue (`cognitive_fatigue`) added to the physical in slowing the decision, at most 1; the condition weighing his endurance in the fatigue plugin; the muscle damage (`damage`) a fatigue the match never gives back, half-time included. The defaults leave the match as it was: the three goldens identical. Twins `out_of_condition_he_tires_sooner`, `the_mental_fatigue_he_brings_tires_his_mind`; P1 `the_damage_he_brings_stays_all_match`. The plugin contract gains `condition`.
146: the channels from the bench (decision AP, ADR-068, DD-10; supersedes ADR-059 §2). Partita vera: *how do the actors recognise it?* The coach shouting "higher!", the full back who turns to the bench and sees the gesture, the captain handed the note during an injury, the team round the coach at a water break. *What does it depend on?* How much there is to say and when play stops. *Who produces it?* The coach, the player who looks at the bench, the referee who stops play. Every instruction has a channel (`Instruction.channel`): by voice to all from the next tick; by a gesture when one of his players on the pitch has the bench in his cone of sight (and passes it on); at the next pause after it was given (a knock, half-time, a water break, a substitution: the man coming on brings the note). The game chooses when the manager does not: quick instructions, transitions, intention and substitutions by voice; a board moving at most two places by voice, more at a pause. Water breaks as a rule of the competition (`MatchRules.water_breaks`): at the first stoppage after each minute, a pause, three minutes added (FIFA's cooling break). The instruction's fact is told when it reaches the team. A recommended module (`Match.channels`, on): off, everything by voice, as before. The three goldens identical (their instructions go by voice). Tests `a_new_board_waits_for_a_pause`, `by_voice_at_once_by_a_gesture_when_they_look`, `the_water_breaks_stop_play_and_add_time`.
147: the small steps (decision BJ, ADR-069, DD-11). Partita vera: *how do the actors recognise it?* The ball that bounces to the right height, the defender a step earlier, the ball that goes out or stays in by a hair. *What does it depend on?* The fine time of the body and of the ball. *Who produces it?* The players and physics; the step of the computation decides nothing. The bodies and the ball move in five small steps a tick (50 a second), the decisions, perception, thought, fatigue and the Laws stay at 10. Inside, positions in hundredths of a centimetre and velocities per step; every constant per step derived from the per-tick one (speeds a fifth, accelerations a twenty-fifth, gravity 40 hundredths per step² exactly, the friction of air and grass as the fifth root of the per-tick share, in millionths with `dmath`); the record and every reader in centimetres, unchanged. A body's target is the tick's, his braking, wanted speed and turn are recomputed each step. The foresight of the ball (`ball_at_line`) uses the same steps. The 2 m drop now comes back up into FIFA's band (0.60-1.00 m). About 3 s a match. Not yet: the contact with the ball in the air and the reception are still checked once a tick (OI-102 next). ds-reference 42-101, 146 -> 147: goals 2.18 -> 2.95, throw-ins 16.8 -> 25.9, goal kicks 9.8 -> 12.6, corners 9.4 -> 10.8, red cards 0.08 -> 0.23, clearances 42.5 -> 49.2, aerial duels 6.6 -> 6.2; 11 of 22 in band. Goldens regenerated, glibc and musl. The packed record of a match is about 2.3 MB.
148: the contacts at every small step (ADR-069, the "not yet" of 147). Partita vera: *how do the actors recognise it?* The ball that crosses a man's head between two tenths of a second is his to head; a ball coming down is met at the moment it reaches the foot. *What does it depend on?* Where the ball and each body were at the same small step. *Who produces it?* The ball's flight and the men's runs, step by step. The ball's flight and every body's run record where they are at the end of each of the five small steps (`SpatialState::ball_path`, `body_at_step`); the contact in the air is judged at the first step where the ball passes within someone's reach, each man where he was at that step, and the reception at the nearest point of each stretch of the ball's path against where the man was at its end. A ball struck, placed or met since it moved, or a man placed since he moved, is only where he is now. With it, a rule of the same kind as "nobody receives his own pass": nobody meets in the air the ball he has just played until someone else touches it. Without it, met at every step, a header up and out was still at its author's head the step after (913 header clearances in 6 matches, 624 by the same man in a row). No new number. Goldens regenerated, the same on glibc and musl. ds-reference 42-101: 11 of 22 in band, the same set; goals 2.95 -> 3.50, own goals 0.58 -> 0.68, clearances 49.2 -> 58.5, throw-ins 25.9 -> 19.5, aerial duels 6.2 -> 6.1. The aerial duels do not come from the time of the contact: OI-102 stays the second man on the high ball, for the calibration. The same version brings ADR-070 (DD-12): the team orders are asks in each player's choice, through his discipline (more pressing and the counter-press ask the two nearest the carrier to press, the drop back asks everyone to hold, the counter asks to run, keeping it asks to offer), and a man asked to attack the depth holds on the last defender's shoulder, onside. A match without orders is the same: goldens and measure unchanged by it. The twins: with more pressing the nearest man 8.5 m from the carrier instead of 9.7; asked to run, the striker 35.9 m ahead of the ball instead of 31.3 (at 147 the ask moved him 1 cm).
149: the high ball (OI-102). Partita vera: *how do the actors recognise it?* The long ball to the target man with the centre-back on him; the header that goes wide or into the keeper's arms; the keeper who stays on his line when the striker gets there first. *What does it depend on?* Who is stronger in the air, how well he heads, who gets there first. *Who produces it?* The passer, the header, the keeper, each with his own reading. Four changes, each with its cause found by measuring. (1) The foresight in small steps (ADR-069): `drop_point` and `intercept` read the flight in the steps the ball moves in. (2) The passer weighs a contested long ball by who wins it in the air (ADR-052 note): his teammate's aerial force he knows, the opponent's from his size; a man marked within `PASS_FREEDOM_CM` is still a target in the air (before, not even an option: of 66 balls met in the air a match, 61 were met by one man alone, the nearest opponent beyond 5 m in 49). (3) The header at goal has the kick's error with his heading (ADR-051 note): before, every header went at the middle with no error. (4) The keeper comes for a ball in his area only if he believes he gets there first (ADR-051 note): before, he came for every one, and 56 of 63 headers at goal found him off his line (headed goals 2.5 a match, one in two headers). No new number. Goldens regenerated, the same on glibc and musl. ds-reference 42-101: 11 of 22 in band, the same set; goals 3.50 -> 3.72, shots on target 9.35 -> 8.72 (real 8.69), saves 11.60 -> 12.07, aerial duels 6.1 -> 7.9, own goals 0.68 -> 0.77. The aerial duels are still a fifth of the real: the rest of OI-102 is the defenders' marking on the high ball, and the crosses (7.8 a match). Two twins of ADR-070 follow what the order asks: the board swap picks the first seed from 49 where the man stays on (a knock now takes him off in seed 49), and the run measures his distance from the last defender's line (2.4 m nearer with the ask; ahead of the ball it was only 1.2 m, a measure that moves with the ball too).
150: nobody is put anywhere (the maintainer's report on seed 43, his pure error 2a and his answers of 17 November; `BEHAVIOURS.md` 25-27; ADR-021 and ADR-069 notes). Partita vera: *how do the actors recognise it?* The taker who runs to the ball and the restart that waits for him; the opponents who walk back 9.15 m; after a goal the players walking back to their half and the kick-off a minute later; the substitute who comes on at the halfway line and runs to his place; a man who walks round another standing in his way. *What does it depend on?* The legs of each man, and the Laws (3, 8, 13-15). *Who produces it?* The players and the referee who waits. A restart called during play (`call_restart`) places the ball and sends its taker, the nearest man (the maintainer: for now; a penalty is still the best kicker's), to it running; the opponents out of the distance, the wall, everyone out of the area at a penalty go to their places at their pace; the restart is taken when the taker is at the ball and the men sent are there, the referee waiting no longer than `goal_to_kick_off_s`. After a goal everyone walks back to his half, and the kick-off waits that time, 60.6 s, measured (the median from a goal to the next kick-off, Serie A 2017-18, 1009 goals, `balance/stoppage_time.json` and `SOURCES.md`), and only with everyone but the kicker in his own half (Law 8). While a restart waits the ball is out of play, nobody plays it, and the clock runs. A taker who goes off (the fouled man, replaced at that stoppage) leaves it to the nearest of his side. A substitute comes on at the halfway line on the benches' side (y = 0, declared) and runs to his place. The separation of the bodies no longer moves a man faster than his legs, and a man goes round a body standing on his way by the side that lengthens his way less (on equal ways by his right, declared), with his acceleration and turn; the standing man does not move for him. Tests: `no_teleport` (nobody faster than the fastest legs in the golden, but at the start of a period; after a goal the kick-off waits and the sides are in their halves; a substitute comes on at the halfway line; no restart stops the match) and the scene `a_man_goes_round_a_man_standing_on_his_way`. Before, 74 jumps of 10 to 27 m in the match of seed 43. The twin of more pressing reads the two nearest men over twenty matches (over six the spread of the match was larger than the order's effect, about 0.7 m). Goldens regenerated, the same on glibc and musl. ds-reference 42-101: 11 of 22 in band (passes 1308 -> 1196, take-ons 20.4 in band; red cards 0.02 out); the ball is in play less, as in a real match.
151: in a dribble the ball is ahead (the maintainer's pure error 2b; `BEHAVIOURS.md` 8; ADR-053 note). Partita vera: *how do the actors recognise it?* The dribbler with the ball in front of him, at his foot, the body between it and the opponent. *What does it depend on?* His touch, his pace, his turn. *Who produces it?* The carrier. Measured on the record of seed 43: the ball more than half a metre behind the carrier in 31% of the ticks with one, and in one tick of a steady dribble in two. Three causes: the touch was weighed as if the ball started at his feet while it lay where he was a tick ago (now it takes that gap, and it has rolled within the tick of the touch); while it rolled ahead he ran past it at his top speed towards where he meant to go (now he runs to where he meets it, at the pace that brings him there when it is due); and turning he played it the new way while his body still went the old one (now, turning by more than 60 degrees, the turn of the movement, he keeps it under his sole and plays it ahead once he runs that way). In a steady dribble it is now behind in about 3% of the ticks (test `carry::in_a_dribble_the_ball_is_ahead`). A ball won at full speed and run past is the reception (OI-123), open. No new number. Goldens regenerated, the same on glibc and musl. ds-reference 42-101: 10 of 22 in band; passes 1196 -> 1336 (the carrier keeps it), take-ons 24.0 out again, shots on target 7.23.
152: the place with the ball, chosen within the zone of the role (ADR-071, DD-13; decisions BK and BL, layer 2 of `BEHAVIOURS.md`; the maintainer's priority, the players' football education). Partita vera: *how do the actors recognise it?* The forward who comes down between the lines when the ball is deep, the winger who stays wide while the full-back comes inside, nobody in a teammate's corridor, the centre-backs behind the ball. *What does it depend on?* The space he sees, his role. *Who produces it?* The player; the coach's shape gives the zone he starts from. With his side on the ball, a man who holds his place chooses it: within his corridor and the two beside it (the pitch in five), on his line or one ahead or behind (a third of the block's depth with the ball), never beyond the offside line; a space is worth what the game measures (its value to the side, as free as he would be there, by the clean line from the carrier and the plausible distance of a pass, ADR-040); never two in a corridor on a line; a man of the back line behind the ball. No new number. Measured with `ds-reference scene` on the golden: the two most advanced from 30.4 to 27.3 m ahead of the ball; the direction of the passes unchanged (23% forward, 23% back, against 31% and 16% in Serie A 2017-18): the passes back are the carrier's and the offering man's, the next step. Goldens regenerated, the same on glibc and musl. ds-reference 42-101: goals 3.87, corners 14.7, saves 14.7 (the calibration is stopped, decision BK).











136: the man a defender would mark is a danger as much as a pass can reach him (OI-117 (A), the maintainer's choice of 2026-11-16). Partita vera: *how do the actors recognise it?* The defender nearest the carrier steps out to him when the man behind is covered, and stays with the man when a pass to him is on. *What does it depend on?* Whether a ground pass from the carrier would reach the man before one of the defenders: the defender imagines it as a passer does (ADR-052, `who_first`, everyone at the pace and reaction he believes, the pass weighed to arrive rolling). No new number. Before, the man weighed his whole danger, as if he already had the ball: beyond 6 m the defender nearest the carrier marked another man in 54% of his choices (seed 42), and a carrier had an opponent within 5 m 33% of the time (Metrica: 46-50%). Test `a_man_no_pass_can_reach_is_less_of_a_danger`. ds-reference seed 42-101, 135 -> 136: ground duels 125 -> 191 (in band), fouls 9.1 -> 13.3, yellow cards 3.15 -> 5.08, take-ons won 18.5 -> 32.9 (above its band, OI-103), passes 1190 -> 1319, goals 2.35 -> 2.23. Goldens regenerated, glibc and musl.

135: a pass struck into a body comes off it (OI-119, delegated DD-4). Partita vera: *how do the actors recognise it?* A pass hits the leg of the man pressing the passer and runs loose. *What does it depend on?* Where his body stands on the ball's line; not whether he has reacted (SIM 133: a man plays a ball only once he has reacted to it, but his body is there before his mind is). No choice: physics. The ball comes off his body reflected about the line from its centre to the ball (the engine's body, `BODY_RADIUS_CM`), at the half speed of a ball nobody masters (the heavy touch), and the pass is over. No new number. Before, an opponent who had not reacted let a ball struck into him go through him: StatsBomb open data (Liga 2020/21) counts 24.9 passes blocked a match, the engine 134 had 2 in 15 matches. Now 49.1 a match (seeds 42-56): the engine's body (45 cm) is wider than a man's legs, a question for the body's own model, not a number to retune. Test `a_pass_struck_into_a_body_comes_off_it`. ds-reference seed 42-101, 134 -> 135: interceptions 150 -> 134, shots 21.6 -> 19.7, on target 11.8 -> 10.8, saves 12.4 -> 12.5, own goals 0.30 -> 0.38, clearances 37.6 -> 39.4 (just above its band, 38.2), passes 1245 -> 1190. Goldens regenerated, glibc and musl.

134: the choice at the reception, the first-time clearance (ADR-064, delegated DD-3; OI-118). Partita vera: *how do the actors recognise it?* A defender in his box, a forward on him, a ball coming at him: he hits it away first time instead of stopping it. *What does it depend on?* Whether his touch masters the ball (ADR-058, which he knows), whether he would play it before the forward reaches him (his own time to decide, `latency.json`, against the forward's run at the pace and reaction he believes, `thought.json`) or his body would hold the challenge (his shield), and what the clearance's landing is worth (ADR-052). No new number. Before, a ball reaching a player was only controlled or touched heavily: the choice came after, at his feet, and the foot clearance almost never happened (0.5 a match; StatsBomb open data, Liga 2020/21: 16.2 by the foot, 13.8 by the head). The first-time pass and shot are the next step of ADR-064. Test `a_defender_who_would_be_robbed_clears_first_time`. Foot clearances (seeds 42-56) 0.5 -> 7.4 a match. ds-reference seed 42-101, 133 -> 134: clearances 30.7 -> 37.6 (in band), goals 2.35 -> 2.62, shots 19.7 -> 21.6, on target 10.5 -> 11.8, corners 12.4 -> 13.5, own goals 0.42 -> 0.30 (in band). Goldens regenerated, glibc and musl.

133: an opponent reacts to a ball before he plays it (OI-100). Partita vera: *how do the actors recognise it?* A man a step from the passer sees the ball struck past him before his foot can move; one five metres away, who read it, cuts it out. *What does it depend on?* His reactivity: the body's reaction to a ball struck, the keeper's formula (`save.json` `reaction_fast_ticks`, `reaction_slow_ticks`, 0.3-0.6 s), no new number; the passer already reckoned with a reaction when he imagined who reaches his pass first (`thought.json` `believed_reaction_ticks`). Before, an opponent within reach of the ball's path took it on the tick it left the foot: 118 of the 290 interceptions a match came within 1-3 m of the passer. Found after correcting `ds-reference`, which did not count the intercepted passes (told as `Interception` only). Test `a_ball_just_struck_is_past_the_man_who_has_not_reacted`. ds-reference seed 42-101, 132 -> 133: passes accurate 74.6% -> 83.9% (Wyscout 83.3%), interceptions 291 -> 153, shots 35.7 -> 19.7, on target 20.4 -> 10.5, saves 19.8 -> 10.9, corners 19.4 -> 12.4, goals 3.92 -> 2.35, take-ons won 26.9 -> 19.7, all in their bands; ground duels 200 -> 125 and fouls 12.3 -> 8.5 below theirs. Goldens regenerated, glibc and musl.

132: the pass and the clearance under pressure (OI-100). Partita vera: *how do the actors recognise it?* A man with an opponent on him plays a hurried ball: it strays from his line, the more the closer the man and the less calm he is; a clearance under pressure is sliced into touch. *What does it depend on?* The same pressure and composure as the shot (`kick.json` `error_pressure_rad`, `error_rad_composed`, ADR-057): the nearest outfield opponent within `shot_choice.json` `kick_pressure_radius_cm`. Before, the passer reckoned with the pressure when he imagined his pass (`read.rs`), but the pass and the clearance he struck were computed with none (`resolve_pass`, `resolve_clearance`: pressure 0.0). No new number. Test `the_pressed_passer_strays_more`. Measured also, StatsBomb open data (Liga 2020/21, 20 matches, open play): 1,073 passes a match, 86.2% completed; 10-20 m 93.2% free and 86.4% pressed; 0-10 m 85.1% and 75.6%. The engine 131: 96.4% completed; 10-20 m 98.3% free and pressed alike. At 132: 95.4%; the rest of the gap is in the reception and the interception, not in the kick (OI-100). ds-reference seed 42-101, 131 -> 132: throw-ins 8.6 -> 14.8, clearances 29.2 -> 32.9, goals 3.02 -> 3.92, own goals 0.25 -> 0.50 (out of band), corners 15.8 -> 19.4. Goldens regenerated, glibc and musl.

131: the three forces of a contact on one scale (OI-117). Partita vera: *how do the actors recognise it?* A defender steps in on a man who turns his back to shield the ball: two men of the same strength and balance have the same chance; the better one wins it more. *What does it depend on?* Each force is the mean of what makes it, by its weights in `duel.json` (the tackle: tackling, burst, strength, balance; the shield: the carrier's touch and body plus his strength and balance; the take-on: dribbling, burst, balance). Before, the shield added its strength and balance on top of the carrier's force: its weights summed to 1.30 against the tackle's 0.95, and the carrier who shielded kept 99% of the contacts at his feet (seeds 42-71). Now (seeds 42-71, contacts with the ball at the feet, the margin unchanged): the carrier keeps 40.7%, loose 30.8%, the defender clean 28.5%; Wyscout Serie A 2017-18, ground attacking duels (144 a match): won 49.7%, neutral 28.2%, lost 22.0%. No number changed or fitted. Test `equal_men_meet_with_equal_forces`. ds-reference seed 42-101, 130 -> 131: goals 3.97 -> 3.02, shots 43.4 -> 32.5, on target 23.6 -> 18.0, saves 23.8 -> 19.0, corners 20.2 -> 15.8; but fouls 19.6 -> 12.2 (a contact the defender wins is not a foul, ADR-042 §4, and he wins more: OI-114), take-ons won 8.2 -> 26.4 (above its band, 21.7), throw-ins 9.2 -> 8.6, penalties 0.28 -> 0.08. Goldens regenerated, glibc and musl.

130: a ball nobody played to him is received like a pass (ADR-058; OI-117). Partita vera: *how do the actors recognise it?* A ball loose from a contact, deflected, cleared or bouncing comes at a man fast; a good first touch kills it, a poor one lets it get away. *What does it depend on?* His first touch against the ball's speed and the nearest opponent, as for a pass (`resolve_reception`, no new number). Before, whoever reached a loose ball first had it at his feet, whatever its speed (`plugins/technique.rs` `control_loose_ball`: the reception was checked only on a pass). Not mastered, it runs on at half its speed, as a pass he does not control. Test `a_loose_ball_is_received_like_a_pass`. Measured also (OI-117): StatsBomb open data (Liga 2020/21, 20 matches): of the challenges the carrier keeps the ball in 52.9%, the defender takes it clean in 14.9%, it comes loose in 23.1% and goes out in 9.1%; the engine 62.5%, 34.9%, 2.6%. A loose band on the defender's side alone was tried and dropped (it reached 4.1%). ds-reference seed 42-101, 129 -> 130: throw-ins 9.07 -> 9.17, goals 4.02 -> 3.97, nothing else moves by more than its noise: the loose balls the engine has are slow. Goldens regenerated, glibc and musl.

129: the block's width with the ball measured in open play (OI-100). Partita vera: *how do the actors recognise it?* With the ball in play a side spreads 47 m between its widest men; it gathers only at a restart. *What does it depend on?* The same measure as 127, read correctly. At 127 the width came from every tracking frame, the dead-ball ones too, when the players gather for a throw-in, a free kick or a goal kick (42.2 m); the block is the shape of open play, and the engine places the restarts by their own rules. With the ball present in the tracking (Metrica sample games 1 and 2): `block.json` `width_in_possession_cm` 4200 -> 4730 (47.3 m), `width_out_of_possession_cm` 3700 (37.0 m, unchanged); the slope on the ball's side was already measured on these frames. Measured also, to locate the narrow play: from the middle of the pitch real passes reach the outer tenth of the width 7.5-18% of the time (Wyscout), the engine's 0.5-1.6%. No new test: the twin of 127 holds. ds-reference seed 42-101, 128 -> 129: throw-ins 7.97 -> 9.07, goals 4.33 -> 4.02, fouls 18.8 -> 19.5 (back in band), ground duels 209.8 -> 211.1 (just out). Goldens regenerated, glibc and musl.

128: the ball bounces (OI-100). Partita vera: *how do the actors recognise it?* A clearance comes down, kicks up off the grass and hops on, over the feet of the man in its way, towards the touchline; it is met by whoever reads its bounces, or it goes out. *What does it depend on?* The grass and the speed it lands at. Before, a ball that landed stopped going up and slid on along the grass at the speed of its flight (about 22 m/s for a clearance), so whoever stood on its line met it at his feet: of about 300 clearances in seeds 42-51 none went out, while in Serie A 2017-18 a clearance comes before 21.5% of the throw-ins (Wyscout). Now `spatial.rs` `bounce`: it leaves the grass upwards at `ball.json` `restitution_milli` (632, derived from FIFA's vertical ball rebound, the 'excellent' band 0.60-1.00 m from a 2.00 m drop) of the speed it came down at, with the same step in `intercept`, so what is foreseen is what happens; the horizontal speed is kept at the bounce (FIFA's angle-rebound requirement, Test Manual II, not read). Test `a_ball_dropped_from_two_metres_bounces_as_on_a_good_pitch`. ds-reference seed 42-101, 127 -> 128: throw-ins 3.85 -> 7.97, goals 4.67 -> 4.33 (back on the edge of its band), goal kicks 11.4 -> 11.8, corners 19.9 -> 21.1, aerial duels 11.4 -> 11.1. Goldens regenerated, glibc and musl.

127: the block's width is the distance between its widest players, measured (OI-100). Partita vera: *how do the actors recognise it?* With the ball the wide man stands on his touchline when the ball is on his side (a real side's nearest man to a touchline with the ball: median 4 m), and the side slides towards the ball by about a quarter of the ball's distance from the middle. *What does it depend on?* The formation and where the ball is. Before, the 4-4-2 grid's widest slots (15% and 85%) were read as 15% and 85% of the block, so the widest men stood 30% of the block's width inside it, while the block's slide to the ball was clamped as if they stood on its edges: the wide men never came nearer than 9 m to a touchline. Now the widest slots are the block's sides (`shape.rs` `slot_role`), and `block.json` `width_in_possession_cm` 6000 -> 4200, `width_out_of_possession_cm` 4000 -> 3700, `ball_side_shift_milli` 600 -> 270 are measured on Metrica Sports sample games 1 and 2 (`balance/SOURCES.md`): undocumented before, sourced now. Test `the_wide_man_holds_the_touchline_on_the_ball_side`. ds-reference seed 42-101, 126 -> 127: throw-ins 4.65 -> 3.85 (the width was not their cause: they follow passes, and a pass in the engine rarely goes astray, OI-100), goals 3.65 -> 4.67 and fouls 19.7 -> 18.5 out of their band (the wider block out of possession leaves the gaps a real one leaves; the shot stays too precise, OI-100). Nothing retuned to bring them back (A5-bis). Goldens regenerated, glibc and musl.

126: the choice is the best option the player sees, even a bad one (ADR-052; OI-85, OI-115). Partita vera: *how do the actors recognise it?* A pressed defender with nothing good on plays the least bad thing; a carrier with a fair pass on plays it, and carries only when carrying is worth more. *What does it depend on?* The worth he imagines of each option (the thought, his reading and his appetite for risk already in it). Before, a formula from `choice.json` weighted that worth a second time by game reading and risk management (`reading_weight`, `risk_weight`, `floor`), every worth was cut at zero, and below `clarity_threshold` he carried the ball whatever else was on: the remains of the choice before ADR-052 (OI-85). Now `resolve_choice` takes the best worth, negative or not; a weak decider still strays to the second within `(1 - decision_making) * decision_bias` of the best's size; an option his reading does not notice cannot win. Four parameters removed from `choice.json`. Test `he_plays_the_best_option_he_sees_even_when_it_is_bad` (replaces the clarity test). ds-reference seed 42-101, 125 -> 126: goals 3.43 -> 3.65, shots 44.8 -> 46.3, fouls 18.4 -> 19.7 (back in its band), throw-ins 4.90 -> 4.65 (the clearance into touch is still worth less, by the player's own measure, than a risky pass: the throw-ins' cause is elsewhere). Goldens regenerated, glibc and musl.

125: the clearance into touch (OI-115). Partita vera: *how do the actors recognise it?* A full back pressed by his own corner flag, every man upfield marked, hoofs it into the stand: a throw-in for them is better than the ball lost in front of his goal. *What does it depend on?* The same measure as every option (ADR-052): out, the throw-in is theirs for sure, worth what they gain with the ball there; among his landings he takes the least bad, and the worth is cut at nothing only after. *Who produces it?* The defender (`read.rs` `clear_candidate`). No new number. Test `pressed_with_nothing_on_he_puts_it_into_touch` (twin: his teammate free where it would come down, he clears to him). Measured, ds-reference seed 42-101: identical to 124, line by line, and the goldens identical: the clearance into touch is the least bad of the clearances, but the choice cuts every option's worth at zero before comparing them, so a least bad option never beats one barely positive (OI-115, the cause the throw-ins still wait for). The bump declares the change of the clearance's landing, which the goldens cannot see (precedent: 112).

124: the keeper stands on the bisector of the angle the ball makes with his posts (plan L0-CLOSE 10d, the keeper's position read, not a rule). Partita vera: *how do the actors recognise it?* A winger cuts in from the right: the keeper steps across to his near post side, so that the near and the far corner are equally hard to find; a striker straight in front finds him in the middle. *What does it depend on?* The geometry of the goal: the bisector of the angle between the two posts as seen from the ball (`spatial::keeper_across`, half the distance between the posts from the Laws, `laws.json`); his depth off the line stays that of `keeper.json`. Before, he stood on the line from the ball to the middle of the goal, and a ball out wide left the near side open. *Who produces it?* The keeper. No new number. Test `the_keeper_splits_the_angle_of_his_posts`. ds-reference seed 42-101, 123 -> 124: goals 5.52 -> 3.43 (real 2.68), shots 54.9 -> 44.8 (24.6), shots on target 30.3 -> 23.7 (8.7), saves 27.6 -> 22.7 (9.4), corners 22.7 -> 19.2 (10.3): the shooters see a keeper who covers his goal, and find fewer clear chances. Goldens regenerated, glibc and musl.

123: the clearance's error (plan L0-CLOSE, the roots left: the error of the clearance). Partita vera: *how do the actors recognise it?* A defender under pressure hoofs it towards the wing and it sails off the side, or slices into the middle; a good kicker puts it where he meant. *What does it depend on?* The miss of a kick, the pass's own (`kick.json`): his technique (`passing`) and the foot he kicks with, at full power; the contact in his stride picks the side it runs off his line, as for the pass and the shot (ADR-054). *Who produces it?* The kicker (`resolve_clearance`). No new number. Twin `the_better_kicker_clears_nearer_his_aim` (met clean in the stride, it goes straight). ds-reference seed 42-101, 122 -> 123: throw-ins 4.65 -> 4.88 (real 42.7), goals 5.08 -> 5.52, clearances 33.7 -> 33.9; the throw-ins are not explained by it. Goldens regenerated, glibc and musl.

122: the wall at a free kick (Law 13; OI-105, OI-109; plan L0-CLOSE 10h-b). Partita vera: *how do the actors recognise it?* A free kick at the edge of the box: the defenders line up shoulder to shoulder 9.15 m from the ball, covering the near post, the keeper the rest; from the middle of the pitch, no wall. *What does it depend on?* The part of the goal their keeper cannot reach (his standing reach and his dive, `save.json`, against the width of the goal, `obstacles.json`), seen from the ball and measured at the wall, over the width of a body (`BODY_RADIUS_CM`); only within the shooter's range (`shot_choice.json` `shot_range_cm`). The outfield defenders nearest to the places take them and hold there (`Cover` to their place) until the kick, then choose again (`form_wall`, `release_wall`). *Who produces it?* The defending side. No new number. Test `a_free_kick_in_range_has_its_wall` (twin: from the middle, none). Not yet: how each man stays or turns by his courage (OI-109). ds-reference seed 42-101, 120 -> 122: goals 5.53 -> 5.08, shots 51.7 -> 54.2, saves 25.6 -> 27.4, corners 20.8 -> 22.8, offsides 7.2 -> 6.6; of the order of the engine's chaos. Goldens regenerated (the instructed one unchanged: no free kick in range in it), glibc and musl.

121: the psychology plugin may be switched off (ADR-063 §4, ADR-013's plugin configuration; plan PLUGINS step 6). `Match.psychology` (default on, saved with the match): off, courage holds nobody back at a high ball and composure leaves the pressure on the shot as it is, the match as it was before 119. The environment and the knocks have their cards (`registry::OPTIONAL_CARDS`, version 0, no code yet: OI-110). Test `the_match_is_played_with_psychology_switched_off` (played to the end, another match). Goldens identical (on in them); ds-reference not affected.

120: the minimal separation of the bodies no longer depends on the order the players are visited in (ADR-063, plan PLUGINS step 1). Partita vera: *how do the actors recognise it?* Two men who run into each other are pushed apart the same way whoever the engine looks at first. *What does it depend on?* Where each one's own step took him: every push of MD-12 is computed from those positions, then all are applied together (`spatial::separate`); before, the pairs were pushed one after the other in the order of the ids, and a pair saw the result of the pair before it, the one dependence on the order found in the tick's substrate (the step of each player is computed from the snapshot; the off-ball thought reads what was decided before its round). *Who produces it?* The bodies. No new number. Test `the_separation_does_not_depend_on_the_order`. ds-reference seed 42-101, 119 -> 120: goals 5.15 -> 5.53, offsides 5.9 -> 7.2 (real 4.3, now above its band), aerial duels 14.0 -> 12.8, fouls 21.0 -> 19.3; differences of the order of the engine's chaos. The cup golden now takes the first seed from 42 that goes to penalties (44), with the seed in its hash, so that it does not need moving at every version. Goldens regenerated, glibc and musl.

119: courage and composure, roots of their own (ADR-057 §6; OI-109 option A, decided by Claude on the maintainer's delegation of 2026-11-15). Partita vera: *how do the actors recognise it?* A slight winger does not go up for a header the big centre back can reach, a brave one does; a calm striker finishes with a defender at his shoulder as he would alone, a nervous one snatches at it. *What does it depend on?* Courage: he goes up for a high ball a heavier man he has seen can reach only if his courage is at least that man's share of their two weights; against a man no heavier, always (`goes_up`). Composure: the pressure term of the shot's error is scaled by 2 (1 - composure): middling (0.5) as before, calm less, nervous up to twice (`error_rad_composed`). *Who produces it?* The player. No new number in the engine; the test players' profiles get [0.5, 0.15] for both in every role (`profiles.json`, grade 3 declared, data of the players and not of the engine: OI-109 option A). Twins: `the_braver_goes_up_against_the_heavier_man`, the composure assertions in `kick.rs`. ds-reference seed 42-101, 118 -> 119: aerial duels 16.2 -> 14.0 (real 38.1), goals 4.87 -> 5.15, shots 54.4 -> 52.2, goal kicks 13.1 -> 11.5 (17.2, now just out of its band), offsides 6.4 -> 5.9. The cup golden moves to seed 80 (seed 45 now has a winner after extra time). Goldens regenerated, glibc and musl.

118: the coach who changes players during the match (plan L0-CLOSE 10h-c, Law 3), when his squad has `coach_reacts` (default off). Partita vera: *how do the actors recognise it?* A winger who can no longer go past his man is taken off for a fresh one in the same role. *What does it depend on?* What the coach sees from the bench: at a stoppage, an outfield player is replaced by a substitute of the same role who, fresh, runs faster than he can now, while he was the faster of the two fresh (the coach chose him to start); within `MatchRules.subs_allowed`. *Who produces it?* The coach, from the movement's own speeds dimmed by fatigue (`GroundTruth::top_speed_now`). No new number. Test `a_reacting_coach_changes_tired_players` (twin: the same match without, no change). Seen: in 10 matches 0 to 4 changes, some in the first 15 minutes (seeds 45, 46, 49, 50, 51): fatigue dims the top speed early enough that a nearly as fast substitute is already better, a cause to look at in the calibration (OI-111), not a delay to add. Goldens identical (the flag is off in them); ds-reference not affected (its squads do not react).

117: extra time and the penalty shoot-out, when the competition asks for them (Laws 7 and 10, IFAB Laws of the Game 2026/27; plan L0-CLOSE 10h-a). Partita vera: *how do the actors recognise it?* A cup tie level at 90' goes on for two periods of 15 minutes; still level, the teams take their kicks from the mark, five each, alternately, until one cannot be caught, then one each until one scores and the other does not. *What does it depend on?* The rules of the match (`MatchRules.extra_time`, `penalty_shootout`), the score, and the coin: the tosses are an input of the match, like the seed (`Match.tosses`, maintainer 2026-11-15), the save keeps them; the winner of a toss kicks off (the possession is worth more than an end on a neutral pitch) and kicks first in the shoot-out. *Who produces it?* Each team picks its order from the players on the pitch at the end, best taker first, by the same measure as a penalty in the match (`set_piece_taker`); each kick is a penalty of the match, the kicker strikes and the keeper reads, completed when saved, in, out or played by another; its goals are not the match's (`MatchResult.shootout`). Two new halves in the events, `ExtraFirst`, `ExtraSecond`, and `Shootout`. Found on the way and fixed: a penalty taker who weighed holding the ball or passing it gave the ball back to open play; now a penalty is a kick at the goal and nothing else (Law 14). No new number of football (the 300 ticks of `KICK_TICKS` are a guard, far longer than any kick). Tests: `a_cup_match_level_at_ninety_goes_to_extra_time` (twin: the same match as a friendly ends at 90'), `a_cup_match_won_at_ninety_ends_there`, `a_cup_match_level_after_extra_time_goes_to_penalties` (a winner, every kick a shot, the score untouched), and a third golden, `golden_l0_cup_is_deterministic` (seed 45 to penalties, 3-4), the same on glibc and musl. The two goldens of the friendlies are identical, and ds-reference seed 42-101 is identical to 116, line by line: no friendly reached a penalty taker who did not shoot.

116: the carrier's scene places every man where he has seen him (OI-106, decision A; ADR-052). Partita vera: *how do the actors recognise it?* A midfielder turns and plays into a lane he saw open a second ago, and a defender who came across meanwhile cuts it out; a forward shoots because the last time he looked nobody was between him and the goal. *What does it depend on?* His last look at each man (the memory of 115): until now `scene_from` gave the carrier every position of the pitch, true and current. *Who produces it?* The carrier, perceiving. `MatchState::as_seen_by`: each man where the carrier last saw him, himself where he is, a man never seen left out. No new number. Test `the_carriers_scene_holds_whom_he_has_seen`. ds-reference seed 42-101: offsides 2.87 -> 6.40 (real 4.26: the line is read from where the defenders were last seen, ADR-062 (c) in effect), shots 47.9 -> 54.4 (24.6), goals 4.55 -> 4.87, own goals 0.20 -> 0.17, aerial duels 13.4 -> 16.2 (38.1), accurate passes 1171 -> 1065 of 1109 (96%). Golden regenerated, the same on glibc and musl.

115: each player's scene reads the pitch through what he has seen (OI-106, decision A of 2026-11-15; ADR-034, MD-40, ADR-052). Partita vera: *how do the actors recognise it?* A midfielder goes for a loose ball because he has not seen the teammate coming from behind, and they meet on it; a defender steps out to cut a pass he thinks is his, unaware the man he never saw is nearer. *What does it depend on?* What his cone showed him at his last look: every player now keeps, for each man, where he last saw him (`MatchState.seen`, filled each tick from `perception::visible_to`); a man never seen is not in his scene. *Who produces it?* Each player, perceiving. Four readings of the off-ball thought in `decide_off_ball` read the whole pitch and now read the memory: `rank` (teammates nearer the target), `rank_drop` (teammates nearer where a high ball comes down; found in the recheck, the same leak), `loose_rival_cm` (the nearest opponent to the ball), the cut-out of a pass on the grass (`cut_target`: he goes before the receiver he has seen, or wherever he meets the ball if he has not seen him; ahead of him only the teammates he has seen). No new number. Tests: `a_player_knows_only_whom_he_has_seen` (the anti-leak test of the scene: after the first look a player knows exactly the men in his cone; red before, `perceived` did not exist), `a_man_is_remembered_where_he_was_seen`, the twin `he_cuts_out_a_pass_from_what_he_has_seen`. ds-reference seed 42-101: goals 4.63 -> 4.55, own goals 0.32 -> 0.20 (real 0.10), offsides 2.97 -> 2.87, aerial duels 11.3 -> 13.4 (38.1), shots 44.9 -> 47.9, accurate passes 1207 -> 1171. Only the men on the pitch are seen and remembered: one replaced or sent off leaves every scene. The memory is a square table over the players (`Memory`), checked with the cone each tick without allocating. Golden regenerated, the same on glibc and musl. The memory never forgets yet: a man seen once stays where he was seen until seen again (ADR-062 (c) reads the line from it next).

114: a runner nearer the line than the eye can tell apart looks level (ADR-062 (b); Wühr, Fasold and Memmert 2015, PLoS ONE 10, e0133687; the same for every observer, Put et al. 2013). Partita vera: *how do the actors recognise it?* A midfielder plays a runner a hand's width past the last defender, sure he was level, and the flag goes up. *What does it depend on?* How far the runner is from him: the eye tells two men apart only beyond about 8 mrad of their distance (10 cm seen from 12 m, 40 cm from 50 m). *Who produces it?* The passer, perceiving (`read.rs` `offside_to`). New number: `scanning.json` `level_threshold_mrad` = 8, sourced (derived), row in SOURCES.md. Twin: `a_runner_a_hair_past_the_line_looks_level` (the exact eye calls him off, the human eye sees him level). ds-reference seed 42-101: offsides 2.05 -> 2.97 (real 4.26), goals 4.27 -> 4.63, own goals 0.27 -> 0.32, shots 44.9, throw-ins 6.5 (42.7). Golden regenerated, the same on glibc and musl. Open (ADR-062 (a) and (c)): the spread of attention as `vision` and the line read from his last look need the memory of what he saw, which the engine does not have yet (MD-40).

113: the offside line read as the eye projects it (ADR-062, the first part; Oudejans, Verheijen, Bakker, Gerritsen et al. 2000, Nature 404, 33). Partita vera: *how do the actors recognise it?* A winger plays a runner who looked level with the last defender and the flag goes up; another holds the pass to a runner who was in fact onside. *What does it depend on?* Where he stands as he looks: two men look 'in line' when they stand on the same line of sight from him (the optical projection on the retina), so off the line he misreads it, the more the wider apart they are; the source finds these errors 'inevitable, regardless of the quality' of the observer (171 flag errors against 31 for the attacker outside far from the assistant, 19 against 84 near him, 200 matches of five leagues and the 1998 World Cup). *Who produces it?* The passer, reading the line. read::offside_to now reads the line through the second-last defender along his line of sight; looking along the depth (the sight line nearer the length of the pitch than its width) the projection tells him nothing and he reads the true line, the depth he would misjudge being the open part of ADR-062 §5. No number: the geometry of the view is the whole model. Measured (ds-reference compare, seeds 42-101, a report, A5-bis), against 112: offsides 1.55 -> 2.05 (real 4.26), goals 4.22 -> 4.27, own goals 0.18 -> 0.27 (0.10). No twin on a characteristic: the source finds the error independent of the observer's quality. Both goldens regenerated, verified on glibc 2.39 and musl. P1 the_line_is_read_as_the_eye_projects_it.

112: the obvious goal-scoring opportunity (Law 12). Partita vera: *how do the actors recognise it?* The striker through on goal with only the keeper to beat is brought down by the last defender: a red card. *What does it depend on?* The distance to the goal and the location and number of defenders (Law 12's considerations). *Who produces it?* The offender, judged by the Laws. The foul denied an obvious chance only if no defender but the offender, the keeper included, stood nearer the goal; the keeper nearly always does, so it almost never was one. Now the keeper is not cover: he is the man the attacker had to beat (rules::obvious_chance, a pure rule). No new number (dogso_distance_cm unchanged, still without a source). Measured (ds-reference compare, seeds 42-101, a report, A5-bis), against 111: red cards 0.03 -> 0.10 (real 0.24), penalties 0.10 -> 0.25 (0.33), yellow cards 4.67 -> 4.50 (3.97), own goals 0.22 -> 0.18. The golden is identical (no foul of seeds 42-51 is an obvious chance); the bump declares the change it cannot see, as at 17. P1 only_the_keeper_to_beat_is_an_obvious_chance.

111: a ball in the air is met on its whole path (OI-102, the own goals). Partita vera: *how do the actors recognise it?* A keeper who cannot reach where a lofted back pass comes down goes to meet it further along its line, where it is still dropping or already rolling, not after it. *What does it depend on?* The ball's flight, by the engine's own physics, and his acceleration and top speed. *Who produces it?* The man who goes to the ball. Followed tick by tick (seed 43, 92'05"): the keeper chose to go to the ball and aimed at the point where it came down to head height, 6.5 m away, which he could not reach in time; once it was below his head that point no longer existed and he chased the ball where it was, and it ran into the net. The point of meeting on its path (SpatialState::intercept) held only for a ball already rolling; now it follows the whole flight with advance_ball's steps (gravity and the air, then the grass, then the stop) and gives the first point within his height he reaches in time; the head-height drop point if he can get there, else further on. No new number. Measured (ds-reference compare, seeds 42-101, a report, A5-bis), against 110: own goals 0.40 -> 0.22 (real 0.10), corners 22.7 -> 18.8 (10.3), goals 4.22 -> 4.12, but throw-ins 16.8 -> 5.6 (42.7) and aerial duels 12.6 -> 10.5 (38.1): the players now meet nearly every ball before it goes out, because they foresee its flight without error. That is the same root as the offside line read perfectly: the perception error of foundations §4 (vision, distance, angle) is not built for positions and flights, a model decision (OI-98, OI-102). The test a_rolling_ball_is_met_on_its_path now says a ball in the air is met where it comes down within his height (before: None, not met). Both goldens regenerated, verified on glibc 2.39 and musl.

110: the lofted pass through the air (OI-102). Partita vera: *how do the actors recognise it?* A long ball lands where it was aimed, at the man's feet or head; one played back towards his own goal, missed, runs on into the net. *What does it depend on?* The speed he gives it, knowing the air as he knows the grass, and the ball's line. *Who produces it?* The passer. Followed tick by tick (seed 42, 3'13"): a 37 m ball fell 4-5 m short of where it was aimed. The kicker reckoned no drag in the air ('no grass slows it'), while advance_ball keeps air_friction_milli of its speed each tick: over 18 ticks about 8% of the distance was lost. Now the horizontal speed carries it the distance through the air, d (1 - a) / (1 - a^T), as the ground pass is weighed through the grass; the passer's thought reckons the same (the speed it arrives at, and, missed, where it runs on: into his own goal if its line goes there, lofted_missed_at, as runs_into_goal for a ground pass). No new number: air_friction_milli is ball.json's. Measured (ds-reference compare, seeds 42-101, a report, A5-bis), against 109: aerial duels 10.4 -> 12.6 (real 38.1), throw-ins 13.1 -> 16.8 (42.7), clearances 28.7 -> 39.6 (29.5), own goals 0.25 -> 0.40 (0.10): lofted back passes to the keeper used to fall short and now arrive, at knee height and 13 m/s, where the keeper is not quite in time (seed 43, 92'05"); the passer reckons his keeper first to the spot, not whether he is there when it comes down, the next cause. Both goldens regenerated, verified on glibc 2.39 and musl. P1 a_lofted_pass_lands_at_its_man, a_missed_lofted_back_pass_runs_into_his_goal.

109: the pass's contact (ADR-054 for the pass, as it left open). Partita vera: *how do the actors recognise it?* A misplaced pass runs off either side of its line, now and then over the touchline. *What does it depend on?* How wide his error is (passing, the foot, the power: unchanged) and the moment his foot meets the ball. *Who produces it?* The passer, at the contact. The drift across the line always went towards the centre of the pitch, a declared placeholder (since 55). Now, as for the shot, the phase of his stride at the contact picks the side: mid-step clean, late the foot opens and the ball runs to the side of the foot he kicks with, early it is pulled back across; PassContext.pitch_centre removed. No new number. Measured (ds-reference compare, seeds 42-101, a report, A5-bis), against 108: throw-ins 15.2 -> 13.1 (real 42.7), goals 3.87 -> 4.37 (2.68, within the spread of the chaos), shots 43.9 -> 45.1, passes completed 94.5% (real 83%). The hypothesis that the inward drift kept the ball in was wrong: passes are short and almost all reach their man, the ball rarely goes out because the play is too accurate (the passing error and the reception), an open question. Both goldens regenerated, verified on glibc 2.39 and musl. P1 the_pass_drifts_where_its_contact_takes_it.

108: the clearance, by the same measure (plan L0-CLOSE 11, the ball that goes out: clearances 144 against 30). Partita vera: *how do the actors recognise it?* A pressed defender near his goal clears when nothing better is on; with a teammate he can find, he plays it. *What does it depend on?* Where the ball would come down and who gets there first, against what a pass would bring. *Who produces it?* The defender, choosing. Measured (seeds 42-51): 147 clearances a match, 30 of them headers; with the foot, the nearest opponent a median 1.2 m away, 12 to 20 m from his goal. In the thought a clearance was worth what every option is worth (gain where it comes down, loss if they get there first) plus the danger of losing the ball where he stood, a bonus no other option had, though a pass that arrives takes the ball away from there too: a weight per kind of option (ADR-052 §6). Removed; the pressure still makes it an option near his goal. No new number. Measured (ds-reference compare, seeds 42-101, a report, A5-bis), against 107: clearances 143.8 -> 27.9 (real 29.5), shots 36.0 -> 43.9 (24.6), shots on target 19.5 -> 24.1 (8.7), goals 2.95 -> 3.87 (2.68), corners 19.0 -> 21.5 (10.3), passes 1169 -> 1242 (888). The bonus held the shots down without being their cause: the pressed defenders now play the ball, and the shots are the next question (where they are born, after which balls lost). Both goldens regenerated, verified on glibc 2.39 and musl. P1 the_clearance_earns_no_bonus_for_the_pressure.

107: the offside (Law 11). Partita vera: *how do the actors recognise it?* The forward who offers for the ball stays level with the last defender, and the passer does not play to a teammate he sees beyond it. *What does it depend on?* The line of the second-last opponent and where the ball is; *who produces it?* The man offering, and the passer. Measured before (seeds 42-51): about 19 offsides a match, and in 188 of 192 the man flagged was the very receiver the passer chose; at each pass 0.73 teammates stood offside, three in four of them offering for a pass, a median 2.75 m beyond the line. The offer went where the line from the carrier was most open, also beyond the last defender; the run already stopped on the line (onside_margin_cm). Now (1) the offer stays onside by the same margin, and (2) the passer sees the line: a pass, lofted pass or through ball to a teammate in an offside position as he sees it is lost (read::offside_to, the engine's own Law 11). No new number. Measured (ds-reference compare, seeds 42-101, a report, A5-bis), against 106: offsides 20.4 -> 1.13 (real 4.3: now he reads the line perfectly; in a real match it is misread and runs are mistimed, the reading of the line by game_reading is the next cause), free kicks 24.8 -> 9.3 (29.5: the offside ones were among them), goals 2.70 -> 2.95, own goals 0.25 -> 0.32, shots 32.9 -> 36.0. Both goldens regenerated, verified on glibc 2.39 and musl. P1 the_passer_sees_the_offside_line. The penalty became rare (0.10 a match, real 0.33) and no longer showed in the repertoire test's seeds: Law 14 is now a pure rule (rules::foul_restart, the same restart as before, the golden unchanged by it) with its own guard, a_foul_in_the_area_is_a_penalty, and the penalty is a declared rare act there, as the red card; why penalties are rare is a distance still to explain.

106: OI-98, the back pass into the run. Partita vera: *how do the actors recognise it?* A defender who plays the ball back to a teammate looks at him and plays it where he is going, not where he was. *What does it depend on?* That he looks at the man he serves (the maintainer, 2026-11-04, SIM 15: the cone does not filter the pass target), and the man's run. *Who produces it?* The passer. Followed from the kick (seed 42, 2'06"): the pass was weighed right (8.5 m/s), but aimed where the teammate stood, behind the passer and out of his cone, while he ran away from that point; he went to meet it (SIM 105) and could not come back in time, and the ball rolled into the goal. The motion a passer read was only that of the players in his cone, against the decision that he looks at the teammate he serves. Now the scene's motion holds the players he sees and the teammates he may serve; the opponents stay filtered by his cone. No new number. Measured (ds-reference compare, seeds 42-101, a report, A5-bis), against 105: own goals 0.48 -> 0.25 (real 0.10), goals 3.07 -> 2.70 (2.68), shots 34.4 -> 32.9 (24.6), passes 1201 -> 1166 (888), offsides 17.0 -> 20.4 (4.3: more passes played into the run, more receivers past the line; still to explain). Guarded by the golden and the measure; the lead into the run is P1 a_pass_is_played_into_the_run (SIM 96). Both goldens regenerated, verified on glibc 2.39 and musl.

105: OI-98, the own goals (plan L0-CLOSE, the order of 2026-11-14). Partita vera: *how do the actors recognise it?* A defender plays the ball back to his keeper, and the keeper comes to meet it on its line, whoever else is nearer. *What does it depend on?* That the pass is played to him; how fast he gets there is his body's. *Who produces it?* The receiver. Followed tick by tick (seed 43, 26'28"): a back pass from 34 m, 1.8 m off the keeper by the pass's own error; the keeper took up his place between the ball and the goal and the ball rolled past him into the net. In the off-ball thought a receiver was ranked by his distance to the ball at that moment: at the kick his teammates were nearer, rank 2 or more, so he held his place. Now a pass played to him is his to meet first (rank_to_meet), as the code already said of the receiver's scene. No new number. Measured (ds-reference compare, seeds 42-101, a report, A5-bis), against 104: own goals 1.37 -> 0.48 (real 0.10), goals 4.40 -> 3.07 (2.68), passes 1087 -> 1201 (888), offsides 12.7 -> 17.0 (4.3: more passes reach their receivers, more receivers are flagged; a distance to explain, not to tune). Still open: a defender passing back across his own goal mouth, whose pass misses the man (seed 42, 2'06"): the passer does not reckon his own pass's error as the shooter reckons his shot's (SIM 88-90). Both goldens regenerated, verified on glibc 2.39 and musl. P1 the_receiver_is_the_first_to_meet_his_pass; the twin of how he meets it is a_rolling_ball_is_met_on_its_path (the quicker meets it sooner).

104: ADR-059 step 2, the team's intention in the off-ball thought. Partita vera: *how do the actors recognise it?* The coach asks his side to play wide, and the disciplined winger holds the touchline while the one who reads the game his own way comes inside where he sees the space. *What does it depend on?* The place the coach's shape gives him, what that place is worth and how free he would be there, and his discipline. *Who produces it?* Each player, reading the coach's shape through his own character (VISION §3). With the ball, holding was valued where he stood, while choosing it sent him to the place the shape gives him: the coach's intention moved him without entering his judgement. Now holding is worth the coach's place (read::space_value there, as free as he would be there), weighed by his discipline with the same rule as without the ball (offball::place_weight, discipline_floor_milli, SIM 83); no new number. Measured (30 seeds 42-71, a report, A5-bis), against 103: goals 4.67 -> 4.27, own goals 1.50 -> 1.30, shots 32.7 -> 33.8, crosses 15.0 -> 14.6, the carrier in his own third 60% -> 61%. Both goldens regenerated, verified on glibc 2.39 and musl. Twin test the_more_disciplined_follows_the_coachs_shape; P1 the_team_plays_wider_when_the_coach_asks.

103, unchanged (2026-11-14, ADR-059 step 1, the input stream; no bump: a match without instructions is the same match, the golden is identical, as F.3-b1 at 11). The coach speaks during the match: Squad.instructions, each an Instruction { tick, order }, the order today Order::Intention (the block and the width the coach can also set before the kick-off, OI-27); it takes effect from the next tick for his whole team, at once (ADR-059 §2), and the fact is told (EventType::Instruction, its minute). It is saved with the match, and the replay plays it back. Partita vera: the coach on the touchline asks the team to step up and play wide, and from that moment the players read the new shape; each one still chooses his own act (ADR-052). A second golden with a fixed instruction stream (home high and wide at tick 18000, away low and narrow at 36000), verified on glibc 2.39 and musl. ds-demo --instruct MIN:BLOCK,WIDTH (after MIN minutes of play: the tick is continuous, so the commentary shows the broadcast minute, stoppage time apart). P1 an_instruction_takes_effect_from_the_next_tick (identical before, different after), the_same_instructions_give_the_same_match, a_saved_match_replays_its_instructions, golden_l0_with_instructions_is_deterministic. Next (ADR-059 step 2): the team's intention in the off-ball thought, with the discipline twin.

103: Plan L0-CLOSE 11, OI-98, ADR-058 (the ball is received where it arrives). Partita vera: *how do the actors recognise it?* A pass is complete when a teammate has the ball, by foot or head, never because a time has passed; a ball that runs past him is lost, and whoever gets there first takes it. *What does it depend on?* The ball's speed as it reaches him, the nearest opponent within a slide's reach (1.68 m, duel.json), his first touch; and, for everyone else, whether he is the nearest of his side and how well he reads the play. *Who produces it?* The receiver with his foot, the opponents who run to it, the passer who imagines it. Before, a pass resolved at due_tick (1.5 s, MD-37): the receiver's first touch was judged wherever he was, almost never failed, the pass was told complete and the ball rolled on with nobody going for it, often into the passer's own goal. Now: (1) the reception happens where the ball reaches a player, its difficulty (speed / speed_ref + pressure) / 2 against first_touch (ds_calculator::control::reception_difficulty), and a pass that goes past its receiver or stops is lost (Pass complete false); reception.json loses freedom_ref_cm, freedom_bonus_max_milli, distance_ref_cm, distance_penalty_max_milli, control_threshold_milli. (2) Opponents go to cut a rolling pass where they reach its path first. (3) The first aim is the ball (ADR-052 note, the maintainer): a ball nobody has, a pass on its way included, is anyone's to go for; the nearest of each side goes at any distance, its passer excepted, the next one if he reads the play above the middle of the scale and wins the race to it (offball::goes_for_loose_ball); before, a loose ball was weighed by proximity against hold_salience, and a pass was only its receiver's until the clock. (4) A ball is met where its path this tick passes a player below the header's band (1 m), not only at the end of the tick: a lofted pass coming down at 15 m/s crossed a receiver's reach between two ticks; a pass won in the air tells its outcome. (5) No clock ends a shot either: a shot is over when it meets the woodwork or stops short of the line, then the ball is anyone's (found by the fatigue test: off the crossbar back into play it stayed a shot nobody might go for, and the match froze for the rest of the half). (6) The passer imagines the reception with the same difficulty, against his teammate's first touch, which a poor reader underrates (as SIM 90), for the ground pass where it arrives and the lofted one where it comes down. No new number. Measured (30 seeds 42-71, a report, not a target, A5-bis), against SIM 102: goals 4.8 -> 4.67, own goals 1.70 -> 1.50, shots 22.8 -> 32.7, crosses 14.5 -> 15.0, passes 975 -> 1116, completed 90%, the carrier in his own third 52% -> 60% of the time. Without (3) the game stayed in its own third (77%): the clock had been doing what the players did not. Verified on glibc 2.39 and musl. Still open: the own goals left are back passes aimed off the keeper (aim and keeper, 10d); crosses below the real 35. Twin tests the_better_reader_goes_second_for_a_loose_ball, first_touch_is_read (ADR-058); P1 the_players_tire_and_half_time_rests_them (it found the frozen half), the_nearest_goes_for_a_loose_ball_at_any_distance, a_ball_coming_down_is_met_on_its_path, the_reception_is_hard_for_a_fast_ball_under_pressure.

102: Plan L0-CLOSE 11, OI-98 (the first half: the tip and the last touch). Partita vera: *how do the actors recognise it?* A keeper who turns a shot round the post sees it go out; a fan who sees a goal knows who scored it, and when it is an own goal. *What does it depend on?* The tip, on how far the slowed ball really runs sideways before the line; the scorer, on who last played the ball and who only touched it. *Who produces it?* The keeper's hands; the scorer is read by the observer from the touches. (1) **The tip follows the ball's real path.** The keeper turned the ball as much as a straight line said it needed to pass outside the post; the ball, slowed by the air and the grass (the integer step loses most of a small sideways speed), still crossed inside it: about one goal a match came from a tipped shot (seeds 42-71: goals after a save 1.07 -> 0). Now he turns it as much as the ball's own path needs, foreseen with the same step that moves it (`SpatialState::ball_step`, `ball_at_line`); near his line it still goes in, as before. (2) **Every goal has its scorer** (Law 10 gives the goal to the side; the scorer is who put it there). The spatial state keeps the last touches (`Touches`: who last played the ball, whether it was a shot, who only touched it on its way): the carrier, a header, a shot; a keeper's parry, a block, a heavy first touch. A ball crossing the line is the carrier's; else the last who only touched it, unless it was a shot (a parried or blocked shot is the shooter's); else the last who played it; a defender's is an own goal. Before, more than half the goals of seeds 42-71 had no scorer (3.3 of 6.7 a match). Measured (30 seeds, not a target; the chaos is about 1.5 goals): goals 6.7 -> 4.8, of which own goals 1.7. **Found, not fixed (the model, for the maintainer):** the own goals are passes resolved by the clock: at `due_tick` the receiver's touch is judged wherever he is, the pass is told complete, and the ball rolls on with nobody going for it, often into the passer's goal. Making the reception spatial alone (tried) makes 99.9% of passes complete and one seed has no shot: most turnovers came from those false receptions, so how a reception fails is a model decision (OI-98). P1 `what_is_foreseen_of_the_ball_is_what_happens`, `the_last_touches_name_who_played_it_and_who_only_touched_it`, `every_goal_has_a_scorer`. **Golden changed** (identical with glibc and musl). `SIM_VERSION` 101 -> 102.

101: Determinism on every platform (before L0-CLOSE 11; OI-99). Partita vera: nothing a player perceives, thinks or does changes; what changes is that the same match is the same match on every machine (00-foundations §4), which it was not. The golden differed on Ubuntu 26.04. Reproduced here with a second C library: the same code and compiler built for x86_64-unknown-linux-musl gave another golden (seed 42: 2-2 with glibc 2.39, 3-5 with musl, apart from 2'03", every seed of 42-51 different). Cause, isolated by putting back one std function at a time with everything else on the new module: std hypot alone (10 of 10 seeds differ between glibc and musl); std ln and powf gave no difference in these seeds, but nothing guarantees their bits elsewhere. Now every such function the engine uses comes from ds_core::dmath: hypot and hypot3 as sqrt of the sum of squares (correctly rounded IEEE steps; 46 calls), ln, log and powf from the libm crate, pure Rust, pinned =0.2.16 (2 log, 2 powf); the two powi(2) of the fatigue are a plain square. clippy.toml forbids the std transcendentals, f64 and f32 (hypot, atan2, sin, cos, tan, exp, ln, log, powf, powi and the rest), so gate C2 refuses them; the GUI's drawing of a player's heading is the one allowed use, it never feeds the simulation. sqrt and mul_add stay: IEEE 754 requires them correctly rounded (mul_add on x86_64 without hardware FMA calls the system fma: declared risk, OI-99). Verified: the golden of 101 is the same with glibc 2.39 and with musl (x86_64 Linux). Not verified: Windows (UCRT), macOS, Ubuntu 26.04 itself (OI-99). Measured (30 seeds 42-71, a report, not a target, A5-bis): goals 5.2 -> 6.7 a match, other counts within a few percent. No cause in the players: the old engine with musl's libm gives the same 6.73 (and 5.77 on seeds 72-101, the same as 101): on these inputs musl's hypot equals the sum of squares, glibc's took another path. So a 30-seed average moves by about 1.5 goals for a last bit alone; measures of step 11 read with that spread. P1 ds_core dmath::tests::the_bits_are_fixed; the clippy ban guards the rest.

100: Plan L0-CLOSE step 11, OI-98 (Law 10 for every ball). A goal existed only after a shot: any other ball crossing a goal line between the posts and under the bar (a back pass the keeper missed, a pass rolling on, a parried ball) was told as a corner or a goal kick, about three goals a match hidden. Now every ball over the line between the posts is a goal for the side attacking that goal (SimState::check_goal_line), the whole ball over (its radius), off the woodwork as for a shot, an own goal if the defending side played it last; a ball the keeper has met on the line (held, parried, tipped) is decided and not counted again. Found on the way: a tipped shot was turned on the line itself and always crossed it inside the posts; now the keeper meets it where he stands and turns it outside the post as far as its speed allows. A ground pass is read missed: if its line goes on into the passer's own goal it is a goal against (read::runs_into_goal). A player meeting a rolling ball now counts his acceleration and his reach (SpatialState::intercept). The keeper comes for their pass rolling into his area. Measured (30 seeds, not a target): goals 2.1 -> 5.2 a match (about 1.7 from passes rolling in before anyone reaches them, the cause open in OI-98), own goals 0.03, corners 23.6 -> 19.9. P1 every_goal_is_a_ball_over_the_line_between_the_posts, a_back_pass_that_runs_into_the_goal_is_seen_as_a_goal.

99: Plan L0-CLOSE step 11 (the corners, 27 a match against 10: 18 of them came from a completed pass of the defending side). A player going for a rolling ball ran to where it was, chasing it, never to where it would be: a keeper meeting a back pass ran behind it and it rolled past him, 1.7 m off, into his goal (and was told as a corner, OI-98). Now a player going for a ball rolling on the grass runs to where its path and his run cross (SpatialState::intercept, the ground friction of advance_ball and his own pace); a ball in the air, or just struck upwards, is met where it comes down as before. Measured (30 seeds, not a target): corners 27.3 -> 23.6, passes 880 -> 945, completed 85% -> 86%, tackles won 36 -> 38. Still open (OI-98): back passes rolling into the goal mouth; a ball crossing a goal line between the posts is told as a corner, not a goal (Law 10). P1 a_rolling_ball_is_met_on_its_path.

98: Plan L0-CLOSE step 11 (the game in the air, the deeper cause found at 97: high balls came down where nobody was). The clearance always went to the same point, 40 m up his own wing, to no one. Now the defender imagines where it comes down (long up his wing, through the middle, across to the other wing) and who gets there first, his teammates or theirs, as for a long ball (the flight, the spread of a long kick, each man's reaction and pace, ADR-052), and kicks it where it is worth most (ChoiceKind::Clear { to }); it leaves from the ball. No new number. Measured (30 seeds, not a target): aerial duels 3.7 -> 11.9 (real 35), throw-ins 50 -> 15 (real 45: clearances up the wing went out over the touchline), shots 18.9 -> 22.3, goals 1.7 -> 2.1. P1 the_clearance_goes_where_his_side_gets_there_first.

97: Plan L0-CLOSE step 11 (the game in the air, a first cause). Measured on five matches: 726 high balls, and when they came down to head height nobody stood within two metres in 80% of them (the nearest at a median 5.5 m). Two causes in how players read a ball in the air: a ball nobody plays to (a clearance, a goal kick, a header away) was read where it was, not where it comes down, and the man sent for it was the one nearest the kicker, at his next turn; and the point where it comes down was unknown until the ball was above head height. Now both sides read such a ball where it comes down, and the nearest to that point chooses at once; the drop point is read from the kick, as the ball climbs; a ball that never rises above head height has none. The remaining cause, deeper: at the decision the man to contest it is about eight metres away with about 1.2 s to go, and reaches it in four cases out of ten, because high balls come down where nobody is (where the clearance is aimed, who marks whom), open. Measured (30 seeds, not a target): aerial duels 3.6 -> 3.7, goals 1.5 -> 1.7. P1 a_ball_in_the_air_is_read_from_the_kick.

96: Plan L0-CLOSE step 11 (the pass into the run). A ground pass went to where the receiver stood when it was struck: running, he found it behind him. Now the passer plays it where the teammate he sees running will be when it arrives (read::lead_point): the pass is weighed to arrive rolling at pass_arrive_cm_per_tick (kick.json), so its flight over d takes ln(arrive / v0) / ln(ground keep) ticks with v0 = arrive + d x (1 - keep), the engine's own physics; the same point in his thought and in his kick; a teammate he does not see, or a lofted ball, goes where the man is. The pass leaves from the ball, where it is at his foot, not from the middle of his body. No new number. Found by the repertoire test (the keeper's claim no longer showed in seeds 42-56; it was 5 in 30 matches before, real two or three a match): a keeper came for a high ball only once it was already in his area, so he never read a cross coming in from wide; and the defender to contest a ball in the air chose at his next turn, often after it had come down. Now the keeper reads where a ball in the air for the other side comes down, and comes for it if it falls in his area; the one to contest it (the keeper for his area, the nearest outfield defender elsewhere) chooses at once, as the receiver does. Measured (30 seeds, not a target): passes 891 -> 887, goals 2.1 -> 1.5, the keeper's claims 5 -> 30 in 30 matches, aerial duels 2.4 -> 3.6. P1 a_pass_is_played_into_the_run; the repertoire test guards the claim.

95: Plan L0-CLOSE step 11, ADR-056 (every choice has a when; the maintainer: "non vedo protezione della palla per far alzare la squadra", "pensa a cosa pensa ed immagina un giocatore", "filtro luna"). Beside his options now the carrier weighs keeping the ball and playing at his next decision (ChoiceKind::Hold): he imagines that moment from what he sees (PerceivedScene::soon: everyone he sees, where they will be by his next decision, going as they go), judges it with the same thought, every option, and it is worth what he keeps (the pressure on him against his shield, GroundTruth::shield_confidence) times how far he trusts what he foresees (his game reading, PerceivedScene::foresight); it takes the visibility of the option he waits for. Holding, he stays on the ball until his next decision. No new number. Measured (30 seeds, not a target): about fifty to seventy waits a match, passes 911 -> 891, goals 1.6 -> 2.1. Twin test the_stronger_carrier_is_surer_to_shield_the_ball; P1 he_weighs_holding_the_ball_while_his_team_comes_up.

94: Plan L0-CLOSE step 11 (the maintainer, 2026-11-13, watching the GUI: "ci sono dei momenti in cui i giocatori invece di conquistare il pallone gli gravitano intorno"). The carrier could touch his own ball again only within his body's width (45 cm), while he wins a loose one within his reach (about a metre, from his reactivity), and the bodies around a ball keep each other 90 cm apart: he circled it for seconds, a teammate beside it unable to take it (in five matches, 40 stretches of two seconds and more with the ball over 1.3 m from its carrier and players around it). Now he touches it again where it is as soon as it is within his reach, the same reach he wins a ball with; a touch rolling ahead is still anyone's until his touch meets it; a ball he wins within his reach but not at his feet he still brings under his foot in a step, it does not jump to him. Stretches 40 -> 17. Measured (30 seeds, not a target): tackles won 52 -> 34, goals 1.3 -> 1.6. P1 he_plays_his_ball_within_his_reach.

93: Plan L0-CLOSE step 11 (the maintainer, 2026-11-13: in the GUI "the ball seems drawn to the players instead of won"; "se normalizziamo tutto non c'è più il gioco"; first_touch is the measure of the control). A player who won the ball within his reach became the carrier and the ball jumped to the middle of his body the next tick. Now his first touch takes it where it is: it keeps first_touch_keep_max x (1 - first_touch) of the speed it came with (reception.json, 0.15, declared) and runs on the way it came, he goes after it, and until his foot is on it again it is anyone's; a dead ball, or one he kills, he brings under his foot in a step. Measured (30 seeds, not a target): tackles won 24 -> 52 (a poorly controlled ball is taken by whoever arrives first), goals 2.2 -> 1.3. Twin test the_better_first_touch_keeps_the_ball_closer; P1 a_ball_won_is_taken_where_it_is. Found by the test of Laws 15-17: a ball that went out under the foot of a player who had just won it stayed out, because the restart could not move a possessed ball and the out-of-play check skipped it; out is now where the ball is, whoever has it (checked again after the carrier moves), the last touch is the carrier's, and the restart frees the ball first.

92: Plan L0-CLOSE step 11, the realism derived (what a cross is). A cross was told only for a lofted pass from wide into the area; the real definition (Opta: a ball played from a wide position into the area near the goal) counts the low cross and the cut-back too. Every pass from outside the width of the penalty area into it is now told as a cross, a keeper's throw excepted (state::is_cross). Measured (30 seeds, not a target): crosses 11.1 -> 23.1 a match (real 35). The measuring sheet's "interceptions" row (real 20) compared every pass the opponents took, read or misplaced, against Opta's interceptions alone: by the same definition (passes lost, interceptions plus recoveries) the engine has 157 against about 160 (900 passes, 18% lost). Corrected in the sheet, no engine change. P1 a_ball_from_wide_into_the_area_is_a_cross.

91: Plan L0-CLOSE step 11, the realism derived (the shot is a spread from the contact, ADR-054). The miss is no longer one known offset: its width is the engine's angle, and where in it the ball goes is the phase of the shooter's stride when his foot meets the ball (AgentState.gait_milli, moving on with the ground he covers; a step is top_speed_max over step_hz). Mid-step clean; late it runs away from the post he chose and rises; early it is pulled back across and kept low; the run's part drags it the way his body moves. He knows the width as he reads it, not the side, and aims so his widest reckoned miss meets the inside of the post (before he kept half as much again). kick.json: error_ref_rad 0.08 -> 0.062 and vertical_share 0.5 -> 2.0, both derived from Shimotashiro and Shinya 2024 (11 m instep kicks, 95% ellipse 6.17 m2); before, arbitrary. Measured (30 seeds, not a target): shots on target, goals plus saves, 14.6 -> 10.6 of 20.7 (51%, real 34%), goals 3.8 -> 2.2. P1 the_contact_spreads_the_shot.

90: Plan L0-CLOSE step 11, the realism derived (the shooter reckons his own miss as he reads the game). Since 88 the causes of his strike he reckons (on the run, tired) are weighed by his game reading; his base miss (finishing and the better foot) was known exactly. The same rule now covers it: KickBalance::miss_reckoned = game_reading x error_ref x (1 - skill); a poor reader underrates his miss and aims tighter. No new number. Measured (30 seeds, not a target): shots on target by the real definition (goals plus saves) 14.4 -> 14.6 of 20.8, about 70% (real 34%): the cause of the gap is not in his reckoning. The first count of 89 (about 90%) also took in blocked shots; corrected. Twin test the_poorer_reader_underrates_his_miss.

89: Plan L0-CLOSE step 11, the realism derived (the touch, ADR-053). The ball is no longer glued to the carrier's feet: carrying it somewhere, he plays it ahead and meets it again, the touch weighed by the ground friction to come back to his foot after 1 + 5 x pace/top x (1 - dribbling) steps at 4 a second (movement.json touch_steps_max, step_hz, declared). Between touches it rolls on its own, can roll out of play, and he goes to it if he turned; he passes, shoots and clears only with it at his foot; a defender who reaches it then wins it, the carrier's force in the contact zero (ADR-042 amended). Holding it, standing, in his hands or at a restart, it is at his feet. Measured (30 seeds, not a target): tackles won 7.6 -> 23.4, goals 6.2 -> 3.3, corners 25.7 -> 21.2. Shots on target unchanged (about 90%): the strike is at his foot, as in a real match; the cause is elsewhere. Twin test the_better_dribbler_touches_the_ball_more_often; P1 a_touch_comes_back_to_his_foot, a_ball_between_two_touches_is_the_defenders.

88: Plan L0-CLOSE step 11, the realism derived (the shooter reckons). The shooter takes into account the causes of 87 as well as he reads the game (ADR-052: he misjudges in proportion to how poorly he reads it): how fast he is running and how tired he is, his balance (`GroundTruth::strike_error_he_expects`, `PerceivedScene::strike_error_rad`), and aims inside the post by them too (`read::aim_inside`). Measured, as expected: more shots on target (64% -> 80%), 6.2 goals a match. It says the remaining gap is a cause the shooter cannot foresee: in the engine the ball is at the carrier's feet, every strike a perfect contact; in a real match it is a step ahead, rolling, touched at the rhythm of the run. Next.

87: Plan L0-CLOSE step 11, the realism derived (the first cause). Measured: 95% of the shots were on target (real: about 34%). Cause found: the shot's error depended only on the shooter's skill, foot, power and pressure, the same in the same scene, and he knew it, aiming inside the post by it; nothing in the engine made a strike less clean than he expected. Three causes a real strike has, added, each from what the player is doing: on the run (`run_error_rad` x his speed / `run_speed_ref_cm_per_tick` x (1 - body_control)), across his body (the same term grows by the sine of the angle between his run and the shot), and fatigue dimming his technique (finishing and the foot, by the share fatigue dims his acceleration: VISION §3, the dimmer). `ShotContext` gains `run_cm_per_tick`, `across`, `fatigue`. The new values are declared arbitrary, set once, not tuned. Seeds 42-71: 64% on target (was 95%), 6.1 goals, 36 shots a match. The shooter's own reckoning does not include these causes yet. The repertoire test gets a declared list of rare acts (the red card: about one match in fifteen in the engine, one in seven in real football), guarded by their own unit tests rather than required in fourteen matches.

86: Plan L0-CLOSE step 10e, the body: turning. `joint_mobility`, a new physical characteristic, sets how well a player turns: changing direction by more than 60 degrees he brakes and turns with `turn_share_min`-`turn_share_max` of his braking (`movement.json`, declared), the stiff player needs longer. Profiles: wide players the most mobile, centre backs the least. Twin test: the mobile player turns quicker. The jump's timing stays open (its root, reading the ball's flight, comes with the moment the ball is met). A derivation of 77 corrected (the repertoire test found it: no save was held any more): the keeper's difficulty to hold a ball is (offset / reach + speed / catch_speed_ref) / 2, because `catch_speed_ref` is defined as the speed at which a ball at the edge of his reach needs perfect hands, both causes together; at 77-85 each cause alone demanded perfect hands. No statistic was looked at.

85: Plan L0-CLOSE step 10d, in goal (first part). With the ball in his hands (a held save, a claim) nobody may challenge the keeper (Law 12), and he throws it to a teammate within `throw_max_cm`, at most `throw_speed_cm_per_tick`, its precision his hands' with both hands (`PassContext::throw`); farther he kicks it. Tried and declared open, each with the cause found missing (none was tuned): the set position (a keeper caught moving reaches less: he is almost always moving, following the ball, and a set position held while the shooter is in range leaves him behind; the moment the shot is shaped, when a keeper plants his feet, does not exist yet); the depth read from the shooter's options (optimised on the imagined chance it went to the extremes, where the engine's shot beat him: the thought and the engine disagree, as the lob did); coming out for the pass that ends in his area (an attacker took it earlier on its way and the goal was empty: the race along the path is missing). The divergence guard found the three. No statistic was looked at.

84: Plan L0-CLOSE step 10c, without the ball (second part). The deflected pass: an opponent a pass reaches controls it by the same touch as a receiver (`resolve_reception`: his first touch, the pass's length, how free he is of the passer's teammates); not controlled, he only deflects it, the ball runs on slower for whoever, and a `Block` event with `pass: true` is emitted (`MatchState::deflects_the_pass`). Before, any opponent a pass reached controlled it. The offer reads the free line: the player getting free of his marker goes, among the moves away from him (straight, turned 45 or 90 degrees, fixed rotations), to the one where the line from the carrier is most open (`TeamView::act_target`, Offer). No statistic was looked at.

83: Plan L0-CLOSE step 10c, without the ball (first part). Two new acts of the defender's thought (ADR-052 §5): Cover, with a teammate pressing the carrier, the space `cover_depth_cm` behind him on the carrier's line to our goal, worth half its danger (if the presser is beaten: he cannot know which) when he gets there in time; Intercept, a step (`lane_step_cm`) in front of his man into the passing lane, worth half the man's danger (it cuts out the ground pass, not the ball over him; goal-side marking guards both). Two new psychological characteristics with their consumers: work rate, the share of the pace his side needs he gives without the ball (`work_rate_floor` in `movement.json`); discipline, how much the place he holds in the shape weighs for him (`discipline_floor_milli` in `offball.json`). All declared. Twin tests: the disciplined player keeps his place, the generous one runs more; P1: behind a presser he covers, near the lane he steps into it. No statistic was looked at.

82: Plan L0-CLOSE step 10b, the rest of the thought (ADR-052 e): three cognitive characteristics, each with its first consumer and its twin test. Anticipation: how far ahead he imagines (`PerceivedScene::horizon_cm`, `GroundTruth::horizon_cm`, between `horizon_min_cm` and `horizon_max_cm` in `thought.json`, declared); an option that ends beyond his horizon he cannot see through, and his confidence in it falls towards a half, the more the farther (`read::foresee`, in every option's worth). Creativity: the options few would see (the through ball, the long ball over the lines, the lob) he notices as much as he is creative, and the read filter of `resolve_choice` does the rest. Concentration: as he tires his decision slows, the more the less concentrated he is (`resolve_latency`: latency x (1 + (1 - concentration) x fatigue); mental fatigue goes with the physical and slows decisions, Smith et al. 2016). Profiles: strikers and centre backs anticipate, central midfielders create, keepers keep their concentration. No statistic was looked at.

81: Plan L0-CLOSE step 10b, vision (ADR-052 e), with its first consumer. `vision`, a new cognitive characteristic, sets how often a player glances around (`GroundTruth::scan_interval_ticks`): between `scan_rate_min_per_s` 0.2 and `scan_rate_max_per_s` 0.8 a second, the rates measured in elite players' visual exploratory activity (Jordet, Bloomfield and Heijmerikx 2013; McGuckian et al. 2018), staggered by player; between glances his gaze is on the ball. Before, everyone scanned every 5 ticks, twice a second. Profiles: central midfielders see the most. Twin test: the player with the better vision looks around more often. The plausibility guard `score_is_plausible` becomes `no_match_diverges` (at most 20 goals in any match), on the squads profiled by role: it guards against a match that runs away, it is not a target (the realism is step 11); the repertoire test plays fourteen seeds so that the rare acts show. No statistic was looked at.

80: Plan L0-CLOSE step 10a, the carry. The direction is his: he imagines carrying the ball towards goal, or turned 35 or 70 degrees to either side into the space beside a man (`read::carry_directions`, fixed rotations, no trig), each worth the space it reaches against who could get there first, as far as the room the opponents in front leave in that direction (`read::carry_salience`); `ChoiceKind::Carry` takes him there. The speed is his: with the ball he keeps `with_ball_share_min`-`with_ball_share_max` of his sprint, read from dribbling (`movement.json`, declared, to be sourced). No statistic was looked at.

79: Plan L0-CLOSE step 10a, the through ball (ADR-051, the pass in its kinds). The carrier sees which teammates are attacking the depth and where they run to (`PerceivedScene::runs`, from their movement, Plane B); for each he imagines the ball played to where they arrive together: the run's point pushed forward by the ground the runner covers, at the pace he believes, while the ball rolls there (`read::through_candidates`); it is worth who reaches it first, as any pass (ADR-052). `ChoiceKind::Through` plays the ball to that point, the runner goes to meet it, offside is judged at the pass. The Pass event carries `through: true`. The aerial duel contested shoulder to shoulder (`MatchState::aerial_contenders`): when one player reaches the ball in the air, any other a body's width beyond his own reach who gets up to that height contests it (since 78 the bodies accelerate and brake like real ones, are kept apart, and the duel had disappeared). No statistic was looked at (the maintainer: the engine first, the realism after).

78: The body, rebuilt on a professional's figures, and the pace is the player's (plan L0-CLOSE step 10, cause 1; the maintainer: "evaluate the acceleration too"). Four defects of the movement found and fixed: (1) a diagonal ran 1.4 times the top speed (the step was applied in full on both axes; now he moves in a straight line); (2) nobody braked or turned: he stopped dead and reversed at full speed (now he brakes by `decel_min`-`decel_max`, read from body control, to stop at his place and to turn); (3) top speed and acceleration were the reference times the characteristic, so an average player reached 4.5 m/s accelerating at 1.5 m/s2 (now the characteristic places him in a professional's range: 7.5-10 m/s, 2.5-5 m/s2, Harper, Carling and Kiely 2019); (4) every run was flat out. The pace is now his: into a contest (pressing, going in, going for the ball, running in behind, carrying it) flat out; out of possession he must be in his place before the ball can get there, so he goes as much faster, against a player as fast as he is, as his place is nearer to him than to the ball; with the ball he takes his place in about `reposition_ticks`; the keeper keeps his angle at every moment. The off-ball race reads the same top speed. Nothing was tuned on a statistic. Seed 42: 9.5 km per outfield player, 60/26/9/3/1% of the time at 0-2/2-4/4-6/6-8/>8 m/s (real: about 10-11 km, 60/25/13/2), fatigue 0.54 at the end. Seeds 42-71: 29 shots a match (real about 25), 86% on target (real about 34%: the shot's error is the next cause), 7.8 goals.

77: The statistics are derived, never adjusted (the maintainer, 2026-11-13; ADR-052 note, `context/RULES.md` A5-bis). Three values chosen on 2026-11-13 by looking at the statistic they produced are re-founded. (1) The keeper holds the ball when his handling reaches offset / reach + speed / catch_speed_ref, each cause at its reference demanding perfect hands as the two values are defined; the weights `catch_offset_weight` and `catch_speed_weight` (chosen for 60% held) are removed. (2) The keeper's reach the shooter imagines over a lob is computed from the reference keeper's height, his arms and an average jump (`save.json`, `duel.json`); `believed_keeper_reach_cm` is removed from `thought.json`. (3) Fatigue's `gain_per_tick` and `v_max_coefficient` are computed from a real match (`FatigueBalance::load`): an average player who runs a professional match's activity (Bangsbo, Mohr and Krustrup 2006) ends it with the fatigue that dims his high-intensity running by the measured 30% (Mohr, Krustrup and Bangsbo 2003). Finding, left as it is: the engine's players end at about 0.9, because they run more than real ones (13.2 km a match, a third of the time above 4 m/s, against about 10-11 km and 15%): nobody chooses his pace yet, every run is at full speed. Seeds 42-71: 8.2 goals, 64 shots a match.

76: Plan L0-CLOSE step 8, the repertoire test (`ds-match/tests/repertoire.rs`, ADR-051 §5): over seeds 42-51 every act of the catalogue shows at least once in the events or the tick record; the acts that cannot show yet are declared with the reason (turn, stop, cover, get set, spill, shield as its own event, the long-pass kinds), and the test fails if one of them shows. Making it green found and fixed, each a thought that did not match the engine (ADR-052) or a law of physics: (1) every save was held (the difficulty multiplied reach and speed; now they add: `catch_offset_weight`, `catch_speed_weight` in `save.json`; about 60% held, the rest parried); (2) the lob was four shots in five (the thought ignored that the keeper takes it over his head; now it imagines the arc with the engine's gravity against `believed_keeper_reach_cm`); (3) the pressure on the kick was one formula in the engine and another in the shooter's mind; now both read `kick_pressure_radius_cm` (an outfield opponent within 15 m hurries the kick, less the farther), and the shooter reckons with it; (4) the shooter aimed so that his expected miss landed on the post; now he keeps half as much again, and the ball and the post, inside it; (5) a pass in the air was contested by nobody: the opponent nearest where it comes down goes to meet it, and a player going to a high ball goes to its drop point (`SpatialState::drop_point`); aerial duels came back (0 to about 3 a match). The pass has its own reference error (`pass_error_ref_rad`). The shot types `lob` and `header` are new. Seeds 42-71: 7.1 goals, 74 shots, 56 saves, 13 crosses, 16 corners a match: the conversion (10%) is real football's, the volume of shots is not; it is the fine calibration of step 10 (the lever: what keeping the ball is worth against a goal, `thought.json` `possession_value`).

75: No composite is stored (maintainer, 2026-11-13: every characteristic must derive from something). The prototype composites `aerial` and `set_pieces` are removed from the latents and the profiles. The set-piece taker is chosen by a quality derived from his roots: finishing (a penalty) or passing (a corner, a free kick) times the quality of his better foot. The legacy aerial branch of `resolve_duel` reads heading (the aerial duel itself reads heading, the jump, strength and height since 71). Other players' values are unchanged (each characteristic has its own key).

74: Plan L0-CLOSE step 7, set pieces, and three bugs found on the way. The taker is chosen by his team (`MatchState::restart_taker`, `GroundTruth::set_piece_taker`): the keeper takes his goal kick; a corner or a free kick goes to the best deliverer (passing and `set_pieces`), a penalty to the best finisher at a dead ball (finishing and `set_pieces`), among the outfield players within 30 m; a kick-off and a throw-in to the nearest. The distances of the Laws are applied (`MatchState::clear_the_spot`): at a free kick or a corner the opponents stand 9.15 m from the ball; at a penalty everyone but the taker and the keeper stands outside the area and 9.15 m behind the mark, the keeper on his line (Law 14). A dead ball is not challenged and not pressed; a penalty is only a shot (`PerceivedScene::penalty_kick`) and is kicked at once. The shooter knows his own foot (ADR-052): he aims inside the post by the miss he expects of himself at that distance and power (`build_choice`'s `own_error`). Bugs: (1) the carrier recognised the opposing keeper by `slot() == 0`, true only of the home keeper, so a home shooter saw the away keeper as a defender in the way (the shots split home 47%, away 53% after the fix, symmetric); (2) the foot that kicks was read from the pitch's side, not the shooter's frame; (3) the keeper on his line counted as pressure on the kicker. Seeds 42-71: 2.6 goals (1.3 home, 1.3 away), 100 shots (too many), 20 corners, 14 free kicks a match.

73: Plan L0-CLOSE step 6, fatigue (OI-70). Fatigue has a producer: each tick a player tires by his effort (his speed squared over a reference sprint) against `endurance`, a new physical characteristic, and recovers when he walks (`balance/fatigue.json`, `ds_calculator::fatigue`, `MatchState::tire_players`); at half-time he recovers a share of it. It dims the body: acceleration (ADR-030, already wired) and now top speed (`v_max_coefficient`). It is observed: the tick record carries `fatigue_milli`. Seed 42: mean fatigue 0.34 at half-time, 0.24 after the rest, 0.50 at the end. Seeds 42-71: 1.6 goals, 66 shots, 19 dribbles a match. Twin test: the enduring twin tires less and is faster at the end. Declared gap: the thought still believes his own pace fresh (the run in behind reads `max_speed`).

72: Plan L0-CLOSE step 5, the carrier answers (ADR-042 step c). When a defender goes in standing or sliding, the carrier chooses how to answer, by what he imagines (ADR-052): shield (his keeping force plus `shield_force x maximal_force` and `shield_body x body_control`) or take him on (`dribbling`, a new technical characteristic, explosive force and body control). His confidence is his own force for that answer against an average defender (`answer_trust_milli`); the gain is the space he keeps or the space 4 m past the man (`MatchState::duel_stakes`), the loss what the opponents would have where he is, weighed by his risk management. A man beaten by a dribble is out of the play for `beaten_ticks` and a `Dribble` event is emitted. Profiles: wide players and strikers dribble best. Seeds 42-71: 1.8 goals, 70 shots, 20 dribbles, 5 tackles a match (to calibrate at step 10). Twin tests: the dribbler beats his man, the strong carrier shields surer.

71: Plan L0-CLOSE step 4, in the air. A ball in the air meets the players it passes within reach of (`MatchState::meet_ball_in_air`): horizontally their reach for a passing ball (reactivity), in height their height plus their jump (`explosive_force x jump_ref_cm`; a keeper in his area adds his arms, `hands_reach_cm`). Two or more meet it: an aerial duel (a `Duel` event with `aerial: true`), won by `heading` (a new technical characteristic, with its first consumer), the jump, `maximal_force` and the height (`DuelBalance::aerial_force`, `GroundTruth::resolve_aerial`); the keeper's hands add `keeper_hands_bonus` in his area. The winner heads it at goal in the opponents' area (a shot, from the head, met by the keeper as any shot), away from his own goal in his half (a `Clearance` with `header: true`), down to his feet elsewhere; a keeper in his area claims it (a `Save` with `claim: true`). **The keeper's standing reach** is `standing_reach_share` (0.75) of his height from the centre of his body (his arm span is about his height, plus a step; it was half his height, a keeper with his arms down), in the engine and in the shooter's judgment of a chance (`save.json`); a plausibility guard (10.8 goals a match on the engine's own test fixture) found it. Seeds 42-71, per match: 1.70 goals, 64 shots (too many), 42 saves, 11.0 corners; the fine calibration comes at the end of the task. Twin test `the_better_header_wins_in_the_air`; P1 `a_ball_in_the_air_can_be_headed`. **Golden changed.** `SIM_VERSION` 70 -> 71. Related: ADR-042, ADR-051, ADR-052.

70: ADR-052, plan L0-CLOSE step 3: the post, the lob and the clearance are judged by the same measure. **The post**: for each post the chance is computed with the keeper where the shooter sees him, off his line (narrowing the goal) and leaning to one side (closing that post: `thought::chance` takes his lateral shift); `other_side_factor` is no longer read. **The lob** (new, `thought::lob_chance`): over a keeper off his line, a soft high shot (`power_lob`, contact high) goes in if the keeper cannot get back before it comes down (his run back, at the speed the shooter believes, against the lob's flight) and if it falls inside the goal. **The clearance** is worth the contested space where it lands, for whoever gets it, plus the danger it takes away from where he stands, as much as he is pressed (the same danger a defender reads); without that term no clearance was chosen in ten matches (`clear_salience` is no longer read). Seeds 42-71, per match: 4.3 goals, 52 shots, 7.4 corners, about 1,430 passes, 40.6 clearances (real about 20): the far post and the lob make the shot worth more than before, and the chance measure is still generous (a measure for the fine calibration at the end of the task, maintainer). P1 `a_keeper_off_his_line_can_be_lobbed`, `the_far_post_from_the_keeper_is_better`. **Golden changed.** `SIM_VERSION` 69 -> 70. Related: ADR-052, ADR-046, ADR-047.

69: ADR-052 step (d), second part: the attack without the ball thinks (plan L0-CLOSE step 2). With a teammate on the ball, a player occupies the space that gives his side the most, as free as he would be there (`MateBall::values`, from `read::space_value` and `read::soon`): **holding** the space he has, **offering a line of pass** in the space away from his marker, or **attacking the depth** to the last line, if he wins the race to it: his run at his own pace (`max_speed`, which he knows) against the nearest defender's at the pace he believes (`ThoughtBalance::race`, twin test `the_fast_twin_wins_the_race`). The weights `offer_salience_milli`, `run_salience_milli`, `run_reach_cm`, `run_room_ref_cm` and `OffBallBalance::run` are removed. Seeds 42-71, per match: 1.43 goals, 34.5 shots, 2.7 corners, about 1,410 passes, **4.03 offsides** (were 0.30: the runners now go for the space when they win the race, and sometimes are beyond the line when the ball is played; real about 4). **Golden changed.** `SIM_VERSION` 68 -> 69. Related: ADR-052, ADR-049, Law 11.

68: ADR-052 step (d), first part: the defence thinks (plan L0-CLOSE step 1). With an opponent on the ball, a defender's act is the one that takes away the most danger (`GroundTruth::defend`), the danger being the value of a space to the opponents (`read::space_value`, the same measure the carrier uses): **press** takes away the carrier's danger if he gets there in time (and not when already in reach: then he goes in or jockeys, ADR-042); **challenge** takes it away as much as his tackle (or the keeper's hands) is good; **mark** takes away the danger of the man nearest to him; **hold** keeps closed the space he guards in the team's structure, a danger as much as an opponent can get into it soon (`read::soon`). A teammate nearer to a danger takes it; the second doubles up as much as he reads the game. The defensive weights of `offball.json` (`hold_salience`, `challenge_salience`, `mark_salience`) are no longer read when an opponent has the ball. Seeds 42-71, per match: **1.67 goals, 23.9 shots** (were 7.2 and 55 with the attack thinking alone), 20.3 fouls, 6.3 yellow cards, 6.7 tackles won (real about 17), 117 interceptions, 3.6 corners, 2.0 crosses, about 1,700 passes (real about 1,000: step 2). **Golden changed.** `SIM_VERSION` 67 -> 68. Related: ADR-052, ADR-049, ADR-042.

67: ADR-052 step (c), first part: the carrier thinks. Each option's worth is no longer a hand-set salience but what he imagines it leads to, from his scene (`ds-orchestrator::read`, on `ds-calculator::thought`): the **pass** is imagined weighed to arrive rolling, and the forecast says who reaches it first along its path (the players he perceives, at the speed and reaction he believes they have, `thought.json`); the **lofted pass** who is first where it lands, after its flight, less the spread of a long kick; the **carry** whether he reaches the space ahead before the nearest opponent; the **shot** the chance quality of his space, read with the keeper where he sees him (off his line he narrows the goal, `thought::chance` now takes his distance off the line) and the defenders who could reach the line of the shot before the ball. Every option is then worth `confidence x the value of the space it reaches - the rest x the value the opponents would have where it is lost x (1.5 - risk appetite)`, in units of keeping the ball (`ThoughtBalance::space_value`: keeping it, more the nearer the goal, plus the space's chance quality). The per-option risk terms are gone (the risk is in the worth). **Still on their old weights, declared:** the clearance's salience, the other-side factor of the shot, the choice formula's reading weight (`choice.json`), and the off-ball choices (step d). **Measured, seeds 42-71, per match: 7.23 goals, 54.6 shots** (real about 2.7 and 25), 7.6 corners, 4.9 crosses, about 1,350 passes, 6.5 blocks. Expected by the ADR's order: the attack thinks, the defence still holds its places on weights; the off-ball choice is next. **Golden changed.** `SIM_VERSION` 66 -> 67. Related: ADR-052, ADR-050, ADR-040.

66: ADR-051, the family "the kick": the lofted pass (the long ball, the switch, the cross). A new candidate of the carrier's choice, `ChoiceKind::Loft`, to a free teammate from `loft_min_cm` away: it goes over the opponents (the lane does not count, and nobody on the line cuts it out at launch), reaches farther (fading out at `loft_max_cm`), and is less precise and harder to control (`loft_salience`). It flies `loft_ticks_per_10m` ticks per ten metres and is lifted by gravity to land at the receiver (`kick.json`); it is controlled once it lands. From a wide position into the opponents' penalty area it is told as a `Cross` (emitted for the first time). Measured for the weight on seeds 42-71: 0.6 gave about 4 lofted passes a match, 1.4 gave 33 corners; 1.2 is kept. Seeds 42-71, per match: 2.13 goals (real about 2.7), 31.0 shots, 4.6 corners (about 10), 4.2 crosses, 35.3 throw-ins, 0.33 penalties, 17.9 fouls, 5.3 yellow cards, about 1,400 passes. **Golden changed.** `SIM_VERSION` 65 -> 66. Related: ADR-051, ADR-050.

65: The restarts and the woodwork are told (L0.05 1.2, the maintainer's decision, 2026-11-13). Three kinds join the closed event set: `ThrowIn` (Law 15), `GoalKick` (Law 16) and `Woodwork` (a post or the crossbar, ADR-048 §1); the restart's taker is the actor. Before, the engine applied every restart but told only the corner, and the maintainer, watching, "never saw a throw-in": they happened unseen (about 22 throw-ins and 15 goal kicks a match). The woodwork's declared debt (`Removed-by: the woodwork event in L0.05`) is paid. No change to the play: the chain gains events, so the golden changes. P1 `the_restarts_are_told`. **Golden changed.** `SIM_VERSION` 64 -> 65. Related: L0.05, ADR-048, ADR-021.

64: ADR-042 step (b): the foul, and Law 12 applied objectively (the referee is the Laws, ADR-021 note). **The foul is a fact of the contact** (the band, ADR-042 §4, `DuelBalance::foul`): a ground contact the defender did not win is a foul when `foul_slide` (a slide) + `foul_behind` (from behind the carrier, geometry) + `foul_temper x aggressiveness x (1 - emotional_control)` + `foul_speed x approach speed / speed_ref` + `foul_technique x (1 - tackling)` reaches `foul_threshold`; a won contact is clean. **Law 12** (`rules::card_for`, thresholds in the new `balance/laws.json`): a slide from behind at speed is reckless (yellow); a slide from behind at a sprint is excessive force (red); a foul that denies an obvious goal-scoring opportunity (near the goal, no defender but the offender, the keeper included, nearer to it) is a red; a second yellow is a red; a red sends the player off (`SpatialState::send_off`: off the pitch, no longer an agent; the record still lists him). Advantage: the side that suffered it keeps the ball in the opponents' half, outside the area: play goes on (the card is shown at once). Otherwise a free kick where it happened, or a **penalty** (`RestartKind::Penalty`, from the mark, Law 14) inside the area. Events `Foul`, `Card` (yellow or red), `FreeKick` and `Penalty` are emitted for the first time. **The players read it**: in his own area a defender is careful (the slide's salience times `area_slide_caution_milli`), and so is a player already cautioned (`booked_slide_caution_milli`). **New characteristic**: `emotional_control` (psychological). Calibration (L3, declared), measured on seeds 42-71: `foul_threshold_milli` 800 (600 gave 49 fouls a match), `excessive_speed_milli` 1150; before the cautioned player's care, 2.7 of the 4 red cards a match were second yellows. Seeds 42-71, per match: 19.9 fouls (real about 22), 5.67 yellow cards (about 4), 0.30 red cards (0.1-0.2), 1.13 penalties (about 0.3: too many), 8.9 free kicks, 1.70 goals, 45 shots. Twin tests `the_hothead_twin_fouls`; P1 `from_behind_is_a_foul`, `the_card_follows_the_facts`, `the_laws_call_fouls_and_cards`. **Golden changed.** `SIM_VERSION` 63 -> 64. Related: ADR-042, ADR-021, ADR-051.

63: ADR-042 accepted (2026-11-13, with the amendment of the same day), step (a): the defender chooses how to go in, and the contact has four outcomes. **How he goes in** (`DuelBalance::choose`, `GroundTruth::choose_tackle`): standing (`standing_salience x (0.5 + 0.5 x tackling)`), slide (`slide_salience x aggressiveness`: it reaches `slide_reach_milli` of the standing reach and adds `slide_bonus` to his force, but if it does not win the ball he is on the ground for `slide_recovery_ticks`), or jockey (`jockey_salience x (1 - aggressiveness) x (0.5 + 0.5 x game_reading)`: he stays in front and waits, no contact). The off-ball choice to challenge now opens within a slide's reach. **The forces read the bodies**: the defender's tackling, explosive force, `maximal_force`, `body_control`; the carrier's first touch, explosive force, `maximal_force`, `body_control`. **Four outcomes** (`DuelBalance::contact`): the defender wins if his force exceeds the carrier's by more than `loose_margin`, the carrier keeps it if his does, and in between the ball comes **loose**, running from the contact away from the defender (a `Duel` event with `loose: true`). **New characteristics** with their first consumer (ADR-051 §4): `maximal_force`, `body_control` (physical), `aggressiveness` (psychological, the first of its family); profiles by role in `profiles.json`. Calibration (L3, declared): `duel.json` `retry_ticks` 30 -> 50, `slide_reach_milli` 1400, tackling and first touch 0.45. Seeds 42-71, per match: 0.97 goals, 35.1 shots, 9.5 saves, 0.73 corners, 19.7 tackles won, about 1,240 passes. The foul (Law 12) is step (b). Twin tests `the_strong_twin_keeps_the_ball`, `the_aggressive_twin_slides`; P1 `close_forces_leave_the_ball_loose`. **Golden changed.** `SIM_VERSION` 62 -> 63. Related: ADR-042, ADR-049, ADR-051.

62: Law 11, offside, as an exact rule (the referee is the Laws, ADR-021 note 2026-11-13). `rules::offside_position`: a player is in an offside position when the ball is played if he is in the opponents' half and nearer to their goal line than both the ball and the second-last opponent (the keeper counts; level is onside). When a pass is played, the passer's teammates in an offside position are noted (`MatchState::mark_offside_at_pass`), except on the first pass after a throw-in, a goal kick or a corner; if one of them then plays the ball, it is an offence: a free kick to the defenders where he is (`RestartKind::FreeKick`, `rules::free_kick`), emitted as `FreeKick` with `reason: "offside"` (the L0 event set has no offside kind; the narrator says "Offside: free kick for ..."). Seeds 42-71, per match: 0.30 offsides (real about 4: the runner stops on the line by his own reading, and the timing of a run is not modelled yet), 1.33 goals, 38.8 shots, 0.80 corners, about 1,260 passes. P1 `offside_is_beyond_the_second_last_and_the_ball`. **Golden changed.** `SIM_VERSION` 61 -> 62. Related: ADR-021, ADR-051, issue #13.

61: Corners and the keeper, ADR-051. **Tipped**: a ball the keeper reaches only at the edge of his reach (beyond `tip_share_milli` of it) is tipped, not parried: his touch turns it and it goes on past the post (a corner when it crosses the goal line); nearer him, a parry pushes it back into play (`SaveOutcome::tipped`). **A block reflects off the body** by where the ball meets it (`ds-calculator::obstacle::reflect_off_body`: square on it comes back, grazing it goes on turned), not always straight back at the shooter. **Calibration** (L3, declared): `save.json` reaction 0.3-0.6 s (was 0.2-0.5), dive 3.5 m/s (was 4.5): the keeper stopped about 90% of the shots on target (real about 70%); `shot_choice.json` `aim_edge_fraction` 0.9 -> 0.8 (the aim about 73 cm inside the post). **Measured on 30 seeds (42-71) per match**, the first measure on more than ten: 1.47 goals (real about 2.7), 38.9 shots (about 25), 9.0 saves, 7.0 blocks, 0.73 corners (about 10). Corners stay few: they come mostly from crosses and clearances out of play, which do not exist or rarely leave the pitch yet. P1 `a_body_reflects_by_where_the_ball_meets_it`. **Golden changed.** `SIM_VERSION` 60 -> 61. Related: ADR-051, ADR-048.

60: Two causes of the few goals, measured shot by shot (seeds 42-46: 182 shots, 50 on target, every one that reached the keeper passed within 0-170 cm of him, rolling at about 13 m/s). **Both sides of the goal** (ADR-046 §1: two phrases per side): the shot is offered to both posts, the side of his other foot at `other_side_factor` (0.8) of the salience and kicked with that foot; the keeper hides the side he covers (ADR-047 §3), so the shooter can pick the post away from him; before, only his foot's side existed and he shot at the keeper. **The strike reaches the height he aims at**: the shot leaves rising by half of what gravity takes away during the flight (`ShotContext` gains the ball's gravity); before, the aim ignored gravity, a low shot hit the grass at once and rolled to the keeper at half its speed. Seeds 42-51: 18-60 shots, 3-13 saves, 0-17 blocks, 14-31 clearances, 14 goals in ten matches (1.4 a match; real about 2.7), corners still 0 (blocks and parries rebound into the pitch). P1 `the_shooter_can_aim_at_either_post`. **Golden changed.** `SIM_VERSION` 59 -> 60. Related: ADR-046, ADR-047, ADR-051, ADR-048.

59: ADR-051, "clear": near his own goal and under pressure a carrier may kick the ball away, a candidate of his choice (`ChoiceKind::Clear`): `clear_salience x pressure x (1 - distance / clear_zone_cm) x (1 - 0.5 x appetite)` (`shot_choice.json`): the more he is pressed and the nearer his goal, the more he clears, and a cautious player more than a bold one. The clearance is his strongest kick, lifted (`kick.json` `clear_loft`), towards a point `clear_length_cm` up the pitch on his side's wing, to no one (`GroundTruth::resolve_clearance`, `MatchState::play_clearance`); the `Clearance` event is emitted for the first time. `PerceivedScene` gains the goal he defends. Weight measured: 1.0 never won, 3.0 gave 25-40 clearances a match; 2.5 is kept. Seeds 42-51: 10-31 clearances, 2-10 blocks, 17-47 shots, 3 goals in ten matches (too few: a measure for the next step), corners 0-3, about 1,200 passes (seed 48: 1,876). P1 `a_pressed_defender_can_clear_the_ball`. **Golden changed.** `SIM_VERSION` 58 -> 59. Related: ADR-051, ADR-050.

58: ADR-051, "block": a shot on its way that passes within an outfield defender's reach for a passing ball (his `reactivity`) and below `block_height_cm` is blocked: it comes back at `block_rebound_milli` of its speed, loose, and the possession is the defenders' (out over the goal line it is a corner). The `Block` event is emitted for the first time (`MatchState::block_shot`, weights in `offball.json`). Seeds 42-51: 0-18 blocks a match (seed 47 has too many: 18), 15-50 shots, 2-13 saves, 11 goals in ten matches, about 1,250 passes (seed 48: 1,941); corners still 0-1: blocks and parries rebound into the pitch, and the clearance does not exist yet. P1 `a_defender_can_block_a_shot`. **Golden changed.** `SIM_VERSION` 57 -> 58. Related: ADR-051.

57: Calibration (L3, declared) of ADR-050's progression, from a measure: in seed 42 two thirds of the carriers' decisions were taken within 20 m of their own goal, a loop of sideways passes between the keeper and the two centre backs (slots 0, 1, 4: about 560 passes at the same depth), although eight or nine teammates were in sight: the easy pass always beat the forward one. `shot_choice.json` `progression_gain` 1.0 -> 1.5, `shot_distance_ref_cm` 2500 -> 1800 (back to the 53 value). Measured for the gain: 2.5 gave about 1,100 passes but 42-84 shots a match, 4.0 about 920 passes and 77-120 shots; 1.5 is kept. Seeds 42-51: about 1,240 passes, 27-46 shots, 7-16 saves, 6 goals in ten matches. **Still wrong**: almost every shot comes from 15-20 m (207 of 227 in seeds 42, 46, 49): the carrier runs to the edge of the box, where carrying stops by rule, and shoots; nobody closes him down or blocks, and the keeper stops about 96% of those on target (real: about 75%). The next acts are the block and closing down. **Golden changed.** `SIM_VERSION` 56 -> 57. Related: ADR-050, ADR-051.

56: Calibration (L3, declared) of the kick's error, from a measure: the shots on target crossed the goal near its centre, at the keeper (seeds 42-47: the crossing was within 1 m of the centre in most of them), because the error (0.15 rad) was as large as the gap between the post and the point aimed at, so a shot went either wide or into the middle. `kick.json` `error_ref_rad` 0.15 -> 0.08, `error_pressure_rad` 0.15 -> 0.08: a shot aimed at the post now ends near the post, in or out. Seeds 42-51: 6-27 shots, 2-10 saves, 0-2 goals (9 in ten matches), about 1,430 passes. **Finding for the next step**: the shots are few not because of the shot but because the attacks rarely start: two thirds of the carriers' decisions in seed 42 (1,031 of 1,630) are taken within 20 m of their own goal; the sides circulate the ball between the keeper and the back line. The build-up (who is free further up, the opponents' pressing, when the vertical pass is worth its risk) is the next thing to read. **Golden changed.** `SIM_VERSION` 55 -> 56. Related: ADR-051, ADR-050.

55: ADR-051, the family "without the ball" (defending, receiving) and fixes found by measuring. **Mark**: with the opponents on the ball, the nearest defender who reads the game marks the nearest other opponent, goal-side of him (`offball.json` `mark_*`). **Reach for a ball passing by** reads `reactivity` (`ball_reach_*`): the nearest player within his own reach takes it. **Receive**: a pass on its way is its receiver's to meet, and he goes at once (the pass changes his scene); an opponent cuts it out only if it passes within his reach; the keeper meets a pass meant for him wherever it comes from; nobody receives his own pass; **a player who has just received decides after his own time**, not on the same tick. Found by measuring, all hidden since `SIM_VERSION` 52: the passer took his own pass back on the tick he kicked it (most "passes" never left: the 18% of ticks at someone's feet was that), and a receiver played every ball first time. **Symmetry**: the shooter's foot side is read in his own frame (a right foot facing +x aims at -y), and the side of a miss is the way his body moves when he strikes, or, standing, the side he aimed at; before, the absolute side and the shooter's position made one team shoot wide (29 home goals to 3 in ten seeds). A pass drifts towards the centre of the pitch (before, always left of its line: a passer on his goal line kicked it out every time, and seed 48 played 1,495 corners). A wide shot gives a goal kick (before, the possession went to the defenders before the ball left, and every wide shot gave a corner). Calibration (L3, declared): `kick.json` `error_ref_rad` 0.15, `error_pressure_rad` 0.15, `speed_ref_cm_per_tick` 340; `shot_choice.json` `carry_salience` 0.6, `shot_distance_ref_cm` 2500. Seeds 42-51: 7-21 shots (about 31% on target), 1-7 saves, 0-3 goals a match (5 in ten matches: too few, the keeper stops about 89% of the shots on target, real is about 70%), about 1,300 passes (real about 1,000), 0-1 corners (blocks and clearances do not exist yet). Twin test `the_reactive_twin_reaches_the_ball_farther`; P1 `the_nearest_marks_the_man`; `the_ball_is_mostly_at_someone_s_feet` and `pressure_is_not_a_flood` hold again. **Golden changed.** `SIM_VERSION` 54 -> 55. Related: ADR-051, ADR-049, ADR-044, ADR-046.

54: ADR-051, "carry": carrying the ball is an option of the carrier like the others, not only the fallback when nothing is clear. Its salience is `carry_salience x (room / carry_room_ref_cm, at most 1) x brings` (`shot_choice.json`): the room is the distance to the nearest opponent ahead of him towards the goal, and what it brings is the same term as a pass, to the point the room lets him reach (`ds-orchestrator::read::carry_salience`). Measured on seeds 42-51 for the weight: at 0.8 shots went to 47-110 a match (every carrier ran into the box), at 0.25 carrying never won (unchanged from 53); 0.4 is kept. Seeds 42-51 at 0.4: 9-37 shots (mean about 23), 1-14 saves, 0-3 goals, 4-28 corners, 140-225 duels; **passes still about 2,100** (twice a real match: the pace of the carrier's decisions, MD-16, calibration). P1 `with_room_in_front_carrying_is_an_option`. **Golden changed.** `SIM_VERSION` 53 -> 54. Related: ADR-051, ADR-050, IDEA-DESIGN-010.

53: ADR-051, first pass, the family "without the ball": with a teammate on the ball, every player reads what he sees of it (`MateBall`: his distance to the carrier, whether an opponent marks him, whether he is ahead of the ball and how far from the opponents' last defender, the room behind that line) and may choose two new acts. **Offer** (get free to receive): salient when he is marked and near the ball, more for a good reader; he moves `offer_step_cm` away from his marker. **Run** (attack the space): salient near the last defender's line with room behind it, for a player with pace (`max_speed`) who reads the game: a fast forward attacks the space, a slow one holds; he runs to the line, just short of it (offside is not a law of the engine yet, issue #13). Weights in `offball.json`, the run's salience in `ds-calculator::offball::OffBallBalance::run`. **The shot's salience** (ADR-050 consequence, measured: from 10-30 m the carrier passed back 83 times and shot 4 times in seed 42): the goal cone is read against how the goal looks from the penalty spot (`shot_cone_ref`), and the shot carries the same `brings` term as a forward pass, with full progression. Seeds 42-51: 8-22 shots, 0-5 goals, 1-9 saves, 3-19 corners, 150-220 duels a match; **passes still about 2,100** (about twice a real match: the carrier passes as soon as he receives; carrying is not yet an option with its own salience). Twin test `the_fast_twin_attacks_the_space`; P1 `a_marked_player_near_the_ball_offers`, `with_room_behind_the_line_some_run`. **Golden changed.** `SIM_VERSION` 52 -> 53. Related: ADR-051, ADR-049, ADR-050.

52: ADR-051, first pass, the family "the kick": the shot and the pass are the kicker's. **The shot** leaves at the speed of the kick, from the chosen power and the shooter's explosive force (`speed_ref x (base + (1 - base) x power) x (base + (1 - base) x explosive)`: a placed shot by an average player about 17 m/s, a strong one about 23 m/s, the strongest about 28 m/s); before, it left at 30 m/s if on target and 15 m/s if not (the cause after the effect). Its miss is an angle, from his shooting (`finishing`), the quality of the foot he kicks with (the side he aims at: `left_foot`/`right_foot` are read for the first time), the power and the pressure: the same player misses by more from farther away, and a strong shot is less precise than a placed one. He aims low or high by his contact choice (`aim_low_cm`, `aim_high_cm`), and a miscued strike rises; before, every shot aimed at the crossbar and went into the top corner. **The pass is weighed**: the passer gives it the speed that brings it to the receiver still rolling at 6 m/s, knowing the ground friction, within his strongest kick; his passing and his foot make it drift across the line and overhit. Before, every pass left at the same speed whatever the distance, and a short pass ran past the receiver and out: about 150 corners a match since `SIM_VERSION` 50 (not measured then). Formula in `ds-calculator::kick`, numbers in the new `balance/kick.json`; `reception.json` `speed_base` and `speed_passing_bonus` are no longer read (`Removed-by: the pass kinds of ADR-051`). **A pass is received where it reaches a player** ("receive"): before, nobody could touch a pass until the fixed 1.5 s of the pending had passed, so a weighed pass rolled past its receiver and the ball was "in flight" in 98% of the ticks of seed 42, played by players who never had it; now the player it reaches controls it with his first touch (a heavy touch sends it on, slower, and he needs a moment), or an opponent it reaches intercepts it (the interception's reach is still the control radius, not his reactivity: next). At someone's feet: 2% of the ticks before, 18% after. Seeds 42-51: 2-11 corners, 0-4 goals a match (9 in ten matches), saves in every match, 78-227 duels, about 2,200 passes (about twice a real match) and **few shots (2-13 a match)**: the teams keep the ball and create nothing, because what creates chances (offering for a pass, runs in behind) is the family "without the ball". Twin tests `the_explosive_twin_kicks_harder`, `the_better_shooter_and_the_good_foot_miss_by_less`; P1 `the_ball_is_mostly_at_someone_s_feet`. **Golden changed.** `SIM_VERSION` 51 -> 52. Related: ADR-051, ADR-046, ADR-031, ADR-048.

51: ADR-051 (accepted), first pass, the family "in goal": the save is the keeper's. The keeper reacts after his reaction time (the new characteristic `reactivity`, physical, entering with its first consumer: `reaction = slow - (slow - fast) x reactivity`, 0.5 s to 0.2 s), dives in the time left before the ball reaches him (`explosive_force`: speed and length of the dive), and stops what is within his standing reach (half his height) plus the dive, measured from the centre of his body across and up, **where the ball passes him**, not on the goal line (off his line he narrows the angle and has less time). His hands (`goalkeeping`, the prototype of handling) hold the ball or parry it: harder at the edge of the reach and on a faster ball; a parried ball comes back into play pushed away from the centre of the goal, loose (a new act: the narrator says "parries"). Formula in `ds-calculator::save`, numbers in the new `balance/save.json`. Also: without the ball the keeper is never deeper than half the ball's depth (`keeper.json` `depth_share_milli`): before, with the ball close to the line at a side he advanced 4 m and stood behind the shooter (seed 42: beyond the ball in 63% of the ticks near his goal, now 3%). Seeds 42-51: 249 shots, 48 stopped (19 held, 29 parried), 81 goals: **still about 8 goals a match**. Measured causes, in other families of the catalogue: half the shots are taken from 2-5 m, inside the goal area (nobody marks, covers or blocks: "without the ball"); every shot goes to the top corner with almost no error, even from 3 m ("the kick": precision, power, foot); from that close the ball arrives in 0.2 s and no keeper has time to dive (real). Twin tests `the_reactive_twin_reaches_farther`, `the_explosive_twin_dives_farther`, `good_hands_hold_poor_hands_parry`; P1 `more_time_more_reach`, `the_keeper_stays_between_the_ball_and_his_goal`. **Golden changed.** `SIM_VERSION` 50 -> 51. Related: ADR-051, ADR-045, ADR-035, OI-92.

50: Bug fix: a ball leaves at the speed it was given. `resolve_pass` and `resolve_shot` multiplied the unit direction (thousandths) by the speed with `i16::saturating_mul`, which saturated at 32,767: every pass and every shot left at about 32 cm/tick (3 m/s) whatever its speed, so shots died before the goal line or were picked up (seeds 42-51: 365 shots, 231 "on target", 1 goal). Now the product is in `i32` (`GroundTruth::velocity`). With the ball at its speed a second bug showed: a carrier who reached the goal line with no pass on could only push towards the goal, and stood there for minutes, challenged every three seconds (seed 42, 70': 1,129 duels). Now a carrier without room (blocked, or at the edge of the box) turns away with the ball, towards the open pitch. Pressure is on a carrier only: a ball in flight or loose is pressed by nobody (before, every opponent a fast pass flew past "pressed" it, 2,238 Pressure events in seed 42); `pressure_is_not_a_flood` counts every new carrier, not only the changes of side; seeds 42-51: about 800-870 Pressure events a match, still about twice a real one. Seeds 42-51 after: 6-24 shots a side, 200-290 duels, about 1,450 passes, and **far too many goals (3 to 17 a match)**. The cause is visible: the save reads only the keeper's standing reach (half his height x goalkeeping), with no dive and no reaction time, so a keeper stops only a shot at his body. That is the next decision (the action catalogue, ADR-051). Also measured: a player's running speed follows his `max_speed` (seed 42: 99.5th percentile from 39 cm/tick at 0.30 to 96 at 0.79). P1 `a_ball_leaves_at_its_speed`, `a_carrier_with_nothing_on_does_not_stand_still`. **Golden changed.** `SIM_VERSION` 49 -> 50. Related: OI-97, ADR-039.

49: ADR-050 (accepted): the carrier reads what an option brings. A pass's salience is multiplied by `max(progression_floor, 1 + progression_gain x appetite x (1 - pressure) x progression)`: `progression` in [-1, 1] is how much nearer to the goal the receiver is than the carrier, `appetite` is the carrier's `risk_management` (read by the actor, `Actor::decide`, and passed to `build_choice`), `pressure` grows as the nearest opponent comes within `pressure_radius_cm`; a pass back stays an option. The shot is an option within `shot_range_cm` (3500) and its distance factor is `1 / (1 + (d / shot_distance_ref_cm)^2)`, no longer falling linearly to zero at 25 m. The constants are in `L0-match/balance/shot_choice.json` (`SHOOT_RANGE_CM` leaves the code). Also: a defender beaten in a challenge waits `retry_ticks` before trying again (before, he retried at the next tick: 4,500 duels in a match). Seeds 42-51: 10-96 shots (median about 30), 1,000-2,000 passes, 6-56 duels, no freeze; goals still rare. P1 `a_pass_forward_brings_more_than_a_pass_back`, `under_pressure_progression_counts_less`, `a_shot_from_thirty_metres_is_still_an_option`. **Golden changed.** `SIM_VERSION` 48 -> 49. Related: ADR-050, ADR-040, ADR-047, ADR-049.

48: ADR-049 (accepted, amended: the goalkeeper included): every player, with or without the ball, chooses his own act. A player without the ball, when his latency expires, reads his scene (`OffBallScene`: whether he sees the ball, whether it is loose, his distance to it and to the opposing carrier, how many teammates he sees closer, whether the carrier is in reach, for the keeper whether the ball is in his area) and chooses (`GroundTruth::choose_off_ball`, `ds-orchestrator::offball`, weights in the new `L0-match/balance/offball.json`): **hold** his place, **press**, **challenge** in reach (a good tackler commits, a poor one keeps pressing), **go to** a loose ball. The goalkeeper holds his position, comes out and challenges only inside his area, never presses. A presser who arrives in reach chooses at once (the scene changed). The engine applies the choice: `move_agents` targets the ball for press/challenge/go, the block or the keeper's rule for hold; `challenge_carrier` applies a duel only for a player who chose it, once per choice (the keeper's one-on-one reads his goalkeeping). **Removed:** the engine's "nearest player" rules (`chasers`, the nearest defender challenging). Seeds 42-51: duels 3-93 and passes 900-2,500 a match (before: 4,500-8,000 and 8,500-12,000), no freeze; shots are rare again (the carrier's choice, ADR-040, the next decision). P1 `the_nearest_goes_the_others_hold`, `in_reach_he_challenges`, `the_keeper_stays_in_his_area`, `the_engine_does_not_pick_the_chaser`, `embedded_offball_is_valid`. **Golden changed.** `SIM_VERSION` 47 -> 48. Related: ADR-049, ADR-043, ADR-042, VISION §3.

47: ADR-042, core, at the maintainer's go: the ground duel without a die. The opponent nearest to the ball, within `reach_cm` (120) of it, challenges the carrier at most once every `retry_ticks` (30); the carrier who loses the ball waits as long before he can win it back (`MatchState::challenge_carrier`, `challenges`). The contact is deterministic (`resolve_duel`, weights in the new `L0-match/balance/duel.json` read by `ds-calculator::duel::DuelBalance`): the defender's tackling and explosive force against the carrier's first touch and explosive force; ties to the carrier; if the defender wins, the ball is his (Duel and Tackle events, `Fact::Contact`). **The last die is removed** (ADR-042 §5): `GroundTruth` holds no random stream any more. Seeds 42-46: shots 6-40 a match and goals again; no freeze. Declared distance from ADR-042: no body control, maximal force, posture or point of impact (the latents do not exist), no foul band, the duel is not yet a carrier's choice candidate. **Still wrong:** the density, duels and passes are 5-10 times a real match (MD-16, to calibrate). `stoppage_time.json` `duel` 10 -> 0: a clean duel does not stop play (with thousands of duels a match lasted 328 minutes). **Declared debt against the vision (maintainer, 2026-11-12: every player is the orchestrator, chooser and actor of his own part):** here the engine picks the challenger (the nearest within reach) and the block places the players; the next step makes every player an actor that chooses his own off-ball act. P1 `a_defender_can_win_the_ball`, `tackling_counts`, `embedded_duel_is_valid`. **Golden changed.** `SIM_VERSION` 46 -> 47. Related: ADR-042, ADR-035, ADR-041, OI-97.


46: The match never freezes. A carrier who has not got anywhere since his last decision (under 50 cm) is blocked, and plays his best option even below the clarity threshold, as one with no room left does (`last_decision`, `can_carry`). Before, a carrier blocked by an opponent chose to carry again every second, the separation pushed him back to the same spot, and the whole match stood still: seed 42 froze from 6'57" for 38 minutes. Seeds 42-51: the longest still spell is now under 2 s. The real fix is the duel (ADR-042, the maintainer's decision): the blocking opponent should be able to win the ball. P1 `the_match_never_freezes`. **Golden changed.** `SIM_VERSION` 45 -> 46. Related: OI-97, IDEA-DESIGN-010.

45: Pressure is not a flood. An opponent presses the carrier when he comes within `pressure_radius_cm` (500, new in `L0-match/balance/influence.json`) of the ball, and keeps pressing until he is `pressure_exit_milli` (1300) of it away. Before, the radius was `space_threshold_cm` (20 m) and there was no hysteresis: an opponent on the edge "entered" again every tick, and a match had 2,000-9,000 Pressure events. Seed 42: about 2,100 -> 1,072 (the rest are re-entries at the many possession changes, which the duel will reduce). The influence weights were also parsed from JSON at every tick: now `PressureBalance` is loaded once. P1 `pressure_is_not_a_flood`, `embedded_pressure_is_valid`. **Golden changed.** `SIM_VERSION` 44 -> 45. Related: F.5 (entry 18), OI-97.

44: The players are recognisable, and the keeper steps up (maintainer, 2026-11-12: "random but sensible"; "the keepers do not step up and act as an outlet"). (1) **Profiles by role**, `L0-match/balance/profiles.json`: for goalkeeper, centre back, full back, central and wide midfielder, striker, a `[mean, spread]` per hidden characteristic and per body measure. A characteristic is still derived by key (ADR-029), now as `mean + spread * z`, with `z` mixing the player's own key and a key shared by his family (physical, technical, cognitive), so that what goes together goes together; the strong foot follows the passing, the weak foot is a fraction of it, a quarter of the players are left-footed. In the vault (`ds-db::profiles`, `Vault::register_roles`); `GroundTruth::for_match` registers each player's visible role, used by the driver and by `replay`. Before, every characteristic was uniform in [0, 1) and independent: a centre back could finish like a striker. (2) **The fixtures** (`ds-fixtures`) give each player his 4-4-2 role and a body that fits it, by key from the same profiles (visible data only; the tool does not read the vault). Before, eleven "midfielders" of 180 cm and 78 kg. (3) The keyed derivation `unit_from_key` moves to `ds-core::hash` (a primitive, shared by the vault and the fixtures). (4) **The keeper steps up**: while his side has the ball he follows his defensive line at `support_gap_cm` (2500) behind it, never farther than `support_max_cm` (2200) from his line (`balance/keeper.json`); he is an outlet for the pass back. `ds-demo --roster` prints the squads (letter, name, role, height, weight; no hidden value). P1: `the_roles_are_recognisable`, `roles_and_bodies_make_sense`, `the_keeper_steps_up_when_his_side_has_the_ball`. **Golden changed.** `SIM_VERSION` 43 -> 44. Related: L0.11 §11.1, IDEA-DESIGN-007, ADR-029, ADR-041.

43: Modularity and cleanup (maintainer, 2026-11-12: "one responsibility per crate, as modular as possible, moddable"). Two pure modules: `ds-match::rules` (the restarts, Laws 8 and 15-17: `out_of_play`, `kick_off`, `Restart`, `Side`; data in, a decision out; `Removed-by: OI-48`) and `ds-match::shape` (the team block: `roles`, `place`, `Situation`; data in, a point out). The engine applies them (`apply_restart`, `block_target`). Removed: the formation `slots` and `SpatialState::slot` (unused since 42), a duplicated block that turned the carrier to the goal twice, `heading_from_to` (a copy of `perception::heading_towards`), an unused `shot_distance`. **One behaviour change, declared:** every restart now clears a pass or shot still pending; before, a pass in flight at half-time "landed" in the second half after the kick-off. Golden changed in one seed. Unit tests for both modules (`rules`: on the pitch, throw-in, corner, goal kick, kick-off spot; `shape`: the block follows the ball, stretches with it, kicks off in its own half). `SIM_VERSION` 42 -> 43. Related: ADR-041, OI-48, VISION §3.

42: OI-97, OI-98, the team moves without instructions. (1) **The team block** (VISION §3, the role as a default intention; `L0-match/balance/block.json`, `SpatialState::block_target`): each outfield player keeps his role's place (depth and width, read from the formation) inside a block that follows the ball. The defensive line sits behind the ball within a minimum and a maximum distance from its own goal, the block shifts towards the ball's side; with the ball the side stretches (45 m deep, 60 m wide), without it it compacts (32 x 40 m). It replaces the old rule (the whole-pitch grid translated by 30% of the ball's offset), under which a still ball meant a still team. (2) **Pressing and the loose ball**: when someone has the ball the nearest player of the other side presses him; when nobody has it, the nearest of each side goes for it (before: the nearest of either side, whoever he was). (3) **Kick-off positions** (Law 8, OI-98): each side in its own half, in its compact block; before, 22 players in 14 cells. (4) **Restarts** in minimal form (Laws 15-17, `MatchState::restart_if_out`, `Removed-by: OI-48`): throw-in, goal kick, corner (the Corner event is emitted), the side "that played it last" read as the side in possession, declared. (5) **Nobody dribbles into the goal**: a carrier stops at the edge of the box (`carry_stop_from_goal_cm` 1650) and, with no room left, plays his best option even below the clarity threshold (`can_carry` in the scene and the choice); before, a carrier stood on the goal line holding the ball, and the separation pushed him off the pitch (now clamped). (6) **A pass to a marked teammate**: `PASS_FREEDOM_CM` 800 -> 300; a receiver 3 m from his marker is an option, the reception decides. Seeds 42-51: 9-15 players moving per tick, 450-1600 passes, the ball always on the pitch. Still wrong, to decide with the maintainer: shots are rare (the pass formula has no progression term, ADR-040; the shot's weight falls to zero at 25 m) and nobody can win the ball from a carrier (the duel, ADR-042 proposed). P1 `the_ball_never_stays_out_of_the_pitch`, `at_kick_off_each_side_is_in_its_own_half`, `without_room_the_best_option_is_played`, `embedded_block_is_valid`. **Golden changed.** `SIM_VERSION` 41 -> 42. Related: VISION §3, OI-97, OI-98, OI-48, ADR-040, ADR-041 §5.

41: OI-97, the match starts and flows: five changes that only hold together. (1) **Only the carrier plays the ball**: `decide` returns unless the elected actor is the carrier. Before, the nearest player of the side in possession could kick a ball he was not touching, and the ball left from his position (a teleport): that is what kept the match alive, falsely. (2) **The ball follows the carrier** (`SpatialState::follow_carrier`), and when he chooses to carry it he runs with it towards the goal (`carry_target`, `L0-match/balance/carry.json` `carry_distance_cm` 1000, IDEA-DESIGN-010); before, Advance moved nobody. (3) **Kick-off** (Law 8, `MatchState::kick_off`): a player of the side that kicks off stands on the ball and it is his; home kicks off the first half, away the second. Before, nobody was on the ball and nobody saw it. (4) **The ball is the reference of the gaze**: between two scans the gaze returns to the ball; the scan is a glance (ADR-034 amended in behaviour, declared: before, a marked player looked at his mark for good and never saw the ball again, so nobody went for it). (5) **After a goal the side that conceded kicks off** from the centre spot; before, the ball stayed in the net for the rest of the match. Measured on seeds 42-51: the ball stays on the pitch; six seeds of ten have shots and goals (the first goals of the project's golden range). Still wrong, next: two seeds still stall; passes are too many (a ping-pong); the ball leaving the pitch elsewhere than into the goal has no restart (OI-48); at kick-off the teams still overlap (OI-98). P1 `the_kick_off_has_a_taker`. **Golden changed.** `SIM_VERSION` 40 -> 41. Related: OI-97, ADR-041 §5, ADR-034, IDEA-DESIGN-010, Law 8.

40: OI-97, ADR-048 §4 (first part): the ball's physics are declared data, and the grass slows the ball. New `L0-match/balance/ball.json` and `BallBalance`: gravity (10 cm/tick², derived from g, not arbitrary), air friction (990, the old single value), **ground friction (957, new)**, stop threshold (1); the four constants leave `spatial.rs`. With one friction of 990 everywhere a ground pass rolled about 150 m and left the pitch; with 957 on the ground a pass at about 15 m/s rolls about 35 m. `control_loose_ball` now applies to a ball on the ground, at rest **or rolling** (a footballer controls a moving ball), still never with a pass or shot pending. P1 `a_ground_pass_rolls_tens_of_metres`. **Golden changed.** Known, next: a player can still play a ball he is not touching (the nearest is elected and the ball leaves from his position), and a ball that leaves the pitch has no restart (OI-48). `SIM_VERSION` 39 -> 40. Related: ADR-048 §3-§4, ADR-044 §4, OI-97.

39: OI-97, the latency in real-football seconds. `latency.json` `base_ticks` 1334 -> 7. The old value was derived to reproduce the prototype's mean wait between actions (1/0.0005 = 2000 ticks, the density of the old die), not the time a footballer takes to choose: measured latencies were 142-394 s, and a carrier waited minutes with the ball. With 7, a strong decider in space chooses in about 0.7 s and a weak decider under pressure in about 2-3 s (10 ticks = 1 s), the order of magnitude of a real match. The form of the formula is unchanged (decision_making, crowding, nearest threat). P1 `a_decision_takes_seconds_not_minutes`. **Golden changed.** `SIM_VERSION` 38 -> 39. Related: OI-97, IDEA-DESIGN-011, ADR-044 §6.

38: OI-97, ADR-041 §5 implemented: the carrier is state, computed from the substrate. A loose ball, with no pass or shot pending, belongs to the nearest player once he is within `loose_control_radius_cm` (100, new in `L0-match/balance/reception.json`, arbitrary until L3), and his team has the possession (`MatchState::control_loose_ball`, called in `tick` after the movement). Before, the carrier was set only at the actor's next decision, so a loose ball stayed nobody's for the whole latency and the match stood still around it. No event is emitted for the control (the L0 set has no recovery kind). **Golden changed.** `SIM_VERSION` 37 -> 38. Related: ADR-041 §5, ADR-019 deferred 3, OI-97.

37: OI-97, the direction of a pass and of a shot. `GroundTruth::direction_3d` computed the unit vector as `d * (1000 / len)` in integers: for any distance over 1000 cm the factor was zero, so every pass and every shot longer than 10 m left with a null velocity and stopped where it started on the next tick. Found with the 2D view (M1): the ball "rolled" in place at the centre while a pass was pending. Now `d * 1000 / len`. Test `a_long_pass_has_a_direction`. **Golden changed:** long passes travel for the first time. This, more than any formula, is why seeds 42-51 never produced a shot. `SIM_VERSION` 36 -> 37. Related: OI-97, ADR-035, MD-19.

36: ADR-048 §1 implemented -- the static obstacle (post, crossbar) deflects the ball (OI-52 closed). New `L0-match/balance/obstacles.json` (goal geometry Law 1, post radius and elasticity declared arbitrary until L3) and `ObstacleBalance` in `ds-match/src/balance.rs`. New `ds-calculator/src/obstacle.rs`: `ObstacleReflection::reflect` reverses the velocity's normal component, scaled by the combined elasticity, integer, no die. `check_shot_crossing` reads the pure geometry `MatchState::obstacle_hit` (a ball within `post_radius` of the post, or within `crossbar_thickness` of the bar, is deflected); the pending is **not** cleared: the ball keeps flying, no goal, no save. `SpatialState::deflect_ball` puts the ball back at the impact point on the goal plane with the reflected velocity (the tick's integration may have carried it behind the line; no-op if not in flight). The post reverses `vx`; the crossbar reverses `vx` and `vz` (a `vz`-only reflection left the ball travelling into the goal past the plane, where no crossing is checked: no goal, no out, a ball lost behind the line). `MatchState::goal_plane_deflection` is the pure step (geometry + reflection). `reflect` saturates toward the reversed sign on overflow. The hardcoded `366`/`244` in `check_shot_crossing` move to `ObstacleBalance` (foundations §5). `ds-match` gains a direct dependency on `ds-calculator` (ADR-048 §5: the formula lives in the calculator, the engine applies it). Four tests on `obstacle_hit` (post, crossbar, inside, outside); P1 on the trajectory both ways (`a_shot_on_the_post_changes_trajectory`, `a_shot_on_target_is_not_deflected`, plus the crossbar and `deflect_ball_restarts_from_the_impact_and_comes_back_into_the_pitch`). **Declared debt:** the deflection emits no event; the L0 event set is closed and has no woodwork kind (`Removed-by: the woodwork event in L0.05`, the maintainer's decision). **Golden identical:** seeds 42-51 emit no shot, so the ball never reaches the goal plane and the branch never fires (same as 17, 22, 27, 28, 32, 33, 34, 35). `SIM_VERSION` 35 -> 36. Related: ADR-048 (accepted 2026-11-12), ADR-044 §3-§5, OI-52 (closed), OI-81-step-c (unblocked after this).
35: OI-81-step-move -- the goalkeeper positions himself (ADR-045, ADR-046, OI-81). The anchor is lifted: the keeper is no longer excluded from `move_agents`, and he moves with `resolve_movement` like every other agent -- only his target differs. `SpatialState` gains `goalkeepers: (PlayerId, PlayerId)`, populated in `from_squads` from `home.players[0].id` and `away.players[0].id` (a fact of the formation, not a geometry derived from the position: a keeper is not "who stands on the line", and a lineup change does not need to lie). The target is on the ball-goal segment: `t = clamp(1 - d_ball_goal / POSITIONING_RANGE_CM, 0, 1)`, `y_target = goal_y + (ball_y - goal_y) * t`, `x_target = goal_x +/- (GK_LINE_OFFSET_CM + ADVANCE_CM * t)`. **The `GK_LINE_OFFSET_CM` term is derived from the existing fact** (the keeper does not stand on the goal line): the cantiere's formula had `x_target = goal_x +/- ADVANCE_CM * t`, which at `t = 0` puts the keeper on the line, not at his rest position; the code uses the existing constant. `POSITIONING_RANGE_CM` and `ADVANCE_CM` in `L0-match/balance/keeper.json`, declared arbitrary until L3. `GK_ANCHOR_CM` is removed (unused). The keeper is excluded from the `nearest` computation in `move_agents` (a keeper who runs after the ball is not a keeper). The old test `move_agents_does_not_move_the_goalkeeper` is removed (it asserted the defect); it is replaced by `move_agents_moves_the_goalkeeper_toward_the_ball_segment` (P1: the keeper moves, and he advances, not retreats). `MatchState` gains `keeper_balance: crate::balance::KeeperBalance`, `move_agents` gains the parameter; both test call sites updated. **Golden hash identical:** seeds 42-51 emit no shot and the ball never gets close enough to the home or away goal in those seeds for `t > 0` -- the keeper stays at rest, so the target equals the position (same shape as 17, 22, 27, 28, 32, 33, 34: the bump declares the behaviour change, the golden cannot see it). `SIM_VERSION` 34 -> 35. Related: ADR-045 (the keeper is an actor), ADR-046 (the shot and the keeper meet), OI-81 (this cantiere), `process/analysis/2026-11-11-goalkeeper-cantiere.md` (the formula, with the correction on `GK_LINE_OFFSET_CM`), `L0-match/balance/keeper.json`.
34: OI-81-goalkeeper, first commit -- the keeper is in the scene, F4 (ADR-047 section 3). `PerceivedScene` gains `keeper: Option<PlayerId>` and `keeper_reach_cm: f64`. `scene_from` populates them from the opponent with slot 0, with `reach = height_cm / 2` (a visible fact, Plane B). `build_choice`'s shot candidate now computes `visibility = keeper_visibility(...)`: if the keeper's body is on the segment from the shooter to the post he covers, the shot from that side is hard to notice; farther out, it becomes visible again. The form is a linear ramp on the point-to-segment distance: `d = reach` -> factor 0, `d = 2 * reach` -> factor 1. The keeper does not weigh the shot: he opens or closes a side (ADR-047: recognition, not weighing). `geometric` is unchanged. Golden hash identical: seeds 42-51 emit 0 shots, where the change bites (same as entries 17, 22, 27, 28, 32). `SIM_VERSION` 33 -> 34. Related: ADR-045 (the keeper is an actor), ADR-046 (the shot and the keeper meet), ADR-047 section 3 (the shooter recognises the phrases against the keeper), OI-81, OI-85-form (the `visibility` field, reused).
33: OI-88 -- the shot is a complete phrase, the actor's choice (ADR-046 §1, ADR-047). `ChoiceKind::Shoot` becomes `Shoot { aim_y: f64, power: f64, contact_height: f64 }`: the lateral component of the aim on the goal plane (cm), the force (0..=1), the vertical choice (0.0 low, 1.0 high). `ChoiceKind` loses `Eq` (`PartialEq` stays; the tie-break in `resolve_choice` does not use `Eq` on `kind`). `ShotContext` gains the three fields, and `resolve_shot` reads them: `dy = (aim_y - to.1) + deviation`, `dz` from `contact_height` (sign) and `spread` (magnitude); the old `lateral_sign` from position is no longer a direction, it stays only as the band's declared placeholder (`Removed-by: ADR-031 + OI-65 step 3+`). `difficulty` no longer decides the vertical direction; it stays in `spread`. **The read is the actor's (ADR-047):** `Actor::decide` derives `preferred_foot` from the custodian (`LeftFoot`/`RightFoot`, a fact of the body, Plane B by the time the read sees it), calls `read::build_choice(scene, preferred_foot, balance)`, and returns the choice with the pass freedoms. The engine (`MatchState`) no longer builds the `ChoiceContext`; it passes a `PerceivedScene` and receives a `ChoiceKind`. **Two complete actions per side** (the side of the preferred foot, `aim_edge_fraction` from `L0-match/balance/shot_choice.json`, `power_strong` and `power_placed`); `contact_height` is 0.0 in this cantiere (`Removed-by: ADR-046 §5`, the lob needs the flight's time); `reading_weight`/`risk_weight` stay inert (`Removed-by: OI-81`). `ShotChoiceBalance` is re-exported by `ds-orchestrator`. **Golden hash identical:** seeds 42-51 emit 0 shots, where the change bites; the bump declares the behaviour change, the golden cannot see it (same as 17, 22, 27, 28, 32). `SIM_VERSION` 32 -> 33. Related: ADR-046 §1, ADR-047, ADR-039, OI-88, OI-81 (the keeper in the scene), OI-79 (the lateral placeholder).
32: OI-85-form -- the choice filter reads visibility, not geometric (ADR-039 §2, OI-85). `ChoiceCandidate` gains `visibility` (0..=1): how easy the option is to notice, distinct from `geometric` (how good it is). The read derives it from facts it already computes (pass: `line_quality`; shot: `goal_cone`; `Advance`: 1.0). `resolve_choice` gains a **read filter** before the score: a candidate with `visibility < 1 - game_reading` cannot win (its effective score is 0; the raw score stays in `scores`, visible, D5). `Advance` is the fallback and is never filtered. Two earlier shapes were written and reverted before commit: a relative threshold on `geometric` cannot change the argmax (the argmax is the maximum, so filtering the worst options changes nothing) — demonstrated, not shipped. This one does change it: the hidden option is better but hard to notice, invisible to the weak reader and visible to the strong one. Test P1 `a_weak_reader_misses_the_hidden_option_a_strong_reader_sees` (fails without the filter, passes with: positive proof). `perception_floor`/`perception_range` in `L0-match/balance/choice.json` are **not used** by this shape (they were for the geometric filter; kept, inert, to be removed or reused). **Golden hash identical:** seeds 42-51 emit 0 shots, where the filter bites hardest; the bump declares the behaviour change, the golden cannot see it (same as entries 17, 22, 27, 28). `SIM_VERSION` 31 -> 32. Related: ADR-039 §2, ADR-046, OI-85, OI-88 (blocked by this: the shot's complete actions need the per-candidate filter), OI-86 (normalisation, unaffected: the filter is on visibility, not on the scales).
31: R.1-relation -- the latency reads the scene, not only the actor (ADR-044 §6). `resolve_latency` is moved after `squads()`/`goal` in `decide()`, and the formula gains two scene terms: `crowding` (opponents within `CROWD_RADIUS_CM` 1500, saturated at `CROWD_MAX` 3, normalised `[0,1]`) and `nearest_threat` (`1 - d_min / THREAT_RADIUS_CM` 2000, clamped `[0,1]`), both geometric and deterministic, computed by `MatchState::latency_scene`. `Datum` gains `Crowding` and `NearestThreat`; `LatencyContext` gains `crowding` and `nearest_threat`. Formula in `pcfm-calculator/src/latency.rs`: `latency_ticks = round(base * (1 + (1 - dm) * bias) * (1 + crowd * cw + threat * tw))`, `crowd_weight` 0.5 and `threat_weight` 0.5 in `L0-match/balance/latency.json`, declared arbitrary until L3. **A third term, `plausible_options`, is not implemented:** reading it inside `resolve_latency` would duplicate `build_choice`'s plausibility (D7) or make the latency depend on the outcome of the choose (circular). It is tracked as OI-96. `SIM_VERSION` 30 -> 31, golden hash regenerated (seeds 42-51 change: the scene now modulates the pace). Related: ADR-044 §6, OI-84 (the `Situation` trait is still generic; the two terms go via `LatencySituation`, not via a `SceneTerm`), IDEA-DESIGN-011, R.1-latency (entry 30, now superseded by the shape, not by the numbers).
30: R.1-latency -- the tick is a clock, the pace comes from the actor (IDEA-DESIGN-011, OI-78 point 4). `action_roll` is removed: no die decides whether an action happens in a tick. Each actor has a **latency** before he decides again, a deterministic function of his `decision_making` (ADR-035: no die). Formula in `pcfm-calculator/src/latency.rs`: `latency_ticks = round(base * (1 + (1 - decision_making) * bias))`, `base_ticks` 1334 and `decision_bias` 1.0 in `L0-match/balance/latency.json`, both derived from the prototype `action_probability` 0.0005 (mean wait 2000 ticks; with decision_making 0.5, 2000 = 1334 * 1.5) and declared arbitrary until L3. `resolve_latency` in `pcfm-hidden` (orchestrator, ADR-028 section 5): custodian view, formula, rounded to ticks, clamped `>= 1`. **The tick was split**: `tick()` is the substrate (pending, ball, movement, coverage, counters) and does not read the latency; `decide()` is the decision (elect the actor, resolve his latency, choose, execute the branch). The orchestrator calls `tick` then `decide` for every absolute tick. Declared distance from the ideal: the substrate is still inside `tick()`; making the tick a pure counter is a separate refactor, tracked in IDEA-DESIGN-011's implementation note. `action_probability` is removed from `MatchConfig`; `actions_stream` and the `StreamName` import are removed from `state.rs`. Fatigue and pressure are declared terms of the latency (`Removed-by: OI-70` for fatigue, no producer for pressure today). `SIM_VERSION` 29 -> 30, golden hash regenerated (seeds 42-51 change: the pace is a behaviour change). Related: IDEA-DESIGN-011, ADR-035, OI-78 point 4 (closed by the direction), OI-26 (a tick without a due decision moves the body, not the decision), OI-70 (fatigue producer, cited), OI-48 (the two halves + stoppage are Law 7, a rule, not the tick; dated note 2026-11-09).

29: F.6-sal -- the choice's geometry is salience, not probability (ADR-040, OI-87). The `geometric` term of each candidate stops being "probability of success disguised" and becomes "strength of the signal". `build_choice` (state.rs) derives it with three helpers, no trig (L0.11 sec 4.10), no die. **Shot**: `geometric = distance_factor * goal_cone`, where `distance_factor = (SHOOT_RANGE_CM - goal_dist) / SHOOT_RANGE_CM` and `goal_cone = |cross(a,b)| / (|a|*|b|)` from the two posts (`GOAL_WIDTH_CM` 732, Law 1) and the shooter's position; clipped `[0,1]`. **Pass**: `geometric = freedom_norm * line_quality * distance_plausible`, where `freedom_norm = freedom / (freedom + PASS_FREEDOM_CM)` (in `[0,1)`), `line_quality = 1 - blocked/3` with three samples at 25/50/75% of the segment (a sample is blocked when some opponent is within `DUEL_PROXIMITY_CM` 500 of it), and `distance_plausible` is a piecewise-linear hump (`PASS_MIN_CM` 500, plateau `PASS_NEAR_CM` 1000 to `PASS_FAR_CM` 3000, zero at `PASS_MAX_CM` 4500). **Duel and Advance**: `ChoiceKind::Advance` added; the carrier's `ChoiceContext` no longer carries a `Duel` candidate (a carrier has the ball, the opponent does not; `ball_contested` is false by construction, so the duel is not an option for him). `resolve_choice` gains a `clarity_threshold` (0.25, `L0-match/balance/choice.json`): if the chosen candidate's score does not exceed it, the actor carries the ball (`Advance`); otherwise the chosen kind wins. `build_choice` pushes an `Advance` fallback with `geometric = 0.0` only if no other candidate exists, so `resolve_choice` always has a candidate and the empty-context panic is unreachable. The `Duel` branch in the tick's `match chosen` stays for Plan E (`resolve_duel` migration) but is unreachable from the carrier's scene today; declared with `Removed-by: Plan E`. Arbitrary parameters declared here: `GOAL_WIDTH_CM` 732 (regulatory), `PASS_MIN_CM` 500, `PASS_NEAR_CM` 1000, `PASS_FAR_CM` 3000, `PASS_MAX_CM` 4500, `DUEL_PROXIMITY_CM` 500, `clarity_threshold` 0.25. Calibration L3. Behaviour change: seeds 42-51 change, the golden hash is regenerated. `SIM_VERSION` 28 -> 29. Related: ADR-040, IDEA-DESIGN-010 (carrying the ball is a choice), IDEA-DESIGN-011 (the tick is a clock), IDEA-DESIGN-012 (role labels are a residue; `pick_from_space` still returns `(attacker, defender)`, `Removed-by: IDEA-DESIGN-012`), OI-86 (now implementable), OI-87 (closed by ADR-040), OI-88 (aim+power, cited), OI-89 (tick heading, sidestepped by goal_cone), OI-90 (orphan receiver_freedom_cm, cited).
28: E step 3 -- `resolve_save` loses the die, the hold is geometric (ADR-035, OI-50). The `next_unit(&mut self.streams.save)` draw is removed. `SaveContext` gains `offset_cm` (lateral distance from the keeper to the crossing point, computed by `check_shot_crossing`) and `reach_cm` (standing reach = height/2, already computed); `point` stays for the `Save` event payload, not for the formula. Formula: `saved = offset_cm < goalkeeping * reach_cm` -- the effective reach is `goalkeeping * reach`, so a keeper with skill near 0 reaches almost nothing and one with skill 1 covers the full arm span. The `goalkeeping` latent is consumed (was declared in L0.11 §7.2 as the save consumer, now actually consumed). `Streams::save` removed (no consumer). **Not observable in seeds 42-51** (0 shots in the range, so 0 saves, same as entries 17, 22, 27): the behaviour is exercised by the new `save_forced.rs` (4 tests: centre always saved, far outside never saved, deterministic, reach widens the window). Test count 211 -> 215 in README, STRUCTURE, CONTRIBUTING, specs/_index. `SIM_VERSION` 27 -> 28, golden hash identical. Balance note: with real goalkeeping at L0 most shots in the reach are saved; the calibration (which fraction) is L3. Related: ADR-035 Consequences, OI-50, E step 2 (entry 27), OI-62 (the dive extends reach, deferred).
27: E step 2 -- `resolve_shot` loses the die, the deviation is geometric (ADR-035, OI-50). The two `next_unit` draws (`streams.shot` for `y_rand` and `z_rand`) are removed. The deviation is now: `spread = (1 - finishing) * 400 + difficulty * 2` (magnitude, unchanged from before); lateral side `dy_sign = +1 if from.y < to.y else -1` (a real shooter misses toward the side he is on, geometric from positions); vertical `dz_sign = +1 if difficulty > 50 else -1` (up under pressure, down otherwise, threshold declared); amplitude per axis `spread / 4` (the average magnitude of the old uniform `(rand - 0.5) * spread`, same scale); `to = (ctx.to.x, ctx.to.y + dy_sign*dy_abs, ctx.to.z + dz_sign*dz_abs)`. `on_target` and `shot_type` stay geometric (unchanged). `Streams::shot` removed from `pcfm-hidden` (no consumer). `speed_from_success` renamed `speed_from_on_target` (`success` param -> `on_target`; the function is a function of on-target, not of the die). **Not observable in seeds 42-51** (0 shots in the golden range, same as entries 17 and 22): the behaviour is exercised by `shot_forced.rs` (5 tests now: schema, determinism, miss-toward-shooter-side, high/low pressure vertical, pressure widens lateral deviation; two of the old tests removed because they tested the die). Test count 209 -> 211 in README, STRUCTURE, CONTRIBUTING, specs/_index. `SIM_VERSION` 26 -> 27, golden hash identical (0 shots in the range). Balance note: with real-world `finishing` values at L0 the shot never goes over the bar nor wide; it is a calibration issue (L3), declared here, not a silent limit. Related: ADR-035 Consequences, OI-50, REC (entry 26). **Note 2026-11-08 (OI-79):** the lateral rule stated above ("a real shooter misses toward the side he is on") is **false in a real match**: the direction depends on the foot (ADR-031), the point of impact and the posture, not on the shooter's position. `lateral_sign` (named `dy_sign` at the time of this entry) is a **placeholder**, the foot is not read here yet (`Removed-by: ADR-031 + OI-65 step 3+`). No behaviour change, no new bump: this note corrects the record, it does not change the simulation.
26: REC -- reception of the pass, the first step of E (ADR-035). `resolve_pass` loses the die: `complete` is no longer in `PassOutcome`; the flight velocity is a function of the passer's `passing` (`speed_from_passing`, `L0-match/balance/reception.json`). `resolve_reception` (geometric: `first_touch` + freedom - distance, no die) runs at the pending's expiry: if an interceptor was on the line, the possession flips as before; otherwise the receiver tries to control, and the `Pass` event is emitted at landing (`Fact::Trajectory::Pass`, `success = controlled`), not at launch. `PassContext` gains `receiver_freedom_cm`; `Pending` carries `receiver`, `freedom_cm`, `distance_cm`. `TechnicalLatents` gains `first_touch` (from_key, no shift thanks to OI-53). Behaviour change: seeds 42-51 all change; passes now fail only by interception or by a failed reception, not by a die. `SIM_VERSION` 25 -> 26.

25: OI-71 -- scanning: the gaze turns towards the most salient candidate (ADR-034). `perception::scan` chooses among the three relational candidates the L0 state supports: the ball, the opponent nearest the ball, the opponent nearest the player (the mark). Candidates 4 (receiving space) and 5 (line) are declared, not implemented (`Removed-by: OI-71`). The cascade is the declared one: the mark beats the ball when within `mark_radius_cm`; otherwise the highest salience wins, ties by `PlayerId::slot()`. `move_agents` takes `tick` and a `ScanningBalance`; `write_back` runs the scan every `scan_interval_ticks` (5), in a second pass so the movement loop keeps `&mut`. `AgentState::gaze` is no longer "always the ball": the F.6 step 2 filter (`perceived_ball` in `move_agents`) is now real -- an agent that does not see the ball does not chase it. Parameters in `L0-match/balance/scanning.json`, embedded at compile time (`include_str!`), no RNG, no float. Behaviour change: seeds 42-51 all change, because the gaze now differs from "towards the ball" for most of a match and the chaser is chosen from those who perceive the ball. `SIM_VERSION` 24 -> 25.

24: OI-72 -- gaze separated from heading (ADR-033). AgentState gains gaze: u16, the direction the agent looks at, distinct from heading (the body direction of motion). At kickoff both squads look at the ball (gaze = 90 home, 270 away); move_agents updates gaze every tick via heading_towards(agent.position, ball), a simplification declared Removed-by: OI-71. visible_to and perceived_ball now read gaze, not heading: the cone filters what the agent sees, not where he runs. Behaviour change. Not visible in seeds 43-46 and 49-51 (identical hashes); seeds 47-48 change because gaze differs from heading changes the interceptor visibility in a pass. Seed 42 coincides with SIM_VERSION 19 by RNG artefact, verified not a regression (toggle visible_to back to heading restores 9c632da4, 73/73 lib tests green). SIM_VERSION 23 -> 24.
23: F.7 step 10 — the movement formula reads fatigue. `L0-match/balance/movement.json` gains `fatigue_coefficient` (0.5) and `fatigue_floor` (0.5); `MovementBalance::formula()` multiplies acceleration by `Clamp(Sum(1, -c*fatigue), floor, 1)`. The grammar `Formula` gains `Sum(Vec<Self>)`, the minimum needed to express `1 - x` (a `Datum(FatigueModifier)` was rejected: it would hide the coefficient in Rust, against ADR-028). **Not observable in seeds 42-51:** no producer writes `fatigue` during a match (`PlayerBody.fatigue = Player.physical.fatigue = 0.0` always), so the modifier is identity there. The behaviour is exercised by `crates/pcfm-hidden/tests/movement.rs` (3 tests: stride by height, acceleration by weight, fatigue reduces acceleration). `SIM_VERSION` 22 -> 23: the bump declares the behaviour change (possible, not observed), same as entry 20.
22: F.7 step 8 — goalkeeper reach is geometry, not a constant. `GK_REACH_CM` (200 cm, arbitrary) is removed. The goalkeeper now saves if the crossing point is within `reach_cm = height_cm / 2` (arm span ≈ height, an anthropometric fact: Plane B, no new number) of his current position, and `resolve_save` decides whether he holds it. **Geometry decides "does he arrive", the die decides "does he hold"** — the same separation as MD-20 (the goal is geometric, the save is a precondition plus a die). **Not observable in seeds 42–51** (0 shots in the golden range): the behaviour is exercised by `shot_forced.rs` and the functional test, not by the golden. The dive is **deferred**: `dive_ref_cm` and `dive_time_s` need a cited sports-science source, and a term with no derivation does not enter the model (F.7 principle). The standing reach `height_cm / 2` is the fallback, and `Removed-by: F.gk-dive` is declared. `GK_ANCHOR_CM` stays: the keeper does not run after the ball; lateral repositioning for crosses is a separate phenomenon, out of F.7. `SIM_VERSION` 21 → 22, golden hash **identical** (0 shots in the range): the bump declares the behaviour change, the golden cannot see it. Same as entry 17.
21: F.7 step 7b — `move_agents` applies per-agent movement. Each agent's displacement now comes from `resolve_movement` (the movement formula in `L0-match/balance/movement.json`, `pcfm-calculator`), not from a fixed step. The formula `a = accel_ref * explosive_force * (weight_ref / weight)` is a *force* (cm/tick²); `v_max` and `stride` scale it to displacement (`v_max * stride`, `a * stride`). `PlayerBody { height_cm, weight_kg, fatigue }` is built once in `MatchState::new` from the squads and passed to `move_agents`. `velocity` stores the previous displacement; its L-infinity norm is the accumulated speed (no reader today; the tick record will read displacement, IDEA-INFRA-018). `MAX_STEP_CM` is removed (the formula replaces the clamp). The model is per-axis (L-infinity): a diagonal step has the same per-axis magnitude as a cardinal one, matching the previous rule (MD-09). `GK_ANCHOR_CM` still excludes the goalkeeper from the pass (declared simplification; step 8 replaces it with the same model). Golden hash regenerated; seeds 42-51 change, as expected: every agent's displacement is now a function of the player's body and the formula, not a clamp. **Note on `v_max`:** it is a per-axis clamp (`v_max * stride` per component), not a Euclidean norm; the field's doc says so. `SIM_VERSION` 20 → 21.

20: OI-58 — `next_unit` never returns 1.0. The function took `(next_u32 as f32) / (u32::MAX as f32)`: `u32::MAX as f32` rounds to 2^32, so a `next_u32` near the maximum gave exactly `1.0` in f32, outside the declared `[0, 1)`. A 1000-draw test did not catch it (P1: a test that cannot fail). The fix takes the top 24 bits of the `u32` (f32 has a 24-bit mantissa) and divides by 2^24: every value is exactly representable, the maximum is `1 - 2^-24`, strictly below 1.0. A test forces the edge case. **The golden hash is identical**: in seeds 42-51 the edge case does not occur, and the change does not alter the values that do occur. The bump is declared for the behaviour change (possible, not observed), and for the new `pcfm-core` test. `SIM_VERSION` 19 → 20.

19: OI-53 — latents derived by key, not by sequence (ADR-029). `PlayerLatents::from_rng` is removed; a player's latents are derived from `hash(world_seed, player_id, characteristic)` (`PlayerLatents::from_key`), with **no `match_id`**: a player's strength is the same in every match of his world. The `latents` stream disappears from `Streams`. Adding a characteristic no longer shifts the others: each value is derived from its own purpose string. **Every match changes**, because every latent changes: the golden hash is regenerated once (this bump), and then it is stable. The distribution is still uniform in `[0,1)` (prototype; the factor structure is L3's job, L0.11 §11.1); this bump fixes the derivation, not the distribution. **The noise half of ADR-029 is decided but not implemented**: it waits for the per-agent loop (OI-44), where the iteration order would otherwise matter. `SIM_VERSION` 18 → 19.
**Known gap (OI-14):** the 10-season golden test (an L1 artefact) is not implemented. The L0 golden test (N matches, seeds 42..51) exists and is regenerated on every bump.


`SIM_VERSION` is defined in `pcfm-match` (`crates/pcfm-match/src/save.rs`) and embedded in every save (see `L0.07` §2.1).

---

## When to bump

`SIM_VERSION` is bumped when **any** of the following is true:

1. **RNG behaviour changes** — algorithm, seed derivation, or stream naming (see `L0.01` §2.6).
2. **Event emission changes** — an event type is added, removed, or its emission rules change in a way that affects the event stream (see `L0.05`).
3. **Resolution logic changes** — a `resolve_*` function in `pcfm-hidden` returns a different outcome for the same inputs (see `L0.03` §3.2).
4. **Tick loop changes** — the order of operations within a tick changes, or the tick granularity changes (see `L0.04` §2.2).
5. **Random event catalogue changes** — a `RandomEventDef` is added, removed, or its probability function changes in a way that affects the distribution (see `L0.08` §2.5).

If none of the above is true, `SIM_VERSION` is **not** bumped. A refactor that preserves behaviour does not bump `SIM_VERSION`. A new feature that does not affect the match does not bump `SIM_VERSION`.

---

## Bump procedure

A `SIM_VERSION` bump requires **all five steps**, in order. In phase 1 (ruleset disabled, no CI, single maintainer) the local C2 verification replaces the CI merge gate; see `AGENT-POLICY.md` §1 and §9.

1. **Bump the value** in `pcfm-match` (`crates/pcfm-match/src/save.rs`). The value is a monotonic integer. It never goes down.
2. **Regenerate the golden hash** — run `cargo test -p ds-match --test suite golden_l0`, copy the `left:` value from the failure, replace `EXPECTED` in `tests/cases/golden_l0.rs`. At L0 the test is N matches (not 10 seasons: L1 does not exist). The L1 form is a future artefact.
3. **Write the entry** in the "History" section below, using the template.
4. **Verify save compatibility** — every existing save fixture must still load. If a save cannot be reconstructed under the new `SIM_VERSION`, the entry must say so explicitly and a migration must be provided (see `L0.07` §2.3).
5. **Review by a second pair of eyes.** In phase 1 there is one maintainer: the post-push review (`AGENT-POLICY.md` §9) replaces the second reviewer. In phase 2 (ruleset active, branch + PR), the reviewer confirms that the golden hash change is expected and that the entry is honest.

The migration procedure for **saves** is not written yet: it arrives with the first save-breaking change (foundations §5: no save-breaking change without a migration script).

---

## Entry template

Every entry has the same shape. Copy this template and fill it in.

```markdown
## [SIM_VERSION N] — YYYY-MM-DD

### Why the bump

One paragraph. What changed and why it could not be done without changing
simulation behaviour.

### What changes for the same seed

Concrete. "Match X (seed 12345) now ends 2-1 instead of 1-1 because ...".
If the change is statistical (affects distributions but not any single
match), say so and give the distribution delta.

### Saves affected

- Can existing saves still be loaded? (yes / no / with migration)
- Can existing saves be replayed? (yes / no / with warning)
- If a migration is required, where does it live? (link)

### Golden hash

- Previous: `0x...`
- New: `0x...`
- Verified by: (reviewer name or handle)

### Related changes

- `CHANGELOG.md` entry: (link or "none")
- `CHANGELOG-REGISTRY.md` entry: (link or "none")
- ADRs affected: (list or "none")
```

---

## History

## [SIM_VERSION 8] — 2026-11-01

### Why the bump

Slice B2: the acting agent comes from space, not from a random roll.
`pick_players` (random attacker + random defender, same roll) is replaced
by `pick_from_space`: the attacker is the agent of the possessing team
nearest the ball, the defender the nearest opponent to the attacker.
`SpatialState.carrier` tracks who has the ball. A duel won by the
defender emits `Tackle` with `from`/`to`.

### What changes for the same seed

The attacker and defender are no longer random: they are the agents
nearest the ball. The event stream changes (who shoots, who passes, who
duels), so the score and the chain hash change. Seed 42 produces a
different match.

### Saves affected

No schema change. The `SIM_VERSION` bump is for the simulation behaviour,
not the save format. A v7 save loads under v8, but re-simulation differs.

### Golden hash

- Before: `d8470ee8...`
- After: the new value in `crates/pcfm-match/tests/golden_l0.rs:21`
  (regenerated from the failure, as the procedure says).

### Related changes

- `crates/pcfm-match/src/state.rs`: `SpatialState` field, `pick_from_space`,
  `Tackle` on a won duel, carrier set/cleared.
- `crates/pcfm-match/src/orchestrator.rs`: `MatchState::new(ctx.match_)`.
- `crates/pcfm-match/src/save.rs`: `SIM_VERSION` 7 -> 8.

## [SIM_VERSION 7] — 2026-11-01

### Why the bump

ADR-026 phase 2 calibration. The initial stoppage-time contributions
(goal 30s, foul 30s) were a first proposal, not a calibrated value. The
match recovery was ~2'50" for a half with 4 goals, low against the real
game. New values: goal 60s, foul 20s, duel 10s, base 0.

### What changes for the same seed

The recovery (and therefore the timestamp of the boundary events) changes.
Seed 42: first half 45'+2' -> 45'+4', second half 45'+1' -> 45'+2'.

### Saves affected

No schema change. The `SIM_VERSION` bump is for the simulation behaviour,
not the save format. A v6 save loads under v7, but the timestamps of a
re-simulation differ.

### Golden hash

- Before: not recorded. OI-14 tracks the missing golden test.
- After: not recorded. OI-14 still open.

### Related changes

- `L0-match/balance/stoppage_time.json`: goal 60, foul 20, duel 10, base 0.
- `pcfm-match::balance` tests updated.

## [SIM_VERSION 6] — 2026-11-01

### Why the bump

ADR-026 phase 2: two halves with separate clocks, `Half` field on every
event, four-phase tick loop, fixed-point stoppage time, boundary events
(`KickOff`, `HalfTime`, `SecondHalf`, `FullTime`) emitted.

### What changes for the same seed

Events carry `half: Half`; `timestamp_minute`/`timestamp_second` are
relative to the half. The match runs two halves of 45' plus a fixed-point
stoppage time each. `FullTime` is at `(Second, 45+rec2)`.

### Saves affected

Breaking: `half` added to every event. A v5 save does not load under v6.

### Golden hash

- Before: not recorded. OI-14 tracks the missing golden test.
- After: not recorded. OI-14 still open.

### Related changes

- ADR-026 (Match boundaries and stoppage time), amended 2026-11-01.
- `L0-match/balance/stoppage_time.json` (new, embedded with `include_str!`).
- `pcfm-match::orchestrator`: four-phase loop.
- `pcfm-demo`: board shows `1T`/`2T` and `45'+X'`.

## [SIM_VERSION 5] — 2026-11-01

### Why the bump

ADR-026 (accepted 2026-11-01): match boundaries and stoppage time.
`MatchEvent.actor: Option<ActorId>` replaces `player_id: PlayerId`; new
`EventType::KickOff` and `EventType::SecondHalf`; `HalfTime` and `FullTime`
emitted; two halves with separate clocks and separate stoppage time.

### What changes for the same seed

Events carry `actor` instead of `player_id`. Boundaries (`KickOff`,
`HalfTime`, `SecondHalf`, `FullTime`) have `actor: None`. Match end time is
`45+rec1` for the first half and `45+rec2` for the second, with `rec`
computed from the events of each half (fixed point, no cap; guard at 45').

### Saves affected

Breaking: `player_id` removed, `actor` added. A v4 save does not load under
v5. Migration is not provided: no v4 save exists outside test fixtures, and
the fixture is regenerated. The `docs/MIGRATION-01-04.md` procedure is the
template; a `MIGRATION-04-05.md` is not needed for phase 1.

### Golden hash

- Before: not recorded. OI-14 tracks the missing golden test.
- After: not recorded. OI-14 still open.

### Related changes

- ADR-026 (Match boundaries and stoppage time) — accepted 2026-11-01.
- `pcfm-core`: `ActorId` (`Player | Referee`).
- `pcfm-stats::MatchEvent`: `player_id` -> `actor`.
- `pcfm-stats::EventType`: `KickOff`, `SecondHalf` added.
- `pcfm-match::narrative`: handles `actor` and the new event types.
- `pcfm-demo::viewmodel`: `EventRow.team` gains `Match` for boundary events.

## [SIM_VERSION 4] — 2026-10-31

### Why the bump

`MatchEvent` lost three fields: `career_minutes_cumulative`, `season_label`, `observed_attrs_snapshot_hash`. All three were always `None` or a duplicate of another field: `career_minutes_cumulative` equalled `minutes_played_cumulative`, `season_label` was `None` in production, `observed_attrs_snapshot_hash` was `None`. The event payload enters `chain_hash`, so the bump is mandatory. `MatchSave` also lost `config_versions`, always empty: that alone would not require a bump, but it is included in the same change.

### What changes for the same seed

The `chain_hash` differs from SIM_VERSION 3 for the same seed: the serialised `MatchEvent` has three fewer fields. Score, possession, shots and the events themselves (type, minute, player) are identical.

### Saves affected

- Load: yes, unknown fields are ignored.
- Replay: no — `can_replay()` returns false for SIM_VERSION < 4.
- Migration: none. Old saves are read-only.

### Golden hash

- Not recorded. The 10-season golden test does not exist yet (OI-14).
- Verified by: C2, locally, 2026-10-31.

### Related changes

- `crates/pcfm-stats/src/event.rs`: three fields removed from `MatchEvent`; `Hash` import removed.
- `crates/pcfm-match/src/save.rs`: `config_versions` removed; `SIM_VERSION` 3 -> 4.
- Specs: `05-match-events.md` §2.4/§2.5 (removed fields), `07-save-minimal.md` (`config_versions`).
- IDEAS: IDEA-INFRA-012 (career/season data at L1), IDEA-INFRA-013 (observed-attrs snapshot at Plane C).

---

## [SIM_VERSION 3] — 2026-10-31

### Why the bump

`NarrativeEntry` lost `template_key` and `params`. Both were produced by `narrate()` and serialised in the save, but had no consumer: `render.rs` reads only `minute` and `text`, and `locales/` does not exist. The payload is smaller; the save shape changed, so the bump is mandatory.

### What changes for the same seed

The `chain_hash` is unchanged (narrative is not part of the chain). The save JSON differs: `result.narrative[*]` has four fields instead of six. Score, stats, events and `state_hash` are identical. Old saves written at SIM_VERSION 2 load (serde ignores unknown fields) but cannot be replayed under SIM_VERSION 3.

### Saves affected

- Load: yes, unknown fields are ignored.
- Replay: no — the sim version differs, `can_replay()` returns false.
- Migration: none.

### Golden hash

- Not recorded. The 10-season golden test does not exist yet (OI-14).
- Verified by: C2, locally, 2026-10-31.

### Related changes

- `crates/pcfm-match/src/narrative.rs`: `NarrativeEntry` reduced to `{minute, event_id, text, importance}`. `template_for` → `importance_for`; `render_text` takes `EventType`.
- `crates/pcfm-demo/src/render.rs`: test fixture updated.
- IDEAS: IDEA-INFRA-011 (narrative localisation at L1/L3).

---

## [SIM_VERSION 2] — 2026-10-31

### Why the bump

ADR-024 removed `CauseTrace` and `Factor` from L0 (supersedes ADR-023). The event payload changed: `MatchEvent.cause_trace` is gone, `ShotOutcome.xg_value` is gone, the latent-derived modifiers (`injury_risk_modifier`, `temperament_modifier`, `risk_modifier`) are gone. Because the event payload enters `chain_hash`, the same seed now produces a different chain hash. A bump was mandatory.

### What changes for the same seed

Every match: the `chain_hash` differs from SIM_VERSION 1 for the same seed, because the serialised `MatchEvent` has fewer fields. The score, possession and shots are **not** expected to change (the resolution logic is identical; only the serialised payload is smaller). Verified by the test suite and the test-runner (seed 42): no match result changed, only the hash.

### Saves affected

- Can existing saves still be loaded? **Yes** — `serde_json` ignores unknown fields, and the removed fields simply do not deserialise.
- Can existing saves be replayed? **No.** Replay re-simulates under the new rules; the chain hash will not match the saved one. Old saves are therefore **read-only**. The contamination is declared, not hidden.
- Migration: none. The fix does not retroactively clean JSON written before the bump.

### Golden hash

- Previous: not recorded (no golden test existed at SIM_VERSION 1 either).
- New: not recorded. OI-14 tracks the missing golden test.
- Verified by: C2 (build, clippy, 102/102 tests, fmt, test-runner 6/6), locally, 2026-10-31.

### Related changes

- ADR-024 (Remove `CauseTrace` at L0) — accepted, supersedes ADR-023.
- `L0-match/specs/05-match-events.md` §2.3 amended.
- `L0-match/specs/10-ground-truth.md` §2.2 amended.
- `L0-match/specs/06-match-output.md` §2.3 amended (xG removed).
- Anti-leak test: `crates/pcfm-match/tests/anti_leak.rs` (fails pre-fix, passes post-fix).
- IDEAS: IDEA-INFRA-009 (CauseTrace at L1).

---

<!--
  NOTES FOR FUTURE EDITORS:

  - Do not edit a past entry. If a past entry was wrong, add a correction
    entry and reference the one it corrects. The history is append-only.
  - Do not bump SIM_VERSION "to be safe". A bump is a promise to the user
    that their saves and replays are affected. Bump only when necessary.
  - If you are unsure whether a change requires a bump, ask in an issue
    before merging. The cost of a missed bump is a silent determinism
    regression, which is the worst kind of bug this project can have.
  - This file is linked from:
      - doctrine/00-foundations.md §4 (determinism)
      - L0-match/specs/01-rng.md §2.6
      - L0-match/specs/07-save-minimal.md §2.3
    If you move or rename this file, update those links.
-->
