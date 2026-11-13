# L0-match/balance/ — the numbers that drive the match

**Responsibility (one sentence):** every parameter of the match, as data
that anyone can read and change without touching the code. Each file has a
`_note` (what the numbers mean) and a `_source` (where they come from, or
"arbitrary until calibrated").

This is the best place to start if you know football and not Rust.

## What each file controls

| File | What you see on the pitch |
|---|---|
| `profiles.json` | **who the players are**: for each role (goalkeeper, centre back, full back, central and wide midfielder, striker) the mean and spread of every hidden characteristic and of height and weight |
| `block.json` | **the team's shape without instructions**: how deep and wide the block is with and without the ball, how high the defensive line goes, how much it shifts to the ball's side |
| `keeper.json` | **the goalkeeper's position**: how far he comes off his line as the ball approaches, and how far he steps up when his side has the ball |
| `kick.json` | **the kick**: how fast a shot leaves, how precise a kick is (shooting, the foot, power, pressure), how a pass is weighed to reach the receiver |
| `save.json` | **the save**: how fast a keeper reacts, how far and how fast he dives, how hard a ball is to hold, how far a parry sends it |
| `thought.json` | **the player's thought** (ADR-052): what keeping the ball is worth against a goal, the goal's width; the chance a position gives is computed from the shot and the save |
| `laws.json` | **the referee is the Laws**: when a foul is reckless (yellow) or excessive (red), what denies an obvious chance, the penalty area and mark |
| `duel.json` | **the tackle**: how close a defender must be to challenge, how often, and how much tackling counts against keeping the ball |
| `carry.json` | **running with the ball**: how far a carrier goes before choosing again, and where he stops near the goal |
| `ball.json` | **the ball**: gravity, how much the air and the grass slow it, when it stops |
| `obstacles.json` | **the goal**: posts and crossbar, and how much the ball bounces off them |
| `latency.json` | **how fast players decide**: from about 0.7 s (a quick reader in space) to 2-3 s (a slow one under pressure) |
| `choice.json`, `shot_choice.json` | **how players choose**: when an option is clear enough to play, how a shot is aimed |
| `reception.json` | **receiving a pass**: when a first touch keeps the ball, how close a loose ball must be to be controlled |
| `influence.json` | **pressure**: how close an opponent must be to press the carrier |
| `movement.json` | **running**: how force and weight turn into acceleration |
| `scanning.json` | **looking around**: how often a player glances away from the ball, and at what |
| `stoppage_time.json` | **added time**: seconds added per goal, foul, duel |

## How to change a number and see the effect

1. Pick a seed and watch a stretch of the match:

   ```bash
   cargo run --release -p ds-demo -- --seed 42 --tui --from 20 --to 25
   ```

2. Change one number in one file (keep the `_note` and `_source` honest:
   if you have a source, cite it).
3. Run the same command again: same seed, so every difference you see
   comes from your change.
4. Run the tests: `cargo test --workspace`. A test that fails is either a
   rule your change broke, or the golden test telling you that the match
   changed. The second is expected: see the procedure in
   [`../CHANGELOG-SIM.md`](../CHANGELOG-SIM.md), or ask in your PR and a
   maintainer will help.
5. Open a pull request with one file changed, and write what you saw
   before and after (seed, minute).

The files are compiled into the program, so a change needs the
`cargo run` above (it rebuilds). Loading a folder of changed files at
runtime, without rebuilding, is planned.
