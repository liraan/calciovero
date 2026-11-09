//! RNG — deterministic, isolated, named random streams.
//!
//! See `specs/L0-match/01-rng.md` for the full specification.

use std::collections::BTreeMap;

use rand_chacha::ChaCha20Rng;
use rand_core::{RngCore, SeedableRng};

/// A named random stream.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StreamName(String);

impl StreamName {
    /// Creates a new `StreamName` from the given name.
    ///
    /// # Errors
    ///
    /// Returns [`RngError::EmptyStreamName`] if the name is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use ds_core::rng::StreamName;
    ///
    /// let name = StreamName::new("match:abc:micro-decisions").unwrap();
    /// assert_eq!(name.as_str(), "match:abc:micro-decisions");
    /// ```
    pub fn new(name: impl Into<String>) -> Result<Self, RngError> {
        let name = name.into();
        if name.is_empty() {
            return Err(RngError::EmptyStreamName);
        }
        Ok(Self(name))
    }

    /// Returns the name as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Errors that can occur in the RNG module.
#[derive(Debug, thiserror::Error)]
pub enum RngError {
    /// The stream name is empty.
    #[error("stream name must not be empty")]
    EmptyStreamName,
}

/// A registry of named random streams, all derived from a single `world_seed`.
pub struct RngRegistry {
    world_seed: u64,
    streams: BTreeMap<StreamName, ChaCha20Rng>,
}

impl RngRegistry {
    /// Creates a new `RngRegistry` with the given `world_seed`.
    #[must_use]
    pub const fn new(world_seed: u64) -> Self {
        Self {
            world_seed,
            streams: BTreeMap::new(),
        }
    }

    /// Returns the stream with the given name, creating it if it does not exist.
    pub fn stream(&mut self, name: &StreamName) -> &mut ChaCha20Rng {
        let world_seed = self.world_seed;
        self.streams
            .entry(name.clone())
            .or_insert_with(|| ChaCha20Rng::from_seed(derive_seed(world_seed, name)))
    }
}

/// Returns a uniform random number in `[0, 1)` from `rng`.
///
/// The `u32 -> f32` cast is intentional and its precision loss is irrelevant
/// in this context (a random value in `[0, 1)`).
#[allow(clippy::cast_precision_loss)]
#[must_use]
fn derive_seed(world_seed: u64, name: &StreamName) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&world_seed.to_le_bytes());
    hasher.update(name.as_str().as_bytes());
    *hasher.finalize().as_bytes()
}

/// Returns a uniform random number in `[0, 1)` from `rng`.
///
/// The value is taken from the top 24 bits of a `u32` (f32 has a 24-bit
/// mantissa), so every value is exactly representable and the maximum is
/// `1 - 2^-24`, strictly below 1.0. See OI-58 for why the previous form
/// could return exactly 1.0.
#[allow(clippy::cast_precision_loss)]
pub fn next_unit(rng: &mut ChaCha20Rng) -> f32 {
    // Take the top 24 bits (f32 has a 24-bit mantissa) and divide by 2^24.
    // Every value is exactly representable in f32, the maximum is
    // (2^24 - 1) / 2^24 = 1 - 2^-24, strictly below 1.0. The previous
    // form (`next_u32 / (u32::MAX as f32)`) could return exactly 1.0
    // (OI-58): the value was fine in f64 but rounded to 1.0 in f32.
    let bits = rng.next_u32() >> 8;
    #[allow(clippy::cast_precision_loss)]
    let unit = (bits as f32) * (1.0 / (1u32 << 24) as f32);
    unit
}

#[cfg(test)]
mod tests {
    use super::*;

    fn first_n_bytes(rng: &mut ChaCha20Rng, n: usize) -> Vec<u8> {
        let mut buf = vec![0_u8; n];
        rng.fill_bytes(&mut buf);
        buf
    }

    #[test]
    fn empty_stream_name_is_rejected() {
        assert!(matches!(
            StreamName::new(""),
            Err(RngError::EmptyStreamName)
        ));
    }

    #[test]
    fn same_seed_same_sequence() {
        let name = StreamName::new("test").unwrap();
        let mut a = RngRegistry::new(42);
        let mut b = RngRegistry::new(42);
        let seq_a = first_n_bytes(a.stream(&name), 100);
        let seq_b = first_n_bytes(b.stream(&name), 100);
        assert_eq!(seq_a, seq_b);
    }

    #[test]
    fn different_seed_different_sequence() {
        let name = StreamName::new("test").unwrap();
        let mut a = RngRegistry::new(42);
        let mut b = RngRegistry::new(43);
        let seq_a = first_n_bytes(a.stream(&name), 100);
        let seq_b = first_n_bytes(b.stream(&name), 100);
        assert_ne!(seq_a, seq_b);
    }

    #[test]
    fn different_names_different_sequences() {
        let mut r = RngRegistry::new(42);
        let a = StreamName::new("a").unwrap();
        let b = StreamName::new("b").unwrap();
        let seq_a = first_n_bytes(r.stream(&a), 100);
        let seq_b = first_n_bytes(r.stream(&b), 100);
        assert_ne!(seq_a, seq_b);
    }

    #[test]
    fn adding_a_stream_does_not_shift_others() {
        let name = StreamName::new("test").unwrap();
        let mut a = RngRegistry::new(42);
        let seq_a = first_n_bytes(a.stream(&name), 100);
        let mut b = RngRegistry::new(42);
        let other = StreamName::new("other").unwrap();
        let _ = first_n_bytes(b.stream(&other), 50);
        let seq_b = first_n_bytes(b.stream(&name), 100);
        assert_eq!(seq_a, seq_b);
    }

    #[test]
    fn stream_is_idempotent() {
        let name = StreamName::new("test").unwrap();
        let mut r = RngRegistry::new(42);
        let _ = first_n_bytes(r.stream(&name), 10);
        let seq_1 = first_n_bytes(r.stream(&name), 100);
        let mut r2 = RngRegistry::new(42);
        let _ = first_n_bytes(r2.stream(&name), 10);
        let seq_2 = first_n_bytes(r2.stream(&name), 100);
        assert_eq!(seq_1, seq_2);
    }

    #[test]
    fn next_unit_is_in_range() {
        let mut r = RngRegistry::new(42);
        let name = StreamName::new("test").unwrap();
        let rng = r.stream(&name);
        for _ in 0..10_000 {
            let v = next_unit(rng);
            assert!((0.0..1.0).contains(&v), "next_unit out of [0, 1): {v}");
        }
    }

    #[test]
    fn next_unit_never_returns_one_on_the_edge() {
        // The maximum value of `bits` is 2^24 - 1. Divided by 2^24 it is
        // 1 - 2^-24, exactly representable in f32 and strictly below 1.0.
        // The previous form (`next_u32 / (u32::MAX as f32)`) rounded to
        // exactly 1.0 in f32 for a `next_u32` of `u32::MAX` (OI-58).
        let max_bits = (1u32 << 24) - 1;
        #[allow(clippy::cast_precision_loss)]
        let max_unit = (max_bits as f32) * (1.0 / (1u32 << 24) as f32);
        assert!(max_unit < 1.0, "the edge case is not below 1.0: {max_unit}");
        #[allow(clippy::cast_precision_loss)]
        let expected = 1.0 - (1.0 / (1u32 << 24) as f32);
        assert_eq!(max_unit, expected);
    }
}
