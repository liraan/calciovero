# Calcio Vero - Deep Simulator · L0 da zero, e il confronto con L0 di oggi

15 novembre 2026 · copia nel repo del doc omonimo (https://claude.ai/artifact/BStJk3e2bP2D9FVgnLQakj). Se cambia il doc, si riallinea questo file.

## Scopo e confini, pensati da zero

L0 è una partita vera: 22 giocatori che pensano, un pallone che obbedisce alla fisica, un arbitro che applica il regolamento, e alla fine un risultato che nessuno ha deciso a tavolino. Questo piano è scritto dal calcio, non dal codice: com'è fatta oggi L0 entra solo nel confronto, più sotto. Conoscevo già molto di L0 dalle nostre sessioni, quindi "da zero" vuol dire dal calcio, non dall'ignoranza del progetto.

**Le regole della partita sono L0.** Il fallo, il cartellino, il fuorigioco, il recupero, i tempi supplementari e i rigori si decidono dentro la partita. In L1 restano solo le regole tra una partita e l'altra: quante ammonizioni fanno una squalifica, gli spareggi in classifica, le promozioni.

**Dentro L0**

- La partita dal fischio d'inizio al fischio finale, compresi supplementari e rigori quando la competizione li chiede.
- Il pallone, i corpi, quello che ogni giocatore vede, quello che ogni giocatore decide, i gesti tecnici, i duelli, il portiere.
- L'arbitro e il regolamento: falli, vantaggio, cartellini, fuorigioco, palla fuori, riprese di gioco, recupero.
- La panchina: istruzioni, cambi, cambi di schieramento (ADR-059).
- Lo stato che cambia durante la partita: fatica, e chi ha preso un colpo.
- La cronaca, le statistiche e il perché di ogni evento, ricavati dai fatti.
- Il salvataggio e il replay identico.

**Fuori da L0:** quello che succede tra due partite (L1) e quello che costruisce una squadra nel tempo (L2).

**L0 è finito quando:** una partita si gioca, si salva, si ricarica e si rigioca identica su ogni piattaforma; ogni evento ha la sua causa leggibile; le statistiche medie stanno dentro le distanze del calcio vero senza che nessun numero sia stato ritoccato per arrivarci; la partita dura abbastanza poco da far stare una giornata di L1 in sottofondo.

## La struttura: un nucleo e i plugin

Come in L1: un nucleo che non conosce il calcio e plugin che lo fanno, ognuno con la scheda dell'ADR-012.

- **Il nucleo della partita** tiene l'orologio a tick, lo stato osservabile del campo (dove sono i corpi e il pallone, chi lo tocca), il registro dei plugin, gli eventi e il record per il replay.
- **Ogni tick ha sette fasi fisse,** sempre nello stesso ordine: percepire, decidere, agire, muovere (fisica), risolvere i contatti, arbitrare, registrare. Ogni plugin si aggancia solo alle sue fasi.
- **Dentro una fase i 22 giocatori si trattano in un ordine fisso,** e nessuno vede le scelte degli altri prese nello stesso tick: decidono tutti sul mondo del tick prima. Così l'ordine non cambia il risultato e la fase si può anche parallelizzare.
- **Undici plugin sono obbligatori,** perché senza di loro non c'è una partita di calcio. **Tre si possono spegnere** (psicologia in partita, ambiente, infortuni in partita): senza, la partita è più povera ma resta vera (ADR-013 §8).
- **Chi guarda non tocca.** Cronaca, statistiche e GUI leggono gli eventi e lo stato, non scrivono mai nella partita.

> Diagramma (nel doc): le sette fasi del tick in fila, e sotto ciascuna i plugin che vi si agganciano.

Ogni plugin si aggancia solo alle sue fasi; il portiere è l'unico che ne attraversa tre, perché è un attore completo.

## I contratti comuni

Otto regole valgono per ogni plugin della partita.

| Regola | Cosa vuol dire |
| --- | --- |
| Il tempo è il tick | Un passo di durata fissa; tutto quello che succede, succede in un tick e in una fase |
| La partita è una funzione | Seme, squadre, configurazione e istruzioni con il loro tick danno sempre lo stesso record, su ogni piattaforma |
| Nessun dado | L'esito di un gesto viene dalle caratteristiche, dalla situazione e dalla fisica, mai da un'estrazione |
| Ogni giocatore pensa | Ognuno dei 22 vede, riconosce le sue possibilità e sceglie; l'intenzione della squadra è qualcosa che legge, non un peso sulle scelte |
| Tre piani dei dati | Le caratteristiche vere sono nascoste dietro una porta; lo stato del campo è osservabile; quello che il manager vede sono stime |
| Ogni evento ha la sua causa | Tipo e campi bastano a dire perché è successo, senza tracce speciali |
| Le statistiche si ricavano | Nessun numero si ritocca per avvicinarsi al calcio vero: una distanza è una domanda sulle cause |
| Ogni numero ha una fonte | Ogni parametro cita da dove viene, o dice che è arbitrario |

**Il record** è il prodotto della partita: gli eventi con le loro cause, le posizioni tick per tick e le istruzioni ricevute. Da solo basta per il replay, la cronaca, le statistiche e l'analisi.

## I plugin, uno per uno

Quattordici plugin: undici obbligatori, tre che si possono spegnere. Fuori dalla partita, come osservatori, stanno cronaca e statistiche.

| Plugin | Fasi del tick | Obbligatorio | Una frase |
| --- | --- | --- | --- |
| Pallone | muovere | sì | Volo, rimbalzo, rotolamento, effetto, pali |
| Corpo | agire, muovere | sì | Accelerare, frenare, girarsi, l'equilibrio |
| Percezione | percepire | sì | Cosa vede ognuno, dove guarda, cosa ricorda |
| Pensiero | decidere | sì | Riconoscere le possibilità e scegliere, con e senza palla |
| Gesti tecnici | agire | sì | Passaggio, tiro, cross, controllo, dribbling, colpo di testa, spazzata |
| Duelli | risolvere i contatti | sì | Contrasto, protezione, duello aereo, il contatto che diventa fallo |
| Portiere | percepire, decidere, agire | sì | Piazzarsi, parare, uscire, rilanciare |
| Arbitro | arbitrare | sì | Falli, vantaggio, cartellini, fuorigioco, recupero, supplementari e rigori |
| Palle inattive | decidere, agire | sì | Calcio d'inizio, rimesse, angoli, punizioni, rigori, barriera |
| Panchina | decidere | sì | Istruzioni, cambi, schieramento (ADR-059) |
| Fatica | muovere | sì | L'energia che cala e torna nei 90 minuti |
| Psicologia | decidere | no | Lucidità sotto pressione, coraggio, nervi |
| Ambiente | muovere | no | Pioggia, vento, campo pesante |
| Colpi | risolvere i contatti | no | Il contatto che fa male e chi deve uscire |

### Pallone

- **Legge:** il gesto che lo ha colpito (dove, con che parte, con che forza), il campo.
- **Scrive:** posizione, velocità, effetto del pallone a ogni tick.
- **Funzioni:** traiettoria in aria con resistenza ed effetto, rimbalzo, rotolamento con attrito, pali e traversa, la linea superata (gol, fondo, laterale); dire in anticipo dove il pallone sarà, con la stessa formula che poi lo muove.

### Corpo

- **Legge:** l'intenzione di movimento, le caratteristiche fisiche, la fatica.
- **Scrive:** posizione, velocità, orientamento del corpo, equilibrio.
- **Funzioni:** accelerazione e velocità massima, cambi di direzione che costano, girarsi, il corpo e lo sguardo che non puntano per forza nello stesso posto, la distanza a cui si arriva sul pallone.

### Percezione

- **Legge:** lo stato del campo, dove guarda ognuno.
- **Scrive:** la scena di ogni giocatore (compagni, avversari, pallone, spazi), con quello che ricorda di quello che non vede più.
- **Funzioni:** campo visivo, coperture, il guardarsi intorno come comportamento, la memoria che invecchia.

### Pensiero

- **Legge:** la propria scena, l'intenzione della squadra, le proprie caratteristiche, lo stato del pallone.
- **Scrive:** l'azione scelta e il momento in cui partirla.
- **Funzioni:** con la palla, riconoscere passaggi, tiri, conduzioni e dribbling possibili e sceglierne uno; senza palla, l'obiettivo primo del pallone, smarcarsi, coprire, pressare, tenere la posizione; il tempo di ogni scelta.

### Gesti tecnici

- **Legge:** l'azione scelta, il corpo, la pressione addosso, le caratteristiche tecniche.
- **Scrive:** il contatto col pallone (punto, superficie, forza, direzione) e quanto si allontana da quello voluto.
- **Funzioni:** passaggio corto e lungo, tiro, cross, controllo e ricezione dove arriva il pallone, conduzione a tocchi, dribbling, colpo di testa, spazzata, la deviazione involontaria.

### Duelli

- **Legge:** i corpi vicini, chi vuole il pallone, le caratteristiche fisiche e di carattere.
- **Scrive:** chi esce col pallone, il contatto, il contatto falloso.
- **Funzioni:** contrasto in piedi e in scivolata, protezione del pallone, spallata, duello aereo; il contatto che resta lecito e quello che diventa fallo, senza dadi.

### Portiere

- **Legge:** la propria scena, il tiro o il cross in arrivo.
- **Scrive:** piazzamento, tuffo, presa, respinta, uscita, rinvio.
- **Funzioni:** piazzarsi leggendo l'azione, la parata come incontro tra la traiettoria e il corpo, uscire sui cross e in profondità, rilanciare con le mani e coi piedi.

### Arbitro

- **Legge:** contatti, posizioni al momento del passaggio, il pallone, il tempo perso.
- **Scrive:** fischio, ripresa di gioco, cartellino, recupero, fine del tempo.
- **Funzioni:** giudicare il fallo, il vantaggio, l'ammonizione e l'espulsione, il fuorigioco, la palla fuori, il recupero dal tempo perso, i supplementari e i rigori quando la competizione li chiede.

### Palle inattive

- **Legge:** la ripresa decisa dall'arbitro, le istruzioni della panchina.
- **Scrive:** dove si mettono i giocatori, chi batte, la barriera.
- **Funzioni:** calcio d'inizio, rimessa laterale, rinvio dal fondo, angolo, punizione diretta e indiretta, rigore, la serie dei rigori.

### Panchina

- **Legge:** le istruzioni con il loro tick, la fatica, i cartellini, il risultato.
- **Scrive:** l'intenzione della squadra, i cambi, il nuovo schieramento.
- **Funzioni:** istruzioni di squadra e individuali dal manager; le scelte dell'altro allenatore senza dadi; i cambi, con le loro regole (quanti, quando).

### Fatica

- **Legge:** quanto corre e quanto scatta ognuno, le caratteristiche fisiche.
- **Scrive:** l'energia che c'è adesso.
- **Funzioni:** l'energia che scende con gli sforzi e torna nelle pause; la stanchezza che rallenta e peggiora i gesti.

### Psicologia, si può spegnere

- **Funzioni:** la lucidità sotto pressione, il coraggio nei contrasti, i nervi dopo un errore o un torto; tutto come stato che cambia con i fatti della partita.

### Ambiente, si può spegnere

- **Funzioni:** pioggia e campo pesante sul rotolamento, vento sul volo del pallone.

### Colpi, si può spegnere

- **Funzioni:** il contatto che fa male, misurato dalla sua forza, e il giocatore che deve uscire; consegna a L1 l'inizio dell'infortunio.

## Test e ordine di lavoro

**Le prove che chiudono L0:** il golden di una partita su più piattaforme; i test gemelli per ogni caratteristica (due giocatori uguali tranne una, la stessa scena); gli scenari ripetibili (un cross, un rigore, un fuorigioco); il confronto con il calcio vero su almeno 60 semi; la durata di una partita misurata a ogni cambio.

**L'ordine, dal più fisico al più umano**, perché ogni passo regge sul precedente:

1. Il nucleo: tick, fasi, record, replay, determinismo su più piattaforme.
2. Pallone e corpo: la fisica, prima ancora delle persone.
3. Gesti tecnici e ricezione: il contatto col pallone.
4. Percezione: cosa vede ognuno.
5. Pensiero: con e senza palla, con l'intenzione della squadra.
6. Duelli e portiere.
7. Arbitro e palle inattive: la partita diventa regolare.
8. Panchina e fatica: cambi e istruzioni.
9. Il realismo derivato: le distanze dal calcio vero lette come cause.
10. I tre plugin che si possono spegnere.

## Il confronto con L0 di oggi

Il calcio di L0 c'è quasi tutto: dei quattordici plugin ideali, cinque sono fatti, sette sono fatti in parte, due mancano, e anche il nucleo è a metà. La differenza grande è la forma: L0 non ha plugin, è un motore unico. Letto su `main` (`e02c08b`, `SIM_VERSION` 112, 15 novembre).

| Plugin ideale | Dove sta oggi | Stato | Cosa manca |
| --- | --- | --- | --- |
| Nucleo | `ds-match` (tick, stato, record, salvataggio), `ds-stats` (catena degli eventi) | in parte | Nessun registro di plugin né scheda; golden verificato solo su Linux, non ancora su Windows e macOS (OI-99) |
| Pallone | `ds-match/spatial.rs` | in parte | Volo con la resistenza dell'aria, rimbalzo, pali; manca l'effetto (nessuna traccia di spin) |
| Corpo | `ds-match`, ADR-031 e ADR-033 | fatto | Il corpo rispetto alla palla non è ancora letto del tutto (OI-64) |
| Percezione | `ds-match/perception.rs`, ADR-034 | in parte | I giocatori vedono le posizioni esatte: è la causa aperta più grande; ADR-062 ne fissa la forma, la grandezza aspetta una fonte |
| Pensiero | `ds-orchestrator`, ADR-043, 047, 052, 056 | fatto | L'intenzione della squadra è a metà: ADR-059 ai passi 1 e 2, istruzioni rapide e transizioni al passo 3 |
| Gesti tecnici | `ds-orchestrator`, `ds-calculator`, ADR-037, 048, 053, 054, 058 | fatto | Troppo precisi: passaggi riusciti 94,5% contro 83% (OI-100) |
| Duelli | ADR-042, `balance/duel.json` | fatto | I valori di `duel.json` sono in gran parte senza fonte; il portatore protegge quasi sempre (OI-103); il secondo uomo sui palloni alti non c'è (OI-102) |
| Portiere | ADR-045, ADR-046 | in parte | Piazzarsi leggendo il tiro, l'uscita come scelta (passo 10d) |
| Arbitro | `ds-match/rules.rs`, ADR-021, 026 | in parte | Fuorigioco, falli, cartellini, vantaggio, rigori, recupero e la chiara occasione da rete ci sono; supplementari e rigori finali sono solo un campo nel modello, non giocati; `strictness` va tolto (OI-55) |
| Palle inattive | `ds-match`, passo 74 | in parte | Riprese con le distanze delle Leggi; manca la barriera e la serie dei rigori |
| Panchina | `ds-model/squad.rs`, ADR-059 | in parte | Cambi pianificati prima del fischio e fatti alla prima palla ferma; nessun allenatore che reagisce alla partita; istruzioni al passo 3 |
| Fatica | ADR-030, passo 73 | fatto | La concentrazione che cala, secondo appannatore (OI-70) |
| Psicologia | — | manca | Lucidità, coraggio e aggressività sono nel piano (passo 10), non ancora nel codice |
| Ambiente | `ds-model/pitch.rs` | in parte | Il meteo esiste nel modello; quanto pesa sul pallone è da verificare |
| Colpi | — | manca | L'evento `Injury` esiste ma nessuno lo produce |

**Le differenze di struttura,** più importanti delle caselle:

- **Nessun sistema di plugin.** `ds-match` ha sette motivi per cambiare (tick, movimento, percezione, eventi, Leggi, salvataggio, cronaca): l'ADR-041 lo dichiara e rimanda la divisione (OI-94). Il trait `MatchPlugin` dell'ADR-012 non è mai stato definito.
- **Le fasi del tick non sono un contratto scritto.** Chi decide e quando lo stabilisce l'orchestratore, con il suo tempo per ogni scelta (ADR-056). Non ho verificato se tutti decidono sul mondo del tick prima, come chiede il piano ideale.
- **Chi guarda sta dentro il motore.** La cronaca (`narrative.rs`) vive in `ds-match`; il bus degli eventi di `ds-core` non ha consumatori (OI-40).
- **I documenti invecchiano rispetto al codice** (OI-31, OI-49).

## Cosa conviene fare

Non riscrivere L0: il calcio che contiene è già quello del piano ideale, e le distanze aperte hanno cause note. Conviene aggiungere i pezzi che mancano, poi dargli la forma a plugin prima di L1.

1. **Prima di chiudere L0, i pezzi di calcio che L1 userà subito.**
   - Supplementari e serie dei rigori: la coppa di L1 non si gioca senza.
   - Un allenatore avversario che reagisce in partita (cambi e istruzioni), non solo cambi pianificati.
   - La barriera sulle punizioni.
   - Il colpo che fa uscire un giocatore, da consegnare agli infortuni di L1.
2. **La percezione con l'errore (ADR-062) resta la prima causa da chiudere.** Spiega insieme la linea del fuorigioco letta alla perfezione, i palloni che non escono e una parte della precisione troppo alta.
3. **Lucidità, coraggio e aggressività,** già nel passo 10: sono il plugin "psicologia" del piano ideale.
4. **A L0 chiuso, un ADR che divide `ds-match` in plugin** (OI-94), con lo stesso contratto della stagione: un crate pubblico `ds-match-api` come `ds-season-api`, la scheda, le fasi del tick scritte. Va fatto prima del passo 1 di L1, perché i due contratti si somigliano e i contributor imparano una volta sola. Il golden deve restare uguale: è una divisione, non un cambio di calcio.
5. **Le pulizie già in elenco:** togliere `strictness` (OI-55), usare o togliere il bus degli eventi (OI-40), portare la cronaca fuori dal motore, riallineare i documenti (OI-31, OI-49).
6. **Il golden su Windows e macOS** (OI-99): il PC del maintainer è Windows, quindi il primo controllo può farlo Claude Code lì, in pochi minuti.
