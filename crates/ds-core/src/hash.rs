//! Hash — a 32-byte BLAKE3 digest.
//!
//! Used for the `EventChain` hash chain (see `ds-stats::chain`), for
//! `observed_attrs_snapshot_hash`, and for any other place where a
//! deterministic, collision-resistant digest is needed.
//!
//! BLAKE3 is chosen for the same reason as the seed derivation in
//! `ds-core::rng`: fast, deterministic, cryptographic.

use serde::{Deserialize, Serialize};

/// A 32-byte BLAKE3 digest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Hash([u8; 32]);

impl Hash {
    /// The zero hash. Used as the starting value of an event chain.
    pub const ZERO: Self = Self([0_u8; 32]);

    /// Computes the BLAKE3 hash of `data`.
    #[must_use]
    pub fn of_bytes(data: &[u8]) -> Self {
        Self(*blake3::hash(data).as_bytes())
    }

    /// Returns the hash as a byte array.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}
