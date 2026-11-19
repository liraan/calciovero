//! The view's interpolation layer (`tools/ds-gui/DESIGN.md`, "Lo strato
//! di interpolazione"): the match is recorded at 10 ticks a second, the
//! screen draws at its own rate, and every frame is rebuilt for its exact
//! instant between two ticks.
//!
//! **One responsibility:** from the tick records and an instant (a
//! fractional tick), where everyone and the ball are. Drawing only, in
//! `f32`: nothing here goes back to the simulation, and wherever the
//! drawing and the next tick disagree, the tick wins (every curve ends
//! exactly on it).
//!
//! - The players on Hermite curves, their velocity taken from the ticks
//!   either side; their facing turned the short way round.
//! - The ball, in the air, on the parabola of the gravity the match was
//!   played with (the record's header, ADR-065) between the two ticks; at someone's feet or on the
//!   grass, straight.
//! - A contact (a shot, a pass, a header) happens on a tick, and the ball's
//!   path is never smoothed across a tick: it breaks there.
//! - The view draws one tick behind the last one computed, so that it
//!   always has the tick before and the tick after (the caller's delay).

use ds_core::ids::PlayerId;
use ds_match::{BallState, RecordPhysics, TickRecord};

/// One player at an instant.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerAt {
    /// Who.
    pub id: PlayerId,
    /// Where, cm.
    pub at: (f32, f32),
    /// Which way he faces, degrees, `0` towards positive `y`, clockwise.
    pub heading: f32,
    /// His speed between the two ticks, cm a tick.
    pub speed: f32,
    /// How tired he looks at the tick before, `[0, 1]`.
    pub fatigue: f32,
}

/// Everyone and the ball at an instant.
#[derive(Debug, Clone, PartialEq)]
pub struct Moment {
    /// The players.
    pub players: Vec<PlayerAt>,
    /// The ball, cm, `(x, y, z)`.
    pub ball: (f32, f32, f32),
    /// The ball's state at the tick before.
    pub ball_state: BallState,
    /// Who has the ball at the tick before.
    pub carrier: Option<PlayerId>,
    /// The referee, a ghost of the Laws for the view (ADR-021 note): about
    /// ten metres from the ball on his diagonal, running as a referee runs
    /// (`ds_demo::pitchview::referee_step`), cm.
    pub referee: (f32, f32),
}

#[allow(clippy::cast_precision_loss)] // cm and degrees: far inside f32's exact range
fn pos(r: &TickRecord, id: PlayerId) -> Option<(f32, f32)> {
    r.agents
        .iter()
        .find(|a| a.id == id)
        .map(|a| (a.position.0 as f32, a.position.1 as f32))
}

/// Hermite between `p0` and `p1` with tangents `m0`, `m1`, at `s` in
/// `[0, 1]`: it starts at `p0` and ends exactly at `p1`.
fn hermite(p0: f32, p1: f32, m0: f32, m1: f32, s: f32) -> f32 {
    let (s2, s3) = (s * s, s * s * s);
    let h00 = 2.0f32.mul_add(s3, (-3.0f32).mul_add(s2, 1.0));
    let h10 = (-2.0f32).mul_add(s2, s3) + s;
    let h01 = (-2.0f32).mul_add(s3, 3.0 * s2);
    let h11 = s3 - s2;
    h00.mul_add(p0, h10.mul_add(m0, h01.mul_add(p1, h11 * m1)))
}

/// The tangent `m`, no longer than three times the step `seg`.
fn limit(m: (f32, f32), seg: (f32, f32)) -> (f32, f32) {
    let norm = |v: (f32, f32)| {
        #[allow(clippy::cast_possible_truncation)]
        let n = ds_core::dmath::hypot(f64::from(v.0), f64::from(v.1)) as f32;
        n
    };
    let (len, cap) = (norm(m), 3.0 * norm(seg));
    if len <= cap || len <= 0.0 {
        m
    } else {
        (m.0 * cap / len, m.1 * cap / len)
    }
}

/// The short way round from `a` to `b`, degrees, at `s`.
fn turn(a: f32, b: f32, s: f32) -> f32 {
    let d = (b - a + 540.0).rem_euclid(360.0) - 180.0;
    d.mul_add(s, a).rem_euclid(360.0)
}

impl Moment {
    /// The instant `t`, in ticks from the first record (fractional): the
    /// tick `floor(t)` and the next, and how far between them. Past the
    /// last record, the last record. The ball flies with `physics`, the
    /// record's own.
    #[must_use]
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    #[allow(clippy::many_single_char_names)] // drawing geometry
    pub fn at(records: &[TickRecord], t: f64, physics: &RecordPhysics) -> Self {
        let last = records.len().saturating_sub(1);
        let t = t.clamp(0.0, last as f64);
        let i = (t.floor() as usize).min(last);
        let j = (i + 1).min(last);
        let s = if i == j { 0.0 } else { (t - i as f64) as f32 };
        let (r0, r1) = (&records[i], &records[j]);
        let before = &records[i.saturating_sub(1)];
        let after = &records[(j + 1).min(last)];
        let players = r0
            .agents
            .iter()
            .map(|a| {
                let p0 = (a.position.0 as f32, a.position.1 as f32);
                let p1 = pos(r1, a.id).unwrap_or(p0);
                let pb = pos(before, a.id).unwrap_or(p0);
                let pa = pos(after, a.id).unwrap_or(p1);
                // The velocity at each tick, from the ticks either side,
                // limited to three times the step between the two (a
                // restart or a substitution moves a man at once: the curve
                // must not swing out to follow it).
                let seg = (p1.0 - p0.0, p1.1 - p0.1);
                let m0 = limit(((p1.0 - pb.0) / 2.0, (p1.1 - pb.1) / 2.0), seg);
                let m1 = limit(((pa.0 - p0.0) / 2.0, (pa.1 - p0.1) / 2.0), seg);
                let h1 = r1
                    .agents
                    .iter()
                    .find(|b| b.id == a.id)
                    .map_or_else(|| f32::from(a.heading), |b| f32::from(b.heading));
                PlayerAt {
                    id: a.id,
                    at: (
                        hermite(p0.0, p1.0, m0.0, m1.0, s),
                        hermite(p0.1, p1.1, m0.1, m1.1, s),
                    ),
                    heading: turn(f32::from(a.heading), h1, s),
                    #[allow(clippy::cast_possible_truncation)]
                    speed: ds_core::dmath::hypot(f64::from(seg.0), f64::from(seg.1)) as f32,
                    fatigue: f32::from(a.fatigue_milli) / 1000.0,
                }
            })
            .collect();
        let (b0, b1) = (r0.ball.position, r1.ball.position);
        let (x0, y0, z0) = (b0.0 as f32, b0.1 as f32, b0.2 as f32);
        let (x1, y1, z1) = (b1.0 as f32, b1.1 as f32, b1.2 as f32);
        let lerp = |a: f32, b: f32| (b - a).mul_add(s, a);
        let z = if r0.ball.state == BallState::InFlight && (z0 > 0.0 || z1 > 0.0) {
            // In the air: the engine's gravity bends the path between the
            // two ticks, and it ends on the next tick's height.
            let g = f32::from(physics.ball.gravity_cm_per_tick2);
            let half = g / 2.0;
            (half * s)
                .mul_add(-s, (z1 - z0 + half).mul_add(s, z0))
                .max(0.0)
        } else {
            lerp(z0, z1)
        };
        // The referee from the kick-off to these two ticks, a tick at a
        // time, no faster than he runs (the maintainer's report on seed 43:
        // before, he jumped with the ball, and stood on it when it lay on
        // his diagonal).
        let pitch = (
            i32::from(physics.pitch_m.0) * 100,
            i32::from(physics.pitch_m.1) * 100,
        );
        let ball2 = |r: &TickRecord| (r.ball.position.0, r.ball.position.1);
        let mut ref0 = ds_demo::pitchview::referee_at(ball2(&records[0]), pitch);
        for r in &records[..=i] {
            ref0 = ds_demo::pitchview::referee_step(ref0, ball2(r), pitch);
        }
        let ref1 = if j > i {
            ds_demo::pitchview::referee_step(ref0, ball2(r1), pitch)
        } else {
            ref0
        };
        Self {
            players,
            ball: (lerp(x0, x1), lerp(y0, y1), z),
            ball_state: r0.ball.state,
            carrier: r0.carrier,
            referee: (
                lerp(ref0.0 as f32, ref1.0 as f32),
                lerp(ref0.1 as f32, ref1.1 as f32),
            ),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn physics() -> RecordPhysics {
        ds_match::RecordHeader::current(10, (105, 68)).physics
    }

    pub fn records() -> &'static [TickRecord] {
        // One match, played once: the interpolation and scene tests all look
        // at seed 42, and each one used to play it again (19 November 2026).
        static RECORDED: std::sync::LazyLock<Vec<TickRecord>> = std::sync::LazyLock::new(|| {
            let m = ds_fixtures::match_fixture(42).expect("fixture");
            ds_match::simulate_from_seed_recorded(&m, 42, ds_match::MatchConfig::default()).1
        });
        &RECORDED
    }

    /// On a tick the drawing is the tick: the tick wins.
    #[test]
    fn on_a_tick_the_drawing_is_the_record() {
        let r = records();
        for t in [0_usize, 300, 1_234, r.len() - 1] {
            #[allow(clippy::cast_precision_loss)]
            let m = Moment::at(r, t as f64, &physics());
            #[allow(clippy::cast_precision_loss)]
            for (p, a) in m.players.iter().zip(&r[t].agents) {
                assert!((p.at.0 - a.position.0 as f32).abs() < 1e-3);
                assert!((p.at.1 - a.position.1 as f32).abs() < 1e-3);
            }
            #[allow(clippy::cast_precision_loss)]
            let z = r[t].ball.position.2 as f32;
            assert!((m.ball.2 - z).abs() < 1e-3, "tick {t}");
        }
    }

    /// Between two ticks a player is between them (a gentle curve, not a
    /// jump), and the ball in the air never goes under the grass.
    #[test]
    fn between_two_ticks_the_curve_stays_near_the_line() {
        let r = records();
        for t in (100..r.len() - 2).step_by(211) {
            #[allow(clippy::cast_precision_loss)]
            let m = Moment::at(r, t as f64 + 0.5, &physics());
            for p in &m.players {
                let (Some(a), Some(b)) = (pos(&r[t], p.id), pos(&r[t + 1], p.id)) else {
                    continue;
                };
                let dist = |dx: f32, dy: f32| ds_core::dmath::hypot(f64::from(dx), f64::from(dy));
                let step = dist(b.0 - a.0, b.1 - a.1);
                let off = dist(
                    p.at.0 - f32::midpoint(a.0, b.0),
                    p.at.1 - f32::midpoint(a.1, b.1),
                );
                assert!(
                    off <= step.mul_add(0.5, 60.0),
                    "tick {t}: off {off} step {step}"
                );
            }
            assert!(m.ball.2 >= 0.0);
        }
    }

    #[test]
    fn the_facing_turns_the_short_way() {
        assert!((turn(350.0, 10.0, 0.5) - 0.0).abs() < 1e-3);
        assert!((turn(10.0, 350.0, 0.5) - 0.0).abs() < 1e-3);
        assert!((turn(90.0, 180.0, 0.5) - 135.0).abs() < 1e-3);
    }
}
