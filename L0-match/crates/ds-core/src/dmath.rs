//! Deterministic floating-point math: the same bits on every platform.
//!
//! The engine is deterministic by construction (`00-foundations` §4):
//! the same save gives the same match, on every machine. The arithmetic
//! of IEEE 754 (`+ - * /`, `sqrt`, `mul_add`) is correctly rounded, so it
//! gives the same last bit everywhere. The transcendental functions of
//! `std` (`hypot`, `ln`, `powf`, `sin`, ...) are not: they call the C
//! library of the system (glibc and its versions, the Windows CRT,
//! macOS's libm), and each may round the last bit its own way. One bit
//! is enough: a player reaches a ball one tick earlier, and the match
//! goes another way (`SIM_VERSION` 101: seed 42 ended 2-2 with glibc and
//! 3-5 with musl, from 2'03").
//!
//! Every such function the simulation needs lives here, either as
//! correctly rounded arithmetic (`hypot`) or from the `libm` crate, pure
//! Rust, the same code and the same bits on every target. `clippy.toml`
//! forbids the `std` versions in the workspace (gate C2), so they cannot
//! come back. A function enters here with its first consumer (RULES D3).

/// The length of `(x, y)`: `sqrt(x² + y²)`, correctly rounded steps only
/// (Rust never fuses `x * x + y * y` into one operation).
///
/// Not `f64::hypot`, which avoids overflow by a platform-dependent
/// algorithm. On a pitch measured in centimetres the squares are far
/// from overflow (a speed or a distance of 10⁵ squares to 10¹⁰).
#[must_use]
#[allow(
    clippy::suboptimal_flops,
    clippy::imprecise_flops,
    reason = "f64::hypot is platform-dependent: this replaces it"
)]
pub fn hypot(x: f64, y: f64) -> f64 {
    (x * x + y * y).sqrt()
}

/// The length of `(x, y, z)`: `sqrt(x² + y² + z²)`.
#[must_use]
#[allow(
    clippy::suboptimal_flops,
    clippy::imprecise_flops,
    reason = "f64::hypot is platform-dependent: this replaces it"
)]
pub fn hypot3(x: f64, y: f64, z: f64) -> f64 {
    (x * x + y * y + z * z).sqrt()
}

/// The natural logarithm of `x`, from `libm`.
#[must_use]
pub fn ln(x: f64) -> f64 {
    libm::log(x)
}

/// The logarithm of `x` in base `base`: `ln(x) / ln(base)`, as `f64::log`
/// computes it, with the logarithm from `libm`.
#[must_use]
pub fn log(x: f64, base: f64) -> f64 {
    ln(x) / ln(base)
}

/// `x` raised to `y`, from `libm`.
#[must_use]
pub fn powf(x: f64, y: f64) -> f64 {
    libm::pow(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hypot_is_the_length() {
        assert!((hypot(3.0, 4.0) - 5.0).abs() < f64::EPSILON);
        assert!((hypot3(2.0, 3.0, 6.0) - 7.0).abs() < f64::EPSILON);
        assert!(hypot(0.0, 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn the_logarithms_and_the_power_agree_with_their_definitions() {
        assert!((ln(core::f64::consts::E) - 1.0).abs() < 1e-15);
        assert!((log(8.0, 2.0) - 3.0).abs() < 1e-15);
        assert!((powf(2.0, 10.0) - 1024.0).abs() < 1e-12);
        assert!((powf(0.9, 0.5) - 0.9_f64.sqrt()).abs() < 1e-15);
    }

    /// The bits are the code's, not the machine's: fixed values of
    /// `libm` that every platform must reproduce (they are what the
    /// golden of `ds-match` rests on).
    #[test]
    fn the_bits_are_fixed() {
        const LN_097: u64 = 0xbf9f_30b2_d009_1d8a;
        const POW_097_73: u64 = 0x3fe9_9ec9_c4a0_cdc3;
        const HYPOT: u64 = 0x4096_0373_e576_2fe2;
        assert_eq!(ln(0.97).to_bits(), LN_097);
        assert_eq!(powf(0.97, 7.3).to_bits(), POW_097_73);
        assert_eq!(hypot(1234.5, 678.9).to_bits(), HYPOT);
    }
}
