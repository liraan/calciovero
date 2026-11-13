//! Inputs to the `resolve_*` functions (Plane B).
//!
//! These structs are constructed by the engine and passed to
//! `pcfm-hidden`. They carry only Plane B data (positions, ids); latents
//! are read inside the crate.

use ds_core::ids::PlayerId;

/// The type of a duel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DuelType {
    /// Ground duel (tackle, dribble).
    Ground,
    /// Aerial duel (header).
    Aerial,
    /// Contest with the goalkeeper (cross, shot under pressure).
    WithGoalkeeper,
}

/// The context of a duel resolution.
#[derive(Debug, Clone, Copy)]
pub struct DuelContext {
    /// The attacker.
    pub attacker: PlayerId,
    /// The defender.
    pub defender: PlayerId,
    /// The type of duel.
    pub duel_type: DuelType,
    /// How the defender went in (ADR-042 §2, amended).
    pub tackle: ds_calculator::duel::TackleKind,
    /// He came from behind the carrier (geometry, ADR-042 §4).
    pub from_behind: bool,
    /// His speed when he arrived, cm/tick.
    pub approach_cm_per_tick: i32,
    /// What the carrier imagines is at stake (ADR-052 §3), in units of
    /// keeping the ball: the space he keeps if he shields, the space past
    /// the man if he beats him, and what the opponents would have where
    /// he loses it.
    pub stakes: (f64, f64, f64),
}

/// The context of a shot resolution.
#[derive(Debug, Clone, Copy)]
pub struct ShotContext {
    /// The shooter.
    pub shooter: PlayerId,
    /// Where the shot starts (the ball), in centimetres, 3D.
    pub from: (i32, i32, i32),
    /// The goal centre, in centimetres, 3D.
    pub to: (i32, i32, i32),
    /// A `0..=100` value the caller computes from the situation
    /// (opponents near the shooter, distance to goal). Declared model
    /// choice: the die widens its deviation under pressure. Not a latent.
    pub difficulty: i32,
    /// The lateral component of the aim on the goal plane, cm (ADR-046 §1,
    /// ADR-047): the actor's choice, not the engine's optimum. The engine
    /// applies it in `resolve_shot` as the direction.
    pub aim_y: f64,
    /// The force of the shot, 0..=1 (ADR-046 §1): strong or placed. The
    /// actor's choice; the engine maps it to speed and to the band.
    pub power: f64,
    /// The contact height, 0.0 low / 1.0 high (ADR-046 §1, §5): the
    /// actor's choice, not the engine's. `0.0` in the first cantiere
    /// (`Removed-by: ADR-046 §5`, the lob needs the flight's time).
    pub contact_height: f64,
    /// The shooter's lateral speed when he strikes (cm/tick, across the
    /// pitch): a body moving one way drags the ball that way.
    pub momentum_y: i32,
    /// Gravity, cm/tick² (`ball.json`): the shooter strikes the ball so
    /// that it reaches the height he aims at.
    pub gravity_cm_per_tick2: i32,
}

/// The context of a pass resolution.
#[derive(Debug, Clone, Copy)]
pub struct PassContext {
    /// The passer.
    pub passer: PlayerId,
    /// Where the pass starts (the ball), in centimetres, 3D.
    pub from: (i32, i32, i32),
    /// The receiver position, in centimetres, 3D.
    pub to: (i32, i32, i32),
    /// The receiver freedom at launch (cm, distance to the nearest
    /// opponent), computed once in `choose_action` and carried here
    /// (D7, one source of truth).
    pub receiver_freedom_cm: i64,
    /// The ball's ground friction, thousandths per tick: the passer weighs
    /// the pass so that it reaches the receiver still rolling.
    pub ground_friction_milli: i32,
    /// The centre of the pitch, cm: the passer's body is open to the play,
    /// and a pass that drifts drifts that way.
    pub pitch_centre: (i32, i32),
    /// A lofted pass (over the opponents), and gravity for its flight,
    /// cm/tick².
    pub lofted: Option<i32>,
}

/// The context of a reception (REC): the receiver and the geometry of
/// the arrival. The latents (`first_touch`) are read inside the crate.
#[derive(Debug, Clone, Copy)]
pub struct ReceptionContext {
    /// The receiver.
    pub receiver: PlayerId,
    /// Distance covered by the pass, cm.
    pub distance_cm: i64,
    /// Receiver freedom at launch (cm, distance to nearest opponent).
    pub freedom_cm: i64,
}

/// The context of a save resolution.
#[derive(Debug, Clone, Copy)]
pub struct SaveContext {
    /// The goalkeeper.
    pub goalkeeper: PlayerId,
    /// The point where the ball crossed the goal plane, in centimetres, 3D.
    /// The engine computes it from the trajectory; used by the caller for
    /// the `Save` event payload, not by the formula.
    pub point: (i32, i32, i32),
    /// Distance from the centre of the keeper's body to the crossing point,
    /// in the goal plane (across and up), in centimetres.
    pub offset_cm: f64,
    /// The goalkeeper's standing reach (half his height), in cm, from the
    /// centre of his body; the dive adds to it.
    pub reach_cm: f64,
    /// Ticks from the strike to the ball reaching the goal plane: the time
    /// the keeper has to react and dive.
    pub flight_ticks: f64,
    /// The ball's speed at the goal plane, cm/tick.
    pub ball_speed_cm_per_tick: f64,
}
/// The context of a movement resolution (F.7).
///
/// The player, and the Plane B data the movement formula reads: height
/// and weight from `Player` (registry data), fatigue from `MatchState`
/// (state). The situation is passed by the orchestrator; the calculator
/// does not know where it lives.
#[derive(Debug, Clone, Copy)]
pub struct MovementContext {
    /// The moving player.
    pub player: PlayerId,
    /// Height, in cm (Plane B, registry data).
    pub height_cm: f64,
    /// Weight, in kg (Plane B, registry data).
    pub weight_kg: f64,
    /// Fatigue, 0.0–1.0 (Plane B, state).
    pub fatigue: f64,
}

/// The latent attribute used as modulator by an influence query.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SkillKind {
    /// Tackling.
    Tackling,
    /// Passing.
    Passing,
    /// Finishing.
    Finishing,
    /// Aerial.
    Aerial,
    /// Goalkeeping.
    Goalkeeping,
}

/// One agent as seen by an influence query.
///
/// Position in centimetres, fatigue in 0..=1000 (fixed-point, x1000).
/// No latent: the latent is read inside the crate.
#[derive(Debug, Clone, Copy)]
pub struct InfluenceAgent {
    /// The player.
    pub id: PlayerId,
    /// Position in centimetres.
    pub position: (i32, i32),
    /// Fatigue, fixed-point x1000, 0..=1000.
    pub fatigue_milli: i32,
}

/// The context of an influence query.
///
/// Plane B data only: positions and fatigue are observed. The skill term is
/// read inside the crate from the latents; the engine never sees it.
#[derive(Debug, Clone)]
pub struct InfluenceContext {
    /// The point being queried.
    pub point: (i32, i32),
    /// The agents to rank.
    pub agents: Vec<InfluenceAgent>,
    /// Which latent modulates the skill term.
    pub skill: SkillKind,
}

/// Weights and thresholds for the influence formula.
///
/// Prototype values, not calibrated. The form is doctrine; the numbers are
/// calibration (see `L0-match/balance/influence.json`).
#[derive(Debug, Clone, Copy)]
pub struct InfluenceWeights {
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

/// Parameters and thresholds for the reception formula (REC).
///
/// Prototype values, not calibrated (see `L0-match/balance/reception.json`).
#[derive(Debug, Clone, Copy)]
pub struct ReceptionParams {
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
}

/// The context of a latency resolution (IDEA-DESIGN-011, R.1-latency).
///
/// Only the actor: the latency formula reads `decision_making` (Plane A)
/// and nothing else. Fatigue and pressure are declared terms
/// (`Removed-by: OI-70` for fatigue, no producer for pressure today): they
/// enter the formula when their producer exists, not as an unused field
/// here (D3: no structure without content).
#[derive(Debug, Clone, Copy)]
pub struct LatencyContext {
    /// The actor whose pace is resolved.
    pub player: PlayerId,
    /// Crowding around the actor, 0.0-1.0 (scene term, ADR-044 §6).
    pub crowding: f64,
    /// Nearest threat distance, 0.0-1.0 (scene term, ADR-044 §6).
    pub nearest_threat: f64,
    /// His fatigue, 0.0-1.0 (Plane B): concentration dims the decision
    /// as he tires (plan L0-CLOSE 10b).
    pub fatigue: f64,
}
