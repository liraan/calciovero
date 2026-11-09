//! The latency formula: how long the carrier waits before deciding again
//! (IDEA-DESIGN-011, R.1-latency).
//!
//! The phenomenon, in a real match: the tick is a unit of measure, not a
//! cause. A tired player decides slower, a pressed player decides slower,
//! a sharp reader decides faster. The pace is produced by the actor, not
//! by a global die on the tick loop.
//!
//! The formula, in the grammar:
//!
//! `latency_ticks = round(base * (1 + (1 - decision_making) * bias))`
//!
//! `decision_making` carries the base (a strong decider decides sooner).
//! Fatigue and pressure extend it; both terms are declared `Removed-by:`
//! until the corresponding producers exist (OI-70 for fatigue, no term for
//! pressure today). The form is a model choice (one multiplicative bias,
//! no die, ADR-035); the values are declared arbitrary, to be calibrated
//! at L3.

use serde::Deserialize;

use crate::{Characteristic, Formula};

/// The embedded balance file.
const LATENCY_JSON: &str = include_str!("../../../balance/latency.json");

/// The constants of the latency formula.
#[derive(Debug, Clone, Deserialize)]
pub struct LatencyBalance {
    /// Base latency, in ticks (derived from `action_probability` 0.0005).
    pub base_ticks: f64,
    /// Bias: how much `decision_making` modulates the base.
    pub decision_bias: f64,
}

impl LatencyBalance {
    /// Loads the embedded balance file.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid. The test
    /// `embedded_latency_balance_is_valid` catches this at test time.
    #[must_use]
    pub fn load() -> Self {
        serde_json::from_str(LATENCY_JSON).expect("embedded latency balance is valid JSON")
    }

    /// The latency formula, built from the balance values.
    ///
    /// `latency = base * (1 + (1 - decision_making) * bias)`
    ///
    /// The latency is a duration; the orchestrator (`resolve_latency` in
    /// `pcfm-hidden`) converts the f64 result to ticks. The formula does
    /// not round: rounding is the orchestrator's job, so the formula stays
    /// a pure f64 expression.
    #[must_use]
    pub fn formula(&self) -> Formula {
        Formula::Product(vec![
            Formula::Const(self.base_ticks),
            Formula::Sum(vec![
                Formula::Const(1.0),
                Formula::Product(vec![
                    Formula::Const(self.decision_bias),
                    Formula::Sum(vec![
                        Formula::Const(1.0),
                        Formula::Product(vec![
                            Formula::Const(-1.0),
                            Formula::Term(Characteristic::DecisionMaking),
                        ]),
                    ]),
                ]),
            ]),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_latency_balance_is_valid() {
        let b = LatencyBalance::load();
        assert!(b.base_ticks > 0.0);
        assert!(b.decision_bias >= 0.0);
    }

    #[test]
    fn latency_formula_declares_decision_making() {
        let f = LatencyBalance::load().formula();
        assert_eq!(
            f.declared_characteristics(),
            vec![Characteristic::DecisionMaking]
        );
        assert!(
            f.declared_data().is_empty(),
            "latency formula declares no Plane B datum"
        );
    }

    #[test]
    fn latency_formula_validates() {
        let f = LatencyBalance::load().formula();
        assert!(f.validate().is_ok());
    }
}
