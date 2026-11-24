//! `ds-season-api` — the contract of a season plugin (ADR-113).
//!
//! The season core, `ds-season`, keeps only the clock of the season (a day
//! at a time), the registry of the plugins, the events and the save. All
//! the football is in the plugins, and every plugin speaks to the core
//! through this contract: the moments of the day it acts in (§1), its card
//! (§3, shared with the match), and the season as it sees it (§2).
//!
//! # Design rules
//!
//! - No dependency on any `ds-*` crate but the shared card (ADR-113 §3).
//! - No I/O, no clock, no randomness: a plugin is deterministic (ADR-035).
//! - Plugins speak only through events and data (§5).

pub use ds_plugin_api::{CardError, OptionalRead, Tier, order};

/// The five moments of a day, in the order the core calls them (§1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Moment {
    /// The day starts: recoveries, sessions, deadlines.
    DayStarted,
    /// Before the day's matches: line-ups announced, suspensions applied;
    /// the matchday plugin plays the matches here (§6, §7).
    BeforeMatches,
    /// After each match of the day, in the order of the match id.
    AfterMatch,
    /// The day ends: standings, the end of a matchday.
    DayEnded,
    /// Once, at the end of the season: promotions, relegations, renewals.
    SeasonEnded,
}

impl Moment {
    /// The moments of an ordinary day; `AfterMatch` is called once per
    /// match played.
    pub const DAY: [Self; 4] = [
        Self::DayStarted,
        Self::BeforeMatches,
        Self::AfterMatch,
        Self::DayEnded,
    ];
}

/// The card of a season plugin: the shared card, acting in the moments of
/// a day.
pub type PluginCard = ds_plugin_api::PluginCard<Moment>;

/// The event of an input given to a plugin by the human player (ADR-060):
/// its kind and fields, in the plugin's name, before the day it is for.
pub const INPUT_GIVEN: &str = "InputGiven";

/// The kind of the event a matchday plugin emits for every match it plays;
/// the core calls `AfterMatch` once for each (§1, §6).
pub const MATCH_PLAYED: &str = "MatchPlayed";

/// A fact of the season (§5): its day, the moment and the plugin it was
/// born in, its kind (English, ADR-007) and its fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeasonEvent {
    /// The day of the season, from 0.
    pub day: u32,
    /// The moment it was born in.
    pub moment: Moment,
    /// The plugin that emitted it.
    pub plugin: &'static str,
    /// Its kind: `DayStarted`, `MatchPlayed`, `Booking`, ...
    pub kind: String,
    /// Its fields, in the order given.
    pub fields: Vec<(String, String)>,
}

/// Why a plugin could not do its work.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginError(pub String);

/// The season as a plugin sees it, in one moment (§2): the day, the events
/// already born, the match it is called for in `AfterMatch`, and the
/// channel to tell what it did.
pub struct SeasonWorld<'a> {
    day: u32,
    moment: Moment,
    plugin: &'static str,
    seed: u64,
    events: &'a [SeasonEvent],
    current_match: Option<&'a SeasonEvent>,
    out: &'a mut Vec<SeasonEvent>,
}

impl<'a> SeasonWorld<'a> {
    /// The season as `plugin` sees it at `moment` of `day` (built by the core).
    #[must_use]
    pub const fn new(
        (day, moment, plugin, seed): (u32, Moment, &'static str, u64),
        events: &'a [SeasonEvent],
        current_match: Option<&'a SeasonEvent>,
        out: &'a mut Vec<SeasonEvent>,
    ) -> Self {
        Self {
            day,
            moment,
            plugin,
            seed,
            events,
            current_match,
            out,
        }
    }

    /// The day of the season, from 0.
    #[must_use]
    pub const fn day(&self) -> u32 {
        self.day
    }

    /// The seed of the world (ADR-029): a plugin derives its own from it,
    /// never draws one.
    #[must_use]
    pub const fn seed(&self) -> u64 {
        self.seed
    }

    /// The events born before this moment, oldest first.
    #[must_use]
    pub const fn events(&self) -> &[SeasonEvent] {
        self.events
    }

    /// In `AfterMatch`, the match it is called for.
    #[must_use]
    pub const fn current_match(&self) -> Option<&SeasonEvent> {
        self.current_match
    }

    /// Tells a fact of the season, born now, from this plugin.
    pub fn emit(&mut self, kind: &str, fields: &[(&str, String)]) {
        self.out.push(SeasonEvent {
            day: self.day,
            moment: self.moment,
            plugin: self.plugin,
            kind: kind.to_string(),
            fields: fields
                .iter()
                .map(|(k, v)| ((*k).to_string(), v.clone()))
                .collect(),
        });
    }
}

/// A plugin of the season (ADR-113 §2).
pub trait SeasonPlugin {
    /// Its card.
    fn card(&self) -> PluginCard;

    /// What it does at `moment`, on the season as the plugins before it
    /// left it. It is called only at the moments of its card.
    ///
    /// # Errors
    ///
    /// A [`PluginError`] if it cannot do its work; the season stops there.
    fn run(&mut self, moment: Moment, world: &mut SeasonWorld<'_>) -> Result<(), PluginError>;

    /// Its state, as text, for the save (§9). A plugin with no state keeps
    /// the default.
    fn state(&self) -> String {
        String::new()
    }

    /// Its state back from the save.
    ///
    /// # Errors
    ///
    /// A [`PluginError`] if the text is not a state of this version.
    fn restore(&mut self, _state: &str) -> Result<(), PluginError> {
        Ok(())
    }

    /// An input of the human player for this plugin (ADR-060: a choice with
    /// its day), between two days. A plugin keeps it in its state, so the
    /// save replays it.
    ///
    /// # Errors
    ///
    /// A [`PluginError`] if it takes no input of this kind, or not this one.
    fn input(&mut self, kind: &str, _fields: &[(String, String)]) -> Result<(), PluginError> {
        Err(PluginError(format!("takes no input {kind}")))
    }
}
