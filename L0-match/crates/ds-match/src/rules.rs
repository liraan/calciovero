//! The restarts of play, in their minimal form (Laws 8, 15, 16, 17).
//!
//! **One responsibility:** given where the ball left the pitch and which
//! side played it last, say who restarts, from where, and how. Pure
//! functions: data in, a decision out; the engine applies it (it places
//! the taker and the ball). A volunteer can read and test a law here
//! without reading the engine.
//!
//! `Removed-by: OI-48` (the rules module: the rule as data, its
//! application as code, a versioned ruleset in the save). Until then the
//! laws are written as code, declared, and "played it last" is read as
//! "the side in possession", an approximation.

/// A side of the match.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    /// The home side (attacks towards `x = length`).
    Home,
    /// The away side (attacks towards `x = 0`).
    Away,
}

impl Side {
    /// The other side.
    #[must_use]
    pub const fn other(self) -> Self {
        match self {
            Self::Home => Self::Away,
            Self::Away => Self::Home,
        }
    }
}

/// How play restarts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartKind {
    /// From the centre spot (Law 8): the start of a half, or after a goal.
    KickOff,
    /// From the touchline (Law 15).
    ThrowIn,
    /// From the goal area (Law 16).
    GoalKick,
    /// From the corner arc (Law 17).
    Corner,
    /// A free kick, after an offence (Laws 11-13).
    FreeKick,
}

/// Who restarts, from where, and how.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Restart {
    /// The side that restarts.
    pub side: Side,
    /// Where the ball is placed, in cm.
    pub spot: (i32, i32),
    /// How.
    pub kind: RestartKind,
}

/// The kick-off from the centre spot by `side` (Law 8).
#[must_use]
pub const fn kick_off(side: Side, (length, width): (i32, i32)) -> Restart {
    Restart {
        side,
        spot: (length / 2, width / 2),
        kind: RestartKind::KickOff,
    }
}

/// The restart for a ball at `(x, y)`, or `None` if the ball is on the
/// pitch. `last` is the side that played it last.
///
/// Over a touchline: a throw-in for the other side, where the ball went
/// out. Over a goal line (not into the goal: the engine scores the goal
/// first): a corner if the defending side played it last, a goal kick if
/// the attackers did.
#[must_use]
pub fn out_of_play((x, y): (i32, i32), last: Side, (length, width): (i32, i32)) -> Option<Restart> {
    if (0..=length).contains(&x) && (0..=width).contains(&y) {
        return None;
    }
    if !(0..=width).contains(&y) {
        return Some(Restart {
            side: last.other(),
            spot: (x.clamp(100, length - 100), y.clamp(0, width)),
            kind: RestartKind::ThrowIn,
        });
    }
    let home_goal_line = x < 0;
    let defending = if home_goal_line {
        Side::Home
    } else {
        Side::Away
    };
    let line_x = if home_goal_line { 0 } else { length };
    Some(if last == defending {
        Restart {
            side: defending.other(),
            spot: (line_x, if y < width / 2 { 0 } else { width }),
            kind: RestartKind::Corner,
        }
    } else {
        Restart {
            side: defending,
            spot: (if home_goal_line { 550 } else { length - 550 }, width / 2),
            kind: RestartKind::GoalKick,
        }
    })
}

/// A free kick for `side` where the offence happened (Laws 11-13).
#[must_use]
pub const fn free_kick(side: Side, spot: (i32, i32)) -> Restart {
    Restart {
        side,
        spot,
        kind: RestartKind::FreeKick,
    }
}

/// Law 11: whether a player is in an offside position.
///
/// The player attacks towards `goal_x`; the ball is played from `ball_x`.
/// He is offside if he is in the opponents' half and nearer to their goal
/// line than both the ball and the second-last opponent (the keeper
/// counts). Level is onside.
#[must_use]
pub fn offside_position(
    player_x: i32,
    ball_x: i32,
    opponents_x: &[i32],
    goal_x: i32,
    length: i32,
) -> bool {
    // Distance from the goal line he attacks: smaller is nearer.
    let depth = |x: i32| (goal_x - x).abs();
    let mut opp: Vec<i32> = opponents_x.iter().map(|x| depth(*x)).collect();
    opp.sort_unstable();
    let Some(second_last) = opp.get(1).copied() else {
        return false;
    };
    let in_their_half = depth(player_x) < length / 2;
    in_their_half && depth(player_x) < depth(ball_x) && depth(player_x) < second_last
}

#[cfg(test)]
mod tests {
    use super::*;

    const PITCH: (i32, i32) = (10_500, 6_800);

    /// Law 11 (P1): beyond the second-last defender and the ball, in their
    /// half, he is offside; level, or behind the ball, or in his own half,
    /// he is not.
    #[test]
    fn offside_is_beyond_the_second_last_and_the_ball() {
        let defenders = [10_400, 9_000, 8_500];
        assert!(offside_position(9_200, 7_000, &defenders, 10_500, 10_500));
        assert!(
            !offside_position(9_000, 7_000, &defenders, 10_500, 10_500),
            "level"
        );
        assert!(
            !offside_position(9_200, 9_500, &defenders, 10_500, 10_500),
            "behind the ball"
        );
        assert!(
            !offside_position(5_000, 3_000, &[5_100, 4_000], 10_500, 10_500),
            "own half"
        );
        // The away side attacks the goal at x = 0.
        assert!(offside_position(
            1_300,
            3_500,
            &[100, 1_500, 2_000],
            0,
            10_500
        ));
    }

    #[test]
    fn a_ball_on_the_pitch_needs_no_restart() {
        assert_eq!(out_of_play((5000, 3000), Side::Home, PITCH), None);
        assert_eq!(
            out_of_play((0, 0), Side::Home, PITCH),
            None,
            "the lines are in"
        );
    }

    #[test]
    fn over_the_touchline_the_other_side_throws_in_where_it_went_out() {
        let r = out_of_play((4000, -30), Side::Home, PITCH).expect("out");
        assert_eq!(
            r,
            Restart {
                side: Side::Away,
                spot: (4000, 0),
                kind: RestartKind::ThrowIn
            }
        );
    }

    #[test]
    fn over_his_own_goal_line_the_defender_gives_a_corner() {
        let r = out_of_play((-20, 2000), Side::Home, PITCH).expect("out");
        assert_eq!(
            r,
            Restart {
                side: Side::Away,
                spot: (0, 0),
                kind: RestartKind::Corner
            }
        );
    }

    #[test]
    fn over_the_goal_line_the_attacker_gives_a_goal_kick() {
        let r = out_of_play((10_520, 5000), Side::Home, PITCH).expect("out");
        assert_eq!(
            r,
            Restart {
                side: Side::Away,
                spot: (9_950, 3_400),
                kind: RestartKind::GoalKick
            }
        );
    }

    #[test]
    fn the_kick_off_is_on_the_centre_spot() {
        assert_eq!(kick_off(Side::Away, PITCH).spot, (5_250, 3_400));
    }
}
