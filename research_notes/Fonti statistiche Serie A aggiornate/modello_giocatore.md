# Modello del giocatore in Calcio Vero — stato del codice (27/11/2026)

Rapporto dell'agente esploratore, ripulito e conservato qui. Ha letto solo codice e test; non ha modificato il repository.

## 0. Sintesi

Oggi un giocatore non è un unico oggetto:

1. **Piano B, visibile**: `ds_model::Player` contiene identità, nome, ruolo come etichetta, corpo (altezza/peso) e stati fisici/psicologici. Non contiene nessun latente (`L0-match/crates/ds-model/src/player.rs:5-11`, `:173-192`).
2. **Piano A, nascosto**: `PlayerLatents` vive in `ds-db`, non è serializzabile, non ha `Debug`, viene derivato per chiave e letto solo dall'orchestratore (`L0-match/crates/ds-db/src/latents.rs:1-20`, `:39-50`; `ds-db/src/lib.rs:9-15`).

Il giocatore "reale" nel codice attuale è la coppia: `Player` con `PlayerId` stabile + corpo + stati, e `PlayerLatents` derivato da `(world_seed, PlayerId, caratteristica)`, eventualmente modellato dal profilo del ruolo. In L1 si aggiunge una chiave di identità che collega il posto in campo al vero giocatore per tutta la stagione.

Non esiste oggi una porta per inserire i valori di un giocatore reale persona per persona: si può cambiare il profilo comune di un ruolo in `profiles.json`, ma non fare un override del singolo calciatore.

## 1. Esiste nel codice

### 1.1 L'entità visibile: `Player`

`ds-model/src/player.rs:173-192`:

| Campo | Tipo | Significato |
|---|---|---|
| `id` | `PlayerId` | identità generazionale `(slot, generation)` (`ds-core/src/ids.rs:30-32`) |
| `name` | `String` | sola etichetta display |
| `role` | `RoleId` | stringa non vuota; a L0 è un'etichetta (`player.rs:46-72`) |
| `height_cm` | `u16` | dato anagrafico visibile, usato dalle formule |
| `weight_kg` | `u16` | dato anagrafico visibile, usato dalle formule |
| `physical` | `PhysicalStateMinimal` | stato fisico |
| `psychological` | `PsychologicalStateMinimal` | stato mentale |

`Player::try_new` valida nome non vuoto, altezza e peso non zero (`:194-236`).

Stato fisico (`:75-116`): `condition`, `match_sharpness`, `fatigue`, `cognitive_fatigue`, `damage`, tutti 0..1.
Stato psicologico (`:131-160`): `confidence`, `morale`, `pressure_load`, tutti 0..1.

### 1.2 I latenti nascosti: `PlayerLatents`

`ds-db/src/latents.rs:39-50` definisce quattro famiglie, 30 campi `f32`:

| Famiglia | Campi | Riga |
|---|---|---|
| `PhysicalLatents` (8) | `explosive_force`, `max_speed`, `injury_proneness`, `reactivity`, `maximal_force`, `body_control`, `endurance`, `joint_mobility` | `:52-80` |
| `TechnicalLatents` (9) | `finishing`, `passing`, `first_touch`, `tackling`, `heading`, `dribbling`, `goalkeeping`, `left_foot`, `right_foot` | `:82-108` |
| `PsychologicalLatents` (6) | `aggressiveness`, `emotional_control`, `work_rate`, `discipline`, `courage`, `composure` | `:110-135` |
| `CognitiveLatents` (7) | `game_reading`, `decision_making`, `risk_management`, `vision`, `anticipation`, `creativity`, `concentration` | `:137-163` |

Tutti derivano solo `Clone, Copy`: no `Debug`, no `PartialEq`, no `Serialize`. I test confrontano con `f32::to_bits` (`ds-db/src/lib.rs:154-165`). Anche il `Debug` di `Vault` stampa solo identità e numero di giocatori in cache (`lib.rs:129-135`).

I due piedi sono due prestazioni distinte (`:102-107`; test `feet_are_independent_performances`, `:288-297`). Non esistono compositi: `aerial` e `set_pieces` furono rimossi a `SIM_VERSION` 75 (`:30-35`).

### 1.3 Chi deriva e chi riempie i valori

- **Derivazione grezza per chiave**: `PlayerLatents::from_key(world_seed, player)` usa `unit_from_key(world_seed, player_id, "<caratteristica>")` (`:165-212`). Non entra `match_id`. La primitiva è BLAKE3 in `ds-core/src/hash.rs:33-57`.
- **Profili per ruolo**: `ds-db/src/profiles.rs` carica `L0-match/balance/profiles.json` con `include_str!` (`:21`, `:47-57`). Sei ruoli: `goalkeeper`, `centre_back`, `full_back`, `central_midfielder`, `wide_midfielder`, `striker` (`:158-170`). Formula: `mix = 0.4 * chiave_famiglia + 0.6 * chiave_caratteristica`; `valore = mean + spread * (2*mix - 1)`, clamp `[0.02, 0.98]` (`:86-94`). Piede forte segue `passing`, piede debole è una frazione del forte, `left_footed_share = 0.25` (`:95-110`).
- **Vault**: `ds-db/src/lib.rs:35-53`, con `identity = world_seed + match_id`, cache `BTreeMap<PlayerId, PlayerLatents>`, `roles`, `keys`, `profiles`. `latents_for` deriva al primo accesso e mette in cache (`:103-120`); `register_roles` va chiamato prima (`:68-76`); `register_keys` collega posto in campo e identità vera (`:78-83`).
- **L0, fixture**: `tools/ds-fixtures/src/lib.rs:45-115`, undici titolari 4-4-2 e cinque in panchina; sono test data, non produzione (`:1-10`).
- **L1, rose**: `L1-season/crates/ds-roster/src/lib.rs:18-37` ha `id`, `club`, `name`, `born`, `role`, `shirt`, `height_cm`, `weight_kg`; i latenti restano in `ds-db`. `make_squad` (`:42-84`) genera id `club*100 + numero`, età 17-35, nome da `pack.player_name`, corpo da `profiles.body`. `players.json` sovrascrive solo `name`, `role`, `born` (`:69-80`; file attuale `[]`). `PlayerEntry` in `ds-world/src/lib.rs:161-177` conferma quei tre soli campi.

### 1.4 Confine dei piani

- **Plane A → calcolatore**: `ds-calculator/src/lib.rs:52-118` espone solo 10 `Characteristic`: `ExplosiveForce`, `MaximalForce`, `Acceleration`, `MaxSpeed`, `GameReading`, `DecisionMaking`, `RiskManagement`, `Finishing`, `LeftFoot`, `RightFoot`. Porta minima `Custodian::value` (`:162-166`). `Datum::Height`, `Weight`, `Fatigue`, `Crowding`, `NearestThreat` sono Plane B (`:120-133`). Implementazione in `ds-db/src/custodian.rs:20-55`; `Acceleration` ritorna 0.0 perché non è una radice (`:49-52`).
- L'orchestratore ha accesso privato ai 30 latenti (`ds-orchestrator/src/lib.rs:1439-1443`) e un metodo pubblico `characteristic(player, name)` per i plugin (`:1197-1236`).
- **Plane B → viste**: `ds-demo/src/pitchview.rs:139-154` mostra lettera, nome, ruolo, altezza, peso; `ds-demo/src/viewmodel.rs:1-11` vieta accesso a stato interno e Piano A.
- **Plane C**: non implementato. Nessun `Markup`, nessuna stima con intervallo; solo feature names in `tools/ds-testkit/src/feature.rs:61,103,134,142`.

### 1.5 Test che fissano i vincoli

| Test | File e riga | Cosa impedisce |
|---|---|---|
| Derivazione latenti | `ds-db/src/latents.rs:225-297` | valori che cambino per ordine, mondo, giocatore o caratteristica |
| Profili ruolo | `ds-db/src/profiles.rs:158-221` | ruoli indistinguibili o non deterministici |
| Vault e chiavi | `ds-db/src/lib.rs:142-203` | giocatore che cambia valore tra casa/trasferta o tra letture |
| Custodian | `ds-db/src/custodian.rs:63-82` | `finishing` e piedi hardcoded a 0.0 |
| Anti-leak save | `ds-match/tests/cases/anti_leak.rs:15-23,61-85` | nomi dei latenti nel salvataggio |
| Anti-leak ADR-112 | `anti_leak.rs:130-215` | motore che chiami `latents_for(` o legga `.characteristic(` fuori da `World` e dalla card dichiarata |
| Golden L0 | `golden_l0.rs:15-44` | cambi di valori/formule senza aggiornare golden e `SIM_VERSION` |
| Scene d'oro | `golden_scenes.rs:1-21,39-66` | cambi che producano movimenti impossibili nelle stesse partite |
| Movimento | `ds-orchestrator/tests/movement.rs:23-66` | altezza, peso e fatica non modifichino come dichiarato accelerazione e falcata |
| Rose L1 | `ds-roster/tests/world_and_rosters.rs:16-32` | rose non deterministische |

`SIM_VERSION` è 222 in `ds-match/src/save.rs:93`; compatibilità a `:206`.

### 1.6 ADR citati nel codice

ADR-003, 017, 028, 029, 031, 034, 035, 041, 052, 063, 112, 113/114 sono citati nei file indicati dal rapporto originale. **ADR-036 non risulta citato in nessun `.rs`**: è dottrina, non codice.

## 2. Solo pianificato, debito o assente

1. **Distribuzioni realistiche**: `latents.rs:16-20` dice che i valori sono uniformi in `[0, 1)`; i veri sono campanulari con coda lunga; fattori, correlazioni e distribuzioni sono lavoro L3 (`L0.11 §11.1`). `profiles.rs:8-9` dice che i profili attuali sono il prototipo dichiarato del generatore L3. Oggi c'è "random ma sensato", non una ricostruzione statistica.
2. **Plane C / scouting / `Markup`**: assenti.
3. **Editor del giocatore**: `L3Editor` è solo un enum di feature in `ds-testkit/src/feature.rs:61,103,142`. Nessun comando, struct o API per scrivere un latente.
4. **Override del singolo giocatore**: `Vault` ha `register_roles` e `register_keys`, non `set_latents`/`override`. L'unico input numerico modificabile è il profilo comune del ruolo in `profiles.json`, che cambia tutti i giocatori di quel ruolo.
5. **Stati senza tratto**: `ds-model/src/player.rs:29-34` dichiara `confidence` e `morale` stati senza tratto ADR-028. Il commento su `fatigue` è datato: la fatica entra in `movement.json:20` e in `ds-orchestrator/src/lib.rs:1425-1436`.
6. **Consumatori parziali**: `injury_proneness` ha `#[allow(dead_code)]` in `latents.rs:59-63` ma è letto in `ds-orchestrator/src/lib.rs:1549`; manca il modulo infortuni (OI-60). `left_foot`/`right_foot` hanno commento "no consumer yet" in `latents.rs:103-106` ma sono già serviti da `custodian.rs:43-44` e dalla scelta del tiro; manca la formula del passaggio che li usi.
7. **Dati anagrafici L1 senza effetto**: `born` e `shirt` sono generati e salvati, ma `ds-roster/src/lib.rs:27` dice che l'età avanza senza effetto fino a L2.

## 3. Insieme minimo per "ricreare un giocatore reale" nel codice attuale

1. `PlayerId` stabile e `world_seed`: senza di essi i 30 valori cambiano (`hash.rs:33-57`, `latents.rs:165-212`).
2. Un ruolo, se si vuole usare il generatore per ruolo: è la chiave che seleziona `profiles.json` (`ds-db/src/lib.rs:110-119`, `profiles.rs:84-94`).
3. Corpo visibile: `height_cm` e `weight_kg`. Sono input di formula (`ds-calculator/src/lib.rs:120-133`; `movement.json:20`).
4. Stato al calcio d'inizio: `condition`, `match_sharpness`, `fatigue`, `cognitive_fatigue`, `damage`, `confidence`, `morale`, `pressure_load` (`ds-model/src/player.rs:75-96,131-160`).
5. Il vettore nascosto completo, o almeno le radici con consumatore: oggi è generato, non importato.
6. Coerenza del piede: forte e debole non due numeri uguali (`profiles.rs:95-110`; test `:212-221`).
7. Una chiave di identità stagionale se il giocatore compare in più partite: `register_keys` (`ds-db/src/lib.rs:78-83`; test `:142-173`).
8. Nessuna serializzazione dei latenti: ogni via di importo deve passare dal vault/orchestratore e rispettare l'anti-leak (`anti_leak.rs:61-85,130-215`).

Non sono imprescindibili nel codice attuale: il nome (display), il numero di maglia e l'anno di nascita (senza effetto fino a L2), il ruolo come etichetta *se* si forniscono già i latenti espliciti.

## 4. Cosa succederebbe mettendo valori reali domani

- Non c'è dove metterli: serve una nuova API controllata su `Vault`/`Profiles`; oggi l'unica sorgente è la derivazione per chiave.
- I test di derivazione e determinismo presumono che lo stesso seed dia sempre lo stesso valore: un override va modellato come parte della chiave/identità, non come stato mutabile.
- L'anti-leak vieta che i nomi dei latenti finiscano nel salvataggio e che il motore li legga: un importatore non può esporli alle viste.
- Ogni cambiamento di valori che altera le partite richiede `SIM_VERSION` bump da 222, rigenerazione del golden 42..51 e verifica delle scene d'oro.
- Il confine ADR-041/ADR-112 resta: formule in `ds-calculator`, valori in `ds-db`, decisione in `ds-orchestrator`. L'importatore sta a monte del vault, non dentro il motore.

Ho saltato: scrittura diretta del file da parte dell'agente (l'ho fatta io) e lettura del testo completo degli ADR fuori dal codice; la sezione 2 è dedotta dai commenti nei sorgenti.
