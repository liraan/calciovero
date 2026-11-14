//! Board renderer for `ds-demo` (ADR-025).
//!
//! Fixed-width ASCII, no ANSI colour codes, no external dependencies.
//! Reads a [`MatchViewModel`] only: never `MatchResult`, the engine's
//! internal match state, or Plane A. The board is a *consumer* of the view
//! model, not a second simulation.
//!
//! Two functions, one body:
//! - [`render_board`] returns the board as a `String` (testable, pure).
//! - [`print_board`] writes it to stdout (the CLI entry point).

use std::fmt::Write as _;

use crate::viewmodel::{EventRow, MatchViewModel, team_label};

/// Renders the match board as a plain string.
///
/// # Errors
///
/// Propagates the `fmt::Error` of the inner `String` writes.
pub fn render_board(vm: &MatchViewModel) -> Result<String, std::fmt::Error> {
    let mut out = String::new();

    writeln!(
        out,
        "+--------------------------------------------------------------+"
    )?;
    writeln!(
        out,
        "|         CALCIO VERO - DEEP SIMULATOR -- MATCH BOARD          |"
    )?;
    writeln!(
        out,
        "+--------------------------------------------------------------+"
    )?;
    writeln!(out)?;

    writeln!(
        out,
        "  Score        {:>3}  -  {:<3}",
        vm.score.0, vm.score.1
    )?;
    writeln!(
        out,
        "  Possession   {:>5.1}%    {:>5.1}%",
        vm.possession_pct.0, vm.possession_pct.1
    )?;
    writeln!(
        out,
        "  Shots        {:>3}     {:>3}",
        vm.shots.0, vm.shots.1
    )?;
    writeln!(
        out,
        "  Goals        {:>3}     {:>3}",
        vm.goals.0, vm.goals.1
    )?;
    writeln!(out)?;

    writeln!(out, "  State hash   {}", vm.state_hash)?;
    writeln!(out)?;

    writeln!(
        out,
        "  -- Events ({}) ----------------------------------------------",
        vm.events.len()
    )?;
    if vm.events.is_empty() {
        writeln!(out, "  (no events)")?;
    } else {
        for event in &vm.events {
            writeln!(
                out,
                "  {:<12}  {:<20}  {} {}",
                format_time(event),
                event.kind,
                team_label(event.team),
                event.number
            )?;
        }
    }
    writeln!(out)?;

    writeln!(
        out,
        "  -- Narrative ({}) -------------------------------------------",
        vm.narrative.len()
    )?;
    if vm.narrative.is_empty() {
        writeln!(out, "  (no narrative)")?;
    } else {
        for entry in &vm.narrative {
            writeln!(
                out,
                "  {:>3}'  [{:<8}]  {}",
                entry.minute, entry.importance, entry.text
            )?;
        }
    }

    Ok(out)
}

/// Prints the match board to stdout.
///
/// Thin wrapper over [`render_board`]: the formatting lives there, this
/// function only writes. The two cannot diverge.
///
/// # Errors
///
/// Propagates the `fmt::Error` of `render_board`.
pub fn print_board(vm: &MatchViewModel) -> std::fmt::Result {
    let board = render_board(vm)?;
    print!("{board}");
    Ok(())
}

/// Formats an event time: `1T 12'34"` inside regulation, `1T 45'+2'` for
/// stoppage. The half is explicit (ADR-026): the second half restarts from 0.
fn format_time(event: &EventRow) -> String {
    let label = match event.half {
        ds_stats::Half::First => "1T",
        ds_stats::Half::Second => "2T",
    };
    if event.minute > 45 {
        format!("{label} 45'+{}'", event.minute - 45)
    } else {
        format!("{label} {}'{:02}\"", event.minute, event.second)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::viewmodel::{EventRow, NarrativeRow, Team};

    fn empty_vm() -> MatchViewModel {
        MatchViewModel {
            score: (0, 0),
            possession_pct: (50.0, 50.0),
            shots: (0, 0),
            goals: (0, 0),
            events: Vec::new(),
            narrative: Vec::new(),
            state_hash: "0".repeat(64),
        }
    }

    #[test]
    fn empty_board_renders_without_panic() {
        let vm = empty_vm();
        let rendered = render_board(&vm).expect("render");
        assert!(rendered.contains("CALCIO VERO - DEEP SIMULATOR -- MATCH BOARD"));
        assert!(rendered.contains("(no events)"));
        assert!(rendered.contains("(no narrative)"));
    }

    #[test]
    fn board_renders_score_and_events() {
        let mut vm = empty_vm();
        vm.score = (2, 1);
        vm.events.push(EventRow {
            minute: 12,
            second: 34,
            kind: "GOAL",
            half: ds_stats::Half::First,
            team: Team::Away,
            number: 9,
        });
        let rendered = render_board(&vm).expect("render");
        assert!(rendered.contains("2  -  1"));
        assert!(rendered.contains("GOAL"));
        assert!(rendered.contains("12'34"));
        assert!(rendered.contains("Away 9"));
    }

    #[test]
    fn board_renders_narrative() {
        let mut vm = empty_vm();
        vm.narrative.push(NarrativeRow {
            minute: 7,
            text: "GOAL!".to_string(),
            importance: "critical",
        });
        let rendered = render_board(&vm).expect("render");
        assert!(rendered.contains("GOAL!"));
        assert!(rendered.contains("[critical]"));
    }

    #[test]
    fn board_renders_state_hash() {
        let mut vm = empty_vm();
        vm.state_hash = "deadbeef".to_string();
        let rendered = render_board(&vm).expect("render");
        assert!(rendered.contains("deadbeef"));
    }
}
