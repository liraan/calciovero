# Prompt per far riprendere Claude Code dopo l'operatore

> Da incollare in una sessione nuova di Claude Code. Pensato per spendere pochi token: si legge una pagina, non la storia.

---

Riprendi Calcio Vero. Leggi solo `context/REVIEW-QUEUE.md` (le righe "aperto"), la coda di `context/HANDOFF.md` §4 e il registro in fondo; poi `git log --oneline -15` e `git status`. Per ogni riga aperta guarda solo i file e il diff di quel commit (`git show <hash> --stat`, poi il diff dei file che contano): confermala ("ok") o correggila, e scrivi il nuovo stato nel commit del segnalibro. Le righe "DD?" me le porti in un elenco con la tua raccomandazione, senza applicarle. Poi riprendi la coda dove l'operatore l'ha lasciata. Non rileggere i piani interi se una riga non lo chiede. Usa subagenti in sola lettura per le revisioni indipendenti.
