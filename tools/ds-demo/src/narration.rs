//! The commentary joined to the tick record, for the views.
//!
//! The 2D replay and the GUI read it. Moved here from the engine (code audit B5): the engine gives the
//! facts and their record; the commentary (`ds_narrative`) reads them.

pub use ds_narrative::{Importance, NarrativeEntry, narrate};

use ds_match::TickRecord;
use ds_model::Match;
use ds_stats::EventChain;

/// One tick, narrated: what the pitch looks like, and its narrated events.
///
/// IDEA-INFRA-021, reading (a): the narrator consumes the tick record and
/// produces a narrated tick record; the 2D consumes that. A tick with no
/// event is not "nothing to say": the record carries the movement, and the
/// entries are empty.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NarratedTick {
    /// What the pitch looks like.
    pub record: TickRecord,
    /// The narrated events of this tick, in chain order.
    pub entries: Vec<NarrativeEntry>,
}

/// Narrates the tick record: every record with the narrated events of its
/// range. One entry per event, in chain order, so every event appears in
/// exactly one tick.
#[must_use]
pub fn narrate_ticks(
    match_: &Match,
    chain: &EventChain,
    records: Vec<TickRecord>,
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Every event is narrated in exactly one tick, in chain order (moved
    /// here from the engine's suite with `narrate_ticks`, code audit B5).
    #[test]
    fn every_event_is_narrated_in_exactly_one_tick() {
        let (m, result, records) = &*crate::test_support::SEED_42;
        let narrated = narrate_ticks(m, &result.events, records.clone());
        let entries: Vec<_> = narrated.iter().flat_map(|t| t.entries.iter()).collect();
        assert_eq!(entries.len(), result.events.len());
        for (entry, event) in entries.iter().zip(result.events.events()) {
            assert_eq!(entry.event_id, event.event_id, "same order as the chain");
        }
    }
}
