# Da rivedere al ritorno di Claude Code

Una riga per ogni commit di lavoro fatto senza Claude Code (l'operatore la aggiunge nel commit del segnalibro). Claude Code riparte da qui: legge le righe aperte, guarda solo i file indicati, corregge o conferma, e segna la riga.

Stato: **aperto** (da guardare), **ok** (confermato), **corretto in `<hash>`**.

| Data | Hash | Cosa (una riga) | File toccati | Perché (la catena o la fonte) | Misura (in banda prima → dopo) | Dubbi | Stato |
|---|---|---|---|---|---|---|---|
| 2026-11-25 | `4a97817` | SIM 210: il lancio guarda un passo avanti come il passaggio a terra (T33, OI-116 A) | `ds-orchestrator/src/read.rs`, `ds-match/src/save.rs`, golden, semi, `CHANGELOG-SIM.md`, `L0-CALIBRATION.md` | ADR-052: una misura sola per ogni scelta; prima solo il passaggio a terra guardava la giocata successiva; gemello `a_long_ball_to_a_man_with_a_free_pass_is_worth_more` | 12 → 10 su 22 (gol 4.98 → 6.93, angoli 7.23 → 18.97) | `receiver_best_pass` conta anche i lanci del ricevitore (RUN 17); angoli e autogol da indagare | aperto |
| 2026-11-25 | `b619af3` | SIM 211: dopo un blocco chi ha giocato aspetta il rimbalzo come dopo un controllo fallito | `ds-match/src/plugins/technique.rs`, `ds-match/src/save.rs`, golden, semi, guard in `tick_record.rs`, `CHANGELOG-SIM.md`, `L0-CALIBRATION.md` | ADR-051: il blocco dà il possesso ai difensori; RUN 32: 645/1084 block con il passatore di nuovo portatore al +1 | 9 su 22 (tiri 38.73 → 30.50, rimesse 17.77 → 33.32) | parate 6.75 → 5.08 fuori banda; angoli ancora alti | aperto |
