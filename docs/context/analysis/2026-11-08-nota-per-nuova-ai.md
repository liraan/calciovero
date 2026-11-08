---
title: "Nota per una nuova AI -- cose emerse in chat che non hanno trovato posto altrove"
date: "2026-11-08"
status: "analysis"
supersedes: null
---

# Nota per una nuova AI -- Calcio Vero

**Cos'e questo file.** Non e un handoff (quello e
`docs/context/analysis/2026-11-08-session-handoff-v6.md`, letto
dall startup ritual). Non e una regola (quelle sono in
`NEW-SESSION-PROMPT.md`). E la memoria delle cose emerse in
conversazione che non avevano un posto naturale nei documenti, e che
un agente nuovo non troverebbe leggendo solo il repo. Il maintainer
la mostra esplicitamente all'inizio di una sessione con un agente
nuovo; non e auto-caricata.

## 1. Il patto di collaborazione (direttiva del maintainer, 2026-11-08)

Il maintainer ha detto, testualmente: *"il progetto e nostro, non
mio, ci stiamo lavorando entrambi e per quanto riguarda il codice sei
tu l'esperto non certo io"*. Operativamente:

- Sul **codice** l'agente e l'esperto: decide le forme tecniche,
  sceglie le strutture, propone le soluzioni.
- Sul **modello** e sulle **priorita** decide il maintainer (P6:
  model choices and doctrine choices, which are the maintainer's).
- L'agente non deve chiedere il permesso per le scelte che una fonte
  gia decide (direttiva 2026-11-07). Deve chiedere quando la fonte
  tace o quando la scelta e del maintainer.
- L'agente non deve difendere una posizione quando il maintainer
  corregge (P7): riformula e prosegue.

## 2. I due filtri, obbligatori su ogni decisione di design

Non sono opzionali, non sono "quando il maintainer li chiede". Sono
il metro con cui si valuta ogni scelta di modello:

- **LUNA** (A3): *cosa rivela questo esempio che non sapevamo?* La
  risposta inizia con `LUNA:` e non cita identificatori di codice.
  Le conseguenze tecniche vanno in `DITO:` solo se richieste.
- **Partita vera** (D6): *come si riconosce questo fenomeno in una
  partita vera? da cosa dipende? chi lo produce?* Se il modello non
  puo rispondere, e incompleto o sbagliato.

I due filtri hanno salvato OI-88 in questa sessione (vedi §3). Sono
lo strumento, non la formalita.

## 3. La lezione di OI-88 (la trappola da non ripetere)

In questa sessione l'agente ha scritto OI-88 come `ShotIntent`
(potenza / piazzato / normale), una **categoria scelta**. Il
maintainer ha chiesto: *"le distinzioni di tiro rispettano gli
standard del progetto?"*. Non li rispettavano. L0.11 §4.6 e
esplicito: *"The catalogue has no categorical term. The foot is not a
stored label: `left_foot` and `right_foot` are two Plane A
performances, and the label is derived from them, not read by a
formula."*

La forma corretta e due **scelte continue** (`aim_point`, `power`);
`ShotType` resta **derivato** dalle due, per l'etichetta di Piano B
(il tabellino), non come input del motore. Il fenomeno era reale
(potenza e piazzato esistono in partita vera); la **forma** era
sbagliata (categoria vs continuum).

**La domanda da farsi, ogni volta:** *questa distinzione e una scelta
o una lettura di una scelta?* Se e una lettura, va derivata; se e una
scelta, deve essere continua, non un enum.

## 4. Decisioni orali del maintainer (non scritte altrove)

Prese in conversazione, non in un ADR. Vanno rispettate salvo
direttiva contraria:

- **Bump di versione dei crate: no, per ora.** La versione e `0.1.0`
  in tutto il workspace (`version.workspace = true`). Il maintainer
  ha detto: "non e urgente, aspettiamo ADR32"; l'idea era bumpare a
  `0.2.0` quando il rename `pcfm-*` -> `ds-*` atterra, cosi
  "versione nuova" e "prefisso nuovo" coincidono in un evento
  dichiarato. **Non bumpare senza una sua parola.**
- **ADR32 (rename `pcfm-*` -> `ds-*`) e in lista, non urgente, ma
  va fatto.** Testuale: *"il rename lo mettiamo in lista, non e
  urgente ma e da fare, tutto li"*. E gia `[open]` nel Plan con la
  sua prima azione (`ds-core` per primo, un crate per commit,
  `SIM_VERSION` invariato).
- **Codeberg e backup, GitHub e primario.** Testuale: *"github come
  principale e codeberg come backup, all'evolversi del progetto
  magari faremo la migrazione"*. La regola operativa e in
  `AGENT-POLICY.md` §6 ("Two remotes"): ogni push va a entrambi.
  Migrazione futura: decisione del maintainer, non ora.
- **Il dataset di statistiche reali per le caratteristiche dei
  giocatori** (IDEA-DESIGN-007) e un'idea, non una decisione.
  Prerequisito: leggere `pcfm-fixtures` (chi riempie altezza, peso,
  ruolo oggi). Senza quella lettura, qualunque proposta e
  un'invenzione (D1).

## 5. Cose che non hanno un OI ma vanno sapute

- **`resolve_duel` e ancora prototipo.** Legge un dado
  (`noise.mul_add(0.2, attacker_score) > defender_score`). La
  migrazione di ADR-035 non l'ha toccato. Non e tracciato come OI
  separato: e dentro Plan E (OI-50). Se un giorno sembra "strano che
  il duello abbia un dado", non e un bug nuovo: e debito noto.
- **Il tick gira il carrier verso la porta prima di `build_choice`**
  (`heading_from_to(pos, goal)` -> `am.heading = h`). Conseguenza: il
  `heading` del carrier e sempre verso il centro porta. E **OI-89**,
  ma se lo trovi nel codice e non ricordi perche, leggi OI-89 prima
  di "correggerlo".
- **`PassContext.receiver_freedom_cm` e un orfano** (OI-90): il
  commento dice "carried to the reception", nessuna funzione lo
  legge. `resolve_reception` usa `ReceptionContext.freedom_cm`, un
  campo diverso. Se lo trovi, non fidarti del commento.
- **Il conteggio dei test e sbagliato nei documenti.** Il repository
  ha **218** test; README, STRUCTURE, CONTRIBUTING e `specs/_index`
  dicono **215**. `A4-coherence` controlla i documenti fra loro, non
  col reale, quindi il gate e verde e il numero e sbagliato. Va
  corretto in un commit dedicato (non in chiusura).
- **`scripts/closing.sh` non scrive l'handoff.** Esegue C2, gates,
  capture. L'handoff (come v6) e un documento separato, scritto a
  mano quando la sessione lo merita. Se il maintainer lo chiede
  prima di "chiudi", va scritto **prima** del commit finale.
