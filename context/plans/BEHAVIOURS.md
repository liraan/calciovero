# I comportamenti in campo (bozza, da proporre al maintainer)

Prima delle statistiche, i comportamenti (maintainer, 17 novembre 2026): le statistiche dicono *quanto*, i comportamenti dicono *cosa*. Tarare i numeri su giocatori che fanno le cose sbagliate dà una partita che torna nei conti e sembra Subbuteo. Questo catalogo dice cosa fa un calciatore in campo; la taratura viene dopo, sopra comportamenti giusti. Passato con i filtri partita vera e LUNA.

## Il principio: tendenze, non copioni

Non si ragiona per problemi, si ragiona per **tendenze**: le inclinazioni che un calciatore ha durante la partita. Non sono copioni: il motore non comanda mai un giocatore (`VISION.md`; ADR-043, la scelta è dell'attore). Una tendenza è quello che il pensiero di ogni giocatore arriva a fare da sé, dalla sua percezione (ADR-062: vede quello che ha guardato, non la verità) e dalle sue caratteristiche. LUNA: nessun dado, nessun bonus; una tendenza è una causa nel pensiero.

Le tendenze vengono da **due fonti**, e la differenza conta:

| Fonte | Cosa decide | Da dove viene la sua forza | Si cambia |
|---|---|---|---|
| **Il calciatore** | il comportamento generale: come si sta in campo, in ogni squadra | le sue caratteristiche, soprattutto la famiglia cognitiva (lettura del gioco, decisione, visione, concentrazione), che cresce con l'esperienza, cioè con le partite giocate (decisione AS) | no: è lui |
| **La tattica** | il comportamento della squadra: dove, quando, con quale forma | la lavagna dell'allenatore (decisione AL): i disegni, le responsabilità, le scelte | sì: la lavagna, prima e durante la partita |

**Come si combinano.** La tattica dice dove stare e cosa fare in quella fase; il calciatore lo fa con le sue tendenze. La disciplina decide quanto si tiene alla tattica quando la tendenza spinge altrove; la lettura del gioco decide quanto bene sceglie il momento; il corpo decide quanto bene lo fa (`L0-match/design/CHARACTERISTICS-AND-ABILITIES.md` §6, i due filtri del ruolo). Un giocatore esperto e disciplinato fa la cosa giusta nel posto giusto; uno giovane la fa in ritardo, uno indisciplinato la fa dove vuole lui.

**L'ossatura** sono i principi di gioco classici, quelli insegnati nei corsi per allenatori (in attacco: penetrazione, sostegno, ampiezza, mobilità, improvvisazione; in difesa: temporeggiare, coprire, equilibrare, stringere, controllare), letti per il singolo giocatore.

## Gli strati (maintainer, 17 novembre 2026)

Il catalogo si costruisce dalla base, nell'ordine in cui si impara il calcio:

1. **Come si gioca a pallone:** lo scopo del gioco e la sua misura (scritto qui sotto, approvato).
2. **Il posizionamento e le responsabilità di ogni ruolo:** chi occupa quale spazio e perché, con e senza la palla (scritto qui sotto, approvato).
3. **Le coppie affiatate:** i due centrali, il terzino e l'ala, le due punte, il portiere e i centrali, la coppia di centrocampo; l'intesa nasce dalle giocate riuscite insieme (decisione BB) (scritto qui sotto, approvato).
4. **La tattica:** la lavagna dell'allenatore (decisione AL; parte 2 qui sotto).

Le tendenze del calciatore della parte 1 si ridistribuiscono negli strati 1 e 2 man mano che si scrivono.

## Strato 1 · Come si gioca a pallone (approvato dal maintainer)

La base esiste già nella dottrina: è l'ADR-052 (ogni giocatore pensa), con la sua nota del 14 novembre (il primo obiettivo è il pallone). Qui è detta come la vive un calciatore.

1. **Lo scopo:** fare più gol dell'avversario nel tempo della partita, dentro le regole. Da qui l'unica misura del gioco (ADR-052 §3): ogni cosa che un giocatore fa vale quanto avvicina la sua squadra al gol e quanto la allontana dal subirlo.
2. **Si gioca con tre risorse:** il pallone (chi lo ha decide), lo spazio (dove si può giocare), il tempo (quanto se ne ha prima della pressione). Ogni gesto scambia l'una con l'altra: un passaggio laterale compra tempo, un dribbling rischia il pallone per lo spazio.
3. **Il primo obiettivo è avere il pallone** (maintainer, 14 novembre): un pallone libero vale più di qualunque posto nella forma. Ma è un mezzo, non il fine: si tiene finché non si può avanzare, e se si perde, meglio lontano dalla propria porta.
4. **Il gioco ha un verso, in avanti:** il valore di una posizione cresce verso la porta avversaria; un possesso che non avanza vale solo come sicurezza.
5. **Si gioca in undici:** ognuno decide per sé, ma la misura è della squadra; nessuno tiene la palla per sé né corre per sé.
6. **L'avversario gioca allo stesso gioco:** per questo si legge quello che sta per fare, e si anticipa.
7. **Il gioco ha delle fasi,** che il giocatore riconosce da quello che vede, non stati che il motore cambia: con la palla, senza la palla, le due transizioni, le palle ferme.

**Cosa vuol dire per il motore, e perché oggi sembra Subbuteo.** L'ADR-052 §5 dice che anche gli atti senza palla (pressare, coprire, smarcarsi, offrirsi, marcare) si scelgono con la stessa misura. Oggi chi ha la palla pensa; gli altri venti sono messi al loro posto da una formula geometrica (`shape.rs`: la linea a 30 m dietro il pallone, non sotto i 10 m dalla porta, gli attaccanti in cima al blocco). Un giocatore su ventidue usa la misura del gioco, gli altri sono pezzi agganciati al pallone. Lo strato 2 è dove questo si ripara: ogni giocatore senza palla occupa lo spazio che vale di più per la sua squadra, partendo dal suo ruolo, invece di stare in un punto calcolato.

## Strato 2 · Il posizionamento e le responsabilità di ogni ruolo (approvato dal maintainer)

Calcio moderno: non la posizione, l'occupazione dello spazio (nota dell'ADR-052 del 13 novembre).

**Un ruolo è un insieme di funzioni, non un punto.** Il terzino che entra dentro al campo resta un terzino, il centravanti che viene incontro resta un centravanti: cambia lo spazio che occupano. Per ogni ruolo: la zona di partenza (il corridoio e la linea), le responsabilità in ogni fase, le relazioni (con chi stare in coppia, con chi non sovrapporsi).

**Il linguaggio dello spazio,** quello che ogni giocatore percepisce: cinque corridoi (due fasce, due mezzi spazi, il centro); le linee avversarie (la prima linea di pressione, quella di centrocampo, l'ultima); gli spazi fra le linee. Le regole dell'occupazione: mai due compagni nello stesso corridoio e sulla stessa linea; qualcuno dà ampiezza, qualcuno profondità; sempre una linea di passaggio per chi ha la palla; sempre qualcuno dietro la palla (la difesa preventiva).

**Come funziona nel motore (LUNA).** Il giocatore senza palla non va in un punto calcolato: guarda gli spazi intorno alla sua zona di partenza e sceglie quello che vale di più per la squadra, con la misura dello strato 1. Quanto vale uno spazio: la minaccia se la palla gli arrivasse lì, la linea di passaggio pulita che dà, la struttura (non sovrapporsi ai compagni), il rischio se la palla si perdesse adesso; senza palla, quale spazio pericoloso chiude. Il ruolo dice da dove parte e quali responsabilità vengono prima; le sue caratteristiche quanto bene legge il momento; la tattica (strato 4) sposta la zona e aggiunge responsabilità. Sostituisce, con il suo ADR, la formula geometrica di `shape.rs` come modo di scegliere il posto: la forma resta come zona di partenza.

### I ruoli di base

Pochi ruoli di base; le varianti (il terzino invertito, il falso nove, il portiere libero, l'ala a piede invertito) sono responsabilità che l'allenatore assegna sulla lavagna (decisione AL), con la taratura fine sua (maintainer).

| Ruolo | Con la palla | Senza la palla | Nelle transizioni |
|---|---|---|---|
| **Portiere** | partecipa alla costruzione, l'uomo in più dietro; sceglie fra corto e lungo | difende la porta e lo spazio dietro la linea (portiere libero) | appena vinta, riparte subito con la mano o il piede |
| **Difensore centrale** | imposta: si allarga fuori dall'area, porta palla nello spazio libero, rompe la prima linea | guida la linea, copre, marca in area, sta fra l'uomo e la porta | appena persa, protegge la profondità |
| **Terzino** | dà ampiezza, o entra nel mezzo spazio, si sovrappone | chiude la sua fascia, si stringe quando la palla è lontana | appena persa rientra, appena vinta spinge |
| **Mediano, regista** | il vertice basso: sempre una linea di passaggio, cambia gioco | protegge il centro davanti alla difesa, scala | il primo a riaggredire o a coprire |
| **Mezzala, interno** | occupa il mezzo spazio fra le linee, si inserisce | esce in pressione, copre il mezzo spazio | arriva in area, o rientra |
| **Trequartista** | riceve fra le linee, si gira, rifinisce | il primo pressing sul vertice basso avversario | si smarca per la ripartenza |
| **Esterno, ala** | ampiezza e uno contro uno, o si accentra | segue il terzino avversario, o pressa il centrale | attacca la profondità, o ripiega |
| **Centravanti** | il riferimento: attacca la profondità, viene incontro, fa sponda, occupa i centrali | il primo pressing: sceglie dove spingere il portatore | il primo a pressare, o si fa trovare per la ripartenza |
| **Seconda punta** | fra le linee e la profondità, lega il gioco con il centravanti | pressa il vertice basso | si stacca per ricevere |

### Da dove vengono (maintainer)

- **Le zone e le responsabilità di base** sono di grado 2, misurate sui tracking veri di Metrica (le posizioni medie per ruolo e per fase, chi pressa, chi resta dietro); il campione (due partite) va allargato.
- **Anche i manuali di allenamento rappresentano una tendenza** (maintainer): sono una fonte, come i tracking, di quello che un calciatore di quel ruolo tende a fare; si citano.
- **La difesa preventiva:** quanti restano dietro la palla quando si attacca (di solito tre più due nel calcio moderno) è una scelta dell'idea di gioco dell'allenatore (strato 4); lo strato 2 dice chi ne è responsabile nella forma di base.

## Strato 3 · Le coppie affiatate (approvato dal maintainer)

Molto del gioco nasce da piccole unità, coppie e triangoli che si conoscono. L'intesa c'è già (decisione BB): quanto bene uno prevede l'altro, cresciuta dalle giocate riuscite insieme. Questo strato dice cosa fa una coppia affiatata che una coppia nuova non fa:

- **i movimenti complementari:** se il compagno viene corto, io vado lungo, perché so che lo farà;
- **la divisione dei compiti senza parlarsi:** chi esce, chi copre, chi va sul pallone.

Senza intesa, due compagni vanno sulla stessa palla, o la lasciano tutti e due (visto dal maintainer guardando la partita, 17 novembre).

### Le unità del calcio moderno

| Unità | Cosa fa quando è affiatata |
|---|---|
| **Portiere e centrali** | chi esce sul pallone alto e chi resta; il retropassaggio sicuro; la linea che sale e il portiere che copre lo spazio dietro |
| **I due centrali** | uno esce, l'altro copre (la scalata); chi prende il centravanti; chi imposta e chi resta |
| **Terzino e ala, la catena di fascia** | la sovrapposizione o la sottoposizione; chi dà ampiezza (mai tutti e due larghi); il raddoppio in difesa; uno copre l'altro |
| **Mediano e centrali, il doppio mediano** | uno si abbassa a prendere palla, l'altro resta; le coperture davanti alla difesa |
| **Il triangolo di fascia** (mezzala, ala, terzino) | le rotazioni: se uno si sposta, un altro occupa il suo spazio; il triangolo per uscire dalla pressione |
| **Le due punte, o punta e trequartista** | i movimenti opposti: uno viene incontro, l'altro attacca la profondità; la sponda; l'uno-due |
| **Battitore e saltatore sui piazzati** | il tempo del cross e del movimento |

### Come funziona nel motore (LUNA)

1. **Ogni coppia di compagni ha la sua intesa,** non solo quelle della tabella: la tabella dice dove pesa di più, per i ruoli vicini. **Le coppie sono automatiche** (maintainer): l'allenatore può tenerne conto nella formazione, ma non le dichiara.
2. **Il movimento complementare nasce dalla previsione:** più un giocatore prevede bene il compagno, più lo spazio che sceglie con la misura (strato 2) tiene conto di dove andrà l'altro. Nessun copione: una coppia nuova sbaglia i tempi, una affiatata no.
3. **L'intesa iniziale nasce dai fatti** (il vivaio fatto insieme, le stagioni nello stesso club), poi cresce e cala con le giocate.
4. **I giocatori si parlano** (maintainer): "lascia!", "esco io!", "uomo!". Una chiamata è un atto con il suo tick, come i messaggi dalla panchina (decisione AP); a differenza della panchina, in campo **la sente chi è vicino,** come succede davvero (qui non c'è frustrazione per chi gioca: è la fisica del campo, non un suo comando). Chi la sente la segue quanto si fida, cioè quanto è forte l'intesa; e le chiamate riuscite fanno crescere l'intesa. Sono le chiamate che evitano due giocatori sullo stesso pallone.
5. **La voce del capitano e dei leader pesa di più** (maintainer, decisione AO): il compagno gli dà più retta.

### I test

- **Gemelli:** due coppie di centrali uguali tranne l'intesa, lo stesso lancio lungo, e quella affiatata ha uno che va e uno che copre, quella nuova va in due o resta in due; la stessa catena di fascia con e senza intesa, e quella affiatata non si trova mai larga in due.
- **Le chiamate:** un pallone libero fra due compagni, uno chiama, e l'altro lascia.

## Parte 1 · Le tendenze del calciatore

Ogni tendenza: cosa fa un calciatore vero; da quali caratteristiche dipende quanto bene lo fa; come si prova (un test di scena: una situazione fissa, il comportamento atteso con i suoi margini).

### 1.1 Sempre

1. **Sapere dove si è.** Ogni calciatore sa sempre dov'è la palla, dove sono le due porte, dove sono i compagni e gli avversari vicini che ha guardato. Si guarda intorno prima di ricevere (lo sguardo, già in L0). *Dipende da:* visione, concentrazione. *Test:* un giocatore che riceve dopo essersi guardato intorno sceglie meglio di uno uguale che non l'ha fatto.
2. **Stare nel posto giusto rispetto a palla, uomo e porta.** La posizione non è un punto fisso: è un punto rispetto al pallone, all'avversario e alla porta, che si aggiusta di continuo. *Dipende da:* lettura del gioco, concentrazione; la tattica dà il punto di partenza.
3. **Dosare lo sforzo.** Non si corre per niente: si cammina quando il gioco è lontano, si scatta quando serve. *Dipende da:* lettura del gioco, resistenza, generosità (work rate).

### 1.2 Il pallone libero

4. **Chi ci arriva prima ci va.** Un pallone libero ha sempre qualcuno che lo reclama: il giocatore che pensa di arrivarci per primo (sulla sua percezione) ci va; gli altri vicini si preparano alla seconda palla o coprono. Un pallone fermo ai piedi di un giocatore è suo. *Dipende da:* lettura del gioco, reattività, accelerazione. *Test:* un pallone fermo accanto a un giocatore lo prende lui; fra due giocatori, ci va chi arriva prima, e l'altro non lo insegue alla cieca.
5. **Chi lo manca si gira e lo cerca.** Un controllo sbagliato non finisce lì: il giocatore si accorge che il pallone gli è rimasto dietro o di lato e ci torna, se è ancora il più vicino. *Dipende da:* controllo del corpo, reattività, concentrazione.
6. **Vicino alla propria porta, nessun pallone resta libero.** Davanti alla propria area un pallone libero si prende o si spazza, subito; lasciarlo lì è il pericolo più grande. *Dipende da:* lettura, decisione, gestione del rischio. *Test:* un pallone libero nell'area di rigore non ci resta più di un attimo se c'è un difensore vicino.
7. **I palloni alti: si va dove scende.** Si legge dove il pallone scende; ci va chi ci arriva meglio, e attacca il pallone; gli altri prendono la seconda palla, e la squadra accorcia insieme. Non si esce in anticipo per prenderla se non ci si arriva. *Dipende da:* lettura, colpo di testa, salto, coraggio. *Test:* su un lancio lungo un solo difensore va a contrasto, uno copre dietro, la linea sale o scende insieme.

### 1.3 Con la palla

8. **La palla davanti, protetta.** Chi conduce tiene il pallone davanti al piede, e il corpo fra il pallone e l'avversario; il pallone non resta mai dietro a chi lo porta, salvo un controllo sbagliato. *Dipende da:* dribbling, primo controllo, controllo del corpo. *Test:* in conduzione il pallone sta davanti, nella direzione della corsa.
9. **Alzare la testa e scegliere.** Tenere, passare, avanzare, tirare, secondo lo spazio, la pressione e la posizione sul campo. *Dipende da:* visione, decisione, gestione del rischio.
10. **Sicuri vicino alla propria porta, coraggiosi vicino all'altra.** Il rischio che si accetta cresce con la distanza dalla propria porta. *Dipende da:* gestione del rischio, compostezza.
11. **Avanzare quando si può.** Se c'è spazio davanti, si va avanti: con la palla al piede o con il passaggio verso la porta avversaria. Il passaggio laterale o indietro è per quando avanti non si può. *Dipende da:* visione, decisione, coraggio.

### 1.4 Senza palla, la palla è nostra

12. **Offrirsi.** Chi è vicino al portatore gli dà una linea di passaggio pulita, con un angolo e una distanza giusti, non dietro un avversario e non in linea con un compagno. *Dipende da:* lettura, visione.
13. **Dare ampiezza.** Qualcuno tiene il campo largo, così il campo è grande per chi ha la palla.
14. **Dare profondità: attaccare lo spazio.** Quando dietro la linea avversaria si apre uno spazio, chi può ci va, con il tempo giusto per restare in gioco. *Dipende da:* lettura, accelerazione, velocità.
15. **Muoversi per un compagno.** Ci si sposta anche per liberare uno spazio a un altro: venire incontro per portarsi via il difensore, il terzo uomo. *Dipende da:* lettura, visione, generosità.
16. **Gli attaccanti non sono fermi.** Vengono incontro a ricevere, attaccano la profondità, cambiano lato, e quando la palla si perde sono i primi a pressare. *Dipende da:* lettura, generosità, resistenza.

### 1.5 Senza palla, la palla è loro

17. **Il più vicino va sul portatore.** Lo ritarda, gli chiude la strada verso la porta, lo spinge dove vuole la squadra; non entra a vuoto. *Dipende da:* lettura, contrasto, aggressività, generosità.
18. **Il secondo copre.** Chi è dietro al primo copre lo spazio se il primo viene saltato.
19. **Gli altri si stringono.** La squadra si stringe verso il centro e verso la porta; chi è lontano dalla palla equilibra, non la insegue.
20. **Fra l'uomo e la porta.** Chi marca sta sempre dalla parte della propria porta; segue l'inserimento dell'avversario.
21. **In area non si lasciano uomini liberi.** In area si marca: ognuno sa chi ha vicino.
22. **Vicino alla propria area si riconquista.** Un pallone avversario vicino alla propria area non si guarda: il più vicino lo attacca, gli altri chiudono le linee.

### 1.6 Le transizioni

23. **Appena persa:** chi è vicino al pallone lo aggredisce subito; gli altri rientrano verso la propria porta e si ricompongono. *Dipende da:* reattività, generosità.
24. **Appena vinta:** il primo pensiero è in avanti; se lo spazio c'è, si verticalizza subito, se non c'è si mette la palla al sicuro.

### 1.7 Le palle ferme e le ripartenze

25. **Si va al pallone.** Chi batte una rimessa, un angolo o una punizione ci va correndo o camminando, e la ripresa aspetta che arrivi; nessuno appare sul pallone.
26. **Ci si piazza.** Gli altri prendono posto per la ripresa: le marcature in difesa, i movimenti in attacco (dalla tattica).
27. **Dopo il gol.** Chi segna esulta, poi tutti tornano a centrocampo con calma; il tempo passa.

## Parte 2 · Le tendenze tattiche (dalla lavagna)

Sono della squadra, le decide l'allenatore, e si cambiano prima e durante la partita (decisioni AL, AG). I giocatori le eseguono con le tendenze della parte 1, filtrate dalla disciplina.

- **La forma per fase e per terzo di campo:** la costruzione dal basso (il portiere con la palla: dove si allargano i centrali, quanto salgono i terzini, chi viene a prendere palla), lo sviluppo, la rifinitura; senza palla la pressione alta, il blocco medio, la difesa bassa.
- **Le responsabilità di ognuno:** chi esce in pressione, chi copre, chi resta nella difesa preventiva, chi attacca la profondità, chi viene incontro.
- **Il pressing:** l'altezza, gli inneschi (il retropassaggio, il controllo sbagliato, la palla al terzino), dove si spinge il portatore.
- **Il blocco:** altezza della linea, ampiezza, distanza fra i reparti.
- **Le marcature:** a zona, a uomo, miste; sui calci piazzati.
- **Le transizioni:** riaggredire o abbassarsi; ripartire o tenere palla.
- **Il ritmo e il rischio:** gioco diretto o paziente.
- **I calci piazzati:** gli schemi disegnati.

## Parte 3 · Come si prova

1. **Prima le scene.** Ogni tendenza ha il suo test di scena: una situazione fissa (il pallone libero accanto al difensore, il portiere con la palla, il lancio lungo), il comportamento atteso con i suoi margini, deterministico. Le scene si guardano anche nella GUI: chi ha occhio da calcio vede se è giusto.
2. **Poi i gemelli.** Due giocatori uguali tranne una caratteristica (la lettura, la disciplina), e la tendenza si vede di più nel migliore.
3. **Poi le misure sul vero, di grado 2,** sui tracking Metrica e sugli eventi Wyscout: quanto tempo il possesso sta nella metà avversaria, dove stanno i giocatori quando il portiere ha la palla, quanto lontano sono gli attaccanti dalla palla, quanto dura un pallone libero, la direzione dei passaggi.
4. **Solo dopo, la taratura** delle statistiche (decisione L).

## Parte 4 · Cosa resta agli errori puri

Alcune cose viste guardando la partita non sono tendenze, sono errori da correggere subito: il teletrasporto alle ripartenze (la tendenza 25 dice cosa deve succedere), il pallone che segue con un passo di ritardo chi lo porta (la tendenza 8), l'arbitro della vista che sta sopra il pallone, le istruzioni che non si vedono, gli avvisi di Cargo.

## Le scelte da fare con il maintainer

- [ ] Le tendenze della parte 1: manca qualcosa, qualcosa è da togliere?
- [ ] La generosità (work rate) e la lettura del gioco reggono da sole le tendenze senza palla, o serve una caratteristica in più (l'esperienza come radice a sé, invece che come crescita della famiglia cognitiva)?
- [ ] L'ordine: le tendenze del pallone libero, della palla al piede e delle ripartenze (1.2, 1.3, 1.7) per prime, perché sono quelle che si vedono di più?
