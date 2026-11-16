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
    /// Each player's gestures, from the facts of the match: the tick and
    /// the gesture (DESIGN.md: an animation only from a fact).
    gestures: BTreeMap<PlayerId, Vec<(usize, scene::Gesture)>>,
    names: BTreeMap<PlayerId, String>,
    why: String,
    pitch_cm: (i32, i32),
    /// The home side at the kick-off: the eleven and the bench (the bench
    /// post-it).
    home: (Vec<PlayerId>, Vec<PlayerId>),
    /// The record's header (ADR-065): the physics the match was played
    /// with, for the ball's flight between two ticks.
    header: ds_match::RecordHeader,
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
    /// The camera preset.
    pub camera: Option<Camera>,
    /// The slow motion.
    pub slow: bool,
    /// Record the window, frame by frame at 60 frames a second of
    /// playback, into this folder, this many frames, then close
    /// (`--record DIR --frames N`: the README's animation).
    pub record: Option<(String, usize)>,
}

/// The camera presets (DESIGN.md, "La visuale del 2.5D").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Camera {
    /// The whole pitch, tilted: the team's shape, the lines, the distances.
    Whole,
    /// About 38 m around the ball, following it with a damped movement:
    /// the gestures, the duels, the shots.
    Close,
    /// From above, 0°: the board as it always was.
    Above,
}

/// How much pitch the close camera shows across, cm (DESIGN.md).
const CLOSE_SPAN_CM: f32 = 3_800.0;
/// How quickly the close camera catches up with the ball, a share a
/// second (a drawing choice: the damped movement).
const FOLLOW_RATE: f32 = 3.0;
/// The slow motion, a share of the chosen speed (DESIGN.md: about a third).
const SLOW: f64 = 1.0 / 3.0;
/// A stride's length at a run, cm: the stride's phase moves on by the
/// ground covered (a drawing size).
const STRIDE_CM: f32 = 180.0;

/// Each figure's stride: its phase, radians, and where he stood at the
/// last frame, cm.
type Strides = BTreeMap<PlayerId, (f32, (f32, f32))>;

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
    /// The camera's tilt from the vertical, degrees.
    tilt: f32,
    /// The camera preset.
    camera: Camera,
    /// The slow motion is on.
    slow: bool,
    /// Where the close camera looks, cm (it follows the ball).
    focus: std::cell::Cell<(f32, f32)>,
    /// Each figure's stride: its phase and where he was at the last frame.
    strides: std::cell::RefCell<Strides>,
    /// The time of the last frame, seconds (for the damped camera).
    last_dt: std::cell::Cell<f32>,
    /// The recording: the folder, the frames wanted, the frames saved.
    record: Option<(String, usize, usize)>,
    /// The bench post-it: who comes off and who comes on, as chosen.
    pick: (Option<PlayerId>, Option<PlayerId>),
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
            camera: Camera::Whole,
            slow: false,
            focus: std::cell::Cell::new((5_250.0, 3_400.0)),
            strides: std::cell::RefCell::new(BTreeMap::new()),
            last_dt: std::cell::Cell::new(0.0),
            record: None,
            pick: (None, None),
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
        if let Some(c) = start.camera {
            self.camera = c;
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
                self.game = Some(Game {
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
                self.playing = true;
            }
            Err(e) => self.error = Some(e),
        }
    }

    /// An order given while watching (decision T): an input at the tick
    /// on the screen. The match is played again from the same seed with
    /// it; up to that tick it is the same match, so the view stays where
    /// it was and goes on from there.
    fn order(&mut self, order: ds_model::Order) {
        let Some(game) = &self.game else { return };
        let Some(now) = game.records.get(self.tick) else {
            return;
        };
        self.choices.live.push(ds_model::Instruction {
            tick: now.tick,
            order,
        });
        let (tick, carry, playing) = (self.tick, self.carry, self.playing);
        self.play_match();
        self.tick = tick;
        self.carry = carry;
        self.playing = playing;
    }

    /// The bench as a post-it over the match (DESIGN.md, decision T): who
    /// is on, who is on the bench, the orders given and whether they have
    /// been made.
    fn bench_postit(&mut self, ctx: &egui::Context) {
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
                ds_model::Order::Intention(_) => None,
            })
            .collect();
        let paper = egui::Color32::from_rgb(0xf6, 0xe0, 0x7a);
        let ink = egui::Color32::from_rgb(0x2a, 0x26, 0x1c);
        let mut give = None;
        egui::Area::new(egui::Id::new("bench-postit"))
            .anchor(egui::Align2::LEFT_TOP, egui::vec2(12.0, 52.0))
            .show(ctx, |ui| {
                egui::Frame::NONE
                    .fill(paper)
                    .inner_margin(10.0)
                    .shadow(egui::epaint::Shadow {
                        offset: [2, 3],
                        blur: 6,
                        spread: 0,
                        color: egui::Color32::from_black_alpha(90),
                    })
                    .show(ui, |ui| {
                        ui.visuals_mut().override_text_color = Some(ink);
                        ui.set_max_width(220.0);
                        ui.strong("Bench");
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
                        let ready = out.is_some_and(|o| on.contains(&o))
                            && inn.is_some_and(|i| bench.contains(&i));
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
                    });
            });
        if let Some((outgoing, incoming)) = give {
            self.order(ds_model::Order::Substitute { outgoing, incoming });
        }
    }

    /// Moves the playback on by the time since the last frame.
    fn advance(&mut self, dt: f64) {
        let Some(game) = &self.game else { return };
        if !self.playing || game.ticks.is_empty() {
            return;
        }
        let speed = if self.slow {
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
                // A new match: the orders given watching the old one go.
                self.choices.live.clear();
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
            ui.toggle_value(&mut self.slow, "Slow");
            ui.separator();
            ui.label("Camera");
            ui.selectable_value(&mut self.camera, Camera::Whole, "Whole pitch");
            ui.selectable_value(&mut self.camera, Camera::Close, "Close");
            ui.selectable_value(&mut self.camera, Camera::Above, "Above");
            ui.add(
                egui::Slider::new(&mut self.tilt, 0.0..=75.0)
                    .suffix("°")
                    .text("tilt"),
            );
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
        // One tick behind the playhead, so that the drawing always has the
        // tick before and the tick after (the interpolation layer).
        #[allow(clippy::cast_precision_loss)]
        let instant = (self.tick as f64 + self.carry - 1.0).max(0.0);
        let moment = Moment::at(&game.records, instant, &game.header.physics);
        let (origin, size) = ((r.min.x, r.min.y), (r.width(), r.height()));
        let geom = match self.camera {
            Camera::Whole => Geometry::fit(origin, size, 16.0, game.pitch_cm, self.tilt),
            Camera::Above => Geometry::fit(origin, size, 16.0, game.pitch_cm, 0.0),
            Camera::Close => {
                // The damped movement: the camera catches up with the ball.
                let (fx, fy) = self.focus.get();
                let k = (self.last_dt.get() * FOLLOW_RATE).clamp(0.0, 1.0);
                let f = (
                    (moment.ball.0 - fx).mul_add(k, fx),
                    (moment.ball.1 - fy).mul_add(k, fy),
                );
                self.focus.set(f);
                Geometry::around(origin, size, game.pitch_cm, self.tilt, (f, CLOSE_SPAN_CM))
            }
        };
        self.draw_markings(&painter, &geom);
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
                    let (head, head_r) = self.draw_figure(&painter, &geom, f, fill);
                    if matches!(item, Item::Figure(_)) {
                        // His letter above him, when he is big enough to
                        // carry it (the close camera).
                        if head_r >= 4.0 {
                            painter.text(
                                head - egui::vec2(0.0, head_r * 2.6),
                                Align2::CENTER_CENTER,
                                f.letter,
                                FontId::monospace(head_r * 2.0),
                                fill,
                            );
                        }
                        if response
                            .hover_pos()
                            .is_some_and(|p| p.distance(head) <= (head_r * 3.0).max(6.0))
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

    /// One articulated figure (DESIGN.md, "Gli omini articolati"): his
    /// shadow, the ring of the man on the ball, the limbs with his stride,
    /// the head. Returns where his head is and its radius.
    fn draw_figure(
        &self,
        painter: &Painter,
        geom: &Geometry,
        f: &scene::Figure,
        fill: egui::Color32,
    ) -> (Pos2, f32) {
        let t = &self.theme;
        let pos = |s: &scene::Screen| Pos2::new(s.x, s.y);
        let feet = pos(&f.feet);
        // His shadow on the grass, then the figure.
        painter.circle_filled(feet, f.size * 2.2, t.ball_shadow);
        if f.carrier {
            painter.circle_stroke(
                feet,
                f.size * 3.0,
                Stroke::new((f.size * 0.4).max(1.2), t.carrier_ring),
            );
        }
        // The stride moves on with the ground he covers.
        let phase = {
            let mut strides = self.strides.borrow_mut();
            let e = strides.entry(f.id).or_insert((0.0, f.at));
            let moved = ds_core::dmath::hypot(f64::from(f.at.0 - e.1.0), f64::from(f.at.1 - e.1.1));
            #[allow(clippy::cast_possible_truncation)]
            let moved = moved as f32;
            if moved < 500.0 {
                e.0 = (moved / STRIDE_CM)
                    .mul_add(std::f32::consts::TAU, e.0)
                    .rem_euclid(std::f32::consts::TAU);
            }
            e.1 = f.at;
            e.0
        };
        let (parts, head_at, head_r) = scene::limbs(
            geom,
            f.at,
            f.heading,
            scene::Motion {
                phase,
                speed: f.speed,
                gesture: self.gesture_now(f.id),
                fatigue: f.fatigue,
            },
        );
        let sock = fill.gamma_multiply(0.6);
        for l in &parts {
            let c = match l.part {
                scene::Part::Shirt => fill,
                scene::Part::Sock => sock,
            };
            painter.line_segment([pos(&l.a), pos(&l.b)], Stroke::new(l.width, c));
        }
        let head = pos(&head_at);
        painter.circle_filled(head, head_r, t.skin);
        (head, head_r)
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
                scene::Gesture::Jump => (3.0, 4.0),
                scene::Gesture::Lunge => (2.0, 3.0),
                scene::Gesture::Dive(_) | scene::Gesture::Slide => (2.0, 8.0),
                scene::Gesture::Fall => (0.0, 20.0),
                scene::Gesture::Throw => (8.0, 2.0),
                scene::Gesture::Celebrate => (0.0, 40.0),
                scene::Gesture::Card => (0.0, 25.0),
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
        }
        if *saved >= *wanted {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
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
                    out.entry(scene::REFEREE)
                        .or_default()
                        .push((k, scene::Gesture::Card));
                    continue;
                }
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
        let mut dt = f64::from(ctx.input(|i| i.stable_dt).min(0.1));
        if self.record.is_some() {
            // Recording: each frame is 1/60 s of playback, however long it
            // takes to draw.
            dt = 1.0 / 60.0;
            self.save_screenshots(ctx);
        }
        #[allow(clippy::cast_possible_truncation)]
        self.last_dt.set(dt as f32);
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
        if self.record.is_none() {
            self.bench_postit(ctx);
        }
        if self.record.is_some() {
            ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::UserData::default()));
            ctx.request_repaint();
        } else if self.playing {
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
}
