//! The season core with plugins written here, outside it (ADR-113 §12, L1
//! step 2: a plugin made from the template passes without touching the core).

use ds_season::{SEASON_VERSION, Season};
use ds_season_api::{
    MATCH_PLAYED, Moment, PluginCard, PluginError, SeasonPlugin, SeasonWorld, Tier,
};

/// A matchday stand-in: one "match" every third day.
struct Fixtures;

impl SeasonPlugin for Fixtures {
    fn card(&self) -> PluginCard {
        PluginCard {
            name: "fixtures",
            version: "1",
            steps: &[Moment::BeforeMatches],
            depends: &[],
            reads: &[],
            optional_reads: &[],
            writes: &["fixtures.matches"],
            tier: Tier::Core,
            cost_us: 0,
            narrative: &[],
        }
    }
    fn run(&mut self, _: Moment, w: &mut SeasonWorld<'_>) -> Result<(), PluginError> {
        if w.day() % 3 == 0 {
            w.emit(
                MATCH_PLAYED,
                &[
                    ("id", w.day().to_string()),
                    ("home_goals", (w.seed() % 3).to_string()),
                ],
            );
        }
        Ok(())
    }
}

/// A plugin from the template: it counts the matches it is told of.
#[derive(Default)]
struct Tally {
    matches: u32,
}

impl SeasonPlugin for Tally {
    fn card(&self) -> PluginCard {
        PluginCard {
            name: "tally",
            version: "1",
            steps: &[Moment::AfterMatch, Moment::SeasonEnded],
            depends: &["fixtures"],
            reads: &["fixtures.matches"],
            optional_reads: &[],
            writes: &["tally.count"],
            tier: Tier::Recommended,
            cost_us: 0,
            narrative: &[],
        }
    }
    fn run(&mut self, m: Moment, w: &mut SeasonWorld<'_>) -> Result<(), PluginError> {
        match m {
            Moment::AfterMatch => {
                let id = w
                    .current_match()
                    .map(|e| e.fields[0].1.clone())
                    .unwrap_or_default();
                self.matches += 1;
                w.emit("Counted", &[("match", id)]);
            }
            Moment::SeasonEnded => w.emit("SeasonEnded", &[("matches", self.matches.to_string())]),
            _ => {}
        }
        Ok(())
    }
    fn state(&self) -> String {
        self.matches.to_string()
    }
    fn restore(&mut self, s: &str) -> Result<(), PluginError> {
        self.matches = s
            .parse()
            .map_err(|_| PluginError(format!("not a count: {s}")))?;
        Ok(())
    }
}

fn plugins() -> Vec<Box<dyn SeasonPlugin>> {
    vec![Box::new(Tally::default()), Box::new(Fixtures)]
}

/// The order is the validator's, not the registry's; `AfterMatch` runs
/// once per match; the season ends once.
#[test]
fn a_season_runs_its_days_and_its_plugins_in_order() {
    let mut s = Season::new(7, 10, plugins()).expect("valid cards");
    assert_eq!(s.order(), ["fixtures", "tally"]);
    s.run_to_end().expect("runs");
    let count = |k: &str| s.events().iter().filter(|e| e.kind == k).count();
    assert_eq!(count(MATCH_PLAYED), 4); // days 0, 3, 6, 9
    assert_eq!(count("Counted"), 4);
    let end: Vec<_> = s
        .events()
        .iter()
        .filter(|e| e.kind == "SeasonEnded")
        .collect();
    assert_eq!(end.len(), 1);
    assert_eq!(end[0].fields[0].1, "4");
    assert!(s.is_over());
}

/// The same seed, the same season (ADR-113 §12).
#[test]
fn the_same_seed_gives_the_same_season() {
    let hash = |seed| {
        let mut s = Season::new(seed, 12, plugins()).expect("valid");
        s.run_to_end().expect("runs");
        s.hash()
    };
    assert_eq!(hash(5), hash(5));
    assert_ne!(hash(5), hash(6));
}

/// Saved half-way and loaded, it ends as if never saved (ADR-113 §9).
#[test]
fn a_saved_season_goes_on_the_same() {
    let mut whole = Season::new(3, 12, plugins()).expect("valid");
    whole.run_to_end().expect("runs");
    let mut half = Season::new(3, 12, plugins()).expect("valid");
    for _ in 0..5 {
        half.advance_day().expect("runs");
    }
    let save = half.save();
    assert_eq!(save.season_version, SEASON_VERSION);
    let text = serde_json::to_string(&save).expect("serialises");
    let back: ds_season::SeasonSave = serde_json::from_str(&text).expect("parses");
    let mut loaded = Season::load(&back, plugins()).expect("loads");
    loaded.run_to_end().expect("runs");
    assert_eq!(loaded.hash(), whole.hash());
}

/// A save of other plugins is refused, and says why.
#[test]
fn a_save_of_other_plugins_is_refused() {
    let s = Season::new(3, 4, plugins()).expect("valid");
    let save = s.save();
    let only_fixtures: Vec<Box<dyn SeasonPlugin>> = vec![Box::new(Fixtures)];
    assert!(Season::load(&save, only_fixtures).is_err());
}
