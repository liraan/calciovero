---
title: "Report di 5 sezioni — sessione 2026-11-07 (seconda)"
version: "1.0"
status: "analysis"
last_updated: "2026-11-07"
conforms_to: "D1 (honesty), P2 (grep before saying), il filtro partita vera (foundations §5)"
---

# Cose rimaste in chat — sessione 2026-11-07 (seconda)

**Cosa è questo.** Un elenco di fatti emersi in chat che non hanno un posto nel repo: né un OI, né un ADR, né un file di `analysis/`. Alcune sono nella nota sessione del bookmark come etichetta secca ("noto-sbagliato", i cinque item), ma senza il dettaglio e senza la prima azione. **Non è un OI** e non è una decisione: è materia prima per la prossima sessione. Chi lo legge non era qui.

**Cosa NON è.** Non contiene cose già nel repo: ADR-036, ADR-038, foundations v3.0, gate B9, il README riscritto, l'archiviazione dei 5 handoff, il pattern doppio `reception`/`movement` (in `session-facts.md` §3), la direttiva design (in A2), il tiro noto-sbagliato come etichetta (nel Plan). Tutto quello è committato.

---

## 1. Cose che mentono o bloccano su `main`

**1.1 — Il bias del tiro: il codice fa l'opposto del suo nome e del suo test.**
In `resolve_shot` (pcfm-hidden/src/lib.rs) la deviazione laterale è `dy_sign = +1 se from.1 < to.1, altrimenti -1`. Il commento dice "a real shooter misses towards the side he is on". Il test `resolve_shot_misses_toward_shooter_side` asserisce `to.1 > 3400` per un tiratore a `from.1 = 3000`, cioè il **contrario** di quel che il nome dice. Il test verde è fuorviante (P1 al contrario: un test che impone il difetto). `CHANGELOG-SIM` entry 27 ripete la frase falsa in un artefatto persistente: è D1, non un'opinione.
*Perché conta:* è su `main`, pushato, e chi legge il test crede che la proprietà sia verificata.
*Prima azione:* leggere `resolve_shot` e `shot_forced.rs`, confermare il segno, e o scrivere il test che il nome promette (fallirebbe) o rinominare test e commento per dire cosa fanno davvero. Il fix vero è §3.1.

**1.2 — `speed_from_on_target` restituisce {300, 150}, arbitrari e forse di scala sbagliata.**
Ereditato dall'era del dado. Ora è deterministico ma i due valori non hanno `_source`, e 300 cm/tick = 30 m/s (plausibile per un tiro forte), 150 = 15 m/s (lento). La scala è sospetta perché nessuno l'ha derivata.
*Perché conta:* F.7 ha stabilito "nessun numero arbitrario senza `_source`"; questo è un residuo che non è stato dichiarato arbitrario.
*Prima azione:* aggiungere `CHANGELOG-SIM` nota "arbitrario, calibrazione L3" o derivarlo da una fonte citata.

**1.3 — 366/244 (mezza larghezza, altezza porta) duplicati come letterali.**
In `resolve_shot` (`goal_half_width_cm`, `goal_height_cm`) e in `check_shot_crossing` (`goal_half_w`, `goal_h`). Stesso valore, due fonti. 7.32 × 2.44 m è regolamentare, quindi vive come costante fisica, non in `L0-match/balance/`.
*Perché conta:* D7 (una sola fonte di verità). Il giorno in cui una cambia, il motore tira un gol che `check_shot_crossing` classifica fuori.
*Prima azione:* estrarre in `pcfm-core` (o `pcfm-model`) e far leggere da entrambi.

**1.4 — ADR-038 cita un OI che non esiste.**
Nel corpo: "the public API type check would need an AST, not a grep; an OI, not this ADR". Quell'OI non è stato aperto.
*Prima azione:* aprire l'OI con prima azione "scrivere uno script `process/scripts/check-public-api.sh` che grep i tipi pubblici di `pcfm-match` contro la lista `Path|PathBuf|File|SocketAddr|TcpStream`", oppure togliere la frase dall'ADR (ma un ADR non si riscrive: nota datata).

## 2. Residui di E non verificati

**2.1 — `InfluenceAgent` e `InfluenceContext` orfani in `context.rs`.**
Struct pubbliche, zero consumatori dopo la rimozione di `resolve_influence` (`286100e`). Erano solo input di quella funzione.
*Prima azione:* `grep -rn 'InfluenceAgent\|InfluenceContext' crates/`. Se zero hit: rimuovere le due struct o dichiararle `Removed-by: <OI>`.

**2.2 — `InfluenceWeights`: quattro campi formula orfani.**
`w_space`, `w_skill`, `w_cost`, `w_red` non sono più letti da nessuno. `space_threshold_cm` **sì** (`state.rs:649`, `update_coverage`). La struct resta, per metà morta.
*Prima azione:* decidere se rimuovere i quattro campi (e la loro parte di `InfluenceWeightsJson::to_weights`) o dichiararli riservati per ADR-028/L3.

**2.3 — `ObservationEvent::Pass` sospetto orfano, NON verificato.**
`session-facts.md` §5 dice "After REC, `resolve_pass` no longer pushes an `ObservationEvent::Pass`". Non l'ho verificato con `grep`. **[Verificato in questa sessione: assente, zero hit — non è un OI.]**
*Prima azione:* `grep -rn 'ObservationEvent::Pass\|drain_observations' crates/`. Se il variant esiste e nessuno lo legge, o si rimuove o si dichiara per chi esiste.

**2.4 — `action_probability`: dove vive, chi la imposta.**
Collegata a `action_roll` (Plan step J, OI-78 punto 4). Non verificato.
*Prima azione:* `grep -rn 'action_probability' crates/`. Se è configurazione di partita, rimuovere `action_roll` è un cambio di schema (bump); se è costante, è solo logica.

**2.5 — `resolve_duel` non letto in questa sessione.**
So che ha un dado (`streams.duel`) ma non ho letto il corpo, `DuelContext`, `DuelOutcome`, né il call site in `state.rs` che emette `Tackle`. È il prossimo passo E.
*Prima azione:* leggere i quattro punti prima di proporre la forma geometrica. Il prossimo turno decide.

## 3. Decisioni prese in chat ma non scritte

**3.1 — Il modello del contatto (strada 1) è deciso, il contenuto no.**
Il maintainer ha scelto la strada 1 ("modellare il contatto piede-palla") per il fix del tiro. Il repo ha solo l'etichetta "strada 1, ADR-037 + sessione" (Plan riga E e nota sessione). Il **contenuto** della decisione, emerso in chat, non è in nessun file:
- il contatto deve modellare **piede usato, postura, punto d'impatto**;
- la **postura non esiste** in `AgentState`: c'è `heading` (direzione di movimento, ADR-033) e `gaze` (sguardo, ADR-033/034), non la postura del corpo;
- senza postura, qualunque modello del contatto rischia di essere il **terzo bias** della giornata (dopo il lato del campo e il piede), solo più sofisticato e più difficile da vedere;
- il filtro partita vera dice: un tiratore vero sbaglia in **entrambe** le direzioni; l'errore direzionale emerge dal contatto; il contatto non è modellato; quindi oggi il tiro non può rispettare ADR-035 e il fix è il contatto, non la formula.
*Prima azione:* quando si scrive ADR-037, la prima domanda è se la postura va in `AgentState` prima del contatto (due ADR, due sessioni) o se il contatto si accontenta di una postura di default dichiarata (un ADR, ma il bias resta). La scelta è del maintainer.

**3.2 — `resolve_duel`: la forma è ancora aperta.**
In chat ho proposto "forza vs forza, posizione come modulatore", ma non ho letto il codice e non è una decisione. Il Plan la riporta come prima azione.
*Prima azione:* leggere (2.5), poi decidere.

**3.3 — OI-67: la prima azione è troppo debole.**
OI-67 dice "quando la pratica smette di funzionare, aprire una sessione dedicata". Il trigger è già scattato il 2026-11-07 (una sessione non ha scritto la Nota, la successiva ha ricostruito a memoria — è esattamente ciò che OI-67 descrive come sintomo). La prima azione dovrebbe essere eseguibile oggi: "scrivere la regola a 2 voci in §F + closing step 2c, con `Rules-Audit`, in una sessione dedicata".
*Perché conta:* un OI con una prima azione che aspetta un evento futuro è un'idea, non un passo (D3).
*Prima azione:* aggiornare la prima azione di OI-67.

**3.4 — Meccanizzare l'handoff e il residuo: proposto e scartato come ridondante.**
La chat ha discusso `gates.sh handoff` e `Residue:` nel commit di chiusura. Conclusione: ridondanti (`resume` stampa già i puntatori; i conteggi sono in `doctrine/STRUCTURE.md` §5; se un residuo ha prima azione è un OI, se non l'ha è rumore). L'unica azione sensata è 3.3. **Non riproporre `gates.sh handoff` o `Residue:` senza un caso nuovo**: sono stati valutati.

**3.5 — ARM: Cortex-A vs Cortex-M, e il "gira oggi".**
ADR-038 ha il boundary (tre regole, gate B9) ma non la distinzione. In chat: su Cortex-A (`std`, FPU double) il motore gira così com'è, golden release 36 ms/partita; su Cortex-M (`no_std`, `f64`) serve il porting dichiarato, che ADR-038 lascia possibile ma non anticipa. Se qualcuno chiede "e su ARM?", questa è la risposta; non è in un file.
*Prima azione:* una riga in ADR-038 §Consequences o in `session-facts.md`, se e quando qualcuno la chiede.

**3.6 — OI-26 e il tick record: la domanda portante.**
La chat ha collegato OI-26 ("nei tick senza evento cambia qualcosa?") al tick record (Plan step G): è la cosa che lo risolverebbe, e non esiste. Il motore oggi promette tick-based (54.000 tick) e dimostra event-based (~8 eventi). È la coerenza visione-vs-motore più grande, e oggi la risposta è "non lo sappiamo". OI-26 esiste già come OI, ma il collegamento con G e con la promessa più grande del progetto è emerso in chat.
*Prima azione:* quando si apre G, partire da OI-26, non dalla forma del tick record.

## 4. Cose non lette in questa sessione (D1)

Dichiarate, non nascoste, così il prossimo turno non assume che le abbia viste:
- `resolve_duel` (corpo, contesto, outcome, call site) — **non letto**. Vedi 2.5.
- `ObservationEvent::Pass` — **non verificato con grep**. Vedi 2.3. **[Risolto in questa sessione: assente.]**
- `action_probability` — **non verificato**. Vedi 2.4.
- I `Cargo.toml` delle sei crate di simulazione — letti in questa sessione, sì, ma **solo per il boundary** (ADR-038). Non ho letto le versioni pinnate né le feature flag.
- `WORLD-MODEL.md` §4–§5 — **solo indice di riga**, non contenuto. Se ADR-036 e ADR-038 toccano la frontiera simulation/presentation, va verificato.

## 5. Metodo (una lezione non in B1-bis)

Il paste di ~130 righe con heredoc si è rotto **due volte** in questa sessione: una con `^C`, una con un heredoc rimasto pendente (prompt `>`). Il metodo che ha funzionato: scrivere su `/tmp` in blocchi (`>` poi `>>`), verificare `wc -l`, e `mv` finale — così un'interruzione non tocca il file reale. B1-bis copre il code fence dentro l'heredoc, **non** il paste lungo con caratteri non-ASCII (em-dash, emoji, box-drawing). Il criterio: se il blocco contiene più di ~40 righe o caratteri non-ASCII oltre le lettere accentate, scrivere in `/tmp` e `mv`.

---

**Sintesi per la chat nuova.** Tre cose prima di tutto: (1) il tiro mente su `main` (§1.1–1.2), correggere o dichiarare; (2) prima di `resolve_duel`, leggere il codice (§2.5); (3) ADR-037 (contatto) richiede una decisione sulla postura (§3.1), che è del maintainer, non derivabile. Il resto è manutenzione (§1.3, §2.1–2.4) e un OI da rafforzare (§3.3).

**Aggiornamento post-sessione (2026-11-08):**
- §1.1 **risolto** in `dcc0528`: commento PLACEHOLDER con causa nominata, test `resolve_shot_deviates_laterally`, nota `CHANGELOG-SIM` 27. OI-79 `[PARZIALE]`.
- §2.3 **risolto**: `ObservationEvent::Pass` assente (grep zero).
- §3.1 **risolto**: postura decisa (geometria derivata, non campo; foundations §5, ADR-036, D6). Ordine sessione futura: Plan K → ADR-037 → OI-64.
- §3.4 **risolto**: catturato in questa sessione, regola "cattura dopo ogni pausa e ogni task" in E5; `process/scripts/capture.sh` operativo.
- **Non risolti:** §1.2, §1.3, §1.4, §2.1, §2.2, §2.4, §3.3, §3.5, §3.6, §5 (lezione metodo non in B1-bis).
