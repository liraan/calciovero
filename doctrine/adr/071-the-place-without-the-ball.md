# ADR-071 — The place without the ball

- **Status:** Accepted (delegated, 2026-11-17) — to be reviewed by the maintainer; emended 2026-11-19 (test del burattino, §Decision 5), emendamento attuato come causa separata in SIM 165 (2026-11-20)
- **Date:** 2026-11-17; emendamento 2026-11-19
- **Decision:** DD-13, from decisions BK and BL (`context/plans/BEHAVIOURS.md`, layer 2, approved by the maintainer; his corrections of 17 November: cover and compactness are tendencies of the player, layer 2, in L0)
- **Amends:** the block of `shape.rs` (VISION §3, ADR-059 §3) as the way a man chooses his place: it stays as the start zone of his role
- **Depends on:** ADR-052 (every player thinks, §5: the acts without the ball chosen with the measure of the game), ADR-059 §4 (the discipline rule), ADR-062 (he sees what he looked at)

*Written in Italian as the project's working language; the decision is binding as written.*

## Contesto

Il maintainer ha guardato la partita: "sembra più Subbuteo che calcio". Chi ha la palla pensa con la misura del gioco; gli altri venti, quando tengono il posto, vanno in un punto calcolato dalla formula geometrica di `shape.rs`, agganciato al pallone. Lo strumento delle misure (`ds-reference scene`, golden al SIM 151) lo mostra: i due più avanzati stanno 30 m davanti al pallone, fermi; il 23% dei passaggi riusciti va indietro contro il 16% della Serie A, e il 22% in avanti contro il 31%; con la palla, il pallone sta nella propria metà il 79% del tempo.

## LUNA

Senza palla un calciatore occupa lo spazio che vale di più per la sua squadra, partendo dal suo ruolo: non un punto, uno spazio scelto.

## Partita vera

- **Come lo riconoscono gli attori:** l'attaccante che viene incontro fra le linee quando la palla è dietro, l'ala che resta larga mentre il terzino entra dentro, nessuno nello stesso corridoio di un compagno, i centrali dietro la palla.
- **Da cosa dipende:** dallo spazio che vede (ADR-062), dal suo ruolo, dalla sua disciplina e dalla sua lettura.
- **Chi lo produce:** il giocatore, con la sua scelta (ADR-049); la forma dell'allenatore dà solo la zona da cui parte.

## Decisione

1. **La forma resta, come zona di partenza del ruolo:** il punto di `shape.rs` è il suo posto nel disegno dell'allenatore (la lavagna, decisione AL). Non è più dove sta: è da dove guarda.
2. **Con la palla, i candidati** (SIM 152). Il linguaggio dello spazio di `BEHAVIOURS.md`:
   - **corridoi:** il campo in cinque corridoi (le due fasce, i due mezzi spazi, il centro), la larghezza del campo divisa in cinque; i candidati stanno nel suo corridoio e nei due vicini;
   - **linee:** il suo posto, e una linea più avanti e una più indietro; una linea è un terzo della profondità del blocco con la palla (`block.json`, tre reparti); mai oltre la linea del fuorigioco, corta del margine di `offball.json`.
3. **Con la palla, il valore di uno spazio** è la misura del gioco (ADR-052), quella che il pensiero usa già per offrirsi, e la distanza a cui il portatore può giocargli il pallone (la distanza plausibile di ADR-040):
   - quanto vale lo spazio per la squadra (`read::space_value`);
   - quanto vi sarebbe libero (`read::soon`, contro l'avversario più vicino che vede);
   - quanto è pulita la linea di passaggio dal portatore (`read::line_quality`).

   Due regole dell'occupazione, dello strato 2:
   - **mai due compagni nello stesso corridoio e sulla stessa linea:** uno spazio con un compagno nel suo corridoio, a meno di mezza linea, vale zero per lui;
   - **la difesa preventiva:** chi ha il posto nella linea difensiva della forma resta dietro la palla.
4. **La zona è la disciplina.** I candidati non escono dalla sua zona (il suo corridoio e i due vicini, una linea avanti o indietro): restarci è tenere il disegno dell'allenatore; dentro la zona sceglie lui lo spazio che vale di più. La disciplina continua a pesare dove pesava (ADR-059 §4): quanto tiene il posto rispetto alle altre azioni (offrirsi, attaccare la profondità), e quanto segue i compiti della lavagna. Una prima versione faceva passare anche lo spazio per la regola `obey`, con la zona come richiesta: il disciplinato non si muoveva quasi mai dal punto della forma, e il Subbuteo restava (misurato: i due più avanzati da 30,4 a 29,7 m davanti al pallone).
5. **Senza palla** (SIM 154): gli spazi che chiude, per lo stesso ragionamento, dentro le tendenze della copertura e della compattezza (18 e 19: il secondo copre lo spazio se il primo è saltato; chi è lontano dalla palla si stringe verso il centro e la porta) e del lato della porta (20). I candidati sono quelli del §2 — il suo corridoio e i due vicini, la sua linea e una avanti o indietro — con la linea presa dalla profondità del blocco senza palla. Uno spazio vale quanto vale agli avversari (`read::space_value` verso la nostra porta) e quanto presto uno di loro può esserci (`read::soon`), il portatore escluso: il suo spazio è il pressing, un altro atto. Lo chiude chi dei compagni è più vicino a quello spazio: due zone non si sovrappongono. Il peso è dalla parte della porta (20); la copertura dietro chi pressa resta un atto suo (18, `Cover`).

   **Emendamento 2026-11-19 — il test del burattino.** La frase "ogni spazio pesa zero e resta il posto dell'allenatore" è caduta: è il burattino. L'ordine in campo deve essere un effetto della cognizione del giocatore, non un punto imposto dalla forma (obiettivo del maintainer, `context/plans/L0-CALIBRATION.md`). Nel codice dell'epoca (`state.rs::choose_closed_place`) il valore di ogni spazio è `danger * soon`: lontano dalla palla `soon = 0` per tutti, quindi ogni spazio pesa zero e il giocatore resta al punto geometrico della forma — e cambiare la sua lettura, la sua intesa o la sua abilità non cambia nulla. Il test del burattino (cambiare solo la lettura, lo stesso atto deve cambiare) qui fallisce.

   La regola emendata: quando nessun avversario arriva presto in zona, lo spazio non pesa zero — pesa quello che il **giocatore** vi legge con la sua cognizione (lesson del §3: `read::space_value`, `read::soon`, qualità della linea, lettura del compagno), dentro la zona che la disciplina gli assegna. La forma resta da dove guarda (§1), non dove finisce. Nessun numero nuovo: si usa la misura del gioco già presente. L'attuazione è una causa separata, con il suo `SIM_VERSION`, golden, test gemello (lo stesso anchor, cambia solo la lettura: il punto scelto deve cambiare) e misura sulle 60 partite. Attuata in SIM 165: quando nessun avversario arriva presto, la lettura del giocatore sceglie la linea nel suo corridoio; i corridoi accanto restano pesati dal gioco, così l'ala non abbandona la fascia. Gemello `the_reader_leaves_the_given_place_the_blind_man_keeps_it`; intatti il test della pressione e quello del piede.
   **Nota 2026-11-21 (DD-41) — la regola cambia casa, non contenuto.** `choose_closed_place` e la sua scena (`ClosedPlaceScene`) si spostano da `ds-match/src/state.rs` a `ds-orchestrator/src/offball.rs`, il read dell'attore (ADR-041, ADR-043, ADR-099 §4): è il giocatore che legge quali spazi valgono, quindi la valutazione sta con la lettura, non con l'applicazione delle regole. Nessuna formula, peso o soglia cambia: ADR-071 §5 resta la regola, il gemello `the_reader_leaves_the_given_place_the_blind_man_keeps_it` resta il gate. I riferimenti `state.rs::choose_closed_place` qui sopra e in `BEHAVIOURS.md` sono storici (dove la regola è nata nel SIM 165).

6. **Nessun numero nuovo:** i corridoi sono la larghezza del campo divisa in cinque; la linea viene dalla profondità del blocco; i valori dalla misura che c'è.

## Conseguenze

- `shape.rs` resta: dà la zona di partenza. Le istruzioni della lavagna (ADR-070) spostano ancora la zona e aggiungono responsabilità.
- Il test di scena viene dallo strumento delle misure. Al SIM 152, sul golden: i due più avanzati da 30,4 a 27,3 m davanti al pallone; la direzione dei passaggi non cambia (23% in avanti, 23% indietro, contro 31% e 16% della Serie A 2017-18): la causa dei passaggi all'indietro non è il posto di chi tiene la posizione, ma la scelta di chi ha la palla e di chi si offre (il passo dopo).
- Al SIM 154, sul golden (i dieci semi 42-51, `ds-reference scene`): il blocco senza palla, i due più larghi da 39,2 a 40,0 m (vero 37,0, Metrica partite 1 e 2), i due più profondi dietro il pallone da 33,3 a 32,7 m; i passaggi in avanti / laterali / indietro 23/53/24 → 25/52/23 (vero 31/53/16); i due più avanzati davanti al pallone da 26,8 a 27,2 m; il resto fermo (salti 0, il pallone nella propria metà 74,4 → 74,7%). Il ginocchio del portatore non cambia: il suo spazio è il pressing.
- La larghezza del blocco senza palla si allontana dal vero di 0,8 m (era 2,2 m sopra i 37,0 veri, ora 3,0): la scelta non tocca il disegno del blocco, che resta quello dell'allenatore e dei suoi numeri (`block.json`); la sua misura resta aperta per la taratura.
- Le coppie dello strato 3 (i movimenti complementari, le chiamate) si costruiscono sopra: la previsione del compagno entra nel valore dello spazio.
- Le misure del SIM 154 elencate sopra descrivono il comportamento **prima** dell'emendamento del 2026-11-19: il "resto fermo" e il "pallone nella propria metà al 74,7%" sono il burattino che l'emendamento vuole sciogliere. Vanno rimisurate quando la causa separata sarà attuata; il test gemello che cambia solo la lettura è il gate di accettazione, non la tabella delle distanze.

## Alternative scartate

- **Una formula geometrica migliore:** resterebbe un punto calcolato, non una scelta; il Subbuteo cambierebbe forma, non natura.
- **Una griglia fitta di candidati:** il linguaggio del calcio è fatto di corridoi e linee, non di metri; e il costo crescerebbe senza che il giocatore pensi meglio.
