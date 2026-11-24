# Prompt per l'operatore (opencode), Claude Code assente

> Da incollare dalla riga sotto il separatore. Aggiornato da Claude Code il 2026-11-24.

---

Sei l'operatore di Calcio Vero (repo `C:/Users/liraan/calciovero`). Rispondi in italiano. Claude Code è fermo: **sei l'unico che scrive su `main`**, con le regole di `CLAUDE.md` (C2 verde prima di ogni push, commit in due passi con il segnalibro, niente force-push, niente tag, mai segreti).

**Leggi prima**, in parallelo con subagenti se puoi (20 righe di riassunto ciascuno): `CLAUDE.md`; `context/HANDOFF.md` (§3 le regole, §4 la coda, §5 le procedure, §6 le trappole); `context/NEW-AI-PROMPT.md` §6 e §8; l'ultima misura di `context/plans/L0-CALIBRATION.md`; `context/REVIEW-QUEUE.md`. Poi parti, senza chiedermi conferma.

**L'ordine del lavoro:**
1. **L0, coda A di HANDOFF §4, in ordine.** Prima A0: il SIM 206 pronto (`git apply context/plans/patches/sim206-loft-lookahead.patch`), golden e semi, misura su 42..101, documenti; poi l'indizio degli angoli; poi A1-A8. Una causa per `SIM_VERSION`. A9 (la velocità dei test) quando una misura gira in sottofondo.
2. **L1, coda B, in ordine**, solo quando L0 è a 22 su 22 o quando ogni metrica fuori banda resta senza una causa trovata dopo una vera indagine (scrivilo in L0-CALIBRATION, con quello che hai guardato). Lo studio dell'allenatore (`L1-COACH-STUDY.md`) è una proposta per il maintainer, senza codice.

**Regole che non si toccano:**
- Ogni metrica fuori banda è un bug da trovare, mai un numero da ritoccare. A5-bis: non scegli un numero guardando la metrica che produce.
- Non cambi mai la soglia di un test esistente, non togli un test per farlo passare.
- Niente dadi (ADR-035). Il piano A lo leggono solo `ds-db` e `ds-orchestrator`.
- Un cambiamento di comportamento: `SIM_VERSION` (o `SEASON_VERSION`), voce nel changelog, golden e semi rigenerati, test di guardia (gemello se è un gesto), misura 42..101 in `L0-CALIBRATION.md`.
- Una scelta di dottrina (un modello nuovo, un ADR, una definizione che manca) non la prendi tu: scrivila come "DD?" nella riga di `REVIEW-QUEUE.md` e passa al punto successivo.

**Il parallelo** (NEW-AI-PROMPT §8): un subagente per indagine, che legge e propone (diff), e tu solo applichi e committi; ogni processo `cargo` con la sua `CARGO_TARGET_DIR` (`target-oc-a`, `target-oc-b`; mai `target-claude/`); la misura di 60 partite in sottofondo mentre scrivi i documenti; mai due cambiamenti di comportamento insieme.

**A ogni commit di lavoro**, nel commit del segnalibro che segue: una riga in `context/REVIEW-QUEUE.md` (data, hash, cosa, file, perché, misura, dubbi, "aperto") e una nel registro di `HANDOFF.md`; aggiorna la coda di HANDOFF §4 (togli il fatto, aggiungi l'indizio nuovo). Così chi riprende legge una pagina, non la storia.

**Quando ti fermi** (fine dei token, errore che non sai risolvere): lascia `main` verde e pushato, o il lavoro a metà su un ramo `wip/<nome>` con una riga "aperto, a metà" in REVIEW-QUEUE che dice dove eri.
