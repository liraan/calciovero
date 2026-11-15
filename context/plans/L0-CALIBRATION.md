# L0 — la taratura fine (2026-11-15)

**Cos'è.** La fase 4 del mandato del maintainer del 2026-11-15: il motore misurato contro il calcio reale, metrica per metrica, con la sua banda e, per ogni metrica fuori banda, la causa nel modello o l'OI che la cerca. Nessun numero è stato ritoccato per rientrare in una banda (RULES A5-bis).

## Come si misura

- **Strumento:**
  ```bash
  cargo run --release -p ds-reference -- compare --reference L0-match/reference/serie-a-2017-18.json --seeds 42..101
  ```
  60 partite, le squadre di prova, blocco medio e ampiezza normale.
- **Riferimento:** Serie A 2017-18, 380 partite, Wyscout (Pappalardo et al. 2019, *Scientific Data* 6, 236; CC BY 4.0), per partita ed entrambe le squadre.
- **Motore:** `SIM_VERSION` 126 (rimisurato il 2026-11-15, dopo memoria, barriera, spazzata, portiere sulla bisettrice e la scelta dell'ADR-052; al 118 erano in banda 9 su 22). La misura del 116, del 117 e del 118 è identica riga per riga: il 117 e il 118 cambiano solo le partite di coppa e le squadre con l'allenatore che reagisce.
- **La banda:** media reale ± una deviazione standard delle 380 partite reali. La fonte è lo stesso riferimento. È una regola sola per tutte le metriche, presa dalla dispersione del calcio vero e non dai valori del motore. Il caos del motore (circa 1.5 gol fra due esecuzioni che differiscono di un bit, SIM 101) sta dentro questa larghezza.
- **Non confrontabili:** cross e intercetti. Le definizioni Wyscout e del motore differiscono (`ds-reference`, nota della tabella).

## La tabella

| Metrica | Reale | ± | Motore 126 | Banda | Dentro? | Causa o OI |
|---|---|---|---|---|---|---|
| gol | 2.68 | 1.65 | 3.65 | 1.03–4.33 | sì | — |
| autogol | 0.10 | 0.31 | 0.18 | 0–0.41 | sì | — |
| tiri | 24.60 | 5.46 | 46.25 | 19.1–30.1 | no | OI-100 |
| tiri in porta | 8.69 | 3.22 | 25.13 | 5.5–11.9 | no | OI-100 (l'errore del tiro: `kick.json` `error_pressure_rad`, `error_power_gain` senza fonte) |
| rigori | 0.33 | 0.56 | 0.27 | 0–0.89 | sì | — |
| passaggi | 887.7 | 122.8 | 1078.60 | 765–1011 | no | OI-100: il gioco è troppo preciso, la palla si perde poco |
| passaggi riusciti | 739.2 | 124.8 | 1034.73 | 614–864 | no | OI-100 |
| duelli aerei | 38.14 | 12.98 | 13.30 | 25.2–51.1 | no | OI-102: il secondo uomo sui palloni alti |
| duelli a terra | 182.6 | 27.5 | 218.87 | 155–210 | no | OI-103: protezione e dribbling, `duel.json` senza fonte |
| dribbling riusciti | 16.03 | 5.67 | 9.83 | 10.4–21.7 | no | OI-103 |
| falli | 26.00 | 6.69 | 19.73 | 19.3–32.7 | sì | — |
| gialli | 3.97 | 1.96 | 4.43 | 2.0–5.9 | sì | — |
| rossi | 0.24 | 0.49 | 0.08 | 0–0.73 | sì | — |
| angoli | 10.30 | 3.58 | 20.63 | 6.7–13.9 | no | OI-100: seguono le parate (27 contro 9), cioè i tiri in porta |
| rimesse laterali | 42.72 | 8.92 | 4.65 | 33.8–51.6 | no | OI-100: la palla esce poco dai lati; ogni volo è previsto senza errore (ADR-062, i voli senza fonte) |
| rinvii dal fondo | 17.15 | 4.20 | 10.90 | 13.0–21.4 | no | OI-100: il tiro troppo preciso finisce poco fuori, quindi pochi rinvii |
| punizioni | 29.45 | 6.81 | 14.00 | 22.6–36.3 | no | OI-112: probabilmente di conteggio (falli 20.5 e fuorigioco 6.4 danno 27 punizioni, il motore ne racconta 15) |
| fuorigioco | 4.26 | 2.40 | 6.15 | 1.9–6.7 | sì | — |
| spazzate | 29.53 | 8.65 | 34.75 | 20.9–38.2 | sì | — |
| parate | 9.37 | 3.37 | 23.97 | 6.0–12.7 | no | OI-100 (seguono i tiri in porta) |
| recupero, primo tempo (min) | 1.49 | 1.35 | 5.55 | 0.1–2.8 | no | OI-101: il motore aggiunge tutto il tempo perso, gli arbitri circa metà (fonte nell'OI) |
| recupero, secondo tempo (min) | 4.08 | 1.61 | 4.68 | 2.5–5.7 | sì | — |

**In banda (SIM 126): 9 metriche su 22 confrontabili. Fuori banda: 13, ognuna con il suo OI aperto** (OI-100, OI-101, OI-102, OI-103, OI-112, OI-114).

## I parametri di grado 3 (fase 4.1)

L'inventario completo è in `L0-AUDIT.md` §7: 381 parametri, di cui 25 di grado 1, 0 di grado 2 e 356 di grado 3. Nessuno è stato portato a grado 1 o 2 in questa fase:
- **le fonti:** quelle che servirebbero sono negli OI qui sopra (lo StatsBomb degli expected completion per la precisione dei passaggi; gli esiti dei duelli a terra per `duel.json`), e non sono ancora lette;
- **il grado 2:** chiede una metrica reale tenuta da parte, e i fuori banda sono legati fra loro (tiri, tiri in porta, parate, angoli, gol). Fissarne uno guardando un altro della stessa catena non sarebbe una verifica indipendente.

Restano quindi tutti nel registro, con il conteggio in `STATE.md`.

## La prossima misura

Dopo ogni causa corretta: la stessa misura, la tabella aggiornata qui, il `SIM_VERSION` con la sua voce in `CHANGELOG-SIM.md`.
