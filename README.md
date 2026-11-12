<div align="center">

<img src="assets/calcio-vero-transparent.png" alt="Calcio Vero" width="280">

# Calcio Vero

**An open football management game where the match tells you *why*.**<br>
*No star ratings. No hidden dice. No secrets you are not allowed to read.*

[![Status](https://img.shields.io/badge/status-early_prototype-orange)](#where-we-are-honestly)
[![Tests](https://img.shields.io/badge/tests-262_passing-brightgreen)](https://github.com/liraan/calciovero)
[![License](https://img.shields.io/badge/license-MIT_OR_Apache--2.0-blue)](LICENSE-MIT)
[![Built with](https://img.shields.io/badge/docs-AI--assisted-purple)](#ai-disclosure)

[![Ko-fi](https://img.shields.io/badge/Ko--fi-supported-FF5E5B?logo=ko-fi&logoColor=white)](#support)

</div>

---

> You have been here before. Your striker is "in superb form". The
> scout gave him four and a half stars. You lose 3-0 at home, and the
> match engine shrugs.
>
> Why did he miss? Was it his finishing, the defender, the pressure, the
> pass that arrived on his weak foot? Somewhere a number knows. You will
> never see it.
>
> **Calcio Vero is built on the opposite idea.**

## If you play football managers, this is for you

You already know what makes these games great: the squad you built
yourself, the kid from the academy who made it, the season that
turned in February. Calcio Vero keeps all of that. What it changes is
**what you are allowed to understand**.

| You know this feeling… | Calcio Vero's answer |
|---|---|
| "My team played badly" is the whole post-match analysis. | **Every event has a cause you can read.** "Their 8 pressed your 6 from his blind side four times; he lost the ball twice; the second one was the goal." |
| A 4.5-star player turns out to be a 2-star player. | **The scout gives an estimate *and how sure he is*.** A narrow band is a safe signing. A wide band is a gamble, and you can see it is one. |
| Somewhere, a hidden "current ability" decides everything. | **There is no CA/PA.** A player is what he does: his characteristics, read in the situation, from his own point of view. |
| The result feels like a dice roll you are not shown. | **No die is ever rolled in a match.** What looks like luck is a cause you have not spotted yet, and the game can show it to you. |
| You cannot check the formulas, only argue about them on forums. | **Every formula is public.** Every parameter is a JSON file. Change a number, replay the same match, watch what changes. |
| You lose a great save to a patch. | **Same save + same match = same result, on every machine.** Saves are reproducible from their seed. |

## The match: decisions, and a ball with physics

The players are not puppets moved by a script. Each one is an **actor**:
he sees the pitch from where he stands, with his own vision and his own
reading of the game, and he chooses. A late pass is late *because* the
passer was under pressure and looked up a moment too slowly. A defender
who arrives a step late arrived late for a reason.

The ball is the other half. It does not choose: it has declared physical
properties, gravity, friction, the bounce off the post, computed in
integers so that the same match is the same everywhere.

That is the whole design: **the players decide, the ball obeys physics,
and every event carries the reason it happened.** The rest of the game,
from scouting to the academy, is built so that you can ask "why?" and
get an honest answer.

## What you will do as a manager

The loop is familiar, and that is the point:

```
observe  ->  decide  ->  watch  ->  understand  ->  adjust
```

**Observe** what your players actually did, through ratings built from
events, never from hidden numbers. **Decide** the eleven, the tactic,
the training, the signings. **Watch** the match in a 2D view or as radio
commentary. **Understand** what happened, down to the moment and the
reason. **Adjust**, because you understood, not because a number went up.

The soul is old-school: dense, readable screens, text commentary, a
world that reacts. Think of the depth of the modern games with the soul
of the classic Italian ones. The longer version is in
[`doctrine/VISION.md`](doctrine/VISION.md).

## Where we are, honestly

**There is no playable game yet.** This is an early prototype of the
match engine, built in the open, one verified step at a time.

What exists today:

- a **deterministic match engine** for one match, with players who
  perceive, choose and move, a ball that flies, a goalkeeper who
  positions himself, posts and crossbar that deflect the ball;
- a **text board** and an HTML page that show the result of a match;
- **262 tests** and a health check that runs on every commit.

What does not exist yet: a pitch view you can watch, the rules of the
game (throw-ins, offside, fouls), fatigue, the season, the career, the
transfer market. The order in which they arrive is public.

| Milestone | What you get |
|---|---|
| **The match can be watched** | a 2D view of twenty-two players and a ball, from any seed |
| **The match reads as football** | shots, saves, restarts, fouls; a match a fan recognises |
| **A friendly you can play** | pick the line-ups, play ninety minutes, save, reload, read why you conceded |
| **A season**, then **a career** | standings and fixtures; then scouting, the market, the academy |

The second one is the longest. We will not skip it to ship faster: a
match that does not read as football would make everything after it
pointless.

## Try the engine today

Requires **Rust 1.85+ (stable)**. On Windows, use Git Bash or WSL.

    git clone https://github.com/liraan/calciovero.git
    cd calciovero
    cargo run --release --bin ds-demo -- --seed 42 --board

Run it twice: same seed, same match, to the last event. Then open
[`L0-match/balance/`](L0-match/balance/), change a number, and run it
again.

## How you can help, even without writing code

- **Star and watch the repository** to follow the milestones.
- **Tell us what "real football" means to you.** The engine is judged
  by one question: *would a fan recognise this?* Open a
  [Discussion](https://github.com/liraan/calciovero/discussions) with
  the situations a match engine always gets wrong for you.
- **Bring your football knowledge.** Every parameter is waiting for a
  sourced value instead of a guess.
- **Play-test**, when the first friendly is playable.

## For developers

The project is written in Rust, and organised like its own design:
concentric rings, one responsibility per directory, dependencies
pointing inward.

```
doctrine/    the rules everything obeys: foundations, vision, decisions (ADRs)
L0-match/    the match: its spec, its design notes, its parameters, its engine
tools/       demo, fixtures, test harness, test runner
process/     how the work is done: the bookmark, the session notes, the scripts
```

Start with [`doctrine/00-foundations.md`](doctrine/00-foundations.md),
then [`doctrine/STRUCTURE.md`](doctrine/STRUCTURE.md) and
[`L0-match/README.md`](L0-match/README.md). The three rules you will
meet everywhere:

- **Three planes.** The truth (hidden), the observation (events and
  statistics), the estimate (what the manager sees). Nothing leaks from
  the first to the third.
- **No die in a simulation path**
  ([ADR-035](doctrine/adr/035-no-dice-in-simulation.md)). Determinism is
  by construction.
- **No hardcoded balance.** Every parameter lives in a file, with its
  source or with an honest "arbitrary until calibrated".

Contributions are welcome: code, docs, data, bug reports, honest
criticism. One change per PR, English for code and technical docs, a
source for every factual claim, an ADR for every decision that changes an
invariant, and a human review for everything. The full process is in
[`CONTRIBUTING.md`](CONTRIBUTING.md).

**Continuous integration.** Every push runs the full gate: build,
clippy with warnings as errors, the test suite with the golden hash,
fmt, the health check and the documentation gates
([`process/scripts/gates.sh`](process/scripts/gates.sh)). GitHub is the
primary remote ([`.github/workflows/ci.yml`](.github/workflows/ci.yml));
Codeberg is the backup, with the same gate on Woodpecker
([`.woodpecker/ci.yml`](.woodpecker/ci.yml)).

## License

**Dual license** ([ADR-006](doctrine/adr/006-license.md)):

- **Code**: `MIT OR Apache-2.0` ([`LICENSE-MIT`](LICENSE-MIT),
  [`LICENSE-APACHE`](LICENSE-APACHE)).
- **Assets and documentation**: `CC-BY-NC-SA-4.0`
  ([`LICENSE-ASSETS`](LICENSE-ASSETS)).
- **Trademarks**: the "Calcio Vero" name and logo are project
  trademarks; the licences do not grant trademark rights.

Calcio Vero uses no licensed data: no real squads, no real players, no
scraped databases. Players are fictional; the football is real.

Football Manager is a trademark of Sports Interactive. Calcio Vero is
not affiliated with it.

## Support

[![Ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/liraan)

Calcio Vero is free and open. Donations are voluntary, one-off, and do
not buy influence over the roadmap.

## AI disclosure

Some documentation and code in this repository was **drafted with AI**,
then **reviewed, corrected and owned by a human**. Every architectural
decision was made by a person; AI drafts, humans decide. We say it
openly because transparency applies to our own process too.

## Contact

Everything happens in public:
[Discussions](https://github.com/liraan/calciovero/discussions) for ideas
and questions, [Issues](https://github.com/liraan/calciovero/issues) for
bugs.

---

<div align="center">

*Transparent by design. Deterministic by construction. Yours by default.*

</div>
