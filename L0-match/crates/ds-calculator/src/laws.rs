//! Law 1's geometry, from its one source (`balance/laws.json`).
//!
//! **One responsibility:** the measures of the pitch the Laws fix (the goal,
//! the penalty area), read once, for every crate. Before (code audit C1),
//! the goal's width was in `laws.json`, `obstacles.json` and `thought.json`
//! and written by hand in four places of the code, the penalty area in
//! `laws.json` and `offball.json`.

use std::sync::LazyLock;

use serde::Deserialize;

const LAWS_JSON: &str = include_str!("../../../balance/laws.json");

/// The measures of Law 1 the match reads, cm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct Law1 {
    /// Half the distance between the posts (7.32 m).
    pub goal_half_width_cm: i32,
    /// The height of the crossbar (2.44 m).
    pub goal_height_cm: i32,
    /// The radius of a post (posts at most 12 cm wide).
    pub post_radius_cm: i32,
    /// The depth of the penalty area (16.5 m).
    pub penalty_area_depth_cm: i32,
    /// Half the width of the penalty area (40.32 m).
    pub penalty_area_half_width_cm: i32,
    /// The depth of the goal area, where a goal kick is taken (5.5 m).
    pub goal_area_depth_cm: i32,
    /// Half the width of the goal area (18.32 m).
    pub goal_area_half_width_cm: i32,
    /// The radius of the centre circle (9.15 m), the distance the
    /// opponents keep at a restart (Laws 8, 13).
    pub centre_circle_radius_cm: i32,
}

static LAW1: LazyLock<Law1> =
    LazyLock::new(|| serde_json::from_str(LAWS_JSON).expect("embedded laws file is valid JSON"));

/// The ball's radius, cm, whole (Law 2: a circumference of 68-70 cm).
#[allow(clippy::cast_possible_truncation)]
pub const BALL_RADIUS: i32 = ds_core::geometry::BALL_RADIUS_CM as i32;

/// Law 10: the whole ball between the posts and under the crossbar.
///
/// At the goal plane: `lateral` is from the centre of the goal, `z_low`
/// the ball's lowest point (the physics bounces it at 0). The one judge of
/// the goal mouth, for a goal and for a shot on target (SIM 203: before, the
/// shot's flag read the ball's centre, without its size).
#[must_use]
pub const fn in_the_goal_mouth(lateral: i32, z_low: i32, half_width: i32, height: i32) -> bool {
    lateral.abs() + BALL_RADIUS < half_width && z_low >= 0 && z_low + 2 * BALL_RADIUS < height
}

/// Law 1's measures, parsed once.
#[must_use]
pub fn law1() -> Law1 {
    *LAW1
}

#[cfg(test)]
mod tests {
    use super::*;

    /// SIM 203 (T8, Law 10): the whole ball, its side not on the post nor
    /// its top on the bar; the one judge of a goal and of a shot on target.
    #[test]
    fn the_whole_ball_is_in_the_goal_mouth_or_it_is_not() {
        let l = law1();
        let (h, b) = (l.goal_half_width_cm, l.goal_height_cm);
        assert!(in_the_goal_mouth(0, 0, h, b));
        assert!(!in_the_goal_mouth(h - BALL_RADIUS, 0, h, b));
        assert!(in_the_goal_mouth(h - BALL_RADIUS - 1, 0, h, b));
        assert!(!in_the_goal_mouth(0, b - 2 * BALL_RADIUS, h, b));
        assert!(!in_the_goal_mouth(0, -1, h, b));
    }

    #[test]
    fn law_1_is_read_from_the_laws() {
        let l = law1();
        assert_eq!(l.goal_half_width_cm, 366);
        assert_eq!(l.goal_height_cm, 244);
        assert_eq!(l.penalty_area_depth_cm, 1650);
        assert_eq!(l.penalty_area_half_width_cm, 2016);
    }
}
