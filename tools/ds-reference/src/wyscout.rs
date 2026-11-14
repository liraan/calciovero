//! The public Wyscout soccer-logs, read into per-match counts.
//!
//! Source: Pappalardo, L., Cintia, P., Rossi, A. et al. (2019), *A public
//! data set of spatio-temporal match events in soccer competitions*,
//! Scientific Data 6, 236 (doi:10.1038/s41597-019-0247-7); data on
//! figshare (doi:10.6084/m9.figshare.c.4415000), licence CC BY 4.0.
//!
//! The event and tag codes are those of the dataset's own tables
//! (`eventid2name.csv`, `tags2name.csv`).

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::{MatchCounts, Metric};

/// One Wyscout event, only the fields the metrics read.
#[derive(Debug, Deserialize)]
pub struct Event {
    /// The event code (8 pass, 10 shot, ...).
    #[serde(rename = "eventId")]
    pub event_id: u32,
    /// The sub-event code (85 simple pass, 30 corner, ...); empty for a
    /// few events in the files.
    #[serde(rename = "subEventId", deserialize_with = "code_or_empty")]
    pub sub_event_id: Option<u32>,
    /// The tags (101 goal, 1801 accurate, ...).
    pub tags: Vec<Tag>,
    /// The match.
    #[serde(rename = "matchId")]
    pub match_id: u64,
    /// `1H`, `2H`, or extra time and penalties.
    #[serde(rename = "matchPeriod")]
    pub match_period: String,
    /// Seconds from the start of the period.
    #[serde(rename = "eventSec")]
    pub event_sec: f64,
}

/// One tag of an event.
#[derive(Debug, Deserialize)]
pub struct Tag {
    /// The tag code.
    pub id: u32,
}

/// One Wyscout match, only the fields the metrics read.
#[derive(Debug, Deserialize)]
pub struct Match {
    /// The match id, as in [`Event::match_id`].
    #[serde(rename = "wyId")]
    pub wy_id: u64,
    /// The two teams, by team id.
    #[serde(rename = "teamsData")]
    pub teams_data: BTreeMap<String, TeamData>,
}

/// One team in a match.
#[derive(Debug, Deserialize)]
pub struct TeamData {
    /// Its goals.
    pub score: u32,
}

fn code_or_empty<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<u32>, D::Error> {
    let v = serde_json::Value::deserialize(d)?;
    Ok(match v {
        serde_json::Value::Number(n) => n.as_u64().and_then(|x| u32::try_from(x).ok()),
        serde_json::Value::String(s) => s.parse().ok(),
        _ => None,
    })
}

// Event codes.
const DUEL: u32 = 1;
const FOUL: u32 = 2;
const OFFSIDE: u32 = 6;
const PASS: u32 = 8;
const SAVE_ATTEMPT: u32 = 9;
const SHOT: u32 = 10;
// Sub-event codes.
const AIR_DUEL: u32 = 10;
const GROUND_ATTACKING_DUEL: u32 = 11;
const GROUND_DEFENDING_DUEL: u32 = 12;
const GROUND_LOOSE_BALL_DUEL: u32 = 13;
const PROTEST: u32 = 24;
const CORNER: u32 = 30;
const FREE_KICK: u32 = 31;
const FREE_KICK_CROSS: u32 = 32;
const FREE_KICK_SHOT: u32 = 33;
const GOAL_KICK: u32 = 34;
const PENALTY: u32 = 35;
const THROW_IN: u32 = 36;
const CLEARANCE: u32 = 71;
const CROSS: u32 = 80;
// Tags.
const GOAL: u32 = 101;
const OWN_GOAL: u32 = 102;
const TAKE_ON_LEFT: u32 = 503;
const TAKE_ON_RIGHT: u32 = 504;
const WON: u32 = 703;
const INTERCEPTION: u32 = 1401;
const RED_CARD: u32 = 1701;
const YELLOW_CARD: u32 = 1702;
const SECOND_YELLOW: u32 = 1703;
const ACCURATE: u32 = 1801;

const HALF_SECONDS: f64 = 45.0 * 60.0;

/// The per-match counts of every match in `matches`, in their order.
#[must_use]
pub fn measure(events: &[Event], matches: &[Match]) -> Vec<MatchCounts> {
    let mut by_match: BTreeMap<u64, MatchCounts> = BTreeMap::new();
    let mut last: BTreeMap<(u64, bool), f64> = BTreeMap::new();
    for e in events {
        let c = by_match.entry(e.match_id).or_default();
        count(c, e);
        let first = e.match_period == "1H";
        if first || e.match_period == "2H" {
            let t = last.entry((e.match_id, first)).or_insert(0.0);
            *t = t.max(e.event_sec);
        }
    }
    matches
        .iter()
        .map(|m| {
            let mut c = by_match.remove(&m.wy_id).unwrap_or_default();
            let goals: u32 = m.teams_data.values().map(|t| t.score).sum();
            c.add(Metric::Goals, f64::from(goals));
            for (first, metric) in [
                (true, Metric::StoppageFirstHalf),
                (false, Metric::StoppageSecondHalf),
            ] {
                let end = last.get(&(m.wy_id, first)).copied().unwrap_or(HALF_SECONDS);
                c.add(metric, (end - HALF_SECONDS).max(0.0) / 60.0);
            }
            c
        })
        .collect()
}

fn count(c: &mut MatchCounts, e: &Event) {
    let has = |t: u32| e.tags.iter().any(|x| x.id == t);
    let sub = e.sub_event_id.unwrap_or(0);
    let half = 0.5;
    if e.event_id == SHOT || sub == FREE_KICK_SHOT || sub == PENALTY {
        c.add(Metric::Shots, 1.0);
        if has(ACCURATE) || has(GOAL) {
            c.add(Metric::ShotsOnTarget, 1.0);
        }
    }
    if sub == PENALTY {
        c.add(Metric::Penalties, 1.0);
    }
    if e.event_id == PASS {
        c.add(Metric::Passes, 1.0);
        if has(ACCURATE) {
            c.add(Metric::PassesAccurate, 1.0);
        }
        if sub == CROSS {
            c.add(Metric::Crosses, 1.0);
        }
    }
    if e.event_id == DUEL {
        match sub {
            AIR_DUEL => c.add(Metric::AerialDuels, half),
            GROUND_ATTACKING_DUEL | GROUND_DEFENDING_DUEL | GROUND_LOOSE_BALL_DUEL => {
                c.add(Metric::GroundDuels, half);
            }
            _ => {}
        }
        if sub == GROUND_ATTACKING_DUEL && (has(TAKE_ON_LEFT) || has(TAKE_ON_RIGHT)) && has(WON) {
            c.add(Metric::TakeOnsWon, 1.0);
        }
    }
    if e.event_id == FOUL && sub != PROTEST {
        c.add(Metric::Fouls, 1.0);
    }
    if has(YELLOW_CARD) {
        c.add(Metric::YellowCards, 1.0);
    }
    if has(RED_CARD) || has(SECOND_YELLOW) {
        c.add(Metric::RedCards, 1.0);
    }
    match sub {
        CORNER => c.add(Metric::Corners, 1.0),
        THROW_IN => c.add(Metric::ThrowIns, 1.0),
        GOAL_KICK => c.add(Metric::GoalKicks, 1.0),
        FREE_KICK | FREE_KICK_CROSS | FREE_KICK_SHOT => c.add(Metric::FreeKicks, 1.0),
        CLEARANCE => c.add(Metric::Clearances, 1.0),
        _ => {}
    }
    if e.event_id == OFFSIDE {
        c.add(Metric::Offsides, 1.0);
    }
    if has(INTERCEPTION) {
        c.add(Metric::Interceptions, 1.0);
    }
    if e.event_id == SAVE_ATTEMPT {
        c.add(Metric::Saves, 1.0);
    }
    if has(OWN_GOAL) {
        c.add(Metric::OwnGoals, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(json: &str) -> Event {
        serde_json::from_str(json).unwrap()
    }

    #[test]
    fn a_duel_is_counted_once_though_the_file_records_it_twice() {
        let a = ev(
            r#"{"eventId":1,"subEventId":10,"tags":[],"matchId":1,"matchPeriod":"1H","eventSec":10.0}"#,
        );
        let b = ev(
            r#"{"eventId":1,"subEventId":10,"tags":[],"matchId":1,"matchPeriod":"1H","eventSec":10.0}"#,
        );
        let m: Match =
            serde_json::from_str(r#"{"wyId":1,"teamsData":{"1":{"score":2},"2":{"score":1}}}"#)
                .unwrap();
        let c = measure(&[a, b], &[m]);
        assert!((c[0].get(Metric::AerialDuels) - 1.0).abs() < 1e-9);
        assert!((c[0].get(Metric::Goals) - 3.0).abs() < 1e-9);
    }

    #[test]
    fn a_penalty_is_a_shot_and_a_penalty() {
        let p = ev(
            r#"{"eventId":3,"subEventId":35,"tags":[{"id":101}],"matchId":1,"matchPeriod":"2H","eventSec":2760.0}"#,
        );
        let m: Match =
            serde_json::from_str(r#"{"wyId":1,"teamsData":{"1":{"score":1},"2":{"score":0}}}"#)
                .unwrap();
        let c = measure(&[p], &[m]);
        assert!((c[0].get(Metric::Shots) - 1.0).abs() < 1e-9);
        assert!((c[0].get(Metric::ShotsOnTarget) - 1.0).abs() < 1e-9);
        assert!((c[0].get(Metric::Penalties) - 1.0).abs() < 1e-9);
        assert!((c[0].get(Metric::StoppageSecondHalf) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn an_empty_sub_event_code_is_read_as_none() {
        let e = ev(
            r#"{"eventId":5,"subEventId":"","tags":[],"matchId":1,"matchPeriod":"1H","eventSec":1.0}"#,
        );
        assert_eq!(e.sub_event_id, None);
    }
}
