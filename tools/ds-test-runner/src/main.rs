//! `ds-test-runner` — run Calcio Vero - Deep Simulator test scenarios.
//!
//! Formerly `pcfm-debug` (see ADR-020). The demo simulation was split into
//! `ds-demo`; this crate only runs and inspects test scenarios.
//!
//! # Usage
//!
//! ```text
//! ds-test-runner [--seed <U64>]
//! ds-test-runner --check
//! ds-test-runner --list
//! ds-test-runner --coverage
//! ```
//!
//! With no command flag, `--check` is the default.

#![deny(missing_docs)]

mod args;
mod check;
mod commands;
mod features;

use std::process::ExitCode;

use crate::args::{ArgError, HELP, parse};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("ds-test-runner: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = match parse(std::env::args()) {
        Ok(a) => a,
        Err(ArgError::HelpRequested) => {
            print!("{HELP}");
            return Ok(());
        }
        Err(e) => {
            eprintln!("ds-test-runner: {e}");
            eprintln!();
            eprint!("{HELP}");
            return Err(Box::new(e));
        }
    };

    if args.list {
        commands::list();
        return Ok(());
    }
    if args.coverage {
        commands::coverage();
        return Ok(());
    }

    // `--check` is the default when no other command is given.
    let report = check::run(args.seed);
    println!("=== HEALTH CHECK ===");
    for p in &report.passed {
        println!("  ✅ {p}");
    }
    for (name, reason) in &report.failed {
        println!("  ❌ {name}: {reason}");
    }
    println!();
    if report.is_ok() {
        println!("  {}/{} checks passed", report.passed.len(), report.total());
        return Ok(());
    }
    Err("health check failed".into())
}
