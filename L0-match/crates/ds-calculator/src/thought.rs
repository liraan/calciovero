//! The player's thought (ADR-052): what he forecasts, and how he judges it.
//!
//! **One responsibility:** two pure functions the choice is built on.
//! *Who reaches a ball first*, along its path, from the players the actor
//! perceives and the speeds he believes they have; and *the chance a
//! position gives*, derived from the engine's own shot and save, not from
//! a table. No die (ADR-035), no weight per kind of option.

use ds_core::dmath;
use serde::Deserialize;

use crate::kick::KickBalance;
use crate::save::SaveBalance;

const THOUGHT_JSON: &str = include_str!("../../../balance/thought.json");
/// The ball's physics: the thought reads the same gravity as the engine.
const BALL_JSON: &str = include_str!("../../../balance/ball.json");

/// The thought's declared values (`balance/thought.json`).
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct ThoughtBalance {
    /// The goal's width, cm (Law 1).
    pub goal_width_cm: i32,
    /// What keeping the ball is worth, against a goal (1.0).
    pub possession_value: f64,
    /// The forecast samples a ball's path every this many cm.
    pub forecast_step_cm: i32,
    /// The speed he believes the others run at, cm/tick.
    pub believed_speed_cm_per_tick: f64,
    /// The time he believes the others take to move, ticks.
    pub believed_reaction_ticks: f64,
    /// A forecast with this many ticks to spare is sure.
    pub trust_ticks: f64,
    /// The horizon of the least anticipating player, cm.
    pub horizon_min_cm: f64,
    /// The horizon of the most anticipating player, cm.
    pub horizon_max_cm: f64,
    /// How high he believes a keeper reaches with his hands and a jump, cm:
    /// computed, not declared (a reference keeper's height, his arms and an
    /// average jump, `save.json` and `duel.json`), set by the caller.
    #[serde(skip)]
    pub believed_keeper_reach_cm: f64,
    /// Gravity, cm/tick² (`balance/ball.json`, the engine's own).
    #[serde(skip)]
    pub gravity_cm_per_tick2: f64,
}

impl ThoughtBalance {
    /// How sure he is of a forecast that leaves `margin` ticks to spare
    /// (negative: the other side gets there first), in `[0, 1]`.
    #[must_use]
    pub fn confidence(&self, margin: f64) -> f64 {
        (0.5 + margin / (2.0 * self.trust_ticks.max(1.0))).clamp(0.0, 1.0)
    }

    /// How sure a player is to win a race to a space (ADR-052 §5): his run
    /// at his own pace (cm/tick), which he knows, against the defender's at
    /// the pace he believes.
    #[must_use]
    pub fn race(&self, my_cm: f64, my_pace: f64, their_cm: f64) -> f64 {
        let mine = my_cm / my_pace.max(1.0);
        let theirs =
            self.believed_reaction_ticks + their_cm / self.believed_speed_cm_per_tick.max(1.0);
        self.confidence(theirs - mine)
    }

    /// What a space is worth to the side attacking `goal`, in units of
    /// keeping the ball: keeping it there, more the nearer to the goal
    /// (`length` the pitch), plus the chance quality of the space.
    #[must_use]
    pub fn space_value(&self, distance_to_goal_cm: f64, length_cm: f64, chance: f64) -> f64 {
        let near = (1.0 - distance_to_goal_cm / length_cm.max(1.0)).clamp(0.0, 1.0);
        1.0 + near + chance / self.possession_value.max(1e-6)
    }

    /// An option's worth: sure by `confidence`, it reaches a space worth
    /// `gain`; otherwise the opponents have a space worth `loss` to them,
    /// weighed by his risk management (a bold player fears it less).
    #[must_use]
    pub fn expected(confidence: f64, gain: f64, loss: f64, risk_appetite: f64) -> f64 {
        ((1.0 - confidence) * loss)
            .mul_add(-(1.5 - risk_appetite.clamp(0.0, 1.0)), confidence * gain)
    }
}

impl ThoughtBalance {
    /// Loads the embedded values.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid; `embedded_thought_is_valid`
    /// catches it at test time.
    #[must_use]
    pub fn load() -> Self {
        #[derive(Deserialize)]
        struct Ball {
            gravity_cm_per_tick2: f64,
        }
        let mut t: Self =
            serde_json::from_str(THOUGHT_JSON).expect("embedded thought file is valid JSON");
        let ball: Ball = serde_json::from_str(BALL_JSON).expect("embedded ball file is valid JSON");
        t.gravity_cm_per_tick2 = ball.gravity_cm_per_tick2;
        t
    }
}

/// A player as the actor imagines him: where he is, how fast he believes
/// he runs (cm/tick), how long before he moves (ticks), and his side.
#[derive(Debug, Clone, Copy)]
pub struct Imagined {
    /// Where he is, cm.
    pub at: (i32, i32),
    /// The speed the actor believes he has, cm/tick.
    pub speed: f64,
    /// How long before he moves, ticks.
    pub reaction: f64,
    /// Whether he is on the actor's side.
    pub mate: bool,
}

/// Who reaches a ball first, and where.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Forecast {
    /// Index of the player in the list, or `None` if nobody reaches it
    /// before it stops.
    pub first: Option<usize>,
    /// Where he takes it, cm.
    pub at: (i32, i32),
    /// The ticks he has to spare there against the nearest player of the
    /// other side (positive: he gets there first by that much).
    pub margin: f64,
}

/// Ticks for a ground ball leaving at `v0` (cm/tick) to cover `d` cm with
/// a friction that keeps `keep` of its speed per tick; `None` if it stops
/// before.
#[must_use]
pub fn ball_ticks(d: f64, v0: f64, keep: f64) -> Option<f64> {
    if d <= 0.0 {
        return Some(0.0);
    }
    if v0 <= 0.0 {
        return None;
    }
    if keep >= 1.0 {
        return Some(d / v0);
    }
    let reach = v0 / (1.0 - keep);
    if d >= reach {
        return None;
    }
    Some(dmath::log(1.0 - d / reach, keep))
}

/// Who reaches the ball first along its path (ADR-052 §2).
///
/// The ball leaves `from` towards `to` at `v0` and keeps `keep` of its
/// speed per tick (`keep` = 1 for a ball in the air until it lands at
/// `to`); each imagined player runs straight at a point of the path. The
/// first player to be at a point before the ball takes it there.
#[must_use]
pub fn who_first(
    from: (i32, i32),
    to: (i32, i32),
    (v0, keep): (f64, f64),
    players: &[Imagined],
    step_cm: i32,
) -> Forecast {
    let (dx, dy) = (f64::from(to.0 - from.0), f64::from(to.1 - from.1));
    let len = dmath::hypot(dx, dy);
    let step = f64::from(step_cm.max(1));
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let samples = (len / step).ceil() as usize;
    let mut last = (from, None);
    for k in 0..=samples {
        #[allow(clippy::cast_precision_loss)]
        let d = (k as f64 * step).min(len);
        let Some(tb) = ball_ticks(d, v0, keep) else {
            break;
        };
        let t = if len > 0.0 { d / len } else { 0.0 };
        #[allow(clippy::cast_possible_truncation)]
        let point = (
            from.0 + (dx * t).round() as i32,
            from.1 + (dy * t).round() as i32,
        );
        let arrival = |p: &Imagined| {
            let run = dmath::hypot(f64::from(p.at.0 - point.0), f64::from(p.at.1 - point.1));
            p.reaction + run / p.speed.max(1.0)
        };
        let mut best: Option<(usize, f64)> = None;
        for (i, p) in players.iter().enumerate() {
            let a = arrival(p);
            if a <= tb && best.is_none_or(|(_, b)| a < b) {
                best = Some((i, a));
            }
        }
        if let Some((i, a)) = best {
            let mate = players[i].mate;
            let other = players
                .iter()
                .filter(|p| p.mate != mate)
                .map(arrival)
                .fold(f64::INFINITY, f64::min);
            return Forecast {
                first: Some(i),
                at: point,
                margin: other - a,
            };
        }
        last = (point, None);
    }
    Forecast {
        first: last.1,
        at: last.0,
        margin: 0.0,
    }
}

/// The chance a position gives (ADR-052 §3), in `[0, 1]`.
///
/// Derived from the engine's own shot and save, for an average shooter and
/// keeper: the width of the goal the keeper cannot reach in the time the
/// ball takes, against the spread of the shot's error, scaled by how much
/// of the goal he sees.
#[must_use]
pub fn chance(
    (distance_cm, keeper_off_cm, keeper_shift_cm): (f64, f64, f64),
    goal_cone: f64,
    kick: &KickBalance,
    save: &SaveBalance,
    t: &ThoughtBalance,
) -> f64 {
    // The keeper meets the shot `keeper_off_cm` off his line: there the
    // goal looks narrower and the ball has had less time.
    let meet = (distance_cm - keeper_off_cm.clamp(0.0, distance_cm)).max(1.0);
    let share = meet / distance_cm.max(1.0);
    let speed = kick.speed(0.85, 0.5).max(1.0);
    let flight = meet / speed;
    let standing = save.reference_keeper_height_cm * save.standing_reach_share;
    let reach = save.reach_cm(standing, 0.5, 0.5, flight);
    // `keeper_shift_cm`: how far the keeper leans towards the post aimed at
    // (negative: away from it), at his depth.
    let half = f64::from(t.goal_width_cm) / 2.0 * share;
    let open = (half - keeper_shift_cm - reach).max(0.0);
    let spread = meet * kick.error_rad(0.5, 0.75, 0.85, 0.0);
    let hit = if open + spread > 0.0 {
        open / (open + spread)
    } else {
        0.0
    };
    (hit * goal_cone.clamp(0.0, 1.0)).clamp(0.0, 1.0)
}

/// The chance of a lob over a keeper off his line (ADR-052), in `[0, 1]`.
///
/// It goes in if he cannot get back before it comes down (his run back,
/// at the speed the shooter believes, against the lob's slow flight) and
/// if it falls inside the goal (the half width against the spread of a
/// soft kick).
#[must_use]
pub fn lob_chance(
    (distance_cm, keeper_off_cm): (f64, f64),
    power: f64,
    kick: &KickBalance,
    t: &ThoughtBalance,
) -> f64 {
    let v = kick.speed(power, 0.5).max(1.0);
    let flight = distance_cm / v;
    // The arc the engine gives it (ADR-052: the same physics): it reaches
    // the height he aims at on the line, lifted against gravity for the
    // whole flight. Over the keeper it must pass above his reach, or he
    // takes it over his head (2026-11-13: before, the thought ignored it
    // and four shots in five were lobs the keepers caught).
    let to_keeper = (distance_cm - keeper_off_cm).max(0.0);
    let at_keeper = to_keeper / v;
    let height = f64::from(kick.aim_high_cm).mul_add(
        to_keeper / distance_cm.max(1.0),
        t.gravity_cm_per_tick2 / 2.0 * at_keeper * (flight - at_keeper),
    );
    let over = (0.5 + (height - t.believed_keeper_reach_cm) / 100.0).clamp(0.0, 1.0);
    let back = t.believed_reaction_ticks + keeper_off_cm / t.believed_speed_cm_per_tick.max(1.0);
    let beaten = over * t.confidence(back - flight);
    let half = f64::from(t.goal_width_cm) / 2.0;
    let spread = distance_cm * kick.error_rad(0.5, 0.75, power, 0.0);
    beaten * half / (half + spread)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A keeper far off his line can be lobbed; one on his line cannot
    /// (P1, ADR-052).
    #[test]
    fn a_keeper_off_his_line_can_be_lobbed() {
        let k = KickBalance::load();
        let mut t = ThoughtBalance::load();
        t.believed_keeper_reach_cm = 287.0;
        let far = lob_chance((2000.0, 1200.0), 0.2, &k, &t);
        let home = lob_chance((2000.0, 100.0), 0.2, &k, &t);
        assert!(far > home + 0.1 && home < 0.05, "far {far}, home {home}");
    }

    /// The keeper leaning to the post aimed at closes it (P1).
    #[test]
    fn the_far_post_from_the_keeper_is_better() {
        let (k, s, t) = (
            KickBalance::load(),
            SaveBalance::load(),
            ThoughtBalance::load(),
        );
        let open = chance((1600.0, 300.0, -80.0), 0.44, &k, &s, &t);
        let closed = chance((1600.0, 300.0, 80.0), 0.44, &k, &s, &t);
        assert!(open > closed, "open {open}, closed {closed}");
    }

    fn me(at: (i32, i32), mate: bool) -> Imagined {
        Imagined {
            at,
            speed: 60.0,
            reaction: 2.0,
            mate,
        }
    }

    #[test]
    fn embedded_thought_is_valid() {
        let t = ThoughtBalance::load();
        assert_eq!(t.goal_width_cm, 732);
        assert!(t.possession_value > 0.0 && t.possession_value < 1.0);
    }

    /// A ground ball slows down and stops: far enough, it never arrives.
    #[test]
    fn a_ball_on_the_grass_stops() {
        assert!(ball_ticks(1000.0, 150.0, 0.957).is_some());
        assert!(ball_ticks(10_000.0, 150.0, 0.957).is_none());
    }

    /// A free teammate on the line takes the pass; an opponent standing on
    /// the line in between cuts it out (P1, ADR-052 §2).
    #[test]
    fn the_nearest_to_the_path_takes_it() {
        let mate = me((3000, 0), true);
        let free = who_first((0, 0), (3000, 0), (150.0, 0.957), &[mate], 100);
        assert_eq!(free.first, Some(0));
        let cut = who_first(
            (0, 0),
            (3000, 0),
            (150.0, 0.957),
            &[mate, me((1500, 100), false)],
            100,
        );
        assert_eq!(cut.first, Some(1));
    }

    /// Twin test (ADR-051 §3): the same opponent, imagined faster by a
    /// reader who knows him, reaches a pass the other imagined lost.
    #[test]
    fn a_faster_opponent_reaches_more() {
        let mate = me((3000, 0), true);
        let slow = Imagined {
            speed: 20.0,
            ..me((1500, 900), false)
        };
        let fast = Imagined {
            speed: 90.0,
            ..slow
        };
        let a = who_first((0, 0), (3000, 0), (150.0, 0.957), &[mate, slow], 100);
        let b = who_first((0, 0), (3000, 0), (150.0, 0.957), &[mate, fast], 100);
        assert_eq!(a.first, Some(0));
        assert_eq!(b.first, Some(1));
    }

    /// Twin test (ADR-051 §3): two forwards identical but for pace, in the
    /// same race for the space behind the line: the fast one is sure to
    /// win it, the slow one is not.
    #[test]
    fn the_fast_twin_wins_the_race() {
        let t = ThoughtBalance::load();
        let fast = t.race(1500.0, 85.0, 1500.0);
        let slow = t.race(1500.0, 40.0, 1500.0);
        assert!(fast > 0.7 && slow < 0.3, "fast {fast}, slow {slow}");
    }

    /// The chance a position gives falls with the distance and with the
    /// angle, and is derived, not tabled (P1, ADR-052 §3).
    #[test]
    fn closer_and_central_is_a_better_chance() {
        let (k, s, t) = (
            KickBalance::load(),
            SaveBalance::load(),
            ThoughtBalance::load(),
        );
        let spot = chance((1100.0, 0.0, 0.0), 0.6, &k, &s, &t);
        let far = chance((2500.0, 0.0, 0.0), 0.28, &k, &s, &t);
        let tight = chance((1100.0, 0.0, 0.0), 0.15, &k, &s, &t);
        // A keeper off his line narrows the goal (P1).
        assert!(chance((1100.0, 400.0, 0.0), 0.6, &k, &s, &t) < spot);
        assert!(
            spot > far && spot > tight,
            "spot {spot}, far {far}, tight {tight}"
        );
        assert!(far >= 0.0);
    }
}
