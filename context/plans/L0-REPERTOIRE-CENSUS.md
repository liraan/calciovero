# L0 — censimento del repertorio tecnico (ADR-085, passo 1)

**Cos'è.** Il primo passo di ADR-085: per ogni riga `da censire`, ◐ o 👁 del
catalogo, dire dove sta nel codice oggi e quale radice già dichiarata deve
comandarla. Nessun codice nuovo, nessun `SIM_VERSION` +1: questo documento non
cambia il calcio, dice soltanto dov'è. Le radici sono quelle di
`L0-match/design/ACTIONS.md` (che usa `foot` e `risk` come scorciatoie di
`left foot`/`right foot` e `risk management`, e `→ jump` per il salto, che non è
una radice ma un ramo di explosive force, height e joint mobility).

**Stati.** **presente** = il gesto esiste ed è comandato dalle radici;
**parziale** = esiste una parte del gesto, ma manca il dito o l'esecuzione;
**assente** = nessun ramo nel codice; **invisibile** = esiste nel motore e
nell'evento, ma nessun osservatore (narrazione, 2.5D) lo mostra.

**Filtro LUNA.** Ogni riga eredita da ADR-085 il dito che cambia e la luna che
resta: il catalogo è della popolazione, ogni giocatore ne vede ed esegue un
sottoinsieme secondo le proprie radici.

Censimento statico del sorgente: prova che il codice esiste, non quanto spesso
il gesto accade in partita. La frequenza si misura dopo, con 42..101.

## 3. Ricezione e primo controllo

| Gesto | Dove sta | Stato | Radice | Nota |
|---|---|---|---|---|
| Controllo col piede (interno, esterno, suola) | `ds-orchestrator/src/lib.rs:692 resolve_reception`; `ds-match/src/state.rs:91 FEET_BAND_CM` | parziale | first touch, foot, body control | La difficulty del primo controllo è generica; il piede come dito non è modellato. |
| Controllo di petto | `ds-orchestrator/src/lib.rs:692 resolve_reception`; `ds-match/src/state.rs:91 FEET_BAND_CM` | assente | first touch, body control | Il petto compare solo in un commento nella banda sotto 1 m; nessun ramo. |
| Controllo di coscia o ginocchio | `ds-orchestrator/src/lib.rs:692 resolve_reception`, `:874 control_spill`; `ds-match/src/state.rs:91 FEET_BAND_CM` | assente | first touch, body control | Nessun modello coscia/ginocchio; lo spill è agnostico al dito. |
| Controllo di testa | `ds-match/src/plugins/technique.rs:207 meet_ball_in_air`, `:49 aerial_contenders`, `:114 header_at_goal`, `:319 tell_pass_met_in_air` | parziale | first touch, heading, body control | Testa e duelli esistono; manca la ricezione controllata di testa (il pallone alto è completato, tirato o spazzato via). |
| Controllo di prima | `ds-orchestrator/src/actor.rs:169 first_time`; `ds-match/src/plugins/technique.rs:544 plays_first_time` | parziale | first touch, game reading, passing, decision making | Si decide solo controllo vs spazzata; il tiro e il passaggio di prima sono esplicitamente lavoro futuro (OI-100). |
| Controllo in corsa, smorzando (OI-123) | `ds-match/src/plugins/technique.rs:687 receives_loose`; `ds-match/src/spatial.rs:1369 follow_carrier`; `ds-orchestrator/src/lib.rs:874 control_spill` | parziale | first touch, dribbling, acceleration | Esistono controllo di palla libera e spill in movimento; manca il modello del passo che smorza. |
| Ricezione girandosi | `ds-orchestrator/src/actor.rs:169 first_time`; `ds-match/src/spatial.rs:1369 follow_carrier` | assente | first touch, body control, joint mobility | Nessun ramo di ricezione con rotazione del corpo. |
| Ricezione a terra, schiacciando | `ds-orchestrator/src/lib.rs:692 resolve_reception`, `:874 control_spill`; `ds-match/src/plugins/technique.rs:851 control_loose_ball` | parziale | first touch | Il controllo a terra uccide la velocità con `first touch`; manca il dettaglio della suola. |
| Spazzolata via, deviazione | `ds-match/src/plugins/technique.rs:544 plays_first_time`, `:687 receives_loose`, `:1003 deflects_the_pass` | parziale | first touch, passing | Spazzata e deviazione esistono, non come passaggio intenzionale con un tocco. |
| Turnover reception | nessun simbolo `turnover`; il più vicino `ds-match/src/plugins/technique.rs:687 receives_loose`, `:851 control_loose_ball`, `:1003 deflects_the_pass` | assente | first touch, game reading, reactivity | Le palle libere dopo contatti e intercetti sono gestite in modo generico. |

## 4. Passaggio e costruzione

| Gesto | Dove sta | Stato | radice | Nota |
|---|---|---|---|---|
| Passaggio corto | `ds-orchestrator/src/read.rs:1427 ChoiceKind::Pass`; `ds-orchestrator/src/lib.rs:586 resolve_pass` | presente | passing, foot, vision, game reading, risk | Scelta, peso, precisione e immaginazione della ricezione. |
| Passaggio lungo, cambio di gioco | `ds-orchestrator/src/lib.rs:586 resolve_pass`, ramo lofted | presente | passing, foot, explosive force, game reading | Il passaggio lungo è il passaggio a pallone alto. |
| **Passaggio filtrante nello spazio** | `ds-orchestrator/src/read.rs:1528 through_candidates` → `:1582 ChoiceKind::Through`; `choice.rs:34-39` e `:181-193` (visibilità da `creativity`); `ds-match/src/plugins/thought.rs:221-224`, `:473`; `ds-match/src/fact.rs:120-122`; `ds-match/src/state.rs:185-186`, `:318-319` | **invisibile → visibile in SIM 168** | game reading, vision, passing, decision making, risk | Generato dal SIM 79, qualificatore `through: true` nell'evento. Prima della modifica `ds-narrative/src/lib.rs:142-147 pass_verb` controllava solo `cross`; `tools/ds-gui/src/app.rs:1353-1434 gestures` non leggeva `through`; `tools/ds-gui/src/animation.rs:43` mandava ogni `Pass` a `KickOrHeader`. L'unico osservatore che lo nominava era `ds-match/tests/repertoire.rs:89,160`. Ora narratore e 2.5D leggono il qualificatore; la scelta è intatta. |
| **Colpo di tacco** | nessun simbolo `heel`/`backheel` | assente | passing, first touch, body control, game reading | Nessuna scelta, nessun evento, nessun test. |
| Passaggio di esterno, curva | `ds-match/src/spatial.rs:781 natural_foot_at`; `ds-orchestrator/src/lib.rs:586 resolve_pass` (piede/errore/deriva `:604-650`) | assente | passing, foot, body control | L'esterno del piede è solo la scelta del piede rispetto al lato del pallone; nessuna curva. |
| Passaggio schiacciato, rasoterra | `ds-orchestrator/src/read.rs:1427 ChoiceKind::Pass`; `ds-orchestrator/src/lib.rs:586 resolve_pass`, ramo non lofted `:678-680` | parziale | passing, foot | Esiste il passaggio rasoterra generico; manca il gesto schiacciato e il suo qualificatore. |
| Passaggio a muro, uno-due | nessun simbolo one-two/give-and-go; il più vicino `ds-orchestrator/src/read.rs:1427`, `:1528` | assente | passing, game reading, decision making | Non c'è uno stato di restituzione: il passatore sceglie solo ricevente o punto d'arrivo. |
| Passaggio a terzo uomo | `ds-match/src/plugins/thought.rs:202-211 scene.runs`; `ds-orchestrator/src/read.rs:1528 through_candidates` | assente | passing, game reading, vision | Le corse avanti sono visibili al costruttore del filtrante; manca la catena del terzo uomo. |
| **Testata filtrante da palla alta** | `ds-match/src/plugins/technique.rs:207 meet_ball_in_air`, `:319 tell_pass_met_in_air`, `:114 header_at_goal` | assente | heading, first touch, game reading, vision | Un passaggio alto può essere vinto di testa e marcato completato, ma non esiste evento né gesto di passaggio/flick di testa. |
| Passaggio di petto, di spalla | `ds-orchestrator/src/lib.rs:586 resolve_pass`; `ds-match/src/fact.rs:54 TrajectoryKind::Pass` | assente | first touch, passing, body control | La traiettoria del passaggio è solo piede/mani (`foot`, `throw`). |
| Distribuzione del portiere: rotolata | `ds-match/src/plugins/keeper.rs:71-74 in_hands`; `ds-match/src/plugins/thought.rs:409-420` (throw/kick); `ds-match/src/plugins/set_pieces.rs:229 GoalKick taker` | assente | passing, foot, vision, game reading | Esistono lanci dalle mani e rinvio passato; manca il posare a terra e rotolare. |

### La catena del filtrante, per intero

1. `read.rs:1528-1586` costruisce i candidati e spinge `ChoiceKind::Through { to, at }` (`:1582`).
2. `choice.rs:181-193` modula la visibilità di `Through`/`Loft` con `creativity`: è il filtro individuale del repertorio, già attuato.
3. `thought.rs:221-224` traduce la scelta in `ChosenAction::Pass { at: Some(at) }`; `:473` conserva `through: at.is_some()`.
4. `fact.rs:120-122` emette `EventType::Pass` con `{ "complete": ..., "through": true, "cross": ... }`; `state.rs:185-186` e `:318-319` lo trasportano.
5. Fino al SIM 167 nessun osservatore lo nominava: il gesto c'era, chi guarda
   non lo vedeva. Dal SIM 168 `pass_verb` dice «threads it through»,
   l'importanza è `Medium` (il commento lo mostra anche senza «every touch») e
   `gestures` in 2.5D assegna il nuovo `scene::Gesture::Through` dal solo
   qualificatore dell'evento.

Il primo intervento concreto è fatto: il filtrante è visibile senza cambiare la
scelta del giocatore. Restano la misura su 42..101 e il giudizio visivo del
maintainer.

## 5. Dribbling e battere l'uomo

| Gesto | Dove sta | Stato | Radice | Nota |
|---|---|---|---|---|
| Conduzione a testa alta | `ds-calculator/src/movement.rs::touch_period_ticks:139`; `ds-match/src/spatial.rs:1799` | presente | dribbling, run, vision | Il passo del portatore è scalato da dribbling. |
| Take-on, scarto in corsa | `ds-match/src/plugins/duels.rs:125,140 beaten_by_dribble`; `ds-orchestrator/src/lib.rs:231-238 resolve_duel`; `ds-calculator/src/duel.rs:230-238 answer_force` | presente | dribbling, explosive force, body control | **Ciò che il motore chiama `dribble`: esito di duello**, non gesto; l'evento ha solo `{ "past": slot }`. |
| **Doppio passo (finta, scissor)** | `ds-match/src/plugins/duels.rs:140 beaten_by_dribble` | assente | dribbling, body control, explosive force | Nessuna finta: l'unico modo di battere l'uomo è il `TakeOn` generico. |
| Finta di spalla, di passo | `ds-orchestrator/src/choice.rs:26 ChoiceKind` | assente | body control, dribbling, game reading | Nessuna variante di finta. |
| Roulette, girata su sé stesso | `ds-orchestrator/src/choice.rs:26 ChoiceKind`; `ds-calculator/src/movement.rs` (joint mobility → velocità di rotazione) | assente | dribbling, body control, joint mobility | La mobilità articolare comanda la velocità di svolta, non un gesto. |
| Cruyff turn, drag-back | `ds-orchestrator/src/read.rs:972 shift_candidate` | assente | dribbling, first touch, body control | `Shift` è un tocco sul piede forte, non un tirare indietro. |
| Nutmeg, pallone tra le gambe | `ds-match/src/plugins/duels.rs:140 beaten_by_dribble` | assente | dribbling, first touch, vision | Nessun passaggio tra le gambe. |
| **Controllo aereo + scarto** | `ds-match/src/plugins/technique.rs:207 meet_ball_in_air`, `:245 Duel{aerial:true}` | parziale | first touch, body control, dribbling, heading | Il duello aereo e il portare giù ai piedi esistono; manca lo scarto dopo il controllo. |
| Cambiamento di ritmo | `ds-calculator/src/movement.rs::touch_period_ticks:139`; `ds-match/src/spatial.rs:1799` | parziale | dribbling, explosive force | Accelerazione e passo del portatore esistono; non c'è l'accelerare dopo la finta. |
| Proteggere il pallone | `ds-calculator/src/duel.rs:220-228 answer_force` (`Shield`) | presente | maximal force, body control, first touch | `tests/repertoire.rs:62` lo elenca in `NOT_YET` come movimento visibile, non come scelta. |
| Girarsi con il pallone | `ds-match/tests/repertoire.rs:41 NOT_YET ("turn")` | parziale | dribbling, body control, joint mobility | La velocità di svolta esiste ma la direzione cambia istantaneamente e non è visibile. |

Il dribbling oggi, per intero: il portatore sceglie `CarrierAnswer::TakeOn` o
`Shield` in `resolve_duel`; se vince con `TakeOn`, `beaten_by_dribble` mette il
difensore in `OffBallAct::Hold` ed emette `EventType::Dribble { past }`. Non c'è
nessuna scelta su **come** l'uomo è stato battuto: per questo tutte le righe
finta/ritmo/svolta sono assenti, non nascoste.

## 6. Tiro e finalizzazione

| Gesto | Dove sta | Stato | Radice | Nota |
|---|---|---|---|---|
| Tiro di piede | `ds-orchestrator/src/lib.rs:440 resolve_shot`; `ds-orchestrator/src/outcome.rs:42 ShotType` | presente | finishing, foot, explosive force, composure | `finishing` è il nome del documento, `shooting` il latente nel codice. |
| **Tiro di prima** | `ds-match/src/plugins/technique.rs:544 plays_first_time`; `ds-orchestrator/src/actor.rs:161` | assente | first touch, finishing, game reading | Esiste solo la spazzata di prima: il tiro di prima è SIM 166, provato e tolto (OI-100). |
| Tiro con il piede debole | `ds-match/src/spatial.rs:781 natural_foot_at`; `ds-orchestrator/src/lib.rs:604-650` | presente | finishing, foot | Il piede modulo errore e deriva. |
| Volée | `ds-orchestrator/src/context.rs:68 ShotContext.contact_height` | assente | finishing, first touch, body control | `contact_height` è solo il contatto basso/alto di un tiro da terra; il pallone in aria si colpisce di testa o si porta ai piedi. |
| Mezza volée | `ds-match/src/plugins/keeper.rs:80`; `ds-calculator/src/obstacle.rs:55` | assente | finishing, first touch, body control | I rimbalzi sono fisica; non c'è secondo contatto con scelta di tiro. |
| Testata in porta | `ds-match/src/plugins/technique.rs:114 header_at_goal`; `tools/ds-gui/src/animation.rs:44` | presente | heading, → jump, game reading | `ShotType::Header`, visibile. |
| Testata tuffata | `ds-match/src/plugins/technique.rs:114 header_at_goal` | parziale | heading, → jump, body control, courage | La testata c'è, il corpo non si tuffa: la posa `Diving` è solo del portiere. |
| Rovesciata, overhead | `ds-orchestrator/src/outcome.rs:42 ShotType` | assente | finishing, body control, explosive force, courage | Ogni tiro di piede è battuto da un portatore a terra. |
| Pallonetto sul portiere | `ds-orchestrator/src/lib.rs:440 resolve_shot`, `ShotType::Lob` | presente | finishing, passing, game reading | Solo come lob, visibile. |
| Tiro curvo attorno al muro | `ds-orchestrator/src/outcome.rs:42 ShotType` | assente | finishing, foot, body control | Nessuna curva o rotazione nel tiro; le punizioni non aggirano il muro. |
| Seguito di respinta | `ds-match/src/plugins/keeper.rs:80`; `ds-calculator/src/obstacle.rs:55` | assente | finishing, reactivity, game reading | Le respinte sono fisica; non c'è una decisione sulla seconda palla. |
| Rigore, punizione | `ds-orchestrator/src/lib.rs:586 resolve_pass`; `ds-match/src/plugins/set_pieces.rs` | presente | finishing, set pieces, foot | Vedi però l'avvertenza su `set pieces` in fondo. |

Nessun gesto di questa famiglia è presente-ma-invisibile: tutto ciò che il
motore produce è già coperto da `tests/repertoire.rs`. I due atti invisibili che
il test traccia, `turn` e `shield` (`NOT_YET:41,62`), sono stati inclusi nelle
famiglie 5 e 1.

## 1. Il corpo (tutti)

| Gesto | Dove sta | Stato | Radice | Nota |
|---|---|---|---|---|
| Camminare, correre, sprintare | `ds-match/src/spatial.rs:1560 move_agents`; `ds-orchestrator/src/lib.rs:1027 resolve_movement` | presente | max speed, explosive force, weight; fatigue | — |
| Accelerare, ripartire | `ds-calculator/src/movement.rs` | presente | explosive force, acceleration | — |
| Frenare, fermarsi | `ds-calculator/src/movement.rs` | presente | body control | — |
| Girarsi, cambiare direzione, pivot | `ds-calculator/src/movement.rs`; `ds-match/tests/repertoire.rs:41 NOT_YET ("turn")` | parziale / invisibile | joint mobility, body control, weight | La direzione cambia istantaneamente; nessun frame visible. |
| **Saltare, atterrare** | `ds-match/src/spatial.rs:1560 move_agents`; `ds-orchestrator/src/lib.rs:507 aerial_reach_cm`, `:1027`; `ds-match/src/record.rs:37-47 AgentFrame` | parziale / invisibile | explosive force, height, joint mobility | Il salto è un bonus statico di portata in aria; nessun evento di salto o atterraggio, nessun campo verticale nel frame. |
| Tuffarsi, cadere, rialzarsi (fuori campo) | `ds-match/src/plugins/duels.rs:95-99,140-157`; `ds-calculator/src/duel.rs:66-67 slide_recovery_ticks` | assente come gesto | body control, courage | Il tackle mancato allunga solo il cooldown (`down`); nessun tuffo o evento. |
| Reggere i 90 minuti | `ds-orchestrator/src/lib.rs:1107-1140 resolve_latency`; `ds-db/src/latents.rs` | presente | endurance | — |
| Tuffo del portiere (confronto) | `ds-calculator/src/save.rs:60-71 reach_cm`, `:115-122` (gemelli); `ds-match/src/plugins/keeper.rs:24-140` | presente-ma-invisibile | reactivity, explosive force, → jump, handling | La portata cambia con le radici, ma `Save` ha solo `{x,y,z,held}`; il "prepararsi" è in `NOT_YET:54-56`. |

## 2. Gli occhi e la mente

| Gesto | Dove sta | Stato | Radice | Nota |
|---|---|---|---|---|
| Guardarsi intorno, scandagliare | `ds-orchestrator/src/read.rs` (cono visivo) | presente | vision, game reading | — |
| Leggere il gioco, anticipare | `ds-orchestrator/src/read.rs` | presente | game reading | — |
| Decidere | `ds-orchestrator/src/lib.rs:1107-1140 resolve_latency` | presente | decision making, concentration, crowding, fatigue | — |
| Gestire il rischio | `ds-orchestrator/src/choice.rs` | presente | risk | — |
| **Concentrarsi, distrarsi** | `ds-orchestrator/src/lib.rs:1135-1136 resolve_latency`; `ds-db/src/latents.rs:161-163` | parziale / invisibile | concentration | Moltiplica solo la latenza; nessun evento di distrazione, nessuna traccia nel record. |
| Comunicare (gridare, gestire) | `ds-match/src/plugins/bench.rs:417-480` (solo allenatore→squadra via `Instruction`); `ds-model/src/squad.rs:120` | assente (giocatore→giocatore) | **nessuna radice dichiarata** | L'ADR mette «—»: prima di aggiungere il gesto serve una decisione, non un numero inventato. |

## 7. Difendere, liberare, fermare

| Gesto | Dove sta | Stato | Radice | Nota |
|---|---|---|---|---|
| Spazzata di piede, di testa | `ds-match/src/plugins/technique.rs` | presente | passing, explosive force, heading | — |
| **Salvataggio sulla linea** | `ds-match/src/plugins/technique.rs:411 block_shot`; `ds-match/src/plugins/ball.rs:208 check_goal_line`; `ds-match/src/plugins/keeper.rs:76-137` | assente | reactivity, body control, courage | Esiste la spazzata, non il corpo sulla linea; `block_shot` vale solo sotto `off.block_height_cm`. |
| Intercetto | `ds-orchestrator/src/offball.rs:156-190 defend` | presente | game reading, reactivity, sprint | — |
| Blocco di tiro o passaggio | `ds-match/src/plugins/technique.rs:411 block_shot` | presente | reactivity, first touch, courage | — |
| Contrasto in piedi, jockey | `ds-orchestrator/src/offball.rs:156-190 defend`; `ds-calculator/src/duel.rs` | presente | tackling, body control, discipline | — |
| Contrasto in scivolata | `ds-calculator/src/duel.rs:66-67 slide_recovery_ticks`; `ds-match/src/plugins/duels.rs:95-99` | presente | tackling, explosive force, body control | Il costo è un cooldown, non un movimento visibile. |
| Presa, respinta | `ds-orchestrator/src/outcome.rs:82-89 SaveOutcome`; `ds-match/src/plugins/keeper.rs:60-139` | presente | handling, body control | — |
| **Spill del portiere** | `ds-calculator/src/control.rs:11-13 spill_speed`; `ds-orchestrator/src/lib.rs:872-877 control_spill`; `ds-match/src/plugins/technique.rs:1044-1062`; `tests/repertoire.rs:58-60 NOT_YET` | assente per il portiere | handling, body control | Lo spill esiste solo come primo controllo del giocatore di movimento, e legge *first touch*. |
| **Pugno** | `ds-match/src/plugins/keeper.rs:60-139`; `ds-orchestrator/src/lib.rs:730-749 resolve_save` | assente | handling, explosive force | Il `tipped` è una deviazione sulla traversa, non un pugno scelto. |
| Posizione del portiere | `ds-match/src/spatial.rs:1600-1634 move_agents`; `ds-calculator/src/keeper.rs:16-18 line_depth_cm`; `ds-match/src/balance.rs:199 KeeperBalance` | parziale | game reading, concentration | L'angolo usa `foreshight(gk)`. |
| Prepararsi | `ds-orchestrator/src/lib.rs:1107-1140 resolve_latency`; `tests/repertoire.rs:54-56 NOT_YET` | assente | reactivity, concentration | È conflata nel tempo di reazione; nessuna posa. |
| Uscita, uscita su cross | `ds-orchestrator/src/offball.rs:84-93 Place`, `:156-163 may_leave`, `:212-219 Challenge`; `ds-match/src/plugins/thought.rs:689-711 keeper_first` | parziale / invisibile | decision making, courage, → jump, handling | Esce solo per un pallone che ritiene di prendere prima; nessun evento dedicated. |

## 8. Gioco aereo

| Gesto | Dove sta | Stato | Radice | Nota |
|---|---|---|---|---|
| **Duello aereo, salto in timing** | `ds-match/src/plugins/technique.rs:49-111 aerial_contenders`, `:207-313 meet_ball_in_air`; `ds-orchestrator/src/lib.rs:507-542 aerial_reach_cm`; `ds-calculator/src/duel.rs:268-279 aerial_force`; `ds-match/src/plugins/thought.rs:672-688` | parziale: duello sì, timing no | heading, → jump, maximal force, height, game reading, courage | Il vincitore è l'argmax di `aerial_force` statico; il coraggio decide solo se andarci. Nessuna scelta di timing. |
| Testata di contesa | `ds-match/src/plugins/technique.rs:269-312` | presente | heading, → jump, game reading | Tre rami: in porta, via, giù ai piedi. |
| **Flick di testa, deviazione alta** | `ds-match/src/plugins/technique.rs:269-312` | assente | heading, first touch, game reading | Nessuna scelta di direzione di deviazione né contatto di nuca. |
| Contesa su rimessa, angolo, punizione | `ds-match/src/plugins/technique.rs:207-313`; `ds-match/src/plugins/set_pieces.rs:30-58,77-137,270-361`; `ds-match/src/plugins/thought.rs:662-725` | presente-ma-invisibile | heading, → jump, game reading | La contesa generica vale anche dopo le riprese; `Duel` porta solo `aerial:true`, senza contesto di ripresa. |

## 9. Senza palla (la lettura che muove)

| Gesto | Dove sta | Stato | Radice | Nota |
|---|---|---|---|---|
| Tenere il posto | `ds-orchestrator/src/offball.rs` | presente | game reading, discipline | — |
| Pressare | `ds-orchestrator/src/offball.rs:156-190 defend`; `ds-match/src/fact.rs:35-47 RelationEnterExit` | parziale / visibile come `Pressure` | aggressiveness, work rate, game reading, sprint | La salienza è letta; manca una quantità di intensità/durata. |
| Coprire, rientrare, stringere | `ds-orchestrator/src/offball.rs` | presente | game reading, work rate, run | — |
| Marcare, seguire l'uomo | `ds-orchestrator/src/offball.rs:192-198,318-322`; `ds-match/src/plugins/thought.rs:781-810 to_mark` | presente-ma-invisibile | game reading, concentration, discipline, sprint | Atto fuori palla che muove il giocatore; nessun evento o campo frame. |
| Offrirsi per il passaggio | `ds-orchestrator/src/offball.rs` | presente | game reading, decision making | — |
| Attaccare la profondità | `ds-orchestrator/src/offball.rs:24-27,337-348 OffBallAct::Run` | presente | game reading, sprint | — |
| Check back, rientrare sul piede forte | `ds-orchestrator/src/choice.rs:70-74 ChoiceKind::Shift`; `ds-match/src/plugins/thought.rs:324-349` | assente come gesto | game reading, decision making | `Shift` è un tocco sul piede forte e un passo verso il centro, non il corpo che torna verso il pallone. |
| Triangolare, terzo uomo in corsa | `ds-orchestrator/src/choice.rs:32-39 ChoiceKind::Through`; `ds-match/src/plugins/thought.rs:202-211 scene.runs` | parziale: ingredienti | passing, game reading, vision | Il filtrante per un compagno in corsa c'è; non c'è la scelta di catena uno-due/terzo uomo. |
| Fuorigioco, tenere la linea | `ds-match/src/plugins/rules` | presente | game reading, discipline | — |

## 10. Le riprese

| Gesto | Dove sta | Stato | Radice | Nota |
|---|---|---|---|---|
| Calcio d'inizio, rimessa, rinvio, angolo | `ds-match/src/plugins/set_pieces.rs:201-263 restart_taker` | presente | role, passing, set pieces | — |
| **Rimessa lunga, corta** | `ds-match/src/plugins/set_pieces.rs:231-243`; `ds-match/src/plugins/thought.rs:411-431`; `ds-orchestrator/src/lib.rs:586-650`; `ds-calculator/src/kick.rs:28-31` | assente come scelta | passing, foot | La distanza è solo quella del ricevente scelto; solo il lancio dalle mani del portiere ha un limite dedicato. |
| **Angolo interno, esterno** | `ds-orchestrator/src/lib.rs:619-650 resolve_pass`; `ds-match/src/plugins/technique.rs:33-43 compute_cross`; `ds-match/src/fact.rs:52-62 TrajectoryKind` | assente | passing, foot, body control | Nessun curve/spin: la deriva del passaggio è errore di contatto, non cross a curva. |
| Punizione, rigore | `ds-orchestrator/src/lib.rs:586 resolve_pass`; `ds-match/src/plugins/set_pieces.rs` | presente | finishing, set pieces, foot | — |
| Schemi di calcio piazzato (L1) | `ds-match/src/plugins/set_pieces.rs:451-557 form_wall/release_wall`, `:270-361 clear_the_spot`, `:201-263` | parziale: solo muro difensivo | game reading, passing, vision | Il muro legge portata del portiere, porta, distanza, corpo; nessuna routine offensiva. |

## Il portiere non cambia identità

Verifica esplicita richiesta da ADR-084:

- `Place` è `Outfield | Keeper { ball_in_area }` (`ds-orchestrator/src/offball.rs:84-93`), costruito in `ds-match/src/plugins/thought.rs:789-797` da `keeper_id = squad.players.first()` (`:648`). È un **fatto della scena**: gates `may_leave`, esclude press/mark/cover/intercept e sceglie il challenge.
- `in_hands: Option<PlayerId>` è uno **stato del pallone** (`ds-match/src/state.rs:299-301`), posto su una parata trattenuta (`keeper.rs:71-74`) o su una presa alta (`technique.rs:269-279`), rimosso quando gioca il pallone (`thought.rs:271`).
- Con la palla il portiere passa dallo stesso `Actor::decide` / `ChoiceKind` di tutti (`thought.rs:212-241`). L'unica eccezione: un passaggio corto non lofted diventa lancio (`thought.rs:409-414`; `lib.rs:596-623`; `kick.rs:28-31`).
- **Verdetto:** non cambia identità. Stessi latenti, stessa scelta; `Place::Keeper` è lettura della scena e `in_hands` è il possesso. Nel codice il campo si chiama `goalkeeping`, nel catalogo `handling`.

## Avvertenze emerse dal censimento

1. **`set pieces` non è una radice dichiarata** nel modello attuale: il composite prototipo è stato rimosso (`ds-db/src/latents.rs:31`), benché `ACTIONS.md` lo usi ancora. Non si aggiunge una radice inventata: va deciso a parte prima di scrivere gesti che la usino.
2. **Comunicare non ha radice dichiarata** (riga «—» in ADR-085): oggi esiste solo allenatore→squadra. Aggiungere il gesto giocatore→giocatore richiede una decisione, non un numero nuovo.
3. **`finishing` nel catalogo è `shooting` nel codice**, `foot` è `left/right foot`, `→ jump` è un ramo di explosive force + height + joint mobility: sono nomi diversi della stessa radice, non radici nuove.
4. Il censimento è statico: dice dove il codice esiste, non con quale frequenza il gesto accade. La frequenza si misura su 42..101 dopo ogni intervento.
