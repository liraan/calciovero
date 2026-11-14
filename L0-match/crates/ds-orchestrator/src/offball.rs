//! The off-ball choice: what a player without the ball does (ADR-049).
//!
//! **One responsibility:** a player reads his scene and chooses his act;
//! the engine applies it. No die (ADR-035): the salience of each act is a
//! deterministic function of what he perceives and of his characteristics
//! (ADR-040), with the weights of `balance/offball.json`.

use ds_core::ids::PlayerId;

/// What a player without the ball chooses to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OffBallAct {
    /// Go to and keep his place (the block's place for an outfield player,
    /// the positioning rule for the goalkeeper).
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

/// A target the player perceives: how far it is, and how many teammates
/// he sees closer to it than him.
#[derive(Debug, Clone, Copy)]
pub struct Target {
    /// His distance to it, cm.
    pub distance_cm: i64,
    /// How many teammates he sees closer.
    pub rank: u32,
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
    /// Holding his space, as free as he is now.
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
    /// If an opponent has the ball, what each danger is worth to them, in
    /// units of keeping the ball (ADR-052): the carrier's space, the space
    /// of the man to mark, the space he guards in the team's structure.
    pub threats: Option<Threats>,
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

impl crate::GroundTruth {
    /// The defender's thought (ADR-052 §5): the act that takes away the
    /// most danger. Pressing takes away the carrier's, if he gets there in
    /// time; challenging it, as much as his tackle is good; marking takes
    /// away the man's; holding keeps the space he guards closed. Another
    /// teammate nearer to a danger takes it: the second reads whether to
    /// double up.
    fn defend(&mut self, s: &OffBallScene, th: &Threats, c: Target, reading: f64) -> OffBallAct {
        let l = self.latents_for(s.actor);
        let t = ds_calculator::thought::ThoughtBalance::load();
        let keeper = matches!(s.place, Place::Keeper { .. });
        let may_leave = match s.place {
            Place::Outfield => true,
            Place::Keeper { ball_in_area } => ball_in_area,
        };
        let rank = |r: u32| match r {
            0 => 1.0,
            1 => reading * 0.5,
            _ => 0.0,
        };
        #[allow(clippy::cast_precision_loss)]
        let in_time = |d: i64| {
            let run = d.max(0) as f64 / t.believed_speed_cm_per_tick.max(1.0);
            t.confidence(t.trust_ticks - run)
        };
        // Discipline (plan L0-CLOSE 10c) colours how much the place he holds
        // in the team's shape weighs for him (VISION §3: the psychology
        // interprets).
        let discipline = f64::from(l.psychological.discipline);
        let floor = f64::from(self.offball_balance().discipline_floor_milli) / 1000.0;
        let held = th.held * (1.0 - floor).mul_add(discipline, floor);
        let mut best = (held, OffBallAct::Hold);
        let mut offer = |score: f64, act: OffBallAct| {
            if score > best.0 {
                best = (score, act);
            }
        };
        // In reach he no longer presses: he goes in, or he jockeys, which is
        // a way of going in (ADR-042).
        if !keeper && !s.carrier_in_reach {
            offer(
                th.carrier * in_time(c.distance_cm) * rank(c.rank),
                OffBallAct::Press,
            );
        }
        if !keeper {
            if let Some(m) = s.to_mark {
                offer(
                    th.man * in_time(m.distance_cm) * rank(m.rank),
                    OffBallAct::Mark,
                );
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
            if let Some(d) = th.lane_cm {
                offer(0.5 * th.man * in_time(d), OffBallAct::Intercept);
            }
        }
        if s.carrier_in_reach && may_leave {
            let skill = if keeper {
                l.technical.goalkeeping
            } else {
                l.technical.tackling
            };
            offer(th.carrier * f64::from(skill), OffBallAct::Challenge);
        }
        best.1
    }

    /// How far from his feet `player` reaches a ball passing by, cm
    /// (ADR-051, receive and intercept): the control radius, by his
    /// reactivity.
    #[must_use]
    pub fn ball_reach_cm(&mut self, player: PlayerId, radius_cm: i32) -> i64 {
        let reactivity = self.latents_for(player).physical.reactivity;
        self.offball_balance().ball_reach_cm(radius_cm, reactivity)
    }

    /// The player's off-ball act (ADR-049): the most salient of hold, press,
    /// challenge and go to the ball. Ties go to holding his place.
    #[must_use]
    pub fn choose_off_ball(&mut self, s: &OffBallScene) -> OffBallAct {
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
        if let (Some(th), Some(c)) = (s.threats, s.carrier) {
            return self.defend(s, &th, c, reading);
        }
        let mut best = (milli(b.hold_salience_milli), OffBallAct::Hold);
        let mut offer = |score: f64, act: OffBallAct| {
            if score > best.0 {
                best = (score, act);
            }
        };
        // The first aim is the ball (ADR-052 note, 2026-11-14): a ball
        // nobody has is worth more than any place, at any distance.
        if let (Some(t), true) = (s.loose_ball, may_leave) {
            // The second one's race: his own pace (movement.json, his
            // max speed), against the nearest opponent at the pace he believes.
            let mv = &self.movement_balance;
            let pace = (mv.top_speed_max_cm_per_tick - mv.top_speed_min_cm_per_tick).mul_add(
                f64::from(l.physical.max_speed),
                mv.top_speed_min_cm_per_tick,
            );
            #[allow(clippy::cast_precision_loss)] // cm on a pitch
            let first = s.loose_rival_cm.is_none_or(|rival| {
                ds_calculator::thought::ThoughtBalance::load().race(
                    t.distance_cm.max(0) as f64,
                    pace,
                    rival.max(0) as f64,
                ) > 0.5
            });
            if goes_for_loose_ball(t.rank, reading, first) {
                return OffBallAct::GoToBall;
            }
        }
        if let Some(c) = s.carrier {
            if !keeper {
                offer(proximity(c.distance_cm) * rank(c.rank), OffBallAct::Press);
            }
            if s.carrier_in_reach && may_leave {
                let skill = if keeper {
                    l.technical.goalkeeping
                } else {
                    l.technical.tackling
                };
                offer(
                    milli(b.challenge_salience_milli) * 0.5f64.mul_add(f64::from(skill), 0.5),
                    OffBallAct::Challenge,
                );
            }
        }
        if let (Some(t), false) = (s.to_mark, keeper) {
            #[allow(clippy::cast_precision_loss)]
            let near = 1.0 / (1.0 + t.distance_cm.max(0) as f64 / f64::from(b.mark_zone_cm.max(1)));
            offer(
                milli(b.mark_salience_milli) * near * rank(t.rank) * 0.5f64.mul_add(reading, 0.5),
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
            let mut best = (spaces.hold, OffBallAct::Hold);
            if spaces.offer > best.0 {
                best = (spaces.offer, OffBallAct::Offer);
            }
            if m.to_line_cm.is_some() {
                // His own top speed, as his body has it (movement.json).
                let mv = &self.movement_balance;
                let pace = (mv.top_speed_max_cm_per_tick - mv.top_speed_min_cm_per_tick).mul_add(
                    f64::from(l.physical.max_speed),
                    mv.top_speed_min_cm_per_tick,
                );
                #[allow(clippy::cast_precision_loss)]
                let run = spaces.run
                    * thought.race(spaces.run_cm as f64, pace, spaces.defender_cm as f64);
                if run > best.0 {
                    best = (run, OffBallAct::Run);
                }
            }
            let _ = (proximity, reading);
            return best.1;
        }
        best.1
    }
}

/// Whether a player goes for a ball nobody has.
///
/// ADR-052 note, 2026-11-14: the first aim of every player is the ball, and
/// how he seeks it is his. The nearest of his side (`rank` 0) always goes,
/// at any distance: the ball is the whole possession, worth more than any
/// place. The next one (`rank` 1) goes too if he reads the play better than
/// an average player (`game_reading` above the middle of its scale), so he
/// sees that the first may not win it, and if he would get there before the
/// nearest opponent (`first`: his own race, his pace). The others keep
/// their place.
#[must_use]
pub const fn goes_for_loose_ball(rank: u32, game_reading: f64, first: bool) -> bool {
    match rank {
        0 => true,
        1 => game_reading > 0.5 && first,
        _ => false,
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
            threats: None,
        }
    }

    #[allow(clippy::unnecessary_wraps)] // the scene's fields are options
    const fn at(distance_cm: i64, rank: u32) -> Option<Target> {
        Some(Target { distance_cm, rank })
    }

    fn choose(s: &OffBallScene) -> OffBallAct {
        GroundTruth::new(42, MatchId::new(1, 0)).choose_off_ball(s)
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
        assert_eq!(g.choose_off_ball(&s(strong.0)), OffBallAct::Hold);
        assert_eq!(g.choose_off_ball(&s(weak.0)), OffBallAct::Press);
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
