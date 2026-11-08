//! The choice score formula: F.6-cog.
//!
//! The phenomenon, in a real match: the geometric reading of an option
//! (how far, how free, how dangerous) is weighed by the player's
//! cognitive traits -- how well he reads the game, how quickly he
//! decides, how much risk he accepts. A great reader turns a mediocre
//! geometric option into a good choice; a poor decision maker wastes a
//! good one.
//!
//! The grammar is `product` (L0.11 section 4.5). The form:
//!
//!   score = geometric
//!         * (`floor` + (1-`floor`) * `reading_weight`  * `game_reading`)
//!         * (`floor` + (1-`floor`) * `decision_weight` * `decision_making`)
//!         * (1 - `risk_weight` * `risk_management` * `risk`)
//!
//! where `geometric` is a `Const` computed by the engine from the scene
//! (positions, distances) and `risk` is a `Const` the engine derives
//! from the option (how risky the pass is, how far the shot). The
//! three cognitive terms are `Term`s read through the custodian.
//!
//! Not in the formula: the situation terms of L0.11 section 4.6
//! (`receiver_freedom`, `distance`, `pressure`, `angle`). The
//! `Situation` trait does not carry them yet; the extension is
//! tracked as OI-84. Until then, geometry is a `Const` derived in code
//! (ADR-028 section 3: derived values are admitted; the gold standard
//! of a declared balance lives in this file).

use serde::Deserialize;

use crate::{Characteristic, Formula};

/// The embedded balance file.
const CHOICE_JSON: &str = include_str!("../../../balance/choice.json");

/// The constants of the choice formula.
#[derive(Debug, Clone, Deserialize)]
pub struct ChoiceBalance {
    /// Weight of `game_reading` in the score (0..=1).
    pub reading_weight: f64,
    /// Weight of `decision_making` in the score (0..=1).
    pub decision_weight: f64,
    /// Weight of `risk_management` in the risk discount (0..=1).
    pub risk_weight: f64,
    /// Floor of the latent modifiers: they cannot make the score zero.
    pub floor: f64,
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

    /// The choice score formula, built from the balance and the two
    /// engine-derived constants of the option.
    ///
    /// `geometric` and `risk` are Plane B, derived by the engine from
    /// the scene (positions, distances) and the option itself. They are
    /// `Const` here because the `Situation` trait does not carry
    /// per-option terms yet (OI-84).
    #[must_use]
    pub fn formula(&self, geometric: f64, risk: f64) -> Formula {
        let one_minus_floor = 1.0 - self.floor;
        Formula::Product(vec![
            Formula::Const(geometric),
            // reading modifier
            Formula::Sum(vec![
                Formula::Const(self.floor),
                Formula::Product(vec![
                    Formula::Const(one_minus_floor * self.reading_weight),
                    Formula::Term(Characteristic::GameReading),
                ]),
            ]),
            // decision modifier
            Formula::Sum(vec![
                Formula::Const(self.floor),
                Formula::Product(vec![
                    Formula::Const(one_minus_floor * self.decision_weight),
                    Formula::Term(Characteristic::DecisionMaking),
                ]),
            ]),
            // risk discount
            Formula::Sum(vec![
                Formula::Const(1.0),
                Formula::Product(vec![
                    Formula::Const(-self.risk_weight * risk),
                    Formula::Term(Characteristic::RiskManagement),
                ]),
            ]),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_choice_balance_is_valid() {
        let b = ChoiceBalance::load();
        assert!((0.0..=1.0).contains(&b.reading_weight));
        assert!((0.0..=1.0).contains(&b.decision_weight));
        assert!((0.0..=1.0).contains(&b.risk_weight));
        assert!((0.0..=1.0).contains(&b.floor));
    }

    #[test]
    fn choice_formula_declares_the_three_cognitive_terms() {
        let f = ChoiceBalance::load().formula(1.0, 0.5);
        let chars = f.declared_characteristics();
        assert_eq!(
            chars,
            vec![
                Characteristic::GameReading,
                Characteristic::DecisionMaking,
                Characteristic::RiskManagement,
            ]
        );
    }

    #[test]
    fn choice_formula_validates() {
        let f = ChoiceBalance::load().formula(1.0, 0.5);
        assert!(f.validate().is_ok());
    }
}
