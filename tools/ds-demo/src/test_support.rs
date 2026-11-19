//! One match, played once, for the tests that only look at it.
//!
//! Seed 42's friendly was played by `narrated()` seven times — once per
//! test of the board and the commentary — and again by the render test in
//! `main.rs`, for the same fixture and the same configuration (SIM 154).
//! `simulate_from_seed_recorded` returns the result of
//! `simulate_from_seed` (narrative included), so one recorded run serves
//! them all. The tests that check a *different* match, or a save and a
//! replay, still play their own.

use std::sync::LazyLock;

use ds_match::{MatchConfig, MatchResult, TickRecord};
use ds_model::Match;

/// Seed 42's friendly, with its tick record.
pub static SEED_42: LazyLock<(Match, MatchResult, Vec<TickRecord>)> = LazyLock::new(|| {
    let m = ds_fixtures::match_fixture(42).expect("fixture");
    let (result, records) = ds_match::simulate_from_seed_recorded(&m, 42, MatchConfig::default());
    (m, result, records)
});
