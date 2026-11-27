# ADR-116 — La verità è un fascicolo di fonti, non un fornitore (DD-96)

- **Status:** Accepted (delegated, 2026-11-27) — da rivedere da parte del maintainer
- **Date:** 2026-11-27
- **Decision:** DD-96
- **Attua:** la scelta del maintainer di tenere aperta la fonte delle statistiche L0
- **Depends on:** ADR-008, ADR-052, `context/RULES.md` A4 e A5-bis

## Il problema

L0 confronta il motore con `L0-match/reference/serie-a-2017-18.json`, un solo
referto Wyscout. La ricerca del 27 novembre 2026 mostra che fonti diverse
contano la stessa voce in modo diverso (per i passaggi 995 contro 887 in una
stessa partita; per i duelli aerei 15 contese contro 38 righe), e che nessun
fornitore aperto copre da solo tutte le metriche con la definizione che il
motore misura: Wyscout lascia quattro voci non confrontabili, Sofascore ha 22
conteggi ma 14 senza riga evento attribuibile. Fissare un fornitore nel codice
costringerebbe a rifare un ADR a ogni cambio e
inviterebbe a scegliere la fonte guardando quale avvicina il motore alla banda:
è A5-bis.

## Decisione

La verità L0 non è un singolo file, ma un **fascicolo** (`bundle.json`): un
manifesto dati che dichiara, per ogni metrica misurata, quale fonte la fornisce.
Più fonti possono convivere; una voce può restare dichiarata non confrontabile
con il suo motivo, senza essere riempita da un aggregato per fare il conteggio.

Ogni fonte dichiara provenienza, licenza, granularità e se le sue righe sono
attribuibili a un giocatore. Il filtro è sempre **partita vera e LUNA**:

- **partita vera:** valori di partite reali osservate, per partita, con fonte e
  licenza note. Non bastano totali di squadra, numeri di un modello o numeri
  scelti per avvicinare il motore.
- **LUNA:** la riga riconduce l'esito a un attore che legge la scena e sceglie
  COME agire. Un'etichetta di osservatore o un totale di squadra è il dito; può
  essere un cross-check, non la verità su quella scelta.

Il comando `ds-reference bundle` appiattisce il fascicolo nel formato che
`compare` legge già, validando che ogni metrica abbia una fonte, che la fonte sia
di partite reali, con granularità `event` o `frame`, attribuibile a un attore e
con licenza dichiarata. Una fonte `aggregate` può stare nel fascicolo come
cross-check, ma il costruttore la rifiuta come verità. Nessun numero della
banda viene scelto dalla distanza dal motore.

Il nucleo del gioco non legge il fascicolo (ADR-008): solo `ds-reference`, che
è uno strumento. Non cambia `SIM_VERSION`, nessun golden, nessun parametro.

## Conseguenze

- Il fascicolo attuale contiene solo Wyscout 2017-18, quindi i numeri e le
  bande di oggi restano identici: la decisione apre la forma, non cambia le
  misure.
- Un nuovo fornitore con lo stesso schema entra come file JSON e righe del
  manifesto: **nessuna ricompilazione e nessun nuovo ADR**. Un formato nuovo
  aggiunge un lettore e un ramo alla CLI di `ds-reference`: nessun crate L0/L1.
- Un nuovo ADR serve solo se il provider cambia il significato di una metrica
  esistente, se cambia la regola di scelta della verità, o se cambiano bande o
  modello.
- `compare` legge `comparable` e `note` dal reference quando presenti, con
  fallback all'enum: i file storici restano validi e le note per fonte non
  vengono più perse.

## Scartate

- **Sostituire Wyscout con Sofascore come unica verità.** Copre 22 conteggi ma
  14 sono totali di squadra senza riga evento; per il filtro LUNA non può essere
  verità per quelle metriche. Sceglierlo perché certi numeri si avvicinano è
  A5-bis.
- **Un trait o un registry di provider nel codice.** Oggi la scelta è una riga
  di dati; un'astrazione servirebbe solo a preparare un futuro che il manifesto
  già gestisce.
- **Lasciare che `compare` scelga la fonte guardando il risultato.** La scelta
  è nel manifesto, fissata prima della misura.
