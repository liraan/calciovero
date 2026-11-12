//! Internal facts emitted by the engine, classified into `MatchEvent`s by
//! the narrator (T1, T2 in `SESSION-STATE.md`).
//!
//! A fact is Plane A -> Plane B: it carries the cause (who, where, under
//! what condition), not the narrative. It is internal to `ds-match`:
//! not part of `MatchResult`, not serialised, not in `ds-stats`.
//! A new fact species is a `SIM_VERSION` bump if it changes which events
//! are emitted (L0-match/CHANGELOG-SIM.md, "When to bump", point 2).

use ds_core::ids::{ActorId, PlayerId};
use ds_orchestrator::ShotType;
use ds_stats::EventType;

/// A fact the engine produces in a tick. The narrator classifies it into
/// zero or more `MatchEvent`s.
#[derive(Debug, Clone)]
pub enum Fact {
    /// Two agents contest the ball. Emits `Duel` (always) and `Tackle`
    /// (when the defender wins).
    Contact {
        attacker: PlayerId,
        defender: PlayerId,
        winner: PlayerId,
    },
    /// An action sent the ball along a trajectory.
    Trajectory {
        actor: PlayerId,
        kind: TrajectoryKind,
    },
    /// The ball crossed the goal line.
    GoalLineCrossing {
        actor: Option<ActorId>,
        point: (i32, i32, i32),
    },
    /// A coverage relation started or ended between the carrier and an
    /// opponent (MD-04). Classified as `Pressure`: an opponent entering
    /// the carrier's coverage is a pressure event; an exit is not an event
    /// (the pressure ended, nothing new started). Emitted at the change,
    /// not at every tick: `Pressure` is a relation, not a state.
    RelationEnterExit {
        /// The carrier (the player under coverage).
        carrier: PlayerId,
        /// The opponent who entered or exited the coverage.
        opponent: PlayerId,
        /// `true` = entered the coverage, `false` = exited.
        entered: bool,
    },
}

/// The kind of trajectory a `Fact::Trajectory` carries.
#[derive(Debug, Clone, Copy)]
pub enum TrajectoryKind {
    /// A pass. `success` is whether it was completed.
    Pass { success: bool },
    /// A shot. The payload is what F.3-b3 requires (see
    /// `process/analysis/2026-11-04-match-stats-and-traits.md` §3).
    Shot {
        on_target: bool,
        from: (i32, i32, i32),
        to: (i32, i32, i32),
        initial_velocity: (i16, i16, i16),
        shot_type: ShotType,
    },
}

/// One classified event.
#[derive(Debug, Clone)]
pub struct Classified {
    pub actor: Option<ActorId>,
    pub event_type: EventType,
    pub fields: serde_json::Value,
}

/// Classifies a fact into events, in emission order.
pub fn classify(fact: &Fact) -> Vec<Classified> {
    match fact {
        Fact::Contact {
            attacker,
            defender,
            winner,
        } => {
            let mut out = Vec::with_capacity(2);
            out.push(Classified {
                actor: Some(ActorId::Player(*attacker)),
                event_type: EventType::Duel,
                fields: serde_json::json!({ "winner_slot": winner.slot() }),
            });
            if winner != attacker {
                out.push(Classified {
                    actor: Some(ActorId::Player(*defender)),
                    event_type: EventType::Tackle,
                    fields: serde_json::json!({
                        "from": attacker.slot(),
                        "to": defender.slot(),
                    }),
                });
            }
            out
        }
        Fact::Trajectory { actor, kind } => match kind {
            TrajectoryKind::Pass { success } => vec![Classified {
                actor: Some(ActorId::Player(*actor)),
                event_type: EventType::Pass,
                fields: serde_json::json!({ "complete": *success }),
            }],
            TrajectoryKind::Shot {
                on_target,
                from,
                to,
                initial_velocity,
                shot_type,
            } => vec![Classified {
                actor: Some(ActorId::Player(*actor)),
                event_type: EventType::Shot,
                fields: serde_json::json!({
                    "on_target": *on_target,
                    "from": { "x": from.0, "y": from.1, "z": from.2 },
                    "to": { "x": to.0, "y": to.1, "z": to.2 },
                    "initial_velocity": {
                        "x": initial_velocity.0,
                        "y": initial_velocity.1,
                        "z": initial_velocity.2
                    },
                    "shot_type": shot_type.as_str(),
                }),
            }],
        },
        Fact::GoalLineCrossing { actor, point } => vec![Classified {
            actor: *actor,
            event_type: EventType::Goal,
            fields: serde_json::json!({
                "x": point.0,
                "y": point.1,
                "z": point.2,
            }),
        }],
        Fact::RelationEnterExit {
            carrier,
            opponent,
            entered,
        } => {
            if *entered {
                vec![Classified {
                    actor: Some(ActorId::Player(*opponent)),
                    event_type: EventType::Pressure,
                    fields: serde_json::json!({
                        "from": carrier.slot(),
                        "to": opponent.slot(),
                    }),
                }]
            } else {
                Vec::new()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ds_core::ids::PlayerId;

    fn p(slot: u32) -> PlayerId {
        PlayerId::new(slot, 0)
    }

    #[test]
    fn contact_winner_is_attacker_emits_only_duel() {
        let c = classify(&Fact::Contact {
            attacker: p(1),
            defender: p(2),
            winner: p(1),
        });
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].event_type, EventType::Duel);
    }

    #[test]
    fn contact_winner_is_defender_emits_duel_then_tackle() {
        let c = classify(&Fact::Contact {
            attacker: p(1),
            defender: p(2),
            winner: p(2),
        });
        assert_eq!(c.len(), 2);
        assert_eq!(c[0].event_type, EventType::Duel);
        assert_eq!(c[1].event_type, EventType::Tackle);
    }

    #[test]
    fn pass_trajectory_emits_pass_with_complete_field() {
        let c = classify(&Fact::Trajectory {
            actor: p(1),
            kind: TrajectoryKind::Pass { success: true },
        });
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].event_type, EventType::Pass);
        assert_eq!(c[0].fields["complete"], true);
    }

    #[test]
    fn shot_trajectory_emits_shot_with_new_fields() {
        let c = classify(&Fact::Trajectory {
            actor: p(1),
            kind: TrajectoryKind::Shot {
                on_target: true,
                from: (5000, 3400, 0),
                to: (10500, 3500, 244),
                initial_velocity: (300, 5, 14),
                shot_type: ShotType::Power,
            },
        });
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].event_type, EventType::Shot);
        assert_eq!(c[0].fields["on_target"], true);
        assert_eq!(c[0].fields["from"]["x"], 5000);
        assert_eq!(c[0].fields["to"]["y"], 3500);
        assert_eq!(c[0].fields["initial_velocity"]["x"], 300);
        assert_eq!(c[0].fields["shot_type"], "power");
        assert!(
            c[0].fields.get("goal").is_none(),
            "goal removed from Shot: {:?}",
            c[0].fields
        );
    }

    #[test]
    fn goal_line_crossing_emits_goal_with_position() {
        let c = classify(&Fact::GoalLineCrossing {
            actor: Some(ActorId::Player(p(1))),
            point: (10500, 3400, 0),
        });
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].event_type, EventType::Goal);
        assert_eq!(c[0].fields["x"], 10500);
    }

    #[test]
    fn relation_enter_emits_pressure() {
        let c = classify(&Fact::RelationEnterExit {
            carrier: p(9),
            opponent: p(105),
            entered: true,
        });
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].event_type, EventType::Pressure);
        assert_eq!(c[0].fields["from"], 9);
        assert_eq!(c[0].fields["to"], 105);
    }

    #[test]
    fn relation_exit_emits_nothing() {
        let c = classify(&Fact::RelationEnterExit {
            carrier: p(9),
            opponent: p(105),
            entered: false,
        });
        assert_eq!(c.len(), 0);
    }
}
