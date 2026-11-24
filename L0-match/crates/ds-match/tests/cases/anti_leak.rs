//! Anti-leak regression test (L0-match/specs/07-save-minimal.md:149).
//! A serialised save must not contain any known latent field name.
//! Structural, not exhaustive: covers the names in the current prototype.

use ds_core::ids::{MatchId, PlayerId, RefereeId, SquadId};
use ds_core::time::GameInstant;
use ds_match::{MatchConfig, MatchContext, MatchEngine, MatchSave, TextMatchEngine};
use ds_model::{
    FormationId, Match, MatchRules, MatchSeed, PhysicalStateMinimal, Pitch, Player,
    PsychologicalStateMinimal, Referee, RoleId, Squad,
};
use ds_orchestrator::GroundTruth;
use ds_stats::EventChain;

const LATENT_NAMES: &[&str] = &[
    "finishing",
    "passing",
    "injury_proneness",
    "goalkeeping",
    "composure",
    "attacker_score",
    "defender_score",
];

fn make_squad(id: u32, slot_base: u32) -> Squad {
    let players: Vec<Player> = (0..11)
        .map(|i| {
            Player::try_new(
                PlayerId::new(slot_base + i, 0),
                format!("P{}", slot_base + i),
                RoleId::new("midfielder").expect("role"),
                180,
                78,
                PhysicalStateMinimal::default(),
                PsychologicalStateMinimal::default(),
            )
            .expect("player")
        })
        .collect();
    Squad::try_new(
        SquadId::new(id, 0),
        players,
        vec![],
        FormationId::new("4-4-2").expect("formation"),
    )
    .expect("squad")
}

fn make_match() -> Match {
    Match::new(
        MatchId::new(1, 0),
        make_squad(1, 0),
        make_squad(2, 100),
        Pitch::default(),
        Referee::new(RefereeId::new(1, 0)),
        MatchSeed::from_raw(42),
        MatchRules::default(),
    )
}

#[test]
fn save_contains_no_latent_field_name() {
    let match_ = make_match();
    let mut hidden = GroundTruth::new(42, match_.id);
    let mut chain = EventChain::new();
    let ctx = MatchContext {
        match_: &match_,
        hidden: &mut hidden,
        chain: &mut chain,
        config: MatchConfig::default(),
    };
    let mut engine = TextMatchEngine::new();
    let result = engine.simulate(ctx);

    let save = MatchSave::from_result(&result, &match_, 42, GameInstant::from_raw(0));
    let bytes = save.to_bytes().expect("save serialises");
    let json = String::from_utf8(bytes).expect("valid UTF-8");

    for name in LATENT_NAMES {
        assert!(
            !json.contains(name),
            "save contains latent name {name:?}: Plane A leak through Factor.name"
        );
    }
}

/// P1 structural test: the decision is the actor's, not the engine's
/// (ADR-043, A.1-actor).
///
/// `MatchState::decide` must not call `resolve_choice` directly: the
/// call belongs to `ds_orchestrator::Actor::decide`. This test fails the
/// moment a future edit puts the call back in the engine, which is the
/// defect IDEA-DESIGN-013 names.
///
/// It reads its own crate's source (`state.rs` and the plugins, where the
/// decision lives since ADR-063), so it is an artefact of
/// the rule, not a behavioural test: the behaviour could be identical
/// and the rule still broken (P1: a test that cannot fail is worse than
/// no test; this one fails on the exact string).
#[test]
fn match_state_does_not_call_resolve_choice() {
    let sources = [
        include_str!("../../src/state.rs"),
        include_str!("../../src/plugins/ball.rs"),
        include_str!("../../src/plugins/bench.rs"),
        include_str!("../../src/plugins/body.rs"),
        include_str!("../../src/plugins/duels.rs"),
        include_str!("../../src/plugins/fatigue.rs"),
        include_str!("../../src/plugins/keeper.rs"),
        include_str!("../../src/plugins/perception.rs"),
        include_str!("../../src/plugins/referee.rs"),
        include_str!("../../src/plugins/set_pieces.rs"),
        include_str!("../../src/plugins/technique.rs"),
        include_str!("../../src/plugins/thought.rs"),
        // The rest of the engine (2026-11-23): the plugins' module, the
        // psychology plugin and the loop were not read.
        include_str!("../../src/plugins/mod.rs"),
        include_str!("../../src/plugins/psychology.rs"),
        include_str!("../../src/simulate.rs"),
        include_str!("../../src/driver.rs"),
        include_str!("../../src/engine.rs"),
    ];
    assert!(
        !sources.iter().any(|s| s.contains(".resolve_choice(")),
        "MatchState calls resolve_choice directly: the decision must go through \
         ds_orchestrator::Actor::decide (ADR-043, A.1-actor)"
    );
}

/// ADR-112: the engine reads no characteristic of Plane A, by name or whole.
/// It asks the actor for his readings (`GroundTruth::strong_foot`,
/// `believed_run_pace`, `offside_line_misread_cm`, `wall_cover_cm`,
/// `contact_force_milli`, the `resolve_*`); a plugin reads one only through
/// `World::characteristic`, and only what its card declares. Before, six
/// places of the engine read raw characteristics by name and one the whole
/// latents.
#[test]
fn the_engine_reads_no_characteristic_but_through_its_cards() {
    let engine = [
        ("balance", include_str!("../../src/balance.rs")),
        ("context", include_str!("../../src/context.rs")),
        ("driver", include_str!("../../src/driver.rs")),
        ("engine", include_str!("../../src/engine.rs")),
        ("fact", include_str!("../../src/fact.rs")),
        ("lib", include_str!("../../src/lib.rs")),
        ("perception", include_str!("../../src/perception.rs")),
        ("ball", include_str!("../../src/plugins/ball.rs")),
        ("bench", include_str!("../../src/plugins/bench.rs")),
        ("body", include_str!("../../src/plugins/body.rs")),
        ("duels", include_str!("../../src/plugins/duels.rs")),
        ("fatigue", include_str!("../../src/plugins/fatigue.rs")),
        ("keeper", include_str!("../../src/plugins/keeper.rs")),
        ("plugins", include_str!("../../src/plugins/mod.rs")),
        (
            "perception",
            include_str!("../../src/plugins/perception.rs"),
        ),
        (
            "psychology",
            include_str!("../../src/plugins/psychology.rs"),
        ),
        ("referee", include_str!("../../src/plugins/referee.rs")),
        (
            "set_pieces",
            include_str!("../../src/plugins/set_pieces.rs"),
        ),
        ("technique", include_str!("../../src/plugins/technique.rs")),
        ("thought", include_str!("../../src/plugins/thought.rs")),
        ("record", include_str!("../../src/record.rs")),
        ("recording", include_str!("../../src/recording.rs")),
        ("registry", include_str!("../../src/registry.rs")),
        ("result", include_str!("../../src/result.rs")),
        ("save", include_str!("../../src/save.rs")),
        ("shape", include_str!("../../src/shape.rs")),
        ("simulate", include_str!("../../src/simulate.rs")),
        ("spatial", include_str!("../../src/spatial.rs")),
        ("state", include_str!("../../src/state.rs")),
        ("test_support", include_str!("../../src/test_support.rs")),
    ];
    for (name, source) in engine {
        // The code, not its tests: a test may read what it checks.
        let code = source.split("#[cfg(test)]").next().unwrap_or(source);
        assert!(
            !code.contains("latents_for("),
            "{name}: reads the whole latents (ADR-112)"
        );
        let mut rest = code;
        while let Some(at) = rest.find(".characteristic(") {
            let before = &rest[..at];
            let after = &rest[at + ".characteristic(".len()..];
            rest = after;
            // The one gate: the `World`s of plugins/mod.rs, which give only
            // what the card of the plugin they serve declares (code audit S6).
            if name == "plugins" && before.ends_with("self.hidden") {
                continue;
            }
            assert!(
                before.ends_with("world"),
                "{name}: reads a characteristic outside `World` (ADR-112)"
            );
            // The name is a literal, read here (code audit S6: before, the
            // first quoted string anywhere after the call).
            let arg = after.split_once(',').map_or("", |(_, a)| a.trim_start());
            assert!(
                arg.starts_with('"'),
                "{name}: reads a characteristic by a name that is not a literal"
            );
            let quoted = arg.split('"').nth(1).unwrap_or("");
            let declared = ds_match::registry::card(name).reads;
            assert!(
                declared.contains(&quoted),
                "{name}: reads {quoted:?} but its card declares {declared:?} (ADR-063 §4)"
            );
        }
    }
}
