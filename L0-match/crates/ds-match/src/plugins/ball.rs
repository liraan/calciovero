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
    #[must_use]
    pub(crate) const fn obstacle_hit(
        y_at: i32,
        z_at: i32,
        center_y: i32,
        half_w: i32,
        height: i32,
        post_r: i32,
        bar_t: i32,
    ) -> Option<u8> {
        let lateral_edge = (y_at - center_y).abs() - half_w;
        let vertical_edge = z_at - height;
        let hit_post = lateral_edge.abs() <= post_r && z_at <= height;
        let hit_bar = vertical_edge.abs() <= bar_t && (y_at - center_y).abs() < half_w;
        if hit_post {
            Some(0)
        } else if hit_bar {
            Some(2)
        } else {
            None
        }
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
        let hit = Self::obstacle_hit(
            impact.1,
            impact.2,
            center_y,
            b.goal_half_width_cm,
            b.goal_height_cm,
            b.post_radius_cm,
            b.crossbar_thickness_cm,
        )?;
        Some(if hit == 0 {
            b.post_reflection().reflect(velocity)
        } else {
            b.crossbar_reflections()
                .iter()
                .fold(velocity, |v, r| r.reflect(v))
        })
    }

    /// Checks whether the in-flight ball crossed a goal plane this tick
    /// (F.3-b3). The goal is born from the geometric crossing, not from
    /// the die's outcome: `on_target` is the die's intent, the crossing
    /// is what happens.
    ///
    /// If a shot is pending and the ball crossed a goal plane:
    /// - outside the goal mouth: possession to the defending team
    ///   (declared derogation MD-38, ball out stops the match);
    /// - inside, goalkeeper reaches (`reach_cm = height_cm / 2`): save via
    ///   `resolve_save`, possession to the defender;
    /// - inside, no save: `Goal` emitted, score updated.
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
        #[allow(clippy::cast_possible_truncation)]
        let r = ds_orchestrator::read::BALL_RADIUS_CM as i32;
        let (plane_x, defending) = if prev.0 <= pitch_length && curr.0 > pitch_length + r {
            (pitch_length, Possession::Away)
        } else if prev.0 >= 0 && curr.0 < -r {
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
        let goal_half_w = self.obstacle_balance.goal_half_width_cm;
        let goal_h = self.obstacle_balance.goal_height_cm;
        let goal_center_y = width / 2;
        let inside = (y_at - goal_center_y).abs() < goal_half_w && (0..goal_h).contains(&z_at);

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
            self.spatial.deflect_ball(impact, v);
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
        #[allow(clippy::cast_possible_truncation)]
        let r = ds_orchestrator::read::BALL_RADIUS_CM as i32;
        let (plane_x, defending) = if prev.0 <= length && curr.0 > length + r {
            (length, Possession::Away)
        } else if prev.0 >= 0 && curr.0 < -r {
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
        let half = self.obstacle_balance.goal_half_width_cm;
        let inside = (y_at - width / 2).abs() < half
            && (0..self.obstacle_balance.goal_height_cm).contains(&z_at);
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
            self.spatial.deflect_ball(impact, v);
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
    use ds_match_api::MatchPlugin;

    #[test]
    fn the_plugin_has_its_card() {
        assert!(super::Ball.card().is_valid());
        assert_eq!(super::Ball.card().name, "ball");
    }
}
