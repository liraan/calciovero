# ADR-049 — The off-ball act is the player's

- **Status:** accepted
- **Date:** 2026-11-12
- **Supersedes:** —
- **Amends:** ADR-043 (the actor decides: extended from the carrier to every player); ADR-042 (the duel: the challenge becomes the defender's choice); the team block of `SIM_VERSION` 42 (`shape`: from an order to a candidate)
- **Depends on:** VISION §3, ADR-034 (perception), ADR-035 (no dice), ADR-040 (salience), ADR-041 (one crate, one responsibility)

## Context

The maintainer, 2026-11-12: *"every player should be an orchestrator, chooser and actor of his own part"*, *"for every choice on the pitch, I want it to really be the player's choice"*.

Today only the carrier chooses (`Actor::decide`). For everyone else the engine chooses: the nearest perceiving player of each side goes for a loose ball, the nearest opponent presses the carrier and challenges him when in reach (`challenge_carrier`), and everybody else is sent to his place in the block (`block_target`). That is the deus ex machina the vision rules out.

## Decision

**Every player, the goalkeeper included, with or without the ball, chooses his own act, at his own pace, from what he perceives. The engine applies the act; it chooses nothing.**

1. **When.** Each player has a latency (the same formula as the carrier's, ADR-044 §6): he chooses again when it expires. Between two choices he carries out the last one.
2. **What he reads** (Plane B, from his own position, ADR-034): the ball and its state, the carrier if any, his place in the block (the role's intention, `shape`), and how many teammates are closer than him to the ball and to the carrier (a visible fact: he sees them).
3. **What he can choose, without the ball** (first set, the list grows by consumer):
   - **Hold**: go to and keep his place in the block;
   - **Press**: run at the opposing carrier;
   - **Challenge**: try to win the ball, only when the carrier is within reach (ADR-042, contact decides);
   - **Go to the ball**: run to a loose ball.

   The goalkeeper's own set: **hold** his position (his positioning rule, the support position included), **come out** for a loose ball in his penalty area, **challenge** a carrier in reach inside his area (the one-on-one: his goalkeeping against the attacker's finishing). He does not press out of his area.
4. **How he chooses.** Deterministic salience (ADR-040), no die: proximity to the target, his rank among his teammates (the nearest goes, the next one covers), and his characteristics (`game_reading`: whether he notices that he is the one who should go; `decision_making`; `tackling` for the challenge). The weights are data (`balance/offball.json`).
5. **Who owns what** (ADR-041). `ds-orchestrator` owns the off-ball choice (`offball`), as it owns the carrier's; `ds-match` builds the scene and applies the act (movement target, the duel); `shape` gives the place, it no longer gives an order.

## Consequences

- `chasers` in `move_agents` and `challenge_carrier` disappear: the engine no longer picks "the nearest".
- Two players can make the same choice (both go to the ball): that is football, and the rank term makes it rare.
- `SIM_VERSION` bump, golden regenerated, P1 tests (the engine never selects a chaser; a player beyond his teammates holds his place; a player in reach of the carrier may challenge).
- Out of this ADR: marking, covering a space, offering for a pass (the next candidates), the coach's instructions (L2) as weights on these choices. The restarts' taker is still placed by the rule (`rules`), not chosen.

## Alternatives considered

- **Keep the engine's nearest-player rule and add instructions later.** Rejected: it is the deus ex machina; instructions would modulate the engine, not the players.
- **A team-level planner that assigns roles each tick.** Rejected: a global decider (foundations §4: "no global decider").

## Dated notes

- **2026-11-12 — accepted, amended.** The maintainer: "every outfield player, the goalkeeper included; for the rest, yes". The goalkeeper's own set of acts is added to §3; the earlier text left him out.
