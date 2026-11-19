//! The 2D debug view: one narrated tick projected on a text pitch (M1).
//!
//! **One responsibility:** turn a narrated tick into a frame a person can
//! read in a terminal. It is the last link of the chain decided for the
//! 2D (IDEA-INFRA-021): tick record -> narrator -> 2D. It reads only the
//! narrated ticks (Plane B); it never reads the engine.
//!
//! **It invents nothing.** Positions are the background; the only events
//! shown are the narrated events of the ticks played (WORLD-MODEL §4: two
//! players close together are not a tackle unless the engine emitted
//! one). Over a whole playback every event appears exactly once.
//!
//! **No colour.** The doctrine bans hardcoded colours (foundations §5): the
//! teams are told apart by case. Home players are `A`-`K`, away players
//! `a`-`k`, by squad slot; the ball is `*` on the ground and `o` in the air.
//! Plain ANSI only, so the view works in Git Bash (mintty), in a CI log and
//! in any terminal.

use std::fmt::Write as _;

use ds_core::ids::PlayerId;
use ds_match::{BallState, NarratedTick, NarrativeEntry};

/// The size of the drawn pitch, in characters, and of the real one.
#[derive(Debug, Clone, Copy)]
pub struct PitchView {
    /// Columns of the pitch interior.
    pub cols: usize,
    /// Rows of the pitch interior.
    pub rows: usize,
    /// Real length of the pitch, in centimetres (the x axis).
    pub length_cm: i32,
    /// Real width of the pitch, in centimetres (the y axis).
    pub width_cm: i32,
}

/// A ball above this height is drawn as in the air.
const AIR_CM: i32 = 50;
/// How many event lines a frame shows.
pub const LOG_LINES: usize = 6;

impl PitchView {
    /// A view of `cols` x `rows` characters of a pitch of the given size.
    #[must_use]
    pub const fn new(cols: usize, rows: usize, length_m: u16, width_m: u16) -> Self {
        Self {
            cols,
            rows,
            length_cm: length_m as i32 * 100,
            width_cm: width_m as i32 * 100,
        }
    }

    /// The cell of a position, clamped to the pitch.
    fn cell(&self, (x, y): (i32, i32)) -> (usize, usize) {
        let col = scale(x, self.length_cm, self.cols);
        // Row 0 is the top of the screen: the far touchline (max y).
        let row = self.rows - 1 - scale(y, self.width_cm, self.rows);
        (col, row)
    }

    /// Draws one frame: header, pitch, the carrier, and the event log.
    #[must_use]
    pub fn frame(&self, tick: &NarratedTick, log: &[NarrativeEntry]) -> String {
        let r = &tick.record;
        let mut grid = vec![vec![' '; self.cols]; self.rows];
        // The halfway line.
        let mid = self.cols / 2;
        for row in &mut grid {
            row[mid] = ':';
        }
        // The referee (ADR-021 note, plan L0-CLOSE 10g): the Laws, present
        // but without occlusion. A projection for the view, not a body in
        // the match: he follows play on the diagonal, as referees do, and is
        // drawn first, so that a player or the ball covers him, never the
        // other way round.
        let (rc, rr) = self.cell(referee_at(
            (r.ball.position.0, r.ball.position.1),
            (self.length_cm, self.width_cm),
        ));
        grid[rr][rc] = 'R';
        for a in &r.agents {
            let (c, rw) = self.cell(a.position);
            grid[rw][c] = marker(a.id);
        }
        let ball_marker = match r.ball.state {
            BallState::Possessed => None,
            BallState::InFlight if r.ball.position.2 > AIR_CM => Some('o'),
            BallState::InFlight | BallState::Loose => Some('*'),
        };
        if let Some(m) = ball_marker {
            let (c, rw) = self.cell((r.ball.position.0, r.ball.position.1));
            grid[rw][c] = m;
        }

        let (min, sec) = r.clock();
        let ball = match (r.ball.state, r.carrier) {
            (BallState::Possessed, Some(id)) => format!("with {}", marker(id)),
            (BallState::InFlight, _) if r.ball.position.2 > AIR_CM => {
                format!("in the air, {} cm", r.ball.position.2)
            }
            (BallState::InFlight, _) => "rolling".to_string(),
            _ => "loose".to_string(),
        };
        let mut out = String::new();
        let _ = writeln!(
            out,
            " {min:>3}'{sec:02}   HOME {} - {} AWAY   ball: {ball}   tick {}",
            r.score.home, r.score.away, r.tick
        );
        let border: String = std::iter::repeat_n('-', self.cols).collect();
        let _ = writeln!(out, " +{border}+");
        let goal_rows = goal_rows(self.rows);
        for (i, row) in grid.iter().enumerate() {
            let (l, rr) = if goal_rows.contains(&i) {
                (']', '[')
            } else {
                ('|', '|')
            };
            out.push(' ');
            out.push(l);
            out.extend(row.iter());
            out.push(rr);
            out.push('\n');
        }
        let _ = writeln!(out, " +{border}+");
        out.push_str(" home A-K  away a-k  ball * ground, o air  R referee\n");
        for e in log.iter().rev().take(LOG_LINES).rev() {
            let _ = writeln!(out, "  {:>3}'  {}", e.minute, e.text);
        }
        for _ in log.len().min(LOG_LINES)..LOG_LINES {
            out.push('\n');
        }
        out
    }
}

/// The two squads as the 2D shows them: letter, name, role, height and
/// weight. Visible facts only (Plane B): the characteristics stay hidden.
#[must_use]
pub fn roster(m: &ds_model::Match) -> String {
    let mut out = String::new();
    for (label, squad) in [("HOME", &m.home), ("AWAY", &m.away)] {
        let _ = writeln!(out, " {label}");
        for (i, p) in squad.players.iter().chain(squad.bench.iter()).enumerate() {
            if i == squad.players.len() && !squad.bench.is_empty() {
                let _ = writeln!(out, "   bench");
            }
            let _ = writeln!(
                out,
                "   {}  {:<10} {:<20} {} cm  {} kg",
                marker(p.id),
                p.name,
                p.role.as_str().replace('_', " "),
                p.height_cm,
                p.weight_kg
            );
        }
    }
    out
}

/// How far from the ball the referee keeps, cm: about ten metres (the
/// maintainer, 17 November), so that he sees play without standing in it.
pub const REFEREE_DISTANCE_CM: f64 = 1_000.0;

/// How fast the referee runs at most, cm a tick: 6.7 m/s.
///
/// 40 m in 6 s, the sprint of FIFA's fitness test for elite referees
/// (FIFA, *Fitness tests for match officials*, the 6 x 40 m sprint, time
/// limit 6.0 s).
pub const REFEREE_SPEED_CM_PER_TICK: f64 = 4_000.0 / 60.0;

/// Where the referee wants to stand, for the view: about ten metres from
/// the ball, on his diagonal.
///
/// Plan L0-CLOSE 10g; the maintainer's report on seed 43. The diagonal runs
/// from near one corner of the pitch to the other, as the diagonal system
/// of control puts him. When the ball is on the diagonal, or
/// near it, he stands along it, towards the middle of the pitch, so that
/// he is never on the ball. The Laws are present in the view as a ghost:
/// he is no actor of the match (ADR-021 note).
#[must_use]
pub fn referee_at(ball: (i32, i32), (length, width): (i32, i32)) -> (i32, i32) {
    let (ax, ay) = (f64::from(length) * 0.15, f64::from(width) * 0.85);
    let (bx, by) = (f64::from(length) * 0.85, f64::from(width) * 0.15);
    let (dx, dy) = (bx - ax, by - ay);
    let len = ds_core::dmath::hypot(dx, dy).max(1.0);
    let (ux, uy) = (dx / len, dy / len);
    let (px, py) = (f64::from(ball.0) - ax, f64::from(ball.1) - ay);
    // Along the diagonal, from its first end, and how far off it.
    let along = px.mul_add(ux, py * uy);
    let off = px.mul_add(-uy, py * ux).abs();
    let slide = REFEREE_DISTANCE_CM
        .mul_add(REFEREE_DISTANCE_CM, -off * off)
        .max(0.0)
        .sqrt();
    // Towards the middle of the pitch, unless the end is too near.
    let towards: f64 = if along <= len / 2.0 { 1.0 } else { -1.0 };
    let mut t = towards.mul_add(slide, along);
    if t < 0.0 || t > len {
        t = (-towards).mul_add(slide, along);
    }
    let t = t.clamp(0.0, len);
    #[allow(clippy::cast_possible_truncation)]
    let p = (
        ux.mul_add(t, ax).round() as i32,
        uy.mul_add(t, ay).round() as i32,
    );
    p
}

/// The referee one tick on (for the view): from where he was towards where
/// he wants to stand, no faster than a referee runs.
#[must_use]
pub fn referee_step(from: (i32, i32), ball: (i32, i32), pitch: (i32, i32)) -> (i32, i32) {
    let to = referee_at(ball, pitch);
    let (dx, dy) = (f64::from(to.0 - from.0), f64::from(to.1 - from.1));
    let d = ds_core::dmath::hypot(dx, dy);
    if d <= REFEREE_SPEED_CM_PER_TICK {
        return to;
    }
    let k = REFEREE_SPEED_CM_PER_TICK / d;
    #[allow(clippy::cast_possible_truncation)]
    let p = (
        from.0 + (dx * k).round() as i32,
        from.1 + (dy * k).round() as i32,
    );
    p
}

/// The referee at every tick of a match, from where he wants to stand at
/// the kick-off (for the view).
#[must_use]
pub fn referee_path(balls: &[(i32, i32)], pitch: (i32, i32)) -> Vec<(i32, i32)> {
    let mut out = Vec::with_capacity(balls.len());
    let mut at = balls
        .first()
        .map_or((pitch.0 / 2, pitch.1 / 2), |b| referee_at(*b, pitch));
    for b in balls {
        at = referee_step(at, *b, pitch);
        out.push(at);
    }
    out
}

/// The marker of a player: `A`-`K` home, `a`-`k` away, by squad slot.
#[must_use]
pub fn marker(id: PlayerId) -> char {
    let slot = id.slot();
    let n = u8::try_from(slot % 100).unwrap_or(25).min(25);
    if slot >= 100 {
        char::from(b'a' + n)
    } else {
        char::from(b'A' + n)
    }
}

/// `v` in `0..=full` scaled to `0..n`, clamped.
fn scale(v: i32, full: i32, n: usize) -> usize {
    if full <= 0 || n == 0 {
        return 0;
    }
    let v = i64::from(v.clamp(0, full));
    let idx = v * i64::try_from(n).unwrap_or(i64::MAX) / i64::from(full);
    usize::try_from(idx).unwrap_or(0).min(n - 1)
}

/// The rows where the goal mouths are drawn (the middle ~tenth).
fn goal_rows(rows: usize) -> std::ops::Range<usize> {
    let half = (rows / 10).max(1);
    let c = rows / 2;
    c.saturating_sub(half)..(c + half).min(rows)
}

/// Walks a narrated match frame by frame.
///
/// Every `step` ticks one frame is shown, from tick `from` to tick `to`
/// (inclusive bounds on the record indices). The events of the ticks in
/// between are not skipped: each frame carries all the entries born since
/// the previous frame, so a whole playback shows every event exactly once.
pub struct Playback<'a> {
    ticks: &'a [NarratedTick],
    next: usize,
    end: usize,
    step: usize,
    pending_from: usize,
}

impl<'a> Playback<'a> {
    /// A playback of `ticks[from..=to]`, one frame every `step` ticks.
    #[must_use]
    pub fn new(ticks: &'a [NarratedTick], from: usize, to: usize, step: usize) -> Self {
        let end = to.min(ticks.len().saturating_sub(1));
        Self {
            ticks,
            next: from.min(end),
            end,
            step: step.max(1),
            pending_from: 0,
        }
    }
}

impl<'a> Iterator for Playback<'a> {
    /// The tick to draw, and the entries born since the previous frame.
    type Item = (&'a NarratedTick, Vec<NarrativeEntry>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.ticks.is_empty() || self.next > self.end {
            return None;
        }
        let at = self.next;
        let new: Vec<NarrativeEntry> = self.ticks[self.pending_from..=at]
            .iter()
            .flat_map(|t| t.entries.iter().cloned())
            .collect();
        self.pending_from = at + 1;
        self.next = if at == self.end {
            self.end + 1
        } else {
            (at + self.step).min(self.end)
        };
        Some((&self.ticks[at], new))
    }
}

#[cfg(test)]
mod tests {
    use super::referee_at;

    /// The referee follows play on the diagonal (plan L0-CLOSE 10g): near
    /// one corner when the ball is there, near the other at the other end.
    #[test]
    fn the_referee_follows_the_diagonal() {
        let pitch = (10_500, 6_800);
        let near = referee_at((1000, 6000), pitch);
        let far = referee_at((9500, 800), pitch);
        assert!(near.0 < 3000 && far.0 > 7500, "{near:?} {far:?}");
    }

    /// The referee is never on the ball (the maintainer's report on seed
    /// 43): with the ball on his diagonal he stands about ten metres from
    /// it, and he never runs faster than a referee.
    #[test]
    fn the_referee_keeps_off_the_ball_and_runs_as_a_referee() {
        let pitch = (10_500, 6_800);
        let on = (5250, 3400);
        let at = referee_at(on, pitch);
        let d = ds_core::dmath::hypot(f64::from(at.0 - on.0), f64::from(at.1 - on.1));
        assert!((d - REFEREE_DISTANCE_CM).abs() < 2.0, "{d}");
        let path = referee_path(&[(1000, 6000), (9500, 800), (9500, 800)], pitch);
        for w in path.windows(2) {
            let step =
                ds_core::dmath::hypot(f64::from(w[1].0 - w[0].0), f64::from(w[1].1 - w[0].1));
            assert!(step <= REFEREE_SPEED_CM_PER_TICK + 1.0, "{step}");
        }
    }

    use super::*;

    fn narrated() -> (usize, Vec<NarratedTick>) {
        // The seven tests of the board and the commentary all look at seed
        // 42: the match and its narration are made once (19 November 2026).
        static NARRATED: std::sync::LazyLock<(usize, Vec<NarratedTick>)> =
            std::sync::LazyLock::new(|| {
                let (m, result, records) = &*crate::test_support::SEED_42;
                (
                    result.events.len(),
                    ds_match::narrate_ticks(m, &result.events, records.clone()),
                )
            });
        NARRATED.clone()
    }

    /// ADR-022 §3, ADR-025: the view does not read the engine.
    #[test]
    fn pitchview_does_not_reference_forbidden_types() {
        let source = include_str!("pitchview.rs");
        for needle in [
            concat!("pcfm", "_hidden"),
            concat!("Match", "State"),
            concat!("Spatial", "State"),
            concat!("ds_", "orchestrator"),
            concat!("ds_", "db"),
        ] {
            assert!(
                !source.contains(needle),
                "pitchview.rs must not reference {needle}"
            );
        }
    }

    #[test]
    fn a_whole_playback_shows_every_event_exactly_once() {
        let (events, ticks) = narrated();
        for step in [1, 7, 300] {
            let shown: usize = Playback::new(&ticks, 0, usize::MAX, step)
                .map(|(_, new)| new.len())
                .sum();
            assert_eq!(shown, events, "step {step}: no event lost, none invented");
        }
    }

    #[test]
    fn a_playback_ends_on_the_last_tick() {
        let (_, ticks) = narrated();
        let last = Playback::new(&ticks, 0, usize::MAX, 1000)
            .last()
            .expect("frames");
        assert_eq!(last.0.record.tick, ticks.last().expect("ticks").record.tick);
    }

    #[test]
    fn every_frame_has_the_same_shape() {
        let (_, ticks) = narrated();
        let view = PitchView::new(70, 21, 105, 68);
        let lines: Vec<usize> = Playback::new(&ticks, 0, usize::MAX, 5000)
            .map(|(t, new)| view.frame(t, &new).lines().count())
            .collect();
        assert!(lines.windows(2).all(|w| w[0] == w[1]), "{lines:?}");
    }

    /// Every occupied cell is drawn: the view hides no one except where two
    /// players share a cell.
    ///
    /// Finding (2026-11-12, seed 42, seen with this view): at kick-off the
    /// two teams stand on top of each other. The formation slots span the
    /// whole pitch and the away side is their mirror, so the away forwards
    /// stand on the home defenders' spots: 22 players in 14 cells. In a real
    /// match each team kicks off from its own half (Law 8).
    #[test]
    fn every_occupied_cell_is_drawn() {
        let (_, ticks) = narrated();
        let (cols, rows) = (105, 34);
        let view = PitchView::new(cols, rows, 105, 68);
        for t in [&ticks[0], &ticks[ticks.len() / 2]] {
            let cells: std::collections::BTreeSet<_> = t
                .record
                .agents
                .iter()
                .map(|a| view.cell(a.position))
                .collect();
            let f = view.frame(t, &[]);
            let drawn = f
                .lines()
                .skip(2)
                .take(rows)
                .flat_map(|l| l.chars().skip(2).take(cols))
                // The referee's R is not a player (no player is R).
                .filter(|c| c.is_ascii_alphabetic() && *c != 'R')
                .count();
            let ball_cell = view.cell((t.record.ball.position.0, t.record.ball.position.1));
            let ball_drawn = t.record.ball.state != BallState::Possessed;
            let ball_on_a_player = ball_drawn && cells.contains(&ball_cell);
            // A ball in the air is drawn `o`, a letter: off a player's cell
            // it adds one to the count.
            let ball_is_a_letter = ball_drawn
                && !ball_on_a_player
                && t.record.ball.state == BallState::InFlight
                && t.record.ball.position.2 > AIR_CM;
            assert_eq!(
                drawn + usize::from(ball_on_a_player) - usize::from(ball_is_a_letter),
                cells.len(),
                "\n{f}"
            );
        }
    }

    #[test]
    fn the_same_seed_draws_the_same_frames() {
        let (_, a) = narrated();
        let (_, b) = narrated();
        let view = PitchView::new(70, 21, 105, 68);
        assert_eq!(view.frame(&a[6000], &[]), view.frame(&b[6000], &[]));
    }

    #[test]
    fn markers_tell_the_teams_apart() {
        assert_eq!(marker(PlayerId::new(0, 0)), 'A');
        assert_eq!(marker(PlayerId::new(10, 0)), 'K');
        assert_eq!(marker(PlayerId::new(100, 0)), 'a');
        assert_eq!(marker(PlayerId::new(110, 0)), 'k');
    }
}
