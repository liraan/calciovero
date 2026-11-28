//! The Laws as assertions over whole matches (code audit, third phase,
//! point 1): at every tick of the record and at every event of the chain,
//! what the Laws of the Game say must hold. Not a metric: a match that
//! breaks one is a bug, whatever its numbers.
//!
//! What is read is only what an observer sees: the tick record (positions,
//! ball, carrier, score) and the event chain. Each check names its Law.
//! One seed runs in the gate; many seeds run with \`--ignored\`.

use std::collections::{BTreeMap, BTreeSet};

use ds_core::ids::{ActorId, PlayerId};
use ds_fixtures::match_fixture;
use ds_match::{MatchConfig, MatchResult, TickRecord};
use ds_model::Match;
use ds_stats::{EventType, Half, MatchEvent};

/// A side of the match, home or away.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Side {
    Home,
    Away,
}

impl Side {
    const fn other(self) -> Self {
        match self {
            Self::Home => Self::Away,
            Self::Away => Self::Home,
        }
    }
    const fn index(self) -> usize {
        match self {
            Self::Home => 0,
            Self::Away => 1,
        }
    }
}

/// Who is who: the squads of the match, by id and by slot.
struct Squads {
    side: BTreeMap<PlayerId, Side>,
    by_slot: BTreeMap<u64, PlayerId>,
    starters: BTreeSet<PlayerId>,
    keepers: [PlayerId; 2],
}

impl Squads {
    fn of(m: &Match) -> Self {
        let mut side = BTreeMap::new();
        let mut by_slot = BTreeMap::new();
        let mut starters = BTreeSet::new();
        for (s, squad) in [(Side::Home, &m.home), (Side::Away, &m.away)] {
            for p in &squad.players {
                side.insert(p.id, s);
                by_slot.insert(u64::from(p.id.slot()), p.id);
                starters.insert(p.id);
            }
            for p in &squad.bench {
                side.insert(p.id, s);
                by_slot.insert(u64::from(p.id.slot()), p.id);
            }
        }
        Self {
            side,
            by_slot,
            starters,
            keepers: [m.home.players[0].id, m.away.players[0].id],
        }
    }

    fn slot(&self, e: &MatchEvent, key: &str) -> Option<PlayerId> {
        e.fields
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .and_then(|s| self.by_slot.get(&s).copied())
    }
}

/// The player who did the event, if a player did it.
const fn player(e: &MatchEvent) -> Option<PlayerId> {
    match e.actor {
        Some(ActorId::Player(p)) => Some(p),
        _ => None,
    }
}

fn int(e: &MatchEvent, key: &str) -> Option<i32> {
    e.fields
        .get(key)
        .and_then(serde_json::Value::as_i64)
        .and_then(|v| i32::try_from(v).ok())
}

/// How far the ball flies in a tick, at most: the tick before a restart it
/// was within this of the line it crossed.
const REACH_CM: i32 = 600;

/// Every breach of the Laws in one match, each as one line.
#[allow(clippy::too_many_lines, clippy::many_single_char_names)]
fn breaches(m: &Match, result: &MatchResult, records: &[TickRecord]) -> Vec<String> {
    let mut out = Vec::new();
    let squads = Squads::of(m);
    let length = i32::from(m.pitch.length_m) * 100;
    let width = i32::from(m.pitch.width_m) * 100;
    let law1 = ds_calculator::laws::law1();
    #[allow(clippy::cast_possible_truncation)]
    let r = ds_core::geometry::BALL_RADIUS_CM as i32;
    let events = result.events.events();
    // The tick of each event, from the ranges of the record.
    let mut tick_of = vec![0usize; events.len()];
    for (k, rec) in records.iter().enumerate() {
        for i in rec.events.clone() {
            tick_of[i] = k;
        }
    }
    let side_of = |p: PlayerId| squads.side.get(&p).copied();

    // ---- the events, in order -------------------------------------------
    let mut sent_off: BTreeMap<PlayerId, usize> = BTreeMap::new();
    let mut cautioned: BTreeSet<PlayerId> = BTreeSet::new();
    let mut subbed_off: BTreeSet<PlayerId> = BTreeSet::new();
    let mut came_on: BTreeSet<PlayerId> = BTreeSet::new();
    let mut on_pitch: BTreeSet<PlayerId> = squads.starters.clone();
    let mut subs = [0usize; 2];
    let mut keeper: [Option<PlayerId>; 2] = [Some(squads.keepers[0]), Some(squads.keepers[1])];
    let mut score = [0u16; 2];
    let mut last_foul: Option<(usize, PlayerId)> = None;
    for (i, e) in events.iter().enumerate() {
        let k = tick_of[i];
        let at = format!("tick {k} ({:?}) {:?}", e.half, e.event_type);
        let actor = player(e);
        // No man sent off or substituted plays again (Laws 3, 12).
        if let Some(p) = actor {
            let excused = matches!(e.event_type, EventType::Card | EventType::Substitution);
            if !excused && e.half != Half::Shootout {
                if sent_off.get(&p).is_some_and(|&t| t < k) {
                    out.push(format!("{at}: {p:?} plays after his red card (Law 12)"));
                }
                if subbed_off.contains(&p) {
                    out.push(format!(
                        "{at}: {p:?} plays after he was substituted (Law 3)"
                    ));
                }
                if !on_pitch.contains(&p) && !sent_off.contains_key(&p) && !subbed_off.contains(&p)
                {
                    out.push(format!("{at}: {p:?} plays and is not on the pitch (Law 3)"));
                }
            }
        }
        match e.event_type {
            EventType::Card => {
                let Some(p) = actor else {
                    out.push(format!("{at}: a card to nobody"));
                    continue;
                };
                if sent_off.contains_key(&p) {
                    out.push(format!("{at}: {p:?} carded after his red card (Law 12)"));
                }
                match e.fields.get("color").and_then(serde_json::Value::as_str) {
                    Some("red") => {
                        // The card itself says whether it is a second
                        // caution (T22); it must agree with the bookings.
                        let second = e
                            .fields
                            .get("second_yellow")
                            .and_then(serde_json::Value::as_bool);
                        if second != Some(cautioned.contains(&p)) {
                            out.push(format!(
                                "{at}: {p:?} red says second_yellow {second:?}, bookings say {}",
                                cautioned.contains(&p)
                            ));
                        }
                        sent_off.insert(p, k);
                        on_pitch.remove(&p);
                        if let Some(s) = side_of(p) {
                            if keeper[s.index()] == Some(p) {
                                keeper[s.index()] = None;
                            }
                        }
                    }
                    Some("yellow") => {
                        if !cautioned.insert(p) {
                            out.push(format!(
                                "{at}: {p:?} has a second yellow and stays on (Law 12)"
                            ));
                        }
                    }
                    c => out.push(format!("{at}: a card of colour {c:?}")),
                }
            }
            EventType::Substitution => {
                let (Some(off), Some(on)) = (squads.slot(e, "out"), squads.slot(e, "in")) else {
                    out.push(format!("{at}: a substitution without its two men"));
                    continue;
                };
                if e.half == Half::Shootout {
                    out.push(format!("{at}: a substitution in the shoot-out (Law 10)"));
                }
                let (so, si) = (side_of(off), side_of(on));
                if so != si || so.is_none() {
                    out.push(format!("{at}: {off:?} off and {on:?} on are not one side"));
                    continue;
                }
                let s = so.unwrap_or(Side::Home);
                if !on_pitch.contains(&off) {
                    out.push(format!("{at}: {off:?} goes off and was not on (Law 3)"));
                }
                if on_pitch.contains(&on) || came_on.contains(&on) || squads.starters.contains(&on)
                {
                    out.push(format!(
                        "{at}: {on:?} comes on and was not on the bench (Law 3)"
                    ));
                }
                subs[s.index()] += 1;
                if subs[s.index()] > usize::from(m.rules.subs_allowed) {
                    out.push(format!(
                        "{at}: substitution {} of {:?}, {} allowed (Law 3)",
                        subs[s.index()],
                        s,
                        m.rules.subs_allowed
                    ));
                }
                on_pitch.remove(&off);
                subbed_off.insert(off);
                on_pitch.insert(on);
                came_on.insert(on);
                if keeper[s.index()] == Some(off) {
                    keeper[s.index()] = Some(on);
                }
            }
            EventType::Save if e.half != Half::Shootout => {
                if let Some(p) = actor {
                    if let Some(s) = side_of(p) {
                        match keeper[s.index()] {
                            Some(kp) if kp != p => out.push(format!(
                                "{at}: {p:?} saves, the keeper is {kp:?} (Law 3, Law 12 handling)"
                            )),
                            None => keeper[s.index()] = Some(p),
                            _ => {}
                        }
                    }
                }
            }
            EventType::Goal if e.half != Half::Shootout => {
                let (x, y, z) = (int(e, "x"), int(e, "y"), int(e, "z"));
                let (Some(x), Some(y), Some(z)) = (x, y, z) else {
                    out.push(format!("{at}: a goal without its point"));
                    continue;
                };
                // Law 10: the whole ball between the posts and under the bar
                // (`z` is its lowest point, its top two radii higher).
                if (y - width / 2).abs() + r >= law1.goal_half_width_cm {
                    out.push(format!(
                        "{at}: goal at y {y}, {} cm from the centre: the ball is not wholly between the posts (Law 10)",
                        (y - width / 2).abs()
                    ));
                }
                if z + 2 * r >= law1.goal_height_cm || z < 0 {
                    out.push(format!(
                        "{at}: goal at height {z}: the ball is not wholly under the bar (Law 10)"
                    ));
                }
                let scorer = if x >= length { Side::Home } else { Side::Away };
                if x != 0 && x != length {
                    out.push(format!("{at}: goal at x {x}, not on a goal line"));
                }
                score[scorer.index()] += 1;
                let after = &records[k].score;
                if [u16::from(after.home), u16::from(after.away)] != score {
                    out.push(format!(
                        "{at}: the goals told {score:?} are not the score {:?}",
                        (after.home, after.away)
                    ));
                }
            }
            EventType::Foul => {
                if let Some(p) = actor {
                    last_foul = Some((k, p));
                }
            }
            EventType::FreeKick | EventType::Penalty if e.half != Half::Shootout => {
                let taker = actor.and_then(side_of);
                let reason = e.fields.get("reason").and_then(serde_json::Value::as_str);
                match reason {
                    Some("offside") => {
                        let off = squads.slot(e, "offender");
                        if off.and_then(side_of) == taker {
                            out.push(format!(
                                "{at}: the offside free kick goes to the offender's side (Law 11)"
                            ));
                        }
                        if let (Some(p), Some(s)) = (off, off.and_then(side_of)) {
                            let played = e
                                .fields
                                .get("played_tick")
                                .and_then(serde_json::Value::as_u64)
                                .and_then(|t| usize::try_from(t).ok());
                            if let Some(t) = played {
                                let goal_x = if s == Side::Home { length } else { 0 };
                                let read = |rec: &TickRecord| {
                                    let man = rec.agents.iter().find(|a| a.id == p)?.position.0;
                                    let opponents: Vec<i32> = rec
                                        .agents
                                        .iter()
                                        .filter(|a| side_of(a.id) == Some(s.other()))
                                        .filter(|a| sent_off.get(&a.id).is_none_or(|&so| so > t))
                                        .map(|a| a.position.0)
                                        .collect();
                                    Some(ds_match::rules::offside_position(
                                        man,
                                        rec.ball.position.0,
                                        &opponents,
                                        goal_x,
                                        length,
                                    ))
                                };
                                let was = [t.checked_sub(1), Some(t)]
                                    .into_iter()
                                    .flatten()
                                    .filter_map(|j| records.get(j))
                                    .any(|rec| read(rec) == Some(true));
                                if !was {
                                    out.push(format!(
                                        "{at}: {p:?} called offside, not in an offside position at the pass (tick {t}) (Law 11)"
                                    ));
                                }
                            } else {
                                out.push(format!("{at}: an offside without the tick of the pass"));
                            }
                        }
                    }
                    Some("foul") => {
                        match last_foul {
                            Some((fk, p)) if fk == k => {
                                if side_of(p) == taker {
                                    out.push(format!(
                                        "{at}: the free kick goes to the offender's side (Law 12)"
                                    ));
                                }
                            }
                            _ => out.push(format!(
                                "{at}: a free kick for a foul with no foul in its tick"
                            )),
                        }
                        if e.event_type == EventType::Penalty {
                            let x = int(e, "x").unwrap_or(-1);
                            let attacks_right = taker == Some(Side::Home);
                            let want = if attacks_right { length - 1100 } else { 1100 };
                            if x != want {
                                out.push(format!(
                                    "{at}: penalty from x {x}, the mark is at {want} (Law 14)"
                                ));
                            }
                        }
                    }
                    _ => {}
                }
            }
            EventType::ThrowIn | EventType::Corner | EventType::GoalKick
                if e.half != Half::Shootout =>
            {
                let (Some(x), Some(y)) = (int(e, "x"), int(e, "y")) else {
                    out.push(format!("{at}: a restart without its spot"));
                    continue;
                };
                let taker = actor.and_then(side_of);
                match e.event_type {
                    EventType::ThrowIn => {
                        if y != 0 && y != width {
                            out.push(format!(
                                "{at}: throw-in from y {y}, not on a touchline (Law 15)"
                            ));
                        }
                    }
                    EventType::Corner => {
                        if !((x == 0 || x == length) && (y == 0 || y == width)) {
                            out.push(format!(
                                "{at}: corner from ({x}, {y}), not a corner (Law 17)"
                            ));
                        }
                        // The attackers of that goal line take it.
                        let attackers = if x == 0 { Side::Away } else { Side::Home };
                        if taker != Some(attackers) {
                            out.push(format!("{at}: corner at x {x} taken by {taker:?} (Law 17)"));
                        }
                    }
                    _ => {
                        let depth = law1.goal_area_depth_cm;
                        if !((x == depth || x == length - depth) && y == width / 2) {
                            out.push(format!("{at}: goal kick from ({x}, {y}) (Law 16)"));
                        }
                        let defenders = if x < length / 2 {
                            Side::Home
                        } else {
                            Side::Away
                        };
                        if taker != Some(defenders) {
                            out.push(format!(
                                "{at}: goal kick at x {x} taken by {taker:?} (Law 16)"
                            ));
                        }
                    }
                }
                // The ball went out over the line of this restart (Laws 9,
                // 15-17). It crosses and is placed in the same tick, so the
                // record never shows it out: the tick before, it was a
                // tick's flight from that line (at most `REACH_CM`).
                if let Some(prev) = k.checked_sub(1).map(|j| &records[j]) {
                    let (bx, by, _) = prev.ball.position;
                    let from_line = match e.event_type {
                        EventType::ThrowIn => (by - y).abs(),
                        _ => (bx - x).abs(),
                    };
                    let along = match e.event_type {
                        EventType::ThrowIn => (bx - x).abs(),
                        _ => 0,
                    };
                    if from_line > REACH_CM || along > REACH_CM {
                        out.push(format!(
                            "{at}: the ball was at {:?} the tick before, not by the line of this restart at ({x}, {y}) (Laws 9, 15-17)",
                            prev.ball.position
                        ));
                    }
                }
            }
            _ => {}
        }
    }
    let final_score = result.score;
    if [u16::from(final_score.home), u16::from(final_score.away)] != score {
        out.push(format!(
            "the goals told {score:?} are not the final score {:?}",
            (final_score.home, final_score.away)
        ));
    }

    // ---- every tick ------------------------------------------------------
    let red_at: BTreeMap<PlayerId, usize> = sent_off.clone();
    for (k, rec) in records.iter().enumerate() {
        if rec.half == Half::Shootout {
            continue;
        }
        let mut per_side = [0usize; 2];
        for a in &rec.agents {
            let Some(s) = side_of(a.id) else {
                out.push(format!("tick {k}: {:?} is in no squad", a.id));
                continue;
            };
            let off = red_at.get(&a.id).is_some_and(|&t| t < k);
            if off {
                let (x, y) = a.position;
                if (0..=length).contains(&x) && (0..=width).contains(&y) {
                    out.push(format!(
                        "tick {k}: {:?} sent off is on the pitch at {:?} (Law 12)",
                        a.id, a.position
                    ));
                }
                if rec.carrier == Some(a.id) {
                    out.push(format!(
                        "tick {k}: {:?} sent off has the ball (Law 12)",
                        a.id
                    ));
                }
            } else {
                per_side[s.index()] += 1;
            }
        }
        for s in [Side::Home, Side::Away] {
            if per_side[s.index()] > 11 {
                out.push(format!(
                    "tick {k}: {} men of {s:?} on the pitch (Law 3)",
                    per_side[s.index()]
                ));
            }
        }
        if let Some(c) = rec.carrier {
            if !rec.agents.iter().any(|a| a.id == c) {
                out.push(format!(
                    "tick {k}: the carrier {c:?} is not among the players"
                ));
            }
        }
    }
    out
}

fn check(seed: u64, recorded: &(MatchResult, Vec<TickRecord>), m: &Match) -> Vec<String> {
    breaches(m, &recorded.0, &recorded.1)
        .into_iter()
        .map(|b| format!("seed {seed}: {b}"))
        .collect()
}

/// The gate: one match, every Law at every tick.
#[test]
fn the_laws_hold_in_one_match() {
    let seed = 42;
    let m = match_fixture(seed).expect("fixture");
    let rec = crate::ds_match::simulate_from_seed_recorded(&m, seed, MatchConfig::default());
    let found = check(seed, &rec, &m);
    assert!(
        found.is_empty(),
        "{} breaches:\n{}",
        found.len(),
        found.join("\n")
    );
}

/// Many matches, run with \`--ignored\`: each through the engine, not the
/// suite's cache, so a hundred tick records are never held at once.
#[test]
#[ignore = "a hundred matches: run with --ignored"]
fn the_laws_hold_in_many_matches() {
    let from: u64 = std::env::var("LAWS_FROM")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(42);
    let to: u64 = std::env::var("LAWS_TO")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(142);
    let found: Vec<String> = ds_fixtures::each_seed(from..to, |seed| {
        let m = match_fixture(seed).expect("fixture");
        let rec = ::ds_match::simulate_from_seed_recorded(&m, seed, MatchConfig::default());
        check(seed, &rec, &m)
    })
    .into_iter()
    .flatten()
    .collect();
    assert!(
        found.is_empty(),
        "{} breaches:\n{}",
        found.len(),
        found.join("\n")
    );
}
