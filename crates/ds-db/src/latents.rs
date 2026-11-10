//! Latent attributes (Plane A), derived by key, grouped by family.
//!
//! These are the values that must **never** leave `pcfm-hidden`.
//!
//! **Keyed derivation (ADR-029, OI-53).** A latent is derived from the
//! player and the characteristic, not from a sequence:
//! `hash(world_seed, player_id, characteristic)`. The same player in the
//! same world has the same value, regardless of when it is read and of how
//! many players were generated before him. Adding a characteristic does
//! not shift the others, because each value is derived from its own key.
//!
//! **No `match_id` in the key.** A player's strength is the same in every
//! match of his world (ADR-029 §2). The world is identified by the
//! `world_seed` (ADR-013), so no `world_id` is needed either.
//!
//! **The values are still uniform in `[0, 1)`.** This is a prototype: real
//! attributes are roughly bell-shaped, with a long tail of elite players.
//! The factor structure that produces realistic distributions and
//! correlations is L3's job (L0.11 §11.1); this module only fixes the
//! derivation, not the distribution.
//!
//! # The four families (L0.11 §7, §12.2)
//!
//! The latents are grouped by family, as the model does: physical,
//! technical, psychological, cognitive. Adding a characteristic touches
//! one family, not a 24-field blob (L0.11 §12.2). The families that have
//! no datum yet (psychological, cognitive) are empty: their content
//! arrives with the consumer, not before (D3).
//!
//! The prototype fields keep their prototype names and their prototype
//! place: `aerial` and `set_pieces` are composites (L0.11 §7.9 and §10), and
//! they are migrated when their split arrives (OI-48, OI-50).
//! `explosive_force` is the first new-model field: it is read by the
//! movement formula (F.7).

use ds_core::ids::PlayerId;

/// A latent attribute of a player, grouped by family (L0.11 §7, §12.2).
#[derive(Debug, Clone, Copy)]
pub struct PlayerLatents {
    /// Physical performances (L0.11 §7.1).
    pub physical: PhysicalLatents,
    /// Technical performances (L0.11 §7.2).
    pub technical: TechnicalLatents,
    /// Psychological traits (L0.11 §7.3), all modulators.
    #[allow(dead_code)] // no consumer yet: the family is empty (D3)
    pub psychological: PsychologicalLatents,
    /// Cognitive traits (L0.11 §7.4).
    pub cognitive: CognitiveLatents,
}

/// Physical performances (L0.11 §7.1).
#[derive(Debug, Clone, Copy)]
pub struct PhysicalLatents {
    /// Explosive force, consumed by the movement (F.7).
    #[allow(dead_code)] // the consumer is resolve_movement, F.7 step 6
    pub explosive_force: f32,
    /// Max speed, consumed by the movement (F.7).
    pub max_speed: f32,
    /// Susceptibility to injury (prototype field, L0.11 §10 item 1:
    /// physical susceptibility, not a technical skill).
    #[allow(dead_code)]
    // no consumer today: the die that read it is removed (ADR-035); arrives with the injury model (OI-60)
    pub injury_proneness: f32,
}

/// Technical performances (L0.11 §7.2).
#[derive(Debug, Clone, Copy)]
pub struct TechnicalLatents {
    /// Finishing, consumed by the shot (prototype name; L0.11 §7.2 calls
    /// the characteristic "shooting").
    pub finishing: f32,
    /// Passing, consumed by the pass.
    pub passing: f32,
    /// First touch, consumed by the reception of a pass (L0.11 §7.2).
    pub first_touch: f32,
    /// Tackling, consumed by the ground duel.
    pub tackling: f32,
    /// Prototype composite (`heading` × `height` × `explosive_force`): to be
    /// split (OI-50).
    pub aerial: f32,
    /// Goalkeeping, consumed by the save.
    pub goalkeeping: f32,
    /// Prototype composite (free kicks, penalties, corners, throw-ins):
    /// to be split (OI-48, OI-50).
    #[allow(dead_code)]
    // no consumer today: the die that read it is removed (ADR-035); arrives with the set-piece formula (OI-48)
    pub set_pieces: f32,
    /// Left foot, one of the two performances per foot (ADR-031, L0.11 7.2).
    #[allow(dead_code)] // no consumer yet: ADR-031, arrives with the pass formula (OI-64)
    pub left_foot: f32,
    /// Right foot, one of the two performances per foot (ADR-031, L0.11 7.2).
    #[allow(dead_code)] // no consumer yet: ADR-031, arrives with the pass formula (OI-64)
    pub right_foot: f32,
}

/// Psychological traits (L0.11 §7.3), all modulators.
///
/// Empty: the first trait (consistency) arrives with the sampling the
/// calculator needs §4.7, which does not exist yet (D3).
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)] // no consumer yet: the trait arrives with the sampling (D3)
pub struct PsychologicalLatents {}

/// Cognitive traits (L0.11 §7.4).
///
/// Materialised in the same commit as the choice (F.6-cog), as the
/// L0.11 §7.4 note requires: a latent with no consumer is D3, and the
/// consumer is `resolve_choice`. `vision` is not here: it belongs to
/// the scanning rule (ADR-034, F.6 part 2 / OI-71).
#[derive(Debug, Clone, Copy)]
pub struct CognitiveLatents {
    /// Reading the game: what the player understands (consumed by the choice, F.6-cog).
    pub game_reading: f32,
    /// Decision making: how fast and how well he chooses (consumed by the choice, F.6-cog).
    pub decision_making: f32,
    /// Risk management: the risk he accepts (consumed by the choice, F.6-cog).
    pub risk_management: f32,
}

impl PlayerLatents {
    /// Derives a player's latents from the world seed and the player id.
    ///
    /// Deterministic and order-independent: the value of a characteristic
    /// depends only on `(world_seed, player_id, characteristic)`.
    #[must_use]
    pub fn from_key(world_seed: u64, player: PlayerId) -> Self {
        Self {
            physical: PhysicalLatents {
                explosive_force: unit_from_key(world_seed, player, "explosive_force"),
                max_speed: unit_from_key(world_seed, player, "max_speed"),
                injury_proneness: unit_from_key(world_seed, player, "injury_proneness"),
            },
            technical: TechnicalLatents {
                finishing: unit_from_key(world_seed, player, "finishing"),
                passing: unit_from_key(world_seed, player, "passing"),
                first_touch: unit_from_key(world_seed, player, "first_touch"),
                tackling: unit_from_key(world_seed, player, "tackling"),
                aerial: unit_from_key(world_seed, player, "aerial"),
                goalkeeping: unit_from_key(world_seed, player, "goalkeeping"),
                set_pieces: unit_from_key(world_seed, player, "set_pieces"),
                left_foot: unit_from_key(world_seed, player, "left_foot"),
                right_foot: unit_from_key(world_seed, player, "right_foot"),
            },
            psychological: PsychologicalLatents {},
            cognitive: CognitiveLatents {
                game_reading: unit_from_key(world_seed, player, "game_reading"),
                decision_making: unit_from_key(world_seed, player, "decision_making"),
                risk_management: unit_from_key(world_seed, player, "risk_management"),
            },
        }
    }
}

/// Derives a value in `[0, 1)` from `(world_seed, player_id, purpose)`.
///
/// The hash is BLAKE3, the same function used for the stream derivation in
/// `ds-core::rng` and for the event chain. The key material is the world
/// seed, the player's slot and generation (so two different players never
/// collide), and the purpose (the characteristic name).
#[must_use]
pub fn unit_from_key(world_seed: u64, player: PlayerId, purpose: &str) -> f32 {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&world_seed.to_le_bytes());
    hasher.update(b"player:");
    hasher.update(&player.slot().to_le_bytes());
    hasher.update(&player.generation().to_le_bytes());
    hasher.update(b"purpose:");
    hasher.update(purpose.as_bytes());
    let digest = hasher.finalize();
    let bytes = digest.as_bytes();
    let raw = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    // `[0, 1)`: divide by 2^32, not by u32::MAX, so 1.0 is never produced.
    #[allow(clippy::cast_possible_truncation)]
    let unit = (f64::from(raw) / f64::from(u32::MAX) / 1.000_000_1) as f32;
    unit
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(slot: u32) -> PlayerId {
        PlayerId::new(slot, 0)
    }

    #[test]
    fn same_player_same_value() {
        let a = PlayerLatents::from_key(42, p(7));
        let b = PlayerLatents::from_key(42, p(7));
        assert_eq!(a.technical.passing, b.technical.passing);
        assert_eq!(a.technical.finishing, b.technical.finishing);
        assert_eq!(a.physical.explosive_force, b.physical.explosive_force);
    }

    #[test]
    fn different_players_different_values() {
        let a = PlayerLatents::from_key(42, p(7));
        let b = PlayerLatents::from_key(42, p(8));
        assert_ne!(a.technical.passing, b.technical.passing);
    }

    #[test]
    fn different_worlds_different_values() {
        let a = PlayerLatents::from_key(42, p(7));
        let b = PlayerLatents::from_key(43, p(7));
        assert_ne!(a.technical.passing, b.technical.passing);
    }

    #[test]
    fn values_are_in_unit_range() {
        for slot in 0..100 {
            let l = PlayerLatents::from_key(42, p(slot));
            for v in [
                l.physical.explosive_force,
                l.physical.injury_proneness,
                l.technical.finishing,
                l.technical.passing,
                l.technical.tackling,
                l.technical.aerial,
                l.technical.goalkeeping,
                l.technical.set_pieces,
                l.technical.left_foot,
                l.technical.right_foot,
            ] {
                assert!((0.0..1.0).contains(&v), "value {v} out of [0,1)");
            }
        }
    }

    #[test]
    fn order_does_not_matter() {
        // The defect OI-53 fixed: with a sequence, generating slot 8 after
        // slot 7 changed slot 8's values. With a key, it does not.
        let alone = PlayerLatents::from_key(42, p(8));
        let _before = PlayerLatents::from_key(42, p(7));
        let after = PlayerLatents::from_key(42, p(8));
        assert_eq!(alone.technical.passing, after.technical.passing);
        assert_eq!(alone.technical.tackling, after.technical.tackling);
    }

    #[test]
    fn characteristics_are_independent() {
        // Adding a characteristic does not shift the others: each value is
        // derived from its own purpose string.
        let l = PlayerLatents::from_key(42, p(7));
        assert_ne!(l.technical.finishing, l.technical.passing);
        assert_ne!(l.technical.passing, l.technical.tackling);
        assert_ne!(l.physical.explosive_force, l.technical.passing);
    }

    #[test]
    fn feet_are_independent_performances() {
        // ADR-031: left_foot and right_foot are two separate performances,
        // not one value with a label. They share a player but are derived
        // from different purposes, so they must differ.
        let l = PlayerLatents::from_key(42, p(7));
        assert_ne!(l.technical.left_foot, l.technical.right_foot);
        assert_ne!(l.technical.left_foot, l.technical.finishing);
        assert_ne!(l.technical.right_foot, l.technical.finishing);
    }
}
