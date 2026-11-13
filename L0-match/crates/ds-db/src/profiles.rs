//! The generated players' profiles, by role (`balance/profiles.json`).
//!
//! **One responsibility:** turn a role and a key into a player's hidden
//! characteristics that make sense together (maintainer, 2026-11-12:
//! random but sensible, recognisable). Still derived by key (ADR-029): the
//! same world and the same player give the same values. The numbers are
//! data: a volunteer edits a role in the JSON and replays the same seed.
//! The real distributions are the generator's (L3, IDEA-DESIGN-007); this
//! is its declared prototype.

use std::collections::BTreeMap;

use ds_core::hash::unit_from_key;
use ds_core::ids::PlayerId;
use serde::Deserialize;

use crate::latents::{
    CognitiveLatents, PhysicalLatents, PlayerLatents, PsychologicalLatents, TechnicalLatents,
};

const PROFILES_JSON: &str = include_str!("../../../balance/profiles.json");

/// A role's profile: `[mean, spread]` per characteristic.
#[derive(Debug, Clone, Deserialize)]
pub struct RoleProfile {
    /// `[mean, spread]` per characteristic name.
    pub characteristics: BTreeMap<String, [f32; 2]>,
}

/// Every role's profile, and the rules that hold across roles.
#[derive(Debug, Clone, Deserialize)]
pub struct Profiles {
    /// Share of left-footed players.
    pub left_footed_share: f32,
    /// The weak foot as a fraction of the strong one: `[mean, spread]`.
    pub weak_foot_ratio: [f32; 2],
    /// How much a characteristic follows its family's shared key.
    pub family_weight: f32,
    /// The profiles, by role name (the `RoleId` of the player).
    pub roles: BTreeMap<String, RoleProfile>,
}

impl Profiles {
    /// Loads the embedded profiles.
    ///
    /// # Panics
    ///
    /// Panics if the embedded JSON is invalid; `embedded_profiles_are_valid`
    /// catches it at test time.
    #[must_use]
    pub fn load() -> Self {
        serde_json::from_str(PROFILES_JSON).expect("embedded profiles file is valid JSON")
    }

    /// The hidden characteristics of `player` in `role`, derived by key.
    /// `None` if the role has no profile (the caller falls back to the
    /// uniform derivation).
    #[must_use]
    pub fn latents(&self, world_seed: u64, player: PlayerId, role: &str) -> Option<PlayerLatents> {
        let p = self.roles.get(role)?;
        let value = |name: &str, family: &str| -> f32 {
            let [mean, spread] = p.characteristics.get(name).copied().unwrap_or([0.5, 0.15]);
            let own = unit_from_key(world_seed, player, name);
            let shared = unit_from_key(world_seed, player, family);
            let mix = shared.mul_add(self.family_weight, own * (1.0 - self.family_weight));
            spread
                .mul_add(mix.mul_add(2.0, -1.0), mean)
                .clamp(0.02, 0.98)
        };
        let passing = value("passing", "family:technical");
        let [ratio_mean, ratio_spread] = self.weak_foot_ratio;
        let ratio = ratio_spread
            .mul_add(
                unit_from_key(world_seed, player, "weak_foot").mul_add(2.0, -1.0),
                ratio_mean,
            )
            .clamp(0.1, 1.0);
        let strong = value("strong_foot", "family:technical").max(passing);
        let weak = (strong * ratio).clamp(0.02, 0.98);
        let left_footed = unit_from_key(world_seed, player, "left_footed") < self.left_footed_share;
        let (left_foot, right_foot) = if left_footed {
            (strong, weak)
        } else {
            (weak, strong)
        };
        Some(PlayerLatents {
            physical: PhysicalLatents {
                explosive_force: value("explosive_force", "family:physical"),
                max_speed: value("max_speed", "family:physical"),
                injury_proneness: value("injury_proneness", "family:physical"),
                reactivity: value("reactivity", "family:physical"),
                maximal_force: value("maximal_force", "family:physical"),
                body_control: value("body_control", "family:physical"),
            },
            technical: TechnicalLatents {
                finishing: value("finishing", "family:technical"),
                passing,
                first_touch: value("first_touch", "family:technical"),
                tackling: value("tackling", "family:technical"),
                aerial: value("aerial", "family:physical"),
                goalkeeping: value("goalkeeping", "family:technical"),
                set_pieces: value("set_pieces", "family:technical"),
                left_foot,
                right_foot,
            },
            psychological: PsychologicalLatents {
                aggressiveness: value("aggressiveness", "family:psychological"),
            },
            cognitive: CognitiveLatents {
                game_reading: value("game_reading", "family:cognitive"),
                decision_making: value("decision_making", "family:cognitive"),
                risk_management: value("risk_management", "family:cognitive"),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_profiles_are_valid() {
        let p = Profiles::load();
        for role in [
            "goalkeeper",
            "centre_back",
            "full_back",
            "central_midfielder",
            "wide_midfielder",
            "striker",
        ] {
            assert!(p.roles.contains_key(role), "{role}");
        }
    }

    fn mean_of(role: &str, f: impl Fn(&PlayerLatents) -> f32) -> f32 {
        let p = Profiles::load();
        (0..200_u32)
            .map(|i| f(&p.latents(7, PlayerId::new(i, 0), role).expect("role")))
            .sum::<f32>()
            / 200.0
    }

    /// Recognisable (P1): a striker finishes better than a centre back, a
    /// centre back tackles better than a striker, a goalkeeper keeps goal.
    #[test]
    fn the_roles_are_recognisable() {
        assert!(
            mean_of("striker", |l| l.technical.finishing)
                > mean_of("centre_back", |l| l.technical.finishing) + 0.3
        );
        assert!(
            mean_of("centre_back", |l| l.technical.tackling)
                > mean_of("striker", |l| l.technical.tackling) + 0.3
        );
        assert!(mean_of("goalkeeper", |l| l.technical.goalkeeping) > 0.6);
        assert!(
            mean_of("wide_midfielder", |l| l.physical.max_speed)
                > mean_of("centre_back", |l| l.physical.max_speed)
        );
    }

    /// Derived by key: the same world and player give the same values.
    #[test]
    fn the_same_player_is_the_same_player() {
        let p = Profiles::load();
        let a = p.latents(3, PlayerId::new(9, 0), "striker").expect("role");
        let b = p.latents(3, PlayerId::new(9, 0), "striker").expect("role");
        assert_eq!(
            a.technical.finishing.to_bits(),
            b.technical.finishing.to_bits()
        );
    }

    /// Each player has a strong foot and a weaker one.
    #[test]
    fn every_player_has_a_strong_foot() {
        let p = Profiles::load();
        for i in 0..50 {
            let l = p
                .latents(1, PlayerId::new(i, 0), "central_midfielder")
                .expect("role");
            assert!((l.technical.left_foot - l.technical.right_foot).abs() > 0.0);
        }
    }
}
