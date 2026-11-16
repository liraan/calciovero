# Il design della GUI — decisioni del maintainer

Le scelte di design della GUI prese con il maintainer tra il 13 e il 16 novembre 2026. I render sono nella tela del design (artifact "Calcio Vero - GUI", pagine Lavagna e Android). La GUI legge e non scrive: ogni post-it mostra le viste di un plugin dietro il view model (ADR-055); le scelte del manager tornano come input datati.

**Regola di lavoro:** il design si discute prima di applicarlo (RULES A1). I gruppi e i post-it nuovi si decidono quando c'è il contenuto da metterci, non prima.

## La lavagna, sfondo di tutto

- Lo sfondo è la lavagna tattica, in tre temi: **Lavagna**, **Notte**, **Carta**. I post-it seguono il tema.
- La lavagna è pulita: solo le linee del campo e i giocatori, nessun'altra linea o scritta.
- Un selettore di fase: **Con palla**, **Transizione**, **Senza palla**. In partita c'è anche **Nessuna**, e la forma pianificata si vede in trasparenza.
- In alto, la barra di sistema prende i colori della squadra.
- Il pulsante con il logo di Calcio Vero apre solo le opzioni del gioco.

## I post-it

- **Partono come linguette sul lato sinistro**, sempre a sinistra, raggruppate per colore: **Scrivania**, **Squadra**, **Tattica**, **Partita**. La barra in alto non contiene post-it.
- Quando un post-it è aperto, la sua linguetta sparisce dal lato.
- Ogni post-it si sposta e si riduce. La **Rosa** parte aperta al centro.
- **I post-it aperti si agganciano** in un'unica cornice: affiancati con un divisore trascinabile, oppure impilati come schede. Un gruppo ridotto diventa una sola linguetta sotto **Gruppi**; ⇱ lo stacca.
- Cliccando un giorno del calendario si apre l'agenda della **Giornata**: allenamenti, partita, impegni.
- In partita, il post-it delle **istruzioni rapide**: hanno effetto diretto sulla squadra (ADR-059).

## La scheda giocatore (16 novembre 2026)

Ispirata all'idea della skin "Mustermann Iconic" di Football Manager (fasce al posto dei numeri, torta dei percentili, badge), senza copiarne grafica, icone o layout. Da noi l'idea è il modello: il manager non vede le caratteristiche vere, solo le stime di chi lavora per lui (piano C, ADR-057).

1. **Fasce di giudizio, mai numeri.** Quattro fasce: eccellente, buono, nella media, scarso. Quando il giudizio è incerto, l'icona copre due fasce ("tra buono e nella media"); con più partite viste e uno staff migliore diventa netta.
2. **Raggruppate per dove si vedono in campo:** con palla, senza palla, fisico, testa. Ogni gruppo si collega alle fasi della lavagna.
3. **Ogni giudizio ha la firma e la data:** il preparatore per i nostri giocatori, l'osservatore per quelli degli altri. In L1 è la stima del club; in L2 sono persone dello staff, con le loro inclinazioni.
4. **La torta delle statistiche:** i percentili rispetto ai giocatori dello stesso ruolo in campionato. Qui i numeri ci sono, perché le statistiche delle partite giocate sono fatti osservabili (piano B). Compare dopo qualche partita.
5. **I badge si guadagnano** da quello che il giocatore fa davvero in partita (per esempio "regista", "incursore"); il ruolo è un'etichetta (ADR-017).
6. **Lo stile segue i tre temi** della lavagna e i colori della squadra.

**Stato:** il primo render (16 novembre 2026: intestazione con badge, la legenda delle fasce, i quattro gruppi da quattro voci, la firma, la torta di otto statistiche con il 50° tratteggiato) è accettato dal maintainer come base. Si raffina con i betatester, non prima.

## Android (ADR-061)

- **Verticale:** la proposta A, la lavagna con un cassetto che contiene i post-it.
- **Orizzontale:** la schermata partita della proposta C.
- Si parte da qui per popolare la GUI di contenuti; poi si migliora con i betatester.
- Pacchetti grafici leggeri per il telefono (loghi e foto a bassa risoluzione), stessi plugin del PC, distribuzione fuori da Google Play finché la decisione 4 di `context/plans/DECISIONS-2026-11-15.md` non apre un'altra strada.

## La partita in 3D e in 2.5D (16 novembre 2026)

**Le tre viste, una per strumento** (maintainer, 16 novembre 2026):

| Vista | Dove | Con cosa |
|---|---|---|
| 2D dall'alto, a caratteri | il terminale (`ds-demo --tui`) | testo, com'è oggi |
| 2.5D, la lavagna inclinata | la GUI, su desktop e Android | egui |
| 3D | solo desktop, quando arriverà | Bevy, se sarà maturo |

Tutte e tre leggono la stessa registrazione della partita e non scrivono mai nella simulazione.

- **Oggi e fino al 3D: egui**, su desktop e Android (ADR-055).
- **Desktop, quando arriverà il 3D (dopo la 0.1): Bevy, se sarà abbastanza maturo.** Conta la comunità: è la più grande del mondo Rust per i giochi. Fyrox è escluso, con dispiacere: ha l'editor e API stabili dalla 1.0 (marzo 2026), ma una comunità piccola e una grafica ancora OpenGL, a rischio su macOS. Bevy usa wgpu (Vulkan, Metal, DirectX 12).
- **"Abbastanza maturo" si misura così,** al momento della scelta: un anno senza cambiamenti incompatibili che costringano a riscrivere la vista, oppure la 1.0; Windows, Linux e macOS che girano senza problemi noti; bevy_egui mantenuto. Se non ci siamo, il 3D aspetta: il 2.5D in egui basta.
- **La migrazione è piccola per costruzione:** la vista 3D è un programma o un crate separato che legge la registrazione della partita, e non scrive mai nella simulazione (i float del disegno non toccano il determinismo). Con bevy_egui i pannelli egui esistenti (post-it, scheda, lavagna) possono girare dentro Bevy, quindi non si riscrive l'interfaccia.
- **Il 2.5D in egui, su desktop e Android:** la lavagna inclinata in prospettiva, con figurine con altezza e il pallone con la sua ombra (prototipo del 16 novembre). Lo stesso codice per le due piattaforme, quindi il costo è uno solo. **Il 2.5D sostituisce il 2D** (maintainer, 16 novembre 2026): è più utile anche per osservare, perché mostra l'altezza del pallone. Non si perde niente: l'inclinazione a 0° è la vista dall'alto di oggi, e resta come preset della telecamera, non come codice a parte. Sul desktop, quando arriverà, si aggiunge il 3D.
- Le figurine si generano dai colori della squadra, come gli stemmi; nessun asset protetto (decisioni F–K).

## Lo strato di interpolazione della vista (16 novembre 2026)

**La regola:** il motore resta a 10 tick al secondo di gioco, deterministico e veloce; la vista disegna alla frequenza dello schermo (60, 120, 144 Hz o più), ricostruendo ogni fotogramma per l'istante esatto. "120 Hz" vuol dire 120 immagini al secondo, non una partita più veloce: il tempo di gioco scorre alla velocità scelta da chi gioca. Uno strato solo, scritto una volta, per il 2D, il 2.5D e il 3D.

1. **Un tick di ritardo.** La vista disegna 100 ms indietro rispetto all'ultimo tick calcolato, così ha sempre il tick prima e quello dopo: interpola, non estrapola mai.
2. **I giocatori con le curve.** Tra due tick la traiettoria usa posizione e velocità (interpolazione di Hermite), non una linea retta; l'orientamento del corpo si interpola sull'angolo.
3. **Il pallone con la sua fisica.** Tra due tick il pallone vola: la vista ricalcola gravità, resistenza dell'aria e rimbalzo a piccoli passi, con la forma della fisica del motore. È solo disegno, in float, e non torna mai nella simulazione; dove il disegno e il tick successivo non coincidono, vince il tick.
4. **I momenti di contatto.** Un tiro, un passaggio o un colpo di testa avvengono a un tick preciso, noto dagli eventi: la traiettoria del pallone si spezza lì, invece di arrotondare l'angolo. Le figurine possono anticipare il gesto di qualche fotogramma.

**I limiti:**
- Ogni fotogramma si calcola dall'ora esatta, non contando i fotogrammi: se il telefono abbassa la frequenza per batteria o calore, la vista si adegua da sola.
- A velocità molto accelerata (più tick di gioco al secondo che fotogrammi) l'interpolazione non serve più e la vista salta da tick a tick.

**Da verificare nel codice prima di iniziare:** se la registrazione della partita contiene già la velocità di giocatori e pallone, l'altezza del pallone e l'orientamento dei corpi. Se contiene solo le posizioni, la velocità si ricava dai tick vicini, con meno precisione nelle frenate brusche. Aggiungere un campo alla registrazione cambia il formato del salvataggio e del replay, quindi va fatto con il suo ADR.

## Il lavoro del 2.5D

**Quando:** appena L0 è chiuso, prima di aprire L1 (maintainer, 16 novembre 2026). Stima: circa 2–4 sessioni di Claude Code.

La registrazione della partita ha già quello che serve (`ds-match/src/record.rs`): la posizione del pallone in (x, y, z) con la sua altezza, lo stato del pallone, l'orientamento di ogni giocatore (`heading`), la fatica visibile e gli eventi di ogni tick. Le velocità si ricavano dai tick vicini. Nessun cambio al motore né al formato del salvataggio.

1. La proiezione in prospettiva con l'inclinazione e i preset della telecamera (0° = dall'alto), al posto del disegno di `tools/ds-gui/src/scene.rs`.
2. Le figurine con i colori della squadra e l'orientamento; il pallone con l'ombra; le porte con pali e traversa in altezza; l'ordine di disegno per profondità.
3. Lo strato di interpolazione (sopra).
4. I tre temi della lavagna; la prova su Android.
5. **Il README** (maintainer, 16 novembre 2026): appena il 2.5D è disponibile, entra nel README. Una GIF della vista 2.5D (un cross e un colpo di testa, con il seme e la versione del motore nella didascalia) al posto di `assets/demo-match.gif` in cima, lo screenshot della finestra aggiornato, e la vista nelle tabelle dello stato e in "Try the engine today".
