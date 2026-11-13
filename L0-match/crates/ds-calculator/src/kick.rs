//! The kick's declared parameters and formula (`balance/kick.json`,
//! ADR-051, the family "the kick").
//!
//! **One responsibility:** how fast and how precisely a player kicks the
//! ball, from his characteristics and his choice of power. Pure; no die
//! (ADR-035). First consumer: the shot.

use serde::Deserialize;

const KICK_JSON: &str = include_str!("../../../balance/kick.json");

/// The kick's parameters.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct KickBalance {
    /// Speed of the strongest kick by the most explosive player, cm/tick.
    pub speed_ref_cm_per_tick: f64,
    /// Share of the speed a kick has at power 0.
    pub speed_power_base: f64,
    /// Share of the speed a player with no explosive force reaches.
    pub speed_explosive_base: f64,
    /// Angular error of the worst kicker at medium power, radians.
    pub error_ref_rad: f64,
    /// How much power widens the error.
    pub error_power_gain: f64,
    /// Angular error added by full pressure, radians.
    pub error_pressure_rad: f64,
    /// Share of the error that lifts the ball.
    pub vertical_share: f64,
    /// Height aimed at for a low shot, cm.
    pub aim_low_cm: i32,
    /// Height aimed at for a high shot, cm.
    pub aim_high_cm: i32,
    /// Speed a weighed pass still has when it reaches the receiver, cm/tick.
    pub pass_arrive_cm_per_tick: f64,
}

impl KickBalance {
    /// Loads the embedded kick parameters.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid; `embedded_kick_is_valid`
    /// catches it at test time.
    #[must_use]
    pub fn load() -> Self {
        serde_json::from_str(KICK_JSON).expect("embedded kick file is valid JSON")
    }

    /// The speed the ball leaves at, cm/tick, from the chosen power (0..1)
    /// and the kicker's explosive force.
    #[must_use]
    pub fn speed(&self, power: f64, explosive_force: f32) -> f64 {
        let p = (1.0 - self.speed_power_base).mul_add(power.clamp(0.0, 1.0), self.speed_power_base);
        let e = (1.0 - self.speed_explosive_base).mul_add(
            f64::from(explosive_force.clamp(0.0, 1.0)),
            self.speed_explosive_base,
        );
        self.speed_ref_cm_per_tick * p * e
    }

    /// The angular error of the kick, radians, from the kicker's shooting,
    /// the quality of the foot he kicks with, the power and the pressure
    /// (0..1).
    #[must_use]
    pub fn error_rad(&self, shooting: f32, foot: f32, power: f64, pressure: f64) -> f64 {
        let skill = f64::from(shooting.clamp(0.0, 1.0))
            * 0.5f64.mul_add(f64::from(foot.clamp(0.0, 1.0)), 0.5);
        let width = self.error_power_gain.mul_add(power.clamp(0.0, 1.0), 0.5);
        (self.error_ref_rad * (1.0 - skill) * width)
            .mul_add(1.0, self.error_pressure_rad * pressure.clamp(0.0, 1.0))
    }

    /// The height the shooter aims at, cm, from his contact choice (0 low,
    /// 1 high).
    #[must_use]
    pub fn aim_height_cm(&self, contact_height: f64) -> i32 {
        let span = f64::from(self.aim_high_cm - self.aim_low_cm);
        #[allow(clippy::cast_possible_truncation)]
        let h = span.mul_add(contact_height.clamp(0.0, 1.0), f64::from(self.aim_low_cm)) as i32;
        h
    }
}

#[cfg(test)]
mod tests {
    use super::KickBalance;

    #[test]
    fn embedded_kick_is_valid() {
        let b = KickBalance::load();
        assert!(b.speed_ref_cm_per_tick > 0.0 && b.error_ref_rad > 0.0);
        assert!(b.aim_low_cm < b.aim_high_cm);
    }

    /// Twin test (ADR-051 §3): two kickers identical but for explosive
    /// force; the explosive one's strong shot is at least 5 m/s faster.
    #[test]
    fn the_explosive_twin_kicks_harder() {
        let b = KickBalance::load();
        assert!(b.speed(0.85, 0.9) > b.speed(0.85, 0.2) + 50.0);
    }

    /// Twin test: the better shooter, and the one kicking with his good
    /// foot, misses by less; a strong shot is less precise than a placed
    /// one, and pressure widens the error.
    #[test]
    fn the_better_shooter_and_the_good_foot_miss_by_less() {
        let b = KickBalance::load();
        assert!(b.error_rad(0.9, 0.8, 0.35, 0.0) < b.error_rad(0.3, 0.8, 0.35, 0.0) / 2.0);
        assert!(b.error_rad(0.7, 0.9, 0.35, 0.0) < b.error_rad(0.7, 0.2, 0.35, 0.0));
        assert!(b.error_rad(0.7, 0.8, 0.35, 0.0) < b.error_rad(0.7, 0.8, 0.85, 0.0));
        assert!(b.error_rad(0.7, 0.8, 0.35, 0.0) < b.error_rad(0.7, 0.8, 0.35, 1.0));
    }
}
