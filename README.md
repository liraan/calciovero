<div align="center">

<img src="assets/calcio-vero-transparent.png" alt="Calcio Vero" width="280">

# Calcio Vero — Deep Simulator

**An open football management simulation.**<br>
*Transparent by design. Deterministic by construction. Yours by default.*

[![Status](https://img.shields.io/badge/status-L0_ratified-yellow)](https://github.com/liraan/calciovero)
[![Tests](https://img.shields.io/badge/tests-255_passing-brightgreen)](https://github.com/liraan/calciovero)
[![License](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-blue)](LICENSE-MIT)
[![Built with](https://img.shields.io/badge/docs-AI--assisted-purple)](#ai-disclosure)

[![Ko-fi](https://img.shields.io/badge/Ko--fi-supported-FF5E5B?logo=ko-fi&logoColor=white)](#support)

</div>

---

> What does it mean to *know* a footballer? Not his height, not his
> weight, not the number a database assigns him. What does it mean to
> know that a pass was late, that a defender read the play, that a
> striker hesitated for half a second before shooting?
>
> This project is an attempt to answer that question, in code, without
> lying to anyone.

## Why this exists

Football management games are extraordinary, and they are closed. The
datasets are licensed, the formulas are secret, the ecosystems are
gatekept. The genre learned to simulate football by simulating what
could be bought, and forgot that the interesting part — the part that
makes a fan lean forward — is *why* a pass was late, not how much the
passer is rated.

Calcio Vero rebuilds that experience without the walls. No telemetry.
No proprietary feeds. No hidden scalar pretending to be the truth.
Every formula is public, every architectural decision is documented,
every save is reproducible from its seed. You can read this project
down to its bones, and if you disagree with a bone, you can say so.

This is not a Football Manager clone. It is a different philosophy
applied to the same genre: where FM optimises for fidelity to licensed
data, we optimise for **explainability and ownership**.

## The core question: what is the truth, and what do you get to see?

The single most important architectural choice in this project is the
strict separation of three things that most games quietly collapse:
*what the simulation knows*, *what the game shows*, and *what people
believe*.

    ┌─────────────────────────────────────────────────────────┐
    │  PLANE A — Ground Truth                     (hidden)    │
    │  True attributes, real fatigue, latent psyche.          │
    │  Readable only by the match engine and the world gen.   │
    ├─────────────────────────────────────────────────────────┤
    │  PLANE B — Observation                      (events)    │
    │  MatchEvents: shots, passes, duels.                     │
    │  Ratings computed from events, never from latents.      │
    ├─────────────────────────────────────────────────────────┤
    │  PLANE C — Estimation                       (visible)   │
    │  Windowed indexes, projection bands, scout reports.     │
    │  Never feeds back into Plane A.                         │
    └─────────────────────────────────────────────────────────┘

Scouting, in this game, is not a lookup table. It is an *estimate of
an observation of the truth*. The manager is meant to be uncertain.
That uncertainty is the game.

The full doctrine is in [`specs/00-foundations.md`](specs/00-foundations.md).
The enforcement is [ADR-003](docs/adr/003-plane-a-enforcement.md).
The vision of why the engine has this shape is
[`docs/context/L0-POSITIONAL-SLICE.md`](docs/context/L0-POSITIONAL-SLICE.md) §0.

## A match is decisions, and a ball that obeys its properties

The players are not puppets. Each one is an actor: he perceives the
scene from his own point of view, reads it with his own
characteristics, and decides. The engine applies the act; it does not
choose it ([ADR-043](docs/adr/043-the-actor-decides.md)). The
goalkeepers are actors in the same sense
([ADR-045](docs/adr/045-the-goalkeeper-is-an-actor.md)). A misplaced
pass is an event with a cause — the passer's technique, the pressure he
was under, his balance — not a computed force vector. A defender
arriving a step late is a timing event, not an integration of
acceleration.

The ball is the other half. It does not read and it does not choose: it
has **declared physical properties**, and its trajectory is their
effect — gravity, friction, the reflection on a post, the bounce on the
ground — computed in integers, so that the same match is the same on
every machine ([ADR-044](docs/adr/044-the-chain.md) §3–§5,
[ADR-048](docs/adr/048-the-ball-can-be-deflected.md)).

So the engine is **phenomenological for the bodies and physical for the
ball**. It does not simulate biomechanics or contact forces: the effect
of a decision is modelled, not integrated. That keeps it deterministic,
calibratable (parameters in `balance/`, declared arbitrary until
calibrated), and — most importantly — **explainable**: every event
carries the fields to understand why it happened. "The pass was late
because the pressing closed the lane" is the target. "The collision
response exceeded the threshold" is not.

A physics engine for the bodies is not a later step of this project. It
is a different paradigm, not planned.

This section states the direction. How much of it the code does today
is in "What exists today", and it is less than this.

## What "deterministic" means, and why we mean it

Same seed → same match. Always. The same will hold for the season and
the career when L1 and L2 exist; today the contract is local
([ADR-013](docs/adr/013-worlds.md)). This is verified on every commit
by `ds-test-runner --check`:

      ✅ demo teams are valid
      ✅ same seed → same state_hash
      ✅ different seed → different state_hash
      ✅ save/load roundtrip preserves state_hash
      ✅ event chain is valid
      ✅ statistics are sane

The rules that make this possible are not negotiable:

- Determinism is by construction, not by seeded stream: every outcome
  is a deterministic function of the state and the characteristics of
  the actors, and interactions emerge from the local transitions
  (perception, decision, movement, contact) composed geometrically. No
  die is drawn in a simulation path
  ([ADR-035](docs/adr/035-no-dice-in-simulation.md)).
- `BTreeMap` everywhere in a simulation path, never `HashMap`.
- Fixed-point arithmetic for money: zero numerical drift.
- Golden hash tests on every simulation change.
- `SIM_VERSION` bump procedure documented in
  [`CHANGELOG-SIM.md`](CHANGELOG-SIM.md).

## What exists today

**L0 — the match engine — is complete as a contract, ratified,
implemented, and tested.** That sentence is true and it is also
honest: the engine is a *prototype*. It compiles, it is deterministic,
it is covered by **255 tests**, and its health check is green. But most
of the pitch is still a sketch: positions exist, movement is being
rebuilt, the ball is only beginning to travel, five of the thirty-one
event types are emitted.

The technology is Rust, chosen for type safety and for a determinism
that can be *guaranteed*, not hoped for. Money is fixed-point, so it
never drifts. The test harness (`ds-testkit`) is a small crate with
no dependency on the engine, so plugins and tools can contribute
scenarios without coupling. The presentation is TUI-first in v1; the
GUI is a future crate behind the same engine.

The full table of what is settled and what is not lives in
[`STRUCTURE.md`](STRUCTURE.md) §5, and the live tracker of what is
being built is [`docs/context/SESSION-STATE.md`](docs/context/SESSION-STATE.md).
This README will not pretend the prototype is the game.

**No playable build exists yet.** The documentation has to be solid
before the engine is.

## Try it in thirty seconds

Requires **Rust 1.85+ (stable)**. On Windows, use Git Bash or WSL.

    git clone https://github.com/liraan/calciovero.git
    cd calciovero
    cargo run --release --bin ds-test-runner -- --check --seed 42

If `--check` prints `6/6 checks passed`, the framework is healthy.
Then watch a match:

    cargo run --release --bin ds-demo -- --seed 42 --board

Every run with the same seed produces the same state hash. That is the
determinism contract, verified on every commit. The board is the debug
view, not the product; the product is the engine, and the engine is
being built.

## Where to read next

The specification is **concentric**. Read it in this order.

1. [`specs/00-foundations.md`](specs/00-foundations.md) — *Why does
   this project exist? What are the non-negotiable rules?*
2. [`STRUCTURE.md`](STRUCTURE.md) — *How is the repo organised?*
3. [`specs/_index.md`](specs/_index.md) — *What exists at each level,
   and in what state?*
4. [`docs/adr/`](docs/adr/) — *Which decisions have been made, and
   why?*
5. [`docs/GLOSSARY.md`](docs/GLOSSARY.md) — *What does each term mean?*
6. [`docs/PLUGIN_DEVELOPER_GUIDE.md`](docs/PLUGIN_DEVELOPER_GUIDE.md) —
   *How do I write a module? How do I test it?*

If you only have time for one file, read the doctrine. If you only
have time for one commit, read the one that closed the last session.

## How to contribute

Contributions are welcome — code, docs, data, bug reports, honest
criticism. The short version:

- One change per PR (one module, one ADR, one fix). In Phase 1 the
  maintainer pushes directly on `main`; external contributors use
  PRs.
- English is canonical for code, commits, and technical docs
  ([ADR-007](docs/adr/007-canonical-language.md)).
- Every factual claim needs a source. Every decision that changes an
  invariant needs an ADR.
- Every contribution is **reviewed by a human**. AI may draft; humans
  decide.
- Determinism is not negotiable: PRs that break the seed → hash
  invariant require an ADR.

Full process in [`CONTRIBUTING.md`](CONTRIBUTING.md).

## Continuous integration

The full quality gate (**C2**) runs on every push and pull request:
`cargo build --release`, `cargo clippy --workspace --all-targets -- -D warnings`,
`cargo test --workspace` (255 tests), `cargo fmt --check`, the cross-platform
determinism check on the golden hash (OI-58), and `scripts/gates.sh`.

On GitHub this is [`.github/workflows/ci.yml`](.github/workflows/ci.yml). On
Codeberg, [`.woodpecker/ci.yml`](.woodpecker/ci.yml) on `ci.codeberg.org` mirrors the same gate on
the backup remote, to verify that the backup is not a stale copy: Codeberg is
the backup, GitHub is primary.

## License

**Dual license** ([ADR-006](docs/adr/006-license.md), accepted):

- **Code** — `MIT OR Apache-2.0` → [`LICENSE-MIT`](LICENSE-MIT),
  [`LICENSE-APACHE`](LICENSE-APACHE)
- **Assets and documentation** — `CC-BY-NC-SA-4.0` →
  [`LICENSE-ASSETS`](LICENSE-ASSETS)
- **Trademarks** — the "Calcio Vero" name and logo are project
  trademarks; the licenses do not grant trademark rights.

Use the code however you want. Share the assets under the same terms,
for non-commercial purposes.

## Support

[![Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/liraan)

Calcio Vero is free and open. Donations are voluntary, one-off, and do
not buy influence over the roadmap (see OI-69 in
[`docs/context/SESSION-STATE.md`](docs/context/SESSION-STATE.md)).

## AI disclosure

Some documentation and code in this repo was **drafted with AI**, then
**reviewed, corrected, and owned by a human**. We say this openly
because transparency applies to our own process too.

- **AI drafts, humans decide.** Every architectural decision — the
  Three Planes, the ban on CA/PA, the determinism rules — was made by
  a person.
- **AI makes mistakes.** Caught in review: a non-existent crate
  dependency, a hasher that would have broken determinism, a
  contradictory panic strategy at the FFI boundary. Verify before you
  rely.
- **AI has no memory.** Each session starts blank. This repository is
  the memory.
- **No unreviewed AI code.** There is no unreviewed AI-written code in
  this repo. Every line has a human owner.

## Contact

- **Discussions** — [GitHub Discussions](https://github.com/liraan/calciovero/discussions)
- **Bugs / features** — [GitHub Issues](https://github.com/liraan/calciovero/issues)
- **Community chat** — *to be decided*

No private emails, no Telegram, no WhatsApp. Everything happens
publicly on GitHub.

---

<div align="center">

**© 2026 Calcio Vero Team**

*Transparent by design. Open data. Your control.*

</div>
