# ADR-085 — One repertoire, every gesture distinguished (OI-129)

- **Status:** Accepted — DD-25, maintainer directives of 2026-11-20: «fai un adr unico per TUTTI i gesti distinguendoli chiaramente», «devi aiutarmi a metterli tutti», «usa il filtro luna», «gli esempi che ti faccio ti danno una direzione, sta a te poi espandere l'orizzonte per essere il più completo possibile»
- **Date:** 2026-11-20
- **Decision:** how the **whole** technical repertoire enters the engine: not as new event types with their own rules, but as choices of the same player
- **Builds on:** ADR-051 (every act has its roots), ADR-084 (the possession is the movement), `L0-match/design/ACTIONS.md` (the catalogue, first census)

## Il filtro LUNA

Il maintainer dà esempi — passaggio filtrante, tacco, testata filtrante da palla
alta, controllo aereo che scarta l'uomo, doppio passo — e poi corregge due
volte: «i gesti tecnici non sono solo 5» e «gli esempi che ti faccio ti danno
una direzione, sta a te poi espandere l'orizzonte per essere il più completo
possibile». Gli esempi indicano la strada; il catalogo va esteso fino a coprire
tutto ciò che un calciatore può fare, col pallone e senza, ed è aperto: nuove
combinazioni di dito, contatto, spazio e intento sono gesti nuovi, non nuovi
burattini. «Tutto sempre dipendente alle proprie capacità cognitive
psicologiche fisiche e tecniche.»

**Il dito** è ciò che cambia da un gesto all'altro: il piede, il tallone, la
testa, il ginocchio, il petto, le mani del portiere, il corpo che finta,
l'altezza del pallone, il momento del contatto, la direzione.

**La luna** è ciò che non cambia mai: lo stesso giocatore, dal primo all'ultimo
minuto, con la sua identità e le sue capacità cognitive, psicologiche, fisiche
e tecniche, che legge la posizione in campo, gli avversari e i compagni (o no) e
sceglie. Cambia la stanchezza e cambia il fatto di avere palla: non cambia chi
è. Non esiste un "uomo del tacco" o un "uomo della rovesciata": esiste un
calciatore che in quella scena sceglie quel gesto perché le sue qualità e la
lettura glielo rendono il migliore.

Ogni riga del catalogo therefore ha due colonne gemelle: **dito** (ciò che
cambia) e **luna** (l'identità che resta). Un gesto è ammesso solo se si riesce
a riempire entrambe. Se per farlo funzionare occorre cambiare la luna — una
regola nuova, un ruolo nuovo, un evento che comanda l'atto — non è un gesto: è
un burattino e si rigetta. Aggiungere `EventType::Backheel` con una soglia, o
`EventType::Stepover` che scatta a una distanza, farebbe arrivare il gesto al
giocatore invece di essere qualcosa che lui fa. Ogni gesto è una **scelta** con
quantità lette dalle radici già dichiarate in
[`CHARACTERISTICS-AND-ABILITIES.md`](../../L0-match/design/CHARACTERISTICS-AND-ABILITIES.md):
nessun numero nuovo, nessun dado, nessun bonus, nessun copione.

## Partita vera

Lo stesso calciatore, con la stessa identità, fa cose diverse perché vede cose
diverse. Il passaggio filtrante è il passer che vede lo spazio dietro l'ultimo
difensore e il compagno che lo attacca, e mette la palla dove i due si
incontrano. Il tacco è il calciatore che ha un uomo davanti e un compagno
dietro e sceglie di giocare il pallone col tallone. La testata filtrante è lo
stesso gesto con la testa su un pallone alto. Il controllo aereo e il doppio
passo sono modi di battere l'uomo: il primo porta il pallone giù mentre lo si
supera, il secondo manda il corpo da una parte e il pallone dall'altra. La
rovesciata, la volée, il pallonetto sul portiere, la spazzata di testa, il
tackle in scivolata sono tutti modi diversi di rispondere alla stessa scena.
In ogni caso la luna è la stessa: il giocatore che legge e sceglie con le sue
qualità.

## Come si distingue un gesto dall'altro

Un gesto non si distingue per il nome dell'evento, ma per cinque domande sul
**dito**, sempre con la stessa **luna**:

1. **Chi sceglie:** chi ha la palla, chi la riceve, chi la contende in aria, chi
   difende, chi batte una ripresa. (La luna è quel giocatore, non un ruolo
   nuovo.)
2. **Il dito:** piede (destro/sinistro, interno/esterno, punta, suola), tallone,
   testa (fronte, nuca), ginocchio, petto, coscia, mani del portiere, o solo il
   corpo (finta, spalla, passo).
3. **Il contatto:** palla ferma, in movimento (di prima), in aria, in discesa,
   dopo un rimbalzo, fra due tocchi, sul terreno, al volo, a mezza volée.
4. **La direzione e lo spazio:** avanti, indietro, di lato, nello spazio davanti
   a un compagno, dietro la linea, lontano dalla propria porta, tra le mani,
   sopra l'avversario, attorno al muro.
5. **L'intento:** tenere, passare, filtrare, crossare, tirare, spazzare,
   battere l'uomo, fermare, liberare, smarcarsi.

Il repertorio è aperto perché nuove combinazioni di queste risposte sono gesti
nuovi. Ogni gesto però entra con la stessa regola: una riga, le radici, il
gemello, la visibilità.

## Scegliere, ricombinare, inventare (il confine L0/L2)

Il maintainer chiede se un giocatore con tecnica altissima potrebbe «inventarne
di nuovi». Sì, ma vanno distinti tre livelli, e solo due sono di L0:

1. **Scegliere un gesto raro.** Il calciatore con tecnica e creatività alte vede
   il tacco, la testata filtrante, il doppio passo dove gli altri vedono solo il
   passaggio corto. Non inventa nulla: sceglie un gesto che il repertorio già
   contiene. Il motore lo fa già — `creativity` modula il filtro di visibilità
   delle opzioni rare in `choice.rs` (ADR-052, piano 10b) — e qui si estende a
   tutto il catalogo. **Non è esagerazione**: è il livello che stiamo attuando.
2. **Ricombinare il dito.** Un gesto nuovo come combinazione delle cinque
   domande — per esempio il tacco di prima su un pallone in movimento, o la
   finta col corpo seguita dal controllo aereo — non richiede una capacità
   nuova: usa le stesse radici in una scena diversa. Il catalogo è aperto proprio
   per questo. **Non è esagerazione**: è una riga nuova in una famiglia, con lo
   stesso gemello e nessun numero nuovo.
3. **Inventare un gesto motorio mai visto.** Un movimento che nessuno ha fatto,
   che richiede una capacità nuova o una crescita del latente. Qui la luna
   cambia: non è più lo stesso atleta con le sue qualità, è un atleta che ha
   imparato qualcosa. In L0 i latenti sono stabili (caso degenere) e la crescita
   appartiene a L2 (ADR-060, la carriera che si costruisce). **A L0 è
   esagerazione**, e se lo si attuasse con una regola sarebbe un burattino
   travestito: il gesto arriverebbe al giocatore invece di nascere dalle sue
   qualità. Si lascia a L2, con un riferimento esplicito in ADR-060.

Con LUNA: nei livelli 1 e 2 cambia il **dito**, la **luna** è sempre lo stesso
giocatore che sceglie con le sue capacità. Nel livello 3 cambierebbe la luna
stessa: per questo non è L0.

Il maintainer aggiunge il complemento: «giocatori con minore tecnica riescono a
fare solo alcuni trick». Il catalogo qui sopra è della **popolazione**: nessun
calciatore lo possiede intero. Chi ha tecnica altissima attinge a quasi tutte le
righe; chi ne ha meno ne vede e ne esegue solo alcune. Non è una lista di gesti
assegnata per categoria — sarebbe un copione — ma il filtro delle radici che già
esiste: `creativity` decide quali opzioni rare si vedono, `first_touch`,
`dribbling`, `heading`, `foot` e le altre radici decidono se l'esecuzione è
abbastanza buona da sceglierla. Cambia il **sottoinsieme disponibile**, non
l'identità: la luna resta lo stesso giocatore, con le sue qualità. Il gemello di
ogni gesto deve allora mostrare che due giocatori identici salvo che in una
radice scelgono o eseguono un sottoinsieme diverso.

## Le caratteristiche nascoste sono il giocatore: la specializzazione si acquista in L2 e si esprime in L0

Il maintainer aggiunge: «le caratteristiche nascoste che formano le abilità
visibili sono il giocatore singolo, poi come nella vita reale ognuno sceglie in
cosa *specializzarsi* in base alle proprie abilità per massimizzare il proprio
impatto in partita (almeno questo lavoro dovrebbe essere coadiuvato con i
preparatori e allenatore)», e corregge: «in una certa misura dovrebbe anche
essere L0».

Quattro fatti, già nella dottrina, vanno tenuti distinti:

1. **Le caratteristiche nascoste sono il giocatore.** Sono il Piano A
   (ADR-003), derivate in modo chiave dalla identità e dal seme del mondo
   (ADR-029): non sono un ruolo, né una tabella di categorie. Le abilità visibili
   sono il modo in cui quelle caratteristiche si esprimono nella scena letta.
2. **La specializzazione è una scelta nel tempo, non durante la partita.**
   ADR-060 mette in L2 lo sviluppo dei giocatori — allenamento che cresce le
   caratteristiche, piani individuali, età e declino — e i preparatori come
   attori (assistente, preparatore atletico, analisti, medici); mette in L1
   l'allenatore del giorno della partita con undici, panchina, fasi e istruzioni
   (ADR-059). L'allenatore coadiuva dicendo cosa privilegiare, il preparatore
   lavora sul corpo: nessuno dei due scrive un gesto nuovo dentro il giocatore
   durante la partita.
3. **L'espressione della specializzazione è L0.** In partita i latenti sono
   stabili, ma il profilo che il giocatore si è costruito deve cambiare ciò che
   vede e sceglie nella stessa scena: quali gesti entrano nel suo campo delle
   opzioni, quale gesto preferisce, come massimizza l'impatto con le qualità che
   ha. Questo c'è già in parte: `creativity` modula la visibilità di `Through` e
   `Loft` in `choice.rs`, e le radici comandano le quantità. Dove due giocatori
   con radici diverse scelgono esattamente lo stesso gesto nella stessa scena,
   quella è una mancanza di L0, da correggere una causa alla volta con il
   gemello — non una nuova radice `specialization`, non una tabella di gesti per
   categoria.
4. **Ciò che non è L0** è crescere il latente, cambiare specialità a metà
   partita o assegnare un'etichetta di specialista: cambierebbe la luna e
   sarebbero i preparatori a scrivere il gesto dentro il giocatore.

Con LUNA: le caratteristiche nascoste **sono la luna**; il gesto è il dito. La
specializzazione cambia quali diti sono disponibili nel corso della carriera
(L2) e quali il giocatore sceglie in una data scena (L0): in entrambi i casi
resta lo stesso giocatore. Il gemello di ogni gesto confronta giocatori diversi
per una radice, non per un'etichetta di specialità. Solo la crescita e le
istruzioni richiedono una decisione separata con specifica L2 (sviluppo
individuale) e interfaccia L1 (istruzioni); l'espressione in partita è invece
compito di questo ADR e dei suoi `SIM_VERSION`.

## Il catalogo, famiglia per famiglia

**Stato:** ✓ esiste e le quantità leggono le radici; ◐ esiste ma qualche quantità
è una regola; — manca; 👁 esiste ma chi guarda non lo vede; **da censire** = il
gesto è noto nel calcio ma non è ancora stato verificato nel codice: la prima
azione è dire dove sta, non inventarlo. Le radici sono quelle dichiarate;
nessuna nuova.

### 1. Il corpo (tutti)

La luna è il giocatore che si muove: cambia il modo del movimento, non chi si
muove.

| Gesto | Dito (cambia) | Luna (resta) | Radici | Stato |
|---|---|---|---|---|
| Camminare, correre, sprintare | Le gambe, il ritmo | Il giocatore che si muove | max speed, explosive force, weight; fatigue | ✓ |
| Accelerare, ripartire | La spinta iniziale | Il giocatore che si muove | explosive force, acceleration | ✓ |
| Frenare, fermarsi | Il corpo che arresta | Il giocatore che si muove | body control | ✓ |
| Girarsi, cambiare direzione, pivot | Il corpo che cambia asse | Il giocatore che si muove | joint mobility, body control, weight | ✓ |
| Saltare, atterrare | Le gambe che spingono e assorbono | Il giocatore che si muove | explosive force, height, joint mobility | ◐ |
| Tuffarsi, cadere, rialzarsi | Il corpo che si getta | Il giocatore che si muove | body control, courage | da censire (salto del portiere ✓) |
| Reggere i 90 minuti | Tutto, che si consuma | Il giocatore che si muove | endurance | ✓ |

### 2. Gli occhi e la mente

La luna è il giocatore che legge: cambia ciò che nota e il tempo che prende,
non la sua identità.

| Gesto | Dito (cambia) | Luna (resta) | Radici | Stato |
|---|---|---|---|---|
| Guardarsi intorno, scandagliare | Gli occhi, il cono | Il giocatore che legge | vision, game reading | ✓ |
| Leggere il gioco, anticipare | Ciò che si nota | Il giocatore che legge | game reading | ✓ |
| Decidere | Il tempo che si prende | Il giocatore che legge | decision making, concentration, crowding, fatigue | ✓ |
| Gestire il rischio | Quanto si è disposti a perdere | Il giocatore che legge | risk_management | ✓ |
| Concentrarsi, distrarsi | Quanto si mantiene l'attenzione | Il giocatore che legge | concentration | ◐ |
| Comunicare (gridare, gestire) | La voce, le mani | Il giocatore che legge | — | da censire |

### 3. Ricezione e primo controllo

La luna è chi riceve: cambia la parte del corpo e il modo in cui il pallone
viene domato, non il giocatore.

| Gesto | Dito (cambia) | Luna (resta) | Radici | Stato |
|---|---|---|---|---|
| Controllo col piede (interno, esterno, suola) | Il piede che ammortizza | Chi riceve | first touch, foot, body control | ✓ (il piede come dito manca) |
| Controllo di petto | Il petto | Chi riceve | first touch, body control | da censire |
| Controllo di coscia o ginocchio | La coscia, il ginocchio | Chi riceve | first touch, body control | da censire |
| Controllo di testa | La fronte, la nuca | Chi riceve | first touch, heading, body control | da censire |
| Controllo di prima (di prima intenzione) | Il contatto senza fermare | Chi riceve | first touch, game reading | ◐ (tiro di prima provato e tolto) |
| Controllo in corsa, smorzando | Il passo che si allunga | Chi riceve | first touch, dribbling, acceleration | ◐ (OI-123) |
| Controllo in aria, portare giù | Il pallone alto che scende sotto controllo | Chi riceve | first touch, body control, heading | — |
| Ricezione girandosi | Il corpo che ruota | Chi riceve | first touch, body control, joint mobility | da censire |
| Ricezione a terra, schiacciando | Il piede che blocca | Chi riceve | first touch | da censire |
| Spazzolata via, deviazione | Il piede che lascia passare | Chi riceve | first touch, passing | da censire |

Non è un gesto nuovo ma la logica di un atto già censito: il **controllo sotto
pressione** — l'avversario che aspetta e la palla fra due tocchi — è corretto in
[ADR-086](086-control-under-pressure.md) (SIM 170), con la stessa lettura
deterministica del contrasto. Nessuna riga nuova, nessun evento nuovo.

### 4. Passaggio e costruzione

La luna è il portatore: cambia il piede, l'altezza, la direzione, non chi
gioca.

| Gesto | Dito (cambia) | Luna (resta) | Radici | Stato |
|---|---|---|---|---|
| Passaggio corto | Il piede, la direzione | Il portatore | passing, foot, vision, game reading, risk | ✓ |
| Passaggio lungo, cambio di gioco | Il piede, l'altezza | Il portatore | passing, foot, explosive force, game reading | ✓ |
| **Passaggio filtrante nello spazio** | Il piede, lo spazio davanti al compagno | Il portatore | game reading, vision, passing, decision making, risk | ✓ dal SIM 168: scelto dal SIM 79, ora narrato e disegnato |
| **Colpo di tacco** | Il tallone, palla indietro/di lato | Il portatore | passing, first touch, body control, game reading | — |
| Passaggio di esterno, curva | La parte esterna del piede | Il portatore | passing, foot, body control | da censire |
| Passaggio schiacciato, rasoterra | Il piede che tiene il pallone a terra | Il portatore | passing, foot | da censire |
| Pallonetto, chip | Il piede che solleva | Il portatore | passing, foot, explosive force | ✓ (come lob) |
| Passaggio a muro, uno-due | Il piede, il compagno che restituisce | Il portatore | passing, game reading, decision making | da censire |
| Passaggio a terzo uomo | Il piede, la terza corsa | Il portatore | passing, game reading, vision | da censire |
| Cross basso, alto, cut-back | Il piede, l'area | Il portatore | passing, foot | ✓ |
| **Testata filtrante da palla alta** | La testa, palla nello spazio per un compagno | Chi contende il pallone alto | heading, first touch, game reading, vision | — |
| Passaggio di petto, di spalla | Il petto, la spalla | Il portatore | first touch, passing, body control | da censire |
|Distribuzione del portiere (mani, piedi, rotolata) | Le mani o i piedi | Il portiere | passing, foot, vision, game reading | ✓ (mani e piedi; la rotolata da censire) |

### 5. Dribbling e battere l'uomo

La luna è il portatore: cambia il gesto con cui supera l'avversario, non chi
è.

| Gesto | Dito (cambia) | Luna (resta) | Radici | Stato |
|---|---|---|---|---|
| Conduzione a testa alta | I piedi, il pallone avanti | Il portatore | dribbling, run, vision | ✓ |
| Take-on, scarto in corsa | Il corpo che passa l'uomo | Il portatore | dribbling, explosive force, body control | ✓ come esito di duello |
| **Doppio passo (finta, scissor)** | Il corpo va da una parte, la palla dall'altra | Il portatore | dribbling, body control, explosive force | — |
| Finta di spalla, di passo | La spalla, il passo che inganna | Il portatore | body control, dribbling, game reading | da censire |
| Roulette, girata su sé stesso | Il corpo che ruota sul pallone | Il portatore | dribbling, body control, joint mobility | da censire |
| Cruyff turn, drag-back | Il pallone che torna indietro | Il portatore | dribbling, first touch, body control | da censire |
| Nutmeg, pallone tra le gambe | Il pallone che passa tra le gambe | Il portatore | dribbling, first touch, vision | da censire |
| **Controllo aereo + scarto** | Il pallone alto portato giù e superato | Chi contende il pallone alto | first touch, body control, dribbling, heading | — |
| Cambiamento di ritmo | Il passo che si accelera dopo la finta | Il portatore | dribbling, explosive force | da censire |
| Proteggere il pallone | Il corpo fra palla e avversario | Il portatore | maximal force, body control, first touch | ✓ |
| Girarsi con il pallone | Il corpo che ruota e il pallone resta | Il portatore | dribbling, body control, joint mobility | da censire |

### 6. Tiro e finalizzazione

La luna è il tiratore: cambia il piede, il contatto, l'altezza, non chi tira.

| Gesto | Dito (cambia) | Luna (resta) | Radici | Stato |
|---|---|---|---|---|
| Tiro di piede, piazzato o potente | Il piede, la forza | Il tiratore | finishing, foot, explosive force, composure | ✓ |
| **Tiro di prima** | Il contatto senza controllo | Il tiratore | first touch, finishing, game reading | ◐ (provato e tolto, OI-100) |
| Tiro con il piede debole | L'altro piede | Il tiratore | finishing, foot | ✓ |
| Volée | Il piede che incontra il pallone in aria | Il tiratore | finishing, first touch, body control | da censire |
| Mezza volée | Il piede che incontra il pallone dopo il rimbalzo | Il tiratore | finishing, first touch, body control | da censire |
| Testata in porta | La fronte | Il tiratore | heading, → jump, game reading | ✓ |
| Testata tuffata | La fronte, il corpo in volo | Il tiratore | heading, → jump, body control, courage | da censire |
| Rovesciata, overhead | Il piede sopra la testa | Il tiratore | finishing, body control, explosive force, courage | da censire |
| Pallonetto sul portiere | Il piede che solleva oltre l'uomo | Il tiratore | finishing, passing, game reading | ✓ (come lob) |
| Tiro curvo attorno al muro | La parte esterna del piede | Il tiratore | finishing, foot, body control | da censire |
| Seguito di respinta | Il piede sulla seconda palla | Il tiratore | finishing, reactivity, game reading | da censire |
| Rigore | Il piede dal punto | Il battitore | finishing, set pieces, composure | ✓ |
| Punizione diretta, indiretta | Il piede, il muro | Il battitore | finishing, set pieces, foot | ✓ |

### 7. Difendere, liberare, fermare

La luna è il difensore o il portiere: cambia il modo di fermare, non chi è.

| Gesto | Dito (cambia) | Luna (resta) | Radici | Stato |
|---|---|---|---|---|
| Spazzata di piede, di testa | Il piede o la testa, via dalla porta | Il difensore | passing, explosive force, heading | ✓ |
| Salvataggio sulla linea | Il corpo sulla linea | Il difensore | reactivity, body control, courage | da censire |
| Intercetto | Il corpo sulla linea del passaggio | Il difensore | game reading, reactivity, sprint | ✓ |
| Blocco di tiro o passaggio | Il corpo che si para davanti | Il difensore | reactivity, first touch, courage | ✓ |
| Contrasto in piedi, jockey | Il piede che aspetta | Il difensore | tackling, body control, discipline | ✓ |
| Contrasto in scivolata | Il corpo che scivola | Il difensore | tackling, explosive force, body control | ✓ |
| Presa, respinta, spill del portiere | Le mani che trattengono o lasciano | Il portiere | handling, body control | ✓ tranne lo spill |
| Tuffo, parata | Le mani e il corpo in volo | Il portiere | reactivity, explosive force, height | ✓ |
| Pugno | Il pugno che allontana | Il portiere | handling, explosive force | da censire |
| Posizione, preparazione | Il corpo, l'angolo, i piedi a terra | Il portiere | game reading, concentration, reactivity | ◐ / — |
| Uscita, uscita su cross | Le gambe, le mani in area | Il portiere | decision making, courage, → jump | ◐ |

### 8. Gioco aereo

La luna è chi contende: cambia il punto e il modo dell'impatto, non il
giocatore.

| Gesto | Dito (cambia) | Luna (resta) | Radici | Stato |
|---|---|---|---|---|
| Duello aereo, salto in timing | La testa e il salto | Chi contende | heading, → jump, maximal force, height | ◐ (timing mancante) |
| Testata di contesa, in attacco e in difesa | La fronte | Chi contende | heading, → jump, game reading | ✓ |
| Flick di testa, deviazione alta | La nuca, la fronte che devia | Chi contende | heading, first touch, game reading | da censire |
| Contesa su rimessa, angolo, punizione | Il corpo nell'area | Chi contende | heading, → jump, game reading | da censire |

### 9. Senza palla (la lettura che muove)

La luna è il giocatore senza palla: cambia dove va e perché, non chi è.

| Gesto | Dito (cambia) | Luna (resta) | Radici | Stato |
|---|---|---|---|---|
| Tenere il posto | Il corpo che resta | Il giocatore senza palla | game reading, discipline | ✓ |
| Pressare | Il corpo che va addosso | Il giocatore senza palla | game reading, aggressiveness, work rate | ◐ |
| Coprire, rientrare, stringere | Il corpo che torna e si accorcia | Il giocatore senza palla | game reading, work rate, run | ✓ |
| Marcare, seguire l'uomo | Il corpo tra uomo e porta | Il giocatore senza palla | game reading, concentration, discipline | ◐ |
| Offrirsi per il passaggio | Il corpo che si smarca | Il giocatore senza palla | game reading, decision making | ✓ |
| Attaccare la profondità, smarcarsi | Le gambe oltre l'ultimo difensore | Il giocatore senza palla | game reading, sprint | ✓ |
| Rientrare sul piede forte, check back | Il corpo che torna verso il pallone | Il giocatore senza palla | game reading, decision making | da censire |
| Triangolare, terzo uomo in corsa | La corsa che apre la linea | Il giocatore senza palla | game reading, vision, sprint | da censire |
| Fuorigioco, tenere la linea | Il corpo sulla linea | Il giocatore senza palla | game reading, discipline | ✓ |

### 10. Le riprese

La luna è il battitore: cambia il piede e il punto, non il giocatore.

| Gesto | Dito (cambia) | Luna (resta) | Radici | Stato |
|---|---|---|---|---|
| Calcio d'inizio, rimessa, rinvio, angolo | Il piede, chi batte | Il battitore | role, passing, set pieces | ✓ |
| Rimessa lunga, corta | Il piede, la distanza | Il battitore | passing, set pieces | da censire |
| Angolo interno, esterno | Il piede, la curva | Il battitore | passing, foot, set pieces | da censire |
| Punizione, rigore | Il piede, tiro o passaggio | Il battitore | finishing, set pieces, foot | ✓ |
| Schemi di calcio piazzato | La corsa combinata | Il battitore e i compagni | game reading, passing, vision | da censire (L1) |

## Decisione

1. **Un solo repertorio, un solo ADR.** Tutti i gesti, non solo cinque: le
   famiglie qui sopra sono il catalogo vivo, e l'elenco è aperto. Ogni gesto
   nuovo entra con una riga in una famiglia, non con un ADR nuovo.
2. **La prova LUNA per ogni gesto.** Un gesto si accetta solo se si riesce a
   scrivere la riga dicendo insieme il dito che cambia e la luna che resta. Se
   per farlo funzionare occorre cambiare la luna — regola nuova, ruolo nuovo,
   evento che comanda l'atto — il gesto è rigettato come burattino.
3. **Censire prima, inventare dopo.** Per ogni riga `da censire`, 👁 o ◐ la
   prima azione è dire dov'è nel codice e perché non si vede o non è completa;
   solo dopo si scrive il gesto. Nessuna quantità nuova senza una radice che
   la chieda.
4. **Nessun evento speciale con una regola propria.** Dove il gesto cambia solo
   il modo del contatto, entra come **qualificatore** dell'atto che già esiste
   (come `cross` e `through` oggi). Solo quando cambia la traiettoria o la
   catena di decisioni serve una scelta nuova (`ChoiceKind`), presa nel punto
   in cui il giocatore sceglie già, non in un evento a parte.
5. **Ogni gesto nasce dalla scena letta.** La finestra di scelta è ciò che il
   giocatore percepisce in quella posizione: uomo davanti, spazio dietro,
   pallone alto, compagno in corsa. La zona del campo è un fattore della scena,
   non una tabella di comportamenti (ADR-084, punto 5).
6. **Tutte le quantità vengono dalle radici già dichiarate.** Nessun numero di
   grado 3, nessun peso nuovo a mano, nessun dado: la frequenza di un gesto
   cambia cambiando le qualità del giocatore, non una soglia di codice.
7. **Ogni gesto ha il suo gemello.** Due giocatori identici tranne che per una
   radice devono sceglierlo o eseguirlo diversamente; se cambia solo l'atto
   senza cambiare lettura o abilità, il gesto è rigettato come burattino.
8. **Ogni gesto deve essere visibile** in ciò che l'osservatore vede: evento
   qualificato e movimento corrispondente nel tick record, così la 2.5D lo
   mostra senza essere aggiornata (legge il tick record prodotto dal motore).
9. **Una causa alla volta:** un `SIM_VERSION` +1, golden, test del repertorio,
   gemelli, `fmt`, `clippy`, `cargo test --workspace`, gate `closing`, misura
   42..101 e giudizio visivo del maintainer. I semi non si riducono e le
   asserzioni non si indeboliscono. Nessun merge in `main` senza autorizzazione.
10. **Il confine L0/L2:** si attuano la scelta del gesto raro e la ricombinazione
    del dito; l'invenzione di un gesto motorio mai visto resta a L2 (ADR-060),
    perché richiederebbe crescita dei latenti e cambierebbe la luna.
11. **Il repertorio è individuale.** Il catalogo della popolazione non è
   posseduto da tutti: il filtro delle radici (`creativity` per la visione delle
   opzioni rare, le radici tecniche per l'esecuzione) determina il sottoinsieme
   di gesti che ciascuno vede e sa fare. Nessuna tabella di gesti per categoria.
12. **Le caratteristiche nascoste sono la luna; la specializzazione si acquista
    in L2 e si esprime in L0.** Il giocatore è le sue caratteristiche (Piano A,
    derivazione chiave). Preparatori e allenatore coadiuvano nello sviluppo
    (ADR-060) e nelle istruzioni (ADR-059); in partita il profilo risultante deve
    cambiare opzioni e scelte nella scena, con le radici già dichiarate e il
    gemello come prova. Nessuna nuova radice `specialization`.

## Ordine di lavoro

Si completa una famiglia prima di passare alla successiva; dentro una famiglia,
prima censire, poi rendere visibile, poi aggiungere. L'ordine proposto:

1. **Censimento — fatto:** per ogni riga `da censire`, 👁 e ◐, dire dove sta nel
   codice e quale radice dovrebbe comandarla, senza codice nuovo. Il risultato è
   [`context/plans/L0-REPERTOIRE-CENSUS.md`](../../context/plans/L0-REPERTOIRE-CENSUS.md):
   due righe importanti emerse — `set pieces` non è più una radice dichiarata e
   `comunicare` non ha radice — vanno decise prima di scrivere quei gesti.
2. **Col pallone — il filtrante — fatto in SIM 168:** il qualificatore
   `through` è letto dal narratore («threads it through», importanza `Medium`,
   quindi visibile senza «every touch») e dalla 2.5D (gesto dedicato). La
   scelta e lo stato della partita non cambiano, i golden restano identici;
   misura su 42..101 e giudizio visivo del maintainer da fare.
3. **Col pallone — il tacco:** passo qualificato, uomo davanti e compagno dietro.
4. **Col pallone — la testata filtrante:** lo stesso passaggio su pallone alto.
5. **Col pallone — il controllo aereo:** il pallone alto che diventa scarto.
6. **Col pallone — il doppio passo:** la finta col corpo, senza toccare il pallone.
7. **Il resto del catalogo:** famiglia per famiglia, nell'ordine 3, 5, 6, 4, 7,
   8, 1, 2, 9, 10, una riga alla volta con la stessa regola e lo stesso gemello.

Ogni passo ha la sua voce in `CHANGELOG-SIM` e il suo `SIM_VERSION`; l'ADR resta
uno solo e ogni passo ne richiama la famiglia e la riga.

## Conseguenze

ADR-051 e `ACTIONS.md` conservano il catalogo: questo ADR dice **come** i gesti
del catalogo entrano nel motore e lo estende a tutto il repertorio, non lo
sostituisce. ADR-084 resta l'invariante dell'identità del giocatore; questo ADR
ne è l'applicazione al repertorio tecnico. OI-128 (il possesso come movimento) e
OI-129 (il repertorio) restano aperti finché il maintainer non dà il giudizio
visivo sulle partite misurate.
