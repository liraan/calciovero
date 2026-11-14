//! The friendly match window (ADR-055): the manager's choices, the 2D pitch
//! from the tick record, the clock and the score, the commentary and the
//! reasons for every goal conceded.
//!
//! It reads Plane B only (ADR-022 §3): the narrated tick record, the
//! narrative and the squads; it sends back only the manager's choices,
//! through `ds_demo::friendly`, the same functions as the CLI.

use std::collections::BTreeMap;

use ds_core::ids::PlayerId;
use ds_demo::friendly::{self, Choices};
use ds_match::balance::LawsBalance;
use ds_match::{BallState, Importance, MatchConfig, NarratedTick};
use ds_model::{BlockHeight, Width};
use eframe::egui::{
    self, Align2, Color32, FontId, Painter, Pos2, Rect, RichText, Sense, Stroke, StrokeKind, Vec2,
};

use crate::scene::{self, Geometry};
use crate::theme::Theme;

/// Ticks a second of match time (MD-18).
const TICKS_PER_SECOND: f64 = 10.0;
/// The playback speeds offered, times real time.
const SPEEDS: [f64; 5] = [1.0, 4.0, 16.0, 64.0, 256.0];

/// A match played and narrated, ready to watch.
struct Game {
    ticks: Vec<NarratedTick>,
    names: BTreeMap<PlayerId, String>,
    why: String,
    pitch_cm: (i32, i32),
}

/// The application state.
pub struct App {
    theme: Theme,
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
}

impl App {
    /// A window with the default choices, the match not played yet.
    #[must_use]
    pub fn new(theme: Theme) -> Self {
        let mut app = Self {
            theme,
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
        };
        app.play_match();
        app
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
        let Some(now) = game.ticks.get(self.tick) else {
            return;
        };
        let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::hover());
        let r = response.rect;
        let geom = Geometry::fit(
            (r.min.x, r.min.y),
            (r.width(), r.height()),
            16.0,
            game.pitch_cm,
        );
        self.draw_markings(&painter, &geom);
        let frame = scene::frame(&now.record, &geom);
        let t = &self.theme;
        let radius = geom.len(90).max(5.0);
        let pos = |(x, y): (f32, f32)| Pos2::new(x, y);
        // The referee, then the players, then the ball on top.
        painter.circle_filled(pos(frame.referee), radius * 0.6, t.referee);
        let mut hovered = None;
        for d in &frame.dots {
            let fill = match (d.home, d.keeper) {
                (true, true) => t.keeper_home,
                (true, false) => t.home,
                (false, true) => t.keeper_away,
                (false, false) => t.away,
            };
            let c = pos(d.at);
            painter.circle_filled(c, radius, fill);
            // Where he faces: a short tick on the rim.
            let a = f32::from(d.heading).to_radians();
            #[allow(
                clippy::disallowed_methods,
                reason = "drawing only: the view never feeds the simulation"
            )]
            let tip = c + Vec2::new(a.sin(), -a.cos()) * radius * 1.4;
            painter.line_segment([c, tip], Stroke::new(1.5_f32, fill));
            if d.carrier {
                painter.circle_stroke(c, radius + 2.5, Stroke::new(2.0_f32, t.carrier_ring));
            }
            painter.text(
                c,
                Align2::CENTER_CENTER,
                d.letter,
                FontId::monospace(radius * 1.1),
                t.player_text,
            );
            if response
                .hover_pos()
                .is_some_and(|p| p.distance(c) <= radius + 2.0)
            {
                hovered = Some(d.id);
            }
        }
        let b = frame.ball;
        let ground = pos(b.ground);
        #[allow(clippy::cast_precision_loss)] // cm of height
        let lift = geom.len(b.height_cm) * 0.5;
        painter.circle_filled(ground, radius * 0.45, t.ball_shadow);
        let ball_at = ground - Vec2::new(0.0, lift);
        let ball_r = if matches!(b.state, BallState::InFlight) && b.height_cm > 50 {
            radius * 0.55
        } else {
            radius * 0.45
        };
        painter.circle_filled(ball_at, ball_r, t.ball);
        if let Some(id) = hovered {
            if let Some(name) = game.names.get(&id) {
                response.on_hover_text(format!("{} {name}", ds_demo::pitchview::marker(id)));
            }
        }
    }

    /// The markings of Law 1, from `laws.json`.
    fn draw_markings(&self, painter: &Painter, geom: &Geometry) {
        let t = &self.theme;
        let (length, width) = geom.pitch_cm;
        let p = |x: i32, y: i32| {
            let (sx, sy) = geom.at((x, y));
            Pos2::new(sx, sy)
        };
        let field = Rect::from_two_pos(p(0, 0), p(length, width));
        painter.rect_filled(field.expand(geom.len(150)), 0.0, t.pitch);
        // Bands of grass across the pitch.
        let bands = 12;
        for i in (0..bands).step_by(2) {
            let band =
                Rect::from_two_pos(p(length * i / bands, 0), p(length * (i + 1) / bands, width));
            painter.rect_filled(band, 0.0, t.pitch_stripe);
        }
        let line = Stroke::new(1.5_f32, t.lines);
        painter.rect_stroke(field, 0.0, line, StrokeKind::Middle);
        painter.line_segment([p(length / 2, 0), p(length / 2, width)], line);
        let centre = p(length / 2, width / 2);
        painter.circle_stroke(centre, geom.len(self.laws.centre_circle_radius_cm), line);
        painter.circle_filled(centre, 2.0, t.lines);
        let l = self.laws;
        for (line_x, dir) in [(0, 1), (length, -1)] {
            let mid = width / 2;
            let area = |depth: i32, half: i32| {
                Rect::from_two_pos(p(line_x, mid - half), p(line_x + dir * depth, mid + half))
            };
            painter.rect_stroke(
                area(l.penalty_area_depth_cm, l.penalty_area_half_width_cm),
                0.0,
                line,
                StrokeKind::Middle,
            );
            painter.rect_stroke(
                area(l.goal_area_depth_cm, l.goal_area_half_width_cm),
                0.0,
                line,
                StrokeKind::Middle,
            );
            painter.circle_filled(p(line_x + dir * l.penalty_spot_cm, mid), 2.0, t.lines);
            // The goal, behind the line.
            let goal = Rect::from_two_pos(
                p(line_x, mid - l.goal_half_width_cm),
                p(line_x - dir * 200, mid + l.goal_half_width_cm),
            );
            painter.rect_stroke(goal, 0.0, Stroke::new(2.5_f32, t.lines), StrokeKind::Middle);
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
        egui::TopBottomPanel::top("choices").show(ctx, |ui| self.choices_bar(ui));
        egui::TopBottomPanel::bottom("playback").show(ctx, |ui| self.playback_bar(ui));
        egui::SidePanel::right("commentary")
            .default_width(380.0)
            .show(ctx, |ui| self.commentary(ui));
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(Color32::TRANSPARENT))
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
        App::new(Theme::load().expect("theme"))
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
