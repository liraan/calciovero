//! L0 — Match scenarios.
//!
//! These are the six checks that were previously hardcoded in `check.rs`.
//! They are now individual `Scenario` implementations, registered in the
//! `TestRegistry`. Behaviour is unchanged: the output of `--check` is the
//! same six lines as before.

use ds_core::time::GameInstant;
use ds_match::{MatchConfig, MatchResult, MatchSave};
use ds_model::Match;
use ds_testkit::{Feature, Scenario, ScenarioResult, TestContext, TestRegistry};

use ds_fixtures::match_fixture;

/// Register every L0 scenario.
///
/// # Why these six
///
/// The six scenarios are the **acceptance tests** for L0. They cover:
///
/// | Scenario | What it verifies | Why it is here |
/// |----------|------------------|----------------|
/// | `DemoTeamScenario` | The demo squads are valid (11 starters, etc.) | If this fails, every other scenario fails for the wrong reason |
/// | `SameSeedScenario` | Same seed → same state hash | The determinism contract (pillar 1) |
/// | `DifferentSeedScenario` | Different seed → different hash | The seed is actually used, not ignored |
/// | `SaveLoadScenario` | Save → load → replay → same hash | The persistence contract (reconstruction from seed + inputs) |
/// | `EventChainScenario` | The chain hash, recomputed from the events, verifies | The transparency contract (the events are the ones played) |
/// | `StatsSanityScenario` | Both sides had the ball, at least one event | The statistics are not garbage |
///
/// A seventh scenario would be added when L0's `DoD` grows. A "golden
/// test" scenario (many matches, same hash) is in `ds-match`'s own test
/// suite, not here, because it is too slow for a `--check` run.
///
/// When L1, L2, L3 are implemented, their scenarios go in
/// `features/l1_*.rs`, `features/l2_*.rs`, `features/l3_*.rs`, and are
/// registered in `features/mod.rs::register_all`.
pub fn register(registry: &mut TestRegistry) {
    registry.register(DemoTeamScenario);
    registry.register(SameSeedScenario);
    registry.register(DifferentSeedScenario);
    registry.register(SaveLoadScenario);
    registry.register(EventChainScenario);
    registry.register(StatsSanityScenario);
}

// ── Helper ─────────────────────────────────────────────────────────────

fn simulate(match_: &Match, seed: u64) -> MatchResult {
    ds_match::simulate_from_seed(match_, seed, MatchConfig::default())
}
// ── Scenario 1: demo team is valid ─────────────────────────────────────

pub struct DemoTeamScenario;

impl Scenario for DemoTeamScenario {
    fn name(&self) -> &'static str {
        "demo_team_is_valid"
    }

    fn feature(&self) -> Feature {
        Feature::L0Match
    }

    fn run(&self, ctx: &mut TestContext) -> ScenarioResult {
        let mut result = ScenarioResult::new();
        match match_fixture(ctx.seed) {
            Ok(_) => result.pass("demo teams are valid"),
            Err(e) => result.fail("demo teams are valid", e),
        }
        result
    }
}

// ── Scenario 2: same seed → same hash ──────────────────────────────────

pub struct SameSeedScenario;

impl Scenario for SameSeedScenario {
    fn name(&self) -> &'static str {
        "same_seed_same_hash"
    }

    fn feature(&self) -> Feature {
        Feature::L0Match
    }

    fn run(&self, ctx: &mut TestContext) -> ScenarioResult {
        let mut result = ScenarioResult::new();
        let match_ = match match_fixture(ctx.seed) {
            Ok(m) => m,
            Err(e) => {
                result.error(e);
                return result;
            }
        };
        let r1 = simulate(&match_, ctx.seed);
        let r2 = simulate(&match_, ctx.seed);
        if r1.state_hash == r2.state_hash {
            result.pass("same seed → same state_hash");
        } else {
            result.fail(
                "same seed → same state_hash",
                format!("{:?} vs {:?}", r1.state_hash, r2.state_hash),
            );
        }
        result
    }
}

// ── Scenario 3: different seed → different hash ────────────────────────

pub struct DifferentSeedScenario;

impl Scenario for DifferentSeedScenario {
    fn name(&self) -> &'static str {
        "different_seed_different_hash"
    }

    fn feature(&self) -> Feature {
        Feature::L0Match
    }

    fn run(&self, ctx: &mut TestContext) -> ScenarioResult {
        let mut result = ScenarioResult::new();
        let match1 = match match_fixture(ctx.seed) {
            Ok(m) => m,
            Err(e) => {
                result.error(e);
                return result;
            }
        };
        let other_seed = ctx.seed.wrapping_add(1);
        let match2 = match match_fixture(other_seed) {
            Ok(m) => m,
            Err(e) => {
                result.error(e);
                return result;
            }
        };
        let r1 = simulate(&match1, ctx.seed);
        let r3 = simulate(&match2, other_seed);
        if r1.state_hash == r3.state_hash {
            result.fail(
                "different seed → different state_hash",
                "hash is identical, the seed is ignored",
            );
        } else {
            result.pass("different seed → different state_hash");
        }
        result
    }
}

// ── Scenario 4: save/load roundtrip ────────────────────────────────────

pub struct SaveLoadScenario;

impl Scenario for SaveLoadScenario {
    fn name(&self) -> &'static str {
        "save_load_roundtrip"
    }

    fn feature(&self) -> Feature {
        Feature::L0Save
    }

    fn run(&self, ctx: &mut TestContext) -> ScenarioResult {
        let mut result = ScenarioResult::new();
        let match_ = match match_fixture(ctx.seed) {
            Ok(m) => m,
            Err(e) => {
                result.error(e);
                return result;
            }
        };
        let r1 = simulate(&match_, ctx.seed);
        let save = MatchSave::from_result(&r1, &match_, ctx.seed, GameInstant::from_raw(0));
        match save.to_bytes().and_then(|b| MatchSave::from_bytes(&b)) {
            Ok(loaded) => {
                // Played again from the save (2026-11-23): before, the
                // stored hash was read back and compared with itself.
                if ds_match::replay(&loaded).state_hash == r1.state_hash {
                    result.pass("save/load roundtrip preserves state_hash");
                } else {
                    result.fail(
                        "save/load roundtrip preserves state_hash",
                        "hash differs after roundtrip",
                    );
                }
            }
            Err(e) => {
                result.fail("save/load roundtrip", e.to_string());
            }
        }
        result
    }
}

// ── Scenario 5: event chain is valid ───────────────────────────────────

pub struct EventChainScenario;

impl Scenario for EventChainScenario {
    fn name(&self) -> &'static str {
        "event_chain_is_valid"
    }

    fn feature(&self) -> Feature {
        Feature::L0Match
    }

    fn run(&self, ctx: &mut TestContext) -> ScenarioResult {
        let mut result = ScenarioResult::new();
        let match_ = match match_fixture(ctx.seed) {
            Ok(m) => m,
            Err(e) => {
                result.error(e);
                return result;
            }
        };
        let r1 = simulate(&match_, ctx.seed);
        // Recomputed from the events (`EventChain::verify`, 2026-11-23).
        let chain_err: Option<String> = if r1.events.verify(r1.state_hash) {
            None
        } else {
            Some("chain hash does not verify".to_string())
        };
        match chain_err {
            None => result.pass("event chain is valid"),
            Some(reason) => result.fail("event chain is valid", reason),
        }
        result
    }
}

// ── Scenario 6: statistics are sane ────────────────────────────────────

pub struct StatsSanityScenario;

impl Scenario for StatsSanityScenario {
    fn name(&self) -> &'static str {
        "stats_are_sane"
    }

    fn feature(&self) -> Feature {
        Feature::L0Match
    }

    fn run(&self, ctx: &mut TestContext) -> ScenarioResult {
        let mut result = ScenarioResult::new();
        let match_ = match match_fixture(ctx.seed) {
            Ok(m) => m,
            Err(e) => {
                result.error(e);
                return result;
            }
        };
        let r1 = simulate(&match_, ctx.seed);
        // Both sides had the ball (2026-11-23: the sum of the two shares
        // is 1 by construction, and xG was named but never read).
        let (h, a) = r1.stats.possession;
        let stats_ok = h > 0.0 && a > 0.0 && !r1.events.is_empty();
        if stats_ok {
            result.pass("statistics are sane");
        } else {
            result.fail(
                "statistics are sane",
                "a side never had the ball, or no events",
            );
        }
        result
    }
}
