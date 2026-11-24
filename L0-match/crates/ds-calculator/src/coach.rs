//! The coach's formulas (ADR-114): an ability is a formula of the roots.

/// How well a player fits a role (ADR-114 §3).
///
/// From the role's profile (`balance/profiles.json`: the mean and the
/// spread of each characteristic in that role) and his values, as
/// `(mean, spread, value)`: first how much he falls short of what the role
/// asks, in spreads (the less, the better); then how much he goes beyond it.
/// The pair orders the candidates: `(-shortfall, surplus)`, the greater the
/// better. No weight of its own. (A mean weighted by the role's profile
/// put an all-round midfielder in goal: nothing he lacked counted enough.)
#[must_use]
pub fn role_fit(profile: &[(f64, f64, f64)]) -> (f64, f64) {
    let mut shortfall = 0.0;
    let mut surplus = 0.0;
    for &(mean, spread, value) in profile {
        let z = (value - mean) / spread.max(1e-6);
        if z < 0.0 {
            shortfall -= z;
        } else {
            surplus += z;
        }
    }
    (-shortfall, surplus)
}

/// A root of the coach's game idea in `[0, 1)`, in three thirds (ADR-114
/// §4): 0 the low (deep, narrow), 1 the middle, 2 the high (pressing, wide).
#[must_use]
pub fn third(root: f64) -> u8 {
    if root < 1.0 / 3.0 {
        0
    } else if root < 2.0 / 3.0 {
        1
    } else {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::{role_fit, third};

    /// A keeper fits in goal better than an all-round man short of a
    /// keeper's hands; beyond the profile, more is better.
    #[test]
    fn the_fit_is_first_what_he_lacks() {
        // (mean, spread) of two characteristics of the role: goalkeeping,
        // passing.
        let keeper = role_fit(&[(0.75, 0.1, 0.8), (0.45, 0.12, 0.4)]);
        let all_round = role_fit(&[(0.75, 0.1, 0.3), (0.45, 0.12, 0.9)]);
        assert!(keeper > all_round, "{keeper:?} {all_round:?}");
        let better = role_fit(&[(0.75, 0.1, 0.9), (0.45, 0.12, 0.5)]);
        assert!(better > keeper);
        assert_eq!((third(0.1), third(0.5), third(0.9)), (0, 1, 2));
    }
}
