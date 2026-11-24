# Passaggio di consegne: continuare il lavoro senza Claude Code

**Cos'è.** Le istruzioni per un operatore (opencode o un'altra IA) che continua il lavoro quando Claude Code non c'è (token finiti, sessione chiusa). Si legge insieme a `CLAUDE.md`, `context/RULES.md` e `context/STATE.md`; il prompt per presentarsi è `context/NEW-AI-PROMPT.md`, ruolo "operatore". Claude Code lo tiene aggiornato a ogni passo; l'operatore aggiunge la sua riga al registro in fondo.

## 1. Chi scrive su `main`

Un solo scrittore alla volta (decisione O). Quando Claude Code non c'è e il maintainer lo dice, **l'operatore è l'unico scrittore su `main`**. Quando Claude Code torna, legge il registro in fondo e `git log`, e riprende lui. Finché Claude Code lavora, l'operatore indaga e prova soltanto (il protocollo dei checkpoint, sotto).

## 2. All'inizio di ogni sessione

1. `git status` e `git log --oneline -5`: lavoro non committato si verifica (C2) e si chiude prima di altro.
2. Leggi `context/STATE.md` (il bookmark "Last commit verified", "Dove siamo", il **Prossimo**) e la coda qui sotto.
3. Se un altro agente compila nella stessa cartella, usa una cartella di build tua: `CARGO_TARGET_DIR=target-<nome>` (altrimenti Windows blocca l'eseguibile dei test, errore LNK1104).

## 3. Le regole che non si violano (in breve; il testo vero è in `CLAUDE.md` e `RULES.md`)

- **C2 verde prima di ogni push:** `cargo build --release`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`, `cargo test --workspace`, `cargo run --release -p ds-test-runner -- --check`, `bash context/scripts/gates.sh`.
- **A5-bis:** mai scegliere un numero guardando la metrica. **Una metrica fuori banda è un bug da trovare**, nel modo in cui un giocatore vede, pensa, si muove, o in come si applicano le Leggi, o nella misura stessa. L0 si chiude davvero a 22 su 22 (maintainer, 2026-11-24).
- **Niente dadi** (ADR-035), niente `f64::powf`/`powi` (non deterministici fra piattaforme: `ds_core::dmath::powf`), il caveau chiuso (ADR-112: le caratteristiche le legge solo l'attore in `ds-orchestrator`).
- **Commit in due passi:** prima il lavoro *senza* `context/STATE.md` (con `Delegated-Decision: DD-n` se c'è una decisione delegata, e `Co-Authored-By`), poi il commit del bookmark che porta "Last commit verified" a quell'hash, con `Verified: docs-only`. Niente force-push, niente tag. Se il push è rifiutato perché su `origin` c'è un commit nuovo (il maintainer modifica dal sito), si ribasano i propri commit non pubblicati: `git reset --soft HEAD~1` (il bookmark), `git stash` di STATE.md, `git rebase origin/main`, `git stash pop`, bookmark rifatto con l'hash nuovo.

## 4. La coda, in ordine di priorità

**A. L0 verso 22 su 22** (`context/plans/CODE-AUDIT.md`, "Terza passata"; la misura in `L0-CALIBRATION.md`, "La misura del 204"). Al SIM 204: 10 su 22. Aperti, con il punto del codice:
1. **T29, seconda metà — i duelli aerei** (6 contro 38): il duello nasce solo se due uomini arrivano nello stesso piccolo passo (`ds-match/src/plugins/technique.rs`, `meet_ball_in_air`, `contenders`).
2. **T23 — il fuorigioco nel blocco** (9.2 contro 4.3): le punte del blocco alla pari della linea per costruzione (`ds-match/src/shape.rs`, `place`).
3. **T17 — le riprese come palla in movimento** (passaggi ×1.8, rimesse ×0.4): il battitore conduce, spazza, tira; la rimessa è calciata (`plugins/thought.rs`, la scena senza la ripresa).
4. **T9 — il primo tocco verso la propria porta** (autogol ×8) (`spatial.rs`, ramo `won` del primo tocco).
5. **T30, T27, T33 — la previsione e l'esecuzione** (gol ×2, tiri ×1.7): `first_time` su una scena povera, la spazzata pensata a 40 m ed eseguita a 50, il resto della lista.
6. **T16 — i gialli per l'azione promettente** (serve una definizione: una decisione delegata, DD).
7. **T11-T14 — la misura** (`tools/ds-reference`): le parate reali con i tentativi sui gol, i passaggi delle riprese, falli e gialli per sotto-evento, i rigori della serie.
8. **OI-101 — il recupero**: l'arbitro vero recupera circa metà del tempo perso; il nostro tutto. Va modellato con la sua fonte, non tarato.

**B. L1** (`context/plans/L1.md`, `L1-COACH.md`). Fatti i passi 1-4; il 5 (allenatore) e il 6 (corpo) al primo anello. Dopo:
1. il passo 5: la condizione e la rotazione (l'allenatore legge la freschezza che `ds-body` porta), il piano gara sull'avversario, il modulo (la lavagna, decisione AL), rigoristi e battitori, `coaches.json` nel pacchetto del mondo;
2. il passo 6: condizione e ritmo partita, poi gli infortuni con il loro ADR (l'infortunio senza dadi, `L1.md` "Domande aperte");
3. il passo 7, la disciplina (cartellini → squalifiche, le soglie dal regolamento citato in `SOURCES.md`); il passo 8, la settimana;
4. SQLite dietro `ds-db` (decisione AD): oggi le rose sono rifatte dal pacchetto e dal seme;
5. il passo 9, la piramide intera.

## 5. Le procedure provate

**Un cambio di comportamento del motore (un SIM).**
1. La correzione e un test di guardia.
2. `SIM_VERSION` + 1 in `L0-match/crates/ds-match/src/save.rs`.
3. I golden: `cargo test --release -p ds-match --test suite -- golden_l0` fallisce e stampa `left:` (il nuovo) e `right:`; si copia `left` in `EXPECTED`, `EXPECTED_INSTRUCTED` (`tests/cases/golden_l0.rs`).
4. I semi: `cargo test --release -p ds-match --test suite -- --ignored cup_seed_scan` dà il primo seme di coppa con i rigori (`CUP_SHOOTOUT_SEED` in `tests/support/mod.rs`, `SHOOTOUT_KICKS_SEED` in `tests/cases/extra_time.rs`; dal SIM 204 serve la scansione fino a 342); poi il golden di coppa `EXPECTED_CUP` dal suo `left`. `LEVEL_AT_90_SEED`, `DECIDED_AT_90_SEED` (`extra_time.rs`) e `HEADER_OFFSIDE_SEED` (`tests/cases/audit_fixes.rs`): il primo seme da 42 in cui l'amichevole è pari, decisa, ha un colpo di testa in fuorigioco fischiato (una sonda in `tools/ds-reference/examples/`, da non committare).
5. La misura: `cargo run --release -p ds-reference -- compare --reference L0-match/reference/serie-a-2017-18.json --seeds 42..101` (circa 7 minuti).
6. I documenti: la voce in `L0-match/CHANGELOG-SIM.md`; "La misura del N" in `context/plans/L0-CALIBRATION.md` con la tabella "metrica | fuori banda | causa | bug o taratura"; la riga in STATE "Dove siamo" e la riga DD; il ✅ in `CODE-AUDIT.md`.
7. Il golden piccolo della stagione cambia anche lui (`L1-season/crates/ds-standings/tests/season.rs`, `SMALL_GOLDEN`).

**Un cambio della stagione.** `SEASON_VERSION` + 1 (`L1-season/crates/ds-season/src/lib.rs`), la voce in `L1-season/CHANGELOG-SEASON.md`, `SMALL_GOLDEN` dal suo `left`. La Serie A intera: `cargo test --release -p ds-standings -- --ignored the_first_series_is_played_whole --nocapture` (circa 14 minuti).

## 6. Trappole incontrate

- `cargo fmt` riformatta: prima di sostituire un pezzo di codice, rileggilo com'è ora.
- Clippy pedantico: il primo paragrafo di un commento doc è breve; i confronti di `f64` con `total_cmp`; `needless_collect` va permesso dove la raccolta è il parallelismo (`ds-matchday`).
- `PlayerLatents` non ha `Debug` né confronto (S6): nei test si confrontano i valori uno per uno, con `to_bits`.
- Le partite di `ds-demo` non sono quelle delle fixture di `ds-reference`: per indagare sulla misura, usa le fixture (`ds_fixtures::match_fixture`).
- Le sonde nella cartella `tools/ds-reference/examples/` restano fuori dal commit e senza avvisi di clippy (`#![allow(clippy::all, clippy::pedantic)]` in testa).

## 7. Il protocollo con Claude Code presente

Claude Code scrive `READY <n> <hash>` in `target/claude-checkpoint.txt`; l'operatore prova quel commit nel suo worktree `C:\Users\liraan\calciovero-oc` e scrive il rapporto in `target/opencode-run.txt` (`RUN <n>`). `BUSY`: aspetta. `STOP`: fermati.

## Registro

| Data | Chi | Cosa | Commit |
|---|---|---|---|
| 2026-11-24 | Claude Code | SIM 203-204 (10 su 22); L1 passi 1-4, primi anelli di 5 e 6; mondo modificabile; questo file | `0608b33` |
