//! The match plugin (ADR-063 §1, §4).

use crate::{Phase, PluginCard, World};

/// A part of the match that is not the core: it says what it is with its
/// card.
///
/// It acts in the phases of its card through `run`, on the observable
/// state (`World`); the core calls it in the order of the phases.
pub trait MatchPlugin {
    /// Its card.
    fn card(&self) -> PluginCard;

    /// What it does in `phase`, on the world as the previous phase left
    /// it. A plugin acts only in the phases of its card; the others are no
    /// work.
    fn run(&mut self, phase: Phase, world: &mut dyn World) {
        let _ = (phase, world);
    }
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
