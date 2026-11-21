# ADR-091 — Il posto scelto continua a guidare chi è senza palla (DD-32)

- **Status:** Accepted (delegated, 2026-11-21) — da rivedere da parte del maintainer
- **Date:** 2026-11-21
- **Decision:** DD-32
- **Amends:** ADR-071 (§5, il posto senza palla), ADR-030 (la fatica come dimmer)
- **Depends on:** ADR-035 (nessun dado), ADR-052 (ogni giocatore pensa),
  ADR-059 §4 (la disciplina del ruolo), ADR-089/090 (la frenata e il passo della
  partita vera)

## Contesto

Il maintainer, guardando la partita vera seed 42 dopo SIM 175: «continuano ogni
tanto a perdersi la palla e soprattutto la cosa più strana è il movimento senza
palla, i giocatori senza palla si muovono sembra senza una logica che dovrebbe
guidarli per il tema per esempio della fatica, invece di coprire spazi sembra
facciano solo movimenti dondolanti come automi». Poi precisa: «è piuttosto
comune, lo vedo quasi sempre come comportamento senza palla».

Misurato sul tick record della partita vera, 61.800 tick, prima del fix:

- **Il pallone piantato non è un guasto.** Tutte le 7 corse di 607 tick
  consecutivi con `carrier = None` finiscono con un `Goal`: 607 tick = 606 + 1,
  cioè i 60,6 s dichiarati di `goal_to_kick_off_s` a 10 tick/s. Le altre corse
  lunghe (49-65 tick) finiscono con `ThrowIn`, `Corner` o `FreeKick`: sono le
  riprese, non un pallone che nessuno raccoglie. Il rimettitore arriva già a
  41-45 cm dal punto, dentro i 90 cm di `at_ball`; il testo della sosta è dato
  dichiarato e non va modificato. Non c'è quindi un secondo bug da correggere:
  c'è una sosta legale già misurata nella partita vera.
- **Il dondolio è reale.** Contando le inversioni di marcia dei giocatori senza
  palla in gioco aperto, il peggiore giocatore di movimento ne fa 92 in 61.800
  tick, con passi di 2-3 cm: non sta coprendo uno spazio, sta ricalcolando e
  frenando a ogni decisione. I due portieri ne fanno 135 e 131, ma è il loro
  mestiere seguire l'angolo della palla a ogni istante.

Filtrando con LUNA: il dito è l'argmax senza memoria di `choose_closed_place`,
il `hold_at` scartato appena `carrier` è `None`, il raggio corpo di 45 cm e la
frena sul punto esatto; la luna è lo stesso giocatore, che legge la scena con la
propria fatica e le proprie gambe. Cambia il modo in cui esegue, il posto che
tiene e lo spazio che può ancora coprire: non cambia l'atto di calcio, né la
squadra, né il vincitore del pallone. La tattica dà solo la tendenza (pericolo e
tempestività); la scelta resta sua.

## Decisione

1. **Il posto letto vale anche mentre il pallone vola.** In
   `SpatialState::move_agents` il `hold_at` non è più filtrato da
   `self.carrier.is_some()`: prima, durante ogni passaggio in volo, il posto
   scelto veniva buttato via e sostituito dalla geometria della squadra
   (`block_target`), così un difensore che aveva scelto uno spazio seguiva il
   pallone e tornava al posto appena ricompariva un portatore. Il cambio di
   possesso continua a cancellare i posti (`set_home_in_possession`).
2. **Una rilettura vicino al posto tenuto è lo stesso spazio.** `set_hold_at`
   non sostituisce il posto se il nuovo è entro `BODY_RADIUS_CM` (45 cm,
   dichiarato). Nessuna nuova soglia: un corpo di scarto è già la misura con
   cui il motore separa due giocatori e con cui ADR-088/089 ferma chi arriva a
   una ripresa.
3. **La fatica entra nella scelta dello spazio, non solo nella velocità.**
   `TeamView::choose_closed_place` riceve la fatica del giocatore (dichiarata
   in `[0, 1]`) e sconta il valore di uno spazio in proporzione alla distanza
   dal giocatore, misurata in linee del blocco (`depth_out_of_possession_cm /
   3`, già dichiarato e già usato per le candidate). A fatica piena un
   difensore non va a cercare il pericolo oltre la propria linea: copre lo
   spazio che può ancora raggiungere. Nessun numero nuovo, nessun dado.
4. **Chi arriva si ferma entro un corpo del posto.** La frenata di chi non è
   in contesa si esaurisce a `BODY_RADIUS_CM` invece di continuare fino al
   punto esatto: senza questo scarto, la separazione spingeva il giocatore fuori
   e la frenata lo riportava dentro, ogni tick. Il portiere è escluso: lui
   deve seguire l'angolo con la palla a ogni istante. Le contese e i `sent`
   alle riprese hanno già la loro frenata (ADR-088/089), invariata.

## Verifica sulla partita vera

Confronto seed 42, SIM 175 → SIM 176: le inversioni di marcia del peggiore
giocatore di movimento scendono da 92 (61.800 tick) a 60 (63.200 tick, il 2,3%
in più), -35%; i due portieri vanno da 135 e 131 a 141 e 137, perché il loro
compito è seguire il pallone, non tenere un posto. I guardiani sono `fatigue_keeps_the_tired_man_in_the_space_he_can_reach`
(gemello: stessa scena, solo la fatica cambia) e `a_near_reread_keeps_the_place_he_holds`
(una rilettura entro un corpo non sposta il posto). I 175 test di libreria
restano verdi, compreso `move_agents_moves_the_goalkeeper_toward_the_ball_segment`,
che ha imposto l'eccezione del portiere.

## Scartate

- **Cambiare `goal_to_kick_off_s` o i tempi delle riprese:** sono il dato
  misurato della partita vera; il pallone fermo 60,6 s dopo un gol è il
  comportamento corretto, non un guasto. Nessun testo andava toccato.
- **Aggiungere una soglia di isteresi nuova:** il progetto vieta i numeri
  inventati; il raggio corpo dichiarato basta a dare continuità.
- **Rimuovere del tutto il `block_target`:** è la geometria di partenza del
  ruolo quando non c'è ancora un posto letto (ADR-071 §1): resta il fallback.
- **Far decidere al motore chi prende il pallone:** nessuna scelta è cambiata;
  il dondolio era esecuzione e memoria del posto, non agency.

## Conseguenze

`SIM_VERSION` 176; golden e semi dei test rigenerati. Le decisioni delle Leggi
del gioco restano in `plugins/referee.rs` (DD-31), senza modifiche.
