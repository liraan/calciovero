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
    /// Bias of a weak decider on the choice: a poorly deciding actor
    /// accepts the second-best candidate if it is within this fraction
    /// of the best. `decision_making` enters `resolve_choice`, not the
    /// formula (F.6-shape, OI-85; ADR-039 read/choose split).
    pub decision_bias: f64,
    /// Weight of `risk_management` in the risk discount (0..=1).
    pub risk_weight: f64,
    /// Floor of the latent modifiers: they cannot make the score zero.
    pub floor: f64,
    /// Threshold of clarity on the best candidate's score: if the best
    /// candidate's score does not exceed this, the actor carries the ball
    /// (the `Advance` choice, IDEA-DESIGN-010). Arbitrary and declared
    /// until calibration (L3). First value 0.25: a typical pass sits
    /// between 0.1 and 0.4, so below 0.25 it is "not clear enough".
    pub clarity_threshold: f64,
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
            // reading modifier: game_reading modulates how the scene is
            // read, and it is per-candidate only when it interacts with
            // the candidate's geometry. Here it is a multiplier of the
            // product, so the *interaction* is with `geometric` (see
            // OI-85 for the model discussion). The final shape is a
            // product: geo * reading_mod * risk_mod.
            Formula::Sum(vec![
                Formula::Const(self.floor),
                Formula::Product(vec![
                    Formula::Const(one_minus_floor * self.reading_weight),
                    Formula::Term(Characteristic::GameReading),
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
        assert!((0.0..=1.0).contains(&b.decision_bias));
        assert!((0.0..=1.0).contains(&b.risk_weight));
        assert!((0.0..=1.0).contains(&b.floor));
        assert!((0.0..=1.0).contains(&b.clarity_threshold));
    }

    #[test]
    fn choice_formula_declares_reading_and_risk() {
        let f = ChoiceBalance::load().formula(1.0, 0.5);
        let chars = f.declared_characteristics();
        assert_eq!(
            chars,
            vec![Characteristic::GameReading, Characteristic::RiskManagement,]
        );
        // decision_making is not a formula term: it is a threshold on
        // the choice (F.6-shape, OI-85; ADR-039 read/choose split).
    }

    #[test]
    fn choice_formula_validates() {
        let f = ChoiceBalance::load().formula(1.0, 0.5);
        assert!(f.validate().is_ok());
    }
}
