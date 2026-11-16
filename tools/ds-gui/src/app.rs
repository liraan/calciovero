//! The friendly match window (ADR-055): the manager's choices, the tilted
//! board of the 2.5D view from the tick record (decision R, `DESIGN.md`), the clock and the score, the commentary and the
//! reasons for every goal conceded.
//!
//! It reads Plane B only (ADR-022 §3): the narrated tick record, the
//! narrative and the squads; it sends back only the manager's choices,
//! through `ds_demo::friendly`, the same functions as the CLI.

use std::collections::BTreeMap;

use ds_core::ids::PlayerId;
use ds_demo::friendly::{self, Choices};
use ds_match::balance::LawsBalance;
use ds_match::{Importance, MatchConfig, NarratedTick};
use ds_model::{BlockHeight, Width};
use eframe::egui::{self, Align2, FontId, Painter, Pos2, RichText, Sense, Shape, Stroke};

use crate::interp::Moment;
use crate::scene::{self, Geometry, Item};
use crate::theme::Theme;

/// Ticks a second of match time (MD-18).
const TICKS_PER_SECOND: f64 = 10.0;
/// The playback speeds offered, times real time.
const SPEEDS: [f64; 5] = [1.0, 4.0, 16.0, 64.0, 256.0];

/// A match played and narrated, ready to watch.
struct Game {
    ticks: Vec<NarratedTick>,
    records: Vec<ds_match::TickRecord>,
    names: BTreeMap<PlayerId, String>,
    why: String,
    pitch_cm: (i32, i32),
}

/// What the command line asks of the window (`ds-gui --help`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Start {
    /// The board's theme, by name.
    pub theme: Option<String>,
    /// The camera's tilt, degrees.
    pub tilt: Option<f32>,
    /// The seed of the match.
    pub seed: Option<u64>,
    /// The tick to start from.
    pub tick: Option<usize>,
    /// The playback speed, times real time.
    pub speed: Option<f64>,
}

/// The application state.
pub struct App {
    theme: Theme,
    themes: Vec<Theme>,
    laws: LawsBalance,
    seed: u64,
    choices: Choices,
    game: Option<Game>,
    error: Option<String>,
    tick: usize,
    playing: bool,
    speed: f64,
    carry: f64,
    every_event: bool,
    /// The camera's tilt from the vertical, degrees (`scene::PRESETS`).
    tilt: f32,
}

impl App {
    /// A window with the default choices, the match not played yet.
    #[must_use]
    ///
    /// # Panics
    ///
    /// If `themes` is empty (`Theme::all` never returns an empty list).
    pub fn new(themes: Vec<Theme>) -> Self {
        let mut app = Self {
            theme: themes[0].clone(),
            themes,
            laws: LawsBalance::load(),
            seed: 42,
            choices: Choices::default(),
            game: None,
            error: None,
            tick: 0,
            playing: false,
            speed: 16.0,
            carry: 0.0,
            every_event: false,
            tilt: scene::PRESETS[2],
        };
        app.play_match();
        app
    }

    /// The window as the command line asks: the board's theme by name,
    /// the camera's tilt, the seed, the tick to start from, the speed.
    /// Anything not given keeps the default.
    #[must_use]
    pub fn with_start(mut self, start: &Start) -> Self {
        if let Some(name) = &start.theme {
            if let Some(t) = self
                .themes
                .iter()
                .find(|t| t.name.eq_ignore_ascii_case(name))
            {
                self.theme = t.clone();
            }
        }
        if let Some(tilt) = start.tilt {
            self.tilt = tilt;
        }
        if let Some(speed) = start.speed {
            self.speed = speed;
        }
        if let Some(seed) = start.seed {
            self.seed = seed;
            self.play_match();
        }
        if let Some(tick) = start.tick {
            self.tick = tick;
        }
        self
    }

    /// Plays the match with the current seed and choices.
    fn play_match(&mut self) {
        let built =
            ds_fixtures::match_fixture(self.seed).and_then(|m| friendly::apply(m, &self.choices));
        match built {
            Ok(m) => {
                let (result, records) =
                    ds_match::simulate_from_seed_recorded(&m, self.seed, MatchConfig::default());
                let why = friendly::why_conceded(&m, result.events.events());
                let kept = records.clone();
                let ticks = ds_match::narrate_ticks(&m, &result.events, records);
                let names = m
                    .home
                    .players
                    .iter()
                    .chain(m.home.bench.iter())
                    .chain(m.away.players.iter())
                    .chain(m.away.bench.iter())
                    .map(|p| (p.id, p.name.clone()))
                    .collect();
                self.game = Some(Game {
                    ticks,
                    records: kept,
                    names,
                    why,
                    pitch_cm: (
                        i32::from(m.pitch.length_m) * 100,
                        i32::from(m.pitch.width_m) * 100,
                    ),
                });
                self.error = None;
                self.tick = 0;
                self.carry = 0.0;
                self.playing = true;
            }
            Err(e) => self.error = Some(e),
        }
    }

    /// Moves the playback on by the time since the last frame.
    fn advance(&mut self, dt: f64) {
        let Some(game) = &self.game else { return };
        if !self.playing || game.ticks.is_empty() {
            return;
        }
        self.carry = (dt * TICKS_PER_SECOND).mul_add(self.speed, self.carry);
        let whole = self.carry.floor();
        self.carry -= whole;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let step = whole as usize;
        let last = game.ticks.len() - 1;
        self.tick = (self.tick + step).min(last);
        if self.tick == last {
            self.playing = false;
        }
    }

    fn choices_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Calcio Vero");
            ui.separator();
            ui.label("Seed");
            ui.add(egui::DragValue::new(&mut self.seed));
            let block = &mut self.choices.home.block;
            ui.label("Block");
            egui::ComboBox::from_id_salt("block")
                .selected_text(block_label(*block))
                .show_ui(ui, |ui| {
                    for b in [BlockHeight::High, BlockHeight::Mid, BlockHeight::Low] {
                        ui.selectable_value(block, b, block_label(b));
                    }
                });
            let width = &mut self.choices.home.width;
            ui.label("Width");
            egui::ComboBox::from_id_salt("width")
                .selected_text(width_label(*width))
                .show_ui(ui, |ui| {
                    for w in [Width::Narrow, Width::Normal, Width::Wide] {
                        ui.selectable_value(width, w, width_label(w));
                    }
                });
            ui.label("Board");
            let current = self.theme.name.clone();
            egui::ComboBox::from_id_salt("theme")
                .selected_text(current.as_str())
                .show_ui(ui, |ui| {
                    for t in &self.themes {
                        if ui
                            .selectable_label(t.name == current, t.name.as_str())
                            .clicked()
                        {
                            self.theme = t.clone();
                        }
                    }
                });
            if ui.button("Play the match").clicked() {
                self.play_match();
            }
            if let Some(e) = &self.error {
                ui.colored_label(self.theme.critical, e);
            }
        });
    }

    fn playback_bar(&mut self, ui: &mut egui::Ui) {
        let Some(game) = &self.game else { return };
        let last = game.ticks.len().saturating_sub(1);
        let Some(now) = game.ticks.get(self.tick) else {
            return;
        };
        let (min, sec) = now.record.clock();
        let score = now.record.score;
        ui.horizontal(|ui| {
            let label = if self.playing { "Pause" } else { "Play" };
            if ui.button(label).clicked() {
                if self.tick >= last {
                    self.tick = 0;
                }
                self.playing = !self.playing;
            }
            ui.label(
                RichText::new(format!("{min:>3}:{sec:02}"))
                    .monospace()
                    .strong(),
            );
            ui.label(RichText::new(format!("Home {} - {} Away", score.home, score.away)).strong());
            ui.separator();
            for s in SPEEDS {
                ui.selectable_value(&mut self.speed, s, format!("x{s}"));
            }
            ui.separator();
            ui.label("Camera");
            for p in scene::PRESETS {
                ui.selectable_value(&mut self.tilt, p, format!("{p:.0}°"));
            }
            ui.separator();
            ui.spacing_mut().slider_width = (ui.available_width() - 16.0).max(80.0);
            ui.add(egui::Slider::new(&mut self.tick, 0..=last).show_value(false));
        });
    }

    fn commentary(&mut self, ui: &mut egui::Ui) {
        let Some(game) = &self.game else { return };
        let mut jump = None;
        egui::CollapsingHeader::new("Why we conceded")
            .default_open(false)
            .show(ui, |ui| {
                ui.label(if game.why.trim().is_empty() {
                    "No goal conceded."
                } else {
                    game.why.as_str()
                });
            });
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(RichText::new("Commentary").strong());
            ui.checkbox(&mut self.every_event, "every touch");
        });
        egui::ScrollArea::vertical().show(ui, |ui| {
            let upto = self.tick.min(game.ticks.len().saturating_sub(1));
            for (i, t) in game.ticks[..=upto].iter().enumerate().rev() {
                for e in t.entries.iter().rev() {
                    if !self.every_event && e.importance == Importance::Low {
                        continue;
                    }
                    let colour = match e.importance {
                        Importance::Critical => self.theme.critical,
                        Importance::High => self.theme.high,
                        Importance::Medium => self.theme.medium,
                        Importance::Low => self.theme.low,
                    };
                    let text = RichText::new(format!("{:>3}' {}", e.minute, e.text)).color(colour);
                    let text = if e.importance == Importance::Critical {
                        text.strong()
                    } else {
                        text
                    };
                    if ui
                        .add(egui::Label::new(text).sense(Sense::click()))
                        .on_hover_text("Go to this moment")
                        .clicked()
                    {
                        jump = Some(i);
                    }
                }
            }
        });
        if let Some(i) = jump {
            self.tick = i;
            self.playing = false;
        }
    }

    fn pitch(&self, ui: &mut egui::Ui) {
        let Some(game) = &self.game else { return };
        if game.records.is_empty() {
            return;
        }
        let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::hover());
        let r = response.rect;
        let geom = Geometry::fit(
            (r.min.x, r.min.y),
            (r.width(), r.height()),
            16.0,
            game.pitch_cm,
            self.tilt,
        );
        self.draw_markings(&painter, &geom);
        // One tick behind the playhead, so that the drawing always has the
        // tick before and the tick after (the interpolation layer).
        #[allow(clippy::cast_precision_loss)]
        let instant = (self.tick as f64 + self.carry - 1.0).max(0.0);
        let moment = Moment::at(&game.records, instant);
        let frame = scene::frame(&moment, &geom);
        let t = &self.theme;
        let pos = |s: &scene::Screen| Pos2::new(s.x, s.y);
        // The ball's shadow on the grass, under everything.
        for (g, radius) in &frame.shadows {
            painter.circle_filled(pos(g), *radius, t.ball_shadow);
        }
        let mut hovered = None;
        for item in &frame.items {
            match item {
                Item::Figure(f) | Item::Referee(f) => {
                    let fill = match item {
                        Item::Referee(_) => t.referee,
                        _ => match (f.home, f.keeper) {
                            (true, true) => t.keeper_home,
                            (true, false) => t.home,
                            (false, true) => t.keeper_away,
                            (false, false) => t.away,
                        },
                    };
                    let (feet, head) = (pos(&f.feet), pos(&f.head));
                    // His shadow, then where he faces, then the figure.
                    painter.circle_filled(feet, f.size * 1.2, t.ball_shadow);
                    painter.line_segment(
                        [feet, pos(&f.facing)],
                        Stroke::new((f.size * 0.35).max(1.0), fill),
                    );
                    if f.carrier {
                        painter.circle_stroke(
                            feet,
                            f.size * 1.8,
                            Stroke::new((f.size * 0.3).max(1.5), t.carrier_ring),
                        );
                    }
                    let neck = feet + (head - feet) * 0.82;
                    painter.line_segment([feet, neck], Stroke::new(f.size * 1.5, fill));
                    painter.circle_filled(head, f.size, fill);
                    if matches!(item, Item::Figure(_)) {
                        painter.text(
                            head,
                            Align2::CENTER_CENTER,
                            f.letter,
                            FontId::monospace((f.size * 1.4).max(7.0)),
                            t.player_text,
                        );
                        if response
                            .hover_pos()
                            .is_some_and(|p| p.distance(head) <= f.size * 2.0)
                        {
                            hovered = Some(f.id);
                        }
                    }
                }
                Item::Ball(b) => {
                    painter.circle_filled(pos(&b.at), b.radius, t.ball);
                    painter.circle_stroke(
                        pos(&b.at),
                        b.radius,
                        Stroke::new((b.radius * 0.2).max(0.8), t.ball_shadow),
                    );
                }
            }
        }
        if let Some(id) = hovered {
            if let Some(name) = game.names.get(&id) {
                response.on_hover_text(format!("{} {name}", ds_demo::pitchview::marker(id)));
            }
        }
    }

    /// The markings of Law 1 (`laws.json`) and the goals in their height
    /// (`obstacles.json`), on the tilted board.
    #[allow(clippy::cast_precision_loss)] // cm on a pitch
    #[allow(clippy::many_single_char_names)] // drawing geometry
    fn draw_markings(&self, painter: &Painter, geom: &Geometry) {
        let t = &self.theme;
        let (length, width) = geom.pitch_cm;
        let on = |x: i32, y: i32| {
            let (sx, sy) = geom.at((x, y));
            Pos2::new(sx, sy)
        };
        let quad = |x0: i32, y0: i32, x1: i32, y1: i32| {
            vec![on(x0, y0), on(x1, y0), on(x1, y1), on(x0, y1)]
        };
        let m = 150;
        painter.add(Shape::convex_polygon(
            quad(-m - 200, -m, length + m + 200, width + m),
            t.pitch,
            Stroke::NONE,
        ));
        // Bands of grass across the pitch.
        let bands = 12;
        for i in (0..bands).step_by(2) {
            painter.add(Shape::convex_polygon(
                quad(length * i / bands, 0, length * (i + 1) / bands, width),
                t.pitch_stripe,
                Stroke::NONE,
            ));
        }
        let line = Stroke::new(1.5_f32, t.lines);
        painter.add(Shape::closed_line(quad(0, 0, length, width), line));
        painter.line_segment([on(length / 2, 0), on(length / 2, width)], line);
        let circle = |cx: i32, cy: i32, radius: i32| {
            (0..48)
                .map(|k| {
                    let a = (k as f32) * std::f32::consts::TAU / 48.0;
                    #[allow(
                        clippy::disallowed_methods,
                        reason = "drawing only: the view never feeds the simulation"
                    )]
                    let (s, c) = a.sin_cos();
                    #[allow(clippy::cast_possible_truncation)]
                    on(
                        cx + (c * radius as f32) as i32,
                        cy + (s * radius as f32) as i32,
                    )
                })
                .collect::<Vec<_>>()
        };
        let l = self.laws;
        painter.add(Shape::closed_line(
            circle(length / 2, width / 2, l.centre_circle_radius_cm),
            line,
        ));
        painter.circle_filled(on(length / 2, width / 2), 2.0, t.lines);
        for (line_x, dir) in [(0, 1), (length, -1)] {
            let mid = width / 2;
            for (depth, half) in [
                (l.penalty_area_depth_cm, l.penalty_area_half_width_cm),
                (l.goal_area_depth_cm, l.goal_area_half_width_cm),
            ] {
                painter.add(Shape::closed_line(
                    quad(line_x, mid - half, line_x + dir * depth, mid + half),
                    line,
                ));
            }
            painter.circle_filled(on(line_x + dir * l.penalty_spot_cm, mid), 2.0, t.lines);
        }
        // The goals: posts and crossbar in their height, the net behind.
        let o = ds_match::balance::ObstacleBalance::load();
        for (a, b) in geom.goals(o.goal_half_width_cm as f32, o.goal_height_cm as f32) {
            if let (Some(p), Some(q)) = (geom.project(a), geom.project(b)) {
                painter.line_segment(
                    [Pos2::new(p.x, p.y), Pos2::new(q.x, q.y)],
                    Stroke::new(2.0_f32, t.lines),
                );
            }
        }
    }
}

const fn block_label(b: BlockHeight) -> &'static str {
    match b {
        BlockHeight::High => "high",
        BlockHeight::Mid => "mid",
        BlockHeight::Low => "low",
    }
}

const fn width_label(w: Width) -> &'static str {
    match w {
        Width::Narrow => "narrow",
        Width::Normal => "normal",
        Width::Wide => "wide",
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dt = f64::from(ctx.input(|i| i.stable_dt).min(0.1));
        self.advance(dt);
        ctx.set_visuals(if self.theme.dark {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        });
        egui::TopBottomPanel::top("choices").show(ctx, |ui| self.choices_bar(ui));
        egui::TopBottomPanel::bottom("playback").show(ctx, |ui| self.playback_bar(ui));
        egui::SidePanel::right("commentary")
            .default_width(380.0)
            .show(ctx, |ui| self.commentary(ui));
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(self.theme.board))
            .show(ctx, |ui| self.pitch(ui));
        if self.playing {
            ctx.request_repaint();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> App {
        App::new(Theme::all().expect("themes"))
    }

    #[test]
    fn a_new_window_has_a_match_ready_to_watch() {
        let a = app();
        let g = a.game.as_ref().expect("a match is played at start");
        assert!(g.ticks.len() > 50_000, "a whole match: {}", g.ticks.len());
        assert!(a.playing);
    }

    /// The playback runs at the chosen speed and stops at the final whistle.
    #[test]
    fn the_playback_follows_the_clock_and_stops_at_the_end() {
        let mut a = app();
        a.speed = 4.0;
        a.advance(1.0);
        assert_eq!(a.tick, 40, "one second at x4 is forty ticks");
        a.advance(1e9);
        let last = a.game.as_ref().map_or(0, |g| g.ticks.len() - 1);
        assert_eq!(a.tick, last);
        assert!(!a.playing);
    }

    /// The manager's choice reaches the match: a different block, a
    /// different game (the same seed).
    #[test]
    fn the_choices_change_the_match() {
        let mut a = app();
        let before = a.game.as_ref().map(|g| g.ticks.last().cloned());
        a.choices.home.block = BlockHeight::Low;
        a.play_match();
        let after = a.game.as_ref().map(|g| g.ticks.last().cloned());
        assert_ne!(before, after);
    }
}
