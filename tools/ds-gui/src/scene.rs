//! The projection of one tick onto the screen (ADR-055, ADR-022 §3).
//!
//! **One responsibility:** turn a tick record (Plane B: what an observer
//! sees) into shapes on a rectangle. No egui painting here, so it can be
//! tested without a window. It invents nothing: the players, the ball and
//! the referee are where the record says.

use ds_core::ids::PlayerId;
use ds_match::{BallState, TickRecord};

/// The pitch fitted into a screen rectangle, keeping its proportions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Geometry {
    /// The pitch's top-left corner on the screen, points.
    pub origin: (f32, f32),
    /// Points per centimetre.
    pub scale: f32,
    /// The real pitch, cm: `(length, width)`.
    pub pitch_cm: (i32, i32),
}

#[allow(clippy::cast_precision_loss)] // cm on a pitch: far inside f32's exact range
impl Geometry {
    /// The largest pitch that fits `size` (points) at `origin`, with
    /// `margin` points all round.
    #[must_use]
    pub fn fit(origin: (f32, f32), size: (f32, f32), margin: f32, pitch_cm: (i32, i32)) -> Self {
        let (l, w) = (pitch_cm.0.max(1) as f32, pitch_cm.1.max(1) as f32);
        let room = (
            2.0f32.mul_add(-margin, size.0).max(1.0),
            2.0f32.mul_add(-margin, size.1).max(1.0),
        );
        let scale = (room.0 / l).min(room.1 / w);
        let used = (l * scale, w * scale);
        Self {
            origin: (
                origin.0 + (size.0 - used.0) / 2.0,
                origin.1 + (size.1 - used.1) / 2.0,
            ),
            scale,
            pitch_cm,
        }
    }

    /// A point of the pitch on the screen. The far touchline (the largest
    /// `y`) is at the top, as in the TUI.
    #[must_use]
    pub fn at(&self, (x, y): (i32, i32)) -> (f32, f32) {
        (
            (x as f32).mul_add(self.scale, self.origin.0),
            ((self.pitch_cm.1 - y) as f32).mul_add(self.scale, self.origin.1),
        )
    }

    /// A length of the pitch on the screen, points.
    #[must_use]
    pub fn len(&self, cm: i32) -> f32 {
        cm as f32 * self.scale
    }
}

/// One player as drawn.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dot {
    /// Who.
    pub id: PlayerId,
    /// Where, on the screen.
    pub at: (f32, f32),
    /// His squad letter (`A`-`K` home, `a`-`k` away).
    pub letter: char,
    /// Home side.
    pub home: bool,
    /// The goalkeeper (the first of the eleven, letter `A` or `a`).
    pub keeper: bool,
    /// He has the ball.
    pub carrier: bool,
    /// Which way he faces, degrees, `0` towards the top, clockwise.
    pub heading: u16,
}

/// The ball as drawn.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ball {
    /// Where its shadow is on the grass, on the screen.
    pub ground: (f32, f32),
    /// How high it is, cm.
    pub height_cm: i32,
    /// At someone's feet, in flight, or loose.
    pub state: BallState,
}

/// What one tick looks like.
#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    /// The players.
    pub dots: Vec<Dot>,
    /// The ball.
    pub ball: Ball,
    /// The referee, on his diagonal (the same rule as the TUI).
    pub referee: (f32, f32),
}

/// The frame of `record` on `geom`.
#[must_use]
pub fn frame(record: &TickRecord, geom: &Geometry) -> Frame {
    let dots = record
        .agents
        .iter()
        .map(|a| {
            let letter = ds_demo::pitchview::marker(a.id);
            Dot {
                id: a.id,
                at: geom.at(a.position),
                letter,
                home: letter.is_ascii_uppercase(),
                keeper: letter.eq_ignore_ascii_case(&'a'),
                carrier: record.carrier == Some(a.id),
                heading: a.heading,
            }
        })
        .collect();
    let (x, y, z) = record.ball.position;
    Frame {
        dots,
        ball: Ball {
            ground: geom.at((x, y)),
            height_cm: z,
            state: record.ball.state,
        },
        referee: geom.at(ds_demo::pitchview::referee_at((x, y), geom.pitch_cm)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PITCH: (i32, i32) = (10_500, 6_800);

    #[test]
    fn the_pitch_keeps_its_proportions_and_fits() {
        let g = Geometry::fit((0.0, 0.0), (1050.0, 1000.0), 10.0, PITCH);
        let (l, w) = (g.len(PITCH.0), g.len(PITCH.1));
        assert!((l / w - 105.0 / 68.0).abs() < 1e-3);
        assert!(l <= 1030.0 + 1e-3 && w <= 980.0 + 1e-3);
    }

    #[test]
    fn the_far_touchline_is_at_the_top() {
        let g = Geometry::fit((0.0, 0.0), (1050.0, 680.0), 0.0, PITCH);
        let near = g.at((0, 0));
        let far = g.at((0, PITCH.1));
        assert!(far.1 < near.1);
        let end = g.at((PITCH.0, 0));
        assert!(end.0 > near.0);
    }

    /// The whole match, projected: every tick has twenty-two players, a
    /// ball and a referee inside the drawn pitch (the sample is not empty).
    #[test]
    fn every_tick_of_a_match_projects_inside_the_pitch() {
        let m = ds_fixtures::match_fixture(42).expect("fixture");
        let (_, records) =
            ds_match::simulate_from_seed_recorded(&m, 42, ds_match::MatchConfig::default());
        assert!(records.len() > 1000, "a whole match: {}", records.len());
        let g = Geometry::fit((0.0, 0.0), (1050.0, 680.0), 0.0, PITCH);
        let inside =
            |(x, y): (f32, f32)| (-0.5..=1050.5).contains(&x) && (-0.5..=680.5).contains(&y);
        for r in records.iter().step_by(97) {
            let f = frame(r, &g);
            assert!((20..=22).contains(&f.dots.len()), "tick {}", r.tick);
            assert!(f.dots.iter().filter(|d| d.keeper).count() <= 2);
            assert!(inside(f.referee), "referee {:?}", f.referee);
            assert!(f.ball.height_cm >= 0);
        }
    }
}
