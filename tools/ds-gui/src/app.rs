//! The friendly match window (ADR-055): the manager's choices, the tilted
//! board of the 2.5D view from the tick record (decision R, `DESIGN.md`), the clock and the score, the commentary and the
//! reasons for every goal conceded.
//!
//! It reads Plane B only (ADR-022 §3): the narrated tick record, the
//! narrative and the squads; it sends back only the manager's choices,
//! through `ds_demo::friendly`, the same functions as the CLI.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use ds_core::ids::PlayerId;
use ds_demo::friendly::{self, Choices};
use ds_match::balance::LawsBalance;
use ds_match::{Importance, MatchConfig, NarratedTick};
use ds_model::{BlockHeight, Width};
use eframe::egui::{self, Align2, FontId, Painter, Pos2, RichText, Sense, Shape, Stroke};
use rodio::{OutputStream, OutputStreamBuilder, Sink, Source, source::SineWave};

use crate::interp::Moment;
use crate::scene::{self, Geometry, Item};
use crate::theme::Theme;

/// Ticks a second of match time (MD-18).
const TICKS_PER_SECOND: f64 = 10.0;
/// Real seconds the stop whistle holds the picture (the maintainer asked
/// for "qualche secondo"): a viewing choice, never a number in the match.
const WHISTLE_PAUSE_S: f64 = 2.0;
/// Match ticks the held picture lasts: the celebration starts only after
/// the VAR check has confirmed the goal (a viewing choice, not a rule).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
const CEREMONY_TICKS: usize = (WHISTLE_PAUSE_S * TICKS_PER_SECOND) as usize;
/// The playback speeds offered, times real time.
const SPEEDS: [f64; 5] = [1.0, 4.0, 16.0, 64.0, 256.0];

/// A match played and narrated, ready to watch.
#[derive(Clone)]
struct Game {
    ticks: Vec<NarratedTick>,
    records: Vec<ds_match::TickRecord>,
    /// Each player's gestures, from the facts of the match: the tick and
    /// the gesture (DESIGN.md: an animation only from a fact).
    gestures: BTreeMap<PlayerId, Vec<(usize, scene::Gesture)>>,
    names: BTreeMap<PlayerId, String>,
    why: String,
    pitch_cm: (i32, i32),
    /// The home side at the kick-off: the eleven and the bench (the bench
    /// post-it).
    home: (Vec<PlayerId>, Vec<PlayerId>),
    /// What moves the crowd, from the facts of the match: the tick, the
    /// side whose fans rise, and how much (a goal, a shot).
    cheers: Vec<(usize, bool, f32)>,
    /// The stadium and its town, built once for the pitch (drawing only).
    backdrop: (Vec<crate::stadium::Shape>, Vec<crate::stadium::Shape>),
    /// The record's header (ADR-065): the physics the match was played
    /// with, for the ball's flight between two ticks.
    header: ds_match::RecordHeader,
    /// The referee's whistles, read from the facts of the match: a stop
    /// for every dead ball, then the restart. Law 6: stop, then play on.
    whistles: Vec<Whistle>,
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
    /// The zoom, `0` the whole pitch, `1` about 25 m across.
    pub zoom: Option<f32>,
    /// Follow the ball (the default) or not.
    pub follow: Option<bool>,
    /// The slow motion.
    pub slow: bool,
    /// Record the window, frame by frame at 60 frames a second of
    /// playback, into this folder, this many frames, then close
    /// (`--record DIR --frames N`: the README's animation).
    pub record: Option<(String, usize)>,
}

/// How quickly the camera catches up with the ball when it follows it, a
/// share a second (a drawing choice: the damped movement).
const FOLLOW_RATE: f32 = 3.0;
/// The slow motion, a share of the chosen speed (DESIGN.md: about a third).
const SLOW: f64 = 1.0 / 3.0;
/// The application state.
#[derive(Clone)]
#[allow(clippy::struct_excessive_bools, reason = "the window's switches")]
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
    /// The camera's tilt from the vertical, degrees.
    tilt: f32,
    /// The zoom, `0` the whole pitch, `1` the nearest (about 25 m).
    zoom: f32,
    /// The camera follows the ball; dragging the view turns it off.
    follow: bool,
    /// The slow motion is on.
    slow: bool,
    /// The audio device, when the machine has one: a failed device only
    /// mutes the whistle, it never stops the match.
    audio: Option<Arc<(OutputStream, Sink)>>,
    /// Real seconds still to wait at the whistle that stopped play.
    whistle_pause: f64,
    /// The last whistle handled, so one tick is never blown twice.
    last_whistle: Option<usize>,
    /// Where the camera looks, cm.
    focus: (f32, f32),
    /// The time of the last frame, seconds (for the damped camera).
    last_dt: f32,
    /// The recording: the folder, the frames wanted, the frames saved.
    record: Option<(String, usize, usize)>,
    /// The bench post-it: who comes off and who comes on, as chosen.
    pick: (Option<PlayerId>, Option<PlayerId>),
    /// The token clicked on the board: his post-it with his
    /// responsibilities (ADR-059 step 5).
    chosen: Option<PlayerId>,
    /// The recording's clock: the tick it starts from, the frames asked
    /// for so far, and whether a screenshot is on its way.
    rec_clock: (usize, usize, bool),
    /// The camera has been put on the ball once (it starts there).
    placed: bool,
    /// The aid of the orders: the line and the block drawn (off).
    show_shape: bool,
    /// The right panel opens when the pointer reaches its edge (a scomparsa).
    panel_open: bool,
    /// The panel's rectangle last frame, to know when the pointer left it.
    panel_rect: Option<egui::Rect>,
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
            tilt: 45.0,
            zoom: 0.0,
            follow: true,
            slow: false,
            audio: OutputStreamBuilder::open_default_stream()
                .ok()
                .map(|stream| {
                    let sink = Sink::connect_new(stream.mixer());
                    Arc::new((stream, sink))
                }),
            whistle_pause: 0.0,
            last_whistle: None,
            focus: (5_250.0, 3_400.0),
            last_dt: 0.0,
            record: None,
            pick: (None, None),
            chosen: None,
            rec_clock: (0, 0, false),
            placed: false,
            show_shape: false,
            panel_open: false,
            panel_rect: None,
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
        if let Some(z) = start.zoom {
            self.zoom = z.clamp(0.0, 1.0);
        }
        if let Some(f) = start.follow {
            self.follow = f;
        }
        self.slow = start.slow;
        self.record = start.record.clone().map(|(dir, n)| (dir, n, 0));
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
        self.rec_clock.0 = self.tick;
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
                let gestures = gestures(&records, result.events.events());
                let cheers = cheers(&records, result.events.events());
                let whistles = whistles(
                    &records,
                    result.events.events(),
                    i32::from(m.pitch.length_m) * 100,
                );
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
                let header = ds_match::RecordHeader::current(
                    MatchConfig::default().ticks_per_second,
                    (m.pitch.length_m, m.pitch.width_m),
                );
                let home = (
                    m.home.players.iter().map(|p| p.id).collect(),
                    m.home.bench.iter().map(|p| p.id).collect(),
                );
                let pitch = (
                    i32::from(m.pitch.length_m) * 100,
                    i32::from(m.pitch.width_m) * 100,
                );
                let backdrop = (crate::stadium::city(pitch), crate::stadium::stadium(pitch));
                self.game = Some(Game {
                    cheers,
                    whistles,
                    backdrop,
                    home,
                    header,
                    ticks,
                    records: kept,
                    gestures,
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
                self.whistle_pause = 0.0;
                self.last_whistle = None;
                self.playing = true;
            }
            Err(e) => self.error = Some(e),
        }
    }

    /// An order given while watching (decision T): an input at the tick
    /// on the screen. The match is played again from the same seed with
    /// it; up to that tick it is the same match, so the view stays where
    /// it was and goes on from there.
    #[allow(
        clippy::large_types_passed_by_value,
        reason = "an order is kept by value in the choices"
    )]
    fn order(&mut self, order: ds_model::Order) {
        let Some(game) = &self.game else { return };
        let Some(now) = game.records.get(self.tick) else {
            return;
        };
        self.choices.live.push(ds_model::Instruction {
            tick: now.tick,
            channel: None,
            order,
        });
        let (tick, carry, playing) = (self.tick, self.carry, self.playing);
        self.play_match();
        self.tick = tick;
        self.carry = carry;
        self.playing = playing;
    }

    /// The bench (DESIGN.md, decision T): who is on, who is on the bench,
    /// the orders given and whether they have been made. It lives in the
    /// right panel now; the floating post-it sat under the top bar.
    fn bench_here(&mut self, ui: &mut egui::Ui) {
        let Some(game) = &self.game else { return };
        let Some(now) = game.records.get(self.tick) else {
            return;
        };
        let (xi, subs) = &game.home;
        let home: Vec<PlayerId> = xi.iter().chain(subs).copied().collect();
        let on: Vec<PlayerId> = now
            .agents
            .iter()
            .map(|a| a.id)
            .filter(|id| home.contains(id))
            .collect();
        // A man who has come off cannot come back (Law 3): the bench is
        // the substitutes not yet on.
        let came_on = |id: &PlayerId| {
            game.records[..=self.tick]
                .iter()
                .any(|r| r.agents.iter().any(|a| a.id == *id))
        };
        let bench: Vec<PlayerId> = subs.iter().copied().filter(|id| !came_on(id)).collect();
        let name = |id: PlayerId| game.names.get(&id).cloned().unwrap_or_default();
        let orders: Vec<(u64, PlayerId, PlayerId, bool)> = self
            .choices
            .live
            .iter()
            .filter_map(|i| match i.order {
                ds_model::Order::Substitute { outgoing, incoming } => {
                    Some((i.tick, outgoing, incoming, on.contains(&incoming)))
                }
                ds_model::Order::Intention(_)
                | ds_model::Order::Quick { .. }
                | ds_model::Order::Transitions { .. }
                | ds_model::Order::Board(_) => None,
            })
            .collect();
        let side = self.theme.home;
        let mut give = None;
        ui.label(RichText::new("Bench").color(side).strong());
        let (out, inn) = &mut self.pick;
        egui::ComboBox::from_id_salt("sub-off")
            .selected_text(out.map_or_else(|| "off…".to_owned(), name))
            .show_ui(ui, |ui| {
                for &p in on.iter().skip(1) {
                    ui.selectable_value(out, Some(p), name(p));
                }
            });
        egui::ComboBox::from_id_salt("sub-on")
            .selected_text(inn.map_or_else(|| "on…".to_owned(), name))
            .show_ui(ui, |ui| {
                for &p in &bench {
                    ui.selectable_value(inn, Some(p), name(p));
                }
            });
        let ready = out.is_some_and(|o| on.contains(&o)) && inn.is_some_and(|i| bench.contains(&i));
        if ui
            .add_enabled(ready, egui::Button::new("Make the change"))
            .on_hover_text("at the next stoppage, as the Laws say")
            .clicked()
        {
            give = out.zip(*inn);
            *out = None;
            *inn = None;
        }
        for (tick, o, i, made) in &orders {
            let min = tick / 600;
            let state = if *made { "made" } else { "waiting" };
            ui.small(format!("{min}' {} → {}: {state}", name(*o), name(*i)));
        }
        if let Some((outgoing, incoming)) = give {
            self.order(ds_model::Order::Substitute { outgoing, incoming });
        }
    }

    /// The orders during the match (the maintainer's pure error 2d): in the
    /// top bar, under the theme's own colours, with the same menus as the
    /// choices before the match. Each change is an order at the tick on the
    /// screen (ADR-059, ADR-070): the block's height and width, more
    /// pressing, everyone forward, the two transitions. It used to be a
    /// floating post-it at y=44, hidden under the top panel.
    fn orders_here(&mut self, ui: &mut egui::Ui) {
        // What he has said so far, the last word on each.
        let mut now = ds_model::TeamOrders::default();
        let mut intention = self.choices.home;
        for i in &self.choices.live {
            match i.order {
                ds_model::Order::Quick { quick, on } => now = now.with(quick, on),
                ds_model::Order::Transitions { lost, won } => {
                    now.lost = lost;
                    now.won = won;
                }
                ds_model::Order::Intention(it) => intention = it,
                ds_model::Order::Substitute { .. } | ds_model::Order::Board(_) => {}
            }
        }
        let side = self.theme.home;
        let (dot, _) = ui.allocate_exact_size(egui::vec2(14.0, 14.0), Sense::hover());
        ui.painter().circle_filled(dot.center(), 6.0, side);
        ui.label(RichText::new("Orders to Home").color(side).strong());
        ui.separator();
        let give = order_menus(ui, intention, now);
        ui.separator();
        // An aid, off by default (the match has no lines drawn on it): where
        // the side's line and block stand.
        ui.checkbox(&mut self.show_shape, "Show the line and the block");
        for order in give {
            self.order(order);
        }
    }

    /// The markings, and over them the aid of the orders if it is on.
    fn draw_ground(&self, painter: &egui::Painter, geom: &Geometry, moment: &Moment) {
        self.draw_markings(painter, geom);
        if self.show_shape {
            self.draw_shape(painter, geom, moment);
        }
    }

    /// The aid of the orders, off by default: the home side's back line (its
    /// deepest outfield man) and its block (from its deepest to its most
    /// advanced outfield man, from its widest on one side to the other), as
    /// they stand now. Drawn under the tokens.
    fn draw_shape(&self, painter: &egui::Painter, geom: &Geometry, moment: &Moment) {
        let Some(game) = &self.game else { return };
        let home: Vec<(f32, f32)> = moment
            .players
            .iter()
            .filter(|p| p.id.slot() < 100 && Some(p.id) != game.home.0.first().copied())
            .map(|p| p.at)
            .collect();
        if home.len() < 2 {
            return;
        }
        let fold = |f: fn(f32, f32) -> f32, pick: fn(&(f32, f32)) -> f32, init: f32| {
            home.iter().map(pick).fold(init, f)
        };
        let (x0, x1) = (
            fold(f32::min, |p| p.0, f32::MAX),
            fold(f32::max, |p| p.0, f32::MIN),
        );
        let (y0, y1) = (
            fold(f32::min, |p| p.1, f32::MAX),
            fold(f32::max, |p| p.1, f32::MIN),
        );
        #[allow(clippy::cast_possible_truncation)]
        let on = |x: f32, y: f32| {
            let (sx, sy) = geom.at((x as i32, y as i32));
            Pos2::new(sx, sy)
        };
        let side = self.theme.home;
        let fill = egui::Color32::from_rgba_unmultiplied(side.r(), side.g(), side.b(), 40);
        painter.add(Shape::convex_polygon(
            vec![on(x0, y0), on(x1, y0), on(x1, y1), on(x0, y1)],
            fill,
            Stroke::new(1.0_f32, side),
        ));
        #[allow(clippy::cast_precision_loss)]
        let (_, width) = (game.pitch_cm.0 as f32, game.pitch_cm.1 as f32);
        painter.line_segment([on(x0, 0.0), on(x0, width)], Stroke::new(2.5_f32, side));
    }

    /// Moves the playback on by the time since the last frame.
    fn advance(&mut self, dt: f64) {
        if self.whistle_pause > 0.0 {
            self.whistle_pause = (self.whistle_pause - dt).max(0.0);
            return;
        }
        let Some(game) = &self.game else { return };
        if !self.playing || game.ticks.is_empty() {
            return;
        }
        let dead = game
            .whistles
            .iter()
            .rfind(|w| w.tick <= self.tick)
            .is_some_and(|w| !w.restart);
        // While the ball is dead the watch runs at real speed: the walk to
        // the ball has to be seen, not swallowed by the default x16.
        let speed = if dead {
            1.0
        } else if self.slow {
            self.speed * SLOW
        } else {
            self.speed
        };
        self.carry = (dt * TICKS_PER_SECOND).mul_add(speed, self.carry);
        let whole = self.carry.floor();
        self.carry -= whole;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let step = whole as usize;
        let last = game.ticks.len() - 1;
        let from = self.tick;
        self.tick = (self.tick + step).min(last);
        if self.tick == last {
            self.playing = false;
        }
        for w in &game.whistles {
            if w.tick > from
                && w.tick <= self.tick
                && self.last_whistle.is_none_or(|last| w.tick > last)
            {
                self.last_whistle = Some(w.tick);
                blow(self.audio.as_ref(), w.restart);
                if !w.restart {
                    self.whistle_pause = WHISTLE_PAUSE_S;
                }
                break;
            }
        }
    }

    fn choices_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Calcio Vero");
            ui.separator();
            ui.label("Seed");
            ui.add(egui::DragValue::new(&mut self.seed));
            ui.separator();
            if self.game.is_some() {
                // Once the match is on, the top bar gives the orders instead
                // of the pre-match choices: the floating post-it used to sit
                // at y=44, under this panel, and was never seen.
                self.orders_here(ui);
            } else {
                ui.label(RichText::new("Before the match:").italics());
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
                // The coach names his captain as he names his block: a
                // view-only choice for who steps up at a card.
                let captain = self.choices.captain.get_or_insert('D');
                ui.label("Captain");
                egui::ComboBox::from_id_salt("captain")
                    .selected_text(captain.to_string())
                    .show_ui(ui, |ui| {
                        for c in 'A'..='K' {
                            ui.selectable_value(captain, c, c.to_string());
                        }
                    });
            }
            ui.separator();
            self.board_and_play(ui);
            if let Some(e) = &self.error {
                ui.colored_label(self.theme.critical, e);
            }
        });
    }

    /// The board and the play button, in both the pre-match and the in-match
    /// top bar.
    fn board_and_play(&mut self, ui: &mut egui::Ui) {
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
            // A new match: the orders given watching the old one go.
            self.choices.live.clear();
            self.play_match();
        }
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
            if ui.button("Whistle").clicked() {
                blow(self.audio.as_ref(), false);
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
            ui.toggle_value(&mut self.slow, "Slow");
            ui.separator();
            ui.add(
                egui::Slider::new(&mut self.zoom, 0.0..=1.0)
                    .show_value(false)
                    .text("zoom"),
            )
            .on_hover_text("the mouse wheel, or two fingers");
            ui.toggle_value(&mut self.follow, "Follow the ball");
            if ui.button("Back to the ball").clicked() {
                self.follow = true;
            }
            ui.add(
                egui::Slider::new(&mut self.tilt, 0.0..=75.0)
                    .suffix("°")
                    .text("tilt"),
            );
            ui.separator();
            ui.spacing_mut().slider_width = (ui.available_width() - 16.0).max(80.0);
            ui.add(egui::Slider::new(&mut self.tick, 0..=last).show_value(true));
        });
    }

    fn commentary(&mut self, ui: &mut egui::Ui) {
        // The bench first: it needs `&mut self`, the rest of this panel only
        // reads the game.
        if self.game.is_some() {
            egui::CollapsingHeader::new("Bench")
                .default_open(false)
                .show(ui, |ui| self.bench_here(ui));
        }
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

    #[allow(
        clippy::too_many_lines,
        reason = "one pass draws the backdrop, the ground, the tokens and the ball"
    )]
    fn pitch(&mut self, ui: &mut egui::Ui) {
        let Some(pitch_cm) = self
            .game
            .as_ref()
            .filter(|g| !g.records.is_empty())
            .map(|g| g.pitch_cm)
        else {
            return;
        };
        let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::click_and_drag());
        let r = response.rect;
        self.steer(ui, &response, pitch_cm);
        let Some(game) = &self.game else { return };
        // One tick behind the playhead, so that the drawing always has the
        // tick before and the tick after (the interpolation layer).
        #[allow(clippy::cast_precision_loss)]
        let instant = (self.tick as f64 + self.carry - 1.0).max(0.0);
        let moment = Moment::at(&game.records, instant, &game.header.physics);
        let (origin, size) = ((r.min.x, r.min.y), (r.width(), r.height()));
        if self.follow && !self.placed {
            // The camera starts on the ball.
            self.focus = (moment.ball.0, moment.ball.1);
            self.placed = true;
        }
        if self.follow {
            // The damped movement: the camera catches up with the ball.
            let (fx, fy) = self.focus;
            let k = (self.last_dt * FOLLOW_RATE).clamp(0.0, 1.0);
            self.focus = (
                (moment.ball.0 - fx).mul_add(k, fx),
                (moment.ball.1 - fy).mul_add(k, fy),
            );
        }
        let span = self.span(game.pitch_cm);
        let geom = Geometry::view(origin, size, game.pitch_cm, self.tilt, (self.focus, span));
        // What the view could not reach stays where the view is.
        self.focus = geom.looks_at();
        let game = self.game.as_ref().expect("the game drawn");
        #[allow(clippy::cast_possible_truncation)]
        let now = instant as f32;
        self.draw_backdrop(
            &painter,
            &geom,
            r,
            &game.backdrop,
            (now, crowd_mood(&game.cheers, now)),
        );
        self.draw_ground(&painter, &geom, &moment);
        let frame = scene::frame(&moment, &geom);
        let t = &self.theme;
        let pos = |s: &scene::Screen| Pos2::new(s.x, s.y);
        // The ball's shadow on the grass, under everything.
        for (g, radius) in &frame.shadows {
            painter.circle_filled(pos(g), *radius, t.ball_shadow);
        }
        // The referee's beak follows the ball, wherever it is in the frame.
        let ball_ground = frame.items.iter().find_map(|item| match item {
            Item::Ball(b) => Some(pos(&b.ground)),
            _ => None,
        });
        let mut hovered = None;
        for item in &frame.items {
            match item {
                Item::Token(tk) | Item::Referee(tk) => {
                    let referee = matches!(item, Item::Referee(_));
                    let fill = if referee {
                        t.referee
                    } else {
                        match (tk.home, tk.keeper) {
                            (true, true) => t.keeper_home,
                            (true, false) => t.home,
                            (false, true) => t.keeper_away,
                            (false, false) => t.away,
                        }
                    };
                    let (centre, radius) =
                        self.draw_token(&painter, &geom, tk, fill, referee, ball_ground);
                    if !referee
                        && response
                            .hover_pos()
                            .is_some_and(|p| p.distance(centre) <= radius.max(6.0))
                    {
                        hovered = Some(tk.id);
                    }
                }
                Item::Ball(b) => {
                    if b.height_cm > 30.0 {
                        // In flight: a light dashed line down to its
                        // shadow, as on television.
                        painter.extend(Shape::dashed_line(
                            &[pos(&b.ground), pos(&b.at)],
                            Stroke::new(1.0_f32, t.ball.gamma_multiply(0.5)),
                            3.0,
                            3.0,
                        ));
                    }
                    painter.circle_filled(pos(&b.at), b.radius, t.ball);
                    painter.circle_stroke(
                        pos(&b.at),
                        b.radius,
                        Stroke::new((b.radius * 0.18).max(1.0), t.ball_edge),
                    );
                }
            }
        }
        // Law 6 made visible: the stop whistle holds the picture, then the
        // dead ball is played at real speed while the taker walks to it.
        let last = game.whistles.iter().rfind(|w| w.tick <= self.tick);
        if let Some(stop) = last.filter(|w| !w.restart) {
            // The check lasts the held picture, the same number of match
            // ticks at 1x: the view in a recording reads the same lines as
            // the one watching live.
            let held = self.tick < stop.tick.saturating_add(CEREMONY_TICKS);
            let (chip, text) = banner(stop, held);
            var_lower_third(&painter, r, &self.theme, chip, text);
            // The broadcast lines of Law 11, from the pass and not from the
            // whistle: the second-last opponent and the flagged attacker.
            #[allow(clippy::cast_precision_loss)]
            let width = game.pitch_cm.1 as f32;
            if let Some((line, man, _)) = stop.offside_lines {
                #[allow(clippy::cast_precision_loss, reason = "drawing only: cm fit the board")]
                let draw = |x: i32, colour| {
                    if let (Some(a), Some(b)) = (
                        geom.project((x as f32, 0.0, 0.0)),
                        geom.project((x as f32, width, 0.0)),
                    ) {
                        painter.line_segment(
                            [Pos2::new(a.x, a.y), Pos2::new(b.x, b.y)],
                            Stroke::new(3.0_f32, colour),
                        );
                    }
                };
                draw(line, self.theme.card_yellow);
                draw(man, self.theme.critical);
            }
            #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
            let progress =
                ((self.tick - stop.tick).min(CEREMONY_TICKS) as f32) / CEREMONY_TICKS as f32;
            if let Some(record) = game.records.get(stop.call_tick) {
                draw_card_ceremony(
                    &painter,
                    &geom,
                    &self.theme,
                    moment.referee,
                    record,
                    stop,
                    self.choices.captain.unwrap_or('D'),
                    progress,
                );
            }
        }
        let clicked = response.clicked();
        if let Some(id) = hovered {
            if let Some(name) = game.names.get(&id) {
                response.on_hover_text(format!("{} {name}", ds_demo::pitchview::marker(id)));
            }
        }
        if clicked {
            // One of his own: his post-it (ADR-059 step 5); elsewhere, it
            // closes.
            self.chosen = hovered.filter(|id| id.slot() < 100);
        }
    }

    /// The post-it of the token clicked (ADR-059 step 5): what the coach
    /// asks of him with the ball and without it, as an order of the board
    /// at the tick on the screen.
    fn player_postit(&mut self, ctx: &egui::Context) {
        use ds_model::Task as T;
        let (Some(id), Some(game)) = (self.chosen, &self.game) else {
            return;
        };
        let Some(slot) = game.home.0.iter().position(|p| *p == id).or_else(|| {
            // A man who came on takes the slot of the man he replaced.
            game.records.get(self.tick).and_then(|r| {
                let on: Vec<PlayerId> = r
                    .agents
                    .iter()
                    .map(|a| a.id)
                    .filter(|p| p.slot() < 100)
                    .collect();
                game.home
                    .0
                    .iter()
                    .position(|p| !on.contains(p))
                    .filter(|_| on.contains(&id))
            })
        }) else {
            return;
        };
        let mut board = self.board_now();
        let name = game.names.get(&id).cloned().unwrap_or_default();
        let t = &self.theme;
        let (paper, ink, shade) = (t.postit, t.postit_ink, t.postit_shadow);
        let label = |task: T| match task {
            T::Free => "his own reading",
            T::Run => "attack the depth",
            T::Offer => "come short",
            T::Stay => "stay back",
            T::Press => "go out and press",
            T::Cover => "cover",
            T::Mark => "mark his man",
        };
        let before = board;
        let mut close = false;
        egui::Area::new(egui::Id::new("player-postit"))
            .anchor(egui::Align2::LEFT_TOP, egui::vec2(12.0, 360.0))
            .show(ctx, |ui| {
                egui::Frame::NONE
                    .fill(paper)
                    .inner_margin(10.0)
                    .shadow(egui::epaint::Shadow {
                        offset: [2, 3],
                        blur: 6,
                        spread: 0,
                        color: shade,
                    })
                    .show(ui, |ui| {
                        ui.style_mut().visuals = egui::Visuals::light();
                        ui.visuals_mut().override_text_color = Some(ink);
                        ui.set_max_width(220.0);
                        ui.horizontal(|ui| {
                            ui.strong(format!("{} {name}", slot + 1));
                            if ui.small_button("×").clicked() {
                                close = true;
                            }
                        });
                        ui.label("With the ball");
                        let with = &mut board.with_ball_tasks[slot];
                        egui::ComboBox::from_id_salt("task-with")
                            .selected_text(label(*with))
                            .show_ui(ui, |ui| {
                                for k in [T::Free, T::Run, T::Offer, T::Stay] {
                                    ui.selectable_value(with, k, label(k));
                                }
                            });
                        ui.label("Without the ball");
                        let without = &mut board.without_ball_tasks[slot];
                        egui::ComboBox::from_id_salt("task-without")
                            .selected_text(label(*without))
                            .show_ui(ui, |ui| {
                                for k in [T::Free, T::Press, T::Cover, T::Mark, T::Stay] {
                                    ui.selectable_value(without, k, label(k));
                                }
                            });
                    });
            });
        if close {
            self.chosen = None;
        }
        if board != before {
            self.order(ds_model::Order::Board(board));
        }
    }

    /// The board as the coach has it now: the squad's, with every board
    /// he ordered since.
    fn board_now(&self) -> ds_model::Board {
        let mut board = ds_fixtures::match_fixture(self.seed)
            .map(|m| m.home.board)
            .unwrap_or_default();
        for i in &self.choices.live {
            if let ds_model::Order::Board(b) = i.order {
                board = b;
            }
        }
        board
    }

    /// The camera's span across the screen at the current zoom, cm: from
    /// the whole pitch down to about 25 m, evenly in the eye (a log scale).
    fn span(&self, pitch_cm: (i32, i32)) -> f32 {
        let whole = Geometry::whole_span(pitch_cm);
        #[allow(
            clippy::disallowed_methods,
            reason = "drawing only: the view never feeds the simulation"
        )]
        let span = whole * (scene::NEAREST_SPAN_CM / whole).powf(self.zoom.clamp(0.0, 1.0));
        span
    }

    /// The hand on the board (DESIGN.md, "Una partita, molte viste"): the
    /// wheel or two fingers zoom, dragging moves the view and turns
    /// "follow the ball" off, a double click or tap turns it back on.
    fn steer(&mut self, ui: &egui::Ui, response: &egui::Response, pitch_cm: (i32, i32)) {
        if response.hovered() {
            let (wheel, pinch) = ui.input(|i| (i.smooth_scroll_delta.y, i.zoom_delta()));
            #[allow(
                clippy::disallowed_methods,
                reason = "drawing only: the view never feeds the simulation"
            )]
            let steps = pinch.ln().mul_add(1.5, wheel / 400.0);
            self.zoom = (self.zoom + steps).clamp(0.0, 1.0);
        }
        if response.dragged() {
            let d = response.drag_delta();
            if d.length() > 0.0 {
                self.follow = false;
                // Points to cm at the focus: the span over the width; up
                // and down the tilt shortens the grass.
                let per = self.span(pitch_cm) / response.rect.width().max(1.0);
                #[allow(
                    clippy::disallowed_methods,
                    reason = "drawing only: the view never feeds the simulation"
                )]
                let lean = self.tilt.to_radians().cos().max(0.3);
                self.focus = (
                    d.x.mul_add(-per, self.focus.0),
                    (d.y * per).mul_add(1.0 / lean, self.focus.1),
                );
            }
        }
        if response.double_clicked() {
            self.follow = true;
        }
    }

    /// The direction of the mark on a token. Only the referee's is corrected:
    /// he watches the ball, the players watch their own read.
    fn beak_dir(ground: Pos2, facing: Pos2, ball: Option<Pos2>, referee: bool) -> egui::Vec2 {
        let ahead = if referee {
            ball.map(|b| b - ground)
        } else {
            Some(facing - ground)
        };
        ahead
            .filter(|v| v.length() > 1e-3)
            .map_or(egui::vec2(0.0, -1.0), egui::Vec2::normalized)
    }

    /// One token (DESIGN.md, "Pallini nel 2.5D, omini nel 3D"): its shadow
    /// on the grass, the ring of the man on the ball, the disc of his side
    /// with its number and a thin darker edge, the mark of where he faces;
    /// shaped by the gesture a fact of the match gives him. Never smaller
    /// than the readable size. Returns its centre and radius.
    #[allow(
        clippy::disallowed_methods,
        reason = "drawing only: the view never feeds the simulation"
    )]
    #[allow(
        clippy::suboptimal_flops,
        clippy::too_many_lines,
        clippy::similar_names,
        reason = "one token, drawn in order: shadow, ring, disc, mark, number, card"
    )]
    fn draw_token(
        &self,
        painter: &Painter,
        geom: &Geometry,
        tk: &scene::Token,
        fill: egui::Color32,
        referee: bool,
        ball: Option<Pos2>,
    ) -> (Pos2, f32) {
        use std::f32::consts::PI;
        let t = &self.theme;
        let ground = Pos2::new(tk.ground.x, tk.ground.y);
        let r = geom
            .len_at(scene::TOKEN_CM, (tk.at.0, tk.at.1, 0.0))
            .max(scene::MIN_TOKEN_PX);
        // Where he faces, on the screen: the referee always faces the ball.
        let dir = Self::beak_dir(ground, Pos2::new(tk.facing.x, tk.facing.y), ball, referee);
        let side = egui::vec2(-dir.y, dir.x);
        // The gesture: lift, stretch across and along, a shift, a lean.
        let (mut lift, mut wide, mut tall, mut shift, mut lean) =
            (0.0_f32, 1.0_f32, 1.0_f32, egui::Vec2::ZERO, 0.0_f32);
        let mut card = None;
        if let Some((g, p)) = self.gesture_now(tk.id) {
            let bump = (PI * p).sin();
            match g {
                scene::Gesture::Jump => lift = 1.6 * r * bump,
                scene::Gesture::Dive(s) => {
                    wide = 0.9f32.mul_add(bump, 1.0);
                    tall = 0.35f32.mul_add(-bump, 1.0);
                    shift = side * (s * 1.2 * r * bump);
                    lean = 0.6 * s * bump;
                }
                scene::Gesture::Slide | scene::Gesture::Lunge => {
                    tall = 0.45f32.mul_add(-bump, 1.0);
                    wide = 0.25f32.mul_add(bump, 1.0);
                    shift = dir * (0.8 * r * p);
                }
                scene::Gesture::Fall => {
                    // Down at once, up slowly.
                    let down = if p < 0.15 {
                        p / 0.15
                    } else {
                        1.0 - (p - 0.15) / 0.85
                    };
                    tall = 0.6f32.mul_add(-down, 1.0);
                    wide = 0.3f32.mul_add(down, 1.0);
                }
                scene::Gesture::Kick | scene::Gesture::Throw => {
                    shift = dir * (0.35 * r * bump);
                    wide = 0.12f32.mul_add(bump, 1.0);
                    tall = wide;
                }
                scene::Gesture::Through => {
                    // The same kick, stretched towards the space he has read.
                    shift = dir * (0.6 * r * bump);
                    lean = 0.25 * bump;
                    wide = 0.12f32.mul_add(bump, 1.0);
                    tall = wide;
                }
                scene::Gesture::Celebrate => lift = 0.9 * r * (3.0 * PI * p).sin().abs(),
                scene::Gesture::Card(red) => card = Some(red),
            }
        }
        // The shadow stays on the grass.
        let shadow_tilt = self.tilt.to_radians().cos().clamp(0.35, 1.0);
        painter.add(Shape::convex_polygon(
            ellipse(
                ground + shift,
                r * 1.05 * wide,
                r * 0.55 * shadow_tilt.max(0.5),
                0.0,
            ),
            t.ball_shadow,
            Stroke::NONE,
        ));
        if tk.carrier {
            painter.add(Shape::closed_line(
                ellipse(ground, r * 1.6, r * 1.6 * shadow_tilt.max(0.5), 0.0),
                Stroke::new((r * 0.18).max(1.5), t.carrier_ring),
            ));
        }
        // The disc stands on its spot: its centre one radius up.
        let centre = ground + shift - egui::vec2(0.0, r * tall + lift);
        // A thin darker edge; a light one round a black disc (the referee).
        let rim = if luminance(fill) < 0.15 {
            t.rim_light
        } else {
            fill.gamma_multiply(0.45)
        };
        let edge = Stroke::new((r * 0.12).max(1.0), rim);
        let disc = ellipse(centre, r * wide, r * tall, lean);
        painter.add(Shape::convex_polygon(disc.clone(), fill, Stroke::NONE));
        painter.add(Shape::closed_line(disc, edge));
        // The eyes, not the run (ADR-034): a small tip in the token's own
        // colour, just outside the disc. Smaller than the ball, with the
        // disc's edge as its outline, so it reads without taking over.
        let edge_of = centre + dir * (r * wide.max(tall) * 0.92);
        let tip = edge_of + dir * (r * 0.65);
        let base = edge_of + dir * (r * 0.03);
        let arrow = vec![tip, base + side * (r * 0.3), base - side * (r * 0.3)];
        painter.add(Shape::convex_polygon(
            arrow.clone(),
            fill.gamma_multiply(0.8),
            Stroke::NONE,
        ));
        painter.add(Shape::closed_line(
            arrow,
            Stroke::new((r * 0.12).max(1.2), rim),
        ));
        if !referee {
            let size = (r * 1.15).max(8.0);
            let ink = if luminance(fill) > 0.55 {
                t.number_dark
            } else {
                t.player_text
            };
            painter.text(
                centre,
                Align2::CENTER_CENTER,
                tk.number.to_string(),
                FontId::proportional(size),
                ink,
            );
        }
        if let Some(red) = card {
            let colour = if red { t.card_red } else { t.card_yellow };
            let h = (r * 1.4).max(9.0);
            let at = centre - egui::vec2(-r * 0.9, r * 1.6 + h / 2.0);
            painter.rect_filled(
                egui::Rect::from_center_size(at, egui::vec2(h * 0.7, h)),
                1.5,
                colour,
            );
        }
        (centre, r)
    }

    /// The gesture `id` makes at the instant drawn, and how far into it:
    /// each gesture a few ticks either side of its fact (a kick loads
    /// before the contact, a dive lasts after it).
    fn gesture_now(&self, id: PlayerId) -> Option<(scene::Gesture, f32)> {
        let game = self.game.as_ref()?;
        #[allow(clippy::cast_precision_loss)]
        let t = self.tick as f64 + self.carry - 1.0;
        game.gestures.get(&id)?.iter().find_map(|&(tick, g)| {
            let (before, after) = match g {
                scene::Gesture::Kick => (5.0, 2.0),
                scene::Gesture::Through => (6.0, 2.0),
                scene::Gesture::Jump => (3.0, 4.0),
                scene::Gesture::Lunge => (2.0, 3.0),
                scene::Gesture::Dive(_) | scene::Gesture::Slide => (2.0, 8.0),
                scene::Gesture::Fall => (0.0, 20.0),
                scene::Gesture::Throw => (8.0, 2.0),
                // DD-37: the party lasts the measured pause from the goal to
                // the kick-off, so it can be seen at the default x16.
                scene::Gesture::Celebrate => (
                    0.0,
                    ds_match::balance::StoppageTime::load().goal_to_kick_off_s * TICKS_PER_SECOND,
                ),
                scene::Gesture::Card(_) => (0.0, 25.0),
            };
            #[allow(clippy::cast_precision_loss)]
            let start = tick as f64 - before;
            let p = (t - start) / (before + after);
            #[allow(clippy::cast_possible_truncation)]
            (0.0..=1.0).contains(&p).then_some((g, p as f32))
        })
    }

    /// Saves the screenshots of the recording as PPM images, and closes
    /// the window when it has them all.
    fn save_screenshots(&mut self, ctx: &egui::Context) {
        let shots: Vec<std::sync::Arc<egui::ColorImage>> = ctx.input(|i| {
            i.events
                .iter()
                .filter_map(|e| match e {
                    egui::Event::Screenshot { image, .. } => Some(image.clone()),
                    _ => None,
                })
                .collect()
        });
        let Some((dir, wanted, saved)) = &mut self.record else {
            return;
        };
        for image in shots {
            let [w, h] = image.size;
            let mut bytes = format!("P6\n{w} {h}\n255\n").into_bytes();
            for p in &image.pixels {
                bytes.extend_from_slice(&[p.r(), p.g(), p.b()]);
            }
            let path = std::path::Path::new(dir.as_str()).join(format!("frame_{saved:05}.ppm"));
            if let Err(e) = std::fs::write(&path, bytes) {
                eprintln!("ds-gui: {}: {e}", path.display());
            }
            *saved += 1;
            self.rec_clock.2 = false;
        }
        if *saved >= *wanted {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    /// The stadium round the pitch and its town under the sky (drawing
    /// only, `stadium.rs`): the sky down to the horizon, then the town and
    /// the stands from the farthest shape to the nearest.
    #[allow(
        clippy::disallowed_methods,
        reason = "drawing only: the view never feeds the simulation"
    )]
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "colours and mesh indices: small, positive, clamped"
    )]
    fn draw_backdrop(
        &self,
        painter: &Painter,
        geom: &Geometry,
        rect: egui::Rect,
        (city, stands): &(Vec<crate::stadium::Shape>, Vec<crate::stadium::Shape>),
        (now, (home_up, away_up)): (f32, (f32, f32)),
    ) {
        use crate::stadium::{Paint, mix};
        let st = &self.theme.stadium;
        painter.rect_filled(rect, 0.0, st.ground);
        if let Some(h) = geom.horizon() {
            let h = h.clamp(rect.top(), rect.bottom());
            let mut mesh = egui::Mesh::default();
            let i = mesh.vertices.len() as u32;
            for (p, c) in [
                (rect.left_top(), st.sky),
                (rect.right_top(), st.sky),
                (Pos2::new(rect.right(), h), st.horizon),
                (Pos2::new(rect.left(), h), st.horizon),
            ] {
                mesh.colored_vertex(p, c);
            }
            mesh.add_triangle(i, i + 1, i + 2);
            mesh.add_triangle(i, i + 2, i + 3);
            painter.add(Shape::mesh(mesh));
        }
        let cut_near = geom.tilt() > 0.09;
        let t = &self.theme;
        let colour = |paint: Paint| match paint {
            Paint::Track => st.track,
            Paint::Concrete => st.concrete,
            Paint::Roof => st.roof,
            Paint::Mast => st.mast,
            Paint::Lamps => st.lamps,
            Paint::Window => st.window,
            Paint::Building(k) => st.city[usize::from(k.min(3))],
            Paint::Crowd(h) => {
                // Mostly the home side's colours, some away, the rest in
                // their own clothes; all of it toned down to the crowd's.
                let side = match h % 100 {
                    0..38 => t.home,
                    38..58 => t.away,
                    _ => st.crowd,
                };
                let base = side.lerp_to_gamma(st.crowd, 0.35);
                // Faces and clothes: each block a little lighter or darker.
                #[allow(clippy::cast_possible_truncation)]
                let k = 0.7 + f32::from((mix(h) % 60) as u8) / 100.0;
                egui::Color32::from_rgb(
                    (f32::from(base.r()) * k).min(255.0) as u8,
                    (f32::from(base.g()) * k).min(255.0) as u8,
                    (f32::from(base.b()) * k).min(255.0) as u8,
                )
            }
        };
        // The fans as small tokens on their steps, when they are big
        // enough to see; farther off, the steps take their colours.
        let fan = |painter: &Painter, pts: &[Pos2], h: u32| {
            let lerp = |a: Pos2, b: Pos2, k: f32| a + (b - a) * k;
            let across = (pts[1] - pts[0]).length();
            let r = (across / 6.0).min((pts[3] - pts[0]).length() * 0.45);
            if r < 1.1 {
                return false;
            }
            painter.add(Shape::convex_polygon(
                pts.to_vec(),
                st.concrete,
                Stroke::NONE,
            ));
            for k in 0..3_u32 {
                #[allow(clippy::cast_precision_loss)]
                let at = (k as f32 + 0.5) / 3.0;
                let c = lerp(lerp(pts[0], pts[1], at), lerp(pts[3], pts[2], at), 0.4);
                let who = mix(h ^ (k + 1).wrapping_mul(0x51ed));
                let fill = colour(Paint::Crowd(who));
                // A fan of the side that just scored or shot hops (only
                // from a fact of the match; the others stay seated).
                let up = match who % 100 {
                    0..38 => home_up,
                    38..58 => away_up,
                    _ => 0.0,
                };
                #[allow(clippy::cast_precision_loss)]
                let phase = (who >> 8 & 0xff) as f32 / 40.0;
                let hop = up * now.mul_add(0.8, phase).sin().abs() * r * 1.4;
                painter.circle_filled(c - egui::vec2(0.0, r.mul_add(0.6, hop)), r, fill);
            }
            true
        };
        for shapes in [city, stands] {
            let mut drawn: Vec<(f32, Vec<Pos2>, Paint)> = shapes
                .iter()
                .filter(|s| !(cut_near && s.near))
                .filter_map(|s| {
                    let pts: Option<Vec<_>> = s.corners.iter().map(|&c| geom.project(c)).collect();
                    let pts = pts?;
                    #[allow(clippy::cast_precision_loss)]
                    let depth = pts.iter().map(|p| p.depth).sum::<f32>() / pts.len() as f32;
                    let screen: Vec<Pos2> = pts.iter().map(|p| Pos2::new(p.x, p.y)).collect();
                    let r = egui::Rect::from_points(&screen);
                    r.intersects(rect).then_some((depth, screen, s.paint))
                })
                .collect();
            drawn.sort_by(|a, b| b.0.total_cmp(&a.0));
            for (_, pts, paint) in drawn {
                if let Paint::Crowd(h) = paint
                    && pts.len() == 4
                    && fan(painter, &pts, h)
                {
                    continue;
                }
                painter.add(Shape::convex_polygon(pts, colour(paint), Stroke::NONE));
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

/// Every player's gestures, from the facts of the match (DESIGN.md, the
/// table of the animations): the kick of a pass, a cross, a shot or a
/// clearance; the jump of a header or an aerial duel; the keeper's dive
/// towards the ball's side; the slide of a sliding foul; the lunge of a
/// tackle won.
fn gestures(
    records: &[ds_match::TickRecord],
    events: &[ds_stats::MatchEvent],
) -> BTreeMap<PlayerId, Vec<(usize, scene::Gesture)>> {
    let mut out: BTreeMap<PlayerId, Vec<(usize, scene::Gesture)>> = BTreeMap::new();
    let flag = |e: &ds_stats::MatchEvent, k: &str| {
        e.fields.get(k).and_then(serde_json::Value::as_bool) == Some(true)
    };
    for (k, r) in records.iter().enumerate() {
        for e in events.get(r.events.clone()).unwrap_or(&[]) {
            // DD-37 (LUNA): the goal is a fact of the side whose score rose.
            // Every man of that side in the field celebrates — read from the
            // score, so an own goal is the right side's party. No assist, no
            // name: only the score and the men on the grass (ADR-065 §3).
            if e.event_type == ds_stats::EventType::Goal {
                let (mut home, mut away) = (false, false);
                if let Some(prev) = k.checked_sub(1).map(|j| &records[j].score) {
                    home = r.score.home > prev.home;
                    away = r.score.away > prev.away;
                }
                for a in &r.agents {
                    if (a.id.slot() < 100 && home) || (a.id.slot() >= 100 && away) {
                        out.entry(a.id)
                            .or_default()
                            .push((k + CEREMONY_TICKS, scene::Gesture::Celebrate));
                    }
                }
                continue;
            }
            let Some(ds_core::ids::ActorId::Player(p)) = e.actor else {
                continue;
            };
            let header = e
                .fields
                .get("shot_type")
                .and_then(serde_json::Value::as_str)
                == Some("header")
                || flag(e, "header");
            let g = match crate::animation::animation(e.event_type) {
                crate::animation::Animation::KickOrHeader if header => scene::Gesture::Jump,
                crate::animation::Animation::KickOrHeader if flag(e, "through") => {
                    scene::Gesture::Through
                }
                crate::animation::Animation::KickOrHeader => scene::Gesture::Kick,
                crate::animation::Animation::JumpIfAerial if flag(e, "aerial") => {
                    scene::Gesture::Jump
                }
                crate::animation::Animation::Dive => {
                    // Towards the ball's side, as he faces.
                    let side = r.agents.iter().find(|a| a.id == p).map_or(1.0, |a| {
                        #[allow(
                            clippy::disallowed_methods,
                            reason = "drawing only: the view never feeds the simulation"
                        )]
                        let (s, c) = f32::from(a.heading).to_radians().sin_cos();
                        #[allow(clippy::cast_precision_loss)]
                        let (dx, dy) = (
                            (r.ball.position.0 - a.position.0) as f32,
                            (r.ball.position.1 - a.position.1) as f32,
                        );
                        if dx.mul_add(c, -(dy * s)) >= 0.0 {
                            1.0
                        } else {
                            -1.0
                        }
                    });
                    scene::Gesture::Dive(side)
                }
                crate::animation::Animation::Foul => {
                    // The man fouled falls (the field `against`, his slot).
                    let fouled = e
                        .fields
                        .get("against")
                        .and_then(serde_json::Value::as_u64)
                        .and_then(|slot| r.agents.iter().find(|a| u64::from(a.id.slot()) == slot));
                    if let Some(a) = fouled {
                        out.entry(a.id).or_default().push((k, scene::Gesture::Fall));
                    }
                    if flag(e, "slide") {
                        scene::Gesture::Slide
                    } else {
                        continue;
                    }
                }
                crate::animation::Animation::Card => {
                    out.entry(scene::REFEREE).or_default().push((
                        k,
                        scene::Gesture::Card(
                            e.fields.get("color").and_then(serde_json::Value::as_str)
                                == Some("red"),
                        ),
                    ));
                    continue;
                }
                crate::animation::Animation::Down => scene::Gesture::Fall,
                crate::animation::Animation::Throw => scene::Gesture::Throw,
                crate::animation::Animation::Celebrate => scene::Gesture::Celebrate,
                crate::animation::Animation::Lunge => scene::Gesture::Lunge,
                // The fallback pose: no gesture.
                crate::animation::Animation::JumpIfAerial | crate::animation::Animation::Rest => {
                    continue;
                }
            };
            out.entry(p).or_default().push((k, g));
        }
    }
    out
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
        if self.record.is_some() {
            // Recording: frame n is exactly n/60 s of playback, however
            // many times egui draws it; the camera moves once a frame. A
            // new frame is asked for only when the last one is saved, and
            // each carries its number in a corner pixel, so that a frame
            // saved twice is dropped and none is missing.
            self.save_screenshots(ctx);
            let moving = !self.rec_clock.2;
            if moving {
                self.rec_clock.1 += 1;
            }
            let n = self.rec_clock.1 - 1;
            #[allow(clippy::cast_precision_loss)]
            let ticks = n as f64 * TICKS_PER_SECOND * self.speed / 60.0;
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let whole = ticks.floor() as usize;
            self.tick = self.rec_clock.0 + whole;
            self.carry = ticks - ticks.floor();
            self.last_dt = if moving { 1.0 / 60.0 } else { 0.0 };
            #[allow(clippy::cast_possible_truncation)]
            let stamp = egui::Color32::from_rgb((n % 256) as u8, (n / 256 % 256) as u8, 77);
            ctx.layer_painter(egui::LayerId::new(
                egui::Order::Foreground,
                egui::Id::new("stamp"),
            ))
            .rect_filled(
                egui::Rect::from_min_size(Pos2::ZERO, egui::vec2(3.0, 3.0)),
                0.0,
                stamp,
            );
        } else {
            #[allow(clippy::cast_possible_truncation)]
            let dt32 = dt as f32;
            self.last_dt = dt32;
            self.advance(dt);
        }
        ctx.set_visuals(if self.theme.dark {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        });
        egui::TopBottomPanel::top("choices").show(ctx, |ui| self.choices_bar(ui));
        egui::TopBottomPanel::bottom("playback").show(ctx, |ui| self.playback_bar(ui));
        // A scomparsa: the right panel opens on hover at the edge and stays
        // while the pointer is over it (or over a menu it opened).
        let pointer = ctx.pointer_hover_pos();
        let screen = ctx.screen_rect();
        let at_edge =
            pointer.is_some_and(|p| p.x >= screen.max.x - 28.0 && p.y > screen.min.y + 40.0);
        let over_panel = pointer.is_some_and(|p| {
            self.panel_rect.is_some_and(|r| {
                p.x >= r.min.x - 260.0
                    && p.x <= r.max.x + 8.0
                    && p.y >= r.min.y - 8.0
                    && p.y <= r.max.y + 8.0
            })
        });
        let holding = ctx.input(|i| i.pointer.primary_down());
        self.panel_open = at_edge || (self.panel_open && (over_panel || holding));
        if self.panel_open {
            let panel = egui::SidePanel::right("commentary")
                .default_width(380.0)
                .show(ctx, |ui| self.commentary(ui));
            self.panel_rect = Some(panel.response.rect);
        } else {
            egui::SidePanel::right("commentary-handle")
                .default_width(14.0)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label("<");
                    });
                });
            self.panel_rect = None;
        }
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(self.theme.board))
            .show(ctx, |ui| self.pitch(ui));
        if self.record.is_none() {
            self.player_postit(ctx);
        }
        if self.record.is_some() {
            // One screenshot at a time: the next frame is drawn only after
            // this one is saved.
            if !self.rec_clock.2 {
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::UserData::default()));
                self.rec_clock.2 = true;
            }
            ctx.request_repaint();
        } else if self.playing {
            ctx.request_repaint();
        }
    }
}

/// The menus of the orders during the match: the block's height and width,
/// more pressing, everyone forward, the two transitions; the orders the
/// changes give (the maintainer's pure error 2d).
fn order_menus(
    ui: &mut egui::Ui,
    intention: ds_model::Intention,
    now: ds_model::TeamOrders,
) -> Vec<ds_model::Order> {
    let mut give = Vec::new();
    let (mut block, mut width) = (intention.block, intention.width);
    egui::ComboBox::from_id_salt("live-block")
        .selected_text(format!("Block: {}", block_label(block)))
        .show_ui(ui, |ui| {
            for b in [BlockHeight::High, BlockHeight::Mid, BlockHeight::Low] {
                ui.selectable_value(&mut block, b, block_label(b));
            }
        });
    egui::ComboBox::from_id_salt("live-width")
        .selected_text(format!("Width: {}", width_label(width)))
        .show_ui(ui, |ui| {
            for w in [Width::Narrow, Width::Normal, Width::Wide] {
                ui.selectable_value(&mut width, w, width_label(w));
            }
        });
    if (block, width) != (intention.block, intention.width) {
        give.push(ds_model::Order::Intention(ds_model::Intention {
            block,
            width,
        }));
    }
    for (quick, label, on) in [
        (ds_model::Quick::MorePressing, "Pressing", now.press_cm > 0),
        (
            ds_model::Quick::EveryoneForward,
            "Everyone forward",
            now.forward_cm > 0,
        ),
    ] {
        let mut v = on;
        egui::ComboBox::from_id_salt(label)
            .selected_text(format!("{label}: {}", if v { "more" } else { "normal" }))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut v, false, "normal");
                ui.selectable_value(&mut v, true, "more");
            });
        if v != on {
            give.push(ds_model::Order::Quick { quick, on: v });
        }
    }
    let (mut lost, mut won) = (now.lost, now.won);
    egui::ComboBox::from_id_salt("on-loss")
        .selected_text(match lost {
            ds_model::OnLoss::Shape => "Ball lost: shape",
            ds_model::OnLoss::CounterPress => "Ball lost: counter-press",
            ds_model::OnLoss::DropBack => "Ball lost: drop back",
        })
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut lost, ds_model::OnLoss::Shape, "shape");
            ui.selectable_value(&mut lost, ds_model::OnLoss::CounterPress, "counter-press");
            ui.selectable_value(&mut lost, ds_model::OnLoss::DropBack, "drop back");
        });
    egui::ComboBox::from_id_salt("on-win")
        .selected_text(match won {
            ds_model::OnWin::Shape => "Ball won: shape",
            ds_model::OnWin::Counter => "Ball won: counter",
            ds_model::OnWin::Keep => "Ball won: keep it",
        })
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut won, ds_model::OnWin::Shape, "shape");
            ui.selectable_value(&mut won, ds_model::OnWin::Counter, "counter");
            ui.selectable_value(&mut won, ds_model::OnWin::Keep, "keep it");
        });
    if (lost, won) != (now.lost, now.won) {
        give.push(ds_model::Order::Transitions { lost, won });
    }
    give
}

/// The referee's signal (Law 6), made of two sines so it trills: the stop
/// is longer than the restart blip. No audio asset to ship, no match state.
fn blow(audio: Option<&Arc<(OutputStream, Sink)>>, restart: bool) {
    let Some(audio) = audio else { return };
    let sink = &audio.1;
    let hz = if restart { 2100.0 } else { 2600.0 };
    let millis = if restart { 90 } else { 330 };
    let trill = SineWave::new(hz)
        .mix(SineWave::new(hz * 1.02_f32))
        .amplify(0.35)
        .take_duration(Duration::from_millis(millis))
        .fade_in(Duration::from_millis(8))
        .fade_out(Duration::from_millis(40));
    sink.append(trill);
}

/// What stopped play (Law 6), read from the fact, not guessed from the ball.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum StopKind {
    /// A foul that really stopped play (a foul with advantage goes on).
    Foul,
    /// Law 11, read from the free kick's reason.
    Offside,
    Goal,
    ThrowIn,
    GoalKick,
    Corner,
}

impl StopKind {
    /// The sentence while the picture holds, then after it. The maintainer
    /// asked for a broadcast lower third, not a caption across the pitch:
    /// only the view waits; the event stays append-only.
    const fn texts(self) -> (&'static str, &'static str) {
        match self {
            Self::Foul => ("Free kick", "Taking their places"),
            Self::Offside => ("Checking offside", "Offside confirmed"),
            Self::Goal => ("Checking the goal", "Goal confirmed"),
            Self::ThrowIn => ("Throw-in", "Taking their places"),
            Self::GoalKick => ("Goal kick", "Taking their places"),
            Self::Corner => ("Corner", "Taking their places"),
        }
    }
}

/// The lower third at a stop: its chip and sentence. A card owns the line
/// even when it comes with a foul; only a review gets the VAR chip.
#[allow(clippy::missing_const_for_fn)]
fn banner(stop: &Whistle, held: bool) -> (&'static str, &'static str) {
    if let Some(card) = stop.card {
        return match (held, card.red) {
            (true, _) => ("VAR", "Checking the card"),
            (false, true) => ("VAR", "Red card confirmed"),
            (false, false) => ("VAR", "Yellow card confirmed"),
        };
    }
    let chip = match stop.stop {
        Some(StopKind::Offside | StopKind::Goal) => "VAR",
        _ => "",
    };
    let lines = stop.stop.map_or(("", ""), StopKind::texts);
    (chip, if held { lines.0 } else { lines.1 })
}

/// A broadcast lower third at the bottom of the board: a small chip and the
/// sentence beside it, instead of a band of text over the play.
fn var_lower_third(painter: &Painter, r: egui::Rect, theme: &Theme, chip: &str, text: &str) {
    let size = 15.0;
    let chip_w = if chip.is_empty() { 0.0 } else { 34.0 };
    #[allow(clippy::cast_precision_loss)]
    let text_w = text.len() as f32 * size * 0.54;
    let w = chip_w + text_w + 20.0;
    let at = Pos2::new(r.center().x, r.max.y - 24.0);
    let band = egui::Rect::from_center_size(at, egui::vec2(w, 26.0));
    painter.rect_filled(
        band,
        4.0,
        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 190),
    );
    let mut left = band.min.x + 9.0;
    if !chip.is_empty() {
        let tag =
            egui::Rect::from_min_size(Pos2::new(left, band.min.y + 4.0), egui::vec2(chip_w, 18.0));
        painter.rect_filled(tag, 3.0, theme.card_yellow);
        painter.text(
            tag.center(),
            Align2::CENTER_CENTER,
            chip,
            FontId::proportional(11.0),
            theme.number_dark,
        );
        left = tag.max.x + 8.0;
    }
    painter.text(
        Pos2::new(left, band.center().y),
        Align2::LEFT_CENTER,
        text,
        FontId::proportional(size),
        theme.lines,
    );
}

/// A card at a stop: the man and the colour, both facts of the event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CardShown {
    player: PlayerId,
    red: bool,
}

/// A whistle the view hears from the event chain (Law 6: stop, then play on).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Whistle {
    /// When the view shows the whistle (for offside, after the action).
    tick: usize,
    /// When the fact happened: the lines are read from this tick.
    call_tick: usize,
    /// `None` on the restart whistle: nothing stopped then.
    stop: Option<StopKind>,
    card: Option<CardShown>,
    /// The offender's slot at an offside (Law 11).
    offside: Option<u32>,
    /// The defensive line and the attacker at the pass, and the tick read.
    offside_lines: Option<(i32, i32, usize)>,
    /// Who was fouled: his captain speaks for him.
    fouled: Option<u32>,
    restart: bool,
}

/// The two lines of an offside, read from the tick of the pass that made
/// the run (Law 11, `ds_match::rules::offside_position`): the second-last
/// opponent, counting from the goal, and the flagged attacker.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn offside_lines(
    records: &[ds_match::TickRecord],
    events: &[ds_stats::MatchEvent],
    stop: &Whistle,
    length: i32,
) -> Option<(i32, i32, usize)> {
    use ds_core::ids::ActorId;
    use ds_stats::EventType;
    let offender = stop.offside?;
    let goal_x = if offender < 100 { length } else { 0 };
    let depth = |x: i32| (goal_x - x).abs();
    // The most recent pass by his side that left him offside is the one
    // the engine flagged him on: the same Law 11 function, read from the
    // facts. Other passes in the move did not make the run.
    for (k, record) in records.iter().enumerate().take(stop.call_tick).rev() {
        let passed = events
            .get(record.events.clone())
            .unwrap_or(&[])
            .iter()
            .any(|e| {
                let same_side = match e.actor {
                    Some(ActorId::Player(p)) => p.slot() / 100 == offender / 100,
                    _ => false,
                };
                same_side
                    && matches!(
                        e.event_type,
                        EventType::Pass | EventType::Cross | EventType::KeyPass
                    )
            });
        if !passed {
            continue;
        }
        let mut opponents: Vec<i32> = record
            .agents
            .iter()
            .filter(|a| (a.id.slot() < 100) != (offender < 100))
            .map(|a| a.position.0)
            .collect();
        let man = record
            .agents
            .iter()
            .find(|a| a.id.slot() == offender)
            .map(|a| a.position.0)?;
        if !ds_match::rules::offside_position(
            man,
            record.ball.position.0,
            &opponents,
            goal_x,
            length,
        ) {
            continue;
        }
        opponents.sort_unstable_by_key(|x| depth(*x));
        let line = *opponents.get(1)?;
        return Some((line, man, k));
    }
    None
}

/// Every dead ball whistled from the facts. A foul with advantage has no
/// `FreeKick`, so it never stops the match and is never whistled. Partita
/// vera + LUNA: the referee signals what the players did and chose; the
/// view adds no rule and takes no choice away.
fn whistles(
    records: &[ds_match::TickRecord],
    events: &[ds_stats::MatchEvent],
    length: i32,
) -> Vec<Whistle> {
    use ds_core::ids::ActorId;
    use ds_stats::EventType;
    let mut out = Vec::new();
    let mut waiting = false;
    // An offside is decided when the engine flags it, but announced once
    // the attacking action is over: the view waits, the fact does not move.
    let mut announce_after = 0usize;
    for (k, record) in records.iter().enumerate() {
        let born = events.get(record.events.clone()).unwrap_or(&[]);
        let has = |t| born.iter().any(|e| e.event_type == t);
        let free_kick = has(EventType::FreeKick) || has(EventType::Penalty);
        let offside = born.iter().find(|e| {
            e.event_type == EventType::FreeKick
                && e.fields.get("reason").and_then(serde_json::Value::as_str) == Some("offside")
        });
        let offside_slot = offside.and_then(|e| {
            e.fields
                .get("offender")
                .and_then(serde_json::Value::as_u64)
                .and_then(|n| u32::try_from(n).ok())
        });
        let fouled = born
            .iter()
            .find(|e| e.event_type == EventType::Foul)
            .and_then(|e| {
                e.fields
                    .get("against")
                    .and_then(serde_json::Value::as_u64)
                    .and_then(|n| u32::try_from(n).ok())
            });
        let card = born
            .iter()
            .find(|e| e.event_type == EventType::Card)
            .and_then(|e| {
                let ActorId::Player(player) = e.actor? else {
                    return None;
                };
                Some(CardShown {
                    player,
                    red: e.fields.get("color").and_then(serde_json::Value::as_str) == Some("red"),
                })
            });
        let stop = if has(EventType::Goal) {
            Some(StopKind::Goal)
        } else if has(EventType::Corner) {
            Some(StopKind::Corner)
        } else if has(EventType::ThrowIn) {
            Some(StopKind::ThrowIn)
        } else if has(EventType::GoalKick) {
            Some(StopKind::GoalKick)
        } else if offside.is_some() {
            Some(StopKind::Offside)
        } else if has(EventType::Foul) && free_kick {
            Some(StopKind::Foul)
        } else {
            None
        };
        if let Some(kind) = stop {
            let delayed = kind == StopKind::Offside;
            let at = if delayed { k + CEREMONY_TICKS } else { k };
            if delayed {
                announce_after = at;
            }
            let mut whistle = Whistle {
                tick: at,
                call_tick: k,
                stop: Some(kind),
                card,
                offside: offside_slot,
                offside_lines: None,
                fouled,
                restart: false,
            };
            if kind == StopKind::Offside {
                whistle.offside_lines = offside_lines(records, events, &whistle, length);
            }
            out.push(whistle);
            waiting = true;
        } else if waiting && k >= announce_after && record.carrier.is_some() {
            out.push(Whistle {
                tick: k,
                call_tick: k,
                stop,
                card: None,
                offside: None,
                offside_lines: None,
                fouled: None,
                restart: true,
            });
            waiting = false;
        }
    }
    out
}

/// A point along the walk to the referee. The picture is held, so only the
/// view moves the token: the record and the event stay as they were.
fn walked(from: (f32, f32), to: (f32, f32), p: f32) -> (f32, f32) {
    (
        (to.0 - from.0).mul_add(p, from.0),
        (to.1 - from.1).mul_add(p, from.1),
    )
}

/// The captain who speaks for the side that suffered the foul. The home
/// captain is the coach's choice; the opponents' is the same role in the
/// formation, since the engine names no captain (a view-only convention).
fn captain_slot(fouled: Option<u32>, chosen: char) -> u32 {
    let role = u32::from(chosen.to_ascii_uppercase()) - u32::from('A');
    if fouled.is_some_and(|s| s >= 100) {
        100 + role
    } else {
        role
    }
}

/// The referee waits for the offender and the captain before showing the
/// card. Both walks are drawn from the tick's recorded positions; nothing
/// here feeds the simulation.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::too_many_arguments
)]
fn draw_card_ceremony(
    painter: &Painter,
    geom: &Geometry,
    theme: &Theme,
    referee: (f32, f32),
    record: &ds_match::TickRecord,
    stop: &Whistle,
    captain: char,
    progress: f32,
) {
    let Some(card) = stop.card else { return };
    let p = progress.clamp(0.0, 1.0);
    let place = |slot: u32| {
        record
            .agents
            .iter()
            .find(|a| a.id.slot() == slot)
            .map(|a| (a.position.0 as f32, a.position.1 as f32))
    };
    let offender = card.player.slot();
    let captain = captain_slot(stop.fouled, captain);
    for (slot, from) in [(offender, place(offender)), (captain, place(captain))] {
        let Some(from) = from else { continue };
        let colour = if slot < 100 { theme.home } else { theme.away };
        if let (Some(a), Some(b)) = (
            geom.project((from.0, from.1, 0.0)),
            geom.project((referee.0, referee.1, 0.0)),
        ) {
            let at = Pos2::new(a.x, a.y);
            let to = Pos2::new(b.x, b.y);
            let point = walked(from, referee, p);
            let Some(on) = geom.project((point.0, point.1, 0.0)) else {
                continue;
            };
            painter.line_segment([at, to], Stroke::new(1.0_f32, colour.gamma_multiply(0.4)));
            painter.circle_filled(Pos2::new(on.x, on.y), 8.0, colour);
        }
    }
    if p >= 1.0
        && let Some(r) = geom.project((referee.0, referee.1, 0.0))
    {
        let colour = if card.red {
            theme.card_red
        } else {
            theme.card_yellow
        };
        painter.rect_filled(
            egui::Rect::from_center_size(Pos2::new(r.x, r.y - 22.0), egui::vec2(11.0, 15.0)),
            2.0,
            colour,
        );
    }
}

/// What moves the crowd (DESIGN.md: only from a fact of the match): a goal
/// lifts the fans of the side that scored for six seconds, a shot those of
/// the side that shot for a second. The side of a goal is read from the
/// score, so that an own goal is the right side's.
fn cheers(
    records: &[ds_match::TickRecord],
    events: &[ds_stats::MatchEvent],
) -> Vec<(usize, bool, f32)> {
    let mut out = Vec::new();
    for (k, pair) in records.windows(2).enumerate() {
        let (a, b) = (&pair[0], &pair[1]);
        // The crowd waits for the VAR check with the players: the goal is
        // celebrated only once the view has confirmed it.
        if b.score.home > a.score.home {
            out.push((k + 1 + CEREMONY_TICKS, true, 1.0));
        }
        if b.score.away > a.score.away {
            out.push((k + 1 + CEREMONY_TICKS, false, 1.0));
        }
        for e in events.get(b.events.clone()).unwrap_or(&[]) {
            if e.event_type == ds_stats::EventType::Shot
                && let Some(ds_core::ids::ActorId::Player(p)) = e.actor
            {
                out.push((k + 1, p.slot() < 100, 0.4));
            }
        }
    }
    out
}

/// How much each side's fans are up at the instant `now` (ticks), `[0, 1]`:
/// the strongest fact still going, fading out.
fn crowd_mood(cheers: &[(usize, bool, f32)], now: f32) -> (f32, f32) {
    let mut mood = (0.0_f32, 0.0_f32);
    for &(tick, home, strength) in cheers {
        #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
        let lasts = if strength >= 1.0 {
            // DD-37: the fans' roar lasts the goal's party (the measured
            // pause), not six seconds hidden by the default x16.
            (ds_match::balance::StoppageTime::load().goal_to_kick_off_s * TICKS_PER_SECOND) as f32
        } else {
            10.0
        };
        #[allow(clippy::cast_precision_loss)]
        let since = now - tick as f32;
        if (0.0..lasts).contains(&since) {
            let v = strength * (1.0 - since / lasts);
            if home {
                mood.0 = mood.0.max(v);
            } else {
                mood.1 = mood.1.max(v);
            }
        }
    }
    mood
}

/// An ellipse round `c`, `rx` across and `ry` up, turned by `angle`
/// radians, as a polygon.
#[allow(
    clippy::disallowed_methods,
    reason = "drawing only: the view never feeds the simulation"
)]
#[allow(clippy::many_single_char_names, reason = "drawing geometry")]
fn ellipse(c: Pos2, rx: f32, ry: f32, angle: f32) -> Vec<Pos2> {
    let (sa, ca) = angle.sin_cos();
    (0..32)
        .map(|k| {
            #[allow(clippy::cast_precision_loss)]
            let a = k as f32 * std::f32::consts::TAU / 32.0;
            let (s, co) = a.sin_cos();
            let (x, y) = (rx * co, ry * s);
            Pos2::new(x.mul_add(ca, -(y * sa)) + c.x, x.mul_add(sa, y * ca) + c.y)
        })
        .collect()
}

/// How light a colour looks, `[0, 1]`.
fn luminance(c: egui::Color32) -> f32 {
    (0.2126f32.mul_add(
        f32::from(c.r()),
        0.7152f32.mul_add(f32::from(c.g()), 0.0722 * f32::from(c.b())),
    )) / 255.0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fresh window on seed 42. The nine tests that only look at the
    /// pristine match share one played game and clone it (a memcpy, not a
    /// simulation); only the five tests that change the match by an order
    /// pay for a second run. Before, each of the eight calls replayed it.
    fn app() -> App {
        static PRISTINE: std::sync::OnceLock<App> = std::sync::OnceLock::new();
        PRISTINE
            .get_or_init(|| App::new(Theme::all().expect("themes")))
            .clone()
    }

    #[test]
    fn the_referee_keeps_his_beak_on_the_ball() {
        let ground = egui::pos2(0.0, 0.0);
        let facing = egui::pos2(10.0, 0.0);
        let ball = egui::pos2(0.0, 10.0);
        assert_eq!(
            App::beak_dir(ground, facing, Some(ball), true),
            egui::vec2(0.0, 1.0)
        );
        assert_eq!(
            App::beak_dir(ground, facing, Some(ball), false),
            egui::vec2(1.0, 0.0)
        );
    }

    /// The 2.5D shows what the engine already knows: the same pass with the
    /// `through` qualifier gets its own pose (ADR-085, SIM 168). Nothing in
    /// the match is replayed differently — only the view reads one more
    /// field of the fact.
    #[test]
    fn the_through_ball_is_drawn_differently_from_the_same_pass() {
        use ds_core::ids::{ActorId, EventId, MatchId};
        use ds_core::time::GameInstant;
        use ds_stats::{EventType, Half, MatchEvent};

        let a = app();
        let mut record = a.game.as_ref().expect("a match is played at start").records[0].clone();
        record.events = 0..1;
        let id = PlayerId::new(0, 0);
        let mut pass = MatchEvent {
            event_id: EventId::new(1, 0),
            match_id: MatchId::new(1, 0),
            actor: Some(ActorId::Player(id)),
            half: Half::First,
            timestamp_minute: 1,
            timestamp_second: 0,
            minutes_played_cumulative: 1,
            event_type: EventType::Pass,
            fields: serde_json::json!({ "complete": true }),
            created_at: GameInstant::from_raw(10),
        };
        let plain = gestures(std::slice::from_ref(&record), std::slice::from_ref(&pass));
        assert!(
            plain
                .get(&id)
                .is_some_and(|gs| gs.iter().any(|(_, g)| *g == scene::Gesture::Kick)),
            "an ordinary pass is a kick"
        );
        pass.fields = serde_json::json!({ "complete": true, "through": true });
        let through = gestures(std::slice::from_ref(&record), std::slice::from_ref(&pass));
        assert!(
            through
                .get(&id)
                .is_some_and(|gs| gs.iter().any(|(_, g)| *g == scene::Gesture::Through)),
            "the same pass, qualified as through, is drawn as a through ball"
        );
    }

    /// DD-37 (LUNA): the whole side celebrates the goal, read from the score
    /// so an own goal goes to the side that benefits; the party lasts the
    /// measured pause, not a 40-tick blink the default x16 hides.
    #[test]
    fn the_side_that_scores_celebrates_and_the_party_is_seen() {
        let mut a = app();
        let pause = ds_match::balance::StoppageTime::load().goal_to_kick_off_s * TICKS_PER_SECOND;
        assert!(pause > 40.0, "the pause is longer than the old blink");

        let game = a.game.as_ref().expect("game");
        let (tick, home, _) = game
            .cheers
            .iter()
            .find(|c| c.2 >= 1.0)
            .expect("a goal in the golden");
        let record = &game.records[*tick];
        let men: Vec<_> = record
            .agents
            .iter()
            .filter(|m| (m.id.slot() < 100) == *home)
            .collect();
        assert!(men.len() > 1, "a side has more than the scorer");
        for man in men {
            assert!(
                game.gestures.get(&man.id).is_some_and(|gs| {
                    gs.iter()
                        .any(|&(k, g)| k == *tick && g == scene::Gesture::Celebrate)
                }),
                "every man of the side celebrates: {:?}",
                man.id
            );
        }
        // Halfway into the pause the scorer is still seen celebrating at x16.
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let middle = (pause / 2.0) as usize;
        a.tick = *tick + middle;
        assert!(
            a.gesture_now(
                game.gestures
                    .iter()
                    .find_map(|(id, gs)| gs
                        .iter()
                        .any(|&(k, g)| k == *tick && g == scene::Gesture::Celebrate)
                        .then_some(*id))
                    .expect("a scorer")
            )
            .is_some_and(|(g, _)| g == scene::Gesture::Celebrate),
            "the party is visible halfway into the pause"
        );
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

    /// A substitution made while watching (decision T): the view stays at
    /// its tick, the match up to it is the same, and the man comes on
    /// after it.
    #[test]
    fn a_change_from_the_bench_keeps_the_match_up_to_now() {
        let mut a = app();
        a.tick = 6_000;
        let before: Vec<_> = a.game.as_ref().expect("game").records[..=6_000].to_vec();
        let m = ds_fixtures::match_fixture(a.seed).expect("fixture");
        let (off, on) = (m.home.players[9].id, m.home.bench[0].id);
        a.order(ds_model::Order::Substitute {
            outgoing: off,
            incoming: on,
        });
        assert_eq!(a.tick, 6_000, "the view stays where it was");
        let game = a.game.as_ref().expect("game");
        assert_eq!(
            game.records[..=6_000],
            before[..],
            "the same match up to now"
        );
        let came = game
            .records
            .iter()
            .position(|r| r.agents.iter().any(|x| x.id == on))
            .expect("he comes on");
        assert!(came > 6_000);
    }

    /// A responsibility chosen on a player's post-it (ADR-059 step 5) is
    /// an order of the board at the tick on the screen: the match up to now
    /// is the same, and the board the coach has now carries it.
    #[test]
    fn a_responsibility_from_the_post_it_keeps_the_match_up_to_now() {
        let mut a = app();
        a.tick = 9_000;
        let before: Vec<_> = a.game.as_ref().expect("game").records[..=9_000].to_vec();
        let mut board = a.board_now();
        board.with_ball_tasks[9] = ds_model::Task::Run;
        a.order(ds_model::Order::Board(board));
        let game = a.game.as_ref().expect("game");
        assert_eq!(game.records[..=9_000], before[..]);
        assert_eq!(a.board_now().with_ball_tasks[9], ds_model::Task::Run);
    }

    /// A quick instruction from the post-it is an order at the tick on the
    /// screen: the match up to now is the same, and the order is told.
    #[test]
    fn a_quick_instruction_keeps_the_match_up_to_now() {
        let mut a = app();
        a.tick = 9_000;
        let before: Vec<_> = a.game.as_ref().expect("game").records[..=9_000].to_vec();
        a.order(ds_model::Order::Quick {
            quick: ds_model::Quick::MorePressing,
            on: true,
        });
        let game = a.game.as_ref().expect("game");
        assert_eq!(game.records[..=9_000], before[..]);
        assert_eq!(a.choices.live.len(), 1);
    }

    /// The orders during the match reach the engine (the maintainer's pure
    /// error 2d): a block and a width given from the button at the 15th
    /// minute leave the match the same up to then, and change it after.
    #[test]
    fn an_order_from_the_button_reaches_the_engine() {
        let mut a = app();
        a.tick = 9_000;
        let before: Vec<_> = a.game.as_ref().expect("game").records.clone();
        a.order(ds_model::Order::Intention(ds_model::Intention {
            block: BlockHeight::High,
            width: Width::Wide,
        }));
        let game = a.game.as_ref().expect("game");
        assert_eq!(game.records[..=9_000], before[..=9_000]);
        assert!(
            game.records[9_001..]
                .iter()
                .zip(&before[9_001..])
                .any(|(x, y)| x.agents != y.agents),
            "the side plays differently after the order"
        );
    }

    /// The crowd moves only from facts: every goal lifts the fans of the
    /// side whose score went up, and nobody is up at the kick-off.
    #[test]
    fn the_fans_rise_only_at_the_facts() {
        let a = app();
        let game = a.game.as_ref().expect("game");
        let goals: Vec<_> = game.cheers.iter().filter(|c| c.2 >= 1.0).collect();
        let last = game.records.last().expect("records").score;
        assert_eq!(goals.len(), usize::from(last.home + last.away));
        assert_eq!(crowd_mood(&game.cheers, 0.0), (0.0, 0.0));
        if let Some(&&(tick, home, _)) = goals.first() {
            #[allow(clippy::cast_precision_loss)]
            let (h, w) = crowd_mood(&game.cheers, tick as f32 + 1.0);
            assert!(if home { h > 0.9 } else { w > 0.9 });
        }
    }

    /// Law 6, from the facts: every dead ball is whistled, then the ball
    /// back at a man's foot lets play restart. A foul with advantage has
    /// no free kick, so it is not a stop and is not whistled.
    #[test]
    fn the_whistles_follow_every_stop_and_the_restart() {
        let a = app();
        let game = a.game.as_ref().expect("game");
        let stops: Vec<_> = game.whistles.iter().filter(|w| !w.restart).collect();
        assert!(!stops.is_empty(), "seed 42 has called fouls");
        let mut kinds = BTreeMap::new();
        for stop in stops {
            *kinds.entry(stop.stop).or_insert(0usize) += 1;
            let text = &game.ticks[stop.call_tick].entries;
            let told = match stop.stop {
                Some(StopKind::Foul) => text.iter().any(|e| e.text.contains("Foul by")),
                Some(StopKind::Offside) => text.iter().any(|e| e.text.contains("Offside")),
                Some(StopKind::Goal) => text.iter().any(|e| e.text.contains("GOAL")),
                Some(StopKind::ThrowIn) => text.iter().any(|e| e.text.contains("Throw-in")),
                Some(StopKind::GoalKick) => text.iter().any(|e| e.text.contains("Goal kick")),
                Some(StopKind::Corner) => text.iter().any(|e| e.text.contains("Corner")),
                None => false,
            };
            assert!(told, "the stop at tick {} is {:#?}", stop.tick, text);
            let after = game
                .whistles
                .iter()
                .find(|w| w.restart && w.tick > stop.tick)
                .expect("the restart whistle");
            assert!(
                game.records[after.tick].carrier.is_some(),
                "the restart follows the ball at a man's foot"
            );
        }
        assert!(
            kinds.contains_key(&Some(StopKind::Foul)),
            "a foul stops play"
        );
    }

    /// The whistle is seen as well as heard: the picture holds, then the
    /// dead ball runs at real speed so the taker's walk is not swallowed.
    #[test]
    fn the_stop_whistle_holds_the_picture_and_the_taker_walks_to_the_ball() {
        let mut a = app();
        a.audio = None;
        let game = a.game.as_ref().expect("game");
        let (stop, restart) = game
            .whistles
            .iter()
            .find_map(|w| {
                if w.restart {
                    return None;
                }
                let after = game
                    .whistles
                    .iter()
                    .find(|j| j.restart && j.tick > w.tick)?;
                let taker = game.records[after.tick].carrier?;
                let place = |k: usize| {
                    game.records[k]
                        .agents
                        .iter()
                        .find(|x| x.id == taker)
                        .map(|x| x.position)
                };
                let ball = game.records[w.tick].ball.position;
                let away = |p: (i32, i32)| (p.0 - ball.0).abs() + (p.1 - ball.1).abs();
                let (far, near) = (away(place(w.tick)?), away(place(after.tick)?));
                // More than two metres to walk, and inside a body's reach
                // when the referee lets him kick (SIM 150, ADR-088).
                (far > 200 && near < 90).then_some((w.tick, after.tick))
            })
            .expect("a foul whose taker has to walk to the ball");

        a.tick = stop - 1;
        a.advance(1.0 / (TICKS_PER_SECOND * a.speed));
        assert_eq!(a.tick, stop, "the whistle sounds at the foul");
        assert!(a.whistle_pause > 0.0, "the picture holds at the whistle");
        a.advance(WHISTLE_PAUSE_S + 0.01);
        a.advance(0.1);
        assert!(a.tick - stop <= 1, "at real speed the walk is seen");
        assert!(a.tick < restart, "the restart comes after the walk");
    }

    /// The lines of an offside are read from the pass that made the run,
    /// not from the later whistle; the flagged man is beyond the last line.
    #[test]
    fn the_offside_lines_come_from_the_pass() {
        let game = app().game.expect("game");
        let flagged: Vec<_> = game
            .whistles
            .iter()
            .filter(|w| w.stop == Some(StopKind::Offside))
            .collect();
        assert!(!flagged.is_empty(), "seed 42 has offsides");
        let mut from_pass = false;
        for off in flagged {
            let (line, man, at) = off.offside_lines.expect("the lines");
            from_pass |= at < off.call_tick;
            let goal = if off.offside.expect("the offender") < 100 {
                game.pitch_cm.0
            } else {
                0
            };
            let depth = |x: i32| (goal - x).abs();
            assert!(
                depth(man) < depth(line),
                "tick {at}: the man at {man} is beyond the line at {line}"
            );
        }
        assert!(from_pass, "at least one line is read from the pass");
        // The decision is announced after the action, not at the fact.
        assert!(
            game.whistles
                .iter()
                .any(|w| w.stop == Some(StopKind::Offside) && w.tick > w.call_tick),
            "the offside whistle follows the action"
        );
    }

    /// The two screens of the check: a line while the picture holds, the
    /// sentence after. The card's screen owns the foul's.
    #[test]
    fn the_var_check_has_a_question_and_an_answer() {
        let whistle = |card| Whistle {
            tick: 0,
            call_tick: 0,
            stop: Some(StopKind::Goal),
            card,
            offside: None,
            offside_lines: None,
            fouled: None,
            restart: false,
        };
        assert_eq!(banner(&whistle(None), true), ("VAR", "Checking the goal"));
        assert_eq!(banner(&whistle(None), false), ("VAR", "Goal confirmed"));
        let yellow = Some(CardShown {
            player: PlayerId::new(3, 0),
            red: false,
        });
        let red = Some(CardShown {
            player: PlayerId::new(3, 0),
            red: true,
        });
        assert_eq!(banner(&whistle(yellow), true), ("VAR", "Checking the card"));
        assert_eq!(
            banner(&whistle(yellow), false),
            ("VAR", "Yellow card confirmed")
        );
        assert_eq!(banner(&whistle(red), false), ("VAR", "Red card confirmed"));
    }

    /// The man walks from where he stood to the referee; the home captain
    /// is the coach's letter, the opponents' the same role in their shape.
    #[test]
    fn the_captain_who_speaks_is_the_coach_choice() {
        assert_eq!(captain_slot(Some(4), 'D'), 3);
        assert_eq!(captain_slot(Some(104), 'F'), 105);
        assert_eq!(walked((0.0, 0.0), (100.0, 200.0), 0.0), (0.0, 0.0));
        assert_eq!(walked((0.0, 0.0), (100.0, 200.0), 1.0), (100.0, 200.0));
        assert_eq!(walked((0.0, 0.0), (100.0, 200.0), 0.5), (50.0, 100.0));
    }

    /// The players and the crowd wait for the sentence: no celebration at
    /// the goal tick, only after the check has ended.
    #[test]
    fn the_celebration_waits_for_the_var_sentence() {
        let game = app().game.expect("game");
        let goal = game
            .records
            .windows(2)
            .position(|p| p[1].score.home > p[0].score.home || p[1].score.away > p[0].score.away)
            .expect("seed 42 has a goal")
            + 1;
        assert!(
            game.cheers
                .iter()
                .any(|(k, _, _)| *k == goal + CEREMONY_TICKS),
            "the crowd rises after the check"
        );
        assert!(
            !game.cheers.iter().any(|(k, _, _)| *k == goal),
            "the crowd is silent during the check"
        );
        assert!(
            game.gestures
                .values()
                .flatten()
                .any(|(k, g)| *k == goal + CEREMONY_TICKS && *g == scene::Gesture::Celebrate),
            "the players celebrate after the check"
        );
    }
}
