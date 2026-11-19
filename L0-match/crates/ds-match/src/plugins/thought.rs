//! The thought plugin (ADR-063 §4; ADR-052).
//!
//! The thought plugin (ADR-063 §4; ADR-052): every player decides, off the ball first, then the carrier (decision A).
//! Moved here from `state.rs` (2026-11-15) as methods of the match state
//! they act on.

use ds_match_api::{MatchPlugin, PluginCard};

use crate::state::prelude::*;

/// The thought.
#[derive(Debug, Clone, Copy, Default)]
pub struct Thought;

impl MatchPlugin for Thought {
    fn card(&self) -> PluginCard {
        crate::registry::card("thought")
    }
}

impl MatchState {
    /// The decision layer of the tick: elect the actor, resolve his
    /// latency, choose his action, execute the branch. This method is not
    /// the tick: the tick is the time and the substrate (ball, movement,
    /// coverage), this method is the decision that happens *at* the tick.
    /// The orchestrator calls `tick` then `decide` for every absolute tick;
    /// the tick does not know the latency, the decision does (IDEA-DESIGN-011,
    /// capture 2026-11-09 fourth pause: "togliere al tick il potere di
    /// decidere chi agisce").
    ///
    /// Declared distance from the ideal: the substrate (movement, coverage)
    /// is still inside `tick`. The tick is decision-free, which is the
    /// part that mattered; making the tick a pure counter is a separate
    /// refactor, tracked as a note on IDEA-011.
    #[allow(clippy::too_many_lines)]
    pub fn decide(&mut self, tick: u64, ctx: &mut MatchContext<'_>) {
        self.remember(ctx.hidden);
        self.decide_off_ball(tick, ctx);
        let Some((attacker, defender)) = self.pick_from_space(ctx) else {
            return;
        };

        // IDEA-DESIGN-011 (R.1-latency): the actor decides when he is due,
        // not when a die says so. The latency is the actor's own pace,
        // resolved from his `decision_making` (ADR-035: no die). If the
        // actor is not due, the tick advances the body and the scene (above)
        // and returns; nothing needs deciding yet.
        // Only the player who has the ball can play it: the nearest player
        // of the side in possession does not kick a ball he is not
        // touching (OI-97; before, the ball left from his position, a
        // teleport). Without a carrier nobody decides: the players move,
        // and the first to reach the ball controls it (ADR-041 §5).
        if self.spatial.carrier() != Some(attacker) {
            return;
        }
        let due_at = self.latencies.get(&attacker).copied().unwrap_or(0);
        if tick < due_at {
            return;
        }
        // He plays the ball when it is at his foot: between two touches it
        // rolls ahead of him, and he meets it first (plan L0-CLOSE 11).
        if !self.spatial.ball_at_feet() {
            return;
        }

        let (teammates, opponents): (Vec<PlayerId>, Vec<PlayerId>) = {
            let (own, other) = self.squads(ctx);
            (
                own.players.iter().map(|p| p.id).collect(),
                other.players.iter().map(|p| p.id).collect(),
            )
        };
        let goal = match self.possession {
            Possession::Home => (i32::from(ctx.match_.pitch.length_m) * 100, 3400),
            Possession::Away => (0, 3400),
        };

        // Scene terms for the latency (ADR-044 §6: the duration is in
        // the relation). Both are geometric and in [0, 1]; the formula
        // weights them in `L0-match/balance/latency.json`.
        let (crowding, nearest_threat) = self.latency_scene(attacker, &opponents);
        let latency = ctx
            .hidden
            .resolve_latency(&ds_orchestrator::LatencyContext {
                player: attacker,
                crowding,
                nearest_threat,
                fatigue: self
                    .bodies
                    .get(&attacker)
                    .map_or(0.0, crate::spatial::PlayerBody::mind_fatigue),
            });
        self.latencies
            .insert(attacker, tick.saturating_add(latency.ticks));

        // A player looks at what he is about to play: he turns to the goal.
        if let Some(h) = self
            .spatial
            .agent(attacker)
            .and_then(|a| crate::perception::heading_towards(a.position, goal))
        {
            if let Some(am) = self.spatial.agent_mut(attacker) {
                am.heading = h;
            }
        }
        // Their keeper is the first of their squad (the formation's fact,
        // as in `SpatialState::goalkeepers`). Before `SIM_VERSION` 74 this
        // was `slot() == 0`, true only of the home keeper: a home shooter
        // never saw the away keeper as a keeper.
        let gk = opponents.first().copied();
        // Room to carry: farther from the goal than the edge of the box,
        // and not blocked. A carrier who has not got anywhere since his last
        // decision is blocked (an opponent in his way) and plays the ball
        // instead of pushing again (OI-97: one carrier stood 38 minutes
        // against a centre back, and the whole match with him).
        let here = self.spatial.agent(attacker).map(|a| a.position);
        let blocked = matches!(
            (self.last_decision, here),
            (Some((who, then)), Some(now)) if who == attacker && distance_squared(then, now) < 50 * 50
        );
        self.last_decision = here.map(|p| (attacker, p));
        let can_carry = !blocked
            && here.is_some_and(|p| {
                let d = distance_squared(p, goal);
                let stop = i64::from(self.carry_balance.carry_stop_from_goal_cm) + 100;
                d > stop * stop
            });
        let keeper_reach_cm = gk.and_then(|g| self.bodies.get(&g)).map_or(0.0, |b| {
            b.height_cm * ctx.hidden.save_balance().standing_reach_share
        });
        let own_goal = (i32::from(ctx.match_.pitch.length_m) * 100 - goal.0, goal.1);
        let mut scene = scene_from(
            &self.spatial,
            attacker,
            (goal, own_goal),
            teammates,
            opponents.clone(),
            (gk, keeper_reach_cm),
            can_carry,
        );
        // He places every man where he has seen him, not where he is: a man
        // he never saw is not in his scene (OI-106, decision A).
        scene.positions = self.as_seen_by(attacker, std::mem::take(&mut scene.positions));
        // He sees how tall each man is (OI-102, SIM 149).
        scene.reception.heights = scene
            .positions
            .keys()
            .filter_map(|id| self.bodies.get(id).map(|b| (*id, b.height_cm)))
            .collect();
        scene.penalty_kick = self.restart_pass == Some(crate::rules::RestartKind::Penalty);
        scene.horizon_cm = ctx.hidden.horizon_cm(attacker);
        scene.strike_error_rad = ctx.hidden.strike_error_he_expects(
            attacker,
            self.spatial.agent(attacker).map_or(0.0, |a| {
                dmath::hypot(f64::from(a.velocity.0), f64::from(a.velocity.1))
            }),
            self.bodies.get(&attacker).map_or(0.0, |b| b.fatigue),
        );
        // What he sees coming (plan L0-CLOSE 11, holding the ball up; the
        // maintainer: "pensa a cosa pensa ed immagina un giocatore"): the
        // players he sees, where they will be by his next decision, going
        // as they are going. A teammate running up he sees arriving; an
        // opponent getting back, too.
        let soon = latency.ticks;
        let (length, width) = pitch_cm(ctx);
        scene.soon = crate::perception::visible_to(&self.spatial, attacker)
            .into_iter()
            .filter(|id| *id != attacker)
            .filter_map(|id| {
                self.spatial.agent(id).map(|a| {
                    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
                    let ahead = |p: i32, v: i16, max: i32| {
                        (i64::from(p) + i64::from(v) * soon as i64).clamp(0, i64::from(max)) as i32
                    };
                    (
                        id,
                        (
                            ahead(a.position.0, a.velocity.0, length),
                            ahead(a.position.1, a.velocity.1, width),
                        ),
                    )
                })
            })
            .collect();
        scene.shield_keep = ctx.hidden.shield_confidence(attacker);
        scene.foresight = ctx.hidden.foresight(attacker);
        // How the players he sees are moving, and the teammates he may serve
        // (a player looks at the teammate he plays to: the cone does not
        // filter the pass target, SIM_VERSION 15, the maintainer 2026-11-04):
        // he plays the pass into a run. Before, a teammate behind him was
        // served where he stood, not where he was running (OI-98: a back
        // pass in his own box that the running teammate could not get back
        // to, and the ball rolled into the goal).
        scene.motion = crate::perception::visible_to(&self.spatial, attacker)
            .into_iter()
            .chain(scene.teammates.iter().copied())
            .filter_map(|id| self.spatial.agent(id).map(|a| (id, a.velocity)))
            .collect();
        // Two men nearer than his eye can tell apart look level to him
        // (ADR-062 (b)).
        scene.level_share = f64::from(self.scan_balance.level_threshold_mrad) / 1000.0;
        // The teammates he sees attacking the depth, and where they run to
        // (plan L0-CLOSE 10a: the through ball).
        let seen = crate::perception::visible_to(&self.spatial, attacker);
        scene.runs = scene
            .teammates
            .iter()
            .filter(|id| seen.contains(id))
            .filter(|id| self.spatial.act(**id) == Some(ds_orchestrator::OffBallAct::Run))
            .filter_map(|id| self.spatial.act_target(*id).map(|t| (*id, t)))
            .collect();
        let (chosen_kind, _) =
            ds_orchestrator::Actor::new(attacker).decide(&scene, ctx.hidden, tick);
        let chosen = match chosen_kind {
            ds_orchestrator::ChoiceKind::Pass(receiver)
            | ds_orchestrator::ChoiceKind::Loft(receiver) => ChosenAction::Pass {
                receiver,
                lofted: matches!(chosen_kind, ds_orchestrator::ChoiceKind::Loft(_)),
                at: None,
            },
            ds_orchestrator::ChoiceKind::Through { to, at } => ChosenAction::Pass {
                receiver: to,
                lofted: false,
                at: Some(at),
            },
            ds_orchestrator::ChoiceKind::Shoot {
                aim_y,
                power,
                contact_height,
            } => ChosenAction::Shoot {
                aim_y,
                power,
                contact_height,
            },
            ds_orchestrator::ChoiceKind::Duel => ChosenAction::Duel,
            ds_orchestrator::ChoiceKind::Advance => ChosenAction::Advance,
            ds_orchestrator::ChoiceKind::Carry { to } => ChosenAction::Carry(to),
            ds_orchestrator::ChoiceKind::Clear { to } => ChosenAction::Clear(to),
            ds_orchestrator::ChoiceKind::Hold => ChosenAction::Hold,
            ds_orchestrator::ChoiceKind::Shift => ChosenAction::Shift,
        };

        // Only the restart's first pass is free of offside: any other play
        // by the taker ends it.
        // A dead ball is kicked unchallenged: nobody presses it (Laws 13-14).
        let dead_ball = self.restart_pass.is_some();
        // The pressure on his kick, `[0, 1]`: the nearest outfield opponent,
        // the one the shooter and the passer reckon with
        // (`shot_choice.json` `kick_pressure_radius_cm`). Their keeper does
        // not press the kicker; a dead ball is kicked unchallenged.
        let kick_pressure = {
            let at = self.spatial.ball();
            let r = ctx
                .hidden
                .shot_choice_balance()
                .kick_pressure_radius_cm
                .max(1);
            opponents
                .iter()
                .skip(1)
                .filter(|_| !dead_ball)
                .filter_map(|o| {
                    self.spatial
                        .agent(*o)
                        .map(|a| distance_squared(a.position, at).isqrt())
                })
                .min()
                .map_or(0, |d| (r - d).max(0) * 100 / r)
        };
        // The ball leaves the keeper's hands as he plays it.
        let from_hands = self.in_hands.take() == Some(attacker);
        // A penalty is a kick at the goal and nothing else (Law 14): a taker
        // who weighed anything else strikes it hard at the middle.
        let chosen = if self.restart_pass == Some(crate::rules::RestartKind::Penalty)
            && !matches!(chosen, ChosenAction::Shoot { .. })
        {
            ChosenAction::Shoot {
                aim_y: f64::from(goal.1),
                power: ds_orchestrator::ShotChoiceBalance::load().power_strong,
                contact_height: 0.0,
            }
        } else {
            chosen
        };
        if !matches!(chosen, ChosenAction::Pass { .. }) {
            self.restart_pass = None;
        }
        // The action belongs to the nearest agent; the ball follows.
        self.spatial.set_carrier(Some(attacker));
        if chosen == ChosenAction::Advance {
            // He carries the ball towards the goal he attacks
            // (IDEA-DESIGN-010); the ball follows him (`follow_carrier`).
            if let Some(a) = self.spatial.agent(attacker) {
                let (x, y) = a.position;
                let d = self.carry_balance.carry_distance_cm;
                // With room he carries the ball towards the goal; without
                // (blocked, or at the edge of the box with nothing on) he
                // turns away with it, towards the open pitch. Before
                // `SIM_VERSION` 50 he could only push towards the goal: a
                // carrier on the goal line with no pass stood there for
                // minutes, challenged every three seconds (seed 42, 70').
                let centre = (
                    i32::from(ctx.match_.pitch.length_m) * 50,
                    i32::from(ctx.match_.pitch.width_m) * 50,
                );
                let towards = if can_carry { goal } else { centre };
                let dx = towards.0 - x;
                let dy = towards.1 - y;
                let len = i64::from(dx).pow(2) + i64::from(dy).pow(2);
                let len = i32::try_from(len.isqrt()).unwrap_or(1).max(1);
                // Nobody dribbles into the goal: he carries the ball at most
                // to the edge of the box (`carry_stop_from_goal_cm`), where
                // he has to choose.
                let room = if can_carry {
                    len - self.carry_balance.carry_stop_from_goal_cm
                } else {
                    len
                };
                let step = d.min(room).max(0);
                self.spatial
                    .set_carry_target(Some((x + dx * step / len, y + dy * step / len)));
            }
        }
        if chosen == ChosenAction::Shift {
            // A touch inside, towards the middle, onto his better foot.
            let strong = {
                let left = ctx
                    .hidden
                    .characteristic(attacker, "left_foot")
                    .unwrap_or(0.0);
                let right = ctx
                    .hidden
                    .characteristic(attacker, "right_foot")
                    .unwrap_or(0.0);
                if left > right {
                    ds_orchestrator::Side::Left
                } else {
                    ds_orchestrator::Side::Right
                }
            };
            self.spatial.set_ball_foot(Some(strong));
            if let Some(a) = self.spatial.agent(attacker) {
                let (x, y) = a.position;
                let mid = i32::from(ctx.match_.pitch.width_m) * 50;
                let step = (2 * crate::spatial::BODY_RADIUS_CM).min((mid - y).abs());
                self.spatial
                    .set_carry_target(Some((x, y + step * (mid - y).signum())));
            }
        }
        if chosen == ChosenAction::Hold {
            // He stays on it, his body between it and the man on him.
            let at = self.spatial.agent(attacker).map(|a| a.position);
            self.spatial.set_carry_target(at);
        }
        if let ChosenAction::Carry(to) = chosen {
            // Into the space he chose, as far as his carry goes.
            if let Some(a) = self.spatial.agent(attacker) {
                let (x, y) = a.position;
                let (dx, dy) = (to.0 - x, to.1 - y);
                let len = (i64::from(dx).pow(2) + i64::from(dy).pow(2)).isqrt();
                let len = i32::try_from(len).unwrap_or(1).max(1);
                let step = self.carry_balance.carry_distance_cm.min(len);
                self.spatial
                    .set_carry_target(Some((x + dx * step / len, y + dy * step / len)));
            }
        }
        if matches!(chosen, ChosenAction::Carry(_) | ChosenAction::Advance) {
            // Running with it, the ball goes back to the foot of the side
            // where it is (ADR-031).
            self.spatial.set_ball_foot(None);
        }

        if let ChosenAction::Pass {
            receiver,
            lofted,
            at,
        } = chosen
        {
            // From the ball, where it is at his foot (plan L0-CLOSE 11).
            let (bx, by, _) = self.spatial.ball_3d();
            let from = (bx, by, 0);
            // A ground pass into the receiver's run, if the passer sees him
            // running: the same point he imagined (`read::lead_point`).
            let seen = crate::perception::visible_to(&self.spatial, attacker);
            let k = ctx.hidden.kick_balance();
            let keep = f64::from(self.spatial.ground_friction_milli()) / 1000.0;
            let to = at.map_or_else(
                || {
                    self.spatial
                        .agent(receiver)
                        .map_or((from.0 + 2000, from.1, from.2), |a| {
                            let run = if lofted || !seen.contains(&receiver) {
                                (0, 0)
                            } else {
                                a.velocity
                            };
                            let p = ds_orchestrator::read::lead_point(
                                (from.0, from.1),
                                a.position,
                                run,
                                (keep, k.pass_arrive_cm_per_tick, k.speed(1.0, 0.5)),
                            );
                            (p.0, p.1, 0)
                        })
                },
                |p| (p.0, p.1, 0),
            );
            let params = self.reception_params(ctx);
            // From his hands the keeper throws it, if it is within his arm
            // (plan L0-CLOSE 10d); farther he kicks it.
            let throw = from_hands
                && !lofted
                && distance_squared((from.0, from.1), (to.0, to.1))
                    <= i64::from(ctx.hidden.kick_balance().throw_max_cm).pow(2);
            let ctx_pass = ds_orchestrator::PassContext {
                passer: attacker,
                foot: self.spatial.ball_foot(),
                from,
                to,
                throw,
                ground_friction_milli: self.spatial.ground_friction_milli(),
                air_friction_milli: self.spatial.air_friction_milli(),
                gait: self
                    .spatial
                    .agent(attacker)
                    .map_or(0.5, |a| f64::from(a.gait_milli) / 1000.0),
                lofted: lofted.then(|| self.spatial.gravity_cm_per_tick2()),
                #[allow(clippy::cast_precision_loss)]
                pressure: kick_pressure as f64 / 100.0,
            };
            let outcome = ctx.hidden.resolve_pass(&ctx_pass, &params);
            // A cross is the ball from wide into the area, lofted or low
            // (the low cross, the cut-back): not a keeper's throw (ADR-072).
            let cross = if throw {
                false
            } else {
                self.compute_cross(ctx, attacker, ((from.0, from.1), (to.0, to.1)))
            };
            // MD-25/F.4-c: carrier released at launch, ball flies, pending
            // resolves the possession (MD-37).
            self.spatial.set_carrier(None);
            self.spatial
                .set_ball_in_flight(from, outcome.initial_velocity);
            // F.4-d: save the opponents the passer did NOT see at launch.
            // The cone filters the interceptor, not the pass target.
            let seen = crate::perception::visible_to(&self.spatial, attacker);
            let unseen: Vec<PlayerId> = opponents
                .iter()
                .copied()
                .filter(|o| !seen.contains(o))
                .collect();
            // A lofted ball goes over the line: nobody on it cuts it out.
            let interceptor = if lofted {
                None
            } else {
                find_interceptor(&self.spatial, &unseen, (from.0, from.1), (to.0, to.1))
            };
            let expected = if interceptor.is_some() {
                Self::other(self.possession)
            } else {
                self.possession
            };
            self.mark_offside_at_pass(ctx, attacker, from.0);
            self.pending = Some(Pending {
                launched: tick,
                due_tick: tick + FLIGHT_TICKS,
                outcome: expected,
                interceptor,
                passer: Some(attacker),
                receiver: Some(receiver),
                closest_cm: None,
                shot: None,
                through: at.is_some(),
                cross,
                foot: ctx_pass.foot,
            });
            // REC: the Pass event is emitted at landing, in `tick`
            // (resolve_reception), not at launch: `complete` is the
            // reception, not the launch.
        } else if let ChosenAction::Shoot {
            aim_y,
            power,
            contact_height,
        } = chosen
        {
            self.play_shot(
                ctx,
                tick,
                attacker,
                (&opponents, dead_ball),
                (aim_y, power, contact_height),
                None,
            );
        // Reachable from the carrier's scene only when `ChoiceKind::Duel`
        // is emitted; `build_choice` no longer emits it (a carrier has
        // the ball, the opponent does not; `ball_contested` is false by
        // construction). This branch stays for Plan E (the migration of
        // `resolve_duel` and the aerial family, IDEA-DESIGN-008): when a
        // defender is put in the carrier's scene, the branch becomes
        // reachable again. Removed-by: Plan E. Do not delete without
        // the migration: `resolve_duel` would lose its only caller.
        } else if let ChosenAction::Clear(to) = chosen {
            self.play_clearance(ctx, tick, attacker, to);
        } else if chosen == ChosenAction::Duel {
            let ctx_duel = ds_orchestrator::DuelContext {
                attacker,
                defender,
                duel_type: ds_orchestrator::DuelType::Ground,
                tackle: ds_calculator::duel::TackleKind::Standing,
                from_behind: false,
                approach_cm_per_tick: 0,
                stakes: (1.0, 1.0, 1.0),
                carrier_on_ball: true,
            };
            let outcome = ctx.hidden.resolve_duel(&ctx_duel);
            let fact = crate::fact::Fact::Contact {
                attacker,
                defender,
                winner: outcome.winner,
            };
            for c in crate::fact::classify(&fact) {
                self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
            }
            if outcome.winner == defender {
                self.flip_possession();
                self.spatial.set_carrier(Some(defender));
            }
        }
    }

    /// Every player without the ball chooses his own act when his latency
    /// expires (ADR-049): hold his place, press, challenge, go to the ball.
    /// He reads his scene from where he stands; the engine only records the
    /// choice and applies it (`move_agents`, `challenge_carrier`).
    // One pass over the twenty-two, the scene of each built in one place;
    // the scene moves to its own type with the off-ball attack (plan
    // L0-CLOSE step 2).
    #[allow(clippy::too_many_lines)]
    pub(crate) fn decide_off_ball(&mut self, tick: u64, ctx: &mut MatchContext<'_>) {
        let ball = self.spatial.ball();
        let carrier = self.spatial.carrier();
        // A ball nobody has is anyone's to go for (ADR-052 note,
        // 2026-11-14: the first aim of every player is the ball): at rest,
        // rolling, and a pass on its way too, which its receiver goes to
        // meet and anyone may get to first. Its passer does not chase it:
        // nobody receives his own pass. A shot on its way is nobody's.
        // Before, a pass was only its receiver's until a clock (1.5 s) made
        // it anyone's; without the clock the others watched it go (ADR-058).
        let pass_by = self
            .pending
            .as_ref()
            .filter(|p| p.shot.is_none())
            .and_then(|p| p.passer);
        // A ball out of play waiting for its restart is nobody's to go for
        // (SIM 150): its taker is sent to it.
        let ball_loose = carrier.is_none()
            && self.waiting.is_none()
            && self.pending.as_ref().is_none_or(|p| p.shot.is_none());
        // A pass in the air is contested (ADR-042, in the air): the
        // opponent nearest where it comes down goes to meet it, as the
        // receiver does.
        let air_drop = self
            .pending
            .as_ref()
            .filter(|p| p.shot.is_none())
            .and_then(|_| self.spatial.drop_point(180));
        let passing_home = self.possession == Possession::Home;
        let meets_pass = self
            .pending
            .as_ref()
            .filter(|p| p.shot.is_none())
            .and_then(|p| p.receiver);
        // A pass rolling on the grass is the receiver's to meet; an opponent
        // goes to cut it out when he gets to its path before him (ADR-052,
        // ADR-058): before, while a pass travelled the other side only held
        // its places, and the old clock made every pass a loose ball after
        // 1.5 s for them to chase.
        // Who could cut it, where he meets it and how far along the ball's
        // path; whether he gets there before the receiver, and before his
        // teammates, each one judges from what he has seen (OI-106: before,
        // from the whole pitch).
        let mut cutters: Vec<(PlayerId, (i32, i32), i64)> = Vec::new();
        let mut receiver_along: Option<(PlayerId, i64)> = None;
        if let Some(r) = meets_pass {
            if let Some(rp) = self.meets_rolling_ball(ctx, r) {
                let ball_now = self.spatial.ball();
                let along = |pt: (i32, i32)| distance_squared(ball_now, pt);
                receiver_along = Some((r, along(rp)));
                let defenders = if passing_home {
                    &ctx.match_.away
                } else {
                    &ctx.match_.home
                };
                let ids: Vec<PlayerId> = defenders.players.iter().map(|p| p.id).collect();
                for id in ids {
                    if let Some(mp) = self.meets_rolling_ball(ctx, id) {
                        cutters.push((id, mp, along(mp)));
                    }
                }
                cutters.sort_by_key(|c| (c.2, c.0));
            }
        }
        let duel = ctx.hidden.duel_balance();
        // Within a slide's reach he may go in; how is chosen at the contact
        // (ADR-042, amended).
        let slide_reach = i64::from(duel.reach_cm) * i64::from(duel.slide_reach_milli) / 1000;
        let off = ctx.hidden.offball_balance();
        let (length, width) = pitch_cm(ctx);
        for (squad, other, home) in [
            (&ctx.match_.home, &ctx.match_.away, true),
            (&ctx.match_.away, &ctx.match_.home, false),
        ] {
            let carrier_opponent = carrier.filter(|c| other.players.iter().any(|p| p.id == *c));
            // With a teammate on the ball (ADR-051, "without the ball"):
            // the carrier, the opponents' last outfield defender's line and
            // the goal this side attacks.
            let carrier_mate = carrier
                .filter(|c| squad.players.iter().any(|p| p.id == *c))
                .and_then(|c| self.spatial.agent(c))
                .map(|a| a.position);
            let mut held: Vec<(PlayerId, (i32, i32))> = Vec::new();
            let mut view = TeamView::new(&self.spatial, other, home, length, carrier);
            view.mate_carrier = carrier_mate;
            view.ours = squad
                .players
                .iter()
                .filter_map(|p| self.spatial.agent(p.id))
                .map(|a| a.position)
                .collect();
            view.ours_vel = squad
                .players
                .iter()
                .filter_map(|p| self.spatial.agent(p.id))
                .map(|a| (i32::from(a.velocity.0), i32::from(a.velocity.1)))
                .collect();
            view.ground_keep = f64::from(self.spatial.ground_friction_milli()) / 1000.0;
            view.mate_pressing = squad.players.iter().any(|p| {
                matches!(
                    self.spatial.act(p.id),
                    Some(
                        ds_orchestrator::OffBallAct::Press | ds_orchestrator::OffBallAct::Challenge
                    )
                )
            });
            let carrier_pos = carrier_opponent
                .and_then(|c| self.spatial.agent(c))
                .map(|a| a.position);
            let keeper_id = squad.players.first().map(|p| p.id);
            for p in &squad.players {
                let id = p.id;
                let Some(me) = self.spatial.agent(id).map(|a| a.position) else {
                    continue;
                };
                // A presser who arrives within reach chooses at once: the
                // scene has changed, he does not wait for his next turn.
                let arrived = self.spatial.act(id) == Some(ds_orchestrator::OffBallAct::Press)
                    && self.challenges.get(&id).is_none_or(|t| tick >= *t)
                    && carrier_pos.is_some_and(|c| {
                        distance_squared(me, c) <= i64::from(duel.reach_cm).pow(2)
                    });
                let keeper = Some(id) == keeper_id;
                // A ball in the air coming down for the other side: where it
                // will come down, and whether it falls in his own area (the
                // keeper reads the flight, not where the ball is now).
                // A ball in the air nobody plays to is anyone's where it
                // comes down: both sides read it there (a clearance, a
                // goal kick, a header away).
                let against = air_drop
                    .filter(|_| home != passing_home)
                    .or_else(|| ball_loose.then(|| self.spatial.drop_point(180)).flatten());
                let drop_in_box =
                    against.is_some_and(|d| in_keeper_box(d, home, (length, width), &off));
                // Courage (ADR-057, OI-109): he goes up for a high ball where a
                // heavier man he has seen can reach it only if his courage is
                // at least that man's share of their two weights; against a
                // man no heavier than him he always goes.
                let courage = ctx.hidden.courage(id);
                let my_weight = self.bodies.get(&id).map_or(0.0, |b| b.weight_kg);
                let brave = against.is_none_or(|d| {
                    other
                        .players
                        .iter()
                        .filter_map(|q| self.perceived(id, q.id).map(|at| (q.id, at)))
                        .filter(|(_, at)| {
                            distance_squared(*at, d) <= i64::from(duel.reach_cm).pow(2)
                        })
                        .filter_map(|(q, _)| self.bodies.get(&q).map(|b| b.weight_kg))
                        .all(|w| goes_up(courage, my_weight, w))
                });
                // The keeper comes for a ball falling in his area if he
                // believes he gets there before the nearest opponent he has
                // seen (OI-102, SIM 149: his pace, against the pace and
                // reaction he believes theirs; the race of a ball nobody
                // has). Before, he came for every one, and an attacker who
                // got there first headed it into an empty goal.
                let keeper_first = keeper
                    && drop_in_box
                    && against.is_some_and(|d| {
                        let pace = ctx
                            .hidden
                            .top_speed_now(id, self.bodies.get(&id).map_or(0.0, |b| b.fatigue));
                        #[allow(clippy::cast_precision_loss)] // cm on a pitch
                        let rival = other
                            .players
                            .iter()
                            .filter_map(|q| self.perceived(id, q.id))
                            .map(|at| distance_squared(at, d).isqrt() as f64)
                            .fold(f64::INFINITY, f64::min);
                        #[allow(clippy::cast_precision_loss)]
                        let mine = distance_squared(me, d).isqrt() as f64;
                        ds_calculator::thought::ThoughtBalance::load().race(mine, pace, rival) > 0.5
                    });
                let contests = |rank0: bool| keeper_first || (!keeper && rank0 && brave);
                let rank_drop = against.map(|d| {
                    let mine = distance_squared(me, d);
                    squad
                        .players
                        .iter()
                        .filter(|q| q.id != id && Some(q.id) != keeper_id)
                        .filter_map(|q| self.perceived(id, q.id))
                        .all(|at| distance_squared(at, d) >= mine)
                });
                // A pass played to him changes his scene too: he goes to
                // meet it at once, unless he already is; so does a ball in
                // the air he is the one to contest.
                let called = (meets_pass == Some(id) || rank_drop.is_some_and(contests))
                    && self.spatial.act(id) != Some(ds_orchestrator::OffBallAct::GoToBall);
                if Some(id) == carrier
                    || (!arrived && !called && self.offball_due.get(&id).is_some_and(|t| tick < *t))
                {
                    continue;
                }
                // How many outfield teammates he sees closer than him (the
                // passer of a pass on its way is not one of them: it is not
                // his to take).
                let rank = |target: (i32, i32)| {
                    let mine = distance_squared(me, target);
                    u32::try_from(
                        squad
                            .players
                            .iter()
                            .filter(|q| {
                                q.id != id && Some(q.id) != keeper_id && Some(q.id) != pass_by
                            })
                            .filter_map(|q| self.perceived(id, q.id))
                            .filter(|at| distance_squared(*at, target) < mine)
                            .count(),
                    )
                    .unwrap_or(u32::MAX)
                };
                let to_carrier = carrier_pos.map(|c| distance_squared(me, c).isqrt());
                let ball_in_own_box = in_keeper_box(ball, home, (length, width), &off);
                let sees_ball =
                    crate::perception::perceived_ball(&self.spatial, id, ball).is_some();
                // Out of possession, the space he closes in his zone
                // (ADR-071 §5, SIM 154): the shape stays as the zone he
                // starts from, the keeper keeps his own place.
                let anchor = self
                    .spatial
                    .block_target(id, ball, false, false)
                    .unwrap_or(me);
                let closed = (carrier_pos.is_some() && !keeper)
                    .then(|| view.choose_closed_place((me, anchor), (length, width)));
                let partner_intesa = if ball_loose {
                    let nearest_mate = squad
                        .players
                        .iter()
                        .filter(|q| q.id != id && Some(q.id) != keeper_id)
                        .filter_map(|q| self.spatial.agent(q.id).map(|a| (q.id, a.position)))
                        .min_by_key(|(_, p)| distance_squared(*p, ball))
                        .map(|(pid, _)| pid);
                    nearest_mate
                        .and_then(|pid| self.intesa.get(&(id, pid)).copied())
                        .unwrap_or(0.0)
                } else {
                    0.0
                };
                let scene = ds_orchestrator::OffBallScene {
                    actor: id,
                    // His own task first; else what the team's orders ask
                    // (ADR-070).
                    asked: self.spatial.asked(id, carrier_mate.is_some()).or_else(|| {
                        self.spatial
                            .team_asked(id, carrier_mate.is_some(), carrier_pos.map(rank))
                    }),
                    place: if keeper {
                        ds_orchestrator::Place::Keeper {
                            // A pass meant for him is his to meet, wherever
                            // it comes from.
                            ball_in_area: ball_in_own_box || meets_pass == Some(id) || drop_in_box,
                        }
                    } else {
                        ds_orchestrator::Place::Outfield
                    },
                    loose_ball: against.map_or_else(
                        || {
                            // The keeper comes for their pass rolling into
                            // his area, as for a loose ball (plan L0-CLOSE
                            // 11): before, it was only its receiver's, and it
                            // rolled past them into his goal.
                            let sweep = keeper
                                && ball_in_own_box
                                && sees_ball
                                && meets_pass.is_some()
                                && home != passing_home;
                            let cut = cut_target(&cutters, receiver_along, id, |o| {
                                self.perceived(id, o).is_some()
                            })
                            .map(|(at, ahead)| {
                                ds_orchestrator::Target {
                                    distance_cm: distance_squared(me, at).isqrt(),
                                    rank: ahead,
                                }
                            });
                            cut.or_else(|| {
                                ((ball_loose && sees_ball && Some(id) != pass_by)
                                    || meets_pass == Some(id)
                                    || sweep)
                                    .then(|| ds_orchestrator::Target {
                                        distance_cm: distance_squared(me, ball).isqrt(),
                                        // A pass played to him is his to
                                        // meet first, however near the ball
                                        // his teammates are (OI-98: a keeper
                                        // ranked behind them held his place
                                        // and the back pass ran past him).
                                        rank: rank_to_meet(meets_pass == Some(id), rank(ball)),
                                    })
                            })
                        },
                        |d| {
                            // OI-102: on an aerial ball the second man also
                            // contests (he goes for it if he can reach it),
                            // so the duel is not left to one man a side.
                            let rank_ok = if air_drop.is_some() && home != passing_home {
                                rank(d) <= 1
                            } else {
                                rank(d) == 0
                            };
                            contests(rank_ok).then(|| ds_orchestrator::Target {
                                distance_cm: distance_squared(me, d).isqrt(),
                                rank: 0,
                            })
                        },
                    ),
                    loose_rival_cm: other
                        .players
                        .iter()
                        .filter_map(|q| self.perceived(id, q.id))
                        .map(|at| distance_squared(at, ball).isqrt())
                        .min(),
                    carrier: carrier_pos
                        .zip(to_carrier)
                        .map(|(c, d)| ds_orchestrator::Target {
                            distance_cm: d,
                            rank: rank(c),
                        }),
                    carrier_in_reach: to_carrier.is_some_and(|d| d <= slide_reach),
                    with_mate: carrier_mate.map(|c| {
                        let zone = self
                            .spatial
                            .block_target(id, ball, true, false)
                            .unwrap_or(me);
                        // The place he holds, chosen from his zone (ADR-071).
                        let place = if keeper {
                            zone
                        } else {
                            view.choose_place(
                                (me, zone),
                                (c, ball),
                                self.spatial.back_line(id, ball),
                                off.onside_margin_cm,
                                (length, width),
                                // Strato 3: how well this man knows his
                                // teammates; drives the complementary
                                // movement (predicted positions).
                                {
                                    let mut sum = 0.0;
                                    let mut n = 0u32;
                                    for q in &squad.players {
                                        if q.id != id {
                                            if let Some(v) = self.intesa.get(&(id, q.id)) {
                                                sum += v;
                                                n += 1;
                                            }
                                        }
                                    }
                                    if n > 0 { sum / f64::from(n) } else { 0.5 }
                                },
                                ctx.hidden.characteristic(id, "game_reading").unwrap_or(0.5),
                            )
                        };
                        held.push((id, place));
                        view.mate_ball((me, place), c, &off, (length, width))
                    }),
                    to_mark: carrier_pos.and_then(|_| view.man_to_mark(me)).map(|m| {
                        ds_orchestrator::Target {
                            distance_cm: distance_squared(me, m).isqrt(),
                            rank: rank(m),
                        }
                    }),
                    threats: carrier_pos.map(|c| {
                        view.threats(c, me, closed.unwrap_or(anchor), home, (length, width))
                    }),
                    partner_intesa,
                };
                if let Some(place) = closed {
                    held.push((id, place));
                }
                for (who, place) in std::mem::take(&mut held) {
                    self.spatial.set_hold_at(who, Some(place));
                }
                let act = ctx.hidden.choose_off_ball(&scene);
                self.spatial.set_act(id, act);
                if let Some(t) = view.act_target(act, me, &off, (length, width)) {
                    self.spatial.set_act_target(id, t);
                }
                let wait = self.decision_time(ctx, id, other);
                self.offball_due.insert(id, tick.saturating_add(wait));
            }
        }
    }

    /// The attacker and defender of the next action, from space (slice B2).
    ///
    /// The attacker is the agent of the possessing team nearest to the
    /// ball; the defender is the nearest opponent to the attacker. Ties
    /// are broken by `PlayerId` (deterministic).
    ///
    /// Special case of influence (`03-entities-minimal` §2.9): the general
    /// case reads the latents inside `pcfm-hidden`; here only the space term
    /// is used. The per-agent slice replaces this with the full formula.
    pub(crate) fn pick_from_space(&self, ctx: &MatchContext<'_>) -> Option<(PlayerId, PlayerId)> {
        let (attacker_squad, defender_squad) = self.squads(ctx);
        let ball = self.spatial.ball();
        let attacker = nearest_in_squad(&self.spatial, attacker_squad, ball)?;
        let attacker_pos = self.spatial.agent(attacker).map_or(ball, |a| a.position);
        let defender = nearest_in_squad(&self.spatial, defender_squad, attacker_pos)?;
        Some((attacker, defender))
    }

    /// The two scene terms for the latency formula (ADR-044 §6).
    ///
    /// `crowding` is the number of opponents within `CROWD_RADIUS_CM` of
    /// the actor, saturated at `CROWD_MAX` and normalised to `[0, 1]`.
    /// `nearest_threat` is the distance to the nearest opponent, normalised
    /// so that `0.0` means at the actor's feet and `1.0` means at
    /// `THREAT_RADIUS_CM` or beyond. Both are geometric and deterministic;
    /// no die, no latent.
    /// How long `id` takes to choose again, in ticks, read in his scene
    /// against `opponents` (ADR-044 §6).
    pub(crate) fn decision_time(
        &self,
        ctx: &mut MatchContext<'_>,
        id: PlayerId,
        opponents: &ds_model::Squad,
    ) -> u64 {
        let ids: Vec<PlayerId> = opponents.players.iter().map(|q| q.id).collect();
        let (crowding, nearest_threat) = self.latency_scene(id, &ids);
        ctx.hidden
            .resolve_latency(&ds_orchestrator::LatencyContext {
                player: id,
                crowding,
                nearest_threat,
                fatigue: self
                    .bodies
                    .get(&id)
                    .map_or(0.0, crate::spatial::PlayerBody::mind_fatigue),
            })
            .ticks
    }

    /// The shot (ADR-051, the family "the kick"): from his feet, at the
    /// goal he attacks, with the pressure on him and his run. `run`, when
    /// given, is the speed of the ball he strikes first time (ADR-064),
    /// which spoils the contact as his own run does.
    #[allow(clippy::too_many_lines)]
    pub(crate) fn play_shot(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        attacker: PlayerId,
        (opponents, dead_ball): (&[PlayerId], bool),
        (aim_y, power, contact_height): (f64, f64, f64),
        run: Option<f64>,
    ) {
        let from = match self.spatial.agent(attacker) {
            Some(a) => (a.position.0, a.position.1, 0),
            None => self.spatial.ball_3d(),
        };
        let to = match self.possession {
            Possession::Home => (i32::from(ctx.match_.pitch.length_m) * 100, 3400, 244),
            Possession::Away => (0, 3400, 244),
        };
        // Difficulty: the pressure on the kick, the same the shooter
        // reckons with (`shot_choice.json` `kick_pressure_radius_cm`):
        // 0 cm -> 100, the radius -> 0. Their
        // keeper on his line does not press the kicker (before
        // `SIM_VERSION` 74 he did, and a penalty was a kick under
        // pressure).
        let difficulty = opponents
            .iter()
            .skip(1)
            .filter(|_| !dead_ball)
            .filter_map(|o| {
                self.spatial
                    .agent(*o)
                    .map(|a| distance_squared(a.position, (from.0, from.1)).isqrt())
            })
            .min()
            .map_or(0, |d| {
                let r = ctx
                    .hidden
                    .shot_choice_balance()
                    .kick_pressure_radius_cm
                    .max(1);
                i32::try_from(((r - d).max(0) * 100) / r).unwrap_or(0)
            });
        let ctx_shot = ds_orchestrator::ShotContext {
            shooter: attacker,
            foot: self.spatial.ball_foot(),
            from,
            to,
            difficulty,
            aim_y,
            power,
            contact_height,
            momentum_y: self
                .spatial
                .agent(attacker)
                .map_or(0, |a| i32::from(a.velocity.1)),
            gravity_cm_per_tick2: self.spatial.gravity_cm_per_tick2(),
            gait: self
                .spatial
                .agent(attacker)
                .map_or(0.5, |a| f64::from(a.gait_milli) / 1000.0),
            run_cm_per_tick: run.unwrap_or_else(|| {
                self.spatial.agent(attacker).map_or(0.0, |a| {
                    dmath::hypot(f64::from(a.velocity.0), f64::from(a.velocity.1))
                })
            }),
            across: self.spatial.agent(attacker).map_or(0.0, |a| {
                let (vx, vy) = (f64::from(a.velocity.0), f64::from(a.velocity.1));
                let (sx, sy) = (f64::from(to.0 - from.0), aim_y - f64::from(from.1));
                let (v, s) = (dmath::hypot(vx, vy), dmath::hypot(sx, sy));
                if v < 1.0 || s < 1.0 {
                    0.0
                } else {
                    (vx.mul_add(sy, -(vy * sx)) / (v * s)).abs()
                }
            }),
            fatigue: self
                .bodies
                .get(&attacker)
                .map_or(0.0, crate::spatial::PlayerBody::mind_fatigue),
        };
        let outcome = ctx.hidden.resolve_shot(&ctx_shot);
        // F.3-b3: carrier released, ball flies. Goal (or Save) is
        // emitted at the crossing tick by `check_shot_crossing`, not
        // at launch: the goal is geometric, not the die's outcome.
        self.spatial.set_carrier(None);
        self.spatial
            .set_ball_in_flight(from, outcome.initial_velocity);
        self.spatial.struck_shot();
        let fact = crate::fact::Fact::Trajectory {
            actor: attacker,
            kind: crate::fact::TrajectoryKind::Shot {
                on_target: outcome.on_target,
                from,
                to: outcome.to,
                initial_velocity: outcome.initial_velocity,
                shot_type: outcome.shot_type,
            },
        };
        let foot = super::technique::foot_name(ctx_shot.foot);
        for mut c in crate::fact::classify(&fact) {
            if c.event_type == EventType::Shot {
                c.fields["foot"] = foot.into();
            }
            self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
        }
        match self.possession {
            Possession::Home => {
                self.shots.0 += 1;
            }
            Possession::Away => {
                self.shots.1 += 1;
            }
        }
        self.pending = Some(Pending {
            launched: tick,
            due_tick: tick + FLIGHT_TICKS,
            outcome: Self::other(self.possession),
            interceptor: None,
            passer: None,
            receiver: None,
            closest_cm: None,
            through: false,
            cross: false,
            foot: None,
            shot: Some(ShotPending {
                attacker,
                struck: tick,
                from,
            }),
        });
    }

    pub(crate) fn latency_scene(&self, actor: PlayerId, opponents: &[PlayerId]) -> (f64, f64) {
        let Some(actor_pos) = self.spatial.agent(actor).map(|a| a.position) else {
            return (0.0, 0.0);
        };
        let mut crowd_count: u32 = 0;
        let mut nearest_sq: i64 = i64::MAX;
        for &opp in opponents {
            let Some(op) = self.spatial.agent(opp).map(|a| a.position) else {
                continue;
            };
            let dx = i64::from(op.0 - actor_pos.0);
            let dy = i64::from(op.1 - actor_pos.1);
            let d_sq = dx * dx + dy * dy;
            if d_sq < nearest_sq {
                nearest_sq = d_sq;
            }
            let r = i64::from(CROWD_RADIUS_CM);
            if d_sq <= r * r {
                crowd_count += 1;
            }
        }
        let crowd_norm = f64::from(crowd_count.min(CROWD_MAX)) / f64::from(CROWD_MAX);
        // `nearest_sq` is i64::MAX when there are no opponents: threat 0.
        #[allow(clippy::cast_precision_loss)]
        let nearest_cm = if nearest_sq == i64::MAX {
            f64::MAX
        } else {
            #[allow(clippy::cast_precision_loss)]
            {
                (nearest_sq as f64).sqrt()
            }
        };
        let threat = (1.0 - nearest_cm / THREAT_RADIUS_CM).clamp(0.0, 1.0);
        (crowd_norm, threat)
    }

    /// Emits `RelationEnterExit` when an opponent enters (or exits) the
    /// coverage of the carrier (MD-04). Coverage is "opponent within
    /// `space_threshold_cm` of the ball": a geometric relation, not a
    /// per-tick event. The narrator classifies the enter fact as
    /// `Pressure` (T1); the engine emits the fact, not the event.
    pub(crate) fn update_coverage(&mut self, ctx: &mut MatchContext<'_>, tick: u64) {
        // Loaded once (it was parsed from JSON at every tick). An opponent
        // presses within the radius, and keeps pressing until he is
        // `pressure_exit_milli` of it away: without the hysteresis an
        // opponent on the edge "entered" again every tick.
        let p = self.pressure_balance;
        let r = i64::from(p.pressure_radius_cm);
        let threshold_sq = r * r;
        let exit = r * i64::from(p.pressure_exit_milli) / 1000;
        let exit_sq = exit * exit;
        let ball = self.spatial.ball();
        // Pressure is on a carrier: a ball in flight or loose is pressed by
        // nobody. Before `SIM_VERSION` 50 every opponent the ball flew past
        // "pressed" it; with passes at their real speed that was 2,000
        // Pressure events a match.
        if self.spatial.carrier().is_none() {
            self.coverage.clear();
            return;
        }

        // Compute the current coverage set (immutable phase).
        let (nominal_carrier, current): (PlayerId, BTreeSet<PlayerId>) = {
            let (own, opp) = self.squads(ctx);
            let mut set: BTreeSet<PlayerId> = BTreeSet::new();
            for p in &opp.players {
                if let Some(a) = self.spatial.agent(p.id) {
                    let d = distance_squared(a.position, ball);
                    if d <= threshold_sq || (self.coverage.contains(&p.id) && d <= exit_sq) {
                        set.insert(p.id);
                    }
                }
            }
            let Some(carrier) = nearest_in_squad(&self.spatial, own, ball) else {
                return;
            };
            (carrier, set)
        };

        // Possession flipped: the previous coverage referred to the other
        // team, discard it (do not emit enters against a stale reference).
        if self.possession != self.coverage_possession {
            self.coverage.clear();
            self.coverage_possession = self.possession;
        }

        // Diff after the flip handling, so a possession change starts from
        // an empty reference and every current member is an enter.
        let enters: Vec<PlayerId> = current.difference(&self.coverage).copied().collect();

        for opponent in enters {
            let fact = crate::fact::Fact::RelationEnterExit {
                carrier: nominal_carrier,
                opponent,
                entered: true,
            };
            for c in crate::fact::classify(&fact) {
                self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
            }
        }

        self.coverage = current;
    }
}

#[cfg(test)]
mod tests {
    use ds_match_api::MatchPlugin;

    #[test]
    fn the_plugin_has_its_card() {
        assert!(super::Thought.card().is_valid());
        assert_eq!(super::Thought.card().name, "thought");
    }
}
