//! `ds-world` — the world plugin (`L1.md`, "Mondo"; ADR-113).
//!
//! The world is a pack: a folder of clear files (`L1-season/world/v1/`,
//! its format in `FORMAT.md`). What a file says wins; what it leaves out
//! the seed makes, by keyed choices, the same for the same seed (ADR-029,
//! no die). The pack shipped has the real cities, regions and colours of
//! its clubs and invented names (the maintainer, 2026-11-24); real names
//! come only from a user's pack, written by the importers of another
//! repository in the same format.

use std::path::Path;

use ds_season_api::{Moment, PluginCard, PluginError, SeasonPlugin, SeasonWorld, Tier};
use serde::{Deserialize, Serialize};

/// The version of the pack's format this crate reads (`FORMAT.md`).
pub const FORMAT_VERSION: u32 = 1;

/// The event told for every club made (`L1.md`).
pub const CLUB_CREATED: &str = "ClubCreated";

/// The files of a pack, by name, as text.
const V1_FILES: [(&str, &str); 9] = [
    ("nation.json", include_str!("../../../world/v1/nation.json")),
    ("clubs.json", include_str!("../../../world/v1/clubs.json")),
    (
        "players.json",
        include_str!("../../../world/v1/players.json"),
    ),
    (
        "names/word_parts.json",
        include_str!("../../../world/v1/names/word_parts.json"),
    ),
    (
        "names/club_forms.json",
        include_str!("../../../world/v1/names/club_forms.json"),
    ),
    (
        "names/stadium_forms.json",
        include_str!("../../../world/v1/names/stadium_forms.json"),
    ),
    (
        "names/first_names.json",
        include_str!("../../../world/v1/names/first_names.json"),
    ),
    (
        "names/surnames.json",
        include_str!("../../../world/v1/names/surnames.json"),
    ),
    (
        "names/squad_roles.json",
        include_str!("../../../world/v1/names/squad_roles.json"),
    ),
];

/// Why a pack does not load: the file, the entry (if any), the field, why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackError {
    /// The file, from the pack's folder.
    pub file: String,
    /// The entry of a list, from 0.
    pub entry: Option<usize>,
    /// The field.
    pub field: String,
    /// Why, in words.
    pub why: String,
}

impl std::fmt::Display for PackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.entry {
            Some(i) => write!(f, "{}, voce {i}, {}: {}", self.file, self.field, self.why),
            None => write!(f, "{}, {}: {}", self.file, self.field, self.why),
        }
    }
}

fn err(file: &str, entry: Option<usize>, field: &str, why: impl Into<String>) -> PackError {
    PackError {
        file: file.into(),
        entry,
        field: field.into(),
        why: why.into(),
    }
}

/// A series (`nation.json`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeriesSpec {
    /// Its name.
    pub name: String,
    /// How many clubs play in it.
    pub clubs: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Nation {
    format_version: u32,
    #[serde(rename = "nation")]
    name: String,
    series: Vec<SeriesSpec>,
}

/// A colour as the pack writes it: `"#RRGGBB"`, as in every image editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Colour(pub [u8; 3]);

impl Serialize for Colour {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let [r, g, b] = self.0;
        s.serialize_str(&format!("#{r:02X}{g:02X}{b:02X}"))
    }
}

impl<'de> Deserialize<'de> for Colour {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let text = String::deserialize(d)?;
        let hex = text.strip_prefix('#').unwrap_or(&text);
        let byte = |i: usize| {
            hex.get(i..i + 2)
                .and_then(|h| u8::from_str_radix(h, 16).ok())
        };
        match (hex.len(), byte(0), byte(2), byte(4)) {
            (6, Some(r), Some(g), Some(b)) => Ok(Self([r, g, b])),
            _ => Err(serde::de::Error::custom(format!(
                "{text} non è un colore \"#RRGGBB\""
            ))),
        }
    }
}

/// A club as the pack writes it (`clubs.json`): the seat and colours, and
/// what it fixes by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClubEntry {
    /// Its city.
    pub city: String,
    /// Its region.
    pub region: String,
    /// Its two colours.
    pub colours: [Colour; 2],
    /// Its series, from 0.
    pub series: u32,
    /// Its name, if the pack fixes it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Its stadium, if the pack fixes it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stadium: Option<String>,
}

/// A player as the pack writes it (`players.json`): what it fixes by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerEntry {
    /// His club, its index in `clubs.json`.
    pub club: u32,
    /// His place in the squad, from 0.
    pub number: u32,
    /// His name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// His role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// His year of birth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub born: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Parts3 {
    start: Vec<String>,
    middle: Vec<String>,
    end: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Parts2 {
    start: Vec<String>,
    end: Vec<String>,
}

/// A world pack, validated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pack {
    /// The nation.
    pub nation: String,
    /// Its series, from the first.
    pub series: Vec<SeriesSpec>,
    /// Its clubs, in the order of the series.
    pub clubs: Vec<ClubEntry>,
    /// The players it fixes by hand.
    pub players: Vec<PlayerEntry>,
    /// The squad, one role for each place.
    pub squad_roles: Vec<String>,
    words: Parts3,
    club_forms: Vec<String>,
    stadium_forms: Vec<String>,
    first_names: Vec<String>,
    surnames: Parts2,
}

fn parse<T: serde::de::DeserializeOwned>(file: &str, text: &str) -> Result<T, PackError> {
    serde_json::from_str(text)
        .map_err(|e| err(file, None, &format!("riga {}", e.line()), e.to_string()))
}

const ROLES: [&str; 6] = [
    "goalkeeper",
    "full_back",
    "centre_back",
    "wide_midfielder",
    "central_midfielder",
    "striker",
];

impl Pack {
    /// The pack shipped with the game, v1.
    ///
    /// # Panics
    ///
    /// If the shipped pack does not validate (the tests read it).
    #[must_use]
    pub fn v1() -> Self {
        Self::from_files(|name| {
            V1_FILES
                .iter()
                .find(|(n, _)| *n == name)
                .map(|(_, t)| (*t).to_string())
        })
        .unwrap_or_else(|e| panic!("the shipped world pack: {e}"))
    }

    /// A pack from a folder (a user's pack, an importer's).
    ///
    /// # Errors
    ///
    /// A [`PackError`] with the file, the entry, the field and why.
    pub fn load_dir(dir: &Path) -> Result<Self, PackError> {
        Self::from_files(|name| std::fs::read_to_string(dir.join(name)).ok())
    }

    /// A pack from its files, by name; the validator (`FORMAT.md`).
    ///
    /// # Errors
    ///
    /// A [`PackError`] with the file, the entry, the field and why.
    #[allow(clippy::too_many_lines)]
    pub fn from_files(read: impl Fn(&str) -> Option<String>) -> Result<Self, PackError> {
        let need = |name: &str| read(name).ok_or_else(|| err(name, None, "file", "manca"));
        let nation: Nation = parse("nation.json", &need("nation.json")?)?;
        if nation.format_version != FORMAT_VERSION {
            return Err(err(
                "nation.json",
                None,
                "format_version",
                format!(
                    "è {}, questo gioco legge la {FORMAT_VERSION}",
                    nation.format_version
                ),
            ));
        }
        if nation.name.trim().is_empty() {
            return Err(err("nation.json", None, "nation", "vuoto"));
        }
        if nation.series.is_empty() {
            return Err(err("nation.json", None, "series", "serve almeno una serie"));
        }
        for (i, s) in nation.series.iter().enumerate() {
            if s.clubs < 2 {
                return Err(err(
                    "nation.json",
                    Some(i),
                    "clubs",
                    "servono almeno due club",
                ));
            }
        }
        let clubs: Vec<ClubEntry> = parse("clubs.json", &need("clubs.json")?)?;
        let total: u32 = nation.series.iter().map(|s| s.clubs).sum();
        if u32::try_from(clubs.len()).unwrap_or(u32::MAX) != total {
            return Err(err(
                "clubs.json",
                None,
                "lista",
                format!("{} club, le serie ne vogliono {total}", clubs.len()),
            ));
        }
        let mut names: Vec<&str> = Vec::new();
        let mut last_series = 0;
        for (i, c) in clubs.iter().enumerate() {
            if c.city.trim().is_empty() {
                return Err(err("clubs.json", Some(i), "city", "vuota"));
            }
            if c.region.trim().is_empty() {
                return Err(err("clubs.json", Some(i), "region", "vuota"));
            }
            if c.colours[0] == c.colours[1] {
                return Err(err(
                    "clubs.json",
                    Some(i),
                    "colours",
                    "i due colori sono uguali",
                ));
            }
            if c.series as usize >= nation.series.len() || c.series < last_series {
                return Err(err(
                    "clubs.json",
                    Some(i),
                    "series",
                    "non è una serie, o le voci non sono in ordine di serie",
                ));
            }
            last_series = c.series;
            if let Some(n) = &c.name {
                if names.contains(&n.as_str()) {
                    return Err(err(
                        "clubs.json",
                        Some(i),
                        "name",
                        "due club con lo stesso nome",
                    ));
                }
                names.push(n);
            }
        }
        let squad_roles: Vec<String> =
            parse("names/squad_roles.json", &need("names/squad_roles.json")?)?;
        for (i, r) in squad_roles.iter().enumerate() {
            if !ROLES.contains(&r.as_str()) {
                return Err(err(
                    "names/squad_roles.json",
                    Some(i),
                    "ruolo",
                    format!("{r} non è un ruolo"),
                ));
            }
        }
        let players: Vec<PlayerEntry> = match read("players.json") {
            Some(t) => parse("players.json", &t)?,
            None => Vec::new(),
        };
        let mut seen = Vec::new();
        for (i, p) in players.iter().enumerate() {
            if p.club as usize >= clubs.len() {
                return Err(err(
                    "players.json",
                    Some(i),
                    "club",
                    "non è un club di clubs.json",
                ));
            }
            if p.number as usize >= squad_roles.len() {
                return Err(err("players.json", Some(i), "number", "oltre la rosa"));
            }
            if p.role
                .as_ref()
                .is_some_and(|r| !ROLES.contains(&r.as_str()))
            {
                return Err(err("players.json", Some(i), "role", "non è un ruolo"));
            }
            if p.born.is_some_and(|b| !(1950..=2015).contains(&b)) {
                return Err(err("players.json", Some(i), "born", "fuori da 1950-2015"));
            }
            if seen.contains(&(p.club, p.number)) {
                return Err(err(
                    "players.json",
                    Some(i),
                    "number",
                    "due giocatori nello stesso posto",
                ));
            }
            seen.push((p.club, p.number));
        }
        let pack = Self {
            nation: nation.name,
            series: nation.series,
            clubs,
            players,
            squad_roles,
            words: parse("names/word_parts.json", &need("names/word_parts.json")?)?,
            club_forms: parse("names/club_forms.json", &need("names/club_forms.json")?)?,
            stadium_forms: parse(
                "names/stadium_forms.json",
                &need("names/stadium_forms.json")?,
            )?,
            first_names: parse("names/first_names.json", &need("names/first_names.json")?)?,
            surnames: parse("names/surnames.json", &need("names/surnames.json")?)?,
        };
        for (file, empty) in [
            (
                "names/word_parts.json",
                pack.words.start.is_empty()
                    || pack.words.middle.is_empty()
                    || pack.words.end.is_empty(),
            ),
            ("names/club_forms.json", pack.club_forms.is_empty()),
            ("names/stadium_forms.json", pack.stadium_forms.is_empty()),
            ("names/first_names.json", pack.first_names.is_empty()),
            (
                "names/surnames.json",
                pack.surnames.start.is_empty() || pack.surnames.end.is_empty(),
            ),
            ("names/squad_roles.json", pack.squad_roles.is_empty()),
        ] {
            if empty {
                return Err(err(file, None, "lista", "vuota"));
            }
        }
        Ok(pack)
    }

    /// A generated player name (first name and surname) for `key`.
    #[must_use]
    pub fn player_name(&self, seed: u64, key: u64) -> String {
        format!(
            "{} {}{}",
            self.first_names[pick(seed, "first_name", key, self.first_names.len())],
            self.surnames.start[pick(seed, "surname_start", key, self.surnames.start.len())],
            self.surnames.end[pick(seed, "surname_end", key, self.surnames.end.len())],
        )
    }

    /// The pack's files with every field filled from `seed` (the export):
    /// a whole world to retouch.
    #[must_use]
    pub fn export(&self, seed: u64, players: &[PlayerEntry]) -> Vec<(String, String)> {
        let j = |v: &dyn erased::Json| v.json();
        let made = make_clubs(self, seed);
        let clubs: Vec<ClubEntry> = made
            .iter()
            .zip(&self.clubs)
            .map(|(m, c)| ClubEntry {
                name: Some(m.name.clone()),
                stadium: Some(m.stadium.clone()),
                ..c.clone()
            })
            .collect();
        let nation = Nation {
            format_version: FORMAT_VERSION,
            name: self.nation.clone(),
            series: self.series.clone(),
        };
        vec![
            ("nation.json".into(), j(&nation)),
            ("clubs.json".into(), j(&clubs)),
            ("players.json".into(), j(&players)),
            ("names/word_parts.json".into(), j(&self.words)),
            ("names/club_forms.json".into(), j(&self.club_forms)),
            ("names/stadium_forms.json".into(), j(&self.stadium_forms)),
            ("names/first_names.json".into(), j(&self.first_names)),
            ("names/surnames.json".into(), j(&self.surnames)),
            ("names/squad_roles.json".into(), j(&self.squad_roles)),
        ]
    }
}

mod erased {
    /// Pretty JSON of any serialisable value.
    pub trait Json {
        fn json(&self) -> String;
    }
    impl<T: serde::Serialize> Json for T {
        fn json(&self) -> String {
            serde_json::to_string_pretty(self).unwrap_or_default() + "\n"
        }
    }
}

/// A club of the world, every field filled.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Club {
    /// Its index in the world, from 0.
    pub id: u32,
    /// Its name.
    pub name: String,
    /// Its city.
    pub city: String,
    /// Its region.
    pub region: String,
    /// Its stadium.
    pub stadium: String,
    /// Its two colours.
    pub colours: [[u8; 3]; 2],
    /// The series it starts in, from 0 (the first).
    pub series: u32,
}

/// A keyed choice in `0..choices` from the seed (ADR-029): never a draw.
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

/// The clubs of the world from `seed`: what the pack fixes, and the rest
/// made; a made name is invented, unique, and never contains its city.
#[must_use]
pub fn make_clubs(pack: &Pack, seed: u64) -> Vec<Club> {
    let fixed: Vec<String> = pack.clubs.iter().filter_map(|c| c.name.clone()).collect();
    let mut taken: Vec<String> = fixed;
    let mut clubs = Vec::with_capacity(pack.clubs.len());
    for (i, c) in pack.clubs.iter().enumerate() {
        let id = u32::try_from(i).unwrap_or(0);
        let k = u64::from(id);
        let name = c.name.clone().unwrap_or_else(|| {
            let made = (0u64..1000)
                .map(|attempt| {
                    let key = k * 1000 + attempt;
                    let word = format!(
                        "{}{}{}",
                        pack.words.start[pick(seed, "word_start", key, pack.words.start.len())],
                        pack.words.middle[pick(seed, "word_middle", key, pack.words.middle.len())],
                        pack.words.end[pick(seed, "word_end", key, pack.words.end.len())],
                    );
                    pack.club_forms[pick(seed, "club_form", key, pack.club_forms.len())]
                        .replace('@', &word)
                })
                .find(|n| !taken.contains(n) && !n.contains(&c.city))
                .unwrap_or_default();
            taken.push(made.clone());
            made
        });
        let stadium = c.stadium.clone().unwrap_or_else(|| {
            pack.stadium_forms[pick(seed, "stadium", k, pack.stadium_forms.len())]
                .replace("{city}", &c.city)
        });
        clubs.push(Club {
            id,
            name,
            city: c.city.clone(),
            region: c.region.clone(),
            stadium,
            colours: [c.colours[0].0, c.colours[1].0],
            series: c.series,
        });
    }
    clubs
}

/// The world plugin: on the first day it makes the clubs and tells each.
#[derive(Debug)]
pub struct World {
    pack: Pack,
    clubs: Vec<Club>,
}

impl Default for World {
    fn default() -> Self {
        Self::new(Pack::v1())
    }
}

impl World {
    /// The world of `pack`.
    #[must_use]
    pub const fn new(pack: Pack) -> Self {
        Self {
            pack,
            clubs: Vec::new(),
        }
    }

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
        self.clubs = make_clubs(&self.pack, w.seed());
        for c in &self.clubs {
            w.emit(
                CLUB_CREATED,
                &[
                    ("id", c.id.to_string()),
                    ("name", c.name.clone()),
                    ("city", c.city.clone()),
                    ("region", c.region.clone()),
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

    fn v1_with(file: &'static str, text: &'static str) -> Result<Pack, super::PackError> {
        Pack::from_files(|name| {
            if name == file {
                Some(text.to_string())
            } else {
                super::V1_FILES
                    .iter()
                    .find(|(n, _)| *n == name)
                    .map(|(_, t)| (*t).to_string())
            }
        })
    }

    /// The shipped pack: 20 clubs, a name each, never with its city; the
    /// same world from the same seed.
    #[test]
    fn the_world_has_its_clubs_the_same_from_the_same_seed() {
        let pack = Pack::v1();
        let a = make_clubs(&pack, 11);
        assert_eq!(a.len(), 20);
        let mut names: Vec<_> = a.iter().map(|c| c.name.clone()).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), 20, "a name each");
        assert!(a.iter().all(|c| !c.name.contains(&c.city)));
        assert!(a.iter().all(|c| c.colours[0] != c.colours[1]));
        assert_eq!(a, make_clubs(&pack, 11));
        assert_ne!(a, make_clubs(&pack, 12));
    }

    /// What the file says wins over the generator.
    #[test]
    fn a_name_written_in_the_pack_wins() {
        let mut pack = Pack::v1();
        pack.clubs[3].name = Some("Polisportiva Lariana".into());
        let clubs = make_clubs(&pack, 11);
        assert_eq!(clubs[3].name, "Polisportiva Lariana");
        assert_eq!(
            clubs[4],
            make_clubs(&Pack::v1(), 11)[4],
            "the others as they were"
        );
    }

    /// The validator says the file, the entry, the field and why.
    #[test]
    fn a_wrong_pack_says_where_and_why() {
        let e = v1_with("players.json", r#"[{ "club": 99, "number": 0 }]"#).unwrap_err();
        assert_eq!(
            (e.file.as_str(), e.entry, e.field.as_str()),
            ("players.json", Some(0), "club")
        );
        let e = v1_with(
            "nation.json",
            r#"{ "format_version": 2, "nation": "X", "series": [] }"#,
        )
        .unwrap_err();
        assert_eq!(e.field, "format_version");
        assert!(e.to_string().contains("nation.json"));
        let twice = r#"[{ "club": 0, "number": 1 }, { "club": 0, "number": 1 }]"#;
        assert_eq!(v1_with("players.json", twice).unwrap_err().entry, Some(1));
    }

    /// The export writes a pack that loads back to the same world.
    #[test]
    fn the_exported_world_loads_back_the_same() {
        let pack = Pack::v1();
        let files = pack.export(11, &[]);
        let back = Pack::from_files(|name| {
            files
                .iter()
                .find(|(n, _)| n == name)
                .map(|(_, t)| t.clone())
        })
        .expect("loads");
        assert_eq!(make_clubs(&back, 11), make_clubs(&pack, 11));
        assert_eq!(
            make_clubs(&back, 99),
            make_clubs(&back, 11),
            "every name fixed now"
        );
    }
}
