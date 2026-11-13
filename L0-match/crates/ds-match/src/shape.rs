//! The team's shape without instructions: the block (VISION §3).
//!
//! **One responsibility:** where an outfield player stands, given his
//! role in the formation, where the ball is, and whether his side has it.
//! Pure functions: data in, a point out; the movement takes the player
//! there with his own body (the movement formula). The coach's
//! instructions (L2) will change the numbers in `balance/block.json`;
//! they will never script the players.

use std::collections::BTreeMap;

use ds_core::ids::PlayerId;
use ds_model::Squad;

use crate::balance::BlockBalance;

/// The 4-4-2 grid, as `(x percent, y percent)` per slot.
///
/// Slot order: GK, LB, CB, CB, RB, LM, CM, CM, RM, ST, ST. It matches
/// `Squad.players` **by index**. The formation parser is deferred (D3): a
/// non-4-4-2 formation falls back to this grid.
const GRID_4_4_2: [(i32, i32); 11] = [
    (5, 50),
    (25, 15),
    (25, 38),
    (25, 62),
    (25, 85),
    (50, 15),
    (50, 38),
    (50, 62),
    (50, 85),
    (75, 38),
    (75, 62),
];

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

/// Every outfield player's role, read from the formation (goalkeepers
/// have no place in the block: they have their own target).
#[must_use]
pub fn roles(home: &Squad, away: &Squad) -> BTreeMap<PlayerId, Role> {
    let mut out = BTreeMap::new();
    for squad in [home, away] {
        for (i, player) in squad.players.iter().enumerate().skip(1) {
            let (px, py) = GRID_4_4_2[i.min(10)];
            out.insert(
                player.id,
                Role {
                    depth: ((px - 25) * 1000 / 50).clamp(0, 1000),
                    width: py * 10,
                },
            );
        }
    }
    out
}

/// The situation the block answers to.
#[derive(Debug, Clone, Copy)]
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
        behind = behind.max(0);
    }
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
        }
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
