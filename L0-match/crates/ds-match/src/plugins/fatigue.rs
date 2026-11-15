//! The fatigue plugin.
//!
//! The fatigue plugin (ADR-063 §4, plan L0-CLOSE step 6): in the move
//! phase each player tires with the speed he ran at this tick, against his
//! endurance (`balance/fatigue.json`, `ds_calculator::fatigue`). The first
//! plugin that acts only through the contract: it reads and writes the
//! match through `ds_match_api::World`.

use std::collections::BTreeMap;

use ds_core::{dmath, ids::PlayerId};
use ds_match_api::{MatchPlugin, Phase, PlayerKey, PluginCard, World};

use crate::spatial::{PlayerBody, SpatialState};

/// The fatigue of the players.
#[derive(Debug, Clone, Copy)]
pub struct Fatigue {
    balance: ds_calculator::fatigue::FatigueBalance,
}

impl Default for Fatigue {
    fn default() -> Self {
        Self {
            balance: ds_calculator::fatigue::FatigueBalance::load(),
        }
    }
}

impl MatchPlugin for Fatigue {
    fn card(&self) -> PluginCard {
        crate::registry::card("fatigue")
    }

    fn run(&mut self, phase: Phase, world: &mut dyn World) {
        if phase != Phase::Move {
            return;
        }
        for p in world.players() {
            let (Some((vx, vy)), Some(fatigue)) = (world.velocity(p), world.fatigue(p)) else {
                continue;
            };
            let endurance = world.characteristic(p, "endurance").unwrap_or(0.5);
            let speed = dmath::hypot(vx, vy);
            world.set_fatigue(p, self.balance.after_tick(fatigue, speed, endurance));
        }
    }
}

/// The key of a player in the contract: his slot and generation.
#[must_use]
pub const fn key(id: PlayerId) -> PlayerKey {
    PlayerKey(((id.slot() as u64) << 32) | id.generation() as u64)
}

/// The match in the move phase, as the fatigue sees it: the bodies, the
/// spatial state and the characteristics (Plane A through the custodian,
/// `GroundTruth`).
pub(crate) struct MoveWorld<'a> {
    pub(crate) bodies: &'a mut BTreeMap<PlayerId, PlayerBody>,
    pub(crate) spatial: &'a SpatialState,
    pub(crate) hidden: &'a mut ds_orchestrator::GroundTruth,
}

impl MoveWorld<'_> {
    fn id(&self, p: PlayerKey) -> Option<PlayerId> {
        self.bodies.keys().copied().find(|id| key(*id) == p)
    }
}

impl World for MoveWorld<'_> {
    fn players(&self) -> Vec<PlayerKey> {
        self.bodies.keys().map(|id| key(*id)).collect()
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

#[cfg(test)]
mod tests {
    use super::Fatigue;
    use ds_match_api::{MatchPlugin, Phase, PlayerKey, World};

    /// A pitch of two players for the contract alone: no engine.
    struct Two {
        fatigue: [f64; 2],
        speed: [f64; 2],
    }

    impl World for Two {
        fn players(&self) -> Vec<PlayerKey> {
            vec![PlayerKey(0), PlayerKey(1)]
        }
        fn velocity(&self, p: PlayerKey) -> Option<(f64, f64)> {
            Some((self.speed[usize::try_from(p.0).ok()?], 0.0))
        }
        fn fatigue(&self, p: PlayerKey) -> Option<f64> {
            Some(self.fatigue[usize::try_from(p.0).ok()?])
        }
        fn set_fatigue(&mut self, p: PlayerKey, f: f64) {
            if let Ok(i) = usize::try_from(p.0) {
                self.fatigue[i] = f;
            }
        }
        fn characteristic(&mut self, _: PlayerKey, _: &str) -> Option<f64> {
            Some(0.5)
        }
    }

    /// P1 (ADR-063): the plugin acts through the contract only; in the move
    /// phase the one who ran faster tires more, in another phase nobody
    /// tires.
    #[test]
    fn the_faster_tires_more_through_the_contract() {
        let mut w = Two {
            fatigue: [0.0, 0.0],
            speed: [10.0, 60.0],
        };
        let mut f = Fatigue::default();
        f.run(Phase::Perceive, &mut w);
        assert_eq!(w.fatigue, [0.0, 0.0]);
        f.run(Phase::Move, &mut w);
        assert!(w.fatigue[1] > w.fatigue[0]);
    }

    #[test]
    fn the_fatigue_plugin_has_its_card() {
        let card = Fatigue::default().card();
        assert_eq!(card.name, "fatigue");
        assert!(card.is_valid());
    }
}
