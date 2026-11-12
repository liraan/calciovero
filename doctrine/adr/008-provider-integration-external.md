# ADR-008 — Provider Integration Is an External Tool

- **Status:** accepted
- **Date:** 2026-10-25
- **Supersedes:** none
- **Superseded by:** none
- **Blocks:** none

## Context

The original plan included a module (`L2-ingest`) for integrating professional data providers (Wyscout, StatsBomb, Opta) into the core. On review, that plan conflicts with the core principles of the project:

- The core is **offline-first**. A module that talks to Wyscout over the network is not offline-first.
- The core is **sandboxed**. A module that reads credentials and calls external APIs is not sandboxed.
- The core is **lightweight**. Provider integration is a large, provider-specific, license-sensitive feature. It is not part of "can two teams play a match".

The original `L2-ingest` cell was also the most speculative cell in the specification. It would have been designed and written before the core existed, and it would likely have needed a rewrite once the core was in place.

## Decision

**Provider integration is an external tool, not a core module.**

The core does not know that Wyscout exists. It does not read credentials. It does not make network calls. It does not model rate limits or quotas.

Instead:

1. A **tool** (written by the team, by the community, or by a third party) talks to the provider.
2. The tool produces a **pack** (the format is defined in `L3-packs`, planned) containing normalised data: squads, players, competitions, events.
3. The user installs the pack through the normal pack installation flow.
4. The core reads the pack like any other pack.

This is consistent with the existing decisions:

- The core is offline-first (ADR-002, `00-foundations` §2).
- The core uses packs for extension (`L3-packs`, planned).
- The core does not bundle licensed data (`00-foundations` §5).

The tool is responsible for:

- Respecting the provider's rate limits and quotas.
- Handling credentials (keystore local, never in logs, never in the pack).
- Respecting the license (the user must have a valid license; the tool must refuse to activate if the user has not confirmed).
- Normalising data to the core's schema.

The core is responsible for:

- Validating the pack.
- Quarantining a pack that fails validation.
- Providing the schema (in `L1-roster`, `L1-world`, `L1-season`, when they are written).

## Consequences

### Easier

- The core stays offline-first, sandboxed, and lightweight.
- The project does not need to choose a provider, negotiate a license, or maintain an adapter.
- A provider that closes cannot break the core. The tool is updated, the core is not.
- A community member who has a license can write their own tool. The project does not need to ship it.

### Harder

- The user who wants Wyscout must run a tool, get a pack, and install it. It is not "click and import". This is acceptable for a serious management game.
- The project does not ship a reference tool. The user must find one, or write one. This is a deliberate trade-off: it keeps the core clean.

### Locked in

- The core has no network code.
- The core has no credentials.
- The core has no provider-specific logic.
- Provider integration is a tool + a pack.

## References

- `00-foundations.md` §2 (offline-first), §5 (no bundled commercial data), §7 (non-goals)
- `doctrine/STRUCTURE.md` §9 (what packs can and cannot extend)
- `L3-packs` *(planned)*
- `doctrine/_index.md` (the row that was `L2-ingest` is now `🚧 external tool`)
