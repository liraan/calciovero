//! The in-progress state of a match.
//!
//! `MatchState` is mutated by the tick loop. At the end of the match, it
//! produces a `MatchResult` via `finish`.

use ds_core::ids::{ActorId, EventId, PlayerId};
use ds_core::time::GameInstant;
use ds_model::Squad;
use ds_stats::{EventChain, EventType, Half, MatchEvent};
use std::collections::{BTreeMap, BTreeSet};

use crate::context::MatchContext;
use crate::perception::distance_squared;
use crate::result::{MatchResult, MatchStatistics, Score};
use crate::spatial::{PlayerBody, SpatialState};

/// Which team has the ball.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Possession {
    Home,
    Away,
}

/// How many ticks a ball stays in flight before the pending resolves even
/// if it has not landed (MD-37). Arbitrary, declared: 15 ticks = 1.5 s.
const FLIGHT_TICKS: u64 = 15;

/// Radius within which an opponent counts as crowding, in cm. Arbitrary,
/// declared (ADR-044 §6: the duration of the chain is in the relation).
const CROWD_RADIUS_CM: i32 = 1500;

/// The number of crowding opponents that saturates the term to 1.0.
const CROWD_MAX: u32 = 3;

/// Distance at which the nearest-threat term goes to 0.0, in cm.
const THREAT_RADIUS_CM: f64 = 2000.0;

/// Interception radius, in centimetres. A defender within this distance
/// of the pass line intercepts, if the passer did not see him. Arbitrary,
/// declared.
const INTERCEPT_RADIUS_CM: i64 = 200;

/// The squared distance from point `p` to the segment `a`-`b`. Integer
/// only, deterministic (D2).
fn point_segment_dist_sq(a: (i32, i32), b: (i32, i32), p: (i32, i32)) -> i64 {
    let abx = i64::from(b.0 - a.0);
    let aby = i64::from(b.1 - a.1);
    let apx = i64::from(p.0 - a.0);
    let apy = i64::from(p.1 - a.1);
    let ab_dot = abx * abx + aby * aby;
    if ab_dot == 0 {
        return apx * apx + apy * apy;
    }
    let a_to_p = apx * abx + apy * aby;
    if a_to_p <= 0 {
        return apx * apx + apy * apy;
    }
    if a_to_p >= ab_dot {
        let bpx = i64::from(b.0 - p.0);
        let bpy = i64::from(b.1 - p.1);
        return bpx * bpx + bpy * bpy;
    }
    let cross = abx * apy - aby * apx;
    (cross * cross) / ab_dot
}

/// The first unseen opponent within `INTERCEPT_RADIUS_CM` of the segment
/// `a`-`b`, in `unseen` order. Deterministic.
fn find_interceptor(
    spatial: &SpatialState,
    unseen: &[PlayerId],
    a: (i32, i32),
    b: (i32, i32),
) -> Option<PlayerId> {
    let r_sq = INTERCEPT_RADIUS_CM * INTERCEPT_RADIUS_CM;
    for &id in unseen {
        let Some(agent) = spatial.agent(id) else {
            continue;
        };
        if point_segment_dist_sq(a, b, agent.position) <= r_sq {
            return Some(id);
        }
    }
    None
}

/// Info about a pending shot (F.3-b3). The crossing detection reads
/// the attacker to attribute the Goal event.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ShotPending {
    /// The shooter.
    pub attacker: PlayerId,
    /// The tick of the strike: the keeper's time runs from here.
    pub struck: u64,
    /// Where the ball was struck from.
    pub from: (i32, i32, i32),
}

/// A pending fact the engine must resolve at a declared tick (MD-37).
/// The possession is applied only at resolution: the ball in flight
/// belongs to nobody (MD-25).
pub(crate) struct Pending {
    /// F.3-b3: if this pending is a shot, the info needed to detect
    /// a goal-line crossing during the flight.
    pub shot: Option<ShotPending>,
    /// The tick at which the pending resolves even if not landed.
    pub due_tick: u64,
    /// Which team has the ball once the flight resolves.
    pub outcome: Possession,
    /// The opponent who intercepts the pass, if any.
    pub interceptor: Option<PlayerId>,
    /// The passer, for the Interception event (None for a shot).
    pub passer: Option<PlayerId>,
    /// REC: the intended receiver, for `resolve_reception` at landing
    /// (None for a shot).
    pub receiver: Option<PlayerId>,
    /// REC: the receiver freedom at launch, carried to the reception
    /// (D7, one source of truth; None for a shot).
    pub freedom_cm: Option<i64>,
    /// REC: the pass distance at launch, for `resolve_reception`.
    pub distance_cm: Option<i64>,
}
/// The in-progress state of a match.
pub struct MatchState {
    score: Score,
    possession: Possession,
    home_possession_ticks: u64,
    away_possession_ticks: u64,
    shots: (u16, u16),
    next_event_slot: u32,
    /// Which half is currently being played (ADR-026).
    half: Half,
    /// The absolute tick at which the current half started.
    half_start_tick: u64,
    /// The spatial state: positions, ball, carrier (slice B2).
    spatial: SpatialState,
    /// The pending flight, if any (MD-37). At most one at a time today.
    /// F.5: the opponents currently covering the carrier. The set is
    /// persistent across ticks; `update_coverage` diffs it and emits
    /// `RelationEnterExit` for the new members (MD-04). Discarded on
    /// possession change: the previous coverage referred to the other
    /// team.
    coverage: BTreeSet<PlayerId>,
    /// The possession the current `coverage` set refers to.
    coverage_possession: Possession,
    /// The body of each player (height, weight, fatigue). Built once
    /// in `new` from the squads; read by `resolve_movement` each tick.
    bodies: BTreeMap<PlayerId, PlayerBody>,
    /// Scanning parameters (ADR-034), loaded once at construction.
    scan_balance: crate::balance::ScanningBalance,
    keeper_balance: crate::balance::KeeperBalance,
    /// The Laws' thresholds (Law 12, `balance/laws.json`).
    laws: crate::balance::LawsBalance,
    /// The players cautioned once (Law 12): a second yellow is a red.
    cautioned: BTreeSet<PlayerId>,
    obstacle_balance: crate::balance::ObstacleBalance,
    reception_balance: crate::balance::ReceptionBalance,
    carry_balance: crate::balance::CarryBalance,
    pressure_balance: crate::balance::PressureBalance,
    /// Where the carrier was at his last decision: if he has not got
    /// anywhere since, he is blocked (OI-97).
    last_decision: Option<(PlayerId, (i32, i32))>,
    /// When a player who lost the ball may challenge again (ADR-042).
    challenges: BTreeMap<PlayerId, u64>,
    /// When each player without the ball chooses again (ADR-049).
    offball_due: BTreeMap<PlayerId, u64>,
    /// Law 11: the passer's teammates in an offside position when the
    /// ball was played; one who then plays it commits the offence.
    offside_at_pass: BTreeSet<PlayerId>,
    /// The restart that put the ball back in play, until its first pass:
    /// there is no offside from a throw-in, a goal kick or a corner.
    restart_pass: Option<crate::rules::RestartKind>,
    pending: Option<Pending>,
    /// Per-actor latency (IDEA-DESIGN-011): the tick at which the actor
    /// can decide again. Absent = 0 = due. The map is cleared on possession
    /// change? No: the actor's pace is his own, persistent across possessions.
    /// A weak decider who wins the ball back carries his own delay with him.
    latencies: BTreeMap<PlayerId, u64>,
}

/// The action an agent chooses from its point of view (MD-01, MD-43).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ChosenAction {
    /// Pass to the teammate with the most space, and how much space he
    /// has (cm, distance to the nearest opponent). The freedom is computed
    /// once in `choose_action` and carried to the reception, not recomputed
    /// (D7, one source of truth).
    Pass {
        receiver: PlayerId,
        freedom_cm: i64,
        lofted: bool,
    },
    /// Shoot at the opponent goal.
    Shoot {
        aim_y: f64,
        power: f64,
        contact_height: f64,
    },
    /// Contest the nearest opponent.
    Duel,
    /// Carry the ball forward: the carrier's gesture when the scene shows
    /// nothing clearer. No event, no ball flight; the tick's movement
    /// already advances the carrier each tick (ADR-040 note 2026-11-09).
    Advance,
    /// Clear the ball away (ADR-051).
    Clear,
}

/// Builds the `PerceivedScene` from the engine's spatial state.
///
/// The engine provides the facts (positions, goal, the two squads); the
/// actor reads them in `ds_orchestrator::read::build_choice` (A.1-bis).
/// Plane B by construction: positions, not latents (D2).
#[must_use]
fn scene_from(
    spatial: &SpatialState,
    attacker: PlayerId,
    (goal, own_goal): ((i32, i32), (i32, i32)),
    teammates: Vec<PlayerId>,
    opponents: Vec<PlayerId>,
    (keeper, keeper_reach_cm): (Option<PlayerId>, f64),
    can_carry: bool,
) -> ds_orchestrator::PerceivedScene {
    let mut positions = std::collections::BTreeMap::new();
    for id in teammates.iter().chain(opponents.iter()) {
        if let Some(a) = spatial.agent(*id) {
            positions.insert(*id, a.position);
        }
    }
    ds_orchestrator::PerceivedScene {
        actor: attacker,
        teammates,
        opponents,
        positions,
        goal,
        own_goal,
        ground_keep: f64::from(spatial.ground_friction_milli()) / 1000.0,
        keeper,
        keeper_reach_cm,
        can_carry,
        penalty_kick: false,
    }
}

/// Whether the ball is inside the penalty area of the home (`home`) or
/// away side's goal (Law 1, `offball.json`).
const fn in_keeper_box(
    ball: (i32, i32),
    home: bool,
    (length, width): (i32, i32),
    off: &ds_calculator::offball::OffBallBalance,
) -> bool {
    let depth = off.keeper_box_depth_cm;
    let in_depth = if home {
        ball.0 <= depth
    } else {
        ball.0 >= length - depth
    };
    in_depth && (ball.1 - width / 2).abs() <= off.keeper_box_half_width_cm
}

/// What a side sees of its opponents when it has the ball (ADR-051, the
/// family "without the ball"): the line of their last outfield defender,
/// the goal it attacks, where they stand.
struct TeamView {
    forward: i32,
    their_goal_x: i32,
    last_line: Option<i32>,
    opponents: Vec<(i32, i32)>,
    /// Their outfield players other than the carrier: the men to mark.
    men: Vec<(i32, i32)>,
}

impl TeamView {
    fn new(
        spatial: &SpatialState,
        other: &ds_model::Squad,
        home: bool,
        length: i32,
        carrier: Option<PlayerId>,
    ) -> Self {
        let forward = if home { 1 } else { -1 };
        let their_keeper = other.players.first().map(|p| p.id);
        let last_line = other
            .players
            .iter()
            .filter(|q| Some(q.id) != their_keeper)
            .filter_map(|q| spatial.agent(q.id))
            .map(|a| a.position.0 * forward)
            .max()
            .map(|u| u * forward);
        let opponents = other
            .players
            .iter()
            .filter_map(|q| spatial.agent(q.id))
            .map(|a| a.position)
            .collect();
        let men = other
            .players
            .iter()
            .filter(|q| Some(q.id) != their_keeper && Some(q.id) != carrier)
            .filter_map(|q| spatial.agent(q.id))
            .map(|a| a.position)
            .collect();
        Self {
            forward,
            their_goal_x: if home { length } else { 0 },
            last_line,
            opponents,
            men,
        }
    }

    fn marker(&self, me: (i32, i32)) -> Option<(i32, i32)> {
        self.opponents
            .iter()
            .copied()
            .min_by_key(|o| distance_squared(*o, me))
    }

    /// The dangers a defender at `me` reads (ADR-052 §5): the value to
    /// them of the carrier's space, of the man he would mark, and of the
    /// space he guards (`held`), as much as an opponent can get into it
    /// soon. `home`: the defender's side.
    fn threats(
        &self,
        carrier: (i32, i32),
        me: (i32, i32),
        held: (i32, i32),
        home: bool,
        (length, width): (i32, i32),
    ) -> ds_orchestrator::Threats {
        let ours = (if home { 0 } else { length }, width / 2);
        let theirs = (length - ours.0, ours.1);
        let threat = |p: (i32, i32)| ds_orchestrator::read::space_value(p, ours, theirs);
        let into = self
            .marker(held)
            .map_or(0.0, |o| ds_orchestrator::read::soon(o, held));
        ds_orchestrator::Threats {
            carrier: threat(carrier),
            man: self.man_to_mark(me).map_or(0.0, threat),
            held: threat(held) * into,
        }
    }

    /// The man a defender at `me` would mark: the nearest of their outfield
    /// players other than the carrier.
    fn man_to_mark(&self, me: (i32, i32)) -> Option<(i32, i32)> {
        self.men
            .iter()
            .copied()
            .min_by_key(|o| distance_squared(*o, me))
    }

    /// What a player at `me` sees of his teammate's ball at `c`.
    fn mate_ball(
        &self,
        me: (i32, i32),
        c: (i32, i32),
        off: &ds_calculator::offball::OffBallBalance,
        (length, width): (i32, i32),
    ) -> ds_orchestrator::MateBall {
        let mark_radius_cm = off.mark_radius_cm;
        let ahead = (me.0 - c.0) * self.forward > 0;
        // ADR-052 §5: the value to his side of the spaces he could occupy,
        // as free as he would be there.
        let goal = (self.their_goal_x, width / 2);
        let other = (length - self.their_goal_x, width / 2);
        let worth = |p: (i32, i32)| ds_orchestrator::read::space_value(p, goal, other);
        let free = |p: (i32, i32)| {
            1.0 - self
                .marker(p)
                .map_or(0.0, |o| ds_orchestrator::read::soon(o, p))
        };
        let offer_at = self
            .act_target(ds_orchestrator::OffBallAct::Offer, me, off, (length, width))
            .unwrap_or(me);
        let run_at = self
            .act_target(ds_orchestrator::OffBallAct::Run, me, off, (length, width))
            .unwrap_or(me);
        let values = ds_orchestrator::SpaceValues {
            hold: worth(me) * free(me),
            offer: worth(offer_at) * free(offer_at),
            run: worth(run_at),
            run_cm: distance_squared(me, run_at).isqrt(),
            defender_cm: self
                .marker(run_at)
                .map_or(i64::MAX / 4, |o| distance_squared(o, run_at).isqrt()),
        };
        ds_orchestrator::MateBall {
            values,
            to_carrier_cm: distance_squared(me, c).isqrt(),
            marked: self
                .marker(me)
                .is_some_and(|o| distance_squared(o, me) <= i64::from(mark_radius_cm).pow(2)),
            to_line_cm: self
                .last_line
                .map(|l| i64::from((l - me.0) * self.forward))
                .filter(|d| ahead && *d >= 0),
            room_cm: self
                .last_line
                .map_or(0, |l| i64::from((self.their_goal_x - l).abs())),
        }
    }

    /// Where a player who chose to offer or to run goes: away from his
    /// marker to offer; to the last defender's line, just short of it, to
    /// run.
    fn act_target(
        &self,
        act: ds_orchestrator::OffBallAct,
        me: (i32, i32),
        off: &ds_calculator::offball::OffBallBalance,
        (length, width): (i32, i32),
    ) -> Option<(i32, i32)> {
        match act {
            ds_orchestrator::OffBallAct::Offer => self.marker(me).map(|o| {
                let (dx, dy) = (f64::from(me.0 - o.0), f64::from(me.1 - o.1));
                let len = dx.hypot(dy).max(1.0);
                let step = f64::from(off.offer_step_cm);
                #[allow(clippy::cast_possible_truncation)]
                let t = (
                    (me.0 + (dx / len * step) as i32).clamp(0, length),
                    (me.1 + (dy / len * step) as i32).clamp(0, width),
                );
                t
            }),
            ds_orchestrator::OffBallAct::Run => self.last_line.map(|l| {
                (
                    (l - self.forward * off.onside_margin_cm).clamp(0, length),
                    me.1,
                )
            }),
            ds_orchestrator::OffBallAct::Mark => self.man_to_mark(me).map(|m| {
                // Goal-side of him: between him and the goal we defend.
                let own_goal = (length - self.their_goal_x, width / 2);
                let (dx, dy) = (f64::from(own_goal.0 - m.0), f64::from(own_goal.1 - m.1));
                let len = dx.hypot(dy).max(1.0);
                let gap = f64::from(off.mark_gap_cm);
                #[allow(clippy::cast_possible_truncation)]
                let t = (
                    (m.0 + (dx / len * gap) as i32).clamp(0, length),
                    (m.1 + (dy / len * gap) as i32).clamp(0, width),
                );
                t
            }),
            _ => None,
        }
    }
}

/// Where a straight shot from `from` to `cross` (on the goal plane) passes
/// the depth `x` of the keeper, and the share of the flight it has done
/// there. A keeper behind the strike or beyond the plane meets it at the
/// plane (share 1).
fn meeting_point(from: (i32, i32, i32), cross: (i32, i32, i32), x: i32) -> ((i32, i32, i32), f64) {
    let total = i64::from(cross.0 - from.0);
    let done = i64::from(x - from.0);
    if total == 0 || done.signum() != total.signum() || done.abs() >= total.abs() {
        return (cross, 1.0);
    }
    let at = |a: i32, b: i32| a + i32::try_from(i64::from(b - a) * done / total).unwrap_or(0);
    #[allow(clippy::cast_precision_loss)] // centimetres on a pitch
    let share = done as f64 / total as f64;
    ((x, at(from.1, cross.1), at(from.2, cross.2)), share)
}

/// The rules' name for a side.
const fn side_of(p: Possession) -> crate::rules::Side {
    match p {
        Possession::Home => crate::rules::Side::Home,
        Possession::Away => crate::rules::Side::Away,
    }
}

/// The pitch in centimetres, `(length, width)`.
fn pitch_cm(ctx: &MatchContext<'_>) -> (i32, i32) {
    (
        i32::from(ctx.match_.pitch.length_m) * 100,
        i32::from(ctx.match_.pitch.width_m) * 100,
    )
}

impl MatchState {
    /// Creates the initial state: kickoff, home possession.
    ///
    /// # Panics
    ///
    /// Panics if the literal stream names used internally are empty. They
    /// are compile-time constants (`"match:actions"`, `"match:action_kind"`,
    /// `"match:player_pick"`), so this panic is unreachable in practice.
    #[must_use]
    pub fn new(match_: &ds_model::Match) -> Self {
        Self {
            score: Score { home: 0, away: 0 },
            possession: Possession::Home,
            home_possession_ticks: 0,
            away_possession_ticks: 0,
            shots: (0, 0),
            next_event_slot: 0,
            half: Half::First,
            half_start_tick: 0,
            spatial: SpatialState::from_squads(&match_.pitch, &match_.home, &match_.away),
            pending: None,
            scan_balance: crate::balance::ScanningBalance::load(),
            keeper_balance: crate::balance::KeeperBalance::load(),
            laws: crate::balance::LawsBalance::load(),
            cautioned: BTreeSet::new(),
            obstacle_balance: crate::balance::ObstacleBalance::load(),
            reception_balance: crate::balance::ReceptionBalance::load(),
            carry_balance: crate::balance::CarryBalance::load(),
            pressure_balance: crate::balance::PressureBalance::load(),
            last_decision: None,
            challenges: BTreeMap::new(),
            offball_due: BTreeMap::new(),
            offside_at_pass: BTreeSet::new(),
            restart_pass: None,
            bodies: {
                let mut m = BTreeMap::new();
                for p in match_.home.players.iter().chain(match_.away.players.iter()) {
                    m.insert(
                        p.id,
                        PlayerBody {
                            height_cm: f64::from(p.height_cm),
                            weight_kg: f64::from(p.weight_kg),
                            fatigue: f64::from(p.physical.fatigue),
                        },
                    );
                }
                m
            },
            coverage: BTreeSet::new(),
            coverage_possession: Possession::Home,
            latencies: BTreeMap::new(),
        }
    }

    /// Switches the current half and records the absolute tick at which it
    /// started (ADR-026).
    ///
    /// The timestamp of an event is relative to its half. The tick loop is
    /// continuous (the absolute tick keeps growing); this method is the
    /// only place the two are reconciled.
    pub const fn begin_half(&mut self, half: Half, start_tick: u64) {
        self.half = half;
        self.half_start_tick = start_tick;
    }
    /// Advances the time by one tick: the substrate of the match.
    ///
    /// Resolves the due pending (MD-37), advances the ball, checks the
    /// shot crossing, moves the agents, updates the coverage, counts the
    /// possession. It does **not** decide: no election of an actor, no
    /// latency, no `resolve_choice`. The decision is in `decide`, called
    /// by the orchestrator right after this method (IDEA-DESIGN-011,
    /// capture 2026-11-09 fourth pause: "togliere al tick il potere di
    /// decidere chi agisce").
    ///
    /// The prototype description of the old tick (2026-11-02: a roll on
    /// `action_probability`, four outcomes) is in `git log`; the roll is
    /// gone (ADR-035), the pace comes from the actor's latency.
    ///
    /// Declared distance from the ideal: the substrate is still inside
    /// `tick()`. Making the tick a pure counter is a separate refactor,
    /// tracked in IDEA-DESIGN-011's implementation note (2026-11-09).
    pub fn tick(&mut self, tick: u64, ctx: &mut MatchContext<'_>) {
        // Resolve a due pending first (MD-37): the possession is applied
        // here, not at launch. F.4-d: an unseen opponent on the line
        // intercepts, and the event is emitted at resolution.
        if let Some(p) = &self.pending {
            if tick >= p.due_tick {
                let outcome = p.outcome;
                let interceptor = p.interceptor;
                let passer = p.passer;
                let receiver = p.receiver;
                let freedom_cm = p.freedom_cm;
                let distance_cm = p.distance_cm;
                self.pending = None;
                self.possession = outcome;
                self.spatial.set_carrier(None);
                if let (Some(who), Some(by)) = (interceptor, passer) {
                    self.emit_event(
                        ctx,
                        tick,
                        Some(ActorId::Player(who)),
                        EventType::Interception,
                        serde_json::json!({
                            "from": by.slot(),
                            "to": who.slot(),
                        }),
                    );
                } else if let (Some(rcv), Some(fr), Some(di), Some(by)) =
                    (receiver, freedom_cm, distance_cm, passer)
                {
                    // REC: no interceptor at launch, the receiver tries to
                    // control. The pass succeeds if the receiver controls.
                    let params = self.reception_balance.to_params();
                    let rctx = ds_orchestrator::ReceptionContext {
                        receiver: rcv,
                        distance_cm: di,
                        freedom_cm: fr,
                    };
                    let rec = ctx.hidden.resolve_reception(&rctx, &params);
                    if !rec.controlled {
                        self.possession = Self::other(self.possession);
                    }
                    let fact = crate::fact::Fact::Trajectory {
                        actor: by,
                        kind: crate::fact::TrajectoryKind::Pass {
                            success: rec.controlled,
                        },
                    };
                    for c in crate::fact::classify(&fact) {
                        self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
                    }
                }
            }
        }
        let prev_ball = self.spatial.ball_3d();
        self.spatial.advance_ball();
        let curr_ball = self.spatial.ball_3d();
        self.check_shot_crossing(ctx, tick, prev_ball, curr_ball);
        self.restart_if_out(ctx, tick);
        self.spatial
            .set_home_in_possession(self.possession == Possession::Home);
        self.spatial.move_agents(
            i32::from(ctx.match_.pitch.length_m) * 100,
            i32::from(ctx.match_.pitch.width_m) * 100,
            &self.bodies,
            ctx.hidden,
            tick,
            &self.scan_balance,
            &self.keeper_balance,
        );
        self.spatial.follow_carrier();
        self.tire_players(ctx);
        self.meet_ball_in_air(ctx, tick);
        self.control_loose_ball(ctx, tick);
        self.challenge_carrier(ctx, tick);
        self.update_coverage(ctx, tick);
        match self.possession {
            Possession::Home => self.home_possession_ticks += 1,
            Possession::Away => self.away_possession_ticks += 1,
        }
    }

    /// The decision layer of the tick: elect the actor, resolve his
    /// latency, choose his action, execute the branch. This method is not
    /// the tick: the tick is the time and the substrate (ball, movement,
    /// coverage), this method is the decision that happens *at* the tick.
    /// The orchestrator calls `tick` then `decide` for every absolute tick;
    /// the tick does not know the latency, the decision does (IDEA-DESIGN-011,
    /// capture 2026-11-09 fourth pause: "togliere al tick il potere di
    /// decidere chi agisce").
    ///
    /// Declared distance from the ideal: the substrate (movement, coverage)
    /// is still inside `tick`. The tick is decision-free, which is the
    /// part that mattered; making the tick a pure counter is a separate
    /// refactor, tracked as a note on IDEA-011.
    #[allow(clippy::too_many_lines)]
    pub fn decide(&mut self, tick: u64, ctx: &mut MatchContext<'_>) {
        self.decide_off_ball(tick, ctx);
        let Some((attacker, defender)) = self.pick_from_space(ctx) else {
            return;
        };

        // IDEA-DESIGN-011 (R.1-latency): the actor decides when he is due,
        // not when a die says so. The latency is the actor's own pace,
        // resolved from his `decision_making` (ADR-035: no die). If the
        // actor is not due, the tick advances the body and the scene (above)
        // and returns; nothing needs deciding yet.
        // Only the player who has the ball can play it: the nearest player
        // of the side in possession does not kick a ball he is not
        // touching (OI-97; before, the ball left from his position, a
        // teleport). Without a carrier nobody decides: the players move,
        // and the first to reach the ball controls it (ADR-041 §5).
        if self.spatial.carrier() != Some(attacker) {
            return;
        }
        let due_at = self.latencies.get(&attacker).copied().unwrap_or(0);
        if tick < due_at {
            return;
        }

        let (teammates, opponents): (Vec<PlayerId>, Vec<PlayerId>) = {
            let (own, other) = self.squads(ctx);
            (
                own.players.iter().map(|p| p.id).collect(),
                other.players.iter().map(|p| p.id).collect(),
            )
        };
        let goal = match self.possession {
            Possession::Home => (i32::from(ctx.match_.pitch.length_m) * 100, 3400),
            Possession::Away => (0, 3400),
        };

        // Scene terms for the latency (ADR-044 §6: the duration is in
        // the relation). Both are geometric and in [0, 1]; the formula
        // weights them in `L0-match/balance/latency.json`.
        let (crowding, nearest_threat) = self.latency_scene(attacker, &opponents);
        let latency = ctx
            .hidden
            .resolve_latency(&ds_orchestrator::LatencyContext {
                player: attacker,
                crowding,
                nearest_threat,
            });
        self.latencies
            .insert(attacker, tick.saturating_add(latency.ticks));

        // A player looks at what he is about to play: he turns to the goal.
        if let Some(h) = self
            .spatial
            .agent(attacker)
            .and_then(|a| crate::perception::heading_towards(a.position, goal))
        {
            if let Some(am) = self.spatial.agent_mut(attacker) {
                am.heading = h;
            }
        }
        // Their keeper is the first of their squad (the formation's fact,
        // as in `SpatialState::goalkeepers`). Before `SIM_VERSION` 74 this
        // was `slot() == 0`, true only of the home keeper: a home shooter
        // never saw the away keeper as a keeper.
        let gk = opponents.first().copied();
        // Room to carry: farther from the goal than the edge of the box,
        // and not blocked. A carrier who has not got anywhere since his last
        // decision is blocked (an opponent in his way) and plays the ball
        // instead of pushing again (OI-97: one carrier stood 38 minutes
        // against a centre back, and the whole match with him).
        let here = self.spatial.agent(attacker).map(|a| a.position);
        let blocked = matches!(
            (self.last_decision, here),
            (Some((who, then)), Some(now)) if who == attacker && distance_squared(then, now) < 50 * 50
        );
        self.last_decision = here.map(|p| (attacker, p));
        let can_carry = !blocked
            && here.is_some_and(|p| {
                let d = distance_squared(p, goal);
                let stop = i64::from(self.carry_balance.carry_stop_from_goal_cm) + 100;
                d > stop * stop
            });
        let keeper_reach_cm = gk.and_then(|g| self.bodies.get(&g)).map_or(0.0, |b| {
            b.height_cm * ctx.hidden.save_balance().standing_reach_share
        });
        let own_goal = (i32::from(ctx.match_.pitch.length_m) * 100 - goal.0, goal.1);
        let mut scene = scene_from(
            &self.spatial,
            attacker,
            (goal, own_goal),
            teammates,
            opponents.clone(),
            (gk, keeper_reach_cm),
            can_carry,
        );
        scene.penalty_kick = self.restart_pass == Some(crate::rules::RestartKind::Penalty);
        let (chosen_kind, freedoms) =
            ds_orchestrator::Actor::new(attacker).decide(&scene, ctx.hidden, tick);
        let chosen = match chosen_kind {
            ds_orchestrator::ChoiceKind::Pass(receiver)
            | ds_orchestrator::ChoiceKind::Loft(receiver) => {
                let freedom_cm = freedoms
                    .get(&receiver)
                    .copied()
                    .unwrap_or(ds_orchestrator::read::PASS_FREEDOM_CM);
                ChosenAction::Pass {
                    receiver,
                    freedom_cm,
                    lofted: matches!(chosen_kind, ds_orchestrator::ChoiceKind::Loft(_)),
                }
            }
            ds_orchestrator::ChoiceKind::Shoot {
                aim_y,
                power,
                contact_height,
            } => ChosenAction::Shoot {
                aim_y,
                power,
                contact_height,
            },
            ds_orchestrator::ChoiceKind::Duel => ChosenAction::Duel,
            ds_orchestrator::ChoiceKind::Advance => ChosenAction::Advance,
            ds_orchestrator::ChoiceKind::Clear => ChosenAction::Clear,
        };

        // Only the restart's first pass is free of offside: any other play
        // by the taker ends it.
        // A dead ball is kicked unchallenged: nobody presses it (Laws 13-14).
        let dead_ball = self.restart_pass.is_some();
        if !matches!(chosen, ChosenAction::Pass { .. }) {
            self.restart_pass = None;
        }
        // The action belongs to the nearest agent; the ball follows.
        self.spatial.set_carrier(Some(attacker));
        if chosen == ChosenAction::Advance {
            // He carries the ball towards the goal he attacks
            // (IDEA-DESIGN-010); the ball follows him (`follow_carrier`).
            if let Some(a) = self.spatial.agent(attacker) {
                let (x, y) = a.position;
                let d = self.carry_balance.carry_distance_cm;
                // With room he carries the ball towards the goal; without
                // (blocked, or at the edge of the box with nothing on) he
                // turns away with it, towards the open pitch. Before
                // `SIM_VERSION` 50 he could only push towards the goal: a
                // carrier on the goal line with no pass stood there for
                // minutes, challenged every three seconds (seed 42, 70').
                let centre = (
                    i32::from(ctx.match_.pitch.length_m) * 50,
                    i32::from(ctx.match_.pitch.width_m) * 50,
                );
                let towards = if can_carry { goal } else { centre };
                let dx = towards.0 - x;
                let dy = towards.1 - y;
                let len = i64::from(dx).pow(2) + i64::from(dy).pow(2);
                let len = i32::try_from(len.isqrt()).unwrap_or(1).max(1);
                // Nobody dribbles into the goal: he carries the ball at most
                // to the edge of the box (`carry_stop_from_goal_cm`), where
                // he has to choose.
                let room = if can_carry {
                    len - self.carry_balance.carry_stop_from_goal_cm
                } else {
                    len
                };
                let step = d.min(room).max(0);
                self.spatial
                    .set_carry_target(Some((x + dx * step / len, y + dy * step / len)));
            }
        }

        if let ChosenAction::Pass {
            receiver,
            freedom_cm,
            lofted,
        } = chosen
        {
            let from = match self.spatial.agent(attacker) {
                Some(a) => (a.position.0, a.position.1, 0),
                None => self.spatial.ball_3d(),
            };
            let to = self
                .spatial
                .agent(receiver)
                .map_or((from.0 + 2000, from.1, from.2), |a| {
                    (a.position.0, a.position.1, 0)
                });
            let params = self.reception_balance.to_params();
            let ctx_pass = ds_orchestrator::PassContext {
                passer: attacker,
                from,
                to,
                receiver_freedom_cm: freedom_cm,
                ground_friction_milli: self.spatial.ground_friction_milli(),
                pitch_centre: (
                    i32::from(ctx.match_.pitch.length_m) * 50,
                    i32::from(ctx.match_.pitch.width_m) * 50,
                ),
                lofted: lofted.then(|| self.spatial.gravity_cm_per_tick2()),
            };
            let outcome = ctx.hidden.resolve_pass(&ctx_pass, &params);
            if lofted {
                self.tell_cross(ctx, tick, attacker, (from.0, from.1), (to.0, to.1));
            }
            // MD-25/F.4-c: carrier released at launch, ball flies, pending
            // resolves the possession (MD-37).
            self.spatial.set_carrier(None);
            self.spatial
                .set_ball_in_flight(from, outcome.initial_velocity);
            // F.4-d: save the opponents the passer did NOT see at launch.
            // The cone filters the interceptor, not the pass target.
            let seen = crate::perception::visible_to(&self.spatial, attacker);
            let unseen: Vec<PlayerId> = opponents
                .iter()
                .copied()
                .filter(|o| !seen.contains(o))
                .collect();
            // A lofted ball goes over the line: nobody on it cuts it out.
            let interceptor = if lofted {
                None
            } else {
                find_interceptor(&self.spatial, &unseen, (from.0, from.1), (to.0, to.1))
            };
            let expected = if interceptor.is_some() {
                Self::other(self.possession)
            } else {
                self.possession
            };
            let pass_distance_cm = {
                let dx = i64::from(to.0 - from.0);
                let dy = i64::from(to.1 - from.1);
                (dx * dx + dy * dy).isqrt()
            };
            self.mark_offside_at_pass(ctx, attacker, from.0);
            self.pending = Some(Pending {
                due_tick: tick + FLIGHT_TICKS,
                outcome: expected,
                interceptor,
                passer: Some(attacker),
                receiver: Some(receiver),
                freedom_cm: Some(freedom_cm),
                distance_cm: Some(pass_distance_cm),
                shot: None,
            });
            // REC: the Pass event is emitted at landing, in `tick`
            // (resolve_reception), not at launch: `complete` is the
            // reception, not the launch.
        } else if let ChosenAction::Shoot {
            aim_y,
            power,
            contact_height,
        } = chosen
        {
            let from = match self.spatial.agent(attacker) {
                Some(a) => (a.position.0, a.position.1, 0),
                None => self.spatial.ball_3d(),
            };
            let to = match self.possession {
                Possession::Home => (i32::from(ctx.match_.pitch.length_m) * 100, 3400, 244),
                Possession::Away => (0, 3400, 244),
            };
            // Difficulty: min distance to an opponent, scaled. Arbitrary,
            // declared: 0 cm -> 100 (max pressure), 1500 cm -> 0. Their
            // keeper on his line does not press the kicker (before
            // `SIM_VERSION` 74 he did, and a penalty was a kick under
            // pressure).
            let difficulty = opponents
                .iter()
                .skip(1)
                .filter(|_| !dead_ball)
                .filter_map(|o| {
                    self.spatial
                        .agent(*o)
                        .map(|a| distance_squared(a.position, (from.0, from.1)).isqrt())
                })
                .min()
                .map_or(0, |d| {
                    i32::try_from(((1500 - d).max(0) * 100) / 1500).unwrap_or(0)
                });
            let ctx_shot = ds_orchestrator::ShotContext {
                shooter: attacker,
                from,
                to,
                difficulty,
                aim_y,
                power,
                contact_height,
                momentum_y: self
                    .spatial
                    .agent(attacker)
                    .map_or(0, |a| i32::from(a.velocity.1)),
                gravity_cm_per_tick2: self.spatial.gravity_cm_per_tick2(),
            };
            let outcome = ctx.hidden.resolve_shot(&ctx_shot);
            // F.3-b3: carrier released, ball flies. Goal (or Save) is
            // emitted at the crossing tick by `check_shot_crossing`, not
            // at launch: the goal is geometric, not the die's outcome.
            self.spatial.set_carrier(None);
            self.spatial
                .set_ball_in_flight(from, outcome.initial_velocity);
            let fact = crate::fact::Fact::Trajectory {
                actor: attacker,
                kind: crate::fact::TrajectoryKind::Shot {
                    on_target: outcome.on_target,
                    from,
                    to: outcome.to,
                    initial_velocity: outcome.initial_velocity,
                    shot_type: outcome.shot_type,
                },
            };
            for c in crate::fact::classify(&fact) {
                self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
            }
            match self.possession {
                Possession::Home => {
                    self.shots.0 += 1;
                }
                Possession::Away => {
                    self.shots.1 += 1;
                }
            }
            self.pending = Some(Pending {
                due_tick: tick + FLIGHT_TICKS,
                outcome: Self::other(self.possession),
                interceptor: None,
                passer: None,
                receiver: None,
                freedom_cm: None,
                distance_cm: None,
                shot: Some(ShotPending {
                    attacker,
                    struck: tick,
                    from,
                }),
            });
        // Reachable from the carrier's scene only when `ChoiceKind::Duel`
        // is emitted; `build_choice` no longer emits it (a carrier has
        // the ball, the opponent does not; `ball_contested` is false by
        // construction). This branch stays for Plan E (the migration of
        // `resolve_duel` and the aerial family, IDEA-DESIGN-008): when a
        // defender is put in the carrier's scene, the branch becomes
        // reachable again. Removed-by: Plan E. Do not delete without
        // the migration: `resolve_duel` would lose its only caller.
        } else if chosen == ChosenAction::Clear {
            self.play_clearance(ctx, tick, attacker, own_goal);
        } else if chosen == ChosenAction::Duel {
            let ctx_duel = ds_orchestrator::DuelContext {
                attacker,
                defender,
                duel_type: ds_orchestrator::DuelType::Ground,
                tackle: ds_calculator::duel::TackleKind::Standing,
                from_behind: false,
                approach_cm_per_tick: 0,
                stakes: (1.0, 1.0, 1.0),
            };
            let outcome = ctx.hidden.resolve_duel(&ctx_duel);
            let fact = crate::fact::Fact::Contact {
                attacker,
                defender,
                winner: outcome.winner,
            };
            for c in crate::fact::classify(&fact) {
                self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
            }
            if outcome.winner == defender {
                self.flip_possession();
                self.spatial.set_carrier(Some(defender));
            }
        }
    }

    /// Emits a match-boundary event (`KickOff`, `HalfTime`, `SecondHalf`,
    /// `FullTime`). Boundary events have `actor = None` (ADR-026 §4).
    pub fn emit_boundary(&mut self, ctx: &mut MatchContext<'_>, tick: u64, event_type: EventType) {
        self.emit_event(ctx, tick, None, event_type, serde_json::json!({}));
    }
    /// The carrier is state, computed from the substrate (ADR-041 §5): a
    /// ball on the ground (at rest or rolling), with no pass or shot
    /// pending, belongs to the nearest
    /// player once he is within `loose_control_radius_cm` of it, and his
    /// team has the possession. Before this the ball stayed loose until the
    /// next decision, and the match stood still around it (OI-97).
    /// A lofted pass from a wide position into the opponents' penalty area
    /// is a cross (ADR-051): it is told as one.
    fn tell_cross(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        crosser: PlayerId,
        from: (i32, i32),
        to: (i32, i32),
    ) {
        let (length, width) = pitch_cm(ctx);
        let home = ctx.match_.home.players.iter().any(|p| p.id == crosser);
        let laws = self.laws;
        let goal_x = if home { length } else { 0 };
        let wide = (from.1 - width / 2).abs() > laws.penalty_area_half_width_cm;
        let into_area = (goal_x - to.0).abs() <= laws.penalty_area_depth_cm
            && (to.1 - width / 2).abs() <= laws.penalty_area_half_width_cm;
        if wide && into_area {
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(crosser)),
                EventType::Cross,
                serde_json::json!({ "x": from.0, "y": from.1 }),
            );
        }
    }

    /// Who meets a ball in the air at `(bx, by, bz)`: within his reach for
    /// a passing ball, and high enough (height, jump, a keeper's arms).
    fn aerial_contenders(
        &self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        (bx, by, bz): (i32, i32, i32),
    ) -> Vec<(PlayerId, f64, bool)> {
        let (length, width) = pitch_cm(ctx);
        let radius = self.reception_balance.loose_control_radius_cm;
        let off = ctx.hidden.offball_balance();
        let passer = self.pending.as_ref().and_then(|p| p.passer);
        let m = ctx.match_;
        let mut contenders: Vec<(PlayerId, f64, bool)> = Vec::new();
        for (squad, home) in [(&m.home, true), (&m.away, false)] {
            let keeper = squad.players.first().map(|p| p.id);
            for p in &squad.players {
                if Some(p.id) == passer || self.challenges.get(&p.id).is_some_and(|t| tick < *t) {
                    continue;
                }
                let Some(a) = self.spatial.agent(p.id).map(|a| a.position) else {
                    continue;
                };
                let reach = ctx.hidden.ball_reach_cm(p.id, radius);
                if distance_squared(a, (bx, by)) > reach * reach {
                    continue;
                }
                let height = self.bodies.get(&p.id).map_or(180.0, |b| b.height_cm);
                let hands =
                    Some(p.id) == keeper && in_keeper_box((bx, by), home, (length, width), &off);
                if ctx.hidden.aerial_reach_cm(p.id, height, hands) >= f64::from(bz) {
                    contenders.push((p.id, height, hands));
                }
            }
        }
        contenders
    }

    /// A header at goal (plan L0-CLOSE step 4): a shot, from the head.
    fn header_at_goal(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        winner: PlayerId,
        (bx, by, bz): (i32, i32, i32),
        goal: (i32, i32),
        speed: f64,
    ) {
        let home = ctx.match_.home.players.iter().any(|p| p.id == winner);
        let (dx, dy) = (f64::from(goal.0 - bx), f64::from(goal.1 - by));
        let len = dx.hypot(dy).max(1.0);
        #[allow(clippy::cast_possible_truncation)]
        let v = ((dx / len * speed) as i16, (dy / len * speed) as i16, 10);
        self.spatial.deflect_ball((bx, by, bz), v);
        self.pending = Some(Pending {
            due_tick: tick + FLIGHT_TICKS,
            outcome: Self::other(self.possession),
            interceptor: None,
            passer: None,
            receiver: None,
            freedom_cm: None,
            distance_cm: None,
            shot: Some(ShotPending {
                attacker: winner,
                struck: tick,
                from: (bx, by, bz),
            }),
        });
        if home {
            self.shots.0 += 1;
        } else {
            self.shots.1 += 1;
        }
        let fact = crate::fact::Fact::Trajectory {
            actor: winner,
            kind: crate::fact::TrajectoryKind::Shot {
                on_target: true,
                from: (bx, by, bz),
                to: (goal.0, goal.1, 100),
                initial_velocity: v,
                shot_type: ds_orchestrator::ShotType::Normal,
            },
        };
        for c in crate::fact::classify(&fact) {
            self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
        }
    }

    /// A ball in the air meets the players it passes within reach of (plan
    /// L0-CLOSE step 4, ADR-042): horizontally their reach for a passing
    /// ball, in height their height and jump (a keeper in his area adds his
    /// arms). Two or more: an aerial duel. The winner heads it at goal in
    /// the opponents' area, away from his goal in his own half, down to his
    /// feet elsewhere; a keeper in his area catches it. A shot on its way
    /// is not met here (it is the keeper's, or a block).
    fn meet_ball_in_air(&mut self, ctx: &mut MatchContext<'_>, tick: u64) {
        if self.spatial.ball_state() != crate::spatial::BallState::InFlight {
            return;
        }
        if self.pending.as_ref().is_some_and(|p| p.shot.is_some()) {
            return;
        }
        let (bx, by, bz) = self.spatial.ball_3d();
        if bz < 100 {
            return; // at the feet: the ground control takes it
        }
        let (length, width) = pitch_cm(ctx);
        let contenders = self.aerial_contenders(ctx, tick, (bx, by, bz));
        let m = ctx.match_;
        let Some(winner) = ctx.hidden.resolve_aerial(&contenders) else {
            return;
        };
        let home = m.home.players.iter().any(|p| p.id == winner);
        if contenders.len() > 1 {
            let against = contenders
                .iter()
                .find(|c| c.0 != winner)
                .map_or(0, |c| c.0.slot());
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(winner)),
                EventType::Duel,
                serde_json::json!({ "winner_slot": winner.slot(), "aerial": true, "against": against }),
            );
        }
        self.pending = None;
        self.offside_at_pass.clear();
        self.possession = if home {
            Possession::Home
        } else {
            Possession::Away
        };
        let hands = contenders.iter().any(|c| c.0 == winner && c.2);
        let goal = (if home { length } else { 0 }, width / 2);
        let own = (length - goal.0, goal.1);
        let speed = f64::from(ctx.hidden.duel_balance().header_speed_cm_per_tick);
        let depth = |g: (i32, i32)| (g.0 - bx).abs();
        let laws = self.laws;
        let in_their_area = depth(goal) <= laws.penalty_area_depth_cm
            && (by - goal.1).abs() <= laws.penalty_area_half_width_cm;
        if hands {
            // The keeper claims it.
            self.spatial.set_carrier(Some(winner));
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(winner)),
                EventType::Save,
                serde_json::json!({ "x": bx, "y": by, "z": bz, "held": true, "claim": true }),
            );
        } else if in_their_area {
            self.header_at_goal(ctx, tick, winner, (bx, by, bz), goal, speed);
        } else if depth(own) < length / 2 {
            // A header away from his goal: up and out.
            let forward = if own.0 == 0 { 1.0 } else { -1.0 };
            #[allow(clippy::cast_possible_truncation)]
            let v = ((forward * speed) as i16, 0, (speed * 0.3) as i16);
            self.spatial.deflect_ball((bx, by, bz), v);
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(winner)),
                EventType::Clearance,
                serde_json::json!({ "x": bx, "y": by, "header": true }),
            );
        } else {
            // Down to his feet.
            self.spatial.set_carrier(Some(winner));
        }
    }

    /// The carrier clears the ball (ADR-051, "clear"): his strongest kick,
    /// lifted, towards a point up the pitch on his side's wing; to no
    /// one. Out over a touchline it is a throw-in for the opponents.
    fn play_clearance(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        kicker: PlayerId,
        own_goal: (i32, i32),
    ) {
        let Some(from) = self.spatial.agent(kicker).map(|a| a.position) else {
            return;
        };
        let (length, width) = pitch_cm(ctx);
        let forward = if own_goal.0 == 0 { 1 } else { -1 };
        let k = ctx.hidden.kick_balance();
        let wing = if from.1 < width / 2 {
            width / 10
        } else {
            width - width / 10
        };
        let to = (
            (from.0 + forward * k.clear_length_cm).clamp(0, length),
            wing,
        );
        let v = ctx
            .hidden
            .resolve_clearance(kicker, (from.0, from.1, 0), to);
        self.spatial.set_carrier(None);
        self.spatial.set_ball_in_flight((from.0, from.1, 0), v);
        self.pending = None;
        self.emit_event(
            ctx,
            tick,
            Some(ActorId::Player(kicker)),
            EventType::Clearance,
            serde_json::json!({ "x": from.0, "y": from.1 }),
        );
    }

    /// Law 11: `offender` was in an offside position when the ball was
    /// played, and now he plays it. A free kick to the defenders where he
    /// is.
    fn call_offside(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        offender: PlayerId,
        offender_home: bool,
        ball: (i32, i32),
    ) {
        self.offside_at_pass.clear();
        let spot = self.spatial.agent(offender).map_or(ball, |a| a.position);
        let side = if offender_home {
            crate::rules::Side::Away
        } else {
            crate::rules::Side::Home
        };
        if let Some(taker) = self.apply_restart(ctx, crate::rules::free_kick(side, spot)) {
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(taker)),
                EventType::FreeKick,
                serde_json::json!({ "reason": "offside", "offender": offender.slot(), "x": spot.0, "y": spot.1 }),
            );
        }
    }

    /// Law 11: when `passer` plays the ball from `ball_x`, which of his
    /// teammates are in an offside position. Not from a throw-in, a goal
    /// kick or a corner (the first pass after such a restart).
    fn mark_offside_at_pass(&mut self, ctx: &MatchContext<'_>, passer: PlayerId, ball_x: i32) {
        self.offside_at_pass.clear();
        let restart = self.restart_pass.take();
        if matches!(
            restart,
            Some(
                crate::rules::RestartKind::ThrowIn
                    | crate::rules::RestartKind::GoalKick
                    | crate::rules::RestartKind::Corner
            )
        ) {
            return;
        }
        let m = ctx.match_;
        let home = m.home.players.iter().any(|p| p.id == passer);
        let (mates, others) = if home {
            (&m.home, &m.away)
        } else {
            (&m.away, &m.home)
        };
        let length = i32::from(m.pitch.length_m) * 100;
        let goal_x = if home { length } else { 0 };
        let opponents_x: Vec<i32> = others
            .players
            .iter()
            .filter_map(|q| self.spatial.agent(q.id))
            .map(|a| a.position.0)
            .collect();
        for q in &mates.players {
            if q.id == passer {
                continue;
            }
            if let Some(a) = self.spatial.agent(q.id) {
                if crate::rules::offside_position(
                    a.position.0,
                    ball_x,
                    &opponents_x,
                    goal_x,
                    length,
                ) {
                    self.offside_at_pass.insert(q.id);
                }
            }
        }
    }

    /// A shot on its way meets a defender (ADR-051, "block"): an outfield
    /// opponent of the shooter whose reach for a passing ball (his
    /// reactivity) it passes within, below his block height, blocks it.
    /// The ball comes back loose; possession is the defenders', so out over
    /// the goal line it is a corner.
    fn block_shot(&mut self, ctx: &mut MatchContext<'_>, tick: u64, shooter: Option<PlayerId>) {
        let Some(shooter) = shooter else { return };
        let off = ctx.hidden.offball_balance();
        let (bx, by, bz) = self.spatial.ball_3d();
        if bz > off.block_height_cm {
            return;
        }
        let m = ctx.match_;
        let shooter_home = m.home.players.iter().any(|p| p.id == shooter);
        let defenders = if shooter_home { &m.away } else { &m.home };
        let keeper = defenders.players.first().map(|p| p.id);
        let radius = self.reception_balance.loose_control_radius_cm;
        let blocker = defenders
            .players
            .iter()
            .map(|p| p.id)
            .filter(|id| Some(*id) != keeper)
            .find(|id| {
                self.spatial.agent(*id).is_some_and(|a| {
                    let reach = ctx.hidden.ball_reach_cm(*id, radius);
                    distance_squared(a.position, (bx, by)) <= reach * reach
                })
            });
        let Some(blocker) = blocker else { return };
        let vel = self.spatial.ball_velocity();
        let body = self.spatial.agent(blocker).map_or((bx, by), |a| a.position);
        self.spatial.deflect_ball(
            (bx, by, bz),
            ds_calculator::obstacle::reflect_off_body(vel, body, (bx, by), off.block_rebound_milli),
        );
        self.pending = None;
        self.possession = if shooter_home {
            Possession::Away
        } else {
            Possession::Home
        };
        self.emit_event(
            ctx,
            tick,
            Some(ActorId::Player(blocker)),
            EventType::Block,
            serde_json::json!({ "from": shooter.slot(), "x": bx, "y": by }),
        );
    }

    /// The nearest player within his own reach of the ball (his
    /// reactivity, ADR-051); one whose touch just failed needs a moment,
    /// and nobody takes his own pass.
    fn who_takes_the_ball(
        &self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        passer: Option<PlayerId>,
        ball: (i32, i32),
    ) -> Option<PlayerId> {
        let radius = self.reception_balance.loose_control_radius_cm;
        let players: Vec<PlayerId> = ctx
            .match_
            .home
            .players
            .iter()
            .chain(ctx.match_.away.players.iter())
            .map(|p| p.id)
            .collect();
        let mut best: Option<(i64, PlayerId)> = None;
        for who in players {
            // Nobody receives his own pass.
            if Some(who) == passer || self.challenges.get(&who).is_some_and(|t| tick < *t) {
                continue;
            }
            let Some(pos) = self.spatial.agent(who).map(|a| a.position) else {
                continue;
            };
            let d = distance_squared(pos, ball);
            let reach = ctx.hidden.ball_reach_cm(who, radius);
            if d <= reach * reach && best.is_none_or(|(bd, _)| d < bd) {
                best = Some((d, who));
            }
        }
        best.map(|(_, id)| id)
    }

    fn control_loose_ball(&mut self, ctx: &mut MatchContext<'_>, tick: u64) {
        // On the ground: at rest, or rolling (a footballer controls a ball
        // that is still moving). A ball in the air is not his yet.
        let on_ground = match self.spatial.ball_state() {
            crate::spatial::BallState::Loose => true,
            crate::spatial::BallState::InFlight => self.spatial.ball_3d().2 == 0,
            crate::spatial::BallState::Possessed => false,
        };
        // A pass on its way can be received where it reaches a player
        // (ADR-051, "receive"): before `SIM_VERSION` 52 nobody could touch
        // it until a fixed 1.5 s had passed, and a weighed pass rolled past
        // its receiver. A shot on its way is the keeper's.
        let pass = match &self.pending {
            Some(p) if p.shot.is_none() => Some((p.passer, p.freedom_cm, p.distance_cm)),
            Some(p) => {
                let shooter = p.shot.as_ref().map(|s| s.attacker);
                self.block_shot(ctx, tick, shooter);
                return;
            }
            None => None,
        };
        if !on_ground {
            return;
        }
        // The nearest player who reaches it, each with his own reach (his
        // reactivity, ADR-051); one whose touch just failed needs a moment.
        let ball = self.spatial.ball();
        let passer = pass.and_then(|(by, _, _)| by);
        let Some(id) = self.who_takes_the_ball(ctx, tick, passer, ball) else {
            return;
        };
        let home = |who: PlayerId| ctx.match_.home.players.iter().any(|p| p.id == who);
        if let Some((Some(by), freedom, distance)) = pass {
            self.pending = None;
            if home(by) == home(id) && self.offside_at_pass.contains(&id) {
                self.call_offside(ctx, tick, id, home(id), ball);
                return;
            }
            self.offside_at_pass.clear();
            if home(by) == home(id) {
                // A teammate receives: his first touch decides (REC).
                let params = self.reception_balance.to_params();
                let rec = ctx.hidden.resolve_reception(
                    &ds_orchestrator::ReceptionContext {
                        receiver: id,
                        distance_cm: distance.unwrap_or(0),
                        freedom_cm: freedom.unwrap_or(params.freedom_ref_cm),
                    },
                    &params,
                );
                let fact = crate::fact::Fact::Trajectory {
                    actor: by,
                    kind: crate::fact::TrajectoryKind::Pass {
                        success: rec.controlled,
                    },
                };
                for c in crate::fact::classify(&fact) {
                    self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
                }
                if !rec.controlled {
                    // A heavy touch: the ball runs on, slower, for whoever.
                    let v = self.spatial.ball_velocity();
                    self.spatial
                        .deflect_ball(self.spatial.ball_3d(), (v.0 / 2, v.1 / 2, 0));
                    let retry = ctx.hidden.duel_balance().retry_ticks;
                    self.challenges.insert(id, tick + retry);
                    return;
                }
            } else {
                self.emit_event(
                    ctx,
                    tick,
                    Some(ActorId::Player(id)),
                    EventType::Interception,
                    serde_json::json!({ "from": by.slot(), "to": id.slot() }),
                );
            }
        }
        self.spatial.set_carrier(Some(id));
        self.possession = if home(id) {
            Possession::Home
        } else {
            Possession::Away
        };
        // He has just controlled it: he decides after his own time, read
        // in his scene (ADR-044 §6), not on the tick he receives it.
        let opponents = if home(id) {
            &ctx.match_.away
        } else {
            &ctx.match_.home
        };
        let wait = self.decision_time(ctx, id, opponents);
        self.latencies.insert(id, tick.saturating_add(wait));
    }

    /// Every player without the ball chooses his own act when his latency
    /// expires (ADR-049): hold his place, press, challenge, go to the ball.
    /// He reads his scene from where he stands; the engine only records the
    /// choice and applies it (`move_agents`, `challenge_carrier`).
    // One pass over the twenty-two, the scene of each built in one place;
    // the scene moves to its own type with the off-ball attack (plan
    // L0-CLOSE step 2).
    #[allow(clippy::too_many_lines)]
    fn decide_off_ball(&mut self, tick: u64, ctx: &mut MatchContext<'_>) {
        let ball = self.spatial.ball();
        let carrier = self.spatial.carrier();
        // A ball nobody has, at rest or rolling, is anyone's to go for. A
        // pass on its way is its receiver's to meet (ADR-051, receive); an
        // opponent cuts it out only if it passes within his reach. A shot
        // on its way is nobody's.
        let ball_loose = carrier.is_none() && self.pending.is_none();
        let meets_pass = self
            .pending
            .as_ref()
            .filter(|p| p.shot.is_none())
            .and_then(|p| p.receiver);
        let duel = ctx.hidden.duel_balance();
        // Within a slide's reach he may go in; how is chosen at the contact
        // (ADR-042, amended).
        let slide_reach = i64::from(duel.reach_cm) * i64::from(duel.slide_reach_milli) / 1000;
        let off = ctx.hidden.offball_balance();
        let (length, width) = pitch_cm(ctx);
        for (squad, other, home) in [
            (&ctx.match_.home, &ctx.match_.away, true),
            (&ctx.match_.away, &ctx.match_.home, false),
        ] {
            let carrier_opponent = carrier.filter(|c| other.players.iter().any(|p| p.id == *c));
            // With a teammate on the ball (ADR-051, "without the ball"):
            // the carrier, the opponents' last outfield defender's line and
            // the goal this side attacks.
            let carrier_mate = carrier
                .filter(|c| squad.players.iter().any(|p| p.id == *c))
                .and_then(|c| self.spatial.agent(c))
                .map(|a| a.position);
            let view = TeamView::new(&self.spatial, other, home, length, carrier);
            let carrier_pos = carrier_opponent
                .and_then(|c| self.spatial.agent(c))
                .map(|a| a.position);
            let keeper_id = squad.players.first().map(|p| p.id);
            for p in &squad.players {
                let id = p.id;
                let Some(me) = self.spatial.agent(id).map(|a| a.position) else {
                    continue;
                };
                // A presser who arrives within reach chooses at once: the
                // scene has changed, he does not wait for his next turn.
                let arrived = self.spatial.act(id) == Some(ds_orchestrator::OffBallAct::Press)
                    && self.challenges.get(&id).is_none_or(|t| tick >= *t)
                    && carrier_pos.is_some_and(|c| {
                        distance_squared(me, c) <= i64::from(duel.reach_cm).pow(2)
                    });
                // A pass played to him changes his scene too: he goes to
                // meet it at once, unless he already is.
                let called = meets_pass == Some(id)
                    && self.spatial.act(id) != Some(ds_orchestrator::OffBallAct::GoToBall);
                if Some(id) == carrier
                    || (!arrived && !called && self.offball_due.get(&id).is_some_and(|t| tick < *t))
                {
                    continue;
                }
                let keeper = Some(id) == keeper_id;
                // How many outfield teammates he sees closer than him.
                let rank = |target: (i32, i32)| {
                    let mine = distance_squared(me, target);
                    u32::try_from(
                        squad
                            .players
                            .iter()
                            .filter(|q| q.id != id && Some(q.id) != keeper_id)
                            .filter_map(|q| self.spatial.agent(q.id))
                            .filter(|a| distance_squared(a.position, target) < mine)
                            .count(),
                    )
                    .unwrap_or(u32::MAX)
                };
                let to_carrier = carrier_pos.map(|c| distance_squared(me, c).isqrt());
                let ball_in_own_box = in_keeper_box(ball, home, (length, width), &off);
                let sees_ball =
                    crate::perception::perceived_ball(&self.spatial, id, ball).is_some();
                let scene = ds_orchestrator::OffBallScene {
                    actor: id,
                    place: if keeper {
                        ds_orchestrator::Place::Keeper {
                            // A pass meant for him is his to meet, wherever
                            // it comes from.
                            ball_in_area: ball_in_own_box || meets_pass == Some(id),
                        }
                    } else {
                        ds_orchestrator::Place::Outfield
                    },
                    loose_ball: ((ball_loose && sees_ball) || meets_pass == Some(id)).then(|| {
                        ds_orchestrator::Target {
                            distance_cm: distance_squared(me, ball).isqrt(),
                            rank: rank(ball),
                        }
                    }),
                    carrier: carrier_pos
                        .zip(to_carrier)
                        .map(|(c, d)| ds_orchestrator::Target {
                            distance_cm: d,
                            rank: rank(c),
                        }),
                    carrier_in_reach: to_carrier.is_some_and(|d| d <= slide_reach),
                    with_mate: carrier_mate.map(|c| view.mate_ball(me, c, &off, (length, width))),
                    to_mark: carrier_pos.and_then(|_| view.man_to_mark(me)).map(|m| {
                        ds_orchestrator::Target {
                            distance_cm: distance_squared(me, m).isqrt(),
                            rank: rank(m),
                        }
                    }),
                    threats: carrier_pos.map(|c| {
                        let held = self
                            .spatial
                            .block_target(id, ball, false, false)
                            .unwrap_or(me);
                        view.threats(c, me, held, home, (length, width))
                    }),
                };
                let act = ctx.hidden.choose_off_ball(&scene);
                self.spatial.set_act(id, act);
                if let Some(t) = view.act_target(act, me, &off, (length, width)) {
                    self.spatial.set_act_target(id, t);
                }
                let wait = self.decision_time(ctx, id, other);
                self.offball_due.insert(id, tick.saturating_add(wait));
            }
        }
    }

    /// The challenge is the defender's choice (ADR-049, ADR-042): a player
    /// who chose to challenge, and has the carrier within reach, tries to
    /// win the ball, once per choice. The contact decides, with no die
    /// (`resolve_duel`; the goalkeeper's one-on-one reads his goalkeeping);
    /// if the challenger wins, the ball is his. The one who lost the ball
    /// does not win it back at once.
    fn challenge_carrier(&mut self, ctx: &mut MatchContext<'_>, tick: u64) {
        let Some(carrier) = self.spatial.carrier() else {
            return;
        };
        // A dead ball is not challenged (Laws 13-17): until the taker
        // plays it, the restart is his.
        if self.pending.is_some() || self.restart_pass.is_some() {
            return;
        }
        let ball = self.spatial.ball();
        let duel = ctx.hidden.duel_balance();
        let reach = i64::from(duel.reach_cm);
        let slide_reach = reach * i64::from(duel.slide_reach_milli) / 1000;
        let (_, opponents) = self.squads(ctx);
        let keeper = opponents.players.first().map(|p| p.id);
        let Some(challenger) = opponents.players.iter().map(|p| p.id).find(|id| {
            self.spatial.act(*id) == Some(ds_orchestrator::OffBallAct::Challenge)
                && self.challenges.get(id).is_none_or(|t| tick >= *t)
                && self.spatial.agent(*id).is_some_and(|a| {
                    distance_squared(a.position, ball) <= slide_reach * slide_reach
                })
        }) else {
            return;
        };
        let in_reach = self
            .spatial
            .agent(challenger)
            .is_some_and(|a| distance_squared(a.position, ball) <= reach * reach);
        let Some(tackle) = self.how_he_goes_in(ctx, (carrier, challenger), keeper, in_reach) else {
            return;
        };
        // One attempt per choice: then he keeps pressing until he chooses again.
        self.spatial
            .set_act(challenger, ds_orchestrator::OffBallAct::Press);
        if tackle == ds_calculator::duel::TackleKind::Jockey {
            // He stays in front and waits: no contact this time.
            self.challenges
                .insert(challenger, tick + duel.retry_ticks / 2);
            return;
        }
        let (from_behind, approach) = self.challenge_geometry(ctx, carrier, challenger, ball);
        let stakes = Self::duel_stakes(ctx, carrier, ball);
        let outcome = ctx.hidden.resolve_duel(&ds_orchestrator::DuelContext {
            attacker: carrier,
            defender: challenger,
            duel_type: if Some(challenger) == keeper {
                ds_orchestrator::DuelType::WithGoalkeeper
            } else {
                ds_orchestrator::DuelType::Ground
            },
            tackle,
            from_behind,
            approach_cm_per_tick: approach,
            stakes,
        });
        self.emit_contact(ctx, tick, carrier, challenger, &outcome);
        let down = if tackle == ds_calculator::duel::TackleKind::Slide {
            duel.slide_recovery_ticks
        } else {
            duel.retry_ticks
        };
        if outcome.foul_committed.is_some() {
            self.challenges.insert(challenger, tick + down);
            let facts = crate::rules::FoulFacts {
                slide: tackle == ds_calculator::duel::TackleKind::Slide,
                from_behind,
                approach_milli: approach * 1000 / duel.speed_ref_cm_per_tick.max(1),
                denies_obvious_chance: false,
            };
            self.apply_foul(
                ctx,
                tick,
                (carrier, challenger),
                facts,
                outcome.ball_retained,
            );
            return;
        }
        if outcome.loose {
            self.loose_from_contact(challenger, ball, duel.loose_speed_cm_per_tick);
            self.challenges.insert(challenger, tick + down);
        } else if outcome.winner == challenger {
            self.flip_possession();
            self.spatial.set_carrier(Some(challenger));
            self.challenges.insert(carrier, tick + duel.retry_ticks);
        } else if outcome.answer == Some(ds_calculator::duel::CarrierAnswer::TakeOn) {
            self.beaten_by_dribble(
                ctx,
                tick,
                (carrier, challenger),
                down.max(duel.beaten_ticks),
            );
        } else {
            // Beaten, he needs time to recover before he can try again; a
            // slide that missed leaves him on the ground longer.
            self.challenges.insert(challenger, tick + down);
        }
    }

    /// Taken on and beaten (ADR-042 step c): the man is left behind for
    /// `down` ticks, and the dribble is an event.
    fn beaten_by_dribble(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        (carrier, challenger): (PlayerId, PlayerId),
        down: u64,
    ) {
        self.challenges.insert(challenger, tick + down);
        self.spatial
            .set_act(challenger, ds_orchestrator::OffBallAct::Hold);
        self.emit_event(
            ctx,
            tick,
            Some(ActorId::Player(carrier)),
            EventType::Dribble,
            serde_json::json!({ "past": challenger.slot() }),
        );
    }

    /// What the carrier imagines is at stake in a challenge (ADR-052 §3):
    /// the space he keeps, the space a few metres past the man towards
    /// the goal he attacks, and what the opponents would have where he is.
    fn duel_stakes(ctx: &MatchContext<'_>, carrier: PlayerId, ball: (i32, i32)) -> (f64, f64, f64) {
        let (length, width) = pitch_cm(ctx);
        let home = ctx.match_.home.players.iter().any(|p| p.id == carrier);
        let (goal, other) = if home {
            ((length, width / 2), (0, width / 2))
        } else {
            ((0, width / 2), (length, width / 2))
        };
        let forward = if home { 1 } else { -1 };
        let past = ((ball.0 + forward * 400).clamp(0, length), ball.1);
        (
            ds_orchestrator::read::space_value(ball, goal, other),
            ds_orchestrator::read::space_value(past, goal, other),
            ds_orchestrator::read::space_value(ball, other, goal),
        )
    }

    /// How the challenger goes in: his choice (ADR-042 §2, amended); the
    /// keeper goes in on his feet, inside his area (ADR-049). `None`: the
    /// keeper is not within reach.
    fn how_he_goes_in(
        &self,
        ctx: &mut MatchContext<'_>,
        (carrier, challenger): (PlayerId, PlayerId),
        keeper: Option<PlayerId>,
        in_reach: bool,
    ) -> Option<ds_calculator::duel::TackleKind> {
        let ball = self.spatial.ball();
        // How he goes in is his choice (ADR-042 §2, amended); the keeper
        // goes in on his feet, inside his area (ADR-049).
        if Some(challenger) == keeper {
            return in_reach.then_some(ds_calculator::duel::TackleKind::Standing);
        }
        {
            // In his own penalty area he is careful (a foul is a penalty).
            let defending_home = !ctx.match_.home.players.iter().any(|p| p.id == carrier);
            let in_area = in_keeper_box(
                ball,
                defending_home,
                pitch_cm(ctx),
                &ctx.hidden.offball_balance(),
            );
            let booked = self.cautioned.contains(&challenger);
            Some(
                ctx.hidden
                    .choose_tackle(challenger, (in_reach, in_area, booked)),
            )
        }
    }

    /// The contact's geometry (ADR-042 §4): whether the challenger comes
    /// from behind the carrier (from the side the carrier is leaving), and
    /// how fast he arrives, cm/tick.
    fn challenge_geometry(
        &self,
        ctx: &MatchContext<'_>,
        carrier: PlayerId,
        challenger: PlayerId,
        ball: (i32, i32),
    ) -> (bool, i32) {
        let ch = self
            .spatial
            .agent(challenger)
            .map(|a| (a.position, a.velocity));
        let carrier_home = ctx.match_.home.players.iter().any(|p| p.id == carrier);
        let forward = if carrier_home { 1 } else { -1 };
        let from_behind = ch.is_some_and(|(p, _)| (p.0 - ball.0) * forward < -50);
        #[allow(clippy::cast_possible_truncation)]
        let approach = ch.map_or(0, |(_, v)| f64::from(v.0).hypot(f64::from(v.1)) as i32);
        (from_behind, approach)
    }

    /// Neither has it: the ball runs from the contact, away from the
    /// defender, for whoever gets there (ADR-042, amended).
    fn loose_from_contact(&mut self, challenger: PlayerId, ball: (i32, i32), speed: i32) {
        let from = self.spatial.agent(challenger).map_or(ball, |a| a.position);
        let (dx, dy) = (f64::from(ball.0 - from.0), f64::from(ball.1 - from.1));
        let len = dx.hypot(dy).max(1.0);
        let sp = f64::from(speed);
        #[allow(clippy::cast_possible_truncation)]
        let v = ((dx / len * sp) as i16, (dy / len * sp) as i16, 0);
        self.spatial.set_carrier(None);
        self.spatial.set_ball_in_flight((ball.0, ball.1, 0), v);
    }

    /// Law 12, applied objectively (ADR-021 note, ADR-042 step b): the foul
    /// is called; its card follows the facts (a second yellow is a red, a
    /// red sends the player off); then the advantage, or the free kick where
    /// it happened, or a penalty if it happened in the penalty area.
    fn apply_foul(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        (fouled, offender): (PlayerId, PlayerId),
        mut facts: crate::rules::FoulFacts,
        kept: bool,
    ) {
        let (length, width) = pitch_cm(ctx);
        let spot = self
            .spatial
            .agent(fouled)
            .map_or_else(|| self.spatial.ball(), |a| a.position);
        let fouled_home = ctx.match_.home.players.iter().any(|p| p.id == fouled);
        let goal = (if fouled_home { length } else { 0 }, width / 2);
        let depth = |p: (i32, i32)| (goal.0 - p.0).abs();
        let laws = self.laws;
        let in_area = depth(spot) <= laws.penalty_area_depth_cm
            && (spot.1 - goal.1).abs() <= laws.penalty_area_half_width_cm;
        // An obvious goal-scoring opportunity: near the goal he attacks, and
        // no defender but the offender, the keeper included, nearer to it.
        let defenders = if fouled_home {
            &ctx.match_.away
        } else {
            &ctx.match_.home
        };
        let cover = defenders
            .players
            .iter()
            .filter(|q| q.id != offender)
            .filter_map(|q| self.spatial.agent(q.id))
            .any(|a| depth(a.position) < depth(spot));
        facts.denies_obvious_chance =
            distance_squared(spot, goal) <= i64::from(laws.dogso_distance_cm).pow(2) && !cover;
        self.emit_event(
            ctx,
            tick,
            Some(ActorId::Player(offender)),
            EventType::Foul,
            serde_json::json!({ "against": fouled.slot(), "x": spot.0, "y": spot.1, "slide": facts.slide, "from_behind": facts.from_behind }),
        );
        let card = crate::rules::card_for(
            &facts,
            laws.reckless_speed_milli,
            laws.excessive_speed_milli,
        );
        let card = match card {
            Some(crate::rules::Card::Yellow) if !self.cautioned.insert(offender) => {
                Some(crate::rules::Card::Red)
            }
            c => c,
        };
        if let Some(c) = card {
            let red = c == crate::rules::Card::Red;
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(offender)),
                EventType::Card,
                serde_json::json!({ "color": if red { "red" } else { "yellow" } }),
            );
            if red {
                // Off the pitch: to the bench, by the halfway line.
                self.spatial.send_off(offender, (length / 2, -300));
            }
        }
        // Advantage: the side that suffered it keeps the ball in the
        // opponents' half, outside the area; play goes on.
        if kept && !in_area && depth(spot) < length / 2 {
            return;
        }
        let side = if fouled_home {
            crate::rules::Side::Home
        } else {
            crate::rules::Side::Away
        };
        let (restart, kind) = if in_area {
            let x = goal.0
                - if fouled_home {
                    laws.penalty_spot_cm
                } else {
                    -laws.penalty_spot_cm
                };
            (
                crate::rules::Restart {
                    side,
                    spot: (x, goal.1),
                    kind: crate::rules::RestartKind::Penalty,
                },
                EventType::Penalty,
            )
        } else {
            (crate::rules::free_kick(side, spot), EventType::FreeKick)
        };
        if let Some(taker) = self.apply_restart(ctx, restart) {
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(taker)),
                kind,
                serde_json::json!({ "reason": "foul", "x": restart.spot.0, "y": restart.spot.1 }),
            );
        }
    }

    /// The contact's events: a `Duel`, and a `Tackle` when the defender wins
    /// it; a loose ball is a duel nobody won (`loose: true`).
    fn emit_contact(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        carrier: PlayerId,
        challenger: PlayerId,
        outcome: &ds_orchestrator::DuelOutcome,
    ) {
        if outcome.loose {
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(carrier)),
                EventType::Duel,
                serde_json::json!({ "winner_slot": carrier.slot(), "loose": true, "against": challenger.slot() }),
            );
            return;
        }
        let fact = crate::fact::Fact::Contact {
            attacker: carrier,
            defender: challenger,
            winner: outcome.winner,
        };
        for c in crate::fact::classify(&fact) {
            self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
        }
    }

    /// The ball out of play gets a restart (Laws 15-17, `crate::rules`):
    /// the rule decides who restarts, from where and how; the engine
    /// applies it. Before, a ball that left the pitch elsewhere than into
    /// the goal stayed out for the rest of the match (OI-97).
    fn restart_if_out(&mut self, ctx: &mut MatchContext<'_>, tick: u64) {
        if self.spatial.ball_state() == crate::spatial::BallState::Possessed {
            return;
        }
        let (x, y, _) = self.spatial.ball_3d();
        let Some(restart) =
            crate::rules::out_of_play((x, y), side_of(self.possession), pitch_cm(ctx))
        else {
            return;
        };
        if let Some(taker) = self.apply_restart(ctx, restart) {
            // Every time the ball leaves the pitch the restart is told
            // (Laws 15-17): throw-in, goal kick, corner.
            let kind = match restart.kind {
                crate::rules::RestartKind::Corner => Some(EventType::Corner),
                crate::rules::RestartKind::ThrowIn => Some(EventType::ThrowIn),
                crate::rules::RestartKind::GoalKick => Some(EventType::GoalKick),
                _ => None,
            };
            if let Some(kind) = kind {
                self.emit_event(
                    ctx,
                    tick,
                    Some(ActorId::Player(taker)),
                    kind,
                    serde_json::json!({ "x": restart.spot.0, "y": restart.spot.1 }),
                );
            }
        }
    }

    /// Fatigue, the first dimmer (plan L0-CLOSE step 6, OI-70): each
    /// player on the pitch tires by the effort of this tick, his speed,
    /// against his endurance, and recovers when he walks.
    fn tire_players(&mut self, ctx: &mut MatchContext<'_>) {
        for (id, body) in &mut self.bodies {
            let Some(a) = self.spatial.agent(*id) else {
                continue;
            };
            let speed = f64::from(a.velocity.0).hypot(f64::from(a.velocity.1));
            body.fatigue = ctx.hidden.tire(*id, body.fatigue, speed);
        }
    }

    /// The kick-off (Law 8): the home side kicks off the first half, the
    /// away side the second. Before, nobody was on the ball and nobody saw
    /// it, so the match began only because the nearest player could kick a
    /// ball he was not touching (OI-97).
    pub fn kick_off(&mut self, ctx: &mut MatchContext<'_>, half: Half) {
        if half == Half::Second {
            // The half-time rest (plan L0-CLOSE step 6).
            let f = ctx.hidden.fatigue_balance();
            for b in self.bodies.values_mut() {
                b.fatigue = f.after_half_time(b.fatigue);
            }
        }
        let side = match half {
            Half::First => crate::rules::Side::Home,
            Half::Second => crate::rules::Side::Away,
        };
        let _ = self.apply_restart(ctx, crate::rules::kick_off(side, pitch_cm(ctx)));
    }

    /// Applies a restart: the nearest player of the restarting side goes to
    /// the spot, the ball is placed at rest at his feet, and it is his.
    /// Returns the taker.
    fn apply_restart(
        &mut self,
        ctx: &mut MatchContext<'_>,
        restart: crate::rules::Restart,
    ) -> Option<PlayerId> {
        let taker = self.restart_taker(ctx, &restart)?;
        self.pending = None;
        self.offside_at_pass.clear();
        self.restart_pass = Some(restart.kind);
        self.spatial.set_position(taker, restart.spot);
        self.spatial
            .set_ball_in_flight((restart.spot.0, restart.spot.1, 0), (0, 0, 0));
        self.spatial.advance_ball();
        self.spatial.set_carrier(Some(taker));
        self.possession = match restart.side {
            crate::rules::Side::Home => Possession::Home,
            crate::rules::Side::Away => Possession::Away,
        };
        self.clear_the_spot(ctx, &restart, taker);
        if restart.kind == crate::rules::RestartKind::Penalty {
            // He kicks before anyone can come back into the area (Law 14:
            // they stay out until the ball is kicked).
            self.latencies.remove(&taker);
        }
        Some(taker)
    }

    /// Who takes a restart (plan L0-CLOSE step 7): the keeper takes his goal
    /// kick; a corner, a free kick or a penalty goes to the best kicker of
    /// that ball among the outfield players within `TAKER_RADIUS_CM`; the
    /// others to the nearest.
    fn restart_taker(
        &self,
        ctx: &mut MatchContext<'_>,
        restart: &crate::rules::Restart,
    ) -> Option<PlayerId> {
        const TAKER_RADIUS_CM: i64 = 3000;
        use crate::rules::RestartKind;
        let squad = match restart.side {
            crate::rules::Side::Home => &ctx.match_.home,
            crate::rules::Side::Away => &ctx.match_.away,
        };
        let nearest = nearest_in_squad(&self.spatial, squad, restart.spot);
        let keeper = squad
            .players
            .first()
            .map(|p| p.id)
            .filter(|id| self.spatial.agent(*id).is_some());
        match restart.kind {
            RestartKind::GoalKick => keeper.or(nearest),
            RestartKind::Corner | RestartKind::FreeKick | RestartKind::Penalty => {
                let mut near: Vec<(i64, PlayerId)> = squad
                    .players
                    .iter()
                    .skip(1)
                    .filter_map(|p| {
                        self.spatial
                            .agent(p.id)
                            .map(|a| (distance_squared(a.position, restart.spot), p.id))
                    })
                    .filter(|(d, _)| *d <= TAKER_RADIUS_CM * TAKER_RADIUS_CM)
                    .collect();
                near.sort_by(|x, y| x.0.cmp(&y.0).then(x.1.slot().cmp(&y.1.slot())));
                let ids: Vec<PlayerId> = near.into_iter().map(|(_, id)| id).collect();
                ctx.hidden
                    .set_piece_taker(&ids, restart.kind == RestartKind::Penalty)
                    .or(nearest)
            }
            RestartKind::KickOff | RestartKind::ThrowIn => nearest,
        }
    }

    /// The distances of the Laws (Laws 13-14), applied to the players: at
    /// a free kick or a corner the opponents stand at least 9.15 m from the
    /// ball; at a penalty everyone but the taker and the defending keeper
    /// leaves the penalty area, and the keeper stands on his line between
    /// the posts.
    fn clear_the_spot(
        &mut self,
        ctx: &MatchContext<'_>,
        restart: &crate::rules::Restart,
        taker: PlayerId,
    ) {
        const DISTANCE_CM: i64 = 915;
        use crate::rules::RestartKind;
        let (length, width) = pitch_cm(ctx);
        let restarting_home = restart.side == crate::rules::Side::Home;
        let ids: Vec<(PlayerId, bool)> = ctx
            .match_
            .home
            .players
            .iter()
            .map(|p| (p.id, true))
            .chain(ctx.match_.away.players.iter().map(|p| (p.id, false)))
            .collect();
        match restart.kind {
            RestartKind::FreeKick | RestartKind::Corner => {
                for (id, home) in ids {
                    if home == restarting_home {
                        continue;
                    }
                    let Some(p) = self.spatial.agent(id).map(|a| a.position) else {
                        continue;
                    };
                    let d2 = distance_squared(p, restart.spot);
                    if d2 >= DISTANCE_CM * DISTANCE_CM {
                        continue;
                    }
                    // Back along the line from the ball, or towards his
                    // goal if he stands on it.
                    let (dx, dy) = (
                        i64::from(p.0 - restart.spot.0),
                        i64::from(p.1 - restart.spot.1),
                    );
                    let len = d2.isqrt();
                    let (ux, uy) = if len == 0 {
                        (if home { -1 } else { 1 }, 0)
                    } else {
                        (dx, dy)
                    };
                    let l = if len == 0 { 1 } else { len };
                    let to = (
                        i32::try_from(i64::from(restart.spot.0) + ux * DISTANCE_CM / l)
                            .unwrap_or(restart.spot.0)
                            .clamp(0, length),
                        i32::try_from(i64::from(restart.spot.1) + uy * DISTANCE_CM / l)
                            .unwrap_or(restart.spot.1)
                            .clamp(0, width),
                    );
                    self.spatial.set_position(id, to);
                }
            }
            RestartKind::Penalty => {
                // The goal the penalty is kicked at.
                let goal_x = if restarting_home { length } else { 0 };
                // Outside the area and 9.15 m behind the mark.
                let back = self
                    .laws
                    .penalty_area_depth_cm
                    .max(self.laws.penalty_spot_cm + 915)
                    + 100;
                let edge = if restarting_home { length - back } else { back };
                let defending_keeper = if restarting_home {
                    ctx.match_.away.players.first().map(|p| p.id)
                } else {
                    ctx.match_.home.players.first().map(|p| p.id)
                };
                for (id, _) in ids {
                    if id == taker {
                        continue;
                    }
                    if Some(id) == defending_keeper {
                        self.spatial.set_position(id, (goal_x, width / 2));
                        continue;
                    }
                    let Some(p) = self.spatial.agent(id).map(|a| a.position) else {
                        continue;
                    };
                    let inside = (p.0 - goal_x).abs() < (edge - goal_x).abs();
                    if inside {
                        self.spatial.set_position(id, (edge, p.1));
                    }
                }
            }
            RestartKind::KickOff | RestartKind::ThrowIn | RestartKind::GoalKick => {}
        }
    }

    /// What the pitch looks like after `tick` (the tick record,
    /// IDEA-INFRA-018). Observable facts only: positions, ball, carrier,
    /// score, clock. `events` is the range of the chain born in this tick.
    pub(crate) fn record(
        &self,
        tick: u64,
        ticks_per_second: u8,
        events: std::ops::Range<usize>,
    ) -> crate::record::TickRecord {
        let tps = u64::from(ticks_per_second).max(1);
        let half_seconds =
            u32::try_from(tick.saturating_sub(self.half_start_tick) / tps).unwrap_or(u32::MAX);
        crate::record::TickRecord {
            tick,
            half: self.half,
            half_seconds,
            ball: crate::record::BallFrame {
                position: self.spatial.ball_3d(),
                state: self.spatial.ball_state(),
            },
            carrier: self.spatial.carrier(),
            agents: {
                let mut agents: Vec<crate::record::AgentFrame> = self
                    .spatial
                    .iter()
                    .map(|(id, a)| crate::record::AgentFrame {
                        id: *id,
                        position: a.position,
                        heading: a.heading,
                        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                        fatigue_milli: self
                            .bodies
                            .get(id)
                            .map_or(0, |b| (b.fatigue * 1000.0).round() as u16),
                    })
                    .collect();
                // The players sent off are listed too, in id order.
                agents.sort_by_key(|a| a.id);
                agents
            },
            score: self.score,
            events,
        }
    }

    /// Produces the final result.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn finish(&self, chain: EventChain) -> MatchResult {
        let total = self.home_possession_ticks + self.away_possession_ticks;
        let possession = if total == 0 {
            (0.5, 0.5)
        } else {
            let home = (self.home_possession_ticks as f32) / (total as f32);
            (home, 1.0 - home)
        };

        let state_hash = chain.chain_hash();

        MatchResult {
            score: self.score,
            events: chain,
            stats: MatchStatistics {
                possession,
                shots: self.shots,
                goals: (self.score.home, self.score.away),
            },
            state_hash,
            narrative: Vec::new(),
        }
    }

    /// The opposite possession, without mutating `self`. The expected
    /// possession is computed at launch and applied only when the pending
    /// resolves (MD-37): the ball in flight belongs to nobody (MD-25).
    const fn other(p: Possession) -> Possession {
        match p {
            Possession::Home => Possession::Away,
            Possession::Away => Possession::Home,
        }
    }
    const fn flip_possession(&mut self) {
        self.possession = match self.possession {
            Possession::Home => Possession::Away,
            Possession::Away => Possession::Home,
        };
    }

    /// The attacker and defender of the next action, from space (slice B2).
    ///
    /// The attacker is the agent of the possessing team nearest to the
    /// ball; the defender is the nearest opponent to the attacker. Ties
    /// are broken by `PlayerId` (deterministic).
    ///
    /// Special case of influence (`03-entities-minimal` §2.9): the general
    /// case reads the latents inside `pcfm-hidden`; here only the space term
    /// is used. The per-agent slice replaces this with the full formula.
    fn pick_from_space(&self, ctx: &MatchContext<'_>) -> Option<(PlayerId, PlayerId)> {
        let (attacker_squad, defender_squad) = self.squads(ctx);
        let ball = self.spatial.ball();
        let attacker = nearest_in_squad(&self.spatial, attacker_squad, ball)?;
        let attacker_pos = self.spatial.agent(attacker).map_or(ball, |a| a.position);
        let defender = nearest_in_squad(&self.spatial, defender_squad, attacker_pos)?;
        Some((attacker, defender))
    }

    const fn squads<'a>(&self, ctx: &'a MatchContext<'_>) -> (&'a Squad, &'a Squad) {
        match self.possession {
            Possession::Home => (&ctx.match_.home, &ctx.match_.away),
            Possession::Away => (&ctx.match_.away, &ctx.match_.home),
        }
    }

    /// The two scene terms for the latency formula (ADR-044 §6).
    ///
    /// `crowding` is the number of opponents within `CROWD_RADIUS_CM` of
    /// the actor, saturated at `CROWD_MAX` and normalised to `[0, 1]`.
    /// `nearest_threat` is the distance to the nearest opponent, normalised
    /// so that `0.0` means at the actor's feet and `1.0` means at
    /// `THREAT_RADIUS_CM` or beyond. Both are geometric and deterministic;
    /// no die, no latent.
    /// How long `id` takes to choose again, in ticks, read in his scene
    /// against `opponents` (ADR-044 §6).
    fn decision_time(
        &self,
        ctx: &mut MatchContext<'_>,
        id: PlayerId,
        opponents: &ds_model::Squad,
    ) -> u64 {
        let ids: Vec<PlayerId> = opponents.players.iter().map(|q| q.id).collect();
        let (crowding, nearest_threat) = self.latency_scene(id, &ids);
        ctx.hidden
            .resolve_latency(&ds_orchestrator::LatencyContext {
                player: id,
                crowding,
                nearest_threat,
            })
            .ticks
    }

    fn latency_scene(&self, actor: PlayerId, opponents: &[PlayerId]) -> (f64, f64) {
        let Some(actor_pos) = self.spatial.agent(actor).map(|a| a.position) else {
            return (0.0, 0.0);
        };
        let mut crowd_count: u32 = 0;
        let mut nearest_sq: i64 = i64::MAX;
        for &opp in opponents {
            let Some(op) = self.spatial.agent(opp).map(|a| a.position) else {
                continue;
            };
            let dx = i64::from(op.0 - actor_pos.0);
            let dy = i64::from(op.1 - actor_pos.1);
            let d_sq = dx * dx + dy * dy;
            if d_sq < nearest_sq {
                nearest_sq = d_sq;
            }
            let r = i64::from(CROWD_RADIUS_CM);
            if d_sq <= r * r {
                crowd_count += 1;
            }
        }
        let crowd_norm = f64::from(crowd_count.min(CROWD_MAX)) / f64::from(CROWD_MAX);
        // `nearest_sq` is i64::MAX when there are no opponents: threat 0.
        #[allow(clippy::cast_precision_loss)]
        let nearest_cm = if nearest_sq == i64::MAX {
            f64::MAX
        } else {
            #[allow(clippy::cast_precision_loss)]
            {
                (nearest_sq as f64).sqrt()
            }
        };
        let threat = (1.0 - nearest_cm / THREAT_RADIUS_CM).clamp(0.0, 1.0);
        (crowd_norm, threat)
    }

    /// Emits `RelationEnterExit` when an opponent enters (or exits) the
    /// coverage of the carrier (MD-04). Coverage is "opponent within
    /// `space_threshold_cm` of the ball": a geometric relation, not a
    /// per-tick event. The narrator classifies the enter fact as
    /// `Pressure` (T1); the engine emits the fact, not the event.
    fn update_coverage(&mut self, ctx: &mut MatchContext<'_>, tick: u64) {
        // Loaded once (it was parsed from JSON at every tick). An opponent
        // presses within the radius, and keeps pressing until he is
        // `pressure_exit_milli` of it away: without the hysteresis an
        // opponent on the edge "entered" again every tick.
        let p = self.pressure_balance;
        let r = i64::from(p.pressure_radius_cm);
        let threshold_sq = r * r;
        let exit = r * i64::from(p.pressure_exit_milli) / 1000;
        let exit_sq = exit * exit;
        let ball = self.spatial.ball();
        // Pressure is on a carrier: a ball in flight or loose is pressed by
        // nobody. Before `SIM_VERSION` 50 every opponent the ball flew past
        // "pressed" it; with passes at their real speed that was 2,000
        // Pressure events a match.
        if self.spatial.carrier().is_none() {
            self.coverage.clear();
            return;
        }

        // Compute the current coverage set (immutable phase).
        let (nominal_carrier, current): (PlayerId, BTreeSet<PlayerId>) = {
            let (own, opp) = self.squads(ctx);
            let mut set: BTreeSet<PlayerId> = BTreeSet::new();
            for p in &opp.players {
                if let Some(a) = self.spatial.agent(p.id) {
                    let d = distance_squared(a.position, ball);
                    if d <= threshold_sq || (self.coverage.contains(&p.id) && d <= exit_sq) {
                        set.insert(p.id);
                    }
                }
            }
            let Some(carrier) = nearest_in_squad(&self.spatial, own, ball) else {
                return;
            };
            (carrier, set)
        };

        // Possession flipped: the previous coverage referred to the other
        // team, discard it (do not emit enters against a stale reference).
        if self.possession != self.coverage_possession {
            self.coverage.clear();
            self.coverage_possession = self.possession;
        }

        // Diff after the flip handling, so a possession change starts from
        // an empty reference and every current member is an enter.
        let enters: Vec<PlayerId> = current.difference(&self.coverage).copied().collect();

        for opponent in enters {
            let fact = crate::fact::Fact::RelationEnterExit {
                carrier: nominal_carrier,
                opponent,
                entered: true,
            };
            for c in crate::fact::classify(&fact) {
                self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
            }
        }

        self.coverage = current;
    }

    /// Which obstacle the ball meets at the goal plane, if any (ADR-048 §1).
    ///
    /// Pure geometry, no state: `Some(0)` a post, `Some(2)` the crossbar,
    /// `None` nothing (the ball goes in or out, the caller decides).
    #[must_use]
    const fn obstacle_hit(
        y_at: i32,
        z_at: i32,
        center_y: i32,
        half_w: i32,
        height: i32,
        post_r: i32,
        bar_t: i32,
    ) -> Option<u8> {
        let lateral_edge = (y_at - center_y).abs() - half_w;
        let vertical_edge = z_at - height;
        let hit_post = lateral_edge.abs() <= post_r && z_at <= height;
        let hit_bar = vertical_edge.abs() <= bar_t && (y_at - center_y).abs() < half_w;
        if hit_post {
            Some(0)
        } else if hit_bar {
            Some(2)
        } else {
            None
        }
    }

    /// The ball's velocity after it meets the woodwork at `impact` on the
    /// goal plane, or `None` if it meets nothing (ADR-048 §1). Pure: the
    /// geometry is `obstacle_hit`, the reflection is the calculator's.
    ///
    /// The post reverses the x component (the ball comes back toward the
    /// pitch); the crossbar reverses x and z (it comes back, and down or up
    /// off the bar).
    fn goal_plane_deflection(
        impact: (i32, i32, i32),
        center_y: i32,
        velocity: (i16, i16, i16),
        b: &crate::balance::ObstacleBalance,
    ) -> Option<(i16, i16, i16)> {
        let hit = Self::obstacle_hit(
            impact.1,
            impact.2,
            center_y,
            b.goal_half_width_cm,
            b.goal_height_cm,
            b.post_radius_cm,
            b.crossbar_thickness_cm,
        )?;
        Some(if hit == 0 {
            b.post_reflection().reflect(velocity)
        } else {
            b.crossbar_reflections()
                .iter()
                .fold(velocity, |v, r| r.reflect(v))
        })
    }

    /// The keeper meets a shot on target (ADR-051, the family "in goal"):
    /// `true` if he stopped it (held or parried; the event is emitted).
    fn keeper_meets_the_shot(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        gk: PlayerId,
        (from, struck): ((i32, i32, i32), u64),
        (plane_x, y_at, z_at): (i32, i32, i32),
        goal_center_y: i32,
    ) -> bool {
        // ADR-051, the family "in goal": the keeper reacts, dives in the
        // time the ball takes, and holds or parries what he reaches. He
        // meets the ball where it passes him, not on the goal line: off his
        // line he narrows the angle and has less time (the line from the
        // strike to the crossing, cut at his depth). The distance is from
        // the centre of his body, across and up.
        let share = ctx.hidden.save_balance().standing_reach_share;
        let reach_cm = self.bodies.get(&gk).map_or(0.0, |b| b.height_cm * share);
        let v = self.spatial.ball_velocity();
        let speed = f64::from(v.0).hypot(f64::from(v.1)).hypot(f64::from(v.2));
        #[allow(clippy::cast_precision_loss)] // ticks of one match: far below 2^52
        let flight_ticks = tick.saturating_sub(struck) as f64;
        let outcome = self.spatial.agent(gk).map(|gk_agent| {
            let (meet, share) = meeting_point(from, (plane_x, y_at, z_at), gk_agent.position.0);
            let body = ctx.hidden.save_balance().body_centre_cm;
            let across = f64::from(meet.1 - gk_agent.position.1);
            let up = f64::from(meet.2) - body;
            ctx.hidden.resolve_save(&ds_orchestrator::SaveContext {
                goalkeeper: gk,
                point: (plane_x, y_at, z_at),
                offset_cm: across.hypot(up),
                reach_cm,
                flight_ticks: flight_ticks * share,
                ball_speed_cm_per_tick: speed,
            })
        });

        let Some(o) = outcome.filter(|o| o.saved) else {
            return false;
        };
        {
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(gk)),
                EventType::Save,
                serde_json::json!({ "x": plane_x, "y": y_at, "z": z_at, "held": o.held }),
            );
            if o.held {
                // In his hands: the ball is his.
                self.spatial.set_carrier(Some(gk));
            } else {
                // Parried: nearer him, pushed back into play away from the
                // centre of the goal; at the edge of his reach only tipped,
                // going on past the post (a corner if it crosses the line).
                let k = i32::from(
                    i16::try_from(ctx.hidden.save_balance().parry_rebound_milli).unwrap_or(0),
                );
                let away = if y_at >= goal_center_y { 1 } else { -1 };
                let rebound = |c: i16| i16::try_from(i32::from(c) * k / 1000).unwrap_or(0);
                let side = i16::try_from(away * i32::from(v.0).abs() * k / 1000).unwrap_or(0);
                let vx = if o.tipped {
                    rebound(v.0)
                } else {
                    rebound(v.0).saturating_neg()
                };
                self.spatial
                    .deflect_ball((plane_x, y_at, z_at), (vx, side, rebound(v.2).abs()));
            }
        }
        true
    }

    /// Checks whether the in-flight ball crossed a goal plane this tick
    /// (F.3-b3). The goal is born from the geometric crossing, not from
    /// the die's outcome: `on_target` is the die's intent, the crossing
    /// is what happens.
    ///
    /// If a shot is pending and the ball crossed a goal plane:
    /// - outside the goal mouth: possession to the defending team
    ///   (declared derogation MD-38, ball out stops the match);
    /// - inside, goalkeeper reaches (`reach_cm = height_cm / 2`): save via
    ///   `resolve_save`, possession to the defender;
    /// - inside, no save: `Goal` emitted, score updated.
    ///
    /// The pending is cleared in all three cases. Pure geometry plus one
    /// `resolve_save`; no other RNG.
    fn check_shot_crossing(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        prev: (i32, i32, i32),
        curr: (i32, i32, i32),
    ) {
        let (attacker, struck, from) = match self.pending.as_ref().and_then(|p| p.shot.as_ref()) {
            Some(s) => (s.attacker, s.struck, s.from),
            None => return,
        };
        let pitch_length = i32::from(ctx.match_.pitch.length_m) * 100;
        let width = i32::from(ctx.match_.pitch.width_m) * 100;

        let (plane_x, defending) = if prev.0 < pitch_length && curr.0 >= pitch_length {
            (pitch_length, Possession::Away)
        } else if prev.0 > 0 && curr.0 <= 0 {
            (0, Possession::Home)
        } else {
            return;
        };
        let dx = i64::from(curr.0 - prev.0);
        if dx == 0 {
            return;
        }
        let t_num = i64::from(plane_x - prev.0);
        let y_at = prev.1 + i32::try_from((i64::from(curr.1 - prev.1) * t_num) / dx).unwrap_or(0);
        let z_at = prev.2 + i32::try_from((i64::from(curr.2 - prev.2) * t_num) / dx).unwrap_or(0);
        let goal_half_w = self.obstacle_balance.goal_half_width_cm;
        let goal_h = self.obstacle_balance.goal_height_cm;
        let goal_center_y = width / 2;
        let inside = (y_at - goal_center_y).abs() < goal_half_w && (0..goal_h).contains(&z_at);

        // ADR-048 §1: the ball meets the post or the crossbar. If it does,
        // the ball is deflected and keeps flying from the impact point: the
        // pending is not cleared, no goal, no save. The geometry and the
        // reflection are `goal_plane_deflection`, pure and testable.
        //
        // The woodwork is told (L0.05 1.2, the maintainer, 2026-11-13).
        let impact = (plane_x, y_at, z_at);
        if let Some(v) = Self::goal_plane_deflection(
            impact,
            goal_center_y,
            self.spatial.ball_velocity(),
            &self.obstacle_balance,
        ) {
            self.spatial.deflect_ball(impact, v);
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(attacker)),
                EventType::Woodwork,
                serde_json::json!({ "x": plane_x, "y": y_at, "z": z_at }),
            );
            return;
        }

        self.pending = None;
        self.spatial.set_carrier(None);

        if !inside {
            // Wide or over: the shooter touched it last, so it is a goal
            // kick when it leaves the pitch (Law 16). Before `SIM_VERSION`
            // 55 the possession went to the defenders here, and the rule
            // read their touch: a corner for every wide shot.
            return;
        }

        let gk = match defending {
            Possession::Home => PlayerId::new(0, 0),
            Possession::Away => PlayerId::new(100, 0),
        };
        if self.keeper_meets_the_shot(
            ctx,
            tick,
            gk,
            (from, struck),
            (plane_x, y_at, z_at),
            goal_center_y,
        ) {
            self.possession = defending;
            return;
        }

        let fact = crate::fact::Fact::GoalLineCrossing {
            actor: Some(ActorId::Player(attacker)),
            point: (plane_x, y_at, z_at),
        };
        for c in crate::fact::classify(&fact) {
            self.emit_event(ctx, tick, c.actor, c.event_type, c.fields);
        }
        match defending {
            Possession::Home => self.score.away += 1,
            Possession::Away => self.score.home += 1,
        }
        self.possession = defending;
        // Law 8: after a goal, the side that conceded kicks off. Before, the
        // ball stayed in the net for the rest of the match (OI-97).
        let _ = self.apply_restart(
            ctx,
            crate::rules::kick_off(side_of(defending), pitch_cm(ctx)),
        );
    }

    fn emit_event(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        actor: Option<ActorId>,
        event_type: EventType,
        fields: serde_json::Value,
    ) {
        let ticks_per_sec = u64::from(ctx.config.ticks_per_second).max(1);
        let total_secs = tick / ticks_per_sec;
        let half_secs = (tick.saturating_sub(self.half_start_tick)) / ticks_per_sec;
        let minute = clamp_u8(half_secs / 60, 120);
        let second = clamp_u8(half_secs % 60, 60);
        let minutes_played = clamp_u16(total_secs / 60);

        let event = MatchEvent {
            event_id: EventId::new(self.next_event_slot, 0),
            match_id: ctx.match_.id,
            actor,
            half: self.half,
            timestamp_minute: minute,
            timestamp_second: second,
            minutes_played_cumulative: minutes_played,
            event_type,
            fields,
            created_at: GameInstant::from_raw(tick),
        };
        self.next_event_slot += 1;
        let result = ctx.chain.try_append(event);
        debug_assert!(
            result.is_ok(),
            "engine produced an invalid event: {result:?}"
        );
    }
}

/// The player of `squad` nearest to `point`, tie-broken by `PlayerId`.
///
/// Returns `None` if the squad has no agent placed in `spatial`. This is
/// a declared invariant, not a fallback: `SpatialState::from_squads`
/// places every starter of every squad. The invariant is enforced at
/// construction, and the caller decides how to degrade. A regression
/// would make the caller skip the action, and `golden_l0` would see it.
/// D2: no panic in a simulation path.
fn nearest_in_squad(spatial: &SpatialState, squad: &Squad, point: (i32, i32)) -> Option<PlayerId> {
    squad
        .players
        .iter()
        .filter_map(|p| {
            spatial
                .agent(p.id)
                .map(|a| (distance_squared(point, a.position), p.id))
        })
        .min_by(|x, y| x.0.cmp(&y.0).then(x.1.slot().cmp(&y.1.slot())))
        .map(|(_, id)| id)
}

/// Clamps a `u64` to a `u8`, with a hard upper bound of `max`.
fn clamp_u8(value: u64, max: u8) -> u8 {
    u8::try_from(value.min(u64::from(max))).unwrap_or(max)
}

/// Clamps a `u64` to a `u16`.
fn clamp_u16(value: u64) -> u16 {
    u16::try_from(value).unwrap_or(u16::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ds_core::ids::SquadId;
    use ds_model::{
        FormationId, PhysicalStateMinimal, Pitch, Player, PsychologicalStateMinimal, RoleId,
    };

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
            FormationId::new("4-4-2").unwrap(),
        )
        .unwrap()
    }

    fn setup() -> (SpatialState, Vec<PlayerId>, Vec<PlayerId>) {
        let home = make_squad(0);
        let away = make_squad(100);
        let s = SpatialState::from_squads(&Pitch::default(), &home, &away);
        let mates: Vec<PlayerId> = home.players.iter().map(|p| p.id).collect();
        let opps: Vec<PlayerId> = away.players.iter().map(|p| p.id).collect();
        (s, mates, opps)
    }

    #[test]
    fn audit_cone_does_not_filter_the_pass_target() {
        let (mut s, _mates, _opps) = setup();
        let attacker = PlayerId::new(1, 0);
        let mate = PlayerId::new(2, 0);
        s.set_position(attacker, (5000, 3400));
        s.agent_mut(attacker).unwrap().heading = 0; // north (+y)
        // The mate is directly south: outside the cone, and the only
        // candidate. The goal is far (no shoot).
        s.set_position(mate, (5000, 2400));
        // Candidates: only the attacker and the mate. No opponent: freedom
        // is maximal, so the mate qualifies. The audit is on perception,
        // not on the selection: `build_choice` is what the engine sees,
        // `resolve_choice` is downstream (it reads the cognitive latents,
        // which are not the subject of this test). If the cone filtered
        // the pass target, the mate would not appear among the Pass
        // candidates (A, 2026-11-04: "filtro partita vera", the target is
        // a choice, not perception).
        let teammates = [attacker, mate];
        let opponents: [PlayerId; 0] = [];
        let scene = scene_from(
            &s,
            attacker,
            ((10500, 3400), (0, 3400)),
            teammates.to_vec(),
            opponents.to_vec(),
            (None, 0.0),
            true,
        );
        let balance = ds_orchestrator::ShotChoiceBalance::load();
        let (ctx, _freedoms) = ds_orchestrator::read::build_choice(
            &scene,
            ds_orchestrator::Side::Right,
            0.5,
            0.04,
            &balance,
        );
        let pass_target_seen = ctx
            .candidates
            .iter()
            .any(|c| matches!(c.kind, ds_orchestrator::ChoiceKind::Pass(p) if p == mate));
        assert!(
            pass_target_seen,
            "the cone filtered the pass target: candidates = {:?}",
            ctx.candidates
        );
    }
}

#[cfg(test)]
mod intercept_tests {
    use super::*;

    #[test]
    fn point_on_the_segment_is_zero() {
        assert_eq!(point_segment_dist_sq((0, 0), (100, 0), (50, 0)), 0);
    }

    #[test]
    fn point_beyond_the_end_uses_the_endpoint() {
        // Beyond b: distance to b, 10 cm.
        assert_eq!(point_segment_dist_sq((0, 0), (100, 0), (110, 0)), 100);
    }
    #[test]

    fn point_before_the_start_uses_the_startpoint() {
        assert_eq!(point_segment_dist_sq((0, 0), (100, 0), (-10, 0)), 100);
    }

    #[test]
    fn perpendicular_distance_is_computed() {
        // 30 cm above the middle of a 100 cm segment.
        assert_eq!(point_segment_dist_sq((0, 0), (100, 0), (50, 30)), 900);
    }

    /// A shot arriving at the goal plane at x = 10500, travelling toward
    /// the goal: `(vx, vy, vz)`.
    const SHOT_V: (i16, i16, i16) = (250, 10, -5);

    #[test]
    fn a_shot_on_the_post_changes_trajectory() {
        // ADR-048 §1 (P1, first way): the post reverses vx, the ball comes
        // back toward the pitch; the other components are unchanged.
        let b = crate::balance::ObstacleBalance::load();
        let impact = (10_500, 3400 + b.goal_half_width_cm, 100);
        let v = MatchState::goal_plane_deflection(impact, 3400, SHOT_V, &b)
            .expect("the post must deflect the ball");
        assert!(v.0 < 0, "vx must be reversed: {v:?}");
        assert_eq!((v.1, v.2), (SHOT_V.1, SHOT_V.2));
    }

    #[test]
    fn a_shot_on_target_is_not_deflected() {
        // ADR-048 §1 (P1, second way): inside the mouth, away from the
        // woodwork, nothing touches the ball; the crossing decides.
        let b = crate::balance::ObstacleBalance::load();
        assert_eq!(
            MatchState::goal_plane_deflection((10_500, 3400, 100), 3400, SHOT_V, &b),
            None
        );
    }

    #[test]
    fn a_shot_on_the_crossbar_comes_back_and_bounces() {
        // The crossbar reverses vx and vz: a z-only reflection would leave
        // the ball travelling into the goal, past the plane, with no outcome.
        let b = crate::balance::ObstacleBalance::load();
        let impact = (10_500, 3400, b.goal_height_cm);
        let v = MatchState::goal_plane_deflection(impact, 3400, SHOT_V, &b)
            .expect("the crossbar must deflect the ball");
        assert!(v.0 < 0, "vx must be reversed: {v:?}");
        assert!(v.2 > 0, "vz must be reversed: {v:?}");
        assert_eq!(v.1, SHOT_V.1);
    }

    #[test]
    fn obstacle_hit_detects_the_post() {
        // Right post's edge: a post, not a goal.
        assert_eq!(
            MatchState::obstacle_hit(3400 + 366, 100, 3400, 366, 244, 6, 6),
            Some(0)
        );
    }

    #[test]
    fn obstacle_hit_does_not_fire_inside_the_goal() {
        assert_eq!(
            MatchState::obstacle_hit(3400, 100, 3400, 366, 244, 6, 6),
            None
        );
    }

    #[test]
    fn obstacle_hit_detects_the_crossbar() {
        assert_eq!(
            MatchState::obstacle_hit(3400, 244, 3400, 366, 244, 6, 6),
            Some(2)
        );
    }

    #[test]
    fn obstacle_hit_does_not_fire_outside_the_post() {
        assert_eq!(
            MatchState::obstacle_hit(3400 + 366 + 100, 100, 3400, 366, 244, 6, 6),
            None
        );
    }
}

#[cfg(test)]
mod control_tests {
    use super::*;
    use ds_core::rng::RngRegistry;
    use ds_orchestrator::GroundTruth;

    /// ADR-041 §5 (P1): a loose ball within reach of a player is his, and
    /// his team has the possession; a loose ball out of reach stays loose.
    #[test]
    fn a_loose_ball_within_reach_belongs_to_the_nearest_player() {
        let m = ds_fixtures::match_fixture(42).expect("fixture");
        let mut hidden = GroundTruth::new(42, m.id);
        let mut rng = RngRegistry::new(42);
        let mut chain = EventChain::new();
        let mut ctx = MatchContext {
            match_: &m,
            hidden: &mut hidden,
            rng: &mut rng,
            chain: &mut chain,
            config: crate::context::MatchConfig::default(),
        };
        let away = m.away.players[5].id;
        let at = (2000, 1500);
        let mut s = MatchState::new(&m);
        s.spatial.set_position(away, at);
        s.possession = Possession::Home;
        // A ball at rest just out of reach of the away player: still loose.
        s.spatial
            .set_ball_in_flight((at.0 + 300, at.1, 0), (0, 0, 0));
        s.spatial.advance_ball();
        assert_eq!(s.spatial.ball_state(), crate::spatial::BallState::Loose);
        s.control_loose_ball(&mut ctx, 0);
        assert_eq!(s.spatial.carrier(), None, "out of reach");
        // At his feet: his.
        s.spatial
            .set_ball_in_flight((at.0 + 50, at.1, 0), (0, 0, 0));
        s.spatial.advance_ball();
        s.control_loose_ball(&mut ctx, 0);
        assert_eq!(s.spatial.carrier(), Some(away));
        assert_eq!(s.possession, Possession::Away);
    }
}
