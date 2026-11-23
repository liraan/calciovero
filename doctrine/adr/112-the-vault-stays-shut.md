# ADR-112 — Il caveau resta chiuso: il motore chiede letture, non caratteristiche (DD-56)

- **Status:** Accepted (delegated, 2026-11-23) — da rivedere da parte del maintainer
- **Date:** 2026-11-23
- **Decision:** DD-56
- **Attua:** `context/plans/CODE-AUDIT.md` B1; ADR-003 (piano A), ADR-041 §3, ADR-063 §4
- **Depends on:** ADR-003, ADR-041, ADR-052, ADR-063, ADR-066, ADR-094

## Il problema

L'audit del codice ha trovato il caveau aperto. `GroundTruth::latents_for` era pubblica ("read-only access for match plugins") e restituiva tutte le caratteristiche nascoste di un giocatore; il motore la usava (la compostezza del colpo di testa). `GroundTruth::characteristic(nome)` era letta direttamente dal motore in sei punti: il piede migliore e la lettura del gioco nel pensiero, la forza esplosiva del portiere per la barriera, la predisposizione agli infortuni per il colpo. Il doc dell'orchestratore diceva "it never exposes a latent value"; ADR-041 §3 dice che il motore non legge il piano A; `MatchContext.hidden` diceva "never read directly; only `resolve_*`". Nessun test lo verificava.

## Decisione

1. **Le latenti intere non escono da `ds-orchestrator`.** `latents_for` è privata.
2. **Il motore chiede all'attore le sue letture, già fatte.** Ogni lettura che usava una caratteristica diventa un metodo con il nome di ciò che l'attore (o il suo corpo) sa, non della caratteristica: `strong_foot` (il piede su cui sposta la palla), `believed_run_pace` (quanto crede che corra un compagno, ADR-094), `choose_closed_place` (il posto che chiude, con la sua lettura del gioco), `offside_line_misread_cm` (quanto sbaglia a leggere la linea, OI-098), `wall_cover_cm` (quanta porta copre il portiere a una punizione), `contact_force_milli` (la forza di un contatto sul suo corpo, ADR-066). La formula del colpo passa in `ds-calculator` (`duel::contact_force_milli`); l'arbitro confronta la forza con la soglia della Legge.
3. **Un plugin legge una caratteristica solo attraverso `World::characteristic` e solo se la sua scheda la dichiara** (ADR-063 §4). Oggi l'unica è `endurance` della fatica.
4. **Un test strutturale lo protegge**: `anti_leak::the_engine_reads_no_characteristic_but_through_its_cards` legge tutto il codice del motore (non i suoi test) e fallisce su un `latents_for(`, su un `.characteristic(` che non sia di `World`, o su una lettura non dichiarata nella scheda.

Con LUNA: il dito è un nome di caratteristica in un file del motore; la luna è che quello che un giocatore è lo sa solo lui (e chi lo conosce), e il mondo vede soltanto quello che fa.

## Conseguenze

- Nessun numero, nessuna regola e nessun comportamento cambiano: golden e semi identici, nessun `SIM_VERSION`.
- Restano fuori da questo ADR e aperti in `CODE-AUDIT.md` (sezione B): il motore che decide al posto dell'attore (`choose_place`, l'uscita del portiere, i cambi dell'allenatore), le formule e la fisica dell'orchestratore che non stanno nel calcolatore.

## Scartate

- **Un metodo per caratteristica** (`game_reading(id)`): sarebbe il caveau aperto con un altro nome.
- **Togliere `World::characteristic` dal contratto**: i plugin esterni (ADR-063) devono poter leggere ciò che dichiarano; la dichiarazione, non il divieto, è la regola.
