# tools/ — crates that are tools, not the simulation

**Responsibility (one sentence):** demonstrate, feed and test the engine
from the outside. Nothing in `L0-match/` depends on a crate here, except
`ds-match`'s tests on `ds-fixtures`.

| Crate | Responsibility |
|---|---|
| `ds-demo` | simulate a match and render it: the 2D debug replay (`--tui`), a text board, JSON, a self-contained HTML page |
| `ds-fixtures` | the hardcoded squads used by the demo and the tests (test data, not production data) |
| `ds-testkit` | the test contract (`Scenario`, `Feature`); no dependency on any `ds-*` crate |
| `ds-test-runner` | run the scenarios; `--check` is the health check of every commit |

```bash
cargo run -p ds-demo -- --seed 42 --tui
cargo run -p ds-demo -- --seed 42 --tui --frame 30
cargo run --release --bin ds-test-runner -- --check --seed 42
```
