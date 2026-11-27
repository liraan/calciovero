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
4. **La tattica:** la lavagna dell'allenatore, già decisa (decisioni AL, AG, AO, AP in `L1-COACH.md`); qui sotto la parte 2 ne dà il riassunto per i comportamenti.

Le tendenze del calciatore della parte 1 si ridistribuiscono negli strati 1 e 2 man mano che si scrivono.

### Il calcio moderno fa parte del gioco (maintainer, 20 novembre 2026)

Il maintainer: «dobbiamo inserire in behaviours i concetti del calcio moderno,
semplicemente perché codifica quello che succede in campo». Non è un modulo
nuovo da insegnare al motore: è la stessa grammatica letta con più risoluzione.
Ogni voce sotto è una tendenza osservabile — percezione più caratteristiche —
non un copione; la lavagna dell'allenatore (strato 4) può assegnarla, non
impiantarla. Nessun numero nuovo, nessun dado.

| Concetto moderno | Quello che si vede in campo | Dove vive già nel catalogo |
|---|---|---|
| **Occupazione dello spazio, ruolo come funzione** | il terzino entra dentro, il centravanti viene incontro: restano il terzino e il centravanti | strato 2, «Il linguaggio dello spazio» e «I ruoli di base» |
| **Costruzione dal basso** | portiere e centrali si offrono, c'è sempre una linea di passaggio, si rompe la prima linea avversaria | strato 2 (portiere, difensore centrale); strato 3 (portiere e centrali) |
| **Terzino invertito, centrale che si allarga, falso nove** | chi occupa il corridoio cambia, la funzione resta; la punta scende e trattiene il difensore | varianti della lavagna (decisione AL) nei «Ruoli di base» |
| **Pressing alto e contropressing** | subito dopo la perdita chi riaggredisce e chi copre la profondità | strato 1 (fasi), strato 2 (colonna «Nelle transizioni») |
| **Difesa preventiva, linea alta e fuorigioco** | quanti restano dietro la palla (di solito tre più due), la linea sale e il portiere copre dietro | «Da dove vengono», strato 3 (portiere e centrali); la scelta è strato 4 |
| **Rotazioni e triangoli** | se uno si sposta, un altro occupa il suo spazio; si esce dalla pressione insieme | strato 3, «Le unità del calcio moderno» |
| **Chiamate e intesa in campo** | «lascia!», «esco io!», «uomo!»: chi le sente le segue quanto si fida | strato 3, «Come funziona nel motore» |
| **Transizioni rapide** | appena vinta o persa, la prima scelta è in avanti, non ricomporsi | strato 2, colonna «Nelle transizioni» |

Questa tabella codifica **cosa succede**, non dichiara che il motore lo esegua già:
dove la catena cognitiva non c'è ancora, la tabella dei test di questa stessa
pagina segna la voce come da verificare.

### L'istinto: presenza comune, qualità personale (maintainer, 20 novembre 2026)

Il maintainer: «dobbiamo rendere determinati concetti *istintivi* per i giocatori
in campo, non tutti dipendendo dalle loro caratteristiche ovviamente». Il
concetto non è un modulo né un copione: è un termine della salienza che c'è in
ogni giocatore anche quando la caratteristica è minima. La caratteristica non
decide *se* l'atto esiste, ma *quanto bene e quanto presto* viene eseguito.

| Asse | Che cosa decide | Dove sta |
|---|---|---|
| **Presenza — l'istinto** | ogni giocatore sa che il pallone libero si prende, che davanti alla propria porta si spazza o si marca, che il più vicino pressa e il secondo copre, che in area si sta fra uomo e porta, che dopo la perdita si riaggredisce o si rientra | grammatica comune del gioco (ADR-052 + Leggi): termine costante della salienza |
| **Qualità — le caratteristiche** | quanto bene e quanto presto lo fa: lettura, reattività, tecnica, corpo, coraggio | i latents già presenti in `ds-db` |
| **Assegnazione — ruolo e tattica** | *chi* lo fa in quella fase: difesa preventiva, ampiezza, profondità, terzino invertito, falso nove, portiere libero | strati 2 e 4 del catalogo |

**La regola in codice.** Dove un istinto incontra una caratteristica, il
termine ha la forma `base + guadagno × caratteristica`, non il solo prodotto:
anche il peggiore agisce, il migliore agisce meglio. La regola è nominata
`instinct(base, guadagno, caratteristica)` in
`ds-orchestrator/src/offball.rs:389`, con il gemello che la prova; la sfida è
`challenge_salience × (0.5 + 0.5 × tackling)` (riga 311), la marcatura ha lo
stesso base sulla lettura (riga 320), e l'offerta e l'inserimento sono
space value × corsa, senza dado. La presenza, poi, è garantita *prima* della
caratteristica: il più vicino va al pallone libero a qualsiasi distanza
(`goes_for_loose_ball(0, reading, _)` è `true` anche con `reading = 0.0`), e il
primo difensore pressa e marca con peso 1 (`rank(0) = 1.0`). I gemelli che lo
provano sono `the_nearest_goes_for_a_loose_ball_at_any_distance` e
`in_reach_he_challenges`.

**Ciò che resta una lettura, non un istinto.** I termini ancora prodotto puro
sono quelli del *secondo* uomo (`rank(1) = reading × secondo lettore`, righe
164-168 e 245-249): decidere se raddoppiare, o se andare in due sullo stesso
pallone, è una scelta di chi legge meglio la scena, non un riflesso. Renderla
istintiva farebbe correre due giocatori su ogni pallone, lascerebbe scoperti
gli spazi e contraddirebbe il gemello ADR-051. Anche l'intesa (strato 3) è un
fattore di assegnazione: chi conosce il compagno cede, chi non lo conosce
può andare. La distinzione va tenuta: istinto = il primo agisce sempre,
lettura = il secondo sceglie se aggiungersi.

### Una regola, una funzione (maintainer, 20 novembre 2026)

Il maintainer: «ogni crate abbia una funzione», «in modo che non siano
ridondanti e non creino regole contraddittorie che mutano il comportamento
dei giocatori in campo». Il criterio è LUNA: se la stessa regola è scritta due
volte, lo stesso giocatore legge la scena con un numero ed esegue con un
altro. Ogni formula vive quindi nel crate che la possiede, e gli altri la
chiamano:

| Regola | Funzione unica | Chi la chiama |
|---|---|---|
| portata dello scivolo | `DuelBalance::slide_reach_cm` (`ds-calculator`) | scena di ricezione (`ds-orchestrator`), duelli e ricezione (`ds-match`) |
| portata in piedi del portiere | `SaveBalance::standing_reach_cm` (`ds-calculator`) | tiro immaginato, parata, barriera (`ds-calculator`, `ds-match`) |
| interpolazione della velocità massima | `GroundTruth::top_speed_base` (`ds-orchestrator`) | movimento, lettura dalla panchina, gare senza palla |

### La casa delle misure (maintainer, 21 novembre 2026)

Le misure condivise con cui un giocatore valuta le opzioni che percepisce
hanno una casa sola: `ds-orchestrator/src/behaviour.rs` (`behaviour`). Una
responsabilità — dire quanto vale un'opzione percepita — in un crate solo,
quello che possiede già la decisione (ADR-041 §1, ADR-043). Chi sceglie
l'atto resta l'attore; chi applica l'effetto resta il motore. Le funzioni
`soon`, `space_value`, `place_weight`, `instinct`, `obey` e
`goes_for_loose_ball` vivono lì una volta sola e gli altri moduli le
chiamano: due copie della stessa misura farebbero leggere la scena con un
numero e agire con un altro.

Dentro `behaviour.rs` ci sono due ingressi della stessa misura, non due
responsabilità (maintainer, 21 novembre 2026):

- **Nozioni base (istinto, sempre presenti in L0):** `instinct`,
  `goes_for_loose_ball`, `soon`, `space_value`. Ogni professionista le sa
  anche con la caratteristica minima: la caratteristica cambia quanto bene
  e quanto presto agisce, non se l'atto esiste (§55-81).
- **Nozioni attivabili (tattica, L1):** `place_weight` e `obey`. Non
  insegnano nulla al giocatore: cambiano quanto conta, in questa partita,
  una cosa che già sa. Arrivano a runtime come intenzione del coach
  (`ds_model::Intention`: forma, altezza del blocco, ampiezza) e come atto
  chiesto (`OffBallScene::asked`); la disciplina dice quanto il giocatore
  segue il piano invece della propria lettura. Mai un peso sull'opzione né
  una caratteristica (ADR-059:41, :58).

Entrambe le nozioni **sottintendono sempre alla lettura di campo del
giocatore**: pesano solo opzioni che lui percepisce (ADR-043, ADR-062); ciò
che non vede non esiste per la sua misura. L'intenzione del coach non
sostituisce la lettura — la disciplina dice quanto segue il piano *invece*
della propria lettura, non quanto la lettura valga meno (ADR-059:41-42).

Questo non è un modulo nuovo che insegna il calcio al motore (§31-38): è la
stessa grammatica con una casa dichiarata. Il catalogo resta per noi; i
giocatori leggono le funzioni, non il documento. Spostare una funzione non
cambia un tick: nessun numero nuovo, nessun dado, nessun SIM bump.

Nello stesso spirito, a L0 non c'è nessun componente casuale, nemmeno
inutilizzato (maintainer, 20 novembre 2026): il modulo RNG e le dipendenze
`rand` sono rimossi. Il giocatore sceglie con la sua lettura della scena e le
sue caratteristiche; un dato che il modello non sa spiegare non si copre con
un numero estratto (ADR-035).

La fatica resta un moltiplicatore esplicito di chi ha già corso: la gara
senza palla usa il passo che il giocatore *crede* di avere, il movimento e la
panchina quello che ha, ma l'interpolazione di partenza è una sola. I gemelli
`reach_has_one_value` e `the_pace_that_moves_and_the_pace_that_is_read_are_one_number`
bloccano i valori vecchi: l'unificazione è aritmeticamente identica (golden
intatti, nessun `SIM_VERSION` nuovo). Non sono unificate le quantità diverse
che hanno lo stesso nome, come i due `mark_radius_cm` (300 cm per offrirsi,
800 cm per lo sguardo): sono due situazioni, non una regola scritta male.

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

### La catena cognitiva, la prova e lo stato nel motore (2026-11-19)

Il catalogo dice *cosa* fa il calciatore; questa tabella dice *come ci arriva* (percepisce → legge → sceglie), *come si verifica*, e *dove siamo davvero*. È il filtro LUNA applicato al background: una tendenza senza catena è un comportamento a occhio, una senza prova non è verificabile, una senza stato onesto fa credere che il motore faccia cose che non fa. Le voci "non trovato" non sono affermazioni: sono il lavoro che resta.

Verificata riga per riga contro il codice il 2026-11-20: ogni voce ha ora un riferimento `file:line` (o la dichiarazione esplicita che il codice non c'è). I numeri citati vengono dal registro del seme 42 (81.567 decisioni) e dalle 60 partite SIM 163.

| # | Tendenza | Catena cognitiva | Si prova con | Stato nel motore |
|---|---|---|---|---|
| 1 | Sapere dove si è | percepisce gli sguardi → legge dove sono palla, porte, vicini visti → si orienta prima di ricevere | test dichiarato (sguardo vs nessuno), non scritto | `perception.rs:106,218`, `state.rs:1266,1279`, `scanning.json`; percezione a ultimo sguardo, non onnisciente |
| 2 | Stare nel posto giusto | percepisce palla, uomo, porta, zona del ruolo → legge quale spazio vale → si aggiusta di continuo | `he_closes_the_space_an_opponent_can_be_played_into`, `the_reader_leaves_the_given_place_the_blind_man_keeps_it` (SIM 165, gemello del burattino) | attuato in SIM 165: `choose_closed_place` (oggi `ds-orchestrator/src/offball.rs::ClosedPlaceScene`, ADR-071 §5 nota 2026-11-21) pesa con la lettura del giocatore la linea nel suo corridoio quando nessuno può arrivarci; i corridoi accanto restano pesati dal gioco, la zona resta la disciplina (ADR-071 §5 emendata) |
| 3 | Dosare lo sforzo | percepisce distanza del gioco e pressione → legge quanto serve correre → cammina o scatta | nessun gemello dedicato | `fatigue.rs:99,115`, `offball.rs:1088 work_share`, `spatial.rs:1820-1906`; camminare è effetto della distanza dal bersaglio, non un atto esplicito |
| 4 | Chi arriva prima ci va | percepisce palla libera, distanze, compagni, avversario → legge chi arriva primo → ci va o copre | `the_nearest_goes_for_a_loose_ball_at_any_distance`, `the_better_reader_goes_second_for_a_loose_ball`, `a_teammate_he_knows_yields_the_loose_ball` | attuato (`offball.rs:406`), ma il rango 0 va anche a 60 m e il rango 1 entra 36/1921 volte |
| 5 | Chi lo manca si gira | percepisce il pallone dietro o di lato → legge se è ancora il più vicino → ci torna | `golden_scenes.rs:19` (OI-123) | emergente: `control_spill` (`lib.rs:874`) → palla libera → riconquista; nessun atto "girati" dedicato |
| 6 | Vicino alla propria porta | percepisce palla libera in area → legge il pericolo → prende o spazza subito | nessun gemello dedicato | parziale: `clear_candidate` (`read.rs:1176`, zona 2500 cm), `technique.rs:97` portiere in area; nessuna regola "la palla libera non resta in area" |
| 7 | I palloni alti | percepisce palla in aria, punto di discesa, chi può arrivarci → legge chi la prende e chi copre → va se ci arriva | `the_second_reaches_the_near_ball_and_holds_for_the_far_one` (SIM 164), `tick_record.rs:459` | attuato in SIM 164 (ADR-081 C): ogni palla alta, entrambi i lati; il secondo ha il rank vero e la race decide. Prima, solo lato difeso + pass in volo |
| 8 | Palla davanti protetta | percepisce l'avversario che arriva → legge dove mettere il corpo → tiene il pallone davanti | `in_a_dribble_the_ball_is_ahead` (`golden_scenes.rs:265`) | attuato: `carry_the_ball`→`follow_carrier` (`technique.rs:1031`), scudo in `lib.rs:890` |
| 9 | Alzare la testa e scegliere | percepisce spazio, pressione, posizione → legge il valore di tenere, passare, avanzare, tirare → sceglie | `a_marked_mate_is_still_a_pass_option` (`read.rs`, DD-81); `actor.rs:61`, `choice.rs:139` | attivo (ADR-052); dal SIM 219 tutte le opzioni di passaggio restano disponibili e lo spazio le gradua (`freedom_share`, ADR-040), non le sopprime |
| 10 | Sicuri vicino alla porta | percepisce la distanza dalla propria porta → legge il rischio accettabile → gioca semplice o rischioso | `shot_choice.json:14 clear_zone_cm=2500` | parziale: `risk_appetite` è un latente fisso (`actor.rs:79-81`), non cresce con la distanza; solo la spazzata è distanza-dipendente |
| 11 | Avanzare quando si può | percepisce spazio davanti → legge che avanti conviene → passa o conduce avanti | `a_pass_forward_brings_more_than_a_pass_back` (`read.rs:2356`); golden 23/53/24 contro 31/53/16 veri | attuato, ma i passaggi in avanti restano sotto il vero (OI-100) |
| 12 | Offrirsi | percepisce portatore, marcatore, linee → legge dove la linea è pulita → si sposta | `a_marked_player_near_the_ball_offers` (`offball.rs:588`) | attuato (`with_mate.values.offer`, `offball.rs:336`) |
| 13 | Dare ampiezza | percepisce chi è largo e i corridoi → legge che serve campo largo → resta o torna largo | `the_wide_man_holds_the_touchline_on_the_ball_side` (`shape.rs:208`), `instructions.rs:89` | attuato: `shape.rs:87,107-112`; i test sono di forma/coaching, non il gemello comportamentale |
| 14 | Dare profondità | percepisce spazio dietro l'ultima linea e il tempo → legge se ci arriva prima del difensore → attacca | `with_room_behind_the_line_some_run` (`offball.rs:605`) | attuato con `race` sul proprio passo |
| 15 | Muoversi per un compagno | percepisce compagno marcato e difensore → legge che lo libera portandogli via l'uomo → viene incontro | nessun gemello | **non trovato**: `OffBallAct` non ha un atto "terzo uomo"; `mate_ball` non ha il termine "porta via il difensore" |
| 16 | Gli attaccanti non sono fermi | percepisce fase e spazio → legge viene incontro, profondità, cambio lato → si muove | `a_midfielder_asked_to_run_waits_on_the_last_line` (`instructions.rs`) — solo se chiesto; dal SIM 225 la punta aspetta sulla linea anche senza ordine (ADR-118) | **non trovato** come comportamento autonomo: gli attaccanti usano `choose_place` generico; 72% di decisioni "stare fermo" |
| 17 | Il più vicino va sul portatore | percepisce portatore, distanza, compagni → legge chi lo ritarda → pressa senza entrare a vuoto | `nobody_chases_a_dead_ball`, `the_disciplined_player_keeps_his_place` | attuato (`Press`, `offball.rs:187`), ma solo 868 pressioni su 81.567 decisioni (1,1%) |
| 18 | Il secondo copre | percepisce il compagno che pressa e lo spazio dietro → legge se il primo è saltato → copre | `behind_the_presser_he_covers` (`offball.rs:703`) | attuato (`Cover`, `offball.rs:203`), ma scelto nello 0,5% delle decisioni |
| 19 | Gli altri si stringono | percepisce palla lontana, centro, porta → legge lo spazio pericoloso → si stringe | `state.rs:1962,1986` | parziale: `danger * soon` (`state.rs:861-868`) fa arretrare il difensore quando un uomo può essere giocato nella sua zona; si muove solo il più vicino al pericolo, gli altri restano |
| 20 | Fra l'uomo e la porta | percepisce uomo e porta → legge da che parte coprire → si mette goal-side | `the_nearest_marks_the_man` (`offball.rs:626`) | attuato (`Mark`, `offball.rs:316`; `man_to_mark` `state.rs:623`) |
| 21 | In area non si lasciano uomini | percepisce gli uomini vicini in area → legge chi è libero → marca | nessun gemello | **non trovato**: `man_to_mark` generico vale dappertutto; in area solo portiere (`technique.rs:97`) e contrasto (`duels.rs:208-219`) |
| 22 | Vicino alla propria area si riconquista | percepisce palla avversaria in zona → legge il pericolo → attacca e chiude le linee | nessun gemello | **non trovato** come regola dedicata: `defend()` (`offball.rs:156-220`) pesa lo spazio verso la propria porta, basta |
| 23 | Appena persa | percepisce la palla persa → legge chi è vicino → riaggredisce o rientra | nessun gemello | parziale: finestra `spatial.rs:753-763`, `OnLoss::CounterPress→Press` (vicino) / `DropBack→Hold` |
| 24 | Appena vinta | percepisce palla vinta e spazio → legge se verticalizzare → primo passaggio avanti | nessun gemello | parziale: `OnWin::Counter→Run`, `Keep→Offer` (`spatial.rs:959-961`), blocco spinto avanti (`shape.rs:116-121`); il primo passaggio avanti non è garantito |
| 25 | Si va al pallone | percepisce palla ferma e chi batte → legge che la ripresa aspetta → ci va | `no_restart_stops_the_match` (`golden_scenes.rs:241`) | attuato: `call_restart`/`send` (`set_pieces.rs:77,97`), ADR-080 SIM 163 |
| 26 | Ci si piazza | percepisce palla ferma e ruolo → legge marcature e movimenti → prende posto | `nobody_moves_faster_than_his_legs` (`golden_scenes.rs:68`) | attuato: `place_for_restart`, `clear_the_spot`, `form_wall` (`set_pieces.rs:62,270,451`); nessun test dedicato ai movimenti d'attacco dalla lavagna |
| 27 | Dopo il gol | percepisce il gol → legge il tempo per tornare → esulta e rientra | `after_a_goal_the_kick_off_waits_and_the_sides_are_in_their_halves` (`golden_scenes.rs:122`) | parziale: rientro + attesa (`set_pieces.rs:74-130`), narrazione (`ds-narrative:287-297`); nessun atto di esultanza |

**Voce 9, letta con LUNA e partita vera (DD-81, 26 novembre 2026).** Il
portatore vede il compagno marcato: è lui che sceglie COME eseguire — il
passaggio filtrato tra due uomini, l'appoggio a chi viene incontro, il
retropassaggio sicuro — non la lavagna a togliergli l'opzione. Nella partita
vera il passaggio a un uomo marcato è una scelta, spesso la più intelligente;
lo spazio lo rende più o meno conveniente, non impossibile. Il gemello
`a_marked_mate_is_still_a_pass_option` mette lo stesso passaggio con il
marcatore a 50 cm e a 25 m: l'opzione c'è in entrambi i casi e vale di meno
nel primo. Nessun numero nuovo: la misura dello spazio è già `freedom_share`
(ADR-040).

**Voce 11, letta con LUNA e partita vera (DD-91(d), 27 novembre 2026).** Se davanti non c'è spazio, lo stesso portatore alza la testa e gira palla; se neanche il giro è pulito, consolida cercando spazio avanti o indietro. Il *dito* è lo spazio occupato, l'evento `Pass` e la carica; la *luna* è l'uomo che rilegge la scena e sceglie il come: la catena è una tendenza di lettura, non un ordine di gesti, e nessuna opzione va tolta (preferences-010). Gemello `when_space_ahead_closes_the_carrier_circulates` (`read.rs`): con spazio sceglie la carica, con un uomo a 2 m davanti sceglie il passaggio, e passaggio e carica restano candidati in entrambe le scene. Il grezzo Wyscout non ha posizioni dei difensori, quindi non prova la causalità; non spiega il volume dei passaggi (RUN 89).

**Voce 12, letta con LUNA e partita vera (DD-92, 27 novembre 2026).** Il portiere non decide se il gol c'è: vede la palla arrivare, si getta e usa la stessa portata che il tiratore immagina e la barriera usa (`standing_reach_cm`, `reach_cm`). Il *dito* è l'attraversamento e l'evento `Save`/`Goal`; la *luna* è l'uomo che esegue, non un filtro sul referto. Gemello `moving_only_the_keeper_across_gives_one_save_boundary` (`keeper.rs`): cambiando solo la sua posizione laterale la parata attraversa un confine geometrico una volta sola; `more_time_more_reach` verifica che la stessa portata muove insieme la scelta immaginata e l'esecuzione. Il grezzo Wyscout non dà posizione del portiere né placement dei tiri, quindi la conversione resta una domanda di definizione, non una taratura (A5-bis).

**Voce 13, letta con LUNA e partita vera (DD-93, 27 novembre 2026).** Sulla palla alta ogni uomo legge quota, discesa e compagni e sceglie COME saltare: il *dito* è il volo, i passi dei contendenti e l'evento `Duel{aerial:true}`, la *luna* è lo stesso giocatore che arriva o copre. Il gemello `the_second_reaches_the_near_ball_and_holds_for_the_far_one` (SIM 164, ADR-081 C) verifica che il secondo uomo prenda la palla vicina e copra quella lontana, senza togliere opzioni. Wyscout conta righe per giocatore dimezzate (0,5 per riga) e la frazione «contesa» cambia con la finestra (19% allo stesso secondo, 85% entro 1 s); il dump non ha quota, traiettoria né salto. Non è l'unità del duello che il motore emette nello stesso tick: nessuna finestra presa dalla tabella e nessun modello nuovo (A5-bis).

**Voce 14, letta con LUNA e partita vera (DD-94, 27 novembre 2026).** Il pallone varca la linea laterale, arriva l'`Interruption` e si va a battere: il *dito* è l'uscita e il `ThrowIn`, la *luna* è il giocatore che ha già l'opzione di calciare in fascia (`pressed_with_nothing_on_he_puts_it_into_touch`) e sceglie COME contrastare, non un gesto «manda fuori» che il referto non registra. Il gemello `no_restart_stops_the_match` verifica che la ripresa aspetti chi deve battere. Emissione e geometria restano quelle di Law 9 (palla interamente fuori): il 99,6% delle 16.235 rimesse grezze segue l'interruzione e il 31,2% di prossimità temporale non è attribuzione. Nessun lookahead e nessuna soglia nuove (A5-bis).

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
