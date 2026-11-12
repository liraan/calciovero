//! The `Scenario` trait: a self-contained unit of testing.

use crate::context::TestContext;
use crate::feature::Feature;
use crate::result::ScenarioResult;

/// A self-contained test scenario.
///
/// A scenario exercises one feature of the framework (or of a plugin) and
/// returns a [`ScenarioResult`] with one or more [`Check`](crate::Check)s.
///
/// Scenarios are registered in a [`TestRegistry`](crate::TestRegistry) and
/// executed by a runner. They must be **deterministic** for a given
/// [`TestContext`] seed, so that failures are reproducible.
pub trait Scenario: Send + Sync {
    /// Stable identifier, used in reports and filtering (`--scenario NAME`).
    fn name(&self) -> &'static str;

    /// The feature this scenario exercises. Used for coverage reporting.
    fn feature(&self) -> Feature;

    /// Optional human-readable description.
    fn description(&self) -> &'static str {
        ""
    }

    /// Run the scenario. Must not panic; return a failed result instead.
    fn run(&self, ctx: &mut TestContext) -> ScenarioResult;
}
