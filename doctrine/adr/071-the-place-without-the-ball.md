# ADR-071 — The place without the ball

- **Status:** Accepted (delegated, 2026-11-17) — to be reviewed by the maintainer
- **Date:** 2026-11-17
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
5. **Senza palla** (SIM 153, il passo dopo): gli spazi che chiude, per lo stesso ragionamento, dentro le tendenze della copertura e della compattezza (18 e 19: il secondo copre lo spazio se il primo è saltato; chi è lontano dalla palla si stringe verso il centro e la porta) e del lato della porta (20).
6. **Nessun numero nuovo:** i corridoi sono la larghezza del campo divisa in cinque; la linea viene dalla profondità del blocco; i valori dalla misura che c'è.

## Conseguenze

- `shape.rs` resta: dà la zona di partenza. Le istruzioni della lavagna (ADR-070) spostano ancora la zona e aggiungono responsabilità.
- Il test di scena viene dallo strumento delle misure. Al SIM 152, sul golden: i due più avanzati da 30,4 a 27,3 m davanti al pallone; la direzione dei passaggi non cambia (23% in avanti, 23% indietro, contro 31% e 16% della Serie A 2017-18): la causa dei passaggi all'indietro non è il posto di chi tiene la posizione, ma la scelta di chi ha la palla e di chi si offre (il passo dopo).
- Le coppie dello strato 3 (i movimenti complementari, le chiamate) si costruiscono sopra: la previsione del compagno entra nel valore dello spazio.

## Alternative scartate

- **Una formula geometrica migliore:** resterebbe un punto calcolato, non una scelta; il Subbuteo cambierebbe forma, non natura.
- **Una griglia fitta di candidati:** il linguaggio del calcio è fatto di corridoi e linee, non di metri; e il costo crescerebbe senza che il giocatore pensi meglio.
