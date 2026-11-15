//! The keeper plugin (ADR-063 §4; ADR-047).
//!
//! The keeper plugin (ADR-063 §4; ADR-047): the shot that meets the keeper, held, parried or tipped.
//! Moved here from `state.rs` (2026-11-15) as methods of the match state
//! they act on.

use ds_match_api::{MatchPlugin, PluginCard};

use crate::state::prelude::*;

/// The keeper.
#[derive(Debug, Clone, Copy, Default)]
pub struct Keeper;

impl MatchPlugin for Keeper {
    fn card(&self) -> PluginCard {
        crate::registry::card("keeper")
    }
}

impl MatchState {
    /// The keeper meets a shot on target (ADR-051, the family "in goal"):
    /// `true` if he stopped it (held or parried; the event is emitted).
    pub(crate) fn keeper_meets_the_shot(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        gk: PlayerId,
        (from, struck): ((i32, i32, i32), u64),
        (plane_x, y_at, z_at): (i32, i32, i32),
        goal_center_y: i32,
    ) -> bool {
        // ADR-051, the family "in goal": the keeper reacts, dives in the
        // time the ball takes, and holds or parries what he reaches. He
        // meets the ball where it passes him, not on the goal line: off his
        // line he narrows the angle and has less time (the line from the
        // strike to the crossing, cut at his depth). The distance is from
        // the centre of his body, across and up.
        let share = ctx.hidden.save_balance().standing_reach_share;
        let reach_cm = self.bodies.get(&gk).map_or(0.0, |b| b.height_cm * share);
        let v = self.spatial.ball_velocity();
        let speed = dmath::hypot3(f64::from(v.0), f64::from(v.1), f64::from(v.2));
        #[allow(clippy::cast_precision_loss)] // ticks of one match: far below 2^52
        let flight_ticks = tick.saturating_sub(struck) as f64;
        let outcome = self.spatial.agent(gk).map(|gk_agent| {
            let (meet, share) = meeting_point(from, (plane_x, y_at, z_at), gk_agent.position.0);
            let body = ctx.hidden.save_balance().body_centre_cm;
            let across = f64::from(meet.1 - gk_agent.position.1);
            let up = f64::from(meet.2) - body;
            ctx.hidden.resolve_save(&ds_orchestrator::SaveContext {
                goalkeeper: gk,
                point: (plane_x, y_at, z_at),
                offset_cm: dmath::hypot(across, up),
                reach_cm,
                flight_ticks: flight_ticks * share,
                ball_speed_cm_per_tick: speed,
            })
        });

        let Some(o) = outcome.filter(|o| o.saved) else {
            return false;
        };
        {
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(gk)),
                EventType::Save,
                serde_json::json!({ "x": plane_x, "y": y_at, "z": z_at, "held": o.held }),
            );
            if o.held {
                // In his hands: the ball is his.
                self.spatial.set_carrier(Some(gk));
                self.in_hands = Some(gk);
            } else {
                // Parried: nearer him, pushed back into play away from the
                // centre of the goal; at the edge of his reach only tipped,
                // going on past the post (a corner if it crosses the line).
                let k = i32::from(
                    i16::try_from(ctx.hidden.save_balance().parry_rebound_milli).unwrap_or(0),
                );
                let away = if y_at >= goal_center_y { 1 } else { -1 };
                let rebound = |c: i16| i16::try_from(i32::from(c) * k / 1000).unwrap_or(0);
                if o.tipped {
                    // Tipped round the post (plan L0-CLOSE 11): he meets it
                    // where he stands, in front of his line, and turns it
                    // sideways as much as it takes to pass outside the post
                    // before it reaches the line, at most as fast as it came
                    // (he adds nothing to it). Too near his line, it still
                    // goes in. Before, it was turned on the line itself and
                    // always crossed it inside the posts.
                    let line = plane_x;
                    let depth = self
                        .spatial
                        .agent(gk)
                        .map_or(0, |a| (a.position.0 - line).abs())
                        .max(1);
                    let inward = if line == 0 { 1 } else { -1 };
                    let meet_x = line + inward * depth;
                    let post = goal_center_y + away * self.obstacle_balance.goal_half_width_cm;
                    #[allow(clippy::cast_possible_truncation)]
                    let r = ds_orchestrator::read::BALL_RADIUS_CM as i32;
                    let gap = (post + away * r - y_at).abs();
                    let vx = rebound(v.0);
                    let speed = i32::from(v.0).abs().max(i32::from(v.1).abs());
                    let vz = rebound(v.2).abs();
                    // As much as it takes: the first estimate is the straight
                    // line, then the path the ball will really follow (the
                    // same step that moves it, the air and the grass slowing
                    // it) decides. Before, the straight line alone was used,
                    // and the slowed ball still crossed inside the post.
                    let mut wide = (i32::from(vx).abs() * gap / depth).min(speed);
                    let clear =
                        goal_center_y + away * (self.obstacle_balance.goal_half_width_cm + r);
                    while wide < speed {
                        let side = i16::try_from(away * wide).unwrap_or(0);
                        let outside = self
                            .spatial
                            .ball_at_line((meet_x, y_at, z_at), (vx, side, vz), line)
                            .is_none_or(|(y, _)| away * (y - clear) >= 0);
                        if outside {
                            break;
                        }
                        wide += 1;
                    }
                    let side = i16::try_from(away * wide).unwrap_or(0);
                    self.spatial
                        .deflect_ball((meet_x, y_at, z_at), (vx, side, vz));
                } else {
                    let side = i16::try_from(away * i32::from(v.0).abs() * k / 1000).unwrap_or(0);
                    self.spatial.deflect_ball(
                        (plane_x, y_at, z_at),
                        (rebound(v.0).saturating_neg(), side, rebound(v.2).abs()),
                    );
                }
                self.spatial.deflected_by(gk);
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use ds_match_api::MatchPlugin;

    #[test]
    fn the_plugin_has_its_card() {
        assert!(super::Keeper.card().is_valid());
        assert_eq!(super::Keeper.card().name, "keeper");
    }
}
