//! Spatial state for the positional engine (ADR-019, slice A).
//!
//! The position is **match state**, not player identity: it lives in
//! `ds-match`, not in `ds-model`. See
//! `L0-match/design/L0-POSITIONAL-SLICE.md` §3.1.
//!
//! Coordinates are integer centimetres (ADR-019 D1: determinism). The pitch
//! is 10500 × 6800 cm by default. The origin is the home team's goal line,
//! bottom-left: `x` grows toward the away goal, `y` toward the far touchline.
//!
//! # Classification of `AgentState` (dated note, 2026-11-05)
//!
//! `position`, `velocity` and `heading` are **match state**, not player
//! data: they exist during the 90 minutes and change every tick. ADR-028
//! asks "is this something the player is, does, or is like today?"; for
//! them the question is different, because they are not the player's.
//! They are **observable facts of the match**, Plane B:
//!
//! - `position` is read by the engine everywhere;
//! - `heading` is read by the cone (`visible_to` in `perception.rs`),
//!   written when the carrier turns to play, and written by the
//!   movement pass (F.7 step 9);
//! - `velocity` is **written** (the movement pass) and read as the
//!   previous tick's displacement (F.7 step 9); it is a derived output,
//!   not a model input.
//!
//! **Futurability.** All three are observable, so the 2D/3D will show them
//! through the tick record (IDEA-INFRA-018, not built). The tick record is
//! the leaf: without it, these fields have no renderer, and the 2D cannot
//! show movement at all.

use std::collections::BTreeMap;

use ds_core::ids::PlayerId;
use ds_model::{Pitch, Squad};

/// The height a ball in the air is met at, cm: a player's head.
const HEAD_HEIGHT_CM: i32 = 180;

/// A player's position, velocity, heading and gaze on the pitch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgentState {
    /// Position in centimetres, `(x, y)`.
    pub position: (i32, i32),
    /// Velocity in centimetres per tick, `(vx, vy)`.
    pub velocity: (i16, i16),
    /// Heading in degrees, `0..=359`, `0` = north (positive `y`), clockwise.
    pub heading: u16,
    /// Gaze in degrees (ADR-033): direction the agent looks at, distinct
    /// from heading (body). Written in `from_squads` (all look at the ball
    /// at kickoff) and in `move_agents` via `perception::scan` (ADR-034).
    pub gaze: u16,
}

// Note (2026-11-07): superseded. `velocity` is written by the movement
// pass and read as the previous tick's displacement; `heading` is read
// by the cone and written by the movement pass. See F.7 step 9. The
// 2026-11-02 record follows and is kept as history.
// They are reserved fields for the per-agent slice, not part of any model
// today. They are the two declared limits of the influence formula
// (`03-entities-minimal` §2.9): heading (orientation) and time to arrive
// (velocity). See OI-25 and OI-29.

/// The `Player` fields the movement formula reads.
///
/// `spatial.rs` must not depend on `Player` (the model is not the
/// match state): this type carries the three fields
/// `resolve_movement` reads, and `MatchState` builds the map once
/// in `new`.
#[derive(Debug, Clone, Copy)]
pub struct PlayerBody {
    /// Height in centimetres (Plane B, registry data).
    pub height_cm: f64,
    /// Weight in kilograms (Plane B, registry data).
    pub weight_kg: f64,
    /// Fatigue, 0.0 to 1.0 (Plane B, state).
    pub fatigue: f64,
}

/// The state of the ball (MD-25). `Possessed` means at the carrier's feet
/// (velocity zero, no physics). `InFlight` means travelling with its own
/// velocity. `Loose` means free and at rest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BallState {
    /// At the carrier's feet.
    Possessed,
    /// Travelling with a velocity.
    InFlight,
    /// Free and at rest.
    Loose,
}
// The ball's physical properties (gravity, friction in the air and on the
// ground, stop threshold) are declared data, `balance/ball.json` (ADR-048
// §4); they used to be four constants here.

/// The spatial state of a match: every agent, plus the ball.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialState {
    agents: BTreeMap<PlayerId, AgentState>,
    ball: BallState,
    ball_position: (i32, i32, i32),
    ball_velocity: (i16, i16, i16),
    /// The ball's declared physics (ADR-048 §4).
    ball_physics: crate::balance::BallBalance,
    /// The team block (VISION §3): where each outfield player stands
    /// without instructions.
    block: crate::balance::BlockBalance,
    /// Each outfield player's place in the block: `(depth, width)` in
    /// thousandths, from the formation (0 = defensive line / left).
    roles: BTreeMap<PlayerId, crate::shape::Role>,
    /// The pitch, in centimetres: `(length, width)`.
    pitch_cm: (i32, i32),
    /// Whether the home side has the ball (set by the match each tick).
    home_in_possession: bool,
    /// Each player's off-ball act, as he chose it (ADR-049).
    acts: BTreeMap<PlayerId, ds_orchestrator::OffBallAct>,
    /// Where a player who chose to offer or to run is going (ADR-051).
    act_targets: BTreeMap<PlayerId, (i32, i32)>,
    /// Players sent off (Law 12): off the pitch, on the bench. Not agents:
    /// nobody sees them, nothing reaches them; the record still shows them.
    off_pitch: BTreeMap<PlayerId, AgentState>,
    /// Where the carrier is taking the ball, if he chose to carry it
    /// (IDEA-DESIGN-010: carrying is a choice). `None`: he holds it.
    carry_target: Option<(i32, i32)>,
    /// The agent carrying the ball, if any.
    carrier: Option<PlayerId>,
    /// The goalkeeper of each team: `(home, away)`. A fact of the
    /// formation, populated at construction (OI-81-step-move). The keeper
    /// moves with `resolve_movement` like every agent; only his target
    /// differs.
    goalkeepers: (PlayerId, PlayerId),
}

/// ~45 cm shoulder half-width; two agents closer than 2R are pushed apart.
const BODY_RADIUS_CM: i32 = 45;

/// Distance from the goal line at which the goalkeeper stands (F.3-b3).
/// Arbitrary, declared: 2 m off the line.
const GK_LINE_OFFSET_CM: i32 = 200;

/// One agent's work tuple for the movement pass: id, current position,
/// formation slot. A named type keeps the borrow simple and the signature
/// readable (clippy: complex type).
type Work = (PlayerId, (i32, i32), (i32, i32));

impl SpatialState {
    /// Builds the initial spatial state from a pitch and two squads.
    ///
    /// Home players are placed on the 4-4-2 grid; away players are the same
    /// grid mirrored on the `x` axis. The goalkeeper (slot 0) starts on the
    /// goal line (F.3-b3), not on the 4-4-2 grid slot. The ball starts at
    /// the centre, loose.
    ///
    /// # Panics
    ///
    /// Panics if a squad does not have exactly 11 players. `Squad` enforces
    /// this at construction, so the panic is unreachable in practice.
    #[must_use]
    pub fn from_squads(pitch: &Pitch, home: &Squad, away: &Squad) -> Self {
        let length = i32::from(pitch.length_m) * 100;
        let width = i32::from(pitch.width_m) * 100;
        let mut agents = BTreeMap::new();
        // The goalkeepers start on their lines; the outfield players are
        // placed in their block for the kick-off below.
        for (i, player) in home.players.iter().enumerate() {
            let position = if i == 0 {
                (GK_LINE_OFFSET_CM, width / 2)
            } else {
                (length / 2, width / 2)
            };
            agents.insert(
                player.id,
                AgentState {
                    position,
                    velocity: (0, 0),
                    heading: 90,
                    gaze: 90,
                },
            );
        }
        for (i, player) in away.players.iter().enumerate() {
            let position = if i == 0 {
                (length - GK_LINE_OFFSET_CM, width / 2)
            } else {
                (length / 2, width / 2)
            };
            agents.insert(
                player.id,
                AgentState {
                    position,
                    velocity: (0, 0),
                    heading: 270,
                    gaze: 270,
                },
            );
        }
        let mut s = Self {
            agents,
            ball: BallState::Loose,
            ball_position: (length / 2, width / 2, 0),
            ball_velocity: (0, 0, 0),
            ball_physics: crate::balance::BallBalance::load(),
            block: crate::balance::BlockBalance::load(),
            roles: crate::shape::roles(home, away),
            pitch_cm: (length, width),
            home_in_possession: true,
            carry_target: None,
            acts: BTreeMap::new(),
            act_targets: BTreeMap::new(),
            off_pitch: BTreeMap::new(),
            carrier: None,
            goalkeepers: (home.players[0].id, away.players[0].id),
        };
        // Kick-off positions (Law 8, OI-98): each side in its own half, in
        // its compact block around a ball on the centre spot. Before, the
        // two formations spanned the whole pitch and stood on top of each
        // other.
        let centre = (length / 2, width / 2);
        let ids: Vec<PlayerId> = s.roles.keys().copied().collect();
        for id in ids {
            if let Some(t) = s.block_target(id, centre, false, true) {
                if let Some(a) = s.agents.get_mut(&id) {
                    a.position = t;
                }
            }
        }
        s
    }

    /// A player's off-ball act, as he chose it (ADR-049).
    pub fn set_act(&mut self, id: PlayerId, act: ds_orchestrator::OffBallAct) {
        self.acts.insert(id, act);
        self.act_targets.remove(&id);
    }

    /// Where a player who chose to offer or to run is going.
    pub fn set_act_target(&mut self, id: PlayerId, target: (i32, i32)) {
        self.act_targets.insert(id, target);
    }

    /// A player's off-ball act, if he has chosen one.
    #[must_use]
    pub fn act(&self, id: PlayerId) -> Option<ds_orchestrator::OffBallAct> {
        self.acts.get(&id).copied()
    }

    /// The side in possession, for the block (set by the match each tick).
    pub const fn set_home_in_possession(&mut self, home: bool) {
        self.home_in_possession = home;
    }

    /// Where an outfield player stands in his team's block (VISION §3:
    /// the role as a default intention). `None` for a goalkeeper.
    ///
    /// The block follows the ball: the defensive line sits behind it,
    /// between a minimum and a maximum distance from its own goal, and the
    /// block shifts towards the ball's side; with the ball the side
    /// stretches, without it it compacts (`balance/block.json`). With
    /// `own_half` the player stays in his own half (the kick-off).
    pub(crate) fn block_target(
        &self,
        id: PlayerId,
        ball: (i32, i32),
        in_possession: bool,
        own_half: bool,
    ) -> Option<(i32, i32)> {
        let role = *self.roles.get(&id)?;
        let situation = crate::shape::Situation {
            ball,
            in_possession,
            home: crate::perception::team_of(id) == 0,
            pitch: self.pitch_cm,
            kick_off: own_half,
        };
        Some(crate::shape::place(role, situation, &self.block))
    }

    /// The state of one agent, or `None` if the id is not on the pitch.
    #[must_use]
    pub fn agent(&self, id: PlayerId) -> Option<&AgentState> {
        self.agents.get(&id)
    }

    /// The state of one agent, mutably, or `None` if the id is not on the pitch.
    ///
    /// The only mutable door for an agent's position: the simulation must
    /// not reach into the map directly (`00-foundations` §3).
    pub fn agent_mut(&mut self, id: PlayerId) -> Option<&mut AgentState> {
        self.agents.get_mut(&id)
    }

    /// Sets the position of an agent. Returns `false` if the id is not on
    /// the pitch (nothing is changed).
    ///
    /// No clamping here: the pitch bounds belong to the movement model
    /// (F.3), not to the primitive. A target derived from the ball can fall
    /// outside the pitch; what the agent does then is a model decision
    /// declared in F.3, not hidden in a setter.
    pub fn set_position(&mut self, id: PlayerId, position: (i32, i32)) -> bool {
        let Some(agent) = self.agents.get_mut(&id) else {
            return false;
        };
        agent.position = position;
        true
    }

    /// The ball position, in centimetres, 2D `(x, y)`.
    #[must_use]
    pub const fn ball(&self) -> (i32, i32) {
        (self.ball_position.0, self.ball_position.1)
    }

    /// The ball position, in centimetres, 3D `(x, y, z)`.
    #[must_use]
    pub const fn ball_3d(&self) -> (i32, i32, i32) {
        self.ball_position
    }

    /// The ball state.
    #[must_use]
    pub const fn ball_state(&self) -> BallState {
        self.ball
    }

    /// Puts the ball in flight from `position` with `velocity`. A no-op if
    /// the ball is possessed (physics does not run on a possessed ball,
    /// MD-25): the caller must release the carrier first.
    pub const fn set_ball_in_flight(
        &mut self,
        position: (i32, i32, i32),
        velocity: (i16, i16, i16),
    ) {
        if matches!(self.ball, BallState::Possessed) {
            return;
        }
        self.ball = BallState::InFlight;
        self.ball_position = position;
        self.ball_velocity = velocity;
    }

    /// The ball velocity, in centimetres per tick, 3D `(vx, vy, vz)`.
    /// Zero when the ball is not in flight.
    #[must_use]
    pub const fn ball_velocity(&self) -> (i16, i16, i16) {
        self.ball_velocity
    }

    /// Gravity, cm/tick² (`ball.json`).
    #[must_use]
    pub fn gravity_cm_per_tick2(&self) -> i32 {
        i32::from(self.ball_physics.gravity_cm_per_tick2)
    }

    /// The ball's ground friction, thousandths per tick: what a passer
    /// knows of the pitch when he weighs a pass.
    #[must_use]
    pub const fn ground_friction_milli(&self) -> i32 {
        self.ball_physics.ground_friction_milli
    }

    /// Deflects the ball in flight (ADR-048 §1): the ball is put back at the
    /// impact point and keeps flying with the reflected velocity. The
    /// tick's integration may have carried the ball past the obstacle; the
    /// touch happened at `impact`, so the ball restarts from there, not from
    /// behind the goal line. A no-op if the ball is not in flight: the
    /// reflection applies to a moving ball only.
    pub const fn deflect_ball(&mut self, impact: (i32, i32, i32), velocity: (i16, i16, i16)) {
        if matches!(self.ball, BallState::InFlight) {
            self.ball_position = impact;
            self.ball_velocity = velocity;
        }
    }

    /// Where a ball in the air comes down to `height_cm`, by the same
    /// physics `advance_ball` applies (gravity, air friction): where a
    /// player who reads it goes to meet it. `None` if it is not in the air
    /// above that height.
    #[must_use]
    pub fn drop_point(&self, height_cm: i32) -> Option<(i32, i32)> {
        let BallState::InFlight = self.ball else {
            return None;
        };
        let (mut x, mut y, mut z) = self.ball_position;
        if z <= height_cm {
            return None;
        }
        let (mut vx, mut vy, mut vz) = (
            i32::from(self.ball_velocity.0),
            i32::from(self.ball_velocity.1),
            i32::from(self.ball_velocity.2),
        );
        let p = self.ball_physics;
        for _ in 0..200 {
            vz -= i32::from(p.gravity_cm_per_tick2);
            vx = vx * p.air_friction_milli / 1000;
            vy = vy * p.air_friction_milli / 1000;
            x += vx;
            y += vy;
            z += vz;
            if z <= height_cm && vz < 0 {
                return Some((x, y));
            }
        }
        Some((x, y))
    }

    /// Advances the ball one tick if it is in flight (MD-17/18).
    ///
    /// Applies gravity to `vz`, gravity to `z`, friction to the horizontal
    /// velocity, and the stop threshold. `Possessed` and `Loose` are
    /// untouched: only an in-flight ball moves. The constants are declared
    /// (MD-24): gravity is ~10 cm/tick² (derived from g ≈ 9.8 m/s² at the
    /// tick rate), friction `f = 990/1000` per tick (arbitrary, declared),
    /// stop threshold 1 cm/tick (arbitrary, declared).
    pub fn advance_ball(&mut self) {
        let BallState::InFlight = self.ball else {
            return;
        };
        let (mut vx, mut vy, mut vz) = self.ball_velocity;
        let (mut x, mut y, mut z) = self.ball_position;
        let p = self.ball_physics;
        // On the ground the grass slows the ball more than the air does.
        let friction = if z == 0 && vz <= 0 {
            p.ground_friction_milli
        } else {
            p.air_friction_milli
        };
        vz = vz.saturating_sub(p.gravity_cm_per_tick2);
        vx = i16::try_from(i32::from(vx) * friction / 1000).unwrap_or(vx);
        vy = i16::try_from(i32::from(vy) * friction / 1000).unwrap_or(vy);
        x = x.saturating_add(i32::from(vx));
        y = y.saturating_add(i32::from(vy));
        z = z.saturating_add(i32::from(vz));
        if z < 0 {
            z = 0;
            vz = 0;
        }
        self.ball_position = (x, y, z);
        self.ball_velocity = (vx, vy, vz);
        if z == 0
            && vx.abs() <= p.stop_threshold_cm_per_tick
            && vy.abs() <= p.stop_threshold_cm_per_tick
        {
            self.ball = BallState::Loose;
            self.ball_velocity = (0, 0, 0);
        }
    }
    /// The agent carrying the ball, if any.
    #[must_use]
    pub const fn carrier(&self) -> Option<PlayerId> {
        self.carrier
    }

    /// Sets the carrier. `None` means the ball is loose.
    pub const fn set_carrier(&mut self, carrier: Option<PlayerId>) {
        self.carrier = carrier;
        self.carry_target = None;
        if carrier.is_some() {
            self.ball = BallState::Possessed;
            self.ball_velocity = (0, 0, 0);
        } else if matches!(self.ball, BallState::Possessed) {
            self.ball = BallState::Loose;
        }
    }

    /// The carrier chose to carry the ball towards `target` (IDEA-DESIGN-010).
    /// The target is forgotten when the carrier changes.
    pub const fn set_carry_target(&mut self, target: Option<(i32, i32)>) {
        self.carry_target = target;
    }

    /// The ball is at the carrier's feet: it goes where he goes. A no-op
    /// if nobody carries it.
    pub fn follow_carrier(&mut self) {
        if !matches!(self.ball, BallState::Possessed) {
            return;
        }
        if let Some(pos) = self
            .carrier
            .and_then(|c| self.agents.get(&c))
            .map(|a| a.position)
        {
            self.ball_position = (pos.0, pos.1, 0);
        }
    }

    /// Moves every agent one tick toward its target (MD-09, MD-10, MD-12).
    ///
    /// Target: the formation slot, translated by a declared fraction of the
    /// ball's displacement from the pitch centre. One agent (the one nearest
    /// the ball, per team) is deviated toward the ball (placeholder, MD-09).
    /// Displacement (F.7 step 7b): each agent accelerates via
    /// `resolve_movement` (the movement formula), capped by
    /// `v_max * stride` and the remaining distance (no overshoot).
    /// `velocity` stores the previous displacement; its L-infinity
    /// norm is the accumulated speed (no reader today; the tick
    /// record will read displacement, IDEA-INFRA-018).
    /// After the step, a minimal separation
    /// (`BODY_RADIUS_CM`) pushes overlapping agents apart (MD-12).
    /// **OI-81-step-move:** the goalkeeper is no longer anchored. He moves
    /// with the same formula as every agent; only his target differs:
    /// a point on the ball-goal segment, advanced by a fraction that
    /// depends on the ball's distance from the goal (`L0-match/balance/keeper.json`).
    /// At `t = 0` (ball far) he rests at `GK_LINE_OFFSET_CM` from the line;
    /// at `t = 1` (ball on the line) he advances by `ADVANCE_CM` more.
    /// The keeper is not a chaser: he is excluded from the `nearest`
    /// computation (a keeper who runs after the ball is not a keeper).
    ///
    /// Deterministic: no RNG (S1). The variation comes from the inputs
    /// (positions, ball, the block), not from a die.
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::too_many_lines)]
    pub fn move_agents(
        &mut self,
        pitch_length_cm: i32,
        pitch_width_cm: i32,
        bodies: &BTreeMap<PlayerId, PlayerBody>,
        hidden: &mut ds_orchestrator::GroundTruth,
        tick: u64,
        scan_balance: &crate::balance::ScanningBalance,
        keeper_balance: &crate::balance::KeeperBalance,
    ) {
        let ball = self.ball();

        // Snapshot: agents in deterministic (BTreeMap) order.
        // Same operations as the direct version, no per-pair map lookup.
        // F.3-b3: anchored agents (goalkeepers) are skipped.
        let mut work: Vec<Work> = self
            .agents
            .iter()
            .map(|(id, a)| (*id, a.position, a.position))
            .collect();

        // Who goes where is each player's own choice (ADR-049): the engine
        // only reads it. Before, the engine picked "the nearest player" of
        // each side to chase or press.

        // F.7 step 7b: each agent's displacement comes from the movement
        // formula (`L0-match/balance/movement.json`, via `resolve_movement`), not from
        // a fixed step. `velocity` stores the previous tick's displacement;
        // its L-infinity norm is the current speed, which accumulates toward
        // `v_max` (acceleration) and is capped by the remaining distance
        // (no overshoot). The displacement is per-axis (L-infinity): a
        // diagonal step has the same per-axis magnitude as a cardinal one,
        // matching the previous rule (MD-09), not a new model choice.
        // OI-81-step-move: the keeper positions himself on the
        // ball-goal segment, with a fraction that depends on the
        // distance from the ball to the goal (L0-match/balance/keeper.json).
        // The same movement formula as every agent, a different target.
        // The rest x is GK_LINE_OFFSET_CM, not 0: the keeper does not
        // stand on the goal line (existing fact, missing in the cantiere).
        let goal_y = f64::from(pitch_width_cm) / 2.0;
        let range = f64::from(keeper_balance.positioning_range_cm);
        let advance = f64::from(keeper_balance.advance_cm);
        let mut keeper_targets: Vec<(PlayerId, (i32, i32))> = Vec::with_capacity(2);
        for (gk, goal_x) in [
            (self.goalkeepers.0, 0.0_f64),
            (self.goalkeepers.1, f64::from(pitch_length_cm)),
        ] {
            let delta_x = f64::from(ball.0) - goal_x;
            let delta_y = f64::from(ball.1) - goal_y;
            let d = delta_x.hypot(delta_y);
            let t = (1.0 - d / range).clamp(0.0, 1.0);
            let y_target = f64::mul_add(f64::from(ball.1) - goal_y, t, goal_y);
            let sign = if goal_x == 0.0 { 1.0_f64 } else { -1.0_f64 };
            // Never deeper than a share of the ball's depth: he stays
            // between the ball and his goal, on his line when the ball is
            // on the line beside the post (ADR-051).
            let depth_cap = delta_x.abs() * f64::from(keeper_balance.depth_share_milli) / 1000.0;
            let off_line = f64::mul_add(advance, t, f64::from(GK_LINE_OFFSET_CM)).min(depth_cap);
            let x_target = f64::mul_add(sign, off_line, goal_x);
            #[allow(clippy::cast_possible_truncation)]
            let mut tx = (x_target.round() as i32).clamp(0, pitch_length_cm);
            // With the ball his side moves up and so does he: he follows the
            // defensive line at a distance and becomes an outlet for the
            // pass back (maintainer, 2026-11-12).
            let home = goal_x == 0.0;
            if home == self.home_in_possession {
                let line = crate::shape::place(
                    crate::shape::Role {
                        depth: 0,
                        width: 500,
                    },
                    crate::shape::Situation {
                        ball,
                        in_possession: true,
                        home,
                        pitch: self.pitch_cm,
                        kick_off: false,
                    },
                    &self.block,
                );
                let line_u = if home {
                    line.0
                } else {
                    pitch_length_cm - line.0
                };
                let support_u = (line_u - keeper_balance.support_gap_cm)
                    .clamp(GK_LINE_OFFSET_CM, keeper_balance.support_max_cm);
                let current_u = if home { tx } else { pitch_length_cm - tx };
                if support_u > current_u {
                    tx = if home {
                        support_u
                    } else {
                        pitch_length_cm - support_u
                    };
                }
            }
            #[allow(clippy::cast_possible_truncation)]
            let ty = (y_target.round() as i32).clamp(0, pitch_width_cm);
            keeper_targets.push((gk, (tx, ty)));
        }
        let drop = self.drop_point(HEAD_HEIGHT_CM);
        for entry in &mut work {
            let act = self.acts.get(&entry.0).copied();
            let to_the_ball = matches!(
                act,
                Some(
                    ds_orchestrator::OffBallAct::Press
                        | ds_orchestrator::OffBallAct::Challenge
                        | ds_orchestrator::OffBallAct::GoToBall
                )
            );
            let target = if Some(entry.0) == self.carrier {
                // The carrier holds the ball where he is, or carries it
                // where he chose (IDEA-DESIGN-010); the keeper too.
                self.carry_target.unwrap_or(entry.1)
            } else if to_the_ball {
                // A high ball is met where it comes down (head height),
                // not chased where it is.
                if act == Some(ds_orchestrator::OffBallAct::GoToBall) {
                    drop.unwrap_or(ball)
                } else {
                    ball
                }
            } else if let (
                Some(
                    ds_orchestrator::OffBallAct::Offer
                    | ds_orchestrator::OffBallAct::Run
                    | ds_orchestrator::OffBallAct::Mark,
                ),
                Some(t),
            ) = (act, self.act_targets.get(&entry.0))
            {
                *t
            } else if let Some((_, kt)) = keeper_targets.iter().find(|(g, _)| *g == entry.0) {
                *kt
            } else {
                let mine = (crate::perception::team_of(entry.0) == 0) == self.home_in_possession;
                self.block_target(entry.0, ball, mine, false)
                    .unwrap_or(entry.2)
            };
            let Some(body) = bodies.get(&entry.0) else {
                continue;
            };
            let v_prev = self.agents.get(&entry.0).map_or(0, |a| {
                i32::from(a.velocity.0.unsigned_abs().max(a.velocity.1.unsigned_abs()))
            });
            let mctx = ds_orchestrator::MovementContext {
                player: entry.0,
                height_cm: body.height_cm,
                weight_kg: body.weight_kg,
                fatigue: body.fatigue,
            };
            let outcome = hidden.resolve_movement(&mctx);
            let stride = outcome.stride;
            // Displacement space: stride is a positive per-player constant,
            // so `min(v + a, v_max)` in leg-speed space becomes
            // `min(v_disp + a*stride, v_max*stride)`.
            let accel_disp = outcome.acceleration * stride;
            let v_max_disp = outcome.v_max * stride;
            let dist = (target.0 - entry.1.0)
                .abs()
                .max((target.1 - entry.1.1).abs());
            let v_next = (f64::from(v_prev) + accel_disp)
                .min(v_max_disp)
                .min(f64::from(dist));
            #[allow(clippy::cast_possible_truncation)]
            let step = v_next.round() as i32;
            let step_x = (target.0 - entry.1.0).signum() * step;
            let step_y = (target.1 - entry.1.1).signum() * step;
            let nx = (entry.1.0 + step_x).clamp(0, pitch_length_cm);
            let ny = (entry.1.1 + step_y).clamp(0, pitch_width_cm);
            entry.1 = (nx, ny);
        }

        // Minimal separation (MD-12), on the snapshot: no map lookups.
        let min_d = i64::from(2 * BODY_RADIUS_CM);
        let n = work.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let (ax, ay) = work[i].1;
                let (bx, by) = work[j].1;
                let ddx = i64::from(bx - ax);
                let ddy = i64::from(by - ay);
                let d2 = ddx * ddx + ddy * ddy;
                if d2 > 0 && d2 < min_d * min_d {
                    let push = i32::try_from(((min_d - d2.isqrt()) / 2).max(1)).unwrap_or(1);
                    let nx = if ddx == 0 {
                        0
                    } else {
                        i32::try_from(ddx.signum()).unwrap_or(0)
                    };
                    let ny = if ddy == 0 {
                        0
                    } else {
                        i32::try_from(ddy.signum()).unwrap_or(0)
                    };
                    // The separation never pushes anyone off the pitch.
                    work[i].1 = (
                        (ax - nx * push).clamp(0, pitch_length_cm),
                        (ay - ny * push).clamp(0, pitch_width_cm),
                    );
                    work[j].1 = (
                        (bx + nx * push).clamp(0, pitch_length_cm),
                        (by + ny * push).clamp(0, pitch_width_cm),
                    );
                }
            }
        }

        self.write_back(&work, ball, tick, scan_balance);
    }
    /// Writes back the moved positions, with velocity derived from the
    /// displacement, and updates heading and gaze.
    fn write_back(
        &mut self,
        work: &[Work],
        ball: (i32, i32),
        tick: u64,
        scan_balance: &crate::balance::ScanningBalance,
    ) {
        for (id, new_pos, _) in work {
            let Some(agent) = self.agents.get_mut(id) else {
                continue;
            };
            agent.velocity = (
                i16::try_from(new_pos.0 - agent.position.0).unwrap_or(0),
                i16::try_from(new_pos.1 - agent.position.1).unwrap_or(0),
            );
            agent.position = *new_pos;
            if let Some(h) =
                crate::perception::heading_from_velocity(agent.velocity.0, agent.velocity.1)
            {
                agent.heading = h;
            }
        }
        // ADR-034: scan every `scan_interval_ticks`, not every tick. The
        // scan needs `&self` (read-only) to see the other agents; the
        // movement loop above held `&mut`, so the gaze pass is separate.
        //
        // The ball is the reference (OI-97, 2026-11-12): between two scans
        // the gaze goes back to the ball, as a footballer's does; the scan
        // is a glance (at the mark, the opponent on the ball), not a place
        // where the head stays. Before, a marked player looked at his mark
        // for good and never saw the ball again, and nobody went for it.
        let scanning =
            scan_balance.scan_interval_ticks != 0 && tick % scan_balance.scan_interval_ticks == 0;
        for (id, _, _) in work {
            let glance = if scanning {
                crate::perception::scan(self, *id, ball, scan_balance)
            } else {
                None
            };
            if let Some(agent) = self.agents.get_mut(id) {
                if let Some(g) = glance {
                    agent.gaze = g;
                } else if let Some(g) = crate::perception::heading_towards(agent.position, ball) {
                    agent.gaze = g;
                }
            }
        }
    }
    /// The number of agents on the pitch.
    #[must_use]
    pub fn len(&self) -> usize {
        self.agents.len()
    }

    /// Returns `true` if no agent is on the pitch.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.agents.is_empty()
    }

    /// Iterates over every `(PlayerId, AgentState)` pair, in `PlayerId` order.
    pub fn iter(&self) -> impl Iterator<Item = (&PlayerId, &AgentState)> {
        self.agents.iter().chain(self.off_pitch.iter())
    }

    /// The players on the pitch (not those sent off).
    pub fn on_pitch(&self) -> impl Iterator<Item = (&PlayerId, &AgentState)> {
        self.agents.iter()
    }

    /// Sends a player off (Law 12): he leaves the pitch for `bench`, and is
    /// no longer an agent (`agent` returns `None` for him).
    pub fn send_off(&mut self, id: PlayerId, bench: (i32, i32)) {
        if let Some(mut a) = self.agents.remove(&id) {
            a.position = bench;
            a.velocity = (0, 0);
            self.off_pitch.insert(id, a);
        }
        self.acts.remove(&id);
        self.act_targets.remove(&id);
        if self.carrier == Some(id) {
            self.set_carrier(None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ds_core::ids::SquadId;
    use ds_model::{PhysicalStateMinimal, Player, PsychologicalStateMinimal, RoleId};

    fn make_bodies() -> BTreeMap<PlayerId, PlayerBody> {
        let mut m = BTreeMap::new();
        for base in [0u32, 100u32] {
            for i in 0..11u32 {
                m.insert(
                    PlayerId::new(base + i, 0),
                    PlayerBody {
                        height_cm: 180.0,
                        weight_kg: 78.0,
                        fatigue: 0.0,
                    },
                );
            }
        }
        m
    }

    fn make_squad(slot_base: u32) -> Squad {
        let players: Vec<Player> = (0..11)
            .map(|i| {
                Player::try_new(
                    PlayerId::new(slot_base + i, 0),
                    format!("P{}", slot_base + i),
                    RoleId::new("midfielder").unwrap(),
                    180,
                    78,
                    PhysicalStateMinimal::default(),
                    PsychologicalStateMinimal::default(),
                )
                .unwrap()
            })
            .collect();
        Squad::try_new(
            SquadId::new(slot_base, 0),
            players,
            vec![],
            ds_model::FormationId::new("4-4-2").unwrap(),
        )
        .unwrap()
    }

    fn make_state() -> SpatialState {
        SpatialState::from_squads(&Pitch::default(), &make_squad(0), &make_squad(100))
    }

    #[test]
    fn twenty_two_agents_on_the_pitch() {
        assert_eq!(make_state().len(), 22);
    }

    #[test]
    fn same_input_same_output() {
        let a = make_state();
        let b = make_state();
        assert_eq!(a, b);
    }

    #[test]
    fn ball_at_centre() {
        let s = make_state();
        assert_eq!(s.ball(), (10500 / 2, 6800 / 2));
        assert_eq!(s.ball_3d(), (10500 / 2, 6800 / 2, 0));
        assert_eq!(s.ball_state(), BallState::Loose);
    }

    #[test]
    fn goalkeepers_are_on_the_goal_lines() {
        let s = make_state();
        let home_gk = s.agent(PlayerId::new(0, 0)).unwrap();
        let away_gk = s.agent(PlayerId::new(100, 0)).unwrap();
        assert_eq!(home_gk.position.0, GK_LINE_OFFSET_CM);
        assert_eq!(away_gk.position.0, 10500 - GK_LINE_OFFSET_CM);
    }

    #[test]
    fn move_agents_moves_the_goalkeeper_toward_the_ball_segment() {
        let mut s = make_state();
        let gk = PlayerId::new(0, 0);
        s.set_ball_in_flight((500, 3400, 0), (0, 0, 0));
        let before = s.agent(gk).unwrap().position;
        let bodies = make_bodies();
        let mut hidden = ds_orchestrator::GroundTruth::new(42, ds_core::ids::MatchId::new(0, 0));
        for tick in 0..50 {
            s.move_agents(
                10500,
                6800,
                &bodies,
                &mut hidden,
                tick,
                &crate::balance::ScanningBalance::load(),
                &crate::balance::KeeperBalance::load(),
            );
        }
        let after = s.agent(gk).unwrap().position;
        assert_ne!(
            before, after,
            "the keeper must move (P1): OI-81-step-move lifts the anchor"
        );
        assert!(
            after.0 > before.0,
            "the keeper must advance, not retreat: {before:?} -> {after:?}"
        );
    }

    #[test]
    fn every_position_is_inside_the_pitch() {
        let s = make_state();
        for (_, a) in s.iter() {
            assert!((0..=10500).contains(&a.position.0), "x out: {a:?}");
            assert!((0..=6800).contains(&a.position.1), "y out: {a:?}");
        }
    }

    #[test]
    fn away_is_mirrored_on_x() {
        let s = make_state();
        let home_lb = s.agent(PlayerId::new(1, 0)).unwrap();
        let away_lb = s.agent(PlayerId::new(101, 0)).unwrap();
        assert_eq!(home_lb.position.1, away_lb.position.1);
        assert_eq!(home_lb.position.0 + away_lb.position.0, 10500);
    }

    #[test]
    fn set_position_moves_the_agent() {
        let mut s = make_state();
        let id = PlayerId::new(1, 0);
        assert!(s.set_position(id, (1000, 2000)));
        assert_eq!(s.agent(id).unwrap().position, (1000, 2000));
    }

    #[test]
    fn set_position_unknown_id_changes_nothing() {
        let mut s = make_state();
        let before = s.len();
        assert!(!s.set_position(PlayerId::new(999, 0), (1, 1)));
        assert_eq!(s.len(), before);
    }

    #[test]
    fn agent_mut_reaches_the_position() {
        let mut s = make_state();
        let id = PlayerId::new(1, 0);
        s.agent_mut(id).unwrap().position = (500, 500);
        assert_eq!(s.agent(id).unwrap().position, (500, 500));
    }

    #[test]
    fn set_carrier_sets_possessed() {
        let mut s = make_state();
        let id = PlayerId::new(1, 0);
        s.set_carrier(Some(id));
        assert_eq!(s.ball_state(), BallState::Possessed);
        assert_eq!(s.ball_velocity(), (0, 0, 0));
    }

    #[test]
    fn set_ball_in_flight_is_a_noop_when_possessed() {
        let mut s = make_state();
        let id = PlayerId::new(1, 0);
        s.set_carrier(Some(id));
        let before = s.ball_3d();
        s.set_ball_in_flight((100, 200, 50), (10, 5, 20));
        assert_eq!(s.ball_3d(), before);
        assert_eq!(s.ball_state(), BallState::Possessed);
    }

    #[test]
    fn set_ball_in_flight_works_when_loose() {
        let mut s = make_state();
        s.set_ball_in_flight((100, 200, 50), (10, 5, 20));
        assert_eq!(s.ball_3d(), (100, 200, 50));
        assert_eq!(s.ball_state(), BallState::InFlight);
        assert_eq!(s.ball_velocity(), (10, 5, 20));
    }

    #[test]
    fn deflect_ball_restarts_from_the_impact_and_comes_back_into_the_pitch() {
        // ADR-048 §1 (P1): the integration carried the ball 250 cm past the
        // goal plane at x = 10500; the post touched it on the plane. After
        // the deflection the ball is on the plane, and the next tick moves
        // it back toward the pitch, not deeper behind the line.
        let mut s = make_state();
        s.set_ball_in_flight((10_750, 3766, 100), (250, 0, 0));
        s.deflect_ball((10_500, 3766, 100), (-200, 0, 0));
        assert_eq!(s.ball_3d(), (10_500, 3766, 100));
        s.advance_ball();
        assert!(
            s.ball_3d().0 < 10_500,
            "the ball must come back: {:?}",
            s.ball_3d()
        );
    }

    #[test]
    fn deflect_ball_is_a_noop_when_not_in_flight() {
        let mut s = make_state();
        let before = (s.ball_3d(), s.ball_velocity());
        s.deflect_ball((10_500, 3766, 100), (-200, 0, 0));
        assert_eq!((s.ball_3d(), s.ball_velocity()), before);
    }

    /// ADR-048 §4 (P1): a ground pass at 15 m/s rolls tens of metres, not
    /// hundreds. With one friction of 990 everywhere it rolled about 150 m
    /// and left the pitch (OI-97).
    #[test]
    fn a_ground_pass_rolls_tens_of_metres() {
        let mut s = make_state();
        s.set_ball_in_flight((0, 3400, 0), (150, 0, 0));
        for _ in 0..2000 {
            s.advance_ball();
        }
        assert_eq!(s.ball_state(), BallState::Loose, "it comes to rest");
        let x = s.ball_3d().0;
        assert!((2500..=5000).contains(&x), "rolled {x} cm");
    }

    #[test]
    fn advance_ball_is_a_noop_when_possessed() {
        let mut s = make_state();
        let id = PlayerId::new(1, 0);
        s.set_carrier(Some(id));
        let before = s.ball_3d();
        s.advance_ball();
        assert_eq!(s.ball_3d(), before);
        assert_eq!(s.ball_state(), BallState::Possessed);
    }

    #[test]
    fn advance_ball_moves_z_down_and_x_forward_under_gravity() {
        let mut s = make_state();
        s.set_ball_in_flight((5000, 3400, 200), (100, 0, 0));
        s.advance_ball();
        // gravity pulls z down by 10 (vz 0 -> -10), x advances by ~99 (friction 0.99)
        let (x, y, z) = s.ball_3d();
        assert!(z < 200, "z should fall under gravity, got {z}");
        assert!(x > 5000, "x should advance, got {x}");
        assert_eq!(y, 3400);
        let (_, _, vz) = s.ball_velocity();
        assert!(vz < 0, "vz should be negative after gravity, got {vz}");
    }

    #[test]
    fn advance_ball_lands_and_stops() {
        let mut s = make_state();
        s.set_ball_in_flight((5000, 3400, 0), (0, 0, 0));
        // With zero velocity and zero height, the ball is already at rest
        // and the first tick should mark it Loose (threshold).
        s.advance_ball();
        assert_eq!(s.ball_3d(), (5000, 3400, 0));
        assert_eq!(s.ball_state(), BallState::Loose);
    }

    #[test]
    fn move_agents_heading_follows_velocity() {
        // F.7 step 9: after the movement pass, every agent that moved has
        // a heading that matches its velocity (the invariant the cone will
        // read once F.6 makes it per-agent).
        let mut s = make_state();
        let bodies = make_bodies();
        let mut hidden = ds_orchestrator::GroundTruth::new(42, ds_core::ids::MatchId::new(0, 0));
        s.move_agents(
            10500,
            6800,
            &bodies,
            &mut hidden,
            0,
            &crate::balance::ScanningBalance::load(),
            &crate::balance::KeeperBalance::load(),
        );
        let mut moved = 0;
        for (id, a) in s.iter() {
            if a.velocity == (0, 0) {
                continue;
            }
            moved += 1;
            let expected = crate::perception::heading_from_velocity(a.velocity.0, a.velocity.1);
            assert_eq!(
                Some(a.heading),
                expected,
                "agent {id:?} moved with velocity {:?} but heading is {}",
                a.velocity,
                a.heading
            );
        }
        assert!(moved > 0, "no agent moved; the test proves nothing (P1)");
    }
}
