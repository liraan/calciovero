# Il campo — audit e piano (2026-11-22)

**Cos'è.** Il quadro completo di ciò che succede su un campo vero e non succede nel simulatore, e il piano ordinato per colmarlo. La fonte è il codice.e la partita vera, non la diagnosi dei documenti vecchi: dove un documento dice una cosa e il codice fa un'altra, vale il codice. Ogni voce è letta con i due filtri: **partita vera** (chi, dove, quando, evento, palla, corpi) e **LUNA** (lo stesso giocatore legge, sceglie ed esegue con le proprie caratteristiche e abilità). Ogni decisione di campo è una derivata di caratteristiche e abilità: il vocabolario delle scelte non si restringe, le abilità ne cambiano costo e qualità d'esecuzione; le uniche chiusure legittime sono percezione e Regole.

## 1. Il quadro — cosa manca

| Fenomeno (ciò che vede un tifoso) | Stato | Prova nel codice | Cosa manca davvero | Riferimento |
|---|---|---|---|---|
| Gol 9,53 contro 2,68; tiri 38,07 contro 24,60; in porta 17,45 contro 8,69; parate 20,80 contro 9,37; angoli 23,40 contro 10,30 | parziale | `read.rs:787-895`; `thought.rs:250-319`; `L0-CALIBRATION.md:171-174` | Il portatore sceglie il tiro in scene troppo comode: manca il tiro di prima, la testata in area, il tiro con l'uomo addosso e il portiere che chiude. Parate e angoli sono conseguenze | OI-100; ADR-087 non si riapre |
| Passaggi 1753 contro 888, accuratezza 82,5% contro 83,3% | parziale | `read.rs:1295-1330,1405-1412` | Il passatore legge un livello in più e non paga il costo di tenere palla/girarsi: passa troppo, non passa male | ADR-101 chiusa, volume in OI-100 |
| Duelli aerei 9,08 contro 38,14; autogol 1,97 contro 0,10 | parziale | `technique.rs:49-103`; `offball.rs:44-56,343-363` | Nessuna decisione su *quando* saltare, nessun secondo uomo sulla respinta; il portiere entra solo se stima di vincere | OI-102 |
| Attaccanti fermi (72% «stare fermo»), pressing 1,1%, copertura 0,5%, squadra che non si stringe, nessun terzo uomo, nessuna marcatura in area, transizioni deboli | mancante | `offball.rs:187,203`; `BEHAVIOURS.md:267-276` | Il senza palla non è una decisione dei giocatori: manca l'intensità letta, non il gesto | in gran parte da aprire |
| Tacco, flick di testa, finte, volée, curva, uno-due, nutmeg, controllo di petto/coscia, pugno, spill, get set, rimessa lunga/corta, angolo interno/esterno | mancante | `ADR-085:239,246,259,263,264,276,278-279,303`; `event.rs:21-91` | Il vocabolario delle scelte è più povero del reale: il dito non può nemmeno essere scelto | OI-129 |
| Portiere risolutore sul piano porta, non attore | parziale | `keeper.rs:24-143`; `ball.rs:170-173`; `technique.rs:272-282` | Non sceglie di uscire, posizionarsi, parare a pugni, rotolare; l'identità è uno slot hardcoded | OI-129 |
| Contatto corpo: separazione geometrica e duello, nient'altro | parziale | `spatial.rs:471-472,1585`; `duels.rs:28-195`; `set_pieces.rs:482-596` | Non esiste contatto mantenuto né tatto come percezione del movimento avversario; il muro è statico | da aprire |
| Percezione: posizioni e punto di caduta esatti | parziale | `ADR-062:39`; `read.rs:155-193`; `latency.rs:16-26` | Mancano ampiezza dell'attenzione, memoria dell'ultimo sguardo, flash-lag, fatica cognitiva come produttore | OI-098, OI-104 |
| Vantaggio senza evento, fuorigioco come nota JSON, recupero 1T 9,01 contro 1,49 min | parziale | `referee.rs:326-331,393-418,567-570`; `balance.rs:86` | Le Leggi sono applicate ma il fenomeno osservabile sparisce tra motore e vista | OI-101, OI-112 |
| Infortuni da contatto senza fallo | mancante | `ADR-066:39-40`; `balance.rs:772-773` | Solo il fallo chiamato produce un knock; circa metà degli infortuni reali non ha fallo | limite dichiarato |
| Fischio udibile e pausa implementati; mancano gesto arbitro, bandierina, eventi per turn/stop/cover/shield/marca/salto, folla e telecamere | parziale | `animation.rs:40-78`; `record.rs:37-70`; `stadium.rs:40-168` | Lo spettatore sente il fischio ma non vede chi fischia né perché; atti del motore non raccontati | GUI |

Difetti latenti, non fenomeni di campo: `referee.rs:319-324` ha `card_after` con la guardia invertita e funziona solo perché il chiamante passa il booleano negato (`:547-552`); `observation.rs:3-4,11-13` promette un consumatore `ds-stats` che non esiste; `RandomEvent` ha narratore e animazione ma nessun produttore.

## 2. Il piano — cosa fare, in che ordine

Ogni fase con cambio motore ha ADR/DD proprio, gemello, SIM bump, golden e semi rigenerati, misura `ds-reference`, collaudo GUI e commit separato. Il maintainer ha chiesto di **ricalcolare ogni volta**: nessun accorpamento per risparmiare rigenerazioni. Nessun numero adattato alla metrica giudicata (A5-bis), nessuna deroga alla soglia L di 16/22.

| Fase | Cosa cambia in campo | Da che cosa deve derivare | Non cambia | Prova che è risolta |
|---|---|---|---|---|
| 0 — Debiti | SIM 186 committato e pushato; `card_after` riscritto senza cambiare l'esito; canale delle osservazioni collegato o tolto; `RandomEvent` con produttore o eliminato | — | Nessun comportamento | Golden identici, C2 verde |
| 1 — Percezione | Posizione e caduta lette con errore, attenzione come `vision`, memoria dell'ultimo sguardo, flash-lag, fatica cognitiva che rallenta la decisione | `vision`, `concentration`, `game_reading`, `decision_making` | Regole, fisica del corpo, vocabolario | Gemelli «non vede ciò che non ha guardato», trace percezione contro verità |
| 2 — Piede e aria | Tiro di prima, testata in area, tiro sotto pressione, scelta del salto, secondo uomo sulla respinta | `composure`, `technique`, `reactivity`, `courage` come qualità d'esecuzione | ADR-087 invariata, nessun clamp | Log per tiro (distanza, contatto, pressione, esito) e gemelli |
| 3 — Senza palla | Pressing con durata, secondo che copre, compattezza come lettura, terzo uomo, marcatura in area, prime transizioni | `work_rate`, `aggressiveness`, `discipline`, `concentration` | Nessuna destinazione imposta, nessun latente nuovo | Scene gemelle con lo stesso compagno marcato |
| 4 — Corpo | Contatto mantenuto come senso del movimento, jostle continuo e sui piazzati, knock da contatto non falloso | `body_control`, `courage`, forza del contatto già misurata | Nessuna forza nuova, nessuna opzione nuova; ADR-066 solo con ADR nuovo | Gemello «sente il movimento senza ricevere una scelta» |
| 5 — Regole visibili | Evento `Advantage`, bandierina, cartellini coerenti, recupero frazionato, fischio stop/ripartenza con pausa e battitore che cammina | Arbitro = Leggi oggettive (ADR-021) | Nessun errore arbitrale inventato | Event chain completa, capture GUI |
| 6 — Portiere e repertorio | Portiere attore (claim, get set, uscita, pugno, spill, rotolata); gesti rimasti raggruppati per meccanica condivisa | `courage`, `concentration`, `technique`, `composure` | Tutte le opzioni restano disponibili | Evento per ogni gesto, test del repertorio |
| 7 — Presentazione | Eventi per atti invisibili con `RECORD_FORMAT_VERSION`; poi figurine, folla, telecamere, replay | — | Nessun cambio di motore | Narrativa e GUI leggono lo stesso fatto |

Le fasi 1-3 attaccano la radice delle 16 metriche fuori banda; la 4 è la richiesta sul contatto e sui piazzati; la 5 chiude i segnali dell'arbitro; la 7 resta ultima perché è solo vista.

## 3. Stato di questa consegna

Il fischio di stop e di ripartenza, la pausa reale di 2 secondi e il cartello visibile sono implementati in `tools/ds-gui/src/app.rs`, con i gemelli `the_two_whistles_follow_the_foul_and_the_restart` e `the_stop_whistle_holds_the_picture_and_the_taker_walks_to_the_ball`: 27 test GUI verdi. Nessun numero, regola o scelta del motore toccata dal fischio, quindi nessun SIM bump per esso. Resta il collaudo finale del maintainer sulla GUI.
