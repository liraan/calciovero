//! Public test harness for PCFM.
//!
//! This crate defines the *contract* for testable units in the PCFM
//! ecosystem. It deliberately has **no dependency** on any other `pcfm-*`
//! crate: core, plugins, and tools all implement the same traits against
//! this crate, and the runner (`pcfm-test-runner`) aggregates them.
//!
//! # Design rules
//!
//! - No dependency on `ds-core`, `ds-match`, or any other crate.
//! - No I/O, no network, no environment access.
//! - Traits are object-safe so the registry can hold `Box<dyn Scenario>`.

pub mod context;
pub mod feature;
pub mod registry;
pub mod result;
pub mod scenario;

pub use context::TestContext;
pub use feature::Feature;
pub use registry::{RunReport, TestRegistry};
pub use result::{Check, ScenarioResult};
pub use scenario::Scenario;
