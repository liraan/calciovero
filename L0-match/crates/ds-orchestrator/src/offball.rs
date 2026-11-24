//! The off-ball choice: what a player without the ball does (ADR-049).
//!
//! **One responsibility:** a player reads his scene and chooses his act;
//! the engine applies it. No die (ADR-035): the salience of each act is a
//! deterministic function of what he perceives and of his characteristics
//! (ADR-040), with the weights of `balance/offball.json`.

use ds_core::geometry::distance_squared;
use ds_core::ids::PlayerId;

pub use crate::behaviour::{goes_for_loose_ball, instinct, obey, place_weight};

use crate::behaviour::{soon, space_value};

/// What a player without the ball chooses to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OffBallAct {
    /// Go to and keep his place: with the ball, the place he chose in his
    /// coach's zone (ADR-071); without it, the space he closes in that zone
    /// (ADR-071 §5). The goalkeeper keeps the positioning rule.
    Hold,
    /// Run at the opposing carrier.
    Press,
    /// Try to win the ball from the carrier, who is within reach.
    Challenge,
    /// Run to a loose ball (for the goalkeeper: come out for it).
    GoToBall,
    /// With a teammate on the ball: get free of his marker to receive.
    Offer,
    /// With a teammate on the ball: attack the space behind the last
    /// defender, as far as the line.
    Run,
    /// With an opponent on the ball: mark a man, goal-side of him.
    Mark,
    /// With an opponent on the ball and a teammate pressing him: cover the
    /// space behind the presser, on the carrier's line to our goal (plan
    /// L0-CLOSE 10c).
    Cover,
    /// With an opponent on the ball: step into the passing lane between the
    /// carrier and the man (plan L0-CLOSE 10c).
    Intercept,
}

/// Whether `act` is the second reader's phrase (BEHAVIOURS.md 83-91).
///
/// He doubles up on the carrier or on a loose ball only when another
/// teammate is nearer. The first man's same act is a reflex, available in
/// the tick he sees it; this one is recognised a tick later (ADR-099).
#[must_use]
pub fn second_read(act: OffBallAct, s: &OffBallScene) -> bool {
    match act {
        OffBallAct::Press => s.carrier.is_some_and(|c| c.rank >= 1),
        OffBallAct::Mark => s.to_mark.is_some_and(|m| m.rank >= 1),
        OffBallAct::GoToBall => s.loose_ball.is_some_and(|t| t.rank >= 1),
        _ => false,
    }
}

/// A target the player perceives: how far it is, and how many teammates
/// he sees closer to it than him.
#[derive(Debug, Clone, Copy)]
pub struct Target {
    /// His distance to it, cm.
    pub distance_cm: i64,
    /// How many teammates he sees closer.
    pub rank: u32,
    /// The share of it his nearer teammates leave him, as he believes
    /// (SIM 191, [`left_by_nearer`]): `1` with none nearer.
    pub left: f64,
}

/// The share of a danger his nearer teammates leave him (SIM 191).
///
/// ADR-052 §5: for each teammate he sees nearer to it, the chance that one does not
/// get there in time, read with the same trust and the same believed pace as
/// his own run (`ThoughtBalance::confidence`); the product over them, since
/// any one of them can take it. No rank cut: a third man closes a danger
/// the first two will not reach, and leaves one they will.
#[must_use]
pub fn left_by_nearer(nearer_cm: &[i64]) -> f64 {
    let t = ds_calculator::thought::ThoughtBalance::load();
    nearer_cm
        .iter()
        .map(|d| {
            // A teammate: another man, with the reaction he believes (SIM
            // 197; before, without it, unlike every other reading of a man).
            #[allow(clippy::cast_precision_loss)] // cm on a pitch
            let run = (*d).max(0) as f64;
            1.0 - t.in_time(run, true)
        })
        .product()
}

/// What a player sees when a teammate has the ball.
#[derive(Debug, Clone, Copy)]
pub struct MateBall {
    /// His distance to the carrier, cm.
    pub to_carrier_cm: i64,
    /// Whether an opponent marks him (within the mark radius).
    pub marked: bool,
    /// If he is ahead of the ball and not beyond the last defender: his
    /// distance to that defender's line, cm.
    pub to_line_cm: Option<i64>,
    /// The room behind the last defender, to the goal line, cm.
    pub room_cm: i64,
    /// What each choice would give his side (ADR-052 §5, in units of
    /// keeping the ball): the space he holds, the space he would offer in,
    /// the space on the last line, each as free as he would be there.
    pub values: SpaceValues,
}

/// The value of the spaces an attacker without the ball weighs.
#[derive(Debug, Clone, Copy, Default)]
pub struct SpaceValues {
    /// Holding the place his coach's shape gives him, as free as he would
    /// be there (ADR-059).
    pub hold: f64,
    /// Offering a line of pass where he would go, as free as he would be.
    pub offer: f64,
    /// Attacking the depth: the space on the last line.
    pub run: f64,
    /// His run to that space, cm.
    pub run_cm: i64,
    /// The nearest defender's run to it, cm.
    pub defender_cm: i64,
}

/// The player's place in the team, for the choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Place {
    /// An outfield player.
    Outfield,
    /// The goalkeeper; whether the ball is inside his penalty area.
    Keeper {
        /// The ball is in his area.
        ball_in_area: bool,
    },
}

/// What the player perceives, from where he stands (Plane B, ADR-034).
#[derive(Debug, Clone, Copy)]
pub struct OffBallScene {
    /// Who chooses.
    pub actor: PlayerId,
    /// Outfield player or goalkeeper.
    pub place: Place,
    /// A loose ball he sees, if any.
    pub loose_ball: Option<Target>,
    /// How far the nearest opponent is from that ball, cm (a fact of the
    /// scene, not yet filtered by what he sees).
    pub loose_rival_cm: Option<i64>,
    /// The opposing carrier, if an opponent has the ball.
    pub carrier: Option<Target>,
    /// Whether the carrier is within the duel's reach.
    pub carrier_in_reach: bool,
    /// If a teammate has the ball: what he sees of it.
    pub with_mate: Option<MateBall>,
    /// If an opponent has the ball: the nearest other opponent, to mark.
    pub to_mark: Option<Target>,
    /// What his coach asks of him in this phase (ADR-059 step 4): the act,
    /// if any.
    pub asked: Option<OffBallAct>,
    /// If an opponent has the ball, what each danger is worth to them, in
    /// units of keeping the ball (ADR-052): the carrier's space, the space
    /// of the man to mark, the space he guards in the team's structure.
    pub threats: Option<Threats>,
    /// How well the actor knows the teammate nearest to a loose ball (strato 3):
    /// the intesa that makes him yield it (or not). `[0, 1]`.
    pub partner_intesa: f64,
    /// The ball is dead for a restart: everyone recognises it, so nobody
    /// presses or challenges the kicker before it is in play (Laws 13-17,
    /// ADR-080). It is a fact of what he perceives, not an order from the
    /// engine.
    pub dead_ball: bool,
    /// A pass is on its way (SIM 192): the `carrier` or the `with_mate` of
    /// this scene is the man it goes to, where he meets it, as he reads the
    /// ball's path. The ball comes first for whoever goes to it; the others
    /// read the play that is coming.
    pub carrier_to_come: bool,
}

/// The dangers a defender reads (ADR-052 §5), each the value of a space to
/// the opponents (`read::space_value`).
#[derive(Debug, Clone, Copy)]
pub struct Threats {
    /// The carrier's space.
    pub carrier: f64,
    /// The space of the man he would mark.
    pub man: f64,
    /// The space he guards if he keeps his place in the structure.
    pub held: f64,
    /// The space behind the teammate pressing the carrier, if one is, and
    /// how far he is from covering it, cm (plan L0-CLOSE 10c).
    pub cover: Option<(f64, i64)>,
    /// How far he is from the passing lane to the man he would mark, cm.
    pub lane_cm: Option<i64>,
}

/// The facts the actor uses to weigh the dangers (ADR-100): the engine
/// supplies where he stands and where he sees the others; the value he puts
/// on each space is the actor's own, computed here.
#[derive(Debug, Clone)]
pub struct ThreatFacts {
    /// Where the defender stands.
    pub me: (i32, i32),
    /// Where he sees the opposing carrier.
    pub carrier: (i32, i32),
    /// The man he would mark.
    pub man: Option<(i32, i32)>,
    /// The space he holds.
    pub held: (i32, i32),
    /// The nearest opponent to the space he holds.
    pub held_marker: Option<(i32, i32)>,
    /// The cover point, if a teammate presses the carrier.
    pub cover: Option<(i32, i32)>,
    /// The passing lane to the man he would mark.
    pub lane: Option<(i32, i32)>,
    /// Whether the defender plays for the home side.
    pub home: bool,
    /// Pitch length, cm.
    pub length: i32,
    /// Pitch width, cm.
    pub width: i32,
    /// Their men other than the carrier: the other side of the race.
    pub opponents: Vec<(i32, i32)>,
    /// Our men: this side of the race.
    pub ours: Vec<(i32, i32)>,
    /// How much speed the ground keeps from a rolling pass.
    pub ground_keep: f64,
}

/// How sure a ground pass from `carrier` to `man` reaches one of theirs
/// before one of ours (ADR-052, the same thought as the passer's). The man
/// is a danger as much as a pass can reach him first (OI-117 (A), SIM 134),
/// not as if he already had the ball.
fn reaches(f: &ThreatFacts, carrier: (i32, i32), man: (i32, i32)) -> f64 {
    let thought = ds_calculator::thought::ThoughtBalance::load();
    let kick = ds_calculator::kick::KickBalance::load();
    let imagined = |at: (i32, i32), mate: bool| ds_calculator::thought::Imagined {
        at,
        speed: thought.believed_speed_cm_per_tick,
        reaction: thought.believed_reaction_ticks,
        mate,
    };
    let players: Vec<_> = f
        .opponents
        .iter()
        .filter(|p| **p != carrier)
        .map(|p| imagined(*p, true))
        .chain(f.ours.iter().map(|p| imagined(*p, false)))
        .collect();
    #[allow(clippy::cast_precision_loss)]
    let dist = distance_squared(carrier, man).isqrt() as f64;
    let v0 = kick.weighed_pass_speed(dist, f.ground_keep);
    let forecast = ds_calculator::thought::who_first(
        carrier,
        man,
        (v0, f.ground_keep),
        &players,
        thought.forecast_step_cm,
    );
    match forecast.first {
        Some(first) if players[first].mate => thought.confidence(forecast.margin),
        Some(_) => thought.confidence(-forecast.margin),
        None => 0.0,
    }
}

/// The dangers a defender reads (ADR-052 §5, ADR-100): the value to them of
/// the carrier's space, of the man he would mark, and of the space he guards,
/// as much as an opponent can get into it soon.
#[must_use]
pub fn threats(f: &ThreatFacts) -> Threats {
    let ours = (if f.home { 0 } else { f.length }, f.width / 2);
    let theirs = (f.length - ours.0, ours.1);
    let threat = |p: (i32, i32)| space_value(p, ours, theirs);
    let into = f.held_marker.map_or(0.0, |o| soon(o, f.held));
    let dist = |p: (i32, i32)| distance_squared(f.me, p).isqrt();
    Threats {
        carrier: threat(f.carrier),
        man: f.man.map_or(0.0, |m| threat(m) * reaches(f, f.carrier, m)),
        held: threat(f.held) * into,
        cover: f.cover.map(|p| (threat(p), dist(p))),
        lane_cm: f.lane.map(dist),
    }
}

impl crate::GroundTruth {
    /// The defender's thought (ADR-052 §5): the act that takes away the
    /// most danger. Pressing takes away the carrier's, if he gets there in
    /// time; challenging it, as much as his tackle is good; marking takes
    /// away the man's; holding keeps the space he guards closed. Another
    /// teammate nearer to a danger takes it: the second reads whether to
    /// double up.
    fn defend(&mut self, s: &OffBallScene, th: &Threats, c: Target) -> OffBallAct {
        let l = self.latents_for(s.actor);
        let t = ds_calculator::thought::ThoughtBalance::load();
        let keeper = matches!(s.place, Place::Keeper { .. });
        let may_leave = match s.place {
            Place::Outfield => true,
            Place::Keeper { ball_in_area } => ball_in_area,
        };
        #[allow(clippy::cast_precision_loss)]
        // His own run: he is already deciding, no reaction to wait.
        let in_time = |d: i64| t.in_time(d.max(0) as f64, false);
        // Discipline (plan L0-CLOSE 10c) colours how much the place he holds
        // in the team's shape weighs for him (VISION §3: the psychology
        // interprets).
        let discipline = f64::from(l.psychological.discipline);
        let floor = f64::from(self.offball_balance().discipline_floor_milli) / 1000.0;
        let held = th.held * place_weight(discipline, floor);
        let mut options = vec![(held, OffBallAct::Hold)];
        let mut offer = |score: f64, act: OffBallAct| options.push((score, act));
        // In reach he no longer presses: he goes in, or he jockeys, which is
        // a way of going in (ADR-042). On a dead ball no one goes in: the
        // kick is unchallenged until it is taken (Laws 13-17).
        if !s.dead_ball && !keeper && !s.carrier_in_reach {
            offer(
                th.carrier * in_time(c.distance_cm) * c.left,
                OffBallAct::Press,
            );
        }
        if !keeper {
            if let Some(m) = s.to_mark {
                offer(th.man * in_time(m.distance_cm) * m.left, OffBallAct::Mark);
            }
        }
        if !keeper {
            // Covering: if the presser is beaten (he cannot know: half),
            // the carrier is into the space behind him.
            if let Some((space, d)) = th.cover {
                offer(0.5 * space * in_time(d), OffBallAct::Cover);
            }
            // In the lane he cuts out the ground pass to the man, not the
            // ball over him (he cannot know which: half); goal-side he
            // guards both.
            if let (false, Some(d)) = (s.dead_ball, th.lane_cm) {
                offer(0.5 * th.man * in_time(d), OffBallAct::Intercept);
            }
        }
        if !s.dead_ball && s.carrier_in_reach && may_leave {
            let skill = if keeper {
                l.technical.goalkeeping
            } else {
                l.technical.tackling
            };
            offer(th.carrier * f64::from(skill), OffBallAct::Challenge);
        }
        obey(&options, s.asked, discipline)
    }

    /// How far from his feet `player` reaches a ball passing by, cm
    /// (ADR-051, receive and intercept): the control radius, by his
    /// reactivity.
    #[must_use]
    pub fn ball_reach_cm(&mut self, player: PlayerId, radius_cm: i32) -> i64 {
        let reactivity = self.latents_for(player).physical.reactivity;
        self.offball_balance().ball_reach_cm(radius_cm, reactivity)
    }

    /// The player's off-ball act and the tick its phrase became available
    /// (ADR-049, ADR-099 Form B). The first man's reflex is in his read at
    /// once; the second man's phrase — doubling up, going after the same
    /// loose ball — is there a tick later. The engine writes the tick in
    /// `offball_due`; no choice is made or delayed there.
    #[must_use]
    pub fn choose_off_ball(&mut self, s: &OffBallScene, tick: u64) -> (OffBallAct, u64) {
        let act = self.choose_act(s);
        let available_at = if second_read(act, s) {
            match self.recognition.get(&s.actor) {
                Some((seen, first)) if *seen == act && *first < tick => *first,
                _ => {
                    self.recognition.insert(s.actor, (act, tick));
                    tick.saturating_add(1)
                }
            }
        } else {
            self.recognition.insert(s.actor, (act, tick));
            tick
        };
        (act, available_at)
    }

    /// Whether he gets to a ball before the nearest opponent: his own pace
    /// (movement.json, his max speed), against the opponent at the pace he
    /// believes.
    fn first_to_it(&self, s: &OffBallScene, t: Target, max_speed: f32) -> bool {
        let pace = self.top_speed_base(max_speed);
        #[allow(clippy::cast_precision_loss)] // cm on a pitch
        s.loose_rival_cm.is_none_or(|rival| {
            ds_calculator::thought::ThoughtBalance::load().race(
                t.distance_cm.max(0) as f64,
                pace,
                rival.max(0) as f64,
            ) > 0.5
        })
    }

    /// The most salient act (ADR-049): hold, press, challenge, go to the
    /// ball. Ties go to holding his place.
    fn choose_act(&mut self, s: &OffBallScene) -> OffBallAct {
        let b = self.offball_balance();
        let l = self.latents_for(s.actor);
        let milli = |v: i32| f64::from(v) / 1000.0;
        let reading = f64::from(l.cognitive.game_reading);
        let proximity = |d: i64| {
            #[allow(clippy::cast_precision_loss)]
            let d = d.max(0) as f64;
            1.0 / (1.0 + d / f64::from(b.proximity_ref_cm.max(1)))
        };
        let rank = |r: u32| match r {
            0 => 1.0,
            1 => reading * milli(b.second_reader_milli),
            _ => 0.0,
        };
        let keeper = matches!(s.place, Place::Keeper { .. });
        let may_leave = match s.place {
            Place::Outfield => true,
            Place::Keeper { ball_in_area } => ball_in_area,
        };
        // SIM 192: a pass on its way. Whoever goes to the ball goes first,
        // by the same race as a loose ball; a teammate who yields it does not
        // stand still for it, he reads the play that is coming.
        if let (Some(t), true, true) = (s.loose_ball, may_leave, s.carrier_to_come) {
            let yield_threshold = f64::from(self.offball_balance().intesa_yield_milli) / 1000.0;
            let yields = t.rank >= 1 && s.partner_intesa >= yield_threshold;
            let first = self.first_to_it(s, t, l.physical.max_speed);
            if !yields && goes_for_loose_ball(t.rank, reading, first) {
                return OffBallAct::GoToBall;
            }
        }
        if let (Some(th), Some(c)) = (s.threats, s.carrier) {
            return self.defend(s, &th, c);
        }
        let mut best = (milli(b.hold_salience_milli), OffBallAct::Hold);
        let mut offer = |score: f64, act: OffBallAct| {
            if score > best.0 {
                best = (score, act);
            }
        };
        // The first aim is the ball (ADR-052 note, 2026-11-14): a ball
        // nobody has is worth more than any place, at any distance. On a
        // restart the ball is dead, so it is nobody's to run to.
        if let (Some(t), true, false, false) =
            (s.loose_ball, may_leave, s.dead_ball, s.carrier_to_come)
        {
            // Strato 3 (the call): a loose ball between two teammates of the
            // same side. The nearest (rank 0) always goes. A teammate who
            // knows him (intesa high enough) trusts that he will get there
            // and yields: he covers instead of going for the same ball. A
            // teammate who does not know him may go too (the second reader),
            // as before: two on the same ball, or both staying, is what a
            // pair that has just met does.
            if t.rank >= 1 {
                let yield_threshold = f64::from(self.offball_balance().intesa_yield_milli) / 1000.0;
                if s.partner_intesa >= yield_threshold {
                    return OffBallAct::Hold;
                }
            }
            // The second one's race: his own pace (movement.json, his
            // max speed), against the nearest opponent at the pace he believes.
            let first = self.first_to_it(s, t, l.physical.max_speed);
            if goes_for_loose_ball(t.rank, reading, first) {
                return OffBallAct::GoToBall;
            }
        }
        if let Some(c) = s.carrier {
            if !s.dead_ball && !keeper {
                offer(proximity(c.distance_cm) * rank(c.rank), OffBallAct::Press);
            }
            if !s.dead_ball && s.carrier_in_reach && may_leave {
                let skill = if keeper {
                    l.technical.goalkeeping
                } else {
                    l.technical.tackling
                };
                offer(
                    milli(b.challenge_salience_milli) * instinct(0.5, 0.5, f64::from(skill)),
                    OffBallAct::Challenge,
                );
            }
        }
        if let (Some(t), false) = (s.to_mark, keeper) {
            #[allow(clippy::cast_precision_loss)]
            let near = 1.0 / (1.0 + t.distance_cm.max(0) as f64 / f64::from(b.mark_zone_cm.max(1)));
            offer(
                milli(b.mark_salience_milli) * near * rank(t.rank) * instinct(0.5, 0.5, reading),
                OffBallAct::Mark,
            );
        }
        if let (Some(m), false) = (s.with_mate, keeper) {
            // ADR-052 §5: he occupies the space that gives his side the
            // most, as free as he would be there; on the last line, if he
            // gets there before the defender (his own pace, which he knows,
            // against the one he believes the defender has).
            let thought = ds_calculator::thought::ThoughtBalance::load();
            let spaces = m.values;
            // How much the place in his coach's shape weighs for him: his
            // discipline (ADR-059 §4, the same rule as without the ball).
            let floor = milli(b.discipline_floor_milli);
            let discipline = f64::from(l.psychological.discipline);
            let held = spaces.hold * place_weight(discipline, floor);
            let mut options = vec![(held, OffBallAct::Hold), (spaces.offer, OffBallAct::Offer)];
            if m.to_line_cm.is_some() {
                // His own top speed, as his body has it (movement.json).
                let pace = self.top_speed_base(l.physical.max_speed);
                #[allow(clippy::cast_precision_loss)]
                let run = spaces.run
                    * thought.race(spaces.run_cm as f64, pace, spaces.defender_cm as f64);
                options.push((run, OffBallAct::Run));
            }
            let _ = (proximity, reading);
            return obey(&options, s.asked, discipline);
        }
        best.1
    }
}

/// The facts the engine hands the actor to value the place he would hold
/// (ADR-043 Option B): only positions and the coach's geometry, no choice.
pub struct ClosedPlaceScene<'a> {
    /// Where this player stands.
    pub me: (i32, i32),
    /// The place his zone gives him, from the coach's shape.
    pub anchor: (i32, i32),
    /// Pitch length and width, cm.
    pub pitch: (i32, i32),
    /// `1` attacking to the right, `-1` to the left.
    pub forward: i32,
    /// The declared depth of the block out of possession, cm.
    pub block_depth_cm: i32,
    /// The goal he defends.
    pub our_goal: (i32, i32),
    /// The goal he attacks.
    pub their_goal: (i32, i32),
    /// Where his teammates stand.
    pub ours: &'a [(i32, i32)],
    /// Where the opponents stand.
    pub opponents: &'a [(i32, i32)],
    /// The opposing carrier, if one has the ball.
    pub carrier: Option<(i32, i32)>,
    /// How well he reads the game, in `[0, 1]`.
    pub game_reading: f64,
    /// How tired he is, in `[0, 1]`.
    pub fatigue: f64,
}

/// The place this player would hold if he chose `Hold` (ADR-071 §5).
///
/// It is the *content* of one option, not the act: the engine passes the
/// scene, the actor values the candidates with the shared measures
/// (`behaviour::space_value`, `behaviour::soon`) and his own reading and
/// fatigue, and returns the place. `ds-match` then only stores it for `Hold`
/// and uses it to value that act against the others. Same arithmetic as
/// before this function moved out of the engine: no number, no tick changed.
#[must_use]
#[allow(clippy::cast_precision_loss)] // cm on a pitch
pub fn choose_closed_place(scene: &ClosedPlaceScene<'_>) -> (i32, i32) {
    let ClosedPlaceScene {
        me,
        anchor,
        pitch: (length, width),
        forward,
        block_depth_cm,
        our_goal,
        their_goal,
        ours,
        opponents,
        carrier,
        game_reading,
        fatigue,
    } = *scene;
    let corridor = (width / 5).max(1);
    let line = (block_depth_cm / 3).max(1);
    // The space is his only if no teammate is nearer to it: zones do not
    // overlap, as the men of a real block do not.
    let his = |p: (i32, i32)| {
        let mine = distance_squared(me, p);
        ours.iter()
            .all(|m| *m == me || distance_squared(*m, p) >= mine)
    };
    let danger = |p: (i32, i32)| space_value(p, our_goal, their_goal);
    let soon = |p: (i32, i32)| {
        opponents
            .iter()
            .filter(|o| Some(**o) != carrier)
            .map(|o| soon(*o, p))
            .fold(0.0, f64::max)
    };
    // Never two in the same corridor on the same line.
    let crowded = |p: (i32, i32)| {
        ours.iter()
            .any(|m| *m != me && (m.1 / corridor == p.1 / corridor) && (m.0 - p.0).abs() < line / 2)
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
            let x = (anchor.0 + step * line * forward).clamp(0, length);
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
        let worth = danger(p)
            * if quiet && p.1 / corridor == k0 {
                game_reading.clamp(0.0, 1.0)
            } else {
                soon(p)
            };
        // SIM 176: a tired man covers the space he can still reach. One
        // line of the block is the declared reach a fresh man steps
        // across; at full fatigue even that is beyond him, so he holds
        // the nearer space instead of chasing danger with legs that no
        // longer carry him there. No new number: fatigue is declared
        // in [0, 1] and `line` is the declared depth divided by three.
        let far = (distance_squared(me, p).isqrt() as f64 / f64::from(line.max(1))).min(1.0);
        worth * fatigue.clamp(0.0, 1.0).mul_add(-far, 1.0)
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

/// What a man of the side with the ball has before him to choose his place.
///
/// ADR-071, SIM 152: the facts the engine gives (ADR-043
/// Option B). Moved here with `choose_offer_place` from the engine, where
/// the choice was made (code audit B2).
#[derive(Debug, Clone, Copy)]
pub struct OfferPlaceScene<'a> {
    /// Where he stands.
    pub me: (i32, i32),
    /// His role's zone in the shape.
    pub anchor: (i32, i32),
    /// The man on the ball.
    pub carrier: (i32, i32),
    /// Where the ball is.
    pub ball: (i32, i32),
    /// Whether he is of the back line (it stays behind the ball).
    pub back_line: bool,
    /// The block's depth with the ball, cm.
    pub block_depth: i32,
    /// How far short of the offside line he stops, cm.
    pub onside_margin_cm: i32,
    /// Pitch length and width, cm.
    pub pitch: (i32, i32),
    /// How well he knows his teammates (strato 3).
    pub intesa: f64,
    /// `1` attacking to the right, `-1` to the left.
    pub forward: i32,
    /// The goal line he attacks, cm.
    pub their_goal_x: i32,
    /// Their last line as his side reads it.
    pub last_line: Option<i32>,
    /// Their three lines (ADR-052).
    pub lines: crate::read::Lines,
    /// Where the opponents stand.
    pub opponents: &'a [(i32, i32)],
    /// Where his teammates stand.
    pub ours: &'a [(i32, i32)],
    /// Their velocity this tick, parallel to `ours`.
    pub ours_vel: &'a [(i32, i32)],
}

/// The place a man of the side with the ball holds.
///
/// ADR-071, SIM 152: from his role's zone `anchor` (the shape), the space worth most to
/// his side among his corridor and the two beside it (the pitch in five
/// corridors), on his line and one line ahead or behind (a line, a
/// third of the block's depth with the ball), never beyond the offside
/// line. A space is worth what the game measures (ADR-052): its value
/// to the side, as free as he would be there, by how clean the line
/// from the carrier is and how plausible a pass that far is (ADR-040). Never two in the same corridor on the same
/// line; a man of the back line stays behind the ball. Staying in his
/// zone is his discipline: within it he chooses (ADR-071 §4). `me` is
/// himself, `carrier` the man on the ball, `ball` where it is.
#[allow(clippy::cast_precision_loss, clippy::too_many_lines)]
#[must_use]
pub fn choose_offer_place(
    scene: &OfferPlaceScene<'_>,
    line_misread_cm: i32,
    discipline: f64,
) -> (i32, i32) {
    const PREDICT_TICKS: i32 = 5;
    let OfferPlaceScene {
        me,
        anchor,
        carrier,
        ball,
        back_line,
        block_depth,
        onside_margin_cm,
        pitch: (length, width),
        intesa,
        ..
    } = *scene;
    // OI-98: the offside line is not read perfectly. A poor reader
    // sees it further forward (more room) than it is, so he may offer
    // beyond the real line and be caught offside: how far is his own
    // reading (`GroundTruth::offside_line_misread_cm`, ADR-112).
    let perceived_last = scene.last_line.map(|l| l + scene.forward * line_misread_cm);
    let corridor = (width / 5).max(1);
    let line = (block_depth / 3).max(1);
    let goal = (scene.their_goal_x, width / 2);
    let other = (length - scene.their_goal_x, width / 2);
    let value = |p: (i32, i32)| {
        let free = 1.0
            - scene
                .opponents
                .iter()
                .copied()
                .min_by_key(|o| distance_squared(*o, p))
                .map_or(0.0, |o| crate::read::soon(o, p));
        // Within the reach of a pass from the carrier (the plausible
        // distance of ADR-040): a space he cannot be played into gives
        // his side nothing now.
        let reach = crate::read::distance_plausible(distance_squared(carrier, p).isqrt());
        crate::read::space_value(p, goal, other)
            // ADR-052: a place a pass can reach past their lines is
            // worth what those lines must recover from (the pocket).
            * crate::read::line_break(carrier, p, &scene.lines, scene.forward)
            * free
            * crate::read::line_quality(carrier, p, scene.opponents)
            * reach
    };
    // Never two in the same corridor on the same line. Strato 3: a
    // teammate I know (intesa high enough) is also predicted where he is
    // going, so the space he will occupy counts as crowded for me too.
    let predicted: Vec<(i32, i32)> = if intesa >= 0.5 {
        scene
            .ours
            .iter()
            .zip(scene.ours_vel.iter())
            .filter(|(m, _)| **m != me)
            .map(|(m, v)| (m.0 + v.0 * PREDICT_TICKS, m.1 + v.1 * PREDICT_TICKS))
            .collect()
    } else {
        Vec::new()
    };
    let crowded =
        |p: (i32, i32)| {
            scene.ours.iter().any(|m| {
                *m != me && (m.1 / corridor == p.1 / corridor) && (m.0 - p.0).abs() < line / 2
            }) || predicted
                .iter()
                .any(|m| (m.1 / corridor == p.1 / corridor) && (m.0 - p.0).abs() < line / 2)
        };
    let onside = |x: i32| {
        perceived_last.map_or(x, |l| {
            let edge = l - scene.forward * onside_margin_cm;
            if (x - edge) * scene.forward > 0 {
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
        anchor.0 + line * scene.forward,
        anchor.0 - line * scene.forward,
    ];
    let their_lines: Vec<i32> = [scene.lines.first, scene.lines.middle, scene.lines.last]
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
            let behind_ball = (p.0 - ball.0) * scene.forward <= 0;
            let v = if (back_line && !behind_ball) || crowded(p) {
                0.0
            } else {
                value(p)
            };
            options.push((v, p));
        }
    }
    // The zone of his role, first: the coach's ask (ADR-059 §4); kept onside
    // like every other place he weighs (SIM 203, T24, Law 11: before, the
    // place of his role was taken as it was, beyond the line, and near their
    // goal it was worth most, so the man holding it stood offside).
    let anchor = (onside(anchor.0).clamp(0, length), anchor.1);
    let zone = (value(anchor), anchor);
    let best = options
        .iter()
        .copied()
        .fold(zone, |b, o| if o.0 > b.0 { o } else { b });
    // The coach's ask against his own best, as for every act he is asked
    // (`obey`, ADR-059 §4): he holds his zone if it is worth at least
    // `1 - discipline` of his best (SIM 203: before, only a tie kept it,
    // and with his zone onside the central spaces always won, so a team
    // asked to play wide played narrower than one asked to play narrow).
    if zone.0 >= best.0 * (1.0 - discipline.clamp(0.0, 1.0)) {
        zone.1
    } else {
        best.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GroundTruth;
    use ds_core::ids::MatchId;

    fn scene() -> OffBallScene {
        OffBallScene {
            actor: PlayerId::new(6, 0),
            place: Place::Outfield,
            loose_ball: None,
            loose_rival_cm: None,
            carrier: None,
            carrier_in_reach: false,
            with_mate: None,
            to_mark: None,
            asked: None,
            threats: None,
            partner_intesa: 0.0,
            dead_ball: false,
            carrier_to_come: false,
        }
    }

    #[allow(clippy::unnecessary_wraps)] // the scene's fields are options
    const fn at(distance_cm: i64, rank: u32) -> Option<Target> {
        Some(Target {
            distance_cm,
            rank,
            // Teammates nearer and already on it: nothing left to him.
            left: if rank == 0 { 1.0 } else { 0.0 },
        })
    }

    fn facts(ours: Vec<(i32, i32)>, man: Option<(i32, i32)>) -> ThreatFacts {
        ThreatFacts {
            opponents: vec![(5000, 3400), (6500, 3400)],
            me: (4500, 3400),
            carrier: (5000, 3400),
            man,
            held: (4200, 3400),
            held_marker: Some((4600, 3400)),
            cover: None,
            lane: None,
            home: true,
            length: 10500,
            width: 6800,
            ours,
            ground_keep: 0.957,
        }
    }

    /// Twin test (OI-117 (A), SIM 134, moved here by ADR-100): the same man,
    /// the same carrier; only a defender standing on the line between them
    /// changes. With the line open a pass reaches him and he is a danger;
    /// with the line shut he is far less of one.
    #[test]
    fn a_man_no_pass_can_reach_is_less_of_a_danger() {
        let carrier = (5000, 3400);
        let man = (6500, 3400);
        let open = reaches(&facts(vec![(1000, 1000)], Some(man)), carrier, man);
        let shut = reaches(&facts(vec![(5750, 3400)], Some(man)), carrier, man);
        assert!(open > 0.5, "open {open}");
        assert!(shut < open / 2.0, "shut {shut} open {open}");
    }

    /// ADR-100: the danger the actor weighs comes from what he sees of the
    /// man, not from a value the engine supplies. Same scene, one man
    /// perceived nearer, and the value changes; the same perception twice
    /// gives the same value.
    #[test]
    fn the_danger_is_the_actors_own_reading() {
        let near = threats(&facts(vec![(1000, 1000)], Some((6000, 3400))));
        let far = threats(&facts(vec![(1000, 1000)], Some((6500, 3400))));
        assert!(near.man > far.man, "near {} far {}", near.man, far.man);
        let again = threats(&facts(vec![(1000, 1000)], Some((6500, 3400))));
        assert_eq!(far.man, again.man);
    }

    fn choose(s: &OffBallScene) -> OffBallAct {
        GroundTruth::new(42, MatchId::new(1, 0))
            .choose_off_ball(s, 0)
            .0
    }

    /// Twin test (BEHAVIOURS.md, l'istinto): the weakest player keeps the
    /// base, the strongest gets base plus gain. A low characteristic cannot
    /// take the act away; it only makes it worse.
    #[test]
    fn instinct_is_in_everyone_and_better_in_the_gifted() {
        assert!((instinct(0.5, 0.5, 0.0) - 0.5).abs() < 1e-12);
        assert!((instinct(0.5, 0.5, 1.0) - 1.0).abs() < 1e-12);
        assert!(instinct(0.5, 0.5, 1.0) > instinct(0.5, 0.5, 0.0));
    }

    /// ADR-099 Form B: the same phrase is in the first man's read at once
    /// and in the second man's one tick later, before the choice. Once it
    /// is in his read it stays there: the next tick does not add a second
    /// wait after the argmax.
    #[test]
    fn the_second_man_gets_the_phrase_one_tick_later() {
        let mut g = GroundTruth::new(42, MatchId::new(1, 0));
        let second = (1..=200_u32)
            .map(|slot| PlayerId::new(slot, 0))
            .find(|&actor| {
                let s = OffBallScene {
                    actor,
                    loose_ball: at(1000, 1),
                    ..scene()
                };
                GroundTruth::new(42, MatchId::new(1, 0))
                    .choose_off_ball(&s, 0)
                    .0
                    == OffBallAct::GoToBall
            })
            .expect("a second reader of a loose ball");
        let first = OffBallScene {
            actor: second,
            loose_ball: at(1000, 0),
            ..scene()
        };
        let later = OffBallScene {
            loose_ball: at(1000, 1),
            ..first
        };
        assert_eq!(g.choose_off_ball(&first, 10), (OffBallAct::GoToBall, 10));
        // A different player: the first man's recognition is not his.
        let mut g = GroundTruth::new(42, MatchId::new(1, 0));
        assert_eq!(g.choose_off_ball(&later, 20), (OffBallAct::GoToBall, 21));
        assert_eq!(g.choose_off_ball(&later, 21), (OffBallAct::GoToBall, 20));
    }

    /// The first aim is the ball (ADR-052 note, 2026-11-14): the nearest of
    /// his side goes for a loose ball however far it is (before, beyond
    /// about 35 m he held his place) (P1).
    #[test]
    fn the_nearest_goes_for_a_loose_ball_at_any_distance() {
        let far = OffBallScene {
            loose_ball: at(6000, 0),
            ..scene()
        };
        assert_eq!(choose(&far), OffBallAct::GoToBall);
    }

    /// Twin test (ADR-059 step 4): two players equal but for discipline,
    /// the same options, the coach asking for the run that his own reading
    /// values at half of his best: the disciplined runs, the other does
    /// what he reads; asked for what is not on, both do what they read.
    #[test]
    fn the_disciplined_does_what_he_is_asked() {
        let options = [
            (0.4, OffBallAct::Hold),
            (1.0, OffBallAct::Offer),
            (0.5, OffBallAct::Run),
        ];
        let asked = Some(OffBallAct::Run);
        assert_eq!(obey(&options, asked, 0.9), OffBallAct::Run);
        assert_eq!(obey(&options, asked, 0.2), OffBallAct::Offer);
        assert_eq!(obey(&options, None, 0.9), OffBallAct::Offer);
        assert_eq!(
            obey(&options, Some(OffBallAct::Mark), 1.0),
            OffBallAct::Offer
        );
    }

    /// Twin test (ADR-059 §4): two players equal but for discipline, the
    /// same place in the coach's shape and the same space of their own:
    /// the disciplined one takes up the coach's place, the other goes where
    /// he sees the space.
    #[test]
    fn the_more_disciplined_follows_the_coachs_shape() {
        let (place, own, floor) = (1.0, 0.8, 0.5);
        assert!(
            place * place_weight(0.9, floor) > own,
            "disciplined: the place"
        );
        assert!(place * place_weight(0.2, floor) < own, "his own reading");
        assert!((place_weight(1.0, floor) - 1.0).abs() < 1e-12);
        assert!((place_weight(0.0, floor) - floor).abs() < 1e-12);
    }

    /// Twin test (ADR-051 §3): two players second nearest to a loose ball,
    /// equal but for their reading of the play: the better reader goes too,
    /// the poorer one keeps his place.
    #[test]
    fn the_better_reader_goes_second_for_a_loose_ball() {
        assert!(goes_for_loose_ball(1, 0.8, true));
        assert!(!goes_for_loose_ball(1, 0.3, true));
        assert!(
            !goes_for_loose_ball(1, 0.8, false),
            "he would not get there first"
        );
        assert!(goes_for_loose_ball(0, 0.0, false));
        assert!(!goes_for_loose_ball(2, 1.0, true));
    }

    /// Twin test (ADR-081, SIM 164, C): the second man, the same rival at
    /// the drop point, the same reading — only his distance changes. Near,
    /// his own pace gets him there first and he goes; far, he holds his
    /// place and leaves the high ball to the nearest man.
    #[test]
    fn the_second_reaches_the_near_ball_and_holds_for_the_far_one() {
        let second = |distance_cm| OffBallScene {
            loose_ball: at(distance_cm, 1),
            loose_rival_cm: Some(1500),
            ..scene()
        };
        assert_eq!(choose(&second(300)), OffBallAct::GoToBall);
        assert_eq!(choose(&second(8000)), OffBallAct::Hold);
    }

    /// The nearest player goes for a loose ball close to him; one with two
    /// teammates closer holds his place (P1, ADR-049).
    #[test]
    fn the_nearest_goes_the_others_hold() {
        let near = OffBallScene {
            loose_ball: at(500, 0),
            ..scene()
        };
        assert_eq!(choose(&near), OffBallAct::GoToBall);
        let third = OffBallScene {
            loose_ball: at(500, 2),
            ..near
        };
        assert_eq!(choose(&third), OffBallAct::Hold);
    }

    /// Twin test (strato 3, the call): the same second-nearest player, the
    /// same loose ball he could get to first; only the intesa with the
    /// teammate who goes changes. Knowing him, he yields and holds (he
    /// covers); not knowing him, he goes for the same ball.
    #[test]
    fn a_teammate_he_knows_yields_the_loose_ball() {
        let knows = OffBallScene {
            loose_ball: at(500, 1),
            partner_intesa: 0.8,
            ..scene()
        };
        assert_eq!(choose(&knows), OffBallAct::Hold);
        let strangers = OffBallScene {
            loose_ball: at(500, 1),
            partner_intesa: 0.1,
            ..scene()
        };
        // With low intesa the second reader may still go (reading above 0.5).
        assert!(
            choose(&strangers) == OffBallAct::GoToBall || choose(&strangers) == OffBallAct::Hold
        );
    }

    /// In reach of the carrier, an average tackler challenges; far from him,
    /// he holds.
    #[test]
    fn in_reach_he_challenges() {
        let reach = OffBallScene {
            carrier: at(80, 0),
            carrier_in_reach: true,
            ..scene()
        };
        assert_eq!(choose(&reach), OffBallAct::Challenge);
        let far = OffBallScene {
            carrier: at(5000, 0),
            ..scene()
        };
        assert_eq!(choose(&far), OffBallAct::Hold);
    }

    #[allow(clippy::unnecessary_wraps)] // the scene's field is an option
    fn mate(marked: bool, to_line_cm: Option<i64>, room_cm: i64) -> Option<MateBall> {
        // Marked: his space is worth little, the free one he would offer in
        // more; with room behind the line, the depth is worth the most.
        Some(MateBall {
            to_carrier_cm: 1000,
            marked,
            to_line_cm,
            room_cm,
            values: SpaceValues {
                hold: if marked { 0.5 } else { 2.0 },
                offer: if marked { 1.8 } else { 1.0 },
                run: if room_cm > 1000 { 6.0 } else { 0.0 },
                run_cm: to_line_cm.unwrap_or(0),
                defender_cm: 1200,
            },
        })
    }

    /// With a teammate on the ball, a marked player near him gets free; a
    /// free one far from the play holds (P1, ADR-051).
    #[test]
    fn a_marked_player_near_the_ball_offers() {
        let marked = OffBallScene {
            with_mate: mate(true, None, 0),
            ..scene()
        };
        assert_eq!(choose(&marked), OffBallAct::Offer);
        let free = OffBallScene {
            with_mate: mate(false, None, 0),
            ..scene()
        };
        assert_eq!(choose(&free), OffBallAct::Hold);
    }

    /// Ahead of the ball, near the last defender, with room behind him,
    /// some players attack the space (those with pace, the twin test is in
    /// `ds-calculator::offball`); with no room nobody does (P1, ADR-051).
    #[test]
    fn with_room_behind_the_line_some_run() {
        let runs = |room_cm| {
            (1..=10)
                .filter(|slot| {
                    let s = OffBallScene {
                        actor: PlayerId::new(*slot, 0),
                        with_mate: mate(false, Some(300), room_cm),
                        ..scene()
                    };
                    choose(&s) == OffBallAct::Run
                })
                .count()
        };
        assert!(runs(3000) > 0, "with room, somebody runs");
        assert_eq!(runs(100), 0, "with no room, nobody runs");
    }

    /// With an opponent on the ball, the defender nearest to another
    /// opponent marks him; one with two teammates closer holds (P1,
    /// ADR-051).
    #[test]
    fn the_nearest_marks_the_man() {
        let near = OffBallScene {
            to_mark: at(300, 0),
            ..scene()
        };
        assert_eq!(choose(&near), OffBallAct::Mark);
        let third = OffBallScene {
            to_mark: at(300, 2),
            ..scene()
        };
        assert_eq!(choose(&third), OffBallAct::Hold);
    }

    fn defending(th: Threats) -> OffBallScene {
        OffBallScene {
            carrier: at(2500, 1),
            threats: Some(th),
            ..scene()
        }
    }

    const QUIET: Threats = Threats {
        carrier: 1.0,
        man: 0.0,
        held: 0.2,
        cover: None,
        lane_cm: None,
    };

    /// Twin test (ADR-080): the same player, near the same ball, presses and
    /// challenges when it is live; when he perceives a restart nobody can
    /// take, he keeps his place, even if the coach asks him to press. The
    /// same is true of a ball nobody has: a dead ball is not there to chase.
    #[test]
    fn nobody_chases_a_dead_ball() {
        let pressing = OffBallScene {
            carrier: at(400, 0),
            threats: Some(QUIET),
            ..scene()
        };
        assert_eq!(choose(&pressing), OffBallAct::Press);
        let dead = OffBallScene {
            dead_ball: true,
            asked: Some(OffBallAct::Press),
            ..pressing
        };
        assert_eq!(choose(&dead), OffBallAct::Hold, "even when asked");

        let tackling = OffBallScene {
            carrier: at(100, 0),
            carrier_in_reach: true,
            threats: Some(Threats { held: 0.0, ..QUIET }),
            ..scene()
        };
        assert_eq!(choose(&tackling), OffBallAct::Challenge);
        let dead = OffBallScene {
            dead_ball: true,
            ..tackling
        };
        assert_eq!(choose(&dead), OffBallAct::Hold);

        let loose = OffBallScene {
            loose_ball: at(500, 0),
            ..scene()
        };
        assert_eq!(choose(&loose), OffBallAct::GoToBall);
        let dead = OffBallScene {
            dead_ball: true,
            ..loose
        };
        assert_eq!(choose(&dead), OffBallAct::Hold);
    }

    /// With a teammate pressing the carrier, a defender near the space
    /// behind him covers it (P1, plan L0-CLOSE 10c); with nobody pressing
    /// there is nothing to cover.
    #[test]
    fn behind_the_presser_he_covers() {
        let cover = defending(Threats {
            cover: Some((4.0, 300)),
            ..QUIET
        });
        assert_eq!(choose(&cover), OffBallAct::Cover);
        assert_ne!(choose(&defending(QUIET)), OffBallAct::Cover);
    }

    /// Twin test (SIM 192): the same defender, the same man the pass goes
    /// to, 6 m from where he meets it. Before the ball arrives he already
    /// reads the play that is coming and closes him; when he does not see
    /// a pass coming (no carrier yet in his scene), he has only his place.
    #[test]
    fn on_the_pass_the_defender_steps_to_the_man_it_goes_to() {
        let coming = OffBallScene {
            carrier: at(600, 0),
            threats: Some(QUIET),
            carrier_to_come: true,
            ..scene()
        };
        assert_eq!(choose(&coming), OffBallAct::Press);
        let unseen = OffBallScene {
            carrier: None,
            threats: None,
            ..coming
        };
        assert_eq!(choose(&unseen), OffBallAct::Hold);
        // The ball comes first: the man the pass is in reach of goes to it.
        let cut = OffBallScene {
            loose_ball: at(200, 0),
            ..coming
        };
        assert_eq!(choose(&cut), OffBallAct::GoToBall);
    }

    /// Twin test (SIM 191): the same second man, the same carrier 6 m from
    /// him, the same danger; only where the first teammate is changes. When
    /// the first is on the carrier, he leaves it to him and holds; when the
    /// first is barely nearer and may not get there in time, the danger is
    /// still his too and he doubles up. Before, the second weighed it by a
    /// flat half of his reading and the third not at all.
    #[test]
    fn the_second_man_doubles_up_only_when_the_first_may_not_get_there() {
        let second = |first_cm: i64| OffBallScene {
            carrier: Some(Target {
                distance_cm: 600,
                rank: 1,
                left: left_by_nearer(&[first_cm]),
            }),
            threats: Some(QUIET),
            ..scene()
        };
        assert_eq!(choose(&second(100)), OffBallAct::Hold);
        assert_eq!(choose(&second(550)), OffBallAct::Press);
        // What his mates leave him falls with each one nearer and in time.
        assert!((left_by_nearer(&[]) - 1.0).abs() < 1e-12);
        assert!(left_by_nearer(&[1000]) > left_by_nearer(&[1000, 1000]));
        assert!(left_by_nearer(&[100]) < left_by_nearer(&[1500]));
    }

    /// Near the passing lane to a dangerous man and far from his back, he
    /// steps into the lane (P1, plan L0-CLOSE 10c).
    #[test]
    fn near_the_lane_he_steps_into_it() {
        let lane = OffBallScene {
            to_mark: at(2500, 0),
            ..defending(Threats {
                man: 4.0,
                lane_cm: Some(200),
                ..QUIET
            })
        };
        assert_eq!(choose(&lane), OffBallAct::Intercept);
    }

    /// Twin test (plan L0-CLOSE 10c, discipline): the same scene, the
    /// carrier near and his place as dangerous; the most disciplined keeps
    /// his place, the least leaves it to press.
    #[test]
    fn the_disciplined_player_keeps_his_place() {
        let mut g = GroundTruth::new(42, MatchId::new(1, 0));
        let (mut weak, mut strong) = ((0u32, 1.0f32), (0u32, 0.0f32));
        for slot in 1..=200u32 {
            let d = g
                .latents_for(PlayerId::new(slot, 0))
                .psychological
                .discipline;
            if d < weak.1 {
                weak = (slot, d);
            }
            if d > strong.1 {
                strong = (slot, d);
            }
        }
        let s = |slot: u32| OffBallScene {
            actor: PlayerId::new(slot, 0),
            carrier: at(400, 0),
            threats: Some(Threats {
                carrier: 1.0,
                held: 1.0,
                ..QUIET
            }),
            ..scene()
        };
        assert_eq!(g.choose_off_ball(&s(strong.0), 0).0, OffBallAct::Hold);
        assert_eq!(g.choose_off_ball(&s(weak.0), 0).0, OffBallAct::Press);
    }

    /// The goalkeeper does not press, and comes out only inside his area.
    #[test]
    fn the_keeper_stays_in_his_area() {
        let gk = OffBallScene {
            actor: PlayerId::new(0, 0),
            place: Place::Keeper {
                ball_in_area: false,
            },
            ..scene()
        };
        let press = OffBallScene {
            carrier: at(300, 0),
            ..gk
        };
        assert_eq!(choose(&press), OffBallAct::Hold);
        let out = OffBallScene {
            loose_ball: at(600, 0),
            ..gk
        };
        assert_eq!(choose(&out), OffBallAct::Hold, "outside his area");
        let inside = OffBallScene {
            place: Place::Keeper { ball_in_area: true },
            ..out
        };
        assert_eq!(choose(&inside), OffBallAct::GoToBall);
    }
}
