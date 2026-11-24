//! `ds-career` — the career (the playable preview; decision BN).
//!
//! A season of a world pack with the human player as one club's coach and
//! every module on. It is what a screen reads and does, with no screen: the
//! desktop and the phone read the same (ADR-061). The human player sees what
//! his coach sees: names, roles, ages, results, the table, the bans, how
//! tired his men are (plane B); never a characteristic (plane A, ADR-112).

use ds_season::{Season, SeasonError, SeasonSave};
use ds_season_api::{MATCH_PLAYED, SeasonEvent, SeasonPlugin};
use ds_world::{Club, Pack};
use serde::{Deserialize, Serialize};

/// Why the career cannot do what was asked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CareerError(pub String);

impl From<SeasonError> for CareerError {
    fn from(e: SeasonError) -> Self {
        Self(format!("{e:?}"))
    }
}

/// A match of the calendar, as the screens show it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fixture {
    /// The day it is played.
    pub day: u32,
    /// The matchday, from 0.
    pub matchday: u32,
    /// The home club.
    pub home: u32,
    /// The away club.
    pub away: u32,
    /// The goals, home and away, once played.
    pub score: Option<(u32, u32)>,
}

/// A man of the human player's squad, as his coach sees him.
#[derive(Debug, Clone, PartialEq)]
pub struct SquadMan {
    /// His identity.
    pub id: u32,
    /// His name.
    pub name: String,
    /// His role.
    pub role: String,
    /// His age this season.
    pub age: u32,
    /// His shirt number.
    pub shirt: u32,
    /// What he carries this morning, 0 fresh to 1 spent (`ds-body`).
    pub tired: f64,
    /// Whether he may not play on the next matchday (`ds-discipline`).
    pub banned: bool,
    /// His cautions this season.
    pub cautions: u32,
}

/// A line of the table, with the club's name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableLine {
    /// The club.
    pub club: u32,
    /// Its line.
    pub row: ds_standings::Row,
}

/// The save of a career: the club, and the season's save (ADR-113 §9).
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CareerSave {
    club: u32,
    season: SeasonSave,
}

/// A career.
pub struct Career {
    pack: Pack,
    seed: u64,
    club: u32,
    season: Season,
}

fn plugins(pack: &Pack, club: u32) -> Vec<Box<dyn SeasonPlugin>> {
    vec![
        Box::new(ds_world::World::new(pack.clone())),
        Box::new(ds_calendar::Calendar::default()),
        Box::new(ds_roster::Roster::new(pack.clone())),
        Box::new(ds_coach::Coach::with_human(pack.clone(), club, Vec::new())),
        Box::new(ds_matchday::Matchday::new(pack.clone())),
        Box::new(ds_standings::Standings::default()),
        Box::new(ds_body::Body::default()),
        Box::new(ds_discipline::Discipline::default()),
        Box::new(ds_week::WeekPlugin::new(pack.clone())),
    ]
}

fn field(e: &SeasonEvent, key: &str) -> Option<u32> {
    e.fields
        .iter()
        .find(|(k, _)| k == key)
        .and_then(|(_, v)| v.parse().ok())
}

impl Career {
    /// A new career in the first series of `pack`'s world from `seed`, the
    /// human player coaching `club`.
    ///
    /// # Errors
    ///
    /// A [`CareerError`] if `club` is not a club of the first series, or
    /// the season does not start.
    pub fn new(pack: Pack, seed: u64, club: u32) -> Result<Self, CareerError> {
        let first = pack.series.first().map_or(0, |s| s.clubs);
        if club >= first {
            return Err(CareerError(format!(
                "club {club} is not in the first series"
            )));
        }
        let days = ds_calendar::season_days(first);
        let mut season = Season::new(seed, days, plugins(&pack, club))?;
        // Day 0 makes the world, the squads and the calendar, with no match:
        // the first screen already has them.
        season.advance_day()?;
        Ok(Self {
            pack,
            seed,
            club,
            season,
        })
    }

    /// The club the human player coaches.
    #[must_use]
    pub const fn club(&self) -> u32 {
        self.club
    }

    /// The world's clubs, every field filled.
    #[must_use]
    pub fn clubs(&self) -> Vec<Club> {
        ds_world::make_clubs(&self.pack, self.seed)
    }

    /// The name of `club`'s coach.
    #[must_use]
    pub fn coach_name(&self, club: u32) -> String {
        ds_coach::coach_name(&self.pack, self.seed, club)
    }

    /// The next day to play, from 0.
    #[must_use]
    pub const fn day(&self) -> u32 {
        self.season.day()
    }

    /// Whether the season is over.
    #[must_use]
    pub const fn is_over(&self) -> bool {
        self.season.is_over()
    }

    /// The events so far (the screens' source, plane B).
    #[must_use]
    pub fn events(&self) -> &[SeasonEvent] {
        self.season.events()
    }

    /// Plays one day; returns the matches played that day.
    ///
    /// # Errors
    ///
    /// A [`CareerError`] if a plugin fails.
    pub fn advance(&mut self) -> Result<Vec<Fixture>, CareerError> {
        let day = self.season.day();
        self.season.advance_day()?;
        Ok(self
            .fixtures()
            .into_iter()
            .filter(|f| f.day == day && f.score.is_some())
            .collect())
    }

    /// Plays days up to and including the human club's next match; returns
    /// every match played on that last day.
    ///
    /// # Errors
    ///
    /// A [`CareerError`] if a plugin fails.
    pub fn advance_to_next_match(&mut self) -> Result<Vec<Fixture>, CareerError> {
        let Some(next) = self.next_fixture() else {
            return Ok(Vec::new());
        };
        let mut last = Vec::new();
        while !self.is_over() && self.season.day() <= next.day {
            last = self.advance()?;
        }
        Ok(last)
    }

    /// The calendar, every match, with the score once played.
    #[must_use]
    pub fn fixtures(&self) -> Vec<Fixture> {
        let events = self.season.events();
        let played: Vec<&SeasonEvent> = events.iter().filter(|e| e.kind == MATCH_PLAYED).collect();
        events
            .iter()
            .filter(|e| e.kind == ds_calendar::FIXTURE_SCHEDULED)
            .filter_map(|e| {
                let (day, matchday, home, away) = (
                    field(e, "day")?,
                    field(e, "matchday")?,
                    field(e, "home")?,
                    field(e, "away")?,
                );
                let score = played
                    .iter()
                    .find(|m| {
                        m.day == day
                            && field(m, "home") == Some(home)
                            && field(m, "away") == Some(away)
                    })
                    .and_then(|m| Some((field(m, "home_goals")?, field(m, "away_goals")?)));
                Some(Fixture {
                    day,
                    matchday,
                    home,
                    away,
                    score,
                })
            })
            .collect()
    }

    /// The human club's next match not yet played.
    #[must_use]
    pub fn next_fixture(&self) -> Option<Fixture> {
        self.fixtures()
            .into_iter()
            .filter(|f| f.score.is_none() && f.day >= self.season.day())
            .find(|f| f.home == self.club || f.away == self.club)
    }

    /// The table so far.
    #[must_use]
    pub fn table(&self) -> Vec<TableLine> {
        let mut rows = ds_standings::table_of(self.season.events());
        // Every club has a line, also before the first match.
        let first = self.pack.series.first().map_or(0, |s| s.clubs);
        for c in 0..first {
            if !rows.iter().any(|(r, _)| *r == c) {
                rows.push((c, ds_standings::Row::default()));
            }
        }
        rows.into_iter()
            .map(|(club, row)| TableLine { club, row })
            .collect()
    }

    /// The human club's squad, as his coach sees it.
    #[must_use]
    pub fn squad(&self) -> Vec<SquadMan> {
        let events = self.season.events();
        let today = self.season.day().saturating_sub(1);
        let tired = ds_coach::tired_on(events, today);
        let banned = ds_discipline::banned_now(events);
        let cautions = ds_discipline::cautions_of(events);
        ds_roster::make_squad(&self.pack, self.seed, self.club)
            .into_iter()
            .map(|p| SquadMan {
                tired: tired.get(&p.id).copied().unwrap_or(0.0),
                banned: banned.contains(&p.id),
                cautions: cautions.get(&p.id).copied().unwrap_or(0),
                age: ds_roster::FIRST_SEASON.saturating_sub(p.born),
                id: p.id,
                name: p.name,
                role: p.role,
                shirt: p.shirt,
            })
            .collect()
    }

    /// The vice's proposal for the next match (`L1-COACH.md`): the line-up
    /// the club's coach would choose, the human player's starting point.
    #[must_use]
    pub fn proposal(&self) -> Option<ds_coach::Lineup> {
        let next = self.next_fixture()?;
        let opponent = if next.home == self.club {
            next.away
        } else {
            next.home
        };
        let events = self.season.events();
        let mut today = ds_coach::today_for(events, self.club, opponent, next.day);
        today.unavailable = ds_discipline::banned_now(events);
        today.tired = ds_coach::tired_on(events, self.season.day().saturating_sub(1));
        Some(ds_coach::choose(&self.pack, self.seed, self.club, &today))
    }

    /// The places of a line-up, by role (the eleven of the 4-4-2, then the
    /// bench).
    #[must_use]
    pub fn places(&self) -> Vec<&'static str> {
        ds_coach::places(ds_coach::bench_for(&self.pack, self.club))
    }

    /// Gives the human player's line-up for his next match.
    ///
    /// # Errors
    ///
    /// A [`CareerError`] if there is no next match or the coach refuses the
    /// input (it is checked again on the day: `LineupRefused`).
    pub fn give_lineup(&mut self, places: &[u32], block: u8, width: u8) -> Result<(), CareerError> {
        let next = self
            .next_fixture()
            .ok_or_else(|| CareerError("no match left".into()))?;
        let places = places
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(",");
        self.season.input(
            "coach",
            "lineup",
            &[
                ("day".into(), next.day.to_string()),
                ("places".into(), places),
                ("block".into(), block.to_string()),
                ("width".into(), width.to_string()),
            ],
        )?;
        Ok(())
    }

    /// The save, as text.
    #[must_use]
    pub fn save(&self) -> String {
        serde_json::to_string(&CareerSave {
            club: self.club,
            season: self.season.save(),
        })
        .unwrap_or_default()
    }

    /// A career back from its save, in the world of `pack`.
    ///
    /// # Errors
    ///
    /// A [`CareerError`] if the text is not a save of this version.
    pub fn load(pack: Pack, text: &str) -> Result<Self, CareerError> {
        let s: CareerSave =
            serde_json::from_str(text).map_err(|e| CareerError(format!("save: {e}")))?;
        let season = Season::load(&s.season, plugins(&pack, s.club))?;
        Ok(Self {
            seed: s.season.seed,
            club: s.club,
            pack,
            season,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Career;

    fn pack(clubs: usize) -> ds_world::Pack {
        ds_world::Pack::from_files(|name| {
            let text = ds_world::Pack::v1_file(name)?;
            match name {
                "nation.json" => {
                    Some(text.replace("\"clubs\": 20", &format!("\"clubs\": {clubs}")))
                }
                "clubs.json" => {
                    let all: Vec<serde_json::Value> = serde_json::from_str(&text).ok()?;
                    serde_json::to_string(&all[..clubs]).ok()
                }
                _ => Some(text),
            }
        })
        .expect("the cut pack validates")
    }

    /// A career: the human player's line-up is played as he gave it, the
    /// table moves, and the save gives back the same career, which goes on
    /// the same as one never saved.
    #[test]
    fn a_career_is_played_saved_and_resumed() {
        let mut c = Career::new(pack(4), 42, 1).expect("a career");
        assert_eq!(c.squad().len(), 23);
        assert_eq!(c.table().len(), 4);
        let mut l = c.proposal().expect("a proposal");
        l.places.swap(9, 10);
        c.give_lineup(&l.places, 0, 2).expect("taken");
        let played = c.advance_to_next_match().expect("plays");
        assert!(played.iter().any(|f| f.home == 1 || f.away == 1));
        let mine = c
            .events()
            .iter()
            .filter(|e| e.kind == ds_coach::LINEUP_ANNOUNCED)
            .filter_map(ds_coach::lineup_of)
            .find(|x| x.club == 1)
            .expect("my line-up");
        assert_eq!(
            (mine.places.clone(), mine.block, mine.width),
            (l.places.clone(), 0, 2)
        );
        assert!(c.table().iter().map(|t| t.row.played).sum::<u32>() > 0);
        let saved = c.save();
        let mut back = Career::load(pack(4), &saved).expect("loads");
        assert_eq!(back.day(), c.day());
        let a = c.advance_to_next_match().expect("plays");
        let b = back.advance_to_next_match().expect("plays");
        assert_eq!(a, b);
    }
}
