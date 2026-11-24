//! `ds-world` — the world plugin (`L1.md`, "Mondo"; ADR-113).
//!
//! It reads the world pack (data: the nation, the format of its first
//! series, the parts names are made of) and, from the seed, makes the clubs:
//! a city, a name, a stadium, two colours, the series they start in. The
//! world is invented and has no real name (decision F); real names come
//! only from a user's pack (L3). No die (ADR-035): every choice is a keyed
//! hash of the seed, the same world from the same seed.

use ds_season_api::{Moment, PluginCard, PluginError, SeasonPlugin, SeasonWorld, Tier};
use serde::{Deserialize, Serialize};

const PACK_JSON: &str = include_str!("../../../world/v1.json");

/// The event told for every club made (`L1.md`).
pub const CLUB_CREATED: &str = "ClubCreated";

/// A series of the pack.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SeriesSpec {
    /// Its name.
    pub name: String,
    /// How many clubs play in it.
    pub clubs: u32,
}

/// The world pack (`L1-season/world/v1.json`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Pack {
    /// The nation.
    pub nation: String,
    /// Its series, from the first.
    pub series: Vec<SeriesSpec>,
    city_start: Vec<String>,
    city_middle: Vec<String>,
    city_end: Vec<String>,
    club_forms: Vec<String>,
    stadium_forms: Vec<String>,
    palette: Vec<[u8; 3]>,
}

impl Pack {
    /// The embedded pack v1.
    ///
    /// # Panics
    ///
    /// If the embedded file is not valid (the tests read it).
    #[must_use]
    pub fn v1() -> Self {
        serde_json::from_str(PACK_JSON).expect("the embedded world pack is valid")
    }
}

/// A club of the world.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Club {
    /// Its index in the world, from 0.
    pub id: u32,
    /// Its name.
    pub name: String,
    /// Its city.
    pub city: String,
    /// Its stadium.
    pub stadium: String,
    /// Its two colours.
    pub colours: [[u8; 3]; 2],
    /// The series it starts in, from 0 (the first).
    pub series: u32,
}

/// A keyed choice in `0..n` from the seed (ADR-029): never a draw.
#[must_use]
pub fn pick(seed: u64, label: &str, key: u64, choices: usize) -> usize {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&seed.to_le_bytes());
    hasher.update(label.as_bytes());
    hasher.update(&key.to_le_bytes());
    let digest = hasher.finalize();
    let mut word = [0u8; 8];
    word.copy_from_slice(&digest.as_bytes()[..8]);
    usize::try_from(u64::from_le_bytes(word) % (choices.max(1) as u64)).unwrap_or(0)
}

/// The clubs of the world from `seed`: the first series' clubs, each with
/// a city of its own.
#[must_use]
pub fn make_clubs(pack: &Pack, seed: u64) -> Vec<Club> {
    let mut clubs = Vec::new();
    let mut cities: Vec<String> = Vec::new();
    let mut id = 0u32;
    for (series, spec) in pack.series.iter().enumerate() {
        for _ in 0..spec.clubs {
            // A city no other club has: the next keyed attempt otherwise.
            // At most a thousand keyed attempts; the pack has far more cities.
            let city = (0u64..1000)
                .map(|attempt| {
                    let k = u64::from(id) * 1000 + attempt;
                    format!(
                        "{}{}{}",
                        pack.city_start[pick(seed, "city_start", k, pack.city_start.len())],
                        pack.city_middle[pick(seed, "city_middle", k, pack.city_middle.len())],
                        pack.city_end[pick(seed, "city_end", k, pack.city_end.len())],
                    )
                })
                .find(|c| !cities.contains(c))
                .unwrap_or_default();
            cities.push(city.clone());
            let k = u64::from(id);
            let name = pack.club_forms[pick(seed, "club_form", k, pack.club_forms.len())]
                .replace('@', &city);
            let stadium = pack.stadium_forms[pick(seed, "stadium", k, pack.stadium_forms.len())]
                .replace('@', &city);
            let first = pick(seed, "colour_1", k, pack.palette.len());
            let second = (first + 1 + pick(seed, "colour_2", k, pack.palette.len() - 1))
                % pack.palette.len();
            clubs.push(Club {
                id,
                name,
                city,
                stadium,
                colours: [pack.palette[first], pack.palette[second]],
                series: u32::try_from(series).unwrap_or(0),
            });
            id += 1;
        }
    }
    clubs
}

/// The world plugin: on the first day it makes the clubs and tells each.
#[derive(Debug, Default)]
pub struct World {
    clubs: Vec<Club>,
}

impl World {
    /// The clubs made so far.
    #[must_use]
    pub fn clubs(&self) -> &[Club] {
        &self.clubs
    }
}

impl SeasonPlugin for World {
    fn card(&self) -> PluginCard {
        PluginCard {
            name: "world",
            version: "1",
            steps: &[Moment::DayStarted],
            depends: &[],
            reads: &[],
            optional_reads: &[],
            writes: &["world.club", "world.city", "world.stadium"],
            tier: Tier::Core,
            cost_us: 0,
            narrative: &["club_created"],
        }
    }

    fn run(&mut self, _: Moment, w: &mut SeasonWorld<'_>) -> Result<(), PluginError> {
        if w.day() != 0 || !self.clubs.is_empty() {
            return Ok(());
        }
        self.clubs = make_clubs(&Pack::v1(), w.seed());
        for c in &self.clubs {
            w.emit(
                CLUB_CREATED,
                &[
                    ("id", c.id.to_string()),
                    ("name", c.name.clone()),
                    ("city", c.city.clone()),
                    ("stadium", c.stadium.clone()),
                    ("series", c.series.to_string()),
                ],
            );
        }
        Ok(())
    }

    fn state(&self) -> String {
        serde_json::to_string(&self.clubs).unwrap_or_default()
    }

    fn restore(&mut self, state: &str) -> Result<(), PluginError> {
        self.clubs = if state.is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(state).map_err(|e| PluginError(format!("world state: {e}")))?
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Pack, make_clubs};

    /// The first series of the pack has its real count, every club its own
    /// city and name, and the same seed makes the same world.
    #[test]
    fn the_world_has_its_clubs_the_same_from_the_same_seed() {
        let pack = Pack::v1();
        let a = make_clubs(&pack, 11);
        assert_eq!(a.len(), 20);
        let mut cities: Vec<_> = a.iter().map(|c| c.city.clone()).collect();
        cities.sort();
        cities.dedup();
        assert_eq!(cities.len(), 20, "a city each");
        assert!(a.iter().all(|c| c.colours[0] != c.colours[1]));
        assert_eq!(a, make_clubs(&pack, 11));
        assert_ne!(a, make_clubs(&pack, 12));
    }
}
