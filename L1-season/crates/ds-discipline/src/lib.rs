//! `ds-discipline` — the discipline plugin (`L1.md`, "Disciplina", step 7).
//!
//! The cards of every match (told by the matchday in `MatchPlayed`) are
//! counted over the season: a one-match ban at each threshold of yellow
//! cards, and after a red card (`L1-season/balance/discipline.json`, with
//! its sources). A ban is served when his club plays. Each morning the plugin
//! tells who may not play that day; the coach does not pick them. It may be
//! switched off (`Tier::Recommended`): then nobody is ever banned.

use std::collections::BTreeMap;

use ds_season_api::{
    MATCH_PLAYED, Moment, PluginCard, PluginError, SeasonEvent, SeasonPlugin, SeasonWorld, Tier,
};
use serde::Deserialize;

const DISCIPLINE_JSON: &str = include_str!("../../../balance/discipline.json");

/// The field of `MatchPlayed` the matchday writes the cards in, as
/// `identity:yellow|red`, comma-separated.
pub const CARDS_FIELD: &str = "cards";
/// The event of a caution or a sending-off counted.
pub const BOOKING: &str = "Booking";
/// The event of a ban.
pub const SUSPENSION: &str = "Suspension";
/// The event of a ban served.
pub const SUSPENSION_SERVED: &str = "SuspensionServed";
/// The event each morning: who may not play, comma-separated identities.
pub const UNAVAILABLE: &str = "Unavailable";

#[derive(Debug, Clone, Deserialize)]
struct DisciplineBalance {
    yellow_ban_at: Vec<u32>,
    #[serde(default)]
    ban_every_yellow_after_last: bool,
    red_ban_matches: u32,
}

fn balance() -> DisciplineBalance {
    serde_json::from_str(DISCIPLINE_JSON).unwrap_or_else(|_| DisciplineBalance {
        yellow_ban_at: vec![5, 10],
        ban_every_yellow_after_last: false,
        red_ban_matches: 1,
    })
}

/// Writes cards as `identity:colour`, comma-separated.
#[must_use]
pub fn write_cards(cards: &[(u32, bool)]) -> String {
    cards
        .iter()
        .map(|(id, red)| format!("{id}:{}", if *red { "red" } else { "yellow" }))
        .collect::<Vec<_>>()
        .join(",")
}

/// Reads cards back: (identity, red).
#[must_use]
pub fn read_cards(text: &str) -> Vec<(u32, bool)> {
    text.split(',')
        .filter_map(|c| {
            let (id, colour) = c.split_once(':')?;
            Some((id.parse().ok()?, colour == "red"))
        })
        .collect()
}

/// Who may not play today, from the morning's event.
#[must_use]
pub fn unavailable_of(events: &[SeasonEvent], day: u32) -> Vec<u32> {
    events
        .iter()
        .rev()
        .find(|e| e.kind == UNAVAILABLE && e.day == day)
        .and_then(|e| e.fields.first())
        .map(|(_, t)| t.split(',').filter_map(|x| x.parse().ok()).collect())
        .unwrap_or_default()
}

/// Who is banned now, from the events told (plane B): a man with a ban told
/// after the last one he served. The screens read it before the morning
/// says it.
#[must_use]
pub fn banned_now(events: &[SeasonEvent]) -> Vec<u32> {
    let mut open: BTreeMap<u32, bool> = BTreeMap::new();
    for e in events {
        let Some(id) = field(e, "player") else {
            continue;
        };
        if e.kind == SUSPENSION {
            open.insert(id, true);
        } else if e.kind == SUSPENSION_SERVED {
            open.insert(id, false);
        }
    }
    open.into_iter()
        .filter(|(_, b)| *b)
        .map(|(id, _)| id)
        .collect()
}

/// The cautions of every man this season that count towards a ban (not
/// those of a second caution), from the events told.
#[must_use]
pub fn cautions_of(events: &[SeasonEvent]) -> BTreeMap<u32, u32> {
    let booked: Vec<(u32, u32, bool)> = events
        .iter()
        .filter(|e| e.kind == BOOKING)
        .filter_map(|e| {
            let red = e.fields.iter().any(|(k, v)| k == "red" && v == "true");
            Some((e.day, field(e, "player")?, red))
        })
        .collect();
    let mut n = BTreeMap::new();
    for &(day, id, red) in &booked {
        if !red && !booked.iter().any(|&(d, i, r)| d == day && i == id && r) {
            *n.entry(id).or_insert(0) += 1;
        }
    }
    n
}

/// The discipline plugin.
#[derive(Debug, Default)]
pub struct Discipline {
    yellows: BTreeMap<u32, u32>,
    banned: BTreeMap<u32, u32>,
}

fn field(e: &SeasonEvent, key: &str) -> Option<u32> {
    e.fields
        .iter()
        .find(|(k, _)| k == key)
        .and_then(|(_, v)| v.parse().ok())
}

impl SeasonPlugin for Discipline {
    fn card(&self) -> PluginCard {
        PluginCard {
            name: "discipline",
            version: "1",
            steps: &[Moment::DayStarted, Moment::AfterMatch],
            depends: &[],
            reads: &["matchday.result"],
            optional_reads: &[],
            writes: &["discipline.cautions", "discipline.bans"],
            tier: Tier::Recommended,
            cost_us: 0,
            narrative: &["booking", "suspension", "suspension_served"],
        }
    }

    fn run(&mut self, moment: Moment, w: &mut SeasonWorld<'_>) -> Result<(), PluginError> {
        match moment {
            Moment::DayStarted => {
                if !self.banned.is_empty() {
                    let ids = self
                        .banned
                        .keys()
                        .map(u32::to_string)
                        .collect::<Vec<_>>()
                        .join(",");
                    w.emit(UNAVAILABLE, &[("players", ids)]);
                }
            }
            Moment::AfterMatch => {
                let m = w
                    .current_match()
                    .ok_or_else(|| PluginError("AfterMatch with no match".into()))?
                    .clone();
                if m.kind != MATCH_PLAYED {
                    return Ok(());
                }
                let clubs = [field(&m, "home"), field(&m, "away")];
                // A ban is served when his club plays (the players' club is
                // their identity / 100, ds-roster).
                let served: Vec<u32> = self
                    .banned
                    .keys()
                    .copied()
                    .filter(|id| clubs.contains(&Some(id / 100)))
                    .collect();
                for id in served {
                    if let Some(left) = self.banned.get_mut(&id) {
                        *left -= 1;
                        if *left == 0 {
                            self.banned.remove(&id);
                            w.emit(SUSPENSION_SERVED, &[("player", id.to_string())]);
                        }
                    }
                }
                let b = balance();
                let cards = m
                    .fields
                    .iter()
                    .find(|(k, _)| k == CARDS_FIELD)
                    .map(|(_, t)| read_cards(t))
                    .unwrap_or_default();
                // A yellow then a red to the same man in the same match is a second
                // caution: its cautions do not count (discipline.json,
                // "_double_yellow").
                let sent_off: Vec<u32> = cards
                    .iter()
                    .filter(|(_, red)| *red)
                    .map(|(id, _)| *id)
                    .collect();
                for (id, red) in cards {
                    w.emit(
                        BOOKING,
                        &[("player", id.to_string()), ("red", red.to_string())],
                    );
                    let ban = if red {
                        b.red_ban_matches
                    } else if sent_off.contains(&id) {
                        0
                    } else {
                        let n = self.yellows.entry(id).or_default();
                        *n += 1;
                        let past_last = b.ban_every_yellow_after_last
                            && b.yellow_ban_at.last().is_some_and(|l| *n > *l);
                        u32::from(b.yellow_ban_at.contains(n) || past_last)
                    };
                    if ban > 0 {
                        *self.banned.entry(id).or_default() += ban;
                        w.emit(
                            SUSPENSION,
                            &[("player", id.to_string()), ("matches", ban.to_string())],
                        );
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn state(&self) -> String {
        serde_json::to_string(&(&self.yellows, &self.banned)).unwrap_or_default()
    }

    fn restore(&mut self, state: &str) -> Result<(), PluginError> {
        if !state.is_empty() {
            (self.yellows, self.banned) = serde_json::from_str(state)
                .map_err(|e| PluginError(format!("discipline state: {e}")))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CARDS_FIELD, Discipline, SUSPENSION, SUSPENSION_SERVED, balance, read_cards, write_cards,
    };
    use ds_season_api::{MATCH_PLAYED, Moment, SeasonEvent, SeasonPlugin, SeasonWorld};

    /// A match of club 3 at home to club 4 on `day`, with `cards`.
    fn played(day: u32, cards: &[(u32, bool)]) -> SeasonEvent {
        SeasonEvent {
            day,
            moment: Moment::BeforeMatches,
            plugin: "matchday",
            kind: MATCH_PLAYED.into(),
            fields: vec![
                ("home".into(), "3".into()),
                ("away".into(), "4".into()),
                (CARDS_FIELD.into(), write_cards(cards)),
            ],
        }
    }

    fn after(d: &mut Discipline, m: &SeasonEvent) -> Vec<SeasonEvent> {
        let mut out = Vec::new();
        let mut w = SeasonWorld::new(
            (m.day, Moment::AfterMatch, "discipline", 42),
            &[],
            Some(m),
            &mut out,
        );
        d.run(Moment::AfterMatch, &mut w).expect("runs");
        out
    }

    /// CGS art. 9 c. 5: the fifth caution bans; the cautions of a second
    /// caution do not count; the ban is served at his club's next match.
    #[test]
    fn the_fifth_caution_bans_and_a_double_yellow_does_not_count() {
        let mut d = Discipline::default();
        let mut bans = 0;
        for day in 0..4 {
            bans += after(&mut d, &played(day, &[(305, false)]))
                .iter()
                .filter(|e| e.kind == SUSPENSION)
                .count();
        }
        assert_eq!(bans, 0, "four cautions, no ban");
        // A double yellow: a yellow and a red in the same match. One match
        // for the red, the caution not counted.
        let out = after(&mut d, &played(4, &[(305, false), (305, true)]));
        assert_eq!(out.iter().filter(|e| e.kind == SUSPENSION).count(), 1);
        let out = after(&mut d, &played(5, &[]));
        assert!(out.iter().any(|e| e.kind == SUSPENSION_SERVED));
        // Still four counted: the fifth bans.
        let out = after(&mut d, &played(6, &[(305, false)]));
        assert_eq!(out.iter().filter(|e| e.kind == SUSPENSION).count(), 1);
    }

    #[test]
    fn the_thresholds_come_from_the_data_and_the_cards_round_trip() {
        let b = balance();
        // CGS art. 9 c. 5: 5, 10, 14, 17, 19, then every caution.
        assert_eq!(b.yellow_ban_at, vec![5, 10, 14, 17, 19]);
        assert!(b.ban_every_yellow_after_last);
        assert_eq!(b.red_ban_matches, 1);
        let t = write_cards(&[(305, false), (1107, true)]);
        assert_eq!(read_cards(&t), vec![(305, false), (1107, true)]);
    }
}
