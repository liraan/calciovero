# L1 · L'allenatore (`ds-coach`)

Il piano del plugin allenatore di L1: cosa sceglie ogni allenatore prima della partita, da cosa, e come lo fa senza dadi. In partita reagisce la panchina di L0 (decisione T), che legge il piano scritto qui. Decisioni AF, AG e AH (maintainer, 16 novembre 2026), con i filtri partita vera, LUNA e futuribilità.

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

Il tipo `Actor` di `ds-orchestrator`, oggi solo il giocatore, diventa l'attore di ogni tipo (giocatore, allenatore, poi staff e dirigente), con la stessa porta verso il caveau. Serve un ADR prima del passo 5 di L1. Il manager umano è l'unico senza caratteristiche: le sue scelte sono input datati.

## L'albero dell'allenatore

Una proposta da correggere con il maintainer ramo per ramo, come quella dei giocatori. Le caratteristiche nascono dal livello del club e da come hanno giocato le squadre vere nella stagione di riferimento (i dati di squadra e delle partite del riferimento Wyscout), con nomi inventati; si misurano sull'effetto (grado 2) e si citano in `balance/SOURCES.md`, tranne dove è detto.

**Le radici, in quattro famiglie:**

| Famiglia | Caratteristiche |
|---|---|
| **Idea di gioco** (i gusti, non la bravura) | il modulo preferito, l'altezza del blocco, l'ampiezza, il gioco diretto o di possesso, le scelte di transizione |
| **Mente tattica** | la lettura della partita, la conoscenza tattica, l'adattabilità (quanto si sposta dalla sua idea) |
| **Metodo** | l'analisi dell'avversario (grado 3 in L1, dal livello del club, finché lo staff di L2 non la rende misurabile), l'attenzione alla condizione (quanto pesa la freschezza contro la gerarchia) |
| **Carattere** | la propensione al rischio (un giovane, un attaccante in più, la linea alta), la tenuta sotto pressione, l'ostinazione |

**I rami, le abilità (formule):**

| Abilità | Da quali radici | Dove agisce |
|---|---|---|
| Preparare il piano gara | analisi, conoscenza tattica, adattabilità | prima della partita |
| Scegliere l'undici | attenzione alla condizione, propensione al rischio, conoscenza tattica | prima della partita |
| Leggere la partita | lettura, tenuta sotto pressione | in partita, nella panchina di L0: quando reagisce (decisione Y) |
| Correggere in corsa | conoscenza tattica, adattabilità, propensione al rischio | in partita: cosa cambia |

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

## Fuori da L1

L'esonero e il cambio di allenatore (vengono dalla società, L2); lo staff come persone (L2); gli osservatori per il mercato (L2).
