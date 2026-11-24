//! `MatchSave` — save and load a single match.
//!
//! See `L0-match/specs/07-save-minimal.md` for the full specification.
//!
//! # What is saved
//!
//! - The `Match` (squads, pitch, referee, seed, rules).
//! - The `EventChain` (append-only events).
//! - The `MatchResult` (score, stats, state hash, narrative).
//! - Metadata: `schema_version`, `sim_version`, `world_seed`, `match_id`,
//!   `created_at`.
//! - For a finished match, its tick record, packed (ADR-065, schema v3):
//!   the match can always be watched as it was, even when a newer engine
//!   would play it differently. Optional: a v2 save has none and loads.
//!
//! # What is NOT saved
//!
//! - `GroundTruth` (Plane A). It is reconstructed from `(world_seed,
//!   match_id)` at load time. See `replay`.
//! - The RNG registry. It is reconstructed from `world_seed`.
//!
//! # Format
//!
//! For now, the save is serialised with `serde_json`. The target format is
//! `postcard` (compact, deterministic), which will be adopted when
//! `MatchEvent.fields` becomes a `String` (JSON-serialised) instead of a
//! `serde_json::Value`. That change requires an ADR.
//!
//! # Known limitation: `serde_json` and `f64`
//!
//! `MatchEvent.fields` is a `serde_json::Value`. When it contains an
//! `f64` derived from an `f32` (e.g. `xg_value`), the roundtrip
//! `serialize -> deserialize` may lose 1 ulp (last-bit) of precision.
//! This is a property of `serde_json`, not of the simulation.
//!
//! The identity of a saved match is verified via `state_hash`, not via
//! bit-for-bit equality of the events. The `replay` function re-runs the
//! simulation and compares `state_hash`, `score`, and `stats` — which are
//! stable across the roundtrip.
//!
//! The target format for the save is `postcard` (compact, deterministic),
//! which requires `MatchEvent.fields` to be a `String` (JSON-serialised)
//! instead of a `serde_json::Value`. That change is a future ADR.
//!
//! # Compatibility
//!
//! - If `schema_version` is newer than the current, the save is rejected.
//! - If `sim_version` is different from the current, the save is readable
//!   but `can_replay()` returns `false`. The result is still shown, but a
//!   `replay` would produce a different result.
//! - If the chain hash does not verify, the save is rejected (tampering).

use ds_core::ids::MatchId;
use ds_core::time::GameInstant;
use ds_model::Match;
use ds_orchestrator::GroundTruth;
use ds_stats::EventChain;
use serde::{Deserialize, Serialize};

use crate::context::{MatchConfig, MatchContext};
use crate::engine::{MatchEngine, TextMatchEngine};
use crate::record::TickRecord;
use crate::recording::{PackedRecord, RecordHeader};
use crate::result::MatchResult;

/// The current schema version.
///
/// Bumped whenever the structure of `MatchSave` changes (new fields, renamed
/// fields, removed fields). A bump requires a migration script.
///
/// v1 -> v2 (2026-11-07, OI-65): `Player.preferred_foot` removed (ADR-031).
/// No migration script: no save exists in the repo (Phase 1, prototype).
///
/// v2 -> v3 (2026-11-16, ADR-065): the optional packed tick record
/// (`record`). A v2 save loads unchanged, with no record; no migration.
///
/// v3 -> v4 (2026-11-23, code audit B5): `MatchResult.narrative` removed; the
/// commentary is read from the events (`ds_narrative::narrate`). A v3 save
/// loads unchanged: its `narrative` is an unknown field, ignored.
///
/// v4 -> v5 (2026-11-24, code audit D): `MatchSave.events` removed; the
/// chain is the result's (`result.events`), which is now the one verified.
/// It was written twice and only the unread copy was checked. A v4 save
/// loads unchanged: its `events` is an unknown field, ignored.
pub const SCHEMA_VERSION: u32 = 5;

/// The current simulation version.
///
/// Bumped whenever the simulation behaviour changes (RNG, resolution,
/// event emission). A bump requires a `L0-match/CHANGELOG-SIM.md` entry and a
/// golden-test regeneration. See `doctrine/adr/003-plane-a-enforcement.md` and
/// `L0-match/CHANGELOG-SIM.md`.
pub const SIM_VERSION: u32 = 200;

/// The save of a single match.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchSave {
    /// The schema version of this save file.
    pub schema_version: u32,
    /// The simulation version the match was simulated with.
    pub sim_version: u32,
    /// The world seed used to create the `GroundTruth`.
    pub world_seed: u64,
    /// The match id.
    pub match_id: MatchId,
    /// The time of the save (simulation time).
    pub created_at: GameInstant,
    /// The match (squads, pitch, referee, seed, rules).
    pub match_: Match,
    /// The result of the simulation (score, stats, state hash, narrative).
    pub result: MatchResult,
    /// The tick record of the finished match, packed (ADR-065). Absent in
    /// a v2 save, and in a save made without it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<PackedRecord>,
}

impl MatchSave {
    /// Creates a `MatchSave` from a result and a match.
    #[must_use]
    pub fn from_result(
        result: &MatchResult,
        match_: &Match,
        world_seed: u64,
        created_at: GameInstant,
    ) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            sim_version: SIM_VERSION,
            world_seed,
            match_id: match_.id,
            created_at,
            match_: match_.clone(),
            result: result.clone(),
            record: None,
        }
    }

    /// The same save, keeping the match's tick record (ADR-065): what a
    /// finished match saves, so that it can be watched as it was.
    #[must_use]
    pub fn with_record(mut self, ticks: &[TickRecord], ticks_per_second: u8) -> Self {
        let pitch = (self.match_.pitch.length_m, self.match_.pitch.width_m);
        self.record = Some(PackedRecord::pack(
            RecordHeader::current(ticks_per_second, pitch),
            ticks,
        ));
        self
    }

    /// Serialises the save to bytes.
    ///
    /// # Errors
    ///
    /// Returns a serialisation error if the save cannot be serialised. By
    /// construction, `MatchSave` is serialisable, so this is unreachable
    /// in practice.
    pub fn to_bytes(&self) -> Result<Vec<u8>, SaveError> {
        serde_json::to_vec(self).map_err(SaveError::Serialization)
    }

    /// Deserialises a save from bytes and validates it.
    ///
    /// # Errors
    ///
    /// - [`SaveError::Deserialization`] if the bytes are not a valid save.
    /// - [`SaveError::SchemaVersionTooNew`] if `schema_version` is newer
    ///   than the current.
    /// - [`SaveError::ChainHashMismatch`] if the chain hash does not match.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SaveError> {
        let save: Self = serde_json::from_slice(bytes).map_err(SaveError::Deserialization)?;
        save.validate()?;
        Ok(save)
    }

    /// Validates the save.
    ///
    /// # Errors
    ///
    /// See [`from_bytes`](Self::from_bytes).
    pub fn validate(&self) -> Result<(), SaveError> {
        if self.schema_version > SCHEMA_VERSION {
            return Err(SaveError::SchemaVersionTooNew {
                save: self.schema_version,
                current: SCHEMA_VERSION,
            });
        }
        if !self.result.events.verify(self.result.state_hash) {
            return Err(SaveError::ChainHashMismatch);
        }
        if let Some(r) = &self.record
            && r.header.sim_version != self.sim_version
        {
            return Err(SaveError::RecordMismatch);
        }
        Ok(())
    }

    /// Returns `true` if the save can be replayed with the current
    /// `sim_version`.
    ///
    /// If `false`, the save is still readable, but a replay would produce a
    /// different result (the simulation changed).
    #[must_use]
    pub const fn can_replay(&self) -> bool {
        self.sim_version == SIM_VERSION
    }
}

/// Replays the match in the save.
///
/// Reconstructs `GroundTruth` from `(world_seed, match_id)`, re-runs the
/// simulation, and returns the `MatchResult`. The result is deterministic:
/// two calls with the same save produce the same result.
///
/// If `save.can_replay()` is `false`, the result may differ from
/// `save.result`. The caller should check `can_replay()` before trusting
/// the replay.
#[must_use]
pub fn replay(save: &MatchSave) -> MatchResult {
    let mut hidden = GroundTruth::for_match(save.world_seed, &save.match_);
    let mut chain = EventChain::new();
    let config = MatchConfig::default();
    let ctx = MatchContext {
        match_: &save.match_,
        hidden: &mut hidden,
        chain: &mut chain,
        config,
    };
    let mut engine = TextMatchEngine::new();
    engine.simulate(ctx)
}

/// Errors that can occur when saving or loading a match.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SaveError {
    /// The save could not be serialised.
    #[error("serialisation error: {0}")]
    Serialization(serde_json::Error),

    /// The save could not be deserialised.
    #[error("deserialisation error: {0}")]
    Deserialization(serde_json::Error),

    /// The save's `schema_version` is newer than the current.
    #[error("save schema_version {save} is newer than current {current}")]
    SchemaVersionTooNew {
        /// The version in the save.
        save: u32,
        /// The current version.
        current: u32,
    },

    /// The chain hash does not match the expected hash.
    #[error("chain hash mismatch: the save has been tampered with")]
    ChainHashMismatch,

    /// The record was not played by the engine the save names.
    #[error("the saved record was played by another engine than the save's")]
    RecordMismatch,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_roundtrips() {
        let (match_, result) = &*crate::test_support::SIM42_FOR_MATCH;
        let save = MatchSave::from_result(result, match_, 42, GameInstant::from_raw(0));
        let bytes = save.to_bytes().unwrap();
        let loaded = MatchSave::from_bytes(&bytes).unwrap();
        assert_eq!(loaded.schema_version, save.schema_version);
        assert_eq!(loaded.sim_version, save.sim_version);
        assert_eq!(loaded.world_seed, save.world_seed);
        assert_eq!(loaded.match_id, save.match_id);
        assert_eq!(loaded.result.state_hash, save.result.state_hash);
        // Note: we compare state_hash, not events.events(). The reason is
        // documented in the module-level docs ("Known limitation: serde_json
        // and f64"). The state_hash is the identity of the chain; the
        // individual event fields (a serde_json::Value) may lose 1 ulp on f64
        // roundtrip through JSON.
        assert_eq!(loaded.result.events.len(), save.result.events.len());
    }

    /// A finished match saves its record and gets it back, tick for tick.
    #[test]
    fn the_record_is_saved_and_comes_back() {
        let (m, result, ticks) = &*crate::test_support::RECORDED42;
        let save = MatchSave::from_result(result, m, 42, GameInstant::from_raw(0))
            .with_record(ticks, MatchConfig::default().ticks_per_second);
        let loaded = MatchSave::from_bytes(&save.to_bytes().unwrap()).unwrap();
        let back = loaded.record.expect("record").unpack().expect("unpack");
        assert_eq!(back.as_slice(), ticks.as_slice());
    }

    /// Compatibility (ADR-065): a schema v2 save, without the record,
    /// still loads under v3 and replays to the same match.
    #[test]
    fn a_v2_save_still_loads() {
        let (match_, result) = &*crate::test_support::SIM42_FOR_MATCH;
        let save = MatchSave::from_result(result, match_, 42, GameInstant::from_raw(0));
        let mut v2: serde_json::Value = serde_json::from_slice(&save.to_bytes().unwrap()).unwrap();
        v2["schema_version"] = 2.into();
        assert!(v2.get("record").is_none(), "v2 had no record");
        let loaded = MatchSave::from_bytes(&serde_json::to_vec(&v2).unwrap()).unwrap();
        assert_eq!(loaded.schema_version, 2);
        assert!(loaded.record.is_none());
        assert_eq!(replay(&loaded).state_hash, result.state_hash);
    }

    /// A record from another engine than the save's is refused.
    #[test]
    fn a_record_from_another_engine_is_refused() {
        let (m, result, ticks) = &*crate::test_support::RECORDED42;
        let mut save = MatchSave::from_result(result, m, 42, GameInstant::from_raw(0))
            .with_record(&ticks[..10], 10);
        save.sim_version += 1;
        let err = MatchSave::from_bytes(&save.to_bytes().unwrap()).unwrap_err();
        assert!(matches!(err, SaveError::RecordMismatch));
    }

    #[test]
    fn chain_hash_is_verified_on_load() {
        let (match_, result) = &*crate::test_support::SIM42_FOR_MATCH;
        let mut save = MatchSave::from_result(result, match_, 42, GameInstant::from_raw(0));
        // Corrupt the state hash so the chain no longer verifies.
        save.result.state_hash = ds_core::hash::Hash::ZERO;
        let bytes = serde_json::to_vec(&save).unwrap();
        let err = MatchSave::from_bytes(&bytes).unwrap_err();
        assert!(matches!(err, SaveError::ChainHashMismatch));
    }

    #[test]
    fn schema_version_too_new_is_rejected() {
        let (match_, result) = &*crate::test_support::SIM42_FOR_MATCH;
        let mut save = MatchSave::from_result(result, match_, 42, GameInstant::from_raw(0));
        save.schema_version = SCHEMA_VERSION + 1;
        let bytes = serde_json::to_vec(&save).unwrap();
        let err = MatchSave::from_bytes(&bytes).unwrap_err();
        assert!(matches!(err, SaveError::SchemaVersionTooNew { .. }));
    }

    #[test]
    fn can_replay_returns_true_for_current_sim_version() {
        let (match_, result) = &*crate::test_support::SIM42_FOR_MATCH;
        let save = MatchSave::from_result(result, match_, 42, GameInstant::from_raw(0));
        assert!(save.can_replay());
    }

    #[test]
    fn can_replay_returns_false_for_old_sim_version() {
        let (match_, result) = &*crate::test_support::SIM42_FOR_MATCH;
        let mut save = MatchSave::from_result(result, match_, 42, GameInstant::from_raw(0));
        save.sim_version = SIM_VERSION + 1;
        assert!(!save.can_replay());
    }

    #[test]
    fn replay_is_deterministic() {
        let (match_, result) = &*crate::test_support::SIM42_FOR_MATCH;
        let save = MatchSave::from_result(result, match_, 42, GameInstant::from_raw(0));
        let r1 = replay(&save);
        let r2 = replay(&save);
        assert_eq!(r1.state_hash, r2.state_hash);
        assert_eq!(r1.score, r2.score);
        assert_eq!(r1.stats, r2.stats);
    }

    #[test]
    fn replay_matches_original_result() {
        let (_, result) = &*crate::test_support::SIM42_FOR_MATCH;
        let replayed = &*crate::test_support::REPLAY42;
        assert_eq!(replayed.state_hash, result.state_hash);
        assert_eq!(replayed.score, result.score);
        assert_eq!(replayed.stats, result.stats);
    }
}
