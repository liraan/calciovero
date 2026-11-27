//! The friendly you can play (plan L0-CLOSE step 9).
//!
//! The manager (the home side) chooses his eleven from the squad and the bench, and his
//! intention (how high the team defends, how wide it plays); he watches the
//! match, saves it, reloads it, and reads *why* he conceded.
//!
//! Everything here reads Plane B only: the squads (registry data) and the
//! events of the match. The reasons are the facts that led to the goal, in
//! the order they happened, never a hidden number.

use ds_core::ids::PlayerId;
use ds_model::{BlockHeight, Intention, Match, Player, Squad, Width};

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
    /// The captain he wants speaking for his side, by roster letter (GUI
    /// only: the view shows the approach, the engine takes no order from it).
    pub captain: Option<char>,
    /// His substitutions: from this minute, this roster letter off, that
    /// one on (plan L0-CLOSE 10f).
    pub subs: Vec<(u16, char, char)>,
    /// His instructions during the match (ADR-059): after this many minutes
    /// of play, this intention.
    pub instructions: Vec<(u16, Intention)>,
    /// The orders he gives while he watches (decision T): each an input
    /// with its tick, as the window took it.
    pub live: Vec<ds_model::Instruction>,
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

/// Parses an instruction during the match, `MIN:BLOCK,WIDTH` (`30:high,wide`).
///
/// # Errors
///
/// Not in that form, or a block or width that does not exist.
pub fn parse_instruct(s: &str) -> Result<(u16, Intention), String> {
    let err = || format!("--instruct takes MIN:BLOCK,WIDTH, e.g. 30:high,wide, not {s}");
    let (min, rest) = s.split_once(':').ok_or_else(err)?;
    let (block, width) = rest.split_once(',').ok_or_else(err)?;
    Ok((
        min.trim().parse().map_err(|_| err())?,
        Intention {
            block: parse_block(block.trim())?,
            width: parse_width(width.trim())?,
        },
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
    // The manager plays his side himself: its coach does not react on his
    // own; the opponents' coach reads the match (decision Y).
    m.home = m.home.with_coach_reacting(false);
    m.away = m.away.with_intention(c.away);
    if !c.instructions.is_empty() || !c.live.is_empty() {
        // After MIN minutes of play: ten ticks a second (the match's clock,
        // continuous across the halves; stoppage time is not counted).
        let instructions = c
            .instructions
            .iter()
            .map(|(minute, intention)| ds_model::Instruction {
                tick: u64::from(*minute) * 60 * u64::from(ds_match::context::TICKS_PER_SECOND),
                channel: None,
                order: ds_model::Order::Intention(*intention),
            })
            .chain(c.live.iter().copied())
            .collect();
        m.home = m.home.with_instructions(instructions);
    }
    Ok(m)
}

/// The facts that led to each goal conceded: the commentary's own, in
/// `ds_narrative::why` (code audit B6).
pub use ds_narrative::why::why_conceded;

#[cfg(test)]
mod tests {
    use super::*;
    use ds_stats::EventType;

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
        // The plain seed-42 match is already played for the board and
        // commentary tests; borrow it rather than play it again.
        let r0 = &crate::test_support::SEED_42.1;
        assert_ne!(r0.state_hash, r.state_hash, "the choices changed nothing");
    }

    /// A planned substitution is made (P1, plan L0-CLOSE 10f): from its
    /// minute, at the next stoppage, the player from the bench comes on and
    /// plays (he is on the pitch in the tick record), the one who came off is
    /// gone; saved and reloaded, it is the same match.
    #[test]
    fn the_substitute_comes_on() {
        let seed = 8;
        let choices = Choices {
            subs: vec![parse_sub("60:J=P").expect("sub")],
            ..Choices::default()
        };
        let game =
            apply(ds_fixtures::match_fixture(seed).expect("fixture"), &choices).expect("apply");
        let (r, records) =
            ds_match::simulate_from_seed_recorded(&game, seed, ds_match::MatchConfig::default());
        let p_slot = game
            .home
            .bench
            .iter()
            .find(|p| letter(p.id) == 'P')
            .map(|p| u64::from(p.id.slot()))
            .expect("P");
        // His change: a knock can bring P on earlier for another man, so
        // match both ends of the planned 60:J=P, not just the incoming slot.
        let j_slot = game
            .home
            .players
            .iter()
            .find(|p| letter(p.id) == 'J')
            .map(|p| u64::from(p.id.slot()))
            .expect("J");
        let sub = r
            .events
            .events()
            .iter()
            .find(|e| {
                e.event_type == EventType::Substitution
                    && e.fields.get("out").and_then(serde_json::Value::as_u64) == Some(j_slot)
                    && e.fields.get("in").and_then(serde_json::Value::as_u64) == Some(p_slot)
            })
            .expect("J for P at the 60th minute");
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
            // Seed 42's plain match is the one the whole board/commentary
            // suite looks at; read it instead of playing it a second time.
            if seed == 42 {
                let (m, r, _) = &*crate::test_support::SEED_42;
                let text = why_conceded(m, r.events.events());
                let conceded = usize::from(r.score.away);
                assert_eq!(text.matches(" scored").count(), conceded, "{text}");
                if conceded > 0 {
                    assert!(text.contains("The move began"), "{text}");
                    assert!(!text.contains("to ."), "{text}");
                }
                continue;
            }
            let m = ds_fixtures::match_fixture(seed).expect("fixture");
            let r = ds_match::simulate_from_seed(&m, seed, ds_match::MatchConfig::default());
            let text = why_conceded(&m, r.events.events());
            let conceded = usize::from(r.score.away);
            assert_eq!(text.matches(" scored").count(), conceded, "{text}");
            if conceded > 0 {
                assert!(text.contains("The move began"), "{text}");
                assert!(!text.contains("to ."), "{text}");
            }
        }
    }
}
