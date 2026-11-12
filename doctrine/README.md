# doctrine/ — the centre

**Responsibility (one sentence):** what every level of the project obeys.
If a file anywhere else contradicts a file here, the other file is wrong.

| File | What it says |
|---|---|
| [`00-foundations.md`](00-foundations.md) | the doctrine: the pillars, the three planes, determinism, the banned things, the nest |
| [`VISION.md`](VISION.md) | what the manager does, and why they choose this game (draft) |
| [`WORLD-MODEL.md`](WORLD-MODEL.md) | the map of everything that can exist in a match, by level and horizon |
| [`STRUCTURE.md`](STRUCTURE.md) | how the repository is organised, and the rules that keep it coherent |
| [`_index.md`](_index.md) | the module map: every planned and written module, L0 to L3 |
| [`GLOSSARY.md`](GLOSSARY.md) | the terms |
| [`adr/`](adr/) | the decisions, one per file |
| [`PLUGIN_DEVELOPER_GUIDE.md`](PLUGIN_DEVELOPER_GUIDE.md), [`PLUGINS-WORLDS-READING-GUIDE.md`](PLUGINS-WORLDS-READING-GUIDE.md) | how to extend the engine (ADR-012, ADR-013) |

Read in this order the first time: foundations, VISION, STRUCTURE.

## Contribute here

- **Question a decision.** Every ADR can be reviewed: if the options are
  unbalanced or the decision contradicts the doctrine, say so in an issue
  or a PR. That is the most valuable review there is.
- **Propose a decision** for a new piece of the model: copy
  [`adr/000-template.md`](adr/000-template.md), status `proposed`.
- **Make a page clearer.** A document a newcomer cannot follow is a bug.

See [`../CONTRIBUTING.md`](../CONTRIBUTING.md).
