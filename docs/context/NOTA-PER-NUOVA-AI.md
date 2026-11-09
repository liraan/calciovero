# Nota per una nuova AI — Calcio Vero

**Status:** briefing (not normative, not an ADR, not a handoff).
**Ultima revisione:** 2026-11-09. **LCV al momento della revisione:** `316a01d`.
**Supersedes:** `docs/context/analysis/2026-11-08-nota-per-nuova-ai.md` (resta
nel repo come storia).

**Cos'è questo file.** La memoria delle cose emerse in conversazione che
non hanno un posto naturale nei documenti, e che un agente nuovo non
troverebbe leggendo solo il repo. **Non** è un handoff (quello è
`docs/context/analysis/*-session-handoff-*.md`, letto dallo startup
ritual). **Non** è una regola (quelle sono in `NEW-SESSION-PROMPT.md`).
È il "come si lavora", mai il "cosa è successo": se una voce risponde a
"dove siamo", va nel bookmark; se risponde a "come si fa", va qui.

**Lettura obbligatoria (E1, emendato 2026-11-09).** Ogni agente, nuovo
o no, legge questo file **in full** nello step 2 dello startup ritual,
dopo il bookmark e prima delle idee. Non è auto-caricato: è un comando
esplicito, e il report di step 5 dichiara se è stato letto.

**Aggiornamento obbligatorio (E2, emendato 2026-11-09).** A ogni
chiusura, prima del commit finale, la sessione aggiorna questo file:
aggiunge le voci nuove, corregge quelle stale, aggiorna la data e la
LCV. Se non c'è niente di nuovo, la sessione aggiunge una riga
`**YYYY-MM-DD — no change** (sessione N, LCV \`xxxxxxx\`)` nella sezione
"Changelog di questa nota". Il "niente da dire" è un artefatto
visibile, non un silenzio.

## 1. Il patto di collaborazione

Il maintainer, testualmente: *"il progetto è nostro, non mio, ci stiamo
lavorando entrambi e per quanto riguarda il codice sei tu l'esperto non
certo io"*. Operativamente:

- Sul **codice** l'agente è l'esperto: decide le forme tecniche, sceglie
  le strutture, propone le soluzioni.
- Sul **modello** e sulle **priorità** decide il maintainer (P6).
- L'agente **non** chiede il permesso per le scelte che una fonte già
  decide (direttiva 2026-11-07): deriva, applica, dichiara.
- L'agente **non** difende una posizione quando il maintainer corregge
  (P7): riformula e prosegue.
- **"Puoi anche contraddirmi se vuoi."** La contraddizione è attesa
  quando i filtri puntano in una direzione. Non è scortesia, è il patto.

## 2. I due filtri, obbligatori su ogni decisione di design

- **LUNA** (A3): *cosa rivela questo esempio che non sapevamo?* Inizia
  con `LUNA:`, non cita identificatori di codice; le conseguenze
  tecniche in `DITO:` solo se richieste.
- **Partita vera** (D6): *come si riconosce in una partita vera? da cosa
  dipende? chi lo produce?* Se il modello non può rispondere, è
  incompleto.

**La pletora non esiste.** In partita vera il giocatore con palla non ha
davanti venti opzioni: ne ha **due o tre chiare**, e spesso una sola. La
pletorà è lo sguardo dell'allenatore in panchina, non del giocatore. Il
modello **non deve dare un punteggio a tutto** ciò che è fisicamente
possibile: è il prototipo da cui ADR-040 è nato per uscire. Se una
proposta enumera tutte le azioni e prende l'argmax, questa frase è la
risposta.

**La tattica agisce in due modi, non uno.** (a) Cambia **cosa il
giocatore guarda** (un difensore a cui è detto "non salire" smette di
scansionare l'opzione di salita). (b) Cambia **quanto pesa** ciò che
vede (a chi è detto "tira appena puoi" il tiro pesa di più, il passaggio
resta visibile). Due meccanismi (percezione vs salienza), da tenere
separati quando L2 arriverà. A L0 non c'è nessuno dei due.

**L'avanzata non è passività.** "Portare palla" ha un esito osservabile:
il corpo va verso la porta, la palla resta ai piedi, il difensore viene
attirato. È un gesto, non un non-evento: per questo `ChoiceKind::Advance`
esiste come scelta e non come "nessuna azione".

## 3. La lezione di OI-88 (la trappola da non ripetere)

L'agente scrisse OI-88 come `ShotIntent` (potenza / piazzato / normale),
una **categoria scelta**. L0.11 §4.6 è esplicito: *"The catalogue has no
categorical term. The foot is not a stored label: `left_foot` and
`right_foot` are two Plane A performances, and the label is derived from
them, not read by a formula."* La forma corretta è due **scelte
continue** (`aim_point`, `power`); `ShotType` resta **derivato**.

**La domanda, ogni volta:** *questa distinzione è una scelta o una
lettura di una scelta?* Se è una lettura, va derivata; se è una scelta,
deve essere continua, non un enum. La stessa trappola è tornata con la
traiettoria del passaggio ("alto/teso" è una scelta continua) e con
l'avanzata (`advance_room` come formula competitiva: sbagliata, vedi §5).

## 4. Il vocabolario: quattro parole, quattro significati

Non sono sinonimi, e citarne uno per l'altro è un errore D1:
**verificabile** (D5: il maintainer può controllarlo senza fidarsi
dell'agente), **tracciabile** (A2: l'alternativa è scritta come OI con
una prima azione), **artefatto** (D5: ciò che rende visibile una
condizione legata al giudizio — un trailer, un campo nel report, una
riga di `git log`), **fonte di verità** (`STRUCTURE.md` §5: il posto
unico dove un numero o un fatto vive). Overlap sì, sinonimi no.

## 5. Regole per le formule di salienza (dalla sessione 2026-11-09)

**Misura la presenza di un segnale che varia, non l'assenza di una
condizione rara.** `advance_room` tornava 1.0 quasi sempre, perché
"nessun avversario davanti" è la condizione **normale** in un campo: una
salienza costante non è una salienza, non può competere con un passaggio
che vale 0.1–0.4. Qualsiasi formula futura (aereo, portiere) deve
**guardare la distribuzione del proprio output**: se il 90% dei casi
vale 1.0, la formula è rotta. La forma atterrata per l'avanzata è a
**soglia** (`clarity_threshold`), non a competizione.

**`freedom_norm` ha un pavimento a 0.5, e nessuno lo sa.** La formula
del passaggio è `freedom / (freedom + PASS_FREEDOM_CM)`; `build_choice`
salta i riceventi con `freedom < PASS_FREEDOM_CM`, quindi il minimo è
0.5, non 0. Toccare `PASS_FREEDOM_CM` "per rendere più severo il
filtro" cambia anche la forma della salienza, senza che sia ovvio.

**La salienza misura la scena; la cognizione decide cosa farne.** Tre
strati, già in codice: la lettura allarga ciò che si vede, la capacità
di decidere sposta la soglia, la gestione del rischio pesa le opzioni.
Non si fondono in un unico punteggio.

**La soglia ha un rischio aperto.** `clarity_threshold` fisso a 0.25: se
i punteggi di un'intera squadra stanno sotto, quella squadra porta palla
sempre e il motore non lo segnala. La calibrazione (L3) deve controllare
per squadra e per livello.

**Il test funzionale è un canarino deliberato.** `functional_l0_the_engine_emits_the_claimed_shape`
fallisce se la soglia sale o la salienza scende ("no Pass emitted"). È il
guardiano della soglia, non un test da rilassare.

## 6. Le quattro alternative al tick (se un agente chiede "cambiamo l'orologio?")

Solo la prima è tracciata (IDEA-DESIGN-011). Le altre tre vivono qui:

- **Event-driven discreto.** Il motore salta da un evento al successivo.
  Costo alto: perdi il sottofondo (movimento, copertura, sguardo) che il
  tick dà gratis, e devi dimostrare che "tra un evento e l'altro non
  cambia niente" (OI-26, ancora aperto). Il 2D (IDEA-INFRA-018) vuole
  fotogrammi: senza tick, il fotogramma è un'invenzione del renderer.
- **Passo variabile (adattivo).** Piccolo vicino al contatto, grande
  quando non succede niente. Deterministico solo se il passo è funzione
  dello stato; la regola "quando lo stato cambia" nasconde accoppiamenti.
- **Due orologi dichiarati.** Lento per le decisioni, veloce per
  movimento e palla. È **già in parte** la forma di oggi (scan ogni 5
  tick). Costo basso, la più economica da provare, ma va dichiarato e
  testato che nessuna decisione venga saltata.
- **Fase per agente.** Ogni agente ha il suo battito, sfalsato. Toglie
  l'artefatto "tutti decidono nello stesso tick", ma cambia l'ordine di
  iterazione: `SIM_VERSION` bump e prova che l'ordine non contamina.

**La frase del maintainer, da citare a chi propone il cambio:**
*"non cambiare l'orologio, cambia chi ha il permesso di fermarlo"*.
IDEA-011 è la sua forma operativa: il tick resta, `action_roll` sparisce,
la latenza è dell'attore.

## 7. Decisioni orali del maintainer (non scritte altrove)

- **Bump di versione dei crate: no, per ora.** La versione è `0.1.0` in
  tutto il workspace. "Non è urgente, aspettiamo ADR32": l'idea era
  bumpare a `0.2.0` quando il rename `pcfm-*` -> `ds-*` atterra, così
  "versione nuova" e "prefisso nuovo" coincidono in un evento
  dichiarato. **Non bumpare senza una sua parola.**
- **ADR32 (rename `pcfm-*` -> `ds-*`) è in lista, non urgente, ma va
  fatto.** Testuale: *"il rename lo mettiamo in lista, non è urgente ma
  è da fare, tutto lì"*. Già `[open]` nel Plan con la sua prima azione.
- **Codeberg è backup, GitHub è primario.** Testuale: *"github come
  principale e codeberg come backup, all'evolversi del progetto magari
  faremo la migrazione"*. La regola operativa è in `AGENT-POLICY.md` §6.
- **IDEA-DESIGN-007** (caratteristiche da statistiche reali, giocatori
  fittizi) è un'idea, non una decisione. Prerequisito: leggere
  `pcfm-fixtures` (chi riempie altezza, peso, ruolo oggi).

## 8. Cose che non hanno un OI ma vanno sapute

- **`resolve_duel` è ancora prototipo** (legge un dado). Debito noto in
  Plan E (OI-50): non è un bug nuovo.
- **Il tick gira il carrier verso la porta prima di `build_choice`**
  (OI-89): il `heading` del carrier è sempre verso il centro porta. Non
  "correggerlo" senza leggere OI-89.
- **`PassContext.receiver_freedom_cm` è un orfano** (OI-90): il commento
  dice "carried to the reception", nessuna funzione lo legge.
- **`scripts/closing.sh` non scrive l'handoff.** C2, gates, capture. Se
  il maintainer chiede l'handoff prima di "chiudi", va scritto **prima**
  del commit finale.
- **Il conteggio test nei documenti** (README, STRUCTURE, CONTRIBUTING,
  `specs/_index`) può essere stale rispetto al reale; `A4-coherence`
  controlla i documenti fra loro, non col reale. Verificare con C2.

## 9. Meta-istruzioni osservate in sessione

- **"Filtra la decisione con calcio vero e luna, poi presentami la tua
  conclusione."** Quando una decisione è bloccata, il maintainer vuole
  una **conclusione**, non un menu di opzioni (P6 rinforzato). Un agente
  che riceve un menu dopo questa frase ha sbagliato la forma.
- **"Voglio il tuo granello di sabbia"** = contribuisci con una
  proprietà che l'esempio rivela, non con un'assoluzione. La forma è
  `LUNA:`.
- **Domande-provocazione.** "Allora ad oggi questo non è un motore
  fenomenologico è un motore a tick" era una provocazione per testare la
  risposta, non un'affermazione. La risposta giusta distingue ciò che è
  vero da ciò che è mezzo vero: substrato vs crepa, pezzi fatti vs pezzi
  dichiarati.
- **Le pause del maintainer possono durare a lungo e produrre più
  decisioni.** "Pausa" non è "fermati": è "discutiamo". Al "riprendi",
  il primo comando è E5 (catturare), non riprendere il codice.

## 10. Un no-op silenzioso che non è in B1-bis

Scrivere un blocco in `/tmp`, `awk` con un anchor che non matcha: il
file resta **identico**, `wc -l` non si muove, il comando sembra
riuscito. Non è la trappola "awk che consuma il file" (quella è in
B1-bis); qui il comando non ha fatto niente. **Regola:** dopo ogni
inserimento, `grep -c` la stringa inserita, non solo `wc -l`. Il
conteggio righe non distingue "inserito" da "non inserito" se l'anchor
non matcha. Pagato in questa sessione (la nota di sessione "terza" non
fu mai scritta, scoperta alla chiusura).

## 11. Changelog di questa nota

- **2026-11-09 — revisione 1** (LCV `316a01d`). Assorbe la
  `2026-11-08-nota-per-nuova-ai.md`, che resta come storia. Nuove voci:
  §2 (pletora, tattica a due meccanismi, avanzata non passività), §5
  (formule di salienza), §6 (quattro alternative al tick), §9 (meta),
  §10 (no-op silenzioso). La lettura è obbligatoria a inizio sessione
  (E1) e l'aggiornamento a fine sessione (E2), emendati oggi.
