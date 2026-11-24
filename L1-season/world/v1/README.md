# Il mondo v1: come si modifica

Questa cartella è il mondo con cui parte ogni carriera: la nazione, le sue serie, i club, i giocatori. Si modifica con un qualunque editor di testo; non serve toccare il codice. Il formato è descritto in [`FORMAT.md`](FORMAT.md).

**La regola: il file vince sul generatore.** Ogni campo scritto qui si usa così com'è; quello che manca lo genera il seme della carriera, sempre uguale per lo stesso seme. Per cambiare il nome di una squadra basta scrivere il suo `name`; il resto resta com'era.

| File | Cosa contiene |
| --- | --- |
| `nation.json` | la versione del formato, la nazione, le serie con il numero di club |
| `clubs.json` | un club per voce, nell'ordine delle serie: `city`, `region`, `colours` (due colori `"#RRGGBB"`, per esempio `"#1E50AA"`), `series` (0 la prima); facoltativi `name` e `stadium` |
| `players.json` | facoltativo: giocatori scritti a mano, ognuno con `club` (il numero del club in `clubs.json`, da 0), `number` (il posto nella rosa, da 0) e i campi che si vogliono fissare: `name`, `role`, `born` |
| `names/` | le parti dei nomi inventati: le parole dei club (`word_parts.json`), le forme dei nomi dei club (`club_forms.json`, "@" è la parola), degli stadi (`stadium_forms.json`, "{city}" è la città), nomi propri, cognomi, i ruoli della rosa |

**Esempio: dare un nome a una squadra.** In `clubs.json`, alla voce del club, aggiungere `"name": "Polisportiva Orobica"`.

**Esempio: fissare un giocatore.** In `players.json`: `[{ "club": 0, "number": 9, "name": "Marco Rinaldi", "role": "striker", "born": 2001 }]`.

**Partire dal mondo intero.** `cargo run -p ds-world -- export <cartella>` scrive il mondo generato da un seme con ogni campo compilato: lo si ritocca invece di partire da un foglio bianco.

**Gli errori.** Se qualcosa non va, il gioco dice il file, la voce, il campo e perché (per esempio `clubs.json, voce 7, name: due club con lo stesso nome`), e non parte.

Il mondo distribuito ha città, regioni e colori veri e nomi inventati, mai con la città dentro (decisione del 2026-11-24). I nomi veri arrivano solo da un pacchetto dell'utente; gli importatori stanno in un'altra repo e scrivono pacchetti in questo formato.
