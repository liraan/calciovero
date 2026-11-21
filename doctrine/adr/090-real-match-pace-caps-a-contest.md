# ADR-090 — Chi contesta corre come nella partita vera, non a tutta velocità (DD-30)

- **Status:** Accepted (2026-11-21)
- **Date:** 2026-11-21
- **Decision:** DD-30
- **Amends:** ADR-089 (DD-29, SIM 174: il freno vale anche per il portatore)
- **Depends on:** ADR-035 (nessun dado), ADR-051 (la scena letta), ADR-054 (il contatto non è visibile)

## Contesto

Il maintainer, guardando la partita vera alla 2.5D, osserva: «i giocatori non
corrono quasi mai alla loro massima velocità», e chiede: «se ci sono statistiche
in tal senso implementale». La statistica c'è, dichiarata:
`L0-match/balance/fatigue.json` `match_profile` è una distribuzione
velocità (cm/tick) → quota di tempo: 15 cm/tick per il 60%, 35 per il 25%, 50
per il 13%, 75 per il 2%. Finora serviva solo a derivare la fatica; la velocità
di chi contesta un pallone restava `top_s`, il massimo del giocatore.

Sulla partita vera seed 42 (SIM 174), su 3775 campioni di pallone fermo e
giocatore quasi fermo, 13 (0,3%) correvano oltre 60 cm/tick, con un picco di
90 cm/tick: più del massimo dichiarato di 75 cm/tick. Il 90 era
`effort_ref_cm_per_tick`, il riferimento dello sforzo pieno, non una velocità
mai vista nella partita.

## LUNA

Il dito: la distanza dal pallone, `top_s`, lo stride, il tick, il picco di
75 cm/tick del profilo dichiarato e il riferimento di 90 cm/tick.

La luna: lo stesso giocatore, con la propria velocità massima, la propria
reattività e la propria lettura della scena. Non gli viene cambiato l'atto
(va ancora a prendere il pallone), né chi lo prende, né il modo in cui legge il
contatto: gli viene solo tolta la velocità che in partita vera non tiene quasi
mai. La quota è un rapporto, non un numero assoluto, quindi resta diversa per
ogni giocatore: l'agency è nel come esegue, non nel quale gesto.

## Decisione

1. **La quota deriva dal dato dichiarato.** `FatigueBalance::load()` calcola
   `contest_pace_share = max(match_profile speed) / effort_ref_cm_per_tick`,
   limitato a `[0, 1]`. Con i numeri dichiarati è 75/90 = 0,833. Nessun numero
   nuovo, nessun dato inventato.
2. **Vale solo per le contese.** In `spatial::move_agents`, ramo `contests`,
   `contest_top = top_s * contest_pace_share`. Sia il caso semplice sia la
   frenata di ADR-088/089 (riprese e portatore con tocco scaduto) usano
   `contest_top`. Chi occupa il proprio spazio (`in_time`) e chi porta il
   pallone (`carrying`) resta invariato.
3. **Il gemello del burattino regge.** A scena uguale il giocatore sceglie lo
   stesso atto e lo stesso vincitore del pallone; cambia solo la velocità
   dell'arrivo. Il guardiano `a_contest_runs_at_the_real_match_sprint_share`
   misura lo spazio per tick e falliva prima del fix (95 cm/tick contro un
   limite di 80 cm/tick).

## Verifica sulla partita vera

Confronto seed 42, SIM 174 → SIM 175, sugli stessi campioni: picco 90 → 74
cm/tick (sotto il massimo dichiarato 75), casi oltre 60 cm/tick 13 → 7
(0,3% → 0,1%). Il giocatore resta identificabile per slot e per atto; nessun
evento di scelta cambia per il fatto della quota. Il cap esclude chi è `sent`
al punto di una rimessa: si posiziona, non contende; applicarlo anche lì
fermava il battitore a 97 cm dal punto (fuori dai 90 cm di `at_ball`), con il
pallone morto 1510 tick su seed 42 contro la soglia di 1212. Golden rigenerati
con `SIM_VERSION = 175`.

## Scartate

- **Media del profilo (25,75 cm/tick):** porterebbe tutti i contrasti a un
  passo, cancellando la corsa che la partita vera mostra il 2% del tempo e la
  differenza tra giocatori.
- **Tetto assoluto di 75 cm/tick:** `top_s` è per sotto-passo e dipende dal
  giocatore; un numero fisso appiattisce le identità.
- **Abbassare `effort_ref_cm_per_tick`:** è il riferimento dichiarato dello
  sforzo, non una misura della partita; cambiarlo altererebbe anche la fatica.

## Conseguenze

`SIM_VERSION` 175, golden e semi dei test aggiornati; la struttura delle regole
del gioco resta in `plugins/referee.rs` (vedi DD-31), senza cambio di
comportamento da quel lato.
