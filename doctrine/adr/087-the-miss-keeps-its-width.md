# ADR-087 — Il tiro non diventa preciso perché è vicino (OI-100)

- **Status:** Accepted (delegated, 2026-11-20) — da rivedere da parte del maintainer
- **Date:** 2026-11-20
- **Decision:** DD-27
- **Amends:** ADR-054 §Decision 1 (la forma angolare dell'errore), ADR-076 (l'altezza del tiro)
- **Depends on:** ADR-035 (nessun dado), ADR-051 (la famiglia del calcio), ADR-052 (la scena letta), ADR-057 (l'autocontrollo)

## Contesto

`error_ref_rad` 0,062 viene da Shimotashiro e Shinya 2024: calci da fermo a
**11 m**, pallone a terra, senza avversario, con uno scarto di circa 0,57 m per
asse (ADR-054). Il motore lo applica come angolo costante: lo scarto è
`distanza × angolo`. Sotto gli 11 m lo scarto si rimpicciolisce in proporzione,
finché a 3 m il tiro è quasi infallibile. Nelle stesse 60 partite 42..101 lo
specchio sotto gli 11 m resta troppo preciso (96% contro il 46% di StatsBomb,
prima ancora che il maintainer chiedesse il tiro di prima): il criterio di
fallimento scritto in `L0-match/balance/SOURCES.md` per `error_pressure_rad`
scatta, e chiede l'ADR sulla forma dell'errore, non un altro numero.

La causa non è la distanza in sé. A 3 m dal portiere il calciatore non ha il
tempo e il corpo che ha a 11 m sul pallone fermo: ha l'avversario addosso, il
pallone che arriva, il passo che non è assestato, il portiere che chiude. Il
tempo disponibile e l'assetto del corpo sono già nella scena letta — pressione,
passo (`gait`), corsa e traverso del corpo, stanchezza — e il maintainer ha
chiesto che l'errore dipenda da essi oltre che dalla pressione. Quello che non
può essere vero è che, avendo meno tempo e meno spazio, l'errore *assoluto*
diventi piccolo solo perché la distanza è piccola. Il primo tentativo (SIM 166
di prova, aggiungere il tiro di prima a ogni ricezione) ha aumentato i tiri da
30,5 a 40,5 a partita contro 24,6 reali e fatto uscire i rinvii dal fondo dalla
fascia: la causa era reale, ma l'attuazione era un burattino che vinceva troppo
spesso. È stata revertita senza toccare semi, bande o asserzioni.

## LUNA

Il dito: l'angolo di errore, la distanza in centimetri, la fase del passo, il
contatore di tick, l'evento `Shot` con il suo `on_target`.

La luna: lo stesso calciatore, con la sua finalizzazione, il suo piede, il suo
autocontrollo, la sua corsa e la sua lettura, che davanti a una porta vicina
sceglie dove tirare e calcia con il corpo che ha in quel momento. Non cambia
identità da 11 m a 3 m: cambiano il tempo che ha, l'assetto e il portiere che
gli sta davanti. L'errore è la sua misura di sé, non un regalo del motore.

## Decisione

1. **La larghezza misurata non si accorcia sotto la distanza misurata.** Per il
   tiro, la distanza usata per l'errore è la maggiore fra la distanza reale e
   **1100 cm**, la distanza del campione da cui viene `error_ref_rad`. Non è un
   numero nuovo: è la distanza già dichiarata della fonte di ADR-054. Sopra 11 m
   il modello non cambia; sotto, il tiro conserva lo scarto assoluto che la
   stessa tecnica ha a 11 m, mentre il tempo e l'assetto già letti (pressione,
   passo, corsa, traverso, fatica) continuano a decidere *quanto* e *dove*
   sbaglia. Nessun dado (ADR-035).
2. **Il calciatore non lo vede, quindi non lo usa per mirare.** ADR-054 §4:
   legge l'angolo della sua tecnica e la pressione, ma non la fase del passo con
   cui il piede incontra il pallone. La larghezza minima è la parte dell'errore
   che viene dall'assetto e dal tempo, non una mira che può scegliere: quindi
   `aim_inside` resta `distanza × angolo` e la scelta del tiro non cambia. Non è
   un burattino: cambia solo ciò che il contatto fa al pallone, non un ordine
   dato al giocatore. Il gemello del burattino regge (la stessa scena dà la
   stessa scelta).
3. **Vale per il tiro e per la testata.** La testata usa lo stesso
   `error_rad_composed` e lo stesso angolo; la stessa distanza effettiva si
   applica al suo scarto, senza una regola separata. I passaggi no: la loro
   precisione ha un riferimento e una destinazione propri e restano intatti.

## Scartate

- **Alzare ancora `error_pressure_rad`:** la fascia 11-18 m è già a 48% contro
  46%; a 0,16 scende al 28% e peggiora tutto. `SOURCES.md` vieta esplicitamente
  di cercare un altro numero dopo questo criterio di fallimento.
- **Rimettere il tiro di prima di SIM 166:** aumenta i tiri fuori area e rompe
  i rinvii dal fondo; la causa va cercata nella forma dell'errore, non nel
  conteggio delle occasioni.
- **Un errore diverso per ogni zona:** sarebbe una tabella di casi, non la
  lettura di uno stesso giocatore con capacità diverse.

## Conseguenze

- `SIM_VERSION` +1, golden rigenerati, voce in `CHANGELOG-SIM.md`, P1 e gemello
  nei test del tiro, misura 42..101 e tabella autorevole aggiornata.
- Il numero di tiri e la scelta del tiro non cambiano: cambia dove arriva il
  pallone quando il tiro parte vicino. La misura attesa è meno tiri in porta
  sotto gli 11 m, con le altre fasce e i gol da ricontrollare.
- Nessun merge in `main` senza autorizzazione.
