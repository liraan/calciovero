//! `ds-calculator` — the second of the three components of ADR-028 §5.
//!
//! The engine has three components, and each has one responsibility:
//!
//! - the **custodian** (`pcfm-hidden`) holds the hidden performances and
//!   answers one characteristic at a time;
//! - the **calculator** (this crate) applies a formula from
//!   `balance/*.json` to a situation and a player;
//! - the **orchestrator** (`resolve_*`) decides when, with which
//!   situation, for which formula.
//!
//! # Two kinds of input
//!
//! A formula reads two kinds of datum, and the calculator keeps them
//! apart (L0.11 §4.6):
//!
//! - **Plane A, characteristics**: hidden performances, read through the
//!   [`Custodian`] trait, one at a time;
//! - **Plane B, data**: registry data (height, weight) and states
//!   (fatigue, confidence), read through the [`Situation`] trait.
//!
//! The calculator does not know who holds which: the orchestrator passes
//! both, and the formula declares its terms.
//!
//! # The grammar
//!
//! A formula is an expression on a **fixed grammar** of operators
//! (L0.11 §4.5): product, ratio, clamp. The sum is the product with
//! weight 1. A formula cannot invent an operator: a new operator is an
//! amendment to the model, not a formula (L0.11 §4.10).

pub mod choice;
pub mod latency;
pub mod movement;

use std::collections::BTreeSet;

use thiserror::Error;

/// A Plane A characteristic the custodian knows how to answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Characteristic {
    /// Explosive force (Plane A, physical).
    ExplosiveForce,
    /// Maximal force (Plane A, physical).
    MaximalForce,
    /// Acceleration (Plane A, physical).
    Acceleration,
    /// Max speed (Plane A, physical).
    MaxSpeed,
    /// Reading the game: what the player understands (Plane A, cognitive).
    GameReading,
    /// Decision making: how fast and how well he chooses (Plane A, cognitive).
    DecisionMaking,
    /// Risk management: the risk he accepts (Plane A, cognitive).
    RiskManagement,
    /// Finishing, consumed by the shot choice (OI-92, ADR-046).
    /// L0.11 7.2 calls the characteristic "shooting"; the latent
    /// rename is OI-50.
    Finishing,
    /// Left foot, one of the two performances per foot (ADR-031).
    /// Consumer: the shot choice (OI-92, ADR-046); the pass formula
    /// is OI-64.
    LeftFoot,
    /// Right foot, one of the two performances per foot (ADR-031).
    /// Consumer: the shot choice (OI-92, ADR-046); the pass formula
    /// is OI-64.
    RightFoot,
}

impl Characteristic {
    /// The name used in `balance/*.json` and in error messages.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::ExplosiveForce => "explosive_force",
            Self::MaximalForce => "maximal_force",
            Self::Acceleration => "acceleration",
            Self::MaxSpeed => "max_speed",
            Self::GameReading => "game_reading",
            Self::DecisionMaking => "decision_making",
            Self::RiskManagement => "risk_management",
            Self::Finishing => "finishing",
            Self::LeftFoot => "left_foot",
            Self::RightFoot => "right_foot",
        }
    }

    /// Parses a name from `balance/*.json`.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "explosive_force" => Some(Self::ExplosiveForce),
            "maximal_force" => Some(Self::MaximalForce),
            "acceleration" => Some(Self::Acceleration),
            "max_speed" => Some(Self::MaxSpeed),
            "game_reading" => Some(Self::GameReading),
            "decision_making" => Some(Self::DecisionMaking),
            "risk_management" => Some(Self::RiskManagement),
            "finishing" => Some(Self::Finishing),
            "left_foot" => Some(Self::LeftFoot),
            "right_foot" => Some(Self::RightFoot),
            _ => None,
        }
    }
}

/// A Plane B datum the orchestrator can provide (L0.11 §4.6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Datum {
    /// Height, in cm (registry data).
    Height,
    /// Weight, in kg (registry data).
    Weight,
    /// Fatigue, 0.0–1.0 (state).
    Fatigue,
    /// Crowding around the actor, 0.0-1.0 (scene term, §4.6 Decision family).
    Crowding,
    /// Nearest threat distance, 0.0-1.0 (scene term, §4.6 Decision family).
    NearestThreat,
}

impl Datum {
    /// The name used in `balance/*.json` and in error messages.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Height => "height",
            Self::Weight => "weight",
            Self::Fatigue => "fatigue",
            Self::Crowding => "crowding",
            Self::NearestThreat => "nearest_threat",
        }
    }

    /// Parses a name from `balance/*.json`.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "height" => Some(Self::Height),
            "weight" => Some(Self::Weight),
            "fatigue" => Some(Self::Fatigue),
            "crowding" => Some(Self::Crowding),
            "nearest_threat" => Some(Self::NearestThreat),
            _ => None,
        }
    }
}

/// The custodian API, as seen by the calculator (Plane A).
pub trait Custodian {
    /// Returns the value of `characteristic` for `player`, in `[0, 1)`.
    fn value(&mut self, characteristic: Characteristic) -> f64;
}

/// The Plane B data, as seen by the calculator.
pub trait Situation {
    /// Returns the value of `datum`, in the unit the formula expects.
    fn datum(&self, datum: Datum) -> f64;
}

/// A formula, as read from `balance/*.json`.
#[derive(Debug, Clone)]
pub enum Formula {
    /// A Plane A characteristic, read through the custodian.
    Term(Characteristic),
    /// A Plane B datum, read through the situation.
    Datum(Datum),
    /// A constant (a number declared in `balance/*.json`).
    Const(f64),
    /// The product of the operands, applied left to right.
    Product(Vec<Self>),
    /// The sum of the operands, applied left to right.
    Sum(Vec<Self>),
    /// The ratio numerator / denominator.
    Ratio(Box<Self>, Box<Self>),
    /// The inner value, clamped to [min, max].
    Clamp(Box<Self>, f64, f64),
}

impl Formula {
    /// The Plane A characteristics of the formula, sorted.
    #[must_use]
    pub fn declared_characteristics(&self) -> Vec<Characteristic> {
        let mut set = BTreeSet::new();
        self.collect_characteristics(&mut set);
        set.into_iter().collect()
    }

    /// The Plane B data of the formula, sorted.
    #[must_use]
    pub fn declared_data(&self) -> Vec<Datum> {
        let mut set = BTreeSet::new();
        self.collect_data(&mut set);
        set.into_iter().collect()
    }

    fn collect_characteristics(&self, out: &mut BTreeSet<Characteristic>) {
        match self {
            Self::Term(c) => {
                out.insert(*c);
            }
            Self::Datum(_) | Self::Const(_) => {}
            Self::Product(items) | Self::Sum(items) => {
                for item in items {
                    item.collect_characteristics(out);
                }
            }
            Self::Ratio(a, b) => {
                a.collect_characteristics(out);
                b.collect_characteristics(out);
            }
            Self::Clamp(inner, _, _) => inner.collect_characteristics(out),
        }
    }

    fn collect_data(&self, out: &mut BTreeSet<Datum>) {
        match self {
            Self::Datum(d) => {
                out.insert(*d);
            }
            Self::Term(_) | Self::Const(_) => {}
            Self::Product(items) | Self::Sum(items) => {
                for item in items {
                    item.collect_data(out);
                }
            }
            Self::Ratio(a, b) => {
                a.collect_data(out);
                b.collect_data(out);
            }
            Self::Clamp(inner, _, _) => inner.collect_data(out),
        }
    }

    /// Validates the formula: it must have at least one term.
    ///
    /// # Errors
    ///
    /// Returns [`CalculatorError::EmptyFormula`] if the formula has no
    /// term (a constant alone is not a formula).
    pub fn validate(&self) -> Result<(), CalculatorError> {
        if self.declared_characteristics().is_empty() && self.declared_data().is_empty() {
            return Err(CalculatorError::EmptyFormula);
        }
        Ok(())
    }
}

/// The calculator: applies a formula to a player and a situation.
pub struct Calculator;

impl Calculator {
    /// Applies `formula`, using `custodian` for Plane A and `situation`
    /// for Plane B.
    ///
    /// The player is not a parameter: the orchestrator already knows it,
    /// and the custodian it passes is bound to that player. The trait
    /// asks by characteristic only, and the mechanical test verifies the
    /// calls are exactly the formula's terms.
    #[must_use]
    pub fn apply(
        formula: &Formula,
        custodian: &mut dyn Custodian,
        situation: &dyn Situation,
    ) -> f64 {
        Self::eval(formula, custodian, situation)
    }

    fn eval(formula: &Formula, custodian: &mut dyn Custodian, situation: &dyn Situation) -> f64 {
        match formula {
            Formula::Term(c) => custodian.value(*c),
            Formula::Datum(d) => situation.datum(*d),
            Formula::Const(v) => *v,
            Formula::Product(items) => {
                let mut acc = 1.0;
                for item in items {
                    acc *= Self::eval(item, custodian, situation);
                }
                acc
            }
            Formula::Sum(items) => {
                let mut acc = 0.0;
                for item in items {
                    acc += Self::eval(item, custodian, situation);
                }
                acc
            }
            Formula::Ratio(a, b) => {
                let num = Self::eval(a, custodian, situation);
                let den = Self::eval(b, custodian, situation);
                if den == 0.0 { num } else { num / den }
            }
            Formula::Clamp(inner, min, max) => {
                Self::eval(inner, custodian, situation).clamp(*min, *max)
            }
        }
    }
}

/// Errors of the calculator.
#[derive(Debug, Error)]
pub enum CalculatorError {
    /// A formula has no terms.
    #[error("formula has no terms")]
    EmptyFormula,
}

#[cfg(test)]
mod tests {
    use super::*;

    struct RecordingCustodian {
        calls: Vec<Characteristic>,
    }

    impl RecordingCustodian {
        fn new() -> Self {
            Self { calls: Vec::new() }
        }
    }

    impl Custodian for RecordingCustodian {
        fn value(&mut self, characteristic: Characteristic) -> f64 {
            self.calls.push(characteristic);
            0.5
        }
    }

    struct RecordingSituation {
        calls: std::cell::RefCell<Vec<Datum>>,
        height: f64,
        weight: f64,
    }

    impl RecordingSituation {
        fn new(height: f64, weight: f64) -> Self {
            Self {
                calls: std::cell::RefCell::new(Vec::new()),
                height,
                weight,
            }
        }
    }

    impl Situation for RecordingSituation {
        fn datum(&self, datum: Datum) -> f64 {
            self.calls.borrow_mut().push(datum);
            match datum {
                Datum::Height => self.height,
                Datum::Weight => self.weight,
                Datum::Fatigue | Datum::Crowding | Datum::NearestThreat => 0.0,
            }
        }
    }

    fn movement_formula() -> Formula {
        Formula::Product(vec![
            Formula::Const(2.0),
            Formula::Term(Characteristic::ExplosiveForce),
            Formula::Ratio(
                Box::new(Formula::Const(78.0)),
                Box::new(Formula::Datum(Datum::Weight)),
            ),
        ])
    }

    #[test]
    fn calculator_asks_exactly_the_characteristics() {
        let formula = movement_formula();
        let mut custodian = RecordingCustodian::new();
        let situation = RecordingSituation::new(180.0, 78.0);
        let _ = Calculator::apply(&formula, &mut custodian, &situation);
        let mut asked = custodian.calls.clone();
        asked.sort_by_key(|c| c.name());
        asked.dedup();
        assert_eq!(asked, vec![Characteristic::ExplosiveForce]);
    }

    #[test]
    fn calculator_asks_exactly_the_data() {
        let formula = movement_formula();
        let mut custodian = RecordingCustodian::new();
        let situation = RecordingSituation::new(180.0, 78.0);
        let _ = Calculator::apply(&formula, &mut custodian, &situation);
        let asked = situation.calls.borrow();
        assert_eq!(*asked, vec![Datum::Weight]);
    }

    #[test]
    fn calculator_does_not_ask_for_a_term_not_declared() {
        let formula = movement_formula();
        let mut custodian = RecordingCustodian::new();
        let situation = RecordingSituation::new(180.0, 78.0);
        let _ = Calculator::apply(&formula, &mut custodian, &situation);
        assert!(!custodian.calls.contains(&Characteristic::MaxSpeed));
        assert!(!situation.calls.borrow().contains(&Datum::Height));
    }

    #[test]
    fn ratio_compensates_mass() {
        let formula = movement_formula();
        let mut light_c = RecordingCustodian::new();
        let light_s = RecordingSituation::new(180.0, 78.0);
        let a_light = Calculator::apply(&formula, &mut light_c, &light_s);

        let mut heavy_c = RecordingCustodian::new();
        let heavy_s = RecordingSituation::new(180.0, 97.5);
        let a_heavy = Calculator::apply(&formula, &mut heavy_c, &heavy_s);

        assert!(
            a_heavy < a_light,
            "heavier must accelerate less: {a_heavy} vs {a_light}"
        );
    }

    #[test]
    fn empty_formula_is_rejected() {
        let formula = Formula::Const(1.0);
        assert!(matches!(
            formula.validate(),
            Err(CalculatorError::EmptyFormula)
        ));
    }

    #[test]
    fn catalogue_names_roundtrip() {
        for c in [
            Characteristic::ExplosiveForce,
            Characteristic::MaximalForce,
            Characteristic::Acceleration,
            Characteristic::MaxSpeed,
            Characteristic::GameReading,
            Characteristic::DecisionMaking,
            Characteristic::RiskManagement,
        ] {
            assert_eq!(Characteristic::from_name(c.name()), Some(c));
        }
        for d in [
            Datum::Height,
            Datum::Weight,
            Datum::Fatigue,
            Datum::Crowding,
            Datum::NearestThreat,
        ] {
            assert_eq!(Datum::from_name(d.name()), Some(d));
        }
    }

    #[test]
    fn unknown_name_is_rejected() {
        assert_eq!(Characteristic::from_name("strength"), None);
        assert_eq!(Datum::from_name("strength"), None);
        assert_eq!(Characteristic::from_name("height"), None);
        assert_eq!(Datum::from_name("explosive_force"), None);
    }

    #[test]
    fn sum_adds_left_to_right() {
        struct NoTerms;
        impl Custodian for NoTerms {
            fn value(&mut self, _: Characteristic) -> f64 {
                panic!("no characteristic expected")
            }
        }
        struct NoData;
        impl Situation for NoData {
            fn datum(&self, _: Datum) -> f64 {
                panic!("no datum expected")
            }
        }
        let formula = Formula::Sum(vec![Formula::Const(2.0), Formula::Const(3.5)]);
        let v = Calculator::apply(&formula, &mut NoTerms, &NoData);
        assert!((v - 5.5).abs() < 1e-9, "sum: {v}");
    }

    #[test]
    fn sum_inside_clamp_models_a_modifier() {
        // `max(floor, 1 - c*x)` is the shape of the fatigue modifier
        // (ADR-030): a multiplicative floor. Here x is Weight (a datum we
        // control), c = 0.5, floor = 0.5.
        struct NoTerms;
        impl Custodian for NoTerms {
            fn value(&mut self, _: Characteristic) -> f64 {
                panic!("no characteristic expected")
            }
        }
        struct Weight(f64);
        impl Situation for Weight {
            fn datum(&self, _: Datum) -> f64 {
                self.0
            }
        }
        let formula = Formula::Clamp(
            Box::new(Formula::Sum(vec![
                Formula::Const(1.0),
                Formula::Product(vec![Formula::Const(-0.5), Formula::Datum(Datum::Weight)]),
            ])),
            0.5,
            1.0,
        );
        let fresh = Calculator::apply(&formula, &mut NoTerms, &Weight(0.0));
        let tired = Calculator::apply(&formula, &mut NoTerms, &Weight(1.0));
        let floored = Calculator::apply(&formula, &mut NoTerms, &Weight(5.0));
        assert!((fresh - 1.0).abs() < 1e-9, "fresh: {fresh}");
        assert!((tired - 0.5).abs() < 1e-9, "tired: {tired}");
        assert!((floored - 0.5).abs() < 1e-9, "floor should bind: {floored}");
    }
}
