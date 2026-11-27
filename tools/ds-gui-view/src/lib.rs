//! The record-driven 2.5D view, shared by the desktop and Android GUIs
//! (ADR-055, decision U: one recording, many views).
//!
//! **One responsibility:** turn a match's tick record into a view — the
//! camera, the interpolated frame, the gestures read from events and the
//! narration. Nothing here goes back to the simulation. The modules are the
//! ones the first 2.5D view used; this crate gives them a home both GUIs can
//! depend on.

// Drawing only: the view's f32 curves and perspective never feed the simulation.
#![allow(clippy::disallowed_methods)]
#![allow(clippy::float_cmp)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::suboptimal_flops)]
#![allow(clippy::cast_precision_loss)]

pub mod animation;
pub mod gestures;
pub mod interp;
pub mod scene;
pub mod stadium;
pub mod view;

pub use view::{MatchView, Palette};
