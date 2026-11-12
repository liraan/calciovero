//! The ground duel's declared parameters (`balance/duel.json`, ADR-042).
//!
//! The contact is deterministic: no die (ADR-035, ADR-042 §5). The weights
//! are data, so a volunteer can change how much tackling counts against
//! keeping the ball without touching the code.

use serde::Deserialize;

const DUEL_JSON: &str = include_str!("../../../balance/duel.json");

/// The ground duel's parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct DuelBalance {
    /// An opponent this close to the ball challenges the carrier, cm.
    pub reach_cm: i32,
    /// An opponent challenges at most once every this many ticks.
    pub retry_ticks: u64,
    /// Weight of the defender's tackling, thousandths.
    pub defender_tackling_milli: i32,
    /// Weight of the defender's explosive force, thousandths.
    pub defender_explosive_milli: i32,
    /// Weight of the carrier's first touch, thousandths.
    pub carrier_first_touch_milli: i32,
    /// Weight of the carrier's explosive force, thousandths.
    pub carrier_explosive_milli: i32,
}

impl DuelBalance {
    /// Loads the embedded duel parameters.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid; `embedded_duel_is_valid`
    /// catches it at test time.
    #[must_use]
    pub fn load() -> Self {
        serde_json::from_str(DUEL_JSON).expect("embedded duel file is valid JSON")
    }

    /// The defender's contact force, from his tackling and explosive force.
    #[must_use]
    pub fn defender_force(&self, tackling: f32, explosive: f32) -> f32 {
        milli(self.defender_tackling_milli)
            .mul_add(tackling, milli(self.defender_explosive_milli) * explosive)
    }

    /// The carrier's contact force, from his first touch and explosive force.
    #[must_use]
    pub fn carrier_force(&self, first_touch: f32, explosive: f32) -> f32 {
        milli(self.carrier_first_touch_milli)
            .mul_add(first_touch, milli(self.carrier_explosive_milli) * explosive)
    }
}

fn milli(v: i32) -> f32 {
    f32::from(i16::try_from(v).unwrap_or(i16::MAX)) / 1000.0
}

#[cfg(test)]
mod tests {
    use super::DuelBalance;

    #[test]
    fn embedded_duel_is_valid() {
        let b = DuelBalance::load();
        assert!(b.reach_cm > 0 && b.retry_ticks > 0);
    }

    /// A better tackler wins more contacts: the force grows with tackling.
    #[test]
    fn tackling_counts() {
        let b = DuelBalance::load();
        assert!(b.defender_force(0.9, 0.5) > b.defender_force(0.3, 0.5));
    }
}
