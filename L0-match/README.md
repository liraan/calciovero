# L0-match/ — the first ring: two teams play one match

**Responsibility (one sentence):** a friendly match, played, saved,
reloaded, with the same result — its contract, its design, its data and
its code, together. It depends only on `doctrine/`.

| Directory | What it holds |
|---|---|
| [`specs/`](specs/) | the contract, ratified (L0.00 → L0.11). If the code and the spec disagree, the spec wins. |
| [`design/`](design/) | how the contract is implemented: the positional slice (§0 is the vision of the engine), the movement note, the architecture, the code tour, the event coverage, the decision table |
| [`balance/`](balance/) | the declared parameters, one JSON per formula, each with `_source`; arbitrary until calibrated (L3) |
| [`crates/`](crates/) | the engine, one responsibility per crate (ADR-041) |
| [`CHANGELOG-SIM.md`](CHANGELOG-SIM.md) | every change of simulation behaviour, with its `SIM_VERSION` |

## The crates, from the inside out

| Crate | Responsibility | Depends on |
|---|---|---|
| `ds-core` | the primitives: ids, time, hash, deterministic geometry | — |
| `ds-model` | the identity of the world: Player, Squad, Pitch, Match | core |
| `ds-stats` | the observation: events and statistics (Plane B), append-only | core, model |
| `ds-calculator` | a formula from `balance/`, applied to a player and a situation | — |
| `ds-db` | the vault: the players' performances, derived by key, one door | core, calculator |
| `ds-orchestrator` | the decision: which actor, when, with which scene; the contact | core, model, db, calculator |
| `ds-match` | the engine: the tick, the ball, the effect, the events, the save | all of the above |

`ds-match` has more than one reason to change, declared (OI-94).
