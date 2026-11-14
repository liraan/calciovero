# Real football, measured

**What this is.** Per-match averages of real matches, measured from open
data by [`tools/ds-reference`](../../tools/ds-reference/): what the engine is
compared against in step 11 of `context/plans/L0-CLOSE.md`. These are
measurements, not parameters. The engine is never tuned on them (RULES
A5-bis, ADR-052): a distance from these numbers is a question for the
abilities, never a value to adjust.

| File | Matches | Source | Licence |
|---|---|---|---|
| `serie-a-2017-18.json` | 380 (the whole season) | Wyscout soccer-logs: Pappalardo, L. and Massucco, E. (2019), figshare, doi:10.6084/m9.figshare.c.4415000; described in Pappalardo, L., Cintia, P., Rossi, A. et al. (2019), *A public data set of spatio-temporal match events in soccer competitions*, Scientific Data 6, 236, doi:10.1038/s41597-019-0247-7 | CC BY 4.0 |

The raw data is not in the repository: only the averages, with their
source. Each metric is per match, both teams together; `sd` is the
spread between matches. A `note` says where the two sides do not count
the same thing.

**Checked against a second source:** 2.676 goals a match × 380 = 1017,
the season's total goals as published (Wikipedia, *2017-18 Serie A*;
futbol24, season statistics).

## Read it beside the engine

    cargo run --release -p ds-reference -- compare --reference L0-match/reference/serie-a-2017-18.json --seeds 42..71

Engine 103, seeds 42-71 (30 matches), 2026-11-14:

```
metric                          real        ±    engine        ±    ratio
goals                           2.68     1.65      4.67     2.02     1.74
own_goals                       0.10     0.31      1.50     1.48    15.46
shots                          24.60     5.46     32.73     6.86     1.33
shots_on_target                 8.69     3.22     17.90     4.47     2.06
penalties                       0.33     0.56      0.20     0.48     0.60
passes                        887.68   122.81   1115.77    39.50     1.26
passes_accurate               739.17   124.79   1004.33    38.29     1.36
crosses                        33.83     8.78     15.00     3.52     0.44  (not comparable)
aerial_duels                   38.14    12.98     13.43     3.08     0.35
ground_duels                  182.63    27.54    214.90    20.63     1.18
take_ons_won                   16.03     5.67      6.10     3.83     0.38
fouls                          26.00     6.69     21.83     6.29     0.84
yellow_cards                    3.97     1.96      4.87     1.71     1.23
red_cards                       0.24     0.49      0.03     0.18     0.14
corners                        10.30     3.58     19.90     4.61     1.93
throw_ins                      42.72     8.92      9.07     4.07     0.21
goal_kicks                     17.15     4.20      9.43     3.13     0.55
free_kicks                     29.45     6.81     20.90     4.19     0.71
offsides                        4.26     2.40     11.47     2.60     2.69
clearances                     29.53     8.65    158.03    18.18     5.35
interceptions                  89.86    13.97    328.07    22.43     3.65  (not comparable)
saves                           9.37     3.37     16.20     4.31     1.73
stoppage_first_half_min         1.49     1.35      6.33     2.51     4.26
stoppage_second_half_min        4.08     1.61      5.61     2.29     1.37
```

Ratios are engine / real. The ones marked *not comparable* count different things on the two sides (see each metric's `note`).
