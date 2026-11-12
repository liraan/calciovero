//! `ds-demo` — simulate a PCFM match and print the result.
//!
//! This is the demo binary split out of `pcfm-debug` (see ADR-020). It
//! builds two hardcoded squads, simulates a match, and renders the result
//! as text, JSON, or a self-contained HTML page.
//!
//! # Usage
//!
//! ```text
//! ds-demo [--seed <U64>] [--json] [--html <PATH>] [--quiet] [--help]
//! ```

mod board;
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
    --quiet         Print only the final score
    --help          Print this help

EXAMPLE:
    ds-demo --seed 42 --html /tmp/match.html
";

/// Parsed command-line arguments.
#[derive(Debug)]
struct Args {
    seed: u64,
    json: bool,
    html: Option<String>,
    quiet: bool,
    board: bool,
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
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    if [json, html.is_some(), board].iter().filter(|x| **x).count() > 1 {
        return Err("--json, --html and --board are mutually exclusive".to_string());
    }

    Ok(Some(Args {
        seed,
        json,
        html,
        quiet,
        board,
    }))
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let Some(args) = parse_args()? else {
        return Ok(());
    };

    let match_ = ds_fixtures::match_fixture(args.seed)?;
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

#[cfg(test)]
mod tests {
    use super::*;

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
