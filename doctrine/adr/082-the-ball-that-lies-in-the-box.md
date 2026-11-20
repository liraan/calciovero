# ADR-082 — The ball that lies in the box (OI-126)

- **Status:** Accepted — DD-22, maintainer chose A on 2026-11-20
- **Date:** 2026-11-20 (accepted 2026-11-20; attuazione dopo l'ADR angolare di OI-100)
- **Decision:** what a loose ball in the penalty area is *for the players*, and who goes to it
- **Depends on:** ADR-041 §5, ADR-058, ADR-051, ADR-049
- **Supersedes:** nothing

## LUNA

Il dito: nella 2.5D e nel TUI il pallino copre la palla, quindi sembra che i
giocatori le passino sopra. La luna: nessuno di loro la tratta come **propria**.
Un pallone a terra in area non è un oggetto in attesa che qualcuno lo raccolga
passando: è una situazione che ogni giocatore legge — chi ci arriva prima, se
un compagno è meglio piazzato, quanto vale per la squadra prenderla adesso. Il
10 rosso che si avvicina e si allontana e l'8 blu che l'attraversa senza
spostarla sono la stessa tendenza: **la palla non è di nessuno, quindi nessuno
agisce per averla**.

## Partita vera

Un tifoso riconosce che in area un pallone libero viene preso subito: chi è a un
metro lo controlla, chi è più lontano si accorge che lo prenderà un altro e si
sposta. Nessun difensore professionista vede una palla ferma a 1 m e torna
indietro. La misura del difetto è già nel motore: `ds-reference scene` seed 42
riporta *free slow ball taken at once* 85.7% (42 su 49) — 7 palloni lenti a
portata di un giocatore non sono presi al tick dopo. Non è un numero da
calibrare: è la traccia del comportamento sbagliato.

## La tendenza, non il copione

Nessun dado, nessun bonus, nessun numero nuovo, nessuno che legga il piano A.
La causa deve stare nel pensiero del singolo giocatore: dalla sua percezione
(chi è vicino alla palla, chi può arrivarci, dove sono gli avversari) e dalle
sue caratteristiche già presenti (reattività, lettura del gioco, velocità) deve
venire da sé la scelta di andarci. Il gemello del burattino deve reggere: se
cambiamo solo chi prende la palla senza cambiare ciò che il giocatore vede o
immagina, è un burattino e la decisione è rigettata (ADR-071, DD-19).

## Scelte di modello

1. **A — La palla libera è di chi legge di poterci arrivare.** Ogni giocatore se
   la vede come disponibile; chi giudica di arrivarci prima di ogni altro va a
   prenderla, gli altri leggono che ci va e si comportano di conseguenza. Usa la
   percezione e la portata già esistenti (`ball_reach_cm`,
   `loose_control_radius_cm`), nessun numero nuovo. Cambia la lettura, quindi
   regge il test del burattino.
2. **B — La palla libera resta di chi l'ha calciata.** L'obbligo di recuperarla
   resta alla squadra che ha toccato per ultima; gli altri la lasciano. È una
   regola di possesso, non una cosa che un giocatore vede: spiega meno la
   partita vera e rischia il burattino.
3. **C — La assegna il motore al più vicino.** Nessuna lettura, solo geometria:
   è esattamente il burattino vietato, si scarta.

## Raccomandazione

**A**, da attuare solo dopo aver verificato sul tick osservato se il blocco è
nella lettura del giocatore o in una ripresa in gioco (`waiting`): in quest'ultimo
caso la palla è fuori gioco e non è questa la causa.

## Conseguenze se accettata

Una causa alla volta, dopo la decisione ancora aperta sull'errore angolare del
tiro (OI-100): `SIM_VERSION` +1, voce in `CHANGELOG-SIM`, gemello del burattino,
golden, test mirato, `fmt`, `clippy`, `cargo test --workspace`, gate `closing`,
misura 42..101, aggiornamento della tabella autorevole. Nessun merge in `main`
senza autorizzazione.
