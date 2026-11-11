//! Health check entry point.
//!
//! Builds a `TestRegistry`, registers all known scenarios (via
//! `features::register_all`), runs them with the given seed, and adapts
//! the resulting `RunReport` into the `CheckReport` shape the CLI expects.
//!
//! The actual test logic lives in `features/l0_match.rs` (and, in the
//! future, `features/l1_*`, `features/l2_*`, `features/l3_*`, plugins).
//! This file only knows how to run a registry and format the report.

use ds_testkit::TestRegistry;

use crate::features;

/// The report of a health check.
///
/// Preserved from the previous implementation for CLI compatibility.
pub struct CheckReport {
    /// The checks that passed.
    pub passed: Vec<&'static str>,
    /// The checks that failed, with a reason.
    pub failed: Vec<(&'static str, String)>,
}

impl CheckReport {
    /// Returns `true` if all checks passed.
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.failed.is_empty()
    }

    /// Returns the total number of checks.
    #[must_use]
    pub fn total(&self) -> usize {
        self.passed.len() + self.failed.len()
    }
}

/// Runs the health check.
///
/// # Panics
///
/// Does not panic. Scenarios that panic are caught by the runtime and
/// reported as a failing check named `"scenario"`.
#[must_use]
pub fn run(seed: u64) -> CheckReport {
    let mut registry = TestRegistry::new();
    features::register_all(&mut registry);
    let report = registry.run_all(seed);
    adapt(&report)
}

/// Flatten a `RunReport` into the `CheckReport` shape expected by the CLI.
///
/// One check = one line of output. Scenario-level errors are surfaced as
/// a failing check named `"scenario"`.
fn adapt(report: &ds_testkit::RunReport) -> CheckReport {
    let mut out = CheckReport {
        passed: Vec::new(),
        failed: Vec::new(),
    };
    for outcome in report.outcomes() {
        if let Some(err) = &outcome.result.error {
            out.failed
                .push(("scenario", format!("{}: {}", outcome.name, err)));
        }
        for check in &outcome.result.checks {
            if check.passed {
                out.passed.push(check.name);
            } else {
                out.failed
                    .push((check.name, check.detail.clone().unwrap_or_default()));
            }
        }
    }
    out
}
