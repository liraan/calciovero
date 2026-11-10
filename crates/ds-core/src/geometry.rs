//! Deterministic geometry primitives: integer, no trig, no float in the
//! simulation path (D2, L0.11 §4.10).
//!
//! `distance_squared` was in `pcfm-match::perception`; it moved here when
//! the orchestrator (`ds-orchestrator`, ADR-041 §4) needed the same
//! primitive to read the scene. One source of truth (D5): a geometric
//! fact is a primitive of the world, not a rule of the engine.

/// The squared Euclidean distance between two points, in cm².
///
/// Integer, deterministic, symmetric. `i64` because a pitch is 10500 x
/// 6800 cm: the squared diagonal is ~1.5e8, well inside `i32`, but the
/// sum of two squares of that size is safer in `i64`.
#[must_use]
pub const fn distance_squared(a: (i32, i32), b: (i32, i32)) -> i64 {
    let dx = (a.0 - b.0) as i64;
    let dy = (a.1 - b.1) as i64;
    dx * dx + dy * dy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_squared_is_symmetric() {
        let a = (0, 0);
        let b = (3, 4);
        assert_eq!(distance_squared(a, b), 25);
        assert_eq!(distance_squared(b, a), 25);
    }

    #[test]
    fn distance_squared_of_a_point_is_zero() {
        assert_eq!(distance_squared((7, 9), (7, 9)), 0);
    }
}
