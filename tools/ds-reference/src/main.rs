//! `ds-reference` — real football beside the engine.
//!
//! ```text
//! ds-reference wyscout --events events_Italy.json --matches matches_Italy.json \
//!     --label "Serie A 2017-18" --out L0-match/reference/serie-a-2017-18.json
//! ds-reference compare --reference L0-match/reference/serie-a-2017-18.json --seeds 42..71
//! ds-reference bundle --manifest L0-match/reference/bundle.json --out bundle.json
//! ds-reference scene --seeds 43..43
//! ```

use std::process::ExitCode;

use ds_reference::{Metric, Summary, bundle, engine, round3, summarise, wyscout};

const HELP: &str = "\
ds-reference — real football beside the engine

USAGE:
    ds-reference wyscout --events <PATH> --matches <PATH> --label <TEXT> [--out <PATH>]
        Per-match averages of the public Wyscout soccer-logs (CC BY 4.0).
        Without --out, print the JSON.
    ds-reference compare --reference <PATH> [--seeds A..B]
        Play the engine on the seeds (default 42..71) and print both sides.
    ds-reference bundle --manifest <PATH> [--out <PATH>]
        Flatten a bundle of real-match sources into one reference file.
        A team aggregate may be listed but cannot be truth for a metric.
        Without --out, print the JSON.
    ds-reference scene [--seeds A..B]
        Play the engine on the seeds (default 42..51) and print what the
        matches look like: teleports, the free ball, the keeper's mates in
        his area, the half the ball is in, the passes' direction, the
        forwards ahead of the ball, the ball behind its carrier.
    ds-reference --help
";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("ds-reference: {e}");
            ExitCode::FAILURE
        }
    }
}

fn value<'a>(args: &'a [String], key: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("wyscout") => run_wyscout(&args),
        Some("bundle") => run_bundle(&args),
        Some("compare") => run_compare(&args),
        Some("scene") => run_scene(&args),
        _ => {
            print!("{HELP}");
            Ok(())
        }
    }
}

fn run_scene(args: &[String]) -> Result<(), String> {
    let (first, last) = parse_seeds(value(args, "--seeds").unwrap_or("42..51"))?;
    let mut all = Vec::new();
    for seed in first..=last {
        let m = ds_fixtures::match_fixture(seed)?;
        let pitch = (
            i32::from(m.pitch.length_m) * 100,
            i32::from(m.pitch.width_m) * 100,
        );
        let (result, record) =
            ds_match::simulate_from_seed_recorded(&m, seed, ds_match::MatchConfig::default());
        all.push(ds_reference::scene::measure(
            &record,
            result.events.events(),
            pitch,
        ));
    }
    println!(
        "engine {} (seeds {first}..{last}, {} matches)",
        ds_match::SIM_VERSION,
        all.len()
    );
    print!(
        "{}",
        ds_reference::scene::report(&ds_reference::scene::sum(&all), all.len())
    );
    Ok(())
}

fn read<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, String> {
    let f = std::fs::File::open(path).map_err(|e| format!("{path}: {e}"))?;
    serde_json::from_reader(std::io::BufReader::new(f)).map_err(|e| format!("{path}: {e}"))
}

fn run_wyscout(args: &[String]) -> Result<(), String> {
    let events_path = value(args, "--events").ok_or("--events is required")?;
    let matches_path = value(args, "--matches").ok_or("--matches is required")?;
    let label = value(args, "--label").ok_or("--label is required")?;
    let events: Vec<wyscout::Event> = read(events_path)?;
    let matches: Vec<wyscout::Match> = read(matches_path)?;
    let summary = summarise(&wyscout::measure(&events, &matches));
    let json = reference_json(label, &summary);
    let text = serde_json::to_string_pretty(&json).map_err(|e| e.to_string())? + "\n";
    if let Some(out) = value(args, "--out") {
        return std::fs::write(out, text).map_err(|e| format!("{out}: {e}"));
    }
    print!("{text}");
    Ok(())
}

fn run_bundle(args: &[String]) -> Result<(), String> {
    let path = value(args, "--manifest").ok_or("--manifest is required")?;
    let manifest = bundle::read_manifest(path)?;
    let sources = bundle::load_sources(path, &manifest)?;
    let json = bundle::build(&manifest, &sources)?;
    let text = serde_json::to_string_pretty(&json).map_err(|e| e.to_string())? + "\n";
    if let Some(out) = value(args, "--out") {
        return std::fs::write(out, text).map_err(|e| format!("{out}: {e}"));
    }
    print!("{text}");
    Ok(())
}

fn reference_json(label: &str, s: &Summary) -> serde_json::Value {
    let metrics: serde_json::Map<String, serde_json::Value> = s
        .stats
        .iter()
        .map(|(m, st)| {
            let mut v = serde_json::json!({ "mean": st.mean, "sd": st.sd });
            if !m.note().is_empty() {
                v["note"] = serde_json::Value::from(m.note());
            }
            (m.key().to_owned(), v)
        })
        .collect();
    serde_json::json!({
        "label": label,
        "per": "match, both teams together",
        "matches": s.matches,
        "source": {
            "data": "Wyscout soccer-logs, Pappalardo, L. and Massucco, E. (2019), figshare, doi:10.6084/m9.figshare.c.4415000",
            "paper": "Pappalardo, L., Cintia, P., Rossi, A. et al. (2019), A public data set of spatio-temporal match events in soccer competitions, Scientific Data 6, 236, doi:10.1038/s41597-019-0247-7",
            "licence": "CC BY 4.0 (https://creativecommons.org/licenses/by/4.0/)",
            "made_by": "tools/ds-reference (the raw data is not in the repository)"
        },
        "metrics": metrics
    })
}

fn parse_seeds(s: &str) -> Result<(u64, u64), String> {
    let (a, b) = s.split_once("..").ok_or("--seeds is A..B (inclusive)")?;
    let a: u64 = a.parse().map_err(|_| "--seeds: A is not a number")?;
    let b: u64 = b.parse().map_err(|_| "--seeds: B is not a number")?;
    if b < a {
        return Err("--seeds: B is before A".into());
    }
    Ok((a, b))
}

fn run_compare(args: &[String]) -> Result<(), String> {
    let path = value(args, "--reference").ok_or("--reference is required")?;
    let reference: serde_json::Value = read(path)?;
    let (first, last) = parse_seeds(value(args, "--seeds").unwrap_or("42..71"))?;
    let played = (first..=last)
        .map(engine::play)
        .collect::<Result<Vec<_>, _>>()?;
    let ours = summarise(&played);
    println!(
        "{} ({} matches)  vs  engine {} (seeds {first}..{last}, {} matches)",
        reference["label"].as_str().unwrap_or("reference"),
        reference["matches"],
        ds_match::SIM_VERSION,
        ours.matches
    );
    println!(
        "{:<26} {:>9} {:>8} {:>9} {:>8} {:>8}",
        "metric", "real", "±", "engine", "±", "ratio"
    );
    for m in Metric::ALL {
        let real = &reference["metrics"][m.key()];
        let (rm, rs) = (real["mean"].as_f64(), real["sd"].as_f64());
        let comparable = real["comparable"]
            .as_bool()
            .unwrap_or_else(|| m.comparable());
        let e = ours.stat(m).ok_or("missing metric")?;
        let ratio = match rm {
            Some(x) if x.abs() > 1e-9 => format!("{:.2}", round3(e.mean / x)),
            _ => "-".into(),
        };
        println!(
            "{:<26} {:>9} {:>8} {:>9.2} {:>8.2} {:>8}{}",
            m.key(),
            rm.map_or_else(|| "-".into(), |x| format!("{x:.2}")),
            rs.map_or_else(|| "-".into(), |x| format!("{x:.2}")),
            e.mean,
            e.sd,
            ratio,
            if comparable { "" } else { "  (not comparable)" }
        );
    }
    for m in Metric::ALL {
        let real = &reference["metrics"][m.key()];
        if real["comparable"]
            .as_bool()
            .unwrap_or_else(|| m.comparable())
        {
            continue;
        }
        let note = real["note"]
            .as_str()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| m.note());
        if !note.is_empty() {
            println!("{}: {}", m.key(), note);
        }
    }
    Ok(())
}
