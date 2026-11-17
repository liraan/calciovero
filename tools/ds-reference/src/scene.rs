//! What a match looks like, measured on its record.
//!
//! The maintainer's point 3, 17 November; `context/plans/BEHAVIOURS.md`,
//! part 3: the behaviours one sees watching it, as numbers, so that every
//! correction shows in them and can become a scene test.
//!
//! Only the record is read (ADR-065): positions, ball, carrier, ten times a
//! second. The geometry is the Laws' (Law 1: the penalty area 16.5 m deep,
//! 40.32 m wide); the home side attacks from the left.

use ds_core::ids::PlayerId;
use ds_match::{BallState, TickRecord};
use ds_stats::{EventType, MatchEvent};

/// The behaviours measured on one match.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct SceneCounts {
    /// Steps of a player longer than the fastest legs carry him, in ticks
    /// of the same period (the teleports).
    pub jumps: u32,
    /// Balls on the grass slower than 5 m/s within 80 cm of a player that
    /// someone has at the next tick, and all such balls.
    pub loose_taken: (u32, u32),
    /// With the ball in a keeper's hands or at his feet: how many of his
    /// teammates stand in his penalty area, summed, and the ticks.
    pub mates_in_box: (u32, u32),
    /// Ticks with a carrier, and of them those with the ball in the
    /// carrier's own half.
    pub own_half: (u32, u32),
    /// Completed passes (the ball from one man to a teammate), forward,
    /// lateral, back: within 45 degrees of his attack, of the touchlines,
    /// of his own goal.
    pub passes: (u32, u32, u32),
    /// With the ball, the two most advanced men of the side that has it:
    /// how far ahead of the ball they stand along the attack, cm, summed,
    /// and the ticks.
    pub forwards_ahead: (f64, u32),
    /// Ticks with a carrier who runs (2 m/s), and of them those with the
    /// ball more than half a metre behind where he looks.
    pub ball_behind: (u32, u32),
}

/// Real football beside it: the direction of the accurate passes.
///
/// Serie A 2017-18, Wyscout soccer-logs (Pappalardo and Massucco 2019,
/// figshare doi:10.6084/m9.figshare.c.4415000, CC BY 4.0), measured on
/// 2026-11-17. The accurate passes (280 884), by the same 45-degree
/// sectors on the pitch in metres (105 x 68): forward 31.1%, lateral
/// 52.7%, back 16.2%.
pub const REAL_PASSES: (f64, f64, f64) = (31.1, 52.7, 16.2);
/// The share of passes struck in the passer's own half, Serie A 2017-18
/// (the same source, 337 317 passes): near, not the same, as the time the
/// ball spends in the half of the side that has it.
pub const REAL_PASSES_FROM_OWN_HALF: f64 = 52.9;

/// The penalty area, Law 1, cm: depth and half width.
const BOX: (i32, i32) = (1650, 2016);
/// The fastest legs of `balance/movement.json`, cm a tick, and the whole
/// centimetres either end of a step.
const FASTEST_CM: f64 = 102.0;

const fn home(id: PlayerId) -> bool {
    id.slot() < 100
}

const fn keeper(id: PlayerId) -> bool {
    id.slot() % 100 == 0
}

fn hypot(x: f64, y: f64) -> f64 {
    ds_core::dmath::hypot(x, y)
}

/// The behaviours of one match from its record and its events; `length`
/// and `width` the pitch, cm.
#[must_use]
#[allow(clippy::cast_precision_loss, clippy::too_many_lines)] // cm and ticks of a match
pub fn measure(
    record: &[TickRecord],
    events: &[MatchEvent],
    (length, width): (i32, i32),
) -> SceneCounts {
    let mut c = SceneCounts::default();
    let mut last_carrier: Option<(PlayerId, (i32, i32))> = None;
    // Out of play, from a restart told (or a goal) until someone has the
    // ball again: the ball waits for its taker, it is no free ball.
    let mut dead = false;
    for pair in record.windows(2) {
        let (before, now) = (&pair[0], &pair[1]);
        if events[before.events.clone()].iter().any(|e| {
            matches!(
                e.event_type,
                EventType::Goal
                    | EventType::ThrowIn
                    | EventType::Corner
                    | EventType::GoalKick
                    | EventType::FreeKick
                    | EventType::Penalty
            )
        }) {
            dead = true;
        }
        if before.carrier.is_some() {
            dead = false;
        }
        let same_period = before.half == now.half;
        // The teleports.
        if same_period {
            for a in &now.agents {
                if let Some(b) = before.agents.iter().find(|x| x.id == a.id) {
                    let d = hypot(
                        f64::from(a.position.0 - b.position.0),
                        f64::from(a.position.1 - b.position.1),
                    );
                    if d > FASTEST_CM {
                        c.jumps += 1;
                    }
                }
            }
        }
        // The free ball nobody claims.
        let ball = before.ball.position;
        let speed = hypot(
            f64::from(now.ball.position.0 - ball.0),
            f64::from(now.ball.position.1 - ball.1),
        );
        if before.carrier.is_none()
            && !dead
            && ball.2 == 0
            && before.ball.state != BallState::Possessed
            && speed < 50.0
            && before.agents.iter().any(|a| {
                hypot(
                    f64::from(a.position.0 - ball.0),
                    f64::from(a.position.1 - ball.1),
                ) < 80.0
            })
        {
            c.loose_taken.1 += 1;
            if now.carrier.is_some() {
                c.loose_taken.0 += 1;
            }
        }
        let Some(carrier) = now.carrier else {
            continue;
        };
        let Some(me) = now.agents.iter().find(|a| a.id == carrier) else {
            continue;
        };
        let mine = home(carrier);
        let forward = if mine { 1 } else { -1 };
        let at = now.ball.position;
        // With the keeper on it: his teammates in his area.
        if keeper(carrier) {
            let goal_x = if mine { 0 } else { length };
            let n = now
                .agents
                .iter()
                .filter(|a| a.id != carrier && home(a.id) == mine)
                .filter(|a| {
                    (a.position.0 - goal_x).abs() <= BOX.0
                        && (a.position.1 - width / 2).abs() <= BOX.1
                })
                .count();
            c.mates_in_box.0 += u32::try_from(n).unwrap_or(0);
            c.mates_in_box.1 += 1;
        }
        // The half the ball is in.
        c.own_half.1 += 1;
        if (at.0 - length / 2) * forward < 0 {
            c.own_half.0 += 1;
        }
        // The two most advanced of his side, ahead of the ball.
        let mut ahead: Vec<i32> = now
            .agents
            .iter()
            .filter(|a| home(a.id) == mine && !keeper(a.id))
            .map(|a| (a.position.0 - at.0) * forward)
            .collect();
        ahead.sort_unstable_by(|a, b| b.cmp(a));
        if ahead.len() >= 2 {
            c.forwards_ahead.0 += f64::from(ahead[0] + ahead[1]) / 2.0;
            c.forwards_ahead.1 += 1;
        }
        // The ball behind a man running with it.
        if let Some(b) = before.agents.iter().find(|x| x.id == carrier) {
            let (vx, vy) = (
                f64::from(me.position.0 - b.position.0),
                f64::from(me.position.1 - b.position.1),
            );
            if before.carrier == Some(carrier) && hypot(vx, vy) >= 20.0 {
                let h = f64::from(me.heading).to_radians();
                #[allow(
                    clippy::disallowed_methods,
                    reason = "a measure of the record: it never feeds the simulation"
                )]
                let (sin, cos) = h.sin_cos();
                let along = f64::from(at.0 - me.position.0)
                    .mul_add(sin, f64::from(at.1 - me.position.1) * cos);
                c.ball_behind.1 += 1;
                if along < -50.0 {
                    c.ball_behind.0 += 1;
                }
            }
        }
        // A completed pass: the ball from one man to a teammate.
        if let Some((from, from_at)) = last_carrier {
            if from != carrier && home(from) == mine && before.carrier != Some(carrier) {
                let (dx, dy) = (
                    f64::from((me.position.0 - from_at.0) * forward),
                    f64::from(me.position.1 - from_at.1),
                );
                if dx >= dy.abs() {
                    c.passes.0 += 1;
                } else if -dx >= dy.abs() {
                    c.passes.2 += 1;
                } else {
                    c.passes.1 += 1;
                }
            }
        }
        last_carrier = Some((carrier, (at.0, at.1)));
    }
    c
}

/// Sums the counts of many matches.
#[must_use]
pub fn sum(all: &[SceneCounts]) -> SceneCounts {
    all.iter().fold(SceneCounts::default(), |mut t, c| {
        t.jumps += c.jumps;
        t.loose_taken = (
            t.loose_taken.0 + c.loose_taken.0,
            t.loose_taken.1 + c.loose_taken.1,
        );
        t.mates_in_box = (
            t.mates_in_box.0 + c.mates_in_box.0,
            t.mates_in_box.1 + c.mates_in_box.1,
        );
        t.own_half = (t.own_half.0 + c.own_half.0, t.own_half.1 + c.own_half.1);
        t.passes = (
            t.passes.0 + c.passes.0,
            t.passes.1 + c.passes.1,
            t.passes.2 + c.passes.2,
        );
        t.forwards_ahead = (
            t.forwards_ahead.0 + c.forwards_ahead.0,
            t.forwards_ahead.1 + c.forwards_ahead.1,
        );
        t.ball_behind = (
            t.ball_behind.0 + c.ball_behind.0,
            t.ball_behind.1 + c.ball_behind.1,
        );
        t
    })
}

/// The counts as the lines the tool prints.
#[must_use]
pub fn report(c: &SceneCounts, matches: usize) -> String {
    let pct = |a: u32, b: u32| 100.0 * f64::from(a) / f64::from(b.max(1));
    let per = |a: u32| f64::from(a) / f64::from(u32::try_from(matches.max(1)).unwrap_or(1));
    let passes = c.passes.0 + c.passes.1 + c.passes.2;
    format!(
        "jumps longer than the fastest legs   {:>8.1} a match\n\
         free slow ball taken at once         {:>7.1}%  ({} of {})\n\
         mates in the area, keeper on the ball {:>7.2}\n\
         with the ball, ball in own half      {:>7.1}%   (real: passes from own half {:.1}%)\n\
         passes forward / lateral / back      {:>5.0}% {:>3.0}% {:>3.0}%   (real {:.0}% {:.0}% {:.0}%)\n\
         two most advanced ahead of the ball  {:>7.1} m\n\
         ball behind a man running with it    {:>7.1}%\n",
        per(c.jumps),
        pct(c.loose_taken.0, c.loose_taken.1),
        c.loose_taken.0,
        c.loose_taken.1,
        f64::from(c.mates_in_box.0) / f64::from(c.mates_in_box.1.max(1)),
        pct(c.own_half.0, c.own_half.1),
        REAL_PASSES_FROM_OWN_HALF,
        pct(c.passes.0, passes),
        pct(c.passes.1, passes),
        pct(c.passes.2, passes),
        REAL_PASSES.0,
        REAL_PASSES.1,
        REAL_PASSES.2,
        c.forwards_ahead.0 / f64::from(c.forwards_ahead.1.max(1)) / 100.0,
        pct(c.ball_behind.0, c.ball_behind.1),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The measures of one match are within their ranges, and a match has
    /// passes and possession to measure.
    #[test]
    fn one_match_is_measured() {
        let m = ds_fixtures::match_fixture(43).expect("fixture");
        let (result, record) =
            ds_match::simulate_from_seed_recorded(&m, 43, ds_match::MatchConfig::default());
        let c = measure(&record, result.events.events(), (10_500, 6_800));
        assert!(c.own_half.1 > 10_000, "{c:?}");
        assert!(c.passes.0 + c.passes.1 + c.passes.2 > 300, "{c:?}");
        assert!(c.loose_taken.0 <= c.loose_taken.1);
        assert!(c.ball_behind.0 <= c.ball_behind.1);
    }
}
