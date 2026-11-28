//! The ball plugin (ADR-063 §4; ADR-048).
//!
//! The ball plugin (ADR-063 §4; ADR-048): the ball against the goal frame and across the lines.
//! Moved here from `state.rs` (2026-11-15) as methods of the match state
//! they act on.

use ds_match_api::{MatchPlugin, PluginCard};

use crate::state::prelude::*;

/// The ball.
#[derive(Debug, Clone, Copy, Default)]
pub struct Ball;

impl MatchPlugin for Ball {
    fn card(&self) -> PluginCard {
        crate::registry::card("ball")
    }
}

impl MatchState {
    /// Which obstacle the ball meets at the goal plane, if any (ADR-048 §1).
    ///
    /// Pure geometry, no state: `Some(0)` a post, `Some(2)` the crossbar,
    /// `None` nothing (the ball goes in or out, the caller decides).
    ///
    /// Law 1: 7.32 m between the inner edges of the posts, 2.44 m to the
    /// lower edge of the crossbar, posts and crossbar of the same width. So a
    /// post's axis is a post radius outside `half_w`, the bar's a radius
    /// above `height`, and the ball meets either when its own radius
    /// reaches it. `z_at` is the height of the ball's lowest point (the
    /// physics bounces it at 0), its centre a radius higher. Before SIM 202
    /// the post stood on the inner edge and the ball was a point: a ball
    /// whose centre passed 10 cm inside the post, its side over it, was a
    /// goal (code audit, third phase, seed 48).
    #[must_use]
    pub(crate) const fn obstacle_hit(
        y_at: i32,
        z_at: i32,
        center_y: i32,
        b: &crate::balance::ObstacleBalance,
    ) -> Option<u8> {
        let r = ds_calculator::laws::BALL_RADIUS;
        let lateral = (y_at - center_y).abs();
        let centre_z = z_at + r;
        let post_axis = b.goal_half_width_cm + b.post_radius_cm;
        let bar_axis = b.goal_height_cm + b.post_radius_cm;
        let reach = b.post_radius_cm + r;
        let hit_post = (lateral - post_axis).abs() <= reach && centre_z <= bar_axis;
        let hit_bar = (centre_z - bar_axis).abs() <= reach && lateral <= post_axis;
        if hit_post {
            Some(0)
        } else if hit_bar {
            Some(2)
        } else {
            None
        }
    }

    /// Law 10: the whole ball between the posts and under the crossbar, at
    /// the goal plane (`z_at` its lowest point, as in `obstacle_hit`). The
    /// one reading of the goal mouth for a shot and for every other ball.
    #[must_use]
    pub(crate) const fn in_the_goal_mouth(
        y_at: i32,
        z_at: i32,
        center_y: i32,
        b: &crate::balance::ObstacleBalance,
    ) -> bool {
        ds_calculator::laws::in_the_goal_mouth(
            y_at - center_y,
            z_at,
            b.goal_half_width_cm,
            b.goal_height_cm,
        )
    }

    /// The ball's velocity after it meets the woodwork at `impact` on the
    /// goal plane, or `None` if it meets nothing (ADR-048 §1). Pure: the
    /// geometry is `obstacle_hit`, the reflection is the calculator's.
    ///
    /// The post reverses the x component (the ball comes back toward the
    /// pitch); the crossbar reverses x and z (it comes back, and down or up
    /// off the bar).
    pub(crate) fn goal_plane_deflection(
        impact: (i32, i32, i32),
        center_y: i32,
        velocity: (i16, i16, i16),
        b: &crate::balance::ObstacleBalance,
    ) -> Option<(i16, i16, i16)> {
        let hit = Self::obstacle_hit(impact.1, impact.2, center_y, b)?;
        Some(if hit == 0 {
            b.post_reflection().reflect(velocity)
        } else {
            b.crossbar_reflections()
                .iter()
                .fold(velocity, |v, r| r.reflect(v))
        })
    }

    /// Checks whether the in-flight ball crossed a goal plane this tick
    /// (F.3-b3). The goal is born from the geometric crossing, the whole
    /// ball over the line (Law 10): `on_target` is where the strike was
    /// meant to go, the crossing is what happens.
    ///
    /// If a shot is pending and the ball crossed a goal plane:
    /// - outside the goal mouth: nothing here; the ball is out, and the
    ///   restart is the Laws' (`check_goal_line`);
    /// - inside, the keeper on the pitch meets it with his reach
    ///   (`save.json`, `standing_reach_share` of his height plus the dive):
    ///   save via `resolve_save`;
    /// - inside, no save: `Goal` emitted, score updated.
    ///
    /// (2026-11-23, code audit F: the doc said the possession went to the
    /// defenders on a wide shot and the reach was half the height.)
    ///
    /// The pending is cleared in all three cases. Pure geometry plus one
    /// `resolve_save`; no other RNG.
    pub(crate) fn check_shot_crossing(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        prev: (i32, i32, i32),
        curr: (i32, i32, i32),
    ) {
        // A restart waiting: the ball is out of play (SIM 150).
        if self.waiting.is_some() {
            return;
        }
        let (attacker, struck, from) = match self.pending.as_ref().and_then(|p| p.shot.as_ref()) {
            Some(s) => (s.attacker, s.struck, s.from),
            None => return,
        };
        let pitch_length = i32::from(ctx.match_.pitch.length_m) * 100;
        let width = i32::from(ctx.match_.pitch.width_m) * 100;

        // The whole ball over the line (Law 10), as for every other ball
        // (`check_goal_line`): beyond it by its radius (SIM 196). Before, a
        // shot crossed with its centre on the line, any other ball whole.
        let r = ds_calculator::laws::BALL_RADIUS;
        let (plane_x, defending) = if prev.0 <= pitch_length + r && curr.0 > pitch_length + r {
            (pitch_length, Possession::Away)
        } else if prev.0 >= -r && curr.0 < -r {
            (0, Possession::Home)
        } else {
            return;
        };
        let dx = i64::from(curr.0 - prev.0);
        if dx == 0 {
            return;
        }
        let t_num = i64::from(plane_x - prev.0);
        let y_at = prev.1 + i32::try_from((i64::from(curr.1 - prev.1) * t_num) / dx).unwrap_or(0);
        let z_at = prev.2 + i32::try_from((i64::from(curr.2 - prev.2) * t_num) / dx).unwrap_or(0);
        let goal_center_y = width / 2;
        let inside = Self::in_the_goal_mouth(y_at, z_at, goal_center_y, &self.obstacle_balance);

        // ADR-048 §1: the ball meets the post or the crossbar. If it does,
        // the ball is deflected and keeps flying from the impact point, no
        // goal, no save: the shot is over, and the ball off the woodwork is
        // anyone's (ADR-058: no clock ends it any more; before, a shot back
        // into play stayed a shot nobody might go for, and the match froze).
        // The geometry and the reflection are `goal_plane_deflection`, pure
        // and testable.
        //
        // The woodwork is told (L0.05 1.2, the maintainer, 2026-11-13).
        let impact = (plane_x, y_at, z_at);
        if let Some(v) = Self::goal_plane_deflection(
            impact,
            goal_center_y,
            self.spatial.ball_velocity(),
            &self.obstacle_balance,
        ) {
            self.spatial.deflect_off_woodwork(impact, v);
            self.pending = None;
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(attacker)),
                EventType::Woodwork,
                serde_json::json!({ "x": plane_x, "y": y_at, "z": z_at }),
            );
            return;
        }

        self.pending = None;
        self.spatial.set_carrier(None);

        if !inside {
            // Wide or over: the shooter touched it last, so it is a goal
            // kick when it leaves the pitch (Law 16). Before `SIM_VERSION`
            // 55 the possession went to the defenders here, and the rule
            // read their touch: a corner for every wide shot.
            return;
        }

        // The keeper on the pitch now (SIM 196): the first of the live
        // squad, the man who came on for him after a substitution. Before,
        // the ids of the starting keepers: after a change of keeper nobody
        // met a shot and every one on target was a goal.
        let keeper_of = match defending {
            Possession::Home => &ctx.match_.home,
            Possession::Away => &ctx.match_.away,
        };
        let Some(gk) = keeper_of.players.first().map(|p| p.id) else {
            return;
        };
        if self.keeper_meets_the_shot(
            ctx,
            tick,
            gk,
            (from, struck),
            (plane_x, y_at, z_at),
            goal_center_y,
        ) {
            self.possession = defending;
            return;
        }

        let fact = crate::fact::Fact::GoalLineCrossing {
            actor: Some(ActorId::Player(attacker)),
            point: (plane_x, y_at, z_at),
        };
        for c in crate::fact::classify(&fact) {
            self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
        }
        match defending {
            Possession::Home => self.score.away += 1,
            Possession::Away => self.score.home += 1,
        }
        self.possession = defending;
        // Law 8: after a goal, the side that conceded kicks off. Before, the
        // ball stayed in the net for the rest of the match (OI-97).
        let _ = self.call_restart(
            ctx,
            tick,
            crate::rules::kick_off(side_of(defending), pitch_cm(ctx)),
            Some(crate::plugins::set_pieces::GoalParty {
                side_home: defending == Possession::Away,
                point: (plane_x, y_at),
            }),
        );
    }

    /// Law 10 for every ball, not only a shot (plan L0-CLOSE 11, OI-98): a
    /// ball that crosses a goal line between the posts and under the bar is
    /// a goal for the side attacking that goal, whoever played it last; the
    /// woodwork turns it back as for a shot. Played last by the defending
    /// side, it is an own goal. A shot on its way is `check_shot_crossing`'s.
    #[allow(clippy::too_many_lines)]
    pub(crate) fn check_goal_line(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        prev: (i32, i32, i32),
        curr: (i32, i32, i32),
    ) {
        // A restart waiting: the ball is out of play (SIM 150).
        if self.waiting.is_some() {
            return;
        }
        if self.pending.as_ref().is_some_and(|p| p.shot.is_some()) {
            return;
        }
        let (length, width) = pitch_cm(ctx);
        // The whole ball over the line (Law 10): beyond it by its radius.
        // Seen from the tick before whenever the ball was not yet wholly
        // over, its centre past the line or not (SIM 203, T4: before, a ball
        // whose centre ended a tick within a radius past the line was never
        // a goal, and the next tick gave a goal kick or a corner).
        let r = ds_calculator::laws::BALL_RADIUS;
        let (plane_x, defending) = if prev.0 <= length + r && curr.0 > length + r {
            (length, Possession::Away)
        } else if prev.0 >= -r && curr.0 < -r {
            (0, Possession::Home)
        } else {
            return;
        };
        let dx = i64::from(curr.0 - prev.0);
        if dx == 0 {
            return;
        }
        let t_num = i64::from(plane_x - prev.0);
        let y_at = prev.1 + i32::try_from((i64::from(curr.1 - prev.1) * t_num) / dx).unwrap_or(0);
        let z_at = prev.2 + i32::try_from((i64::from(curr.2 - prev.2) * t_num) / dx).unwrap_or(0);
        let inside = Self::in_the_goal_mouth(y_at, z_at, width / 2, &self.obstacle_balance);
        // Who scored (Law 10 gives the goal to the side; the scorer is who
        // put it there): the man on it; else the last who only touched it on
        // its way, unless it was a shot (a parried or blocked shot is the
        // shooter's); else the last who played it. A defender's own play, or
        // his touch on a ball that was not a shot, is an own goal.
        let touches = self.spatial.touches();
        let last = self.spatial.carrier().or(match touches.deflected {
            Some(d) if !touches.shot => Some(d),
            _ => touches.played,
        });
        let impact = (plane_x, y_at, z_at);
        if let Some(v) = Self::goal_plane_deflection(
            impact,
            width / 2,
            self.spatial.ball_velocity(),
            &self.obstacle_balance,
        ) {
            self.spatial.set_carrier(None);
            self.spatial.deflect_off_woodwork(impact, v);
            self.emit_event(
                ctx,
                tick,
                last.map(ActorId::Player),
                EventType::Woodwork,
                serde_json::json!({ "x": plane_x, "y": y_at, "z": z_at }),
            );
            return;
        }
        if !inside {
            return;
        }
        // Law 15: a throw-in scores no goal until another player touches
        // it. The throw that goes straight in is out of play, and the last
        // touch (the thrower's) says whether it is a goal kick or a corner.
        if self.pending.as_ref().is_some_and(|p| p.throw_in)
            && self.spatial.touches().deflected.is_none()
        {
            if let Some(restart) =
                crate::rules::out_of_play((curr.0, y_at), side_of(self.possession), (length, width))
            {
                self.pending = None;
                self.spatial.set_carrier(None);
                let _ = self.call_restart(ctx, tick, restart, None);
            }
            return;
        }
        // His keeper meets it first, as he meets a shot, unless he is the
        // man on it (SIM 203, T3).
        let keeper_of = match defending {
            Possession::Home => &ctx.match_.home,
            Possession::Away => &ctx.match_.away,
        };
        if let Some(gk) = keeper_of.players.first().map(|p| p.id) {
            let course = self.ball_course;
            if self.spatial.carrier() != Some(gk)
                && self.spatial.agent(gk).is_some()
                && self.keeper_meets_the_ball(
                    ctx,
                    tick,
                    gk,
                    (course.from, course.since),
                    impact,
                    width / 2,
                    false,
                )
            {
                self.pending = None;
                self.possession = defending;
                return;
            }
        }
        let home = |p: PlayerId| ctx.match_.home.players.iter().any(|q| q.id == p);
        let own_goal = crate::rules::own_goal(last.map(&home), defending == Possession::Home);
        self.pending = None;
        self.spatial.set_carrier(None);
        let fact = crate::fact::Fact::GoalLineCrossing {
            actor: last.map(ActorId::Player),
            point: impact,
        };
        for mut c in crate::fact::classify(&fact) {
            if own_goal {
                if let Some(obj) = c.fields.as_object_mut() {
                    obj.insert("own_goal".to_string(), serde_json::json!(true));
                }
            }
            self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
        }
        match defending {
            Possession::Home => self.score.away += 1,
            Possession::Away => self.score.home += 1,
        }
        self.possession = defending;
        let _ = self.call_restart(
            ctx,
            tick,
            crate::rules::kick_off(side_of(defending), pitch_cm(ctx)),
            Some(crate::plugins::set_pieces::GoalParty {
                side_home: defending == Possession::Away,
                point: (impact.0, impact.1),
            }),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ds_match_api::MatchPlugin;
    use ds_orchestrator::GroundTruth;
    use ds_stats::EventChain;

    /// Law 15 twin: the same throw crosses the same goal line; only the
    /// fact that it was thrown in changes. A pass would score, a throw
    /// goes out for a goal kick.
    #[test]
    fn a_direct_throw_into_the_goal_is_not_a_goal() {
        let m = ds_fixtures::match_fixture(42).expect("fixture");
        let run = |throw_in: bool| {
            let mut hidden = GroundTruth::new(42, m.id);
            let mut chain = EventChain::new();
            let mut ctx = MatchContext {
                match_: &m,
                hidden: &mut hidden,
                chain: &mut chain,
                config: crate::context::MatchConfig::default(),
            };
            let mut s = MatchState::new(&m);
            let taker = m.home.players[3].id;
            let mate = m.home.players[6].id;
            let keeper = m.away.players[0].id;
            s.spatial.set_position(taker, (10000, 3400));
            s.spatial.set_position(mate, (10200, 3400));
            s.spatial.set_position(keeper, (10450, 100));
            s.spatial.played_by(taker);
            s.spatial.set_ball_in_flight((10400, 3400, 0), (200, 0, 0));
            s.ball_course = BallCourse {
                from: (10400, 3400, 0),
                since: 100,
                touches: (None, None, None),
            };
            s.possession = Possession::Home;
            s.pending = Some(Pending {
                launched: 100,
                shot: None,
                interceptor: None,
                passer: Some(taker),
                receiver: Some(mate),
                closest_cm: None,
                through: false,
                cross: false,
                foot: None,
                restart: true,
                throw_in,
            });
            s.check_goal_line(&mut ctx, 100, (10400, 3400, 0), (10600, 3400, 0));
            (s.score, s.restart_pass)
        };
        let (scored, after_pass) = run(false);
        assert_eq!(scored.home, 1, "a pass into the goal scores");
        let (not_scored, after_throw) = run(true);
        assert_eq!(not_scored.home, 0, "a throw cannot score directly");
        assert_eq!(not_scored.away, 0, "no own goal either");
        assert_eq!(
            after_throw,
            Some(crate::rules::RestartKind::GoalKick),
            "the throw goes out for a goal kick, not {after_throw:?}"
        );
        assert_eq!(
            after_pass,
            Some(crate::rules::RestartKind::KickOff),
            "the goal restarts with a kick-off, not {after_pass:?}"
        );
    }

    #[test]
    fn the_plugin_has_its_card() {
        assert!(super::Ball.card().is_valid());
        assert_eq!(super::Ball.card().name, "ball");
    }
}
