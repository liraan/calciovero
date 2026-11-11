//! Feature scenarios for `pcfm-test-runner`.
//!
//! Each submodule registers the scenarios for one feature area. The
//! `register_all` function is the only entry point the rest of the binary
//! uses; when L1/L2/L3 modules exist, their registration goes here.

pub mod l0_match;

use ds_testkit::TestRegistry;

/// Register every scenario known to `pcfm-test-runner`.
///
/// Called once at startup. When L1, L2, L3, or plugin scenarios exist,
/// they are added here.
pub fn register_all(registry: &mut TestRegistry) {
    l0_match::register(registry);

    // TODO(L1): l1_world::register(registry);
    // TODO(L1): l1_season::register(registry);
    // TODO(L1): l1_roster::register(registry);
    // TODO(L2): l2_market::register(registry);
    // TODO(L2): l2_finance::register(registry);
    // TODO(L3): l3_ui::register(registry);
    // TODO(plugins): pcfm_plugin_gui::register_tests(registry);
}
