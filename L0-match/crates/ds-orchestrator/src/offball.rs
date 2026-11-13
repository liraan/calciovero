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
    /// The opposing carrier, if an opponent has the ball.
    pub carrier: Option<Target>,
    /// Whether the carrier is within the duel's reach.
    pub carrier_in_reach: bool,
    /// If a teammate has the ball: what he sees of it.
    pub with_mate: Option<MateBall>,
}

impl crate::GroundTruth {
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
        let mut best = (milli(b.hold_salience_milli), OffBallAct::Hold);
        let mut offer = |score: f64, act: OffBallAct| {
            if score > best.0 {
                best = (score, act);
            }
        };
        if let (Some(t), true) = (s.loose_ball, may_leave) {
            offer(
                proximity(t.distance_cm) * rank(t.rank),
                OffBallAct::GoToBall,
            );
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
        if let (Some(m), false) = (s.with_mate, keeper) {
            let half = 0.5f64;
            if m.marked {
                offer(
                    milli(b.offer_salience_milli)
                        * proximity(m.to_carrier_cm)
                        * half.mul_add(reading, half),
                    OffBallAct::Offer,
                );
            }
            if let Some(d) = m.to_line_cm {
                offer(
                    b.run(d, m.room_cm, l.physical.max_speed, l.cognitive.game_reading),
                    OffBallAct::Run,
                );
            }
        }
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
            carrier: None,
            carrier_in_reach: false,
            with_mate: None,
        }
    }

    #[allow(clippy::unnecessary_wraps)] // the scene's fields are options
    const fn at(distance_cm: i64, rank: u32) -> Option<Target> {
        Some(Target { distance_cm, rank })
    }

    fn choose(s: &OffBallScene) -> OffBallAct {
        GroundTruth::new(42, MatchId::new(1, 0)).choose_off_ball(s)
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
        Some(MateBall {
            to_carrier_cm: 1000,
            marked,
            to_line_cm,
            room_cm,
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
