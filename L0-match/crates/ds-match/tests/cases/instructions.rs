//! The coach speaks during the match (ADR-059, step 1): an instruction is an
//! input with its tick, recorded with the match and played back by the
//! replay; it takes effect from the next tick, for the whole team.

use crate::ds_match::{simulate_from_seed, simulate_from_seed_recorded};
use ds_core::time::GameInstant;
use ds_fixtures::match_fixture;
use ds_match::{MatchConfig, MatchSave, replay};
use ds_model::{BlockHeight, Instruction, Intention, Match, Order, Width};
use ds_stats::EventType;

/// The tick of the instruction: the 15th minute (10 ticks a second).
const AT: u64 = 9_000;

fn instructed(seed: u64, at: u64) -> Match {
    let mut m = match_fixture(seed).expect("fixture");
    m.home = m.home.clone().with_instructions(vec![Instruction {
        tick: at,
        channel: None,
        order: Order::Intention(Intention {
            block: BlockHeight::High,
            width: Width::Wide,
        }),
    }]);
    m
}

/// P1 (ADR-059 §2): the match is the same up to the tick the coach speaks,
/// and his team plays differently from the next one; the fact is told.
#[test]
fn an_instruction_takes_effect_from_the_next_tick() {
    let plain = match_fixture(42).expect("fixture");
    let (_, before) = simulate_from_seed_recorded(&plain, 42, MatchConfig::default());
    let m = instructed(42, AT);
    let (result, after) = simulate_from_seed_recorded(&m, 42, MatchConfig::default());
    let upto = usize::try_from(AT).expect("tick fits");
    assert!(before.len() > upto + 1 && after.len() > upto + 1);
    assert_eq!(
        before[..=upto],
        after[..=upto],
        "nothing changes before he speaks"
    );
    assert!(
        before[upto + 1..]
            .iter()
            .zip(&after[upto + 1..])
            .any(|(a, b)| a.agents != b.agents),
        "the team plays differently after the instruction"
    );
    let told: Vec<_> = result
        .events
        .events()
        .iter()
        .filter(|e| e.event_type == EventType::Instruction)
        .collect();
    assert_eq!(told.len(), 1, "one instruction, told once");
    assert_eq!(told[0].fields["block"], "high");
}

/// P1 (ADR-059 §1): the same seed, the same teams and the same instructions
/// at the same ticks give the same match; the same instruction a minute
/// later gives another one.
#[test]
fn the_same_instructions_give_the_same_match() {
    let a = simulate_from_seed(&instructed(43, AT), 43, MatchConfig::default());
    // The engine again, past the suite's cache (2026-11-23): through the
    // cache the second run was the first one's copy.
    let b = ::ds_match::simulate_from_seed(&instructed(43, AT), 43, MatchConfig::default());
    assert_eq!(a.state_hash, b.state_hash);
    let later = simulate_from_seed(&instructed(43, AT + 600), 43, MatchConfig::default());
    assert_ne!(a.state_hash, later.state_hash);
}

/// P1 (ADR-059 §1): the instructions are saved with the match, and the
/// replay plays them back.
#[test]
fn a_saved_match_replays_its_instructions() {
    let m = instructed(44, AT);
    let result = simulate_from_seed(&m, 44, MatchConfig::default());
    let save = MatchSave::from_result(&result, &m, 44, GameInstant::from_raw(0));
    let loaded = MatchSave::from_bytes(&save.to_bytes().expect("bytes")).expect("load");
    assert_eq!(loaded.match_.home.instructions, m.home.instructions);
    assert_eq!(replay(&loaded).state_hash, result.state_hash);
}

/// P1 (ADR-059 §3, step 2): the team reads the coach's shape with the ball.
/// The same match, the coach asking at the first tick to play wide or
/// narrow: his outfield players spread wider across the pitch when his side
/// has the ball.
#[test]
#[allow(clippy::cast_precision_loss)] // ten players
fn the_team_plays_wider_when_the_coach_asks() {
    let spread = |width: Width| {
        let mut m = match_fixture(45).expect("fixture");
        m.home = m.home.clone().with_instructions(vec![Instruction {
            tick: 0,
            channel: None,
            order: Order::Intention(Intention {
                block: BlockHeight::Mid,
                width,
            }),
        }]);
        let home: Vec<_> = m.home.players.iter().skip(1).map(|p| p.id).collect();
        let (_, records) = simulate_from_seed_recorded(&m, 45, MatchConfig::default());
        let (mut sum, mut n) = (0.0_f64, 0.0_f64);
        for r in records
            .iter()
            .filter(|r| r.carrier.is_some_and(|c| home.contains(&c)))
        {
            let ys: Vec<f64> = r
                .agents
                .iter()
                .filter(|a| home.contains(&a.id))
                .map(|a| f64::from(a.position.1))
                .collect();
            let mean = ys.iter().sum::<f64>() / ys.len().max(1) as f64;
            sum += ys.iter().map(|y| (y - mean).abs()).sum::<f64>() / ys.len().max(1) as f64;
            n += 1.0;
        }
        assert!(n > 100.0, "the home side has the ball");
        sum / n
    };
    let (wide, narrow) = (spread(Width::Wide), spread(Width::Narrow));
    assert!(wide > narrow, "wide {wide:.0} cm, narrow {narrow:.0} cm");
}

/// The home side with these substitutions ordered at `at` (decision T).
fn ordered(seed: u64, at: u64, pairs: &[(usize, usize)]) -> Match {
    let mut m = match_fixture(seed).expect("fixture");
    let orders = pairs
        .iter()
        .map(|&(off, on)| Instruction {
            tick: at,
            channel: None,
            order: Order::Substitute {
                outgoing: m.home.players[off].id,
                incoming: m.home.bench[on].id,
            },
        })
        .collect();
    m.home = m.home.clone().with_instructions(orders);
    m
}

/// P1 (decision T): a substitution ordered during the match is an input
/// with its tick. The match is the same up to that tick; the order is told
/// at its minute; the man comes on at the first stoppage after it, for the
/// man named, and the replay plays it back.
#[test]
fn a_substitution_ordered_live_is_made_at_the_next_stoppage() {
    let plain = match_fixture(46).expect("fixture");
    let (_, before) = simulate_from_seed_recorded(&plain, 46, MatchConfig::default());
    let m = ordered(46, AT, &[(9, 0)]);
    let (off, on) = (m.home.players[9].id, m.home.bench[0].id);
    let (result, after) = simulate_from_seed_recorded(&m, 46, MatchConfig::default());
    let upto = usize::try_from(AT).expect("tick fits");
    assert_eq!(
        before[..=upto],
        after[..=upto],
        "nothing changes before he orders it"
    );
    let events = result.events.events();
    let told = events
        .iter()
        .find(|e| e.event_type == EventType::Instruction)
        .expect("the order is told");
    assert_eq!(told.fields["off"], off.slot());
    let made = after
        .iter()
        .position(|r| r.agents.iter().any(|a| a.id == on))
        .expect("he comes on");
    assert!(made > upto, "not before the order");
    assert!(
        after[made].agents.iter().all(|a| a.id != off),
        "the man named goes off"
    );
    assert!(
        events
            .iter()
            .any(|e| e.event_type == EventType::Substitution),
        "the substitution is told"
    );
    let save = MatchSave::from_result(&result, &m, 46, GameInstant::from_raw(0));
    assert_eq!(replay(&save).state_hash, result.state_hash);
}

/// P1 (Law 3): the orders beyond the substitutions allowed are not made.
#[test]
fn no_more_substitutions_than_the_laws_allow() {
    let mut m = ordered(47, AT, &[(1, 0), (2, 1), (3, 2), (4, 3), (5, 4)]);
    m.rules.subs_allowed = 3;
    let allowed = usize::from(m.rules.subs_allowed);
    let (result, _) = simulate_from_seed_recorded(&m, 47, MatchConfig::default());
    let bench: Vec<_> = m.home.bench.iter().map(|p| p.id.slot()).collect();
    // Counted from the substitutions told, not from who is on at the end:
    // a man named in an order may be sent off first, and a substitute who
    // came on may be sent off after (SIM 192, seed 47: three home reds).
    let made = result
        .events
        .events()
        .iter()
        .filter(|e| e.event_type == EventType::Substitution)
        .filter(|e| {
            e.fields["in"]
                .as_u64()
                .is_some_and(|s| bench.iter().any(|b| u64::from(*b) == s))
        })
        .count();
    assert!(made >= 1, "the orders are made");
    assert!(made <= allowed, "{made} came on, {allowed} allowed");
}

/// P1 (ADR-059 step 3): a quick instruction and the transition choices are
/// inputs with their tick: the match is the same up to it, then the team
/// plays differently; each is told at its minute, and the replay plays
/// them back.
#[test]
fn quick_instructions_take_effect_from_their_tick() {
    let plain = match_fixture(48).expect("fixture");
    let (_, before) = simulate_from_seed_recorded(&plain, 48, MatchConfig::default());
    let mut m = match_fixture(48).expect("fixture");
    m.home = m.home.clone().with_instructions(vec![
        Instruction {
            tick: AT,
            channel: None,
            order: Order::Quick {
                quick: ds_model::Quick::MorePressing,
                on: true,
            },
        },
        Instruction {
            tick: AT,
            channel: None,
            order: Order::Transitions {
                lost: ds_model::OnLoss::CounterPress,
                won: ds_model::OnWin::Counter,
            },
        },
    ]);
    let (result, after) = simulate_from_seed_recorded(&m, 48, MatchConfig::default());
    let upto = usize::try_from(AT).expect("tick fits");
    assert_eq!(
        before[..=upto],
        after[..=upto],
        "nothing changes before he speaks"
    );
    assert!(
        before[upto + 1..]
            .iter()
            .zip(&after[upto + 1..])
            .any(|(a, b)| a.agents != b.agents),
        "the team plays differently after it"
    );
    let told = result
        .events
        .events()
        .iter()
        .filter(|e| e.event_type == EventType::Instruction)
        .count();
    assert_eq!(told, 2, "both told");
    let save = MatchSave::from_result(&result, &m, 48, GameInstant::from_raw(0));
    assert_eq!(replay(&save).state_hash, result.state_hash);
}

/// P1 (decisions AL and AG): the board changes during the match as an
/// order with its tick. The wide men swapped at the 15th minute: the match
/// is the same up to then; from then the man who stood on one side stands
/// on the other.
#[test]
#[allow(clippy::cast_precision_loss)] // ticks of a match
fn the_board_changes_from_its_tick() {
    // The first seed from 49 where the man stays on the pitch (from SIM
    // 149 a knock takes him off in seed 49 before the order). The wave stops
    // at that seed; the other cores are spent in parallel, not serially.
    let (seed, (_id, before)) = ds_fixtures::first_seed(
        49..60,
        |seed| {
            let plain = match_fixture(seed).expect("fixture");
            let (_, before) = simulate_from_seed_recorded(&plain, seed, MatchConfig::default());
            (plain.home.players[5].id, before)
        },
        |(id, before)| before.iter().all(|t| t.agents.iter().any(|a| a.id == *id)),
    )
    .expect("a seed where he plays the whole match");
    let mut m = match_fixture(seed).expect("fixture");
    let swapped = m.home.board.swapped(5, 8);
    m.home = m.home.clone().with_instructions(vec![Instruction {
        tick: AT,
        channel: None,
        order: Order::Board(swapped),
    }]);
    let left = m.home.players[5].id;
    let (_, after) = simulate_from_seed_recorded(&m, seed, MatchConfig::default());
    let upto = usize::try_from(AT).expect("tick fits");
    assert_eq!(
        before[..=upto],
        after[..=upto],
        "the same match up to the order"
    );
    // His mean side in the ten minutes after the order, against the
    // ten minutes before it.
    let mean_y = |r: &[ds_match::TickRecord]| {
        let ys: Vec<f64> = r
            .iter()
            .filter_map(|t| t.agents.iter().find(|a| a.id == left))
            .map(|a| f64::from(a.position.1))
            .collect();
        ys.iter().sum::<f64>() / ys.len().max(1) as f64
    };
    let (was, now) = (
        mean_y(&after[upto - 6_000..upto]),
        mean_y(&after[upto + 600..upto + 6_600]),
    );
    assert!(now > was + 1_500.0, "he moved across: {was:.0} -> {now:.0}");
}

/// P1 (ADR-059 step 4, ADR-070): an individual instruction is a
/// responsibility on the board, changed during the match like the rest of
/// it. The same matches twice, in one the striker asked from the 15th
/// minute to attack the depth: from then, on average over six matches,
/// when a teammate has the ball he waits nearer the last defender's line
/// than without the ask, by at least a metre; before it, the same match.
/// (Until SIM 148 the twin measured how far ahead of the ball he stood,
/// which moves with where the ball goes too: at 149 the ask still brought
/// him 2.4 m nearer the line, but only 1.2 m farther ahead of the ball.)
#[test]
#[allow(clippy::cast_precision_loss)] // ticks of a match
fn a_striker_asked_to_run_waits_on_the_last_line() {
    let upto = usize::try_from(AT).expect("tick fits");
    let (mut asked, mut free) = (0.0, 0.0);
    for (a, f) in ds_fixtures::each_seed(50..56, |seed| {
        let plain = match_fixture(seed).expect("fixture");
        let striker = plain.home.players[9].id;
        let home: Vec<_> = plain.home.players.iter().map(|p| p.id).collect();
        let mut m = match_fixture(seed).expect("fixture");
        let mut board = m.home.board;
        board.with_ball_tasks[9] = ds_model::Task::Run;
        m.home = m.home.clone().with_instructions(vec![Instruction {
            tick: AT,
            channel: None,
            order: Order::Board(board),
        }]);
        let (_, before) = simulate_from_seed_recorded(&plain, seed, MatchConfig::default());
        let (_, after) = simulate_from_seed_recorded(&m, seed, MatchConfig::default());
        assert_eq!(
            before[..=upto],
            after[..=upto],
            "the same match up to the ask"
        );
        // How far he stands from the second-last opponent's line (Law 11).
        let from_line = |r: &[ds_match::TickRecord]| {
            let ds: Vec<f64> = r
                .iter()
                .filter(|t| t.carrier.is_some_and(|c| home.contains(&c) && c != striker))
                .filter_map(|t| {
                    let me = t.agents.iter().find(|a| a.id == striker)?.position.0;
                    let mut xs: Vec<i32> = t
                        .agents
                        .iter()
                        .filter(|a| !home.contains(&a.id))
                        .map(|a| a.position.0)
                        .collect();
                    xs.sort_unstable_by(|a, b| b.cmp(a));
                    Some(f64::from((xs.get(1)? - me).abs()))
                })
                .collect();
            ds.iter().sum::<f64>() / ds.len().max(1) as f64
        };
        let window = upto + 1..upto + 18_000;
        (
            from_line(&after[window.clone()]),
            from_line(&before[window]),
        )
    }) {
        asked += a;
        free += f;
    }
    // The sums are of six matches: a metre each.
    assert!(
        asked + 600.0 < free,
        "asked {:.0} cm from the line, free {:.0} cm",
        asked / 6.0,
        free / 6.0
    );
}
