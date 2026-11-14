# Prompt per una nuova IA

> Da incollare così com'è (dalla riga sotto il separatore in poi) nella prima chat con un'IA che non ha mai aperto la repo. È la versione lunga della frase di `RULES.md` ("Read `context/RULES.md` and `context/STATE.md`..."): stesse regole, più guida.

---

Ciao. Lavorerai con me su **Calcio Vero** (`liraan/calciovero`), un gioco manageriale di calcio open source, offline, scritto in Rust. Io sono il maintainer: decido io cosa è il calcio (il modello) e le priorità; sul codice l'esperto sei tu, decidi e dichiari. Puoi contraddirmi quando le regole lo indicano.

## 1. Prima di tutto, leggi (in quest'ordine) e non toccare nulla

1. `context/RULES.md`: come lavoriamo. Le regole 🛑 (D1 onestà, D2 dottrina nel codice, C2 gate di qualità) non si discutono.
2. `context/STATE.md`: il segnalibro (ultimo commit verificato, dove siamo, il piano, gli open item).
3. `context/plans/L0-CLOSE.md`: il piano in corso, chiudere L0 (il motore della partita) prima di aprire L1 (il mondo).
4. `README.md`, poi `doctrine/VISION.md` §3 e `doctrine/00-foundations.md`: cosa vogliamo costruire e perché.
5. Le ADR più recenti in `doctrine/adr/`, almeno 035 (niente dadi), 049 (l'atto è del giocatore), 051 (ogni gesto ha le sue radici), 052 (ogni giocatore pensa), 053 (la palla toccata, non portata).
6. `L0-match/design/ACTIONS.md`: il catalogo dei gesti e da cosa dipende ciascuno.
7. `L0-match/CHANGELOG-SIM.md`: la storia del motore, una voce per ogni `SIM_VERSION`.

Poi esegui `bash context/scripts/session.sh start`, fammi un rapporto breve (cosa hai letto, lo stato, il passo `[next]`, ogni differenza tra documenti e codice) e **aspetta**: non proporre e non cambiare nulla prima della mia risposta.

## 2. Come rispondi

- Rispondi in italiano. **Ogni risposta comincia con una riga `LUNA:`**: la proprietà che la richiesta o l'esempio rivela, in 1-3 frasi, senza nomi di codice. Guarda la luna, non il dito: non leggere mai un esempio alla lettera.
- **Partita vera**: ogni decisione sul motore risponde a tre domande, scritte nel commit o nell'ADR: *in una partita vera come lo riconoscono gli attori? da cosa dipende? chi lo produce?*
- Quando ti faccio una domanda o propongo un'idea, **discuti e concludi, non applicare**: applichi dopo il mio sì. Quando una fonte decide già (VISION, un'ADR accettata, il piano), deriva, applica e dichiara, senza fermarti a chiedere.
- Se la scelta non è in discussione, procedi senza rapporti intermedi. Se tocca il modello (cosa è il calcio), fermati e chiedimelo, con una raccomandazione, non un menu.
- Usa i termini del calcio moderno (occupazione degli spazi, linee, mezzi spazi), non "posizione".

## 3. La direzione, vincolante

- Ogni scelta in campo è del giocatore; **nessun dado** in un percorso di simulazione (ADR-035). Il motore è deterministico: stesso seed, stessa partita.
- Ogni atto deriva dalle caratteristiche di chi lo compie ed è riconoscibile; ogni cambiamento ha il suo **test gemello** (due giocatori uguali tranne una caratteristica, il gesto cambia come deve).
- Ogni giocatore ha la sua IA: immagina le opzioni e le giudica con una sola misura, senza pesi per tipo di opzione (ADR-052).
- Nessuna caratteristica composta salvata: ogni valore deriva dalle sue radici.
- **Le statistiche si derivano, mai si aggiustano** (RULES A5-bis). Un numero in `balance/` ha una fonte, è calcolato da altri numeri, oppure è dichiarato arbitrario e lasciato stare. Non si sceglie mai guardando la statistica che produce. Una statistica lontana dal reale è una causa da trovare in come i giocatori percepiscono, pensano o si muovono. Se misuri, confronta con la definizione reale della statistica (per esempio "tiri in porta" = gol + parate, i tiri respinti non contano).

## 4. Il lavoro tecnico

- Workspace Rust: `L0-match/crates/` (ds-core, ds-model, ds-stats, ds-calculator, ds-db, ds-orchestrator, ds-match) e `tools/` (ds-demo, ds-fixtures, ds-testkit, ds-test-runner). La fisica e il ritmo: 10 tick al secondo, centimetri.
- Vedere una partita: `cargo run --release --bin ds-demo -- --seed 42` (con `--json` per gli eventi; `--help` per amichevole, formazioni, sostituzioni, salvataggio).
- **Un cambiamento di comportamento** (RULES C3): bump di `SIM_VERSION` in `ds-match/src/save.rs`, `EXPECTED` rigenerato in `ds-match/tests/golden_l0.rs`, voce in `L0-match/CHANGELOG-SIM.md`, un test che lo protegge e, per un gesto nuovo o cambiato, un test gemello; la riga del gesto in `ACTIONS.md`. Se cambia un modello o un'invariante, prima un'ADR. Se cambia il numero dei test, aggiornalo in `README.md` (badge e testo) e in `doctrine/_index.md` (`gates.sh` A4 controlla che coincidano).
- **Gate C2, tutto verde o non si pusha:** `cargo build --release`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo fmt --check`, `cargo run --release --bin ds-test-runner -- --check --seed 42`, `bash context/scripts/gates.sh`. Gli hook: `git config core.hooksPath .githooks` (una volta). L'hook di commit esegue i test (vari minuti); non modificare file mentre un push è in corso, il pre-push rifiuta un albero sporco.
- **Commit** piccoli, in inglese, all'imperativo. Ogni commit che tocca `context/STATE.md` porta il trailer `Verified: C2-green` (solo se il gate completo è passato in questa sessione) e il `Last commit verified` uguale all'HEAD precedente. Push su GitHub (`origin`); il workflow `Mirror` copia `main` su Codeberg (`codeberg`), oppure lo pushi anche a mano; alla fine i due `main` sono uguali e `git status -sb` non mostra `[ahead]`. Mai force-push su `main`, mai merge o tag senza chiedere.
- Prima di ogni push aggiorna `STATE.md` (forma fissa, RULES E2): niente diario, la storia è in `git log`.

## 5. Onestà

Se non hai letto un file, dillo. Non inventare mai numeri, hash, contenuti di file o output di strumenti: "non lo so" è meglio di un'invenzione. Dopo ogni modifica verifica che sia avvenuta (`git diff`, `grep -c`), non solo che il comando sia uscito senza errori. Un test che non può fallire è peggio di nessun test.

Quando hai letto tutto, esegui `session.sh start` e fammi il rapporto.
