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

use ds_core::dmath;
use std::collections::BTreeMap;

use ds_core::geometry::distance_squared;
use ds_core::ids::PlayerId;
use ds_model::{Pitch, Squad};

/// The height a ball in the air is met at, cm: a player's head.
const HEAD_HEIGHT_CM: i32 = 180;

/// A carrier's touch (plan L0-CLOSE 11): where he played the ball from,
/// which way, how hard, and when he meets it again.
#[derive(Debug, Clone, Copy)]
struct Touch {
    /// Where the ball left his foot, cm.
    from: (f64, f64),
    /// The direction he played it, a unit vector.
    dir: (f64, f64),
    /// Its speed off his foot, cm/tick.
    speed: f64,
    /// The tick of the touch.
    at: u64,
    /// Ticks until he meets it again, at his pace.
    period: f64,
    /// A ball he has just controlled, still under his foot: how far from
    /// him he took it, cm (0 for a touch played ahead).
    held_cm: f64,
}

// Compared bit for bit: the values are computed, never NaN.
impl PartialEq for Touch {
    fn eq(&self, o: &Self) -> bool {
        let bits = |t: &Self| {
            [
                t.from.0, t.from.1, t.dir.0, t.dir.1, t.speed, t.period, t.held_cm,
            ]
            .map(f64::to_bits)
        };
        bits(self) == bits(o) && self.at == o.at
    }
}
impl Eq for Touch {}

/// The speed a touch needs to be met again after `period` ticks.
///
/// The ball, slowed by the ground (it keeps `keep` of its speed each
/// tick), is met by a carrier running at `pace` (plan L0-CLOSE 11): it rolls
/// `u (1 - keep^t) / (1 - keep)`, he runs `pace t`; they meet at `period`.
#[must_use]
pub fn touch_speed(pace: f64, period: f64, keep: f64) -> f64 {
    let lost = 1.0 - dmath::powf(keep, period);
    if lost <= f64::EPSILON {
        return pace;
    }
    pace * period * (1.0 - keep) / lost
}

/// A ball under his foot `off` from his feet, brought to them in `step`
/// ticks (plan L0-CLOSE 11).
fn under_foot(off: (f64, f64), at: u64, step: f64) -> Touch {
    Touch {
        from: off,
        dir: (0.0, 0.0),
        speed: 0.0,
        at,
        period: step.max(1.0),
        held_cm: dmath::hypot(off.0, off.1),
    }
}

/// How far a man covers in `t` ticks from `speed`, accelerating from rest
/// at `accel` and less as he nears `top`, to nothing at it (cm, cm/tick;
/// SIM 194, the acceleration-speed profile of `movement.json`): his speed
/// closes on `top` as `top - (top - speed) e^(-accel t / top)`.
fn run_in(t: f64, speed: f64, accel: f64, top: f64) -> f64 {
    let top = top.max(speed).max(1.0);
    if accel <= 0.0 {
        return speed * t;
    }
    let k = accel / top;
    (top - speed).mul_add(
        -(1.0 - dmath::powf(std::f64::consts::E, -k * t)) / k,
        top * t,
    )
}

/// How far a touch of speed `speed` has rolled after `t` ticks.
#[must_use]
pub fn rolled(speed: f64, t: f64, keep: f64) -> f64 {
    if keep >= 1.0 {
        return speed * t;
    }
    speed * (1.0 - dmath::powf(keep, t)) / (1.0 - keep)
}

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
    /// Where he is in his stride, thousandths of a step (plan L0-CLOSE
    /// 11): it moves on with the ground he covers. A strike meets the ball
    /// at this phase of his step.
    pub gait_milli: u16,
    /// His position between two ticks, hundredths of a centimetre (decision
    /// BJ, ADR-069), and his velocity over the last small step, hundredths
    /// of a centimetre per step.
    pub fine: FineBody,
}

/// A body between two ticks (ADR-069): position and the last step's
/// velocity, hundredths of a centimetre.
pub type FineBody = ((i64, i64), (i64, i64));

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
    /// Mental fatigue from before the match, 0.0 to 1.0 (decision AB).
    pub mental: f64,
    /// Condition, 0.0 to 1.0 (decision AB): his endurance counts for this
    /// share.
    pub condition: f64,
    /// Muscle damage, 0.0 to 1.0 (decision AB): his fatigue never goes
    /// below it in this match.
    pub damage: f64,
}

impl PlayerBody {
    /// The body of a player as he starts (decision AB): his starting
    /// state, the damage already in his fatigue.
    #[must_use]
    pub fn of(p: &ds_model::Player) -> Self {
        let s = p.physical;
        Self {
            height_cm: f64::from(p.height_cm),
            weight_kg: f64::from(p.weight_kg),
            fatigue: f64::from(s.fatigue.max(s.damage)),
            mental: f64::from(s.cognitive_fatigue),
            condition: f64::from(s.condition),
            damage: f64::from(s.damage),
        }
    }

    /// How tired his mind is: the physical fatigue and the mental one he
    /// brought, together (Smith et al. 2016: mental fatigue slows the
    /// decision), at most 1.
    #[must_use]
    pub fn mind_fatigue(&self) -> f64 {
        (self.fatigue + self.mental).min(1.0)
    }
}

/// Small steps per tick (decision BJ, ADR-069): the decisions at 10 a
/// second, the bodies and the ball at 50.
pub(crate) const SUBSTEPS: i32 = 5;
/// The fine unit of the bodies and the ball: hundredths of a centimetre
/// (ADR-069), so that a small step of gravity or of acceleration is a
/// whole number.
pub(crate) const FINE: i64 = 100;
/// The small steps of a tick, as a length.
#[allow(clippy::cast_sign_loss)]
pub(crate) const STEPS: usize = SUBSTEPS as usize;

/// The ball's constants per small step, in fine units, derived from the
/// per-tick ones of `ball.json` (ADR-069: no new number).
#[derive(Debug, Clone, Copy)]
struct FineKick {
    /// Gravity, hundredths of a cm per step²: per tick² over the steps².
    gravity: i64,
    /// What the air and the grass keep of the horizontal speed per step,
    /// millionths: the fifth root of the per-tick share.
    air_ppm: i64,
    ground_ppm: i64,
    /// The restitution of the bounce, thousandths (it does not depend on
    /// the step).
    restitution_milli: i64,
    /// The stop threshold, hundredths of a cm per step.
    stop: i64,
}

impl FineKick {
    fn of(p: &crate::balance::BallBalance) -> Self {
        let steps = i64::from(SUBSTEPS);
        #[allow(clippy::cast_possible_truncation)]
        let root = |milli: i32| {
            (ds_core::dmath::powf(f64::from(milli) / 1000.0, 1.0 / f64::from(SUBSTEPS)) * 1e6)
                .round() as i64
        };
        Self {
            gravity: i64::from(p.gravity_cm_per_tick2) * FINE / (steps * steps),
            air_ppm: root(p.air_friction_milli),
            ground_ppm: root(p.ground_friction_milli),
            restitution_milli: i64::from(p.restitution_milli),
            stop: i64::from(p.stop_threshold_cm_per_tick) * FINE / steps,
        }
    }
}

/// The ball in fine units between two ticks (ADR-069).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct FineBall {
    /// Position, hundredths of a cm.
    at: (i64, i64, i64),
    /// Velocity, hundredths of a cm per small step.
    v: (i64, i64, i64),
    /// The centimetre state it was last in step with.
    synced: ((i32, i32, i32), (i16, i16, i16)),
}

impl FineBall {
    /// From a ball in centimetres and centimetres per tick.
    fn from_cm(p: (i32, i32, i32), v: (i16, i16, i16)) -> Self {
        let steps = i64::from(SUBSTEPS);
        Self {
            at: (
                i64::from(p.0) * FINE,
                i64::from(p.1) * FINE,
                i64::from(p.2) * FINE,
            ),
            v: (
                i64::from(v.0) * FINE / steps,
                i64::from(v.1) * FINE / steps,
                i64::from(v.2) * FINE / steps,
            ),
            synced: (p, v),
        }
    }

    /// In centimetres, and centimetres per tick, rounded.
    fn to_cm(self) -> ((i32, i32, i32), (i16, i16, i16)) {
        let steps = i64::from(SUBSTEPS);
        let cm = |f: i64| i32::try_from((f + FINE / 2).div_euclid(FINE)).unwrap_or(0);
        let per_tick = |f: i64| i16::try_from((f * steps + FINE / 2).div_euclid(FINE)).unwrap_or(0);
        (
            (cm(self.at.0), cm(self.at.1), cm(self.at.2)),
            (per_tick(self.v.0), per_tick(self.v.1), per_tick(self.v.2)),
        )
    }

    /// One tick: its small steps of gravity, the friction of the air or
    /// of the grass, the bounce; whether it stopped.
    fn tick(&mut self, k: &FineKick) -> bool {
        self.tick_steps(k, &mut [(0, 0, 0); STEPS])
    }

    /// One tick, with where the ball is, in centimetres, at the end of
    /// each small step (a ball that stops stays where it stopped).
    fn tick_steps(&mut self, k: &FineKick, path: &mut [(i32, i32, i32); STEPS]) -> bool {
        let cm = |f: i64| i32::try_from((f + FINE / 2).div_euclid(FINE)).unwrap_or(0);
        for s in 0..STEPS {
            // On the ground the grass slows the ball more than the air.
            let keep = if self.at.2 == 0 && self.v.2 <= 0 {
                k.ground_ppm
            } else {
                k.air_ppm
            };
            self.v.2 -= k.gravity;
            self.v.0 = self.v.0 * keep / 1_000_000;
            self.v.1 = self.v.1 * keep / 1_000_000;
            self.at.0 += self.v.0;
            self.at.1 += self.v.1;
            self.at.2 += self.v.2;
            if self.at.2 < 0 {
                self.at.2 = 0;
                // The FIFA vertical rebound; a bounce lower than one step
                // of gravity is a roll.
                let up = -self.v.2 * k.restitution_milli / 1000;
                self.v.2 = if up > k.gravity { up } else { 0 };
            }
            path[s] = (cm(self.at.0), cm(self.at.1), cm(self.at.2));
            if self.at.2 == 0 && self.v.0.abs() <= k.stop && self.v.1.abs() <= k.stop {
                let here = path[s];
                for rest in path.iter_mut().skip(s + 1) {
                    *rest = here;
                }
                return true;
            }
        }
        false
    }
}

/// The state of the ball (MD-25, ADR-083).
///
/// `Possessed` means at the carrier's feet (velocity zero; the physics step
/// leaves it there). `InFlight` means travelling with its own velocity.
/// `Loose` means free and no longer owned by a kick, but still physical: it
/// keeps rolling until friction spends it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BallState {
    /// At the carrier's feet.
    Possessed,
    /// Travelling with a velocity.
    InFlight,
    /// Free of a kick and no one's, but still physical: it keeps rolling
    /// until the declared friction spends it (ADR-083).
    Loose,
}
// The ball's physical properties (gravity, friction in the air and on the
// ground, stop threshold) are declared data, `balance/ball.json` (ADR-048
// §4); they used to be four constants here.

/// The last touches of the ball, as an observer tells them after a goal.
///
/// The last man who played it (had it at his feet or in his hands, or struck
/// it on purpose: a pass, a shot, a header, a clearance), whether that was a
/// shot, and the last man who only touched it on its way (a keeper's parry,
/// a block, a ball that bounced off him) after it was played.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Touches {
    /// The last man who played the ball.
    pub played: Option<PlayerId>,
    /// Whether his play was a shot.
    pub shot: bool,
    /// The last man it only touched on its way, after it was played.
    pub deflected: Option<PlayerId>,
}

/// The spatial state of a match: every agent, plus the ball.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialState {
    agents: BTreeMap<PlayerId, AgentState>,
    ball: BallState,
    ball_position: (i32, i32, i32),
    ball_velocity: (i16, i16, i16),
    /// Who touched the ball last, and how (Law 10: who scored, or whose
    /// own goal it was).
    touches: Touches,
    /// The ball's declared physics (ADR-048 §4).
    ball_physics: crate::balance::BallBalance,
    /// The ball between two ticks (decision BJ, ADR-069): its position in
    /// hundredths of a centimetre and its velocity per small step, and the
    /// centimetre state they were last in step with.
    ball_fine: FineBall,
    /// Where the ball was at the end of each small step of the last tick,
    /// centimetres (ADR-069, SIM 148): the contacts are judged there.
    ball_steps: [(i32, i32, i32); STEPS],
    /// Where each body was at the end of each small step of the last
    /// tick, centimetres (ADR-069, SIM 148).
    body_steps: BTreeMap<PlayerId, [(i32, i32); STEPS]>,
    /// How far short of the last defender's line a run waits (Law 11, the
    /// margin of `offball.json`): where a man asked to attack the depth
    /// holds (SIM 148).
    onside_margin_cm: i32,
    /// The team block (VISION §3): where each outfield player stands
    /// without instructions.
    block: crate::balance::BlockBalance,
    /// Each outfield player's place in the block: `(depth, width)` in
    /// thousandths, from the formation (0 = defensive line / left).
    roles: BTreeMap<PlayerId, usize>,
    /// The foot the carrier chose to touch the ball onto (ADR-031), until
    /// he runs with it again or loses it; otherwise the ball is on the foot
    /// of the side where it is.
    ball_foot: Option<ds_orchestrator::Side>,
    /// The coaches' boards, home and away (decision AL): each slot's place
    /// in the block, with the ball and without.
    boards: (ds_model::Board, ds_model::Board),
    /// The coaches' intentions, home and away (OI-27).
    intentions: (ds_model::Intention, ds_model::Intention),
    /// The coaches' quick instructions and transition choices, home and
    /// away (ADR-059 step 3).
    orders: (ds_model::TeamOrders, ds_model::TeamOrders),
    /// Until which tick the side that last won the ball is in its
    /// transition, and the tick now (ADR-059 §6).
    transition: (u64, u64),
    /// The pitch, in centimetres: `(length, width)`.
    pitch_cm: (i32, i32),
    /// Half the distance between the posts, cm (Law 1): the keeper covers
    /// the angle the ball makes with them.
    goal_half_width_cm: i32,
    /// Whether the home side has the ball (set by the match each tick).
    home_in_possession: bool,
    /// The gap a marker keeps goal-side of his man, used by the press too
    /// (SIM 181): the same declared distance, not a new number.
    press_gap_cm: i32,
    /// Each player's off-ball act, as he chose it (ADR-049).
    acts: BTreeMap<PlayerId, ds_orchestrator::OffBallAct>,
    /// Where a player who chose to offer or to run is going (ADR-051).
    act_targets: BTreeMap<PlayerId, (i32, i32)>,
    /// Who is going to a place for a restart, and whether he runs
    /// (`BEHAVIOURS.md` 25-27, SIM 150): the taker to the ball, the
    /// opponents out of the distance, the wall, everyone back to his half
    /// after a goal. Nobody is put there.
    sent: BTreeMap<PlayerId, ((i32, i32), bool)>,
    /// The place each man of the side with the ball chose to hold (ADR-071,
    /// SIM 152): the space worth most to his side, from his role's zone.
    hold_at: BTreeMap<PlayerId, (i32, i32)>,
    /// Players sent off (Law 12): off the pitch, on the bench. Not agents:
    /// nobody sees them, nothing reaches them; the record still shows them.
    off_pitch: BTreeMap<PlayerId, AgentState>,
    /// Where the carrier is taking the ball, if he chose to carry it
    /// (IDEA-DESIGN-010: carrying is a choice). `None`: he holds it.
    carry_target: Option<(i32, i32)>,
    /// The agent carrying the ball, if any.
    carrier: Option<PlayerId>,
    /// The carrier's last touch (plan L0-CLOSE 11): between two touches
    /// the ball rolls on its own. `None`: the ball is at his feet.
    touch: Option<Touch>,
    /// The velocity of the ball he has just won, cm/tick, until his first
    /// touch reads it (plan L0-CLOSE 11).
    won: Option<(i16, i16)>,
    /// How far the carrier reaches for his ball, cm (his reactivity, the
    /// same reach he wins a loose ball with), and the tick it was read.
    carrier_reach: (i64, u64),
    /// The goalkeeper of each team: `(home, away)`. A fact of the
    /// formation, populated at construction (OI-81-step-move). The keeper
    /// moves with `resolve_movement` like every agent; only his target
    /// differs.
    goalkeepers: (PlayerId, PlayerId),
}

/// ~45 cm shoulder half-width; two agents closer than 2R are pushed apart.
pub(crate) const BODY_RADIUS_CM: i32 = 45;

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
                    gait_milli: 0,
                    fine: ((0, 0), (0, 0)),
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
                    gait_milli: 0,
                    fine: ((0, 0), (0, 0)),
                },
            );
        }
        let mut s = Self {
            agents,
            ball: BallState::Loose,
            ball_position: (length / 2, width / 2, 0),
            ball_velocity: (0, 0, 0),
            touches: Touches::default(),
            ball_physics: crate::balance::BallBalance::load(),
            ball_fine: FineBall::default(),
            ball_steps: [(length / 2, width / 2, 0); STEPS],
            body_steps: BTreeMap::new(),
            onside_margin_cm: ds_calculator::offball::OffBallBalance::load().onside_margin_cm,
            press_gap_cm: ds_calculator::offball::OffBallBalance::load().mark_gap_cm,
            block: crate::balance::BlockBalance::load(),
            roles: crate::shape::slots(home, away),
            boards: (home.board, away.board),
            ball_foot: None,
            intentions: (home.intention, away.intention),
            orders: (
                ds_model::TeamOrders::default(),
                ds_model::TeamOrders::default(),
            ),
            transition: (0, 0),
            pitch_cm: (length, width),
            goal_half_width_cm: crate::balance::LawsBalance::load().goal_half_width_cm,
            home_in_possession: true,
            carry_target: None,
            touch: None,
            won: None,
            carrier_reach: (i64::from(BODY_RADIUS_CM), 0),
            acts: BTreeMap::new(),
            act_targets: BTreeMap::new(),
            sent: BTreeMap::new(),
            hold_at: BTreeMap::new(),
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

    /// The place `id` chose to hold, with his side on the ball (ADR-071);
    /// `None` clears it (his side lost the ball: the shape again).
    pub fn set_hold_at(&mut self, id: PlayerId, place: Option<(i32, i32)>) {
        match place {
            Some(p) => {
                // SIM 176: a place within a body of the one he is already
                // holding is the same space. Without this dead band a read
                // that flips between two near-identical spots made him brake
                // and set off again every decision (the automaton's dance).
                let same = self.hold_at.get(&id).is_some_and(|held| {
                    dmath::hypot(f64::from(p.0 - held.0), f64::from(p.1 - held.1))
                        <= f64::from(BODY_RADIUS_CM)
                });
                if !same {
                    self.hold_at.insert(id, p);
                }
            }
            None => {
                self.hold_at.remove(&id);
            }
        }
    }

    /// Whether `id`'s place in the shape is on its back line, with the
    /// ball (ADR-071: the preventive defence), and the depth of the block
    /// with the ball, cm (three lines in it).
    #[must_use]
    pub fn back_line(&self, id: PlayerId, ball: (i32, i32)) -> (bool, i32) {
        let depth = self.block.depth_in_possession_cm;
        let Some(slot) = self.roles.get(&id).copied() else {
            return (false, depth);
        };
        let home = crate::perception::team_of(id) == 0;
        let board = if home { &self.boards.0 } else { &self.boards.1 };
        let length = self.pitch_cm.0.max(1);
        let along = if home { ball.0 } else { length - ball.0 };
        let role: crate::shape::Role = board.spot(slot, true, along * 1000 / length).into();
        (role.depth == 0, depth)
    }

    /// Sends `id` to `place` for a restart, running or not (SIM 150): he
    /// gets there at his own pace, the restart waits.
    pub fn send(&mut self, id: PlayerId, place: (i32, i32), run: bool) {
        self.sent.insert(id, (place, run));
    }

    /// Where `id` was sent for the restart, if he was.
    #[must_use]
    pub fn sent_to(&self, id: PlayerId) -> Option<(i32, i32)> {
        self.sent.get(&id).map(|s| s.0)
    }

    /// Whether everyone sent is where he was sent, within two bodies'
    /// width (the separation keeps two men that far apart).
    #[must_use]
    pub fn all_arrived(&self) -> bool {
        let reach = i64::from(2 * BODY_RADIUS_CM);
        self.sent.iter().all(|(id, (place, _))| {
            self.agents
                .get(id)
                .is_none_or(|a| distance_squared(a.position, *place) <= reach * reach)
        })
    }

    /// Whether every player but the kicker is in his own half (Law 8, the
    /// kick-off): the home side attacks from the left.
    #[must_use]
    pub fn all_in_own_half(&self, kicker: PlayerId) -> bool {
        let half = self.pitch_cm.0 / 2;
        self.agents
            .iter()
            .filter(|(id, _)| **id != kicker)
            .all(|(id, a)| {
                if crate::perception::team_of(*id) == 0 {
                    a.position.0 <= half
                } else {
                    a.position.0 >= half
                }
            })
    }

    /// For the kick-off (Law 8): everyone but `kicker` out of his half and
    /// not yet sent is sent, walking, to his place in his side's block.
    pub fn send_back_to_own_half(&mut self, kicker: PlayerId, spot: (i32, i32)) {
        let half = self.pitch_cm.0 / 2;
        let out: Vec<PlayerId> = self
            .agents
            .iter()
            .filter(|(id, a)| {
                **id != kicker
                    && !self.sent.contains_key(*id)
                    && if crate::perception::team_of(**id) == 0 {
                        a.position.0 > half
                    } else {
                        a.position.0 < half
                    }
            })
            .map(|(id, _)| *id)
            .collect();
        for id in out {
            let mine = (crate::perception::team_of(id) == 0) == self.home_in_possession;
            let place = self.block_target(id, spot, mine, true).unwrap_or_else(|| {
                // A keeper: his goal.
                let x = if crate::perception::team_of(id) == 0 {
                    0
                } else {
                    self.pitch_cm.0
                };
                (x, self.pitch_cm.1 / 2)
            });
            self.sent.insert(id, (place, false));
        }
    }

    /// The restart is taken: nobody is sent any more.
    pub fn clear_sent(&mut self) {
        self.sent.clear();
    }

    /// Where a player who chose to offer or to run is going.
    pub fn set_act_target(&mut self, id: PlayerId, target: (i32, i32)) {
        self.act_targets.insert(id, target);
    }

    /// A substitution (Law 3, plan L0-CLOSE 10f): `incoming` takes the place
    /// of `outgoing` on the pitch, where he was, in his role in the shape,
    /// from a standstill. Returns `false` if `outgoing` is not on the pitch.
    pub fn substitute(&mut self, outgoing: PlayerId, incoming: PlayerId) -> bool {
        let Some(mut agent) = self.agents.remove(&outgoing) else {
            return false;
        };
        // He comes on at the halfway line, on the side of the benches (Law
        // 3; the benches by the touchline at y = 0, declared), and runs to
        // his place from there (SIM 150). Before, he appeared where his man
        // went off.
        let entry = (self.pitch_cm.0 / 2, 0);
        agent.position = entry;
        agent.fine = (
            (i64::from(entry.0) * FINE, i64::from(entry.1) * FINE),
            (0, 0),
        );
        agent.velocity = (0, 0);
        self.agents.insert(incoming, agent);
        if let Some(role) = self.roles.remove(&outgoing) {
            self.roles.insert(incoming, role);
        }
        self.acts.remove(&outgoing);
        self.act_targets.remove(&outgoing);
        if self.goalkeepers.0 == outgoing {
            self.goalkeepers.0 = incoming;
        }
        if self.goalkeepers.1 == outgoing {
            self.goalkeepers.1 = incoming;
        }
        if self.carrier == Some(outgoing) {
            self.carrier = Some(incoming);
        }
        true
    }

    /// Where a player's chosen act takes him (offer, run, mark), if it
    /// has a place: what a teammate sees of his movement.
    #[must_use]
    pub fn act_target(&self, id: PlayerId) -> Option<(i32, i32)> {
        self.act_targets.get(&id).copied()
    }

    /// A player's off-ball act, if he has chosen one.
    #[must_use]
    pub fn act(&self, id: PlayerId) -> Option<ds_orchestrator::OffBallAct> {
        self.acts.get(&id).copied()
    }

    /// The side in possession, for the block (set by the match each tick
    /// `tick`): when it changes, both sides are in their transition for
    /// `window` ticks (ADR-059 §6).
    pub fn set_home_in_possession(&mut self, home: bool, tick: u64, window: u64) {
        if home != self.home_in_possession {
            self.transition.0 = tick + window;
            // The places chosen with the ball are of that possession
            // (ADR-071).
            self.hold_at.clear();
        }
        self.home_in_possession = home;
        self.transition.1 = tick;
    }

    /// The foot a man who gets the ball has it on (ADR-031): on the wing,
    /// the outside foot, the one towards the touchline he is near, in his
    /// own frame (attacking `+x`, his left is `+y`); in the middle third of
    /// the pitch's width, in front of him.
    fn natural_foot(&self, p: PlayerId) -> Option<ds_orchestrator::Side> {
        let y = self.agents.get(&p)?.position.1;
        self.natural_foot_at(p, y)
    }

    /// The foot the ball is on for `p` at the width `y` (ADR-031): the
    /// outside foot on the wing, none in the middle third.
    #[must_use]
    pub fn natural_foot_at(&self, p: PlayerId, y: i32) -> Option<ds_orchestrator::Side> {
        let width = self.pitch_cm.1.max(1);
        let home = crate::perception::team_of(p) == 0;
        let high = y > width - width / 3;
        let low = y < width / 3;
        match (home, low, high) {
            (true, false, true) | (false, true, false) => Some(ds_orchestrator::Side::Left),
            (true, true, false) | (false, false, true) => Some(ds_orchestrator::Side::Right),
            _ => None,
        }
    }

    /// Where the carrier is taking the ball, if he chose to carry it.
    #[must_use]
    pub const fn carry_target(&self) -> Option<(i32, i32)> {
        self.carry_target
    }

    /// The foot the carrier has the ball on (ADR-031): the one he touched
    /// it onto, else the foot of the side where he is (the outside foot on
    /// the wing), else none, in front of him.
    #[must_use]
    pub fn ball_foot(&self) -> Option<ds_orchestrator::Side> {
        self.ball_foot
            .or_else(|| self.carrier.and_then(|p| self.natural_foot(p)))
    }

    /// He touches the ball onto `foot` (ADR-031): his choice.
    pub const fn set_ball_foot(&mut self, foot: Option<ds_orchestrator::Side>) {
        self.ball_foot = foot;
    }

    /// What his coach asks of a player in this phase (ADR-059 step 4): the
    /// responsibility of his slot on the board, as the act it means.
    #[must_use]
    pub fn asked(&self, id: PlayerId, with_ball: bool) -> Option<ds_orchestrator::OffBallAct> {
        use ds_model::Task as T;
        use ds_orchestrator::OffBallAct as A;
        let slot = (*self.roles.get(&id)?).min(10);
        let home = crate::perception::team_of(id) == 0;
        let board = if home { &self.boards.0 } else { &self.boards.1 };
        let task = if with_ball {
            board.with_ball_tasks[slot]
        } else {
            board.without_ball_tasks[slot]
        };
        match task {
            T::Free => None,
            T::Run => Some(A::Run),
            T::Offer => Some(A::Offer),
            T::Stay => Some(A::Hold),
            T::Press => Some(A::Press),
            T::Cover => Some(A::Cover),
            T::Mark => Some(A::Mark),
        }
    }

    /// A side's board as its players have it now.
    #[must_use]
    pub const fn board(&self, home: bool) -> &ds_model::Board {
        if home { &self.boards.0 } else { &self.boards.1 }
    }

    /// The coach of one side changes his whole board (decision AG): from
    /// now on his players read it.
    pub const fn set_board(&mut self, home: bool, board: ds_model::Board) {
        if home {
            self.boards.0 = board;
        } else {
            self.boards.1 = board;
        }
    }

    /// The coach of one side changes his quick instructions or his
    /// transition choices (ADR-059 step 3).
    pub const fn set_orders(&mut self, home: bool, orders: ds_model::TeamOrders) {
        if home {
            self.orders.0 = orders;
        } else {
            self.orders.1 = orders;
        }
    }

    /// The quick instructions and transition choices of one side.
    #[must_use]
    pub const fn orders(&self, home: bool) -> ds_model::TeamOrders {
        if home { self.orders.0 } else { self.orders.1 }
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
        let slot = *self.roles.get(&id)?;
        let home = crate::perception::team_of(id) == 0;
        let board = if home { &self.boards.0 } else { &self.boards.1 };
        let length = self.pitch_cm.0.max(1);
        let along = if home { ball.0 } else { length - ball.0 };
        let role = board
            .spot(slot, in_possession, along * 1000 / length)
            .into();
        let situation = crate::shape::Situation {
            ball,
            in_possession,
            home,
            pitch: self.pitch_cm,
            kick_off: own_half,
            intention: if home {
                self.intentions.0
            } else {
                self.intentions.1
            },
            orders: self.orders(home),
            transition: self.transition.1 < self.transition.0,
        };
        let mut at = crate::shape::place(role, situation, &self.block);
        // Asked to attack the depth (ADR-070), with the ball he holds on
        // the last defender's shoulder, onside, in his own lane: from there
        // he goes. Before, the ask counted only when the run was already
        // one of his options, and a striker behind the ball never had it.
        if in_possession && board.with_ball_tasks[slot.min(10)] == ds_model::Task::Run {
            let forward = if home { 1 } else { -1 };
            if let Some(line) = self.second_last(home) {
                let x = line - forward * self.onside_margin_cm;
                if (x - at.0) * forward > 0 {
                    at.0 = x.clamp(0, length);
                }
            }
        }
        Some(at)
    }

    /// The line of the second-last opponent of side `home` (Law 11), along
    /// the pitch, as they stand.
    fn second_last(&self, home: bool) -> Option<i32> {
        let mut xs: Vec<i32> = self
            .agents
            .iter()
            .filter(|(id, _)| (crate::perception::team_of(**id) == 0) != home)
            .map(|(_, a)| a.position.0)
            .collect();
        if home {
            xs.sort_unstable_by(|a, b| b.cmp(a));
        } else {
            xs.sort_unstable();
        }
        xs.get(1).copied()
    }

    /// What the coach's team orders ask of `id` now (ADR-070): the quick
    /// instructions and the transitions as asks in his choice, not only as
    /// a place in the block. `with_ball` is whether his side has the ball;
    /// `rank` how many teammates are nearer the opposing carrier than him.
    /// Without the ball: more pressing, or the counter-press in the
    /// seconds after losing it, ask the two nearest to press; the drop
    /// back asks everyone to hold. With it, in the seconds after winning
    /// it: the counter asks to run, keeping it asks to offer.
    #[must_use]
    pub fn team_asked(
        &self,
        id: PlayerId,
        with_ball: bool,
        rank: Option<u32>,
    ) -> Option<ds_orchestrator::OffBallAct> {
        use ds_orchestrator::OffBallAct as A;
        let home = crate::perception::team_of(id) == 0;
        let o = self.orders(home);
        let transition = self.transition.1 < self.transition.0;
        let near = rank.is_some_and(|r| r <= 1);
        if with_ball {
            return match (transition, o.won) {
                (true, ds_model::OnWin::Counter) => Some(A::Run),
                (true, ds_model::OnWin::Keep) => Some(A::Offer),
                _ => None,
            };
        }
        rank?;
        match (transition, o.lost) {
            (true, ds_model::OnLoss::CounterPress) if near => Some(A::Press),
            (true, ds_model::OnLoss::DropBack) => Some(A::Hold),
            _ => (o.press_cm > 0 && near).then_some(A::Press),
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

    /// The air's friction on the ball, thousandths per tick: what a passer
    /// knows of a ball in flight (`ball.json`).
    #[must_use]
    pub const fn air_friction_milli(&self) -> i32 {
        self.ball_physics.air_friction_milli
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

    /// Where a player at `from` first meets the moving ball, by the physics
    /// `advance_ball` applies, the same steps (gravity and the air in
    /// flight, the grass once it is down, the stop) and his own: from his
    /// speed now he accelerates at `accel` up to `top` (cm/tick), and he
    /// needs only to come within `reach` cm of it. The first point of its
    /// path within his height (at most head height, coming down or on the
    /// grass) that he gets to no later than it does, or where it stops. His
    /// speed counts only as far as it takes him towards each point (SIM 194):
    /// a man running across or away starts from nothing that way. Before, his
    /// whole speed counted whichever way he ran, and a keeper drifting along
    /// his line went for a point far up the path that he never reached, and
    /// the back pass ran past him into the net. A
    /// ball in the air he meets where it comes down if he gets there in
    /// time, else further on (`SIM_VERSION` 111, OI-102: a keeper who could
    /// not reach where a lofted back pass came down chased it where it was,
    /// and it ran past him into the net). `None` if the ball is at
    /// someone's feet or still.
    #[must_use]
    pub fn intercept(
        &self,
        from: (i32, i32),
        (velocity, accel, top): ((f64, f64), f64, f64),
        reach: f64,
    ) -> Option<(i32, i32)> {
        if matches!(self.ball, BallState::Possessed) {
            return None;
        }
        let (_, _, z0) = self.ball_position;
        let vel = self.ball_velocity;
        if z0 == 0 && vel.2 <= 0 && vel.0 == 0 && vel.1 == 0 {
            return None;
        }
        // In the same small steps the ball moves in (ADR-069, SIM 149), so
        // where he goes to meet it is where it passes.
        let kick = FineKick::of(&self.ball_physics);
        let mut fine = if self.ball_fine.synced == (self.ball_position, vel) {
            self.ball_fine
        } else {
            FineBall::from_cm(self.ball_position, vel)
        };
        let mut stopped = false;
        let mut at = None;
        for t in 1..=80_u32 {
            if !stopped {
                stopped = fine.tick(&kick);
            }
            let ((x, y, z), (_, _, vz)) = fine.to_cm();
            let vz = if stopped { 0 } else { vz };
            // Within his height: coming down to his head or lower, or on the
            // grass; a ball still climbing is not his yet.
            if z > HEAD_HEIGHT_CM || (z > 0 && vz > 0) {
                continue;
            }
            at = Some((x, y));
            #[allow(clippy::cast_precision_loss)]
            let way = (distance_squared(from, (x, y)) as f64).sqrt();
            let toward = if way > 0.0 {
                velocity
                    .0
                    .mul_add(f64::from(x - from.0), velocity.1 * f64::from(y - from.1))
                    / way
            } else {
                0.0
            };
            let mine = (way - reach).max(0.0);
            if mine <= run_in(f64::from(t), toward.max(0.0), accel, top) {
                return at;
            }
        }
        at
    }

    /// Where a ball in the air comes down to `height_cm`, by the same
    /// physics `advance_ball` applies (gravity, air friction): where a
    /// player who reads it goes to meet it, from the moment it is struck.
    /// `None` if it is below that height and not rising.
    #[must_use]
    pub fn drop_point(&self, height_cm: i32) -> Option<(i32, i32)> {
        let BallState::InFlight = self.ball else {
            return None;
        };
        let z = self.ball_position.2;
        // Below that height and not rising, it is not in the air for him; a
        // ball just struck and still climbing he reads from the kick.
        if z <= height_cm && self.ball_velocity.2 <= 0 {
            return None;
        }
        // In the same small steps the ball flies in (ADR-069, SIM 149):
        // before, it was read tick by tick, and where it came down was not
        // quite where it was read to come down.
        let k = FineKick::of(&self.ball_physics);
        let mut fine = if self.ball_fine.synced == (self.ball_position, self.ball_velocity) {
            self.ball_fine
        } else {
            FineBall::from_cm(self.ball_position, self.ball_velocity)
        };
        // Only a ball that goes above that height comes down to it.
        let mut above = z > height_cm;
        let mut last = self.ball_position;
        for _ in 0..200 {
            let mut path = [(0, 0, 0); STEPS];
            let stopped = fine.tick_steps(&k, &mut path);
            for b in path {
                above |= b.2 > height_cm;
                if b.2 <= height_cm && b.2 < last.2 {
                    return above.then_some((b.0, b.1));
                }
                last = b;
            }
            if stopped {
                break;
            }
        }
        above.then_some((last.0, last.1))
    }

    /// Where the ball was at the end of each small step of this tick
    /// (ADR-069, SIM 148); a ball struck, placed or met since it moved is
    /// only where it is now.
    #[must_use]
    pub fn ball_path(&self) -> [(i32, i32, i32); STEPS] {
        if self.ball_steps[STEPS - 1] == self.ball_position {
            self.ball_steps
        } else {
            [self.ball_position; STEPS]
        }
    }

    /// Where `id` was at the end of small step `s` of this tick (ADR-069,
    /// SIM 148); a man placed since he moved is only where he is now.
    #[must_use]
    pub fn body_at_step(&self, id: PlayerId, s: usize) -> Option<(i32, i32)> {
        let now = self.agents.get(&id)?.position;
        Some(
            self.body_steps
                .get(&id)
                .filter(|p| p[STEPS - 1] == now)
                .map_or(now, |p| p[s.min(STEPS - 1)]),
        )
    }

    /// Advances the ball one tick (MD-17/18, ADR-083).
    ///
    /// Applies gravity to `vz`, gravity to `z` and friction to the horizontal
    /// velocity to every ball: a ball under a foot has velocity zero and stays
    /// where his foot puts it; a free ball keeps rolling after it crosses the
    /// stop threshold, until the declared friction spends its last speed.
    /// A restart waiting for its kicker is not advanced at all: the match's
    /// step returns before this (SIM 150). The constants are declared (MD-24)
    /// in `balance/ball.json`.
    pub fn advance_ball(&mut self) {
        self.ball_steps = [self.ball_position; STEPS];
        // In small steps (decision BJ, ADR-069), in hundredths of a
        // centimetre; whoever struck, deflected or placed the ball in
        // centimetres since the last tick sets the fine state afresh.
        let k = FineKick::of(&self.ball_physics);
        if self.ball_fine.synced != (self.ball_position, self.ball_velocity) {
            self.ball_fine = FineBall::from_cm(self.ball_position, self.ball_velocity);
        }
        let stopped = self.ball_fine.tick_steps(&k, &mut self.ball_steps);
        let (position, velocity) = self.ball_fine.to_cm();
        self.ball_position = position;
        self.ball_velocity = velocity;
        self.ball_fine.synced = (position, velocity);
        if stopped && self.ball == BallState::InFlight {
            self.ball = BallState::Loose;
        }
    }

    /// Where a ball struck from `from` with `velocity` crosses the line
    /// `x = line_x`, by the same step the ball moves with: `(y, z)` at the
    /// crossing, or `None` if it stops (or turns back) before reaching it.
    #[must_use]
    pub fn ball_at_line(
        &self,
        from: (i32, i32, i32),
        velocity: (i16, i16, i16),
        line_x: i32,
    ) -> Option<(i32, i32)> {
        let p = self.ball_physics;
        let towards = (line_x - from.0).signum();
        if towards == 0 {
            return Some((from.1, from.2));
        }
        // The same small steps the ball moves with (ADR-069), so what is
        // foreseen is what happens.
        let k = FineKick::of(&p);
        let mut fine = FineBall::from_cm(from, velocity);
        let mut pos = from;
        for _ in 0..600 {
            let stopped = fine.tick(&k);
            let (next, _) = fine.to_cm();
            if (line_x - next.0).signum() != towards {
                let dx = i64::from(next.0 - pos.0).max(1);
                let t = i64::from(line_x - pos.0);
                let at = |a: i32, b: i32| a + i32::try_from(i64::from(b - a) * t / dx).unwrap_or(0);
                return Some((at(pos.1, next.1), at(pos.2, next.2)));
            }
            if stopped || (next.0 - pos.0).signum() != towards {
                return None;
            }
            pos = next;
        }
        None
    }

    /// The coach of one side changes his team's intention (ADR-059): from
    /// now on his players read it.
    pub const fn set_intention(&mut self, home: bool, intention: ds_model::Intention) {
        if home {
            self.intentions.0 = intention;
        } else {
            self.intentions.1 = intention;
        }
    }

    /// The agent carrying the ball, if any.
    #[must_use]
    pub const fn carrier(&self) -> Option<PlayerId> {
        self.carrier
    }

    /// The last touches of the ball.
    #[must_use]
    pub const fn touches(&self) -> Touches {
        self.touches
    }

    /// `who` plays the ball on purpose without keeping it (a header, a
    /// first-time clearance): he is now the last man who played it.
    pub const fn played_by(&mut self, who: PlayerId) {
        self.touches = Touches {
            played: Some(who),
            shot: false,
            deflected: None,
        };
    }

    /// The last man who played the ball has struck it at goal.
    pub const fn struck_shot(&mut self) {
        self.touches.shot = true;
    }

    /// The ball touched `who` on its way (a parry, a block, a ball he did
    /// not control): it was not his play.
    pub const fn deflected_by(&mut self, who: PlayerId) {
        self.touches.deflected = Some(who);
    }

    /// Sets the carrier. `None` means the ball is loose.
    pub fn set_carrier(&mut self, carrier: Option<PlayerId>) {
        if carrier != self.carrier {
            self.ball_foot = None;
        }
        self.carrier = carrier;
        self.carry_target = None;
        self.touch = None;
        self.won = None;
        if let Some(p) = carrier {
            self.touches = Touches {
                played: Some(p),
                shot: false,
                deflected: None,
            };
            if !matches!(self.ball, BallState::Possessed) {
                self.won = Some((self.ball_velocity.0, self.ball_velocity.1));
            }
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

    /// The carrier plays the ball ahead and meets it again (plan L0-CLOSE
    /// 11): at each touch he pushes it where he chose to take it, weighed to come back
    /// to his foot after `period_of(carrier, pace)` ticks; between touches
    /// it rolls on its own, slowed by the ground. Standing, or with the ball
    /// in his hands (`hold`), it is at his feet. If he has not reached it
    /// when the touch is due (he turned, he was held up) it rolls on until
    /// he does. A no-op if nobody carries it.
    #[allow(clippy::too_many_lines)] // the won-branch now reads the receiver's run
    pub fn follow_carrier(
        &mut self,
        tick: u64,
        hold: bool,
        mut period_of: impl FnMut(PlayerId, f64) -> f64,
        mut spill_of: impl FnMut(PlayerId, f64) -> f64,
        reach_cm: i64,
    ) {
        if !matches!(self.ball, BallState::Possessed) {
            return;
        }
        let Some((carrier, agent)) = self
            .carrier
            .and_then(|c| self.agents.get(&c).map(|a| (c, *a)))
        else {
            return;
        };
        let pos = agent.position;
        let keep = f64::from(self.ball_physics.ground_friction_milli) / 1000.0;
        #[allow(clippy::cast_precision_loss)]
        let since = |touch: &Touch| tick.saturating_sub(touch.at) as f64;
        // He touches it again where it is, as soon as it is within his
        // reach (the maintainer, 2026-11-13: players circled the ball they
        // could not take; before, he had to have it within his body's width,
        // which the bodies around it often kept him from).
        let reach = reach_cm.max(i64::from(BODY_RADIUS_CM));
        self.carrier_reach = (reach, tick);
        let gap = distance_squared(pos, (self.ball_position.0, self.ball_position.1));
        let reached = gap <= reach.pow(2);
        // At his feet: within his body's width, nothing to bring under him.
        let at_body = gap <= i64::from(BODY_RADIUS_CM).pow(2);
        let due = self.touch.as_ref().is_none_or(|tc| since(tc) >= tc.period);
        // A ball brought under his foot is at his feet when the step ends.
        let reached = reached
            || self
                .touch
                .as_ref()
                .is_some_and(|tc| tc.held_cm > 0.0 && since(tc) >= tc.period);
        if hold {
            self.touch = None;
        } else if let Some(v) = self.won.take().filter(|_| self.touch.is_none()) {
            // He has just won it (the maintainer, 2026-11-13: before, the
            // ball jumped to the middle of his body, as if drawn to him).
            // His first touch takes it where it is: the better it is, the
            // more of its speed it kills. What is left runs on, and he goes
            // after it; a dead ball he brings under his foot in a step.
            let (vx, vy) = (f64::from(v.0), f64::from(v.1));
            let incoming = dmath::hypot(vx, vy);
            let spill = spill_of(carrier, incoming);
            let from = (
                f64::from(self.ball_position.0),
                f64::from(self.ball_position.1),
            );
            // OI-123 (ADR-058 §5): the first touch is his, not the ball's.
            // Running into the pass he takes it into his own stride; playing
            // it back where it came from would leave it behind for him to
            // fetch (the maintainer, 2026-11-21: it never happens in a real
            // match). The same touch as any other: his pace sets where and
            // how far it rolls, and he meets it again when it is due.
            let run = (f64::from(agent.velocity.0), f64::from(agent.velocity.1));
            let pace = dmath::hypot(run.0, run.1);
            // The sign gates a touch: keep the arithmetic bit-identical to SIM 177.
            #[allow(clippy::suboptimal_flops)]
            let with_the_ball = run.0 * vx + run.1 * vy;
            if spill >= 1.0 {
                // He also takes it with him when he is simply faster than the
                // ball he has just won: the same reading of the ball's roll
                // and his own stride, not a new rule (OI-123, SIM 178).
                let (dir, speed, period) = if pace >= 1.0 && (with_the_ball < 0.0 || spill < pace) {
                    let period = period_of(carrier, pace).max(1.0);
                    (
                        (run.0 / pace, run.1 / pace),
                        touch_speed(pace, period, keep),
                        period,
                    )
                } else {
                    ((vx / incoming, vy / incoming), spill, 0.0)
                };
                self.touch = Some(Touch {
                    from,
                    dir,
                    speed,
                    at: tick,
                    period,
                    held_cm: 0.0,
                });
            } else if !at_body {
                let off = (from.0 - f64::from(pos.0), from.1 - f64::from(pos.1));
                self.touch = Some(under_foot(off, tick, period_of(carrier, 0.0)));
            }
        } else if self.touch.is_none() && !at_body {
            // At his feet in no other way (a restart, the keeper putting it
            // down): under his foot in a step.
            let off = (
                f64::from(self.ball_position.0 - pos.0),
                f64::from(self.ball_position.1 - pos.1),
            );
            self.touch = Some(under_foot(off, tick, period_of(carrier, 0.0)));
        } else if due
            && !reached
            && self
                .touch
                .as_ref()
                .is_some_and(|tc| tc.period > 0.0 && since(tc) >= 2.0 * tc.period)
        {
            // SIM 182 (DD-39): a touch played ahead that he never reached
            // (bodies held him) came to rest out of his reach; waiting for
            // him to meet it froze the ball and left the carrier standing.
            // After two of his own touch steps, bring it back under his foot.
            let off = (
                f64::from(self.ball_position.0 - pos.0),
                f64::from(self.ball_position.1 - pos.1),
            );
            self.touch = Some(under_foot(off, tick, period_of(carrier, 0.0)));
        } else if due && (reached || self.touch.is_none()) {
            // He plays it where he chose to take it, at the pace he runs
            // that way; holding it, he keeps it at his feet.
            let (vx, vy) = (f64::from(agent.velocity.0), f64::from(agent.velocity.1));
            let dir = self.carry_target.and_then(|to| {
                let (dx, dy) = (f64::from(to.0 - pos.0), f64::from(to.1 - pos.1));
                let len = dmath::hypot(dx, dy);
                (len >= 1.0).then(|| (dx / len, dy / len))
            });
            let pace = dir.map_or(0.0, |d| vx.mul_add(d.0, vy * d.1));
            // Turning by more than 60 degrees (the turn of the movement,
            // plan L0-CLOSE 10e), he keeps it under his sole while he turns,
            // and plays it ahead once he runs that way (SIM 151): before, he
            // played it the new way while his body still went the old one,
            // and left it behind him.
            self.touch = match dir {
                Some(dir) if pace >= 1.0 && pace >= 0.5 * dmath::hypot(vx, vy) => {
                    let period = period_of(carrier, pace).max(1.0);
                    // He meets it where it is, which may be a step behind
                    // him (it lay where he was a tick ago): the touch takes
                    // that gap too, so the ball runs ahead of him and comes
                    // back to his foot at the end of his steps (SIM 151,
                    // pure error 2b, `BEHAVIOURS.md` 8). Before, it was
                    // weighed as if it started at his feet, and it stayed a
                    // tick behind him: 31% of his ticks more than half a
                    // metre behind where he looked.
                    // He touched it as he reached it, within this tick: it
                    // has rolled this tick already, and he meets it again
                    // `period` ticks from then.
                    let behind = f64::from(pos.0 - self.ball_position.0)
                        .mul_add(dir.0, f64::from(pos.1 - self.ball_position.1) * dir.1);
                    let run = pace * period;
                    let stretch = pace.mul_add(period - 1.0, behind.max(0.0)) / run;
                    Some(Touch {
                        from: (
                            f64::from(self.ball_position.0),
                            f64::from(self.ball_position.1),
                        ),
                        dir,
                        speed: touch_speed(pace, period, keep) * stretch,
                        at: tick.saturating_sub(1),
                        period,
                        held_cm: 0.0,
                    })
                }
                _ => None,
            };
        }
        let at = self.touch.as_ref().map_or(pos, |tc| {
            if tc.held_cm > 0.0 {
                // Under his foot: from where he took it to his feet, in a step.
                let left = (1.0 - since(tc) / tc.period).clamp(0.0, 1.0);
                #[allow(clippy::cast_possible_truncation)]
                return (
                    tc.from.0.mul_add(left, f64::from(pos.0)).round() as i32,
                    tc.from.1.mul_add(left, f64::from(pos.1)).round() as i32,
                );
            }
            let d = rolled(tc.speed, since(tc), keep);
            #[allow(clippy::cast_possible_truncation)]
            (
                tc.dir.0.mul_add(d, tc.from.0).round() as i32,
                tc.dir.1.mul_add(d, tc.from.1).round() as i32,
            )
        });
        self.ball_position = (at.0, at.1, 0);
    }

    /// Whether the carrier can play the ball now: it is at his foot, not
    /// rolling ahead between two touches (plan L0-CLOSE 11).
    #[must_use]
    pub fn ball_at_feet(&self) -> bool {
        // A ball he has just controlled is under his foot as far as he
        // reached for it; one played ahead is his again within his reach
        // once he meets it (the touch is due); rolling in between, it is not.
        let (reach, now) = self.carrier_reach;
        #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
        let foot = match &self.touch {
            None => reach,
            Some(tc) if tc.held_cm > 0.0 => i64::from(BODY_RADIUS_CM).max(tc.held_cm.ceil() as i64),
            Some(tc) if now.saturating_sub(tc.at) as f64 >= tc.period => reach,
            Some(_) => i64::from(BODY_RADIUS_CM),
        };
        self.carrier
            .and_then(|c| self.agents.get(&c))
            .is_some_and(|a| {
                distance_squared(a.position, (self.ball_position.0, self.ball_position.1))
                    <= foot.pow(2)
            })
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
            let d = dmath::hypot(delta_x, delta_y);
            let t = (1.0 - d / range).clamp(0.0, 1.0);
            let sign = if goal_x == 0.0 { 1.0_f64 } else { -1.0_f64 };
            // Never deeper than a share of the ball's depth: he stays
            // between the ball and his goal, on his line when the ball is
            // on the line beside the post (ADR-051).
            let depth_cap = delta_x.abs() * f64::from(keeper_balance.depth_share_milli) / 1000.0;
            // 10d: how far he leaves his line is his own read, not a fixed
            // number. A keeper who reads the play comes off his line to
            // narrow the angle and claim space; one who does not stays
            // ready on his line (ADR-078).
            let read = hidden.foresight(gk);
            let advance_cm = ds_calculator::keeper::line_depth_cm(advance, t, read);
            let off_line = (advance_cm + f64::from(GK_LINE_OFFSET_CM)).min(depth_cap);
            let x_target = f64::mul_add(sign, off_line, goal_x);
            // Across, he stands on the bisector of the angle the ball makes
            // with his two posts, at his depth (SIM_VERSION 124): there he
            // covers the near and the far side alike. Before, he stood on the
            // line from the ball to the middle of the goal, and a ball out
            // wide left him nearer the far post than the angle wanted.
            let y_target = keeper_across(
                (f64::from(ball.0), f64::from(ball.1)),
                (goal_x, goal_y),
                f64::from(self.goal_half_width_cm),
                x_target,
            );
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
                        intention: if home {
                            self.intentions.0
                        } else {
                            self.intentions.1
                        },
                        orders: self.orders(home),
                        transition: self.transition.1 < self.transition.0,
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
        #[allow(clippy::cast_precision_loss)]
        let overdue = self
            .touch
            .as_ref()
            .is_some_and(|tc| tick.saturating_sub(tc.at) as f64 >= tc.period)
            // SIM 195: a ball he has won away from his feet, with no touch on
            // its way (a challenge won where the contact was): he goes to it.
            // Before, he stood there and never decided, as he decides only
            // with the ball at his foot (seed 42: 39 minutes).
            || (self.touch.is_none() && self.carrier.is_some() && !self.ball_at_feet());
        // The ball he played ahead (SIM 151, pure error 2b): where it will be
        // when he meets it again, and in how many ticks. He runs there to
        // arrive with it, not past it.
        let keep = f64::from(self.ball_physics.ground_friction_milli) / 1000.0;
        #[allow(clippy::cast_precision_loss)]
        let meet = self
            .touch
            .as_ref()
            .filter(|tc| tc.held_cm <= 0.0 && tc.speed > 0.0 && tc.period > 0.0)
            .map(|tc| {
                let d = rolled(tc.speed, tc.period, keep);
                let at = (
                    tc.dir.0.mul_add(d, tc.from.0),
                    tc.dir.1.mul_add(d, tc.from.1),
                );
                let left = (tc.at as f64 + tc.period - tick as f64).max(1.0);
                #[allow(clippy::cast_possible_truncation)]
                ((at.0.round() as i32, at.1.round() as i32), left)
            });
        let mut fines: Vec<(PlayerId, FineBody)> = Vec::with_capacity(work.len());
        // His top speed this tick, cm a tick: a push of the separation does
        // not move him faster than his legs (SIM 150).
        let mut legs: BTreeMap<PlayerId, f64> = BTreeMap::new();
        let mut steps_of: Vec<(PlayerId, [(i32, i32); STEPS])> = Vec::with_capacity(work.len());
        for entry in &mut work {
            let act = self.acts.get(&entry.0).copied();
            let sent = self.sent.get(&entry.0).copied();
            let to_the_ball = sent.is_none()
                && matches!(
                    act,
                    Some(
                        ds_orchestrator::OffBallAct::Press
                            | ds_orchestrator::OffBallAct::Challenge
                            | ds_orchestrator::OffBallAct::GoToBall
                    )
                );
            let target = if let Some((place, _)) = sent {
                place
            } else if Some(entry.0) == self.carrier {
                // The carrier holds the ball where he is, or carries it
                // where he chose (IDEA-DESIGN-010); the keeper too.
                // A ball rolling past the moment he meant to meet it
                // (he turned, he was held up): he goes to it.
                if overdue {
                    ball
                } else if let Some((at, _)) = meet {
                    at
                } else {
                    self.carry_target.unwrap_or(entry.1)
                }
            } else if to_the_ball {
                // A high ball is met where it comes down (head height),
                // not chased where it is. The press aims at the opposing
                // player, not at the ball that can lie ahead of his foot:
                // he is the target, the ball is only contested (SIM 181).
                if act == Some(ds_orchestrator::OffBallAct::GoToBall) {
                    drop.unwrap_or(ball)
                } else if act == Some(ds_orchestrator::OffBallAct::Press) {
                    self.carrier
                        .and_then(|c| self.agents.get(&c))
                        .map_or(ball, |a| a.position)
                } else {
                    ball
                }
            } else if let (
                Some(
                    ds_orchestrator::OffBallAct::Offer
                    | ds_orchestrator::OffBallAct::Run
                    | ds_orchestrator::OffBallAct::Mark
                    | ds_orchestrator::OffBallAct::Cover
                    | ds_orchestrator::OffBallAct::Intercept,
                ),
                Some(t),
            ) = (act, self.act_targets.get(&entry.0))
            {
                *t
            } else if let Some((_, kt)) = keeper_targets.iter().find(|(g, _)| *g == entry.0) {
                *kt
            } else {
                // The place he chose (ADR-071): in his zone with the ball,
                // the space he closes without it; else his side's shape.
                // SIM 176: the place he read keeps guiding him while the ball
                // is in flight too. Before, it was thrown away as soon as no
                // one carried the ball and the side's geometry took over: a
                // man who had chosen a space followed the ball instead, then
                // went back when a carrier appeared again. A possession
                // change clears the place (`set_home_in_possession`).
                self.hold_at
                    .get(&entry.0)
                    .copied()
                    .or_else(|| {
                        self.block_target(
                            entry.0,
                            ball,
                            (crate::perception::team_of(entry.0) == 0) == self.home_in_possession,
                            false,
                        )
                    })
                    .unwrap_or(entry.2)
            };
            let Some(body) = bodies.get(&entry.0) else {
                continue;
            };
            let v_prev = self.agents.get(&entry.0).map_or((0.0, 0.0), |a| {
                (f64::from(a.velocity.0), f64::from(a.velocity.1))
            });
            let mctx = ds_orchestrator::MovementContext {
                player: entry.0,
                height_cm: body.height_cm,
                weight_kg: body.weight_kg,
                fatigue: body.fatigue,
            };
            let outcome = hidden.resolve_movement(&mctx);
            // Displacement space: stride is a positive per-player constant,
            // so the leg speeds and accelerations scale by it.
            let stride = outcome.stride;
            legs.insert(entry.0, outcome.v_max * stride);
            let accel = outcome.acceleration * stride;
            let decel = outcome.deceleration * stride;
            // With the ball he runs at the share his dribbling keeps.
            let top = outcome.v_max
                * stride
                * if Some(entry.0) == self.carrier {
                    outcome.with_ball_share
                } else {
                    1.0
                };
            // A ball rolling on the grass he meets where its path and his
            // run cross, not where it is (the maintainer, 2026-11-13: players
            // circled the ball instead of winning it; a keeper chasing a
            // back pass let it roll past him into his goal).
            let mut target = if to_the_ball
                && act != Some(ds_orchestrator::OffBallAct::Press)
                && Some(entry.0) != self.carrier
            {
                let reach = f64::from(BODY_RADIUS_CM) * 2.0;
                self.intercept(entry.1, (v_prev, accel, top), reach)
                    .unwrap_or(target)
            } else {
                target
            };
            // SIM 181: he still aims at the opponent, but takes the lane
            // goal-side of him — the gap a marker keeps, not a new number.
            if act == Some(ds_orchestrator::OffBallAct::Press) && sent.is_none() {
                let home = crate::perception::team_of(entry.0) == 0;
                let own_goal = (if home { 0 } else { self.pitch_cm.0 }, self.pitch_cm.1 / 2);
                target =
                    crate::state::goal_side_of(target, own_goal, self.press_gap_cm, self.pitch_cm);
            }
            // A body standing on his way he goes round (SIM 150, OI-122):
            // by the side that lengthens his way less.
            let round = self.way_round(entry.0, entry.1, target);
            // His pace is his (2026-11-13): he goes hard when he contests
            // something, up to the real match's fastest running (SIM 175);
            // taking up his place he goes at the speed that gets him there
            // in time.
            let keeper = keeper_targets.iter().any(|(g, _)| *g == entry.0);
            let contests = sent.is_some_and(|(_, run)| run)
                || Some(entry.0) == self.carrier
                || to_the_ball
                || act == Some(ds_orchestrator::OffBallAct::Run);
            let defending = (crate::perception::team_of(entry.0) == 0) != self.home_in_possession;
            // In small steps (decision BJ, ADR-069): the target and his
            // choices are the tick's; his body moves five steps of a fifth
            // of a tick, in hundredths of a centimetre, with his speeds per
            // step a fifth and his accelerations a twenty-fifth of the
            // tick's.
            let steps = f64::from(SUBSTEPS);
            let sprint_s = outcome.v_max * stride / steps;
            let (top_s, accel_s, decel_s) = (
                top / steps,
                accel / (steps * steps),
                decel / (steps * steps),
            );
            let start = (
                (i64::from(entry.1.0) * FINE, i64::from(entry.1.1) * FINE),
                (0, 0),
            );
            let mut fine = self.agents.get(&entry.0).map_or(start, |a| {
                let here = (
                    i64::from(a.position.0) * FINE,
                    i64::from(a.position.1) * FINE,
                );
                // Placed since the last tick (a restart, a push): his
                // fine state starts from where he is.
                if (a.fine.0.0 + FINE / 2).div_euclid(FINE) == i64::from(a.position.0)
                    && (a.fine.0.1 + FINE / 2).div_euclid(FINE) == i64::from(a.position.1)
                {
                    a.fine
                } else {
                    let v = |c: i16| i64::from(c) * FINE / i64::from(SUBSTEPS);
                    (here, (v(a.velocity.0), v(a.velocity.1)))
                }
            });
            let mut path = [entry.1; STEPS];
            #[allow(clippy::cast_precision_loss)] // hundredths of a cm on a pitch
            for at_step in &mut path {
                let at = (fine.0.0 as f64 / FINE as f64, fine.0.1 as f64 / FINE as f64);
                let v_prev = (fine.1.0 as f64 / FINE as f64, fine.1.1 as f64 / FINE as f64);
                // Towards the point beside the body in his way while he has
                // not reached it, then towards where he is going; the ground
                // left is the whole way, round the body.
                let beside = round.filter(|w| {
                    dmath::hypot(f64::from(w.0) - at.0, f64::from(w.1) - at.1)
                        > f64::from(BODY_RADIUS_CM)
                });
                let aim = beside.unwrap_or(target);
                let (dx, dy) = (f64::from(aim.0) - at.0, f64::from(aim.1) - at.1);
                let leg = dmath::hypot(dx, dy);
                let dist = leg
                    + beside.map_or(0.0, |w| {
                        dmath::hypot(f64::from(target.0 - w.0), f64::from(target.1 - w.1))
                    });
                // Without the ball his side must be in its place before the
                // ball can get there: he goes as much faster, against a
                // player as fast as he is, as his place is nearer to him than
                // to the ball. With the ball he takes his place in his own
                // time. The keeper keeps his angle at every moment.
                let in_time = if keeper {
                    top_s
                } else if defending {
                    // As much of it as his work rate gives (plan L0-CLOSE 10c).
                    let to_ball =
                        dmath::hypot(f64::from(target.0 - ball.0), f64::from(target.1 - ball.1));
                    top_s * (dist / to_ball.max(1.0)) * outcome.work_share
                } else {
                    dist / (outcome.reposition_ticks * steps).max(1.0)
                };
                // He brakes to stop at his place; into a contest he goes at speed.
                // With the ball rolling ahead of him, at the pace that brings
                // him to it when it is due.
                let carrying = meet
                    .filter(|_| Some(entry.0) == self.carrier && !overdue)
                    .map(|(_, left)| dist / (left * steps));
                let wanted = carrying
                    .map_or_else(
                        || {
                            if contests {
                                // A man sent to a restart spot, or the carrier
                                // going back to his own ball after his touch
                                // ran out, runs there and brakes to a stop at
                                // it (SIM 173/174): "run" is how he goes, not
                                // a licence to sprint past the ball. He stops
                                // within a body of it, with no momentum left
                                // to carry him beyond it and back again.
                                // SIM 175: even into a contest he runs no
                                // faster than the fastest running seen in a
                                // real match (75/90 of his sprint). A man sent
                                // to a restart spot is placing himself, not
                                // racing for a ball: the brake alone stops
                                // him, and capping it left him short of the
                                // spot with the ball dead.
                                let contest_top = if sent.is_some() {
                                    top_s
                                } else {
                                    top_s * outcome.contest_pace_share
                                };
                                // SIM 194: running to a point (the man he
                                // presses, the ball he meets) he brakes to
                                // arrive, as he does at a restart spot; going
                                // in (a challenge) he goes in at speed.
                                // Before, he ran at every point at full speed
                                // and past it: with the acceleration of a
                                // real player he ran past the man he pressed
                                // and back again, metres each way.
                                if act == Some(ds_orchestrator::OffBallAct::Challenge) {
                                    contest_top
                                } else {
                                    let arrive = f64::from(BODY_RADIUS_CM);
                                    contest_top
                                        .min((2.0 * decel_s * (dist - arrive).max(0.0)).sqrt())
                                }
                            } else {
                                // SIM 176: a place is covered when he is
                                // within a body of it. Braking to the exact
                                // point made him walk back into the push of
                                // `separate` every tick (the automaton's
                                // dance); a body of slack is the same space.
                                // The keeper keeps his angle: he has no
                                // slack, or he would stop following the ball.
                                let arrive = if keeper {
                                    0.0
                                } else {
                                    f64::from(BODY_RADIUS_CM)
                                };
                                in_time
                                    .min(top_s)
                                    .min((2.0 * decel_s * (dist - arrive).max(0.0)).sqrt())
                            }
                        },
                        |pace| pace.min(top_s),
                    )
                    .min(dist);
                let (ux, uy) = if leg > 0.0 {
                    (dx / leg, dy / leg)
                } else {
                    (0.0, 0.0)
                };
                let (wx, wy) = (ux * wanted, uy * wanted);
                // Towards the velocity he wants: speeding up by his
                // acceleration, slowing down or turning by his braking.
                let (cx, cy) = (wx - v_prev.0, wy - v_prev.1);
                let change = dmath::hypot(cx, cy);
                let now = dmath::hypot(v_prev.0, v_prev.1);
                // Turning by more than 60 degrees (plan L0-CLOSE 10e): he
                // brakes and turns with the share of his braking his joint
                // mobility gives him.
                let turning = now > 0.0
                    && wanted > 0.0
                    && v_prev.0.mul_add(wx, v_prev.1 * wy) < 0.5 * now * wanted;
                // Speeding up, his acceleration falls as his speed nears his
                // sprint, to nothing at it (SIM 194, the acceleration-speed
                // profile, movement.json).
                let limit = if turning {
                    decel_s * outcome.turn_share
                } else if wanted > now {
                    accel_s * (1.0 - now / sprint_s.max(1e-9)).max(0.0)
                } else {
                    decel_s
                };
                let k = if change > limit { limit / change } else { 1.0 };
                let (mut vx, mut vy) = (cx.mul_add(k, v_prev.0), cy.mul_add(k, v_prev.1));
                let speed = dmath::hypot(vx, vy);
                if speed > top_s {
                    vx *= top_s / speed;
                    vy *= top_s / speed;
                }
                #[allow(clippy::cast_possible_truncation)]
                let step = (
                    (vx * FINE as f64).round() as i64,
                    (vy * FINE as f64).round() as i64,
                );
                let nx = (fine.0.0 + step.0).clamp(0, i64::from(pitch_length_cm) * FINE);
                let ny = (fine.0.1 + step.1).clamp(0, i64::from(pitch_width_cm) * FINE);
                fine = ((nx, ny), (nx - fine.0.0, ny - fine.0.1));
                let cm = |f: i64| i32::try_from((f + FINE / 2).div_euclid(FINE)).unwrap_or(0);
                *at_step = (cm(fine.0.0), cm(fine.0.1));
            }
            entry.1 = path[STEPS - 1];
            fines.push((entry.0, fine));
            steps_of.push((entry.0, path));
        }

        // Minimal separation (MD-12): every push is computed from where the
        // players' own steps took them, then all are applied together, so
        // that no pair sees the result of another pair in the same phase
        // (ADR-063: within a phase nobody reads what another writes; before
        // `SIM_VERSION` 120 the pairs were pushed one after the other, in
        // the order of the ids).
        let stepped: Vec<(i32, i32)> = work.iter().map(|w| w.1).collect();
        for (entry, at) in work
            .iter_mut()
            .zip(separate(&stepped, (pitch_length_cm, pitch_width_cm)))
        {
            // No faster than his legs (SIM 150): pushed beyond his top
            // speed, he is moved only as far as it lets him.
            let from = self.agents.get(&entry.0).map_or(at, |a| a.position);
            let top = legs.get(&entry.0).copied().unwrap_or(f64::INFINITY);
            let (dx, dy) = (f64::from(at.0 - from.0), f64::from(at.1 - from.1));
            let d = dmath::hypot(dx, dy);
            entry.1 = if d > top && d > 0.0 {
                #[allow(clippy::cast_possible_truncation)]
                let p = (
                    from.0 + (dx * top / d).trunc() as i32,
                    from.1 + (dy * top / d).trunc() as i32,
                );
                p
            } else {
                at
            };
        }

        self.write_back(&work, ball, (tick, hidden), scan_balance);
        // Where each was at each small step; the separation's push is put
        // on his last.
        self.body_steps.clear();
        for (id, mut path) in steps_of {
            if let Some(a) = self.agents.get(&id) {
                path[STEPS - 1] = a.position;
                self.body_steps.insert(id, path);
            }
        }
        // Their fine state for the next tick (ADR-069); a man the
        // separation pushed starts from where it put him.
        for (id, f) in fines {
            if let Some(a) = self.agents.get_mut(&id) {
                let here = (
                    i64::from(a.position.0) * FINE,
                    i64::from(a.position.1) * FINE,
                );
                let cm = |v: i64| (v + FINE / 2).div_euclid(FINE);
                a.fine = if (cm(f.0.0), cm(f.0.1)) == (here.0 / FINE, here.1 / FINE) {
                    f
                } else {
                    (here, f.1)
                };
            }
        }
    }
    /// Where `id`, at `from` and going to `to`, passes a body standing on
    /// his way (SIM 150, OI-122; the maintainer, 17 November: how a man
    /// goes, as walking is): the nearest other man whose body the straight
    /// line crosses before `to`, unless he is at `to` (the man he goes to).
    /// He passes him two bodies' width to the side, by the side that makes
    /// his way shorter; on equal ways, by his right (a fixed rule,
    /// declared). How well he does it is his acceleration and his turn
    /// (`movement.json`: braking, joint mobility), not a number of its own.
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn way_round(&self, id: PlayerId, from: (i32, i32), to: (i32, i32)) -> Option<(i32, i32)> {
        let wide = f64::from(2 * BODY_RADIUS_CM);
        let (dx, dy) = (f64::from(to.0 - from.0), f64::from(to.1 - from.1));
        let len = dmath::hypot(dx, dy);
        if len <= wide {
            return None;
        }
        let (ux, uy) = (dx / len, dy / len);
        let (body, along) = self
            .agents
            .iter()
            .filter(|(o, _)| **o != id)
            .filter_map(|(_, a)| {
                let (ox, oy) = (
                    f64::from(a.position.0 - from.0),
                    f64::from(a.position.1 - from.1),
                );
                let along = ox.mul_add(ux, oy * uy);
                let across = ox.mul_add(-uy, oy * ux);
                let at_target = dmath::hypot(
                    f64::from(a.position.0 - to.0),
                    f64::from(a.position.1 - to.1),
                ) <= wide;
                (along > 0.0 && along < len && across.abs() < wide && !at_target)
                    .then_some((a.position, along))
            })
            .min_by(|a, b| a.1.total_cmp(&b.1))?;
        let _ = along;
        let side = |s: f64| {
            (
                (s * uy).mul_add(-wide, f64::from(body.0)).round() as i32,
                (s * ux).mul_add(wide, f64::from(body.1)).round() as i32,
            )
        };
        let way = |w: (i32, i32)| {
            dmath::hypot(f64::from(w.0 - from.0), f64::from(w.1 - from.1))
                + dmath::hypot(f64::from(to.0 - w.0), f64::from(to.1 - w.1))
        };
        // Right of his direction is -1 (the left normal is (-uy, ux)).
        let (right, left) = (side(-1.0), side(1.0));
        let (wr, wl) = (way(right), way(left));
        let pick = if wl < wr { left } else { right };
        let (length, width) = self.pitch_cm;
        Some((pick.0.clamp(0, length), pick.1.clamp(0, width)))
    }

    /// Writes back the moved positions, with velocity derived from the
    /// displacement, and updates heading and gaze.
    fn write_back(
        &mut self,
        work: &[Work],
        ball: (i32, i32),
        (tick, hidden): (u64, &mut ds_orchestrator::GroundTruth),
        scan_balance: &crate::balance::ScanningBalance,
    ) {
        let step_cm = hidden.step_length_cm().max(1.0);
        for (id, new_pos, _) in work {
            let Some(agent) = self.agents.get_mut(id) else {
                continue;
            };
            // His stride moves on with the ground he covers.
            let covered = dmath::hypot(
                f64::from(new_pos.0 - agent.position.0),
                f64::from(new_pos.1 - agent.position.1),
            );
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let advance = ((covered / step_cm) * 1000.0).round() as u32 % 1000;
            #[allow(clippy::cast_possible_truncation)]
            let gait = ((u32::from(agent.gait_milli) + advance) % 1000) as u16;
            agent.gait_milli = gait;
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
        // ADR-034: he scans now and then, not every tick. The
        // scan needs `&self` (read-only) to see the other agents; the
        // movement loop above held `&mut`, so the gaze pass is separate.
        //
        // The ball is the reference (OI-97, 2026-11-12): between two scans
        // the gaze goes back to the ball, as a footballer's does; the scan
        // is a glance (at the mark, the opponent on the ball), not a place
        // where the head stays. Before, a marked player looked at his mark
        // for good and never saw the ball again, and nobody went for it.
        for (id, _, _) in work {
            // His vision sets how often he glances (plan L0-CLOSE 10b);
            // the glances are staggered by player.
            let every = hidden.scan_interval_ticks(
                *id,
                (
                    scan_balance.scan_rate_min_per_s,
                    scan_balance.scan_rate_max_per_s,
                ),
                10.0,
            );
            let scanning = (tick + u64::from(id.slot())) % every == 0;
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

/// The minimal separation of the bodies (MD-12, ADR-063): every push is
/// computed from `at`, then all are applied together, so the result does
/// not depend on the order of the players; nobody is pushed off the pitch.
fn separate(at: &[(i32, i32)], (length, width): (i32, i32)) -> Vec<(i32, i32)> {
    let min_d = i64::from(2 * BODY_RADIUS_CM);
    let n = at.len();
    let mut shift = vec![(0i32, 0i32); n];
    for i in 0..n {
        for j in (i + 1)..n {
            let (ax, ay) = at[i];
            let (bx, by) = at[j];
            let ddx = i64::from(bx - ax);
            let ddy = i64::from(by - ay);
            let d2 = ddx * ddx + ddy * ddy;
            if d2 > 0 && d2 < min_d * min_d {
                let push = i32::try_from(((min_d - d2.isqrt()) / 2).max(1)).unwrap_or(1);
                let nx = i32::try_from(ddx.signum()).unwrap_or(0);
                let ny = i32::try_from(ddy.signum()).unwrap_or(0);
                shift[i] = (shift[i].0 - nx * push, shift[i].1 - ny * push);
                shift[j] = (shift[j].0 + nx * push, shift[j].1 + ny * push);
            }
        }
    }
    at.iter()
        .zip(shift)
        .map(|(p, (sx, sy))| ((p.0 + sx).clamp(0, length), (p.1 + sy).clamp(0, width)))
        .collect()
}

/// Where across the keeper stands at depth `x` (`SIM_VERSION` 124).
///
/// On the bisector of the angle the ball makes with the two posts of the
/// goal at `goal` (`half` the distance between them). Geometry, no number
/// of its own. A ball on the goal line itself leaves him at the middle.
#[must_use]
pub fn keeper_across(ball: (f64, f64), goal: (f64, f64), half: f64, x: f64) -> f64 {
    let posts = [(goal.0, goal.1 - half), (goal.0, goal.1 + half)];
    let mut dir = (0.0, 0.0);
    for p in posts {
        let (dx, dy) = (p.0 - ball.0, p.1 - ball.1);
        let len = dmath::hypot(dx, dy);
        if len > 0.0 {
            dir = (dir.0 + dx / len, dir.1 + dy / len);
        }
    }
    if dir.0.abs() < 1e-9 {
        return goal.1;
    }
    let s = (x - ball.0) / dir.0;
    s.mul_add(dir.1, ball.1)
}

#[cfg(test)]
mod tests {

    /// Twin test (decision AB): the same physical fatigue, and the mental
    /// one he brought: his mind is the more tired; never above 1.
    #[test]
    fn the_mental_fatigue_he_brings_tires_his_mind() {
        let body = |mental| PlayerBody {
            height_cm: 180.0,
            weight_kg: 78.0,
            fatigue: 0.3,
            mental,
            condition: 1.0,
            damage: 0.0,
        };
        assert!(body(0.2).mind_fatigue() > body(0.0).mind_fatigue());
        assert!((body(0.9).mind_fatigue() - 1.0).abs() < 1e-12);
    }

    /// P1 (`SIM_VERSION` 124): the keeper stands on the bisector of the
    /// angle the ball makes with his posts: in the middle for a central
    /// ball, towards the near post for a wide one, the two posts at equal
    /// angles from the ball's line through him.
    #[test]
    fn the_keeper_splits_the_angle_of_his_posts() {
        let goal = (0.0, 3400.0);
        let centre = super::keeper_across((1800.0, 3400.0), goal, 366.0, 200.0);
        assert!((centre - 3400.0).abs() < 1e-6);
        let ball = (1100.0, 5400.0);
        let y = super::keeper_across(ball, goal, 366.0, 200.0);
        assert!(y > 3400.0, "towards the near post: {y}");
        let angle = |p: (f64, f64)| {
            let (ax, ay) = (200.0 - ball.0, y - ball.1);
            let (bx, by) = (p.0 - ball.0, p.1 - ball.1);
            ax.mul_add(bx, ay * by)
                / (ds_core::dmath::hypot(ax, ay) * ds_core::dmath::hypot(bx, by))
        };
        assert!((angle((0.0, 3034.0)) - angle((0.0, 3766.0))).abs() < 1e-9);
    }

    /// P1 (ADR-063, `SIM_VERSION` 120): the separation of three bodies
    /// pressed together does not depend on the order they are visited in.
    #[test]
    fn the_separation_does_not_depend_on_the_order() {
        let pitch = (10_500, 6_800);
        let a = [(5000, 3000), (5020, 3010), (5010, 3030)];
        let ab = super::separate(&a, pitch);
        let rev: Vec<(i32, i32)> = a.iter().rev().copied().collect();
        let mut ba = super::separate(&rev, pitch);
        ba.reverse();
        assert_eq!(ab, ba);
        assert_ne!(ab, a.to_vec(), "they are pushed apart");
    }

    /// The touch (P1, plan L0-CLOSE 11): weighed to come back to his foot
    /// after the period at his pace, ahead of him in between.
    #[test]
    fn a_touch_comes_back_to_his_foot() {
        let keep = 0.957;
        let (pace, period) = (80.0, 15.0);
        let u = super::touch_speed(pace, period, keep);
        assert!(pace.mul_add(-period, super::rolled(u, period, keep)).abs() < 1e-6);
        let half = period / 2.0;
        assert!(
            super::rolled(u, half, keep) > pace * half,
            "ahead of him in between"
        );
    }

    use super::*;
    use crate::test_support::make_squad;

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
                        mental: 0.0,
                        condition: 1.0,
                        damage: 0.0,
                    },
                );
            }
        }
        m
    }

    fn make_state() -> SpatialState {
        SpatialState::from_squads(&Pitch::default(), &make_squad(0), &make_squad(100))
    }

    #[test]
    fn twenty_two_agents_on_the_pitch() {
        assert_eq!(make_state().len(), 22);
    }

    /// ADR-091, SIM 176: a re-read within a body of the place he holds is the
    /// same space, so his target does not flip and he does not brake and set
    /// off again; a real change of space still moves him.
    #[test]
    fn a_near_reread_keeps_the_place_he_holds() {
        let mut s = make_state();
        let id = PlayerId::new(0, 0);
        s.set_hold_at(id, Some((4000, 3400)));
        s.set_hold_at(id, Some((4020, 3410)));
        assert_eq!(s.hold_at.get(&id).copied(), Some((4000, 3400)));
        s.set_hold_at(id, Some((5000, 3400)));
        assert_eq!(s.hold_at.get(&id).copied(), Some((5000, 3400)));
    }

    /// SIM 181 (LUNA): the press keeps the opponent as its target; the body
    /// takes the lane between him and the goal we defend. Before, the man
    /// ran at the ball and arrived behind the carrier.
    #[test]
    fn the_presser_takes_the_goal_side_of_the_carrier() {
        let mut s = make_state();
        let carrier = PlayerId::new(10, 0);
        let presser = PlayerId::new(105, 0);
        let ids: Vec<PlayerId> = s.iter().map(|(id, _)| *id).collect();
        for other in ids {
            if other != carrier && other != presser {
                let at = s.agent(other).unwrap().position;
                s.send(other, at, false);
            }
        }
        let spot = (7_800, 3_700);
        s.set_position(carrier, spot);
        s.set_position(presser, (7_500, 3_400));
        s.ball_position = (spot.0, spot.1, 0);
        s.set_carrier(Some(carrier));
        s.set_act(presser, ds_orchestrator::OffBallAct::Press);
        let bodies = make_bodies();
        let mut hidden = ds_orchestrator::GroundTruth::new(42, ds_core::ids::MatchId::new(0, 0));
        for tick in 1..120 {
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
        let p = s.agent(presser).unwrap().position;
        let c = s.agent(carrier).unwrap().position;
        // Goal-side means the body is on the own-goal side of him, not just
        // nearer the ball: the press keeps the man as its target (SIM 181).
        let own = (10_500, 3_400); // the away side defends x=10500
        let to_goal = (f64::from(own.0 - c.0), f64::from(own.1 - c.1));
        let step = (f64::from(p.0 - c.0), f64::from(p.1 - c.1));
        assert!(
            step.0.mul_add(to_goal.0, step.1 * to_goal.1) > 0.0,
            "the presser stayed behind: {p:?} vs {c:?}"
        );
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

    /// Scene test (SIM 150, OI-122; seed 47 at a kick-off): a man walking
    /// to his place finds a man standing on his way, and goes round him;
    /// the standing man does not move for him. Before, the bodies pushed
    /// each other back on the line between them, and he stood there.
    #[test]
    fn a_man_goes_round_a_man_standing_on_his_way() {
        let mut s = make_state();
        let walker = PlayerId::new(10, 0);
        let standing = PlayerId::new(110, 0);
        let ids: Vec<PlayerId> = s.iter().map(|(id, _)| *id).collect();
        for id in ids {
            let at = s.agent(id).unwrap().position;
            s.send(id, at, false);
        }
        s.set_position(walker, (5347, 4011));
        s.set_position(standing, (5293, 4089));
        s.send(walker, (5150, 4208), false);
        s.send(standing, (5293, 4089), false);
        s.set_ball_in_flight((5250, 3400, 0), (0, 0, 0));
        let bodies = make_bodies();
        let mut hidden = ds_orchestrator::GroundTruth::new(42, ds_core::ids::MatchId::new(0, 0));
        for tick in 0..150 {
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
        let at = s.agent(walker).unwrap().position;
        let d = distance_squared(at, (5150, 4208)).isqrt();
        assert!(
            d <= i64::from(2 * BODY_RADIUS_CM),
            "he stands {d} cm from his place, at {at:?}"
        );
        let still = s.agent(standing).unwrap().position;
        assert!(
            distance_squared(still, (5293, 4089)) <= i64::from(2 * BODY_RADIUS_CM).pow(2),
            "the standing man stays: {still:?}"
        );
    }

    /// A man sent running to a restart spot brakes to a stop where the ball
    /// is given to him (the referee's arrival radius), not onto or past it:
    /// "run" is how he goes, not a licence to carry momentum into the kick.
    #[test]
    fn a_man_sent_running_stops_at_his_place() {
        let mut s = make_state();
        let walker = PlayerId::new(10, 0);
        let spot = (5150, 4208);
        let ids: Vec<PlayerId> = s.iter().map(|(id, _)| *id).collect();
        for id in ids {
            let at = s.agent(id).unwrap().position;
            s.send(id, at, false);
        }
        s.set_position(walker, (5347, 4011));
        s.send(walker, spot, true);
        s.set_ball_in_flight((5250, 3400, 0), (0, 0, 0));
        let bodies = make_bodies();
        let mut hidden = ds_orchestrator::GroundTruth::new(42, ds_core::ids::MatchId::new(0, 0));
        let mut arrived = None;
        for tick in 0..200 {
            s.move_agents(
                10500,
                6800,
                &bodies,
                &mut hidden,
                tick,
                &crate::balance::ScanningBalance::load(),
                &crate::balance::KeeperBalance::load(),
            );
            let d = distance_squared(s.agent(walker).unwrap().position, spot).isqrt();
            if d <= i64::from(2 * BODY_RADIUS_CM) && arrived.is_none() {
                arrived = Some(tick);
            }
            if let Some(at_tick) = arrived {
                assert!(
                    d <= i64::from(2 * BODY_RADIUS_CM),
                    "he ran past his place at tick {tick} ({d} cm), first there at {at_tick}"
                );
            }
        }
        assert!(arrived.is_some(), "he never reached his place");
        let final_d = distance_squared(s.agent(walker).unwrap().position, spot).isqrt();
        assert!(
            final_d <= i64::from(BODY_RADIUS_CM),
            "he stopped {final_d} cm short of the ball instead of on it"
        );
    }

    #[test]
    fn the_carrier_brakes_when_his_touch_runs_out() {
        let mut s = make_state();
        let carrier = PlayerId::new(10, 0);
        let ball = (2000, 1_000);
        let ids: Vec<PlayerId> = s.iter().map(|(id, _)| *id).collect();
        for id in ids {
            if id == carrier {
                continue;
            }
            let at = s.agent(id).unwrap().position;
            s.send(id, at, false);
        }
        s.set_position(carrier, (2600, 1_000));
        s.carrier = Some(carrier);
        s.ball_position = (ball.0, ball.1, 0);
        // A touch whose period ran out: he goes back to the ball he played.
        s.touch = Some(Touch {
            from: (f64::from(ball.0), f64::from(ball.1)),
            dir: (1.0, 0.0),
            speed: 0.0,
            at: 0,
            period: 0.0,
            held_cm: 0.0,
        });
        let bodies = make_bodies();
        let mut hidden = ds_orchestrator::GroundTruth::new(42, ds_core::ids::MatchId::new(0, 0));
        let mut arrived = None;
        for tick in 1..200 {
            s.move_agents(
                10500,
                6800,
                &bodies,
                &mut hidden,
                tick,
                &crate::balance::ScanningBalance::load(),
                &crate::balance::KeeperBalance::load(),
            );
            let d = distance_squared(s.agent(carrier).unwrap().position, ball).isqrt();
            if d <= i64::from(2 * BODY_RADIUS_CM) && arrived.is_none() {
                arrived = Some(tick);
            }
            if arrived.is_some() {
                assert!(
                    d <= i64::from(2 * BODY_RADIUS_CM),
                    "he ran past his own ball at tick {tick} ({d} cm)"
                );
            }
        }
        assert!(arrived.is_some(), "he never reached his own ball");
        let final_d = distance_squared(s.agent(carrier).unwrap().position, ball).isqrt();
        assert!(
            final_d <= i64::from(BODY_RADIUS_CM),
            "he stopped {final_d} cm short of his own ball"
        );
    }

    /// SIM 175: a man going for a loose ball runs no faster than the
    /// fastest running in the declared real-match profile, as a share of
    /// his own top speed. Before, a contest ran flat out at `v_max`.
    #[test]
    fn a_contest_runs_at_the_real_match_sprint_share() {
        let mut s = make_state();
        let id = PlayerId::new(10, 0);
        let ball = (6_000, 1_000);
        let ids: Vec<PlayerId> = s.iter().map(|(id, _)| *id).collect();
        for other in ids {
            if other == id {
                continue;
            }
            let at = s.agent(other).unwrap().position;
            s.send(other, at, false);
        }
        s.set_position(id, (2_000, 1_000));
        s.carrier = None;
        s.ball = BallState::Loose;
        s.ball_position = (ball.0, ball.1, 0);
        s.touch = None;
        s.set_act(id, ds_orchestrator::OffBallAct::GoToBall);
        s.set_act_target(id, ball);

        let bodies = make_bodies();
        let body = &bodies[&id];
        let mctx = ds_orchestrator::MovementContext {
            player: id,
            height_cm: body.height_cm,
            weight_kg: body.weight_kg,
            fatigue: body.fatigue,
        };
        let mut top_hidden =
            ds_orchestrator::GroundTruth::new(42, ds_core::ids::MatchId::new(0, 0));
        let outcome = top_hidden.resolve_movement(&mctx);
        let limit = outcome.v_max * outcome.stride * outcome.contest_pace_share;

        let mut hidden = ds_orchestrator::GroundTruth::new(42, ds_core::ids::MatchId::new(0, 0));
        let mut max_speed = 0.0f64;
        for tick in 1..200 {
            let before = s.agent(id).unwrap().position;
            s.move_agents(
                10500,
                6800,
                &bodies,
                &mut hidden,
                tick,
                &crate::balance::ScanningBalance::load(),
                &crate::balance::KeeperBalance::load(),
            );
            let after = s.agent(id).unwrap().position;
            let step =
                ds_core::dmath::hypot(f64::from(after.0 - before.0), f64::from(after.1 - before.1));
            max_speed = max_speed.max(step);
        }
        assert!(
            max_speed > 10.0,
            "he never went for the ball ({max_speed:.0} cm/tick)"
        );
        assert!(
            max_speed <= limit + 1.0,
            "contest at {max_speed:.0} cm/tick beats the declared share {limit:.0}"
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

    /// The first touch (P1, plan L0-CLOSE 11): the ball he wins stays where
    /// he took it, it does not jump to the middle of his body; a perfect
    /// touch kills it there, a poor one lets it run on the way it came.
    #[test]
    fn a_ball_won_is_taken_where_it_is() {
        let id = PlayerId::new(1, 0);
        let won = |spill: f64| {
            let mut s = make_state();
            let at = s.agent(id).map(|a| a.position).unwrap_or_default();
            let from = (at.0 + 80, at.1, 0);
            s.set_ball_in_flight(from, (60, 0, 0));
            s.set_carrier(Some(id));
            s.follow_carrier(1, false, |_, _| 2.5, |_, _| spill, 45);
            let first = s.ball();
            s.follow_carrier(4, false, |_, _| 2.5, |_, _| spill, 45);
            (at, first, s.ball())
        };
        let (at, first, later) = won(0.0);
        assert_ne!(first, at, "not drawn to him");
        assert_eq!(later, at, "a dead ball under his foot in a step");
        let (_, first, later) = won(10.0);
        assert!(first.0 > at.0 && later.0 > first.0, "a poor touch runs on");
    }

    /// OI-123 (ADR-058 §5), SIM 177: the first touch is the receiver's. Running
    /// into the pass he takes it with him, instead of leaving it behind and
    /// going back for it. One running the ball's way still plays it ahead.
    #[test]
    fn a_man_running_into_the_pass_takes_it_with_him() {
        let id = PlayerId::new(1, 0);
        let receive = |run: i16| {
            let mut s = make_state();
            let at = s.agent(id).map_or((0, 0), |a| a.position);
            s.agent_mut(id).unwrap().velocity = (run, 0);
            s.set_ball_in_flight((at.0 + 80, at.1, 0), (60, 0, 0));
            s.set_carrier(Some(id));
            s.follow_carrier(1, false, |_, _| 2.5, |_, _| 10.0, 45);
            s.follow_carrier(4, false, |_, _| 2.5, |_, _| 10.0, 45);
            (at, s.ball())
        };
        let (at, against) = receive(-60);
        assert!(
            against.0 < at.0 - BODY_RADIUS_CM,
            "taken into his own run, not back where it came: {at:?} {against:?}"
        );
        let (at, along) = receive(60);
        assert!(
            along.0 > at.0 + BODY_RADIUS_CM,
            "still played ahead when running with it: {at:?} {along:?}"
        );
    }

    /// OI-123 (SIM 178): a man faster than the ball he has just won takes
    /// it into his own stride, instead of running past a ball that only
    /// rolls at a fraction of his pace. Same ball, same touch, same
    /// direction: only his pace changes.
    #[test]
    fn a_man_faster_than_the_spilled_ball_takes_it_with_him() {
        let id = PlayerId::new(1, 0);
        let first = |run: i16| {
            let mut s = make_state();
            let at = s.agent(id).map_or((0, 0), |a| a.position);
            s.agent_mut(id).unwrap().velocity = (run, 0);
            s.set_ball_in_flight((at.0 + 80, at.1, 0), (4, 0, 0));
            s.set_carrier(Some(id));
            s.follow_carrier(1, false, |_, _| 10.0, |_, _| 1.0, 45);
            s.follow_carrier(4, false, |_, _| 10.0, |_, _| 1.0, 45);
            s.ball().0 - at.0
        };
        let fast = first(60);
        let slow = first(3);
        assert!(
            fast > slow + 100,
            "the faster man plays it at his own pace: {fast} vs {slow}"
        );
        assert!(
            slow < 100,
            "a slower man still lets the spill roll on: {slow}"
        );
    }

    /// His own ball within his reach (P1, plan L0-CLOSE 11): he plays it
    /// where it is once his touch meets it; with a shorter reach it is not
    /// his to play yet. Bodies around it no longer keep him from it.
    #[test]
    fn he_plays_his_ball_within_his_reach() {
        let id = PlayerId::new(1, 0);
        let at_feet = |reach: i64| {
            let mut s = make_state();
            let at = s.agent(id).map(|a| a.position).unwrap_or_default();
            s.set_ball_in_flight((at.0 + 80, at.1, 0), (0, 0, 0));
            s.set_carrier(Some(id));
            s.follow_carrier(1, false, |_, _| 2.5, |_, _| 0.0, reach);
            s.follow_carrier(5, false, |_, _| 2.5, |_, _| 0.0, reach);
            (s.ball(), at, s.ball_at_feet())
        };
        let (ball, at, mine) = at_feet(120);
        assert!(mine, "within his reach");
        assert_eq!(ball, at, "brought under his foot, not left behind");
    }

    /// SIM 182 (DD-39): the same ball, a reach too short to meet the touch.
    /// Before, the touch stayed due but unreached forever: the ball rested
    /// out of range, the carrier stood, and the match lost a minute and a
    /// half on one man. The same `under_foot` as any ball he did not reach.
    #[test]
    fn an_unreached_touch_comes_back_under_his_foot() {
        let id = PlayerId::new(1, 0);
        let mut s = make_state();
        let at = s.agent(id).map_or((0, 0), |a| a.position);
        s.agent_mut(id).unwrap().velocity = (60, 0);
        s.set_ball_in_flight((at.0 + 80, at.1, 0), (60, 0, 0));
        s.set_carrier(Some(id));
        // A touch played ahead at his running pace, not a ball brought under
        // him: `held_cm` is 0 and the ball is 80 cm beyond his reach.
        s.follow_carrier(1, false, |_, _| 2.5, |_, _| 10.0, 45);
        // Bodies hold him where he is: he cannot run to the touch.
        s.agent_mut(id).unwrap().velocity = (0, 0);
        for tick in 2..=4 {
            s.follow_carrier(tick, false, |_, _| 2.5, |_, _| 10.0, 45);
        }
        assert!(
            !s.ball_at_feet(),
            "the touch has not had time to fail yet: {:?}",
            s.ball()
        );
        for tick in 5..=10 {
            s.follow_carrier(tick, false, |_, _| 2.5, |_, _| 10.0, 45);
        }
        assert!(
            s.ball_at_feet(),
            "the ball is still out of reach: {:?}",
            s.ball()
        );
    }

    /// A ball in the air is read from the kick (P1, plan L0-CLOSE 11): just
    /// struck and still below head height, it already has a place where it
    /// comes down, up the pitch; one that never rises above his head has
    /// none.
    #[test]
    fn a_ball_in_the_air_is_read_from_the_kick() {
        let mut s = make_state();
        s.set_ball_in_flight((3000, 3400, 0), (150, 0, 80));
        let d = s.drop_point(180).expect("read from the kick");
        assert!(d.0 > 3000 + 1000, "{d:?}");
        s.set_ball_in_flight((3000, 3400, 0), (150, 0, 40));
        assert_eq!(s.drop_point(180), None, "never above his head");
    }

    /// Meeting a rolling ball (P1, plan L0-CLOSE 11): he runs to where its
    /// path and his run cross, ahead of the ball, not to where it is now;
    /// a slower man meets it farther along; a ball at rest, or in the air,
    /// he goes to as before.
    #[test]
    fn a_rolling_ball_is_met_on_its_path() {
        let mut s = make_state();
        s.set_ball_in_flight((3000, 3400, 0), (60, 0, 0));
        let fast = s
            .intercept((4000, 2400), ((0.0, 0.0), 5.0, 90.0), 0.0)
            .expect("rolling");
        let slow = s
            .intercept((4000, 2400), ((0.0, 0.0), 2.0, 40.0), 0.0)
            .expect("rolling");
        assert!(fast.0 > 3000 && slow.0 > fast.0, "{fast:?} {slow:?}");
        // In the air (SIM_VERSION 111): met where it comes down within his
        // height, ahead of where it is now, not chased.
        s.set_ball_in_flight((3000, 3400, 0), (60, 0, 50));
        let lofted = s
            .intercept((4000, 2400), ((0.0, 0.0), 5.0, 90.0), 0.0)
            .expect("met on its path");
        assert!(lofted.0 > 3000, "{lofted:?}");
    }

    /// Twin test (SIM 194): his speed helps him only the way he is running.
    /// Running away from the path he meets the ball where a man standing
    /// still would; running towards it, sooner.
    #[test]
    fn a_man_running_away_meets_the_ball_no_sooner_than_one_standing() {
        let mut s = make_state();
        s.set_ball_in_flight((3000, 3400, 0), (60, 0, 0));
        let still = s.intercept((4000, 2400), ((0.0, 0.0), 3.0, 80.0), 0.0);
        let away = s.intercept((4000, 2400), ((0.0, -60.0), 3.0, 80.0), 0.0);
        let toward = s.intercept((4000, 2400), ((0.0, 60.0), 3.0, 80.0), 0.0);
        let (still, away, toward) = (
            still.expect("met"),
            away.expect("met"),
            toward.expect("met"),
        );
        assert_eq!(away, still);
        assert!(toward.0 < still.0, "{toward:?} {still:?}");
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
    fn what_is_foreseen_of_the_ball_is_what_happens() {
        // OI-98 (P1): `ball_at_line` imagines the path with the same step
        // `advance_ball` applies, so the keeper turning a ball round the post
        // turns it as much as the slowed ball really needs. Here a slow ball
        // with a small sideways component: the straight line would put it
        // 12 cm further out than the slowed ball really goes.
        let mut s = make_state();
        let (from, v, line) = ((10_090, 3660, 170), (46_i16, 12_i16, 5_i16), 10_500);
        let foreseen = s.ball_at_line(from, v, line).expect("it reaches the line");
        s.set_ball_in_flight(from, v);
        let mut prev = s.ball_3d();
        while s.ball_3d().0 < line {
            prev = s.ball_3d();
            s.advance_ball();
        }
        let now = s.ball_3d();
        let y = prev.1 + (now.1 - prev.1) * (line - prev.0) / (now.0 - prev.0).max(1);
        assert_eq!(foreseen.0, y, "foreseen {foreseen:?}, happened y = {y}");
        let straight = from.1 + i32::from(v.1) * (line - from.0) / i32::from(v.0);
        assert!(
            straight > y,
            "the straight line overstates the sideways run"
        );
    }

    #[test]
    fn the_last_touches_name_who_played_it_and_who_only_touched_it() {
        // Law 10 (P1): the carrier is the man who played it; a touch on its
        // way does not make it his play; a new carrier starts again.
        let mut s = make_state();
        let (a, b) = (PlayerId::new(1, 0), PlayerId::new(101, 0));
        s.set_carrier(Some(a));
        s.set_carrier(None);
        s.struck_shot();
        s.deflected_by(b);
        let t = s.touches();
        assert_eq!((t.played, t.shot, t.deflected), (Some(a), true, Some(b)));
        s.set_carrier(Some(b));
        let t = s.touches();
        assert_eq!((t.played, t.shot, t.deflected), (Some(b), false, None));
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

    /// Twin (ADR-083, OI-127): the roll is the ball's, not the act of whoever
    /// is looking at it. Changing only a player's act, with the same ball and
    /// the same feet, leaves its path untouched.
    #[test]
    fn the_roll_is_the_balls_whatever_the_players_act() {
        let trace = |act: ds_orchestrator::OffBallAct| {
            let mut s = make_state();
            s.set_act(PlayerId::new(1, 0), act);
            s.set_ball_in_flight((0, 3400, 0), (150, 0, 0));
            while s.ball_state() == BallState::InFlight {
                s.advance_ball();
            }
            let mut xs = vec![s.ball_3d().0];
            for _ in 0..50 {
                s.advance_ball();
                xs.push(s.ball_3d().0);
            }
            xs
        };
        assert_eq!(
            trace(ds_orchestrator::OffBallAct::GoToBall),
            trace(ds_orchestrator::OffBallAct::Hold)
        );
    }

    /// P1 (ADR-083, OI-127): crossing the stop threshold changes the name of
    /// the state, not the physics. A free ball keeps rolling until friction
    /// spends what is left of its speed.
    #[test]
    fn a_free_ball_keeps_rolling_after_it_slows_below_the_stop_threshold() {
        let mut s = make_state();
        s.set_ball_in_flight((0, 3400, 0), (150, 0, 0));
        while s.ball_state() == BallState::InFlight {
            s.advance_ball();
        }
        assert_eq!(s.ball_state(), BallState::Loose);
        let at = s.ball_3d().0;
        for _ in 0..50 {
            s.advance_ball();
        }
        assert!(s.ball_3d().0 > at, "a loose ball keeps rolling, not frozen");
    }

    /// Twin test (SIM 128, OI-100; ADR-069): the same ball dropped from
    /// 2 m on the same grass, with and without the bounce. With it, it comes
    /// back up into FIFA's band for a good natural pitch, 0.60 to 1.00 m
    /// (the ratio 0.632 is the middle's; in small steps the fall is no longer
    /// cut at the landing as with the tick of 0.1 s); with no restitution it
    /// stays on the grass.
    #[test]
    fn a_ball_dropped_from_two_metres_bounces_as_on_a_good_pitch() {
        let top_after = |p: &crate::balance::BallBalance| {
            let k = FineKick::of(p);
            let mut b = FineBall::from_cm((0, 0, 200), (0, 0, 0));
            let (mut landed, mut top) = (false, 0);
            for _ in 0..30 {
                b.tick(&k);
                let (at, _) = b.to_cm();
                landed |= at.2 == 0;
                if landed {
                    top = top.max(at.2);
                }
            }
            top
        };
        let p = crate::balance::BallBalance::load();
        let top = top_after(&p);
        assert!((60..=100).contains(&top), "rebound {top} cm");
        let dead = crate::balance::BallBalance {
            restitution_milli: 0,
            ..p
        };
        assert_eq!(top_after(&dead), 0);
    }

    #[test]
    fn advance_ball_moves_z_down_and_x_forward_under_gravity() {
        let mut s = make_state();
        s.set_ball_in_flight((5000, 3400, 200), (100, 0, 0));
        s.advance_ball();
        // Gravity pulls z down, friction slows x a little: one tick of
        // small steps (ADR-069).
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
