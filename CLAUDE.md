# Calcio Vero: istruzioni per Claude alla ripresa

Rispondi in italiano. Lavora in autonomia; chiedi solo per scelte che toccano la dottrina o che il maintainer deve prendere.

## Per prima cosa, a ogni ripresa
1. Leggi `context/STATE.md`: il bookmark ("Last commit verified"), "Dove siamo", il **Prossimo**, la tabella DD.
2. Leggi il piano in corso in `context/plans/` (oggi `CODE-AUDIT.md`, in fondo la "Terza fase").
3. `git status` e `git log --oneline -5`: lavoro su disco non committato va prima verificato e chiuso.

## Regole del progetto (non negoziabili)
- **C2 verde prima di ogni push:** `cargo build --release`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo fmt --all --check`, `ds-test-runner --check`, `gates.sh`.
- **Commit in due passi:** prima il lavoro *senza* `context/STATE.md`, poi il commit del bookmark che porta "Last commit verified" a quell'hash (l'hook rifiuta un commit che tocca STATE.md con il bookmark diverso da HEAD). Niente force-push, niente tag.
- **Cambio di comportamento:** bump di `SIM_VERSION` (`ds-match/src/save.rs`), voce in `CHANGELOG-SIM.md`, golden e semi rigenerati, un test di guardia, misura `ds-reference 42..101` scritta in `L0-CALIBRATION.md`.
- **A5-bis:** mai scegliere un numero guardando la metrica che produce.
- Decisioni delegate: riga DD in STATE.md e trailer `Delegated-Decision: DD-n` nel commit.
- Un crate per responsabilità (ADR-041): formule in ds-calculator, Piano A solo in ds-db/ds-orchestrator (ADR-112), cronaca solo nelle viste.
- Commit con `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`. Mai segreti nei commit.

## Come si chiude L0
Almeno 16 metriche su 22 in banda (media ± 1 DS di ds-reference), e ogni metrica fuori banda spiegata come bug corretto o come taratura dichiarata. Ogni metrica fuori banda si indaga prima come possibile bug. Poi L1 (`context/plans/L1.md`).
