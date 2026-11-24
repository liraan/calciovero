# Il formato del pacchetto del mondo, versione 1

Il contratto fra il gioco e chi scrive pacchetti (gli importatori, in un'altra repo; chi modifica a mano). Un pacchetto è una cartella con questi file, in JSON (UTF-8). Il validatore è `ds_world::Pack::load_dir`: un pacchetto che non lo passa non si carica, e l'errore dice file, voce, campo e perché.

## `nation.json` (obbligatorio)

| Campo | Tipo | Regola |
| --- | --- | --- |
| `format_version` | intero | `1`; un'altra versione è rifiutata con il suo numero |
| `nation` | testo | non vuoto |
| `series` | lista di `{ "name": testo, "clubs": intero }` | almeno una; `clubs` ≥ 2 |

## `clubs.json` (obbligatorio)

Una lista; la somma dei `clubs` delle serie è la sua lunghezza.

| Campo | Tipo | Regola |
| --- | --- | --- |
| `city` | testo | non vuoto |
| `region` | testo | non vuoto |
| `colours` | due testi `"#RRGGBB"` (esadecimale, come in ogni editor di immagini) | i due colori diversi |
| `series` | intero | l'indice di una serie di `nation.json`; le voci in ordine di serie |
| `name` | testo, facoltativo | unico fra i club; se manca, inventato dal seme |
| `stadium` | testo, facoltativo | se manca, dalle forme di `names/stadium_forms.json` |

## `coaches.json` (facoltativo)

Una lista di allenatori scritti a mano, ognuno con `club` (il numero del club in `clubs.json`, da 0) e i campi che si vogliono fissare: `name`, `born` (1930-2000), `nationality`. Un club per voce. Quello che manca lo fa il seme. Le disposizioni dell'allenatore (l'idea di gioco, l'adattabilità e le altre, ADR-114) non si scrivono qui: sono nascoste, come le caratteristiche dei giocatori.

## In `nation.json`, per ogni serie: `bench` (facoltativo)

Quanti sostituti una squadra può mettere in distinta (la regola della competizione). L'allenatore ne nomina fino a quanti il motore sa giocarne (oggi 7); senza `bench`, quanti il motore ne sa giocare.

## `players.json` (facoltativo)

Una lista di giocatori che vincono sul generatore.

| Campo | Tipo | Regola |
| --- | --- | --- |
| `club` | intero | l'indice di un club di `clubs.json` |
| `number` | intero | il posto nella rosa, minore del numero dei ruoli di `names/squad_roles.json` |
| `name` | testo, facoltativo | |
| `role` | testo, facoltativo | uno dei ruoli di `names/squad_roles.json` |
| `born` | intero, facoltativo | l'anno di nascita, 1950-2015 |

Due voci con lo stesso `club` e `number` sono rifiutate.

## `names/` (obbligatorio)

| File | Contenuto |
| --- | --- |
| `word_parts.json` | `{ "start": [...], "middle": [...], "end": [...] }`: le parole inventate dei club (`middle` può contenere `""`) |
| `club_forms.json` | lista di forme con `@` (la parola) |
| `stadium_forms.json` | lista di forme con `{city}` |
| `first_names.json` | lista di nomi propri |
| `surnames.json` | `{ "start": [...], "end": [...] }` |
| `squad_roles.json` | la rosa, un ruolo per posto (`goalkeeper`, `full_back`, `centre_back`, `wide_midfielder`, `central_midfielder`, `striker`) |

Ogni lista non vuota. Un cambio del formato alza `format_version` e ha la sua voce in fondo a questo file.

## Versioni

- **1** (2026-11-24): prima versione.
