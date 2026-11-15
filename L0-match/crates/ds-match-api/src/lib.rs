//! `ds-match-api` — the contract of a match plugin (ADR-063).
//!
//! The match core keeps only the clock, the observable state of the
//! pitch, the registry of the plugins, the events, and the record and
//! replay. Everything else the match does is a plugin, and every plugin
//! speaks to the core through this contract: the phase of the tick it
//! acts in, and its card (ADR-012: name, version, what it reads, what it
//! writes, its weight in the level of detail, the texts it gives the
//! commentary).
//!
//! # Design rules
//!
//! - No dependency on any `ds-*` crate (ADR-063 §3), like `ds-testkit`.
//! - No I/O, no clock, no randomness: a plugin is deterministic (ADR-035).
//! - Within a phase nobody reads what another writes in that phase
//!   (ADR-063 §2): every phase reads the world as the previous one left it.

pub mod card;
pub mod phase;
pub mod plugin;
pub mod world;

pub use card::{PluginCard, Weight};
pub use phase::Phase;
pub use plugin::{MANDATORY, MatchPlugin, OPTIONAL};
pub use world::{PlayerKey, World};
