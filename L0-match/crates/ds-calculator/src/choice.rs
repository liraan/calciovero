//! The choice's one constant of its own (ADR-052, OI-85).
//!
//! The worth of every option is the player's own imagined outcome
//! (`thought.rs`, `ds-orchestrator::read`), his game reading and his
//! appetite for risk already in it. Until `SIM_VERSION` 126 a formula here
//! weighted that worth again by game reading and risk management, and a
//! clarity threshold sent the carrier forward whenever nothing reached it;
//! both are gone. What is left is how far a weak decider strays from his
//! best (`resolve_choice`).

use serde::Deserialize;

/// The embedded balance file.
const CHOICE_JSON: &str = include_str!("../../../balance/choice.json");

/// The constant of the choice that the thought does not already weigh
/// (ADR-052; OI-85): how far a weak decider strays from his best.
#[derive(Debug, Clone, Deserialize)]
pub struct ChoiceBalance {
    /// A poorly deciding actor takes the second-best candidate if it is
    /// within this share of the best's size, scaled by `1 -
    /// decision_making` (`resolve_choice`).
    pub decision_bias: f64,
}

impl ChoiceBalance {
    /// Loads the embedded balance file.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid. The test
    /// `embedded_choice_balance_is_valid` catches this at test time.
    #[must_use]
    pub fn load() -> Self {
        serde_json::from_str(CHOICE_JSON).expect("embedded choice balance is valid JSON")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_choice_balance_is_valid() {
        let b = ChoiceBalance::load();
        assert!((0.0..=1.0).contains(&b.decision_bias));
    }
}
