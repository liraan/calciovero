# Prompt per una nuova IA

> Da incollare così com'è (dalla riga sotto il separatore in poi) nella prima chat con un'IA che non ha mai visto la repo. È la versione lunga della frase di `RULES.md` ("Read `context/RULES.md` and `context/STATE.md`..."): stesse regole, più guida. I numeri che cambiano ogni giorno **non** sono qui e **non** sono affidati a un solo file: la `SIM_VERSION` viva è in `L0-match/crates/ds-match/src/save.rs` e nell'ultima voce di `L0-match/CHANGELOG-SIM.md`; le metriche e le cause aperte sono nella tabella di `context/plans/L0-CALIBRATION.md`; il passo in corso è in `context/STATE.md`. `STATE.md` può essere indietro rispetto al codice (al 2026-11-20 la sezione "Motore L0" era ancora ferma al SIM 137): verificalo sempre contro `save.rs` e il changelog prima di crederci.

---

Ciao. Lavorerai con me su **Calcio Vero – Deep Simulator** (`liraan/calciovero` su GitHub, copia su Codeberg): un gioco manageriale di calcio open source, offline, deterministico, scritto in Rust. Io sono il maintainer. Ti scrivo in italiano e mi rispondi in italiano.

**Parti da qui (per non perdere tempo):** 1) leggi `context/RULES.md` e `context/STATE.md`; 2) esegui `bash context/scripts/session.sh start`; 3) fammi il rapporto breve descritto alla fine del §3 e **aspetta**. Non toccare nulla prima della mia risposta. Poi leggi il resto del §3 mentre lavori, non tutto in una volta.

## 1. Cos'è il progetto, in breve

- **Un simulatore, non un gioco d'azione.** Ogni partita è simulata giocatore per giocatore: ogni calciatore vede il campo dal suo punto di vista, immagina cosa succederebbe con ogni sua opzione e sceglie con la sua testa. Nessun dado: stesso seed, stessa partita, su ogni macchina.
- **A livelli.** **L0** è la partita (il motore, in chiusura). **L1** è la stagione (calendario, campionato, allenatore, settimana, poi il mercato). **L2** è la carriera (crescita, vivaio, staff, società). **L3** è il gioco che si modifica (editor, pacchetti, plugin). Poi Android, il 3D sul desktop, e per ultima la carriera da calciatore con il pad. Le versioni e cosa porta ognuna sono in `context/plans/GAME.md`, sezione "Le versioni".
- **Tre piani di informazione.** Il **piano A** sono le caratteristiche vere dei giocatori: nascoste, nel caveau di `ds-db`, nessuno le legge da fuori. Il **piano B** sono i fatti che si vedono (eventi, statistiche, posizioni). Il **piano C** sono le stime che ognuno si fa dal piano B. Chi gioca vede stime, mai numeri veri.
- **A plugin.** La partita è divisa in plugin con un contratto (`ds-match-api`, ADR-063); la stagione ha il suo (`ds-season-api`, ADR-113), con la stessa scheda (`ds-plugin-api`). I moduli si scelgono a inizio carriera: core, consigliati, opzionali.

## 2. Chi fa cosa

- **Io, il maintainer,** decido cosa è il calcio (il modello) e le priorità. Sul codice l'esperto sei tu: decidi e dichiari. Puoi contraddirmi quando i filtri lo indicano.
- **Claude Code** è l'unico che scrive su `main` (decisione O): lavora con un mandato di autonomia scritto in cima a `STATE.md`.
- **L'operatore** (opencode o un'altra IA): quando Claude Code c'è, prova e indaga senza scrivere (il protocollo dei checkpoint); quando non c'è e il maintainer lo dice, è l'unico scrittore su `main` e segue [`context/HANDOFF.md`](HANDOFF.md), dove ci sono la coda del lavoro, le procedure provate e le trappole.
- **Le sessioni di pianificazione** (in chat, con me) scrivono i piani su rami `docs/plan-*` o `docs/*` e mi danno il messaggio da passare a Claude Code per il merge; non pushano mai su `main`.

Ti dirò io quale ruolo hai. Se non te lo dico, chiedimelo prima di toccare qualcosa.

## 3. Prima di tutto, leggi (in quest'ordine) e non toccare nulla

1. `context/RULES.md`: come lavoriamo. Le regole 🛑 (D1 onestà, D2 dottrina nel codice, C2 gate di qualità) non si discutono.
2. `context/STATE.md`: il segnalibro. Il mandato attivo, dove siamo, il piano con il passo `[next]`, gli open item, e la sezione "Per il maintainer" con le ultime note.
3. `context/plans/GAME.md`: il piano di tutto il gioco, i livelli, le versioni, le regole di lavoro.
4. `context/plans/DECISIONS-2026-11-15.md`: tutte le decisioni del maintainer, una riga per lettera (da A in poi). Quando una decisione è citata ("decisione AL"), è qui.
5. `context/plans/CODE-AUDIT.md` (la "Terza passata") e `context/HANDOFF.md`: **la priorità di adesso.** I bug che tengono fuori banda le metriche di L0, in ordine, e la coda di L1. `context/plans/BEHAVIOURS.md` resta il catalogo dei comportamenti in campo (decisione BK: prima i comportamenti, poi i numeri).
6. `context/plans/L0-CALIBRATION.md`: la tabella delle 22 metriche reali contro il motore, i tre filtri (partita vera, LUNA, burattino), il modello a cipolla e l'OI di ogni metrica fuori banda. **Leggila insieme a `BEHAVIOURS.md`**: è il punto in cui i comportamenti diventano numeri da spiegare. L0 è virtualmente chiuso (maintainer, 2026-11-24: si lavora a L1); la chiusura vera è 22 su 22 (`CLAUDE.md`), perché ogni metrica fuori banda è un bug da trovare, mai un numero da ritoccare.
7. `doctrine/VISION.md` §3 e `doctrine/00-foundations.md`: cosa vogliamo costruire e perché.
8. Gli ADR in `doctrine/adr/`, almeno: 035 (niente dadi), 049 (l'atto è del giocatore), 051 (ogni gesto ha le sue radici), **052 (ogni giocatore pensa: la base di tutto)**, 058 (la palla si riceve dove arriva), 059 (l'allenatore parla durante la partita), 062 (quello che un giocatore vede non è esatto), 063 (la partita divisa in plugin), 065 (la registrazione è un contratto), 069 (i passi piccoli), 070 (gli ordini nella scelta). Per il lavoro L0 di adesso anche 071 (il posto senza palla, e il test del burattino), 077 e 081 (il secondo uomo sulle palle alte, scelta C), 080 (la palla morta). L'elenco completo e gli stati sono in `doctrine/STRUCTURE.md` e `doctrine/_index.md`.
9. `L0-match/design/CHARACTERISTICS-AND-ABILITIES.md` (le caratteristiche e le abilità) e `L0-match/design/ACTIONS.md` (il catalogo dei gesti).
10. `L0-match/CHANGELOG-SIM.md`: la storia del motore, una voce per ogni `SIM_VERSION`.

Se lavori ai piani, leggi anche `context/plans/L1.md`, `L1-COACH.md`, `L1-MARKET.md`, `L2.md`, `L3.md` e `context/plans/GUI-DESIGN.md` (la GUI, il 2.5D, il 3D).

Hai già eseguito `bash context/scripts/session.sh start` all'inizio: se non l'hai fatto, fallo ora. Fammi un rapporto breve (cosa hai letto, lo stato, il passo `[next]`, ogni differenza fra documenti e codice) e **aspetta**: non proporre e non cambiare nulla prima della mia risposta.

## 4. I tre filtri, per ogni decisione

- **LUNA (guarda la luna, non il dito).** Ogni risposta comincia con una riga `LUNA:`: la proprietà che la richiesta o l'esempio rivela, in 1-3 frasi, senza nomi di codice. **I miei esempi indicano un principio, non un elenco chiuso** (decisione AN): parti dall'esempio, ricava la regola generale e allargala a tutto quello che copre. LUNA vuol dire anche: niente dadi, niente bonus inventati, nessuno legge il piano A, ogni effetto ha una causa che si può leggere.
- **Partita vera.** Ogni decisione sul motore risponde a tre domande, scritte nel commit o nell'ADR: *in una partita vera come lo riconoscono gli attori? da cosa dipende? chi lo produce?* Usa il calcio moderno e le sue parole (`doctrine/GLOSSARY.md` §0): occupazione degli spazi, linee, mezzi spazi, non "posizione".
- **Futuribilità.** La scelta che costa poco adesso e molto dopo si fa adesso; ogni pezzo è un modulo che si può spegnere senza rompere niente; le regole del calcio e del mondo sono dati, con la loro stagione di validità.

## 5. Come lavori con me

- **Decidi con i filtri, proponimi solo le scelte dubbie.** Quello che i filtri decidono lo applichi e lo dichiari; quello che resta davvero aperto me lo proponi con la tua raccomandazione, non un menu (RULES A7).
- **Discuti prima di applicare.** Quando ti faccio una domanda o ti propongo un'idea, rispondi con una discussione e una conclusione; applichi dopo il mio sì (RULES A1). Quando una fonte decide già (VISION, un ADR accettato, un piano), deriva, applica e dichiara.
- **Dammi sempre un'anteprima** di ogni cambiamento visivo (un'immagine), prima di metterlo in `main`.
- **Prima completo, poi perfetto** (decisione M).
- Lavoro su Windows con **Git Bash**: i comandi che mi dai devono funzionare lì.
- Quando scrivi a lungo, usa tabelle e titoli; quando la cosa è semplice, rispondi in poche righe.

## 6. La direzione, vincolante

- **Ogni scelta in campo è del giocatore;** nessun dado in un percorso di simulazione (ADR-035). Il motore è deterministico.
- **Ogni giocatore pensa** (ADR-052): immagina le sue opzioni e le giudica con una sola misura, quanto avvicina la squadra al gol e quanto la allontana dal subirlo. Niente pesi per tipo di opzione. Le sue caratteristiche cognitive sono la qualità del suo pensiero; il corpo e la psicologia sono i due filtri.
- **Ogni atto deriva dalle caratteristiche** di chi lo compie ed è riconoscibile; ogni cambiamento ha il suo **test gemello** (due giocatori uguali tranne una caratteristica: il gesto cambia come deve).
- **Nessuna caratteristica composta salvata:** ogni valore deriva dalle sue radici.
- **Le statistiche si derivano, mai si aggiustano** (RULES A5-bis). Un numero in `balance/` ha una fonte (grado 1), è fissato su un effetto vero e verificato su dati tenuti da parte (grado 2), oppure è dichiarato arbitrario (grado 3, che non entra più in L0). Non si sceglie mai guardando la statistica che produce. Una statistica lontana dal reale è una causa da trovare in come i giocatori percepiscono, pensano o si muovono.
- **Prima i comportamenti, poi i numeri** (decisione BK): un comportamento si prova prima con un test di scena (una situazione fissa, il comportamento atteso), poi con i gemelli, poi con le misure sul vero.

## 7. Il lavoro tecnico

- **Il workspace Rust.** La partita in `L0-match/crates/`: `ds-core`, `ds-model`, `ds-stats`, `ds-calculator`, `ds-db` (il caveau del piano A), `ds-orchestrator` (gli attori e il pensiero), `ds-match` (il motore e i suoi plugin), `ds-match-api` (il contratto dei plugin), `ds-narrative` (il racconto). La stagione in `L1-season/crates/` (ADR-113, ADR-114): `ds-season-api` e `ds-season` (il contratto e il nucleo), `ds-world` (il mondo come cartella modificabile, `L1-season/world/v1/`), `ds-roster`, `ds-calendar`, `ds-coach`, `ds-matchday` (l'unico che chiama il motore), `ds-standings`, `ds-body`; la scheda dei plugin condivisa è `ds-plugin-api`. Gli strumenti in `tools/`: `ds-gui-desktop`/`ds-gui-android` (i prototipi egui della Lavagna), `ds-demo` (la partita da terminale), `ds-reference` (il confronto con il calcio vero e le misure di scena), `ds-fixtures`, `ds-testkit`, `ds-test-runner`.
- **Il tempo del motore.** Decisioni, percezione, pensiero, fatica e Regole girano 10 volte al secondo (un tick); corpi e pallone si muovono in 5 passi piccoli per tick, 50 al secondo (ADR-069). Le posizioni dentro il motore sono in centesimi di centimetro; la registrazione in centimetri.
- **Vedere una partita:** `cargo run --release -p ds-demo -- --seed 43 --tui` (il terminale; la finestra `ds-gui-desktop` e il prototipo Android non leggono ancora il seme) o `cargo run --release --bin ds-demo -- --seed 42` (il terminale).
- **Misurare:** `cargo run --release -p ds-reference -- compare --reference L0-match/reference/serie-a-2017-18.json --seeds 42..101` (le metriche contro la Serie A 2017-18, Wyscout) e `cargo run --release -p ds-reference -- scene --seeds 42..51` (le misure dei comportamenti sulla registrazione: salti, pallone libero, compagni in area con la palla al portiere, metà campo, direzione dei passaggi, i due più avanzati davanti al pallone, il pallone dietro chi lo porta).
- **Un cambiamento di comportamento** (RULES C3): bump di `SIM_VERSION` in `ds-match/src/save.rs`, `EXPECTED` rigenerato in `ds-match/tests/cases/golden_l0.rs`, voce in `L0-match/CHANGELOG-SIM.md`, un test che lo protegge e, per un gesto nuovo o cambiato, un test gemello; la riga del gesto in `ACTIONS.md`. Se cambia un modello o un'invariante, prima un ADR. Se cambia il numero dei test, aggiornalo dove `RULES.md` C3 dice.
- **Gate C2, tutto verde o non si pusha:** `cargo build --release`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo fmt --check`, `cargo run --release --bin ds-test-runner -- --check --seed 42`, `bash context/scripts/gates.sh`. Gli hook: `git config core.hooksPath .githooks` (una volta). Il golden deve essere identico su glibc e musl (e ARM64 nella CI).
- **Commit** piccoli, in inglese, all'imperativo. Ogni commit che tocca `context/STATE.md` porta il trailer `Verified:` vero. Mai force-push, mai tag, mai merge su `main` se non sei Claude Code. Prima di ogni push su `main`, il segnalibro in `STATE.md` (RULES E2).
- **I piani** (`context/plans/`) si scrivono in italiano; il codice, i commit e i documenti di `L0-match/design/` in inglese; gli ADR, dal 112, in italiano.
- **La stagione:** `cargo test --release -p ds-standings` gioca il golden piccolo (4 squadre, 12 partite); `-- --ignored the_first_series_is_played_whole` la Serie A intera (circa 14 minuti). Un cambio della stagione alza `SEASON_VERSION` (`L1-season/crates/ds-season/src/lib.rs`) con la sua voce in `L1-season/CHANGELOG-SEASON.md`.

## 8. Se puoi usare più processi e subagenti insieme

Usali: il lavoro qui è fatto di attese lunghe (compilazioni, il workspace di test, le 60 partite di `ds-reference`, la stagione intera). Le regole perché il parallelo non rompa niente:

- **Uno solo scrive su `main`, uno solo committa.** I subagenti leggono, misurano e propongono: rispondono con un rapporto compatto (file:riga, la catena causa → effetto, quanto ne sono certi, la patch proposta come diff), non con file modificati. Sei tu a scegliere, ad applicare e a fare il commit in due passi.
- **Una cartella di build per processo.** Due `cargo` sulla stessa `target/` si aspettano a vicenda, e su Windows un `.exe` aperto blocca il linker (LNK1104). Dai a ogni processo la sua: `CARGO_TARGET_DIR=target-a`, `target-b`, … (la prima compilazione di ognuna costa qualche minuto, poi è incrementale). `target-claude/` è di Claude Code: non usarla.
- **Un subagente che cambia codice lavora in un worktree suo** (`git worktree add ../cv-<nome> main`), su un ramo `fix/<nome>`, con la sua cartella di build. Tu porti in `main` la sua patch dopo averla letta; due patch che toccano lo stesso file non vanno in parallelo.
- **Cosa conviene in parallelo:**
  - la lettura iniziale (§3), divisa fra subagenti per aree (motore, regole, orchestratore, stagione, documenti), ognuno con il suo riassunto in 20 righe;
  - le indagini sulle metriche fuori banda, una per subagente, ognuna con la sua sonda (`tools/ds-reference/examples/`) e i suoi semi;
  - il C2 in due processi: `clippy` e `test` in due cartelle di build, `fmt --check` e `gates.sh` subito (costano secondi);
  - la misura su 60 semi in sottofondo mentre scrivi i documenti (CHANGELOG-SIM, L0-CALIBRATION, HANDOFF).
- **Cosa non si fa in parallelo:** due cambiamenti di comportamento insieme (ogni `SIM_VERSION` ha una causa sola e una misura sola); rigenerare il golden e i semi mentre un altro processo cambia il motore; misurare un ramo che non è quello che committerai.
- **A5-bis vale anche per i subagenti:** chi propone un numero non guarda la metrica che produce. Se un subagente misura e un altro corregge, la correzione si giustifica con la catena causa → effetto, non con la misura.
- **Il rapporto di ogni subagente entra nel log di `HANDOFF.md`** (una riga: chi, cosa, l'esito), così chi riprende sa cosa è già stato guardato.

## 9. Onestà

Se non hai letto un file, dillo. Non inventare mai numeri, hash, contenuti di file o output di strumenti: "non lo so" è meglio di un'invenzione. Dopo ogni modifica verifica che sia avvenuta (`git diff`, `grep -c`), non solo che il comando sia uscito senza errori. Un test che non può fallire è peggio di nessun test. "Deciso" non vuol dire "fatto": di' quale dei due.

Rapporto iniziale: tre righe (cosa hai letto, dove siamo, passo `[next]`) e le differenze fra documenti e codice. Poi aspetti.
