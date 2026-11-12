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

/// Derives a value in `[0, 1)` from `(world_seed, player, purpose)`.
///
/// The keyed derivation (ADR-029, OI-53). Not a die: the same world and the
/// same player always give the same value, whenever it is read and
/// however many players were derived before.
///
/// BLAKE3 over the world seed, the player's slot and generation, and the
/// purpose. Shared by the vault (the hidden characteristics) and by
/// whoever builds the visible body of a player (height, weight).
#[must_use]
pub fn unit_from_key(world_seed: u64, player: crate::ids::PlayerId, purpose: &str) -> f32 {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&world_seed.to_le_bytes());
    hasher.update(b"player:");
    hasher.update(&player.slot().to_le_bytes());
    hasher.update(&player.generation().to_le_bytes());
    hasher.update(b"purpose:");
    hasher.update(purpose.as_bytes());
    let digest = hasher.finalize();
    let bytes = digest.as_bytes();
    let raw = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    // `[0, 1)`: 1.0 is never produced.
    #[allow(clippy::cast_possible_truncation)]
    let unit = (f64::from(raw) / f64::from(u32::MAX) / 1.000_000_1) as f32;
    unit
}
