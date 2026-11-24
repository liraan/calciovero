# Writing a season plugin

A season plugin is a type that implements `ds_season_api::SeasonPlugin` (ADR-113). It is all you need: the core `ds-season` is not touched. The test `L1-season/crates/ds-season/tests/season.rs` has two such plugins, written outside the core, as a working example.

```rust
use ds_season_api::{Moment, PluginCard, PluginError, SeasonPlugin, SeasonWorld, Tier};

/// What it does, in one line.
#[derive(Default)]
pub struct MyPlugin {
    // Its state: saved with `state`, given back with `restore`.
    count: u32,
}

impl SeasonPlugin for MyPlugin {
    fn card(&self) -> PluginCard {
        PluginCard {
            name: "my_plugin",              // unique; its tables are "my_plugin.*"
            version: "1",                   // a change of behaviour is a new version
            steps: &[Moment::AfterMatch],   // when it acts (ADR-113 §1)
            depends: &["matchday"],         // who acts before it (§4)
            reads: &["matchday.results"],   // what it must find
            optional_reads: &[],            // what it reads if there, and what it does without (decision B)
            writes: &["my_plugin.count"],   // only its own tables
            tier: Tier::Optional,           // Core, Recommended or Optional (decision X)
            cost_us: 0,                     // its cost per day, measured
            narrative: &["my_event"],       // the commentary lines it gives
        }
    }

    fn run(&mut self, _moment: Moment, world: &mut SeasonWorld<'_>) -> Result<(), PluginError> {
        // No dice (ADR-035): derive from world.seed() and the state, never draw.
        self.count += 1;
        world.emit("MyEvent", &[("count", self.count.to_string())]);
        Ok(())
    }

    fn state(&self) -> String {
        self.count.to_string()
    }

    fn restore(&mut self, state: &str) -> Result<(), PluginError> {
        self.count = state.parse().map_err(|_| PluginError(format!("not a count: {state}")))?;
        Ok(())
    }
}
```

**The rules.** One plugin, one responsibility (ADR-041). It speaks to the others only through events and data. It reads only what its card declares, and writes only its own tables. The same seed and the same choices give the same season: `cargo test -p ds-season` checks the core, and your plugin needs its own twin test (the same scene with and without the fact it reads).
