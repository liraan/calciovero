# L1 · L'allenatore (`ds-coach`)

Il piano del plugin allenatore di L1: cosa sceglie ogni allenatore prima della partita, da cosa, e come lo fa senza dadi. In partita reagisce la panchina di L0 (decisione T), che legge il piano scritto qui. Decisioni da AF ad AM (maintainer, 16 novembre 2026), con i filtri partita vera, LUNA e futuribilità.

Il quadro è in [`L1.md`](L1.md); le istruzioni in partita sono dell'[ADR-059](../../doctrine/adr/059-the-coach-speaks-during-the-match.md).

## Il principio

Il calcio di riferimento è quello moderno: ogni club ha analisti e dati su ogni avversario, e prepara ogni partita con un piano. L'allenatore del gioco fa lo stesso. Sceglie sulle stime, mai sul piano A; le sue scelte sono geometria e intenzioni che il motore di L0 sa già leggere; nessuna scelta cambia una caratteristica di un giocatore (ADR-059 §3).

## Cosa sceglie prima della partita

1. **Le persone:** l'undici, la panchina, il capitano, i rigoristi, chi batte punizioni e angoli.
2. **Il piano:** il modulo e la disposizione di ogni fase (con la palla, nelle transizioni, senza la palla), l'altezza del blocco, l'ampiezza, le due scelte di transizione, le istruzioni rapide accese, le istruzioni individuali, i cambi già decisi.
3. **Il piano gara sull'avversario** (sotto, "Il piano gara").

Quanti in panchina e quanti cambi lo dicono le regole, che sono dati modulari con la loro stagione di validità.

## Un attore come i giocatori (decisione AH)

L'allenatore avversario non è un'intelligenza artificiale a parte: è un **attore**, costruito come un giocatore, e lo stesso varrà per lo staff e la dirigenza in L2. Ognuno sceglie con le sue caratteristiche e le sue abilità. La catena è quella di `VISION.md` §3 e di `L0-match/design/CHARACTERISTICS-AND-ABILITIES.md`:

- **le caratteristiche** sono le radici: nascoste (piano A, nel caveau di `ds-db`, dietro la sua porta), stabili per mesi;
- **le abilità** sono formule, non dati: caratteristiche combinate in un tipo di situazione, mai salvate;
- **gli stati del momento** le attenuano, e ognuno si vede da un segno nel piano B;
- **la scelta** è dell'attore (ADR-043): riceve la scena percepita, fatti osservabili e stime, mai un elenco di opzioni già valutate, e decide con le sue abilità.

Il tipo `Actor` di `ds-orchestrator`, oggi solo il giocatore, diventa l'attore di ogni tipo (giocatore, allenatore, poi staff e dirigente), con la stessa porta verso il caveau. Serve un ADR prima del passo 5 di L1.

## Il giocatore umano sceglie il suo ruolo (decisione AI)

Chi gioca a Calcio Vero non è per forza l'allenatore: all'inizio della carriera sceglie il suo posto nel club, l'allenatore, il vice, il direttore sportivo, il direttore tecnico, il presidente, l'amministratore delegato. Per questo **ogni posto è occupato da un attore**, con le sue caratteristiche e abilità; il giocatore umano prende un posto, e le scelte di quel posto diventano sue, come input datati. Tutti gli altri posti restano agli attori.

- **I ruoli sono dati.** Ogni ruolo è dichiarato in una scheda: quali scelte gli spettano, cosa vede e con che precisione (un presidente non conosce la condizione dei giocatori come l'allenatore che li allena), a chi risponde. Ogni scelta di ogni plugin dichiara il ruolo a cui spetta. Un modder può aggiungere un ruolo (il preparatore dei portieri, il team manager) senza toccare il nucleo.
- **Le scelte degli altri restano degli altri.** Se sei il presidente, la formazione la fa l'allenatore attore, e la partita la guardi dalla tribuna; se sei l'allenatore, i giocatori li compra il direttore sportivo attore, e tu glieli chiedi. Si chiede, si propone, si discute attraverso i canali del ruolo; chi decide decide con le sue caratteristiche.
- **In L1 c'è il meccanismo, con i posti che L1 ha.** Le scelte di L1 (la formazione, il piano, l'allenamento) sono già dichiarate con il loro ruolo dal primo giorno, così L2 aggiunge ruoli senza riscrivere niente. Il posto giocabile in L1 è l'allenatore; gli altri posti arrivano con i plugin che hanno le loro scelte (mercato, staff, società, finanze: L2).
- Le risposte del maintainer sul ruolo sono più sotto (decisione AJ).

## L'albero dell'allenatore

Una proposta da correggere con il maintainer ramo per ramo, come quella dei giocatori. Le caratteristiche nascono dal livello del club e da come hanno giocato le squadre vere nella stagione di riferimento (i dati di squadra e delle partite del riferimento Wyscout), con nomi inventati; si misurano sull'effetto (grado 2) e si citano in `balance/SOURCES.md`, tranne dove è detto.

**Le radici, in cinque famiglie:**

| Famiglia | Caratteristiche |
|---|---|
| **Idea di gioco** (i gusti, non la bravura) | il modulo preferito, l'altezza del blocco, l'ampiezza, il gioco diretto o di possesso, le scelte di transizione |
| **Mente tattica** | la lettura della partita, la conoscenza tattica, l'adattabilità (quanto si sposta dalla sua idea) |
| **Metodo** | l'analisi dell'avversario (grado 3 in L1, dal livello del club, finché lo staff di L2 non la rende misurabile), l'attenzione alla condizione (quanto pesa la freschezza contro la gerarchia) |
| **Carattere** | la propensione al rischio (un giovane, un attaccante in più, la linea alta), la tenuta sotto pressione, l'ostinazione |
| **Comunicazione** | il carisma, la chiarezza, l'empatia (decisione AM) |

**I rami, le abilità (formule):**

| Abilità | Da quali radici | Dove agisce |
|---|---|---|
| Preparare il piano gara | analisi, conoscenza tattica, adattabilità | prima della partita |
| Scegliere l'undici | attenzione alla condizione, propensione al rischio, conoscenza tattica | prima della partita |
| Leggere la partita | lettura, tenuta sotto pressione | in partita, nella panchina di L0: quando reagisce (decisione Y) |
| Correggere in corsa | conoscenza tattica, adattabilità, propensione al rischio | in partita: cosa cambia |
| Parlare alla squadra | carisma, chiarezza, tenuta sotto pressione | prima, durante, dopo la partita, in settimana |
| Parlare al singolo | empatia, chiarezza, carisma | i colloqui, i richiami |
| Spiegare la tattica | chiarezza, conoscenza tattica | la riunione tattica |

**Gli stati del momento:** la pressione (la classifica, il risultato, i risultati recenti), la fiducia. In L1 si vedono nel piano B dalle scelte (più prudenti, più nervose); in L2 si aggiunge la pressione della società.

Le caratteristiche degli allenatori avversari si vedono come stime, ricavate da come hanno fatto giocare le loro squadre: lo stile di un allenatore si riconosce dalle sue partite.

## Cosa sa: le stime

- **I suoi giocatori:** stime strette, perché li allena ogni giorno.
- **Il gioco dell'avversario:** stima stretta. Modulo, blocco, ampiezza, transizioni, chi tocca più palloni, da dove arrivano i suoi pericoli: si ricava dagli eventi delle partite che l'avversario ha già giocato, che il gioco conserva per tutte (piano B). Quanto è stretta la decide l'analisi dell'avversario.
- **Le caratteristiche dei giocatori avversari:** stime larghe, da risultati e statistiche. Gli osservatori di L2 servono per il mercato, non per preparare la partita.

Nessuna di queste stime legge il piano A (ADR-003).

## Il piano gara

Ogni allenatore prepara la partita sull'avversario; l'adattabilità decide quanto si allontana dalla sua idea.

- **Le contromisure sono quelle del calcio moderno:** il raddoppio sulla stella, l'attenzione preventiva di chi le sta vicino, la pressione su chi le passa il pallone, il blocco che toglie lo spazio da cui l'avversario è pericoloso. La marcatura a uomo è una scelta rara, di un allenatore che la ha nella sua idea.
- **Solo istruzioni che il motore sa eseguire** (ADR-059 §6). Se ne serve una nuova, nasce in L0 con il suo ADR e il suo `SIM_VERSION`, poi il piano la usa.
- **Il costo è causato:** un raddoppio lascia libero qualcun altro, una linea alta lascia spazio dietro (ADR-059 §5). Nessun bonus né malus assegnato.

## La lavagna libera (decisione AL)

Le scelte dell'allenatore sono il più libere possibile. L'allenatore **disegna**, non sceglie da un menu: un 4-3-3 è un disegno salvato con un nome, come "la mia costruzione a tre". Ogni elemento è geometria o una scelta che il giocatore legge, e ognuno lo segue con la sua disciplina e la sua lettura (ADR-059 §3-4); i costi sono causati, mai assegnati (ADR-059 §5).

**Oggi il motore conosce un solo modulo:** la griglia del 4-4-2 in `shape.rs`, a cui ogni altro modulo ricade. In L0, nella lista della decisione T, la griglia fissa lascia il posto ai disegni liberi: ogni giocatore ha il suo punto nel blocco (profondità e ampiezza, la rappresentazione che il motore usa già), e il motore lo legge per fase.

**Cosa si può dire alla squadra:**

1. **Le posizioni libere per fase.** Le fasi sono tre: con la palla, senza la palla, in transizione. Si disegnano solo le prime due, e sono obbligatorie; facoltativi, un disegno per terzo di campo (costruzione, sviluppo, rifinitura; senza palla: pressione alta, blocco medio, difesa bassa), e il motore passa dall'uno all'altro con continuità secondo dove sta il pallone. Forme asimmetriche ammesse (il 3-2-5 in possesso e il 4-4-2 senza, il terzino che entra dentro).
2. **Le responsabilità di ogni giocatore,** dentro ogni disegno: ognuno ha i suoi compiti in quella fase (chi esce in pressione e chi copre, chi resta nella difesa preventiva, chi attacca la profondità e chi viene incontro, chi segue chi sui calci piazzati). È il modo moderno di dare un ruolo: non un nome, ma cosa fa in ogni fase.
3. **Il blocco con valori continui:** l'altezza della linea, l'ampiezza, la distanza fra i reparti, in metri (oggi tre scatti: alto, medio, basso).
4. **Il pressing:** dove comincia, cosa lo fa scattare (il retropassaggio, il controllo sbagliato, la palla al terzino), chi esce per primo e chi copre, se si spinge l'avversario verso la fascia o verso il centro.
5. **Le marcature:** a zona, a uomo o miste, per fase e per calcio piazzato, con l'avversario preciso.
6. **La costruzione e il ritmo:** il portiere corto o lungo, quanto si rischia nel passaggio, il ritmo.
7. **I movimenti e le rotazioni:** attaccare la profondità, venire incontro, sovrapporsi, restare largo, accentrarsi; due giocatori che si scambiano il posto durante l'azione; lo scambio di posizione (decisione AG).
8. **La difesa preventiva:** quanti restano dietro la palla quando si attacca, e dove.
9. **Le transizioni non si disegnano: si scelgono.** Palla persa: riaggredire, ricomporsi, abbassarsi; palla recuperata: ripartire o tenere palla (ADR-059 §6); con quanti secondi di riaggressione e chi la fa. Una scelta di base prima della partita, che l'allenatore cambia con le fasi della partita, quando la legge.
10. **I calci piazzati disegnati:** posizioni, corse e tiratori per angoli, punizioni e rimesse, in attacco e in difesa, salvati come schemi.
11. **La gestione:** perdere tempo, il ritmo a fine partita.

**La libertà comprende l'errore.** Una tattica assurda (dieci uomini in attacco) si può fare; le conseguenze arrivano da sole, e il motore le racconta. In L2 lo staff lo fa notare prima.

**Gli allenatori attori usano la stessa lavagna:** la loro idea di gioco è un disegno, preso dalle posizioni medie per fase delle squadre vere della stagione di riferimento (grado 2).

**Le tattiche sono dati:** file di testo, che si salvano, si caricano e si scambiano come i pacchetti (decisione AD, punto 9). Un verbo nuovo della lavagna nasce in L0 con il suo ADR.

## I discorsi (decisione AM)

La stessa regola della tattica: chi parla sceglie liberamente cosa dire, chi ascolta lo legge con le sue caratteristiche. Nessun dado, nessun "+5 al morale".

**Spiegare il piano.** Una tattica funziona se la squadra la capisce. La **riunione tattica** è una seduta della settimana in `ds-week` (il video sull'avversario, la spiegazione del piano gara), con il suo carico mentale. La **comprensione del piano** è uno stato di ogni giocatore: cresce con le riunioni, con le prove in campo e con le partite giocate in quel modo, quanto dice la sua lettura del gioco e l'abilità "spiegare la tattica" di chi la spiega. In partita, chi ha capito poco esegue peggio e chi ha capito bene la applica anche quando la situazione si complica: dalla sua scena, non da un numero aggiunto.

**Il discorso si compone a pezzi** (più gestibile del testo libero, che andrebbe interpretato e romperebbe il determinismo o ingannerebbe):

- **a chi:** la squadra, un reparto, un giocatore;
- **quando:** prima della partita, all'intervallo, dopo, in settimana, in un colloquio privato;
- **cosa:** elogio, richiesta, calma, stimolo, critica, un promemoria tattico, una promessa;
- **su cosa:** un fatto vero (il suo errore, il suo gol, l'avversario, la classifica);
- **come:** calmo, deciso, duro.

**Chi lo riceve lo legge da attore:** con le sue caratteristiche psicologiche (la sensibilità, l'ego, la professionalità) e i suoi stati (fiducia, morale, nervi). L'effetto è sugli stati psicologici, che si vedono dai segni nel piano B. Test gemello: lo stesso discorso a due giocatori uguali tranne la sensibilità, due reazioni diverse.

**La credibilità.** Il giocatore ha vissuto la partita e sa cosa è successo: un elogio dopo una partita orribile non vale, una promessa non mantenuta pesa sulla prossima. Ogni giocatore ha un **rapporto con il suo allenatore**, uno stato costruito solo da fatti: le promesse mantenute o no, la coerenza fra le parole e quello che succede. Per il giocatore umano, che non ha caratteristiche, la forza dei discorsi è questa storia; per gli allenatori attori si aggiunge la loro comunicazione.

**Le promesse di L1** sono quelle che L1 sa verificare: i minuti e il posto da titolare. Quelle sul mercato e sul contratto arrivano con L2.

**L'intervallo e i richiami dalla panchina** sono durante la partita: input del motore con il loro tick, come le istruzioni (ADR-059), letti dal plugin psicologia di L0. Vanno in L0, nella lista della decisione T, con un'aggiunta all'ADR-059. I discorsi fuori dalla partita sono di L1.

## Lo scambio di posizione (decisione AG)

Un allenatore può scambiare due giocatori sulla lavagna, prima e durante la partita: invertire le ali, spostare un centrocampista a terzino, allargare il centravanti. È geometria del piano, non un numero del giocatore (ADR-059 §3). Le conseguenze vengono dal piede di ciascuno: il destro a sinistra si accentra per tirare (l'ala a piede invertito), il terzino sul lato sbagliato crossa col piede debole. Nessun effetto da inventare.

Va in L0, nella lista della decisione T, con il suo test gemello: la stessa partita con le ali invertite, e ognuna tira e crossa con il piede che ha.

## Come sceglie, senza dadi

- **L'undici:** per ogni posto del piano, il giocatore con la stima migliore per quel posto pesata dal suo stato del giorno (freschezza, condizione, ritmo partita, il carico della settimana). L'abilità "scegliere l'undici" decide quanto pesa lo stato contro la gerarchia.
- **A parità gioca chi ha giocato meno di recente.** È un criterio del calcio vero, non un'estrazione, e la rotazione nasce da sola.
- **Lo stesso mondo, la stessa data e le stesse stime danno la stessa formazione.**

## Quando sceglie

La formazione si annuncia in un momento della giornata, prima del calcio d'inizio, con l'anticipo delle regole, con l'evento `LineupAnnounced`. Fino a quell'ora il manager la cambia; se un giocatore si fa male in allenamento prima dell'annuncio, l'allenatore ne tiene conto.

## Il manager e la proposta del vice

Quando il manager apre la formazione trova **la proposta del vice**: quella che sceglierebbe un vice con le caratteristiche dell'allenatore del club, scritta come tale. Il vice è un attore come gli altri, con il suo albero. Il manager la cambia come vuole; un pulsante "come l'ultima" riprende la formazione della partita precedente. Le scelte del manager entrano come input datati (ADR-060). In L2 il vice diventa una persona dello staff, con le sue caratteristiche; il meccanismo resta questo. È anche il modo per chi gioca sul telefono di non rifare la formazione ogni settimana.

## Le regole che non si violano

Nessun giocatore squalificato, infortunato o fuori lista. La lista si fissa all'inizio della stagione (in L1 non c'è il mercato), con le regole della lista come dati modulari.

## I test

- **Gemelli:** due rose uguali tranne un giocatore stanco, e l'allenatore sceglie l'altro; lo stesso allenatore contro un avversario più forte e uno più debole, e il blocco si sposta quanto dice la sua adattabilità; due allenatori uguali tranne l'analisi dell'avversario, e il piano gara del primo colpisce il vero pericolo dell'avversario più spesso.
- **Determinismo:** lo stesso mondo dà la stessa formazione e lo stesso piano.
- **Le regole:** nessuna formazione con un giocatore squalificato, infortunato o fuori lista; panchina e cambi come dicono le regole della stagione.
- **Taratura (grado 2, sui dati tenuti da parte):** la distribuzione dei moduli, delle altezze del blocco e della rotazione nelle settimane fitte del campionato simulato contro quelle vere.

## Il ruolo: le risposte del maintainer (decisione AJ, 16 novembre 2026)

1. **Un posto solo, ma chi sta sopra può prendere il posto di chi sta sotto.** Il giocatore umano tiene un posto; i ruoli hanno una gerarchia (dichiarata nelle schede dei ruoli), e chi sta più in alto può avocare a sé una scelta di un ruolo sotto di lui: il presidente che impone un acquisto al direttore sportivo, il direttore tecnico che si siede in panchina dopo un esonero. Vale per l'umano e per gli attori. Le conseguenze sono causate, non assegnate: chi si vede togliere una scelta lo legge con le sue caratteristiche (la fiducia, il rapporto con chi l'ha scavalcato).
2. **La carriera con le offerte di lavoro.** Durante la carriera arrivano offerte da altri club, anche per ruoli diversi da quello che hai. Le fanno i dirigenti attori di quei club, con le loro caratteristiche, leggendo la tua storia: i risultati e le scelte che hai fatto, come le vede chiunque (piano B). Serve la società come attori: L2.
3. **Il resto più avanti.** Il vice umano, gli altri posti giocabili e la gerarchia che avoca arrivano con i plugin che hanno le loro scelte. In L1 resta il meccanismo: ogni scelta dichiara il suo ruolo dal primo giorno.

## Le carriere giocabili (decisione AK)

Ogni ruolo diventa giocabile quando esistono le scelte che gli spettano. I tre poli di un club vero: il campo, il mercato, i soldi.

| Quando | Ruolo giocabile | Perché allora |
|---|---|---|
| L1 | Allenatore | Le scelte di L1 sono le sue: formazione, piano, allenamento |
| L2 | Direttore sportivo | Arriva il mercato: acquisti, cessioni, contratti, osservatori |
| L2 | Presidente | Arrivano società e finanze: budget, nomine ed esoneri, la gerarchia che avoca, le offerte di lavoro |
| Dopo (L3 o i modder) | Vice, direttore tecnico, amministratore delegato, preparatore, team manager | Ruoli in dati, si aggiungono senza toccare il nucleo |

## Fuori da L1

L'esonero e il cambio di allenatore (vengono dalla società, L2); lo staff come persone (L2); gli osservatori per il mercato (L2).
