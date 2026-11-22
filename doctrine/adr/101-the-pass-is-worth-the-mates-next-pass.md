# ADR-101 — Il passaggio vale la scelta che il compagno avrà (DD-43)

- **Status:** Accepted (delegated, 2026-11-22) — da rivedere da parte del maintainer
- **Date:** 2026-11-22
- **Decision:** DD-43
- **Attua:** OI-116 (A), scelta dal maintainer il 2026-11-16 e confermata il 2026-11-22
- **Depends on:** ADR-035 (nessun dado), ADR-039 (read/choose/contact/band), ADR-040 (salienza del passaggio), ADR-050 (il portatore legge cosa porta un'opzione), ADR-052 (ogni giocatore pensa), ADR-064 (il primo tocco del ricevitore)

## Decisione

Il valore di un passaggio non è solo lo spazio del punto dove arriva: è anche
la migliore scelta che il ricevente avrà *in quel punto*, letta con lo stesso
pensiero di chi passa, un passo più in là. Il passatore mette il compagno al
punto d'arrivo nella scena che già vede, gli dà la palla ai piedi e legge le
sue opzioni; il passaggio vale almeno quanto la migliore di quelle. Nessun
peso nuovo, nessuna moltiplicazione aggiuntiva, nessuna scelta imposta al
compagno: tutte le sue opzioni restano disponibili e la sceglie lui quando
ci arriva.

Con i due filtri: il **dito** è il punto d'arrivo del pallone, le posizioni
viste dei compagni e degli avversari, la linea di sfondo e il tick; la
**luna** è lo stesso passatore che immagina un passo del pensiero del
compagno, e il compagno che, arrivato lì, sceglie come giocare. Cambia cosa
il passatore *legge* del futuro, non l'etichetta del gesto, non la Legge, non
la fisica del pallone e non la libertà di scelta del ricevente.

## Conseguenze

- `worth` resta la misura dell'ADR-052; `worth_of` pesa la stessa quantità
  quando il guadagno è già noto. Il ramo ground di `pass_candidates` legge
  `receiver_best_pass`, che rimanda a un solo livello di `pass_candidates`
  dalla posizione del ricevente: il pensiero resta limitato alla decisione
  successiva, come tenere la palla (ADR-052 §4). I rami loftato e filtrante
  non cambiano in questa decisione.
- Nessun numero nuovo in `balance/`. Il gemello
  `a_pass_to_a_man_with_a_free_pass_is_worth_more` ha lo stesso primo passo
  nelle due scene: cambia solo l'opzione successiva del ricevente (compagno
  libero contro compagno marcato). Prima della modifica i due valori sono
  uguali; dopo, il passo in più li distingue.
- La misura `ds-reference 42..101` si rilegge senza adattare numeri: il
  conteggio delle metriche in banda è un effetto della forma del modello,
  non un obiettivo di taratura (A5-bis).

## Scartate

- Valutare il passaggio con le cinque conduzioni del ricevente (la prova
  sopra SIM 134): premia lo spazio in avanti, non quello largo; ampiezza
  quasi invariata e spazzate, autogol, fuorigioco e angoli fuori banda.
  Tolta.
- Un numero nuovo per il "tempo sulla palla" del ricevente: servirebbe una
  fonte e sarebbe un peso, non una lettura (opzione B di OI-116).
- Seguire più di un passo di fila: il pensiero non è più limitato e il costo
  cresce senza una misura che lo giustifichi.

## Note datate

- **2026-11-22 — Status: accepted (delegated, DD-43).** Il maintainer ha
  scelto (A) il 2026-11-16 e ha confermato "oi-116 (a) ok procedi" il
  2026-11-22. Attuazione con SIM 186.
