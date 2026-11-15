# L0 — la taratura fine (2026-11-15)

**Cos'è.** La fase 4 del mandato del maintainer del 2026-11-15: il motore misurato contro il calcio reale, metrica per metrica, con la sua banda e, per ogni metrica fuori banda, la causa nel modello o l'OI che la cerca. Nessun numero è stato ritoccato per rientrare in una banda (RULES A5-bis).

## Come si misura

- **Strumento:**
  ```bash
  cargo run --release -p ds-reference -- compare --reference L0-match/reference/serie-a-2017-18.json --seeds 42..101
  ```
  60 partite, le squadre di prova, blocco medio e ampiezza normale.
- **Riferimento:** Serie A 2017-18, 380 partite, Wyscout (Pappalardo et al. 2019, *Scientific Data* 6, 236; CC BY 4.0), per partita ed entrambe le squadre.
- **Motore:** `SIM_VERSION` 127 (rimisurato il 2026-11-15, dopo l'ampiezza del blocco misurata sul tracking Metrica; al 126 erano in banda 9 su 22, al 118 pure 9). La misura del 116, del 117 e del 118 è identica riga per riga: il 117 e il 118 cambiano solo le partite di coppa e le squadre con l'allenatore che reagisce.
- **La banda:** media reale ± una deviazione standard delle 380 partite reali. La fonte è lo stesso riferimento. È una regola sola per tutte le metriche, presa dalla dispersione del calcio vero e non dai valori del motore. Il caos del motore (circa 1.5 gol fra due esecuzioni che differiscono di un bit, SIM 101) sta dentro questa larghezza.
- **Non confrontabili:** cross e intercetti. Le definizioni Wyscout e del motore differiscono (`ds-reference`, nota della tabella).

## La tabella

| Metrica | Reale | ± | Motore 127 | Banda | Dentro? | Causa o OI |
|---|---|---|---|---|---|---|
| gol | 2.68 | 1.65 | 4.67 | 1.03–4.33 | no | OI-100: il blocco senza palla, ora largo quanto quello vero (37 m), lascia i varchi che lascia un blocco vero, e il tiro resta troppo preciso |
| autogol | 0.10 | 0.31 | 0.12 | 0–0.41 | sì | — |
| tiri | 24.60 | 5.46 | 45.90 | 19.1–30.1 | no | OI-100 |
| tiri in porta | 8.69 | 3.22 | 25.15 | 5.5–11.9 | no | OI-100 (l'errore del tiro: `kick.json` `error_pressure_rad`, `error_power_gain` senza fonte) |
| rigori | 0.33 | 0.56 | 0.25 | 0–0.89 | sì | — |
| passaggi | 887.7 | 122.8 | 1104.53 | 765–1011 | no | OI-100: il gioco è troppo preciso, la palla si perde poco |
| passaggi riusciti | 739.2 | 124.8 | 1066.25 | 614–864 | no | OI-100 |
| duelli aerei | 38.14 | 12.98 | 11.38 | 25.2–51.1 | no | OI-102: il secondo uomo sui palloni alti |
| duelli a terra | 182.6 | 27.5 | 208.63 | 155–210 | sì | OI-103 (al bordo) |
| dribbling riusciti | 16.03 | 5.67 | 8.25 | 10.4–21.7 | no | OI-103 |
| falli | 26.00 | 6.69 | 18.48 | 19.3–32.7 | no | OI-114 |
| gialli | 3.97 | 1.96 | 4.40 | 2.0–5.9 | sì | — |
| rossi | 0.24 | 0.49 | 0.12 | 0–0.73 | sì | — |
| angoli | 10.30 | 3.58 | 19.93 | 6.7–13.9 | no | OI-100: seguono le parate (24 contro 9), cioè i tiri in porta |
| rimesse laterali | 42.72 | 8.92 | 3.85 | 33.8–51.6 | no | OI-100: seguono quasi tutte un passaggio (43 su 44, semi 42-51) e il passaggio del motore va fuori di rado; non è l'ampiezza: a SIM 127 le ali stanno sulla linea come quelle vere (Metrica) e le rimesse non salgono |
| rinvii dal fondo | 17.15 | 4.20 | 11.40 | 13.0–21.4 | no | OI-100: il tiro troppo preciso finisce poco fuori, quindi pochi rinvii |
| punizioni | 29.45 | 6.81 | 14.75 | 22.6–36.3 | no | OI-112: probabilmente di conteggio |
| fuorigioco | 4.26 | 2.40 | 6.57 | 1.9–6.7 | sì | — |
| spazzate | 29.53 | 8.65 | 29.73 | 20.9–38.2 | sì | — |
| parate | 9.37 | 3.37 | 23.88 | 6.0–12.7 | no | OI-100 (seguono i tiri in porta) |
| recupero, primo tempo (min) | 1.49 | 1.35 | 6.02 | 0.1–2.8 | no | OI-101: il motore aggiunge tutto il tempo perso, gli arbitri circa metà (fonte nell'OI) |
| recupero, secondo tempo (min) | 4.08 | 1.61 | 4.81 | 2.5–5.7 | sì | — |

**In banda (SIM 127): 8 metriche su 22 confrontabili. Fuori banda: 14, ognuna con il suo OI aperto** (al 126: 9; il 127 porta dentro i duelli a terra e fuori gol e falli, con tre numeri che ora hanno una fonte) (OI-100, OI-101, OI-102, OI-103, OI-112, OI-114).

## I parametri di grado 3 (fase 4.1)

L'inventario completo è in `L0-AUDIT.md` §7: 381 parametri, di cui 25 di grado 1, 0 di grado 2 e 356 di grado 3. Nessuno è stato portato a grado 1 o 2 in questa fase:
- **le fonti:** quelle che servirebbero sono negli OI qui sopra (lo StatsBomb degli expected completion per la precisione dei passaggi; gli esiti dei duelli a terra per `duel.json`), e non sono ancora lette;
- **il grado 2:** chiede una metrica reale tenuta da parte, e i fuori banda sono legati fra loro (tiri, tiri in porta, parate, angoli, gol). Fissarne uno guardando un altro della stessa catena non sarebbe una verifica indipendente.

Restano quindi tutti nel registro, con il conteggio in `STATE.md`.

## L'ampiezza del gioco (2026-11-15)

- **Reale, Wyscout Serie A 2017-18** (337.317 passaggi): il 20,3% parte dal decimo esterno della larghezza, da un lato o dall'altro.
- **Reale, Metrica Sports** (partite campione 1 e 2): la palla sta nel decimo esterno il 7,5-9,8% del tempo per lato; con la palla l'uomo più vicino a una linea laterale ne dista 4 m (mediana).
- **Motore 126** (semi 42-51, dal registro dei tick): l'1,6% dei passaggi parte dal decimo esterno; le ali non andavano oltre 9 m dalla linea (la lettura della griglia, corretta al 127).

## La prossima misura

Dopo ogni causa corretta: la stessa misura, la tabella aggiornata qui, il `SIM_VERSION` con la sua voce in `CHANGELOG-SIM.md`.
