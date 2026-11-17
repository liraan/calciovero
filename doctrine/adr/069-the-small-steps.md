# ADR-069 — The small steps

- **Status:** Accepted (delegated, 2026-11-17) — to be reviewed by the maintainer
- **Date:** 2026-11-17
- **Decision:** DD-11, from decision BJ (`context/plans/DECISIONS-2026-11-15.md`); the unit approved by the maintainer on 17 November
- **Depends on:** ADR-048 §4 (the ball's physics as data), ADR-030 (movement), ADR-065 (the record)

*Written in Italian as the project's working language; the decision is binding as written.*

## Contesto

Il motore pensa e si muove 10 volte al secondo. La decisione BJ vuole che il movimento dei corpi e la fisica del pallone girino a 50 passi al secondo in tutte le partite, mentre le decisioni restano a 10. Serve perché un giorno un umano guiderà un calciatore con il pad, e la risposta deve arrivare in qualche centesimo di secondo. Senza un umano in campo, i passi piccoli muovono solo corpi e pallone.

Il motore lavorava in centimetri interi e velocità intere per tick. In un passo di 0,02 s la gravità vale 0,4 cm e un'accelerazione da 0,1 a 0,24 cm/passo: in centimetri interi si arrotondano a zero. Il maintainer ha approvato la proposta di un'unità interna più fine.

## LUNA

Il calcio è continuo: un corpo accelera e un pallone cade in ogni istante, non a scatti di un decimo di secondo.

## Partita vera

- **Come lo riconoscono gli attori:** il pallone che rimbalza all'altezza giusta, il difensore che arriva un passo prima, il pallone che esce o resta in campo di un soffio.
- **Da cosa dipende:** dal tempo fine del corpo e del pallone.
- **Chi lo produce:** i giocatori e la fisica; il passo di calcolo non decide nulla.

## Decisione

1. **Cinque passi piccoli per tick** (`SUBSTEPS`): 50 al secondo con i 10 tick al secondo della partita. Le decisioni, la percezione, il pensiero, la fatica e le regole restano per tick.
2. **L'unità interna è il centesimo di centimetro** (`FINE`). Il pallone e ogni corpo tengono la loro posizione e la velocità dell'ultimo passo in quest'unità tra un tick e l'altro. Tutto il resto del motore, e la registrazione (ADR-065, formato invariato), continua a leggere centimetri e centimetri per tick, arrotondati.
3. **Nessun numero nuovo:** ogni costante per passo si ricava da quella per tick.
   - **Velocità e accelerazioni:** le velocità per passo valgono un quinto di quelle per tick, le accelerazioni un venticinquesimo. Per la gravità: 10 cm/tick² sono 40 centesimi per passo², esatti.
   - **Attriti:** la quota che l'aria e l'erba lasciano per passo è la radice quinta di quella per tick, in milionesimi, calcolata una volta con `dmath`.
   - **Soglia di arresto:** 20 centesimi per passo.
   - **Rimbalzo:** la restituzione non dipende dal passo. Un rimbalzo più basso di un passo di gravità è un rotolare.
4. **Il bersaglio di un corpo si sceglie per tick, il corpo lo raggiunge per passi.** La frenata, la velocità voluta e la svolta si ricalcolano a ogni passo, dalla posizione di quel passo.
5. **Lo stato fine riparte dai centimetri** quando qualcuno ha messo il corpo o il pallone altrove (una ripresa, un calcio, una deviazione, la separazione dei corpi).
6. **Chi immagina il volo del pallone** (`ball_at_line`) usa gli stessi passi piccoli: quello che si prevede è quello che succede.

## Conseguenze

- **I golden sono rigenerati,** e restano uguali su glibc e su musl: l'aritmetica è intera o `dmath`.
- **Il rimbalzo da 2 m** ora risale dentro la banda FIFA del buon campo naturale (0,60-1,00 m). Con il tick di 0,1 s il passo tagliava parte della caduta.
- **Il costo:** circa 3 secondi a partita in release, con la percezione e il pensiero ancora a 10 al secondo.
- **La misura** (60 partite) è in `L0-CALIBRATION.md`: le rimesse salgono da 16,8 a 25,9 (vere 42,7) e gli angoli e i rossi si avvicinano al vero.
- **Non ancora (al 147):** il contatto con il pallone in aria (`meet_ball_in_air`) e la ricezione si controllavano ancora una volta per tick. Fatto al 148: vedi la nota datata.

## Note datate

- **2026-11-17 — i contatti a ogni passo piccolo (`SIM_VERSION` 148).** Il volo del pallone e la corsa di ogni corpo tengono dove sono alla fine di ognuno dei cinque passi del tick. Il contatto in aria si giudica al primo passo in cui il pallone passa alla portata di qualcuno, con ognuno dove era in quel passo. La ricezione si giudica tratto per tratto del cammino del pallone, contro dove era l'uomo alla fine del tratto. Con questo, una regola dello stesso tipo di "nessuno riceve il proprio passaggio": nessuno incontra in aria il pallone che ha appena giocato, finché un altro non lo tocca. Senza, il pallone colpito di testa era ancora alla testa di chi l'aveva colpito al passo dopo, e lui lo colpiva ancora. Nessun numero nuovo. I duelli aerei non cambiano (6,1 contro 38): OI-102 non viene dal tempo del contatto, resta il secondo uomo sul pallone alto, per la taratura.
