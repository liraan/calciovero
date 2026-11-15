//! The commentary in the engine's crate: the narrative itself lives in
//! `ds-narrative` (ADR-063, plan `PLUGINS.md` step 5); here only what joins
//! it to the tick record.

pub use ds_narrative::*;

use ds_model::Match;
use ds_stats::EventChain;
use serde::{Deserialize, Serialize};

/// One tick, narrated: what the pitch looks like, and its narrated events.
///
/// IDEA-INFRA-021, reading (a): the narrator consumes the tick record and
/// produces a narrated tick record; the 2D consumes that.
///
/// A tick with no event is not "nothing to say": the record carries the
/// movement, and the entries are empty.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NarratedTick {
    /// What the pitch looks like.
    pub record: crate::record::TickRecord,
    /// The narrated events of this tick, in chain order.
    pub entries: Vec<NarrativeEntry>,
}

/// Narrates the tick record: every record with the narrated events of its
/// range. The narrative is one entry per event, in chain order, so every
/// event appears in exactly one tick.
#[must_use]
pub fn narrate_ticks(
    match_: &Match,
    chain: &EventChain,
    records: Vec<crate::record::TickRecord>,
) -> Vec<NarratedTick> {
    let entries = narrate(match_, chain);
    records
        .into_iter()
        .map(|record| {
            let range =
                record.events.start.min(entries.len())..record.events.end.min(entries.len());
            NarratedTick {
                entries: entries[range].to_vec(),
                record,
            }
        })
        .collect()
}
