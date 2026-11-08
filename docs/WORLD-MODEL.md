# World Model — PCFM

**Purpose:** a map of everything that can exist in a match, organised by level and by category. It is a **vision document**, not a contract. It gives every element a place, so that future decisions do not have to invent one.

**Status:** living document (update when the vision changes).

**See also:** `docs/ARCHITECTURE.md` (how the crates interact), `docs/adr/019-match-engine-contract.md` (the contract), `specs/00-foundations.md` §9 (concentric levels).

---

## 1. Scope

This document answers three questions:

1. **What levels exist?** (L0 match, L1 season, L2 career, L3 presentation)
2. **What categories of things exist?** (agents, physical objects, modifiers, processes)
3. **What belongs to v1.0, to 2D, to post-v1.0?**

It does **not** specify how any of it is implemented. That is the job of `specs/` and `docs/adr/`.

---

## 2. Concentric levels (L0–L3)

The project is organised as **concentric levels** (see `00-foundations.md` §9). Each level expands the previous one without replacing it. Dependencies flow **inward**: a level N may depend on levels `< N`, never on `> N`, never laterally.

| Level | What it is | Contains | Depends on |
|---|---|---|---|
| **L0 — Match** | One match. Deterministic, savable. | Engine, 22 players, ball, referee, synchronous rules (offside, fouls) | — |
| **L1 — Season** | A league, a calendar, a world. | Standings, fixtures, nations, cities, stadiums (generated) | L0 |
| **L2 — Career** | The management game. | Market, finance, youth, scouting, psyche, **crowd** (ADR-018), media (v1.1) | L0, L1 |
| **L3 — Presentation** | How the game is shown. | UI, **3D rendering**, sound, replay, **visual weather**, **crowd rendering**, commentary | L0, L1, L2 (read-only) |

**Rule:** L3 reads L0–L2. L0–L2 never read L3. No game rule lives in the interface (doctrine, pillar 2).

---

## 3. Taxonomy (seven categories)

Everything that can exist in a match falls into one of these categories. The category determines **what it can do** and **where it lives**.

| Category | Physical presence? | Decides? | Examples | Horizon |
|---|---|---|---|---|
| **Competitive agents** | Yes (in the pitch) | Yes (play) | 22 players | v1.0 |
| **Normative agents (in-pitch)** | Yes (in the pitch) | Yes (rules) | Referee | v1.0 (position) / 2D (full physics) |
| **Normative agents (peripheral)** | No (along touchlines) | Yes (rules) | Assistant referees | v1.0 (abstract) / 2D (position) |
| **Normative processes** | No | Yes (rules, delayed) | VAR / review | post-v1.0 |
| **Physical objects** | Yes | No | Ball, goal (posts + crossbar + net), pitch lines, corner flags | v1.0 (ball, goal line) / 2D (posts, net, flags) |
| **Global modifiers** | No | No (parameters) | Gameplay weather (rain, wind), pitch condition, temperature | v1.0 (basic) / post (advanced) |
| **Scenic entities** | Yes (aesthetic) | No | Benches, technical area, corner flags (visual) | 2D |

**Excluded by decision:** drones (they clutter the view of the pitch), sensor-in-the-ball (unnecessary: the event timestamp is already the moment of contact).

---

## 4. Simulation vs presentation

The most important distinction in this document. **Everything that influences the result is simulation. Everything that only shows the result is presentation.**

| Aspect | Simulation (L0–L2) | Presentation (L3) |
|---|---|---|
| **Where it lives** | `pcfm-match`, `pcfm-stats`, `pcfm-hidden` | UI, renderer, sound engine |
| **Can read Plane A?** | Yes (via `resolve_*` only) | **Never** |
| **Affects the result?** | Yes | No |
| **Deterministic?** | Yes (local determinism, ADR-013) | No requirement (frame rate varies) |
| **Examples** | Player positions, ball state, events, crowd atmosphere value | 3D meshes, textures, camera, HUD, sound |

**Rule (2026-11-03) — the renderer does not derive events from positions.** A renderer shows the events the engine emitted and uses positions as the background. If two agents are close in the 2D or 3D view, that is **not** a `Tackle`: a `Tackle` exists only if the engine emitted it. A renderer that infers events from geometry invents football the engine did not simulate, which is the opposite of radical transparency (pillar 5). This is the shared rule for 2D and 3D, and the acceptance criterion of the debug view: frames change, events are not invented.

### Two "weather" concepts

- **Gameplay weather** (L0/L1, a modifier): rain makes the pitch slippery, wind curves the ball, heat drains stamina. It **changes the simulation**.
- **Visual weather** (L3, rendering): raindrops on screen, grey sky, puddles on the 3D pitch. It **shows** the gameplay weather but does not affect the result.

The two are **separate**. The renderer reads the gameplay weather and renders it; it cannot invent weather that the simulation does not have.

> **Note (2026-11-08) — the gameplay half is out of the new engine.** T5 (2026-11-03) excludes weather and pitch wear from the new engine, so the first bullet above describes the prototype. The second (visual weather, L3) remains a projection question, defined when a renderer exists. The 2026-11-05 note in §5 covers the horizon table; this note covers the two concepts themselves.

### Two "crowd" concepts

- **Crowd content** (L2, `L2-crowd` per ADR-018): attendance, atmosphere value, impact on players (when implemented). It **changes the simulation** (or at least the context).
- **Crowd rendering** (L3): 45,000 animated people in the stands, chants, flags. It **shows** the crowd content but does not affect the result.

The two are **separate**. A 3D stadium with 45,000 people is rendering; the crowd's atmosphere value is `L2-crowd`.

---

## 5. Horizons (v1.0 / 2D / post-v1.0)

| Horizon | What it includes | What it does not include |
|---|---|---|
| **v1.0** (L0 positional) | 22 competitive agents, ball as entity, referee with position (synchronous rules), abstract assistants, basic gameplay weather, goal line as boundary, synchronous offside | VAR, sensor, physical ball dynamics, net as constraint, pitch condition, full referee physics, assistant positions, 3D rendering |
| **2D** | Ball with physics, posts and crossbar as obstacles, net as constraint, referee with full physics, assistant positions along lines, pitch condition (wet/dry), visual weather, 3D rendering prototype | VAR process, advanced weather (wind curves ball with physics), full crowd simulation |
| **post-v1.0** | VAR (`VARReview` event delaying the decision when uncertain), advanced weather, crowd simulation (ADR-018, v1.1), full 3D, commentary, replay, spectator mode | — |


**Note (2026-11-02) — ball, posts, pitch condition.** This document is a vision, updated when the vision changes; the table above stays as the record of the earlier horizon, this note records the change.

- **"Physical ball dynamics" is redefined** as *computed from forces* (friction, gravity, spin, bounce integration). A **declared geometric rebound** (the ball travels, hits a static obstacle, leaves at a computed angle) is **not** physical dynamics: it is deterministic geometry, and it is admitted at v1.0.
- **Posts and crossbar are static obstacles at v1.0** (moved earlier from 2D). The goal is a static field datum; the rebound angle is computed. Rationale: the ball has an intermediate position (see below), and "hit the post" is an event a fan recognises (D6).
- **`Pitch.condition` stays reserved at v1.0**: the field exists, no consumer reads it. It is read when the ball's movement model uses it (wet/dry affecting the roll).
- **The ball has an intermediate position at v1.0** (`InFlight` with position): it is not deferred to 2D. It is persistent state, saved, in the hash.

The 2D horizon keeps "ball with physics" in the force sense: a renderer or a future engine that integrates trajectories. The difference between the two is *computed from forces* vs *declared geometry*, and it is the same distinction as `L0-POSITIONAL-SLICE` §0: effects are modelled, not simulated.
**Note (2026-11-03) — the 3D is a projection, not a physics engine.** The 3D view (post-v1.0) consumes the same tick record as the 2D view; it is a new projection of the same phenomenological engine, not a new engine. "Physical ball dynamics" stays defined as *computed from forces* (§5 note, 2026-11-02), and it is **not** on the roadmap: a physics engine is a different paradigm, not a later step (`L0-POSITIONAL-SLICE` §0). Adding physics would require an ADR that changes the doctrine, not a rendering decision. Recorded so that "for the 3D we need physics" cannot be introduced as an implementation detail.


**Note (2026-11-05) — weather and pitch condition are out, and the pitch is geometry.** Two points in the table above and in the 2026-11-02 note are stale.

- **Gameplay weather is out of the new engine.** The table lists "basic gameplay weather" in v1.0; T5 (`SESSION-STATE.md`, decision of 2026-11-03) excludes weather and pitch wear from the new engine. Weather is an input to perception and execution in the *old* prototype; the new engine does not model it yet, and the first phenomena (the ball travelling, the geometric goal, the save) do not depend on it. It is not "reserved": it is out, and it will be reintroduced with an ADR if and when a consumer exists.
- **`Pitch.condition` is not "reserved" either.** The 2026-11-02 note says it "stays reserved at v1.0: the field exists, no consumer reads it". T5 decides that pitch wear is out. Moreover, in the new engine the pitch is **only geometry**: a grass surface with `length_m` and `width_m`, on which the actors run and the ball rolls. It has no condition and no weather. The prototype's `Pitch` has four fields; the new engine inherits two.
- **What this note does not change.** The horizon table stays as the record of the earlier vision; the ball, the posts, the `InFlight` position and the 3D-as-projection notes all stay valid.

**Rule:** a horizon is not a promise. It is a place to put things so that the contract does not have to grow to accommodate them prematurely.

---

## 6. What this document is not

- It is **not** a contract. The contract is `docs/adr/019-match-engine-contract.md`.
- It is **not** a specification. The specification is `specs/`.
- It is **not** a promise of implementation. Some of it may never be implemented.
- It is **not** a briefing. Briefings live in `docs/context/`.

It is a **map**. If something new appears, it gets a place here first, then (if it needs a contract) an ADR, then (if it needs a spec) a module.
