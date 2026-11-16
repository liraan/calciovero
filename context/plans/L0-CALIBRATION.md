# L0 — la taratura fine (2026-11-15)

**Cos'è.** La fase 4 del mandato del maintainer del 2026-11-15: il motore misurato contro il calcio reale, metrica per metrica, con la sua banda e, per ogni metrica fuori banda, la causa nel modello o l'OI che la cerca. Nessun numero è stato ritoccato per rientrare in una banda (RULES A5-bis).

## Come si misura

- **Strumento:**
  ```bash
  cargo run --release -p ds-reference -- compare --reference L0-match/reference/serie-a-2017-18.json --seeds 42..101
  ```
  60 partite, le squadre di prova, blocco medio e ampiezza normale.
- **Riferimento:** Serie A 2017-18, 380 partite, Wyscout (Pappalardo et al. 2019, *Scientific Data* 6, 236; CC BY 4.0), per partita ed entrambe le squadre.
- **Motore:** `SIM_VERSION` 133 (rimisurato il 2026-11-16, dopo la reazione dell'avversario al pallone calciato e il conteggio dei passaggi intercettati; al 132, la pressione sul passaggio e sulla spazzata; al 131, le forze del contrasto sulla stessa scala; al 130, la ricezione di ogni pallone libero; al 129, l'ampiezza con la palla in gioco, 47 m; al 128, il rimbalzo FIFA, 9 su 22; al 127, l'ampiezza del blocco dal tracking Metrica, 8 su 22; al 126 erano in banda 9 su 22, al 118 pure 9). La misura del 116, del 117 e del 118 è identica riga per riga: il 117 e il 118 cambiano solo le partite di coppa e le squadre con l'allenatore che reagisce.
- **La banda:** media reale ± una deviazione standard delle 380 partite reali. La fonte è lo stesso riferimento. È una regola sola per tutte le metriche, presa dalla dispersione del calcio vero e non dai valori del motore. Il caos del motore (circa 1.5 gol fra due esecuzioni che differiscono di un bit, SIM 101) sta dentro questa larghezza.
- **Non confrontabili:** cross e intercetti. Le definizioni Wyscout e del motore differiscono (`ds-reference`, nota della tabella).

## La tabella

| Metrica | Reale | ± | Motore 133 | Banda | Dentro? | Causa o OI |
|---|---|---|---|---|---|---|
| gol | 2.68 | 1.65 | 2.35 | 1.03–4.33 | sì | — |
| autogol | 0.10 | 0.31 | 0.42 | 0–0.41 | no | OI-100: dal 132 la spazzata sotto pressione devia di più; da guardare |
| tiri | 24.60 | 5.46 | 19.68 | 19.1–30.1 | sì | — |
| tiri in porta | 8.69 | 3.22 | 10.48 | 5.5–11.9 | sì | — |
| rigori | 0.33 | 0.56 | 0.02 | 0–0.89 | sì | — |
| passaggi | 887.7 | 122.8 | 1251.15 | 765–1011 | no | OI-100: troppi passaggi, e troppi intercettati (290 a partita); dal 2026-11-16 `ds-reference` conta anche i passaggi intercettati, che il motore racconta solo come `Interception` (prima 1065) |
| passaggi riusciti | 739.2 | 124.8 | 1049.22 | 614–864 | no | OI-100: l'83,9% dei tentati (Wyscout 83,3%), dal 133; troppi passaggi in tutto |
| duelli aerei | 38.14 | 12.98 | 7.42 | 25.2–51.1 | no | OI-102: il secondo uomo sui palloni alti |
| duelli a terra | 182.6 | 27.5 | 124.75 | 155–210 | no | OI-117: dal 133 meno intercetti, meno contrasti |
| dribbling riusciti | 16.03 | 5.67 | 19.70 | 10.4–21.7 | sì | — |
| falli | 26.00 | 6.69 | 8.50 | 19.3–32.7 | no | OI-114: dal 131 il difensore vince più contrasti, e un contrasto vinto non è fallo (ADR-042 §4) |
| gialli | 3.97 | 1.96 | 2.95 | 2.0–5.9 | sì | — |
| rossi | 0.24 | 0.49 | 0.02 | 0–0.73 | sì | — |
| angoli | 10.30 | 3.58 | 12.40 | 6.7–13.9 | sì | — |
| rimesse laterali | 42.72 | 8.92 | 15.05 | 33.8–51.6 | no | OI-100: raddoppiate dal 127 (3.85) con il rimbalzo; nel calcio vero il 21,5% segue una spazzata, il 19,7% un tocco, il 31% un duello, il 24% un passaggio sbagliato (Wyscout); nel motore un duello non manda quasi mai la palla fuori e il passaggio sbaglia poco |
| rinvii dal fondo | 17.15 | 4.20 | 7.07 | 13.0–21.4 | no | OI-100: il tiro troppo preciso finisce poco fuori, quindi pochi rinvii |
| punizioni | 29.45 | 6.81 | 12.75 | 22.6–36.3 | no | OI-112: probabilmente di conteggio |
| fuorigioco | 4.26 | 2.40 | 5.75 | 1.9–6.7 | sì | — |
| spazzate | 29.53 | 8.65 | 30.67 | 20.9–38.2 | sì | — |
| parate | 9.37 | 3.37 | 10.92 | 6.0–12.7 | sì | — |
| recupero, primo tempo (min) | 1.49 | 1.35 | 3.01 | 0.1–2.8 | no | OI-101: il motore aggiunge tutto il tempo perso, gli arbitri circa metà (fonte nell'OI) |
| recupero, secondo tempo (min) | 4.08 | 1.61 | 2.17 | 2.5–5.7 | no | — |

**In banda (SIM 133): 11 metriche su 22 confrontabili. Fuori banda: 11, ognuna con il suo OI aperto** (al 126: 9; al 127: 8, dentro i duelli a terra e fuori gol e falli; il 128 riporta dentro i gol, al bordo) (OI-100, OI-101, OI-102, OI-103, OI-112, OI-114).

## I parametri di grado 3 (fase 4.1)

L'inventario completo è in `L0-AUDIT.md` §7: 381 parametri, di cui 25 di grado 1, 0 di grado 2 e 356 di grado 3. Nessuno è stato portato a grado 1 o 2 in questa fase:
- **le fonti:** quelle che servirebbero sono negli OI qui sopra (lo StatsBomb degli expected completion per la precisione dei passaggi; gli esiti dei duelli a terra per `duel.json`), e non sono ancora lette;
- **il grado 2:** chiede una metrica reale tenuta da parte, e i fuori banda sono legati fra loro (tiri, tiri in porta, parate, angoli, gol). Fissarne uno guardando un altro della stessa catena non sarebbe una verifica indipendente.

Restano quindi tutti nel registro, con il conteggio in `STATE.md`.

## L'ampiezza del gioco (2026-11-15)

- **Reale, Wyscout Serie A 2017-18** (337.317 passaggi): il 20,3% parte dal decimo esterno della larghezza, da un lato o dall'altro.
- **Reale, Metrica Sports** (partite campione 1 e 2): la palla sta nel decimo esterno il 7,5-9,8% del tempo per lato; con la palla l'uomo più vicino a una linea laterale ne dista 4 m (mediana).
- **Motore 126** (semi 42-51, dal registro dei tick): l'1,6% dei passaggi parte dal decimo esterno; le ali non andavano oltre 9 m dalla linea (la lettura della griglia, corretta al 127).
- **Dal centro verso la fascia:** dai decimi 2-4 della larghezza il 7,5-17,9% dei passaggi veri arriva nel decimo esterno del proprio lato (Wyscout); nel motore 128 lo 0,5-1,6%. Dalla fascia invece il motore è vicino al vero (dal decimo esterno ci resta il 39% contro il 29%). Manca il passaggio verso la linea, e il compagno che la tiene: con la palla in gioco e al centro, il più largo vero sta a 9-10 m dalla linea, quello del motore a circa 13 m (al 128; con la palla dovunque, mediana 10,6 m).

## La prossima misura

Dopo ogni causa corretta: la stessa misura, la tabella aggiornata qui, il `SIM_VERSION` con la sua voce in `CHANGELOG-SIM.md`.
