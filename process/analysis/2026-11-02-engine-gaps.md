---
title: "Engine representation — gaps and decisions (2026-11-02)"
version: "1.0"
status: "analysis"
last_updated: "2026-11-02"
conforms_to: "L0-POSITIONAL-SLICE.md §0; doctrine/00-foundations.md; NEW-SESSION-PROMPT.md (D6)"
---

# Engine representation — gaps and decisions

**Scope:** the engine as it is today, compared with a real match (D6 lens).
**Not a plan:** the plan is in `SESSION-STATE.md`. This report records what
was verified and what was decided on 2026-11-02, so the next session does
not have to re-derive it.

## 1. Verified facts (measured, not estimated)

| Fact | Value | How |
|---|---|---|
| Events emitted per match (seed 42) | **8** (4 Shot, 2 Goal, 1 Duel, 1 Foul) | `pcfm-demo --seed 42 --json` |
| Boundary events | 4 (KickOff, HalfTime, SecondHalf, FullTime) | same |
| Actions executed by the engine (estimated) | ~25 (60% pass, 25% shot, 12% duel, 3% foul) | `action_kind_roll` branches |
| **`Pass` events emitted** | **0** | grep: `resolve_pass` is called, no `EventType::Pass` emission |
| `PassContext` | `Copy`, 2 usages | `pcfm-hidden/src/context.rs:39`, `state.rs:125` |
| `nearest_teammates` | `Vec<Neighbour { id, distance_squared }>`, distance from the passer | `perception.rs` |
| "Freedom" of a candidate (distance from nearest defender) | **not computed** | composition of `nearest_opponents` needed |
| `Interception` | in the enum, viewmodel, narrative; **no `resolve_*`, never emitted** | grep |
| Per-tick interface (callback, frame, snapshot) | **absent** | `orchestrator.rs`, `state.rs` |
| Per-agent loop, synthetic (22 agents x 54,000 ticks = 1,188,000 agent-ticks) | **18 ms** (17-19, stable under 4x parallel load) | `/tmp/tickbench`, release; lower bound: no `resolve_*`, no event building, no real perception. The real cost will be in the resolves |
| `PassContext` in spec `10` §2.2 | contexts = observed state + PlayerId; outcomes = observable only | conforms |

## 2. Gaps, by category

**Space:** no direction of play (attack toward), no offside line, no penalty
area, no pitch lines. `Pitch.condition` and `Weather` exist as data, never
read by the engine.

**Agents:** goalkeeper behaves as any agent (`resolve_save` never called);
referee type exists, no agent in the loop; assistants absent; substitutions
absent.

**Events:** `Pass` never emitted (60% of actions invisible); `Interception`,
`Cross`, `Dribble`, `Clearance`, `Save`, `Card`, `Corner`, `FreeKick`,
`Penalty`, `Substitution`, `Injury`, `Pressure`, `ProgressiveCarry/Pass`,
`RandomEvent` — in the enum, none emitted. Out-of-play absent.

**Rules:** no rules engine. No offside, no foul consequence (a `Foul` flips
possession and nothing else), no out-of-play, no advantage. **This is the
structural gap.**

**Relations:** no marking (the defender is "the nearest", not an assigned
player), no coordinated pressing, no coverage, no passing lanes, no
collective shape.

**Time:** no continuous movement (velocity `(0,0)`); no fatigue
(`Player.condition` exists, never read); no momentum; no effective time.

**Conditions:** `Pitch.condition` / `Weather` present, unread. Contradiction
with `WORLD-MODEL` §5 (see §4).

**Consequences:** an event changes possession and score. Nothing else. A
`Card` changes no behaviour; an `Injury` changes no squad; a `Foul` gives no
free kick.

## 3. Conceptual shadows (design not yet decided)

1. **Event vs state is not systematised.** Possession quality = event field
   (decided today); ball `InFlight` = state (decided today). But pressure,
   movement, shape, marking? The criterion "physical presence = causality
   budget" (2026-11-02) helps but is not a taxonomy. Missing: the general
   rule for when a thing is state and when it is an event.
2. **Referee perception.** Referee position = visual layer, whistle =
   normative (2026-11-02). But offside depends on perception (the assistant
   sees the line). If the referee does not perceive, offside is always
   exact — unrealistic. The "visual" decision holds for position, not for
   decisions that depend on perception.
3. **Tick granularity not measured.** 22 agents × 54,000 ticks ≈ 1.19M
   decisions per match. Feasibility unknown. If infeasible, the per-agent
   model (ADR-019 deferred 1) does not hold and the 2D test fails.
4. **Action vs intention.** An action is a roll. In a real match it is
   intention + execution. Receiver choice is the first piece of intention;
   why shoot, why dribble, why press remain rolls.
5. **Direction of play.** Without "attack toward", every action is local and
   purposeless.
6. **Player memory.** No memory between events (a beaten defender is not
   more cautious; a striker who missed is not nervous).

## 4. Decisions taken on 2026-11-02 (to be annotated on the contracts)

| Decision | Contract to annotate | Kind |
|---|---|---|
| Physical presence = causality budget; referee position is visual, whistle normative and position-independent | ADR-019 D9 (dated note) | interpretation |
| Ball has intermediate position (`InFlight` with position) | ADR-019 deferred 3 ("closed for v1.0") | closes a deferred |
| Posts as static obstacles; goal as static data; rebound angle computed (geometric, not physical) | `WORLD-MODEL` §3/§5 (dated note; redefine "physical ball dynamics" = computed from forces) | vision update |
| `Pitch.condition` is a shell (no consumer) or reserved | `03-entities-minimal.md` §2.5 + `WORLD-MODEL` §5 | resolves an existing contradiction |
| Receiver chosen by `hidden` among candidates with freedom; `PassOutcome { receiver, complete }` | `L0-POSITIONAL-SLICE` §3.4 (receiver is no longer "not modelled") | plan update |
| `Interception`: one resolve, two events (third way) | ADR-019 D11 (implementation note) | interpretation |
| 2D ASCII top-down = debug TUI, not primary presentation | ADR-022 (dated note); ADR-025 (motivation: the pitch requires positional L0, not L2) | interpretation |
| Rules = L0 rule module (ADR-019 D11), not L3 plugin; ruleset id in the save; golden per ruleset | IDEA-RULE-001 (content); ADR-019 D11; ADR-013 (consequence) | direction |
| `resolve_stoppage` core, plugin for tuning only | ADR-026 (dated note, when the referee model exists) | direction |
| OI-28 (§3.4 vs §6-ter on movement) is aggravated by the ball decision: close it before the slice | `L0-POSITIONAL-SLICE` §3.4 | contradiction |

## 5. What the 2D test does not cover

The ASCII debug view shows movement and the ball. It does **not** show:
offside, foul consequences, out-of-play, collective shape, fatigue,
psychology. It can pass (frames change) while the engine is still far from
a match. The acceptance criterion "a fan recognises football" must be
limited to "recognises movement". The rules gap is not covered by the
renderer; it is covered by the rules engine, when it exists.

## 6. Not read (D1)

ADR-021 (referee non-physical); `09-context-modifiers.md`;
`08-random-events.md`; `06-match-output.md`; ADR-012 in full (read:
headings + §1-§4); `perception.rs` in full (read: signatures). Some "gaps"
may already be covered by those documents; verify before writing an OI that
says "missing X".
