//! The plugins of the match, behind the contract of `ds-match-api`
//! (ADR-063, plan `PLUGINS.md` step 4): moved here one at a time, each
//! with its card, the goldens identical at every step.

pub mod ball;
pub mod bench;
pub mod body;
pub mod duels;
pub mod fatigue;
pub mod keeper;
pub mod perception;
pub mod psychology;
pub mod referee;
pub mod set_pieces;
pub mod technique;
pub mod thought;

use std::collections::BTreeMap;

use ds_core::ids::PlayerId;
use ds_match_api::{PlayerKey, World};

use crate::spatial::{PlayerBody, SpatialState};

/// The key of a player in the contract: his slot and generation.
#[must_use]
pub const fn key(id: PlayerId) -> PlayerKey {
    PlayerKey(((id.slot() as u64) << 32) | id.generation() as u64)
}

/// The match as the plugins see it (`ds_match_api::World`): the bodies,
/// the spatial state and the characteristics (Plane A through the
/// custodian, `GroundTruth`).
pub(crate) struct EngineWorld<'a> {
    pub(crate) bodies: &'a mut BTreeMap<PlayerId, PlayerBody>,
    pub(crate) spatial: &'a SpatialState,
    pub(crate) hidden: &'a mut ds_orchestrator::GroundTruth,
}

impl EngineWorld<'_> {
    fn id(&self, p: PlayerKey) -> Option<PlayerId> {
        self.spatial
            .on_pitch()
            .map(|(id, _)| *id)
            .find(|id| key(*id) == p)
    }
}

impl World for EngineWorld<'_> {
    fn players(&self) -> Vec<PlayerKey> {
        // The men on the pitch: one sent off, or replaced, does not look,
        // is not looked at, and no longer tires.
        self.spatial.on_pitch().map(|(id, _)| key(*id)).collect()
    }

    fn position(&self, p: PlayerKey) -> Option<(i32, i32)> {
        self.spatial.agent(self.id(p)?).map(|a| a.position)
    }

    fn gaze(&self, p: PlayerKey) -> Option<u16> {
        self.spatial.agent(self.id(p)?).map(|a| a.gaze)
    }

    fn velocity(&self, p: PlayerKey) -> Option<(f64, f64)> {
        let a = self.spatial.agent(self.id(p)?)?;
        Some((f64::from(a.velocity.0), f64::from(a.velocity.1)))
    }

    fn fatigue(&self, p: PlayerKey) -> Option<f64> {
        self.bodies.get(&self.id(p)?).map(|b| b.fatigue)
    }

    fn set_fatigue(&mut self, p: PlayerKey, fatigue: f64) {
        if let Some(id) = self.id(p) {
            if let Some(b) = self.bodies.get_mut(&id) {
                b.fatigue = fatigue;
            }
        }
    }

    fn characteristic(&mut self, p: PlayerKey, name: &str) -> Option<f64> {
        let id = self.id(p)?;
        self.hidden.characteristic(id, name)
    }
}
