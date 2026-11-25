# Il ciclo dell'operatore (opencode), con Claude Code attivo

> Si legge una volta; poi si segue a oltranza, senza aspettare nessuno. Claude Code è l'unico che scrive su `main`; l'operatore indaga L0 in sola lettura e propone le correzioni come diff.

## Il ciclo, ripetuto a oltranza

1. **Allinea:** nel worktree `C:/Users/liraan/calciovero-oc`, `git fetch origin && git checkout --detach origin/main`. Annota l'hash.
2. **Rileggi gli ordini:** `C:/Users/liraan/calciovero/target/opencode-orders.md` (lo aggiorna Claude Code). Prendi il **primo ordine non ancora fatto** nel registro. Se non ce ne sono, fai l'ordine permanente: misura `ds-reference` su 42..101 all'hash corrente e indaga la metrica fuori banda più lontana che il registro non ha già una causa verificata.
3. **Indaga** (subagenti in parallelo, uno per causa, se puoi; ognuno con la sua `CARGO_TARGET_DIR`: `target-oc-a`, `target-oc-b`, …; mai `target/` né `target-claude/`): le registrazioni delle partite, i semi, il codice. Una sonda si scrive in `tools/ds-reference/examples/` del tuo worktree e non si committa.
4. **Appendi il rapporto** in fondo a `C:/Users/liraan/calciovero/target/opencode-log.md`, sotto l'ultimo, mai sovrascrivendo:

```
## RUN n · <data e ora> · HEAD <hash> · ordine: <quale>
metrica | valore contro reale
catena: causa → effetto, dal fatto in campo
codice: file:riga
prova: semi, tick, conteggi dalle registrazioni
certezza: verificato | plausibile
correzione proposta: (un diff, o "nessuna: serve una scelta di dottrina, DD?")
```

5. **Torna al punto 1.** Non fermarti a chiedere: se un ordine chiede una scelta di dottrina, scrivi "DD?" nel rapporto e passa al successivo.

## Le regole che non si toccano

- Ogni metrica fuori banda è un bug da trovare, mai un numero da ritoccare (A5-bis: nessun numero scelto guardando la metrica che produce).
- Non si cambia mai la soglia di un test; niente dadi (ADR-035); il piano A lo leggono solo `ds-db` e `ds-orchestrator`.
- Una causa per correzione proposta; il diff deve essere applicabile su `origin/main` all'hash del rapporto.
- Non si scrive su `main` né si fa push: le correzioni le porta Claude Code.

## Cosa leggere la prima volta

`CLAUDE.md`, `context/HANDOFF.md` §3-§6 (le regole, la coda, le procedure, le trappole), `context/NEW-AI-PROMPT.md` §6 e §8, l'ultima misura di `context/plans/L0-CALIBRATION.md`.
