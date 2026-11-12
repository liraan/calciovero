---
title: "Repo description and external pages"
version: "1.0"
status: "reference"
last_updated: "2026-11-07"
conforms_to: "README.md (the source), OI-69 (donate), the 2026-11-07 session"
---

# Repo description and external pages

**What this is.** The texts that live *outside* the repository — GitHub
"About", Ko-fi bio, Ko-fi frontpage — kept here so the next session does
not rewrite them from scratch. The README is the source: if a text here
diverges from the README, the README wins (D1).

**Not a public page.** GitHub and Ko-fi render their own copies. This
file is the artefact, not the source.

---

## GitHub "About" (description, variant b)

> Calcio Vero — an open, offline-first football management game. Deterministic match engine, a community alternative to commercial football manager games. Human-directed, AI-assisted: the design, the doctrine and every decision are human; the AI proposes and executes under explicit rules, written in the repo.

**Tags (10):** `football` · `football-manager` · `football-simulation` · `football-game` · `simulation` · `game-engine` · `rust` · `deterministic` · `offline-first` · `open-source`

---

## Ko-fi bio

> Calcio Vero — an open, offline-first football management game. Deterministic match engine, community alternative to commercial FM games. Human-directed, AI-assisted. Donations fund development time; they do not buy influence over the roadmap.

---

## Ko-fi frontpage (chosen: version 1, no Three Planes)

**Calcio Vero — Deep Simulator**

An open, offline-first football management game. Transparent by design. Deterministic by construction. Yours by default.

Not a Football Manager clone: where commercial games optimise for licensed fidelity, Calcio Vero optimises for explainability and ownership.

No telemetry. No closed data feeds. No hidden CA/PA scalars. Every formula is public, every decision is in an ADR, every save is reproducible from its seed.

Code and doctrine: https://github.com/liraan/calciovero

---

## What was rejected, and why

- **Ko-fi frontpage v2** (Three Planes instead of the no/yes list): more elegant, but a cold reader learns less. The Three Planes answer "how does it work inside", which belongs in the README, not on a landing page.
- **Ko-fi frontpage v3** (the full README): too long, Ko-fi cuts around 700 characters.
- **"free forever"** in any text: no artefact backs it. `LICENSE-MIT` / `LICENSE-APACHE` exist, `"free and open"` is descriptive; "forever" is a promise.
- **An embedded Ko-fi RSS feed on the project page:** Ko-fi does not support embedding an external RSS feed on the donation page. Rejected on facts, not on preference; re-propose only if Ko-fi adds the feature and the reason is declared.
- **The Ko-fi badge: text-only and the official Ko-fi badge were both considered, shields.io was chosen.** Text-only is less visible in the README header; the official badge is heavier and forces Ko-fi colours that clash with the neutral README palette. The shields.io badge keeps the header consistent and reads as "supported" without decorating.

---

## Thank-you message to donors

**Context.** Aligned with `FUNDING.yml` and `## Support` in the README:
one-off donations, no goods or services in return, no influence over
the roadmap. No amounts, no donor names, no promised recognition: a
public thank-you with names would weaken "donations do not buy
influence", and the repo has no artefact that governs that visibility.

### Short (automatic thank-you after a donation)

> Thank you. Calcio Vero is built in the open, one commit at a time, and
> your donation buys exactly one thing: time to keep building it. It does
> not buy influence over the roadmap — the doctrine in the repo decides
> that, not donors. If you want to follow the work, the commit log is
> public: https://github.com/liraan/calciovero

### Medium (public post on Ko-fi)

> **Thank you.**
>
> Calcio Vero is built in the open, one commit at a time. Every donation
> buys exactly one thing: time to keep building it. Nothing else — not a
> feature, not a priority, not a place on the roadmap. Those are decided
> by the doctrine in the repository, which is public and the same for
> everyone.
>
> What you are supporting is a different way of building a football
> management game: no telemetry, no closed data, no hidden CA/PA, every
> formula public, every decision in an ADR, every save reproducible from
> its seed. It is slow work, and it is done in the open.
>
> If you want to see where the time goes, the commit log is the honest
> answer: https://github.com/liraan/calciovero
>
> — the developer

### Italian

> **Grazie.**
>
> Calcio Vero si costruisce allo scoperto, un commit alla volta. Ogni
> donazione compra esattamente una cosa: il tempo per continuare a
> costruirlo. Nient'altro — non una funzionalità, non una priorità, non un
> posto nella roadmap. Quelle le decide la dottrina nel repository, che è
> pubblica e uguale per tutti.
>
> Quello che stai sostenendo è un modo diverso di fare un gestionale
> calcistico: niente telemetria, niente dati chiusi, niente CA/PA
> nascosti, ogni formula pubblica, ogni decisione in un ADR, ogni
> salvataggio riproducibile dal suo seed. È lavoro lento, e si fa allo
> scoperto.
>
> Se vuoi vedere dove finisce il tempo, il commit log è la risposta
> onesta: https://github.com/liraan/calciovero
>
> — lo sviluppatore

### Caveat

Whether Ko-fi offers a custom post-donation thank-you message was not
verified (D1): if the field exists, use the short; if not, the medium
goes as a public post.
