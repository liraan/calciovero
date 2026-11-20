//! The integration suite: one binary, one cache, all the old guarantees.
//!
//! Before, each file under `tests/` was its own binary and its own process,
//! so the same seed, the same cup scan and the same tick record were played
//! again in every file. Here the files are modules of one binary and the two
//! simulation entry points are replaced by the shared cache in `support`:
//! all the tests, names and assertions stay, the repeated matches go away.

mod support;

/// The real crate, with the two simulation entry points served from the
/// shared cache. The tests keep importing `ds_match::simulate_from_seed`;
/// inside this binary that name resolves here.
mod ds_match {
    pub use ::ds_match::*;

    pub fn simulate_from_seed(
        match_: &::ds_model::Match,
        seed: u64,
        config: MatchConfig,
    ) -> MatchResult {
        crate::support::simulate_from_seed(match_, seed, config)
    }

    pub fn simulate_from_seed_recorded(
        match_: &::ds_model::Match,
        seed: u64,
        config: MatchConfig,
    ) -> (MatchResult, Vec<TickRecord>) {
        crate::support::simulate_from_seed_recorded(match_, seed, config)
    }
}

mod cases;
