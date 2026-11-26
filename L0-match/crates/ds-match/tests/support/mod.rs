//! One cache for the whole integration suite.
//!
//! A match is deterministic in `(match, seed, config)`: the same three inputs
//! give the same result and the same tick record. Every test used to replay its
//! own matches — one binary per file, one process per binary — so the same
//! seed 42 and the same cup scan were paid again and again. Here every test
//! module is part of one binary and shares this cache by key.
//!
//! There is one key per simulation, not one per caller: recording does not
//! change the match (asserted in `cases::tick_record`), so the recorded entry
//! point is the only one that runs the engine. The plain entry point reads the
//! same result and drops the records. A determinism test calls the engine
//! itself, `::ds_match::simulate_from_seed`, for its second run: through
//! the cache it would compare a result with its own copy (2026-11-23: four
//! tests did, and could not fail).

use std::collections::HashMap;
use std::sync::{Arc, Condvar, LazyLock, Mutex, OnceLock};

use ds_match::{MatchConfig, MatchResult, TickRecord};
use ds_model::Match;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Key {
    seed: u64,
    fingerprint: String,
}

struct Payload {
    result: MatchResult,
    records: Vec<TickRecord>,
}

type Slot = Arc<OnceLock<Payload>>;

/// The first cup tie that reaches the shoot-out under SIM 219, found once by
/// the 42..342 scan (see `cases::golden_l0::cup_seed_scan`). The routine tests
/// play this seed directly: the 75-match scan is a one-off, not a per-run cost.
pub const CUP_SHOOTOUT_SEED: u64 = 166;

static CACHE: LazyLock<Mutex<HashMap<Key, Slot>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

/// At most one match per core. `each_seed` already fans a wave out over all
/// the cores, and `cargo test` runs several tests at once: without this gate
/// the two multiply into hundreds of threads fighting for the same cores, and
/// a match that takes 1.6 s alone takes 26 s under that contention.
static GATE: LazyLock<(Mutex<usize>, Condvar)> = LazyLock::new(|| {
    (
        Mutex::new(std::thread::available_parallelism().map_or(1, std::num::NonZero::get)),
        Condvar::new(),
    )
});

struct Permit;

fn acquire() -> Permit {
    let (lock, cv) = &*GATE;
    let mut free = lock.lock().expect("gate lock");
    while *free == 0 {
        free = cv.wait(free).expect("gate wait");
    }
    *free -= 1;
    Permit
}

impl Drop for Permit {
    fn drop(&mut self) {
        let (lock, cv) = &*GATE;
        let mut free = lock.lock().expect("gate lock");
        *free += 1;
        drop(free);
        cv.notify_one();
    }
}

fn slot(seed: u64, match_: &Match, config: MatchConfig) -> Slot {
    let fingerprint = format!(
        "{}|{:?}",
        serde_json::to_string(match_).expect("a match serialises"),
        config
    );
    let key = Key { seed, fingerprint };
    let mut cache = CACHE.lock().expect("cache lock");
    Arc::clone(
        cache
            .entry(key)
            .or_insert_with(|| Arc::new(OnceLock::new())),
    )
}

fn compute(match_: &Match, seed: u64, config: MatchConfig) -> Payload {
    let _permit = acquire();
    let (result, records) = ds_match::simulate_from_seed_recorded(match_, seed, config);
    Payload { result, records }
}

/// The cached result of `ds_match::simulate_from_seed`.
pub fn simulate_from_seed(match_: &Match, seed: u64, config: MatchConfig) -> MatchResult {
    slot(seed, match_, config)
        .get_or_init(|| compute(match_, seed, config))
        .result
        .clone()
}

/// The cached result and tick record of `simulate_from_seed_recorded`.
pub fn simulate_from_seed_recorded(
    match_: &Match,
    seed: u64,
    config: MatchConfig,
) -> (MatchResult, Vec<TickRecord>) {
    let binding = slot(seed, match_, config);
    let p = binding.get_or_init(|| compute(match_, seed, config));
    (p.result.clone(), p.records.clone())
}
