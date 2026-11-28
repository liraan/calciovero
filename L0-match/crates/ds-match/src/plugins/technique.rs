//! The technique plugin (ADR-063 §4; ADR-051, ADR-058).
//!
//! The technique plugin (ADR-063 §4; ADR-051, ADR-058): the reception where the ball arrives, the high ball met, the header, the clearance, the block, the carry.
//! Moved here from `state.rs` (2026-11-15) as methods of the match state
//! they act on.

use ds_match_api::{MatchPlugin, PluginCard};
use ds_orchestrator::AerialContender;

use crate::spatial::STEPS;
use crate::state::prelude::*;

/// The technique.
#[derive(Debug, Clone, Copy, Default)]
pub struct Technique;

impl MatchPlugin for Technique {
    fn card(&self) -> PluginCard {
        crate::registry::card("technique")
    }
}

impl MatchState {
    /// The carrier is state, computed from the substrate (ADR-041 §5): a
    /// ball on the ground (at rest or rolling), with no pass or shot
    /// pending, belongs to the nearest
    /// player once he is within `loose_control_radius_cm` of it, and his
    /// team has the possession. Before this the ball stayed loose until the
    /// next decision, and the match stood still around it (OI-97).
    /// A pass from a wide position into the opponents' penalty area is a
    /// cross (ADR-051), lofted or low (since `SIM_VERSION` 92: the real
    /// definition counts the low cross and the cut-back): it is told as one,
    /// with the foot it was struck with (ADR-031).
    pub(crate) fn compute_cross(
        &self,
        ctx: &MatchContext<'_>,
        crosser: PlayerId,
        (from, to): ((i32, i32), (i32, i32)),
    ) -> bool {
        let (length, width) = pitch_cm(ctx);
        let home = ctx.match_.home.players.iter().any(|p| p.id == crosser);
        let goal_x = if home { length } else { 0 };
        is_cross(from, to, (goal_x, width), &self.laws)
    }

    /// Who meets a ball in the air at `(bx, by, bz)`, at small step `step`
    /// of the tick (ADR-069, SIM 148: each man where he was at that step):
    /// within his reach for a passing ball, and high enough (height, jump,
    /// a keeper's arms).
    pub(crate) fn aerial_contenders(
        &self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        (bx, by, bz): (i32, i32, i32),
        step: usize,
    ) -> Vec<AerialContender> {
        let (length, width) = pitch_cm(ctx);
        let radius = self.reception_balance.loose_control_radius_cm;
        let off = ctx.hidden.offball_balance();
        let passer = self.pending.as_ref().and_then(|p| p.passer);
        // Nobody meets in the air the ball he has just played, as nobody
        // receives his own pass (SIM 148): met at every small step, a ball
        // he had just headed up and out was still at his head the step
        // after, and he headed it again, and again.
        let touches = self.spatial.touches();
        let own = touches
            .played
            .filter(|p| touches.deflected.is_none() || touches.deflected == Some(*p));
        let m = ctx.match_;
        let mut contenders: Vec<AerialContender> = Vec::new();
        // Those who reach it (`near`), and those a body's width farther who
        // can only contest it against them, shoulder to shoulder.
        let mut shoulder: Vec<AerialContender> = Vec::new();
        for (squad, home) in [(&m.home, true), (&m.away, false)] {
            let keeper = Some(self.spatial.goalkeeper(home));
            for p in &squad.players {
                if Some(p.id) == passer
                    || Some(p.id) == own
                    || self.challenges.get(&p.id).is_some_and(|t| tick < *t)
                {
                    continue;
                }
                let Some(a) = self.spatial.body_at_step(p.id, step) else {
                    continue;
                };
                // In the air he contests it shoulder to shoulder: a body's
                // width beyond his reach, against one who reaches it (plan
                // L0-CLOSE 10; before, two bodies kept apart were almost
                // never both within reach of the ball in the same tick).
                let reach = ctx.hidden.ball_reach_cm(p.id, radius);
                let contest = reach + i64::from(2 * crate::spatial::BODY_RADIUS_CM);
                let d2 = distance_squared(a, (bx, by));
                if d2 > contest * contest {
                    continue;
                }
                let height = self.bodies.get(&p.id).map_or(180.0, |b| b.height_cm);
                let hands =
                    Some(p.id) == keeper && in_keeper_box((bx, by), home, (length, width), &off);
                if ctx.hidden.aerial_reach_cm(p.id, height, hands) >= f64::from(bz) {
                    let c = AerialContender {
                        id: p.id,
                        height_cm: height,
                        hands,
                        step,
                        within_reach: d2 <= reach * reach,
                    };
                    if c.within_reach {
                        contenders.push(c);
                    } else {
                        shoulder.push(c);
                    }
                }
            }
        }
        if !contenders.is_empty() {
            contenders.extend(shoulder);
        }
        contenders
    }

    fn header_shot_choice(
        &self,
        ctx: &mut MatchContext<'_>,
        winner: PlayerId,
        (bx, by): (i32, i32),
        goal: (i32, i32),
    ) -> Option<(f64, f64, f64)> {
        let (length, _) = pitch_cm(ctx);
        let home = ctx.match_.home.players.iter().any(|p| p.id == winner);
        let (my, their) = if home {
            (&ctx.match_.home, &ctx.match_.away)
        } else {
            (&ctx.match_.away, &ctx.match_.home)
        };
        let own_goal = (length - goal.0, goal.1);
        let gk = Some(self.spatial.goalkeeper(!home));
        let keeper_reach_cm = gk.and_then(|g| self.bodies.get(&g)).map_or(0.0, |b| {
            ctx.hidden.save_balance().standing_reach_cm(b.height_cm)
        });
        let teammates = my.players.iter().map(|p| p.id).collect();
        let opponents: Vec<_> = their.players.iter().map(|p| p.id).collect();
        let mut scene = crate::state::scene_from(
            &self.spatial,
            winner,
            (goal, own_goal),
            teammates,
            opponents,
            (gk, keeper_reach_cm),
            false,
        );
        scene.ball_foot = None;
        scene.strike_error_rad = 0.0;
        scene.reception.heights = scene
            .positions
            .keys()
            .filter_map(|id| self.bodies.get(id).map(|b| (*id, b.height_cm)))
            .collect();
        // His choice, with his own appetite and miss (ADR-112).
        ctx.hidden.choose_header_shot(winner, &scene, (bx, by))
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn header_at_goal_aimed(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        winner: PlayerId,
        (bx, by, bz): (i32, i32, i32),
        goal: (i32, i32),
        (aim_y, power, contact_height, _speed): (f64, f64, f64, f64),
    ) {
        let home = ctx.match_.home.players.iter().any(|p| p.id == winner);
        let contested = self
            .aerial_contenders(ctx, tick, (bx, by, bz), 0)
            .iter()
            .any(|c| c.id != winner);
        let difficulty = if contested { 100 } else { 0 };
        let ctx_shot = ds_orchestrator::ShotContext {
            shooter: winner,
            foot: None,
            from: (bx, by, bz),
            to: (goal.0, goal.1, 100),
            difficulty,
            aim_y,
            power,
            contact_height,
            momentum_y: 0,
            gravity_cm_per_tick2: self.spatial.gravity_cm_per_tick2(),
            air_friction_milli: self.spatial.air_friction_milli(),
            gait: 0.5,
            run_cm_per_tick: 0.0,
            across: 0.0,
            fatigue: self
                .bodies
                .get(&winner)
                .map_or(0.0, crate::spatial::PlayerBody::mind_fatigue),
            is_header: true,
        };
        let outcome = ctx.hidden.resolve_shot(&ctx_shot);
        self.spatial
            .deflect_ball((bx, by, bz), outcome.initial_velocity);
        self.spatial.played_by(winner);
        self.spatial.struck_shot();
        // Law 11 at a header at goal too (SIM 203, T21).
        self.mark_offside_at_pass(ctx, tick, winner, bx);
        self.pending = Some(Pending {
            launched: tick,
            interceptor: None,
            passer: None,
            receiver: None,
            closest_cm: None,
            through: false,
            cross: false,
            foot: None,
            restart: false,
            throw_in: false,
            indirect: false,
            shot: Some(ShotPending {
                attacker: winner,
                struck: tick,
                from: (bx, by, bz),
            }),
        });
        if home {
            self.shots.0 += 1;
        } else {
            self.shots.1 += 1;
        }
        let fact = crate::fact::Fact::Trajectory {
            actor: winner,
            kind: crate::fact::TrajectoryKind::Shot {
                on_target: outcome.on_target,
                from: (bx, by, bz),
                to: outcome.to,
                initial_velocity: outcome.initial_velocity,
                shot_type: outcome.shot_type,
            },
        };
        for c in crate::fact::classify(&fact) {
            self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
        }
    }

    /// A ball in the air meets the players it passes within reach of (plan
    /// L0-CLOSE step 4, ADR-042): horizontally their reach for a passing
    /// ball, in height their height and jump (a keeper in his area adds his
    /// arms). Two or more: an aerial duel. The winner heads it at goal in
    /// the opponents' area, away from his goal in his own half, down to his
    /// feet elsewhere; a keeper in his area catches it. A shot on its way
    /// is not met here (it is the keeper's, or a block).
    #[allow(clippy::too_many_lines)]
    pub(crate) fn meet_ball_in_air(&mut self, ctx: &mut MatchContext<'_>, tick: u64) {
        // A restart waiting: the ball is out of play (SIM 150).
        if self.waiting.is_some() {
            return;
        }
        if self.spatial.ball_state() != crate::spatial::BallState::InFlight {
            return;
        }
        if self.pending.as_ref().is_some_and(|p| p.shot.is_some()) {
            return;
        }
        // At the first small step of the tick where it passes within
        // someone's reach (ADR-069, SIM 148): the ball and the men where
        // each was at that step. Before, only where the tick ended, and a
        // ball crossing a man's head between two ticks went past him.
        // Below the band of the header it is the feet's.
        let (length, width) = pitch_cm(ctx);
        let path: Vec<(usize, (i32, i32, i32))> = self
            .spatial
            .ball_path()
            .into_iter()
            .enumerate()
            .filter(|(_, b)| b.2 >= FEET_BAND_CM)
            .collect();
        let Some(((bx, by, bz), mut contenders)) = path
            .iter()
            .map(|&(s, b)| (b, self.aerial_contenders(ctx, tick, b, s)))
            .find(|(_, c)| !c.is_empty())
        else {
            return;
        };
        // Everyone who reaches it along its way in this tick contests it, at
        // the point the first man meets it (SIM 205, T29 second half: before,
        // only a man who reached it in the very same small step, a fiftieth
        // of a second: one a step later never contested, and aerial duels
        // were six a match against thirty-eight).
        for &(s, b) in &path {
            for c in self.aerial_contenders(ctx, tick, b, s) {
                if !contenders.iter().any(|x| x.id == c.id) {
                    contenders.push(c);
                }
            }
        }
        let m = ctx.match_;
        let Some(winner) = ctx.hidden.resolve_aerial(&contenders) else {
            return;
        };
        let home = m.home.players.iter().any(|p| p.id == winner);
        if contenders.len() > 1 {
            let against = contenders
                .iter()
                .find(|c| c.id != winner)
                .map_or(0, |c| c.id.slot());
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(winner)),
                EventType::Duel,
                serde_json::json!({ "winner_slot": winner.slot(), "aerial": true, "against": against }),
            );
        }
        // Law 11 (SIM 196): a teammate of the passer who was in an offside
        // position when the ball was played and wins it in the air is
        // penalised as one who receives it on the ground. Before, the air
        // cleared the marks unread: a header from offside was never called.
        let passer_home = self
            .pending
            .as_ref()
            .filter(|p| p.shot.is_none())
            .and_then(|p| p.passer)
            .map(|p| ctx.match_.home.players.iter().any(|q| q.id == p));
        if passer_home == Some(home) && self.offside_at_pass.contains(&winner) {
            let played = self.pending.take().map(|p| p.launched);
            self.call_offside(ctx, tick, winner, home, ((bx, by), played));
            return;
        }
        // Law 12: read the deliberate back-pass before the flight is over;
        // the air does not make handling a teammate's kick lawful (DD-102).
        let back_pass = self.is_back_pass_to(winner);
        self.tell_pass_met_in_air(ctx, tick, home);
        self.pending = None;
        self.offside_at_pass.clear();
        self.possession = if home {
            Possession::Home
        } else {
            Possession::Away
        };
        let hands = contenders.iter().any(|c| c.id == winner && c.hands);
        let goal = (if home { length } else { 0 }, width / 2);
        let own = (length - goal.0, goal.1);
        let speed = f64::from(ctx.hidden.duel_balance().header_speed_cm_per_tick);
        let depth = |g: (i32, i32)| (g.0 - bx).abs();
        let laws = self.laws;
        let in_their_area = depth(goal) <= laws.law1.penalty_area_depth_cm
            && (by - goal.1).abs() <= laws.law1.penalty_area_half_width_cm;
        if hands {
            if back_pass && self.call_back_pass_offence(ctx, tick, winner, (bx, by)) {
                return;
            }
            // The keeper claims it.
            self.spatial.set_carrier(Some(winner));
            self.in_hands = Some(winner);
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(winner)),
                EventType::Save,
                serde_json::json!({ "x": bx, "y": by, "z": bz, "held": true, "claim": true }),
            );
        } else if in_their_area {
            if let Some((aim_y, power, contact_height)) =
                self.header_shot_choice(ctx, winner, (bx, by), goal)
            {
                self.header_at_goal_aimed(
                    ctx,
                    tick,
                    winner,
                    (bx, by, bz),
                    goal,
                    (aim_y, power, contact_height, speed),
                );
            } else {
                // No header at goal he would choose (SIM 204, T29): he
                // brings it down, as anywhere else on the pitch. Before, the
                // engine made every high ball won in their area a header at
                // the middle of the goal.
                self.spatial.set_carrier(Some(winner));
            }
        } else if depth(own) < length / 2 && passer_home != Some(home) {
            // Their ball, or nobody's, in his own half (SIM 204, T29: a pass
            // of his own side he brings down; before, every high ball in his
            // half was headed away, his keeper's short ball too).
            // A header away from his goal: up and out. A header is a play
            // like any other: he needs the same moment before he can play
            // the ball again as after a first-time clearance (SIM 135).
            // Without it, after the ball came back off the crosser's body,
            // the same defender headed it twice in two ticks.
            let retry = ctx.hidden.duel_balance().retry_ticks;
            self.challenges.insert(winner, tick + retry);
            let forward = if own.0 == 0 { 1.0 } else { -1.0 };
            #[allow(clippy::cast_possible_truncation)]
            let v = ((forward * speed) as i16, 0, (speed * 0.3) as i16);
            self.spatial.deflect_ball((bx, by, bz), v);
            self.spatial.played_by(winner);
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(winner)),
                EventType::Clearance,
                serde_json::json!({ "x": bx, "y": by, "header": true }),
            );
        } else {
            // Down to his feet.
            self.spatial.set_carrier(Some(winner));
        }
    }

    /// A pass met in the air is complete if a teammate of the passer wins
    /// it, lost if an opponent does (the real definition: a completed pass
    /// is one his side receives, by foot or head); `home` is the winner's
    /// side.
    fn tell_pass_met_in_air(&mut self, ctx: &mut MatchContext<'_>, tick: u64, home: bool) {
        let m = ctx.match_;
        if let Some((by, through)) = self
            .pending
            .as_ref()
            .filter(|p| p.shot.is_none())
            .and_then(|p| p.passer.map(|by| (by, p.through)))
        {
            let mate = m.home.players.iter().any(|p| p.id == by) == home;
            let fact = crate::fact::Fact::Trajectory {
                actor: by,
                kind: crate::fact::TrajectoryKind::Pass {
                    success: mate,
                    through,
                    cross: self.pending.as_ref().is_some_and(|p| p.cross),
                    foot: self.pending.as_ref().and_then(|p| p.foot),
                },
            };
            for c in crate::fact::classify(&fact) {
                self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
            }
        }
    }

    /// The carrier clears the ball (ADR-051, "clear"): his strongest kick,
    /// lifted, towards a point up the pitch on his side's wing; to no
    /// one. Out over a touchline it is a throw-in for the opponents.
    pub(crate) fn play_clearance(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        kicker: PlayerId,
        to: (i32, i32),
    ) {
        let (fx, fy, _) = self.spatial.ball_3d();
        let from = (fx, fy);
        // Where he aimed is where the ball goes (DD 5): clamping the point
        // back inside the pitch turned a deliberate kick out into a pass.
        // The pressure on his kick, as for the pass (SIM 132, OI-100).
        let pressure = {
            let (_, opponents) = self.squads(ctx);
            let r = ctx
                .hidden
                .shot_choice_balance()
                .kick_pressure_radius_cm
                .max(1);
            // Law 3 (SIM 230, DD-103): the man on his line does not press
            // the kicker. Read who holds the gloves now, not the first
            // entry of the squad: after a send-off in play that entry is
            // an outfield player who does press.
            let keeper = self.spatial.goalkeeper(self.possession == Possession::Away);
            opponents
                .players
                .iter()
                .filter(|o| o.id != keeper)
                .filter_map(|o| self.spatial.agent(o.id))
                .map(|a| distance_squared(a.position, from).isqrt())
                .min()
                .map_or(0.0, |d| {
                    #[allow(clippy::cast_precision_loss)]
                    let p = ((r - d).max(0) as f64) / (r as f64);
                    p
                })
        };
        let v = ctx.hidden.resolve_clearance(
            kicker,
            (from.0, from.1, 0),
            to,
            (
                self.spatial
                    .agent(kicker)
                    .map_or(0.5, |a| f64::from(a.gait_milli) / 1000.0),
                pressure,
            ),
        );
        self.spatial.set_carrier(None);
        self.spatial.set_ball_in_flight((from.0, from.1, 0), v);
        // He played it (SIM 203, T7): first time he was never its carrier, so
        // the last touch the referee reads (Laws 9, 10, 15-17) stayed the
        // attacker's pass or an older deflection.
        self.spatial.played_by(kicker);
        self.pending = None;
        // A clearance is not a pass: nobody receives his own (ADR-051,
        // ADR-072). The kicker needs a moment before he plays it again,
        // as after a failed touch (SIM 135, OI-119).
        let retry = ctx.hidden.duel_balance().retry_ticks;
        self.challenges.insert(kicker, tick + retry);
        self.emit_event(
            ctx,
            tick,
            Some(ActorId::Player(kicker)),
            EventType::Clearance,
            serde_json::json!({ "x": from.0, "y": from.1 }),
        );
    }

    /// A shot on its way meets a defender (ADR-051, "block"): an outfield
    /// opponent of the shooter whose reach for a passing ball (his
    /// reactivity) it passes within, below his block height, blocks it.
    /// The ball comes back loose; possession is the defenders', so out over
    /// the goal line it is a corner.
    pub(crate) fn block_shot(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        shooter: Option<PlayerId>,
    ) {
        let Some(shooter) = shooter else { return };
        let off = ctx.hidden.offball_balance();
        let m = ctx.match_;
        let shooter_home = m.home.players.iter().any(|p| p.id == shooter);
        let defenders = if shooter_home { &m.away } else { &m.home };
        let keeper = Some(self.spatial.goalkeeper(!shooter_home));
        let radius = self.reception_balance.loose_control_radius_cm;
        // Along the ball's way this tick, in the small steps it moved in
        // (ADR-069), as a high ball is met (SIM 148): the first point below
        // his block height within an outfield man's reach, the nearest of
        // them there (SIM 203, T6: before, only where the tick ended, so a
        // shot at 25 m/s, 2.5 m a tick, went through the man in its way;
        // and the first of the squad's order, not the nearest).
        let in_its_way = self
            .spatial
            .ball_path()
            .into_iter()
            .find_map(|(bx, by, bz)| {
                if bz > off.block_height_cm {
                    return None;
                }
                defenders
                    .players
                    .iter()
                    .map(|p| p.id)
                    .filter(|id| Some(*id) != keeper)
                    .filter_map(|id| {
                        let a = self.spatial.agent(id)?;
                        let reach = ctx.hidden.ball_reach_cm(id, radius);
                        let d = distance_squared(a.position, (bx, by));
                        (d <= reach * reach).then_some((d, id))
                    })
                    .min()
                    .map(|(_, id)| (id, (bx, by, bz)))
            });
        let Some((man, (bx, by, bz))) = in_its_way else {
            return;
        };
        let vel = self.spatial.ball_velocity();
        let body = self.spatial.agent(man).map_or((bx, by), |a| a.position);
        self.spatial.deflect_ball(
            (bx, by, bz),
            ds_calculator::obstacle::reflect_off_body(vel, body, (bx, by), off.block_rebound_milli),
        );
        self.spatial.deflected_by(man);
        // The block gives the ball to the defenders: the man who shot needs
        // the same moment a failed touch does before he takes it back
        // (SIM 211; before, it could come back to his own foot at +1).
        let retry = ctx.hidden.duel_balance().retry_ticks;
        self.challenges.insert(shooter, tick + retry);
        self.pending = None;
        self.possession = if shooter_home {
            Possession::Away
        } else {
            Possession::Home
        };
        self.emit_event(
            ctx,
            tick,
            Some(ActorId::Player(man)),
            EventType::Block,
            serde_json::json!({ "from": shooter.slot(), "x": bx, "y": by }),
        );
    }

    /// The nearest player within his own reach of the ball (his
    /// reactivity, ADR-051); one whose touch just failed needs a moment,
    /// and nobody takes his own pass.
    pub(crate) fn who_takes_the_ball(
        &self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        passer: Option<PlayerId>,
        path: ((i32, i32, i32), (i32, i32, i32)),
    ) -> Option<PlayerId> {
        let radius = self.reception_balance.loose_control_radius_cm;
        // The tick's small steps, when the path given is this tick's.
        let steps = {
            let p = self.spatial.ball_path();
            if p[STEPS - 1] == path.1 {
                p
            } else {
                [path.1; STEPS]
            }
        };
        let players: Vec<PlayerId> = ctx
            .match_
            .home
            .players
            .iter()
            .chain(ctx.match_.away.players.iter())
            .map(|p| p.id)
            .collect();
        let mut best: Option<(i64, PlayerId)> = None;
        for who in players {
            // Nobody receives his own pass.
            if Some(who) == passer {
                continue;
            }
            // A man whose touch just failed, or who just went in, needs a
            // moment before he plays a ball; a pass struck into his body
            // still hits him (SIM 135, DD-4, OI-119).
            if self.challenges.get(&who).is_some_and(|t| tick < *t) {
                continue;
            }
            let unready = self.unready(ctx, tick, passer, who);
            // Where the ball passes nearest him this tick, and how high it
            // is there: below the band of the header it is his foot's (a
            // ball coming down fast crosses his reach between two ticks).
            // Small step by small step (ADR-069, SIM 148): each stretch of
            // the ball's path against where he was at its end.
            let Some((d, _)) = (0..STEPS)
                .filter_map(|s| {
                    let pos = self.spatial.body_at_step(who, s)?;
                    let from = if s == 0 { path.0 } else { steps[s - 1] };
                    let (near, z) = nearest_on_path((from, steps[s]), pos);
                    Some((distance_squared(pos, near), z))
                })
                .filter(|(_, z)| *z < FEET_BAND_CM)
                .min_by_key(|(d, _)| *d)
            else {
                continue;
            };
            let reach = if unready {
                i64::from(crate::spatial::BODY_RADIUS_CM)
            } else if passer.is_none()
                && self.spatial.act(who) == Some(ds_orchestrator::OffBallAct::GoToBall)
            {
                // The movement towards a free ball includes the control
                // (ADR-084): the reach is his, chosen by his reading, not a
                // separate event that hands him the ball.
                i64::from(radius)
            } else {
                ctx.hidden.ball_reach_cm(who, radius)
            };
            if d <= reach * reach && best.is_none_or(|(bd, _)| d < bd) {
                best = Some((d, who));
            }
        }
        best.map(|(_, id)| id)
    }

    /// The choice at the reception (ADR-064, OI-118, SIM 134): before his
    /// touch, `id` imagines the scene he will have with the ball where it
    /// reaches him and weighs controlling it (`controls`: whether his touch
    /// masters it) against clearing it first time. Returns whether he
    /// cleared it.
    pub(crate) fn plays_first_time(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        id: PlayerId,
        controls: bool,
    ) -> bool {
        let (length, _) = pitch_cm(ctx);
        let home = ctx.match_.home.players.iter().any(|p| p.id == id);
        let (own, other) = if home {
            (&ctx.match_.home, &ctx.match_.away)
        } else {
            (&ctx.match_.away, &ctx.match_.home)
        };
        let teammates: Vec<PlayerId> = own.players.iter().map(|p| p.id).collect();
        let opponents: Vec<PlayerId> = other.players.iter().map(|p| p.id).collect();
        // The middle of the goal on this pitch (code audit C1: 3400).
        let goal = (if home { length } else { 0 }, pitch_cm(ctx).1 / 2);
        let own_goal = (length - goal.0, goal.1);
        let gk = opponents.first().copied();
        let keep = ctx.hidden.save_balance();
        let keeper_reach_cm = gk
            .and_then(|g| self.bodies.get(&g))
            .map_or(0.0, |b| keep.standing_reach_cm(b.height_cm));
        let mut scene = crate::state::scene_from(
            &self.spatial,
            id,
            (goal, own_goal),
            teammates,
            opponents,
            (gk, keeper_reach_cm),
            false,
        );
        scene.positions = self.as_seen_by(id, std::mem::take(&mut scene.positions));
        // He imagines himself where the ball reaches him.
        scene.positions.insert(id, self.spatial.ball());
        // Controlled, does he play it before the man on him gets there? His
        // own time to decide (`latency.json`, the carrier's), against the
        // nearest opponent's run at the pace and reaction he believes; if
        // he is challenged first, his body may still hold it (his shield).
        let (crowding, nearest_threat) = self.latency_scene(id, &scene.opponents);
        let decide = ctx
            .hidden
            .resolve_latency(&ds_orchestrator::LatencyContext {
                player: id,
                crowding,
                nearest_threat,
                fatigue: self.bodies.get(&id).map_or(0.0, |b| b.fatigue),
                ticks_to_scan: 0,
            })
            .ticks;
        let thought = ds_calculator::thought::ThoughtBalance::load();
        let ball = self.spatial.ball();
        #[allow(clippy::cast_precision_loss)]
        let arrive = scene
            .opponents
            .iter()
            .filter_map(|o| self.spatial.agent(*o))
            .map(|a| distance_squared(a.position, ball).isqrt() as f64)
            .fold(f64::INFINITY, f64::min)
            / thought.believed_speed_cm_per_tick.max(1.0)
            + thought.believed_reaction_ticks;
        #[allow(clippy::cast_precision_loss)]
        let challenged = thought.confidence(decide as f64 - arrive);
        let kept = challenged.mul_add(-(1.0 - ctx.hidden.shield_confidence(id)), 1.0);
        let Some(to) =
            ds_orchestrator::Actor::new(id).first_time(&scene, ctx.hidden, (controls, kept))
        else {
            return false;
        };
        self.pending = None;
        self.possession = if home {
            Possession::Home
        } else {
            Possession::Away
        };
        self.play_clearance(ctx, tick, id, to);
        true
    }

    /// Whether `who`, an opponent of the passer, has not yet reacted to the
    /// pass just struck (SIM 133, OI-100): his reaction from his
    /// reactivity, the keeper's formula. Until then he cannot play it; a
    /// ball struck into his body still hits him (SIM 135, DD-4).
    pub(crate) fn unready(
        &self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        passer: Option<PlayerId>,
        who: PlayerId,
    ) -> bool {
        let (Some(by), Some(launched)) = (passer, self.pending.as_ref().map(|p| p.launched)) else {
            return false;
        };
        let home = |x: PlayerId| ctx.match_.home.players.iter().any(|p| p.id == x);
        #[allow(clippy::cast_precision_loss)]
        let early = ((tick - launched) as f64) < ctx.hidden.reaction_ticks(who);
        home(by) != home(who) && early
    }

    /// A pass struck into the body of `id`, who had not reacted to it,
    /// comes off it (OI-119, SIM 135): reflected about the line from his
    /// body's centre to the ball, at the half speed of a ball nobody
    /// masters; the pass is already over.
    fn off_the_body(&mut self, ctx: &mut MatchContext<'_>, tick: u64, id: PlayerId, by: PlayerId) {
        self.comes_off(id);
        let retry = ctx.hidden.duel_balance().retry_ticks;
        self.challenges.insert(id, tick + retry);
        // The pass is over and the rebound is loose: the man who played it
        // needs the same moment as a failed touch before he takes it back
        // (SIM 211). Before, only the man it hit had to wait.
        self.challenges.insert(by, tick + retry);
        self.emit_event(
            ctx,
            tick,
            Some(ActorId::Player(id)),
            EventType::Block,
            serde_json::json!({ "pass": true, "from": by.slot() }),
        );
    }

    /// A ball `id` does not master comes off his body (OI-119): reflected
    /// about the line from his body's centre to the ball, at the half speed
    /// of a ball nobody masters (the heavy touch); a ball already going away
    /// from him goes on. The one contact for a pass he had not reacted to,
    /// a touch that fails, a pass he only gets a foot to (SIM 203, T2:
    /// before, a failed touch let the ball run through him at half speed,
    /// on its way: a defender who missed a ball rolling at his goal walked
    /// it in).
    fn comes_off(&mut self, id: PlayerId) {
        let v = self.spatial.ball_velocity();
        let ball = self.spatial.ball();
        let body = self.spatial.agent(id).map_or(ball, |a| a.position);
        let (nx, ny) = (f64::from(ball.0 - body.0), f64::from(ball.1 - body.1));
        let len = dmath::hypot(nx, ny);
        let (vx, vy) = (f64::from(v.0), f64::from(v.1));
        let (rx, ry) = if len > 0.0 {
            let (ux, uy) = (nx / len, ny / len);
            let dot = vx.mul_add(ux, vy * uy);
            if dot < 0.0 {
                ((2.0 * dot).mul_add(-ux, vx), (2.0 * dot).mul_add(-uy, vy))
            } else {
                (vx, vy)
            }
        } else {
            (-vx, -vy)
        };
        #[allow(clippy::cast_possible_truncation)]
        self.spatial.deflect_ball(
            self.spatial.ball_3d(),
            ((rx / 2.0) as i16, (ry / 2.0) as i16, 0),
        );
        self.spatial.deflected_by(id);
    }

    /// A moving ball nobody played to `id` (SIM 130, ADR-058; ADR-064):
    /// he chooses to play it first time or to control it, and his touch
    /// masters it or not. Returns whether he has it at his feet.
    fn receives_loose(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        id: PlayerId,
        their_side_home: bool,
    ) -> bool {
        let params = self.reception_params(ctx);
        let rctx = self.arrival(ctx, id, their_side_home);
        let controls = ctx.hidden.resolve_reception(&rctx, &params).controlled;
        if self.plays_first_time(ctx, tick, id, controls) {
            return false;
        }
        if !controls {
            self.comes_off(id);
            let retry = ctx.hidden.duel_balance().retry_ticks;
            self.challenges.insert(id, tick + retry);
            return false;
        }
        true
    }

    /// Where `id` would meet a pass rolling on the grass, with his own
    /// speed, acceleration and top speed (ADR-052: who gets there first;
    /// ADR-058). `None` if the ball is not rolling.
    pub(crate) fn meets_rolling_ball(
        &self,
        ctx: &mut MatchContext<'_>,
        id: PlayerId,
    ) -> Option<(i32, i32)> {
        if self.spatial.ball_state() != crate::spatial::BallState::InFlight
            || self.spatial.ball_3d().2 != 0
        {
            return None;
        }
        let a = self.spatial.agent(id)?;
        let body = self.bodies.get(&id)?;
        let o = ctx
            .hidden
            .resolve_movement(&ds_orchestrator::MovementContext {
                player: id,
                height_cm: body.height_cm,
                weight_kg: body.weight_kg,
                fatigue: body.fatigue,
            });
        let velocity = (f64::from(a.velocity.0), f64::from(a.velocity.1));
        let reach = f64::from(crate::spatial::BODY_RADIUS_CM) * 2.0;
        #[allow(clippy::cast_possible_truncation)]
        let head = body.height_cm.round() as i32;
        // At the pace he runs into a contest, as he will run (SIM 203, T1).
        self.spatial.intercept(
            a.position,
            (
                velocity,
                o.acceleration * o.stride,
                o.v_max * o.stride * o.contest_pace_share,
            ),
            (reach, head),
        )
    }

    /// The reception's references, from where they are defined (ADR-058):
    /// the kick's reference speed, and the farthest an opponent can contest
    /// the ball, his slide (`duel.json`: `reach_cm` x `slide_reach_milli`).
    /// Beyond it he cannot play the ball; nearer, a contest with the body is
    /// the duel's, with its fouls (ADR-042; the maintainer, 2026-11-14:
    /// shielding the ball is one thing, using the arms to take it another).
    pub(crate) fn reception_params(
        &self,
        ctx: &MatchContext<'_>,
    ) -> ds_orchestrator::ReceptionParams {
        let duel = ctx.hidden.duel_balance();
        let reach = duel.slide_reach_cm();
        self.reception_balance.to_params(
            ctx.hidden.kick_balance().speed_ref_cm_per_tick,
            i32::try_from(reach).unwrap_or(i32::MAX),
        )
    }

    /// What `receiver` meets as the ball reaches him (ADR-058): its speed
    /// now, and how far the nearest player of the other side (`home` the
    /// side that presses him) is from the ball.
    pub(crate) fn arrival(
        &self,
        ctx: &MatchContext<'_>,
        receiver: PlayerId,
        home: bool,
    ) -> ds_orchestrator::ReceptionContext {
        let ball = self.spatial.ball();
        let v = self.spatial.ball_velocity();
        let squad = if home {
            &ctx.match_.home
        } else {
            &ctx.match_.away
        };
        let opponent_cm = squad
            .players
            .iter()
            .filter_map(|p| self.spatial.agent(p.id))
            .map(|a| distance_squared(a.position, ball).isqrt())
            .min()
            .unwrap_or(i64::MAX);
        ds_orchestrator::ReceptionContext {
            receiver,
            ball_speed_cm_per_tick: dmath::hypot(f64::from(v.0), f64::from(v.1)),
            opponent_cm,
        }
    }

    /// A pass on its way that has gone past its receiver (farther from him
    /// than the closest it came, by more than his reach) or has stopped is
    /// lost (ADR-058): `Pass { complete: false }`, and the ball is anyone's.
    /// The possession changes only when someone controls it.
    pub(crate) fn lose_a_missed_pass(&mut self, ctx: &mut MatchContext<'_>, tick: u64) {
        let Some(p) = self.pending.as_ref() else {
            return;
        };
        // A shot that stopped short of the line is over too: the ball is
        // anyone's (no clock ends it, ADR-058).
        if p.shot.is_some() {
            if self.spatial.ball_state() != crate::spatial::BallState::InFlight {
                self.pending = None;
            }
            return;
        }
        if p.interceptor.is_some() {
            return;
        }
        let (Some(receiver), Some(passer), through) = (p.receiver, p.passer, p.through) else {
            return;
        };
        let Some(at) = self.spatial.agent(receiver).map(|a| a.position) else {
            return;
        };
        let d = distance_squared(at, self.spatial.ball()).isqrt();
        let reach = ctx
            .hidden
            .ball_reach_cm(receiver, self.reception_balance.loose_control_radius_cm);
        let stopped = self.spatial.ball_state() != crate::spatial::BallState::InFlight;
        let closest = p.closest_cm.map_or(d, |c| c.min(d));
        let past = d > closest + reach;
        if let Some(p) = self.pending.as_mut() {
            p.closest_cm = Some(closest);
        }
        if !(stopped || past) {
            return;
        }
        let (cross, foot) = self
            .pending
            .as_ref()
            .map_or((false, None), |p| (p.cross, p.foot));
        self.pending = None;
        // The marks stay (SIM 200, code audit S12, Law 11): a man in an
        // offside position when it was played who takes the missed pass is
        // offside; the marks go when someone not marked takes it.
        let fact = crate::fact::Fact::Trajectory {
            actor: passer,
            kind: crate::fact::TrajectoryKind::Pass {
                success: false,
                through,
                cross,
                foot,
            },
        };
        for c in crate::fact::classify(&fact) {
            self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
        }
    }

    #[allow(clippy::too_many_lines)] // pre-existing 105/100, extract at next pass
    pub(crate) fn control_loose_ball(&mut self, ctx: &mut MatchContext<'_>, tick: u64) {
        // Low enough for his foot somewhere on its path this tick: at rest,
        // rolling, or coming down (a footballer controls a ball that is
        // still moving, and a lofted pass where it comes down: ADR-058).
        // A ball high in the air is not his foot's yet.
        // A restart waiting: the ball is out of play (SIM 150).
        let low = self.waiting.is_none()
            && match self.spatial.ball_state() {
                crate::spatial::BallState::Loose => true,
                crate::spatial::BallState::InFlight => {
                    // Its lowest at any small step (ADR-069, SIM 148).
                    self.spatial
                        .ball_path()
                        .iter()
                        .map(|b| b.2)
                        .fold(self.ball_from.2, i32::min)
                        < FEET_BAND_CM
                }
                crate::spatial::BallState::Possessed => false,
            };
        // A pass on its way can be received where it reaches a player
        // (ADR-051, "receive"): before `SIM_VERSION` 52 nobody could touch
        // it until a fixed 1.5 s had passed, and a weighed pass rolled past
        // its receiver. A shot on its way is the keeper's.
        let pass = match &self.pending {
            Some(p) if p.shot.is_none() => Some((p.passer, p.through)),
            Some(p) => {
                let shooter = p.shot.as_ref().map(|s| s.attacker);
                self.block_shot(ctx, tick, shooter);
                return;
            }
            None => None,
        };
        if !low {
            return;
        }
        // The nearest player who reaches it, each with his own reach (his
        // reactivity, ADR-051), where its path this tick passes him; one
        // whose touch just failed needs a moment.
        let ball = self.spatial.ball();
        let passer = pass.and_then(|(by, _)| by);
        let path = (self.ball_from, self.spatial.ball_3d());
        let Some(id) = self.who_takes_the_ball(ctx, tick, passer, path) else {
            return;
        };
        let home = |who: PlayerId| ctx.match_.home.players.iter().any(|p| p.id == who);
        // A loose ball (a missed pass, a deflection) taken by a man who was
        // in an offside position when his teammate played it: offside (Law
        // 11, SIM 200, code audit S12; before, the marks were cleared when
        // the pass was missed). Taken by anyone else, the marks go.
        if pass.is_none() {
            if self.offside_at_pass.contains(&id) {
                let offender_home = home(id);
                let played = self.offside_played;
                self.call_offside(ctx, tick, id, offender_home, (ball, played));
                return;
            }
            self.offside_at_pass.clear();
        }
        // A ball nobody played to him (loose from a contact, deflected,
        // cleared, bouncing) is a reception like any other (ADR-058; OI-117,
        // SIM 130): his first touch against its speed and the nearest
        // opponent as it reaches him. Not mastered, it runs on, slower, for
        // whoever. Before, whoever reached it first had it at his feet,
        // whatever its speed.
        if pass.is_none()
            && self.spatial.ball_state() == crate::spatial::BallState::InFlight
            && !self.receives_loose(ctx, tick, id, !home(id))
        {
            return;
        }
        if let Some((Some(by), through)) = pass {
            // Before the pass is over: whether he had reacted to it (OI-119).
            let struck = self.unready(ctx, tick, Some(by), id);
            let (cross, foot, played, restart) = self
                .pending
                .as_ref()
                .map_or((false, None, None, false), |p| {
                    (p.cross, p.foot, Some(p.launched), p.restart)
                });
            self.pending = None;
            if home(by) == home(id) && self.offside_at_pass.contains(&id) {
                self.call_offside(ctx, tick, id, home(id), (ball, played));
                return;
            }
            self.offside_at_pass.clear();
            if home(by) == home(id) {
                // A teammate receives: his first touch against the ball and
                // the opponent as it arrives (ADR-058).
                let params = self.reception_params(ctx);
                let rctx = self.arrival(ctx, id, !home(id));
                let rec = ctx.hidden.resolve_reception(&rctx, &params);
                let fact = crate::fact::Fact::Trajectory {
                    actor: by,
                    kind: crate::fact::TrajectoryKind::Pass {
                        success: rec.controlled,
                        through,
                        cross,
                        foot,
                    },
                };
                // One kick, one event: the restart was told at the whistle.
                // Wyscout records the goal kick, corner, throw-in or free
                // kick once, not again as a Pass when the ball lands.
                let told = if restart {
                    Vec::new()
                } else {
                    crate::fact::classify(&fact)
                };
                for c in told {
                    self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
                }
                if !rec.controlled {
                    // A heavy touch: the ball comes off him, slower, for
                    // whoever.
                    self.comes_off(id);
                    let retry = ctx.hidden.duel_balance().retry_ticks;
                    self.challenges.insert(id, tick + retry);
                    return;
                }
                // Strato 3: a completed pass between two teammates grows
                // their mutual intesa (decision BB).
                // A pair with no intesa yet (a man who came on) starts from
                // the baseline (`offball.json`; code audit C5: 0.1 and 0.6
                // were written here).
                let off = ds_calculator::offball::OffBallBalance::load();
                let (baseline, step) = (
                    f64::from(off.intesa_baseline_milli) / 1000.0,
                    f64::from(off.intesa_step_milli) / 1000.0,
                );
                for pair in [(by, id), (id, by)] {
                    let v = self.intesa.entry(pair).or_insert(baseline);
                    *v = (*v + step).min(1.0);
                }
            } else {
                // Struck into him before he reacted: it hits him and runs on.
                if struck {
                    self.off_the_body(ctx, tick, id, by);
                    return;
                }
                if self.deflects_the_pass(ctx, tick, (id, by)) {
                    return;
                }
                let mut fields = serde_json::json!({
                    "from": by.slot(),
                    "to": id.slot()
                });
                if restart {
                    fields["restart"] = serde_json::json!(true);
                }
                self.emit_event(
                    ctx,
                    tick,
                    Some(ActorId::Player(id)),
                    EventType::Interception,
                    fields,
                );
            }
        }
        self.spatial.set_carrier(Some(id));
        self.possession = if home(id) {
            Possession::Home
        } else {
            Possession::Away
        };
        // He has just controlled it: he decides after his own time, read
        // in his scene (ADR-044 §6), not on the tick he receives it.
        let opponents = if home(id) {
            &ctx.match_.away
        } else {
            &ctx.match_.home
        };
        let wait = self.decision_time(ctx, id, opponents);
        self.latencies.insert(id, tick.saturating_add(wait));
        self.clock_from_reception = Some(id);
    }

    /// An opponent the pass reaches controls it by the same touch as a
    /// receiver (plan L0-CLOSE 10c, ADR-058): his first touch against the
    /// ball's speed and the passer's nearest teammate as it arrives. Not
    /// controlled, he only deflects it: the ball runs on, slower, for
    /// whoever. Returns `true` if he deflected it.
    pub(crate) fn deflects_the_pass(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        (id, by): (PlayerId, PlayerId),
    ) -> bool {
        let by_home = ctx.match_.home.players.iter().any(|p| p.id == by);
        let params = self.reception_params(ctx);
        let rctx = self.arrival(ctx, id, by_home);
        let rec = ctx.hidden.resolve_reception(&rctx, &params);
        if self.plays_first_time(ctx, tick, id, rec.controlled) {
            return true;
        }
        if rec.controlled {
            return false;
        }
        self.comes_off(id);
        let retry = ctx.hidden.duel_balance().retry_ticks;
        self.challenges.insert(id, tick + retry);
        // The pass is over and the rebound is loose: the man who played it
        // needs the same moment as a failed touch before he takes it back
        // (SIM 211). Before, only the man it hit had to wait.
        self.challenges.insert(by, tick + retry);
        self.emit_event(
            ctx,
            tick,
            Some(ActorId::Player(id)),
            EventType::Block,
            serde_json::json!({ "pass": true, "from": by.slot() }),
        );
        true
    }

    /// The carrier and the ball move together (plan L0-CLOSE 11): his
    /// touches, his first touch, and the lines the ball may cross as he
    /// takes it.
    pub(crate) fn carry_the_ball(&mut self, ctx: &mut MatchContext<'_>, tick: u64) {
        // A restart waiting: the ball is out of play (SIM 150).
        if self.waiting.is_some() {
            return;
        }
        let before_carry = self.spatial.ball_3d();
        let hold = self.in_hands.is_some() || self.restart_pass.is_some();
        let keep_max = f64::from(self.reception_balance.first_touch_keep_max_milli) / 1000.0;
        let radius = self.reception_balance.loose_control_radius_cm;
        let reach = self
            .spatial
            .carrier()
            .map_or(0, |c| ctx.hidden.ball_reach_cm(c, radius));
        let hidden = &mut *ctx.hidden;
        let periods = std::cell::RefCell::new(hidden);
        self.spatial.follow_carrier(
            tick,
            hold,
            |p, pace| periods.borrow_mut().touch_period_ticks(p, pace),
            |p, incoming| periods.borrow_mut().control_spill(p, incoming, keep_max),
            reach,
        );
        // The carried ball has moved with him: it may have crossed a line.
        let after_carry = self.spatial.ball_3d();
        self.check_goal_line(ctx, tick, before_carry, after_carry);
        self.restart_if_out(ctx, tick);
    }
}

/// The foot a ball was struck with, as the facts tell it (ADR-031):
/// `either` when he had it in front of him.
pub(crate) const fn foot_name(foot: Option<ds_orchestrator::Side>) -> &'static str {
    match foot {
        Some(ds_orchestrator::Side::Left) => "left",
        Some(ds_orchestrator::Side::Right) => "right",
        None => "either",
    }
}

#[cfg(test)]
mod tests {
    use ds_match_api::MatchPlugin;

    #[test]
    fn the_plugin_has_its_card() {
        assert!(super::Technique.card().is_valid());
        assert_eq!(super::Technique.card().name, "technique");
    }

    /// SIM 218: the kick of a restart and the pass at its landing are one
    /// act. Same scene, only the origin of the pass changes.
    #[test]
    fn a_restart_kick_is_not_told_as_a_second_pass() {
        use crate::context::MatchContext;
        use crate::state::{MatchState, Pending, Possession};
        use ds_stats::{EventChain, EventType};

        let m = ds_fixtures::match_fixture(42).expect("fixture");
        let passer = m.home.players[6].id;
        let mate = m.home.players[9].id;
        let at = (5000, 3400);
        let told = |restart: bool| {
            let mut hidden = ds_orchestrator::GroundTruth::for_match(42, &m);
            let mut chain = EventChain::new();
            let mut ctx = MatchContext {
                match_: &m,
                hidden: &mut hidden,
                chain: &mut chain,
                config: crate::context::MatchConfig::default(),
            };
            let mut s = MatchState::new(&m);
            s.spatial.set_position(passer, (at.0 - 150, at.1));
            s.spatial.set_position(mate, (at.0 + 60, at.1));
            s.possession = Possession::Home;
            s.pending = Some(Pending {
                launched: 100,
                shot: None,
                interceptor: None,
                passer: Some(passer),
                receiver: Some(mate),
                closest_cm: None,
                through: false,
                cross: false,
                foot: None,
                restart,
                throw_in: false,
                indirect: false,
            });
            s.spatial.set_ball_in_flight((at.0, at.1, 0), (20, 0, 0));
            s.ball_from = s.spatial.ball_3d();
            s.spatial.advance_ball();
            s.control_loose_ball(&mut ctx, 110);
            ctx.chain
                .events()
                .iter()
                .filter(|e| e.event_type == EventType::Pass)
                .count()
        };
        assert_eq!(told(false), 1, "a normal pass is told once");
        assert_eq!(told(true), 0, "the restart is not told again as a Pass");
    }

    /// Law 3 twin (DD-103/SIM 230): the kicker and both opponents stand in
    /// the same places; only the man holding the gloves changes. Pressure
    /// follows the gloves, not the first entry of the squad.
    #[test]
    fn the_clearance_pressure_follows_the_gloves_not_the_slot() {
        use crate::context::{MatchConfig, MatchContext};
        use crate::state::{MatchState, Possession};
        use ds_stats::EventChain;

        let m = ds_fixtures::match_fixture(42).expect("fixture");
        let kicker = m.home.players[6].id;
        let far = m.away.players[0].id;
        let near = m.away.players[1].id;
        let length = i32::from(m.pitch.length_m) * 100;
        let centre = i32::from(m.pitch.width_m) * 100 / 2;
        let at = (length / 2, centre);
        let kick = |gloves_to_near: bool| {
            let mut hidden = ds_orchestrator::GroundTruth::for_match(42, &m);
            let mut chain = EventChain::new();
            let mut ctx = MatchContext {
                match_: &m,
                hidden: &mut hidden,
                chain: &mut chain,
                config: MatchConfig::default(),
            };
            let mut s = MatchState::new(&m);
            s.possession = Possession::Home;
            s.spatial.set_position(kicker, at);
            s.spatial.set_position(far, (length, centre));
            s.spatial.set_position(near, (at.0 - 120, at.1));
            if gloves_to_near {
                s.spatial.give_gloves(false, near);
            }
            s.spatial.set_ball_in_flight((at.0, at.1, 0), (0, 0, 0));
            s.play_clearance(&mut ctx, 1, kicker, (at.0 + 3000, at.1));
            s.spatial.ball_velocity()
        };
        let slot_keeper = kick(false);
        let near_keeper = kick(true);
        assert_ne!(
            slot_keeper, near_keeper,
            "the same clearance met different pressure only because the gloves moved"
        );
    }
}
