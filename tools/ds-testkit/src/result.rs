//! Result types for scenarios.

/// The outcome of a single check inside a scenario.
#[derive(Debug, Clone)]
pub struct Check {
    /// Stable identifier of the check (used in reports).
    pub name: &'static str,
    /// `true` if the check passed.
    pub passed: bool,
    /// Optional human-readable detail (failure reason, measured value).
    pub detail: Option<String>,
}

impl Check {
    /// Create a passing check.
    ///
    /// Not `const`: the struct contains a `String` field (`detail`), which
    /// cannot be constructed in a const context on stable Rust.
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn pass(name: &'static str) -> Self {
        Self {
            name,
            passed: true,
            detail: None,
        }
    }

    /// Create a failing check with a reason.
    ///
    /// Not `const`: `Into<String>` is not const-callable on stable Rust.
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn fail(name: &'static str, reason: impl Into<String>) -> Self {
        Self {
            name,
            passed: false,
            detail: Some(reason.into()),
        }
    }
}

/// The outcome of a scenario.
#[derive(Debug, Clone, Default)]
pub struct ScenarioResult {
    /// All checks performed by the scenario (in order).
    pub checks: Vec<Check>,
    /// If the scenario could not run at all, the error message.
    pub error: Option<String>,
}

impl ScenarioResult {
    /// Create an empty result (no checks yet, no error).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a check.
    pub fn record(&mut self, check: Check) {
        self.checks.push(check);
    }

    /// Record a passing check.
    ///
    /// Not `const`: calls `record`, which mutates a `Vec` (`Vec::push` is
    /// not const on stable Rust).
    #[allow(clippy::missing_const_for_fn)]
    pub fn pass(&mut self, name: &'static str) {
        self.record(Check::pass(name));
    }

    /// Record a failing check.
    ///
    /// Not `const`: calls `record`, which mutates a `Vec`.
    #[allow(clippy::missing_const_for_fn)]
    pub fn fail(&mut self, name: &'static str, reason: impl Into<String>) {
        self.record(Check::fail(name, reason));
    }

    /// Record a scenario-level error (the scenario could not run).
    pub fn error(&mut self, reason: impl Into<String>) {
        self.error = Some(reason.into());
    }

    /// `true` if the scenario ran to completion and all checks passed.
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.error.is_none() && self.checks.iter().all(|c| c.passed)
    }
}
