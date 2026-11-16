# ADR-066 — The knock that forces a substitution

- **Status:** Accepted (delegated, 2026-11-16) — to be reviewed by the maintainer
- **Date:** 2026-11-16
- **Decision:** DD-8, from decision Y (`context/plans/DECISIONS-2026-11-15.md`), point 1
- **Closes:** OI-110 (the knock)
- **Depends on:** ADR-035 (no dice), ADR-021 note (Law 12 applied objectively), ADR-063 (the plugins' contract)

*Written in Italian as the project's working language; the decision is binding as written.*

## Contesto

Nel motore un contatto può essere un fallo, con il suo cartellino, ma nessun contatto fa male a nessuno. La caratteristica `injury_proneness` esiste e non ha consumatori. La decisione Y del maintainer vuole il colpo che costringe al cambio in L0, letto dalla forza del contatto e dal corpo del giocatore, con la frequenza misurata sull'effetto come grado 2. Durata e tipo dell'infortunio restano a L1.

## LUNA

Un corpo che si rompe cambia la partita: la squadra perde un giocatore che non ha scelto di togliere.

## Partita vera

- **Come lo riconoscono gli attori:** il giocatore resta a terra, l'arbitro ferma il gioco, la panchina deve cambiarlo.
- **Da cosa dipende:** dalla forza del contatto e dal corpo di chi lo subisce.
- **Chi lo produce:** il contrasto, non il motore e non un dado.

## Decisione

1. **La forza del contatto** è la velocità con cui l'avversario arriva nel fallo, in millesimi della velocità di riferimento. È lo stesso fatto che la Regola 12 legge per il cartellino (`FoulFacts.approach_milli`).
2. **Il corpo la pesa:** forza × (0,5 + `injury_proneness`). Se il prodotto arriva alla soglia `knock_force_milli` (`balance/knock.json`), il giocatore non può continuare. Niente dadi: lo stesso contatto sullo stesso corpo dà sempre lo stesso esito. Il fatto è l'evento `Injury`, al suo tick, con chi l'ha causato e la forza.
3. **Alla prima interruzione esce.** Lo sostituisce un uomo della panchina del suo ruolo (il portiere con il portiere), altrimenti un qualsiasi uomo di movimento. Se i cambi sono finiti, o non c'è nessuno da far entrare, esce lo stesso e la squadra resta in dieci (Regola 3). Il cambio forzato viene prima di quelli previsti, ordinati o decisi dall'allenatore.
4. **La soglia è di grado 2, fissata sull'effetto.** L'effetto sono i cambi fatti prima del 45' non dopo un'espulsione della stessa squadra: nel calcio vero sono quasi tutti forzati. In Serie A 2017-18 (Wyscout) sono 0,19 a partita nelle partite con id pari e 0,25 in quelle con id dispari.
   - **Fissata:** sui 120 semi pari del motore (42-281), 0,19 colpi a partita nel primo tempo.
   - **Verificata:** sui 120 semi dispari, 0,19 contro lo 0,25 vero. Le due metà vere differiscono anch'esse di 0,06: è il campionamento, circa 0,035 per metà.
   - **Coerenza con lo studio UEFA sugli infortuni nei club d'élite** (Ekstrand, Hägglund e Waldén 2011): 27,5 infortuni ogni 1000 ore di partita, circa 0,9 a partita, quasi tutti da contatto. Non tutti costringono a uscire subito.
5. **Il contratto dei plugin** (`ds_match_api::World`) ha due metodi nuovi: `knocked` (gli uomini di una squadra fermati da un colpo, ancora in campo) e `carry_off`.

## Conseguenze

- `injury_proneness` ha il suo consumatore (RULES D3).
- Il colpo nasce solo dai falli fischiati. Oggi il motore ne fischia 13,9 a partita contro i 26 veri (OI-114): se i falli salgono, la soglia va rifissata con la stessa procedura, non ritoccata.
- I contatti senza fallo non fanno male a nessuno; il calcio vero ne ha (Ekstrand: circa metà degli infortuni da contatto non sono fallo). È un limite dichiarato, da riprendere con il modello dei contatti.
- Nel 2.5D l'evento `Injury` ha la sua riga nella tabella evento → animazione (ADR-065): il pallino resta a terra e si rialza piano.
- L1 riceve il fatto (chi, quando, la forza) e ne fa un infortunio con la sua durata.
