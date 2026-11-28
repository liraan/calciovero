# ADR-122 — Il retropassaggio preso con le mani è punito (DD-102)

- **Status:** Accepted (delegated, 2026-11-28) — da rivedere da parte del maintainer
- **Date:** 2026-11-28
- **Decision:** DD-102
- **Depends on:** Law 12, Law 10, ADR-045, ADR-119, ADR-063
- **Amends:** ADR-119 (chiude il residuo T17 della punizione indiretta)

## Il problema

T22 dell'audit, incrociato con il residuo T17: `keeper.rs::keeper_meets_the_ball`
trattava ogni pallone che arrivava al portiere — tiro, deviazione, passaggio
deliberato di un compagno — come `Save` con `claim: true`. Il passaggio
deliberato al portiere preso con le mani, che la Legge 12 punisce con una
punizione indiretta, era invece raccontato come una normale presa: l'arbitro
non fischiava e la squadra avversaria non ripartiva.

## I filtri

- **partita vera:** il dito è il passaggio calciato da un compagno al proprio
  portiere, l'uso delle mani e il punto in cui il pallone viene preso; la luna
  è lo stesso portiere che ha visto il passaggio dei suoi e sceglie come
  giocare il pallone. La Legge 12 non sceglie al posto suo: dice solo che, se
  usa le mani su quel passaggio, il gioco riprende con un calcio libero
  indiretto per l'altra squadra.
- **LUNA:** il portiere resta lo stesso uomo con le sue capacità; non gli si
  impone il piede né gli si toglie il passaggio come opzione. Il fatto
  «passaggio deliberato a me» è letto dal volo ancora in corso, non da una
  posizione nominale o da un'etichetta decisa dal motore. La regola non
  aggiunge una scelta: agisce sull'uso delle mani che il motore già risolve.

## Decisione

1. `Keeper::is_back_pass_to` legge il `Pending` del passaggio in volo: non è un
   tiro, non è una ripresa, non è una rimessa, il ricevitore è il portiere e
   il passatore è della sua squadra. Nessun numero, gate o parametro nuovo.
2. Se il portiere prende il pallone con le mani — lo trattenga o lo parry —
   `call_back_pass_offence` fischia la punizione indiretta per l'altra squadra
   dal punto del contatto. Se il contatto è dentro l'area piccola della sua
   porta, il calcio si batte dalla linea dell'area, nello stesso punto in y,
   come prescrive la Legge 12. Nessun cartellino: il retropassaggio è un
   fallo tecnico, non una condotta scorretta (ADR-119 resta invariato).
3. La ripresa indiretta usa lo stesso battitore, lo stesso muro e la stessa
   attesa delle altre punizioni. Il suo calcio porta il flag `indirect` nel
   `Pending`: finché un altro uomo non tocca il pallone, un gol non conta e
   si esce con un calcio di porta (Legge 10). Il battitore può ancora scegliere
   il tiro: è la regola a non contare il gol, non un veto sulla scelta.
4. Il gemello `handling_a_back_pass_is_punished_but_the_same_claim_from_a_teammate_is_not`
   tiene fissi pallone, portiere e contatto e cambia solo il ricevitore del
   passaggio; il gemello `a_direct_shot_from_an_indirect_free_kick_is_not_a_goal`
   fa lo stesso sul calcio del battitore.

## Conseguenze

- `SIM_VERSION` 229, voce in `CHANGELOG-SIM.md`. Il nuovo `RestartKind::IndirectFreeKick`
  è una variante del canale già esistente `restart_pass`: serve a portare il
  fatto «indiretto» fino al calcio del battitore senza un secondo stato
  parallelo. Nessun evento, numero o gesto nuovo.
- Golden L0 base, istruito e coppa, semi e golden L1 rigenerati: il seed della
  coppa passa da 58 a 131. Misura `ds-reference 42..101` scritta in
  `L0-CALIBRATION.md` §229; ogni riga fuori banda conserva la sua causa,
  nessun numero preso guardando la banda (A5-bis).

## Limiti dichiarati

- Il motore risolve l'arrivo di un retropassaggio con le mani: non esiste
  ancora la scelta del portiere di giocarlo a piedi. La sanzione si applica
  all'uso delle mani che il motore compie già; non si inventa una scelta né
  si nasconde il fatto dietro un gate di percezione (ADR-062).
- La ripresa indiretta non ha un evento dedicato: è un `FreeKick` con
  `reason: "back_pass_handled"`, come gli altri calci liberi (ADR-024).

## Scartate

- **Aggiungere un evento `IndirectFreeKick`.** Nessun lettore lo chiede: il
  conteggio dei calci liberi resta uno solo e la qualifica è un campo
  dell'evento, non un tipo nuovo.
- **Impedire al portiere di prendere il passaggio con le mani.** Sarebbe un
  clamp sulla sua scelta con la scusa della regola: la Legge 12 punisce
  l'atto, non lo vieta (preferences-010).
- **Giudicare il passaggio dal solo ricevitore finale.** L'etichetta del
  ricevitore è già nel `Pending` del volo; leggere le posizioni vere dopo
  l'arrivo avrebbe introdotto una seconda verità.
