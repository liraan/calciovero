# ADR-063 (proposto) — La partita divisa in plugin

- **Status:** proposto, non accettato, non implementato. Aspetta il sì del maintainer dopo la partita di prova di L0.
- **Date:** 2026-11-15
- **Closes:** OI-94
- **Amends:** ADR-041 (lo split di `ds-match`, rinviato), ADR-012 (la scheda del plugin)
- **Depends on:** decisione A e decisione 4 di `plans/DECISIONS-2026-11-15.md`, ADR-035, ADR-052, ADR-059

*Lo scrivo in `context/plans/`, non in `doctrine/adr/`, perché il gate B6 non ammette ADR proposti fra quelli della dottrina. Accettato, va in `doctrine/adr/063-...` con il suo indice.*

## Contesto

`ds-match` ha più motivi di cambiamento (OI-94): il tick, il movimento, la percezione, il pensiero senza palla, le Leggi, gli eventi, il salvataggio, la cronaca. Il maintainer ha deciso il 15 novembre 2026 di non riscrivere L0: prima si chiude il calcio, poi lo si divide in plugin con lo stesso contratto che avrà L1 (`ds-season-api`), così un contributor impara un modello solo.

## Decisione

1. **Il nucleo della partita** fa solo cinque cose:
   - l'orologio a tick, 10 al secondo;
   - lo stato osservabile del campo (posizioni, velocità, il pallone, il possesso, il punteggio, i tempi);
   - il registro dei plugin, con la scheda di ADR-012: nome, versione, cosa legge, cosa scrive, peso nel livello di dettaglio, testi della cronaca;
   - gli eventi (Piano B, append-only);
   - il record e il replay.
2. **Le fasi del tick sono un contratto**, otto e in quest'ordine (decisione A del 15 novembre):
   1. percepire (il cono, lo sguardo, la memoria);
   2. decidere senza palla;
   3. decidere con palla (il portatore vede i movimenti appena decisi);
   4. agire;
   5. muovere;
   6. risolvere i contatti;
   7. arbitrare;
   8. registrare.
   Ogni plugin dichiara in quale fase scrive. Nella stessa fase nessuno legge ciò che un altro scrive in quella fase.
3. **Un crate pubblico `ds-match-api`**, gemello del futuro `ds-season-api` di L1: il contratto del plugin, i tipi dello stato osservabile e degli eventi, le fasi. Senza dipendenze dagli altri `ds-*`, come `ds-testkit`.
4. **Undici plugin obbligatori:** pallone, corpo, percezione, pensiero, gesti tecnici, duelli, portiere, arbitro, palle inattive, panchina, fatica. **Tre si possono spegnere:** psicologia in partita, ambiente, colpi. Le letture facoltative si dichiarano nella scheda e le controlla il validatore (decisione B).
5. **Gli ufficiali compilati dentro, gli esterni in WebAssembly** (decisione 4): isolati, deterministici, NaN canonicalizzati, con carburante; un contratto solo.
6. **La divisione non cambia il calcio:** a ogni passo i tre golden restano identici, bit per bit, su glibc e musl.

## Verifica nel codice di oggi (SIM 118): tutti decidono sul mondo del tick precedente?

Non del tutto:
- **Senza palla** (`state.rs` `decide_off_ball`): ogni giocatore legge posizioni, memoria e la vista della squadra (`TeamView`) calcolate prima del giro sui giocatori. Il suo atto lo scrive per sé e nessun compagno lo legge nello stesso giro. Qui l'ordine non conta.
- **Il portatore** legge gli atti appena decisi senza palla. È voluto (decisione A, fase 3 dopo la 2), e il contratto lo dichiara.
- **Il substrato in `MatchState::tick`** (movimento, palla, contrasti, copertura) gira giocatore per giocatore. Non ho verificato se due giocatori che si muovono nello stesso tick dipendono dall'ordine (la separazione minima, il contrasto). È la prima cosa da controllare prima di parallelizzare: se dipendono, le fasi 5 e 6 vanno fatte in due tempi, prima calcolare tutto dal mondo precedente e poi scrivere.

## Tabella: modulo, motivo di cambiamento, plugin futuro (dal codice di oggi)

| Modulo di oggi | Motivo di cambiamento | Plugin futuro |
|---|---|---|
| `simulate.rs`, `driver.rs` | l'orologio, i tempi, supplementari e rigori | nucleo (orologio), palle inattive (la serie) |
| `spatial.rs` (posizioni, volo, rimbalzo, intercetto) | la fisica del pallone e dei corpi | pallone; corpo |
| `perception.rs`, `MatchState::remember` | il cono, lo sguardo, la memoria | percezione |
| `state.rs` `decide_off_ball`, `ds-orchestrator/offball.rs` | il pensiero senza palla | pensiero |
| `state.rs` scelta del portatore, `ds-orchestrator/read.rs` | il pensiero con palla | pensiero |
| `ds-orchestrator/lib.rs` `resolve_*`, `ds-calculator/kick.rs` | passaggio, tiro, controllo | gesti tecnici |
| `ds-calculator/duel.rs`, `state.rs` `challenge_carrier` | i contrasti | duelli |
| `ds-calculator/save.rs`, il portiere in `state.rs` | parate, uscite, la palla in mano | portiere |
| `rules.rs`, i falli e i cartellini in `state.rs` | le Leggi | arbitro |
| `state.rs` `apply_restart`, `clear_the_spot`, `restart_taker` | rimesse, punizioni, angoli, rigori | palle inattive |
| `state.rs` `substitute_due`, `tired_for_fresh`, `instruct_due` | cambi e istruzioni | panchina |
| `ds-calculator/fatigue.rs`, `tire_players` | la fatica | fatica |
| (non ancora) composure, coraggio | gli appannatori del momento | psicologia in partita (si può spegnere) |
| (non ancora) campo, meteo | il terreno, il vento | ambiente (si può spegnere) |
| (non ancora) `Injury` | il colpo che fa uscire | colpi (si può spegnere) |
| `fact.rs`, `ds-stats` | gli eventi | nucleo (eventi) |
| `save.rs`, `record.rs` | salvataggio, replay, tick record | nucleo (record e replay) |
| `narrative.rs` | la cronaca: chi guarda non scrive nella partita | fuori dal motore (un lettore degli eventi) |

## Conseguenze

- Ogni plugin ha i suoi test e il suo gemello; il golden è il test del nucleo.
- Il costo è uno split lungo, un passo per plugin, ognuno a golden identico.
- Il primo passo è il controllo dell'ordine nel substrato (sopra): se l'ordine conta, va reso indipendente prima di dividere, e quel passo cambia il calcio, quindi porta il suo `SIM_VERSION`.
