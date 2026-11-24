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
    /// What the plugin using it declares it reads (its card, ADR-063 §4):
    /// a characteristic not declared is not given (code audit S6).
    pub(crate) reads: &'static [&'static str],
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
                // Never below the damage he brought (decision AB).
                b.fatigue = fatigue.max(b.damage);
            }
        }
    }

    fn condition(&self, p: PlayerKey) -> f64 {
        self.id(p)
            .and_then(|id| self.bodies.get(&id))
            .map_or(1.0, |b| b.condition)
    }

    fn characteristic(&mut self, p: PlayerKey, name: &str) -> Option<f64> {
        // Only what the plugin's card declares (ADR-063 §4, code audit S6).
        if !self.reads.contains(&name) {
            debug_assert!(false, "{name:?} read without its card declaring it");
            return None;
        }
        let id = self.id(p)?;
        self.hidden.characteristic(id, name)
    }
}

/// The whole match as a plugin sees it through the contract: the state,
/// the characteristics and the live squads. The core lends it to a plugin
/// for one call (the plugin is taken out of the state for it), so that the
/// plugin reads and writes only through `World`.
pub(crate) struct StateWorld<'a> {
    pub(crate) state: &'a mut crate::state::MatchState,
    pub(crate) hidden: &'a mut ds_orchestrator::GroundTruth,
    pub(crate) live: &'a mut ds_model::Match,
    pub(crate) minute: u64,
    /// What the plugin using it declares it reads (its card, ADR-063 §4).
    pub(crate) reads: &'static [&'static str],
}

impl StateWorld<'_> {
    fn id(&self, p: PlayerKey) -> Option<PlayerId> {
        let squads = [&self.live.home, &self.live.away];
        squads
            .iter()
            .flat_map(|s| s.players.iter().chain(s.bench.iter()))
            .map(|q| q.id)
            .find(|id| key(*id) == p)
    }

    fn player(&self, p: PlayerKey) -> Option<&ds_model::Player> {
        let squads = [&self.live.home, &self.live.away];
        squads
            .into_iter()
            .flat_map(|s| s.players.iter().chain(s.bench.iter()))
            .find(|q| key(q.id) == p)
    }

    const fn squad(&self, home: bool) -> &ds_model::Squad {
        if home {
            &self.live.home
        } else {
            &self.live.away
        }
    }
}

impl World for StateWorld<'_> {
    fn players(&self) -> Vec<PlayerKey> {
        self.state
            .spatial
            .on_pitch()
            .map(|(id, _)| key(*id))
            .collect()
    }

    fn position(&self, p: PlayerKey) -> Option<(i32, i32)> {
        self.state.spatial.agent(self.id(p)?).map(|a| a.position)
    }

    fn gaze(&self, p: PlayerKey) -> Option<u16> {
        self.state.spatial.agent(self.id(p)?).map(|a| a.gaze)
    }

    fn velocity(&self, p: PlayerKey) -> Option<(f64, f64)> {
        let a = self.state.spatial.agent(self.id(p)?)?;
        Some((f64::from(a.velocity.0), f64::from(a.velocity.1)))
    }

    fn fatigue(&self, p: PlayerKey) -> Option<f64> {
        let id = self.id(p)?;
        self.state.bodies.get(&id).map_or_else(
            || self.player(p).map(|q| f64::from(q.physical.fatigue)),
            |b| Some(b.fatigue),
        )
    }

    fn set_fatigue(&mut self, p: PlayerKey, fatigue: f64) {
        if let Some(id) = self.id(p) {
            if let Some(b) = self.state.bodies.get_mut(&id) {
                b.fatigue = fatigue;
            }
        }
    }

    fn characteristic(&mut self, p: PlayerKey, name: &str) -> Option<f64> {
        // Only what the plugin's card declares (ADR-063 §4, code audit S6).
        if !self.reads.contains(&name) {
            debug_assert!(false, "{name:?} read without its card declaring it");
            return None;
        }
        let id = self.id(p)?;
        self.hidden.characteristic(id, name)
    }

    fn minute(&self) -> u64 {
        self.minute
    }

    fn carrier(&self) -> Option<PlayerKey> {
        self.state.spatial.carrier().map(key)
    }

    fn lineup(&self, home: bool) -> Vec<PlayerKey> {
        self.squad(home).players.iter().map(|q| key(q.id)).collect()
    }

    fn bench(&self, home: bool) -> Vec<PlayerKey> {
        self.squad(home).bench.iter().map(|q| key(q.id)).collect()
    }

    fn role(&self, p: PlayerKey) -> Option<String> {
        self.player(p).map(|q| q.role.as_str().to_string())
    }

    fn substituted(&self, p: PlayerKey) -> bool {
        self.id(p)
            .is_some_and(|id| self.state.substituted.contains(&id))
    }

    fn subs_allowed(&self) -> usize {
        usize::from(self.live.rules.subs_allowed)
    }

    fn planned(&self, home: bool) -> Vec<(u16, PlayerKey, PlayerKey)> {
        self.squad(home)
            .substitutions
            .iter()
            .map(|s| (s.minute, key(s.outgoing), key(s.incoming)))
            .collect()
    }

    fn ordered(&self, home: bool) -> Vec<(PlayerKey, PlayerKey)> {
        let side = if home {
            &self.state.ordered.0
        } else {
            &self.state.ordered.1
        };
        side.iter().map(|&(off, on)| (key(off), key(on))).collect()
    }

    fn coach_reacts(&self, home: bool) -> bool {
        self.squad(home).coach_reacts
    }

    fn goals(&self, home: bool) -> u8 {
        if home {
            self.state.score.home
        } else {
            self.state.score.away
        }
    }

    fn knocked(&self, home: bool) -> Vec<PlayerKey> {
        let squad = self.squad(home);
        self.state
            .knocked
            .iter()
            .filter(|id| squad.players.iter().any(|q| q.id == **id))
            .filter(|id| self.state.spatial.agent(**id).is_some())
            .map(|id| key(*id))
            .collect()
    }

    fn carry_off(&mut self, p: PlayerKey) {
        if let Some(id) = self.id(p) {
            let half = i32::from(self.live.pitch.length_m) * 50;
            self.state.spatial.send_off(id, (half, -300));
        }
    }

    fn on_pitch(&self, p: PlayerKey) -> bool {
        self.state.spatial.on_pitch().any(|(id, _)| key(*id) == p)
    }

    fn substitute(&mut self, off: PlayerKey, on: PlayerKey) -> bool {
        let (Some(off), Some(on)) = (self.id(off), self.id(on)) else {
            return false;
        };
        self.state.make_substitution(self.live, off, on)
    }
}
