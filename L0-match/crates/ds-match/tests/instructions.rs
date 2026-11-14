//! The coach speaks during the match (ADR-059, step 1): an instruction is an
//! input with its tick, recorded with the match and played back by the
//! replay; it takes effect from the next tick, for the whole team.

use ds_core::time::GameInstant;
use ds_fixtures::match_fixture;
use ds_match::{MatchConfig, MatchSave, replay, simulate_from_seed, simulate_from_seed_recorded};
use ds_model::{BlockHeight, Instruction, Intention, Match, Order, Width};
use ds_stats::EventType;

/// The tick of the instruction: the 15th minute (10 ticks a second).
const AT: u64 = 9_000;

fn instructed(seed: u64, at: u64) -> Match {
    let mut m = match_fixture(seed).expect("fixture");
    m.home = m.home.clone().with_instructions(vec![Instruction {
        tick: at,
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
    let b = simulate_from_seed(&instructed(43, AT), 43, MatchConfig::default());
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
