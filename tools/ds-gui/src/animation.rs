//! The table event → animation (ADR-065 §3, `DESIGN.md` "La visuale del
//! 2.5D"): which animation each type of fact of the match gives the view.
//!
//! **One responsibility:** the row of every event type, and nothing else.
//! The table is written as a `match` with no catch-all, so a new event type
//! does not compile until it has its row; and a test reads the event types
//! from their source and fails if one is missing from [`ROWS`]. A type with
//! nothing to show gets the fallback pose, [`Animation::Rest`]: the man
//! as he is, running or standing, never an invented gesture.

use ds_stats::EventType;

/// What a fact of the match shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Animation {
    /// The kick; the jump if the event says it was a header.
    KickOrHeader,
    /// The jump if the event says it was in the air, else the fallback.
    JumpIfAerial,
    /// The keeper's dive, towards the ball's side.
    Dive,
    /// The fouled man falls; the foul is a slide if the event says so.
    Foul,
    /// The referee shows the card.
    Card,
    /// The throw-in.
    Throw,
    /// The scorer celebrates.
    Celebrate,
    /// The standing tackle.
    Lunge,
    /// The fallback pose: no gesture, the man as he moves.
    Rest,
}

/// The row of an event type.
#[must_use]
pub const fn animation(kind: EventType) -> Animation {
    use EventType as E;
    match kind {
        E::Shot | E::Pass | E::Cross | E::Clearance => Animation::KickOrHeader,
        E::Duel => Animation::JumpIfAerial,
        E::Save => Animation::Dive,
        E::Foul => Animation::Foul,
        E::Card => Animation::Card,
        E::ThrowIn => Animation::Throw,
        E::Goal => Animation::Celebrate,
        E::Tackle => Animation::Lunge,
        // Facts with no gesture of their own: the kick that takes a corner,
        // a free kick or a goal kick is its own `Pass` or `Shot` event; the
        // rest are counts, the clock or the bench.
        E::Dribble
        | E::KeyPass
        | E::Interception
        | E::Block
        | E::Corner
        | E::FreeKick
        | E::Penalty
        | E::Substitution
        | E::Injury
        | E::Assist
        | E::ProgressiveCarry
        | E::ProgressivePass
        | E::Pressure
        | E::RandomEvent
        | E::KickOff
        | E::HalfTime
        | E::SecondHalf
        | E::FullTime
        | E::MatchAbandoned
        | E::GoalKick
        | E::Woodwork
        | E::Instruction => Animation::Rest,
    }
}

/// Every event type, in the order of its declaration.
#[cfg_attr(
    not(test),
    allow(dead_code, reason = "the list the table's test checks")
)]
pub const ROWS: &[EventType] = {
    use EventType as E;
    &[
        E::Shot,
        E::Pass,
        E::Cross,
        E::Dribble,
        E::KeyPass,
        E::Tackle,
        E::Interception,
        E::Clearance,
        E::Block,
        E::Save,
        E::Duel,
        E::Foul,
        E::Card,
        E::Corner,
        E::FreeKick,
        E::Penalty,
        E::Substitution,
        E::Injury,
        E::Goal,
        E::Assist,
        E::ProgressiveCarry,
        E::ProgressivePass,
        E::Pressure,
        E::RandomEvent,
        E::KickOff,
        E::HalfTime,
        E::SecondHalf,
        E::FullTime,
        E::MatchAbandoned,
        E::ThrowIn,
        E::GoalKick,
        E::Woodwork,
        E::Instruction,
    ]
};

#[cfg(test)]
mod tests {
    use super::*;

    /// The event types as their source declares them.
    fn declared() -> Vec<String> {
        let src = include_str!("../../../L0-match/crates/ds-stats/src/event.rs");
        let body = src
            .split("pub enum EventType {")
            .nth(1)
            .and_then(|s| s.split("\n}").next())
            .expect("EventType in its source");
        body.lines()
            .map(str::trim)
            .filter(|l| !l.starts_with("//") && !l.starts_with('#') && !l.is_empty())
            .map(|l| l.trim_end_matches(',').to_owned())
            .collect()
    }

    /// Every event type has its row: a type missing from the table fails
    /// here (ADR-065 §3).
    #[test]
    fn every_event_type_has_its_row() {
        let rows: Vec<String> = ROWS
            .iter()
            .map(|t| {
                serde_json::to_string(t)
                    .expect("name")
                    .trim_matches('"')
                    .to_owned()
            })
            .collect();
        assert_eq!(rows, declared(), "the table and the event types differ");
    }

    /// The gestures come only from facts: the fallback is the rest, and
    /// the facts that move a body are the ones in the DESIGN table.
    #[test]
    fn the_fallback_is_no_gesture() {
        assert_eq!(animation(EventType::Substitution), Animation::Rest);
        assert_eq!(animation(EventType::Shot), Animation::KickOrHeader);
        let moving = ROWS
            .iter()
            .filter(|&&t| animation(t) != Animation::Rest)
            .count();
        assert_eq!(moving, 11);
    }
}
