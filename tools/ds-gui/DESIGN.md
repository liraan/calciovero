# Il design della GUI — decisioni del maintainer

Le scelte di design della GUI prese con il maintainer tra il 13 e il 16 novembre 2026. I render sono nella tela del design (artifact "Calcio Vero - GUI", pagine Lavagna e Android). La GUI legge e non scrive: ogni post-it mostra le viste di un plugin dietro il view model (ADR-055); le scelte del manager tornano come input datati.

**Regola di lavoro:** il design si discute prima di applicarlo (RULES A1), e **ogni cambio visivo si mostra prima al maintainer con un'anteprima** (maintainer, 16 novembre 2026: "dammi sempre una preview"): un prototipo o uno screenshot, prima di passarlo al codice. I gruppi e i post-it nuovi si decidono quando c'è il contenuto da metterci, non prima.

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

## La visuale del 2.5D: telecamere, omini, animazioni (16 novembre 2026)

Decisa dal maintainer guardando il prototipo con gli omini articolati alla grandezza vera.

**La grandezza vera decide.** Un giocatore è alto 1,8 m. Sul campo intero, in una finestra larga circa 700 pixel, è alto circa 10 pixel: si vede che corre e che salta, non la gamba che calcia. Con la telecamera vicina (circa 38 m di campo nella larghezza) è alto circa 40 pixel e ogni gesto si legge. Le animazioni servono solo insieme alla telecamera giusta.

**Le telecamere:**

| Preset | Cosa mostra | A cosa serve |
|---|---|---|
| Campo intero | tutto il campo, inclinato | la forma della squadra, le linee, le distanze |
| Telecamera vicina | circa 38 m attorno al pallone; segue il pallone con un movimento smorzato, senza uscire dal campo | l'azione: i gesti, i duelli, i tiri |
| Dall'alto | inclinazione 0° | la lavagna di sempre |

Più il cursore dell'inclinazione e un tasto per il rallentatore (circa un terzo della velocità), utile anche per trovare i difetti del motore.

**Gli omini articolati.** Gambe, busto, braccia e testa disegnati dal codice, nessun asset: maglia del colore della squadra, calzettoni più scuri, il portiere nel suo colore, l'ombra a terra. Lo spessore delle linee e la testa hanno un minimo in pixel, così sul campo intero restano visibili.

**Le animazioni nascono solo dai fatti della partita.** Mai una posa per riempire: se il giocatore salta, c'è un duello aereo; se cade, c'è un fallo. Così la vista resta uno specchio del motore.

| Animazione | Da dove la ricava la vista |
|---|---|
| La falcata, con il ritmo e l'ampiezza dalla velocità; fermo, quasi immobile | la velocità ricavata dai tick |
| L'inclinazione del busto in accelerazione, frenata e curva | il cambio di velocità e di orientamento (`heading`) |
| Il tiro, il passaggio, il cross: la gamba che carica e calcia | l'evento al tick del contatto, anticipato di qualche fotogramma |
| Il salto con le braccia alte, nel colpo di testa e nel duello aereo | l'evento del duello o del colpo di testa, con l'altezza del pallone |
| Il tuffo del portiere, la presa, la respinta | l'evento della parata e la posizione del pallone |
| La scivolata e il contrasto in piedi | l'evento del contrasto |
| La caduta dopo un fallo, il rialzarsi | l'evento del fallo |
| La rimessa con le mani sopra la testa, il rilancio del portiere | l'evento della ripresa di gioco |
| L'esultanza dopo il gol | l'evento del gol |
| La stanchezza: passo più pesante, busto curvo | la fatica visibile (`fatigue_milli`) |
| L'arbitro che mostra il cartellino | l'evento del cartellino |

**L'ordine:** prima le telecamere e gli omini con la falcata; poi tiro, salto, tuffo e scivolata; poi il resto della tabella. Anche questo è solo disegno: niente `SIM_VERSION`, golden identico.

## Pallini nel 2.5D, omini nel 3D (16 novembre 2026)

Decisione del maintainer guardando gli omini del 2.5D renderizzati dal codice di main: nel 2.5D i giocatori sono **pallini colorati con il numero**, con tutte e tre le telecamere; gli omini articolati e i gesti da vicino sono del **3D** (desktop, Bevy). Sostituisce la sezione "Gli omini articolati" qui sopra per il 2.5D.

**Perché:** un pallino con il numero dice sempre chi è chi e resta leggibile anche sul campo intero, dove un omino è alto circa 10 pixel; ha il gusto dei vecchi manageriali; costa poco. Il gioco aereo resta visibile, perché il 2.5D ha l'altezza.

**Il pallino:**
- il disco del colore della squadra (il portiere nel suo colore, l'arbitro in nero), con il numero di maglia dentro e un bordo sottile;
- un piccolo segno sul bordo che indica dove guarda (`heading`);
- l'ombra a terra, un'ellisse schiacciata e semitrasparente;
- l'anello attorno al portatore di palla;
- la fatica come colore un po' più spento, se si legge bene, altrimenti niente.

**I gesti dei pallini** (stessa tabella evento → animazione dell'ADR-065, stessa regola: solo dai fatti della partita):

| Fatto | Il pallino |
|---|---|
| Duello aereo, colpo di testa | si stacca da terra e ricade, l'ombra resta sotto |
| Tuffo del portiere | si allunga e si sposta verso il lato del pallone |
| Scivolata, contrasto | si schiaccia e scivola un poco in avanti |
| Caduta dopo un fallo | si schiaccia a terra e si rialza piano |
| Tiro, passaggio, cross | un piccolo impulso verso la direzione del colpo |
| Gol | piccoli salti |
| Cartellino | il pallino dell'arbitro mostra un rettangolo giallo o rosso |

**Il pallone:** un raggio minimo più grande di oggi e un bordo scuro sottile; in volo, una linea leggera fino all'ombra, come in televisione.

**Il codice degli omini articolati** esce dalla GUI (RULES D3: resta solo ciò che si usa) e vive nella storia di git; il 3D con Bevy avrà il suo. La tabella evento → animazione resta, perché la usano i pallini e la userà il 3D.

## Una partita, molte viste: lo zoom al posto dei preset (16 novembre 2026)

**Il principio** (maintainer): la partita è una sola, la registrazione del motore; ogni vista la guarda da un'altra distanza o con un altro vestito (caratteri nel terminale, pallini nel 2.5D, omini nel 3D) e non cambia mai quello che è successo.

Di conseguenza nel 2.5D "campo intero" e "telecamera vicina" non sono più modi diversi di disegnare, ma solo livelli di zoom. I tre preset diventano:
- **uno zoom continuo**: rotellina del mouse sul desktop, pizzico con due dita su Android; dal campo intero fino a circa 25 m di campo nella larghezza;
- **"segui il pallone"** acceso o spento (acceso, la telecamera segue il pallone con un movimento smorzato, senza uscire dal campo);
- **l'inclinazione**, come oggi, con 0° che resta la vista dall'alto.

**La vista si sposta trascinando** (maintainer, 16 novembre 2026, guardando l'anteprima dei pallini): si trascina il campo con il mouse sul desktop e con un dito su Android. Appena si trascina, "segui il pallone" si spegne da solo; un tasto "torna al pallone" (e il doppio clic o doppio tocco sul campo) lo riaccende e ricentra. Lo spostamento resta dentro i limiti del campo, con un piccolo margine.

I pallini, il numero, il bordo e il pallone hanno grandezze minime in pixel, così restano leggibili a ogni zoom. Il rallentatore resta.

## Il pubblico in 3D (decisione BG, 16 novembre 2026)

Il 3D (desktop, Bevy se maturo, dopo la 1.1) mostra anche gli spalti e il pubblico, che legge la partita e i tifosi senza scrivere mai nella simulazione.

- **Quanti e dove:** gli spettatori veri della partita (piano B) nei settori dello stadio (decisione AZ); la curva dove sta la curva, i tifosi ospiti nel loro settore.
- **Cosa fanno:** reagiscono agli eventi veri (il gol, l'occasione, il fischio, il cartellino) e all'umore dei tifosi come attori collettivi (decisione BC, la 1.1); nelle partite che contano, le coreografie, decise dai tifosi attori. LUNA: il pubblico mostra solo quello che è successo e lo stato che i tifosi hanno.
- **Striscioni, bandiere, maglie:** generati dai colori del club; e **l'utente può inserire i suoi** (maintainer): immagini e testi in un formato di pacchetto documentato (decisione G), sul suo computer.
- **I cori:** un sonoro generato che segue gli eventi e l'umore (il boato, i fischi, il silenzio); e **la possibilità di inserire i cori originali** (maintainer): file audio dell'utente, sul suo computer, collegati a un club e a un momento (l'ingresso in campo, il gol, la vittoria). Noi non distribuiamo registrazioni né testi di cori: molti sono su melodie protette (domanda per il legale nel documento della consulenza).
- **Il sonoro serve anche all'accessibilità:** con la radiocronaca e i suoni del campo (decisione BE).
- **Nell'elenco ufficiale** solo pacchetti di pubblico con licenze pulite e senza contenuti discriminatori (decisione K).

## Le scelte del 3D (decisione BH, 16 novembre 2026)

Decise con i filtri (partita vera, LUNA, futuribilità), con il maintainer.

1. **Gli omini sono generati,** come stemmi e figurine (decisione F): un modello parametrico dal corpo che si vede (altezza, corporatura; piano B, mai il piano A), colori e numero dalla squadra. Le animazioni vengono dalla tabella evento → animazione (decisione V), sui movimenti veri della registrazione; sono solo disegno. **Il formato è aperto:** modelli e texture dell'utente in glTF, il formato standard, dentro un pacchetto documentato (decisione G).
2. **I volti nel 3D:** teste generate, coerenti con il modello. Le foto dei pacchetti di FM (decisione BG) restano nell'interfaccia (schede, menu, la panchina), perché una foto piatta su una testa 3D sembra una maschera. Proposta, da confermare.
3. **Le telecamere:** la televisiva (alta, di lato), la tattica dall'alto, dietro la porta, quella del giocatore, e la libera che gira intorno al campo; il "segui il pallone" come nel 2.5D.
4. **I replay:** ogni azione si rivede da ogni telecamera, perché la registrazione è un contratto (ADR-065) e la partita è deterministica: il replay è la partita stessa, non un video. Gli highlights si compongono da soli dagli eventi più importanti, con il racconto di `ds-narrative`.
5. **Il computer minimo:** una scheda grafica integrata di fascia media di circa cinque anni fa, a 1080p e 60 fotogrammi al secondo, con i livelli di dettaglio (il pubblico lontano disegnato come sagome, quello vicino in 3D). Il numero si fissa misurando un prototipo, non stimando; la GUI mostra il costo come per i moduli (decisione AX).
6. **Dal 3D si danno le istruzioni:** è lo stesso gioco. I pannelli egui (la lavagna, i post-it, la scheda) girano dentro Bevy con bevy_egui; le istruzioni sono input della partita (ADR-059), non scritture della vista, quindi il determinismo resta intatto.

