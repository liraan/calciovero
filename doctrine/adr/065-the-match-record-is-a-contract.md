# ADR-065 — The match record is a contract

- **Status:** Accepted (delegated, 2026-11-16) — to be reviewed by the maintainer
- **Date:** 2026-11-16
- **Decision:** DD-6, from decision U (`context/plans/DECISIONS-2026-11-15.md`; the mandate of 2026-11-16, `context/STATE.md`)
- **Amends:** IDEA-INFRA-018 (the tick record was "derived, never saved"); `L0-match/specs/07-save-minimal.md` (the save gains an optional field, schema v3)
- **Depends on:** ADR-048 §4 (the ball's physics as data), ADR-055 (the GUI behind the view model), ADR-061 (Android is the same game)

*Written in Italian as the project's working language; the decision is binding as written.*

## Contesto

Le viste (il 2D a caratteri, il 2.5D della GUI, un giorno il 3D) disegnano la registrazione dei tick: dove sono i ventidue e il pallone, dieci volte al secondo, chi ha la palla, il punteggio, gli eventi nati in quel tick (`ds-match/src/record.rs`). Finora la registrazione non entrava nel salvataggio: si rigenerava dal seme. Funziona finché il motore è quello che ha giocato la partita. Con un motore nuovo la stessa partita, dallo stesso seme, è un'altra partita (`can_replay()` è falso): il risultato salvato resta, ma la partita non si può più guardare. In più il 2.5D faceva volare il pallone fra due tick con la gravità del motore di oggi, non con quella della partita. La decisione U del maintainer fa della registrazione un contratto pubblico.

## LUNA

Una partita giocata è un fatto: resta com'è stata, chi la guarda dopo vede quella partita, non un'altra rigiocata da un motore diverso.

## Partita vera

- **Come lo riconoscono gli attori:** la partita rivista, la sintesi, il gol rivisto il giorno dopo; è la stessa partita che hanno giocato.
- **Da cosa dipende:** solo da ciò che è successo in campo, registrato. Non dal motore di oggi.
- **Chi lo produce:** la partita stessa; le viste la mostrano e basta.

## Decisione

1. **Per le partite finite il salvataggio tiene la registrazione dei tick, compressa.** `MatchSave.record: Option<PackedRecord>`, con `MatchSave::with_record`. Lo schema del salvataggio passa da v2 a v3. Il campo è facoltativo: un salvataggio v2 non lo ha e si carica com'è, senza migrazione; il test `a_v2_save_still_loads` lo prova. Una registrazione di un altro motore rispetto al salvataggio è rifiutata (`SaveError::RecordMismatch`). La compressione: colonna per colonna (ogni campo di tutti i tick insieme); ogni valore come differenza dalla previsione (il valore prima; per le posizioni il valore prima più l'ultimo passo); varint zig-zag; poi deflate (`miniz_oxide`, puro Rust, versione fissata) e base64, perché il salvataggio è JSON. Una partita del seme 42 passa da 131 MB di JSON a 1,9 MB, base64 compreso. La partita torna identica, tick per tick (`a_match_comes_back_exactly`).
2. **Una versione del formato della registrazione, separata da `SIM_VERSION`:** `RECORD_FORMAT_VERSION` (v1 sono i campi di `TickRecord` al SIM 137). Il motore può cambiare il calcio senza cambiare la forma della registrazione, e ogni registrazione tiene la versione con cui è stata scritta. Le regole:
   - **un campo nuovo è solo facoltativo:** è una colonna nuova in fondo, letta solo dalle registrazioni del formato che la ha; una registrazione più vecchia gli dà il suo valore di riserva, e la vista disegna il suo comportamento di riserva;
   - **un campo non si toglie e non cambia significato:** sarebbe un formato nuovo, con la sua migrazione;
   - **un formato più nuovo di quello della build si rifiuta,** non si legge male (`RecordError::FormatTooNew`);
   - **un dato rovinato è un errore,** mai un panico né una partita sbagliata.

   I primi 30 tick del seme 42 al formato v1 sono congelati in `ds-match/tests/fixtures/record-v1.json`: se il formato cambia senza la sua versione, `the_v1_record_still_reads` fallisce.
3. **La tabella evento → animazione, con una posa di riserva** (`tools/ds-gui/src/animation.rs`). Ogni tipo di evento ha la sua riga. Gli eventi senza un gesto proprio hanno la posa di riserva `Rest`: l'uomo com'è, in corsa o fermo, mai un gesto inventato. La tabella è un `match` senza ramo generico, quindi un tipo nuovo non compila finché non ha la sua riga. Il test `every_event_type_has_its_row` legge i tipi di evento dal loro sorgente e fallisce se uno manca. Undici tipi muovono un corpo, come nella tabella di `DESIGN.md`; gli altri ventidue hanno la posa di riserva.
4. **I parametri fisici nell'intestazione della registrazione:** `RecordHeader { format_version, sim_version, physics }`, dove `physics` è `RecordPhysics`:
   - i tick al secondo;
   - il pallone di `balance/ball.json` (gravità, attriti, restituzione);
   - il campo in metri.

   Il 2.5D fa volare il pallone fra due tick con la gravità della registrazione, non con quella del motore di oggi.
5. **Il motore non si cambia mai per la vista.** Un campo nuovo nella registrazione nasce da un cambio di calcio, con il suo ADR e il suo `SIM_VERSION`; poi la registrazione lo porta e la vista lo mostra. Le viste (2D, 2.5D, 3D) dipendono solo dal formato della registrazione, mai dal motore.

## Conseguenze

- Questo ADR non cambia il calcio: nessun `SIM_VERSION`, i tre golden identici. La registrazione non entra nell'hash dello stato.
- Il salvataggio di una partita con registrazione pesa circa 2 MB. Una stagione con le partite della propria squadra (38, più le coppe) è sotto i 100 MB. Le partite degli altri non si registrano: di quelle restano il risultato e gli eventi. Se il peso pesa, la scelta di quali partite tenere va al giocatore (ADR-049).
- `ds-demo --save` salva la registrazione. `--load` avvisa che la partita giocata è conservata quando il motore è cambiato. Una vista che la mostra a schermo da un salvataggio è il passo dopo, con la carriera (L1).
- Il passaggio del salvataggio a `postcard` (spec 07) resta aperto e non cambia con questo ADR: la registrazione impacchettata è già binaria, dentro una stringa.

## Alternative scartate

- **Rigiocare dal seme con il motore vecchio:** vorrebbe dire tenere ogni motore di ogni versione dentro il gioco.
- **La registrazione come JSON compresso:** 9,2 MB a partita contro 1,9 MB, e un JSON che cambia forma a ogni campo nuovo.
- **Una nuova versione del salvataggio a ogni campo nuovo della registrazione:** lega due cose che cambiano per ragioni diverse. La versione del formato della registrazione esiste per questo.

## Note datate

- **2026-11-16 — gli input futuri della partita** (il piano di L2, `context/plans/L2.md`, decisioni AZ e BB): le misure e il manto del campo (0.4, fisica di grado 1: le misure vere, i coefficienti del manto dalle norme e dagli studi) e l'intesa di ogni coppia di compagni (0.4, la precisione con cui uno immagina il movimento dell'altro quando sceglie il passaggio, grado 2 sugli effetti degli studi). Entrano nel contratto della partita quando arriva il loro blocco, ognuno con il suo `SIM_VERSION` e, se la registrazione li deve portare, con un campo facoltativo nuovo e la sua `RECORD_FORMAT_VERSION` (§2). Non sono nella lista di L0 di oggi.
