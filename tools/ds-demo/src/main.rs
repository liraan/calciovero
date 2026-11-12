//! `ds-demo` — simulate a PCFM match and print the result.
//!
//! This is the demo binary split out of `pcfm-debug` (see ADR-020). It
//! builds two hardcoded squads, simulates a match, and renders the result
//! as text, JSON, or a self-contained HTML page.
//!
//! # Usage
//!
//! ```text
//! ds-demo [--seed <U64>] [--json] [--html <PATH>] [--board] [--tui ...] [--quiet] [--help]
//! ```

mod board;
mod pitchview;
mod render;
mod viewmodel;

use std::process::ExitCode;

use ds_match::MatchConfig;

const HELP: &str = "\
ds-demo — simulate a PCFM match and print the result

USAGE:
    ds-demo [OPTIONS]

OPTIONS:
    --seed <U64>    RNG seed (default: 0)
    --json          Print the result as JSON
    --board         Print a board (score, events, narrative, hash)
    --html <PATH>   Write a self-contained HTML page to PATH
    --tui           Replay the match on a text pitch (2D debug view)
    --quiet         Print only the final score
    --help          Print this help

TUI OPTIONS (with --tui):
    --speed <N>     Match seconds per real second (default: 30)
    --from <MIN>    Start at this minute (default: 0)
    --to <MIN>      Stop at this minute (default: the end)
    --step          Advance one frame each time Enter is pressed
    --frame <MIN>   Print the frame at this minute and exit (no animation)
    --cols <N>      Pitch width in characters (default: 70)

EXAMPLES:
    ds-demo --seed 42 --html /tmp/match.html
    ds-demo --seed 42 --tui --speed 60
    ds-demo --seed 42 --tui --frame 12
";

/// Parsed command-line arguments.
#[derive(Debug)]
struct Args {
    seed: u64,
    json: bool,
    html: Option<String>,
    quiet: bool,
    board: bool,
    tui: Option<TuiArgs>,
}

/// Options of the 2D debug replay.
#[derive(Debug, Clone, PartialEq)]
struct TuiArgs {
    speed: u32,
    from_min: u32,
    to_min: Option<u32>,
    step: bool,
    frame_min: Option<u32>,
    cols: usize,
}

impl Default for TuiArgs {
    fn default() -> Self {
        Self {
            speed: 30,
            from_min: 0,
            to_min: None,
            step: false,
            frame_min: None,
            cols: 70,
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("ds-demo: {e}");
            ExitCode::FAILURE
        }
    }
}

fn parse_args() -> Result<Option<Args>, String> {
    parse_args_from(std::env::args().skip(1))
}

/// Parses arguments from an iterator. Split from `parse_args` so the
/// validation logic is testable without touching the process environment.
fn parse_args_from<I: Iterator<Item = String>>(mut iter: I) -> Result<Option<Args>, String> {
    let mut seed = 0u64;
    let mut json = false;
    let mut html = None;
    let mut quiet = false;
    let mut board = false;
    let mut tui = false;
    let mut t = TuiArgs::default();
    let mut tui_option = false;

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                print!("{HELP}");
                return Ok(None);
            }
            "--seed" => {
                let v = iter.next().ok_or("--seed requires a value")?;
                seed = v.parse().map_err(|_| format!("invalid seed: {v}"))?;
            }
            "--json" => json = true,
            "--html" => {
                let v = iter.next().ok_or("--html requires a path")?;
                html = Some(v);
            }
            "--quiet" => quiet = true,
            "--board" => board = true,
            "--tui" => tui = true,
            "--speed" | "--from" | "--to" | "--frame" | "--cols" => {
                tui_option = true;
                let v = iter
                    .next()
                    .ok_or_else(|| format!("{arg} requires a value"))?;
                let n: u32 = v.parse().map_err(|_| format!("invalid {arg}: {v}"))?;
                match arg.as_str() {
                    "--speed" => t.speed = n.max(1),
                    "--from" => t.from_min = n,
                    "--to" => t.to_min = Some(n),
                    "--frame" => t.frame_min = Some(n),
                    _ => t.cols = usize::try_from(n.clamp(20, 400)).unwrap_or(70),
                }
            }
            "--step" => {
                tui_option = true;
                t.step = true;
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    if [json, html.is_some(), board, tui]
        .iter()
        .filter(|x| **x)
        .count()
        > 1
    {
        return Err("--json, --html, --board and --tui are mutually exclusive".to_string());
    }
    if tui_option && !tui {
        return Err("--speed, --from, --to, --step, --frame and --cols need --tui".to_string());
    }

    Ok(Some(Args {
        seed,
        json,
        html,
        quiet,
        board,
        tui: tui.then_some(t),
    }))
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let Some(args) = parse_args()? else {
        return Ok(());
    };

    let match_ = ds_fixtures::match_fixture(args.seed)?;
    if let Some(t) = &args.tui {
        return run_tui(&match_, args.seed, t);
    }
    let result = ds_match::simulate_from_seed(&match_, args.seed, MatchConfig::default());

    if args.quiet {
        println!("{} - {}", result.score.home, result.score.away);
        return Ok(());
    }

    if args.board {
        let vm = viewmodel::MatchViewModel::from_result(&result);
        board::print_board(&vm)?;
        return Ok(());
    }

    if args.json {
        render::print_json(&result)?;
    } else {
        render::print_text(&result)?;
    }

    if let Some(path) = &args.html {
        render::write_html(&result, path)?;
        eprintln!("HTML written to {path}");
    }

    Ok(())
}

/// The 2D debug replay: regenerate the match from the seed with its tick
/// record, narrate it, and draw it frame by frame (M1, IDEA-INFRA-021).
fn run_tui(
    match_: &ds_model::Match,
    seed: u64,
    t: &TuiArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write as _;

    let config = MatchConfig::default();
    let tps = u32::from(config.ticks_per_second).max(1);
    let (result, records) = ds_match::simulate_from_seed_recorded(match_, seed, config);
    let ticks = ds_match::narrate_ticks(match_, &result.events, records);
    let rows = (t.cols * 3 / 10).max(10);
    let view = pitchview::PitchView::new(t.cols, rows, match_.pitch.length_m, match_.pitch.width_m);
    let index_of_minute = |min: u32| -> usize {
        ticks
            .iter()
            .position(|nt| nt.record.clock().0 >= min)
            .unwrap_or_else(|| ticks.len().saturating_sub(1))
    };

    let mut out = std::io::stdout().lock();
    if let Some(min) = t.frame_min {
        let at = index_of_minute(min);
        let log: Vec<_> = ticks[..=at]
            .iter()
            .flat_map(|nt| nt.entries.clone())
            .collect();
        write!(out, "{}", view.frame(&ticks[at], &log))?;
        return Ok(());
    }

    let from = index_of_minute(t.from_min);
    let to = t.to_min.map_or(usize::MAX, index_of_minute);
    // Ten frames per real second; each frame advances `speed / 10` match
    // seconds.
    let step = usize::try_from(t.speed * tps / 10).unwrap_or(1).max(1);
    let mut log = Vec::new();
    let stdin = std::io::stdin();
    for (nt, new) in pitchview::Playback::new(&ticks, from, to, step) {
        log.extend(new);
        // Clear the screen and draw from the top-left corner.
        write!(out, "\x1b[H\x1b[2J{}", view.frame(nt, &log))?;
        out.flush()?;
        if t.step {
            write!(out, " [Enter: next frame, Ctrl+C: quit]")?;
            out.flush()?;
            let mut line = String::new();
            if stdin.read_line(&mut line)? == 0 {
                break;
            }
        } else {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    writeln!(
        out,
        " final: HOME {} - {} AWAY   events: {}",
        result.score.home,
        result.score.away,
        result.events.len()
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tui_options_need_tui() {
        let err = parse(&["--speed", "60"]).expect_err("should reject");
        assert!(err.contains("need --tui"), "got: {err}");
    }

    #[test]
    fn tui_options_are_parsed() {
        let args = parse(&[
            "--tui", "--speed", "60", "--from", "10", "--to", "20", "--step",
        ])
        .expect("parse")
        .expect("some");
        let t = args.tui.expect("tui");
        assert_eq!(
            (t.speed, t.from_min, t.to_min, t.step),
            (60, 10, Some(20), true)
        );
    }

    #[test]
    fn tui_rejects_board() {
        let err = parse(&["--tui", "--board"]).expect_err("should reject");
        assert!(err.contains("mutually exclusive"), "got: {err}");
    }

    #[test]
    fn demo_match_is_valid() {
        let m = ds_fixtures::match_fixture(42).unwrap();
        assert_eq!(m.home.players.len(), 11);
        assert_eq!(m.away.players.len(), 11);
    }

    #[test]
    fn rendering_a_result_does_not_panic() {
        let match_ = ds_fixtures::match_fixture(42).unwrap();
        let result = ds_match::simulate_from_seed(&match_, 42, MatchConfig::default());
        let _ = render::print_text(&result);
        let _ = render::print_json(&result);
    }

    fn parse(argv: &[&str]) -> Result<Option<Args>, String> {
        parse_args_from(argv.iter().map(|s| (*s).to_string()))
    }

    #[test]
    fn board_flag_is_accepted_alone() {
        let args = parse(&["--board"]).expect("parse").expect("some");
        assert!(args.board);
        assert!(!args.json);
        assert!(args.html.is_none());
        assert!(!args.quiet);
    }

    #[test]
    fn board_rejects_json() {
        let err = parse(&["--board", "--json"]).expect_err("should reject");
        assert!(err.contains("mutually exclusive"), "got: {err}");
    }

    #[test]
    fn board_rejects_html() {
        let err = parse(&["--board", "--html", "x.html"]).expect_err("should reject");
        assert!(err.contains("mutually exclusive"), "got: {err}");
    }

    #[test]
    fn json_rejects_html() {
        let err = parse(&["--json", "--html", "x.html"]).expect_err("should reject");
        assert!(err.contains("mutually exclusive"), "got: {err}");
    }

    #[test]
    fn quiet_alone_is_accepted() {
        let args = parse(&["--quiet"]).expect("parse").expect("some");
        assert!(args.quiet);
        assert!(!args.board);
    }
}
