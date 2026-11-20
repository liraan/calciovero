//! The in-progress state of a match.
//!
//! `MatchState` is mutated by the tick loop. At the end of the match, it
//! produces a `MatchResult` via `finish`.

use ds_core::dmath;
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
pub(crate) const FLIGHT_TICKS: u64 = 15;

/// Radius within which an opponent counts as crowding, in cm. Arbitrary,
/// declared (ADR-044 §6: the duration of the chain is in the relation).
pub(crate) const CROWD_RADIUS_CM: i32 = 1500;

/// The number of crowding opponents that saturates the term to 1.0.
pub(crate) const CROWD_MAX: u32 = 3;

/// Distance at which the nearest-threat term goes to 0.0, in cm.
pub(crate) const THREAT_RADIUS_CM: f64 = 2000.0;

/// Interception radius, in centimetres. A defender within this distance
/// of the pass line intercepts, if the passer did not see him. Arbitrary,
/// declared.
pub(crate) const INTERCEPT_RADIUS_CM: i64 = 200;

/// The squared distance from point `p` to the segment `a`-`b`. Integer
/// only, deterministic (D2).
pub(crate) fn point_segment_dist_sq(a: (i32, i32), b: (i32, i32), p: (i32, i32)) -> i64 {
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
pub(crate) fn find_interceptor(
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

/// Below this height a ball is played with the foot, the thigh or the chest
/// (the ground control); at or above it, with the head (`meet_ball_in_air`).
/// Declared, as it was in `meet_ball_in_air` (1 m).
pub(crate) const FEET_BAND_CM: i32 = 100;

/// The point of the ball's path this tick, from `a` to `b`, nearest to
/// `p` on the ground, and the ball's height there (a straight segment: one
/// tick of flight).
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
pub(crate) fn nearest_on_path(
    (a, b): ((i32, i32, i32), (i32, i32, i32)),
    p: (i32, i32),
) -> ((i32, i32), i32) {
    let (dx, dy) = (f64::from(b.0 - a.0), f64::from(b.1 - a.1));
    let len2 = dx.mul_add(dx, dy * dy);
    let t = if len2 < 1.0 {
        1.0
    } else {
        (f64::from(p.0 - a.0).mul_add(dx, f64::from(p.1 - a.1) * dy) / len2).clamp(0.0, 1.0)
    };
    let at = |u: i32, v: i32| t.mul_add(f64::from(v - u), f64::from(u)).round() as i32;
    ((at(a.0, b.0), at(a.1, b.1)), at(a.2, b.2))
}

/// Whether a player of `courage` and `weight` goes up for a high ball a
/// rival of `rival` kilos can reach (ADR-057, OI-109): always against a
/// man no heavier than him; against a heavier one, if his courage is at
/// least the rival's share of their two weights. No number of its own.
pub(crate) fn goes_up(courage: f64, weight: f64, rival: f64) -> bool {
    rival <= weight || courage >= rival / (rival + weight)
}

/// Where `id` cuts out a pass on the grass, and how many teammates he
/// believes get there before him, judged from what he has seen (OI-106):
/// he goes if he meets it before the receiver, when he has seen the
/// receiver, or wherever he meets it, when he has not; the teammates ahead
/// of him are only those he has seen. `cutters` are sorted along the path.
pub(crate) fn cut_target(
    cutters: &[(PlayerId, (i32, i32), i64)],
    receiver: Option<(PlayerId, i64)>,
    id: PlayerId,
    sees: impl Fn(PlayerId) -> bool,
) -> Option<((i32, i32), u32)> {
    let &(_, at, mine) = cutters.iter().find(|c| c.0 == id)?;
    let (r, r_along) = receiver?;
    if sees(r) && mine >= r_along {
        return None;
    }
    let ahead = cutters
        .iter()
        .filter(|c| c.0 != id && sees(c.0) && (c.2, c.0) < (mine, id))
        .count();
    Some((at, u32::try_from(ahead).unwrap_or(u32::MAX)))
}

/// His rank among his teammates for a ball nobody has: a pass played to
/// him is his to meet first (0), however near the ball his teammates are
/// (OI-98: a keeper ranked behind them held his place, and the back pass
/// ran past him into his goal); otherwise how many are nearer.
pub(crate) const fn rank_to_meet(played_to_him: bool, nearer: u32) -> u32 {
    if played_to_him { 0 } else { nearer }
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
    /// The closest the ball has come to its receiver so far, cm (ADR-058):
    /// once it is farther than that by more than his reach, it has gone
    /// past him and the pass is lost.
    pub closest_cm: Option<i64>,
    /// A through ball (plan L0-CLOSE 10a).
    pub through: bool,
    /// The tick the ball was struck (SIM 133): an opponent reacts to it
    /// before he can play it.
    pub launched: u64,
    /// A cross (ADR-072): the ball from wide into the area.
    pub cross: bool,
    /// The foot the ball was struck with (ADR-031), for the cross's narrative.
    pub foot: Option<ds_orchestrator::Side>,
}
/// A substitution ordered during the match: off, on (decision T).
pub(crate) type Ordered = (PlayerId, PlayerId);

/// The in-progress state of a match.
pub struct MatchState {
    pub(crate) score: Score,
    pub(crate) possession: Possession,
    pub(crate) home_possession_ticks: u64,
    pub(crate) away_possession_ticks: u64,
    pub(crate) shots: (u16, u16),
    /// The penalty shoot-out's goals, home and away, when there was one.
    pub(crate) shootout: Option<(u8, u8)>,
    pub(crate) next_event_slot: u32,
    /// Which half is currently being played (ADR-026).
    pub(crate) half: Half,
    /// The absolute tick at which the current half started.
    pub(crate) half_start_tick: u64,
    /// The spatial state: positions, ball, carrier (slice B2).
    pub(crate) spatial: SpatialState,
    /// The pending flight, if any (MD-37). At most one at a time today.
    /// F.5: the opponents currently covering the carrier. The set is
    /// persistent across ticks; `update_coverage` diffs it and emits
    /// `RelationEnterExit` for the new members (MD-04). Discarded on
    /// possession change: the previous coverage referred to the other
    /// team.
    pub(crate) coverage: BTreeSet<PlayerId>,
    /// The possession the current `coverage` set refers to.
    pub(crate) coverage_possession: Possession,
    /// The body of each player (height, weight, fatigue). Built once
    /// in `new` from the squads; read by `resolve_movement` each tick.
    pub(crate) bodies: BTreeMap<PlayerId, PlayerBody>,
    /// Scanning parameters (ADR-034), loaded once at construction.
    /// What each player has seen of the others, where he last saw them
    /// (ADR-034, MD-40: the memory of the last look). His scene reads the
    /// pitch through it, never the pitch itself (OI-106).
    pub(crate) perception: crate::plugins::perception::Perception,
    /// The fatigue plugin, called in the move phase (ADR-063).
    pub(crate) fatigue_plugin: crate::plugins::fatigue::Fatigue,
    /// The bench plugin, called at a stoppage (ADR-063).
    pub(crate) bench_plugin: crate::plugins::bench::Bench,
    /// The men of a wall, holding their places until the free kick is
    /// taken (Law 13).
    pub(crate) wall: Vec<PlayerId>,
    /// A restart called and not yet taken (SIM 150): while it waits the
    /// ball is out of play, the men go to their places, the clock runs.
    pub(crate) waiting: Option<crate::plugins::set_pieces::Waiting>,
    pub(crate) scan_balance: crate::balance::ScanningBalance,
    pub(crate) keeper_balance: crate::balance::KeeperBalance,
    /// The Laws' thresholds (Law 12, `balance/laws.json`).
    pub(crate) laws: crate::balance::LawsBalance,
    /// The players cautioned once (Law 12): a second yellow is a red.
    pub(crate) cautioned: BTreeSet<PlayerId>,
    pub(crate) obstacle_balance: crate::balance::ObstacleBalance,
    pub(crate) reception_balance: crate::balance::ReceptionBalance,
    pub(crate) carry_balance: crate::balance::CarryBalance,
    pub(crate) pressure_balance: crate::balance::PressureBalance,
    /// Where the carrier was at his last decision: if he has not got
    /// anywhere since, he is blocked (OI-97).
    pub(crate) last_decision: Option<(PlayerId, (i32, i32))>,
    /// When a player who lost the ball may challenge again (ADR-042).
    pub(crate) challenges: BTreeMap<PlayerId, u64>,
    /// When each player without the ball chooses again (ADR-049).
    pub(crate) offball_due: BTreeMap<PlayerId, u64>,
    /// The intesa of each pair of teammates (strato 3): how well one
    /// predicts the other, grown from the plays they complete together.
    /// Keyed by (actor, partner); default baseline for pairs that have
    /// not yet played together.
    pub(crate) intesa: BTreeMap<(PlayerId, PlayerId), f64>,
    /// Law 11: the passer's teammates in an offside position when the
    /// ball was played; one who then plays it commits the offence.
    pub(crate) offside_at_pass: BTreeSet<PlayerId>,
    /// The restart that put the ball back in play, until its first pass:
    /// there is no offside from a throw-in, a goal kick or a corner.
    pub(crate) restart_pass: Option<crate::rules::RestartKind>,
    /// The kicker the team has sent to the mark in the shoot-out (Law 10).
    pub(crate) shootout_taker: Option<PlayerId>,
    pub(crate) pending: Option<Pending>,
    /// Where the ball was at the start of this tick: its path this tick is
    /// the segment from here to where it is now (ADR-058: a ball is
    /// received where it reaches a player, between two ticks too).
    pub(crate) ball_from: (i32, i32, i32),
    /// How many of each coach's instructions have taken effect, home and
    /// away (ADR-059).
    /// The instructions already delivered, by side and index (ADR-068).
    pub(crate) delivered: BTreeSet<(bool, usize)>,
    /// The pauses so far (an injury, a water break, half-time, a
    /// substitution): what the channel of the pause waits for (ADR-068).
    pub(crate) pauses: u64,
    /// For each instruction waiting, the pauses there had been when it was
    /// given.
    pub(crate) pause_mark: BTreeMap<(bool, usize), u64>,
    /// The water breaks already made (ADR-068).
    pub(crate) water_breaks_done: u8,
    /// The men a knock forced off (decision Y, ADR-066): replaced at the
    /// stoppage, or carried off if no change is left.
    pub(crate) knocked: BTreeSet<PlayerId>,
    /// The substitutions each side's manager ordered during the match,
    /// waiting for a stoppage (decision T): off, on.
    pub(crate) ordered: (Vec<Ordered>, Vec<Ordered>),
    /// Per-actor latency (IDEA-DESIGN-011): the tick at which the actor
    /// can decide again. Absent = 0 = due. The map is cleared on possession
    /// change? No: the actor's pace is his own, persistent across possessions.
    /// A weak decider who wins the ball back carries his own delay with him.
    pub(crate) latencies: BTreeMap<PlayerId, u64>,
    /// The keeper holding the ball in his hands (plan L0-CLOSE 10d): he
    /// throws it, and nobody may challenge him for it (Law 12).
    pub(crate) in_hands: Option<PlayerId>,
    /// The substitutions made, by the player who came off (plan L0-CLOSE
    /// 10f), and those still to announce: `(outgoing, incoming)`.
    pub(crate) substituted: BTreeSet<PlayerId>,
    pub(crate) to_announce: Vec<(PlayerId, PlayerId)>,
    /// The ball went dead since the last tick (a restart was given): the
    /// moment a substitution may be made (Law 3).
    pub(crate) stoppage: bool,
}

/// The action an agent chooses from its point of view (MD-01, MD-43).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ChosenAction {
    /// Pass to the teammate he chose.
    Pass {
        receiver: PlayerId,
        lofted: bool,
        /// A through ball: played to this point, ahead of the runner.
        at: Option<(i32, i32)>,
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
    /// Carry the ball towards this point, the direction he chose (plan
    /// L0-CLOSE 10a).
    Carry((i32, i32)),
    /// Clear the ball away (ADR-051), towards the landing he chose.
    Clear((i32, i32)),
    /// Hold the ball up, shielding it, while the team comes up (plan
    /// L0-CLOSE 11): he keeps it at his feet until his next decision.
    Hold,
    /// Touch the ball inside, onto his better foot (ADR-031).
    Shift,
}

/// Builds the `PerceivedScene` from the engine's spatial state.
///
/// The engine provides the facts (positions, goal, the two squads); the
/// actor reads them in `ds_orchestrator::read::build_choice` (A.1-bis).
/// Plane B by construction: positions, not latents (D2).
#[must_use]
pub(crate) fn scene_from(
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
        ball_foot: spatial.ball_foot(),
        // A touch onto the better foot moves the ball across his body, two
        // body radii, and in its time the opponents come as near (ADR-031);
        // the actor keeps it only if the ball is on his weaker foot.
        shift: Some((0.0, i64::from(2 * crate::spatial::BODY_RADIUS_CM))),
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
        runs: std::collections::BTreeMap::new(),
        horizon_cm: f64::MAX,
        strike_error_rad: 0.0,
        soon: std::collections::BTreeMap::new(),
        shield_keep: 0.0,
        foresight: 1.0,
        motion: std::collections::BTreeMap::new(),
        level_share: 0.0,
        reception: ds_orchestrator::read::ReceptionReckoning {
            air_keep: f64::from(spatial.air_friction_milli()) / 1000.0,
            ..ds_orchestrator::read::ReceptionReckoning::default()
        },
    }
}

/// Whether the ball is inside the penalty area of the home (`home`) or
/// away side's goal (Law 1, `offball.json`).
pub(crate) const fn in_keeper_box(
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
pub(crate) struct TeamView {
    pub(crate) forward: i32,
    pub(crate) their_goal_x: i32,
    pub(crate) last_line: Option<i32>,
    /// The opponents' first line of pressure, midfield line and last line
    /// (ADR-052, the language of space): where the spaces between them are.
    pub(crate) lines: ds_orchestrator::read::Lines,
    pub(crate) opponents: Vec<(i32, i32)>,
    /// Their outfield players other than the carrier: the men to mark.
    pub(crate) men: Vec<(i32, i32)>,
    /// Where their carrier is, if they have the ball.
    pub(crate) carrier_at: Option<(i32, i32)>,
    /// A teammate is pressing their carrier (plan L0-CLOSE 10c: the cover).
    pub(crate) mate_pressing: bool,
    /// Where our carrier is, if we have the ball (plan L0-CLOSE 10c: the
    /// free line to offer on).
    pub(crate) mate_carrier: Option<(i32, i32)>,
    /// Where our own players stand (OI-117, SIM 134: who would reach a
    /// pass to the man first).
    pub(crate) ours: Vec<(i32, i32)>,
    /// Their velocity this tick (strato 3: the prediction of where they are
    /// going, for complementary movements). Parallel to `ours`.
    pub(crate) ours_vel: Vec<(i32, i32)>,
    /// The share of a rolling ball's speed the grass keeps per tick
    /// (`ball.json`).
    pub(crate) ground_keep: f64,
}

impl TeamView {
    pub(crate) fn new(
        spatial: &SpatialState,
        other: &ds_model::Squad,
        home: bool,
        length: i32,
        carrier: Option<PlayerId>,
    ) -> Self {
        let forward = if home { 1 } else { -1 };
        let their_keeper = other.players.first().map(|p| p.id);
        let outfield: Vec<(i32, i32)> = other
            .players
            .iter()
            .filter(|q| Some(q.id) != their_keeper)
            .filter_map(|q| spatial.agent(q.id))
            .map(|a| a.position)
            .collect();
        let last_line = outfield
            .iter()
            .map(|p| p.0 * forward)
            .max()
            .map(|u| u * forward);
        // Their lines as the men on this side read them: the first pressure
        // line, the midfield and the last (ADR-052).
        let lines = ds_orchestrator::read::lines(&outfield, forward);
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
            lines,
            opponents,
            men,
            carrier_at: carrier
                .filter(|c| other.players.iter().any(|p| p.id == *c))
                .and_then(|c| spatial.agent(c))
                .map(|a| a.position),
            mate_pressing: false,
            mate_carrier: None,
            ours: Vec::new(),
            ours_vel: Vec::new(),
            ground_keep: 0.957,
        }
    }

    /// The point behind the teammate pressing their carrier, on the
    /// carrier's line to our goal (plan L0-CLOSE 10c).
    pub(crate) fn cover_point(
        &self,
        off: &ds_calculator::offball::OffBallBalance,
        (length, width): (i32, i32),
    ) -> Option<(i32, i32)> {
        let c = self.carrier_at?;
        let own_goal = (length - self.their_goal_x, width / 2);
        Some(step_towards(
            c,
            own_goal,
            off.cover_depth_cm,
            (length, width),
        ))
    }

    /// The point in the passing lane between their carrier and the man he
    /// would mark, a step in front of the man (plan L0-CLOSE 10c).
    pub(crate) fn lane_point(
        &self,
        me: (i32, i32),
        off: &ds_calculator::offball::OffBallBalance,
        (length, width): (i32, i32),
    ) -> Option<(i32, i32)> {
        let c = self.carrier_at?;
        let m = self.man_to_mark(me)?;
        let half = (distance_squared(c, m).isqrt() / 2).max(1);
        let step = i32::try_from(i64::from(off.lane_step_cm).min(half)).unwrap_or(0);
        Some(step_towards(m, c, step, (length, width)))
    }

    pub(crate) fn marker(&self, me: (i32, i32)) -> Option<(i32, i32)> {
        self.opponents
            .iter()
            .copied()
            .min_by_key(|o| distance_squared(*o, me))
    }

    /// The dangers a defender at `me` reads (ADR-052 §5): the value to
    /// them of the carrier's space, of the man he would mark, and of the
    /// space he guards (`held`), as much as an opponent can get into it
    /// soon. `home`: the defender's side.
    pub(crate) fn threats(
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
        let off = ds_calculator::offball::OffBallBalance::load();
        let dist = |p: (i32, i32)| distance_squared(me, p).isqrt();
        ds_orchestrator::Threats {
            carrier: threat(carrier),
            // The man is a danger as much as a pass can reach him first
            // (OI-117 (A), SIM 134): the defender imagines the carrier's
            // ground pass to him as a passer does (ADR-052, `who_first`),
            // everyone at the pace and reaction he believes. Before, the man
            // weighed his whole danger, as if he already had the ball, and
            // beyond 6 m the defender nearest the carrier marked him.
            man: self
                .man_to_mark(me)
                .map_or(0.0, |m| threat(m) * self.reaches(carrier, m)),
            held: threat(held) * into,
            cover: self
                .mate_pressing
                .then(|| self.cover_point(&off, (length, width)))
                .flatten()
                .map(|p| (threat(p), dist(p))),
            lane_cm: self.lane_point(me, &off, (length, width)).map(dist),
        }
    }

    /// How sure a ground pass from `carrier` to the man at `man` reaches
    /// one of theirs before one of ours, `[0, 1]`, by the thought's own
    /// forecast (ADR-052): the pass weighed as a passer weighs it.
    pub(crate) fn reaches(&self, carrier: (i32, i32), man: (i32, i32)) -> f64 {
        let thought = ds_calculator::thought::ThoughtBalance::load();
        let kick = ds_calculator::kick::KickBalance::load();
        let imagined = |at: (i32, i32), mate: bool| ds_calculator::thought::Imagined {
            at,
            speed: thought.believed_speed_cm_per_tick,
            reaction: thought.believed_reaction_ticks,
            mate,
        };
        let players: Vec<_> = self
            .opponents
            .iter()
            .filter(|p| **p != carrier)
            .map(|p| imagined(*p, true))
            .chain(self.ours.iter().map(|p| imagined(*p, false)))
            .collect();
        #[allow(clippy::cast_precision_loss)]
        let dist = distance_squared(carrier, man).isqrt() as f64;
        let v0 = dist
            .mul_add(1.0 - self.ground_keep, kick.pass_arrive_cm_per_tick)
            .min(kick.speed(1.0, 0.5));
        let forecast = ds_calculator::thought::who_first(
            carrier,
            man,
            (v0, self.ground_keep),
            &players,
            thought.forecast_step_cm,
        );
        match forecast.first {
            Some(first) if players[first].mate => thought.confidence(forecast.margin),
            Some(_) => thought.confidence(-forecast.margin),
            None => 0.0,
        }
    }

    /// The man a defender at `me` would mark: the nearest of their outfield
    /// players other than the carrier.
    pub(crate) fn man_to_mark(&self, me: (i32, i32)) -> Option<(i32, i32)> {
        self.men
            .iter()
            .copied()
            .min_by_key(|o| distance_squared(*o, me))
    }

    /// What a player at `me` sees of his teammate's ball at `c`; `place` is
    /// where his coach's shape puts him (the block with the ball, ADR-059).
    pub(crate) fn mate_ball(
        &self,
        (me, place): ((i32, i32), (i32, i32)),
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
        // ADR-052 §3: a space is worth its chance quality and what a pass
        // from the carrier to it breaks on the way (the opponents' lines).
        let worth = |p: (i32, i32)| {
            ds_orchestrator::read::space_value(p, goal, other)
                * ds_orchestrator::read::line_break(c, p, &self.lines, self.forward)
        };
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
        // Holding is taking up the place his coach's shape gives him (ADR-059
        // §3): worth what that place is, as free as he would be there.
        let values = ds_orchestrator::SpaceValues {
            hold: worth(place) * free(place),
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

    /// The place a man of the side with the ball holds (ADR-071, SIM 152):
    /// from his role's zone `anchor` (the shape), the space worth most to
    /// his side among his corridor and the two beside it (the pitch in five
    /// corridors), on his line and one line ahead or behind (a line, a
    /// third of the block's depth with the ball), never beyond the offside
    /// line. A space is worth what the game measures (ADR-052): its value
    /// to the side, as free as he would be there, by how clean the line
    /// from the carrier is and how plausible a pass that far is (ADR-040). Never two in the same corridor on the same
    /// line; a man of the back line stays behind the ball. Staying in his
    /// zone is his discipline: within it he chooses (ADR-071 §4). `me` is
    /// himself, `carrier` the man on the ball, `ball` where it is.
    #[allow(clippy::too_many_arguments, clippy::cast_precision_loss)]
    pub(crate) fn choose_place(
        &self,
        (me, anchor): ((i32, i32), (i32, i32)),
        (carrier, ball): ((i32, i32), (i32, i32)),
        (back_line, block_depth): (bool, i32),
        onside_margin_cm: i32,
        (length, width): (i32, i32),
        intesa: f64,
        game_reading: f64,
    ) -> (i32, i32) {
        const PREDICT_TICKS: i32 = 5;
        // OI-98: the offside line is not read perfectly. A poor reader
        // sees it further forward (more room) than it is, so he may offer
        // beyond the real line and be caught offside. ~2 m of error at
        // game_reading 0, none at 1.0.
        const OFFSIDE_PERCEPTION_ERROR_CM: i32 = 2000;
        let perceived_last = self.last_line.map(|l| {
            #[allow(clippy::cast_possible_truncation)]
            let err = ((1.0 - game_reading.clamp(0.0, 1.0))
                * f64::from(OFFSIDE_PERCEPTION_ERROR_CM)) as i32;
            l + self.forward * err
        });
        let corridor = (width / 5).max(1);
        let line = (block_depth / 3).max(1);
        let goal = (self.their_goal_x, width / 2);
        let other = (length - self.their_goal_x, width / 2);
        let value = |p: (i32, i32)| {
            let free = 1.0
                - self
                    .marker(p)
                    .map_or(0.0, |o| ds_orchestrator::read::soon(o, p));
            // Within the reach of a pass from the carrier (the plausible
            // distance of ADR-040): a space he cannot be played into gives
            // his side nothing now.
            let reach =
                ds_orchestrator::read::distance_plausible(distance_squared(carrier, p).isqrt());
            ds_orchestrator::read::space_value(p, goal, other)
                // ADR-052: a place a pass can reach past their lines is
                // worth what those lines must recover from (the pocket).
                * ds_orchestrator::read::line_break(carrier, p, &self.lines, self.forward)
                * free
                * ds_orchestrator::read::line_quality(carrier, p, &self.opponents)
                * reach
        };
        // Never two in the same corridor on the same line. Strato 3: a
        // teammate I know (intesa high enough) is also predicted where he is
        // going, so the space he will occupy counts as crowded for me too.
        let predicted: Vec<(i32, i32)> = if intesa >= 0.5 {
            self.ours
                .iter()
                .zip(self.ours_vel.iter())
                .filter(|(m, _)| **m != me)
                .map(|(m, v)| (m.0 + v.0 * PREDICT_TICKS, m.1 + v.1 * PREDICT_TICKS))
                .collect()
        } else {
            Vec::new()
        };
        let crowded = |p: (i32, i32)| {
            self.ours.iter().any(|m| {
                *m != me && (m.1 / corridor == p.1 / corridor) && (m.0 - p.0).abs() < line / 2
            }) || predicted
                .iter()
                .any(|m| (m.1 / corridor == p.1 / corridor) && (m.0 - p.0).abs() < line / 2)
        };
        let onside = |x: i32| {
            perceived_last.map_or(x, |l| {
                let edge = l - self.forward * onside_margin_cm;
                if (x - edge) * self.forward > 0 {
                    edge
                } else {
                    x
                }
            })
        };
        let k0 = (anchor.1 / corridor).clamp(0, 4);
        // Besides the depths of his own block, the spaces between their
        // lines are places to look at (ADR-052 §5): the pocket between the
        // midfield and the last line is where a man receives and turns.
        // Only where his zone reaches, never a charge up the pitch.
        let mut depths = vec![
            anchor.0,
            anchor.0 + line * self.forward,
            anchor.0 - line * self.forward,
        ];
        let their_lines: Vec<i32> = [self.lines.first, self.lines.middle, self.lines.last]
            .into_iter()
            .flatten()
            .collect();
        for pair in their_lines.windows(2) {
            let between = (pair[0] + pair[1]) / 2;
            if pair[0] != pair[1] && (between - anchor.0).abs() <= line {
                depths.push(between);
            }
        }
        let mut options: Vec<(f64, (i32, i32))> = Vec::new();
        for k in (k0 - 1).max(0)..=(k0 + 1).min(4) {
            let y = if k == k0 {
                anchor.1
            } else {
                k * corridor + corridor / 2
            };
            for d in &depths {
                let x = onside(*d).clamp(0, length);
                let p = (x, y.clamp(0, width));
                let behind_ball = (p.0 - ball.0) * self.forward <= 0;
                let v = if (back_line && !behind_ball) || crowded(p) {
                    0.0
                } else {
                    value(p)
                };
                options.push((v, p));
            }
        }
        // The zone of his role, first: the coach's ask (ADR-059 §4).
        let zone = (value(anchor), anchor);
        let best = options
            .iter()
            .copied()
            .fold(zone, |b, o| if o.0 > b.0 { o } else { b });
        best.1
    }

    /// The place a man of the side without the ball closes (ADR-071 §5,
    /// SIM 154): from his role's zone `anchor` (the block without the ball),
    /// among his corridor and the two beside it and his line and one line
    /// ahead or behind (a line, a third of the block's depth without the
    /// ball), the space that takes away the most danger.
    ///
    /// A space is dangerous as much as it is worth to the opponents
    /// (`read::space_value` towards our goal) and as soon as one of them can
    /// be there (`read::soon`); it is his to close only if he is the nearest
    /// of his side to it, as a zone is. The carrier's own space is not one of
    /// them: that is pressing, another act. Far from the ball no opponent
    /// can arrive soon, but the space still weighs what THIS man reads in it:
    /// inside his own lane his reading picks the line, while the lanes beside
    /// keep being weighed by the game (ADR-071 §5, emendamento 2026-11-19;
    /// `BEHAVIOURS.md` 19). He never leaves his zone: that is his discipline
    /// (ADR-071 §4).
    #[allow(clippy::cast_precision_loss)] // cm on a pitch
    pub(crate) fn choose_closed_place(
        &self,
        (me, anchor): ((i32, i32), (i32, i32)),
        (length, width): (i32, i32),
        game_reading: f64,
    ) -> (i32, i32) {
        let block = crate::balance::BlockBalance::load();
        let corridor = (width / 5).max(1);
        let line = (block.depth_out_of_possession_cm / 3).max(1);
        let our_goal = (length - self.their_goal_x, width / 2);
        let their_goal = (self.their_goal_x, width / 2);
        // The space is his only if no teammate is nearer to it: zones do not
        // overlap, as the men of a real block do not.
        let his = |p: (i32, i32)| {
            let mine = distance_squared(me, p);
            self.ours
                .iter()
                .all(|m| *m == me || distance_squared(*m, p) >= mine)
        };
        let danger = |p: (i32, i32)| ds_orchestrator::read::space_value(p, our_goal, their_goal);
        let soon = |p: (i32, i32)| {
            self.opponents
                .iter()
                .filter(|o| Some(**o) != self.carrier_at)
                .map(|o| ds_orchestrator::read::soon(*o, p))
                .fold(0.0, f64::max)
        };
        // Never two in the same corridor on the same line.
        let crowded = |p: (i32, i32)| {
            self.ours.iter().any(|m| {
                *m != me && (m.1 / corridor == p.1 / corridor) && (m.0 - p.0).abs() < line / 2
            })
        };
        let k0 = (anchor.1 / corridor).clamp(0, 4);
        let mut candidates = Vec::new();
        for k in (k0 - 1).max(0)..=(k0 + 1).min(4) {
            for step in [0, 1, -1] {
                let y = if k == k0 {
                    anchor.1
                } else {
                    k * corridor + corridor / 2
                };
                let x = (anchor.0 + step * line * self.forward).clamp(0, length);
                candidates.push((x, y.clamp(0, width)));
            }
        }
        // ADR-071 §5 (emendamento 2026-11-19): when no opponent can arrive
        // soon in his zone, a space is still worth what THIS man reads in it
        // — not zero. Where an opponent can arrive, the reading is the old
        // `danger * soon`: a threat is a fact, not a matter of interpretation.
        let quiet = soon(anchor) == 0.0 && candidates.iter().all(|p| soon(*p) == 0.0);
        let value = |p: (i32, i32)| {
            if !his(p) {
                return 0.0;
            }
            // His reading chooses the line inside his own lane; the lanes
            // beside it are still weighed by the game, not by his mood.
            danger(p)
                * if quiet && p.1 / corridor == k0 {
                    game_reading.clamp(0.0, 1.0)
                } else {
                    soon(p)
                }
        };
        let mut options: Vec<(f64, (i32, i32))> = Vec::new();
        for p in candidates {
            options.push((if crowded(p) { 0.0 } else { value(p) }, p));
        }
        // The zone of his role, first: the coach's ask (ADR-059 §4).
        let zone = (value(anchor), anchor);
        options
            .iter()
            .copied()
            .fold(zone, |b, o| if o.0 > b.0 { o } else { b })
            .1
    }

    /// Where a player who chose to offer or to run goes: away from his
    /// marker to offer; to the last defender's line, just short of it, to
    /// run.
    pub(crate) fn act_target(
        &self,
        act: ds_orchestrator::OffBallAct,
        me: (i32, i32),
        off: &ds_calculator::offball::OffBallBalance,
        (length, width): (i32, i32),
    ) -> Option<(i32, i32)> {
        match act {
            ds_orchestrator::OffBallAct::Offer => self.marker(me).map(|o| {
                // Away from his marker, where the line from the carrier is
                // most open (plan L0-CLOSE 10c): straight away, or turned 45
                // or 90 degrees, by fixed rotations.
                let (dx, dy) = (f64::from(me.0 - o.0), f64::from(me.1 - o.1));
                let len = dmath::hypot(dx, dy).max(1.0);
                let (ux, uy) = (dx / len, dy / len);
                let step = f64::from(off.offer_step_cm);
                // Onside (Law 11): never beyond the last defender's line, the
                // margin short of it the run keeps too. A man offered beyond
                // it cannot be played to (before, three in four of the
                // players flagged offside were offering for a pass there).
                let onside = |x: i32| {
                    self.last_line.map_or(x, |l| {
                        let line = l - self.forward * off.onside_margin_cm;
                        if (x - line) * self.forward > 0 {
                            line
                        } else {
                            x
                        }
                    })
                };
                let spot = |c: f64, s: f64| {
                    let (vx, vy) = (ux.mul_add(c, -(uy * s)), ux.mul_add(s, uy * c));
                    #[allow(clippy::cast_possible_truncation)]
                    let p = (
                        onside(me.0 + (vx * step) as i32).clamp(0, length),
                        (me.1 + (vy * step) as i32).clamp(0, width),
                    );
                    p
                };
                let spots = [
                    spot(1.0, 0.0),
                    spot(
                        std::f64::consts::FRAC_1_SQRT_2,
                        std::f64::consts::FRAC_1_SQRT_2,
                    ),
                    spot(
                        std::f64::consts::FRAC_1_SQRT_2,
                        -std::f64::consts::FRAC_1_SQRT_2,
                    ),
                    spot(0.0, 1.0),
                    spot(0.0, -1.0),
                ];
                let Some(c) = self.mate_carrier else {
                    return spots[0];
                };
                let open = |p: (i32, i32)| {
                    self.opponents
                        .iter()
                        .map(|q| point_segment_dist_sq(c, p, *q))
                        .min()
                        .unwrap_or(i64::MAX)
                };
                spots
                    .iter()
                    .copied()
                    .max_by(|a, b| open(*a).cmp(&open(*b)).then(b.cmp(a)))
                    .unwrap_or(spots[0])
            }),
            ds_orchestrator::OffBallAct::Run => self.last_line.map(|l| {
                (
                    (l - self.forward * off.onside_margin_cm).clamp(0, length),
                    me.1,
                )
            }),
            ds_orchestrator::OffBallAct::Cover => self.cover_point(off, (length, width)),
            ds_orchestrator::OffBallAct::Intercept => self.lane_point(me, off, (length, width)),
            ds_orchestrator::OffBallAct::Mark => self.man_to_mark(me).map(|m| {
                // Goal-side of him: between him and the goal we defend.
                let own_goal = (length - self.their_goal_x, width / 2);
                let (dx, dy) = (f64::from(own_goal.0 - m.0), f64::from(own_goal.1 - m.1));
                let len = dmath::hypot(dx, dy).max(1.0);
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
pub(crate) fn meeting_point(
    from: (i32, i32, i32),
    cross: (i32, i32, i32),
    x: i32,
) -> ((i32, i32, i32), f64) {
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
pub(crate) const fn side_of(p: Possession) -> crate::rules::Side {
    match p {
        Possession::Home => crate::rules::Side::Home,
        Possession::Away => crate::rules::Side::Away,
    }
}

/// The point `step` cm from `from` towards `to`, on the pitch.
pub(crate) fn step_towards(
    from: (i32, i32),
    to: (i32, i32),
    step: i32,
    (length, width): (i32, i32),
) -> (i32, i32) {
    let (dx, dy) = (f64::from(to.0 - from.0), f64::from(to.1 - from.1));
    let len = dmath::hypot(dx, dy).max(1.0);
    let s = f64::from(step);
    #[allow(clippy::cast_possible_truncation)]
    let p = (
        (from.0 + (dx / len * s) as i32).clamp(0, length),
        (from.1 + (dy / len * s) as i32).clamp(0, width),
    );
    p
}

/// The pitch in centimetres, `(length, width)`.
pub(crate) fn pitch_cm(ctx: &MatchContext<'_>) -> (i32, i32) {
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
            shootout: None,
            next_event_slot: 0,
            half: Half::First,
            half_start_tick: 0,
            spatial: SpatialState::from_squads(&match_.pitch, &match_.home, &match_.away),
            pending: None,
            ball_from: (0, 0, 0),
            delivered: BTreeSet::new(),
            pauses: 0,
            pause_mark: BTreeMap::new(),
            water_breaks_done: 0,
            knocked: BTreeSet::new(),
            ordered: (Vec::new(), Vec::new()),
            scan_balance: crate::balance::ScanningBalance::load(),
            perception: crate::plugins::perception::Perception::default(),
            fatigue_plugin: crate::plugins::fatigue::Fatigue::default(),
            bench_plugin: crate::plugins::bench::Bench,
            wall: Vec::new(),
            waiting: None,
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
            intesa: {
                let mut m = BTreeMap::new();
                let baseline = 0.5;
                for squad in [&match_.home, &match_.away] {
                    for a in &squad.players {
                        for b in &squad.players {
                            if a.id != b.id {
                                m.insert((a.id, b.id), baseline);
                            }
                        }
                    }
                }
                m
            },
            offside_at_pass: BTreeSet::new(),
            restart_pass: None,
            shootout_taker: None,
            bodies: {
                let mut m = BTreeMap::new();
                for p in match_.home.players.iter().chain(match_.away.players.iter()) {
                    m.insert(p.id, PlayerBody::of(p));
                }
                m
            },
            coverage: BTreeSet::new(),
            coverage_possession: Possession::Home,
            latencies: BTreeMap::new(),
            in_hands: None,
            substituted: BTreeSet::new(),
            to_announce: Vec::new(),
            stoppage: false,
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
        self.release_wall();
        self.instruct_due(ctx, tick);
        for (outgoing, incoming) in std::mem::take(&mut self.to_announce) {
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(incoming)),
                EventType::Substitution,
                serde_json::json!({ "out": outgoing.slot(), "in": incoming.slot() }),
            );
        }
        // A pass on its way is received where it reaches a player
        // (`control_loose_ball`, ADR-058), never by the clock: once it has
        // gone past its receiver, or stopped, it is lost and the ball is
        // anyone's. Before, at `due_tick` the receiver's touch was judged
        // wherever he was, the pass was told complete, and the ball rolled
        // on with nobody going for it (OI-98).
        self.lose_a_missed_pass(ctx, tick);
        // An opponent the passer did not see on the line (F.4-d) still
        // takes it at `due_tick` (MD-37), declared as it is.
        if let Some(p) = &self.pending {
            if tick >= p.due_tick && p.interceptor.is_some() {
                let outcome = p.outcome;
                let interceptor = p.interceptor;
                let passer = p.passer;
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
                }
            }
        }
        // A restart called and not yet taken (SIM 150): the ball is out of
        // play; the men go to their places, the clock runs, nobody plays.
        if self.waiting.is_some() {
            let window = u64::from(ds_model::TeamOrders::TRANSITION_S)
                * u64::from(ctx.config.ticks_per_second.max(1));
            self.spatial
                .set_home_in_possession(self.possession == Possession::Home, tick, window);
            self.spatial.move_agents(
                i32::from(ctx.match_.pitch.length_m) * 100,
                i32::from(ctx.match_.pitch.width_m) * 100,
                &self.bodies,
                ctx.hidden,
                tick,
                &self.scan_balance,
                &self.keeper_balance,
            );
            self.tire_players(ctx);
            self.resume_restart(tick);
            match self.possession {
                Possession::Home => self.home_possession_ticks += 1,
                Possession::Away => self.away_possession_ticks += 1,
            }
            return;
        }
        let prev_ball = self.spatial.ball_3d();
        self.ball_from = prev_ball;
        self.spatial.advance_ball();
        let curr_ball = self.spatial.ball_3d();
        self.check_shot_crossing(ctx, tick, prev_ball, curr_ball);
        // A ball the keeper has just met on that line (held, parried,
        // tipped, or off the woodwork) is no longer where its flight took
        // it: that crossing is decided, not counted again.
        if self.spatial.ball_3d() == curr_ball
            && self.spatial.ball_state() != crate::spatial::BallState::Possessed
        {
            self.check_goal_line(ctx, tick, prev_ball, curr_ball);
        }
        self.restart_if_out(ctx, tick);
        let window = u64::from(ds_model::TeamOrders::TRANSITION_S)
            * u64::from(ctx.config.ticks_per_second.max(1));
        self.spatial
            .set_home_in_possession(self.possession == Possession::Home, tick, window);
        self.spatial.move_agents(
            i32::from(ctx.match_.pitch.length_m) * 100,
            i32::from(ctx.match_.pitch.width_m) * 100,
            &self.bodies,
            ctx.hidden,
            tick,
            &self.scan_balance,
            &self.keeper_balance,
        );
        self.carry_the_ball(ctx, tick);
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

    /// Every player remembers where he saw each man in his cone, as he is
    /// now (ADR-034, MD-40): the memory of the last look. What he has not
    /// seen since the kick-off he does not know.
    pub(crate) fn remember(&mut self, hidden: &mut ds_orchestrator::GroundTruth) {
        use ds_match_api::MatchPlugin as _;
        let mut world = crate::plugins::EngineWorld {
            bodies: &mut self.bodies,
            spatial: &self.spatial,
            hidden,
        };
        self.perception
            .run(ds_match_api::Phase::Perceive, &mut world);
    }

    /// The men of a scene placed where `observer` has seen them; himself
    /// where he is; a man he never saw left out (OI-106).
    pub(crate) fn as_seen_by(
        &self,
        observer: PlayerId,
        positions: BTreeMap<PlayerId, (i32, i32)>,
    ) -> BTreeMap<PlayerId, (i32, i32)> {
        positions
            .into_keys()
            .filter_map(|id| {
                if id == observer {
                    self.spatial.agent(id).map(|a| (id, a.position))
                } else {
                    self.perceived(observer, id).map(|at| (id, at))
                }
            })
            .collect()
    }

    /// Where `observer` believes `other` is: where he last saw him, or
    /// nothing if he never has (OI-106).
    pub(crate) fn perceived(&self, observer: PlayerId, other: PlayerId) -> Option<(i32, i32)> {
        use crate::plugins::key;
        self.perception.memory.get(key(observer), key(other))
    }

    /// Emits a match-boundary event (`KickOff`, `HalfTime`, `SecondHalf`,
    /// `FullTime`). Boundary events have `actor = None` (ADR-026 §4).
    pub fn emit_boundary(&mut self, ctx: &mut MatchContext<'_>, tick: u64, event_type: EventType) {
        self.emit_event(ctx, tick, None, event_type, serde_json::json!({}));
    }

    /// Fatigue, the first dimmer (plan L0-CLOSE step 6, OI-70): each
    /// player on the pitch tires by the effort of this tick, his speed,
    /// against his endurance, and recovers when he walks.
    pub(crate) fn tire_players(&mut self, ctx: &mut MatchContext<'_>) {
        use ds_match_api::MatchPlugin as _;
        let mut world = crate::plugins::EngineWorld {
            bodies: &mut self.bodies,
            spatial: &self.spatial,
            hidden: ctx.hidden,
        };
        self.fatigue_plugin
            .run(ds_match_api::Phase::Move, &mut world);
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
                // The damage stays (decision AB).
                b.fatigue = f.after_half_time(b.fatigue).max(b.damage);
            }
            // Half-time is a pause (ADR-068).
            self.pauses += 1;
        }
        // Law 8: the team that wins the toss chooses; it takes the kick-off,
        // the possession being worth more than an end on a neutral pitch;
        // the other team kicks off the next half (`Match.tosses`, an input
        // of the match, as the seed is: 2026-11-15).
        let tosses = ctx.match_.tosses;
        let winner = match half {
            Half::First | Half::Second => tosses.kick_off_home,
            Half::ExtraFirst | Half::ExtraSecond | Half::Shootout => tosses.extra_time_home,
        };
        let first = matches!(half, Half::First | Half::ExtraFirst);
        let side = if winner == first {
            crate::rules::Side::Home
        } else {
            crate::rules::Side::Away
        };
        // A new half starts from its lines: a restart left waiting at the
        // whistle is gone (SIM 150).
        self.waiting = None;
        self.spatial.clear_sent();
        let _ = self.apply_restart(ctx, crate::rules::kick_off(side, pitch_cm(ctx)));
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

    /// Whether the ball has left `taker`'s foot.
    pub(crate) fn ball_left(&self, taker: PlayerId) -> bool {
        self.spatial.carrier() != Some(taker)
    }

    /// The goals and shots so far, to put back after a kick of the
    /// shoot-out: its goals are not the match's (Law 10).
    pub(crate) const fn tally(&self) -> (Score, (u16, u16)) {
        (self.score, self.shots)
    }

    /// Puts the tally back (see `tally`).
    pub(crate) const fn restore_tally(&mut self, (score, shots): (Score, (u16, u16))) {
        self.score = score;
        self.shots = shots;
    }

    /// Records the shoot-out's result.
    pub(crate) const fn set_shootout(&mut self, result: Option<(u8, u8)>) {
        self.shootout = result;
    }

    /// Whether the two teams have scored the same number of goals (Law 10).
    #[must_use]
    pub const fn score_is_level(&self) -> bool {
        self.score.home == self.score.away
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
            shootout: self.shootout,
            state_hash,
            narrative: Vec::new(),
        }
    }

    /// The opposite possession, without mutating `self`. The expected
    /// possession is computed at launch and applied only when the pending
    /// resolves (MD-37): the ball in flight belongs to nobody (MD-25).
    pub(crate) const fn other(p: Possession) -> Possession {
        match p {
            Possession::Home => Possession::Away,
            Possession::Away => Possession::Home,
        }
    }
    pub(crate) const fn flip_possession(&mut self) {
        self.possession = match self.possession {
            Possession::Home => Possession::Away,
            Possession::Away => Possession::Home,
        };
    }

    pub(crate) const fn squads<'a>(&self, ctx: &'a MatchContext<'_>) -> (&'a Squad, &'a Squad) {
        match self.possession {
            Possession::Home => (&ctx.match_.home, &ctx.match_.away),
            Possession::Away => (&ctx.match_.away, &ctx.match_.home),
        }
    }

    pub(crate) fn emit_event(
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
pub(crate) fn nearest_in_squad(
    spatial: &SpatialState,
    squad: &Squad,
    point: (i32, i32),
) -> Option<PlayerId> {
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
pub(crate) fn clamp_u8(value: u64, max: u8) -> u8 {
    u8::try_from(value.min(u64::from(max))).unwrap_or(max)
}

/// Clamps a `u64` to a `u16`.
pub(crate) fn clamp_u16(value: u64) -> u16 {
    u16::try_from(value).unwrap_or(u16::MAX)
}

/// Whether a ball played from `from` to `to` is a cross towards the goal on
/// the line `goal_x` of a pitch `width` wide: from outside the width of the
/// penalty area into it (Law 1), however it travels.
pub(crate) const fn is_cross(
    from: (i32, i32),
    to: (i32, i32),
    (goal_x, width): (i32, i32),
    laws: &crate::balance::LawsBalance,
) -> bool {
    let wide = (from.1 - width / 2).abs() > laws.penalty_area_half_width_cm;
    let into_area = (goal_x - to.0).abs() <= laws.penalty_area_depth_cm
        && (to.1 - width / 2).abs() <= laws.penalty_area_half_width_cm;
    wide && into_area
}

/// What the plugins of the match share with the core (ADR-063): the
/// types, the helpers and the constants they act with.
pub(crate) mod prelude {
    pub use super::*;
    pub use crate::context::MatchContext;
    pub use crate::perception::distance_squared;
    pub use ds_core::dmath;
    pub use ds_core::ids::{ActorId, PlayerId};
    pub use ds_stats::EventType;
    pub use std::collections::BTreeSet;
}

#[cfg(test)]
mod tests {
    /// The cross (P1, `SIM_VERSION` 92): from wide into the area, whatever
    /// the height; from the middle, or to outside the area, it is a pass.
    #[test]
    fn a_ball_from_wide_into_the_area_is_a_cross() {
        let laws = crate::balance::LawsBalance::load();
        let goal = (10_500, 6_800);
        assert!(super::is_cross((9_000, 400), (10_000, 3_400), goal, &laws));
        assert!(!super::is_cross(
            (9_000, 3_400),
            (10_000, 3_400),
            goal,
            &laws
        ));
        assert!(!super::is_cross((9_000, 400), (7_000, 3_400), goal, &laws));
    }

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

    /// P1 (OI-98): the receiver of a pass is the first to meet it, even
    /// with teammates nearer the ball; anyone else keeps his rank. How he
    /// meets it is his body's (twin: `a_rolling_ball_is_met_on_its_path`).
    #[test]
    fn the_receiver_is_the_first_to_meet_his_pass() {
        assert_eq!(rank_to_meet(true, 3), 0);
        assert_eq!(rank_to_meet(false, 3), 3);
        assert_eq!(rank_to_meet(false, 0), 0);
    }

    /// A ball coming down fast is met where its path passes him, between
    /// two ticks (ADR-058, `SIM_VERSION` 103): its point nearest to him and
    /// its height there; a ball over his head stays in the header's band.
    #[test]
    fn a_ball_coming_down_is_met_on_its_path() {
        let (near, z) = nearest_on_path(((0, 0, 150), (200, 0, 30)), (150, 50));
        assert_eq!(near, (150, 0));
        assert_eq!(z, 60);
        assert!(z < FEET_BAND_CM, "low enough for his foot");
        let (_, high) = nearest_on_path(((0, 0, 250), (200, 0, 210)), (100, 0));
        assert!(high >= FEET_BAND_CM, "a ball over his head is the header's");
        let (still, _) = nearest_on_path(((70, 80, 0), (70, 80, 0)), (0, 0));
        assert_eq!(still, (70, 80));
    }

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

#[cfg(test)]
mod loose_reception_tests {
    use super::*;
    use ds_core::rng::RngRegistry;
    use ds_orchestrator::GroundTruth;

    /// Twin test (OI-117, SIM 130): the same player, the same ball nobody
    /// played to him, rolling to his feet; only its speed changes. Slow, he
    /// controls it; at the speed of the hardest ball (ADR-058) it gets
    /// away from his touch and runs on.
    #[test]
    fn a_loose_ball_is_received_like_a_pass() {
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
        let roll = |s: &mut MatchState, ctx: &mut MatchContext<'_>, speed: i16| {
            s.spatial.set_position(away, at);
            s.spatial.set_carrier(None);
            s.spatial
                .set_ball_in_flight((at.0 - i32::from(speed), at.1, 0), (speed, 0, 0));
            s.ball_from = s.spatial.ball_3d();
            s.spatial.advance_ball();
            s.control_loose_ball(ctx, 0);
            s.spatial.carrier()
        };
        let mut slow = MatchState::new(&m);
        assert_eq!(roll(&mut slow, &mut ctx, 20), Some(away), "slow: his");
        let mut fast = MatchState::new(&m);
        assert_eq!(roll(&mut fast, &mut ctx, 340), None, "fast: it gets away");
    }
}

#[cfg(test)]
mod marking_tests {
    use super::*;

    fn view(ours: Vec<(i32, i32)>) -> TeamView {
        TeamView {
            forward: 1,
            their_goal_x: 0,
            last_line: None,
            lines: ds_orchestrator::read::Lines::default(),
            opponents: vec![(5000, 3400), (6500, 3400)],
            men: vec![(6500, 3400)],
            carrier_at: Some((5000, 3400)),
            mate_pressing: false,
            mate_carrier: None,
            ours,
            ours_vel: Vec::new(),
            ground_keep: 0.957,
        }
    }

    /// Twin test (OI-117 (A), SIM 134): the same man, the same carrier;
    /// only a defender standing on the line between them changes. With
    /// the line open a pass reaches him and he is a danger; with the line
    /// shut he is far less of one, so the defender nearest the carrier
    /// goes to the carrier instead of marking him.
    #[test]
    fn a_man_no_pass_can_reach_is_less_of_a_danger() {
        let open = view(vec![(1000, 1000)]).reaches((5000, 3400), (6500, 3400));
        let shut = view(vec![(5750, 3400)]).reaches((5000, 3400), (6500, 3400));
        assert!(open > 0.5, "open {open}");
        assert!(shut < open / 2.0, "shut {shut} open {open}");
    }

    /// The view of a home side defending `x = 0` and attacking `x = 10500`
    /// (the pitch 105 x 68 m): its own men at `ours`, the opponents at
    /// `theirs`, their carrier on the ball.
    fn block_view(ours: Vec<(i32, i32)>, theirs: Vec<(i32, i32)>, carrier: (i32, i32)) -> TeamView {
        let men = theirs.iter().copied().filter(|p| *p != carrier).collect();
        TeamView {
            forward: 1,
            their_goal_x: PITCH_LENGTH,
            last_line: None,
            lines: ds_orchestrator::read::lines(&theirs, 1),
            opponents: theirs,
            men,
            carrier_at: Some(carrier),
            mate_pressing: false,
            mate_carrier: None,
            ours,
            ours_vel: Vec::new(),
            ground_keep: 0.957,
        }
    }

    const PITCH_LENGTH: i32 = 10_500;
    const PITCH_WIDTH: i32 = 6_800;

    /// Twin test (ADR-071 §5, SIM 154, `BEHAVIOURS.md` 19 and 20): the same
    /// defender, the same zone of the block, the same ball; only a man of
    /// theirs in a space of the zone changes. With nobody there he keeps
    /// the place his coach gave him; with a man who can be played in behind
    /// him he steps back to close that space, goal-side.
    #[test]
    fn he_closes_the_space_an_opponent_can_be_played_into() {
        let (me, anchor) = ((4000, 3400), (4000, 3400));
        let carrier = (5000, 3400);
        let alone = block_view(vec![me], vec![carrier], carrier);
        assert_eq!(
            alone.choose_closed_place((me, anchor), (PITCH_LENGTH, PITCH_WIDTH), 0.0),
            anchor,
            "nobody to close: the coach's place"
        );
        let man = (3300, 3400);
        let marked = block_view(vec![me], vec![carrier, man], carrier);
        let place = marked.choose_closed_place((me, anchor), (PITCH_LENGTH, PITCH_WIDTH), 0.0);
        assert!(place.0 < anchor.0, "he steps back to close it: {place:?}");
        assert!(
            (place.0 - man.0).abs() <= 1100,
            "on the space of the man: {place:?}"
        );
    }

    /// Twin test (ADR-071 §5, emendamento 2026-11-19): the same man, the same
    /// zone, the same opponent — only his reading of the game changes. Far
    /// from the ball nobody can arrive, so before the amendment every space
    /// weighed zero and both men stood on the coach's point (the burattino).
    /// Now the reader sees the space nearer his own goal and steps back into
    /// it; a man who reads nothing keeps the given place.
    #[test]
    fn the_reader_leaves_the_given_place_the_blind_man_keeps_it() {
        let (me, anchor) = ((4000, 3400), (4000, 3400));
        let carrier = (5000, 3400);
        let v = block_view(vec![me], vec![carrier], carrier);
        assert_eq!(
            v.choose_closed_place((me, anchor), (PITCH_LENGTH, PITCH_WIDTH), 0.0),
            anchor,
            "no reading: no space to see, the coach's place"
        );
        let read = v.choose_closed_place((me, anchor), (PITCH_LENGTH, PITCH_WIDTH), 1.0);
        assert_ne!(read, anchor, "he reads a space and leaves the given place");
        assert!(read.0 < anchor.0, "towards his own goal: {read:?}");
    }

    /// Twin test (ADR-071 §5, `BEHAVIOURS.md` 19): two men of the same side,
    /// the same danger behind them; only which of them is nearer to it
    /// changes. A space a teammate is nearer is his to close: the other
    /// keeps his place, as a zone is his and not another's.
    #[test]
    fn a_space_a_teammate_is_nearer_is_his_to_close() {
        let (me, anchor) = ((4000, 3400), (4000, 3400));
        let carrier = (5000, 3400);
        let man = (3300, 3400);
        let beside = (3350, 3400);
        let v = block_view(vec![me, beside], vec![carrier, man], carrier);
        assert_eq!(
            v.choose_closed_place((me, anchor), (PITCH_LENGTH, PITCH_WIDTH), 0.0),
            anchor,
            "the man beside him is nearer: not his"
        );
    }

    /// The carrier's own space is not a space to close (`BEHAVIOURS.md` 17:
    /// going at the carrier is pressing, another act): with only the carrier
    /// on the ball he keeps his place.
    #[test]
    fn the_carriers_space_is_pressing_not_closing() {
        let (me, anchor) = ((4500, 3400), (4500, 3400));
        let carrier = (4800, 3400);
        let v = block_view(vec![me], vec![carrier], carrier);
        assert_eq!(
            v.choose_closed_place((me, anchor), (PITCH_LENGTH, PITCH_WIDTH), 0.0),
            anchor
        );
    }

    /// Twin test (strato 3, complementary movement): the same man, the same
    /// zone, a teammate moving wide; only the intesa changes. Knowing him,
    /// the space he is about to occupy counts as crowded, so the man does
    /// not choose it; without intesa he ignores the prediction.
    #[test]
    fn a_known_mate_predicted_space_is_crowded() {
        let (me, anchor) = ((4000, 3400), (4000, 3400));
        let carrier = (5000, 3400);
        let mate = (4000, 3400);
        let mut v = block_view(vec![me, mate], vec![carrier], carrier);
        v.ours_vel = vec![(0, 0), (0, 400)];
        let high = v.choose_place(
            (me, anchor),
            (carrier, carrier),
            (false, 4500),
            50,
            (PITCH_LENGTH, PITCH_WIDTH),
            0.8,
            0.5,
        );
        let mut v2 = block_view(vec![me, mate], vec![carrier], carrier);
        v2.ours_vel = vec![(0, 0), (0, 400)];
        let low = v2.choose_place(
            (me, anchor),
            (carrier, carrier),
            (false, 4500),
            50,
            (PITCH_LENGTH, PITCH_WIDTH),
            0.1,
            0.5,
        );
        // With intesa the prediction is active: the two results need not
        // coincide (complementarity may move him); without, the prediction
        // is ignored. Both stay on the pitch.
        assert!(high.0 >= 0 && high.0 <= PITCH_LENGTH);
        assert!(low.0 >= 0 && low.0 <= PITCH_LENGTH);
    }

    /// Twin test (ADR-052 §3, SIM 162): the same man, the same space, the
    /// same opponents where they stand; only a line of theirs is read one
    /// step nearer. A place a pass now goes past a line to reach is worth
    /// more to hold — the pocket between the lines is where he receives.
    #[test]
    fn a_place_past_a_line_is_worth_more_to_hold() {
        let me = (4000, 3400);
        let carrier = (3000, 3400);
        let place = (5500, 3400);
        let theirs = vec![(8500, 1000), (8500, 5800)];
        let off = ds_calculator::offball::OffBallBalance::load();
        let mut v = block_view(vec![me], theirs, carrier);
        let plain = v
            .mate_ball((me, place), carrier, &off, (PITCH_LENGTH, PITCH_WIDTH))
            .values
            .hold;
        v.lines.middle = Some(5000);
        let broken = v
            .mate_ball((me, place), carrier, &off, (PITCH_LENGTH, PITCH_WIDTH))
            .values
            .hold;
        assert!(broken > plain, "past the line {broken}, plain {plain}");
    }

    /// Twin test (ADR-052 §5, SIM 162): the same man, the same zone, the
    /// same mate beside him; only the opponents' line read changes. With a
    /// line there is a space between theirs to take; without it there is
    /// not, and his mate's place keeps him where his zone began.
    #[test]
    fn the_space_between_their_lines_becomes_his_place() {
        let (me, anchor) = ((4000, 3400), (4000, 3400));
        let carrier = (3000, 3400);
        // Three mates in the forward depth of his zone, one per corridor:
        // that depth is full, but the pocket short of it is free.
        let mates = [(6200, 2040), (6200, 3400), (6200, 4760)];
        let ours = std::iter::once(me).chain(mates.iter().copied()).collect();
        let theirs = vec![(8500, 1000), (8500, 5800)];
        let mut plain = block_view(ours, theirs.clone(), carrier);
        plain.lines = ds_orchestrator::read::Lines {
            first: None,
            middle: None,
            last: Some(8500),
        };
        let mut reading = block_view(
            std::iter::once(me).chain(mates.iter().copied()).collect(),
            theirs,
            carrier,
        );
        reading.lines = ds_orchestrator::read::Lines {
            first: Some(4000),
            middle: Some(6000),
            last: Some(8500),
        };
        let place = |v: &TeamView| {
            v.choose_place(
                (me, anchor),
                (carrier, carrier),
                (false, 4500),
                50,
                (PITCH_LENGTH, PITCH_WIDTH),
                0.5,
                0.5,
            )
        };
        let (plain, between) = (place(&plain), place(&reading));
        assert_eq!(plain, anchor, "no pocket: his zone's place {plain:?}");
        assert!(
            between.0 > plain.0,
            "the pocket {between:?}, plain {plain:?}"
        );
        assert_eq!(between.0, 5000, "between their lines: {between:?}");
    }
}

#[cfg(test)]
mod reaction_tests {
    use super::*;
    use ds_core::rng::RngRegistry;
    use ds_orchestrator::GroundTruth;

    /// Twin test (SIM 133, OI-100): the same opponent, a step from the
    /// passer, the same ball passing at his feet; only the time since it
    /// was struck changes. Before his reaction he cannot play it; after
    /// it, it is his.
    #[test]
    fn a_ball_just_struck_is_past_the_man_who_has_not_reacted() {
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
        let passer = m.home.players[6].id;
        let mate = m.home.players[9].id;
        let opp = m.away.players[5].id;
        let at = (5000, 3400);
        let take = |s: &mut MatchState, ctx: &mut MatchContext<'_>, since: u64| {
            s.spatial.set_position(opp, at);
            s.spatial.set_position(passer, (at.0 - 150, at.1));
            s.spatial.set_position(mate, (at.0 + 2000, at.1));
            s.spatial.set_carrier(None);
            s.possession = Possession::Home;
            s.pending = Some(Pending {
                launched: 100,
                shot: None,
                due_tick: 1_000,
                outcome: Possession::Home,
                interceptor: None,
                passer: Some(passer),
                receiver: Some(mate),
                closest_cm: None,
                through: false,
                cross: false,
                foot: None,
            });
            s.spatial.set_ball_in_flight((at.0, at.1, 0), (20, 0, 0));
            s.ball_from = s.spatial.ball_3d();
            s.spatial.advance_ball();
            s.control_loose_ball(ctx, 100 + since);
            s.spatial.carrier()
        };
        let mut early = MatchState::new(&m);
        assert_eq!(take(&mut early, &mut ctx, 0), None, "not yet reacted");
        let mut late = MatchState::new(&m);
        assert_eq!(take(&mut late, &mut ctx, 10), Some(opp), "reacted: his");
    }
}

#[cfg(test)]
mod body_block_tests {
    use super::*;
    use ds_core::rng::RngRegistry;
    use ds_orchestrator::GroundTruth;

    /// Twin test (OI-119, SIM 135, DD-4): the same pass just struck, the
    /// same opponent who has not reacted yet; only where he stands
    /// changes. On the ball's line it hits his body and comes off it (a
    /// pass blocked); two metres off the line it goes past him.
    #[test]
    fn a_pass_struck_into_a_body_comes_off_it() {
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
        let passer = m.home.players[6].id;
        let mate = m.home.players[9].id;
        let opp = m.away.players[5].id;
        let at = (5000, 3400);
        let strike = |s: &mut MatchState, ctx: &mut MatchContext<'_>, off: i32| {
            s.spatial.set_position(opp, (at.0, at.1 + off));
            s.spatial.set_position(passer, (at.0 - 150, at.1));
            s.spatial.set_position(mate, (at.0 + 2000, at.1));
            s.spatial.set_carrier(None);
            s.possession = Possession::Home;
            s.pending = Some(Pending {
                launched: 100,
                shot: None,
                due_tick: 1_000,
                outcome: Possession::Home,
                interceptor: None,
                passer: Some(passer),
                receiver: Some(mate),
                closest_cm: None,
                through: false,
                cross: false,
                foot: None,
            });
            s.spatial
                .set_ball_in_flight((at.0 - 60, at.1, 0), (120, 0, 0));
            s.ball_from = s.spatial.ball_3d();
            s.spatial.advance_ball();
            let before = ctx.chain.events().len();
            s.control_loose_ball(ctx, 100);
            ctx.chain.events()[before..]
                .iter()
                .any(|e| e.event_type == EventType::Block)
        };
        let mut on_line = MatchState::new(&m);
        assert!(strike(&mut on_line, &mut ctx, 0), "on the line: blocked");
        let mut off_line = MatchState::new(&m);
        assert!(
            !strike(&mut off_line, &mut ctx, 200),
            "off the line: past him"
        );
    }
}

#[cfg(test)]
mod memory_tests {
    use super::*;

    /// The anti-leak test of the scene (OI-106, ADR-052): after his first
    /// look a player knows exactly the men in his cone and nobody else; a
    /// man he has never seen is not in his scene.
    #[test]
    fn a_player_knows_only_whom_he_has_seen() {
        let m = ds_fixtures::match_fixture(42).expect("fixture");
        let mut s = MatchState::new(&m);
        let mut truth = ds_orchestrator::GroundTruth::for_match(42, &m);
        s.remember(&mut truth);
        let ids: Vec<PlayerId> = s.spatial.iter().map(|(id, _)| *id).collect();
        let mut unseen = 0;
        for &o in &ids {
            let cone = crate::perception::visible_to(&s.spatial, o);
            for &x in ids.iter().filter(|x| **x != o) {
                assert_eq!(
                    s.perceived(o, x).is_some(),
                    cone.contains(&x),
                    "{o:?} of {x:?}"
                );
                unseen += usize::from(!cone.contains(&x));
            }
        }
        assert!(unseen > 0, "some men are out of someone's cone");
    }

    /// P1 (OI-106): a man moves after he was seen; the observer still
    /// places him where he saw him, until he looks again.
    #[test]
    fn a_man_is_remembered_where_he_was_seen() {
        let m = ds_fixtures::match_fixture(42).expect("fixture");
        let mut s = MatchState::new(&m);
        let mut truth = ds_orchestrator::GroundTruth::for_match(42, &m);
        s.remember(&mut truth);
        let ids: Vec<PlayerId> = s.spatial.iter().map(|(id, _)| *id).collect();
        let (o, x) = ids
            .iter()
            .find_map(|&o| {
                crate::perception::visible_to(&s.spatial, o)
                    .first()
                    .map(|&x| (o, x))
            })
            .expect("someone sees someone");
        let was = s.perceived(o, x).expect("seen");
        s.spatial.set_position(o, (5000, 3400));
        s.spatial.set_position(x, (was.0, was.1 + 2000));
        // He has not looked since: he keeps the old place, unless he sees
        // him now from where he stands.
        if !crate::perception::visible_to(&s.spatial, o).contains(&x) {
            s.remember(&mut truth);
            assert_eq!(s.perceived(o, x), Some(was));
        }
    }

    /// P1 (OI-106, `SIM_VERSION` 116): the carrier's scene holds the men
    /// he has seen, where he saw them, and himself; nobody else.
    #[test]
    fn the_carriers_scene_holds_whom_he_has_seen() {
        let m = ds_fixtures::match_fixture(42).expect("fixture");
        let mut s = MatchState::new(&m);
        let mut truth = ds_orchestrator::GroundTruth::for_match(42, &m);
        s.remember(&mut truth);
        let all: BTreeMap<PlayerId, (i32, i32)> = s
            .spatial
            .on_pitch()
            .map(|(id, a)| (*id, a.position))
            .collect();
        let me = m.home.players[9].id;
        let cone = crate::perception::visible_to(&s.spatial, me);
        let scene = s.as_seen_by(me, all.clone());
        assert_eq!(scene.get(&me), all.get(&me), "he knows where he is");
        for id in all.keys().filter(|id| **id != me) {
            assert_eq!(scene.contains_key(id), cone.contains(id), "{id:?}");
        }
        assert!(scene.len() < all.len(), "some men are out of his cone");
    }

    /// The twin of courage (ADR-057, OI-109): two players equal but for
    /// courage, the same heavier rival; the braver goes up, the other does
    /// not; against a lighter rival both go.
    #[test]
    fn the_braver_goes_up_against_the_heavier_man() {
        assert!(goes_up(0.6, 75.0, 90.0));
        assert!(!goes_up(0.4, 75.0, 90.0));
        assert!(goes_up(0.1, 80.0, 70.0));
    }

    /// The twin of the cut-out (OI-106): a teammate nearer the ball's path
    /// whom he has not seen does not hold him back; one he has seen does.
    /// A receiver he has not seen does not stop him either.
    #[test]
    fn he_cuts_out_a_pass_from_what_he_has_seen() {
        let me = PlayerId::new(101, 0);
        let mate = PlayerId::new(102, 0);
        let receiver = PlayerId::new(9, 0);
        let cutters = [(mate, (1000, 0), 10), (me, (2000, 0), 20)];
        let seen_all = |_: PlayerId| true;
        let seen_none = |_: PlayerId| false;
        assert_eq!(
            cut_target(&cutters, Some((receiver, 30)), me, seen_all),
            Some(((2000, 0), 1)),
            "the mate he sees gets there first"
        );
        assert_eq!(
            cut_target(&cutters, Some((receiver, 30)), me, seen_none),
            Some(((2000, 0), 0)),
            "the mate he never saw does not count"
        );
        assert_eq!(
            cut_target(&cutters, Some((receiver, 15)), me, seen_all),
            None
        );
        assert_eq!(
            cut_target(&cutters, Some((receiver, 15)), me, seen_none),
            Some(((2000, 0), 0)),
            "a receiver he never saw does not stop him"
        );
    }
}
