# Nota per una nuova AI — Calcio Vero

**Status:** briefing (not normative, not an ADR, not a handoff).
**Ultima revisione:** 2026-11-11 (rev. 8). **LCV:** `57f6c74`.
**Supersedes:** `docs/context/analysis/2026-11-08-nota-per-nuova-ai.md` (storia).

**Cos'è.** La memoria delle cose emerse in chat che un agente nuovo non
troverebbe leggendo solo il repo. **Non** è un handoff (quello è in
`analysis/*-session-handoff-*.md`). **Non** è una regola: le regole sono
in `NEW-SESSION-PROMPT.md`, e la nota **non le duplica** — le cita.
**Lettura obbligatoria a inizio sessione (E1). Aggiornamento obbligatorio
a fine sessione (E2 step 2d).**

**Il filtro di questa nota (vedi §9).** Una voce entra solo se (a) non è
già mandatory nel prompt, (b) senza di essa un agente nuovo sbaglierebbe
qualcosa di concreto (`why-new:`), (c) sta in una riga. Se il prompt la
copre, la voce è un **puntatore**, non una rispiegazione.

## 1. Il patto, in una riga

Il maintainer: *"il progetto è nostro, non mio, ci stiamo lavorando
entrambi e per quanto riguarda il codice sei tu l'esperto"*. Sul codice
l'agente è l'esperto; sul modello e sulle priorità decide il maintainer
(P6). Non chiedere permesso per ciò che una fonte già decide (direttiva
2026-11-07): deriva, applica, dichiara. Quando il maintainer corregge,
riformula (P7). **"Puoi anche contraddirmi se vuoi"**: la contraddizione
è attesa quando i filtri puntano lì.

## 2. I due filtri — puntatori

**LUNA** (A3) e **partita vera** (D6) sono già regole del prompt: qui non
si rispiegano. Ciò che il prompt **non** dice, e che è il motivo per cui
la nota esiste, è in §3.

## 3. Ciò che i filtri hanno prodotto (non è nel prompt)

**La pletora non esiste.** Il giocatore con palla non ha davanti venti
opzioni: ne ha **due o tre chiare**, spesso una. La pletora è lo sguardo
dell'allenatore, non del giocatore. Il modello **non deve dare un
punteggio a tutto** ciò che è fisicamente possibile: è il prototipo da
cui ADR-040 è nato per uscire.

**La tattica agisce in due modi, non uno.** (a) Cambia **cosa il
giocatore guarda** (a chi è detto "non salire" smette di scansionare
l'opzione di salita). (b) Cambia **quanto pesa** ciò che vede (a chi è
detto "tira appena puoi" il tiro pesa di più, il passaggio resta
visibile). Due meccanismi, percezione vs salienza, da tenere separati
quando L2 arriverà. A L0 non c'è nessuno dei due.

**L'avanzata non è passività.** "Portare palla" ha un esito osservabile:
il corpo va verso la porta, la palla resta ai piedi, il difensore viene
attirato. È un gesto, non un non-evento — per questo
`ChoiceKind::Advance` è una scelta e non "nessuna azione".

## 4. La lezione di OI-88 (la trappola da non ripetere)

Una distinzione è una **scelta** o una **lettura di una scelta**? Se è
una lettura, va derivata; se è una scelta, deve essere continua, non un
enum. OI-88 fu scritto come `ShotIntent` (categoria scelta), vietato da
L0.11 §4.6; la forma corretta è due scelte continue (`aim_point`,
`power`), con `ShotType` derivato. La stessa trappola è tornata con la
traiettoria del passaggio e con `advance_room`.

**Il vocabolario a quattro parole** (verificabile, tracciabile,
artefatto, fonte di verità) è già nel prompt: puntatore, non rispiegazione.

## 5. Regole per le formule di salienza (dalla sessione 2026-11-09)

**Misura un segnale che varia, non l'assenza di una condizione rara.**
`advance_room` tornava 1.0 quasi sempre ("nessun avversario davanti" è la
condizione **normale**): una salienza costante non compete con un
passaggio che vale 0.1-0.4. Qualsiasi formula futura (aereo, portiere)
deve **guardare la distribuzione del proprio output**: se il 90% dei casi
vale 1.0, è rotta. L'avanzata è atterrata a **soglia**
(`clarity_threshold`), non a competizione.

**`freedom_norm` ha un pavimento a 0.5.** La formula è `freedom /
(freedom + PASS_FREEDOM_CM)`; `build_choice` salta i riceventi con
`freedom < PASS_FREEDOM_CM`, quindi il minimo è 0.5, non 0. Toccare
`PASS_FREEDOM_CM` cambia anche la forma della salienza.

**La salienza misura la scena; la cognizione decide cosa farne.** Tre
strati in codice: la lettura allarga ciò che si vede, la capacità di
decidere sposta la soglia, la gestione del rischio pesa le opzioni. Non
si fondono in un unico punteggio.

**La soglia ha un rischio aperto.** `clarity_threshold` fisso a 0.25: se
i punteggi di un'intera squadra stanno sotto, quella squadra porta palla
sempre e il motore non lo segnala. La calibrazione (L3) deve controllare
per squadra e per livello.

**Il test funzionale è un canarino deliberato.**
`functional_l0_the_engine_emits_the_claimed_shape` fallisce se la soglia
sale o la salienza scende ("no Pass emitted"). È il guardiano, non un
test da rilassare.

**Un filtro relativo al massimo non cambia l'argmax.** Una soglia `x/max < k` taglia le opzioni peggiori; l'argmax è il massimo, resta visibile a tutti, e due lettori diversi scelgono lo stesso. Per far divergere la scelta serve un campo per-candidato **indipendente dal punteggio** (`visibility`), non una soglia sul punteggio. `why-new:` senza, un agente ripropone il filtro geometrico e rifà l'errore due volte.

## 6. Cambiare l'orologio? Prima leggi questo

IDEA-011 è atterrata (`c3c0b7f`): il tick è il substrato, il pace viene dall'attore. Se qualcuno propone di cambiare l'orologio, la frase da citare è *"non cambiare l'orologio, cambia chi ha il permesso di fermarlo"*. Le quattro alternative (event-driven, passo variabile, due orologi, fase per agente) sono state valutate e non riprese: la seconda e la terza sono le uniche compatibili con il determinismo, e la forma attuale ne è già un caso parziale (scan ogni 5 tick). Non riaprirle senza un ADR.


## 7. Decisioni orali del maintainer (non scritte altrove)

- **Bump di versione dei crate: no, per ora.** `0.1.0` nel workspace.
  Bump a `0.2.0` quando atterra ADR32 (rename `pcfm-*` -> `ds-*`), così
  "versione nuova" e "prefisso nuovo" coincidono.
- **ADR32 è in lista, non urgente, ma va fatto.** Già `[open]` nel Plan.
- **Codeberg è backup, GitHub è primario.** Regola in `AGENT-POLICY.md`
  §6: ogni push va a entrambi.
- **IDEA-DESIGN-007 è un'idea, non una decisione.** Prerequisito: leggere
  `pcfm-fixtures`.

## 8. Cose che non hanno un OI ma vanno sapute

- **ADR-044 `accepted` (e ADR-045): la catena, leggi prima di toccare un `resolve_*`.** Il tiro e una direzione verso la porta ristretta da caratteristiche, con la modalita (pallonetto/potenza/piazzata) come scelta sempre disponibile; la salienza pesa, non sceglie. `why-new:` senza, un agente legge il codice e crede che il tiro scelga un punto, e che il pallonetto sia un caso speciale da aggiungere. Caratteristiche -> relazioni -> atti, nessuna eccezione (atto sbagliato, omissione, portiere, palla con catena corta). Caratteristiche (attori, latenti) e proprieta fisiche (palla, suolo, dati dichiarati) sono due parole, non una. `why-new:` senza, un agente legge il codice e crede che l'omissione non esista e che il portiere sia una soglia — e sono le eccezioni che ADR-044 vieta.

- **`resolve_duel` è ancora prototipo** (legge un dado). Debito in Plan E
  (OI-50), non un bug nuovo.
- **Il tick gira il carrier verso la porta prima di `build_choice`**
  (OI-89): il `heading` del carrier è sempre verso il centro porta.
- **`PassContext.receiver_freedom_cm` è un orfano** (OI-90): il commento
  dice "carried to the reception", nessuno lo legge.
- **`scripts/closing.sh` non scrive l'handoff.** C2, gates, capture. Se
  il maintainer lo chiede prima di "chiudi", va scritto prima del commit.
- **Il conteggio test nei documenti può essere stale** rispetto al reale:
  `A4-coherence` controlla i documenti fra loro, non col reale. Verifica
  con C2.

## 9. Meta-istruzioni osservate in sessione

- **"Filtra la decisione con calcio vero e luna, poi presentami la tua
  conclusione."** Quando una decisione è bloccata, il maintainer vuole
  una **conclusione**, non un menu (P6 rinforzato).
- **"Voglio il tuo granello di sabbia"** = contribuisci con una proprietà
  che l'esempio rivela, non con un'assoluzione. La forma è `LUNA:`.
- **Domande-provocazione.** "Allora ad oggi questo non è un motore
  fenomenologico è un motore a tick" era una provocazione per testare la
  risposta. La forma giusta distingue ciò che è vero da ciò che è mezzo
  vero: substrato vs crepa, pezzi fatti vs pezzi dichiarati.
- **Le pause possono durare a lungo e produrre più decisioni.** "Pausa"
  non è "fermati": è "discutiamo". Al "riprendi", il primo comando è E5
  (catturare), non riprendere il codice.
- **I filtri giudicano il modello, non l'architettura.** Un ADR che divide crate o sposta responsabilità non passa per partita vera/luna (un tifoso non vede il layout): dichiara quali asserzioni sono filtrate e quali sono giustificate da D5/D7/ADR-032. Un ADR di modello (il duello, la scelta) passa per i filtri, e i filtri lo correggono (ADR-042: `ball_contested` geometrico, `maximal_force`, il fallo come esito del contatto). `why-new:` senza, un agente applica i filtri a un ADR di architettura e si blocca, o non li applica a un ADR di modello e accetta una contraddizione (ADR-042 vs ADR-040).
- **"Siamo sicuri?" è una domanda, non un'assoluzione.** Il maintainer la fa quando il documento è troppo piccolo per la domanda (es. "siamo sicuri che i 7 campi coprano tutti gli eventi?" rivelò che OI-92 era scritta male: misurare la copertura, non aggiungere termini). `why-new:` senza, un agente risponde "sì, il gate è verde" e chiude il finding invece di riaprirlo.

## 10. Il no-op silenzioso (non è in B1-bis)

Un blocco in `/tmp` con un `awk` il cui anchor non matcha: il file resta
**identico**, `wc -l` non si muove, il comando sembra riuscito. Non è
"awk che consuma il file" (quello è in B1-bis); qui il comando non ha
fatto niente. **Regola:** dopo ogni inserimento, `grep -c` la stringa
inserita, non solo `wc -l`. Pagato in questa sessione (una nota di
sessione non fu mai scritta, scoperta alla chiusura).

## 11. Il filtro di questa nota, e il suo changelog

**Il filtro (E2 step 2d, meccanico).** Una voce nuova entra solo se:
1. **`dup-check`**: il termine chiave **non** è già mandatory in
   `NEW-SESSION-PROMPT.md`. Se c'è, la voce è un **puntatore**
   (`vedi <regola>`), non una rispiegazione.
2. **`why-new:`**: una riga che dice cosa un agente nuovo
   **sbaglierebbe** senza la voce. Se non sai scriverla, la voce non
   entra.
3. **Una riga fisica** (max ~200 caratteri) con un titolo. Una cosa che
   richiede un paragrafo è un'idea o un ADR, non una nota.
4. **Tetto:** la nota non supera **200 righe**. Se un aggiornamento la
   porterebbe sopra, la sessione **fonde o rimuove** una voce prima di
   aggiungere, e lo dichiara (`replaced: §X with §Y, N lines saved`).
   Se non può, scrive `overflow: <motivo>` e decide il maintainer.

Artefatti nel report di chiusura: `dup-check: <termine> -> prompt:<regola>|none`,
`wc -l` della nota, `replaced:` se applicabile.

**Changelog:**
- **2026-11-09 — rev. 1** (LCV `316a01d`). Assorbe la nota 2026-11-08.
- **2026-11-09 — rev. 2** (LCV `34cf372`). Compattata: §1, §2 e il
  vocabolario erano già nel prompt, ora sono puntatori; nuove §5 (formule
  di salienza), §6 (alternative al tick), §9 (meta), §10 (no-op), §11
  (filtro). 224 -> ~180 righe. `dup-check`: A3, D6, P6, P7, glossario ->
  prompt (puntatori); pletora, tattica a due meccanismi, avanzata,
  `freedom_norm` 0.5, canarino, quattro alternative al tick, decisioni
  orali, no-op -> none (restano).
- **2026-11-09 — rev. 3** (LCV `8798623`). Filtro reso meccanico in E2 step 2d (`dup-check` solo contro `NEW-SESSION-PROMPT.md`, `why-new:`, una riga fisica, tetto 200 righe con `replaced:`/`overflow:`). La nota è a 195 righe. `Rules-Audit: 1 change, 0 contradictions, 0 leaks`.
- **2026-11-09 — rev. 4** (LCV `f4c8098`). Una voce in §9: i filtri giudicano il modello, non l'architettura (ADR-041 non filtrabile, ADR-042 sì e corretto dai filtri). `dup-check`: A3/D6 -> prompt (puntatori); "filtri modello vs architettura" -> none (nuovo). Nota a 196 righe, sotto il tetto 200.
- **2026-11-09 — rev. 5** (LCV `3ab20b2`). Una voce in §9: "siamo sicuri?" è una domanda, non un'assoluzione. `dup-check`: "siamo sicuri" -> prompt:none; "frasi brevi" -> prompt:none. `replaced: none`; **`overflow:` §4.3 ("le frasi brevi sono decisioni") non entra**, già coperta in spirito da §9 "domande-provocazione" e "le pause possono durare"; la nota è a 199 righe, sotto il tetto 200. `Rules-Audit` non richiesto (nessuna regola toccata).
- **2026-11-09 — no change** (session 11, LCV `48c8134`). Nessuna voce nuova dopo la rev. 5: ADR-043 (proposed) è un artefatto ADR, non una regola di lavoro; il filtro non ha prodotto nulla di non scritto altrove.
- **2026-11-10 — rev. 6** (LCV `b129afc`). Una voce in §8: ADR-044 proposed, la catena. `dup-check`: "catena" -> prompt:none; "caratteristiche vs proprieta fisiche" -> prompt:none (il vocabolario e in ADR-044, non nel prompt). `why-new:` chiaro. `replaced:` §6 (le quattro alternative al tick, 20 righe) con una sintesi di 6: IDEA-011 e atterrata, le alternative restano valutate, non serve il dettaglio. 16 righe risparmiate, la nota e a 186 righe, sotto il tetto 200. `Rules-Audit` non richiesto (nessuna regola toccata).
- **2026-11-10 — rev. 7** (LCV `e50fb4e`). Aggiornata la voce §8 (ADR-044 era `proposed`, ora `accepted`; aggiunto il tiro come direzione+restringimento+modalita, ADR-045). `dup-check`: "tiro" -> prompt:none; "modalita" -> prompt:none. `why-new:` chiaro. `replaced: none`, la nota e a 188 righe, sotto il tetto 200. `Rules-Audit` non richiesto (nessuna regola toccata).
- **2026-11-11 — rev. 8** (LCV `45c946d`). Una voce in §5: un filtro relativo al massimo non cambia l'argmax (serve un campo per-candidato indipendente dal punteggio, `visibility`). `dup-check`: "argmax" -> prompt:none; "filtro relativo" -> prompt:none. `why-new:` senza, un agente ripropone il filtro geometrico (errore pagato due volte l'11-11). `replaced: none`, 190 righe, sotto il tetto 200. `Rules-Audit` non richiesto per questa voce (la NOTA non è un file di regole; il prompt è stato toccato nel commit `4721835`, già con `Rules-Audit: 2 changes, 0 contradictions, 0 leaks`).
- **2026-11-11 — no change** (LCV `57f6c74`). Nessuna voce nuova: ADR-047 (la scelta e' riconoscimento) e' un artefatto ADR, non una regola di lavoro; il filtro (`dup-check`: "riconoscimento" -> prompt:none; "preferred_foot" -> prompt:none) non ha prodotto nulla di non scritto altrove. La lezione "il corpo non pesa, apre o chiude" e' la forma di ADR-047 §3, non una nota. 191 righe, sotto il tetto 200.
- **2026-11-12 — no change** (LCV `86e86b2`). Nessuna voce nuova: la lezione di metodo della sessione (`sed r` > `awk` per inserimenti, `git checkout` + riapplica invece di patchare) è già in B1-bis `630d0b6` (P3) e nelle lezioni esistenti; il posizionamento del portiere è un artefatto di codice (`CHANGELOG-SIM` 35, ADR-045), non una regola di lavoro. `dup-check`: "sed r" -> prompt:B1-bis; "positioning" -> prompt:none (è ADR-045/cantiere); "GK_LINE_OFFSET" -> prompt:none (è codice). 192 righe, sotto il tetto 200. `Rules-Audit` non richiesto (nessuna regola toccata).
