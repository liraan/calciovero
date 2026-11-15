//! The psychology plugin.
//!
//! The psychology plugin (ADR-063 §4, ADR-057): courage at a high ball
//! against a heavier man (`goes_up` in the thought), composure in the shot
//! under pressure (`ds_calculator::kick`). It may be switched off
//! (`Match.psychology`): then courage holds nobody back and composure
//! leaves the pressure as it is.

use ds_match_api::{MatchPlugin, PluginCard};

/// The psychology of the players in the match.
#[derive(Debug, Clone, Copy, Default)]
pub struct Psychology;

impl MatchPlugin for Psychology {
    fn card(&self) -> PluginCard {
        crate::registry::card("psychology")
    }
}
