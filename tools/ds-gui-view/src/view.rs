//! A match ready to watch: the record, the narration, the gestures and the
//! camera (decision W, one match many views). Drawing only — the view never
//! feeds a choice back to the engine.

use std::collections::BTreeMap;

use ds_demo::friendly::{self, Choices};
use ds_demo::narration::{NarratedTick, NarrativeEntry, narrate_ticks};
use ds_match::{MatchConfig, RecordHeader, RecordPhysics, TickRecord};
use ds_model::Match;
use eframe::egui::{Align2, Color32, FontId, Painter, Pos2, Rect, Sense, Shape, Stroke, Ui, Vec2};

use crate::gestures::gestures;
use crate::interp::Moment;
use crate::scene::{self, Geometry, Item, NEAREST_SPAN_CM};

/// Speeds offered to the viewer, times real time.
const SPEEDS: [f64; 4] = [1.0, 4.0, 16.0, 64.0];
/// How fast the camera catches up with the ball (a viewing choice).
const FOLLOW_RATE: f32 = 8.0;
/// Ticks the drawing stays behind the playhead, so it has a tick on either
/// side of every instant (the interpolation layer).
const DELAY: f64 = 1.0;

/// The colours the host GUI lends to the view. No palette is hardcoded here:
/// the desktop and Android themes stay the hosts' responsibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palette {
    /// The grass outside the lines.
    pub grass: Color32,
    /// Every other band of grass.
    pub stripe: Color32,
    /// The lines and the goals.
    pub chalk: Color32,
    /// The home shirt.
    pub home: Color32,
    /// The number on a home shirt.
    pub home_ink: Color32,
    /// The away shirt.
    pub away: Color32,
    /// The number on an away shirt.
    pub away_ink: Color32,
    /// The referee's shirt.
    pub referee: Color32,
    /// The ball.
    pub ball: Color32,
    /// The clock and the score.
    pub time: Color32,
}

/// One recorded match, narrated, with its playback and camera.
pub struct MatchView {
    records: Vec<TickRecord>,
    narrated: Vec<NarratedTick>,
    gestures: BTreeMap<ds_core::ids::PlayerId, Vec<(usize, scene::Gesture)>>,
    physics: RecordPhysics,
    pitch_cm: (i32, i32),
    tick: f64,
    playing: bool,
    speed: f64,
    zoom: f32,
    tilt: f32,
    focus: (f32, f32),
    follow: bool,
}

impl MatchView {
    /// Plays and records a friendly from `seed`, with the default choices.
    ///
    /// The engine's result is the same as the CLI's; recording only observes.
    ///
    /// # Errors
    /// Returns the fixture or choice error if the seed cannot be prepared.
    pub fn new(seed: u64) -> Result<Self, String> {
        let match_ = ds_fixtures::match_fixture(seed)
            .and_then(|m| friendly::apply(m, &Choices::default()))?;
        Ok(Self::from_match(&match_, seed))
    }

    /// Builds the view of a match already chosen by a host GUI.
    #[must_use]
    pub fn from_match(match_: &Match, seed: u64) -> Self {
        let config = MatchConfig::default();
        let (result, records) = ds_match::simulate_from_seed_recorded(match_, seed, config);
        let narrated = narrate_ticks(match_, &result.events, records.clone());
        let gestures = gestures(&records, result.events.events());
        let physics = RecordHeader::current(
            config.ticks_per_second,
            (match_.pitch.length_m, match_.pitch.width_m),
        )
        .physics;
        let pitch_cm = (
            i32::from(match_.pitch.length_m) * 100,
            i32::from(match_.pitch.width_m) * 100,
        );
        #[allow(clippy::cast_precision_loss)]
        let focus = (pitch_cm.0 as f32 / 2.0, pitch_cm.1 as f32 / 2.0);
        Self {
            records,
            narrated,
            gestures,
            physics,
            pitch_cm,
            tick: 0.0,
            playing: true,
            speed: 16.0,
            zoom: 0.0,
            tilt: 0.0,
            focus,
            follow: true,
        }
    }

    /// The whole tick under the playhead.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn tick(&self) -> usize {
        self.tick as usize
    }

    /// How many ticks the record has.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Whether there is no record to draw.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Whether the playhead is running.
    #[must_use]
    pub fn is_playing(&self) -> bool {
        self.playing
    }

    /// Starts or stops the playhead.
    pub fn toggle(&mut self) {
        self.playing = !self.playing;
    }

    /// Goes back to the kick-off and plays again.
    pub fn rewind(&mut self) {
        self.tick = 0.0;
        self.playing = true;
    }

    /// The current speed, times real time.
    #[must_use]
    pub fn speed(&self) -> f64 {
        self.speed
    }

    /// Moves to the next offered speed, wrapping round.
    pub fn cycle_speed(&mut self) {
        let i = SPEEDS.iter().position(|&s| s == self.speed).unwrap_or(0);
        self.speed = SPEEDS[(i + 1) % SPEEDS.len()];
    }

    /// The camera tilt, in degrees.
    #[must_use]
    pub fn tilt(&self) -> f32 {
        self.tilt
    }

    /// Tilts the camera, 0° from above to 75° from the near touchline.
    pub fn set_tilt(&mut self, degrees: f32) {
        self.tilt = degrees.clamp(0.0, 75.0);
    }

    /// Whether the camera follows the ball.
    #[must_use]
    pub fn is_following(&self) -> bool {
        self.follow
    }

    /// Gives the camera back to the ball.
    pub fn follow_ball(&mut self) {
        self.follow = true;
    }

    /// The broadcast clock at the playhead, `mm:ss`.
    #[must_use]
    pub fn clock(&self) -> String {
        let i = self.tick().min(self.records.len().saturating_sub(1));
        let (m, s) = self.records[i].clock();
        format!("{m:02}:{s:02}")
    }

    /// The score at the playhead.
    #[must_use]
    pub fn score(&self) -> (u8, u8) {
        let i = self.tick().min(self.records.len().saturating_sub(1));
        (self.records[i].score.home, self.records[i].score.away)
    }

    /// The last `limit` narrated facts that have happened, oldest first.
    #[must_use]
    pub fn commentary(&self, limit: usize) -> Vec<String> {
        let upto = self.tick().min(self.narrated.len().saturating_sub(1));
        self.narrated[..=upto]
            .iter()
            .flat_map(|t| &t.entries)
            .rev()
            .take(limit)
            .map(|e: &NarrativeEntry| format!("{}' {}", e.minute, e.text))
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }

    /// Moves the playhead on by `dt` real seconds.
    #[allow(clippy::cast_precision_loss)]
    pub fn advance(&mut self, dt: f32) {
        if !self.playing || self.records.len() < 2 {
            return;
        }
        let last = (self.records.len() - 1) as f64;
        self.tick = (self.tick
            + f64::from(dt) * f64::from(ds_match::context::TICKS_PER_SECOND) * self.speed)
            .min(last);
        if self.tick >= last {
            self.playing = false;
        }
    }

    /// The gesture `id` makes at the instant drawn, and how far into it.
    fn gesture_now(&self, id: ds_core::ids::PlayerId) -> Option<(scene::Gesture, f32)> {
        #[allow(clippy::cast_precision_loss)]
        let t = (self.tick - DELAY).max(0.0);
        self.gestures.get(&id)?.iter().find_map(|&(tick, g)| {
            let (before, after) = match g {
                scene::Gesture::Kick => (5.0, 2.0),
                scene::Gesture::Through => (6.0, 2.0),
                scene::Gesture::Jump => (3.0, 4.0),
                scene::Gesture::Lunge => (2.0, 3.0),
                scene::Gesture::Dive(_) | scene::Gesture::Slide => (2.0, 8.0),
                scene::Gesture::Fall => (0.0, 20.0),
                scene::Gesture::Throw => (8.0, 2.0),
                scene::Gesture::Celebrate => (0.0, 60.0),
                scene::Gesture::Card(_) => (0.0, 25.0),
            };
            #[allow(clippy::cast_precision_loss)]
            let p = (t - (tick as f64 - before)) / (before + after);
            #[allow(clippy::cast_possible_truncation)]
            (0.0..=1.0).contains(&p).then_some((g, p as f32))
        })
    }

    /// Draws the view on `rect`, taking the host's colours from `palette`.
    ///
    /// The wheel or a pinch zooms, dragging moves the camera and turns
    /// "follow the ball" off, and a double click gives the ball back.
    pub fn paint(&mut self, ui: &mut Ui, rect: Rect, palette: &Palette) {
        if self.is_empty() {
            return;
        }
        let (response, painter) = ui.allocate_painter(rect.size(), Sense::click_and_drag());
        let r = response.rect;
        self.steer(ui, &response);
        #[allow(clippy::cast_precision_loss)]
        let instant = (self.tick - DELAY).max(0.0);
        let live = Moment::at(&self.records, instant, &self.physics);
        if self.follow {
            let k = (ui.input(|i| i.stable_dt) * FOLLOW_RATE).clamp(0.0, 1.0);
            self.focus = (
                (live.ball.0 - self.focus.0).mul_add(k, self.focus.0),
                (live.ball.1 - self.focus.1).mul_add(k, self.focus.1),
            );
        }
        let whole = Geometry::whole_span(self.pitch_cm);
        let span = whole * (NEAREST_SPAN_CM / whole).powf(self.zoom);
        let geom = Geometry::view(
            (r.min.x, r.min.y),
            (r.width(), r.height()),
            self.pitch_cm,
            self.tilt,
            (self.focus, span),
        );
        self.focus = geom.looks_at();
        self.markings(&painter, &geom, palette);
        let frame = scene::frame(&live, &geom);
        let pos = |s: &scene::Screen| Pos2::new(s.x, s.y);
        for (at, radius) in &frame.shadows {
            painter.circle_filled(pos(at), *radius, palette.grass.gamma_multiply(0.7));
        }
        let ball = frame.items.iter().find_map(|item| match item {
            Item::Ball(b) => Some(pos(&b.ground)),
            _ => None,
        });
        for item in &frame.items {
            match item {
                Item::Token(tk) | Item::Referee(tk) => {
                    let referee = matches!(item, Item::Referee(_));
                    let ground = pos(&tk.ground);
                    let radius = geom
                        .len_at(scene::TOKEN_CM, (tk.at.0, tk.at.1, 0.0))
                        .max(scene::MIN_TOKEN_PX);
                    let (lift, shift) = self.gesture_at(tk.id, radius, ground, referee, ball);
                    let at = ground + shift - Vec2::UP * lift;
                    let fill = if referee {
                        palette.referee
                    } else if tk.home {
                        palette.home
                    } else {
                        palette.away
                    };
                    let ink = if tk.home {
                        palette.home_ink
                    } else {
                        palette.away_ink
                    };
                    painter.circle_filled(at, radius, fill);
                    painter.circle_stroke(at, radius, Stroke::new(1.0_f32, ink));
                    painter.text(
                        at,
                        Align2::CENTER_CENTER,
                        tk.number.to_string(),
                        FontId::proportional(radius * 1.1),
                        ink,
                    );
                    if tk.carrier {
                        painter.circle_stroke(at, radius + 2.0, Stroke::new(1.5_f32, palette.ball));
                    }
                }
                Item::Ball(b) => {
                    painter.circle_filled(pos(&b.at), b.radius, palette.ball);
                    painter.circle_stroke(
                        pos(&b.at),
                        b.radius,
                        Stroke::new(1.0_f32, palette.chalk),
                    );
                }
            }
        }
        self.overlay(&painter, r, palette);
    }

    /// The hand on the board (DESIGN.md, "Una partita, molte viste").
    fn steer(&mut self, ui: &Ui, response: &egui::Response) {
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
                let whole = Geometry::whole_span(self.pitch_cm);
                let span = whole * (NEAREST_SPAN_CM / whole).powf(self.zoom);
                let per = span / response.rect.width().max(1.0);
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

    /// How a fact moves the token drawn now: a few ticks either side only.
    fn gesture_at(
        &self,
        id: ds_core::ids::PlayerId,
        radius: f32,
        ground: Pos2,
        referee: bool,
        ball: Option<Pos2>,
    ) -> (f32, Vec2) {
        use std::f32::consts::PI;
        let Some((g, p)) = self.gesture_now(id) else {
            return (0.0, Vec2::ZERO);
        };
        let bump = (PI * p).sin();
        let dir = ball.unwrap_or(ground) - ground;
        let dir = if referee || dir.length() > 0.0 {
            dir.normalized()
        } else {
            Vec2::UP
        };
        match g {
            scene::Gesture::Jump | scene::Gesture::Celebrate => (radius * bump * 1.6, Vec2::ZERO),
            scene::Gesture::Kick | scene::Gesture::Throw | scene::Gesture::Through => {
                (0.0, dir * (radius * 0.5 * bump))
            }
            scene::Gesture::Dive(s) => (0.0, Vec2::new(-dir.y, dir.x) * (s * radius * bump)),
            scene::Gesture::Slide | scene::Gesture::Lunge | scene::Gesture::Fall => {
                (0.0, dir * (radius * 0.7 * p))
            }
            scene::Gesture::Card(_) => (radius * 0.3, Vec2::ZERO),
        }
    }

    /// Grass, bands, lines and goals, projected through the camera.
    fn markings(&self, painter: &Painter, geom: &Geometry, palette: &Palette) {
        let (length, width) = self.pitch_cm;
        let on = |x: i32, y: i32| {
            let (sx, sy) = geom.at((x, y));
            Pos2::new(sx, sy)
        };
        let quad = |x0: i32, y0: i32, x1: i32, y1: i32| {
            vec![on(x0, y0), on(x1, y0), on(x1, y1), on(x0, y1)]
        };
        let margin = 150;
        painter.add(Shape::convex_polygon(
            quad(
                -margin - 200,
                -margin,
                length + margin + 200,
                width + margin,
            ),
            palette.grass,
            Stroke::NONE,
        ));
        for i in (0..12).step_by(2) {
            painter.add(Shape::convex_polygon(
                quad(length * i / 12, 0, length * (i + 1) / 12, width),
                palette.stripe,
                Stroke::NONE,
            ));
        }
        let line = Stroke::new(1.5_f32, palette.chalk);
        painter.add(Shape::closed_line(quad(0, 0, length, width), line));
        painter.line_segment([on(length / 2, 0), on(length / 2, width)], line);
        let circle = (0..48)
            .map(|k| {
                use std::f32::consts::TAU;
                #[allow(clippy::disallowed_methods, reason = "drawing only")]
                let (s, c) = ((k as f32) * TAU / 48.0).sin_cos();
                #[allow(clippy::cast_possible_truncation)]
                on(
                    length / 2 + (c * 9_150.0) as i32,
                    width / 2 + (s * 9_150.0) as i32,
                )
            })
            .collect();
        painter.add(Shape::closed_line(circle, line));
        for (x, dir) in [(0, 1), (length, -1)] {
            for (depth, half) in [(1_650, 2_016), (550, 916)] {
                painter.add(Shape::closed_line(
                    quad(x, width / 2 - half, x + dir * depth, width / 2 + half),
                    line,
                ));
            }
        }
        let o = ds_match::balance::ObstacleBalance::load();
        for (a, b) in geom.goals(o.goal_half_width_cm as f32, o.goal_height_cm as f32) {
            if let (Some(p), Some(q)) = (geom.project(a), geom.project(b)) {
                painter.line_segment([Pos2::new(p.x, p.y), Pos2::new(q.x, q.y)], line);
            }
        }
    }

    /// Clock, score and play state, in the corner of the view.
    fn overlay(&self, painter: &Painter, r: Rect, palette: &Palette) {
        let (home, away) = self.score();
        let mark = if self.playing { "●" } else { "❚❚" };
        let text = format!("{}  {} – {}  {}", self.clock(), home, away, mark);
        let box_ = Rect::from_center_size(
            Pos2::new(r.center().x, r.top() + 18.0),
            Vec2::new(r.width().min(210.0), 30.0),
        );
        painter.rect_filled(box_, egui::CornerRadius::same(8), palette.grass);
        painter.text(
            box_.center(),
            Align2::CENTER_CENTER,
            text,
            FontId::monospace(14.0),
            palette.time,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn view() -> MatchView {
        MatchView::new(42).expect("the seed's fixture")
    }

    /// The view reads the record: the clock and the score come from a tick.
    #[test]
    fn the_view_reads_the_record() {
        let v = view();
        assert!(v.len() > 1_000);
        assert_eq!(v.tick(), 0);
        assert_eq!(v.clock(), "00:00");
        assert!(v.score().0 <= 20 && v.score().1 <= 20);
    }

    /// Playback follows real time and stops at the last tick.
    #[test]
    #[allow(clippy::cast_precision_loss)]
    fn playback_advances_and_stops() {
        let mut v = view();
        v.advance(1.0);
        assert!(v.tick() > 0);
        let last = v.len() - 1;
        v.tick = last as f64 - 1.0;
        v.advance(10.0);
        assert_eq!(v.tick(), last);
        assert!(!v.is_playing());
    }

    /// Every fact that has happened can be read as commentary.
    #[test]
    fn commentary_comes_from_the_ticks() {
        let mut v = view();
        let kick_off = v.commentary(3);
        assert_eq!(kick_off.len(), 1, "the kick-off is already a fact");
        assert!(kick_off[0].contains("Kick-off"));
        v.tick = (v.len() - 1) as f64;
        let late = v.commentary(3);
        assert_eq!(late.len(), 3);
        assert!(late.iter().all(|s| s.contains('\'')));
        assert_ne!(late, kick_off);
    }
}
