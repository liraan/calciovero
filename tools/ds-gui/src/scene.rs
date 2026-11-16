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

/// The camera presets, degrees of tilt from the vertical: `0` is the view
/// from above (the 2D board of before), the others lean towards the near
/// touchline.
pub const PRESETS: [f32; 4] = [0.0, 30.0, 45.0, 60.0];

/// How far the camera stands from the centre of the pitch, as a share of
/// the pitch's length: a drawing choice, the strength of the perspective.
const DISTANCE_SHARE: f32 = 1.3;
/// A figure's height on the board, cm (a drawing size, not the player's).
pub const FIGURE_CM: f32 = 180.0;
/// The ball's radius, cm (Law 2: a circumference of 68-70 cm).
const BALL_RADIUS_CM: f32 = 11.0;
/// The goal's depth behind the line, cm (a drawing size).
const GOAL_DEPTH_CM: f32 = 200.0;
/// The thinnest a line of a figure is drawn, points: at true size a man
/// on the whole pitch is about 10 points high (DESIGN.md, "La grandezza
/// vera decide"), and he must stay visible.
pub const MIN_LINE: f32 = 1.2;

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
    /// Points per unit of projected size.
    scale: f32,
    /// The screen point of the projected origin, points.
    offset: (f32, f32),
}

#[allow(clippy::cast_precision_loss)] // cm on a pitch: far inside f32's exact range
impl Geometry {
    /// The camera at `tilt_deg`, fitted so that the pitch, its goals and
    /// the figures on its lines fit `size` (points) at `origin`, with
    /// `margin` points all round.
    #[must_use]
    pub fn fit(
        origin: (f32, f32),
        size: (f32, f32),
        margin: f32,
        pitch_cm: (i32, i32),
        tilt_deg: f32,
    ) -> Self {
        let (l, w) = (pitch_cm.0.max(1) as f32, pitch_cm.1.max(1) as f32);
        let tilt = tilt_deg.clamp(0.0, 75.0).to_radians();
        let d = DISTANCE_SHARE * l;
        #[allow(
            clippy::disallowed_methods,
            reason = "drawing only: the view never feeds the simulation"
        )]
        let eye = (l / 2.0, d.mul_add(-tilt.sin(), w / 2.0), d * tilt.cos());
        let mut g = Self {
            pitch_cm,
            tilt,
            eye,
            scale: 1.0,
            offset: (0.0, 0.0),
        };
        // The box of everything drawn: the pitch with its goals, a figure
        // standing on each corner.
        let mut lo = (f32::INFINITY, f32::INFINITY);
        let mut hi = (f32::NEG_INFINITY, f32::NEG_INFINITY);
        for x in [-GOAL_DEPTH_CM, l + GOAL_DEPTH_CM] {
            for y in [0.0, w] {
                for z in [0.0, FIGURE_CM] {
                    if let Some((px, py)) = g.raw((x, y, z)) {
                        lo = (lo.0.min(px), lo.1.min(py));
                        hi = (hi.0.max(px), hi.1.max(py));
                    }
                }
            }
        }
        let room = (
            2.0f32.mul_add(-margin, size.0).max(1.0),
            2.0f32.mul_add(-margin, size.1).max(1.0),
        );
        let span = ((hi.0 - lo.0).max(1e-6), (hi.1 - lo.1).max(1e-6));
        let scale = (room.0 / span.0).min(room.1 / span.1);
        let used = (span.0 * scale, span.1 * scale);
        g.scale = scale;
        g.offset = (
            lo.0.mul_add(-scale, (size.0 - used.0).mul_add(0.5, origin.0)),
            lo.1.mul_add(-scale, (size.1 - used.1).mul_add(0.5, origin.1)),
        );
        g
    }

    /// The close camera (DESIGN.md, "La visuale del 2.5D"): about
    /// `span_cm` of pitch across the rectangle, centred on `focus` (the
    /// ball, followed with a damped movement by the caller), never past
    /// the pitch's edges.
    #[must_use]
    #[allow(clippy::many_single_char_names)] // drawing geometry
    pub fn around(
        origin: (f32, f32),
        size: (f32, f32),
        pitch_cm: (i32, i32),
        tilt_deg: f32,
        (focus, span_cm): ((f32, f32), f32),
    ) -> Self {
        let (l, w) = (pitch_cm.0.max(1) as f32, pitch_cm.1.max(1) as f32);
        let half = (span_cm / 2.0).min(l / 2.0);
        let depth_half = (half * size.1 / size.0.max(1.0)).min(w / 2.0);
        let fx = focus.0.clamp(half, l - half);
        let fy = focus.1.clamp(depth_half, w - depth_half);
        let tilt = tilt_deg.clamp(0.0, 75.0).to_radians();
        let d = DISTANCE_SHARE * span_cm;
        #[allow(
            clippy::disallowed_methods,
            reason = "drawing only: the view never feeds the simulation"
        )]
        let eye = (fx, d.mul_add(-tilt.sin(), fy), d * tilt.cos());
        let mut g = Self {
            pitch_cm,
            tilt,
            eye,
            scale: 1.0,
            offset: (0.0, 0.0),
        };
        let (Some(a), Some(b), Some(c)) = (
            g.raw((fx - half, fy, 0.0)),
            g.raw((fx + half, fy, 0.0)),
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

/// One player as drawn: a figure standing on his spot.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Figure {
    /// Who.
    pub id: PlayerId,
    /// His feet on the screen.
    pub feet: Screen,
    /// His head on the screen.
    pub head: Screen,
    /// The tip of the line on the grass that shows where he faces.
    pub facing: Screen,
    /// His squad letter (`A`-`K` home, `a`-`k` away).
    pub letter: char,
    /// Home side.
    pub home: bool,
    /// The goalkeeper (the first of the eleven, letter `A` or `a`).
    pub keeper: bool,
    /// He has the ball.
    pub carrier: bool,
    /// His size on the screen, points: the radius of his head (11 cm).
    pub size: f32,
    /// His spot on the grass, cm.
    pub at: (f32, f32),
    /// Which way he faces, degrees.
    pub heading: f32,
    /// His speed, cm a tick.
    pub speed: f32,
    /// How tired he looks, `[0, 1]`.
    pub fatigue: f32,
}

/// What a limb is dressed in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    /// The shirt: the side's colour.
    Shirt,
    /// The legs and socks: the side's colour, darker.
    Sock,
}

/// One limb of a figure on the screen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Limb {
    /// One end.
    pub a: Screen,
    /// The other end.
    pub b: Screen,
    /// Its thickness, points.
    pub width: f32,
    /// What it is dressed in.
    pub part: Part,
}

/// How a figure moves at an instant: his stride, and the gesture a fact of
/// the match gives him (DESIGN.md: an animation only from a fact).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Motion {
    /// Where his stride is, radians: it moves on with the ground he
    /// covers.
    pub phase: f32,
    /// His speed, cm a tick.
    pub speed: f32,
    /// The gesture a fact of the match gives him now, and how far into it
    /// he is, `[0, 1]`.
    pub gesture: Option<(Gesture, f32)>,
    /// How tired he looks, `[0, 1]` (the record's `fatigue_milli`): a
    /// heavier step, the body bent.
    pub fatigue: f32,
}

/// A gesture, only from a fact of the match (DESIGN.md, the table of the
/// animations).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gesture {
    /// The leg that loads and kicks: a pass, a cross, a shot, a clearance.
    Kick,
    /// The jump with the arms high: a header, an aerial duel.
    Jump,
    /// The keeper's dive, towards the side `1` (his right) or `-1`.
    Dive(f32),
    /// The slide along the grass: a sliding foul.
    Slide,
    /// The standing tackle: the lunge, one leg out.
    Lunge,
    /// The fall after a foul, and getting up.
    Fall,
    /// The throw-in, the ball held over his head.
    Throw,
    /// The celebration after a goal: arms high, small jumps.
    Celebrate,
    /// The referee showing a card, his arm high.
    Card,
}

/// The articulated figure (DESIGN.md, "Gli omini articolati"): legs,
/// body, arms and head, from his spot, his facing and his stride, at true
/// size, never thinner than `MIN_LINE`, in the pose of the gesture a fact
/// gives him. Built in the pitch's space and projected, so it has the
/// camera's perspective.
#[must_use]
#[allow(
    clippy::many_single_char_names,
    clippy::too_many_lines,
    clippy::suboptimal_flops
)] // drawing geometry
pub fn limbs(
    geom: &Geometry,
    (x, y): (f32, f32),
    heading: f32,
    m: Motion,
) -> (Vec<Limb>, Screen, f32) {
    use std::f32::consts::PI;
    #[allow(
        clippy::disallowed_methods,
        reason = "drawing only: the view never feeds the simulation"
    )]
    let ((hs, hc), (ps, pc)) = (heading.to_radians().sin_cos(), m.phase.sin_cos());
    let fwd = (hs, hc);
    let side = (hc, -hs);
    // A full stride at a run (8 m/s, 80 cm a tick), barely moving still.
    let tired = m.fatigue.clamp(0.0, 1.0);
    let mut amp = (m.speed / 80.0).clamp(0.0, 1.0) * 45.0 * (1.0 - tired * 0.35);
    let mut lean = (m.speed / 80.0).clamp(0.0, 1.0) * 14.0 + tired * 16.0;
    // The pose of the gesture: how high he is off the grass, how low his
    // hips, where his arms and his kicking leg go, how far he lies over.
    let mut lift = 0.0f32;
    let mut hip_z = 92.0f32;
    let mut hands_z = 92.0f32;
    let mut kick: Option<(f32, f32)> = None;
    let mut over = 0.0f32;
    if let Some((g, p)) = m.gesture {
        #[allow(
            clippy::disallowed_methods,
            reason = "drawing only: the view never feeds the simulation"
        )]
        let arc = (PI * p).sin();
        match g {
            Gesture::Kick => {
                amp = 0.0;
                // Back as he loads it, through as he strikes.
                let swing = if p < 0.6 {
                    -40.0 * p / 0.6
                } else {
                    (p - 0.6) / 0.4 * 95.0 - 40.0
                };
                kick = Some((swing, (swing.max(0.0) / 55.0) * 40.0));
            }
            Gesture::Jump => {
                amp = 0.0;
                lift = arc * 45.0;
                hands_z = 92.0 + arc * 120.0;
            }
            Gesture::Dive(sgn) => {
                amp = 0.0;
                over = sgn * arc;
                hip_z = 92.0 - arc * 60.0;
                hands_z = 92.0 + arc * 100.0;
            }
            Gesture::Slide => {
                amp = 0.0;
                hip_z = 92.0 - arc * 70.0;
                lean = -arc * 25.0;
                kick = Some((arc * 80.0, 5.0));
            }
            Gesture::Lunge => {
                amp = 0.0;
                lean = arc * 25.0;
                kick = Some((arc * 60.0, 8.0));
            }
            Gesture::Fall => {
                amp = 0.0;
                // Down at once, up slowly.
                let down = if p < 0.25 {
                    p / 0.25
                } else {
                    1.0 - (p - 0.25) / 0.75
                };
                hip_z = 92.0 - down * 72.0;
                lean = down * 90.0;
            }
            Gesture::Throw => {
                amp = 0.0;
                hands_z = 92.0 + 135.0;
                lean = -arc * 12.0;
            }
            Gesture::Celebrate => {
                amp = 0.0;
                #[allow(
                    clippy::disallowed_methods,
                    reason = "drawing only: the view never feeds the simulation"
                )]
                let hop = (PI * 6.0 * p).sin().abs();
                lift = hop * 25.0;
                hands_z = 92.0 + 125.0;
            }
            Gesture::Card => {
                amp = 0.0;
                hands_z = 92.0 + 120.0;
            }
        }
    }
    let at = |f: f32, sd: f32, z: f32| -> World {
        (
            f.mul_add(fwd.0, sd.mul_add(side.0, x)),
            f.mul_add(fwd.1, sd.mul_add(side.1, y)),
            z + lift,
        )
    };
    // Lying over to a side (the dive): the upper body leans along `side`.
    let up = |f: f32, sd: f32, z: f32| -> World {
        let off = over * (z - hip_z).max(0.0) * 0.9;
        at(
            f,
            off.mul_add(1.0, sd),
            z - over.abs() * (z - hip_z).max(0.0) * 0.6,
        )
    };
    let w = |cm: f32, p: World| geom.len_at(cm, p).max(MIN_LINE);
    let mut out = Vec::new();
    let mut seg = |a: World, b: World, cm: f32, part: Part| {
        if let (Some(pa), Some(pb)) = (geom.project(a), geom.project(b)) {
            out.push(Limb {
                a: pa,
                b: pb,
                width: w(cm, a),
                part,
            });
        }
    };
    for (k, sd) in [(1.0f32, 11.0f32), (-1.0, -11.0)] {
        let swing = k * ps * amp;
        let step = (k * pc).max(0.0) * amp * 0.25;
        let hip = at(lean * 0.3, sd, hip_z);
        // The kicking leg is his right one (k = 1).
        let (foot_f, foot_z) = match kick {
            Some((f, z)) if k > 0.0 => (f, z),
            _ => (swing, step),
        };
        let foot = at(foot_f, sd, foot_z);
        let knee = at(foot_f.mul_add(0.5, 6.0), sd, f32::midpoint(foot_z, hip_z));
        seg(hip, knee, 15.0, Part::Sock);
        seg(knee, foot, 12.0, Part::Sock);
        // The arm swings against the leg on its side.
        let shoulder = up(lean, sd * 1.8, hip_z + 53.0);
        let hand = up(-swing.mul_add(0.6, -lean), sd * 2.4, hands_z);
        seg(shoulder, hand, 9.0, Part::Shirt);
    }
    seg(
        at(lean * 0.3, 0.0, hip_z),
        up(lean, 0.0, hip_z + 58.0),
        32.0,
        Part::Shirt,
    );
    let head_at = up(lean * 1.1, 0.0, hip_z + 76.0);
    let head = geom.project(head_at).unwrap_or(Screen {
        x: 0.0,
        y: 0.0,
        depth: 0.0,
    });
    (out, head, w(11.0, head_at))
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
    Figure(Figure),
    /// The referee, as a figure without a side.
    Referee(Figure),
    /// The ball.
    Ball(Ball),
}

impl Item {
    /// How far from the camera, cm.
    #[must_use]
    pub const fn depth(&self) -> f32 {
        match self {
            Self::Figure(f) | Self::Referee(f) => f.feet.depth,
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

fn figure(
    geom: &Geometry,
    id: PlayerId,
    ((x, y), speed, fatigue): ((f32, f32), f32, f32),
    heading: f32,
    (letter, carrier): (char, bool),
) -> Option<Figure> {
    let feet = geom.project((x, y, 0.0))?;
    let head = geom.project((x, y, FIGURE_CM))?;
    #[allow(
        clippy::disallowed_methods,
        reason = "drawing only: the view never feeds the simulation"
    )]
    let (s, c) = heading.to_radians().sin_cos();
    // Heading: 0 towards positive y, clockwise (the record's convention).
    let facing = geom.project((s.mul_add(90.0, x), c.mul_add(90.0, y), 0.0))?;
    Some(Figure {
        id,
        feet,
        head,
        facing,
        letter,
        home: letter.is_ascii_uppercase(),
        keeper: letter.eq_ignore_ascii_case(&'a'),
        carrier,
        size: geom.len_at(11.0, (x, y, FIGURE_CM)).max(MIN_LINE),
        at: (x, y),
        heading,
        speed,
        fatigue,
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
            let letter = ds_demo::pitchview::marker(p.id);
            figure(
                geom,
                p.id,
                (p.at, p.speed, p.fatigue),
                p.heading,
                (letter, m.carrier == Some(p.id)),
            )
            .map(Item::Figure)
        })
        .collect();
    #[allow(clippy::cast_possible_truncation)] // a point on the pitch, cm
    let (rx, ry) =
        ds_demo::pitchview::referee_at((m.ball.0 as i32, m.ball.1 as i32), geom.pitch_cm);
    #[allow(clippy::cast_precision_loss)] // cm on a pitch
    if let Some(r) = figure(
        geom,
        referee,
        ((rx as f32, ry as f32), 0.0, 0.0),
        0.0,
        ('·', false),
    ) {
        items.push(Item::Referee(r));
    }
    let (bx, by, bz) = m.ball;
    let mut shadows = Vec::new();
    if let (Some(at), Some(ground)) = (
        geom.project((bx, by, bz.max(0.0) + BALL_RADIUS_CM)),
        geom.project((bx, by, 0.0)),
    ) {
        let radius = geom
            .len_at(BALL_RADIUS_CM, (bx, by, bz))
            .max(MIN_LINE * 1.5);
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

    fn fit(tilt: f32) -> Geometry {
        Geometry::fit((0.0, 0.0), (1200.0, 800.0), 10.0, PITCH, tilt)
    }

    /// From above (0°) the board is the 2D board of before: the far
    /// touchline at the top, the pitch with its proportions.
    #[test]
    #[allow(clippy::many_single_char_names)]
    fn from_above_the_pitch_keeps_its_proportions() {
        let g = fit(0.0);
        let (a, b) = (g.at((0, 0)), g.at((PITCH.0, 0)));
        let c = g.at((0, PITCH.1));
        let (l, w) = (b.0 - a.0, a.1 - c.1);
        assert!(w > 0.0, "the far touchline is at the top");
        assert!((l / w - 105.0 / 68.0).abs() < 0.02, "{l} {w}");
    }

    /// Tilted, the far touchline looks shorter than the near one
    /// (perspective), and a figure stands up from the grass.
    #[test]
    fn tilted_the_far_side_is_smaller_and_figures_stand_up() {
        let g = fit(45.0);
        let near = g.at((PITCH.0, 0)).0 - g.at((0, 0)).0;
        let far = g.at((PITCH.0, PITCH.1)).0 - g.at((0, PITCH.1)).0;
        assert!(far < near, "far {far} near {near}");
        let feet = g.project((5250.0, 3400.0, 0.0)).expect("feet");
        let head = g.project((5250.0, 3400.0, FIGURE_CM)).expect("head");
        assert!(head.y < feet.y, "the head above the feet");
    }

    /// The nearer figure is drawn after the farther one.
    #[test]
    fn the_nearer_is_drawn_last() {
        let m = ds_fixtures::match_fixture(42).expect("fixture");
        let (_, records) =
            ds_match::simulate_from_seed_recorded(&m, 42, ds_match::MatchConfig::default());
        let g = fit(45.0);
        let f = frame(
            &crate::interp::Moment::at(&records, 500.0, &crate::interp::tests::physics()),
            &g,
        );
        let depths: Vec<f32> = f.items.iter().map(Item::depth).collect();
        assert!(depths.windows(2).all(|w| w[0] >= w[1]));
        assert!(f.items.len() >= 22, "{}", f.items.len());
    }

    /// The whole match, projected at every preset: every figure's feet
    /// inside the drawn rectangle.
    #[test]
    fn every_tick_of_a_match_projects_inside_the_board() {
        let m = ds_fixtures::match_fixture(42).expect("fixture");
        let (_, records) =
            ds_match::simulate_from_seed_recorded(&m, 42, ds_match::MatchConfig::default());
        for tilt in PRESETS {
            let g = fit(tilt);
            let inside =
                |s: &Screen| (-1.0..=1201.0).contains(&s.x) && (-1.0..=801.0).contains(&s.y);
            for t in (0..records.len()).step_by(197) {
                #[allow(clippy::cast_precision_loss)]
                let f = frame(
                    &crate::interp::Moment::at(
                        &records,
                        t as f64,
                        &crate::interp::tests::physics(),
                    ),
                    &g,
                );
                for i in &f.items {
                    if let Item::Figure(p) = i {
                        assert!(inside(&p.feet), "tilt {tilt} tick {t}: {:?}", p.feet);
                    }
                }
            }
        }
    }
}
