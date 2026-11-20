//! The projection of one tick onto the screen (ADR-055, ADR-022 §3), the
//! tilted board of the 2.5D view (`tools/ds-gui/DESIGN.md`, decision R).
//!
//! **One responsibility:** turn what the record says (Plane B: what an
//! observer sees) into shapes on a rectangle, through a camera that looks
//! at the pitch from above, tilted towards the near touchline. No egui
//! painting here, so it can be tested without a window. It invents
//! nothing: the players, the ball and the referee are where the record
//! says; the drawing is in `f32` and never goes back to the simulation.

use ds_core::ids::PlayerId;
use ds_match::BallState;

use crate::interp::Moment;

/// How far the camera stands from what it looks at, as a share of the
/// span of pitch it shows across: a drawing choice, the strength of the
/// perspective.
const DISTANCE_SHARE: f32 = 1.3;
/// The margin round the pitch the view may show and move into, cm (a
/// drawing size: the whole pitch with its goals and a little ground).
pub const MARGIN_CM: f32 = 800.0;
/// The nearest the zoom goes: about this much pitch across, cm (DESIGN.md,
/// "Una partita, molte viste": about 25 m).
pub const NEAREST_SPAN_CM: f32 = 2_500.0;
/// A token's radius, cm (a drawing size, about a man's shoulders).
pub const TOKEN_CM: f32 = 55.0;
/// The smallest a token is drawn, its radius in points: readable at every
/// zoom (DESIGN.md).
pub const MIN_TOKEN_PX: f32 = 6.5;
/// The smallest the ball is drawn, its radius in points.
pub const MIN_BALL_PX: f32 = 3.5;
/// The ball's radius, cm (Law 2: a circumference of 68-70 cm).
const BALL_RADIUS_CM: f32 = 11.0;
/// The goal's depth behind the line, cm (a drawing size).
const GOAL_DEPTH_CM: f32 = 200.0;
/// A point of the pitch, cm, `(x, y, z)`: `z` up from the grass.
pub type World = (f32, f32, f32);

/// A point on the screen, points, with its depth from the camera, cm.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Screen {
    /// Across, points.
    pub x: f32,
    /// Down, points.
    pub y: f32,
    /// How far from the camera, cm: larger is farther.
    pub depth: f32,
}

/// The camera fitted to a screen rectangle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Geometry {
    /// The real pitch, cm: `(length, width)`.
    pub pitch_cm: (i32, i32),
    /// The tilt from the vertical, radians.
    tilt: f32,
    /// The camera, cm.
    eye: World,
    /// The point of the grass the camera looks at, cm.
    centre: (f32, f32),
    /// Points per unit of projected size.
    scale: f32,
    /// The screen point of the projected origin, points.
    offset: (f32, f32),
}

#[allow(clippy::cast_precision_loss)] // cm on a pitch: far inside f32's exact range
impl Geometry {
    /// The span across the screen that shows the whole pitch, cm.
    #[must_use]
    pub fn whole_span(pitch_cm: (i32, i32)) -> f32 {
        2.0f32.mul_add(MARGIN_CM, pitch_cm.0.max(1) as f32)
    }

    /// The camera at `tilt_deg` (`0` from above), showing about `span_cm`
    /// of pitch across the rectangle, centred on `focus` as far as the
    /// pitch and its margin allow: the view never leaves the pitch, at
    /// every zoom (DESIGN.md, "Una partita, molte viste").
    #[must_use]
    #[allow(clippy::many_single_char_names)] // drawing geometry
    pub fn view(
        origin: (f32, f32),
        size: (f32, f32),
        pitch_cm: (i32, i32),
        tilt_deg: f32,
        (focus, span_cm): ((f32, f32), f32),
    ) -> Self {
        let (l, w) = (pitch_cm.0.max(1) as f32, pitch_cm.1.max(1) as f32);
        let span = span_cm.clamp(NEAREST_SPAN_CM.min(l), Self::whole_span(pitch_cm));
        let half = span / 2.0;
        let depth_half = half * size.1 / size.0.max(1.0);
        let keep = |f: f32, half: f32, len: f32| {
            if half >= len / 2.0 + MARGIN_CM {
                len / 2.0
            } else {
                f.clamp(half - MARGIN_CM, len - half + MARGIN_CM)
            }
        };
        let (fx, fy) = (keep(focus.0, half, l), keep(focus.1, depth_half, w));
        let tilt = tilt_deg.clamp(0.0, 75.0).to_radians();
        let d = DISTANCE_SHARE * span;
        #[allow(
            clippy::disallowed_methods,
            reason = "drawing only: the view never feeds the simulation"
        )]
        let eye = (fx, d.mul_add(-tilt.sin(), fy), d * tilt.cos());
        let mut g = Self {
            pitch_cm,
            tilt,
            eye,
            centre: (fx, fy),
            scale: 1.0,
            offset: (0.0, 0.0),
        };
        // The span is measured on the nearest line of grass in view, where
        // the perspective makes the pitch widest: what is asked fits there.
        let near = (fy - depth_half).clamp(-MARGIN_CM, fy);
        let (Some(a), Some(b), Some(c)) = (
            g.raw((fx - half, near, 0.0)),
            g.raw((fx + half, near, 0.0)),
            g.raw((fx, fy, 0.0)),
        ) else {
            return g;
        };
        g.scale = size.0 / (b.0 - a.0).max(1e-6);
        g.offset = (
            c.0.mul_add(-g.scale, size.0.mul_add(0.5, origin.0)),
            c.1.mul_add(-g.scale, size.1.mul_add(0.5, origin.1)),
        );
        g
    }

    /// Where the camera looks on the grass, cm: the focus once kept
    /// inside the pitch and its margin.
    #[must_use]
    pub const fn looks_at(&self) -> (f32, f32) {
        self.centre
    }

    /// The camera's axes: forward (to the centre), right, up on screen.
    #[allow(
        clippy::disallowed_methods,
        reason = "drawing only: the view never feeds the simulation"
    )]
    fn axes(&self) -> (World, World, World) {
        let (s, c) = (self.tilt.sin(), self.tilt.cos());
        ((0.0, s, -c), (1.0, 0.0, 0.0), (0.0, c, s))
    }

    /// The projection before the fit: `(across, down)` and the depth.
    #[allow(clippy::many_single_char_names)] // drawing geometry
    fn raw_depth(&self, p: World) -> Option<(f32, f32, f32)> {
        let (f, r, u) = self.axes();
        let d = (p.0 - self.eye.0, p.1 - self.eye.1, p.2 - self.eye.2);
        let dot = |a: World| a.0.mul_add(d.0, a.1.mul_add(d.1, a.2 * d.2));
        let depth = dot(f);
        if depth <= 1.0 {
            return None;
        }
        Some((dot(r) / depth, -dot(u) / depth, depth))
    }

    fn raw(&self, p: World) -> Option<(f32, f32)> {
        self.raw_depth(p).map(|(x, y, _)| (x, y))
    }

    /// A point of the pitch on the screen, with its depth; `None` behind
    /// the camera.
    #[must_use]
    pub fn project(&self, p: World) -> Option<Screen> {
        self.raw_depth(p).map(|(x, y, depth)| Screen {
            x: x.mul_add(self.scale, self.offset.0),
            y: y.mul_add(self.scale, self.offset.1),
            depth,
        })
    }

    /// The camera's tilt from the vertical, radians.
    #[must_use]
    pub const fn tilt(&self) -> f32 {
        self.tilt
    }

    /// Where the horizon is on the screen, points down; `None` looking
    /// straight down (no horizon).
    #[must_use]
    pub fn horizon(&self) -> Option<f32> {
        if self.tilt < 0.02 {
            return None;
        }
        let far = (self.eye.0, self.eye.1 + 1.0e9, 0.0);
        self.project(far).map(|s| s.y)
    }

    /// A point of the grass on the screen (`z = 0`), points.
    #[must_use]
    pub fn at(&self, (x, y): (i32, i32)) -> (f32, f32) {
        self.project((x as f32, y as f32, 0.0))
            .map_or((0.0, 0.0), |s| (s.x, s.y))
    }

    /// A length of the pitch at `at`, on the screen, points: how big
    /// `cm` looks there (smaller farther away).
    #[must_use]
    pub fn len_at(&self, cm: f32, at: World) -> f32 {
        self.raw_depth(at)
            .map_or(0.0, |(_, _, depth)| cm / depth * self.scale)
    }

    /// The goals: for each end, its two posts and crossbar, and the back
    /// of the net on the grass, as segments of the pitch, cm (Law 1: the
    /// posts `goal_half_width_cm` either side of the middle, the bar at
    /// `goal_height_cm`).
    #[must_use]
    #[allow(clippy::many_single_char_names)] // drawing geometry
    pub fn goals(&self, goal_half_width_cm: f32, goal_height_cm: f32) -> Vec<(World, World)> {
        let (l, w) = (self.pitch_cm.0 as f32, self.pitch_cm.1 as f32);
        let (a, b) = (w / 2.0 - goal_half_width_cm, w / 2.0 + goal_half_width_cm);
        let h = goal_height_cm;
        let mut out = Vec::new();
        for (x, back) in [(0.0, -GOAL_DEPTH_CM), (l, l + GOAL_DEPTH_CM)] {
            out.push(((x, a, 0.0), (x, a, h)));
            out.push(((x, b, 0.0), (x, b, h)));
            out.push(((x, a, h), (x, b, h)));
            out.push(((x, a, h), (back, a, 0.0)));
            out.push(((x, b, h), (back, b, 0.0)));
            out.push(((back, a, 0.0), (back, b, 0.0)));
        }
        out
    }
}

/// One player as drawn: a token on his spot (DESIGN.md, "Pallini nel
/// 2.5D, omini nel 3D").
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    /// Who.
    pub id: PlayerId,
    /// His spot on the grass, on the screen.
    pub ground: Screen,
    /// A point a metre ahead of him on the grass, on the screen: where he
    /// faces.
    pub facing: Screen,
    /// His shirt number: his place in the squad, from 1.
    pub number: u32,
    /// Home side.
    pub home: bool,
    /// The goalkeeper (the first of the eleven).
    pub keeper: bool,
    /// He has the ball.
    pub carrier: bool,
    /// His spot on the grass, cm.
    pub at: (f32, f32),
    /// Which way he faces, degrees.
    pub heading: f32,
}

/// A gesture, only from a fact of the match (DESIGN.md, the table of the
/// animations).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gesture {
    /// A small push towards the kick: a pass, a cross, a shot, a clearance.
    Kick,
    /// A longer push and a lean: the pass into the space a teammate attacks
    /// (ADR-085, SIM 168), read from the `through` qualifier of the event.
    Through,
    /// Off the ground and down again: a header, an aerial duel.
    Jump,
    /// The keeper stretches and leans towards the side `1` (his right) or `-1`.
    Dive(f32),
    /// Flattened, sliding a little forward: a sliding foul.
    Slide,
    /// Flattened, a little forward: a tackle.
    Lunge,
    /// Flattened on the grass after a foul, rising slowly.
    Fall,
    /// The throw-in: a small push.
    Throw,
    /// Small hops after a goal.
    Celebrate,
    /// The referee showing a card: red or yellow.
    Card(bool),
}

/// The ball as drawn.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ball {
    /// Where it is on the screen.
    pub at: Screen,
    /// Its shadow on the grass, on the screen.
    pub ground: Screen,
    /// How high it is, cm.
    pub height_cm: f32,
    /// Its radius on the screen, points.
    pub radius: f32,
    /// At someone's feet, in flight, or loose.
    pub state: BallState,
}

/// Something to draw, in the order of depth.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Item {
    /// A player.
    Token(Token),
    /// The referee, as a token without a side.
    Referee(Token),
    /// The ball.
    Ball(Ball),
}

impl Item {
    /// How far from the camera, cm.
    #[must_use]
    pub const fn depth(&self) -> f32 {
        match self {
            Self::Token(t) | Self::Referee(t) => t.ground.depth,
            Self::Ball(b) => b.at.depth,
        }
    }
}

/// What one instant looks like.
#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    /// The players, the referee and the ball, farthest first.
    pub items: Vec<Item>,
    /// Every ball shadow, drawn on the grass before the figures.
    pub shadows: Vec<(Screen, f32)>,
}

fn token(
    geom: &Geometry,
    id: PlayerId,
    (x, y): (f32, f32),
    heading: f32,
    carrier: bool,
) -> Option<Token> {
    let ground = geom.project((x, y, 0.0))?;
    #[allow(
        clippy::disallowed_methods,
        reason = "drawing only: the view never feeds the simulation"
    )]
    let (s, c) = heading.to_radians().sin_cos();
    // Heading: 0 towards positive y, clockwise (the record's convention).
    let facing = geom.project((s.mul_add(100.0, x), c.mul_add(100.0, y), 0.0))?;
    let slot = id.slot();
    Some(Token {
        id,
        ground,
        facing,
        number: slot % 100 + 1,
        home: slot < 100,
        keeper: slot % 100 == 0,
        carrier,
        at: (x, y),
        heading,
    })
}

/// The referee's id in a frame and in the gestures: he is no player.
pub const REFEREE: PlayerId = PlayerId::new(u32::MAX, 0);

/// The frame of the instant `m` on `geom`.
#[must_use]
pub fn frame(m: &Moment, geom: &Geometry) -> Frame {
    let referee = REFEREE;
    let mut items: Vec<Item> = m
        .players
        .iter()
        .filter_map(|p| {
            token(geom, p.id, p.at, p.heading, m.carrier == Some(p.id)).map(Item::Token)
        })
        .collect();
    if let Some(r) = token(geom, referee, m.referee, 0.0, false) {
        items.push(Item::Referee(r));
    }
    let (bx, by, bz) = m.ball;
    let mut shadows = Vec::new();
    if let (Some(at), Some(ground)) = (
        geom.project((bx, by, bz.max(0.0) + BALL_RADIUS_CM)),
        geom.project((bx, by, 0.0)),
    ) {
        let radius = geom.len_at(BALL_RADIUS_CM, (bx, by, bz)).max(MIN_BALL_PX);
        shadows.push((ground, radius));
        items.push(Item::Ball(Ball {
            at,
            ground,
            height_cm: bz,
            radius,
            state: m.ball_state,
        }));
    }
    // Farthest first, so that the nearer covers the farther.
    items.sort_by(|a, b| b.depth().total_cmp(&a.depth()));
    Frame { items, shadows }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PITCH: (i32, i32) = (10_500, 6_800);

    fn whole(tilt: f32) -> Geometry {
        Geometry::view(
            (0.0, 0.0),
            (1200.0, 800.0),
            PITCH,
            tilt,
            ((0.0, 0.0), Geometry::whole_span(PITCH)),
        )
    }

    /// From above (0°) the board is the 2D board of before: the far
    /// touchline at the top, the pitch with its proportions, and zoomed
    /// out the camera looks at the centre whatever the focus.
    #[test]
    #[allow(clippy::many_single_char_names)]
    fn from_above_the_pitch_keeps_its_proportions() {
        let g = whole(0.0);
        let (a, b) = (g.at((0, 0)), g.at((PITCH.0, 0)));
        let c = g.at((0, PITCH.1));
        let (l, w) = (b.0 - a.0, a.1 - c.1);
        assert!(w > 0.0, "the far touchline is at the top");
        assert!((l / w - 105.0 / 68.0).abs() < 0.02, "{l} {w}");
        assert_eq!(g.looks_at().0, 5_250.0);
    }

    /// Tilted, the far touchline looks shorter than the near one
    /// (perspective).
    #[test]
    fn tilted_the_far_side_is_smaller() {
        let g = whole(45.0);
        let near = g.at((PITCH.0, 0)).0 - g.at((0, 0)).0;
        let far = g.at((PITCH.0, PITCH.1)).0 - g.at((0, PITCH.1)).0;
        assert!(far < near, "far {far} near {near}");
    }

    /// Zoomed in, the view stays inside the pitch and its margin wherever
    /// the focus is asked to go (DESIGN.md: dragging stays in bounds).
    #[test]
    fn the_view_never_leaves_the_pitch() {
        for focus in [
            (-9_000.0, -9_000.0),
            (20_000.0, 20_000.0),
            (5_250.0, 3_400.0),
        ] {
            let g = Geometry::view((0.0, 0.0), (1200.0, 800.0), PITCH, 45.0, (focus, 2_500.0));
            let (x, _) = g.looks_at();
            assert!(
                (1_250.0 - MARGIN_CM..=9_250.0 + MARGIN_CM).contains(&x),
                "{x}"
            );
        }
    }

    /// The nearer token is drawn after the farther one; the tokens carry
    /// their numbers from 1, the keeper first.
    #[test]
    fn the_nearer_is_drawn_last() {
        let records = crate::interp::tests::records();
        let g = whole(45.0);
        let f = frame(
            &crate::interp::Moment::at(records, 500.0, &crate::interp::tests::physics()),
            &g,
        );
        let depths: Vec<f32> = f.items.iter().map(Item::depth).collect();
        assert!(depths.windows(2).all(|w| w[0] >= w[1]));
        assert!(f.items.len() >= 22, "{}", f.items.len());
        for i in &f.items {
            if let Item::Token(t) = i {
                assert!((1..=18).contains(&t.number));
                assert_eq!(t.keeper, t.number == 1);
            }
        }
    }

    /// The whole match, zoomed out at every tilt: every token inside the
    /// drawn rectangle.
    #[test]
    fn every_tick_of_a_match_projects_inside_the_board() {
        let records = crate::interp::tests::records();
        for tilt in [0.0, 30.0, 45.0, 60.0] {
            let g = whole(tilt);
            let inside =
                |s: &Screen| (-1.0..=1201.0).contains(&s.x) && (-1.0..=801.0).contains(&s.y);
            for t in (0..records.len()).step_by(197) {
                #[allow(clippy::cast_precision_loss)]
                let f = frame(
                    &crate::interp::Moment::at(records, t as f64, &crate::interp::tests::physics()),
                    &g,
                );
                for i in &f.items {
                    if let Item::Token(p) = i {
                        assert!(inside(&p.ground), "tilt {tilt} tick {t}: {:?}", p.ground);
                    }
                }
            }
        }
    }
}
