//! The static obstacle's reflection (ADR-048 §1).
//!
//! The ball's trajectory meets the obstacle's geometry; the effect is a
//! declared reflection: the component of the velocity normal to the
//! obstacle is reversed and scaled by the combined elasticity. Integer,
//! deterministic, no die (ADR-035, ADR-044 §5).

/// A declared obstacle's reflection parameters (ADR-048 §1).
///
/// The obstacle is a plane (a post's face, the crossbar's underside). The
/// normal is the axis along which the ball's velocity is reversed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObstacleReflection {
    /// The obstacle's normal axis: 0 = x, 1 = y, 2 = z.
    pub axis: u8,
    /// The elasticity numerator (`elasticity_num / elasticity_den`).
    pub elasticity_num: i32,
    /// The elasticity denominator.
    pub elasticity_den: i32,
}

impl ObstacleReflection {
    /// Applies the reflection to `velocity`: the component on `axis` is
    /// reversed and scaled by the elasticity; the other components are
    /// unchanged. The ball keeps moving after the touch (a post is not a
    /// catch).
    ///
    /// # Panics
    ///
    /// Panics if `axis` is not 0, 1 or 2, or if `elasticity_den` is zero.
    /// The values come from a declared `L0-match/balance/` file, so an invalid value
    /// is a bug, not a runtime input.
    #[must_use]
    pub fn reflect(&self, velocity: (i16, i16, i16)) -> (i16, i16, i16) {
        assert!(self.axis <= 2, "obstacle axis must be 0, 1 or 2");
        assert!(self.elasticity_den != 0, "elasticity_den must not be zero");
        let apply = |v: i16| -> i16 {
            let reflected = -(i32::from(v) * self.elasticity_num / self.elasticity_den);
            // Saturate toward the sign of the result (an elasticity > 1
            // could overflow `i16`): a reversed velocity keeps its new sign.
            i16::try_from(reflected).unwrap_or(if reflected > 0 { i16::MAX } else { i16::MIN })
        };
        match self.axis {
            0 => (apply(velocity.0), velocity.1, velocity.2),
            1 => (velocity.0, apply(velocity.1), velocity.2),
            _ => (velocity.0, velocity.1, apply(velocity.2)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ObstacleReflection;

    #[test]
    fn reflect_reverses_the_normal_component_and_keeps_the_others() {
        let o = ObstacleReflection {
            axis: 0,
            elasticity_num: 8,
            elasticity_den: 10,
        };
        let v = o.reflect((100, 30, -20));
        assert_eq!(v, (-80, 30, -20));
    }

    #[test]
    fn reflect_on_z_flips_vz_only() {
        let o = ObstacleReflection {
            axis: 2,
            elasticity_num: 5,
            elasticity_den: 10,
        };
        assert_eq!(o.reflect((10, 20, 40)), (10, 20, -20));
    }

    #[test]
    fn an_overflowing_reflection_saturates_with_the_reversed_sign() {
        let o = ObstacleReflection {
            axis: 0,
            elasticity_num: 2,
            elasticity_den: 1,
        };
        assert_eq!(o.reflect((20_000, 0, 0)).0, i16::MIN);
        assert_eq!(o.reflect((-20_000, 0, 0)).0, i16::MAX);
    }

    #[test]
    fn a_zero_velocity_stays_zero() {
        let o = ObstacleReflection {
            axis: 1,
            elasticity_num: 8,
            elasticity_den: 10,
        };
        assert_eq!(o.reflect((0, 0, 0)), (0, 0, 0));
    }
}
