//! The `TestRegistry`: a collection of scenarios, executed together.

use crate::context::TestContext;
use crate::feature::Feature;
use crate::result::ScenarioResult;
use crate::scenario::Scenario;

/// A collection of scenarios.
///
/// The registry is the aggregation point: core, plugins, and tools all
/// register their scenarios here. The runner (e.g. `ds-test-runner`) creates
/// one registry, registers the known scenarios, and executes them.
#[derive(Default)]
pub struct TestRegistry {
    scenarios: Vec<Box<dyn Scenario>>,
}

impl TestRegistry {
    /// Create an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a scenario.
    pub fn register(&mut self, scenario: impl Scenario + 'static) {
        self.scenarios.push(Box::new(scenario));
    }

    /// Return the registered scenarios (in registration order).
    #[must_use]
    pub fn scenarios(&self) -> &[Box<dyn Scenario>] {
        &self.scenarios
    }

    /// Execute every registered scenario with the given seed.
    #[must_use]
    pub fn run_all(&self, seed: u64) -> RunReport {
        let mut results = Vec::with_capacity(self.scenarios.len());
        for scenario in &self.scenarios {
            let mut ctx = TestContext::new(seed);
            let result = scenario.run(&mut ctx);
            results.push(ScenarioOutcome {
                name: scenario.name(),
                feature: scenario.feature(),
                result,
                logs: ctx.logs().to_vec(),
            });
        }
        RunReport { outcomes: results }
    }
}

/// The result of one scenario in a run.
pub struct ScenarioOutcome {
    /// Scenario name.
    pub name: &'static str,
    /// Feature exercised.
    pub feature: Feature,
    /// The scenario's result.
    pub result: ScenarioResult,
    /// Diagnostic logs collected during the run.
    pub logs: Vec<String>,
}

/// The aggregate result of a [`TestRegistry::run_all`] call.
pub struct RunReport {
    outcomes: Vec<ScenarioOutcome>,
}

impl RunReport {
    /// The individual scenario outcomes.
    #[must_use]
    pub fn outcomes(&self) -> &[ScenarioOutcome] {
        &self.outcomes
    }

    /// `true` if every scenario passed.
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.outcomes.iter().all(|o| o.result.is_ok())
    }

    /// Total number of checks across all scenarios.
    #[must_use]
    pub fn total_checks(&self) -> usize {
        self.outcomes.iter().map(|o| o.result.checks.len()).sum()
    }

    /// Number of passing checks.
    #[must_use]
    pub fn passed_checks(&self) -> usize {
        self.outcomes
            .iter()
            .flat_map(|o| &o.result.checks)
            .filter(|c| c.passed)
            .count()
    }

    /// Number of failing checks.
    #[must_use]
    pub fn failed_checks(&self) -> usize {
        self.total_checks() - self.passed_checks()
    }
}
