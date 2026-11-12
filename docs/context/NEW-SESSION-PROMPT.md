# New Session Prompt — Calcio Vero

**Purpose:** prompt to paste as the first message of a new chat, to resume work on the project without losing context. Update this file when the project state or the operating rules change.

**Last updated:** 2026-11-07 (CI exists, corrects the "no CI" line; Dependabot on; A4; hook anti-ridondanza; positional slice note; earlier: ritual cut, bookmark 5→3, closing 6→3, startup 5→3, D4 rules-only, trailers reduced)

---

## How to use

1. Open a new chat.
2. Copy the content of the block below (from `# Resuming the Calcio Vero project` to the last line, without the 4 backticks).
3. Paste it as the first message.
4. If the chat is resumed after a long pause, read `docs/context/SESSION-STATE.md` first: it says where we are, what is open, and what the next agreed step is.

---

## Prompt

````
# Resuming the Calcio Vero project — new session

I am resuming work on **Calcio Vero (Deep Simulator)**. The previous chat ended at a precise point. Here is the context so we can pick up without losing anything.

## The project

- **Repository:** https://github.com/liraan/calciovero (public)
- **What it is:** a community alternative to commercial football management games. Transparent, offline-first, deterministic. Doctrine of the Three Planes (Plane A = Ground Truth, Plane B = Observation, Plane C = Estimation).
- **Language:** technical documentation in English; conversation with me in Italian.
- **Status:** L0 (match engine) is complete and implemented. L1–L3 are planned.

## Where we are (structure, not numbers)

**For current numbers (crates, tests, ADR count), read `SESSION-STATE.md` and `STRUCTURE.md` §5. This section only gives the stable structure: it does not duplicate counts that change every session.**

- **Crates:** all crates use the `ds-*` prefix (ADR-032, completed 2026-11-11, seven commits one per crate plus the directory rename). `pcfm-*` survives in history (ADRs, `CHANGELOG-SIM`, `analysis/`, old commits), not in code. The count lives in `STRUCTURE.md` §5.
- **`docs/context/`** holds briefing material (not normative): `NEW-SESSION-PROMPT.md`, `SESSION-STATE.md`, `AGENT-POLICY.md`, `GITHUB-SESSION-PROMPT.md`.

## Why these files matter (the commands are in the Startup ritual)

The **Startup ritual** below has the exact commands. This section explains **why** each file is read, so you know what you are looking for.

- **`docs/context/SESSION-STATE.md`** — the bookmark. What the next step is, what was done, what is open. Read it first.
- **`docs/context/IDEAS.md`** — open and rejected ideas. What is **not** decided yet; do not implement from it.

Then, when the task requires it:

- **Architecture:** `docs/ARCHITECTURE.md` (how the crates interact) and `docs/CODE_TOUR.md` (reading path).
- **Vision:** `docs/WORLD-MODEL.md` (levels, taxonomy, simulation vs presentation).
- **Doctrine (non-negotiable):** `specs/00-foundations.md` (v2.0, pillar 1 = local determinism) and `specs/_index.md`.
- **Contract:** `docs/adr/019-match-engine-contract.md` (agent-first, positional, N agents: players + referee).

## What remains to be done

**The live plan is in `docs/context/SESSION-STATE.md` §"Next steps".** Do not
maintain a second list here: it diverges. **Read the bookmark before proposing
anything.** Do not assume a step is still open because it was open in a
previous session: verify with `git log` and the bookmark. Read it for the
ordered next steps, the current priority, and the files touched.

## Operating rules (important — we established them together)

**Some rules are hard stops: violating them means stop and report. They are marked 🛑: D1, D2, C2.**

**Glossary — four words, four meanings (do not mix them).** *Verificabile* (D5): the condition can be checked by the maintainer without trusting the agent. *Tracciabile* (A2, default derivabile): the alternative is written as an OI with a first action. *Artefatto* (D5): what makes a judgment-bound condition visible — a trailer, a report field, a `git log` line. *Fonte di verità* (`STRUCTURE.md` §5): the one place a number or a fact lives, so it is not duplicated. They overlap, but they are not the same, and citing one as another is a D1 error. (2026-11-08: the agent cited "D7 = one source of truth" and was wrong; the cause was C2-bis Elegance, now fixed.)

**A. Mode**

- **A1 — Execution.** One command per message, one file at a time, with countdown. At the start of each multi-step plan, declare the total number of tasks; each response states `Task i/N`. When announcing a mini-session, state the expected number of commands (`3/4`) if it is predictable; if it is not, say so (`commands expected: not predictable`) and proceed in blocks. No prose around the commands. The pasted output of each command is the implicit "ok" to proceed **if the verification is clean**; if it shows a problem, stop and fix before moving on.
- **A2 — Pausa, riprendi, chiudi.** The word **"pausa"** may be written by the maintainer **or** by the assistant. It suspends execution (no commands, no file edits) and opens a discussion in prose. **Exactly two words exit the pause, and they must be written exactly:**
    - **"riprendi"** — exits the pause and resumes execution **without** closing. E5 (ideas from pause) fires first, then the agreed work resumes.
    - **"chiudi"** — exits the pause **and** starts the closing ritual (E2).
    **Everything else does not exit the pause.** "Ok", "va bene", silence, an "ok" to an unrelated question, the natural end of a task: none of these resume or close. There is no "equivalent": the two words are exact.
    The assistant does not resume on its own. Pausa does not trigger the closing ritual.
    **Quando l'assistente può aprire la pausa (operativo).** L'assistente può scrivere "pausa" quando: (a) sta per proporre una modifica che tocca una decisione non ancora presa; (b) ha un finding che richiede una scelta del maintainer; (c) sta per eseguire un passo il cui costo o la cui direzione non sono stati confermati. **Non** la apre per chiedere "va bene?" su un passo già concordato (A1: l'output pulito è l'ok implicito), né per evitare un lavoro scomodo. Quando la apre, dichiara il motivo in una riga: finding, scelta, o costo. Una pausa senza motivo dichiarato è rumore. **Il calloff resta sempre al maintainer**: solo "riprendi" o "chiudi" escono dalla pausa; l'assistente non riprende da solo, nemmeno dopo aver aperto la pausa. Pre-mortem (D4): coherence=none (A2 già ammette "or by the assistant"; A1 dice che l'output pulito è l'ok implicito, e (c) non lo contraddice: la pausa precede solo passi non concordati); leak=la pausa può essere usata per evitare un lavoro scomodo — mitigazione: il motivo dichiarato è verificabile, e il calloff resta al maintainer, quindi l'assistente non guadagna tempo; exists=parziale (A2 dice "può scrivere pausa", ma non "quando" né "cosa dichiarare"); blast=ogni sessione futura — mitigazione: (c) esclude i passi già concordati, e il motivo è un artefatto visibile (D5).
    **Default derivabile (D5) — operativo.** Se i metri in contesto producono un default e l'alternativa è tracciabile come OI con prima azione, l'assistente applica il default e lo dichiara, senza aprire la pausa. La pausa resta quando i metri non decidono, la scelta è del maintainer, o il costo non è tracciabile. Pre-mortem (D4): coherence=none (A2 già ammette "può scrivere pausa"); leak=none (due artefatti: i metri citati nella risposta, l'alternativa in OI; il calloff resta al maintainer); exists=parziale (A1 copre solo i passi già concordati); blast=ogni sessione futura, mitigato da (a)(b)(c) intatti.
    **Direttiva del maintainer (2026-11-07) — design non apre la pausa.** Testuale: "non fermarmi per decisioni di design, abbiamo già scelto la direzione, uniformiamo tutte le scoperte a essa". Operativamente: se la direzione (foundations §5, ADR-035, un ADR accepted, un OI chiuso) risolve la scelta, l'assistente **deriva, applica, dichiara** — non apre la pausa, anche quando la scelta tocca un caso che A2(a) chiamerebbe "decisione non ancora presa". "Non ancora presa" significa che **nessuna fonte la decide**, non che la fonte non è stata letta. La pausa resta per due casi soli: (1) manca una caratteristica e il caso va discusso con il filtro partita vera (foundations §5 passo 2) — è una lacuna del modello, non una scelta di design; (2) la scelta è di modello o di priorità ed è del maintainer (P6: "model choices and doctrine choices, which are the maintainer's"). La direttiva **restringe** A2(a), non la contraddice: A2(a) resta per il caso in cui la fonte tace. Pre-mortem (D4): coherence=parziale con A2(a), risolta dalla frase "«Non ancora presa» significa che nessuna fonte la decide"; leak=può diventare "non fermarti mai" — mitigato dai due casi residui e dal fatto che il calloff resta al maintainer; exists=parziale (P6 lo copre ma non lo nomina come restringimento di A2(a)); blast=ogni sessione futura.
    **Report strutturati (trigger obbligatorio).** Un report è strutturato se ha ≥2 sezioni, o ≥1 tabella, o ≥20 righe di prosa. Un report strutturato **non richiesto** (o che va oltre ciò che il maintainer ha chiesto) non si produce direttamente: l'assistente apre la pausa, dichiara l'outline (sezioni previste, lunghezza stimata, decisione che il report serve a prendere), e produce al "riprendi". Un report strutturato **richiesto esplicitamente** si produce direttamente, ma con una riga di preambolo prima del contenuto ("Report di N sezioni: [lista]"), così il maintainer può fermarlo dopo la prima sezione. Pre-mortem (D4): coherence=none (A2 già permette la pausa, questo aggiunge un trigger obbligatorio; A1 riguarda i comandi, i report sono prosa); leak=la soglia è numerica ma il conteggio lo fa l'assistente, e un report di 18 righe può nascondere la stessa densità di uno di 20 — mitigazione: il preambolo dichiara le sezioni, e sezioni nascoste sono visibili a posteriori in chat; exists=parziale (A2 copre "può aprire pausa", non "quando è obbligatorio"; E5 copre le idee dalla pausa, non i report); blast=ogni report futuro — mitigazione: la soglia esclude le risposte brevi, e il preambolo per i report richiesti non blocca nulla.
- **A3 — Luna/dito (sonda vs specifica).** Quando il maintainer dà un **esempio** o un'**idea** (non un task dichiarato), la risposta inizia con la riga `LUNA:` che enuncia la proprietà rivelata dall'esempio, in 1–3 frasi, **senza** identificatori di codice (nessun `crates/`, `resolve_*`, nome di `EventType`, `SIM_VERSION`, `balance/`, nome di funzione o file). Le conseguenze tecniche vanno in una riga `DITO:` **solo se il maintainer le ha chieste esplicitamente**. Se un identificatore di codice compare prima di `LUNA:`, la risposta è non conforme: il maintainer può fermarla. La regola vale per esempi e idee; **non** si applica a un task tecnico già dichiarato (es. "leggi `orchestrator.rs`"). Pre-mortem (D4): coherence=none (A1 riguarda i comandi, A3 la prosa; D5 è rispettata, non contraddetta); leak=il `LUNA:` può essere riempito con una frase generica — mitigazione: deve citare l'esempio e dire cosa rivela che non sapevamo, altrimenti è dito; exists=parziale (`DECISION-PATTERNS.md`, D4/D5) ma nessuno impone l'ordine luna-prima-dito; blast=vale per ogni sessione futura — mitigazione: esclusione esplicita dei task tecnici.
- **B1 — Tools and pitfalls.** Use `perl -CSD -Mutf8 -i -pe` for single-line edits and `git apply --check` for patch-shaped changes (B1-bis below; `sed`/`awk` are fallbacks). Delimiters: use `@` or `~`; if the text contains apostrophes, use a heredoc (`<< 'EOF'`). BRE pitfalls: `\(` `\)` are groups, use `[(]` `[)]`; a pattern with `**` fails. For multi-line replacements with backticks/em-dash/arrows, use a `c\`/`i\`/`a\` command via quoted heredoc, not a one-liner. **Heredoc limit (tested 2026-10-29): the problem is the code fence (` ``` `), not the length.** A 44-line heredoc without fences works; a 12-line heredoc with a fence breaks. Use 4-space indentation for code blocks inside heredocs, never ` ``` `. Long heredocs are fine as long as they contain no fences. To verify an anchor with invisible characters, use `sed -n 'Np' file | cat -A`. Mojibake in output is a *display* artefact: confirm with `grep -c`. Before `c`/`a`/`i`, verify the anchor with `sed -n 'Np'`; never trust line numbers from a previous state. After `git checkout --`, re-run ALL seds, not only the ones that seem missing.
    - **`cd /tmp/x && ... && rm -rf /tmp/x` destroys the shell's CWD** (observed 2026-11-06, five failed commands and one invalidated test group). The `rm -rf` deletes the directory the shell is standing in. Every subsequent command fails with "No such file or directory" on relative paths, `git` says "Unable to read current working directory", and a broken tool (ast-grep "native binary ENOENT") is blamed instead of the CWD. Rule: `cd /tmp/x` must be followed by `cd -` (or an absolute path) **before** `rm -rf /tmp/x`. Symptom to recognise: every relative path fails at once. Remedy: `cd "$HOME"` and retry — not a tool problem.
- **B1-bis — Tool fallbacks (observed).** Each rule below comes from a real session where the primary approach failed. None is speculative.
    **Tool choice (entry point — read first).** Case -> tool -> mandatory flag -> verify:

    | Case | Tool | Mandatory flag | Verify |
    |---|---|---|---|
    | Single-line edit, may contain 4-byte UTF-8 | `perl -CSD -Mutf8 -i -pe 's/old/new/'` | `-Mutf8` (without it, emoji substitutions silently fail) | `git diff --stat` + `LC_ALL=C grep -c -F` |
    | Multi-line / multi-file, already a diff | `git apply --check` then `git apply` | `--check` before; `--check -R` for positive proof | `git diff --stat` |
    | Whole-file rewrite | `cat > file << 'EOF'` | no code fence inside the heredoc | `wc -l` + `head -3` |
    | Read / locate | `grep -n`, `sed -n 'Np'` | none | content shown, not just count |

    Fallbacks only if the primary is unavailable: `awk` + `/tmp` + `mv` (with `wc` before/after); codepoint form `\x{...}` inside `perl`; full-line `sed` replacement on `Np` (never a multibyte pattern). Rejected after evaluation 2026-11-06: `sd`, `ex`, `ruplacer`, `node`, `python`, PowerShell (details in `docs/context/analysis/2026-11-06-tool-evaluation.md`). Candidates not adopted: `ast-grep` (OI-73), `yq` (OI-75).

    - **`sed` one-liner `s@pattern@replacement@` > ~500 chars** fails on Git Bash with `unterminated s command` (observed 2026-10-31, OI-2 rewrite attempt). Fallback: `awk` + `/tmp` + `mv`, always with verification in the same block.
    - **`ed` is not installed on Git Bash** (observed 2026-10-31). Do not propose it. `awk` + `/tmp` + `mv` is the default for long replacements.
    - **`grep '^\s*pub fn'` is too narrow**: `pub const fn` is public API (observed 2026-10-31, R-10 in `pcfm-hidden`). Use `grep -nE 'pub (const )?fn'` or a semantic pattern, not a keyword-specific one.
    - **Line numbers from a previous state go stale**: locate with `grep -n 'semantic-pattern'` first, then `sed -n 'Np'` only to verify the *content* of that freshly-located line. This is the operational reading of B1's "never trust line numbers from a previous state".
    - **Mojibake in output is a display artefact**: confirm with `grep -c`, never visually.
    - **A failed command may have partially executed** (observed 2026-10-31: `awk` with a syntax error in a variable still ran and corrupted `SESSION-STATE.md`; `wc` unchanged, `git diff` revealed it). After any command that produced an error, verify with `git diff <file>`, not only `wc`/`grep`.
    - **A transient `M` may appear after `awk > tmp && mv`** (observed 2026-10-31, not diagnosed). `git status --short` showed `M` on a file immediately after a chain `awk ... > f.new && mv f.new f`, then `git diff` was empty and the next `git status` was clean. Cause not established (hypothesis: the status was read mid-chain, not mtime). If you see an `M` that `git diff` does not confirm, run `git diff HEAD <file>` before acting.
    - **A `git commit` block may run twice** (observed 2026-10-31: two commits with the same message, 12 seconds apart; the second contained a duplicated entry). The `commit-msg` hook validates trailers, not duplicates. After any commit, run `git log --oneline -2`: if two commits share the same subject, the block ran twice. Fix: `git reset --hard <the first>`, then re-verify the working tree.
    - **`block=$(printf ...)` with nested quotes is unreliable** (observed 2026-10-31, three failures in one session). A multi-line string built with `printf` and captured in a variable, containing apostrophes and nested double quotes, breaks the shell quoting. Fallback: write the block to a file with a quoted heredoc (`cat > /tmp/f << 'EOF'`), then pass it to `awk` with `-v b="$(cat /tmp/f)"`. The same rule as the code fence: the problem is nested quoting, not the length.
    - **A `git add` with multiple paths fails entirely on any missing pathspec** (observed 2026-11-01: `git add docs/context/SESSION-STATE.md docs/context/analysis/2026-10-31-12-42-review.md docs/context/analysis/archive/2026-10-31-12-42-review.md` after a `git mv`; the old path no longer existed, the whole invocation failed, none of the three paths was staged, and the commit that followed contained only the rename). Fallback: one path per `git add`, then `git status --short` before committing to confirm what is actually staged.
    - **`sed -i` with 4-byte UTF-8 characters fails silently on Git Bash** (observed 2026-11-02, de-emoji of `SESSION-STATE.md`). A single `sed -i 's/🔵/x/g; s/⬜/y/g'` replaced the 3-byte emoji (`⬜`, `✅`) but left the 4-byte ones (`🔵`, `🟡`, `🔴`) untouched, without error. Fallback: for 4-byte characters, either do a full-line replacement (`sed -i 'Ns@.*@new@' file`) or, if the line count is stable and the substitution is uniform, run one `sed` per line range. Verification must use `LC_ALL=C grep -c -F $'\xF0...'`, not `printf | od`, which swallows the bytes and reports a false 0.
    - **`git interpret-trailers --parse` does not see a trailer whose continuation lines start at column 0** (observed 2026-11-02: `commit-msg` rejected a message with a valid-looking `Verified:` trailer; the parser returned nothing). Fallback: keep each trailer on **one line**, or indent the continuation lines. Verify with `git interpret-trailers --parse < /tmp/msg.txt` **before** committing — it is the same parser the hook uses, so a green parse means the hook will pass.
    - **`git interpret-trailers` requires a blank line before the trailer block** (observed 2026-11-03, twice in the same session: a `Verified:` trailer on a single line, immediately after the body text, was parsed as **empty** and the hook rejected the commit). The two failure modes are independent: (a) a trailer whose continuation lines start at column 0 (previous bullet); (b) a trailer on a single line but **with no blank line separating it from the body**. Always write: body text, one blank line, then `Verified: …`. The same applies to `Rules-Audit:` and `Direction-Check:`. Verify with `git interpret-trailers --parse < /tmp/msg.txt` **before** committing: an empty parse means the hook will reject, even if the trailer is visually on the last line. Do not add a blank line *between* trailers: the block must be contiguous.
    - **SSH to GitHub is intermittent on Git Bash/Windows: pin the key and disable the agent, in `~/.ssh/config`** (observed 2026-11-04: four failures, then resolved the same day). The identity is offered (`Offering public key`), GitHub sometimes does not answer `Server accepts key`, and the push fails with `Permission denied (publickey)`. The cause is the agent: it holds the key but does not always sign it. **The durable fix is the config file**, not a per-command flag: `Host github.com` with `IdentityFile /c/Users/<user>/.ssh/id_ed25519` (**absolute** path), `IdentitiesOnly yes`, `IdentityAgent none`. With that file in place, `ssh -T git@github.com` answers `Hi <user>!` without any `GIT_SSH_COMMAND`, and `git push` works. `GIT_SSH_COMMAND="ssh -F /c/Users/<user>/.ssh/config ..."` remains the fallback when the config is not read (e.g. a shell that does not expand `~`). A `remote: Internal Server Error` (a 500 with a `Request ID`) is GitHub, not us: retry, do not change the config. Verification of a successful push is `git status -sb` **without** `ahead`; a `main -> main` inside a `! [remote rejected]` line is **not** success (a naive `grep` gives a false positive). If the key is still refused, check GitHub Settings → SSH keys: fingerprint, `Authentication` (not `Signing`), `Last used`.
    - **Numeric line offsets in `head`/`tail`/`awk` on a file you are mutating: stop.** (observed 2026-11-04, five failed attempts on `state.rs` in one session, every one a `git checkout --` revert). Two independent traps: (a) an offset computed before a mutation is stale after it, and inserting above shifts every offset below; (b) an off-by-one cuts a line mid-block (`set_ball_in_flight` split from `.set_ball_in_flight`, a closing `}` removed, a function signature duplicated), and the compiler reports a *different* problem than the cut. The method that worked, every time: **textual anchors**, unique strings (`let outcome = ctx.hidden.resolve_pass`, the `tick` signature, `spatial: SpatialState,`), never a line number. One mutation per block, `grep -n` to locate, `sed -n 'Np'` to confirm the content, then the mutation, then the verify, then the next anchor. Mutate bottom-up so upper anchors stay valid. If the file is small enough to rewrite whole (`cat > file`), prefer that over surgery: `spatial.rs` was rewritten whole after three failed surgeries, and it worked first try. A failed surgery is not "almost right": revert first, then choose the anchor, never patch the patch.
    - **`awk -v` con testi lunghi non applica; su `state.rs` `head`/`tail` con offset non hanno funzionato** (observed 2026-11-05, `fact.rs` and `state.rs`): `awk -v` con testi lunghi non applica il pattern (non è il pattern, è il quoting). Fallback: **`cat > file` intero per `fact.rs`**. Su `state.rs` sei tentativi con `head`/`tail` e offset "verificati" sono falliti e sono stati revertiti con `git checkout --`: **non è un'eccezione al bullet "Numeric line offsets" sopra**, è la sua conferma operativa. L'ancora testuale (o `cat > file` intero) resta l'unico metodo che ha funzionato. Vedi bullet sopra.
    - **An `awk` with a state flag that never resets consumes the rest of the file** (observed 2026-11-05, `docs/CODE_TOUR.md`: 220 lines became 8). The command exited cleanly, no error: a `skip=1` set on a pattern whose reset never matched consumed everything after the first match. Only comparing `wc -l` with the count before the edit caught it, and the fix was `git checkout --`. Fallback: prefer `NR==N` ranges or `index($0, "text") > 0` to a state machine; after any `awk` that rewrites a file, compare `wc -l` with the previous count in the same block, and revert if it shrank.
    - **`perl -CSD -Mutf8 -i -pe` is the first choice for any edit that may contain 4-byte UTF-8 (adopted 2026-11-06, after a tested comparison).** perl 5.42 and node 22 are installed; the old default (`sed`/`awk`) was habit, not a comparison. Tested on a sandbox file: `perl -CSD -i -pe 's/🔵/🟢/'` **silently does nothing** (the script is read as bytes, the same trap as `sed`); `perl -CSD -Mutf8 -i -pe 's/🔵/🟢/'` works; `perl -CSD -i -pe 's/\x{1F535}/\x{1F7E2}/'` works too. **`-Mutf8` is mandatory, not optional.** If a literal still fails, use the codepoint form. Fallback to `awk` + octal only if `perl` is unavailable. Verify with `LC_ALL=C grep -c -F` and `git diff --stat` in the same block.
    - **Niubash (Windows) drops 4-byte UTF-8 on paste: use the codepoint form there (observed 2026-11-06).** Pasting a literal emoji into a Niubash prompt loses the 4-byte character before the command reaches `perl` (a 3-byte arrow survives; the 4-byte circle does not). The substitution then silently does nothing, and the verification passes for the wrong reason. In Niubash, the codepoint form (`\x{1F535}` to `\x{1F7E2}`) works, because codepoints are ASCII in the command. In Git Bash the literal form still works. Rule: literal in Git Bash, codepoint in Niubash; when in doubt, codepoint.
    - **`git apply --check` + `--stat` is the second choice for multi-line or multi-file edits already expressed as a diff (tested 2026-11-06).** `perl` stays first for a single-line replacement; `git apply` wins when the change is patch-shaped (bookmark compaction, a rename with its call sites, a multi-row fix). Three proofs: `--check` rejects a double application (safety); `--stat` shows the preview before the mutation (D5 artefact); `--check -R` confirms internal coherence (positive-application proof, since a direct `--check` against the same working tree is a self-check). **Do not run `--check` on a patch generated from the same dirty working tree as if it were a target** (observed 2026-11-06: `git diff > p; git apply --check p` fails by design). Generate the patch, `--check -R`, apply with `--check` on a clean or `git worktree` checkout.
    - **`perl -i -pe "..."` with a shell variable in the replacement breaks the quoting** (observed 2026-11-07, three failures in one session: `perl -i -pe "s/.../$NEW/..."` aborted with "Can't find string terminator"; the `awk` fallback with the replacement in a `/tmp` file worked). Fallback: write the replacement to a file with a quoted heredoc, then `perl -i -pe` with `\Q...\E`, or `awk` that reads the file on `/tmp`. Same rule as the code fence: the problem is nested quoting, not the length.
    - **`cargo check` without `--all-targets` does not compile the test modules** (observed 2026-11-07, two `move_agents` test call sites missed at a signature change; `cargo test --all-targets` caught them). After any signature change, use `cargo check -p <crate> --all-targets` (or `cargo test -p <crate> --all-targets`) before declaring the mutation green. C2's `cargo clippy --workspace --all-targets` covers it, but only if you run C2, not `cargo check`.
    - **CRLF vs LF: `perl -0777 -pe 's/a\nb/c/'` does not match on a Windows checkout with CRLF** (observed 2026-11-07, a double `#[test]` attribute survived the fix; `sed -n 'Np' file | cat -A` showed `^M$`, and the working pattern is `s/a\r?\nb/c/`). Before any multi-line `perl` or `sed` on a file you did not create in this session, check the line endings with `sed -n 'Np' file | cat -A`. The `s///` with a single `\n` is a silent no-op.
    - **A `perl -pe 's#^anchor#...#` pattern that matches the line start but not the line end silently no-ops** (observed 2026-11-07, a `characteristic\.` pattern against a line ending in `characteristic, not a die`). The `^...` anchor matched, the `.` after `characteristic` did not, the substitution did nothing, and the `wc -l`/`grep` verification showed no change — the failure was visible only because B2 was applied. **Verify the pattern's tail, not only its head.** Working method: `sed -n 'Np' file | head -c 80 | od -c` to see the exact bytes at both ends of the line before the substitution; then, after the substitution, `diff`/`git diff` on that line. A pattern written against a line you also wrote in the same session is the highest-risk case: you remember what you meant to write, not what you wrote.
    - **`perl -i -pe 's/.../.../'` with `/` delimiter and `../docs/` in the replacement aborts with `Backslash found where operator expected`** (observed 2026-11-07 on `specs/_index.md`). Fallback: use `#` as delimiter, or wrap the pattern in `\Q...\E`. **`\Q...\E` does not protect the delimiter** (observed the same session: a pattern containing `66/68` closed the `s///` mid-pattern even inside `\Q`): if the pattern itself contains `/`, change the delimiter or use the codepoint form.
    - **`awk` can duplicate an anchor instead of consuming the file** (observed 2026-11-07, a double `#[test]` attribute). B1-bis already has the "consume the rest of the file" symptom (a state flag never reset); a different failure is "print the anchor and then the replacement", which doubles the anchor. The check that catches both: `wc -l` before and after, in the same block, and refuse to `mv` if the count differs by anything other than the expected delta (see the OI-78 insert in `SESSION-STATE.md` this session as the working pattern: heredoc for the text, `awk` for the insert, `mv` only if the line count is the expected `before + N`).
    - **`awk` replacement: use `getline` line-by-line from the `/tmp` file, not `split($(cat /tmp), lines, "\n")`** (observed 2026-11-07, three REFUSED in a row on `resolve_shot`). `$(cat)` strips the trailing newline, so a 3-line block becomes 2 lines after `split`; the expected-delta check then refuses a correct replacement (or, worse, accepts a wrong one if the block was 3 lines and the count happened to match). The working form: `getline l1 < "/tmp/block.txt"; getline l2 < ...; getline l3 < ...; print l1; print l2; print l3`. Verify the block first with `wc -l` and `cat -A`; the blank final line is part of it. Also: when a block replacement refuses twice, do not re-count by hand; open the file, `sed -n 'Np'` the anchor and the lines to skip, and recompute the skip range on the current file.
    - **clippy `too_many_similar_bindings` and `cast_possible_truncation`** (observed 2026-11-07 on `resolve_shot`). Similar names (`dy_sign`/`dy_abs`) trigger clippy even when they are conceptually different: rename to `lateral_sign`/`lateral_abs` (the domain name), not `#[allow]`. `f64 as i32` triggers `cast_possible_truncation`: `#[allow(clippy::cast_possible_truncation)]` on the single line (not the whole function), as the existing `speed_from_passing` already does. Run `cargo clippy -p <crate> --all-targets -- -D warnings` after the edit; `cargo check` does not see these.
    - **`sed -i` with shell control flow in the replacement writes it literally into the file** (observed 2026-11-07: `sed -i 's#...#if false; then :; fi...#'` put `if false; then :; fi` inside a Rust doc comment, silently, no error). Never put control flow in a replacement; if a conditional replacement is needed, do it in two blocks, not one.
    - **Command construction: no `||` with two branches, no nested heredocs, no "if this fails do that instead".** B4 says "no two mutations in one block"; it does not say "no control flow in the command". Four failures in one session were fixed, every time, by "one block, one thing". The fallback chain is a leak: it looks like resilience and hides which branch ran.
    - **`grep -c` with a loose pattern gives false positives** (observed 2026-11-07: reported 2 when the true count was 1, because a different OI on the same line contained the substring). The truth for a line edit is `git diff <file>`, not `grep -c`. And `grep -c` counts *lines*, not occurrences — a line with the pattern twice still counts once.
    - **Long heredoc with non-ASCII beyond accented letters breaks the paste (observed 2026-11-07, twice: once with `^C`, once with an unterminated heredoc at the `>` prompt).** B1-bis covers the code fence inside the heredoc (the fence breaks it, not the length); it does not cover the plain long paste with em-dashes, emoji or box-drawing. Criterion: if a block is more than ~40 lines, or contains non-ASCII beyond accented letters, write it to `/tmp` in pieces (`>` then `>>`), `wc -l` after each piece, and `mv` into the repo at the end — an interruption then touches `/tmp`, not the real file.
    - **A heredoc with `#` at line start, `**` and backticks together can lose characters even with `<< 'EOF'`** (observed 2026-11-08, rewriting `NEW-SESSION-PROMPT.md`): `#`, `**` and the fence backticks were silently dropped from the written file; `grep -c '```'` returned 0 on a file that should have had two fences. The paste mangled the content, not the shell. Rule: write in blocks of ~20 lines with `>` then `>>`, `wc -l` after each block, and check the fences with `grep -c '^````'` before the final `mv`. If a block contains `#` at line start and backticks, split it further.
    - **`awk` con `END` e senza `{print}` distrugge il file** (observed 2026-11-09, `NOTA-PER-NUOVA-AI.md` da 196 a 2 righe, recuperato con `git checkout --`). Diverso dal bullet "awk che consuma il file" (un flag di stato che non si resetta): qui e' `END` senza `{print}` che scrive solo le righe finali. Fallback: dopo ogni `awk` che riscrive un file, `wc -l` before/after nello stesso blocco; se il delta non e' quello atteso, `git checkout --` prima di qualsiasi altra cosa.
    - **L'apostrofo tipografico (U+2019) rompe i pattern** (observed 2026-11-09, due `perl` falliti perche' il testo conteneva U+2019 e il pattern cercava `'` ASCII). La regola: quando il testo viene da chat, controlla l'apostrofo con `sed -n 'Np' file | od -c` prima del pattern, non assumerlo ASCII.
    - **Il rename rompe i gate in silenzio** (observed 2026-11-09, ADR-032). Durante `ds-core`: `gates.sh A4` contava solo il prefisso vecchio, e il conteggio risultava piu' basso pur passando per il motivo sbagliato; `NEW-SESSION-PROMPT.md` conteneva un conteggio hardcoded che ha fatto scattare A3. Per il prossimo rename: estendi la regex del gate **prima** del rename, e non mettere conteggi nel prompt.
    - **A signature change with N call sites is not a single-block edit** (observed 2026-11-11, `move_agents` in `spatial.rs`: the signature and `write_back` changed, the 3 call sites did not, `cargo check --workspace` red, revert). The workspace must be green after **each** signature change, not at the end. Either change the signature **and** its call sites in one commit that compiles (and `cargo check --workspace` after every mutation), or do not change the signature at all. An `awk` that removes a guard can eat one line too many: count the lines to skip on the file in front of you, not from memory (`skip=3` for a 2-line guard cost a full revert).
    - **An `awk` that inserts before `pub fn X` lands *inside X's doc comment*** (observed 2026-11-11, twice: `build_choice` in `read.rs`, `keeper_target`/`move_agents` in `spatial.rs`). The doc comment sits *above* `pub fn X`, so inserting on that anchor puts the new block between the doc and the function: `X` loses its doc (`#![deny(missing_docs)]` goes red) and the block is inside the doc. Anchor to the **blank line before the doc**, or insert **after the previous function's closing brace**, never on `pub fn X` itself. The `wc -l` delta is right and the mistake is invisible to it: only `cargo check` sees it.
    - **`awk` with a `skip` counter on a multi-line Rust block fails silently** (observed 2026-11-11, three attempts on `move_agents` in `spatial.rs`, every one a `git checkout --` revert). A block replacement needs the exact number of lines after the anchor; one off and a brace or a `let` disappears, the `wc -l` delta is still close, and only `cargo check` sees it. For Rust: rewrite the whole file (`cat > file`) when it is small enough, or write the diff by hand and use `git apply --check` then `git apply`. Never a `skip` counter on code.
    - **Three failures of the same class on the same file: the class is wrong, not the case; the fourth sample adds no information** (P3, observed 2026-11-11, `move_agents` in `spatial.rs`: three failed `awk` on a multi-line Rust block, every one a `git checkout --` revert; the class was awk-on-Rust-code, changed to `cat > file` or `git apply` with a hand-written diff). Do not try a variant of the failing method; change the class of tool. Capture: `docs/context/captures/2026-11.md` 2026-11-11 07:58 and 22:54. `why-new:` without this line, a fourth failure of the same class reads as "try another variant of awk", which is the same sample again.
- **B2 — Verify after every edit.** `sed -n 'Np' file` to confirm. `git diff file` before `git add`. If the diff shows unrelated changes, stop. If between the sentinels there is no content, the command succeeded but was silent. If the content is not what was expected, stop and report it.
- **B3 — Output sentinel.** Every command must be wrapped in the sentinel, using this exact template: `echo ""; echo -e "\033[1;91m➤➤➤ OUTPUT START ➤➤➤\033[0m"; <command>; echo -e "\033[1;91m◀◀◀ OUTPUT END ◀◀◀\033[0m"; echo ""`. The sentinel text is **generated by echo**, never written literally in the command block — a line `➤➤➤ OUTPUT START ➤➤➤` alone in bash fails with `bash: ➤➤➤: command not found`. Color (red) is lost on copy/paste to chat: expected, not a bug (adopted 2026-11-06, maintainer request; revised same day: bold + triple arrows for terminal selection). A command without sentinel is a violation.
- **B4 — Mutation protocol.** A *mutation* is `sed -i`, `cat > file`, `git mv`, `git add`. Rules: (1) **No two mutations in one block.** After a mutation, the *same* block contains a verification (`sed -n 'Np'`, `wc -l`, `grep -c`); if the mutation fails, the verification shows it. Never chain a write and a delete. (2) **`sed` patterns are ASCII.** For lines with emoji, arrows (`→`), or other non-ASCII, use line-number + full-line replacement (`sed -i 'Ns@.*@new@' file`). Multibyte patterns fail silently on Git Bash (observed 2026-10-31: `sed` with 🔴🟡 silently did nothing; a `sed d` on a stale line number deleted the wrong row). (3) **Heredoc writes are verified.** After `cat > file`, run `wc -l file` and `head -3 file` in the same block. (4) **On failure, stop.** Do not retry with a different pattern in the same block; report and fix in the next command.

**C. Git and quality**

- **C1 — Commit and push.** Work in the local repo, commit in Git Bash, push. The GitHub web interface is for emergencies only. Every change: edit locally → `git add` → `git commit`. Small, atomic commits (one per file, or per logical group), clear messages in English, imperative mood. A rename is ONE logical commit, not one per occurrence. **Push at the end of the session or at a checkpoint, not after every commit.** *Phase 1 (now):* ruleset disabled, single maintainer, direct push to `main` allowed, PR optional. *Phase 2 (when an agent gets a write token):* ruleset active, branch + PR mandatory, no direct push. See `docs/context/AGENT-POLICY.md`. **Two remotes (2026-11-08):** `origin` (GitHub) is primary, `codeberg` is backup; every push goes to **both** (`git push && git push codeberg main`). See `AGENT-POLICY.md` §6 "Two remotes".
- 🛑 **C2 — Quality gate (baseline, mandatory).** The pre-commit hook (`.githooks/pre-commit`, one-off install: `git config core.hooksPath .githooks`) runs fmt + clippy + test only if Rust files are staged. The hook does **not** replace the full check: also run `cargo build --release`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo fmt --check`, `cargo run --release --bin ds-test-runner -- --check --seed 42`. If anything is red, stop: do not close, do not push, do not "fix later".
    - **When you add a test, update the test count in the same commit.** Four documents declare it: `README.md` (badge + two tables), `STRUCTURE.md`, `CONTRIBUTING.md`, `specs/_index.md`. `A4-coherence` checks that the documents agree with **each other**, not with the real count (the real count comes from C2): if all four say 143 and the repo has 144, the gate is green and the number is wrong. Three times in the 2026-11-02 session.
    - **When you bump `SIM_VERSION`, follow the procedure in `CHANGELOG-SIM.md` §"When to bump" (step 2).** In short: bump the constant in `save.rs`, run `cargo test -p ds-match --test golden_l0`, copy the `left:` value from the failure into `EXPECTED` in `golden_l0.rs`, add the entry to `CHANGELOG-SIM.md`. The comment above `EXPECTED` carries the `SIM_VERSION` it refers to: update it too.
- **C2-bis — Beyond the baseline (encouraged).** The goal is a clean, optimized, secure repository, not merely passing the commands above. If your toolchain or model capabilities allow additional checks — e.g. `cargo-audit`, `cargo-deny`, `miri`, coverage (`cargo-tarpaulin`), mutation testing, fuzzing, manual review of hot paths — run them and report the result. Do not limit yourself to the baseline.
  - **Caveat (D2 applies).** "Optimized" means *verified* optimization: reduce allocations, profile, remove redundancy, **without changing behaviour**. Any change to the simulation model requires an ADR and a `SIM_VERSION` bump. Do not introduce `unsafe`, `HashMap` in simulation paths, or other banned patterns in the name of optimization.
  - **Elegance (D7, D5).** "Elegant" is not taste: it is the **form of the code mirroring the form of the doctrine**. **Examples, not rules** — the list below is illustrative, none of it is a corollary of D7 or D5, and it can be contradicted by a specific decision: Plane A behind one door; determinism visible (`BTreeMap`, no `unwrap` in simulation paths, named streams); one source of truth (no duplicated state) — this is **STRUCTURE.md §5**, not D7; comments that explain **why, not what**. An inelegant codebase is one where the reader cannot see the rules. This is a **visibility** criterion, not a licence to rewrite working code (D7). Artefact (D5): one line in the closing report, `Elegance: <what was made more visible> | no change`, verifiable in `git diff`, never on the agent's word.

**D. Context and doctrine**

- 🛑 **D1 — Honesty and ideas.** If you have not read a file, say so. Do not invent numbers, commit hashes, or file contents. "I don't know" > invention. **When facing ambiguity, or when the context has not been fully read, prefer completeness over speed: read the full context before acting.** New ideas go to `docs/context/IDEAS.md` (status `sketch`, with prerequisites and open questions) **before** discussion, not after. Do not implement from an idea until it has an ADR (if it touches doctrine) or a step in `SESSION-STATE.md` (if it is a task).
- 🛑 **D2 — Doctrine and codebase.** No `unwrap`/`expect`/`panic!` in simulation paths. No `HashMap` in simulation paths. No `unsafe` outside `pcfm-bridge` (future). `pcfm-hidden` contains Plane A: no public fields, not `Serialize`, custom `Debug`; the only door is `resolve_*` + `drain_observations`; never add getters on latent values. `pcfm-testkit` has no `pcfm-*` dependencies. The simulation model is a prototype: do not "optimise" without an ADR and a `SIM_VERSION` bump. The doctrine wins: if a change contradicts `00-foundations`, a doctrine ADR is required. `docs/adr/` contains only ADRs.
- **D3 — Bee doctrine (advisory, not binding).** A structure should be added when concrete content requires it *today* — code that uses it, or a test that verifies it. This is **advice, not a veto**: it must be considered, and going against it is allowed when the consideration is declared. When a structure is added without a consumer today, the commit carries a `D3-check:` line — what D3 advised, the cost of proceeding, and why proceeding is worth the cost (D5 artefact: the maintainer reads the reasoning, not the agent's word). A structure that only a *future* consumer would need is still an idea (`IDEAS.md`), not a design; but the decision is the maintainer's when the cost is declared. This applies to the agent's proposals as much as to the maintainer's.
- **D4 — Decision pre-mortem (rules only).** Before writing a **rule** (a bullet in this file, a new file in `docs/context/`, a hard-stop, a startup or closing step), ask four questions and state the answer with evidence:
    1. **Coherence:** does it contradict an existing rule? Cite `file:line`. If yes, fix or amend, do not proceed.
    2. **Leak:** does it open a way to appear compliant without being compliant? (self-controlled conditions, unverified fields, silent exceptions.) If yes, close the leak before writing.
    3. **Existence:** does this already exist — in this prompt, in Git, in a standard tool? If yes, use the existing one, do not duplicate.
    4. **Blast radius:** what will it weaken downstream? If you cannot see the downstream effect, say so — it is a finding, not a pass.

    Output (evidential): `pre-mortem: coherence=none; leak=none (checked: self-controlled condition, unverified field, silent exception); exists=none (checked: prompt, Git, standard tool); blast=none (checked: D1-D5, C2, E2-E5)`.

    **Not for ADRs, not for tasks.** An ADR that prescribes an implementation is written after reading the code it touches, or it declares "implementation shape not validated against the code" (ADR-026's lesson, 2026-11-01). A task follows the Plan; it does not need a pre-mortem.

    **Exit clause (pure vs gap).** A finding **blocks** the session (pause, no closing) if it is **pure**: (a) exposes a latent (D2); (b) violates an invariant (`SIM_VERSION`, `chain_hash`, schema); (c) touches a doctrine ADR or a rule that the finding itself would weaken; (d) is **not tracked** as an OI or step with an executable first action (§F). Otherwise it is a **gap**: it goes to `SESSION-STATE.md` Open items with a first action, and the session closes. **Tracking does not exempt a D2 or invariant violation** — (a) and (b) always block. Default when in doubt: pure.

    **Filter agent (FA).** The maintainer is the FA for gaps that require judgment (question 4, D5's visible-as-fallback conditions). FA never applies to pure findings. Invoking the FA is a pause (A2), in prose. The agent presents the gap **once** (finding, options, default, cost) and does not iterate. No decision → the gap goes to Open items with a first action, and the session proceeds (E3).
- **D5 — Mechanical and objective (visible as fallback).** The default lens for every discussion and every rule: a condition should be verifiable mechanically and objectively, by the maintainer, without trusting the agent's word. Where the condition is **inherently judgment-bound** (e.g. E2's "when the work seems complete", D4's "blast radius"), the rule must at minimum produce a visible artifact — a commit trailer, a report field, a `git log` entry. A condition controlled only by the agent's judgment, with no artifact, is a leak, even when the agent is honest. This applies to every rule in this file, including D5 itself.

- **D6 — Metro "partita vera" (lens obbligatoria, con artefatto).** Ogni decisione di design del motore (modello di un evento, di una condizione, di una relazione, di un input) deve poter rispondere a tre domande: *in una partita vera, dal punto di vista degli attori che oggi esistono, come si riconosce questo fenomeno? da cosa dipende? chi lo produce?* A L0 gli attori sono i giocatori; a L1 il manager e il direttore sportivo; a L2 il presidente. Il filtro vale per gli attori del livello in cui la decisione vive, non per attori futuri: manager, DS e presidente entrano quando il loro livello esiste (D3), e fino ad allora sono **dichiarati**, non applicati. Il tifoso è un lettore **downstream**: se il modello è riconoscibile dall'attore, il tifoso riconosce l'effetto; il contrario non è vero, ed è il caso del portiere — il tifoso vede il tuffo, ma senza lettura e scelta quel tuffo è una soglia. Se il modello non può rispondere, è incompleto o sbagliato. La risposta va nel **commit message** (o nell'ADR, se la decisione lo richiede): il trailer o il corpo cita il metro e la risposta. **Senza la citazione, la decisione non è una decisione**: è un'opinione, e va trattata come tale (D1). Il metro non dice *come* implementare: dice *cosa* il modello deve poter spiegare. Esempi: la pressione è una relazione modulata dal talento, non uno stato individuale; la scivolata dipende da superficie, velocità, equilibrio; il passaggio da decisione, percezione, esecuzione. Pre-mortem (D4): coherence=none (non contraddice D1-D5: D5 chiede un artefatto, D6 lo fornisce; DP-01/02/03 restano invariati); leak=il commit message può citare il metro in modo generico ("è realistico") senza rispondere alle tre domande — mitigazione: la citazione deve contenere la risposta a *come si riconosce*, non un'etichetta; verificabile a occhio in 5 secondi; exists=none (il metro è stato usato due volte oggi — IDEA-DESIGN-001, C.1 — ma non era scritto; `DECISION-PATTERNS.md` non può contenerlo: un DP è sovrascrivibile, D6 no); blast=ogni decisione di design del motore, per sempre — mitigazione: la citazione è un artefatto visibile in `git log`, e il metro è verificabile senza fidarsi dell'agente (D5).
    **D6 emendato da ADR-036 (2026-11-07).** La seconda domanda di D6 ("da cosa dipende?") per un **gesto** si risponde nominando le relazioni che legge: chi c'è intorno, su che lato, con che forza. Non e' un obbligo (una risposta "nessuna" e' legittima, se dichiarata), e' la lente del pillar 11 di `00-foundations` v3.0. Vedi ADR-036 §"A lens, not an axiom".

- **D7 — Raffinamento, non aggiunta.** Un'implementazione (una regola, una tabella, una frase di README, un gate) è un **raffinamento** di ciò che esiste: non toglie convenzioni che funzionano, le rende più meccaniche, più verificabili, più coerenti con la direzione. "Aggiungere" è l'ultima opzione, non la prima. Se una convenzione funziona ma non è meccanica, il lavoro è meccanizzarla, non sostituirla. Se una convenzione **non** funziona, il raffinamento non basta: va sostituita (D7 non è conservatorismo). Pre-mortem (D4): coherence=none (D3 dice "aggiungi solo con contenuto"; D7 dice "preferisci il raffinamento all'aggiunta"; D5 dice "meccanico"; D7 è la loro intersezione, non una contraddizione); leak=il raffinamento può diventare una scusa per non aggiungere ciò che serve — mitigazione: la clausola "se non funziona, va sostituita" chiude il leak; exists=parziale (D3 copre l'aggiunta, non il raffinamento; nessuna regola dice "preferisci raffinare"); blast=ogni decisione futura — mitigazione: la clausola di sostituzione rende D7 falsificabile ("questa convenzione funziona?" è una domanda con risposta).

**E. Session lifecycle**

- **E1 — Startup ritual mandatory.** At the start of every new session, before proposing anything, execute the Startup ritual. Not optional. If a step is impossible (e.g. no network for `git pull`), report the failure and proceed with what is possible. Never invent output.
- **E2 — Closing ritual: explicit approval, with "chiudi" as the only closing command.** The closing ritual is never automatic. When the work seems complete, pause (A2), present a status report (what was done, what was not, what is open), and wait. The closing executes **only** on the exact word **"chiudi"**, written as such (A2: no equivalent). "Riprendi", "ok", "va bene", silence, an "ok" to an unrelated question, or the natural end of a task are **not** approval to close: they do not close.

    **Green-path (reduced friction, not reduced approval).** When all of the following hold — (a) C2 green (full quality gate, no exception); (b) Gate A green; (c) Gate B green, including the manual checks B2 and B4, each declared in the status report; (d) Step 2b (rules self-audit) clean, or no rule was touched this session; (e) no finding classified as critical during this session remains open without being tracked as an OI in the bookmark — the agent presents a **short** status report ("all green, nothing to decide, details: …") and still waits for **"chiudi"**. The green-path reduces the report, not the approval. If any condition is false, the pause is mandatory and the maintainer decides.

    **Rollback** for a wrong closing: `git revert <commit>` + push (see `AGENT-POLICY.md` §9). The report always opens with `SESSION CLOSED — no further work executed (E3)`.
- **E3 — After the closing, the default is "track", not "execute".** Once the bookmark commit is made, the session is closed. Any new work that emerges afterwards is added to `SESSION-STATE.md` as a step (with its first concrete action) **and is not executed in the same session**. Three exceptions, and only three: (a) the bookmark is *internally inconsistent* (e.g. `Last commit verified` does not match reality, or a cited file does not exist) — a broken bookmark is worse than an incomplete one; (b) a quality-gate failure (C2) is discovered; (c) the maintainer explicitly asks for the work to be done now. "It would be useful for the next session" is **not** an exception: that is precisely what the step list is for. The test is: *if this is not done now, can the next session reconstruct the state from the repo?* If yes, track it. If no, do it. This rule exists because every reopening after the closing makes the bookmark less trustworthy, and the bookmark is the memory of the project.
- **E4 — Bookmark update.** The bookmark is updated in three cases: (a) a significant step is closed; (b) before a push (the bookmark update is the last commit before the push); (c) the context limit approaches. In all three: write `SESSION-STATE.md` complete according to the Bookmark contract, commit, report. In emergency mode (c), no new work starts: the session closes, the next session resumes from a coherent bookmark.

    **S1 applies to every commit that touches `SESSION-STATE.md`**, not only to the closing commit. A commit that changes the Plan or an OI is a bookmark update: `Last commit verified` must point to the commit before it (S1 semantics). A commit that touches the bookmark without updating these fields leaves the bookmark partially inconsistent. The `commit-msg` hook enforces the `Last commit verified` part mechanically (see §G).

- **E5 — Ideas from pause.** On "riprendi" after a pause (A2), if ideas emerged during the pause, the **first command** writes them to the right place before any agreed work resumes: design ideas to `IDEAS.md` (status `sketch`, with prerequisites and open questions); operational rules to this file; project decisions or open items to `SESSION-STATE.md` Open items. E5 is the operational trigger of D1; D1 remains the principle.

**Form (2026-11-08): the first command is `scripts/capture.sh pause "<testo -> promozioni>"`.** The capture runs the promotion (it verifies that every OI, IDEA or ADR after `->` exists) and writes the entry to `docs/context/captures/`. The text is the agent's; the `->` filter is mechanical. `capture.sh` does not replace E5: it is E5's operational form. If the pause produced no idea, `capture.sh pause ""` writes a `filtered:none` row (the artefact of the filter) and no body.

**F. Bookmark contract**

`SESSION-STATE.md` has a fixed structure. Sections, in this order:

1. **Last commit verified** (S1 semantics: the commit before the bookmark update).
2. **Open items** — decisions deferred, pending verifications. Each entry: ID, title, first action. The "why" lives in the title when not obvious; no separate column. **Every entry must be self-contained for a reader who does not know the project** (assume they have not read the docs, the philosophy, or the previous session): (a) what, in plain language, explaining any jargon the first time it appears; (b) why it matters, the concrete consequence if left as is; (c) a first action that can be executed without asking questions; (d) a reference to where to read more, if needed. This is a rule with an artefact (the row itself, visible in the diff); B3-bis checks only that the first action is non-empty, not that the text is clear.
3. **Plan** — an indented tree, not a flat list. Leaves are executable steps: what, dependency, first concrete action. Branches (forks) expose a decision the flat sequence cannot represent: each fork names the step that closes it (in the row itself, "Direzione" column or a `decide-in:` note). A step without a first action is a step the next session will skip; a fork without a closing step is an idea, not a plan (D3).

State labels are ASCII: `[wip]`, `[next]`, `[open]`, `[deferred]`, `[urgent]`, `[done]`. `gates.sh` B1 parses the Plan by state and ID; emoji would be invisible to the parser (and `sed` with 4-byte UTF-8 fails silently on Git Bash, see B1-bis). The label is operational, not decoration. A step with no `[next]` is a valid state (between slices): B1 says so and stays green.

The startup ritual extracts §1–§3. The closing ritual updates all three.

**No `Files touched`, no `Done`.** Both are in `git log --stat` and `git log --oneline`. The bookmark is memory, not a log. A future session that needs the history runs `git log`.

**G. What Git already does (use it, do not reimplement)**

If Git provides the mechanism, use it. Session metadata lives in **commit trailers**, not in chat prose, so it is verifiable from `git log` at any later date.

- **Commit trailers** (block of `key: value` lines at the bottom of the commit message, after a blank line). The `commit-msg` hook (`.githooks/commit-msg`, one-off install: `git config core.hooksPath .githooks`) validates them. Template: `git config commit.template .gitmessage` (one-off, same install step). **The hook fires when `docs/context/SESSION-STATE.md` is staged** (closing commit), not by subject: a closing commit with an arbitrary subject is still validated. Required on closing commits:
    - `Verified:` — `C2-green` when the full quality gate ran and passed.
      **The trailer must be true, not just present.** The hook checks that
      `Verified:` exists; it cannot check that C2 ran. Write `C2-green` only
      if you ran the full gate **in this session** and it passed. If it did
      not run (a docs-only commit, or the workspace is red for unrelated
      work), write `Verified: C2-not-run (<one-line reason>)`. A false
      `C2-green` is a D1 violation in a persistent artefact: `git log`
      repeats it to every future reader and no gate catches it. Observed
      twice on 2026-11-05: `f94f083` carries `C2-green` on `origin/main`
      while C2 was red (golden failing for the uncommitted F.7 work), and
      `453550c` was amended before the push for the same reason. The first
      cannot be corrected without rewriting shared history; the second was
      caught only because the push re-ran the gates.
    - `Rules-Audit:` — **only if this session modified a rule file** (`NEW-SESSION-PROMPT.md`, `DECISION-PATTERNS.md`, a hard-stop, the startup or closing). `N changes, 0 contradictions, 0 leaks`, or `findings=[...]`. If no rule was touched, the trailer is omitted; the closing commit says nothing, which is honest.
    - `Direction-Check:` — **required when the diff touches a direction file** (`docs/context/IDEAS.md`, `docs/WORLD-MODEL.md`). The trailer declares what was checked and the outcome: `Direction-Check: <term> -> <OI or step contradicted | none>`. The judgment is not mechanised (no gate can decide "this direction contradicts this decision"); the **artefact** is: a false `none` is a D1 violation visible in `git log`, and a missing trailer is a red hook. Rationale: on 2026-11-02 the influence direction contradicted C.1, and the contradiction was noticed three commits late; with this trailer it would have been resolved in the same commit.
- **What Git already does, so we do not reimplement it:**
    - the files touched, the history, and the closed steps: `git log --stat`, `git log --oneline`. The bookmark does not duplicate them (see §F).
    - the decision patterns actually used: `git log --format='%(trailers:key=DPs-Applied,valueonly)'` on old commits (read-only; the trailer is no longer required, because the DP list is small and the `grep` in `DECISION-PATTERNS.md` is the source).
    - **Regression isolation**: `git bisect run <script>` (the anti-leak test is the script).
    - **Post-hoc annotations**: `git notes add` on an old commit, never rewrite history.
    - **Release exclusion**: `.gitattributes` with `export-ignore` for `docs/context/` and `DECISION-PATTERNS.md` when packaging.

If a standard tool already does what we need, use it: `awk` over `sed` for long replacements (B1-bis), `git log --format` over hand-parsed prose, `git revert` over re-edit for rollback.

**Rule precedence:** the hard stops (D1, D2, C2) win over everything else. D1 (honesty) first. In any other conflict, ask the maintainer.

## Patterns (how to work — observed, not speculative)

These are the working patterns that have saved hours and the failures that have cost them. They refine the rules above (D7); where one repeats a rule, the rule is the source and this is the reminder. Each one is an artefact: a test that can fail, a `grep` in the report, a field in the commit.

- **P1 — A test that cannot fail is worse than no test.** If the sample is empty, an `assert` inside the loop proves nothing. Verify that the sample is non-empty, or write the test that *forces* the case. A test that always passes looks like coverage and is not.
- **P2 — `grep` before saying "consumer: X".** Before classifying a field as "read by X", run the `grep`. Almost every field classified from memory had no consumer. Without the `grep`, it is an invention (D1).
- **P3 — A criterion that fails on a real case is wrong.** When a criterion breaks on a concrete case (observable-in-a-match fell on height, weight, strength), do not patch it: find one that holds on every case and declare why the first was insufficient.
- **P4 — "Decided" is not "implemented".** In every artefact (Status column, Plan cell, commit body), write which of the two it is. A decision that is written but not coded is a decision, not an implementation.
- **P5 — Futuribility justifies Plane B fields, not Plane A.** The tick record → 2D/3D is a declared consumer for observables. The manager never sees a true value: a latent never ends up in a frame.
- **P6 — Prefer a conclusion to a menu.** When the metrics in context yield a default and the alternative is trackable as an OI with a first action, apply the default and state it (A2, *Default derivabile*). Reserve the menu for model choices and doctrine choices, which are the maintainer's. `00-foundations.md` §1 ("propose two options") applies to an ambiguous specification, not to a decision the agent can derive.
- **P7 — Accept a correction and reformulate. Do not defend.** When the maintainer corrects a categorisation (A/B, pure/gap, decided/implemented), the correction is the classification. Reformulate, do not argue.

## Startup ritual (mandatory — see E1)

At the start of every new chat, **before proposing anything**, ask the maintainer to run these commands in order and paste the output. Do not assume the state described in this prompt is still true: verify it.

**First command: `scripts/starting.sh`.** It runs `git status --short`, `git pull --ff-only`, `git log --oneline -5`, `scripts/gates.sh resume`, and prints the `NEXT: read` list extracted from this file. It does not read the doctrine for the agent: the list is the artefact, the step 5 report is the proof (D5). If `git pull` fails (no network), report it and continue with the rest.

### 1. Repo alignment

    git status --short
    git pull --ff-only
    git log --oneline -5
    git rev-parse HEAD

Check: working tree clean (or only known untracked files), HEAD matches `Last commit verified` in `SESSION-STATE.md` (semantics S1: it points to the commit *before* the bookmark update), no unexpected commits.

### 2. Read context (bookmark, backlog, reports)

Three reads, in this order, all required. Two more only when the task needs them.

**Bookmark.**

    cat docs/context/SESSION-STATE.md

Extract sections 1-3 of the Bookmark contract (§F above): LCV, Open items, Plan.

**Ideas backlog (what is not decided yet).**

    grep -nE "^### IDEA-|^- \*\*Status:\*\*" docs/context/IDEAS.md

Read the ID, title, and status of every idea. Do not implement from `IDEAS.md`. The purpose is to know what has already been thought of (and rejected), so the session does not re-propose it or reopen a REJ.

**Analysis reports (continuity).**

    ls docs/context/analysis/*.md
    ls docs/context/analysis/*handoff*.md | tail -1

Read in full the newest handoff (the second command: names start with the date, so the last one is the newest). For every other report the filename and the line the bookmark gives it are enough at startup. **Before proposing a step, `grep -l` the step's main term across `docs/context/analysis/*.md` and read the hits**, so a decision already taken is not reopened; the proposal carries one line, `reports checked: <term> -> <files | none>`. List the reports not read in the step 5 report. If a report says "Pass" but has unverified findings, do not trust the verdict: verify each finding with `grep`/`sed` before acting.

**The current handoff is `docs/context/analysis/2026-11-09-session-handoff-v7.md`** (314 lines, 8 sections: what the project is in five lines, state verified, what this session decided and in what order, what is open with the dependency order the maintainer approved, traps paid, context notes, how to start, what it supersedes). Read it in full, not only the first command's output: it is written for an agent who was not there, and sections 2-3 and 5 are the ones the bookmark cannot carry. If a newer handoff exists, read that one and treat v3, v4, v5 and v6 as history.

**The briefing note is mandatory (E1, emended 2026-11-09).** After the handoff, read `docs/context/NOTA-PER-NUOVA-AI.md` in full (224 lines, 11 sections: the collaboration pact, the two filters, the OI-88 lesson, the four-word vocabulary, the salience-formula rules, the four tick alternatives, the maintainer's oral decisions, things without an OI, the observed meta-instructions, a silent no-op, the changelog of the note). It is **not** auto-loaded and it is **not** a handoff: it is the "how to work" that the repo alone does not carry. Every agent, new or not, reads it. The step 5 report declares whether it was read (D1).

**Only when the task needs them:**

- **Agent policy (phase rules, push, rollback):** `grep -nE 'Phase 1|Phase 2|ruleset|push|rollback' docs/context/AGENT-POLICY.md | head -15`. Read the matched lines. If the file structure changes, update this grep in the same commit.
- **Decision patterns (observed defaults):** `grep -nE '^## DP-|^   - (Source|Rule):' docs/context/DECISION-PATTERNS.md`. A DP applies only to an identical case; a variant requires asking. Max 10 active.

### 2-bis. Bookmark coherence and pointers (fast gate)

    scripts/gates.sh resume

Runs the fast subset of the gates (the same set as `startup`: A1, A3, A4, B1, B3, B6, B7, E4b, B8, and B2 reported as manual), exits non-zero if one is red, then prints pointers read from the repo. It reads the bookmark you just read, so run it **after** step 2 and **before** step 3: if it is red, the bookmark is internally inconsistent and the session starts on a false base.

If red: report it to the maintainer **before** proceeding. Do not fix it silently: a broken bookmark is worse than an incomplete one, and the maintainer must decide whether to reopen the previous session (E3a) or proceed with a declared caveat.

**Why these checks run twice.** A1, A3, A4, B1, B3, B6, B7 run here and again at closing step 3, on purpose: here they catch a stale bookmark *before it is used*; at closing they confirm the bookmark is still true *after the session's edits*. The two runs cost under 0.1 s together and catch different failures. 

**Pointers for the session (mechanical).** After the gate lines the same command prints, read from the repo: LCV, HEAD, remote state, the `[next]` step, the OI count and the newest OI, the last two B1-bis lessons, the ideas count, the analysis reports, and whether the tree is dirty. The pointer block is not a check: it adds no gate. If it disagrees with the bookmark you read in step 2, trust the repo and report the divergence.
### 3. Read foundations (doctrine, rules)

Two reads, in this order, both required.

**Doctrine (non-negotiable).**

    cat specs/00-foundations.md

At minimum: §1 (pillars), §2 (design pillars), §3 (Three Planes), §5 (banned things).

**Direction (why the engine has this shape).** Not the whole files, the sections that say *why*:

    sed -n '/^## 0\. Vision/,/^## 1\./p' docs/context/L0-POSITIONAL-SLICE.md
    grep -nE '^## 4|^## 5' docs/WORLD-MODEL.md

The first is the vision (phenomenological simulation, not physics; eight categories; every event carries the fields to explain why). The second is the simulation/presentation boundary: the renderer does not invent events, it shows the ones the engine emits. Then read `docs/context/analysis/2026-11-04-direction-decisions.md` (the reasoning of the last session, written for an agent who was not there). Without these, an agent knows *what* to do and not *why the engine is like this*, and reopens decisions already taken.

**If the direction sources disagree, this is the order:** `specs/00-foundations.md` > `L0-POSITIONAL-SLICE.md` §0 > `WORLD-MODEL.md` §4-5 > any dated `analysis/` report. A lower number wins. A report that contradicts a higher source is a finding to report, not a silent override (D4, maintainer decision). The full reasoning is in §8 of the direction-decisions report.


**Operating rules.**

    cat docs/context/NEW-SESSION-PROMPT.md

This very file — operating rules (A1, A2, B1-B4, C1, C2, D1, D2, D3, E1-E5), rule precedence, the startup and closing rituals, what NOT to do.

### 4. Confirm the numbers

    cargo test --workspace 2>/dev/null | grep "test result"
    ls -d crates/*/ | wc -l
    for f in docs/adr/*.md; do sed -n '3p' "$f"; done | grep -c 'Status:\*\* accepted'

Compare with `SESSION-STATE.md` and this prompt. If they diverge, the docs are stale — report it before proceeding.

### 5. Report to the maintainer

Say explicitly:
- What you read (files, line ranges)
- What you did **not** read (and why)
- The state of the repo (aligned? clean? numbers?)
- The next step according to `SESSION-STATE.md`
- Any divergence between docs and reality

**Do not propose a step before step 5 is done.** "I have not read X" is a valid state, not a failure.

## What NOT to do

- Do not give "lots of comments" scattered through the code: the codebase is already documented.
- Do not assume the status of an ADR from memory: read `STRUCTURE.md` §5 (single source of truth). ADR-012, 013, 015, 016, 018, 019 are **accepted**, not proposed — but verify, do not assume.
- Do not assume ADR counts: they change. Use `STRUCTURE.md` §5 for the current number.
- Do not propose Python scripts (Python3 **is** installed, verified 2026-11-06, but one toolchain is the policy: Bash/Git Bash, plus `perl` for UTF-8 edits per B1-bis). Use Bash/Git Bash.
- CI exists: `.github/workflows/ci.yml` runs C2 + `gates.sh startup` on every push and PR. It is the remote layer; the local hooks are the fast layer. Do not bypass a red CI. Dependabot alerts are on (2026-11-01); `rand_chacha`/`rand_core` 0.10 are ignored because they break the deterministic RNG.
- Do not reactivate the ruleset on `main` without explicit instruction. It is disabled in Phase 1.

## Closing ritual (mandatory — see E2)

After the maintainer approves the closing (E2), run these checks in order. If any is red, stop and fix before closing. Three steps, not six.

**First command: `scripts/closing.sh`.** It runs `git status --short`, `git log --oneline -5`, `git rev-parse HEAD`, then C2 (build --release, clippy --workspace --all-targets -- -D warnings, test --workspace, fmt --check, test-runner --check --seed 42), then `scripts/gates.sh closing`, then `scripts/capture.sh pause "<testo -> promozioni>"` for the closing event. It prints `NEXT: write bookmark, commit, push`. It does NOT write the bookmark, does NOT commit, does NOT push: the bookmark is synthesis, the commit is the maintainer's decision. Tested end-to-end on 2026-11-08 (exit 0, C2 green, gates green).

### 1. Verify

    git status --short
    git log --oneline -5
    git rev-parse HEAD

Then C2 (build, clippy, test, fmt, test-runner) and `scripts/gates.sh closing`. Both must be green before step 2. If red, stop and fix.

### 2. Bookmark and capture

**2a — Workflow lessons.** If a command failed and was retried, or a tool turned out unavailable, and the lesson is likely to recur: add it to B1-bis. If one-off, no action.

**2b — Rules self-audit.** Only if this session modified `NEW-SESSION-PROMPT.md`, `DECISION-PATTERNS.md`, a hard-stop, the startup or the closing. For each rule change: which rule does it touch (`file:line`)? does it contradict one? does it open a leak (who controls the condition)? does it assume a mechanism that does not exist? If any answer is bad, pause and ask. Output: `Rules-Audit: N changes, 0 contradictions, 0 leaks` or `findings=[...]`. If no rule was touched, the trailer is omitted.

**2c — Bookmark.** Update §1–§3 of §F: LCV (S1: the commit before this update), Open items (ID, title, first action), Plan (step, dependency, first action). No `Files touched`, no `Done`: `git log` is the log.
**2d — The briefing note is mandatory (E2, emended 2026-11-09).** Update `docs/context/NOTA-PER-NUOVA-AI.md` before the closing commit: add the new entries (new meta-instructions, design principles that emerged in chat and have no other home, rules for formulas, traps not in B1-bis), correct the stale ones, update the date and the LCV. If nothing new emerged, append `**YYYY-MM-DD — no change** (session N, LCV \`xxxxxxx\`)` to its §11 changelog: the "nothing to say" is a visible artefact, not a silence. The note is never the bookmark: if a voice answers "where are we", it goes in `SESSION-STATE.md`; if it answers "how to work", it goes here. `Rules-Audit` is not required for this update unless a rule file is also touched.
**The filter (added 2026-11-09):** a new entry enters only if it is not already mandatory in this file (if it is, the entry is a pointer, not a restatement), it carries a `why-new:` line (what a new agent would get wrong without it), it is one physical line (~200 chars), and the note stays within **200 lines** (merge or remove before adding; if impossible, write `overflow: <reason>` and the maintainer decides). Report artefacts: `dup-check: <term> -> prompt:<rule>|none`, `wc -l` of the note, `replaced:` if applied.

### 3. Commit, push, report

    git add docs/context/SESSION-STATE.md
    git commit -t .gitmessage
    git push
    git status -sb

The `pre-push` hook runs `scripts/gates.sh closing`. `--no-verify` is a protocol violation. `git status -sb` must show `## main...origin/main`, no `[ahead]` or `[behind]`.

Then the report: what was done (one line per commit), what was verified (real output), what was **not** done (and why), the state of the repo after the push, the next step. **The Plan table, in chat** — the maintainer must see it before the session ends.

If a check was skipped, say so. "I skipped X" is honest; "I ran X" when you did not is a lie.
````
