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
        self.keeper_meets_the_ball(
            ctx,
            tick,
            gk,
            (from, struck),
            (plane_x, y_at, z_at),
            goal_center_y,
            true,
        )
    }

    /// Law 12: a ball deliberately kicked to the keeper by a team-mate is
    /// not his to take with his hands. The fact is read from the pass that
    /// is still pending; the keeper still chooses whether to use them.
    pub(crate) fn is_back_pass_to(&self, gk: PlayerId) -> bool {
        self.pending.as_ref().is_some_and(|p| {
            p.shot.is_none()
                && !p.restart
                && !p.throw_in
                && p.receiver == Some(gk)
                && p.passer.is_some_and(|by| {
                    crate::perception::team_of(by) == crate::perception::team_of(gk)
                })
        })
    }

    /// Law 12: the offence is where the keeper handled it; inside his own
    /// goal area the indirect free kick is taken from the goal area line at
    /// the nearest point. The side that did not handle it restarts, and the
    /// restart is indirect until another player touches the ball (DD-102).
    pub(crate) fn call_back_pass_offence(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        gk: PlayerId,
        at: (i32, i32),
    ) -> bool {
        let (length, width) = crate::state::pitch_cm(ctx);
        let law1 = ds_calculator::laws::law1();
        let home = crate::perception::team_of(gk) == 0;
        let own_line = if home { 0 } else { length };
        let out = if home {
            law1.goal_area_depth_cm
        } else {
            -law1.goal_area_depth_cm
        };
        let spot = if (at.0 - own_line).abs() <= law1.goal_area_depth_cm
            && (at.1 - width / 2).abs() <= law1.goal_area_half_width_cm
        {
            (own_line + out, at.1)
        } else {
            at
        };
        let side = if home {
            crate::rules::Side::Away
        } else {
            crate::rules::Side::Home
        };
        let _ = self.call_restart(
            ctx,
            tick,
            crate::rules::indirect_free_kick(side, spot),
            None,
        );
        self.emit_event(
            ctx,
            tick,
            Some(ActorId::Player(gk)),
            EventType::FreeKick,
            serde_json::json!({
                "reason": "back_pass_handled", "indirect": true, "x": spot.0, "y": spot.1
            }),
        );
        true
    }

    /// The keeper meets any ball going into his goal (SIM 203, T3; Law 12:
    /// in his area he may use his hands on any ball): a shot, or a back
    /// pass, a deflection, a clearance gone wrong, with the time it has been
    /// on its way from `from` at `struck`. Before, only a shot met him:
    /// every other ball rolled past him into the net. A ball that was not a
    /// shot is told as taken (`claim`), not as a save of a shot.
    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub(crate) fn keeper_meets_the_ball(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        gk: PlayerId,
        (from, struck): ((i32, i32, i32), u64),
        (plane_x, y_at, z_at): (i32, i32, i32),
        goal_center_y: i32,
        shot: bool,
    ) -> bool {
        // ADR-051, the family "in goal": the keeper reacts, dives in the
        // time the ball takes, and holds or parries what he reaches. He
        // meets the ball where it passes him, not on the goal line: off his
        // line he narrows the angle and has less time (the line from the
        // strike to the crossing, cut at his depth). The distance is from
        // the centre of his body, across and up.
        let save = ctx.hidden.save_balance();
        let reach_cm = self
            .bodies
            .get(&gk)
            .map_or(0.0, |b| save.standing_reach_cm(b.height_cm));
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
        let at = self
            .spatial
            .agent(gk)
            .map_or((plane_x, y_at), |a| a.position);
        // Law 12: handling a deliberate back-pass is an offence, not a save
        // (DD-102). Called before the save event so the record tells one act.
        if self.is_back_pass_to(gk) && self.call_back_pass_offence(ctx, tick, gk, at) {
            return true;
        }
        {
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(gk)),
                EventType::Save,
                if shot {
                    serde_json::json!({ "x": plane_x, "y": y_at, "z": z_at, "held": o.held })
                } else {
                    serde_json::json!({
                        "x": plane_x, "y": y_at, "z": z_at, "held": o.held, "claim": true
                    })
                },
            );
            if o.held {
                // In his hands: the ball is his, where he holds it, in front
                // of his line (SIM 203, T5: before, the ball stayed where the
                // tick had carried it, past the line, and in the same tick the
                // referee gave a corner for every shot he held).
                self.spatial.deflect_ball((at.0, at.1, 0), (0, 0, 0));
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
    use super::*;
    use crate::context::MatchConfig;
    use ds_match_api::MatchPlugin;
    use ds_orchestrator::GroundTruth;
    use ds_stats::EventChain;

    #[test]
    fn the_plugin_has_its_card() {
        assert!(super::Keeper.card().is_valid());
        assert_eq!(super::Keeper.card().name, "keeper");
    }

    /// DD?-92(A): one shot, one keeper; only where he stands across the goal
    /// changes. The sweep is a diagnostic grid (A5-bis: no threshold is
    /// chosen from it). It must cross one geometric boundary: closer never
    /// turns a save into a goal, and farther never turns a goal into a save.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    #[test]
    fn moving_only_the_keeper_across_gives_one_save_boundary() {
        let m = ds_fixtures::match_fixture(42).expect("fixture");
        let centre = i32::from(m.pitch.width_m) * 100 / 2;
        let plane_x = i32::from(m.pitch.length_m) * 100;
        let half = crate::balance::LawsBalance::load().law1.goal_half_width_cm;
        let fraction = ds_orchestrator::ShotChoiceBalance::load().aim_edge_fraction;
        let edge = (f64::from(half) * fraction).round() as i32;
        let crossing_y = centre + edge;
        let z = ds_calculator::save::SaveBalance::load().body_centre_cm as i32;
        assert!(edge + (ds_orchestrator::read::BALL_RADIUS_CM as i32) < half);

        let saved_at = |keeper_y: i32| {
            let mut state = MatchState::new(&m);
            let gk = m.away.players[0].id;
            state.spatial.set_carrier(None);
            state.spatial.set_position(gk, (plane_x, keeper_y));
            state
                .spatial
                .set_ball_in_flight((plane_x - 100, crossing_y, z), (250, 0, 0));
            let mut hidden = GroundTruth::new(42, m.id);
            let mut chain = EventChain::new();
            let mut ctx = MatchContext {
                match_: &m,
                hidden: &mut hidden,
                chain: &mut chain,
                config: MatchConfig::default(),
            };
            state.keeper_meets_the_shot(
                &mut ctx,
                6,
                gk,
                ((plane_x - 1500, centre, 0), 0),
                (plane_x, crossing_y, z),
                centre,
            )
        };

        let mut previous = None;
        let mut changes = Vec::new();
        for offset in -half..=half {
            let saved = saved_at(centre + offset);
            if previous.is_some_and(|p| p != saved) {
                changes.push(offset);
            }
            previous = Some(saved);
        }
        assert_eq!(changes.len(), 1, "one lateral boundary: {changes:?}");

        let boundary = centre + changes[0];
        let nearer = f64::from((crossing_y - boundary).abs());
        let farther = f64::from((crossing_y - (boundary - 1)).abs());
        assert!(
            (farther - nearer - 1.0).abs() < 1e-9,
            "one centimetre of keeper movement changed the offset by {:.3} cm",
            farther - nearer
        );
    }

    /// Law 12 twin (DD-102): the same ball, the same keeper, the same touch.
    /// Only the intended receiver changes — his own keeper, or a teammate
    /// standing on the same spot. Handling a deliberate back pass gives an
    /// indirect free kick; the identical claim of a teammate's pass does not.
    #[allow(clippy::cast_possible_truncation)]
    #[test]
    fn handling_a_back_pass_is_punished_but_the_same_claim_from_a_teammate_is_not() {
        let m = ds_fixtures::match_fixture(42).expect("fixture");
        let length = i32::from(m.pitch.length_m) * 100;
        let centre = i32::from(m.pitch.width_m) * 100 / 2;
        let gk = m.away.players[0].id;
        let passer = m.away.players[3].id;
        let mate = m.away.players[6].id;
        let z = ds_calculator::save::SaveBalance::load().body_centre_cm as i32;
        let run = |to_keeper: bool| {
            let mut hidden = GroundTruth::new(42, m.id);
            let mut chain = EventChain::new();
            let mut ctx = MatchContext {
                match_: &m,
                hidden: &mut hidden,
                chain: &mut chain,
                config: MatchConfig::default(),
            };
            let mut s = MatchState::new(&m);
            s.spatial.set_position(gk, (length, centre));
            s.spatial.set_position(mate, (length, centre));
            s.spatial.set_position(passer, (length - 800, centre));
            s.spatial
                .set_ball_in_flight((length - 100, centre, z), (20, 0, 0));
            s.pending = Some(Pending {
                launched: 0,
                shot: None,
                interceptor: None,
                passer: Some(passer),
                receiver: Some(if to_keeper { gk } else { mate }),
                closest_cm: None,
                through: false,
                cross: false,
                foot: None,
                restart: false,
                throw_in: false,
                indirect: false,
            });
            let saved = s.keeper_meets_the_ball(
                &mut ctx,
                1,
                gk,
                ((length - 100, centre, z), 0),
                (length, centre, z),
                centre,
                false,
            );
            (saved, s.restart_pass, s.in_hands)
        };
        let (saved, restart, hands) = run(true);
        assert!(saved, "the keeper reaches the ball either way");
        assert_eq!(
            restart,
            Some(crate::rules::RestartKind::IndirectFreeKick),
            "the back pass restarts indirect, not {restart:?}"
        );
        assert!(hands.is_none(), "the ball is not his after the offence");
        let (saved, restart, hands) = run(false);
        assert!(saved, "the keeper reaches the ball either way");
        assert_eq!(restart, None, "a teammate's pass is not an offence");
        assert_eq!(hands, Some(gk), "the keeper keeps the identical claim");
    }

    /// Law 12: an indirect free kick for a handling inside the offender's
    /// own goal area is taken from its line, at the nearest point. Without
    /// this the restart would be placed inside the six-yard box.
    #[test]
    fn a_back_pass_handled_in_the_goal_area_is_brought_to_its_line() {
        let m = ds_fixtures::match_fixture(42).expect("fixture");
        let length = i32::from(m.pitch.length_m) * 100;
        let centre = i32::from(m.pitch.width_m) * 100 / 2;
        let law1 = ds_calculator::laws::law1();
        let handled = (length - law1.goal_area_depth_cm / 2, centre + 100);
        let mut hidden = GroundTruth::new(42, m.id);
        let mut chain = EventChain::new();
        let mut ctx = MatchContext {
            match_: &m,
            hidden: &mut hidden,
            chain: &mut chain,
            config: MatchConfig::default(),
        };
        let mut s = MatchState::new(&m);
        s.call_back_pass_offence(&mut ctx, 1, m.away.players[0].id, handled);
        assert_eq!(
            s.waiting.as_ref().map(|w| w.spot),
            Some((length - law1.goal_area_depth_cm, handled.1)),
            "the spot is on the goal area line, not {handled:?}"
        );
    }
}
