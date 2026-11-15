//! The fatigue plugin (ADR-063 §4, plan L0-CLOSE step 6): in the move
//! phase each player tires with the speed he ran at this tick, against
//! his endurance (`balance/fatigue.json`, `ds_calculator::fatigue`).

use std::collections::BTreeMap;

use ds_core::{dmath, ids::PlayerId};
use ds_match_api::{MatchPlugin, PluginCard};

use crate::spatial::{PlayerBody, SpatialState};

/// The fatigue of the players.
#[derive(Debug, Clone, Copy, Default)]
pub struct Fatigue;

impl MatchPlugin for Fatigue {
    fn card(&self) -> PluginCard {
        crate::registry::card("fatigue")
    }
}

impl Fatigue {
    /// Each player on the pitch tires with the speed he moved at.
    pub(crate) fn run(
        bodies: &mut BTreeMap<PlayerId, PlayerBody>,
        spatial: &SpatialState,
        hidden: &mut ds_orchestrator::GroundTruth,
    ) {
        for (id, body) in bodies {
            let Some(a) = spatial.agent(*id) else {
                continue;
            };
            let speed = dmath::hypot(f64::from(a.velocity.0), f64::from(a.velocity.1));
            body.fatigue = hidden.tire(*id, body.fatigue, speed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Fatigue;
    use ds_match_api::MatchPlugin;

    #[test]
    fn the_fatigue_plugin_has_its_card() {
        let card = Fatigue.card();
        assert_eq!(card.name, "fatigue");
        assert!(card.is_valid());
    }
}
