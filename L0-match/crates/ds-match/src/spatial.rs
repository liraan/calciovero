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
    /// Where the carrier is taking the ball, if he chose to carry it
    /// (IDEA-DESIGN-010: carrying is a choice). `None`: he holds it.
    carry_target: Option<(i32, i32)>,
    /// The agent carrying the ball, if any.
    carrier: Option<PlayerId>,
    /// The formation slot of each agent, in centimetres. The default
    /// (`GRID_4_4_2`, mirrored) is a placeholder: L2 replaces it (MD-09).
    slots: BTreeMap<PlayerId, (i32, i32)>,
    /// The goalkeeper of each team: `(home, away)`. A fact of the
    /// formation, populated at construction (OI-81-step-move). The keeper
    /// moves with `resolve_movement` like every agent; only his target
    /// differs.
    goalkeepers: (PlayerId, PlayerId),
}

/// The 4-4-2 grid, as `(x percent, y percent)` per slot.
///
/// Slot order: GK, LB, CB, CB, RB, LM, CM, CM, RM, ST, ST. It matches
/// `Squad.players` **by index**. The formation parser is deferred (D3): a
/// non-4-4-2 formation falls back to this grid.
/// How much of the ball's displacement from the pitch centre translates
/// the formation (MD-09). Arbitrary, declared: 30% of the ball's offset.
const TRANSLATION_NUM: i32 = 30;
const TRANSLATION_DEN: i32 = 100;

/// ~45 cm shoulder half-width; two agents closer than 2R are pushed apart.
const BODY_RADIUS_CM: i32 = 45;

/// Distance from the goal line at which the goalkeeper stands (F.3-b3).
/// Arbitrary, declared: 2 m off the line.
const GK_LINE_OFFSET_CM: i32 = 200;

/// One agent's work tuple for the movement pass: id, current position,
/// formation slot. A named type keeps the borrow simple and the signature
/// readable (clippy: complex type).
type Work = (PlayerId, (i32, i32), (i32, i32));
const GRID_4_4_2: [(i32, i32); 11] = [
    (5, 50),
    (25, 15),
    (25, 38),
    (25, 62),
    (25, 85),
    (50, 15),
    (50, 38),
    (50, 62),
    (50, 85),
    (75, 38),
    (75, 62),
];

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
        for (i, player) in home.players.iter().enumerate() {
            let (px, py) = GRID_4_4_2[i];
            let position = if i == 0 {
                (GK_LINE_OFFSET_CM, width / 2)
            } else {
                (length * px / 100, width * py / 100)
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
            let (px, py) = GRID_4_4_2[i];
            let position = if i == 0 {
                (length - GK_LINE_OFFSET_CM, width / 2)
            } else {
                (length - length * px / 100, width * py / 100)
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
        let mut slots: BTreeMap<PlayerId, (i32, i32)> = BTreeMap::new();
        for (i, player) in home.players.iter().enumerate() {
            let (px, py) = GRID_4_4_2[i];
            slots.insert(player.id, (length * px / 100, width * py / 100));
        }
        for (i, player) in away.players.iter().enumerate() {
            let (px, py) = GRID_4_4_2[i];
            slots.insert(player.id, (length - length * px / 100, width * py / 100));
        }
        Self {
            agents,
            ball: BallState::Loose,
            ball_position: (length / 2, width / 2, 0),
            ball_velocity: (0, 0, 0),
            ball_physics: crate::balance::BallBalance::load(),
            carry_target: None,
            carrier: None,
            slots,
            goalkeepers: (home.players[0].id, away.players[0].id),
        }
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

    /// The formation slot of an agent, or `None` if not on the pitch.
    #[must_use]
    pub fn slot(&self, id: PlayerId) -> Option<(i32, i32)> {
        self.slots.get(&id).copied()
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
    /// (positions, ball, slots), not from a die.
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
        let centre = (pitch_length_cm / 2, pitch_width_cm / 2);
        let ball = self.ball();
        let dx = (ball.0 - centre.0) * TRANSLATION_NUM / TRANSLATION_DEN;
        let dy = (ball.1 - centre.1) * TRANSLATION_NUM / TRANSLATION_DEN;

        // Snapshot: agents and slots in deterministic (BTreeMap) order.
        // Same operations as the direct version, no per-pair map lookup.
        // F.3-b3: anchored agents (goalkeepers) are skipped.
        let mut work: Vec<Work> = self
            .slots
            .iter()
            .filter_map(|(id, slot)| {
                let a = self.agents.get(id)?;
                Some((*id, a.position, *slot))
            })
            .collect();

        let nearest = work
            .iter()
            // F.6 step 2: the chaser is the nearest agent who *perceives*
            // the ball. With gaze always towards the ball (OI-72), every
            // agent perceives it, so this filter is identity today: it
            // is now real: an agent that does not see the ball does not chase it (ADR-034).
            .filter(|w| {
                w.0 != self.goalkeepers.0
                    && w.0 != self.goalkeepers.1
                    && crate::perception::perceived_ball(&*self, w.0, ball).is_some()
            })
            .min_by_key(|(_, pos, _)| {
                let ddx = i64::from(pos.0 - ball.0);
                let ddy = i64::from(pos.1 - ball.1);
                ddx * ddx + ddy * ddy
            })
            .map(|(id, _, _)| *id);

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
            let x_target = f64::mul_add(
                sign,
                f64::mul_add(advance, t, f64::from(GK_LINE_OFFSET_CM)),
                goal_x,
            );
            #[allow(clippy::cast_possible_truncation)]
            let tx = (x_target.round() as i32).clamp(0, pitch_length_cm);
            #[allow(clippy::cast_possible_truncation)]
            let ty = (y_target.round() as i32).clamp(0, pitch_width_cm);
            keeper_targets.push((gk, (tx, ty)));
        }
        for entry in &mut work {
            let target = if let Some((_, kt)) = keeper_targets.iter().find(|(g, _)| *g == entry.0) {
                *kt
            } else if Some(entry.0) == self.carrier {
                // The carrier holds the ball where he is, or carries it
                // where he chose (IDEA-DESIGN-010).
                self.carry_target.unwrap_or(entry.1)
            } else if Some(entry.0) == nearest {
                ball
            } else {
                (entry.2.0 + dx, entry.2.1 + dy)
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
                    work[i].1 = (ax - nx * push, ay - ny * push);
                    work[j].1 = (bx + nx * push, by + ny * push);
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
        self.agents.iter()
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
