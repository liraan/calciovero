# L1 · Studio dell'allenatore (proposta per il maintainer)

> 2026-11-24, Claude Code. **Deciso:** il maintainer ha accettato le otto raccomandazioni del §8 (decisione BM, 2026-11-24); gli anelli seguono il §7. Nasce da una richiesta del maintainer: «dobbiamo studiarlo per bene il coach, perché è anche allenamento, crescita dei giocatori, gestione psicologica, crescita cognitiva e fisica… LUNA e partita vera». Mette insieme quello che è già deciso (`L1-COACH.md`, `L1.md` decisione AB, `L2.md` AS/AU/BB/BC, ADR-059, ADR-114) e ci aggiunge le fonti esterne. Le fonti sono state lette quasi tutte come abstract o sintesi: quelle segnate *(da verificare)* vanno controllate sul testo prima di diventare un numero in `balance/` (grado 1).

## 1. LUNA: la proprietà, non l'elenco

**LUNA:** l'allenatore non muove i giocatori. Muove quello che loro sanno, quanto sono freschi e come stanno, e lo fa attraverso quello che lui stesso vede, sempre in modo imperfetto. Ogni suo effetto passa per uno stato che si può leggere (la condizione, la comprensione del piano, la fiducia), mai per un bonus. Le caratteristiche cambiano solo per cause (L2, la crescita), e l'allenatore è una di quelle cause, non l'unica.

**Partita vera:**
- *Come lo riconoscono gli attori?* Dalla squadra che arriva fresca o stanca alla partita, che sa o non sa cosa fare sulle palle inattive, che reagisce o crolla dopo un gol subito, dal cambio fatto al momento giusto.
- *Da cosa dipende?* Da quello che l'allenatore osserva (poco e male in partita: Franks & Miller 1986 dà il 30-45% degli eventi chiave ricordati, una replica il 59%) e dalle sue radici (l'idea di gioco, l'adattabilità, lo stile con le persone).
- *Chi lo produce?* L'allenatore sceglie, i giocatori eseguono con le loro radici, il corpo e la testa filtrano.

## 2. Le sei aree, una per una

Per ogni area: cosa fa un allenatore vero, cosa vede, cosa decide, cosa cambia (uno stato o una caratteristica), il livello, la versione semplice.

### 2.1 L'allenamento e il carico (L1, decisione AB)
- **Il vero:** la settimana fra due partite ha il carico che scende in modo monotono verso la partita. Martín-García et al. 2018 (*JSCR* 32(12)): MD-4 > MD-3 > MD-2 > MD-1; MD-3 è il giorno più vicino alla partita per distanza (57%) ma non per alta velocità (37%) né per sprint (29%); la seduta compensativa del giorno dopo, per chi ha giocato poco, arriva all'80-86%. La "periodizzazione tattica" (Delgado-Bordonau & Mendez-Villanueva 2012) è una convenzione dei tecnici, non un risultato: le fonti non concordano su quale giorno faccia cosa.
- **Il carico e gli infortuni:** il rapporto acuto/cronico (Gabbett 2016) è contestato (Impellizzeri et al. 2020: correlazioni spurie, soglie arbitrarie). **Proposta:** confermare la decisione AB di non usarlo, e far nascere l'infortunio dal carico del giorno contro la capacità del giorno (l'ADR degli infortuni). Dupont et al. 2010 (*AJSM* 38(9)): con due partite in una settimana la prestazione non cala, ma gli infortuni salgono da 4,1 a 25,6 ogni 1000 ore; Carling et al. 2015: il rischio è circa triplo negli ultimi 15 minuti della seconda partita ravvicinata. È la causa vera della rotazione.
- **Cosa vede:** la RPE di sessione (Foster), il benessere dichiarato, e il GPS solo se il club ce l'ha (una struttura, L2 AZ).
- **Cosa decide:** la settimana tipo, chi fa il compensativo, il carico individuale di chi rientra.
- **Cosa cambia:** gli stati (la condizione, la freschezza, il ritmo partita); mai le caratteristiche in L1 (ADR-060).
- **Semplice (Android):** la settimana tipo scelta fra tre (leggera, normale, intensa) e il vice che fa il resto.

### 2.2 La crescita fisica, tecnica e cognitiva (L2, 0.3, decisione AS)
- **Fisica:** l'allenamento aerobico intenso, anche a campo ridotto, alza il VO2max del 5-11% e lo Yo-Yo del 13% circa (Iaia, Rampinini, Bangsbo 2014), in blocchi di 8-12 settimane *(da verificare)*. Il disallenamento è più rapido: −10% di VO2max in 8 settimane di stop *(da verificare)*; la resistenza si perde prima della forza. Il picco è fra 25 e 27 anni (Dendir 2016): gli attaccanti verso i 25, i difensori verso i 27. Dopo i 30 calano corsa e sprint, ma la precisione nel passaggio cresce.
- **Tecnica:** la pratica variata aiuta la ritenzione in laboratorio (SMD 0,63), poco sul campo (0,23) e nulla nei giovani (0,02) (*Scientific Reports* 2024, 54 studi; Brady 2004). Ford et al. 2009: chi arriva in alto ha fatto più ore di calcio fra i 6 e i 12 anni (studio correlazionale).
- **Cognitiva:** lo scanning è associato a più passaggi riusciti (Jordet et al. 2013), ma non separa i fuoriclasse dai buoni (Caso et al. 2024). L'allenamento percettivo-cognitivo trasferisce in partita circa la metà dell'effetto di laboratorio (Zhu et al. 2024: ES 1,51 → 0,65). Il "brain training" generico non trasferisce (Vater et al. 2021). Le funzioni esecutive predicono il rendimento (Vestberg et al. 2012, campione piccolo), ma allenarle non è dimostrato.
- **Proposta, per famiglia:**
  - le radici fisiche crescono e calano con il carico e l'età (le fonti più solide);
  - le tecniche crescono con le ore di pratica specifica, poco con la varietà;
  - le cognitive di lettura del gioco crescono con le esercitazioni che somigliano al gioco (circa metà dell'effetto in partita); le funzioni di base sono una radice quasi fissa;
  - le psicologiche cambiano per esperienze (sotto).
  
  Nessun "potenziale": si cresce per cause, a ritmi diversi per famiglia (decisione AA).
- **L'allenatore ci entra così:** il contenuto delle sedute (che cosa si allena), il minutaggio (le partite sono pratica) e lo stile con le persone (la motivazione). Fuori dal suo controllo: l'età, la genetica, gli infortuni da contatto, la vita fuori dal campo.

### 2.3 La gestione psicologica (L1 in parte, L2 0.4 lo spogliatoio)
- **Il modello:** l'allenatore che sostiene l'autonomia (dà scelte, spiega, riconosce) nutre l'autonomia, la competenza e la relazione, e con loro la motivazione che viene da dentro; lo stile controllante la riduce (Mageau & Vallerand 2003, sulla teoria di Deci e Ryan). È una teoria, non una misura; la correlazione misurata è moderata (ρ ≈ 0,39, Mossman et al. 2022, studi trasversali).
- **La coesione:** è legata alla prestazione con un effetto da moderato a grande (Carron et al. 2002, 46 studi), ma in tutti e due i sensi: anche vincere unisce.
- **Il feedback:** in partita è per lo più prescrittivo e più negativo che positivo (Australian football, 2020); l'orgoglio aiuta, la vergogna no, tanto più se il rapporto è stretto (2021); allenatore e giocatori non concordano su cosa sia stato detto (Mason et al. 2020).
- **Proposta:** una radice dell'allenatore, lo **stile con le persone** (dal sostegno all'autonomia al controllo). Agisce su stati dei giocatori che esistono già nei piani: la fiducia nell'allenatore (la credibilità, AM) e la motivazione. Agisce sempre attraverso eventi: un discorso, una promessa mantenuta o tradita, una panchina. Mai come aura. La fiducia in sé stessi non ha misure trovate: resta una lacuna dichiarata.

### 2.4 La preparazione della partita (L1, AF, AL, AO)
- **Il vero:** il 68% degli allenatori d'élite usa il video dell'avversario (Wright, Atkins & Jones 2012). Nessuno studio misura l'effetto del piano gara sul risultato. Le palle inattive valgono il 30-40% dei gol nelle rassegne (Armatas 2007; Altarriba-Bartes 2019), il 17-18% nelle coppe UEFA 2022/23 (Ivanov & Baharov 2025): valgono la preparazione.
- **Proposta:** il piano gara si costruisce sulla stima del gioco avversario dalle sue partite (piano B → C, AF); quello che passa ai giocatori è la comprensione del piano, uno stato che cresce con la riunione e le prove. I giocatori trattengono circa un terzo delle istruzioni, meno se sono tante (Barça Innovation Hub): **più consegne, meno comprensione di ognuna.** È il costo naturale di un piano complicato, senza bisogno di un peso.

### 2.5 La partita vera (ADR-059, AG, AP; L0 e L1)
- **I cambi:** il punteggio è la causa principale (Del Corral et al. 2008: chi perde cambia prima; i cambi difensivi arrivano più tardi). Per chi perde, il primo cambio fra il 56' e il 70' è associato al pareggio (Gómez, Lago-Peñas & Owen 2016). La regola di Myers 2012 (58', 73', 79') ha limiti di metodo (Silva & Swartz 2016). Chi entra corre circa il 15% in più ad alta intensità (Bradley et al.), e il corpo lo dà già: ha le gambe fresche.
- **Proposta:**
  - l'allenatore AI cambia per cause che vede: il punteggio, la stanchezza stimata dei suoi (dal corpo, con il rumore della sua osservazione), un giallo a rischio;
  - il momento viene dalle sue radici (la lettura, il rischio), e il tempo dei cambi veri resta la verifica (grado 2, T/Y), non la regola;
  - quello che ricorda della partita è parziale: la sua stima della partita ha la qualità della sua lettura del gioco.
- **L'intervallo:** le fonti sono poche e di squadre giovanili: prima lo staff, poi poche istruzioni (Raya-Castellano et al. 2023). Va nel modello dei discorsi (AM): più è lungo, meno passa.

### 2.6 La settimana come flusso di lavoro (L1 `ds-week`, la direzione del maintainer sul genere)
La base del genere è questa: la posta al centro, l'allenamento, gli osservatori, la preparazione, la partita, le conferenze, il mercato. Le critiche ricorrenti sono opinioni di testate e giocatori, non misure:
- la posta che sommerge ("300 email" dopo un salto di tempo);
- le conferenze ripetitive;
- la delega al vice poco affidabile;
- la partita come scatola nera;
- la curva d'ingresso ripida;
- la vacanza come unico modo per saltare i tempi morti.

**Proposta, dieci principi per il nostro flusso** (proposte, non misure):
1. **Un'agenda, non una posta:** poche decisioni aperte a settimana, raggruppate, con la loro scadenza; il resto in un riepilogo da leggere quando si vuole.
2. **Il vice propone, tu decidi:** ogni compito delegato arriva come proposta con il perché, il costo e il rischio stimati. Il vice ha uno stile suo e dichiarato, e i suoi giudizi si possono verificare dopo. È la proposta del vice di `L1-COACH.md`, già nel codice del passo 5.
3. **Stime, non numeri:** intervalli che si stringono con l'osservazione, osservatori che possono non essere d'accordo. È il piano C.
4. **Poche decisioni, pesanti:** meno routine settimanale, più svolte con conseguenze visibili e durature.
5. **La conferenza solo se c'è una notizia:** domande nate dagli eventi veri della stagione (la 0.2).
6. **La partita si spiega:** dopo ogni partita il perché in poche righe, dagli stessi eventi del motore (la cronaca solo nelle viste).
7. **Moduli spegnibili:** l'allenamento, il vivaio, la stampa, lo staff affidati all'automatico, con un risultato onesto e nessuna punizione nascosta.
8. **Il tempo lo fermi tu:** scegli che cosa ferma il calendario; saltare una settimana dà un sommario.
9. **La versione semplice (Android):** una scelta di piano e poche decisioni a settimana, sullo stesso motore.
10. **Si impara per gradi:** i sistemi si aprono quando servono, e le prime volte il vice spiega.

## 3. I due tipi di allenatore (la domanda del maintainer)

**Proposta:** un modello solo, due posti.
- **Il posto umano:** sceglie lui. Il vice gli propone, e nei moduli spenti decide il vice. Fatto al passo 5, secondo anello.
- **Gli allenatori AI:** sono gli stessi attori, e sono una sfida perché hanno radici diverse, non perché barano. Le radici:
  - l'**idea** (blocco, ampiezza; già nel codice);
  - l'**adattabilità** (quanto cambia contro chi è più forte; nel codice dal passo 5);
  - la **lettura del gioco** (quanto bene stima la partita e l'avversario);
  - lo **stile con le persone** (§2.3);
  - l'**inclinazione alla rotazione** (quanto rischia gli stanchi);
  - l'**attenzione ai giovani** contro il risultato subito;
  - il **rischio** (quando e come cambia).

  Un allenatore forte si riconosce da quello che fa la sua squadra, non da un numero.
- **Nel mondo:** `coaches.json` nel pacchetto (`L1-season/world/v1/`) dà il nome, l'età, la nazionalità e l'idea dichiarata (la stampa ne parla); le radici vere stanno nel caveau (piano A), come per i giocatori.

## 4. Le contraddizioni trovate fra i documenti
1. **I tipi di seduta:** la decisione AB elenca forza, resistenza, velocità, partitella; `ds-week` elenca la riunione tattica e la prova dei segnali. **Proposta:** una lista sola in un file di dati, `balance/sessions.json`, ogni tipo con quello che allena e il suo carico.
2. **ADR-114 contro `L1-COACH.md`:**
   - lo spareggio: l'ADR dice per identità, il piano per "chi ha giocato meno di recente";
   - la panchina: l'ADR dice cinque, uno per reparto, il piano dice quanti ne permettono le regole;
   - il blocco: l'ADR lo dà in tre scatti, la lavagna libera (AL) lo vuole continuo;
   - le radici: l'ADR le dà uniformi, la decisione AE vuole distribuzioni con le code.
   
   **Proposta:** l'ADR è il primo anello e va emendato verso il piano (lo stato è ancora "delegated").
3. **Le famiglie dell'allenatore:** quattro in AH, cinque con la comunicazione in AM; la tabella di AH non è aggiornata. **Proposta:** cinque.
4. **Chi riferisce in L1 senza staff:** ADR-059 §7 vuole lo staff, ma lo staff arriva nella 0.3. **Proposta:** in L1 riferisce l'allenatore stesso, con la qualità della sua lettura.
5. **Le conferenze nella 0.2, la delega all'addetto stampa nella 1.1:** **Proposta:** nella 0.2 delega il vice.
6. **Capitano e leader in L1 (AO), il gruppo che li riconosce nella 0.4 (BB):** **Proposta:** in L1 il capitano lo nomina l'allenatore, e il gruppo arriva dopo.

## 5. I buchi: cose che un allenatore vero fa e nessun documento copre
- La programmazione della stagione: il precampionato, i blocchi, lo scarico prima delle partite importanti.
- I convocati e il ritiro la sera prima.
- Le pause per le nazionali: chi parte e chi resta.
- Il dopo partita: il video, il colloquio individuale.
- Il medico in L1, quando lo staff non c'è ancora.
- Il sonno e l'alimentazione (proposta: fuori, o solo come stato della freschezza).
- I giovani della Primavera aggregati alla prima squadra.
- La scelta dei portieri: il titolare fisso, la coppa.

## 6. Cosa resta in L1 e cosa va in L2
| Area | L1 (0.1-0.2) | L2 |
|---|---|---|
| Allenamento | la settimana, le sedute, il carico, la rotazione, gli infortuni (ADR) | le strutture, il GPS, lo staff atletico (0.3) |
| Crescita | — (solo gli stati) | le radici per cause e famiglie (0.3) |
| Psicologia | lo stile, la fiducia nell'allenatore, i discorsi, le promesse | lo spogliatoio, le coppie, il capitano del gruppo (0.4) |
| Preparazione | il piano sull'avversario stimato, le palle inattive, i battitori | l'analista (0.3) |
| Partita | i cambi dell'AI per cause, la lavagna in partita, l'intervallo | — |
| Flusso | l'agenda, il vice che propone, il tempo che fermi tu, la conferenza (0.2) | la stampa, i tifosi (1.1) |

## 7. L'ordine degli anelli di L1 (deciso, BM; ✅ 1, 2 e 3 il 2026-11-24)
1. `coaches.json` e le radici dell'allenatore (§3), con il vault che le tiene; l'emendamento di ADR-114 (§4.2).
2. La rotazione: l'allenatore legge la freschezza (stimata) che `ds-body` porta, e lo spareggio diventa "chi ha giocato meno".
3. `ds-week` minimo: la settimana tipo con i tipi di seduta in `balance/sessions.json`, il carico, il compensativo.
4. L'ADR degli infortuni: il carico del giorno contro la capacità del giorno, senza dadi e senza rapporto acuto/cronico.
5. I cambi dell'AI per cause (§2.5), sul motore L0 che li sa già fare (ADR-059).
6. Le palle inattive e i battitori; il piano sull'avversario stimato.
7. L'agenda e il vice che propone, come flusso della settimana (§2.6), con la sua prima vista nella GUI.

## 8. Domande per il maintainer, con la mia raccomandazione
1. **Un modello, due posti** (§3)? *Sì.*
2. **Le radici dell'allenatore** (§3): idea, adattabilità, lettura, stile con le persone, rotazione, giovani, rischio. Mancano o avanzano radici? *Partire da queste sette.*
3. **L'ACWR fuori, l'infortunio dal carico del giorno contro la capacità** (§2.1)? *Sì, come dice già AB.*
4. **Una lista sola dei tipi di seduta in un file di dati** (§4.1)? *Sì.*
5. **Emendare ADR-114 verso `L1-COACH.md`** (spareggio, panchina dalle regole, blocco continuo, radici con le code)? *Sì, nel primo anello.*
6. **In L1 riferisce l'allenatore stesso, e nella 0.2 le conferenze le delega il vice** (§4.4, §4.5)? *Sì.*
7. **I dieci principi del flusso** (§2.6): quali tieni, quali togli? *Tutti; i più urgenti sono l'1, il 2 e l'8.*
8. **Le nazionali e la programmazione della stagione** (§5): in L1 o dopo? *Le pause per le nazionali in L1 (sono calendario); la programmazione dopo la settimana.*
