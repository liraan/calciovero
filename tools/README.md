# tools/ — crates that are tools, not the simulation

**Responsibility (one sentence):** demonstrate, feed and test the engine
from the outside. Nothing in `L0-match/` depends on a crate here, except
`ds-match`'s tests on `ds-fixtures`.

| Crate | Responsibility |
|---|---|
| `ds-demo` | simulate a match and render it: the 2D debug replay (`--tui`), a text board, JSON, a self-contained HTML page |
| `ds-gui-view` | the record-driven 2.5D view shared by both GUIs: camera, interpolated frames, gestures from events and commentary |
| `ds-gui-desktop` | the Lavagna desktop GUI (egui, ADR-055): tactics board, post-its, phase formations and the match in `ds-gui-view` |
| `ds-gui-android` | the Android A/C GUI prototype: the same record-driven view in portrait and landscape |
| `ds-fixtures` | the hardcoded squads used by the demo and the tests (test data, not production data) |
| `ds-testkit` | the test contract (`Scenario`, `Feature`); no dependency on any `ds-*` crate |
| `ds-test-runner` | run the scenarios; `--check` is the health check of every commit |
| `ds-reference` | measure open real-match data (Wyscout soccer-logs, CC BY 4.0) and the engine the same way, per match; the averages go to `L0-match/reference/` |

## Contribute here

The tools are the easiest Rust to start with: they read the match and
never change it. Ideas: a marker per player's choice in the 2D view, a
timeline of events, a scenario test that proves something about a match.
See [`../CONTRIBUTING.md`](../CONTRIBUTING.md).

```bash
cargo run --release -p ds-gui-desktop
cargo run -p ds-demo -- --seed 42 --tui
cargo run -p ds-demo -- --seed 42 --tui --frame 30
cargo run -p ds-demo -- --seed 42 --roster
cargo run --release --bin ds-test-runner -- --check --seed 42
```
