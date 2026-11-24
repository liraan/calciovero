//! L1 step 3: 20 clubs and their squads, the same from the same seed.

use ds_roster::Roster;
use ds_season::Season;
use ds_season_api::SeasonPlugin;
use ds_world::World;

fn season(seed: u64) -> Season {
    let plugins: Vec<Box<dyn SeasonPlugin>> =
        vec![Box::new(Roster::default()), Box::new(World::default())];
    let mut s = Season::new(seed, 1, plugins).expect("valid cards");
    s.run_to_end().expect("runs");
    s
}

#[test]
fn twenty_clubs_and_their_squads_the_same_from_the_same_seed() {
    let s = season(42);
    assert_eq!(s.order(), ["world", "roster"]);
    let clubs = s
        .events()
        .iter()
        .filter(|e| e.kind == ds_world::CLUB_CREATED)
        .count();
    let squads = s
        .events()
        .iter()
        .filter(|e| e.kind == ds_roster::SQUAD_REGISTERED)
        .count();
    assert_eq!((clubs, squads), (20, 20));
    assert_eq!(s.hash(), season(42).hash());
    assert_ne!(s.hash(), season(43).hash());
}

/// No player in two squads; every squad has three keepers.
#[test]
fn every_player_has_one_club() {
    let mut ids = std::collections::BTreeSet::new();
    for club in 0..20 {
        let squad = ds_roster::make_squad(42, club);
        assert_eq!(squad.iter().filter(|p| p.role == "goalkeeper").count(), 3);
        for p in squad {
            assert_eq!(p.id / 100, club, "the side is read from the id");
            assert!(ids.insert(p.id), "player {} twice", p.id);
        }
    }
}
