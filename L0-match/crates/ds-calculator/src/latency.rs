//! The latency formula: how long the carrier waits before deciding again
//! (IDEA-DESIGN-011, R.1-latency; ADR-044 §6, R.1-relation).
//!
//! The phenomenon, in a real match: the tick is a unit of measure, not a
//! cause. A tired player decides slower, a pressed player decides slower,
//! a sharp reader decides faster. The pace is produced by the actor and by
//! the scene, not by a global die on the tick loop.
//!
//! The formula, in the grammar:
//!
//! `latency_ticks = round(base * (1 + (1 - decision_making) * bias) * (1 + crowding * crowd_weight + nearest_threat * threat_weight))`
//!
//! `decision_making` carries the base (a strong decider decides sooner).
//! The two scene terms carry the duration-in-the-relation (ADR-044 §6): a
//! crowded scene, or a close threat, extends the wait. Both terms are in
//! `[0, 1]` (the orchestrator normalises them). Fatigue is a declared term
//! `Removed-by: OI-70` until its producer exists. The form is a model
//! choice (multiplicative biases, no die, ADR-035); the values are
//! declared arbitrary, to be calibrated at L3.
//!
//! **A third scene term, `plausible_options`, is not implemented.** The
//! capture of 2026-11-10 names it, but reading it inside `resolve_latency`
//! would require the plausibility of `build_choice` (D7: no duplicate) or
//! a circular dependence (the latency would depend on the outcome of the
//! choose). The two remaining terms are enough to make the relation
//! visible; the third is tracked as OI-96.
//!
//! **The formula is built once, in `load()`** (2026-11-09, F.2-bis bench,
//! same fix as movement). `formula()` returns a reference.

use serde::Deserialize;

use crate::{Characteristic, Datum, Formula};

/// The embedded balance file.
const LATENCY_JSON: &str = include_str!("../../../balance/latency.json");

/// The constants of the latency formula, and the formula itself.
#[derive(Debug, Clone)]
pub struct LatencyBalance {
    /// Base latency, in ticks (derived from `action_probability` 0.0005).
    pub base_ticks: f64,
    /// Bias: how much `decision_making` modulates the base.
    pub decision_bias: f64,
    /// Weight of the `crowding` scene term (ADR-044 §6).
    pub crowd_weight: f64,
    /// Weight of the `nearest_threat` scene term (ADR-044 §6).
    pub threat_weight: f64,
    /// The formula, built once from the constants above.
    formula: Formula,
}

/// The JSON shape, parsed before the formula is built.
#[derive(Debug, Clone, Deserialize)]
struct LatencyBalanceRaw {
    base_ticks: f64,
    decision_bias: f64,
    crowd_weight: f64,
    threat_weight: f64,
}

impl LatencyBalance {
    /// Loads the embedded balance file and builds the formula once.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid. The test
    /// `embedded_latency_balance_is_valid` catches this at test time.
    #[must_use]
    pub fn load() -> Self {
        let raw: LatencyBalanceRaw =
            serde_json::from_str(LATENCY_JSON).expect("embedded latency balance is valid JSON");
        let formula = Self::build_formula(&raw);
        Self {
            base_ticks: raw.base_ticks,
            decision_bias: raw.decision_bias,
            crowd_weight: raw.crowd_weight,
            threat_weight: raw.threat_weight,
            formula,
        }
    }

    /// The latency formula, built once in `load()`.
    ///
    /// `latency = base * (1 + (1 - decision_making) * bias) * (1 + crowding * crowd_weight + nearest_threat * threat_weight)`
    ///
    /// The latency is a duration; the orchestrator (`resolve_latency` in
    /// `ds-orchestrator`) converts the f64 result to ticks. The formula
    /// does not round: rounding is the orchestrator's job, so the formula
    /// stays a pure f64 expression.
    #[must_use]
    pub const fn formula(&self) -> &Formula {
        &self.formula
    }

    fn build_formula(b: &LatencyBalanceRaw) -> Formula {
        // `1 + (1 - decision_making) * bias`
        let decision_factor = Formula::Sum(vec![
            Formula::Const(1.0),
            Formula::Product(vec![
                Formula::Const(b.decision_bias),
                Formula::Sum(vec![
                    Formula::Const(1.0),
                    Formula::Product(vec![
                        Formula::Const(-1.0),
                        Formula::Term(Characteristic::DecisionMaking),
                    ]),
                ]),
            ]),
        ]);
        // `1 + crowding * crowd_weight + nearest_threat * threat_weight`
        let scene_factor = Formula::Sum(vec![
            Formula::Const(1.0),
            Formula::Product(vec![
                Formula::Const(b.crowd_weight),
                Formula::Datum(Datum::Crowding),
            ]),
            Formula::Product(vec![
                Formula::Const(b.threat_weight),
                Formula::Datum(Datum::NearestThreat),
            ]),
        ]);
        Formula::Product(vec![
            Formula::Const(b.base_ticks),
            decision_factor,
            scene_factor,
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
        assert!(b.crowd_weight >= 0.0);
        assert!(b.threat_weight >= 0.0);
    }

    #[test]
    fn latency_formula_declares_decision_making_and_scene_terms() {
        let b = LatencyBalance::load();
        let f = b.formula();
        assert_eq!(
            f.declared_characteristics(),
            vec![Characteristic::DecisionMaking]
        );
        assert_eq!(
            f.declared_data(),
            vec![Datum::Crowding, Datum::NearestThreat]
        );
    }

    #[test]
    fn latency_formula_validates() {
        let b = LatencyBalance::load();
        assert!(b.formula().validate().is_ok());
    }
}
