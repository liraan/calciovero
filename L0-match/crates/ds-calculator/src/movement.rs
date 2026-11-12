//! The movement formula: the first formula of the new engine (F.7).
//!
//! The phenomenon, in a real match: a stronger player accelerates more,
//! a heavier one less, and the two effects compensate in ratio, they do
//! not add up. The grammar is `product`, `ratio`, `clamp` (L0.11 §4.5).
//!
//! The formula, in the grammar:
//!
//! `a = accel_ref * explosive_force * (weight_ref / weight)`
//!
//! The constants come from `L0-match/balance/movement.json`, which is embedded at
//! compile time: the engine performs no I/O. A balance change is a
//! `SIM_VERSION` bump (CHANGELOG-SIM).
//!
//! **The formula is built once, in `load()`** (2026-11-09, F.2-bis
//! bench). `formula()` returns a reference. Building it per call
//! allocated a `Vec` and two `Box` on every `resolve_movement`; the
//! bench measured 170 ns of the 245 ns per call in that allocation. The
//! formula is a rule, not a per-scene value: it is built with the
//! balance, once.

use serde::Deserialize;

use crate::{Characteristic, Datum, Formula};

/// The embedded balance file.
const MOVEMENT_JSON: &str = include_str!("../../../balance/movement.json");

/// The constants of the movement formula, and the formula itself.
#[derive(Debug, Clone)]
pub struct MovementBalance {
    /// Reference acceleration, in cm/tick².
    pub accel_ref_cm_per_tick2: f64,
    /// Reference top speed, in cm/tick.
    pub v_max_ref_cm_per_tick: f64,
    /// Reference weight, in kg.
    pub weight_ref_kg: f64,
    /// Reference height, in cm.
    pub height_ref_cm: f64,
    /// Weight of the fatigue term (ADR-030; prototype, not calibrated).
    pub fatigue_coefficient: f64,
    /// Floor of the fatigue modifier: 1 - c*fatigue is clamped here.
    pub fatigue_floor: f64,
    /// The formula, built once from the constants above.
    formula: Formula,
}

/// The JSON shape, parsed before the formula is built.
#[derive(Debug, Clone, Deserialize)]
struct MovementBalanceRaw {
    accel_ref_cm_per_tick2: f64,
    v_max_ref_cm_per_tick: f64,
    weight_ref_kg: f64,
    height_ref_cm: f64,
    fatigue_coefficient: f64,
    fatigue_floor: f64,
}

impl MovementBalance {
    /// Loads the embedded balance file and builds the formula once.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid. The test
    /// `embedded_movement_balance_is_valid` catches this at test time.
    #[must_use]
    pub fn load() -> Self {
        let raw: MovementBalanceRaw =
            serde_json::from_str(MOVEMENT_JSON).expect("embedded movement balance is valid JSON");
        let formula = Self::build_formula(&raw);
        Self {
            accel_ref_cm_per_tick2: raw.accel_ref_cm_per_tick2,
            v_max_ref_cm_per_tick: raw.v_max_ref_cm_per_tick,
            weight_ref_kg: raw.weight_ref_kg,
            height_ref_cm: raw.height_ref_cm,
            fatigue_coefficient: raw.fatigue_coefficient,
            fatigue_floor: raw.fatigue_floor,
            formula,
        }
    }

    /// The movement formula, built once in `load()`.
    ///
    /// `a = accel_ref * explosive_force * (weight_ref / weight) * max(floor, 1 - c*fatigue)`
    ///
    /// The fatigue modifier is ADR-030 (one scalar, multiplicative with a
    /// floor). It is identity at fatigue = 0, so the golden is unchanged
    /// when the match never accumulates fatigue (F.7, no producer yet).
    #[must_use]
    pub const fn formula(&self) -> &Formula {
        &self.formula
    }

    fn build_formula(b: &MovementBalanceRaw) -> Formula {
        Formula::Product(vec![
            Formula::Const(b.accel_ref_cm_per_tick2),
            Formula::Term(Characteristic::ExplosiveForce),
            Formula::Ratio(
                Box::new(Formula::Const(b.weight_ref_kg)),
                Box::new(Formula::Datum(Datum::Weight)),
            ),
            Formula::Clamp(
                Box::new(Formula::Sum(vec![
                    Formula::Const(1.0),
                    Formula::Product(vec![
                        Formula::Const(-b.fatigue_coefficient),
                        Formula::Datum(Datum::Fatigue),
                    ]),
                ])),
                b.fatigue_floor,
                1.0,
            ),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_movement_balance_is_valid() {
        let b = MovementBalance::load();
        assert!(b.accel_ref_cm_per_tick2 > 0.0);
        assert!(b.v_max_ref_cm_per_tick > 0.0);
        assert!(b.weight_ref_kg > 0.0);
        assert!(b.height_ref_cm > 0.0);
    }

    #[test]
    fn movement_formula_declares_force_and_weight() {
        let b = MovementBalance::load();
        let f = b.formula();
        let chars = f.declared_characteristics();
        let data = f.declared_data();
        assert_eq!(chars, vec![Characteristic::ExplosiveForce]);
        assert_eq!(data, vec![Datum::Weight, Datum::Fatigue]);
    }

    #[test]
    fn movement_formula_validates() {
        let b = MovementBalance::load();
        assert!(b.formula().validate().is_ok());
    }
}
