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
    /// The depth of the penalty area (16.5 m).
    pub penalty_area_depth_cm: i32,
    /// Half the width of the penalty area (40.32 m).
    pub penalty_area_half_width_cm: i32,
}

static LAW1: LazyLock<Law1> =
    LazyLock::new(|| serde_json::from_str(LAWS_JSON).expect("embedded laws file is valid JSON"));

/// Law 1's measures, parsed once.
#[must_use]
pub fn law1() -> Law1 {
    *LAW1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn law_1_is_read_from_the_laws() {
        let l = law1();
        assert_eq!(l.goal_half_width_cm, 366);
        assert_eq!(l.goal_height_cm, 244);
        assert_eq!(l.penalty_area_depth_cm, 1650);
        assert_eq!(l.penalty_area_half_width_cm, 2016);
    }
}
