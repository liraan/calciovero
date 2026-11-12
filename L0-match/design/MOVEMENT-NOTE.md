# Movement — design note

**Status:** design note (not a spec, not an ADR). **§4 is implemented** (`move_agents`, F.4-a); **§3 is still direction** (influence not in the movement).
**Date:** 2026-11-03.
**Closes (design):** OI-28, B1 of the 2026-11-03 session.
**Depends on:** `L0-POSITIONAL-SLICE.md` §2/§7 (movement is not in slice C), `IDEA-DESIGN-001` (influence as the common currency), ADR-019 deferred 1 (iteration order).
**Note (2026-11-03, after the analysis) — §3 and §4 are direction, not an executable plan.** §3 ("per tick, per agent") requires the per-agent loop and OI-44 (event-keyed noise) decided; §4 ("slot default, most-influential goes to the ball") requires the ball to move. Without a moving ball, "move only the acting agent" produces a clump at the centre that freezes once the target is reached (verified in the 2026-11-03 analysis): rejected, it is not football. A variant with the target = slot is a no-op (agents start on their slots). **Decision (G):** movement is not implemented now. Sequence: OI-44 (ADR event-keyed noise) -> per-agent -> movement of all 22 + ball slice. The code written on 2026-11-03 was for the rejected model ("move only the acting agent") and was lost with `/tmp`: it was not committed (D3). It is rewritten in the per-agent, where the shape is right; not lost work, moved work. **Do not implement §3/§4 from this note alone**; read OI-44 and the bookmark first.

**Note (2026-11-04, after F.4-a) — what is implemented is §4, not §3.** `move_agents` exists (`spatial.rs`, F.4-a, MD-09, MD-12) and implements §4: the formation slot translated by a fraction of the ball offset, the nearest agent deviated to the ball, a bounded step, and a minimal separation. It does **not** implement §3: the target is not the point of highest influence (`resolve_influence` is not called in the movement). So the "proposal" that became code is the placeholder (§4, declared as such), not the model (§3, still direction). This is not a bug: it is a simplification that had never been written down as one, and it is now. Consequences: (a) §4 is documentation of the behaviour, not a proposal; (b) §3 remains direction, open, and its first action is the same as before — decide whether influence enters the movement, with the per-agent loop and OI-44; (c) the death condition of §6 applies to §4 (implemented), not to §3 (not implemented): this note does not die whole, its off-ball part does. The cost measured on the code (not the note): release 87 ms/match, golden debug 5.48 s (OI-51).

## 1. Why a note, not code

Movement was listed in `L0-POSITIONAL-SLICE.md` §3.4 as part of slice C, but slice C is `Slip` + `ControlError` + `Pressure` (§2, §7): no movement. The contradiction (OI-28) is resolved by this note, not by code: **movement is a precondition of the ball slice, not a slice of its own**. A slip is recognised because someone was moving and fell; without movement, `Slip` is a label on a statue.

This note is **not** a spec (it does not define types or a wire format), **not** an ADR (it takes no irreversible decision), **not** code. It is the design that the code will implement, so that the next session does not invent it while writing `tick`.

## 2. What the engine must explain (D6)

Each row is a sentence a fan says about a real match, the observable input it requires, and the event it produces. If the engine cannot produce the sentence, movement is incomplete.

| Sentence | Observable input | Event |
|---|---|---|
| "He slipped on the wet grass and lost the ball" | surface condition, current velocity | `Slip` |
| "He got there first" (loose ball) | distance to the ball, arrival order | possession change |
| "The press arrived, he rushed the pass" | distance to nearest opponent, closing speed | `Pressure`, `ControlError` |
| "He cut inside" (off-ball) | direction of play, space | movement, `Pass`/`Shot` after |
| "He tracked back" (off-ball) | team shape, ball position | movement, `Tackle`/`Interception` after |

Rows 1–3 are on-ball movement and are covered by slice C + the ball slice. Rows 4–5 are off-ball movement and are the largest part of a real match; §4 says what the engine will do about them today (very little, declared).

## 3. Minimal model (direction — NOT implemented; §4 is what the code does)

State: `AgentState { position: (i32, i32) cm, velocity: (i16, i16) cm/tick, heading: u16 }` (slice A, already in `spatial.rs`). No physics, no collision, no biomechanics: the position is updated by a deterministic function, not by integration of forces.

Per tick, per agent:
- **Target:** the point where the agent's weighted influence is highest, or its formation slot if none is reachable. Influence is a query on `pcfm-hidden` (`resolve_influence`), already exposed as an observable order (`Vec<PlayerId>`); it does not see latents.
- **Step:** move toward the target by a distance bounded by a declared speed (cm/tick). The step is an integer, per ADR-019 D1.
- **Velocity:** derived from the step, stored, and **read** by `Slip` (surface x velocity x balance), not written by it.

**Where the variation comes from (S1).** The movement is deterministic: same position, same influence, same slot = same step. The variation a fan sees (a hesitation, a wrong angle, a late arrival) comes from **observable inputs**: fatigue (a fatigued agent moves less), influence (a contested point moves the target), direction of play (an agent without direction drifts). The RNG is **not** in the movement: a slip is an event that reads the velocity, not a dice roll inside the step. If a future model needs stochastic movement (e.g. a stumble without a slip), it gets a stream `hidden:{id}:movement` **when the model exists** — the convention of `01-rng.md` §2.3 allows adding it without shifting the others.

**Per-tick or per-event (S2).** Per-tick. 22 agents x 54,000 ticks is the cost already measured (18 ms, `/tmp/tickbench`, release, **without** real `resolve_*`); the real cost must be re-measured when the resolves are in the loop. A per-event model would show a match where players teleport between events; that is not football, and it would make the 2D debug view meaningless.

**Relation to the tick record (IDEA-INFRA-018).** Per-tick movement is exactly what the tick record indexes. The record is regenerated from the seed by replay; the live streaming (a separate decision, still open) reads the same per-tick positions. No renderer talks to the engine: the 2D debug view is a projection of the tick record, not a reader of `MatchState`. This is the boundary decided on 2026-11-02, and the movement model is what makes it testable ("frames change at every tick" is the acceptance criterion).

**Cost (S3).** Influence is a per-agent query: 22 queries per tick, 1.19M per match. It is the most expensive part of the model and it is declared here, not hidden. If the measurement shows it is too expensive, the fix is a cheaper formula or a precomputed field — a structure with content, decided with data, not before (D3).

## 4. Off-ball movement (implemented as placeholder, F.4-a)

Today there is no off-ball model. The minimum that is **honest** (declared as a placeholder, not as a model):

- **Default:** an agent without the ball returns toward its formation slot. This is not football; it is the baseline that makes the pitch not collapse to the ball.
- **Exception:** if the agent is the most influential on the ball, it moves toward the ball. This is what slice B2 already does for *who acts* (nearest to the ball), extended to *who moves*.
- **What is missing:** marking (a defender assigned to an attacker, not to the ball), coverage (closing a passing lane), collective transitions (the team moves as a block, not as 11 individuals). These are §5 items: they need L2-tactics and a shape model, neither of which exists.

This section is what makes OI-27 concrete: the contract between L0 and L2 on collective shape is "the engine reads a declared default; L2 replaces it". The default is a placeholder, declared, not a model.

## 5. What stays out, and when it comes back

| Piece | Why out | When |
|---|---|---|
| Collisions between agents | No collision model, no data | After the 2D debug view shows overlapping agents |
| Avoidance / pathfinding | The pitch is open; the obstacle is the opponent | With the first marked attacker (L2-tactics) |
| Collective transitions | A shape is a relation, not an agent | With `L2-tactics` (OI-27) |
| Fatigue narrowing the field | `PhysicalStateMinimal.fatigue` exists, unread | With `ContextModifiers` (L2-psyche) |
| Goalkeeper movement | The goalkeeper is an agent like any other today | With `resolve_save` in the loop |
| Stochastic movement (stumble without slip) | The model does not exist | When the model exists, with a `:movement` stream |

## 6. When this note dies

The note dies when the code exists. Concretely: when `tick` moves agents per §3 and the tests in §8 of `L0-POSITIONAL-SLICE.md` pass, this note is reduced to the module documentation of the movement code, and the design sections (1, 4, 6) are deleted. A design note that outlives its code becomes a second source of truth, and the second source is always the stale one (see `ARCHITECTURE.md` §3, V5 in the 2026-11-03 report: it still cites `pick_players`).

The note dies **also** if the movement model changes direction (e.g. physics-based, or event-only): in that case the note is superseded by the new design, not edited.

## 7. Open decisions (for the session, not for the note)

- **S1 — where the variation comes from:** answered above (observable inputs; no RNG in the step). Open: is fatigue part of the step today, or only when `ContextModifiers` exists?
- **S2 — per-tick vs per-event:** answered above (per-tick). Open: the cost must be measured with real resolves.
- **S3 — influence per-agent vs precomputed:** answered above (per-agent, declared cost). Open: the measurement decides, not the note.
