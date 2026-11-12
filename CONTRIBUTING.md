# Contributing to Calcio Vero

Thank you for wanting to help. Calcio Vero is built in the open by
volunteers, and it is organised so that you can pick **one piece**, work
on it alone, and see the result in a match. You do not need to understand
the whole project to make a useful contribution.

This page tells you **what you can do, where, and how**. Each directory of
the repository has its own `README.md` that goes into the details of that
piece.

---

## 1. Pick your path

| If you… | You can… | Where | Code needed? |
|---|---|---|---|
| **know football** | tell us what a fan would not recognise in a match ("a defender would never do that") | an issue with the template *It is not football* | no |
| **know football** | tune the numbers that drive the match: the players' profiles by role, the team's shape, the duel, the ball, the keeper | [`L0-match/balance/`](L0-match/balance/README.md) | no (JSON) |
| **know the Laws of the Game** | write a law the engine does not apply yet (offside, fouls, advantage) as a pure function | [`rules.rs`](L0-match/crates/ds-match/src/rules.rs) | a little Rust |
| **know tactics** | add a formation, or improve how the block moves without instructions | [`shape.rs`](L0-match/crates/ds-match/src/shape.rs) | a little Rust |
| **like testing** | write a scenario that proves something about a match | [`tools/`](tools/README.md) (`ds-testkit`) | Rust |
| **like tools** | improve the 2D debug view, the board, the HTML page | [`tools/`](tools/README.md) (`ds-demo`) | Rust |
| **like writing** | fix a document, explain something better, translate | any `.md` (§6) | no |
| **want to go deep** | work on the engine itself: perception, choice, movement. Every act on the pitch, what it reads and whether the engine does it yet, is in [`ACTIONS.md`](L0-match/design/ACTIONS.md): a ✗ row is a task | [`L0-match/`](L0-match/README.md) | Rust, and an ADR for model changes |

If you are not sure, open a [Discussion](https://github.com/liraan/calciovero/discussions)
and say what you know: we will find you a piece.

### Open tasks for newcomers

Real tasks the project needs now, each small enough for one pull request,
each with its issue (label `good first issue`). Comment on the issue before
you start, so that two people do not take the same task.

| Task | Path | Where | Code? |
|---|---|---|---|
| [#9](https://github.com/liraan/calciovero/issues/9) Watch three seeds in the 2D view and report what a fan would not recognise | football | an issue *It is not football* | no |
| [#10](https://github.com/liraan/calciovero/issues/10) Give `profiles.json` sourced values: heights and weights by position from public aggregate statistics | football | `L0-match/balance/profiles.json` | no |
| [#11](https://github.com/liraan/calciovero/issues/11) Compare `block.json` with real team shapes (depth and width of a block, in and out of possession) and propose values with a source | football | `L0-match/balance/block.json` | no |
| [#12](https://github.com/liraan/calciovero/issues/12) Add a 4-3-3 next to the 4-4-2 (the grid and the roles) | tactics | `shape.rs` | a little |
| [#13](https://github.com/liraan/calciovero/issues/13) Write offside as a pure function: given the positions at a pass, is the receiver offside? | the Laws | `rules.rs` | a little |
| [#14](https://github.com/liraan/calciovero/issues/14) A test scenario: "a pass to a free teammate a few metres away is completed" | testing | `tools/ds-testkit`, `tools/ds-test-runner` | yes |
| [#15](https://github.com/liraan/calciovero/issues/15) Show each player's choice in the 2D view (who presses, who holds) with a different marker | tools | `tools/ds-demo/src/pitchview.rs` | yes |
| [#16](https://github.com/liraan/calciovero/issues/16) An Italian (or other) translation of the README, as `README.<lang>.md` | writing | root | no |
| [#17](https://github.com/liraan/calciovero/issues/17) Load a folder of changed `balance/` files at runtime, without rebuilding (needs an ADR first: the engine does no I/O, ADR-038) | the engine | `tools/ds-demo`, `L0-match/crates/ds-match` | yes |

The list changes as tasks are done; the maintainer's own plan is in
[`process/SESSION-STATE.md`](process/SESSION-STATE.md) (the `[next]` row).

---

## 2. Set up (ten minutes)

You need **Rust 1.85+ (stable)** and git. On Windows, use Git Bash.

```bash
git clone https://github.com/liraan/calciovero.git
cd calciovero
git config core.hooksPath .githooks
cargo run --release -p ds-demo -- --seed 42 --tui
```

The last command replays a match on a text pitch. If you see players
moving, you are ready. Other useful commands:

```bash
cargo run --release -p ds-demo -- --seed 42 --roster          # who is who
cargo run --release -p ds-demo -- --seed 42 --tui --frame 30  # the pitch at 30'
cargo test --workspace                                         # all the tests
```

The hooks in `.githooks/` run the checks for you before a commit and a
push: they are the same checks the CI runs.

---

## 3. The workflow

1. **Open an issue first** for anything bigger than a typo, so that two
   people do not do the same work.
2. **Fork, branch, change one thing.** One change per pull request: one
   balance file, one law, one fix, one document.
3. **Check it.** For code: `cargo fmt`, `cargo clippy --workspace
   --all-targets -- -D warnings`, `cargo test --workspace`. For balance:
   run a few seeds in the 2D view and write in the PR what you saw before
   and after.
4. **Open the pull request**, filling in the template. Say *why*, not only
   what.
5. **A human reviews it.** Expect questions: they are about the football
   and the rules below, never about you.

Commit messages: imperative mood, a first line of at most 72 characters,
then a body that explains the why.

---

## 4. The rules that never bend

They come from the doctrine ([`doctrine/00-foundations.md`](doctrine/00-foundations.md)).
A pull request that breaks one of them is not merged, however good it is
otherwise.

1. **No dice in a match.** Nothing on the pitch is decided by a random
   draw: what happens comes from the players' characteristics, the
   situation and the geometry ([ADR-035](doctrine/adr/035-no-dice-in-simulation.md)).
2. **Every choice on the pitch is the player's.** The engine applies what a
   player chose; it never chooses for him ([`doctrine/VISION.md`](doctrine/VISION.md) §3).
3. **No hardcoded numbers.** Every parameter lives in `L0-match/balance/`,
   with its source or an honest "arbitrary until calibrated".
4. **The hidden truth stays hidden.** A player's real characteristics are
   never shown, logged or saved outside the engine; the screens show only
   what can be observed.
5. **Same seed, same match.** If your change alters what happens in a
   match, it bumps `SIM_VERSION` and regenerates the golden test, with an
   entry in [`L0-match/CHANGELOG-SIM.md`](L0-match/CHANGELOG-SIM.md) (the
   procedure is at the top of that file). Not sure? Ask in the PR.
6. **A test that can fail.** A behaviour change comes with a test that
   fails without it.

---

## 5. Where things are

The repository is organised in rings, like the project: the rules at the
centre, then the match, then the tools, then the working notes.

```
doctrine/    the rules everything obeys: foundations, vision, decisions (ADRs), glossary
L0-match/    the match: specs/, design/, balance/ (the numbers), crates/ (the engine)
tools/       demo and 2D view, fixtures, test harness, test runner
process/     the maintainer's working notes and scripts: you can ignore it
```

Each directory's `README.md` says what it is for. The full rules of the
repository are in [`doctrine/STRUCTURE.md`](doctrine/STRUCTURE.md).

---

## 6. Documentation and decisions

- **Fixing a document** is always welcome: open a PR.
- **English** is the language of code, commits and technical documents
  ([ADR-007](doctrine/adr/007-canonical-language.md)). A translation is
  welcome as a separate file.
- **A design decision** (how the engine models something) is written as an
  ADR: copy [`doctrine/adr/000-template.md`](doctrine/adr/000-template.md),
  set it to `proposed`, open a PR, and the discussion happens there. A
  `proposed` ADR blocks code in its area until it is accepted. The status
  of every ADR is in [`doctrine/STRUCTURE.md`](doctrine/STRUCTURE.md) §5.
- **A specification** (`L0-match/specs/`) is a ratified contract: change it
  with a version bump in its frontmatter, and with an ADR if the change is
  major ([`doctrine/STRUCTURE.md`](doctrine/STRUCTURE.md) §4 and §8).

---

## 7. Licensing of contributions

By contributing you agree that **code** is licensed under **MIT OR
Apache-2.0**, and **assets and documentation** under **CC-BY-NC-SA-4.0**
([`LICENSE.md`](LICENSE.md), [ADR-006](doctrine/adr/006-license.md)). If
you contribute on behalf of an employer, make sure you may do so.

No licensed or scraped data: players are fictional, and real data stays
outside the repository.

---

## 8. Code of conduct

Be plain. Argue with the idea, not the person. If you are wrong, say so;
if you do not know, say so. The honesty we ask of the engine is the
honesty we ask of each other.

AI-assisted contributions are welcome, and must be reviewed and owned by
the human who opens the PR: say in the PR that AI helped.

---

## Questions?

Open a [Discussion](https://github.com/liraan/calciovero/discussions) or an
issue with the `question` label. Everything happens in public.
