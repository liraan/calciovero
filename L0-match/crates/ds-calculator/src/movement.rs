//! The movement formula: the first formula of the new engine (F.7).
//!
//! The phenomenon, in a real match: a stronger player accelerates more,
//! a heavier one less, and the two effects compensate in ratio, they do
//! not add up. The grammar is `product`, `ratio`, `clamp` (L0.11 §4.5).
//!
//! The formula, in the grammar:
//!
//! `a = (accel_min + (accel_max - accel_min) * explosive_force) * (weight_ref / weight)`
//!
//! The range is a professional footballer's (2026-11-13): explosive force
//! places him in it, never at zero (before, an average player accelerated
//! at half the reference and reached 4.5 m/s).
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
    /// The acceleration of the least explosive professional, cm/tick².
    pub accel_min_cm_per_tick2: f64,
    /// The acceleration of the most explosive professional, cm/tick².
    pub accel_max_cm_per_tick2: f64,
    /// The top speed of the slowest professional, cm/tick.
    pub top_speed_min_cm_per_tick: f64,
    /// The top speed of the fastest professional, cm/tick.
    pub top_speed_max_cm_per_tick: f64,
    /// The braking of the least balanced professional, cm/tick².
    pub decel_min_cm_per_tick2: f64,
    /// The braking of the most balanced professional, cm/tick².
    pub decel_max_cm_per_tick2: f64,
    /// A player taking up his place gets there in about this many ticks.
    pub reposition_ticks: f64,
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
    accel_min_cm_per_tick2: f64,
    accel_max_cm_per_tick2: f64,
    top_speed_min_cm_per_tick: f64,
    top_speed_max_cm_per_tick: f64,
    decel_min_cm_per_tick2: f64,
    decel_max_cm_per_tick2: f64,
    reposition_ticks: f64,
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
            accel_min_cm_per_tick2: raw.accel_min_cm_per_tick2,
            accel_max_cm_per_tick2: raw.accel_max_cm_per_tick2,
            top_speed_min_cm_per_tick: raw.top_speed_min_cm_per_tick,
            top_speed_max_cm_per_tick: raw.top_speed_max_cm_per_tick,
            decel_min_cm_per_tick2: raw.decel_min_cm_per_tick2,
            decel_max_cm_per_tick2: raw.decel_max_cm_per_tick2,
            reposition_ticks: raw.reposition_ticks,
            weight_ref_kg: raw.weight_ref_kg,
            height_ref_cm: raw.height_ref_cm,
            fatigue_coefficient: raw.fatigue_coefficient,
            fatigue_floor: raw.fatigue_floor,
            formula,
        }
    }

    /// The movement formula, built once in `load()`.
    ///
    /// `a = (accel_min + range * explosive_force) * (weight_ref / weight) * max(floor, 1 - c*fatigue)`
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
            Formula::Sum(vec![
                Formula::Const(b.accel_min_cm_per_tick2),
                Formula::Product(vec![
                    Formula::Const(b.accel_max_cm_per_tick2 - b.accel_min_cm_per_tick2),
                    Formula::Term(Characteristic::ExplosiveForce),
                ]),
            ]),
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
        assert!(
            b.accel_min_cm_per_tick2 > 0.0 && b.accel_max_cm_per_tick2 > b.accel_min_cm_per_tick2
        );
        assert!(b.top_speed_max_cm_per_tick > b.top_speed_min_cm_per_tick);
        assert!(b.decel_max_cm_per_tick2 > b.decel_min_cm_per_tick2);
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
