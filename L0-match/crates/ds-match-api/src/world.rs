//! The observable state of the match as a plugin sees it (ADR-063 §1).
//!
//! The core implements [`World`]; a plugin reads and writes through it,
//! never through the engine's own types, so that the same plugin can be
//! compiled in or loaded from outside (decision 4 of 2026-11-15). The
//! methods enter with the plugin that needs them (RULES D3): today the
//! fatigue's.

/// A player of the match, as the contract names him: an opaque key the
/// core gives and reads back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlayerKey(pub u64);

/// What a plugin can read and write of the match in progress.
pub trait World {
    /// The players on the pitch, in a fixed order.
    fn players(&self) -> Vec<PlayerKey>;
    /// How fast `player` moved this tick, cm a tick, per axis.
    fn velocity(&self, player: PlayerKey) -> Option<(f64, f64)>;
    /// How tired `player` is, 0 to 1.
    fn fatigue(&self, player: PlayerKey) -> Option<f64>;
    /// Sets how tired `player` is.
    fn set_fatigue(&mut self, player: PlayerKey, fatigue: f64);
    /// One of `player`'s characteristics, 0 to 1, by its name in the
    /// catalogue (`CHARACTERISTICS-AND-ABILITIES.md`).
    fn characteristic(&mut self, player: PlayerKey, name: &str) -> Option<f64>;
}
