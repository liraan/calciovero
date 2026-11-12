//! Stoppage-time balance values (ADR-026).
//!
//! The contributions live in `L0-match/balance/stoppage_time.json` (doctrine §5: no
//! hardcoded balance values). The file is embedded at compile time with
//! `include_str!`: the engine performs no I/O, and the simulation stays
//! deterministic. Editing the file requires a rebuild, which is correct: a
//! balance change is a `SIM_VERSION` bump.

use ds_stats::EventType;
use serde::Deserialize;

/// The embedded balance file.
const STOPPAGE_TIME_JSON: &str = include_str!("../../../balance/stoppage_time.json");

/// Stoppage-time contributions, in seconds, per event type.
#[derive(Debug, Clone, Deserialize)]
pub struct StoppageTime {
    /// Base contribution, in seconds (added once per half).
    pub base: u32,
    /// Per-goal contribution, in seconds.
    pub goal: u32,
    /// Per-foul contribution, in seconds.
    pub foul: u32,
    /// Per-duel contribution, in seconds.
    pub duel: u32,
}

impl Default for StoppageTime {
    fn default() -> Self {
        // Parsing the embedded literal cannot fail: it is a compile-time
        // constant validated by the test below.
        serde_json::from_str(STOPPAGE_TIME_JSON).expect("embedded balance file is valid JSON")
    }
}

impl StoppageTime {
    /// Loads the embedded balance table.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid. The test
    /// `embedded_balance_is_valid` catches this at test time.
    #[must_use]
    pub fn load() -> Self {
        Self::default()
    }

    /// Returns the contribution of an event type, in seconds.
    #[must_use]
    pub const fn contribution(&self, kind: EventType) -> u32 {
        match kind {
            EventType::Goal => self.goal,
            EventType::Foul => self.foul,
            EventType::Duel => self.duel,
            _ => 0,
        }
    }

    /// Sums the contributions of a slice of event types, plus the base.
    #[must_use]
    pub fn total(&self, kinds: impl IntoIterator<Item = EventType>) -> u32 {
        let mut total = self.base;
        for kind in kinds {
            total = total.saturating_add(self.contribution(kind));
        }
        total
    }
}

/// Stoppage time for a slice of events, in seconds.
///
/// The base is added once. Each event adds its contribution (0 for types
/// not in the table). No cap: the fixed point loop in `orchestrator` stops
/// when the contribution is zero (ADR-026 §2).
#[must_use]
pub fn stoppage_for(events: &[ds_stats::MatchEvent], table: &StoppageTime) -> u32 {
    let mut total = table.base;
    for event in events {
        total = total.saturating_add(table.contribution(event.event_type));
    }
    total
}

/// The embedded influence weights file (`03-entities-minimal` §2.9).
const INFLUENCE_JSON: &str = include_str!("../../../balance/influence.json");

/// Influence weights as they appear in `L0-match/balance/influence.json`.
///
/// Converted to [`ds_orchestrator::InfluenceWeights`] at load time: `pcfm-hidden`
/// does not depend on `serde`, so the JSON shape lives here.
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct InfluenceWeightsJson {
    /// Weight of the space term.
    pub w_space: i32,
    /// Weight of the skill term.
    pub w_skill: i32,
    /// Weight of the fatigue term.
    pub w_cost: i32,
    /// Weight of the redundancy term.
    pub w_red: i32,
    /// Distance (cm) at which the space term reaches zero.
    pub space_threshold_cm: i32,
}

impl Default for InfluenceWeightsJson {
    fn default() -> Self {
        serde_json::from_str(INFLUENCE_JSON).expect("embedded influence file is valid JSON")
    }
}

impl InfluenceWeightsJson {
    /// Loads the embedded influence weights.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid. The test
    /// `embedded_influence_is_valid` catches this at test time.
    #[must_use]
    pub fn load() -> Self {
        Self::default()
    }

    /// Converts to the `pcfm-hidden` type used by `resolve_influence`.
    #[must_use]
    pub const fn to_weights(self) -> ds_orchestrator::InfluenceWeights {
        ds_orchestrator::InfluenceWeights {
            w_space: self.w_space,
            w_skill: self.w_skill,
            w_cost: self.w_cost,
            w_red: self.w_red,
            space_threshold_cm: self.space_threshold_cm,
        }
    }
}
/// The embedded scanning parameters file (ADR-034).
const SCANNING_JSON: &str = include_str!("../../../balance/scanning.json");

/// Scanning parameters (ADR-034): latency N, marking radius, salience
/// weights. Arbitrary and declared until calibration (D6).
///
/// The five relational candidates live in `perception::scan`. Candidates
/// 4 (receiving space) and 5 (line) are not implemented: the per-agent
/// state they need does not exist yet (`Removed-by: OI-71`).
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct ScanningBalance {
    /// Latency of the gaze turn, in ticks (N).
    pub scan_interval_ticks: u64,
    /// Radius within which an opponent counts as "on the ball", in cm.
    pub scan_reach_cm: i32,
    /// Salience weight of the ball.
    pub weight_ball: i32,
    /// Salience weight of the opponent nearest the ball.
    pub weight_opponent_on_ball: i32,
    /// Salience weight of the opponent nearest the player (the mark).
    pub weight_mark: i32,
    /// Radius within which the mark beats the ball, in cm.
    pub mark_radius_cm: i32,
}

impl Default for ScanningBalance {
    fn default() -> Self {
        serde_json::from_str(SCANNING_JSON).expect("embedded scanning file is valid JSON")
    }
}

impl ScanningBalance {
    /// Loads the embedded scanning parameters.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid. The test
    /// `embedded_scanning_is_valid` catches this at test time.
    #[must_use]
    pub fn load() -> Self {
        Self::default()
    }
}

/// The embedded keeper positioning balance file (OI-81-step-move).
const KEEPER_JSON: &str = include_str!("../../../balance/keeper.json");

/// Keeper positioning parameters as they appear in `L0-match/balance/keeper.json`.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct KeeperBalance {
    /// Beyond this distance ball-goal, the keeper stays on the line (cm).
    pub positioning_range_cm: i32,
    /// How far the keeper advances when the ball is on the line (cm).
    pub advance_cm: i32,
}

impl Default for KeeperBalance {
    fn default() -> Self {
        serde_json::from_str(KEEPER_JSON).expect("embedded keeper file is valid JSON")
    }
}

impl KeeperBalance {
    /// Loads the embedded keeper positioning parameters.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid. The test
    /// `embedded_keeper_is_valid` catches this at test time.
    #[must_use]
    pub fn load() -> Self {
        Self::default()
    }
}

/// The embedded reception balance file (REC).
const RECEPTION_JSON: &str = include_str!("../../../balance/reception.json");

/// Reception parameters as they appear in `L0-match/balance/reception.json`.
///
/// Converted to [`ds_orchestrator::ReceptionParams`] at load time: `pcfm-hidden`
/// does not depend on `serde`, so the JSON shape lives here.
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct ReceptionBalance {
    /// Base flight speed, cm/tick.
    pub speed_base_cm_per_tick: i32,
    /// Passing bonus on flight speed, cm/tick.
    pub speed_passing_bonus_cm_per_tick: i32,
    /// Freedom reference distance, cm.
    pub freedom_ref_cm: i64,
    /// Max control bonus from freedom, milli.
    pub freedom_bonus_max_milli: i64,
    /// Distance reference for the control penalty, cm.
    pub distance_ref_cm: i64,
    /// Max control penalty from distance, milli.
    pub distance_penalty_max_milli: i64,
    /// Control threshold, milli.
    pub control_threshold_milli: i64,
    /// A loose ball within this distance of a player is his (ADR-041 §5).
    pub loose_control_radius_cm: i32,
}

impl Default for ReceptionBalance {
    fn default() -> Self {
        serde_json::from_str(RECEPTION_JSON).expect("embedded reception file is valid JSON")
    }
}

impl ReceptionBalance {
    /// Loads the embedded reception parameters.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid. The test
    /// `embedded_reception_is_valid` catches this at test time.
    #[must_use]
    pub fn load() -> Self {
        Self::default()
    }

    /// Converts to the `pcfm-hidden` type used by `resolve_reception`.
    #[must_use]
    pub const fn to_params(self) -> ds_orchestrator::ReceptionParams {
        ds_orchestrator::ReceptionParams {
            speed_base_cm_per_tick: self.speed_base_cm_per_tick,
            speed_passing_bonus_cm_per_tick: self.speed_passing_bonus_cm_per_tick,
            freedom_ref_cm: self.freedom_ref_cm,
            freedom_bonus_max_milli: self.freedom_bonus_max_milli,
            distance_ref_cm: self.distance_ref_cm,
            distance_penalty_max_milli: self.distance_penalty_max_milli,
            control_threshold_milli: self.control_threshold_milli,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_balance_is_valid() {
        let table = StoppageTime::load();
        assert_eq!(table.base, 0);
        assert_eq!(table.goal, 60);
        assert_eq!(table.foul, 20);
        assert_eq!(table.duel, 10);
    }

    #[test]
    fn contribution_maps_event_types() {
        let table = StoppageTime::load();
        assert_eq!(table.contribution(EventType::Goal), 60);
        assert_eq!(table.contribution(EventType::Foul), 20);
        assert_eq!(table.contribution(EventType::Duel), 10);
        assert_eq!(table.contribution(EventType::Shot), 0);
        assert_eq!(table.contribution(EventType::KickOff), 0);
    }

    #[test]
    fn total_sums_base_and_events() {
        let table = StoppageTime::load();
        let total = table.total([EventType::Goal, EventType::Goal, EventType::Foul]);
        assert_eq!(total, 140); // 60 + 60 + 20
    }

    #[test]
    fn stoppage_for_sums_events() {
        let table = StoppageTime::load();
        let events = vec![
            make_event(EventType::Goal),
            make_event(EventType::Goal),
            make_event(EventType::Foul),
        ];
        assert_eq!(stoppage_for(&events, &table), 140);
    }

    #[test]
    fn embedded_influence_is_valid() {
        let w = InfluenceWeightsJson::load();
        assert_eq!(w.w_space, 100);
        assert_eq!(w.w_skill, 40);
        assert_eq!(w.w_cost, 20);
        assert_eq!(w.w_red, 50);
        assert_eq!(w.space_threshold_cm, 2000);
        let hidden = w.to_weights();
        assert_eq!(hidden.w_space, w.w_space);
    }

    #[test]
    fn embedded_keeper_is_valid() {
        let b = KeeperBalance::load();
        assert_eq!(b.positioning_range_cm, 3000);
        assert_eq!(b.advance_cm, 400);
    }

    #[test]
    fn embedded_obstacles_is_valid() {
        let b = ObstacleBalance::load();
        assert_eq!(b.goal_half_width_cm, 366);
        assert_eq!(b.goal_height_cm, 244);
        assert!(b.elasticity_den != 0);
        assert!(b.post_radius_cm > 0);
    }

    #[test]
    fn embedded_scanning_is_valid() {
        let b = ScanningBalance::load();
        assert_eq!(b.scan_interval_ticks, 5);
        assert_eq!(b.scan_reach_cm, 3000);
        assert_eq!(b.weight_ball, 100);
        assert_eq!(b.weight_opponent_on_ball, 80);
        assert_eq!(b.weight_mark, 60);
        assert_eq!(b.mark_radius_cm, 800);
    }
    fn make_event(kind: EventType) -> ds_stats::MatchEvent {
        ds_stats::MatchEvent {
            event_id: ds_core::ids::EventId::new(0, 0),
            match_id: ds_core::ids::MatchId::new(1, 0),
            actor: None,
            half: ds_stats::Half::First,
            timestamp_minute: 0,
            timestamp_second: 0,
            minutes_played_cumulative: 0,
            event_type: kind,
            fields: serde_json::json!({}),
            created_at: ds_core::time::GameInstant::from_raw(0),
        }
    }
}

/// The embedded obstacles balance file (ADR-048).
const OBSTACLES_JSON: &str = include_str!("../../../balance/obstacles.json");

/// Goal geometry and obstacle reflection parameters (ADR-048 §1).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ObstacleBalance {
    /// Half the distance between the posts, in cm (Law 1: 366).
    pub goal_half_width_cm: i32,
    /// Crossbar height, in cm (Law 1: 244).
    pub goal_height_cm: i32,
    /// Post radius, in cm (prototype).
    pub post_radius_cm: i32,
    /// Crossbar thickness, in cm (prototype).
    pub crossbar_thickness_cm: i32,
    /// Elasticity numerator (prototype 0.8).
    pub elasticity_num: i32,
    /// Elasticity denominator.
    pub elasticity_den: i32,
}

impl Default for ObstacleBalance {
    fn default() -> Self {
        serde_json::from_str(OBSTACLES_JSON).expect("embedded obstacles file is valid JSON")
    }
}

impl ObstacleBalance {
    /// Loads the embedded obstacles parameters.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid. The test
    /// `embedded_obstacles_is_valid` catches this at test time.
    #[must_use]
    pub fn load() -> Self {
        Self::default()
    }

    /// The reflection on the x axis (a post's face).
    #[must_use]
    pub const fn post_reflection(&self) -> ds_calculator::obstacle::ObstacleReflection {
        ds_calculator::obstacle::ObstacleReflection {
            axis: 0,
            elasticity_num: self.elasticity_num,
            elasticity_den: self.elasticity_den,
        }
    }

    /// The reflections of the crossbar: x (its face, the ball comes back
    /// toward the pitch) and z (the ball bounces down or up off the bar).
    /// A z-only reflection would leave the ball travelling into the goal
    /// past the plane, where no crossing is checked any more: no goal, no
    /// out, a ball lost behind the line.
    #[must_use]
    pub const fn crossbar_reflections(&self) -> [ds_calculator::obstacle::ObstacleReflection; 2] {
        [
            ds_calculator::obstacle::ObstacleReflection {
                axis: 0,
                elasticity_num: self.elasticity_num,
                elasticity_den: self.elasticity_den,
            },
            ds_calculator::obstacle::ObstacleReflection {
                axis: 2,
                elasticity_num: self.elasticity_num,
                elasticity_den: self.elasticity_den,
            },
        ]
    }
}

/// The embedded ball physics (ADR-048 §4).
const BALL_JSON: &str = include_str!("../../../balance/ball.json");

/// The ball's declared physical properties (ADR-048 §4), per tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BallBalance {
    /// Gravity, cm/tick² (MD-18, derived from g).
    pub gravity_cm_per_tick2: i16,
    /// Horizontal friction in the air, thousandths kept per tick.
    pub air_friction_milli: i32,
    /// Horizontal friction on the ground, thousandths kept per tick.
    pub ground_friction_milli: i32,
    /// A grounded ball slower than this comes to rest, cm/tick.
    pub stop_threshold_cm_per_tick: i16,
}

impl Default for BallBalance {
    fn default() -> Self {
        serde_json::from_str(BALL_JSON).expect("embedded ball file is valid JSON")
    }
}

impl BallBalance {
    /// Loads the embedded ball physics.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid; `embedded_ball_is_valid`
    /// catches it at test time.
    #[must_use]
    pub fn load() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod ball_tests {
    use super::BallBalance;

    #[test]
    fn embedded_ball_is_valid() {
        let b = BallBalance::load();
        assert_eq!(b.gravity_cm_per_tick2, 10);
        assert!(
            b.ground_friction_milli < b.air_friction_milli,
            "grass slows more than air"
        );
        assert!((0..1000).contains(&b.ground_friction_milli));
    }
}

/// The embedded carrying parameters (IDEA-DESIGN-010).
const CARRY_JSON: &str = include_str!("../../../balance/carry.json");

/// How far a carrier who chose to carry the ball takes it (IDEA-DESIGN-010).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct CarryBalance {
    /// Distance towards the opponent goal, cm.
    pub carry_distance_cm: i32,
    /// The carrier stops this far from the goal and chooses, cm.
    pub carry_stop_from_goal_cm: i32,
}

impl Default for CarryBalance {
    fn default() -> Self {
        serde_json::from_str(CARRY_JSON).expect("embedded carry file is valid JSON")
    }
}

impl CarryBalance {
    /// Loads the embedded carrying parameters.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid; `embedded_carry_is_valid`
    /// catches it at test time.
    #[must_use]
    pub fn load() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod carry_tests {
    #[test]
    fn embedded_carry_is_valid() {
        assert!(super::CarryBalance::load().carry_distance_cm > 0);
    }
}

/// The embedded team block (VISION §3, the role as a default intention).
const BLOCK_JSON: &str = include_str!("../../../balance/block.json");

/// The team block: where each outfield player stands without instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct BlockBalance {
    /// Depth of the block with the ball, cm.
    pub depth_in_possession_cm: i32,
    /// Depth of the block without the ball, cm.
    pub depth_out_of_possession_cm: i32,
    /// Width of the block with the ball, cm.
    pub width_in_possession_cm: i32,
    /// Width of the block without the ball, cm.
    pub width_out_of_possession_cm: i32,
    /// The defensive line sits this far behind the ball, with the ball, cm.
    pub line_behind_ball_in_possession_cm: i32,
    /// The defensive line sits this far behind the ball, without it, cm.
    pub line_behind_ball_out_of_possession_cm: i32,
    /// The closest the defensive line comes to its own goal line, cm.
    pub line_min_cm: i32,
    /// The farthest the defensive line goes from its own goal, with the ball, cm.
    pub line_max_in_possession_cm: i32,
    /// The farthest the defensive line goes from its own goal, without it, cm.
    pub line_max_out_of_possession_cm: i32,
    /// How much the block shifts towards the ball's side, thousandths.
    pub ball_side_shift_milli: i32,
}

impl Default for BlockBalance {
    fn default() -> Self {
        serde_json::from_str(BLOCK_JSON).expect("embedded block file is valid JSON")
    }
}

impl BlockBalance {
    /// Loads the embedded team block.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid; `embedded_block_is_valid`
    /// catches it at test time.
    #[must_use]
    pub fn load() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod block_tests {
    #[test]
    fn embedded_block_is_valid() {
        let b = super::BlockBalance::load();
        assert!(
            b.depth_in_possession_cm > b.depth_out_of_possession_cm,
            "it stretches with the ball"
        );
        assert!(
            b.width_in_possession_cm > b.width_out_of_possession_cm,
            "and widens"
        );
        assert!(b.line_min_cm < b.line_max_out_of_possession_cm);
    }
}
