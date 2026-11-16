//! The stadium and its city around the pitch (`DESIGN.md`, "La visuale
//! del 2.5D"): the stands with their crowd, the floodlights, and the town
//! beyond them under the sky.
//!
//! **One responsibility:** the backdrop's shapes in the pitch's space,
//! built once for a pitch's size. Drawing only: generic, the same for every
//! match, nothing from the simulation and nothing back to it. The sizes are
//! drawing sizes of an ordinary modern ground, not data of the game.
//!
//! The stand on the camera's side is not drawn when the camera is tilted:
//! it would stand between the eye and the pitch (the broadcast camera is
//! up in that stand).

#![allow(
    clippy::many_single_char_names,
    clippy::suboptimal_flops,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::too_many_lines,
    reason = "drawing geometry only, in small sizes: x, y, z, the sides of a box"
)]

use crate::scene::World;

/// From the touchlines to the first row, cm.
const SIDE_GAP_CM: f32 = 900.0;
/// From the goal lines to the first row, cm.
const END_GAP_CM: f32 = 1_100.0;
/// How deep a stand is, cm.
const STAND_DEPTH_CM: f32 = 2_400.0;
/// The height of its first row, cm (the wall over the pitch).
const STAND_FOOT_CM: f32 = 200.0;
/// The height of its back row, cm.
const STAND_TOP_CM: f32 = 1_500.0;
/// The roof's height over the back row, cm.
const ROOF_CM: f32 = 700.0;
/// The tiers, front to back.
const TIERS: usize = 20;
/// The seats' blocks along a stand, about one every this many cm.
const BLOCK_CM: f32 = 220.0;
/// The floodlights' height, cm.
const FLOODLIGHT_CM: f32 = 4_500.0;

/// What a shape is painted as.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Paint {
    /// The ground between the pitch and the stands.
    Track,
    /// A block of the crowd: its mix, a fixed number.
    Crowd(u32),
    /// The concrete of a stand: its walls and its back.
    Concrete,
    /// The roof over a stand.
    Roof,
    /// A floodlight's mast.
    Mast,
    /// A floodlight's lamps.
    Lamps,
    /// A building of the town, its shade by the face's side.
    Building(u8),
    /// A lit window.
    Window,
}

/// A flat shape in the pitch's space.
#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    /// Its corners, cm.
    pub corners: Vec<World>,
    /// How it is painted.
    pub paint: Paint,
    /// It belongs to the stand on the camera's side (`y < 0`).
    pub near: bool,
}

/// A fixed mix of the bits of `n` (a drawing's variety, not a die: the
/// same shape always looks the same).
#[must_use]
pub const fn mix(n: u32) -> u32 {
    let mut x = n.wrapping_mul(0x9E37_79B9) ^ 0x85EB_CA6B;
    x ^= x >> 15;
    x = x.wrapping_mul(0x2C1B_3C6D);
    x ^= x >> 12;
    x = x.wrapping_mul(0x297A_2D39);
    x ^ (x >> 15)
}

/// A point on the rectangle `lo..hi` at `s` of its perimeter, going round
/// from `(lo.0, lo.1)` along `x` first, and the outward normal there.
fn round(lo: (f32, f32), hi: (f32, f32), s: f32) -> ((f32, f32), (f32, f32)) {
    let (a, b) = (hi.0 - lo.0, hi.1 - lo.1);
    let p = 2.0f32.mul_add(a, 2.0 * b);
    let s = s.rem_euclid(p);
    if s < a {
        ((lo.0 + s, lo.1), (0.0, -1.0))
    } else if s < a + b {
        ((hi.0, lo.1 + s - a), (1.0, 0.0))
    } else if s < 2.0f32.mul_add(a, b) {
        ((hi.0 - (s - a - b), hi.1), (0.0, 1.0))
    } else {
        ((lo.0, hi.1 - (s - 2.0f32.mul_add(a, b))), (-1.0, 0.0))
    }
}

/// The stadium for a pitch of `pitch_cm`: the ground round it, the
/// stands with their crowd and roof, the floodlights.
#[must_use]
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn stadium(pitch_cm: (i32, i32)) -> Vec<Shape> {
    let (l, w) = (pitch_cm.0 as f32, pitch_cm.1 as f32);
    let lo = (-END_GAP_CM, -SIDE_GAP_CM);
    let hi = (l + END_GAP_CM, w + SIDE_GAP_CM);
    let mut out = vec![Shape {
        corners: vec![
            (lo.0, lo.1, 0.0),
            (hi.0, lo.1, 0.0),
            (hi.0, hi.1, 0.0),
            (lo.0, hi.1, 0.0),
        ],
        paint: Paint::Track,
        near: false,
    }];
    // The bowl: round the rectangle, block by block, tier by tier. Each
    // side is cut into its own blocks so that a corner is a block of its
    // own (the stand turns there).
    let sides = [hi.0 - lo.0, hi.1 - lo.1, hi.0 - lo.0, hi.1 - lo.1];
    let mut start = 0.0_f32;
    let rise = |k: usize| {
        let t = k as f32 / TIERS as f32;
        (
            t * STAND_DEPTH_CM,
            (STAND_TOP_CM - STAND_FOOT_CM).mul_add(t, STAND_FOOT_CM),
        )
    };
    for (side, len) in sides.iter().enumerate() {
        let blocks = (len / BLOCK_CM).ceil().max(1.0) as usize;
        for b in 0..blocks {
            let s0 = (b as f32 / blocks as f32).mul_add(*len, start);
            let s1 = ((b + 1) as f32 / blocks as f32).mul_add(*len, start);
            let (p0, n) = round(lo, hi, s0 + 1.0);
            let (p1, _) = round(lo, hi, s1 - 1.0);
            let at = |p: (f32, f32), (out_cm, z): (f32, f32)| {
                (n.0.mul_add(out_cm, p.0), n.1.mul_add(out_cm, p.1), z)
            };
            let near = side == 0;
            // The wall in front of the first row.
            out.push(Shape {
                corners: vec![
                    (p0.0, p0.1, 0.0),
                    (p1.0, p1.1, 0.0),
                    at(p1, rise(0)),
                    at(p0, rise(0)),
                ],
                paint: Paint::Concrete,
                near,
            });
            for k in 0..TIERS {
                let (a, c) = (rise(k), rise(k + 1));
                out.push(Shape {
                    corners: vec![at(p0, a), at(p1, a), at(p1, c), at(p0, c)],
                    paint: Paint::Crowd(mix(((side * 1_000 + b) * 64 + k) as u32)),
                    near,
                });
            }
            // The back wall and the roof over the upper tiers.
            let top = rise(TIERS);
            let roof = (top.0, top.1 + ROOF_CM);
            out.push(Shape {
                corners: vec![at(p0, top), at(p1, top), at(p1, roof), at(p0, roof)],
                paint: Paint::Concrete,
                near,
            });
            let lip = (STAND_DEPTH_CM * 0.45, ROOF_CM.mul_add(0.8, top.1));
            out.push(Shape {
                corners: vec![at(p0, roof), at(p1, roof), at(p1, lip), at(p0, lip)],
                paint: Paint::Roof,
                near,
            });
        }
        start += len;
    }
    // The corners, where the stand turns: a wedge between the two sides.
    for (corner, n1, n2) in [
        ((lo.0, lo.1), (0.0, -1.0), (-1.0, 0.0)),
        ((hi.0, lo.1), (1.0, 0.0), (0.0, -1.0)),
        ((hi.0, hi.1), (0.0, 1.0), (1.0, 0.0)),
        ((lo.0, hi.1), (-1.0, 0.0), (0.0, 1.0)),
    ] {
        let at = |n: (f32, f32), (out_cm, z): (f32, f32)| {
            (
                n.0.mul_add(out_cm, corner.0),
                n.1.mul_add(out_cm, corner.1),
                z,
            )
        };
        let near = corner.1 < 0.0;
        for k in 0..TIERS {
            let (a, c) = (rise(k), rise(k + 1));
            out.push(Shape {
                corners: vec![at(n1, a), at(n2, a), at(n2, c), at(n1, c)],
                paint: Paint::Crowd(mix((corner.0 + corner.1) as u32 ^ k as u32)),
                near,
            });
        }
        let top = rise(TIERS);
        let roof = (top.0, top.1 + ROOF_CM);
        out.push(Shape {
            corners: vec![at(n1, top), at(n2, top), at(n2, roof), at(n1, roof)],
            paint: Paint::Concrete,
            near,
        });
    }
    // The floodlights at the four corners, behind the stands.
    let back = STAND_DEPTH_CM + 600.0;
    for (cx, cy, sx, sy) in [
        (lo.0, lo.1, -1.0, -1.0),
        (hi.0, lo.1, 1.0, -1.0),
        (hi.0, hi.1, 1.0, 1.0),
        (lo.0, hi.1, -1.0, 1.0),
    ] {
        let (x, y) = (sx * back * 0.7 + cx, sy * back * 0.7 + cy);
        let near = sy < 0.0;
        out.push(Shape {
            corners: vec![
                (x - 60.0, y, 0.0),
                (x + 60.0, y, 0.0),
                (x + 40.0, y, FLOODLIGHT_CM),
                (x - 40.0, y, FLOODLIGHT_CM),
            ],
            paint: Paint::Mast,
            near,
        });
        // The lamps face the pitch.
        out.push(Shape {
            corners: vec![
                (x - 450.0, y, FLOODLIGHT_CM),
                (x + 450.0, y, FLOODLIGHT_CM),
                (x + 450.0, y - sy * 150.0, FLOODLIGHT_CM + 600.0),
                (x - 450.0, y - sy * 150.0, FLOODLIGHT_CM + 600.0),
            ],
            paint: Paint::Lamps,
            near,
        });
    }
    out
}

/// The town round the ground: blocks of buildings, lower near the stadium
/// and taller farther off, on the three sides away from the camera.
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn city(pitch_cm: (i32, i32)) -> Vec<Shape> {
    let (l, w) = (pitch_cm.0 as f32, pitch_cm.1 as f32);
    let centre = (l / 2.0, w / 2.0);
    let mut out = Vec::new();
    // Rings of blocks at 14 m street spacing, from 110 m to 600 m.
    let mut n = 0_u32;
    for ring in 0..9_u32 {
        let r = (ring as f32).mul_add(5_500.0, 11_000.0);
        let count = 16 + ring * 6;
        for k in 0..count {
            n += 1;
            let h = mix(n);
            let a = (k as f32 + (h % 100) as f32 / 100.0) / count as f32;
            let angle = a * std::f32::consts::TAU;
            #[allow(
                clippy::disallowed_methods,
                reason = "drawing only: the view never feeds the simulation"
            )]
            let (s, c) = angle.sin_cos();
            let (x, y) = (r.mul_add(c, centre.0), (r * 0.8).mul_add(s, centre.1));
            if y < centre.1 - 2_000.0 {
                continue; // behind the camera
            }
            let size = 1_800.0 + (h >> 8 & 0x7ff) as f32;
            let depth = 1_500.0 + (h >> 4 & 0x7ff) as f32;
            let height = (ring as f32).mul_add(900.0, 1_200.0) + (h >> 16 & 0xfff) as f32 * 1.4;
            building(&mut out, (x, y), (size, depth), height, h);
        }
    }
    out
}

/// A box of a building: its four walls and its roof, and some lit
/// windows on the walls.
#[allow(clippy::cast_precision_loss)]
fn building(out: &mut Vec<Shape>, (x, y): (f32, f32), (a, b): (f32, f32), h: f32, seed: u32) {
    let (x0, x1, y0, y1) = (x - a / 2.0, x + a / 2.0, y - b / 2.0, y + b / 2.0);
    let walls = [
        ((x0, y0), (x1, y0), 0_u8),
        ((x1, y0), (x1, y1), 1),
        ((x1, y1), (x0, y1), 2),
        ((x0, y1), (x0, y0), 1),
    ];
    for (i, ((ax, ay), (bx, by), shade)) in walls.into_iter().enumerate() {
        out.push(Shape {
            corners: vec![(ax, ay, 0.0), (bx, by, 0.0), (bx, by, h), (ax, ay, h)],
            paint: Paint::Building(shade),
            near: false,
        });
        // Windows: floors of 3 m, a window every 4 m, some lit.
        let floors = (h / 300.0) as u32;
        let cols = ((bx - ax).abs().max((by - ay).abs()) / 400.0) as u32;
        for f in 1..floors {
            for k in 0..cols {
                let lit = mix(seed ^ (i as u32) << 20 ^ f << 10 ^ k);
                if lit % 5 != 0 {
                    continue;
                }
                let t0 = (k as f32 + 0.3) / cols as f32;
                let t1 = (k as f32 + 0.7) / cols as f32;
                let z0 = f as f32 * 300.0 + 80.0;
                let z1 = z0 + 140.0;
                let p = |t: f32, z: f32| ((bx - ax).mul_add(t, ax), (by - ay).mul_add(t, ay), z);
                out.push(Shape {
                    corners: vec![p(t0, z0), p(t1, z0), p(t1, z1), p(t0, z1)],
                    paint: Paint::Window,
                    near: false,
                });
            }
        }
    }
    out.push(Shape {
        corners: vec![(x0, y0, h), (x1, y0, h), (x1, y1, h), (x0, y1, h)],
        paint: Paint::Building(3),
        near: false,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The stands surround the pitch and never step on it.
    #[test]
    fn the_stands_stay_off_the_pitch() {
        let (l, w) = (10_500.0, 6_800.0);
        for s in stadium((10_500, 6_800)) {
            if s.paint == Paint::Track {
                continue;
            }
            for &(x, y, _) in &s.corners {
                let inside = (0.0..=l).contains(&x) && (0.0..=w).contains(&y);
                assert!(!inside, "{:?} on the pitch at ({x}, {y})", s.paint);
            }
        }
    }

    /// The backdrop is the same every time: no die in the drawing.
    #[test]
    fn the_backdrop_is_always_the_same() {
        assert_eq!(stadium((10_500, 6_800)), stadium((10_500, 6_800)));
        assert_eq!(city((10_500, 6_800)), city((10_500, 6_800)));
        assert!(city((10_500, 6_800)).len() > 100);
    }
}
