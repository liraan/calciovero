//! The fatigue plugin.
//!
//! The fatigue plugin (ADR-063 §4, plan L0-CLOSE step 6): in the move
//! phase each player tires with the speed he ran at this tick, against his
//! endurance (`balance/fatigue.json`, `ds_calculator::fatigue`). The first
//! plugin that acts only through the contract: it reads and writes the
//! match through `ds_match_api::World`.

use ds_core::dmath;
use ds_match_api::{MatchPlugin, Phase, PluginCard, World};

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
        fn position(&self, _: PlayerKey) -> Option<(i32, i32)> {
            Some((0, 0))
        }
        fn gaze(&self, _: PlayerKey) -> Option<u16> {
            Some(0)
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
