//! The card of a plugin (ADR-012, ADR-063 §1).

use crate::Phase;

/// How much a plugin weighs in the level of detail of a match (ADR-012):
/// a lighter level may run without the optional plugins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Weight {
    /// Always run: the match is not football without it.
    Core,
    /// Run in a detailed match; may be switched off (ADR-063 §4).
    Detail,
}

/// What a plugin says about itself, for the registry and the validator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginCard {
    /// Its name, unique in the registry.
    pub name: &'static str,
    /// Its version; a change of behaviour is a new `SIM_VERSION` too.
    pub version: &'static str,
    /// The phases it acts in.
    pub phases: &'static [Phase],
    /// What of the observable state it reads.
    pub reads: &'static [&'static str],
    /// What of the observable state it writes.
    pub writes: &'static [&'static str],
    /// Its weight in the level of detail.
    pub weight: Weight,
    /// The keys of the commentary lines it gives (the texts live outside
    /// the engine: who watches does not write in the match).
    pub narrative: &'static [&'static str],
}

impl PluginCard {
    /// Whether the card is well formed: a name, a version, at least one
    /// phase.
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        !self.name.is_empty() && !self.version.is_empty() && !self.phases.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::{PluginCard, Weight};
    use crate::Phase;

    #[test]
    fn a_card_needs_a_name_a_version_and_a_phase() {
        let good = PluginCard {
            name: "fatigue",
            version: "1",
            phases: &[Phase::Move],
            reads: &["speed"],
            writes: &["fatigue"],
            weight: Weight::Core,
            narrative: &[],
        };
        assert!(good.is_valid());
        let no_phase = PluginCard {
            phases: &[],
            ..good
        };
        assert!(!no_phase.is_valid());
    }
}
