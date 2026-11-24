//! The vault: Plane A, the hidden performances, and the custodian door.
//!
//! **One responsibility (ADR-041 §3):** hold the players' performances and
//! derive them by key (`PlayerLatents::from_key`). It does not decide, it
//! does not orchestrate, it does not resolve. The calculator reads one
//! characteristic at a time through the `Custodian` trait; the engine
//! never names a type of this crate.
//!
//! **The door (ADR-003, amended by ADR-041 §3).** No `Serialize`, no
//! `Debug` that prints a value. `Vault::custodian` builds a
//! `CustodianView` for the calculator; the orchestrator, the one crate
//! allowed to read Plane A (ADR-112), reads `PlayerLatents` whole through
//! `Vault::latents_for`. Its fields are public to that crate only by
//! contract: no other crate depends on `ds-db` (today only
//! `ds-orchestrator` does).
//!
//! **What is not here:** the identity (`ds_core::identity::Identity`, a
//! primitive of the world) and the decision (the orchestrator, ADR-041
//! §4). The vault receives the identity; it does not own it.

#![deny(missing_docs)]

pub mod custodian;
pub mod latents;
pub mod profiles;

pub use custodian::CustodianView;
pub use latents::PlayerLatents;

use std::collections::BTreeMap;

use ds_core::identity::Identity;
use ds_core::ids::PlayerId;

/// Plane A: the hidden performances, by player, derived by key.
///
/// The vault does not store every player at construction: `latents_for`
/// derives on first read and caches, so a match that reads only the
/// players on the pitch does not pay for the squad. The cache is a
/// `BTreeMap` (deterministic iteration, D2).
pub struct Vault {
    identity: Identity,
    latents: BTreeMap<PlayerId, PlayerLatents>,
    /// Each player's role, for his profile (`balance/profiles.json`).
    /// A player with no registered role is derived uniformly.
    roles: BTreeMap<PlayerId, String>,
    /// The identity each place of the match stands for (L1, ADR-113 §6): a
    /// player keeps his characteristics in every match of a season, at
    /// home or away, whatever place the match gives him. A place with no
    /// key is its own identity (a friendly, every L0 match).
    keys: BTreeMap<PlayerId, PlayerId>,
    profiles: Option<profiles::Profiles>,
}

impl Vault {
    /// Builds an empty vault for the given match identity.
    #[must_use]
    pub const fn new(identity: Identity) -> Self {
        Self {
            identity,
            latents: BTreeMap::new(),
            roles: BTreeMap::new(),
            keys: BTreeMap::new(),
            profiles: None,
        }
    }

    /// Registers each player's role: his characteristics are then derived
    /// from his role's profile (`balance/profiles.json`), still by key.
    /// Must be called before the first read of those players.
    pub fn register_roles(&mut self, roles: impl IntoIterator<Item = (PlayerId, String)>) {
        self.roles.extend(roles);
        if self.profiles.is_none() {
            self.profiles = Some(profiles::Profiles::load());
        }
    }

    /// Registers the identity each place of the match stands for: his
    /// characteristics are derived from the identity, not from the place.
    /// Must be called before the first read of those players.
    pub fn register_keys(&mut self, keys: impl IntoIterator<Item = (PlayerId, PlayerId)>) {
        self.keys.extend(keys);
    }

    /// A root of a person who is not a player (a coach, ADR-114 §2): derived
    /// by key from the world seed and his identity, in `[0, 1)`; stays in
    /// the vault and is read only by the actor.
    #[must_use]
    pub fn person_root(&self, person: PlayerId, name: &str) -> f64 {
        f64::from(ds_core::hash::unit_from_key(
            self.identity.world_seed,
            person,
            name,
        ))
    }

    /// Returns the match identity the vault was built with.
    #[must_use]
    pub const fn identity(&self) -> Identity {
        self.identity
    }

    /// Returns `player`'s latents, deriving them on first read.
    ///
    /// The derivation is a pure function of `(world_seed, player_id,
    /// characteristic)` (ADR-029, OI-53): the cache is a speed-up, not a
    /// source of truth. Two reads of the same player in the same world
    /// return the same value.
    #[must_use]
    pub fn latents_for(&mut self, player: PlayerId) -> PlayerLatents {
        *self.latents.entry(player).or_insert_with(|| {
            let seed = self.identity.world_seed;
            let key = self.keys.get(&player).copied().unwrap_or(player);
            self.roles
                .get(&player)
                .zip(self.profiles.as_ref())
                .and_then(|(role, p)| p.latents(seed, key, role))
                .unwrap_or_else(|| PlayerLatents::from_key(seed, key))
        })
    }

    /// Builds the custodian view for `player`, the door the calculator
    /// reads through (ADR-028 section 5).
    pub fn custodian(&mut self, player: PlayerId) -> CustodianView {
        CustodianView::new(self.latents_for(player))
    }
}

impl std::fmt::Debug for Vault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vault")
            .field("identity", &self.identity)
            .field("players_cached", &self.latents.len())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// L1 (ADR-113 §6): the same identity at home (place 3) and away (place
    /// 103) has the same characteristics; a place with no key is its own.
    #[test]
    fn a_player_keeps_his_characteristics_in_any_place() {
        let mut v = Vault::new(Identity::new(42, ds_core::ids::MatchId::new(1, 0)));
        let (home, away, key) = (
            PlayerId::new(3, 0),
            PlayerId::new(103, 0),
            PlayerId::new(1207, 0),
        );
        v.register_roles([(home, "striker".to_string()), (away, "striker".to_string())]);
        v.register_keys([(home, key), (away, key)]);
        // `PlayerLatents` shows nothing (no Debug, no comparison: code audit
        // S6): a few of its values, compared one by one.
        let sig = |l: PlayerLatents| {
            [
                l.physical.max_speed,
                l.technical.finishing,
                l.cognitive.game_reading,
                l.psychological.composure,
            ]
            .map(f32::to_bits)
        };
        assert_eq!(sig(v.latents_for(home)), sig(v.latents_for(away)));
        let mut plain = Vault::new(Identity::new(42, ds_core::ids::MatchId::new(1, 0)));
        plain.register_roles([(home, "striker".to_string())]);
        assert_ne!(
            sig(plain.latents_for(home)),
            sig(v.latents_for(home)),
            "the key, not the place"
        );
    }
    use ds_core::ids::MatchId;

    fn ident() -> Identity {
        Identity::new(42, MatchId::new(7, 0))
    }

    #[test]
    fn same_player_same_value_on_second_read() {
        let mut v = Vault::new(ident());
        let a = v.latents_for(PlayerId::new(7, 0));
        let b = v.latents_for(PlayerId::new(7, 0));
        assert_eq!(a.technical.passing, b.technical.passing);
    }

    #[test]
    fn cache_does_not_change_the_derivation() {
        let mut v = Vault::new(ident());
        let p = PlayerId::new(7, 0);
        let from_cache = v.latents_for(p);
        let direct = PlayerLatents::from_key(42, p);
        assert_eq!(from_cache.technical.passing, direct.technical.passing);
    }

    #[test]
    fn different_players_different_values() {
        let mut v = Vault::new(ident());
        let a = v.latents_for(PlayerId::new(7, 0));
        let b = v.latents_for(PlayerId::new(8, 0));
        assert_ne!(a.technical.passing, b.technical.passing);
    }
}
