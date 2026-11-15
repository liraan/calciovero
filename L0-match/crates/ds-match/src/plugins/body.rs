//! The body plugin.
//!
//! The body plugin (ADR-063 §4; ADR-030, MD-09, MD-12): each player's step
//! towards where he chose to go, his acceleration and top speed, the
//! separation of the bodies. The movement acts on the spatial state and
//! lives there (`SpatialState::move_agents`, `spatial::separate`); this is
//! its card.

use ds_match_api::{MatchPlugin, PluginCard};

/// The bodies of the players.
#[derive(Debug, Clone, Copy, Default)]
pub struct Body;

impl MatchPlugin for Body {
    fn card(&self) -> PluginCard {
        crate::registry::card("body")
    }
}

/// Every mandatory plugin of the match, by its type (ADR-063 §4).
#[must_use]
pub fn all() -> Vec<Box<dyn MatchPlugin>> {
    use super::{
        ball::Ball, bench::Bench, duels::Duels, fatigue::Fatigue, keeper::Keeper,
        perception::Perception, referee::Referee, set_pieces::SetPieces, technique::Technique,
        thought::Thought,
    };
    vec![
        Box::new(Ball),
        Box::new(Body),
        Box::new(Perception::default()),
        Box::new(Thought),
        Box::new(Technique),
        Box::new(Duels),
        Box::new(Keeper),
        Box::new(Referee),
        Box::new(SetPieces),
        Box::new(Bench),
        Box::new(Fatigue::default()),
    ]
}

#[cfg(test)]
mod tests {
    /// P1 (ADR-063 §4): every one of the eleven mandatory plugins is a type
    /// of its own with a valid card, in the contract's order.
    #[test]
    fn every_mandatory_plugin_is_behind_the_contract() {
        let names: Vec<&str> = super::all().iter().map(|p| p.card().name).collect();
        assert_eq!(names, ds_match_api::MANDATORY);
        assert!(super::all().iter().all(|p| p.card().is_valid()));
    }
}
