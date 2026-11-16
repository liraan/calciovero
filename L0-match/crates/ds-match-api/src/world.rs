//! The observable state of the match as a plugin sees it (ADR-063 §1).
//!
//! The core implements [`World`]; a plugin reads and writes through it,
//! never through the engine's own types, so that the same plugin can be
//! compiled in or loaded from outside (decision 4 of 2026-11-15). The
//! methods enter with the plugin that needs them (RULES D3): today the
//! fatigue's and the perception's.

/// A player of the match, as the contract names him: an opaque key the
/// core gives and reads back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlayerKey(pub u64);

/// What a plugin can read and write of the match in progress.
pub trait World {
    /// The players on the pitch, in a fixed order.
    fn players(&self) -> Vec<PlayerKey>;
    /// Where `player` is, cm.
    fn position(&self, player: PlayerKey) -> Option<(i32, i32)>;
    /// Where `player` looks, degrees clockwise from north.
    fn gaze(&self, player: PlayerKey) -> Option<u16>;
    /// How fast `player` moved this tick, cm a tick, per axis.
    fn velocity(&self, player: PlayerKey) -> Option<(f64, f64)>;
    /// How tired `player` is, 0 to 1.
    fn fatigue(&self, player: PlayerKey) -> Option<f64>;
    /// Sets how tired `player` is.
    fn set_fatigue(&mut self, player: PlayerKey, fatigue: f64);
    /// One of `player`'s characteristics, 0 to 1, by its name in the
    /// catalogue (`CHARACTERISTICS-AND-ABILITIES.md`).
    fn characteristic(&mut self, player: PlayerKey, name: &str) -> Option<f64>;

    /// The minute of the match, as the broadcast clock shows it.
    fn minute(&self) -> u64 {
        0
    }
    /// Who has the ball at his feet, if anyone.
    fn carrier(&self) -> Option<PlayerKey> {
        None
    }
    /// The eleven of a side as they are now, the keeper first.
    fn lineup(&self, home: bool) -> Vec<PlayerKey> {
        let _ = home;
        Vec::new()
    }
    /// The bench of a side.
    fn bench(&self, home: bool) -> Vec<PlayerKey> {
        let _ = home;
        Vec::new()
    }
    /// The role of `player` (the registry's name for it).
    fn role(&self, player: PlayerKey) -> Option<String> {
        let _ = player;
        None
    }
    /// Whether `player` has already been replaced.
    fn substituted(&self, player: PlayerKey) -> bool {
        let _ = player;
        false
    }
    /// The substitutions the Laws and the competition allow a side.
    fn subs_allowed(&self) -> usize {
        0
    }
    /// The substitutions a side's manager planned: minute, off, on.
    fn planned(&self, home: bool) -> Vec<(u16, PlayerKey, PlayerKey)> {
        let _ = home;
        Vec::new()
    }
    /// The substitutions a side's manager ordered during the match and
    /// not yet made: off, on (decision T).
    fn ordered(&self, home: bool) -> Vec<(PlayerKey, PlayerKey)> {
        let _ = home;
        Vec::new()
    }
    /// A player's condition, `[0, 1]` (decision AB): his endurance counts
    /// for this share.
    fn condition(&self, player: PlayerKey) -> f64 {
        let _ = player;
        1.0
    }
    /// A side's goals so far.
    fn goals(&self, home: bool) -> u8 {
        let _ = home;
        0
    }
    /// Whether a player is on the pitch (not sent off, not replaced).
    fn on_pitch(&self, player: PlayerKey) -> bool {
        let _ = player;
        true
    }
    /// The men of a side a knock forced off who are still on the pitch
    /// (decision Y).
    fn knocked(&self, home: bool) -> Vec<PlayerKey> {
        let _ = home;
        Vec::new()
    }
    /// A man who cannot go on and cannot be replaced leaves the pitch: his
    /// side plays a man short (Law 3).
    fn carry_off(&mut self, player: PlayerKey) {
        let _ = player;
    }
    /// Whether a side's coach changes players from what he sees.
    fn coach_reacts(&self, home: bool) -> bool {
        let _ = home;
        false
    }
    /// How fast `player` can run, as tired as `fatigue`, cm a tick.
    fn top_speed(&mut self, player: PlayerKey, fatigue: f64) -> Option<f64> {
        let _ = (player, fatigue);
        None
    }
    /// Replaces `off` with `on` from the bench (Law 3); whether it was made.
    fn substitute(&mut self, off: PlayerKey, on: PlayerKey) -> bool {
        let _ = (off, on);
        false
    }
}
