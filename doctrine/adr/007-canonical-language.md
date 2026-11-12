# ADR-007 — Canonical Language

- **Status:** accepted
- **Date:** 2026-10-25
- **Supersedes:** none
- **Superseded by:** none
- **Blocks:** translation work, language policy in `doctrine/STRUCTURE.md` §5, every module whose language is not yet decided

## Context

The repository currently mixes Italian and English:

- `README.md`, `doctrine/STRUCTURE.md`, `CONTRIBUTING.md` are in English.
- `doctrine/00-foundations.md` and the `L0-match/*` cells are in English.
- The legacy files `01-architecture.md`, `02-registry.md`, `03-pack-system.md`, `04-data-model.md` are in Italian.
- The archived `Prompt_Master_v5.md`, `v6.md`, `v7.md` are in Italian.

This is not sustainable. Contributors do not know which language to use for new content, which version of a document is authoritative, or whether they should translate or write original prose.

The `doctrine/STRUCTURE.md` §5 already states "English is canonical, Italian is a translation", but this was a working assumption, not a ratified decision. This ADR ratifies it.

## Options considered

### Option A — English canonical, translations optional (RECOMMENDED)

- **Pro:** matches the README and the new modules; international reach; single source of truth; Rust ecosystem is English-first.
- **Con:** legacy Italian modules need translation; Italian contributors must write in a second language.

### Option B — Italian canonical, English as translation

- **Pro:** honours the project's Italian roots ("anima PC Calcio"); easier for the original author.
- **Con:** contradicts the README; limits the international community the project claims to want; Rust ecosystem and code identifiers are English anyway.

### Option C — Bilingual, both authoritative

- **Pro:** maximum accessibility.
- **Con:** two sources of truth drift; review cost doubles; ambiguity about which version wins.

## Decision

**Option A.**

### Rules

1. **English is the canonical language** for every file in `specs/`, `docs/`, `doctrine/STRUCTURE.md`, `CONTRIBUTING.md`, and the `README.md`.
2. **Italian is a translation**, stored in `locales/it/`, mirroring the structure of `specs/`. Translations:
   - Carry a header `translation_of: specs/XX-name.md@vX.Y`
   - Are updated by volunteers
   - Never override the English version
   - May lag behind (and say so)
3. **Code identifiers, commit messages, ADRs, and file names** are English.
4. **The archived `Prompt_Master_v*`** stay in Italian. They are historical documents, not maintained. Their existence does not contradict this ADR.
5. **In-game localisation** (UI strings, narrative templates) is separate from documentation. It is handled by `L2-ui` and `L3-assets` and is fully multilingual by design.

### What "translation" means in practice

- A translated file is a **derived work**. The English original is the source.
- The translation carries the same license (`CC-BY-NC-SA-4.0` per ADR-006).
- When the English original changes, the translation is marked `status: stale` until refreshed.
- The `_index.md` lists each module's translation status.

### Legacy migration

The four Italian legacy modules (`01-architecture.md`, `02-registry.md`, `03-pack-system.md`, `04-data-model.md`) are translated to English as part of the migration described in `docs/MIGRATION-01-04.md`. Until translated, they carry a frontmatter field:

```yaml
translation_status: "italian-only (pending translation; ADR-007 accepted)"
```
