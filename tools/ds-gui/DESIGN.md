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

## Android (ADR-061)

- **Verticale:** la proposta A, la lavagna con un cassetto che contiene i post-it.
- **Orizzontale:** la schermata partita della proposta C.
- Si parte da qui per popolare la GUI di contenuti; poi si migliora con i betatester.
- Pacchetti grafici leggeri per il telefono (loghi e foto a bassa risoluzione), stessi plugin del PC, distribuzione fuori da Google Play finché la decisione 4 di `context/plans/DECISIONS-2026-11-15.md` non apre un'altra strada.
