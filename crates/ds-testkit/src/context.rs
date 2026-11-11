//! Execution context passed to every scenario.

/// Context for a single run of a scenario.
///
/// Carries the seed (so scenarios can construct their own RNG) and a log
/// buffer the scenario can use for diagnostics.
pub struct TestContext {
    /// The seed for this run. Scenarios must derive their RNG from it.
    pub seed: u64,
    log: Vec<String>,
}

impl TestContext {
    /// Create a new context with the given seed.
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        Self {
            seed,
            log: Vec::new(),
        }
    }

    /// Append a line to the diagnostic log.
    pub fn log(&mut self, line: impl Into<String>) {
        self.log.push(line.into());
    }

    /// Return the accumulated log.
    #[must_use]
    pub fn logs(&self) -> &[String] {
        &self.log
    }
}
