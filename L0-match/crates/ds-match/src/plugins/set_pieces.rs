//! The set pieces plugin.
//!
//! The set-pieces plugin (ADR-063 §4; Laws 13-17, Law 10 for the
//! shoot-out): who takes a restart, where everyone stands, the kicks from
//! the mark. Moved here from `state.rs` (2026-11-15) as methods of the
//! match state they act on.

use ds_core::ids::PlayerId;
use ds_match_api::{MatchPlugin, PluginCard};

use crate::context::MatchContext;
use crate::perception::distance_squared;
use crate::state::{MatchState, Possession, nearest_in_squad, pitch_cm};

/// The set pieces.
#[derive(Debug, Clone, Copy, Default)]
pub struct SetPieces;

impl MatchPlugin for SetPieces {
    fn card(&self) -> PluginCard {
        crate::registry::card("set_pieces")
    }
}

impl MatchState {
    /// Applies a restart: the nearest player of the restarting side goes to
    /// the spot, the ball is placed at rest at his feet, and it is his.
    /// Returns the taker.
    pub(crate) fn apply_restart(
        &mut self,
        ctx: &mut MatchContext<'_>,
        restart: crate::rules::Restart,
    ) -> Option<PlayerId> {
        let taker = self.restart_taker(ctx, &restart)?;
        self.stoppage = true;
        self.pending = None;
        self.offside_at_pass.clear();
        self.restart_pass = Some(restart.kind);
        self.spatial.set_position(taker, restart.spot);
        // Out of play nobody has it, even if it went out under his foot.
        self.spatial.set_carrier(None);
        self.spatial
            .set_ball_in_flight((restart.spot.0, restart.spot.1, 0), (0, 0, 0));
        self.spatial.advance_ball();
        self.spatial.set_carrier(Some(taker));
        self.possession = match restart.side {
            crate::rules::Side::Home => Possession::Home,
            crate::rules::Side::Away => Possession::Away,
        };
        self.clear_the_spot(ctx, &restart, taker);
        if restart.kind == crate::rules::RestartKind::Penalty {
            // He kicks before anyone can come back into the area (Law 14:
            // they stay out until the ball is kicked).
            self.latencies.remove(&taker);
        }
        Some(taker)
    }

    /// Who takes a restart (plan L0-CLOSE step 7): the keeper takes his goal
    /// kick; a corner, a free kick or a penalty goes to the best kicker of
    /// that ball among the outfield players within `TAKER_RADIUS_CM`; the
    /// others to the nearest.
    pub(crate) fn restart_taker(
        &self,
        ctx: &mut MatchContext<'_>,
        restart: &crate::rules::Restart,
    ) -> Option<PlayerId> {
        const TAKER_RADIUS_CM: i64 = 3000;
        use crate::rules::RestartKind;
        let squad = match restart.side {
            crate::rules::Side::Home => &ctx.match_.home,
            crate::rules::Side::Away => &ctx.match_.away,
        };
        if restart.kind == RestartKind::Penalty {
            if let Some(t) = self.shootout_taker {
                return Some(t);
            }
        }
        let nearest = nearest_in_squad(&self.spatial, squad, restart.spot);
        let keeper = squad
            .players
            .first()
            .map(|p| p.id)
            .filter(|id| self.spatial.agent(*id).is_some());
        match restart.kind {
            RestartKind::GoalKick => keeper.or(nearest),
            RestartKind::Corner | RestartKind::FreeKick | RestartKind::Penalty => {
                let mut near: Vec<(i64, PlayerId)> = squad
                    .players
                    .iter()
                    .skip(1)
                    .filter_map(|p| {
                        self.spatial
                            .agent(p.id)
                            .map(|a| (distance_squared(a.position, restart.spot), p.id))
                    })
                    .filter(|(d, _)| *d <= TAKER_RADIUS_CM * TAKER_RADIUS_CM)
                    .collect();
                near.sort_by(|x, y| x.0.cmp(&y.0).then(x.1.slot().cmp(&y.1.slot())));
                let ids: Vec<PlayerId> = near.into_iter().map(|(_, id)| id).collect();
                ctx.hidden
                    .set_piece_taker(&ids, restart.kind == RestartKind::Penalty)
                    .or(nearest)
            }
            RestartKind::KickOff | RestartKind::ThrowIn => nearest,
        }
    }

    /// The distances of the Laws (Laws 13-14), applied to the players: at
    /// a free kick or a corner the opponents stand at least 9.15 m from the
    /// ball; at a penalty everyone but the taker and the defending keeper
    /// leaves the penalty area, and the keeper stands on his line between
    /// the posts.
    pub(crate) fn clear_the_spot(
        &mut self,
        ctx: &MatchContext<'_>,
        restart: &crate::rules::Restart,
        taker: PlayerId,
    ) {
        const DISTANCE_CM: i64 = 915;
        use crate::rules::RestartKind;
        let (length, width) = pitch_cm(ctx);
        let restarting_home = restart.side == crate::rules::Side::Home;
        let ids: Vec<(PlayerId, bool)> = ctx
            .match_
            .home
            .players
            .iter()
            .map(|p| (p.id, true))
            .chain(ctx.match_.away.players.iter().map(|p| (p.id, false)))
            .collect();
        match restart.kind {
            RestartKind::FreeKick | RestartKind::Corner => {
                for (id, home) in ids {
                    if home == restarting_home {
                        continue;
                    }
                    let Some(p) = self.spatial.agent(id).map(|a| a.position) else {
                        continue;
                    };
                    let d2 = distance_squared(p, restart.spot);
                    if d2 >= DISTANCE_CM * DISTANCE_CM {
                        continue;
                    }
                    // Back along the line from the ball, or towards his
                    // goal if he stands on it.
                    let (dx, dy) = (
                        i64::from(p.0 - restart.spot.0),
                        i64::from(p.1 - restart.spot.1),
                    );
                    let len = d2.isqrt();
                    let (ux, uy) = if len == 0 {
                        (if home { -1 } else { 1 }, 0)
                    } else {
                        (dx, dy)
                    };
                    let l = if len == 0 { 1 } else { len };
                    let to = (
                        i32::try_from(i64::from(restart.spot.0) + ux * DISTANCE_CM / l)
                            .unwrap_or(restart.spot.0)
                            .clamp(0, length),
                        i32::try_from(i64::from(restart.spot.1) + uy * DISTANCE_CM / l)
                            .unwrap_or(restart.spot.1)
                            .clamp(0, width),
                    );
                    self.spatial.set_position(id, to);
                }
            }
            RestartKind::Penalty => {
                // The goal the penalty is kicked at.
                let goal_x = if restarting_home { length } else { 0 };
                // Outside the area and 9.15 m behind the mark.
                let back = self
                    .laws
                    .penalty_area_depth_cm
                    .max(self.laws.penalty_spot_cm + 915)
                    + 100;
                let edge = if restarting_home { length - back } else { back };
                let defending_keeper = if restarting_home {
                    ctx.match_.away.players.first().map(|p| p.id)
                } else {
                    ctx.match_.home.players.first().map(|p| p.id)
                };
                for (id, _) in ids {
                    if id == taker {
                        continue;
                    }
                    if Some(id) == defending_keeper {
                        self.spatial.set_position(id, (goal_x, width / 2));
                        continue;
                    }
                    let Some(p) = self.spatial.agent(id).map(|a| a.position) else {
                        continue;
                    };
                    let inside = (p.0 - goal_x).abs() < (edge - goal_x).abs();
                    if inside {
                        self.spatial.set_position(id, (edge, p.1));
                    }
                }
            }
            RestartKind::KickOff | RestartKind::ThrowIn | RestartKind::GoalKick => {}
        }
    }

    /// The order in which `side` takes its kicks in the shoot-out (Law 10):
    /// the players on the pitch at the end, best taker first, by the same
    /// measure the team uses for a penalty in the match; `count` of them
    /// (the larger team leaves out its last, to equal the other).
    pub(crate) fn shootout_order(
        &self,
        ctx: &mut MatchContext<'_>,
        home: bool,
        count: usize,
    ) -> Vec<PlayerId> {
        let squad = if home {
            &ctx.match_.home
        } else {
            &ctx.match_.away
        };
        let mut left: Vec<PlayerId> = squad
            .players
            .iter()
            .map(|p| p.id)
            .filter(|id| self.spatial.agent(*id).is_some())
            .collect();
        let mut order = Vec::new();
        while order.len() < count {
            let Some(best) = ctx.hidden.set_piece_taker(&left, true) else {
                break;
            };
            left.retain(|id| *id != best);
            order.push(best);
        }
        order
    }

    /// How many players `home`'s side has on the pitch.
    pub(crate) fn on_pitch_count(&self, ctx: &MatchContext<'_>, home: bool) -> usize {
        let squad = if home {
            &ctx.match_.home
        } else {
            &ctx.match_.away
        };
        squad
            .players
            .iter()
            .filter(|p| self.spatial.agent(p.id).is_some())
            .count()
    }

    /// Sends `taker` to the mark for one kick of the shoot-out (Law 10).
    pub(crate) fn place_shootout_kick(
        &mut self,
        ctx: &mut MatchContext<'_>,
        home: bool,
        taker: PlayerId,
    ) {
        let (length, width) = pitch_cm(ctx);
        let (side, goal) = if home {
            (crate::rules::Side::Home, (length, width / 2))
        } else {
            (crate::rules::Side::Away, (0, width / 2))
        };
        let restart = crate::rules::foul_restart(side, goal, true, goal, self.laws.penalty_spot_cm);
        self.shootout_taker = Some(taker);
        let _ = self.apply_restart(ctx, restart);
        self.shootout_taker = None;
    }

    /// Whether the kick of `taker` is completed (Law 10): the ball has left
    /// his foot and stopped, gone out, or been played by someone else.
    pub(crate) fn shootout_kick_done(&self, taker: PlayerId, struck: bool) -> bool {
        struck
            && (self.spatial.carrier().is_some_and(|c| c != taker)
                || self.spatial.ball_state() == crate::spatial::BallState::Loose
                || self
                    .restart_pass
                    .is_some_and(|k| k != crate::rules::RestartKind::Penalty))
    }
}

#[cfg(test)]
mod tests {
    use ds_match_api::MatchPlugin;

    #[test]
    fn the_plugin_has_its_card() {
        assert!(super::SetPieces.card().is_valid());
        assert_eq!(super::SetPieces.card().name, "set_pieces");
    }
}
