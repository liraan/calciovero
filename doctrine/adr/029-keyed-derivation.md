# ADR-029 — Keyed derivation for latents and noise

- **Status:** accepted
- **Date:** 2026-11-05
- **Supersedes:** —
- **Superseded by:** —
- **Blocks:** OI-53 (keyed latents, and through it F.7), OI-44 (keyed event noise, when the per-agent loop exists)
- **Depends on:** `L0-match/specs/01-rng.md` §2.4 (seed derivation), ADR-003 (Plane A), ADR-013 (worlds), ADR-028 (dimension vs performance), L0.11 (the player model)

> **Note (2026-11-07) -- noise half revised by ADR-035.** The latents half
> of this ADR stays: `hash(world_seed, player_id, characteristic)` is a
> deterministic function, not a die. The noise half (the per-event stream
> `hash(world_seed, match_id, tick, player_id, purpose)`) is revised by
> ADR-035: perception is derived from characteristics, not noisy by hash.
> OI-44 is therefore absorbed by ADR-035, not implemented as keyed noise.

## Context

Calcio Vero is a deterministic football simulation: the same save, the
same match and the same configuration produce the same result. Two kinds
of data are generated at random today, and both are generated the same
way: **in sequence**, from a stream.

1. **The latent performances of a player** (`pcfm-hidden::PlayerLatents`):
   seven `f32`, each drawn by `next_unit` in order, when the player is
   first encountered. `finishing` is the first draw, `passing` the
   second, and so on.
2. **The noise of a resolution** (`resolve_duel`, `resolve_shot`, ...):
   each function draws from its own stream (`:duel`, `:shot`, ...),
   named per purpose (ADR-019, A1), so adding a function no longer
   shifts the others.

**The defect, in a real match.** In a real match, a player's strength
does not depend on how many other players were generated before him, and
the luck of a pass by X does not change because Y shot first. In the
current model, both do:

- **Latents.** Adding a characteristic consumes one more draw per player,
  so every player generated after the first ends up with different
  values. F.7 hit this: adding `strength` shifted the golden hash of every
  match, even though `strength` was not read by anything yet.
- **Noise.** The per-agent loop (planned) makes 22 agents decide every
  tick. The noise an agent receives depends on its position in the
  iteration order: a substitution changes the number of agents, hence the
  order, hence the noise of everyone after. OI-44 records this.

Both are the same defect: **a value that belongs to a player (or to an
event) is derived from a sequence, not from the player (or the event).**

## Decision

### 1. Derive by key, not by sequence

A value that belongs to a player is derived from the player. A value that
belongs to an event is derived from the event.

The hash function is the one already used in `01-rng.md` §2.4
(`blake3`), so the derivation is the same mechanism, extended with more
key material.

### 2. Two keys, because the two values belong to different things

| Value | Key | Why |
|---|---|---|
| **Latent performance** | `hash(world_seed, player_id, characteristic)` | A player's strength is the same in every match of his world. It does not depend on the opponent, the date, or the order of generation. |
| **Event noise** | `hash(world_seed, match_id, tick, player_id, purpose)` | Two events of the same player in different matches (or different ticks) must have different noise. The order of iteration must not matter. |

**The latent key has no `match_id`.** This is the point where OI-53 as
written was wrong: it proposed the same key for both, including
`match_id`. But a player's strength does not change when the opponent
changes. Including `match_id` would give the same player a different
strength in every match, which is false on a real pitch and breaks the
persistence model (L0.11 §11.5: characteristics are rebuilt from the
seed, not per match).

**The world is identified by `world_seed`** (ADR-013). No `world_id` is
needed: two worlds with different seeds are different worlds, and the
seed is the key material.

**`purpose` is the characteristic name for latents** (`finishing`,
`passing`, ...) and the resolution name for noise (`duel`, `shot`, ...).
It keeps the values independent: adding a characteristic does not shift
the others, adding a resolution does not shift the others.

### 3. What this changes in the code

**Latents.** `PlayerLatents::from_rng` disappears, replaced by a
derivation per characteristic:

    strength(player) = unit_from_hash(world_seed, player_id, "strength")

The same call, for the same player and the same world, returns the same
value, regardless of when it is called and how many players were
generated before.

**Noise.** `resolve_*` stops consuming a stream state and derives its
noise from the event:

    noise = unit_from_hash(world_seed, match_id, tick, player_id, purpose)

This half is **decided here and implemented later**, when the per-agent
loop exists (D3: the consumer comes first). Until then, `resolve_*`
keeps its streams.

### 4. What this is not

- It is not a change to `RngRegistry`. The registry stays for the streams
  the engine still uses (action selection, pacing). The derivation is
  local to `pcfm-hidden` and to the calculator.
- It is not a change to the algorithm (ChaCha20 stays where a stream is
  used) or to the hash (`blake3` stays).
- It is not a change to the save: `GroundTruth` is still reconstructed
  from `(world_seed, match_id)`, and now the reconstruction is cheaper
  and order-independent.

## Consequences

- **Adding a characteristic no longer shifts the others.** The golden hash
  changes **once**, when this ADR is implemented (every latent changes,
  because the derivation changes), and then it is stable: adding a
  characteristic that no formula reads does not move it.
- **F.7 is unblocked.** The prerequisite was OI-53; with this ADR the
  derivation is decided, and F.7 can be implemented on top of it.
- **OI-44 is decided, not implemented.** The noise half waits for the
  per-agent loop. The ADR is the decision; the leaf is the per-agent.
- **`SIM_VERSION` bump required.** Every match changes, because every
  latent changes. `CHANGELOG-SIM` entry with the derivation and the two
  keys. The golden is regenerated once.
- **Determinism is strengthened, not weakened.** Today a value depends on
  the order of generation and on the iteration order; with the key it
  depends on the player and on the event. The same save, the same match,
  the same result — with fewer hidden dependencies.
- **Saves.** `GroundTruth` is reconstructed, not saved; nothing changes in
  the save format. But every save created before this ADR will produce a
  different result when replayed, because the latents are different:
  `can_replay()` must return false across the bump, as it already does
  for `SIM_VERSION` changes.

## Alternatives considered

- **Keep the sequence, reorder the fields so `strength` is last.** It
  postpones the problem: the next characteristic added shifts everything
  again. And it does not fix the noise (the iteration order still
  matters).
- **One key for both, with `match_id`.** Rejected: a player's strength
  would change per match, which is false and breaks persistence.
- **One key for both, without `match_id`.** Rejected for noise: two
  events of the same player at the same tick in different matches would
  get the same noise, which is not a problem for latents but is wrong for
  events.
- **Two ADRs, one for latents and one for noise.** Rejected (D7): it is
  one model change, with two keys. Two ADRs would duplicate the decision
  and let the two halves drift. One ADR, two sections, two leaves.
- **A `world_id` in the key.** Rejected: `world_seed` already identifies
  the world (ADR-013). A `world_id` would be a second identifier for the
  same thing.

## References

- `L0-match/specs/01-rng.md` §2.4 (seed derivation, `blake3`), §2.3
  (stream naming, per purpose)
- ADR-003 (Plane A: `GroundTruth` is not `Serialize`, only `resolve_*` as
  door)
- ADR-013 (worlds: local determinism, the `world_seed` identifies the
  world)
- ADR-028 (dimension vs performance: the latents are performances, Plane
  A)
- `L0-match/specs/11-player-characteristics.md` L0.11 (the player model;
  §4.11 already names OI-53 as a prerequisite; §11.5 the persistence
  table)
- OI-44 (event-keyed noise: the defect for the per-agent loop)
- OI-53 (keyed latents: the defect F.7 hit; this ADR corrects the key it
  proposed)
