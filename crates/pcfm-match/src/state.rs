//! The in-progress state of a match.
//!
//! `MatchState` is mutated by the tick loop. At the end of the match, it
//! produces a `MatchResult` via `finish`.

use ds_core::ids::{ActorId, EventId, PlayerId};
use ds_core::time::GameInstant;
use pcfm_model::Squad;
use pcfm_stats::{EventChain, EventType, Half, MatchEvent};
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
    reception_balance: crate::balance::ReceptionBalance,
    pending: Option<Pending>,
    /// Per-actor latency (IDEA-DESIGN-011): the tick at which the actor
    /// can decide again. Absent = 0 = due. The map is cleared on possession
    /// change? No: the actor's pace is his own, persistent across possessions.
    /// A weak decider who wins the ball back carries his own delay with him.
    latencies: BTreeMap<PlayerId, u64>,
}

/// The action an agent chooses from its point of view (MD-01, MD-43).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ChosenAction {
    /// Pass to the teammate with the most space, and how much space he
    /// has (cm, distance to the nearest opponent). The freedom is computed
    /// once in `choose_action` and carried to the reception, not recomputed
    /// (D7, one source of truth).
    Pass { receiver: PlayerId, freedom_cm: i64 },
    /// Shoot at the opponent goal.
    Shoot,
    /// Contest the nearest opponent.
    Duel,
    /// Carry the ball forward: the carrier's gesture when the scene shows
    /// nothing clearer. No event, no ball flight; the tick's movement
    /// already advances the carrier each tick (ADR-040 note 2026-11-09).
    Advance,
}

/// Minimum distance from the nearest opponent for a pass receiver, cm.
const PASS_FREEDOM_CM: i64 = 800;

/// Shoot below this distance from the opponent goal, cm.
const SHOOT_RANGE_CM: i64 = 2500;

/// Regulatory goal width, cm (7.32 m, Laws of the Game, Law 1). A real
/// fact of football, not a model choice.
const GOAL_WIDTH_CM: i64 = 732;

/// Pass distance below this: the pass is not plausible, cm.
const PASS_MIN_CM: i64 = 500;
/// Pass distance: from here the plateau of the plausible short pass, cm.
const PASS_NEAR_CM: i64 = 1000;
/// Pass distance: from here the ramp down of the plausible long pass, cm.
const PASS_FAR_CM: i64 = 3000;
/// Pass distance above this: the pass is not plausible, cm.
const PASS_MAX_CM: i64 = 4500;

/// Duel proximity range: above this distance there is no contest, cm.
/// (Declared, not used yet: the Duel candidate is the prototype fallback,
/// see the note in ADR-040 dated 2026-11-09.)
const DUEL_PROXIMITY_CM: i64 = 500;

/// `distance_plausible(d)`: 0 below `PASS_MIN_CM`, linear up to 1 at
/// `PASS_NEAR_CM`, 1 on the plateau `[PASS_NEAR_CM, PASS_FAR_CM]`, linear
/// down to 0 at `PASS_MAX_CM`, 0 above. Piecewise linear, deterministic,
/// no trig (L0.11 §4.10). The numbers are declared arbitrary; the family
/// (a hump) is ADR-040's design choice (dated note 2026-11-09).
#[must_use]
#[allow(clippy::cast_precision_loss)] // distances in cm: well inside f64's exact range (2^52)
fn distance_plausible(d_cm: i64) -> f64 {
    if d_cm <= PASS_MIN_CM || d_cm >= PASS_MAX_CM {
        return 0.0;
    }
    if d_cm < PASS_NEAR_CM {
        return ((d_cm - PASS_MIN_CM) as f64) / ((PASS_NEAR_CM - PASS_MIN_CM) as f64);
    }
    if d_cm <= PASS_FAR_CM {
        return 1.0;
    }
    ((PASS_MAX_CM - d_cm) as f64) / ((PASS_MAX_CM - PASS_FAR_CM) as f64)
}

/// `line_quality(from, to, opponent_positions)`: 1 - (blocked / samples).
/// Three samples at 25/50/75% of the segment. A sample is "blocked" when
/// some opponent is within `DUEL_PROXIMITY_CM` of it. Declared triangular
/// approximation: a real interceptor's cone is not checked, only the
/// distance. The simplification is stated in ADR-040's dated note.
#[must_use]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)] // sample coords: from + (dx * k) / 4, well inside i32
fn line_quality(from: (i32, i32), to: (i32, i32), opponents: &[(i32, i32)]) -> f64 {
    if opponents.is_empty() {
        return 1.0;
    }
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;
    let mut blocked = 0_u32;
    let samples = 3_u32;
    for k in 1..=samples {
        // k / (samples + 1) for k = 1..=samples -> 25/50/75 of the segment
        let num = i64::from(k);
        let den = i64::from(samples + 1);
        let sx = from.0 + ((i64::from(dx) * num) / den) as i32;
        let sy = from.1 + ((i64::from(dy) * num) / den) as i32;
        let hit = opponents.iter().any(|&op| {
            let dx = i64::from(sx) - i64::from(op.0);
            let dy = i64::from(sy) - i64::from(op.1);
            (dx * dx + dy * dy).isqrt() < DUEL_PROXIMITY_CM
        });
        if hit {
            blocked += 1;
        }
    }
    1.0 - (f64::from(blocked) / f64::from(samples))
}

/// `goal_cone(pos, goal)`: the visible angular width of the goal mouth from
/// `pos`, as `sin(angle)` = `|cross(a, b)| / (|a| * |b|)` with
/// `a = post1 - pos`, `b = post2 - pos`. Polynomial, deterministic, no
/// trig (L0.11 §4.10). Clipped to `[0, 1]`.
#[must_use]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)] // sample coords: from + (dx * k) / 4, well inside i32
fn goal_cone(pos: (i32, i32), goal: (i32, i32)) -> f64 {
    let half = GOAL_WIDTH_CM / 2;
    let a = (
        i64::from(goal.0) - i64::from(pos.0),
        i64::from(goal.1 - half as i32) - i64::from(pos.1),
    );
    let b = (
        i64::from(goal.0) - i64::from(pos.0),
        i64::from(goal.1 + half as i32) - i64::from(pos.1),
    );
    let cross = (a.0 * b.1 - a.1 * b.0).abs();
    let len_a = (a.0 * a.0 + a.1 * a.1).isqrt();
    let len_b = (b.0 * b.0 + b.1 * b.1).isqrt();
    let denom = (len_a * len_b) as f64;
    if denom <= 0.0 {
        return 0.0;
    }
    (cross as f64 / denom).clamp(0.0, 1.0)
}

/// The agent chooses geometrically, from what it sees (MD-43). No die in
/// The heading (degrees, 0 = north = +y, clockwise) that points from
/// `from` toward `to`. Integer, deterministic, no trig in the simulation
/// path (D2). Used to turn the carrier toward the ball before deciding:
/// a player looks at what he is about to play.
#[must_use]
fn heading_from_to(from: (i32, i32), to: (i32, i32)) -> u16 {
    let dx = to.0 - from.0;
    let dy = to.1 - from.1;
    let mut best = 0_u16;
    let mut best_dot = i64::MIN;
    for i in 0..36_u16 {
        let h = i * 10;
        let (hx, hy) = crate::perception::heading_vector(h);
        let dot = i64::from(dx) * i64::from(hx) + i64::from(dy) * i64::from(hy);
        if dot > best_dot {
            best_dot = dot;
            best = h;
        }
    }
    best
}

/// Builds the choice context for `attacker` from the scene, and the
/// freedom of each pass receiver (carried to `resolve_pass`).
///
/// The geometric score and the risk of each option are derived here,
/// geometrically, from positions and distances (ADR-028 §3: derived
/// values are admitted in code). The cognitive latents do not enter
/// here: `resolve_choice` reads them through the custodian. See
/// `pcfm_hidden::choice` for the selection.
///
/// Options, in the order they are built:
/// - one `Pass { receiver }` for each teammate with enough freedom
///   (>= `PASS_FREEDOM_CM`); geometric = `freedom_cm / 1000`, risk
///   grows as `freedom_cm` shrinks towards `PASS_FREEDOM_CM`.
/// - one `Shoot` if the attacker is within `SHOOT_RANGE_CM` of the
///   goal; geometric falls with distance, risk rises with it.
/// - one `Advance` fallback only if no other candidate exists (the
///   actor carries the ball; `resolve_choice` turns `0.0` into
///   `Advance` via the clarity threshold);
///   geometric = 0.0, risk = 0.5 (declared defaults).
///
/// `freedom_cm` is returned alongside, keyed by receiver, so the
/// caller can pass it to `resolve_pass` after the choice.
#[must_use]
#[allow(clippy::cast_precision_loss)] // distances in cm: well inside f64's exact range (2^52)
fn build_choice(
    spatial: &SpatialState,
    attacker: PlayerId,
    goal: (i32, i32),
    teammates: &[PlayerId],
    opponents: &[PlayerId],
) -> (
    pcfm_hidden::ChoiceContext,
    std::collections::BTreeMap<PlayerId, i64>,
) {
    use pcfm_hidden::{ChoiceCandidate, ChoiceContext, ChoiceKind};

    let mut candidates = Vec::new();
    let mut freedoms = std::collections::BTreeMap::new();
    let mut shot_distance: Option<i64> = None;

    let Some(pos) = spatial.agent(attacker).map(|a| a.position) else {
        return (
            ChoiceContext {
                actor: attacker,
                candidates: vec![ChoiceCandidate {
                    kind: ChoiceKind::Advance,
                    geometric: 1.0,
                    risk: 0.5,
                }],
            },
            freedoms,
        );
    };

    let goal_dist = distance_squared(pos, goal).isqrt();
    if goal_dist < SHOOT_RANGE_CM {
        shot_distance = Some(goal_dist);
        // geometric: 1.0 at the goal line, 0.0 at the shoot range edge.
        // ADR-040: geometric = salience, not probability. distance_factor
        // (decreasing linearly with distance) times goal_cone (the visible
        // angular width of the goal mouth from the shooter, no trig).
        let distance_factor = ((SHOOT_RANGE_CM - goal_dist) as f64) / (SHOOT_RANGE_CM as f64);
        let geom = distance_factor * goal_cone(pos, goal);
        // risk: 0.0 at the goal line, 1.0 at the shoot range edge.
        let risk = (goal_dist as f64) / (SHOOT_RANGE_CM as f64);
        candidates.push(ChoiceCandidate {
            kind: ChoiceKind::Shoot,
            geometric: geom,
            risk,
        });
    }
    let _ = shot_distance;

    let opponent_positions: Vec<(i32, i32)> = opponents
        .iter()
        .filter_map(|o| spatial.agent(*o).map(|a| a.position))
        .collect();
    for id in teammates {
        if *id == attacker {
            continue;
        }
        let Some(tpos) = spatial.agent(*id).map(|a| a.position) else {
            continue;
        };
        let freedom = opponents
            .iter()
            .filter_map(|o| spatial.agent(*o).map(|a| a.position))
            .map(|opos| distance_squared(tpos, opos).isqrt())
            .min()
            .unwrap_or(i64::MAX);
        if freedom < PASS_FREEDOM_CM {
            continue;
        }
        // ADR-040: salience, not probability. freedom x line x distance.
        let freedom_norm = (freedom as f64) / (freedom as f64 + PASS_FREEDOM_CM as f64);
        let lq = line_quality(pos, tpos, &opponent_positions);
        let dp = distance_plausible(distance_squared(pos, tpos).isqrt());
        let geom = freedom_norm * lq * dp;
        // risk: 1.0 at PASS_FREEDOM_CM, falling with freedom.
        let risk = (PASS_FREEDOM_CM as f64) / (freedom as f64);
        candidates.push(ChoiceCandidate {
            kind: ChoiceKind::Pass(*id),
            geometric: geom,
            risk,
        });
        freedoms.insert(*id, freedom);
    }

    // Fallback: no candidate today (no teammate with enough freedom,
    // no shot in range). The actor carries the ball; the clarity
    // threshold in `resolve_choice` turns this into `ChoiceKind::Advance`
    // because `0.0` is below `clarity_threshold` (ADR-040 note 2026-11-09,
    // IDEA-DESIGN-010).
    if candidates.is_empty() {
        candidates.push(ChoiceCandidate {
            kind: ChoiceKind::Advance,
            geometric: 0.0,
            risk: 0.5,
        });
    }

    (
        ChoiceContext {
            actor: attacker,
            candidates,
        },
        freedoms,
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
    pub fn new(match_: &pcfm_model::Match) -> Self {
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
            reception_balance: crate::balance::ReceptionBalance::load(),
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
                    let rctx = pcfm_hidden::ReceptionContext {
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
        self.spatial.move_agents(
            i32::from(ctx.match_.pitch.length_m) * 100,
            i32::from(ctx.match_.pitch.width_m) * 100,
            &self.bodies,
            ctx.hidden,
            tick,
            &self.scan_balance,
        );
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
        let Some((attacker, defender)) = self.pick_from_space(ctx) else {
            return;
        };

        // IDEA-DESIGN-011 (R.1-latency): the actor decides when he is due,
        // not when a die says so. The latency is the actor's own pace,
        // resolved from his `decision_making` (ADR-035: no die). If the
        // actor is not due, the tick advances the body and the scene (above)
        // and returns; nothing needs deciding yet.
        let due_at = self.latencies.get(&attacker).copied().unwrap_or(0);
        if tick < due_at {
            return;
        }
        let latency = ctx
            .hidden
            .resolve_latency(&pcfm_hidden::LatencyContext { player: attacker });
        self.latencies
            .insert(attacker, tick.saturating_add(latency.ticks));

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
        if let Some(a) = self.spatial.agent(attacker) {
            let h = heading_from_to(a.position, goal);
            if let Some(am) = self.spatial.agent_mut(attacker) {
                am.heading = h;
            }
        }
        let (choice_ctx, freedoms) =
            build_choice(&self.spatial, attacker, goal, &teammates, &opponents);
        let chosen = match ctx.hidden.resolve_choice(&choice_ctx).chosen {
            pcfm_hidden::ChoiceKind::Pass(receiver) => {
                let freedom_cm = freedoms.get(&receiver).copied().unwrap_or(PASS_FREEDOM_CM);
                ChosenAction::Pass {
                    receiver,
                    freedom_cm,
                }
            }
            pcfm_hidden::ChoiceKind::Shoot => ChosenAction::Shoot,
            pcfm_hidden::ChoiceKind::Duel => ChosenAction::Duel,
            pcfm_hidden::ChoiceKind::Advance => ChosenAction::Advance,
        };

        // The action belongs to the nearest agent; the ball follows.
        self.spatial.set_carrier(Some(attacker));

        if let ChosenAction::Pass {
            receiver,
            freedom_cm,
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
            let ctx_pass = pcfm_hidden::PassContext {
                passer: attacker,
                from,
                to,
                receiver_freedom_cm: freedom_cm,
            };
            let outcome = ctx.hidden.resolve_pass(&ctx_pass, &params);
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
            let interceptor =
                find_interceptor(&self.spatial, &unseen, (from.0, from.1), (to.0, to.1));
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
        } else if chosen == ChosenAction::Shoot {
            let from = match self.spatial.agent(attacker) {
                Some(a) => (a.position.0, a.position.1, 0),
                None => self.spatial.ball_3d(),
            };
            let to = match self.possession {
                Possession::Home => (i32::from(ctx.match_.pitch.length_m) * 100, 3400, 244),
                Possession::Away => (0, 3400, 244),
            };
            // Difficulty: min distance to an opponent, scaled. Arbitrary,
            // declared: 0 cm -> 100 (max pressure), 1500 cm -> 0.
            let difficulty = opponents
                .iter()
                .filter_map(|o| {
                    self.spatial
                        .agent(*o)
                        .map(|a| distance_squared(a.position, (from.0, from.1)).isqrt())
                })
                .min()
                .map_or(0, |d| {
                    i32::try_from(((1500 - d).max(0) * 100) / 1500).unwrap_or(0)
                });
            let ctx_shot = pcfm_hidden::ShotContext {
                shooter: attacker,
                from,
                to,
                difficulty,
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
                shot: Some(ShotPending { attacker }),
            });
        // Reachable from the carrier's scene only when `ChoiceKind::Duel`
        // is emitted; `build_choice` no longer emits it (a carrier has
        // the ball, the opponent does not; `ball_contested` is false by
        // construction). This branch stays for Plan E (the migration of
        // `resolve_duel` and the aerial family, IDEA-DESIGN-008): when a
        // defender is put in the carrier's scene, the branch becomes
        // reachable again. Removed-by: Plan E. Do not delete without
        // the migration: `resolve_duel` would lose its only caller.
        } else if chosen == ChosenAction::Duel {
            let ctx_duel = pcfm_hidden::DuelContext {
                attacker,
                defender,
                duel_type: pcfm_hidden::DuelType::Ground,
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

    /// Emits `RelationEnterExit` when an opponent enters (or exits) the
    /// coverage of the carrier (MD-04). Coverage is "opponent within
    /// `space_threshold_cm` of the ball": a geometric relation, not a
    /// per-tick event. The narrator classifies the enter fact as
    /// `Pressure` (T1); the engine emits the fact, not the event.
    fn update_coverage(&mut self, ctx: &mut MatchContext<'_>, tick: u64) {
        let weights = crate::balance::InfluenceWeightsJson::load().to_weights();
        let threshold_sq =
            i64::from(weights.space_threshold_cm) * i64::from(weights.space_threshold_cm);
        let ball = self.spatial.ball();

        // Compute the current coverage set (immutable phase).
        let (nominal_carrier, current): (PlayerId, BTreeSet<PlayerId>) = {
            let (own, opp) = self.squads(ctx);
            let mut set: BTreeSet<PlayerId> = BTreeSet::new();
            for p in &opp.players {
                if let Some(a) = self.spatial.agent(p.id) {
                    if distance_squared(a.position, ball) <= threshold_sq {
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
        let attacker = match self.pending.as_ref().and_then(|p| p.shot.as_ref()) {
            Some(s) => s.attacker,
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
        let goal_half_w = 366;
        let goal_h = 244;
        let goal_center_y = width / 2;
        let inside = (y_at - goal_center_y).abs() < goal_half_w && (0..goal_h).contains(&z_at);

        self.pending = None;
        self.spatial.set_carrier(None);

        if !inside {
            self.possession = defending;
            return;
        }

        let gk = match defending {
            Possession::Home => PlayerId::new(0, 0),
            Possession::Away => PlayerId::new(100, 0),
        };
        let reach_cm = self.bodies.get(&gk).map_or(0.0, |b| b.height_cm / 2.0);
        let saved = if let Some(gk_agent) = self.spatial.agent(gk) {
            let dy = f64::from((y_at - gk_agent.position.1).abs());
            if dy <= reach_cm {
                let save_ctx = pcfm_hidden::SaveContext {
                    goalkeeper: gk,
                    point: (plane_x, y_at, z_at),
                    offset_cm: dy,
                    reach_cm,
                };
                ctx.hidden.resolve_save(&save_ctx).saved
            } else {
                false
            }
        } else {
            false
        };

        if saved {
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(gk)),
                EventType::Save,
                serde_json::json!({ "x": plane_x, "y": y_at, "z": z_at }),
            );
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
    use pcfm_model::{
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
        let (ctx, _freedoms) = build_choice(&s, attacker, (10500, 3400), &teammates, &opponents);
        let pass_target_seen = ctx
            .candidates
            .iter()
            .any(|c| matches!(c.kind, pcfm_hidden::ChoiceKind::Pass(p) if p == mate));
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
}
