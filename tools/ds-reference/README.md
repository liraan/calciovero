# ds-reference — real football beside the engine

**Responsibility (one sentence):** measure open real-match data and the
engine the same way, so that step 11 of `L0-close` (the realism, derived)
reads distances with a source. It never changes the engine.

## Make the Serie A reference

The data is the public Wyscout soccer-logs (Pappalardo et al., 2019,
licence CC BY 4.0). Download it outside the repository:

    mkdir -p ~/wyscout && cd ~/wyscout
    curl -L -o events.zip  https://ndownloader.figshare.com/files/14464685
    curl -L -o matches.zip https://ndownloader.figshare.com/files/14464622
    unzip events.zip && unzip matches.zip

Then, from the repository:

    cargo run --release -p ds-reference -- wyscout \
        --events ~/wyscout/events_Italy.json --matches ~/wyscout/matches_Italy.json \
        --label "Serie A 2017-18" --out L0-match/reference/serie-a-2017-18.json

The same files hold the 2017-18 Premier League, Liga, Bundesliga and
Ligue 1 (`events_England.json`, ...), the 2018 World Cup and Euro 2016.

## Compare

    cargo run --release -p ds-reference -- compare \
        --reference L0-match/reference/serie-a-2017-18.json --seeds 42..71

Every metric is per match, both teams together. The definitions on the
two sides are in `src/wyscout.rs` and `src/engine.rs`; where they do not
count the same thing, the metric says so (`Metric::note`) and the report
marks it *not comparable*. A reference file may carry its own `note` and
`comparable` per metric; `compare` reads them and falls back to the enum, so
older files keep working.

## Bundle

    cargo run --release -p ds-reference -- bundle \
        --manifest L0-match/reference/bundle.json --out bundle-generated.json

The manifest names, for each metric, the real-match source that supplies it
(ADR-116). Adding a source is data, not a new provider in this crate. The
builder refuses a source that is not from real matches, has no licence, or
is a team aggregate: an aggregate is a cross-check, not a player's choice.
A new format needs only a `measure()` module beside `wyscout.rs` and a CLI
branch; the engine is untouched.
