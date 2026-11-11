//! Informational commands: `--list` and `--coverage`.
//!
//! These commands do not simulate a match. They inspect the registered
//! scenarios and print a report. They are the inspection layer of the
//! coretest framework: `--list` shows what exists, `--coverage` shows
//! what is covered and what is not.

use std::collections::HashSet;

use ds_testkit::{Feature, TestRegistry};

use crate::features;

/// Print the registered scenarios with their feature and description.
pub fn list() {
    let mut registry = TestRegistry::new();
    features::register_all(&mut registry);

    println!("=== SCENARI REGISTRATI ===");
    if registry.scenarios().is_empty() {
        println!("  (nessuno scenario registrato)");
        return;
    }

    let name_width = registry
        .scenarios()
        .iter()
        .map(|s| s.name().len())
        .max()
        .unwrap_or(0);

    for s in registry.scenarios() {
        let name = s.name();
        let feature = s.feature().label();
        let desc = s.description();
        if desc.is_empty() {
            println!("  {name:<name_width$}  [{feature}]");
        } else {
            println!("  {name:<name_width$}  [{feature}]  {desc}");
        }
    }
    println!();
    println!("{} scenari registrati.", registry.scenarios().len());
}

/// Print the feature coverage: which built-in features have at least one
/// registered scenario, and which do not.
pub fn coverage() {
    let mut registry = TestRegistry::new();
    features::register_all(&mut registry);

    let covered: HashSet<Feature> = registry.scenarios().iter().map(|s| s.feature()).collect();

    let all = Feature::all_builtin();
    let total = all.len();
    let covered_count = all.iter().filter(|f| covered.contains(f)).count();

    println!("=== COPERTURA FEATURE ===");
    for f in all {
        let mark = if covered.contains(f) { "✅" } else { "⬜" };
        println!("  [{mark}] {}", f.label());
    }
    println!();
    // `Feature::all_builtin()` is never empty, but `checked_div` keeps the
    // intent explicit and satisfies clippy::manual_checked_ops.
    let pct = (covered_count * 100).checked_div(total).unwrap_or(100);
    println!("{covered_count}/{total} feature coperte ({pct}%).");
}
