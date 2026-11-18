# ADR-072 — Un atto, un evento: il cross è un passaggio

- **Status:** Accepted (2026-11-18)
- **Date:** 2026-11-18
- **Decision:** A3 del mandato (maintainer, 2026-11-18): "un atto, un evento; la qualifica è un campo, non un secondo evento". Sostituisce il doppio evento `Pass` + `Cross` emesso oggi per lo stesso pallone calciato.
- **Depends on:** ADR-065 (la registrazione è un contratto), ADR-051 (ogni gesto ha le sue radici), ADR-063 (la partita divisa in plugin).

*Scritto in italiano come lingua di lavoro del progetto; la decisione è vincolante come scritta.*

## Contesto

Per lo stesso pallone calciato il motore emette oggi due eventi nel chain:
`EventType::Pass` (da `classify`, `ds-match/src/fact.rs`, per `TrajectoryKind::Pass`)
e `EventType::Cross` (da `tell_cross`, `ds-match/src/plugins/technique.rs`,
quando `is_cross` è vero). Il racconto mostra entrambi; `ds-stats` conta `Pass`,
quindi un cross viene contato due volte. Un cross è un passaggio con un bersaglio
e un piede: è un atto solo, non due.

## Decisione

1. Un atto, un evento. `tell_cross` non emette più un `EventType::Cross` separato.
2. Il `Pass` porta la qualifica come campo: `"cross": true|false` e, quando vero, `"foot": "<nome>"`.
3. `ds-narrative` dice "crosses" se `"cross": true`, "passes" altrimenti.
4. `ds-stats` conta un passaggio una volta sola; i cross si leggono dal campo, non da un secondo evento.
5. `EventType::Cross` resta nell'enum, deprecato e non emesso da SIM >= 153. Nessun codice nuovo lo costruisce. Si rimuove solo con un cambio di contratto dichiarato in ADR-065.

## Conseguenze

Il golden cambia: bump di `SIM_VERSION`, `EXPECTED` rigenerato in
`ds-match/tests/golden_l0.rs`, voce in `L0-match/CHANGELOG-SIM.md`.
Il contratto dell'evento (ADR-065) tiene: il payload del `Pass` cresce,
la forma del chain no. Un test gemello lo protegge: lo stesso passaggio,
uno dentro l'area avversaria -> `"cross": true`, uno fuori -> `"cross": false`.

## Alternative scartate

- `classify` emette `Cross` al posto di `Pass`: un cross è un passaggio, non un tipo a parte; romperebbe il conteggio dei passaggi.
- Due eventi: viola "un atto, un evento" e gonfia il conteggio dei passaggi (B2).
- Rimuovere `EventType::Cross` adesso: rompe i log delle SIM precedenti; si tiene deprecato, si rimuove con ADR-065.
