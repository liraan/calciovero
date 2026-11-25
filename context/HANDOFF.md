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
- **Mai cambiare la soglia di un test esistente.** Se un guardiano scatta dopo una correzione, è un indizio: si indaga e si scrive, non si allenta (l'esame del 2026-11-24 l'ha fatto: è l'errore da non ripetere).
- **Niente dadi** (ADR-035), niente `f64::powf`/`powi` (non deterministici fra piattaforme: `ds_core::dmath::powf`), il caveau chiuso (ADR-112: le caratteristiche le legge solo l'attore in `ds-orchestrator`).
- **Commit in due passi:** prima il lavoro *senza* `context/STATE.md` (con `Delegated-Decision: DD-n` se c'è una decisione delegata, e `Co-Authored-By`), poi il commit del bookmark che porta "Last commit verified" a quell'hash, con `Verified: docs-only`. Niente force-push, niente tag. Se il push è rifiutato perché su `origin` c'è un commit nuovo (il maintainer modifica dal sito), si ribasano i propri commit non pubblicati: `git reset --soft HEAD~1` (il bookmark), `git stash` di STATE.md, `git rebase origin/main`, `git stash pop`, bookmark rifatto con l'hash nuovo.

## 4. La coda, in ordine di priorità

**Ruoli.** *Con Claude Code attivo* (com'è dal 2026-11-24 sera): Claude Code è l'unico su `main` e lavora su L1; l'operatore fa L0 nel suo worktree `calciovero-oc` su un ramo `fix/<nome>`, una causa per `SIM_VERSION`, rapporto in `target/opencode-run.txt`, e Claude Code porta le patch in `main`. *Con Claude Code fermo* (token finiti, lo dice il maintainer): l'operatore è l'unico scrittore, con [`context/prompts/OPENCODE.md`](prompts/OPENCODE.md); ogni commit una riga in [`context/REVIEW-QUEUE.md`](REVIEW-QUEUE.md), da cui Claude Code riparte (`context/prompts/CLAUDE-RESUME.md`). Il lavoro in parallelo: `NEW-AI-PROMPT.md` §8.

**A0. Il SIM 210 in attesa** (`context/plans/patches/sim210-loft-lookahead.patch`; il 208 è il recupero della Regola 7.3, il 209 il colpo di testa sotto zero; il 206 è il dribbling, il 207 il primo controllo nell'area). Misurato il 2026-11-25 sopra il 207 (42..101): gol 4.98 → 6.93, autogol 0.23 → 0.47, angoli 7.23 → 18.97, tiri 42.7 → 49.4, cross 7.8 → 14.2, duelli aerei 6.6 → 11.0, rimesse 19.1 → 33.4; 12 → 10 in banda. La correzione è di principio, ma porta alla luce bug a valle dei palloni alti: chi li manda sul fondo (gli angoli), chi li mette nella propria porta (gli autogol), perché ogni pallone alto in area diventa un tiro. Opencode RUN 17 (`target/opencode-log.md`): `receiver_best_pass` conta anche i lanci del ricevitore, così un lancio vale il lancio che seguirà; i difensori nel proprio terzo lanciano verso uomini vicini alla propria porta e il pallone esce sul fondo in volo (angoli 11.65 in più a partita) o entra (autogol). Proposta senza numeri nuovi: nel lookahead del lancio non contare i lanci del ricevitore. **Scelta di dottrina (ADR-052, OI-116 A): DD? per il maintainer.** Il lancio vale come il rasoterra la migliore giocata successiva del ricevitore (ADR-052, OI-116 A; T33 "solo il passaggio guarda un passo avanti"), `SIM_VERSION` 206. Misura rapida su 20 partite (42..61): cross 8.7 → 16.6, duelli aerei 7.9 → 10, rimesse 17.8 → 33.7 (in banda), spazzate 25; ma angoli 10.9 → 22.2 e gol 5.5 → 6.65. La correzione è di principio e resta; gli angoli sono l'indizio successivo (troppi palloni alti finiscono sul fondo: chi li manda, chi li tocca per ultimo). Da fare: applicare la patch (`git apply`), golden, semi, misura su 60, documenti (procedura §5), poi seguire l'indizio degli angoli.

**A-bis. Dal rapporto 15 di opencode (2026-11-25)**: fatti il SIM 206 (il dribbling è superare l'uomo: 25.98 → 9.73, reale 16.03; resta quante volte si prova) e il SIM 207 (T9, autogol in banda). Aperti, verificati sulle registrazioni: le rimesse (17.8 contro 42.7: la palla sulle fasce il 3.1% dei tick, i passaggi puntano al centro: `read.rs` 1413-1473, `shape.rs` 204-210); il fuorigioco (9.25 contro 4.26: il passatore serve un uomo 10 m oltre la linea, `offball.rs` 720-779, OI-98).

**A. L0 verso 22 su 22** (`context/plans/CODE-AUDIT.md`, "Terza passata"; la misura in `L0-CALIBRATION.md`, "La misura del 204"). Al SIM 205: 10 su 22. Aperti, con il punto del codice:
1. **I duelli aerei** (7.9 contro 38): dal SIM 205 contende chi arriva nel tick; la causa grande è che i palloni alti si giocano poco (i cross 8.7 contro 33.8). Prima lettura (2026-11-24, da verificare prima di toccare): in `ds-orchestrator/src/read.rs:1559-1597` un lancio conteso vale zero se chi passa non crede che il suo uomo sia più forte in aria di *tutti* gli avversari che arrivano (`wins()`); è coerente con un duello aereo che il motore risolve sempre a favore del più forte (`resolve_aerial`). **Confermato (2026-11-24):** `ds-orchestrator/src/lib.rs:664` `resolve_aerial` sceglie sempre il più forte in aria (`aerial_force`: colpo di testa, forza esplosiva, forza massima, altezza; più le mani del portiere); chi arriva prima e chi è meglio piazzato non contano. È il duello il bug, e il lancio lo eredita (chi passa prevede bene che il suo uomo perde sempre contro uno più forte). La correzione è di modello: un ADR (il duello aereo dipende anche dall'arrivo e dalla posizione, con le radici di ADR-051 e la scelta C di ADR-081), poi il lancio pensato con la stessa misura. Anche: l'errore del lancio pensato usa un calciatore medio (`error_rad(0.5, 0.75, 0.6, …)`, S18 aperto).
2. **T23 — il fuorigioco nel blocco** (9.2 contro 4.3): le punte del blocco alla pari della linea per costruzione (`ds-match/src/shape.rs`, `place`).
3. **T17 — le riprese come palla in movimento** (passaggi ×1.8, rimesse ×0.4): il battitore conduce, spazza, tira; la rimessa è calciata (`plugins/thought.rs`, la scena senza la ripresa).
4. ✅ **T9 — il primo tocco verso la propria porta**: SIM 207, autogol 0.90 → 0.23, in banda.
5. **T30, T27, T33 — la previsione e l'esecuzione** (gol ×2, tiri ×1.7): `first_time` su una scena povera, la spazzata pensata a 40 m ed eseguita a 50, il resto della lista.
6. **T16 — i gialli per l'azione promettente** (serve una definizione: una decisione delegata, DD).
7. **T11-T14 — la misura** (`tools/ds-reference`): le parate reali con i tentativi sui gol, i passaggi delle riprese, falli e gialli per sotto-evento, i rigori della serie.
8. **La panchina del motore** (`ds-model` `MAX_BENCH` 7): la Serie A ne iscrive 15 (Lega Serie A 2024-25; IFAB Regola 3: da 3 a 15); la stagione ne nomina 7 e lo dichiara. Alzare il limite a 15 è un cambiamento del modello (L0).
9. **Il cartellino dica se il rosso è un secondo giallo** (oggi il motore scrive solo `red`): la disciplina lo deduce da giallo e rosso nella stessa partita.
10. **OI-101 — il recupero**: l'arbitro vero recupera circa metà del tempo perso; il nostro tutto. Va modellato con la sua fonte, non tarato.

**A9. La velocità dei test** (fatto per `ds-standings`, 2026-11-24: la stagione piccola giocata una volta per binario, 208 → 103 s; per `ds-match::suite` non serve: `tests/support/mod.rs` tiene già una cache delle partite per binario, e la lentezza misurata viene da nextest, che lancia ogni test in un processo suo e rigioca la partita. Con `cargo test` (il C2) la condivisione funziona; con nextest servirebbe un gruppo di test per binario, o non usarlo per la suite) (misura della corsa 8 con nextest: 629 test, 839 s di orologio, 5685 s di somma). I lenti sono partite intere ripetute: `ds-standings` gioca la stessa stagione piccola (seme 42, 4 squadre) in tre test (171 s, 101 s, …), `ds-match::suite` rigioca la partita del golden in molti test di `golden_scenes` e `instructions` (99-170 s ciascuno). La cura senza toccare il motore: una partita o una stagione per seme, giocata una volta per binario di test (`std::sync::OnceLock`) e letta da tutti i test che la usano; nessuna soglia cambiata, nessun test tolto. Misura prima e dopo con `cargo nextest run --workspace`.

**B0. L'anteprima giocabile (decisione BN, prima di tutto in L1).** Fatto: `ds-career` (la carriera senza grafica: nuova, avanza, la formazione del giocatore come input datato, rosa vista dall'allenatore, classifica, calendario, salvataggio), la porta degli input della stagione (`Season::input`, `InputGiven`). Fatto anche: `Career::watch` (la partita della stagione rigiocata registrata, stesso risultato: test), `simulate_keyed_recorded`, la Giornata divisa in `build`/`fixtures_on`/`lineups_on`/`start_on`. Installati (2026-11-24, cartella utente, senza amministratore): JDK 17 Temurin (`%LOCALAPPDATA%\Programs\jdk-17…`), Android SDK (`%LOCALAPPDATA%\Android\Sdk`: platform-tools, android-34, build-tools 34, NDK 26.3.11579264), il target `aarch64-linux-android`, `cargo-apk`; `JAVA_HOME`, `ANDROID_HOME`, `ANDROID_NDK_HOME` nel profilo utente. Da fare, nell'ordine della decisione BO: il guscio della GUI (finestre, colonna di icone, lavagna, tablet), la scheda giocatore, gli osservatori, il mercato, l'economia, la build Android.

**B. L1** (`context/plans/L1.md`, `L1-COACH.md`). Fatti i passi 1-4; il 5 (allenatore) al secondo anello (il posto umano, l'adattabilità, DD-70), il 6 (corpo) e il 7 (disciplina) al primo. **Direzione del maintainer (2026-11-24):** la base del genere manageriale (la settimana: posta, allenamento, osservatori, preparazione della partita, partita, conferenza) è il punto di partenza da migliorare come flusso di lavoro, non da copiare (niente nomi, schermate o testi di un gioco esistente). E l'allenatore va **studiato per bene prima di scriverlo**: è anche l'allenamento, la crescita dei giocatori (fisica, tecnica, cognitiva), la gestione psicologica, e la partita vera (le sostituzioni, i cambi di piano, ADR-059), tutto passato ai filtri LUNA e partita vera. Quello studio è dottrina: **scritto** (Claude Code, 2026-11-24) come proposta in `context/plans/L1-COACH-STUDY.md`, aspetta le risposte del maintainer al §8; nessuno lo implementa prima. In origine: la proposta in `context/plans/L1-COACH-STUDY.md` (fonti, cosa fa un allenatore vero, cosa ne vede il giocatore, quali radici, quali moduli spegnibili, la versione semplice per Android), **senza codice**, e lo lascia al maintainer. Dopo:
1. il passo 5 (l'ordine è `L1-COACH-STUDY.md` §7; fatti gli anelli 1, 2 e 3: allenatori e panchina, rotazione, la settimana minima): la condizione e la rotazione (l'allenatore legge la freschezza che `ds-body` porta), il piano gara sull'avversario, il modulo (la lavagna, decisione AL), rigoristi e battitori, `coaches.json` nel pacchetto del mondo;
2. il passo 6: condizione e ritmo partita, poi gli infortuni con il loro ADR (l'infortunio senza dadi, `L1.md` "Domande aperte");
3. il passo 7, la disciplina: primo anello fatto (`ds-discipline`, DD-69); soglie verificate sul CGS (art. 9 c. 5); da fare la squalifica lunga per condotta violenta, la Coppa Italia come conteggio a parte quando arrivano le coppe; in L0 il cartellino dica se il rosso è un secondo giallo; il passo 8, la settimana;
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

Dal 2026-11-25: il ciclo a oltranza di [`context/prompts/OPENCODE-LOOP.md`](prompts/OPENCODE-LOOP.md). Claude Code scrive gli ordini in `target/opencode-orders.md`; l'operatore appende i rapporti in `target/opencode-log.md`, uno sotto l'altro. Il vecchio scambio a checkpoint (`claude-checkpoint.txt`, `opencode-run.txt`) è superato.

Claude Code scrive `READY <n> <hash>` in `target/claude-checkpoint.txt`; l'operatore prova quel commit nel suo worktree `C:\Users\liraan\calciovero-oc` e scrive il rapporto in `target/opencode-run.txt` (`RUN <n>`). `BUSY`: aspetta. `STOP`: fermati.

## Registro

| Data | Chi | Cosa | Commit |
|---|---|---|---|
| 2026-11-24 | Claude Code | SIM 203-204 (10 su 22); L1 passi 1-4, primi anelli di 5 e 6; mondo modificabile; pulizia della documentazione; questo file | `db5221b` |
| 2026-11-24 | Claude Code | SIM 205 (T29 chiuso); l'esame di opencode (7/10: buona disciplina, si ferma alla prima causa, aveva allentato una soglia); la regola sulle soglie | `19050bd` |
| 2026-11-24 | Claude Code | il SIM 206 come patch per l'operatore (L0 a opencode, maintainer); L1 passo 7, primo anello (la disciplina) | `be3273b` |
| 2026-11-24 | Claude Code | L1 passo 5, secondo anello (il posto umano, l'adattabilità, DD-70); il passaggio all'operatore unico scrittore: `prompts/OPENCODE.md`, `REVIEW-QUEUE.md`, `prompts/CLAUDE-RESUME.md` | `1275a83` |
| 2026-11-24 | Claude Code | pulizia (file, rami, worktree); lo studio dell'allenatore; la stagione piccola giocata una volta nei test | `aa0cfd1` |
| 2026-11-24 | Claude Code | decisione BM; la disciplina sul testo del CGS (`0984db5`); l'anello 1 dell'allenatore | `cc9748a` |
| 2026-11-24 | Claude Code | l'anello 2 dell'allenatore, la rotazione | `c2a1f2a` |
| 2026-11-24 | Claude Code | la settimana, primo anello (`ds-week`) | `f95fc57` |
| 2026-11-24 | Claude Code | decisione BN; `ds-career`, `Season::input` | `969c35b` |
| 2026-11-24 | Claude Code | decisione BO; `Career::watch`; la catena Android installata | `89902d5` |
| 2026-11-25 | Claude Code | SIM 206, il dribbling è superare l'uomo (dal rapporto 15 di opencode); la GUI BP sul ramo `feat/career-gui` | `1dfb1ea` |
| 2026-11-25 | Claude Code | SIM 207, T9: autogol in banda (0.90 → 0.23); 12 su 22 | `3980dd4` |
| 2026-11-25 | Claude Code | SIM 208, il recupero della Regola 7.3: 14 su 22; il 209 (lancio lungo) aspetta una scelta (RUN 17) | `89de419` |
