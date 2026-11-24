//! The restarts of play, in their minimal form (Laws 8, 15, 16, 17).
//!
//! **One responsibility:** given where the ball left the pitch and which
//! side played it last, say who restarts, from where, and how. Pure
//! functions: data in, a decision out; the engine applies it (it places
//! the taker and the ball). A volunteer can read and test a law here
//! without reading the engine.
//!
//! `Removed-by: OI-48` (the rules module: the rule as data, its
//! application as code, a versioned ruleset in the save). Until then the
//! laws are written as code, declared, and "played it last" is read as
//! "the side in possession", an approximation.
//!
//! The referee plugin (ADR-063 §4): the Laws applied, objectively (ADR-021
//! note). Moved here from `rules.rs` (2026-11-15); `crate::rules` is the
//! same module, kept as a name for the code that reads it.

use ds_match_api::{MatchPlugin, PluginCard};
use ds_model::{MatchRules, Tosses};
use ds_stats::{Half, MatchEvent};

/// The referee: the Laws of the Game, applied.
#[derive(Debug, Clone, Copy, Default)]
pub struct Referee;

impl MatchPlugin for Referee {
    fn card(&self) -> PluginCard {
        crate::registry::card("referee")
    }
}

/// A side of the match.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    /// The home side (attacks towards `x = length`).
    Home,
    /// The away side (attacks towards `x = 0`).
    Away,
}

impl Side {
    /// The other side.
    #[must_use]
    pub const fn other(self) -> Self {
        match self {
            Self::Home => Self::Away,
            Self::Away => Self::Home,
        }
    }
}

/// How play restarts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartKind {
    /// From the centre spot (Law 8): the start of a half, or after a goal.
    KickOff,
    /// From the touchline (Law 15).
    ThrowIn,
    /// From the goal area (Law 16).
    GoalKick,
    /// From the corner arc (Law 17).
    Corner,
    /// A free kick, after an offence (Laws 11-13).
    FreeKick,
    /// A penalty kick (Law 14), after a foul in the penalty area.
    Penalty,
}

/// A card (Law 12).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Card {
    /// A caution.
    Yellow,
    /// A sending-off.
    Red,
}

/// The facts of a foul, as the Laws read them (ADR-042 §4).
#[derive(Debug, Clone, Copy)]
pub struct FoulFacts {
    /// The offender went in sliding.
    pub slide: bool,
    /// He came from behind.
    pub from_behind: bool,
    /// His approach speed, thousandths of the reference speed.
    pub approach_milli: i32,
    /// Whether, and where, the foul denied an obvious goal-scoring
    /// opportunity.
    pub denial: Denial,
}

/// The denial of an obvious goal-scoring opportunity (Law 12).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Denial {
    /// No obvious goal-scoring opportunity was denied.
    No,
    /// Denied outside his own penalty area, or not trying to play the ball.
    Chance,
    /// Denied in his own penalty area, the penalty awarded, while he tried
    /// to play the ball (Law 12, since 2016: then a caution).
    ChanceForTheBall,
}

/// Law 12's card for a foul.
///
/// Excessive force (a slide from behind at a sprint) or the denial of an
/// obvious goal-scoring opportunity is a red card; a reckless challenge (a
/// slide from behind, at speed) a yellow; a careless one none. The denial
/// in his own penalty area, the penalty awarded, by a challenge that tried
/// to play the ball is a yellow (Law 12, since 2016; SIM 194: before, it was
/// always a red). Thresholds in `balance/laws.json`.
#[must_use]
pub const fn card_for(f: &FoulFacts, reckless_milli: i32, excessive_milli: i32) -> Option<Card> {
    let excessive = f.slide && f.from_behind && f.approach_milli >= excessive_milli;
    if excessive || matches!(f.denial, Denial::Chance) {
        Some(Card::Red)
    } else if matches!(f.denial, Denial::ChanceForTheBall)
        || (f.slide && f.from_behind && f.approach_milli >= reckless_milli)
    {
        Some(Card::Yellow)
    } else {
        None
    }
}

/// Who restarts, from where, and how.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Restart {
    /// The side that restarts.
    pub side: Side,
    /// Where the ball is placed, in cm.
    pub spot: (i32, i32),
    /// How.
    pub kind: RestartKind,
}

/// The kick-off from the centre spot by `side` (Law 8).
#[must_use]
pub const fn kick_off(side: Side, (length, width): (i32, i32)) -> Restart {
    Restart {
        side,
        spot: (length / 2, width / 2),
        kind: RestartKind::KickOff,
    }
}

/// The restart for a ball at `(x, y)`, or `None` if the ball is on the
/// pitch. `last` is the side that played it last.
///
/// Over a touchline: a throw-in for the other side, where the ball went
/// out. Over a goal line (not into the goal: the engine scores the goal
/// first): a corner if the defending side played it last, a goal kick if
/// the attackers did.
#[must_use]
pub fn out_of_play((x, y): (i32, i32), last: Side, (length, width): (i32, i32)) -> Option<Restart> {
    // The whole ball over the line (Laws 9 and 10): beyond it by its radius,
    // as the goal reads it (SIM 200, code audit S1; before, the centre on
    // the line was out, and a ball just in the goal at a tick's end was
    // given as a goal kick before the goal could be scored).
    #[allow(clippy::cast_possible_truncation)]
    let r = ds_core::geometry::BALL_RADIUS_CM as i32;
    if (-r..=length + r).contains(&x) && (-r..=width + r).contains(&y) {
        return None;
    }
    if !(-r..=width + r).contains(&y) {
        return Some(Restart {
            side: last.other(),
            spot: (x.clamp(100, length - 100), y.clamp(0, width)),
            kind: RestartKind::ThrowIn,
        });
    }
    let home_goal_line = x < 0;
    let defending = if home_goal_line {
        Side::Home
    } else {
        Side::Away
    };
    let line_x = if home_goal_line { 0 } else { length };
    Some(if last == defending {
        Restart {
            side: defending.other(),
            spot: (line_x, if y < width / 2 { 0 } else { width }),
            kind: RestartKind::Corner,
        }
    } else {
        Restart {
            side: defending,
            // Law 16, from the goal area (code audit C5: it was 550 here).
            spot: {
                let depth = ds_calculator::laws::law1().goal_area_depth_cm;
                (
                    if home_goal_line {
                        depth
                    } else {
                        length - depth
                    },
                    width / 2,
                )
            },
            kind: RestartKind::GoalKick,
        }
    })
}

/// A free kick for `side` where the offence happened (Laws 11-13).
#[must_use]
pub const fn free_kick(side: Side, spot: (i32, i32)) -> Restart {
    Restart {
        side,
        spot,
        kind: RestartKind::FreeKick,
    }
}

/// Law 12: whether a foul at `spot` denies an obvious goal-scoring
/// opportunity at the goal `goal`.
///
/// Near enough (`dogso_cm`), and no outfield defender other than the
/// offender nearer the goal line than the spot (`cover_depths`, their
/// distances from that line). The keeper is not cover: he is the man the
/// attacker through on goal had to beat.
#[must_use]
pub fn obvious_chance(
    spot: (i32, i32),
    goal: (i32, i32),
    dogso_cm: i32,
    cover_depths: &[i32],
) -> bool {
    let depth = (goal.0 - spot.0).abs();
    let (dx, dy) = (i64::from(goal.0 - spot.0), i64::from(goal.1 - spot.1));
    dx * dx + dy * dy <= i64::from(dogso_cm).pow(2) && cover_depths.iter().all(|d| *d >= depth)
}

/// Laws 13-14: the restart of a foul.
///
/// Inside the defending side's penalty area, a penalty from the mark `penalty_spot_cm` from the goal line at
/// `goal`; elsewhere a free kick where it happened. `side` is the side
/// that suffered it.
#[must_use]
pub const fn foul_restart(
    side: Side,
    spot: (i32, i32),
    in_area: bool,
    goal: (i32, i32),
    penalty_spot_cm: i32,
) -> Restart {
    if in_area {
        let x = if goal.0 == 0 {
            penalty_spot_cm
        } else {
            goal.0 - penalty_spot_cm
        };
        Restart {
            side,
            spot: (x, goal.1),
            kind: RestartKind::Penalty,
        }
    } else {
        free_kick(side, spot)
    }
}

/// Law 11: whether a player is in an offside position.
///
/// The player attacks towards `goal_x`; the ball is played from `ball_x`.
/// He is offside if he is in the opponents' half and nearer to their goal
/// line than both the ball and the second-last opponent (the keeper
/// counts). Level is onside.
#[must_use]
pub fn offside_position(
    player_x: i32,
    ball_x: i32,
    opponents_x: &[i32],
    goal_x: i32,
    length: i32,
) -> bool {
    // Distance from the goal line he attacks: smaller is nearer.
    let depth = |x: i32| (goal_x - x).abs();
    let mut opp: Vec<i32> = opponents_x.iter().map(|x| depth(*x)).collect();
    opp.sort_unstable();
    let Some(second_last) = opp.get(1).copied() else {
        return false;
    };
    let in_their_half = depth(player_x) < length / 2;
    in_their_half && depth(player_x) < depth(ball_x) && depth(player_x) < second_last
}

/// Laws 7 and 10: the periods of extra time.
///
/// The competition asks for a winner and the scores are level at ninety
/// minutes. Empty otherwise; the engine applies the periods.
#[must_use]
pub const fn extra_time(rules: &MatchRules, level: bool) -> &'static [Half] {
    if rules.extra_time && level {
        &[Half::ExtraFirst, Half::ExtraSecond]
    } else {
        &[]
    }
}

/// Law 10: whether a shoot-out follows, when the competition asks for one
/// and the scores are still level after extra time.
#[must_use]
pub const fn shootout(rules: &MatchRules, level: bool) -> bool {
    rules.penalty_shootout && level
}

/// Law 8: who takes the kick-off, from the toss and the period. The side
/// that won the toss takes the first kick-off of its pair of periods.
#[must_use]
pub const fn kick_off_taker_side(tosses: &Tosses, half: Half) -> Side {
    let winner = match half {
        Half::First | Half::Second => tosses.kick_off_home,
        Half::ExtraFirst | Half::ExtraSecond | Half::Shootout => tosses.extra_time_home,
    };
    let first = matches!(half, Half::First | Half::ExtraFirst);
    if winner == first {
        Side::Home
    } else {
        Side::Away
    }
}

/// Law 1: whether a spot is in the penalty area.
///
/// `depth` and `lateral` are read from the caller's own geometry. The foul
/// and the keeper's box use this one predicate; the dimensions stay in
/// their balance files.
#[must_use]
pub const fn in_penalty_area(depth: i32, lateral: i32, depth_cm: i32, half_width_cm: i32) -> bool {
    depth <= depth_cm && lateral <= half_width_cm
}

/// Law 10: the goal goes to the side that put the ball in; when the last
/// man on it played for the defending side, it is an own goal.
#[must_use]
pub fn own_goal(last_is_home: Option<bool>, defending_home: bool) -> bool {
    last_is_home == Some(defending_home)
}

/// Law 11: no offside from the first pass after a throw-in, goal kick or
/// corner.
#[must_use]
pub const fn offside_exempt(restart: Option<RestartKind>) -> bool {
    matches!(
        restart,
        Some(RestartKind::ThrowIn | RestartKind::GoalKick | RestartKind::Corner)
    )
}

/// Law 12: a second yellow is a red.
#[must_use]
pub const fn card_after(already_cautioned: bool, card: Option<Card>) -> Option<Card> {
    match card {
        Some(Card::Yellow) if already_cautioned => Some(Card::Red),
        c => c,
    }
}

/// Law 12: advantage when the fouled side keeps the ball in the
/// opponents' half, outside the penalty area. `depth` is from the goal
/// they attack, `length` the pitch's.
#[must_use]
pub const fn play_advantage(kept: bool, in_area: bool, depth: i32, length: i32) -> bool {
    kept && !in_area && depth < length / 2
}

/// Law 3: the substitutions still available at a stoppage.
#[must_use]
pub const fn subs_remaining(allowed: usize, made: usize, pending: usize) -> usize {
    allowed.saturating_sub(made + pending)
}

/// Competition rule (ADR-068): at the first stoppage at or after a break's
/// minute the referee stops play. `None` when no such break is due.
#[must_use]
pub fn water_break_at(rules: &MatchRules, done: u8, minute: u64) -> Option<u16> {
    let at = *rules.water_breaks.get(usize::from(done))?;
    (at != 0 && minute >= u64::from(at)).then_some(at)
}

/// Law 10: the side that won the toss kicks first.
#[must_use]
pub const fn shootout_first_home(tosses: &Tosses) -> bool {
    tosses.shootout_home
}

/// Law 10: within the first five kicks, a side that cannot be caught has
/// won.
#[must_use]
pub fn shootout_unreachable(goals: [u8; 2], kicks: [usize; 2]) -> bool {
    let reach = |s: usize| usize::from(goals[s]) + 5usize.saturating_sub(kicks[s]);
    kicks[0] <= 5
        && kicks[1] <= 5
        && (usize::from(goals[0]) > reach(1) || usize::from(goals[1]) > reach(0))
}

/// Law 10: after five kicks each, unequal scores decide; level scores go
/// to sudden death.
#[must_use]
pub const fn shootout_after_five(goals: [u8; 2], kicks: [usize; 2]) -> bool {
    kicks[0] >= 5 && goals[0] != goals[1]
}

/// Law 10: a kick ends when the ball is saved, in, out, or played by
/// someone else — not by another shot of the taker's.
#[must_use]
pub fn shootout_kick_event_done(e: &MatchEvent, kicker: PlayerId) -> bool {
    matches!(
        e.event_type,
        EventType::Save
            | EventType::Goal
            | EventType::Corner
            | EventType::GoalKick
            | EventType::ThrowIn
    ) || (e.event_type != EventType::Shot
        // Played by another man: an event with nobody behind it (a board,
        // a pause) does not end the kick (2026-11-23).
        && e.actor.is_some()
        && e.actor != Some(ActorId::Player(kicker)))
        && !matches!(e.event_type, EventType::Pressure | EventType::Woodwork)
}

/// The Laws applied to the match in progress: offside, the foul, the
/// ball out of play (moved from `state.rs`, 2026-11-15).
use crate::state::prelude::*;

impl MatchState {
    /// Law 11: `offender` was in an offside position when the ball was
    /// played, and now he plays it. A free kick to the defenders where he
    /// is.
    ///
    /// `played` is the tick the ball was played, told with the call (SIM
    /// 196): the position is judged then, and a reader of the facts (the
    /// view's lines) needs that moment, not the last pass received.
    pub(crate) fn call_offside(
        &mut self,
        ctx: &mut MatchContext<'_>,
        tick: u64,
        offender: PlayerId,
        offender_home: bool,
        (ball, played): ((i32, i32), Option<u64>),
    ) {
        self.offside_at_pass.clear();
        let spot = self.spatial.agent(offender).map_or(ball, |a| a.position);
        let side = if offender_home {
            crate::rules::Side::Away
        } else {
            crate::rules::Side::Home
        };
        if let Some(taker) = self.call_restart(ctx, tick, crate::rules::free_kick(side, spot), None)
        {
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(taker)),
                EventType::FreeKick,
                serde_json::json!({ "reason": "offside", "offender": offender.slot(), "x": spot.0, "y": spot.1, "played_tick": played }),
            );
        }
    }

    /// Law 11: when `passer` plays the ball from `ball_x`, which of his
    /// teammates are in an offside position. Not from a throw-in, a goal
    /// kick or a corner (the first pass after such a restart).
    pub(crate) fn mark_offside_at_pass(
        &mut self,
        ctx: &MatchContext<'_>,
        tick: u64,
        passer: PlayerId,
        ball_x: i32,
    ) {
        self.offside_at_pass.clear();
        self.offside_played = Some(tick);
        let restart = self.restart_pass.take();
        if crate::rules::offside_exempt(restart) {
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

    /// Law 12, applied objectively (ADR-021 note, ADR-042 step b): the foul
    /// is called; its card follows the facts (a second yellow is a red, a
    /// red sends the player off); then the advantage, or the free kick where
    /// it happened, or a penalty if it happened in the penalty area.
    /// Whether the foul denied an obvious goal-scoring opportunity (Law 12:
    /// the distance to the goal, the location and number of defenders): near
    /// the goal he attacks, and no outfield defender but the offender nearer
    /// to it. The keeper alone does not take it away: he is the man the
    /// attacker through on goal had to beat (`SIM_VERSION` 112; before, the
    /// keeper counted as cover, and since he nearly always stands nearer the
    /// goal, the opportunity almost never was one). Every foul of the engine
    /// is a challenge for the ball: in his own area, the penalty given, it
    /// tried to play it (Law 12, SIM 194).
    fn denial(
        &self,
        ctx: &MatchContext<'_>,
        offender: PlayerId,
        fouled_home: bool,
        (spot, goal): ((i32, i32), (i32, i32)),
        in_area: bool,
    ) -> crate::rules::Denial {
        let depth = |p: (i32, i32)| (goal.0 - p.0).abs();
        let defenders = if fouled_home {
            &ctx.match_.away
        } else {
            &ctx.match_.home
        };
        let their_keeper = defenders.players.first().map(|p| p.id);
        let cover: Vec<i32> = defenders
            .players
            .iter()
            .filter(|q| q.id != offender && Some(q.id) != their_keeper)
            .filter_map(|q| self.spatial.agent(q.id))
            .map(|a| depth(a.position))
            .collect();
        if !crate::rules::obvious_chance(spot, goal, self.laws.dogso_distance_cm, &cover) {
            crate::rules::Denial::No
        } else if in_area {
            crate::rules::Denial::ChanceForTheBall
        } else {
            crate::rules::Denial::Chance
        }
    }

    pub(crate) fn apply_foul(
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
        let in_area = crate::rules::in_penalty_area(
            depth(spot),
            (spot.1 - goal.1).abs(),
            laws.law1.penalty_area_depth_cm,
            laws.law1.penalty_area_half_width_cm,
        );
        facts.denial = self.denial(ctx, offender, fouled_home, (spot, goal), in_area);
        // The knock (decision Y, ADR-066): a contact this strong on this
        // body, and the fouled man cannot go on.
        // The force on his body, read by the actor (ADR-112); the Law
        // compares it with the threshold.
        let force = ctx.hidden.contact_force_milli(fouled, facts.approach_milli);
        let knocked =
            crate::balance::KnockBalance::load().knocks(force) && self.knocked.insert(fouled);
        // Advantage: the side that suffered it keeps the ball in the
        // opponents' half, outside the area; play goes on. Not with a man
        // down (Law 5, SIM 196): the referee stops play for him.
        let advantage =
            !knocked && crate::rules::play_advantage(kept, in_area, depth(spot), length);
        // The foul tells whether play went on (SIM 203, T15): the time lost
        // to it (Law 7) is only that of a foul that stopped play.
        self.emit_event(
            ctx,
            tick,
            Some(ActorId::Player(offender)),
            EventType::Foul,
            serde_json::json!({ "against": fouled.slot(), "x": spot.0, "y": spot.1, "slide": facts.slide, "from_behind": facts.from_behind, "advantage": advantage }),
        );
        if knocked {
            // The man down stops play: a pause for the bench (ADR-068).
            self.pauses += 1;
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(fouled)),
                EventType::Injury,
                serde_json::json!({ "by": offender.slot(), "force_milli": facts.approach_milli }),
            );
        }
        // Law 12: with the advantage played the chance is not denied, and the
        // offender is cautioned, not sent off (SIM 200, code audit S14;
        // before, the red for the denial and play went on).
        let denied = !matches!(facts.denial, crate::rules::Denial::No);
        if advantage {
            facts.denial = crate::rules::Denial::No;
        }
        let card = crate::rules::card_for(
            &facts,
            laws.reckless_speed_milli,
            laws.excessive_speed_milli,
        );
        let card = if advantage && denied && card.is_none() {
            Some(crate::rules::Card::Yellow)
        } else {
            card
        };
        let card = match card {
            Some(crate::rules::Card::Yellow) => {
                crate::rules::card_after(!self.cautioned.insert(offender), card)
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
        if advantage {
            return;
        }
        let side = if fouled_home {
            crate::rules::Side::Home
        } else {
            crate::rules::Side::Away
        };
        // Laws 13-14: a penalty in the area, a free kick elsewhere.
        let restart = crate::rules::foul_restart(side, spot, in_area, goal, laws.penalty_spot_cm);
        let kind = if restart.kind == crate::rules::RestartKind::Penalty {
            EventType::Penalty
        } else {
            EventType::FreeKick
        };
        if let Some(taker) = self.call_restart(ctx, tick, restart, None) {
            self.emit_event(
                ctx,
                tick,
                Some(ActorId::Player(taker)),
                kind,
                serde_json::json!({ "reason": "foul", "x": restart.spot.0, "y": restart.spot.1 }),
            );
        }
    }

    /// The ball out of play gets a restart (Laws 15-17, `crate::rules`):
    /// the rule decides who restarts, from where and how; the engine
    /// applies it. Before, a ball that left the pitch elsewhere than into
    /// the goal stayed out for the rest of the match (OI-97).
    pub(crate) fn restart_if_out(&mut self, ctx: &mut MatchContext<'_>, tick: u64) {
        // A restart waiting: the ball is out of play (SIM 150).
        if self.waiting.is_some() {
            return;
        }
        // Out is where the ball is (Laws 9, 15-17), whoever has it: rolling
        // ahead of him between two touches, or under his foot as he takes it
        // (plan L0-CLOSE 11). At his feet it is where he is, on the pitch.
        let (x, y, _) = self.spatial.ball_3d();
        // The last to touch it, from the same touches the goal reads (code
        // audit S13): the man who has it; else the last who touched it on
        // its way (a deflection, a parry); else the last who played it; the
        // side in possession only when nobody is known.
        let seen = self.spatial.touches();
        let last_man = self.spatial.carrier().or(seen.deflected).or(seen.played);
        let last = last_man.map_or_else(
            || side_of(self.possession),
            |c| {
                if ctx.match_.home.players.iter().any(|p| p.id == c) {
                    crate::rules::Side::Home
                } else {
                    crate::rules::Side::Away
                }
            },
        );
        let Some(restart) = crate::rules::out_of_play((x, y), last, pitch_cm(ctx)) else {
            return;
        };
        if let Some(taker) = self.call_restart(ctx, tick, restart, None) {
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_referee_plugin_has_its_card() {
        use ds_match_api::MatchPlugin;
        assert_eq!(super::Referee.card().name, "referee");
        assert!(super::Referee.card().is_valid());
    }

    use super::*;

    const PITCH: (i32, i32) = (10_500, 6_800);

    /// The Laws of the game played are pure decisions in this module, not
    /// branches scattered through the engine (OI, 2026-11-21).
    #[test]
    fn the_laws_of_the_game_played_are_decided_here() {
        let mut tosses = Tosses::HOME;
        assert_eq!(kick_off_taker_side(&tosses, Half::First), Side::Home);
        assert_eq!(kick_off_taker_side(&tosses, Half::Second), Side::Away);
        tosses.kick_off_home = false;
        assert_eq!(kick_off_taker_side(&tosses, Half::First), Side::Away);
        assert!(shootout_first_home(&Tosses::HOME));

        assert!(in_penalty_area(1000, 600, 1650, 2016));
        assert!(!in_penalty_area(2000, 600, 1650, 2016));
        assert!(!in_penalty_area(1000, 2600, 1650, 2016));
        assert!(in_penalty_area(-100, 600, 1650, 2016));
        assert!(own_goal(Some(true), true));
        assert!(!own_goal(Some(false), true));
        assert!(!own_goal(None, true));

        assert!(offside_exempt(Some(RestartKind::Corner)));
        assert!(!offside_exempt(Some(RestartKind::FreeKick)));
        assert!(!offside_exempt(None));
        assert_eq!(card_after(true, Some(Card::Yellow)), Some(Card::Red));
        assert_eq!(card_after(false, Some(Card::Yellow)), Some(Card::Yellow));
        assert_eq!(card_after(false, Some(Card::Red)), Some(Card::Red));
        // The length of the pitch, as the engine passes it: the advantage
        // is in the opponents' half (before, the test passed half of it and
        // guarded a quarter).
        assert!(play_advantage(true, false, 5000, 10_500));
        assert!(!play_advantage(true, false, 5500, 10_500));
        assert!(!play_advantage(true, true, 1000, 10_500));

        assert_eq!(subs_remaining(5, 3, 1), 1);
        assert_eq!(subs_remaining(5, 5, 1), 0);
        let rules = MatchRules {
            water_breaks: [30, 75],
            ..MatchRules::default()
        };
        assert_eq!(water_break_at(&rules, 0, 29), None);
        assert_eq!(water_break_at(&rules, 0, 30), Some(30));
        assert_eq!(water_break_at(&rules, 1, 74), None);
        assert_eq!(water_break_at(&rules, 1, 75), Some(75));
        assert_eq!(water_break_at(&rules, 2, 90), None);

        assert!(shootout_unreachable([3, 0], [4, 3]));
        assert!(!shootout_unreachable([2, 1], [3, 3]));
        assert!(shootout_after_five([4, 3], [5, 5]));
        assert!(!shootout_after_five([4, 4], [5, 5]));
    }

    /// Laws 7 and 10: the competition's request and the level score are
    /// both needed; the two periods and the shoot-out come from here.
    #[test]
    fn extra_time_and_shootout_need_the_request_and_a_level_score() {
        let mut rules = MatchRules::default();
        assert_eq!(extra_time(&rules, true), []);
        assert!(!shootout(&rules, true));
        rules.extra_time = true;
        rules.penalty_shootout = true;
        assert_eq!(extra_time(&rules, false), []);
        assert!(!shootout(&rules, false));
        assert_eq!(
            extra_time(&rules, true),
            [Half::ExtraFirst, Half::ExtraSecond]
        );
        assert!(shootout(&rules, true));
    }

    /// Law 12 (P1, `SIM_VERSION` 112): through on goal with only the keeper to
    /// beat, a foul denies an obvious chance; with an outfield defender
    /// covering nearer the goal, or far from it, it does not.
    #[test]
    fn only_the_keeper_to_beat_is_an_obvious_chance() {
        let goal = (10_500, 3400);
        assert!(obvious_chance((9000, 3400), goal, 2500, &[]));
        assert!(
            obvious_chance((9000, 3400), goal, 2500, &[2000]),
            "a defender behind him"
        );
        assert!(
            !obvious_chance((9000, 3400), goal, 2500, &[800]),
            "a defender covering"
        );
        assert!(
            !obvious_chance((7000, 3400), goal, 2500, &[]),
            "too far from goal"
        );
    }

    /// Law 14 (P1, the guard of the penalty, a rare act in the repertoire
    /// test): a foul in the area is a penalty from the mark of the goal it
    /// was committed in front of; outside it, a free kick where it happened.
    #[test]
    fn a_foul_in_the_area_is_a_penalty() {
        let r = foul_restart(Side::Home, (9800, 3000), true, (10_500, 3400), 1100);
        assert_eq!(r.kind, RestartKind::Penalty);
        assert_eq!(r.spot, (9400, 3400));
        let r = foul_restart(Side::Away, (700, 3000), true, (0, 3400), 1100);
        assert_eq!(r.spot, (1100, 3400));
        let r = foul_restart(Side::Home, (6000, 3000), false, (10_500, 3400), 1100);
        assert_eq!((r.kind, r.spot), (RestartKind::FreeKick, (6000, 3000)));
    }

    /// Law 12 (P1): a careless foul is no card; a slide from behind,
    /// yellow; a slide from behind at a sprint, red; denying an obvious
    /// chance, red.
    #[test]
    fn the_card_follows_the_facts() {
        let careless = FoulFacts {
            slide: false,
            from_behind: false,
            approach_milli: 300,
            denial: Denial::No,
        };
        assert_eq!(card_for(&careless, 600, 900), None);
        let behind = FoulFacts {
            from_behind: true,
            ..careless
        };
        assert_eq!(card_for(&behind, 600, 900), None, "on his feet from behind");
        let slide_behind = FoulFacts {
            slide: true,
            approach_milli: 700,
            ..behind
        };
        assert_eq!(card_for(&slide_behind, 600, 900), Some(Card::Yellow));
        let brutal = FoulFacts {
            slide: true,
            approach_milli: 950,
            ..behind
        };
        assert_eq!(card_for(&brutal, 600, 900), Some(Card::Red));
        let dogso = FoulFacts {
            denial: Denial::Chance,
            ..careless
        };
        assert_eq!(card_for(&dogso, 600, 900), Some(Card::Red));
        // Twin (Law 12, SIM 194): the same denial in his own area, the
        // penalty given, trying to play the ball, is a caution.
        let in_area = FoulFacts {
            denial: Denial::ChanceForTheBall,
            ..dogso
        };
        assert_eq!(card_for(&in_area, 600, 900), Some(Card::Yellow));
    }

    /// Law 11 (P1): beyond the second-last defender and the ball, in their
    /// half, he is offside; level, or behind the ball, or in his own half,
    /// he is not.
    #[test]
    fn offside_is_beyond_the_second_last_and_the_ball() {
        let defenders = [10_400, 9_000, 8_500];
        assert!(offside_position(9_200, 7_000, &defenders, 10_500, 10_500));
        assert!(
            !offside_position(9_000, 7_000, &defenders, 10_500, 10_500),
            "level"
        );
        assert!(
            !offside_position(9_200, 9_500, &defenders, 10_500, 10_500),
            "behind the ball"
        );
        assert!(
            !offside_position(5_000, 3_000, &[5_100, 4_000], 10_500, 10_500),
            "own half"
        );
        // The away side attacks the goal at x = 0.
        assert!(offside_position(
            1_300,
            3_500,
            &[100, 1_500, 2_000],
            0,
            10_500
        ));
    }

    #[test]
    fn a_ball_on_the_pitch_needs_no_restart() {
        assert_eq!(out_of_play((5000, 3000), Side::Home, PITCH), None);
        assert_eq!(
            out_of_play((0, 0), Side::Home, PITCH),
            None,
            "the lines are in"
        );
    }

    #[test]
    fn over_the_touchline_the_other_side_throws_in_where_it_went_out() {
        let r = out_of_play((4000, -30), Side::Home, PITCH).expect("out");
        assert_eq!(
            r,
            Restart {
                side: Side::Away,
                spot: (4000, 0),
                kind: RestartKind::ThrowIn
            }
        );
    }

    #[test]
    fn over_his_own_goal_line_the_defender_gives_a_corner() {
        let r = out_of_play((-20, 2000), Side::Home, PITCH).expect("out");
        assert_eq!(
            r,
            Restart {
                side: Side::Away,
                spot: (0, 0),
                kind: RestartKind::Corner
            }
        );
    }

    #[test]
    fn over_the_goal_line_the_attacker_gives_a_goal_kick() {
        // The whole ball over (Law 10, SIM 200): its centre 5 cm past the line
        // is still in play; 20 cm past, it is out.
        assert_eq!(out_of_play((10_505, 5000), Side::Home, PITCH), None);
        let r = out_of_play((10_520, 5000), Side::Home, PITCH).expect("out");
        assert_eq!(
            r,
            Restart {
                side: Side::Away,
                spot: (9_950, 3_400),
                kind: RestartKind::GoalKick
            }
        );
    }

    #[test]
    fn the_kick_off_is_on_the_centre_spot() {
        assert_eq!(kick_off(Side::Away, PITCH).spot, (5_250, 3_400));
    }
}
