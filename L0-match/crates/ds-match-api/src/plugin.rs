//! The match plugin (ADR-063 §1, §4).

use crate::PluginCard;

/// A part of the match that is not the core: it says what it is with its
/// card.
///
/// The methods it acts with in each phase arrive with the first
/// plugin moved behind the contract (plan `PLUGINS.md`, step 3), so that
/// no method enters without its consumer (RULES D3).
pub trait MatchPlugin {
    /// Its card.
    fn card(&self) -> PluginCard;
}

/// The eleven plugins every match has (ADR-063 §4).
pub const MANDATORY: [&str; 11] = [
    "ball",
    "body",
    "perception",
    "thought",
    "technique",
    "duels",
    "keeper",
    "referee",
    "set_pieces",
    "bench",
    "fatigue",
];

/// The three that may be switched off (ADR-063 §4).
pub const OPTIONAL: [&str; 3] = ["psychology", "environment", "knocks"];

#[cfg(test)]
mod tests {
    use super::{MANDATORY, OPTIONAL};

    #[test]
    fn eleven_mandatory_three_optional_no_name_twice() {
        let mut all: Vec<&str> = MANDATORY.iter().chain(OPTIONAL.iter()).copied().collect();
        all.sort_unstable();
        all.dedup();
        assert_eq!(all.len(), 14);
    }
}
