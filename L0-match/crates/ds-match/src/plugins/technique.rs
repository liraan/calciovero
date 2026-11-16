//! The technique plugin (ADR-063 §4; ADR-051, ADR-058).
//!
//! The technique plugin (ADR-063 §4; ADR-051, ADR-058): the reception where the ball arrives, the high ball met, the header, the clearance, the block, the carry.
//! Moved here from `state.rs` (2026-11-15) as methods of the match state
//! they act on.

use ds_match_api::{MatchPlugin, PluginCard};

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
    /// definition counts the low cross and the cut-back): it is told as one.
    pub(crate) fn tell_cross(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        crosser: PlayerId,
        from: (i32, i32),
        to: (i32, i32),
    ) {
        let (length, width) = pitch_cm(ctx);
        let home = ctx.match_.home.players.iter().any(|p| p.id == crosser);
        let goal_x = if home { length } else { 0 };
        if is_cross(from, to, (goal_x, width), &self.laws) {
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(crosser)),
                EventType::Cross,
                serde_json::json!({ "x": from.0, "y": from.1 }),
            );
        }
    }

    /// Who meets a ball in the air at `(bx, by, bz)`: within his reach for
    /// a passing ball, and high enough (height, jump, a keeper's arms).
    pub(crate) fn aerial_contenders(
        &self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        (bx, by, bz): (i32, i32, i32),
    ) -> Vec<(PlayerId, f64, bool)> {
        let (length, width) = pitch_cm(ctx);
        let radius = self.reception_balance.loose_control_radius_cm;
        let off = ctx.hidden.offball_balance();
        let passer = self.pending.as_ref().and_then(|p| p.passer);
        let m = ctx.match_;
        let mut contenders: Vec<(PlayerId, f64, bool)> = Vec::new();
        // Those who reach it (`near`), and those a body's width farther who
        // can only contest it against them, shoulder to shoulder.
        let mut shoulder: Vec<(PlayerId, f64, bool)> = Vec::new();
        for (squad, home) in [(&m.home, true), (&m.away, false)] {
            let keeper = squad.players.first().map(|p| p.id);
            for p in &squad.players {
                if Some(p.id) == passer || self.challenges.get(&p.id).is_some_and(|t| tick < *t) {
                    continue;
                }
                let Some(a) = self.spatial.agent(p.id).map(|a| a.position) else {
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
                    if d2 <= reach * reach {
                        contenders.push((p.id, height, hands));
                    } else {
                        shoulder.push((p.id, height, hands));
                    }
                }
            }
        }
        if !contenders.is_empty() {
            contenders.extend(shoulder);
        }
        contenders
    }

    /// A header at goal (plan L0-CLOSE step 4): a shot, from the head.
    pub(crate) fn header_at_goal(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        winner: PlayerId,
        (bx, by, bz): (i32, i32, i32),
        goal: (i32, i32),
        speed: f64,
    ) {
        let home = ctx.match_.home.players.iter().any(|p| p.id == winner);
        let (dx, dy) = (f64::from(goal.0 - bx), f64::from(goal.1 - by));
        let len = dmath::hypot(dx, dy).max(1.0);
        #[allow(clippy::cast_possible_truncation)]
        let v = ((dx / len * speed) as i16, (dy / len * speed) as i16, 10);
        self.spatial.deflect_ball((bx, by, bz), v);
        self.spatial.played_by(winner);
        self.spatial.struck_shot();
        self.pending = Some(Pending {
            launched: tick,
            due_tick: tick + FLIGHT_TICKS,
            outcome: Self::other(self.possession),
            interceptor: None,
            passer: None,
            receiver: None,
            closest_cm: None,
            through: false,
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
                on_target: true,
                from: (bx, by, bz),
                to: (goal.0, goal.1, 100),
                initial_velocity: v,
                shot_type: ds_orchestrator::ShotType::Header,
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
    pub(crate) fn meet_ball_in_air(&mut self, ctx: &mut MatchContext<'_>, tick: u64) {
        if self.spatial.ball_state() != crate::spatial::BallState::InFlight {
            return;
        }
        if self.pending.as_ref().is_some_and(|p| p.shot.is_some()) {
            return;
        }
        let (bx, by, bz) = self.spatial.ball_3d();
        if bz < FEET_BAND_CM {
            return; // at the feet: the ground control takes it
        }
        let (length, width) = pitch_cm(ctx);
        let contenders = self.aerial_contenders(ctx, tick, (bx, by, bz));
        let m = ctx.match_;
        let Some(winner) = ctx.hidden.resolve_aerial(&contenders) else {
            return;
        };
        let home = m.home.players.iter().any(|p| p.id == winner);
        if contenders.len() > 1 {
            let against = contenders
                .iter()
                .find(|c| c.0 != winner)
                .map_or(0, |c| c.0.slot());
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(winner)),
                EventType::Duel,
                serde_json::json!({ "winner_slot": winner.slot(), "aerial": true, "against": against }),
            );
        }
        // A pass met in the air is complete if a teammate of the passer
        // wins it, lost if an opponent does (the real definition: a
        // completed pass is one his side receives, by foot or head).
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
                },
            };
            for c in crate::fact::classify(&fact) {
                self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
            }
        }
        self.pending = None;
        self.offside_at_pass.clear();
        self.possession = if home {
            Possession::Home
        } else {
            Possession::Away
        };
        let hands = contenders.iter().any(|c| c.0 == winner && c.2);
        let goal = (if home { length } else { 0 }, width / 2);
        let own = (length - goal.0, goal.1);
        let speed = f64::from(ctx.hidden.duel_balance().header_speed_cm_per_tick);
        let depth = |g: (i32, i32)| (g.0 - bx).abs();
        let laws = self.laws;
        let in_their_area = depth(goal) <= laws.penalty_area_depth_cm
            && (by - goal.1).abs() <= laws.penalty_area_half_width_cm;
        if hands {
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
            self.header_at_goal(ctx, tick, winner, (bx, by, bz), goal, speed);
        } else if depth(own) < length / 2 {
            // A header away from his goal: up and out.
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
        let (length, width) = pitch_cm(ctx);
        let to = (to.0.clamp(0, length), to.1.clamp(0, width));
        // The pressure on his kick, as for the pass (SIM 132, OI-100).
        let pressure = {
            let (_, opponents) = self.squads(ctx);
            let r = ctx
                .hidden
                .shot_choice_balance()
                .kick_pressure_radius_cm
                .max(1);
            opponents
                .players
                .iter()
                .skip(1)
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
        self.pending = None;
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
        let (bx, by, bz) = self.spatial.ball_3d();
        if bz > off.block_height_cm {
            return;
        }
        let m = ctx.match_;
        let shooter_home = m.home.players.iter().any(|p| p.id == shooter);
        let defenders = if shooter_home { &m.away } else { &m.home };
        let keeper = defenders.players.first().map(|p| p.id);
        let radius = self.reception_balance.loose_control_radius_cm;
        let blocker = defenders
            .players
            .iter()
            .map(|p| p.id)
            .filter(|id| Some(*id) != keeper)
            .find(|id| {
                self.spatial.agent(*id).is_some_and(|a| {
                    let reach = ctx.hidden.ball_reach_cm(*id, radius);
                    distance_squared(a.position, (bx, by)) <= reach * reach
                })
            });
        let Some(blocker) = blocker else { return };
        let vel = self.spatial.ball_velocity();
        let body = self.spatial.agent(blocker).map_or((bx, by), |a| a.position);
        self.spatial.deflect_ball(
            (bx, by, bz),
            ds_calculator::obstacle::reflect_off_body(vel, body, (bx, by), off.block_rebound_milli),
        );
        self.spatial.deflected_by(blocker);
        self.pending = None;
        self.possession = if shooter_home {
            Possession::Away
        } else {
            Possession::Home
        };
        self.emit_event(
            ctx,
            tick,
            Some(ActorId::Player(blocker)),
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
            if Some(who) == passer || self.challenges.get(&who).is_some_and(|t| tick < *t) {
                continue;
            }
            // A ball just struck is played by an opponent only once he has
            // reacted to it (SIM 133, OI-100): his reaction from his
            // reactivity, the keeper's formula. The passer reckons with it
            // (the thought's `believed_reaction_ticks`); before, the man a
            // step from him took it as it left the foot.
            if let (Some(by), Some(launched)) = (passer, self.pending.as_ref().map(|p| p.launched))
            {
                let mates = |a: PlayerId, b: PlayerId| {
                    let home = |x: PlayerId| ctx.match_.home.players.iter().any(|p| p.id == x);
                    home(a) == home(b)
                };
                #[allow(clippy::cast_precision_loss)]
                if !mates(by, who) && ((tick - launched) as f64) < ctx.hidden.reaction_ticks(who) {
                    continue;
                }
            }
            let Some(pos) = self.spatial.agent(who).map(|a| a.position) else {
                continue;
            };
            // Where the ball passes nearest him this tick, and how high it
            // is there: below the band of the header it is his foot's (a
            // ball coming down fast crosses his reach between two ticks).
            let (near, z) = nearest_on_path(path, pos);
            if z >= FEET_BAND_CM {
                continue;
            }
            let d = distance_squared(pos, near);
            let reach = ctx.hidden.ball_reach_cm(who, radius);
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
        let goal = (if home { length } else { 0 }, 3400);
        let own_goal = (length - goal.0, goal.1);
        let gk = opponents.first().copied();
        let keeper_reach_cm = gk.and_then(|g| self.bodies.get(&g)).map_or(0.0, |b| {
            b.height_cm * ctx.hidden.save_balance().standing_reach_share
        });
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
        scene.horizon_cm = ctx.hidden.horizon_cm(id);
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
        let speed = dmath::hypot(f64::from(a.velocity.0), f64::from(a.velocity.1));
        let reach = f64::from(crate::spatial::BODY_RADIUS_CM) * 2.0;
        self.spatial.intercept(
            a.position,
            (speed, o.acceleration * o.stride, o.v_max * o.stride),
            reach,
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
        let reach = i64::from(duel.reach_cm) * i64::from(duel.slide_reach_milli) / 1000;
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
        self.pending = None;
        self.offside_at_pass.clear();
        let fact = crate::fact::Fact::Trajectory {
            actor: passer,
            kind: crate::fact::TrajectoryKind::Pass {
                success: false,
                through,
            },
        };
        for c in crate::fact::classify(&fact) {
            self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
        }
    }

    pub(crate) fn control_loose_ball(&mut self, ctx: &mut MatchContext<'_>, tick: u64) {
        // Low enough for his foot somewhere on its path this tick: at rest,
        // rolling, or coming down (a footballer controls a ball that is
        // still moving, and a lofted pass where it comes down: ADR-058).
        // A ball high in the air is not his foot's yet.
        let low = match self.spatial.ball_state() {
            crate::spatial::BallState::Loose => true,
            crate::spatial::BallState::InFlight => {
                self.ball_from.2.min(self.spatial.ball_3d().2) < FEET_BAND_CM
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
        // A ball nobody played to him (loose from a contact, deflected,
        // cleared, bouncing) is a reception like any other (ADR-058; OI-117,
        // SIM 130): his first touch against its speed and the nearest
        // opponent as it reaches him. Not mastered, it runs on, slower, for
        // whoever. Before, whoever reached it first had it at his feet,
        // whatever its speed.
        if pass.is_none() && self.spatial.ball_state() == crate::spatial::BallState::InFlight {
            let params = self.reception_params(ctx);
            let rctx = self.arrival(ctx, id, !home(id));
            let controls = ctx.hidden.resolve_reception(&rctx, &params).controlled;
            if self.plays_first_time(ctx, tick, id, controls) {
                return;
            }
            if !controls {
                let v = self.spatial.ball_velocity();
                self.spatial
                    .deflect_ball(self.spatial.ball_3d(), (v.0 / 2, v.1 / 2, 0));
                self.spatial.deflected_by(id);
                let retry = ctx.hidden.duel_balance().retry_ticks;
                self.challenges.insert(id, tick + retry);
                return;
            }
        }
        if let Some((Some(by), through)) = pass {
            self.pending = None;
            if home(by) == home(id) && self.offside_at_pass.contains(&id) {
                self.call_offside(ctx, tick, id, home(id), ball);
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
                    },
                };
                for c in crate::fact::classify(&fact) {
                    self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
                }
                if !rec.controlled {
                    // A heavy touch: the ball runs on, slower, for whoever.
                    let v = self.spatial.ball_velocity();
                    self.spatial
                        .deflect_ball(self.spatial.ball_3d(), (v.0 / 2, v.1 / 2, 0));
                    self.spatial.deflected_by(id);
                    let retry = ctx.hidden.duel_balance().retry_ticks;
                    self.challenges.insert(id, tick + retry);
                    return;
                }
            } else {
                if self.deflects_the_pass(ctx, tick, (id, by)) {
                    return;
                }
                self.emit_event(
                    ctx,
                    tick,
                    Some(ActorId::Player(id)),
                    EventType::Interception,
                    serde_json::json!({ "from": by.slot(), "to": id.slot() }),
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
        let v = self.spatial.ball_velocity();
        self.spatial
            .deflect_ball(self.spatial.ball_3d(), (v.0 / 2, v.1 / 2, 0));
        self.spatial.deflected_by(id);
        let retry = ctx.hidden.duel_balance().retry_ticks;
        self.challenges.insert(id, tick + retry);
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

#[cfg(test)]
mod tests {
    use ds_match_api::MatchPlugin;

    #[test]
    fn the_plugin_has_its_card() {
        assert!(super::Technique.card().is_valid());
        assert_eq!(super::Technique.card().name, "technique");
    }
}
