//! The team's shape without instructions: the block (VISION §3).
//!
//! **One responsibility:** where an outfield player stands, given his
//! place on the coach's board (decision AL), where the ball is, and
//! whether his side has it.
//! Pure functions: data in, a point out; the movement takes the player
//! there with his own body (the movement formula). The coach's
//! instructions (L2) will change the numbers in `balance/block.json`;
//! they will never script the players.

use std::collections::BTreeMap;

use ds_core::ids::PlayerId;
use ds_model::Squad;

use crate::balance::BlockBalance;

/// A player's place in the block, in thousandths: `depth` 0 for the
/// defensive line and 1000 for the forward line; `width` 0 on the left
/// and 1000 on the right.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Role {
    /// Depth in the block, thousandths.
    pub depth: i32,
    /// Width in the block, thousandths.
    pub width: i32,
}

/// Every outfield player's slot in his squad's order.
///
/// Goalkeepers have no place in the block: they have their own target. The place itself is
/// read from the coach's board (decision AL), and a man who comes on takes
/// the slot of the man who comes off.
#[must_use]
pub fn slots(home: &Squad, away: &Squad) -> BTreeMap<PlayerId, usize> {
    let mut out = BTreeMap::new();
    for squad in [home, away] {
        for (i, player) in squad.players.iter().enumerate().skip(1) {
            out.insert(player.id, i);
        }
    }
    out
}

impl From<ds_model::Spot> for Role {
    fn from(s: ds_model::Spot) -> Self {
        Self {
            depth: i32::from(s.depth),
            width: i32::from(s.width),
        }
    }
}

/// The situation the block answers to.
#[derive(Debug, Clone, Copy)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "the facts of the moment the block reads"
)]
pub struct Situation {
    /// The ball, in cm.
    pub ball: (i32, i32),
    /// Whether the player's side has the ball.
    pub in_possession: bool,
    /// Whether the player's side is the home side (it attacks `x = length`).
    pub home: bool,
    /// The pitch, `(length, width)` in cm.
    pub pitch: (i32, i32),
    /// The kick-off: the whole block in its own half.
    pub kick_off: bool,
    /// The coach's intention for the player's side.
    pub intention: ds_model::Intention,
    /// His quick instructions and transition choices (ADR-059 step 3).
    pub orders: ds_model::TeamOrders,
    /// The seconds after the ball changed side (ADR-059 §6).
    pub transition: bool,
}

/// Where a player with `role` stands in his side's block.
///
/// The block follows the ball: the defensive line sits behind it, between
/// a minimum and a maximum distance from its own goal, and the block
/// shifts towards the ball's side; with the ball the side stretches,
/// without it it compacts. At the kick-off the forwards stand just behind
/// the halfway line.
#[must_use]
pub fn place(role: Role, s: Situation, blk: &BlockBalance) -> (i32, i32) {
    let (length, width) = s.pitch;
    let (depth, mut span, mut behind, mut line_max) = if s.in_possession {
        (
            blk.depth_in_possession_cm,
            blk.width_in_possession_cm.min(width),
            blk.line_behind_ball_in_possession_cm,
            blk.line_max_in_possession_cm,
        )
    } else {
        (
            blk.depth_out_of_possession_cm,
            blk.width_out_of_possession_cm.min(width),
            blk.line_behind_ball_out_of_possession_cm,
            blk.line_max_out_of_possession_cm,
        )
    };
    // The coach's intention modulates the block (OI-27): the line out of
    // possession, the width with the ball.
    if s.in_possession {
        let w = match s.intention.width {
            ds_model::Width::Wide => 1000 + blk.width_change_milli,
            ds_model::Width::Narrow => 1000 - blk.width_change_milli,
            ds_model::Width::Normal => 1000,
        };
        span = (span * w / 1000).min(width);
        // Everyone forward, and the transition after winning the ball
        // (ADR-059 step 3): the block higher, or held back to keep it.
        let mut push = s.orders.forward_cm;
        if s.transition {
            match s.orders.won {
                ds_model::OnWin::Counter => push = push.max(ds_model::TeamOrders::FORWARD_STEP_CM),
                ds_model::OnWin::Keep => push -= ds_model::TeamOrders::FORWARD_STEP_CM / 2,
                ds_model::OnWin::Shape => {}
            }
        }
        line_max += push;
        behind -= push / 2;
    } else {
        match s.intention.block {
            ds_model::BlockHeight::High => {
                line_max += blk.high_block_line_cm;
                behind -= blk.high_block_line_cm / 2;
            }
            ds_model::BlockHeight::Low => {
                line_max -= blk.low_block_line_cm;
                behind += blk.low_block_line_cm / 2;
            }
            ds_model::BlockHeight::Mid => {}
        }
        // More pressing: the block engages nearer the ball and higher.
        line_max += s.orders.press_cm / 2;
        behind -= s.orders.press_cm;
        // The transition after losing the ball (ADR-059 step 3): stay up on
        // the ball, or drop into a low block.
        if s.transition {
            match s.orders.lost {
                ds_model::OnLoss::CounterPress => {
                    line_max = line_max.max(blk.line_max_in_possession_cm);
                    behind = 0;
                }
                ds_model::OnLoss::DropBack => {
                    line_max -= blk.low_block_line_cm;
                    behind += blk.low_block_line_cm / 2;
                }
                ds_model::OnLoss::Shape => {}
            }
        }
    }
    behind = behind.max(0);
    // Distances along the side's own attacking direction (u = 0 at its own
    // goal line).
    let u_ball = if s.home { s.ball.0 } else { length - s.ball.0 };
    let line = if s.kick_off {
        (length / 2 - 100 - depth).max(blk.line_min_cm)
    } else {
        (u_ball - behind).clamp(blk.line_min_cm, line_max.max(blk.line_min_cm))
    };
    let mut u = (line + depth * role.depth / 1000).min(length - 300);
    if s.kick_off {
        u = u.min(length / 2 - 100);
    }
    let shift = (s.ball.1 - width / 2) * blk.ball_side_shift_milli / 1000;
    let cy = (width / 2 + shift).clamp(span / 2, width - span / 2);
    let y = (cy + span * (role.width - 500) / 1000).clamp(100, width - 100);
    (if s.home { u } else { length - u }, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PITCH: (i32, i32) = (10_500, 6_800);
    const FORWARD: Role = Role {
        depth: 1000,
        width: 500,
    };
    const DEFENDER: Role = Role {
        depth: 0,
        width: 500,
    };

    fn at(ball: (i32, i32), in_possession: bool) -> Situation {
        Situation {
            ball,
            in_possession,
            home: true,
            pitch: PITCH,
            kick_off: false,
            intention: ds_model::Intention::default(),
            orders: ds_model::TeamOrders::default(),
            transition: false,
        }
    }

    /// Twin test (OI-100, SIM 127): the same wide man, the same block,
    /// with the ball; only the ball's side changes. With the ball on his
    /// touchline he stands in the outer tenth of the pitch (a real side's
    /// nearest man to a touchline with the ball: median 4 m, Metrica
    /// sample game 1); with the ball across he stands well inside.
    #[test]
    fn the_wide_man_holds_the_touchline_on_the_ball_side() {
        let blk = BlockBalance::load();
        let wing = Role {
            depth: 500,
            width: 0,
        };
        let near = place(wing, at((6000, 200), true), &blk).1;
        let far = place(wing, at((6000, PITCH.1 - 200), true), &blk).1;
        assert!(near < PITCH.1 / 10, "near the ball {near}");
        assert!(far > near + 1000, "near {near} far {far}");
    }

    /// Twin test (OI-27): two teams identical but for the coach's
    /// intention; out of possession the high block's line stands higher
    /// than the mid block's, the low block's lower; with the ball the wide
    /// team is wider than the narrow one.
    #[test]
    fn the_intention_moves_the_block() {
        let blk = BlockBalance::load();
        let with = |block, width, in_possession| {
            let mut s = at((6000, 3400), in_possession);
            s.intention = ds_model::Intention { block, width };
            s
        };
        let line = |b| place(DEFENDER, with(b, ds_model::Width::Normal, false), &blk).0;
        let (high, mid, low) = (
            line(ds_model::BlockHeight::High),
            line(ds_model::BlockHeight::Mid),
            line(ds_model::BlockHeight::Low),
        );
        assert!(high > mid && mid > low, "high {high} mid {mid} low {low}");
        let wing = Role {
            depth: 500,
            width: 1000,
        };
        let y = |w| place(wing, with(ds_model::BlockHeight::Mid, w, true), &blk).1;
        assert!(y(ds_model::Width::Wide) > y(ds_model::Width::Narrow));
    }

    /// Twin tests (ADR-059 step 3): two teams identical but for one quick
    /// instruction or one transition choice, the same ball.
    #[test]
    fn the_quick_instructions_and_the_transitions_move_the_block() {
        use ds_model::{OnLoss, OnWin, Quick, TeamOrders};
        let blk = BlockBalance::load();
        let ball = (6_000, 3_400);
        let line = |orders: TeamOrders, in_possession: bool, transition: bool| {
            let s = Situation {
                orders,
                transition,
                ..at(ball, in_possession)
            };
            place(DEFENDER, s, &blk).0
        };
        let plain = TeamOrders::default();
        // More pressing: without the ball the line stands nearer the ball.
        let press = plain.with(Quick::MorePressing, true);
        assert!(line(press, false, false) > line(plain, false, false));
        // Everyone forward: with the ball the line is higher.
        let forward = plain.with(Quick::EveryoneForward, true);
        assert!(line(forward, true, false) > line(plain, true, false));
        // After losing the ball: counter-pressing stays up, dropping back
        // goes low; once the transition is over, both take the shape.
        let lost = |l| TeamOrders { lost: l, ..plain };
        let up = line(lost(OnLoss::CounterPress), false, true);
        let back = line(lost(OnLoss::DropBack), false, true);
        let shape = line(plain, false, true);
        assert!(
            up > shape && shape > back,
            "up {up} shape {shape} back {back}"
        );
        assert_eq!(
            line(lost(OnLoss::CounterPress), false, false),
            line(plain, false, false)
        );
        // After winning it: the counter pushes up, keeping it holds back.
        let won = |w| TeamOrders { won: w, ..plain };
        let counter = line(won(OnWin::Counter), true, true);
        let keep = line(won(OnWin::Keep), true, true);
        assert!(counter > line(plain, true, true) && line(plain, true, true) > keep);
    }

    #[test]
    fn the_block_follows_the_ball() {
        let b = BlockBalance::load();
        let deep = place(DEFENDER, at((2_000, 3_400), false), &b);
        let high = place(DEFENDER, at((8_000, 3_400), false), &b);
        assert!(
            high.0 > deep.0,
            "the line moves up with the ball: {deep:?} {high:?}"
        );
    }

    #[test]
    fn with_the_ball_the_side_stretches() {
        let b = BlockBalance::load();
        let spread = |p| {
            place(FORWARD, at((5_000, 3_400), p), &b).0
                - place(DEFENDER, at((5_000, 3_400), p), &b).0
        };
        assert!(spread(true) > spread(false));
    }

    #[test]
    fn at_kick_off_the_block_is_in_its_own_half() {
        let b = BlockBalance::load();
        let s = Situation {
            kick_off: true,
            ..at((5_250, 3_400), false)
        };
        assert!(place(FORWARD, s, &b).0 <= PITCH.0 / 2);
        let away = Situation { home: false, ..s };
        assert!(place(FORWARD, away, &b).0 >= PITCH.0 / 2);
    }
}
