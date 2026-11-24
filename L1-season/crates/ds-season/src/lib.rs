//! `ds-season` — the season core (ADR-113 §1).
//!
//! It keeps the clock of the season, a day at a time, the registry of the
//! plugins in the validator's order, the events and the save. It knows no
//! football: it calls the plugins at the five moments of the day, and
//! `AfterMatch` once for every `MatchPlayed` the day's plugins told.

use ds_season_api::{
    CardError, MATCH_PLAYED, Moment, PluginError, SeasonEvent, SeasonPlugin, SeasonWorld,
};
use serde::{Deserialize, Serialize};

/// The version of the season's behaviour (ADR-113 §8).
///
/// A season that comes out different for the same seed and choices, by a
/// change of a season plugin, is a new `SEASON_VERSION`, with its entry in
/// `CHANGELOG-SEASON.md`. A change of the match engine is `SIM_VERSION`.
pub const SEASON_VERSION: u32 = 5;

/// Why a season stops or does not load.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeasonError {
    /// The cards do not pass the validator.
    Card(CardError),
    /// A plugin failed: its name and why.
    Plugin(&'static str, String),
    /// The save is not of these plugins or of this version.
    Save(String),
}

/// The manifest of a plugin in the save (ADR-013): name, version, state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedPlugin {
    /// Its name.
    pub name: String,
    /// Its version.
    pub version: String,
    /// Its state, as it wrote it.
    pub state: String,
}

/// A saved event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedEvent {
    /// Its day.
    pub day: u32,
    /// Its moment, by name.
    pub moment: String,
    /// The plugin that emitted it.
    pub plugin: String,
    /// Its kind.
    pub kind: String,
    /// Its fields.
    pub fields: Vec<(String, String)>,
}

/// The save of a season (ADR-113 §9): the manifest, the clock, the state of
/// every plugin under its name, the events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeasonSave {
    /// `SEASON_VERSION` when it was saved.
    pub season_version: u32,
    /// The seed of the world.
    pub seed: u64,
    /// The days of the season.
    pub days: u32,
    /// The next day to play.
    pub day: u32,
    /// The plugins, in their order, with their state.
    pub plugins: Vec<SavedPlugin>,
    /// The events so far.
    pub events: Vec<SavedEvent>,
}

/// A season in progress.
pub struct Season {
    seed: u64,
    days: u32,
    day: u32,
    plugins: Vec<Box<dyn SeasonPlugin>>,
    events: Vec<SeasonEvent>,
}

const fn moment_name(m: Moment) -> &'static str {
    match m {
        Moment::DayStarted => "DayStarted",
        Moment::BeforeMatches => "BeforeMatches",
        Moment::AfterMatch => "AfterMatch",
        Moment::DayEnded => "DayEnded",
        Moment::SeasonEnded => "SeasonEnded",
    }
}

fn moment_of(name: &str) -> Option<Moment> {
    [
        Moment::DayStarted,
        Moment::BeforeMatches,
        Moment::AfterMatch,
        Moment::DayEnded,
        Moment::SeasonEnded,
    ]
    .into_iter()
    .find(|m| moment_name(*m) == name)
}

impl Season {
    /// A season of `days` days from `seed`, with `plugins` put in the
    /// validator's order (ADR-113 §4).
    ///
    /// # Errors
    ///
    /// [`SeasonError::Card`] if the cards do not pass the validator.
    pub fn new(
        seed: u64,
        days: u32,
        plugins: Vec<Box<dyn SeasonPlugin>>,
    ) -> Result<Self, SeasonError> {
        let cards: Vec<_> = plugins.iter().map(|p| p.card()).collect();
        let order = ds_season_api::order(&cards).map_err(SeasonError::Card)?;
        let mut slots: Vec<Option<Box<dyn SeasonPlugin>>> = plugins.into_iter().map(Some).collect();
        let plugins = order.into_iter().filter_map(|i| slots[i].take()).collect();
        Ok(Self {
            seed,
            days,
            day: 0,
            plugins,
            events: Vec::new(),
        })
    }

    /// The next day to play, from 0.
    #[must_use]
    pub const fn day(&self) -> u32 {
        self.day
    }

    /// Whether every day has been played.
    #[must_use]
    pub const fn is_over(&self) -> bool {
        self.day >= self.days
    }

    /// The events so far, oldest first.
    #[must_use]
    pub fn events(&self) -> &[SeasonEvent] {
        &self.events
    }

    /// The plugins' names, in the order they act.
    #[must_use]
    pub fn order(&self) -> Vec<&'static str> {
        self.plugins.iter().map(|p| p.card().name).collect()
    }

    fn call(&mut self, moment: Moment, current: Option<usize>) -> Result<(), SeasonError> {
        for i in 0..self.plugins.len() {
            let card = self.plugins[i].card();
            if !card.steps.contains(&moment) {
                continue;
            }
            let mut out = Vec::new();
            {
                let current = current.map(|j| &self.events[j]);
                let mut world = SeasonWorld::new(
                    (self.day, moment, card.name, self.seed),
                    &self.events,
                    current,
                    &mut out,
                );
                self.plugins[i]
                    .run(moment, &mut world)
                    .map_err(|PluginError(why)| SeasonError::Plugin(card.name, why))?;
            }
            self.events.extend(out);
        }
        Ok(())
    }

    /// Plays one day: its four moments, `AfterMatch` once for each match
    /// told that day, in the order told; on the last day `SeasonEnded` too.
    /// Returns whether the season goes on.
    ///
    /// # Errors
    ///
    /// [`SeasonError::Plugin`] if a plugin fails.
    pub fn advance_day(&mut self) -> Result<bool, SeasonError> {
        if self.is_over() {
            return Ok(false);
        }
        let first = self.events.len();
        self.call(Moment::DayStarted, None)?;
        self.call(Moment::BeforeMatches, None)?;
        let matches: Vec<usize> = (first..self.events.len())
            .filter(|&j| self.events[j].kind == MATCH_PLAYED)
            .collect();
        for j in matches {
            self.call(Moment::AfterMatch, Some(j))?;
        }
        self.call(Moment::DayEnded, None)?;
        self.day += 1;
        if self.is_over() {
            self.call(Moment::SeasonEnded, None)?;
        }
        Ok(!self.is_over())
    }

    /// Plays the season to its end.
    ///
    /// # Errors
    ///
    /// [`SeasonError::Plugin`] if a plugin fails.
    pub fn run_to_end(&mut self) -> Result<(), SeasonError> {
        while self.advance_day()? {}
        Ok(())
    }

    /// The fingerprint of the season so far: the hash of its events, in
    /// order (the golden of ADR-113 §12).
    #[must_use]
    pub fn hash(&self) -> String {
        let mut h = blake3::Hasher::new();
        for e in &self.events {
            h.update(&e.day.to_le_bytes());
            h.update(moment_name(e.moment).as_bytes());
            h.update(e.plugin.as_bytes());
            h.update(e.kind.as_bytes());
            for (k, v) in &e.fields {
                h.update(k.as_bytes());
                h.update(b"=");
                h.update(v.as_bytes());
                h.update(b";");
            }
            h.update(b"\n");
        }
        h.finalize().to_hex().to_string()
    }

    /// The save (ADR-113 §9).
    #[must_use]
    pub fn save(&self) -> SeasonSave {
        SeasonSave {
            season_version: SEASON_VERSION,
            seed: self.seed,
            days: self.days,
            day: self.day,
            plugins: self
                .plugins
                .iter()
                .map(|p| {
                    let c = p.card();
                    SavedPlugin {
                        name: c.name.to_string(),
                        version: c.version.to_string(),
                        state: p.state(),
                    }
                })
                .collect(),
            events: self
                .events
                .iter()
                .map(|e| SavedEvent {
                    day: e.day,
                    moment: moment_name(e.moment).to_string(),
                    plugin: e.plugin.to_string(),
                    kind: e.kind.clone(),
                    fields: e.fields.clone(),
                })
                .collect(),
        }
    }

    /// The season back from its save, with the same plugins (by name and
    /// version, in any order).
    ///
    /// # Errors
    ///
    /// [`SeasonError::Save`] if the save is of another version or other
    /// plugins, [`SeasonError::Card`] if the cards do not validate.
    pub fn load(
        save: &SeasonSave,
        plugins: Vec<Box<dyn SeasonPlugin>>,
    ) -> Result<Self, SeasonError> {
        if save.season_version != SEASON_VERSION {
            return Err(SeasonError::Save(format!(
                "saved with SEASON_VERSION {}, this is {SEASON_VERSION}",
                save.season_version
            )));
        }
        let mut season = Self::new(save.seed, save.days, plugins)?;
        if season.plugins.len() != save.plugins.len() {
            return Err(SeasonError::Save(
                "not the plugins it was saved with".into(),
            ));
        }
        for (p, s) in season.plugins.iter_mut().zip(&save.plugins) {
            let c = p.card();
            if c.name != s.name || c.version != s.version {
                return Err(SeasonError::Save(format!(
                    "{} {} is not {} {}",
                    c.name, c.version, s.name, s.version
                )));
            }
            p.restore(&s.state)
                .map_err(|PluginError(why)| SeasonError::Plugin(c.name, why))?;
        }
        let names: Vec<&'static str> = season.plugins.iter().map(|p| p.card().name).collect();
        let mut events = Vec::with_capacity(save.events.len());
        for e in &save.events {
            let plugin = names
                .iter()
                .copied()
                .find(|n| *n == e.plugin)
                .ok_or_else(|| {
                    SeasonError::Save(format!("an event of an unknown plugin: {}", e.plugin))
                })?;
            let moment = moment_of(&e.moment)
                .ok_or_else(|| SeasonError::Save(format!("unknown moment {}", e.moment)))?;
            events.push(SeasonEvent {
                day: e.day,
                moment,
                plugin,
                kind: e.kind.clone(),
                fields: e.fields.clone(),
            });
        }
        season.events = events;
        season.day = save.day;
        Ok(season)
    }
}
