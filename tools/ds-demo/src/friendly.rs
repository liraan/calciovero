//! The friendly you can play (plan L0-CLOSE step 9).
//!
//! The manager (the home side) chooses his eleven from the squad and the bench, and his
//! intention (how high the team defends, how wide it plays); he watches the
//! match, saves it, reloads it, and reads *why* he conceded.
//!
//! Everything here reads Plane B only: the squads (registry data) and the
//! events of the match. The reasons are the facts that led to the goal, in
//! the order they happened, never a hidden number.

use std::fmt::Write as _;

use ds_core::ids::{ActorId, PlayerId};
use ds_model::{BlockHeight, Intention, Match, Player, Squad, Width};
use ds_stats::{EventType, MatchEvent};

/// The manager's choices before the kick-off.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Choices {
    /// His eleven, by roster letter, in the formation's order (goalkeeper,
    /// left back, two centre backs, right back, left, two central and right
    /// midfielders, two strikers). `None`: the default eleven.
    pub xi: Option<Vec<char>>,
    /// His intention.
    pub home: Intention,
    /// The opponents' intention.
    pub away: Intention,
    /// His substitutions: from this minute, this roster letter off, that
    /// one on (plan L0-CLOSE 10f).
    pub subs: Vec<(u16, char, char)>,
}

/// Parses an intention's block height.
///
/// # Errors
///
/// An unknown word.
pub fn parse_block(s: &str) -> Result<BlockHeight, String> {
    match s {
        "high" => Ok(BlockHeight::High),
        "mid" => Ok(BlockHeight::Mid),
        "low" => Ok(BlockHeight::Low),
        _ => Err(format!("block must be high, mid or low, not {s}")),
    }
}

/// Parses an intention's width.
///
/// # Errors
///
/// An unknown word.
pub fn parse_width(s: &str) -> Result<Width, String> {
    match s {
        "narrow" => Ok(Width::Narrow),
        "normal" => Ok(Width::Normal),
        "wide" => Ok(Width::Wide),
        _ => Err(format!("width must be narrow, normal or wide, not {s}")),
    }
}

/// Parses an eleven: eleven distinct roster letters, comma-separated.
///
/// # Errors
///
/// Not eleven, not letters, or a letter twice.
pub fn parse_xi(s: &str) -> Result<Vec<char>, String> {
    let letters: Vec<char> = s
        .split(',')
        .map(str::trim)
        .map(|l| {
            let mut c = l.chars();
            match (c.next(), c.next()) {
                (Some(ch), None) if ch.is_ascii_alphabetic() => Ok(ch.to_ascii_uppercase()),
                _ => Err(format!("--xi takes roster letters, not {l}")),
            }
        })
        .collect::<Result<_, _>>()?;
    if letters.len() != 11 {
        return Err(format!("--xi needs eleven letters, got {}", letters.len()));
    }
    let mut seen = letters.clone();
    seen.sort_unstable();
    seen.dedup();
    if seen.len() != 11 {
        return Err("--xi has a letter twice".to_string());
    }
    Ok(letters)
}

/// Parses a substitution, `MIN:OUT=IN` with roster letters (`60:J=P`).
///
/// # Errors
///
/// Not in that form.
pub fn parse_sub(s: &str) -> Result<(u16, char, char), String> {
    let err = || format!("--sub takes MIN:OUT=IN with roster letters, e.g. 60:J=P, not {s}");
    let (min, rest) = s.split_once(':').ok_or_else(err)?;
    let (out, inn) = rest.split_once('=').ok_or_else(err)?;
    let letter = |l: &str| {
        let mut c = l.trim().chars();
        match (c.next(), c.next()) {
            (Some(ch), None) if ch.is_ascii_alphabetic() => Ok(ch.to_ascii_uppercase()),
            _ => Err(err()),
        }
    };
    Ok((
        min.trim().parse().map_err(|_| err())?,
        letter(out)?,
        letter(inn)?,
    ))
}

/// The roster letter of a home player: `A`-`K` the eleven, `L`-`P` the
/// bench, by squad slot.
fn letter(id: PlayerId) -> char {
    let n = u8::try_from(id.slot() % 100).unwrap_or(25).min(25);
    char::from(b'A' + n)
}

/// The match with the manager's choices applied.
///
/// # Errors
///
/// A letter that is not in the squad.
pub fn apply(mut m: Match, c: &Choices) -> Result<Match, String> {
    if let Some(xi) = &c.xi {
        let all: Vec<Player> = m
            .home
            .players
            .iter()
            .chain(m.home.bench.iter())
            .cloned()
            .collect();
        let mut starters = Vec::with_capacity(11);
        for l in xi {
            let p = all
                .iter()
                .find(|p| letter(p.id) == *l)
                .ok_or_else(|| format!("no player {l} in the squad (see --roster)"))?;
            starters.push(p.clone());
        }
        let bench: Vec<Player> = all
            .into_iter()
            .filter(|p| !starters.iter().any(|s| s.id == p.id))
            .collect();
        m.home = Squad::try_new(m.home.id, starters, bench, m.home.formation.clone())
            .map_err(|e| e.to_string())?;
    }
    if !c.subs.is_empty() {
        let id_of = |l: char, squad: &[Player]| {
            squad
                .iter()
                .find(|p| letter(p.id) == l)
                .map(|p| p.id)
                .ok_or_else(|| format!("no player {l} for that substitution (see --roster)"))
        };
        let subs = c
            .subs
            .iter()
            .map(|(minute, out, inn)| {
                Ok(ds_model::Substitution {
                    minute: *minute,
                    outgoing: id_of(*out, &m.home.players)?,
                    incoming: id_of(*inn, &m.home.bench)?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        m.home = m.home.with_substitutions(subs).map_err(|e| e.to_string())?;
    }
    m.home = m.home.with_intention(c.home);
    m.away = m.away.with_intention(c.away);
    Ok(m)
}

/// A player's name and role, as the manager reads them.
fn who(m: &Match, id: PlayerId) -> String {
    m.home
        .players
        .iter()
        .chain(m.away.players.iter())
        .chain(m.home.bench.iter())
        .chain(m.away.bench.iter())
        .find(|p| p.id == id)
        .map_or_else(
            || format!("player {}", id.slot()),
            |p| format!("{} ({})", p.name, p.role.as_str().replace('_', " ")),
        )
}

const fn player(e: &MatchEvent) -> Option<PlayerId> {
    match e.actor {
        Some(ActorId::Player(id)) => Some(id),
        _ => None,
    }
}

fn is_home(m: &Match, id: PlayerId) -> bool {
    m.home.players.iter().any(|p| p.id == id)
}

/// The slot a field names, as a player of the match.
fn slot_player(m: &Match, e: &MatchEvent, key: &str) -> Option<PlayerId> {
    let slot = u32::try_from(e.fields.get(key)?.as_u64()?).ok()?;
    m.home
        .players
        .iter()
        .chain(m.away.players.iter())
        .map(|p| p.id)
        .find(|id| id.slot() == slot)
}

/// The broadcast clock: the second half starts at 45', stoppage reads
/// 45+2' and 90+3' (the event's own minute restarts at each half).
fn clock(e: &MatchEvent) -> String {
    let (base, m) = match e.half {
        ds_stats::Half::First => (45, u32::from(e.timestamp_minute)),
        ds_stats::Half::Second => (90, u32::from(e.timestamp_minute) + 45),
    };
    if m >= base {
        format!("{base}+{}'", m - base + 1)
    } else {
        format!("{}'", m + 1)
    }
}

/// Seconds since the kick-off, the second half after the first (an hour
/// apart, so a move never spans the break).
fn seconds(e: &MatchEvent) -> u32 {
    let half = match e.half {
        ds_stats::Half::First => 0,
        ds_stats::Half::Second => 3600,
    };
    half + u32::from(e.timestamp_minute) * 60 + u32::from(e.timestamp_second)
}

/// Why the manager (the home side) conceded each goal: who scored and how,
/// how and where the ball was lost, and what happened in between. Read
/// from the events only (Plane B).
#[must_use]
pub fn why_conceded(m: &Match, events: &[MatchEvent]) -> String {
    let mut out = String::from("Why you conceded\n");
    let mut any = false;
    for (g, goal) in events.iter().enumerate() {
        if goal.event_type != EventType::Goal {
            continue;
        }
        let Some(scorer) = player(goal).filter(|id| !is_home(m, *id)) else {
            continue;
        };
        any = true;
        let before = &events[..g];
        let _ = writeln!(
            out,
            "\n {} {} scored{}.",
            clock(goal),
            who(m, scorer),
            how_it_went_in(m, scorer, before)
        );
        if let Some(line) = rebound(m, goal, before) {
            let _ = writeln!(out, "   {line}");
        }
        if let Some(l) = where_it_began(m, before) {
            let _ = writeln!(out, "   {}", how_it_began(m, goal, before, l));
            let _ = writeln!(out, "   {}", in_between(m, &before[l + 1..]));
        }
    }
    if !any {
        out.push_str("\n You did not concede.\n");
    }
    out
}

/// The shot that scored: its kind and its distance.
fn how_it_went_in(m: &Match, scorer: PlayerId, before: &[MatchEvent]) -> String {
    let Some(s) = before
        .iter()
        .rev()
        .take(6)
        .find(|e| e.event_type == EventType::Shot)
    else {
        return String::new();
    };
    let x = s
        .fields
        .pointer("/from/x")
        .and_then(serde_json::Value::as_i64);
    let y = s
        .fields
        .pointer("/from/y")
        .and_then(serde_json::Value::as_i64);
    let goal_x = if is_home(m, scorer) { 10_500 } else { 0 };
    #[allow(clippy::cast_precision_loss)] // centimetres on a pitch
    let dist = x.zip(y).map_or(0.0, |(x, y)| {
        ((x - goal_x) as f64).hypot((y - 3400) as f64) / 100.0
    });
    let kind = match s
        .fields
        .get("shot_type")
        .and_then(serde_json::Value::as_str)
    {
        Some("header") => "a header",
        Some("lob") => "a lob",
        Some("power") => "a strong shot",
        Some("placed") => "a placed shot",
        _ => "a shot",
    };
    format!(": {kind} from {dist:.0} m")
}

/// A rebound: the keeper had saved a shot a few seconds before.
fn rebound(m: &Match, goal: &MatchEvent, before: &[MatchEvent]) -> Option<String> {
    let i = before
        .iter()
        .rposition(|e| e.event_type == EventType::Shot)?;
    let save = before[..i]
        .iter()
        .rev()
        .take(4)
        .find(|e| e.event_type == EventType::Save)?;
    (seconds(goal).saturating_sub(seconds(save)) < 6).then(|| {
        format!(
            "A rebound: {} had saved the first shot, and the ball came back to them.",
            player(save).map(|k| who(m, k)).unwrap_or_default()
        )
    })
}

/// The last time the manager's side had the ball before the goal.
fn where_it_began(m: &Match, before: &[MatchEvent]) -> Option<usize> {
    before.iter().rposition(|e| {
        player(e).is_some_and(|id| is_home(m, id))
            && matches!(
                e.event_type,
                EventType::Pass
                    | EventType::Dribble
                    | EventType::Shot
                    | EventType::Cross
                    | EventType::Clearance
                    | EventType::ThrowIn
                    | EventType::GoalKick
                    | EventType::FreeKick
                    | EventType::Corner
                    | EventType::Duel
            )
    })
}

/// How the ball was lost, and how long before the goal.
fn how_it_began(m: &Match, goal: &MatchEvent, before: &[MatchEvent], l: usize) -> String {
    let e = &before[l];
    let mine = player(e).map(|id| who(m, id)).unwrap_or_default();
    let next = before[l + 1..]
        .iter()
        .find(|x| player(x).is_some_and(|id| !is_home(m, id)));
    let them = next
        .and_then(player)
        .map(|id| who(m, id))
        .unwrap_or_default();
    let reason = match (e.event_type, next.map(|x| x.event_type)) {
        (EventType::Pass, Some(EventType::Interception)) => {
            format!("{mine}'s pass was cut out by {them}")
        }
        (_, Some(EventType::Tackle)) => format!("{them} won the ball from {mine}"),
        (EventType::Clearance, _) => format!("{mine} cleared it, and they won it back"),
        (EventType::Shot, _) => format!("after {mine}'s shot, they had the ball"),
        (_, Some(EventType::Duel)) | (EventType::Duel, _) => {
            format!("{mine} lost a duel to {them}")
        }
        _ => format!("{mine} lost the ball to {them}"),
    };
    let secs = seconds(goal).saturating_sub(seconds(e));
    format!("The move began {secs} s before: {reason}.")
}

/// What happened between the loss and the goal: their passes, crosses,
/// and a set piece, with the foul that gave it.
fn in_between(m: &Match, between: &[MatchEvent]) -> String {
    let theirs = |x: &&MatchEvent| player(x).is_some_and(|id| !is_home(m, id));
    let passes = between
        .iter()
        .filter(|x| x.event_type == EventType::Pass)
        .filter(theirs)
        .count();
    let crosses = between
        .iter()
        .filter(|x| x.event_type == EventType::Cross)
        .count();
    let mut parts = vec![format!(
        "{passes} pass{} of theirs",
        if passes == 1 { "" } else { "es" }
    )];
    if crosses > 0 {
        parts.push(format!(
            "{crosses} cross{}",
            if crosses == 1 { "" } else { "es" }
        ));
    }
    if let Some(sp) = between.iter().rev().filter(theirs).find(|x| {
        matches!(
            x.event_type,
            EventType::Corner | EventType::FreeKick | EventType::Penalty
        )
    }) {
        let kind = match sp.event_type {
            EventType::Corner => "a corner",
            EventType::Penalty => "a penalty",
            _ => "a free kick",
        };
        let foul = between
            .iter()
            .rev()
            .find(|x| x.event_type == EventType::Foul && seconds(x) <= seconds(sp))
            .and_then(|f| player(f).map(|id| (id, slot_player(m, f, "against"))));
        match foul {
            Some((by, Some(on))) if is_home(m, by) => {
                parts.push(format!("{kind} after {} fouled {}", who(m, by), who(m, on)));
            }
            _ => parts.push(kind.to_string()),
        }
    }
    format!("In between: {}.", parts.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_eleven_is_eleven_distinct_letters() {
        assert!(parse_xi("A,B,C,D,E,F,G,H,I,J,K").is_ok());
        assert!(parse_xi("A,B,C").is_err());
        assert!(parse_xi("A,A,C,D,E,F,G,H,I,J,K").is_err());
    }

    /// The manager's eleven goes on the pitch in the order he chose, and
    /// the one he left out sits on the bench.
    #[test]
    fn the_chosen_eleven_plays() {
        let m = ds_fixtures::match_fixture(42).expect("fixture");
        let c = Choices {
            xi: Some(parse_xi("A,B,C,D,E,F,G,H,I,J,P").expect("xi")),
            ..Choices::default()
        };
        let m = apply(m, &c).expect("apply");
        assert_eq!(letter(m.home.players[10].id), 'P');
        assert!(m.home.bench.iter().any(|p| letter(p.id) == 'K'));
    }

    /// The intention reaches the squad, and the match is still played.
    #[test]
    fn the_intention_is_the_managers() {
        let m = ds_fixtures::match_fixture(42).expect("fixture");
        let c = Choices {
            home: Intention {
                block: BlockHeight::High,
                width: Width::Wide,
            },
            ..Choices::default()
        };
        let m = apply(m, &c).expect("apply");
        assert_eq!(m.home.intention.block, BlockHeight::High);
        assert_eq!(m.away.intention, Intention::default());
    }

    /// Save, reload, replay (P1, plan L0-CLOSE step 9): a friendly with
    /// the manager's choices saved and replayed is the same match, to the
    /// last event; and his intention changes the match.
    #[test]
    fn a_friendly_saved_and_reloaded_is_the_same_match() {
        let seed = 42;
        let c = Choices {
            xi: Some(parse_xi("A,B,C,D,E,F,G,H,I,J,P").expect("xi")),
            home: Intention {
                block: BlockHeight::Low,
                width: Width::Narrow,
            },
            ..Choices::default()
        };
        let m = apply(ds_fixtures::match_fixture(seed).expect("fixture"), &c).expect("apply");
        let r = ds_match::simulate_from_seed(&m, seed, ds_match::MatchConfig::default());
        let save =
            ds_match::MatchSave::from_result(&r, &m, seed, ds_core::time::GameInstant::from_raw(0));
        let back = ds_match::MatchSave::from_bytes(&save.to_bytes().expect("bytes")).expect("load");
        assert_eq!(back.match_.home.intention, c.home);
        assert_eq!(ds_match::replay(&back).state_hash, r.state_hash);
        let plain = ds_fixtures::match_fixture(seed).expect("fixture");
        let r0 = ds_match::simulate_from_seed(&plain, seed, ds_match::MatchConfig::default());
        assert_ne!(r0.state_hash, r.state_hash, "the choices changed nothing");
    }

    /// A planned substitution is made (P1, plan L0-CLOSE 10f): from its
    /// minute, at the next stoppage, the player from the bench comes on and
    /// plays (he is on the pitch in the tick record), the one who came off is
    /// gone; saved and reloaded, it is the same match.
    #[test]
    fn the_substitute_comes_on() {
        let seed = 7;
        let choices = Choices {
            subs: vec![parse_sub("60:J=P").expect("sub")],
            ..Choices::default()
        };
        let game =
            apply(ds_fixtures::match_fixture(seed).expect("fixture"), &choices).expect("apply");
        let (r, records) =
            ds_match::simulate_from_seed_recorded(&game, seed, ds_match::MatchConfig::default());
        let sub = r
            .events
            .events()
            .iter()
            .find(|e| e.event_type == EventType::Substitution)
            .expect("a substitution");
        assert_eq!(sub.half, ds_stats::Half::Second);
        assert!(sub.timestamp_minute >= 15, "from the 60th minute");
        let on = game
            .home
            .bench
            .iter()
            .find(|p| letter(p.id) == 'P')
            .expect("P")
            .id;
        let off = game
            .home
            .players
            .iter()
            .find(|p| letter(p.id) == 'J')
            .expect("J")
            .id;
        let last = records.last().expect("records");
        assert!(last.agents.iter().any(|a| a.id == on), "P plays");
        assert!(last.agents.iter().all(|a| a.id != off), "J is off");
        let save = ds_match::MatchSave::from_result(
            &r,
            &game,
            seed,
            ds_core::time::GameInstant::from_raw(0),
        );
        assert_eq!(ds_match::replay(&save).state_hash, r.state_hash);
    }

    #[test]
    fn a_substitution_is_min_out_in() {
        assert_eq!(parse_sub("60:J=P"), Ok((60, 'J', 'P')));
        assert!(parse_sub("J=P").is_err());
    }

    /// Every goal conceded gets its reasons (P1): the scorer, and where the
    /// move began.
    #[test]
    fn every_goal_conceded_has_its_reasons() {
        for seed in 42..46 {
            let m = ds_fixtures::match_fixture(seed).expect("fixture");
            let r = ds_match::simulate_from_seed(&m, seed, ds_match::MatchConfig::default());
            let text = why_conceded(&m, r.events.events());
            let conceded = usize::from(r.score.away);
            assert_eq!(text.matches(" scored").count(), conceded, "{text}");
            if conceded > 0 {
                assert!(text.contains("The move began"), "{text}");
            }
        }
    }
}
