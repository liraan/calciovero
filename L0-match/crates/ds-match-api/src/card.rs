//! The card of a plugin (ADR-012, ADR-063 §1), shared with the season
//! (ADR-113 §3, decision AQ): `ds-plugin-api`'s card over the phases of a
//! tick, with its one validator.

use crate::Phase;

pub use ds_plugin_api::{CardError, OptionalRead, Tier, order};

/// The card of a match plugin: the shared card, acting in the phases of a
/// tick.
pub type PluginCard = ds_plugin_api::PluginCard<Phase>;

#[cfg(test)]
mod tests {
    use super::{PluginCard, Tier};
    use crate::Phase;

    #[test]
    fn a_card_needs_a_name_a_version_and_a_phase() {
        let good = PluginCard {
            name: "fatigue",
            version: "1",
            steps: &[Phase::Move],
            depends: &[],
            reads: &["speed"],
            optional_reads: &[],
            writes: &["fatigue"],
            tier: Tier::Core,
            cost_us: 0,
            narrative: &[],
        };
        assert!(good.is_valid());
        let no_phase = PluginCard { steps: &[], ..good };
        assert!(!no_phase.is_valid());
    }
}
