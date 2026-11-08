//! The custodian view: the Plane A door, visible to the calculator only.
//!
//! ADR-028 section 5 gives the custodian one responsibility: hold the
//! hidden performances and answer one characteristic at a time. The
//! calculator is the only consumer. The engine never sees this type:
//! the module `custodian` is private (`mod custodian;`), so the item is `pub`
//! inside `pcfm-hidden` only: a caller in `pcfm-match` cannot reach it, and
//! the type has no public API.
//!
//! The view holds the player's latents by value (`PlayerLatents` is
//! `Copy`): no borrow of `GroundTruth` crosses a `resolve_*` call, and
//! the keyed derivation (`PlayerLatents::from_key`) makes the value a
//! pure function of `(world_seed, player)`. `GroundTruth` does not
//! implement `Custodian` directly: one door, not two (D7).

use pcfm_calculator::{Characteristic, Custodian};

use crate::latents::PlayerLatents;

/// The custodian view over a player's latents, for one `resolve_*` call.
pub struct CustodianView {
    latents: PlayerLatents,
}

impl CustodianView {
    /// Builds the view from the player's latents.
    ///
    /// `pub` in a private module: the orchestrator inside `pcfm-hidden` is
    /// the only caller.
    pub const fn new(latents: PlayerLatents) -> Self {
        Self { latents }
    }
}

impl Custodian for CustodianView {
    fn value(&mut self, characteristic: Characteristic) -> f64 {
        match characteristic {
            Characteristic::ExplosiveForce => f64::from(self.latents.physical.explosive_force),
            // The catalogue declares three more physical characteristics
            // (`MaximalForce`, `Acceleration`, `MaxSpeed`) that the
            // movement formula does not read. The calculator asks only
            // for declared terms (ADR-028 section 5): this branch is
            // reached only if a future formula declares one of them
            // before the corresponding performance is in `PlayerLatents`.
            // It is `0.0` until Plan H materialises them.
            Characteristic::MaximalForce
            | Characteristic::Acceleration
            | Characteristic::MaxSpeed => 0.0,
        }
    }
}
