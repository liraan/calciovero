//! Anti-leak regression test (specs/L0-match/07-save-minimal.md:149).
//! A serialised save must not contain any known latent field name.
//! Structural, not exhaustive: covers the names in the current prototype.

use ds_core::ids::{MatchId, PlayerId, RefereeId, SquadId};
use ds_core::rng::RngRegistry;
use ds_core::time::GameInstant;
use pcfm_hidden::GroundTruth;
use pcfm_match::{MatchConfig, MatchContext, MatchEngine, MatchSave, TextMatchEngine};
use pcfm_model::{
    FormationId, Match, MatchRules, MatchSeed, PhysicalStateMinimal, Pitch, Player,
    PsychologicalStateMinimal, Referee, RoleId, Squad,
};
use pcfm_stats::EventChain;

const LATENT_NAMES: &[&str] = &[
    "finishing",
    "passing",
    "injury_proneness",
    "goalkeeping",
    "set_pieces",
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
        Referee::try_new(RefereeId::new(1, 0), 0.5, 0.7).expect("referee"),
        MatchSeed::from_raw(42),
        MatchRules::default(),
    )
}

#[test]
fn save_contains_no_latent_field_name() {
    let match_ = make_match();
    let mut hidden = GroundTruth::new(42, match_.id);
    let mut rng = RngRegistry::new(42);
    let mut chain = EventChain::new();
    let ctx = MatchContext {
        match_: &match_,
        hidden: &mut hidden,
        rng: &mut rng,
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
