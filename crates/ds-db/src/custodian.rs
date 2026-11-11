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
    #[must_use]
    pub const fn new(latents: PlayerLatents) -> Self {
        Self { latents }
    }
}

impl Custodian for CustodianView {
    fn value(&mut self, characteristic: Characteristic) -> f64 {
        match characteristic {
            Characteristic::ExplosiveForce => f64::from(self.latents.physical.explosive_force),
            Characteristic::GameReading => f64::from(self.latents.cognitive.game_reading),
            Characteristic::DecisionMaking => f64::from(self.latents.cognitive.decision_making),
            Characteristic::RiskManagement => f64::from(self.latents.cognitive.risk_management),
            Characteristic::Finishing => f64::from(self.latents.technical.finishing),
            Characteristic::LeftFoot => f64::from(self.latents.technical.left_foot),
            Characteristic::RightFoot => f64::from(self.latents.technical.right_foot),
            // The catalogue declares three more physical characteristics
            // (`MaximalForce`, `Acceleration`, `MaxSpeed`) that no
            // formula reads yet. The calculator asks only for declared
            // terms (ADR-028 section 5): this branch is reached only if
            // a future formula declares one of them before the
            // corresponding performance is in `PlayerLatents`. It is
            // `0.0` until Plan H materialises them.
            Characteristic::MaximalForce
            | Characteristic::Acceleration
            | Characteristic::MaxSpeed => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::latents::PlayerLatents;
    use pcfm_calculator::{Characteristic, Custodian};

    /// The three terms OI-92 added (the shot's choice) return the
    /// player's latent, not a hardcoded 0.0. A regression here would
    /// silently turn the shot's choice into a default (ADR-046).
    #[test]
    fn finishing_and_feet_read_the_latent() {
        let latents = PlayerLatents::from_key(42, ds_core::ids::PlayerId::new(7, 0));
        let mut view = CustodianView::new(latents);
        assert_eq!(
            view.value(Characteristic::Finishing),
            f64::from(latents.technical.finishing)
        );
        assert_eq!(
            view.value(Characteristic::LeftFoot),
            f64::from(latents.technical.left_foot)
        );
        assert_eq!(
            view.value(Characteristic::RightFoot),
            f64::from(latents.technical.right_foot)
        );
    }
}
