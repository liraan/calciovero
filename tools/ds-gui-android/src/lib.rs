//! Android GUI for Calcio Vero - fedele ai prototipi Android-A / Android-C.
//! A = portrait (lavagna + cassetto), C = landscape (lavagna + linguette).
#![allow(missing_docs)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::suboptimal_flops)]
#![allow(clippy::too_many_arguments)]

use ds_career::Career;
use ds_world::Pack;
use egui::{Align2, Color32, CornerRadius, FontId, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};

type MatchPos = ([(f32, f32); 11], [(f32, f32); 11]);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ViewMode {
    A,
    C,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scenario {
    Settimana,
    Partita,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    Con,
    Trans,
    Senza,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ATab {
    Cronaca,
    Istruzioni,
    Statistiche,
    Formazione,
    Perche,
}

struct Colors;
impl Colors {
    const BG: Color32 = Color32::from_rgb(0x0E, 0x11, 0x14);
    const PANEL: Color32 = Color32::from_rgb(0x16, 0x1B, 0x20);
    const PANEL2: Color32 = Color32::from_rgb(0x1D, 0x24, 0x2B);
    const LINE: Color32 = Color32::from_rgb(0x2A, 0x32, 0x3B);
    const TEXT: Color32 = Color32::from_rgb(0xE9, 0xEC, 0xEF);
    const MUTED: Color32 = Color32::from_rgb(0xA3, 0xAD, 0xB8);
    const ACCENT: Color32 = Color32::from_rgb(0xF0, 0xB0, 0x4A);
    const HOME: Color32 = Color32::from_rgb(0xF0, 0x8A, 0x5D);
    const AWAY: Color32 = Color32::from_rgb(0x6E, 0xA0, 0xFF);
    const HOME_TXT: Color32 = Color32::from_rgb(0x1A, 0x12, 0x06);
    const AWAY_TXT: Color32 = Color32::from_rgb(0x0E, 0x11, 0x14);
}

pub struct AndroidApp {
    career: Option<Career>,
    view: ViewMode,
    scenario: Scenario,
    phase: Phase,
    a_tab: ATab,
    selected_day: i32,
    view_day: i32,
    match_playing: bool,
    quick: [bool; 6],
}

impl Default for AndroidApp {
    fn default() -> Self {
        let career = Career::new(Pack::v1(), 42, 0).ok();
        let selected_day = career
            .as_ref()
            .map_or(0, |c| i32::try_from(c.day()).unwrap_or(0));
        Self {
            career,
            view: ViewMode::A,
            scenario: Scenario::Settimana,
            phase: Phase::Con,
            a_tab: ATab::Cronaca,
            selected_day,
            view_day: selected_day,
            match_playing: false,
            quick: [true, false, false, false, false, false],
        }
    }
}

impl AndroidApp {
    fn team_colors(&self) -> (Color32, Color32) {
        self.career.as_ref().map_or(
            (
                Color32::from_rgb(0x6E, 0x1E, 0x2A),
                Color32::from_rgb(0xF2, 0xE6, 0xD0),
            ),
            |c| {
                c.clubs().get(c.club() as usize).map_or(
                    (
                        Color32::from_rgb(0x6E, 0x1E, 0x2A),
                        Color32::from_rgb(0xF2, 0xE6, 0xD0),
                    ),
                    |cl| {
                        (
                            Color32::from_rgb(cl.colours[0][0], cl.colours[0][1], cl.colours[0][2]),
                            Color32::from_rgb(cl.colours[1][0], cl.colours[1][1], cl.colours[1][2]),
                        )
                    },
                )
            },
        )
    }
    fn club_name(&self) -> String {
        self.career.as_ref().map_or_else(
            || "Home".to_string(),
            |c| {
                c.clubs()
                    .get(c.club() as usize)
                    .map_or("Home", |cl| cl.name.as_str())
                    .to_string()
            },
        )
    }
    fn advance(&mut self) {
        if let Some(c) = self.career.as_mut() {
            let _ = c.advance();
            self.selected_day = i32::try_from(c.day()).unwrap_or(self.selected_day);
            self.view_day = self.selected_day;
        }
    }

    fn paint_pitch(ui: &egui::Ui, rect: Rect, vertical: bool) {
        let p = ui.painter_at(rect);
        p.rect_filled(
            rect,
            CornerRadius::ZERO,
            Color32::from_rgb(0x14, 0x35, 0x28),
        );
        let line = Stroke::new(1.5_f32, Color32::from_rgba_unmultiplied(233, 236, 239, 100));
        let inner = rect.shrink(8.0);
        p.rect_stroke(inner, CornerRadius::ZERO, line, StrokeKind::Middle);
        if vertical {
            p.line_segment(
                [
                    Pos2::new(inner.left(), inner.center().y),
                    Pos2::new(inner.right(), inner.center().y),
                ],
                line,
            );
            p.circle_stroke(inner.center(), inner.width() * 0.13, line);
            let bw = inner.width() * 0.6;
            let bh = inner.height() * 0.16;
            p.rect_stroke(
                Rect::from_min_size(
                    Pos2::new(inner.center().x - bw / 2.0, inner.top()),
                    Vec2::new(bw, bh),
                ),
                CornerRadius::ZERO,
                line,
                StrokeKind::Middle,
            );
            p.rect_stroke(
                Rect::from_min_size(
                    Pos2::new(inner.center().x - bw / 2.0, inner.bottom() - bh),
                    Vec2::new(bw, bh),
                ),
                CornerRadius::ZERO,
                line,
                StrokeKind::Middle,
            );
        } else {
            p.line_segment(
                [
                    Pos2::new(inner.center().x, inner.top()),
                    Pos2::new(inner.center().x, inner.bottom()),
                ],
                line,
            );
            p.circle_stroke(inner.center(), inner.height() * 0.13, line);
            let bw = inner.width() * 0.16;
            let bh = inner.height() * 0.6;
            p.rect_stroke(
                Rect::from_min_size(
                    Pos2::new(inner.left(), inner.center().y - bh / 2.0),
                    Vec2::new(bw, bh),
                ),
                CornerRadius::ZERO,
                line,
                StrokeKind::Middle,
            );
            p.rect_stroke(
                Rect::from_min_size(
                    Pos2::new(inner.right() - bw, inner.center().y - bh / 2.0),
                    Vec2::new(bw, bh),
                ),
                CornerRadius::ZERO,
                line,
                StrokeKind::Middle,
            );
        }
        let (home_pos, away_pos) = Self::match_positions(vertical);
        for (i, (x, y)) in home_pos.iter().enumerate() {
            let dot = Pos2::new(
                rect.left() + x * rect.width(),
                rect.top() + y * rect.height(),
            );
            p.circle_filled(dot, 13.0, Colors::HOME);
            p.text(
                dot,
                Align2::CENTER_CENTER,
                ((b'A' + i as u8) as char).to_string(),
                FontId::proportional(11.0),
                Colors::HOME_TXT,
            );
        }
        for (i, (x, y)) in away_pos.iter().enumerate() {
            let dot = Pos2::new(
                rect.left() + x * rect.width(),
                rect.top() + y * rect.height(),
            );
            p.circle_filled(dot, 13.0, Colors::AWAY);
            p.text(
                dot,
                Align2::CENTER_CENTER,
                ((b'a' + i as u8) as char).to_string(),
                FontId::proportional(11.0),
                Colors::AWAY_TXT,
            );
        }
        let ball = Pos2::new(
            rect.left() + 0.56 * rect.width(),
            rect.top() + 0.45 * rect.height(),
        );
        p.circle_filled(ball, 5.0, Color32::WHITE);
        p.circle_stroke(
            ball,
            5.0,
            Stroke::new(1.0_f32, Color32::from_rgba_unmultiplied(0, 0, 0, 80)),
        );
    }

    const fn match_positions(vertical: bool) -> MatchPos {
        if vertical {
            (
                [
                    (0.50, 0.95),
                    (0.86, 0.60),
                    (0.62, 0.72),
                    (0.40, 0.73),
                    (0.14, 0.56),
                    (0.82, 0.38),
                    (0.56, 0.52),
                    (0.38, 0.45),
                    (0.18, 0.34),
                    (0.46, 0.26),
                    (0.60, 0.30),
                ],
                [
                    (0.50, 0.05),
                    (0.84, 0.28),
                    (0.60, 0.22),
                    (0.40, 0.21),
                    (0.16, 0.27),
                    (0.70, 0.38),
                    (0.48, 0.40),
                    (0.30, 0.37),
                    (0.88, 0.42),
                    (0.52, 0.50),
                    (0.30, 0.52),
                ],
            )
        } else {
            (
                [
                    (0.05, 0.50),
                    (0.40, 0.86),
                    (0.28, 0.62),
                    (0.27, 0.40),
                    (0.44, 0.14),
                    (0.62, 0.82),
                    (0.48, 0.56),
                    (0.55, 0.38),
                    (0.66, 0.18),
                    (0.74, 0.46),
                    (0.70, 0.60),
                ],
                [
                    (0.95, 0.50),
                    (0.72, 0.84),
                    (0.78, 0.60),
                    (0.79, 0.40),
                    (0.73, 0.16),
                    (0.62, 0.70),
                    (0.60, 0.48),
                    (0.63, 0.30),
                    (0.58, 0.88),
                    (0.50, 0.52),
                    (0.48, 0.30),
                ],
            )
        }
    }

    const fn week_pitch_positions(phase: Phase) -> [(f32, f32); 11] {
        match phase {
            Phase::Con => [
                (0.06, 0.50),
                (0.32, 0.88),
                (0.22, 0.64),
                (0.22, 0.36),
                (0.32, 0.12),
                (0.58, 0.90),
                (0.38, 0.58),
                (0.46, 0.40),
                (0.58, 0.10),
                (0.70, 0.42),
                (0.66, 0.60),
            ],
            Phase::Trans => [
                (0.06, 0.50),
                (0.36, 0.84),
                (0.28, 0.62),
                (0.28, 0.40),
                (0.36, 0.16),
                (0.60, 0.78),
                (0.52, 0.58),
                (0.54, 0.42),
                (0.60, 0.22),
                (0.66, 0.46),
                (0.64, 0.56),
            ],
            Phase::Senza => [
                (0.06, 0.50),
                (0.20, 0.82),
                (0.18, 0.60),
                (0.18, 0.40),
                (0.20, 0.18),
                (0.34, 0.80),
                (0.32, 0.58),
                (0.32, 0.42),
                (0.34, 0.20),
                (0.48, 0.44),
                (0.48, 0.58),
            ],
        }
    }

    fn paint_week_pitch(ui: &egui::Ui, rect: Rect, vertical: bool, phase: Phase) {
        let p = ui.painter_at(rect);
        p.rect_filled(
            rect,
            CornerRadius::ZERO,
            Color32::from_rgb(0x14, 0x35, 0x28),
        );
        let line = Stroke::new(1.5_f32, Color32::from_rgba_unmultiplied(233, 236, 239, 100));
        let inner = rect.shrink(8.0);
        p.rect_stroke(inner, CornerRadius::ZERO, line, StrokeKind::Middle);
        if vertical {
            p.line_segment(
                [
                    Pos2::new(inner.left(), inner.center().y),
                    Pos2::new(inner.right(), inner.center().y),
                ],
                line,
            );
        } else {
            p.line_segment(
                [
                    Pos2::new(inner.center().x, inner.top()),
                    Pos2::new(inner.center().x, inner.bottom()),
                ],
                line,
            );
            p.circle_stroke(inner.center(), inner.height() * 0.13, line);
        }
        for (i, (x, y)) in Self::week_pitch_positions(phase).iter().enumerate() {
            let (px, py) = if vertical { (*y, *x) } else { (*x, *y) };
            let dot = Pos2::new(
                rect.left() + px * rect.width(),
                rect.top() + py * rect.height(),
            );
            p.circle_filled(dot, 13.0, Colors::HOME);
            p.text(
                dot,
                Align2::CENTER_CENTER,
                ((b'A' + i as u8) as char).to_string(),
                FontId::proportional(11.0),
                Colors::HOME_TXT,
            );
        }
    }

    fn top_bar_a(&mut self, ui: &egui::Ui, rect: Rect) {
        let (team, fg) = self.team_colors();
        ui.painter().rect_filled(rect, CornerRadius::ZERO, team);
        let logo = Rect::from_min_size(
            Pos2::new(rect.min.x + 10.0, rect.center().y - 18.0),
            Vec2::splat(36.0),
        );
        if ui
            .interact(logo, ui.id().with("logo"), Sense::click())
            .clicked()
        {
            self.scenario = Scenario::Settimana;
        }
        ui.painter()
            .circle_stroke(logo.center(), 14.0, Stroke::new(2.0_f32, Colors::ACCENT));
        ui.painter().text(
            logo.center(),
            Align2::CENTER_CENTER,
            "CV",
            FontId::proportional(11.0),
            fg,
        );
        let title = if self.scenario == Scenario::Partita {
            "62' · secondo tempo"
        } else {
            "Giovedì 13 · settimana 11"
        };
        let sub = if self.scenario == Scenario::Partita {
            format!("Serie di prova · {} – Real Valcastello", self.club_name())
        } else {
            "Gioca sabato in casa".to_string()
        };
        ui.painter().text(
            Pos2::new(rect.min.x + 58.0, rect.center().y - 8.0),
            Align2::LEFT_CENTER,
            title,
            FontId::proportional(15.0),
            fg,
        );
        ui.painter().text(
            Pos2::new(rect.min.x + 58.0, rect.center().y + 10.0),
            Align2::LEFT_CENTER,
            sub,
            FontId::proportional(11.0),
            fg.gamma_multiply(0.75),
        );
        let pause = Rect::from_center_size(
            Pos2::new(rect.max.x - 30.0, rect.center().y),
            Vec2::splat(40.0),
        );
        if ui
            .interact(pause, ui.id().with("pause"), Sense::click())
            .clicked()
            && self.scenario == Scenario::Partita
        {
            self.match_playing = !self.match_playing;
        }
        ui.painter()
            .rect_filled(pause, CornerRadius::same(10), fg.gamma_multiply(0.12));
        let ic = pause.center();
        if self.scenario == Scenario::Partita && self.match_playing {
            ui.painter().rect_filled(
                Rect::from_center_size(ic - Vec2::new(4.0, 0.0), Vec2::new(3.0, 14.0)),
                CornerRadius::ZERO,
                fg,
            );
            ui.painter().rect_filled(
                Rect::from_center_size(ic + Vec2::new(4.0, 0.0), Vec2::new(3.0, 14.0)),
                CornerRadius::ZERO,
                fg,
            );
        } else {
            ui.painter().text(
                ic,
                Align2::CENTER_CENTER,
                "▶",
                FontId::proportional(16.0),
                fg,
            );
        }
    }

    fn score_strip(&self, ui: &egui::Ui, rect: Rect) {
        ui.painter()
            .rect_filled(rect, CornerRadius::ZERO, Colors::PANEL);
        ui.painter().line_segment(
            [
                Pos2::new(rect.left(), rect.bottom()),
                Pos2::new(rect.right(), rect.bottom()),
            ],
            Stroke::new(1.0_f32, Colors::LINE),
        );
        let y = rect.center().y;
        let cx = rect.center().x;
        ui.painter().rect_filled(
            Rect::from_center_size(Pos2::new(cx - 110.0, y), Vec2::splat(10.0)),
            CornerRadius::same(2),
            Colors::HOME,
        );
        ui.painter().text(
            Pos2::new(cx - 92.0, y),
            Align2::LEFT_CENTER,
            self.club_name(),
            FontId::proportional(14.0),
            Colors::TEXT,
        );
        ui.painter().text(
            Pos2::new(cx, y),
            Align2::CENTER_CENTER,
            "0 – 1",
            FontId::monospace(20.0),
            Colors::TEXT,
        );
        ui.painter().text(
            Pos2::new(cx + 60.0, y),
            Align2::LEFT_CENTER,
            "Valcastello",
            FontId::proportional(14.0),
            Colors::TEXT,
        );
        ui.painter().rect_filled(
            Rect::from_center_size(Pos2::new(cx + 138.0, y), Vec2::splat(10.0)),
            CornerRadius::same(2),
            Colors::AWAY,
        );
    }

    fn quick_bar(&mut self, ui: &egui::Ui, rect: Rect) {
        let labels = [
            "Più pressione",
            "Linea più alta",
            "Allarga il gioco",
            "Palla lunga",
            "Perdi tempo",
            "Tutti avanti",
        ];
        let mut x = rect.min.x + 8.0;
        for (i, lab) in labels.iter().enumerate() {
            let w = lab.len() as f32 * 7.2 + 24.0;
            let r = Rect::from_min_size(Pos2::new(x, rect.min.y + 6.0), Vec2::new(w, 40.0));
            if ui
                .interact(r, ui.id().with(("q", i)), Sense::click())
                .clicked()
            {
                self.quick[i] = !self.quick[i];
            }
            let on = self.quick[i];
            ui.painter().rect_filled(
                r,
                CornerRadius::same(20),
                if on {
                    Colors::ACCENT
                } else {
                    Color32::from_rgba_unmultiplied(14, 17, 20, 235)
                },
            );
            ui.painter().rect_stroke(
                r,
                CornerRadius::same(20),
                Stroke::new(1.0_f32, if on { Colors::ACCENT } else { Colors::LINE }),
                StrokeKind::Middle,
            );
            ui.painter().text(
                r.center(),
                Align2::CENTER_CENTER,
                *lab,
                FontId::proportional(12.0),
                if on {
                    Color32::from_rgb(0x1A, 0x12, 0x06)
                } else {
                    Colors::TEXT
                },
            );
            x += w + 8.0;
            if x > rect.max.x - 20.0 {
                break;
            }
        }
    }

    fn drawer_a(&mut self, ui: &egui::Ui, rect: Rect) {
        ui.painter().rect_filled(
            rect,
            CornerRadius {
                nw: 22,
                ne: 22,
                sw: 0,
                se: 0,
            },
            Colors::PANEL,
        );
        ui.painter().rect_filled(
            Rect::from_center_size(
                Pos2::new(rect.center().x, rect.min.y + 12.0),
                Vec2::new(40.0, 5.0),
            ),
            CornerRadius::same(3),
            Colors::LINE,
        );
        let tabs = [
            (ATab::Cronaca, "Cronaca"),
            (ATab::Istruzioni, "Istruzioni"),
            (ATab::Statistiche, "Statistiche"),
            (ATab::Formazione, "Formazione"),
            (ATab::Perche, "Perché"),
        ];
        let mut tx = rect.min.x + 12.0;
        for (tab, label) in tabs {
            let tw = label.len() as f32 * 7.5 + 24.0;
            let tr = Rect::from_min_size(Pos2::new(tx, rect.min.y + 28.0), Vec2::new(tw, 38.0));
            if ui
                .interact(tr, ui.id().with(("tab", label)), Sense::click())
                .clicked()
            {
                self.a_tab = tab;
            }
            let active = self.a_tab == tab;
            ui.painter().rect_filled(
                tr,
                CornerRadius::same(19),
                if active {
                    Colors::PANEL2
                } else {
                    Color32::TRANSPARENT
                },
            );
            ui.painter().rect_stroke(
                tr,
                CornerRadius::same(19),
                Stroke::new(1.0_f32, if active { Colors::ACCENT } else { Colors::LINE }),
                StrokeKind::Middle,
            );
            ui.painter().text(
                tr.center(),
                Align2::CENTER_CENTER,
                label,
                FontId::proportional(12.0),
                Colors::TEXT,
            );
            tx += tw + 8.0;
            if tx > rect.max.x - 30.0 {
                break;
            }
        }
        let body = Rect::from_min_max(
            Pos2::new(rect.min.x + 16.0, rect.min.y + 74.0),
            rect.max - Vec2::new(16.0, 8.0),
        );
        self.paint_drawer_body(ui, body);
    }

    fn paint_drawer_body(&self, ui: &egui::Ui, body: Rect) {
        let p = ui.painter_at(body);
        match self.a_tab {
            ATab::Cronaca => {
                let rows = [
                    ("62'", "Home 9 calcia da fuori: alto.", false),
                    ("58'", "Ammonito Valcastello 4, fallo su Home 7.", false),
                    ("52'", "Cambio: Home 15 per Home 6.", false),
                    ("34'", "Gol! Valcastello 9 di testa, su angolo.", true),
                    ("21'", "Parata di Home 1 sul tiro di Valcastello 7.", false),
                ];
                let mut y = body.min.y;
                for (t, x, gol) in rows {
                    p.text(
                        Pos2::new(body.min.x, y + 10.0),
                        Align2::LEFT_CENTER,
                        t,
                        FontId::monospace(12.0),
                        Colors::MUTED,
                    );
                    p.text(
                        Pos2::new(body.min.x + 44.0, y + 10.0),
                        Align2::LEFT_CENTER,
                        x,
                        FontId::proportional(13.0),
                        if gol { Colors::ACCENT } else { Colors::TEXT },
                    );
                    y += 34.0;
                    p.line_segment(
                        [Pos2::new(body.min.x, y), Pos2::new(body.max.x, y)],
                        Stroke::new(1.0_f32, Colors::LINE),
                    );
                    if y > body.max.y - 10.0 {
                        break;
                    }
                }
            }
            ATab::Istruzioni => {
                let labels = [
                    "Più pressione",
                    "Linea più alta",
                    "Allarga il gioco",
                    "Palla lunga",
                    "Perdi tempo",
                    "Tutti avanti",
                ];
                for (i, lab) in labels.iter().enumerate() {
                    let r = Rect::from_min_size(
                        Pos2::new(body.min.x, body.min.y + i as f32 * 38.0),
                        Vec2::new(body.width().min(320.0), 32.0),
                    );
                    let on = self.quick[i];
                    p.rect_filled(
                        r,
                        CornerRadius::same(8),
                        if on {
                            Colors::ACCENT.gamma_multiply(0.2)
                        } else {
                            Color32::TRANSPARENT
                        },
                    );
                    p.text(
                        r.left_center() + Vec2::new(8.0, 0.0),
                        Align2::LEFT_CENTER,
                        *lab,
                        FontId::proportional(13.0),
                        if on { Colors::ACCENT } else { Colors::TEXT },
                    );
                }
            }
            ATab::Statistiche => {
                let stats = [
                    ("Possesso", "52%", "48%"),
                    ("Tiri", "7", "5"),
                    ("In porta", "3", "2"),
                    ("xG", "0.8", "1.1"),
                    ("Falli", "9", "11"),
                ];
                let mut y = body.min.y;
                for (n, h, a) in stats {
                    p.text(
                        Pos2::new(body.min.x + body.width() * 0.5, y + 10.0),
                        Align2::CENTER_CENTER,
                        n,
                        FontId::proportional(12.0),
                        Colors::MUTED,
                    );
                    p.text(
                        Pos2::new(body.min.x + 30.0, y + 10.0),
                        Align2::LEFT_CENTER,
                        h,
                        FontId::monospace(13.0),
                        Colors::TEXT,
                    );
                    p.text(
                        Pos2::new(body.max.x - 30.0, y + 10.0),
                        Align2::RIGHT_CENTER,
                        a,
                        FontId::monospace(13.0),
                        Colors::TEXT,
                    );
                    y += 32.0;
                }
            }
            ATab::Formazione => {
                p.text(
                    body.min + Vec2::new(0.0, 10.0),
                    Align2::LEFT_TOP,
                    "4-3-3 · Energia",
                    FontId::proportional(13.0),
                    Colors::MUTED,
                );
                let mut y = body.min.y + 34.0;
                if let Some(c) = &self.career {
                    for m in c.squad().iter().take(11) {
                        p.text(
                            Pos2::new(body.min.x, y),
                            Align2::LEFT_CENTER,
                            format!("{} {}", m.shirt, m.name),
                            FontId::proportional(12.0),
                            Colors::TEXT,
                        );
                        let en = ((1.0 - m.tired).clamp(0.0, 1.0) * 100.0) as i32;
                        p.text(
                            Pos2::new(body.max.x - 4.0, y),
                            Align2::RIGHT_CENTER,
                            format!("{en}%"),
                            FontId::monospace(11.0),
                            Colors::MUTED,
                        );
                        y += 24.0;
                        if y > body.max.y - 8.0 {
                            break;
                        }
                    }
                }
            }
            ATab::Perche => {
                p.text(
                    body.min + Vec2::new(0.0, 10.0),
                    Align2::LEFT_TOP,
                    "62' · AWAY 6",
                    FontId::proportional(11.0),
                    Colors::ACCENT,
                );
                p.text(
                    body.min + Vec2::new(0.0, 30.0),
                    Align2::LEFT_TOP,
                    "Tiro forte da 31 m",
                    FontId::proportional(15.0),
                    Colors::TEXT,
                );
                p.text(
                    body.min + Vec2::new(0.0, 54.0),
                    Align2::LEFT_TOP,
                    "L'azione nasce 2 s prima: Home 6 spazza,\nloro la riconquistano.",
                    FontId::proportional(12.0),
                    Colors::MUTED,
                );
            }
        }
    }

    fn paint_a_partita(&mut self, ui: &egui::Ui, full: Rect) {
        let top = Rect::from_min_size(full.min, Vec2::new(full.width(), 60.0));
        let strip = Rect::from_min_size(
            Pos2::new(full.min.x, top.max.y),
            Vec2::new(full.width(), 52.0),
        );
        let drawer_h = 320.0_f32.min(full.height() * 0.42);
        let quick = Rect::from_min_size(
            Pos2::new(full.min.x, full.max.y - drawer_h - 54.0),
            Vec2::new(full.width(), 54.0),
        );
        let pitch = Rect::from_min_max(
            Pos2::new(full.min.x + 12.0, strip.max.y + 8.0),
            Pos2::new(full.max.x - 12.0, quick.min.y - 4.0),
        );
        let drawer = Rect::from_min_size(
            Pos2::new(full.min.x, full.max.y - drawer_h),
            Vec2::new(full.width(), drawer_h),
        );
        self.top_bar_a(ui, top);
        self.score_strip(ui, strip);
        Self::paint_pitch(ui, pitch, true);
        self.quick_bar(ui, quick);
        self.drawer_a(ui, drawer);
    }

    fn paint_a_settimana(&mut self, ui: &egui::Ui, full: Rect) {
        let top = Rect::from_min_size(full.min, Vec2::new(full.width(), 60.0));
        let strip = Rect::from_min_size(
            Pos2::new(full.min.x, top.max.y),
            Vec2::new(full.width(), 48.0),
        );
        ui.painter()
            .rect_filled(strip, CornerRadius::ZERO, Colors::PANEL);
        let days = [
            "LUN\n10", "MAR\n11", "MER\n12", "GIO\n13", "VEN\n14", "SAB\n15", "DOM\n16",
        ];
        let dw = strip.width() / 7.0;
        for (i, d) in days.iter().enumerate() {
            let r = Rect::from_min_size(
                Pos2::new(strip.min.x + i as f32 * dw, strip.min.y),
                Vec2::new(dw, strip.height()),
            );
            if ui
                .interact(r, ui.id().with(("day", i)), Sense::click())
                .clicked()
            {
                self.view_day = i as i32;
                if i == 5 {
                    self.scenario = Scenario::Partita;
                }
            }
            let sel = self.view_day == i as i32;
            if sel {
                ui.painter().rect_filled(
                    r.shrink2(Vec2::new(4.0, 4.0)),
                    CornerRadius::same(8),
                    Colors::PANEL2,
                );
            }
            ui.painter().text(
                r.center(),
                Align2::CENTER_CENTER,
                *d,
                FontId::proportional(10.0),
                if sel { Colors::ACCENT } else { Colors::MUTED },
            );
        }
        let phase_bar = Rect::from_min_size(
            Pos2::new(full.min.x, strip.max.y),
            Vec2::new(full.width(), 44.0),
        );
        let phases = [
            (Phase::Con, "Con palla"),
            (Phase::Trans, "Transizione"),
            (Phase::Senza, "Senza palla"),
        ];
        let pw = phase_bar.width() / 3.0;
        for (i, (ph, lab)) in phases.iter().enumerate() {
            let r = Rect::from_min_size(
                Pos2::new(phase_bar.min.x + i as f32 * pw + 6.0, phase_bar.min.y + 6.0),
                Vec2::new(pw - 12.0, 32.0),
            );
            if ui
                .interact(r, ui.id().with(("ph", i)), Sense::click())
                .clicked()
            {
                self.phase = *ph;
            }
            let active = self.phase == *ph;
            ui.painter().rect_filled(
                r,
                CornerRadius::same(16),
                if active {
                    Colors::PANEL2
                } else {
                    Color32::TRANSPARENT
                },
            );
            ui.painter().rect_stroke(
                r,
                CornerRadius::same(16),
                Stroke::new(1.0_f32, if active { Colors::ACCENT } else { Colors::LINE }),
                StrokeKind::Middle,
            );
            ui.painter().text(
                r.center(),
                Align2::CENTER_CENTER,
                *lab,
                FontId::proportional(12.0),
                if active { Colors::TEXT } else { Colors::MUTED },
            );
        }
        let pitch = Rect::from_min_max(
            Pos2::new(full.min.x + 12.0, phase_bar.max.y + 8.0),
            Pos2::new(full.max.x - 12.0, full.max.y - 280.0),
        );
        Self::paint_week_pitch(ui, pitch, true, self.phase);
        let drawer = Rect::from_min_size(
            Pos2::new(full.min.x, full.max.y - 272.0),
            Vec2::new(full.width(), 272.0),
        );
        ui.painter().rect_filled(
            drawer,
            CornerRadius {
                nw: 22,
                ne: 22,
                sw: 0,
                se: 0,
            },
            Colors::PANEL,
        );
        ui.painter().rect_filled(
            Rect::from_center_size(
                Pos2::new(drawer.center().x, drawer.min.y + 12.0),
                Vec2::new(40.0, 5.0),
            ),
            CornerRadius::same(3),
            Colors::LINE,
        );
        let tabs = ["Giornata", "Rosa", "Posta", "Prossima", "Istruzioni"];
        let mut tx = drawer.min.x + 12.0;
        for (i, lab) in tabs.iter().enumerate() {
            let tw = lab.len() as f32 * 7.2 + 22.0;
            let tr = Rect::from_min_size(Pos2::new(tx, drawer.min.y + 28.0), Vec2::new(tw, 36.0));
            let active = i == 0;
            ui.painter().rect_filled(
                tr,
                CornerRadius::same(18),
                if active {
                    Colors::PANEL2
                } else {
                    Color32::TRANSPARENT
                },
            );
            ui.painter().rect_stroke(
                tr,
                CornerRadius::same(18),
                Stroke::new(1.0_f32, if active { Colors::ACCENT } else { Colors::LINE }),
                StrokeKind::Middle,
            );
            ui.painter().text(
                tr.center(),
                Align2::CENTER_CENTER,
                *lab,
                FontId::proportional(11.5),
                Colors::TEXT,
            );
            tx += tw + 6.0;
        }
        let body = Rect::from_min_max(
            Pos2::new(drawer.min.x + 16.0, drawer.min.y + 72.0),
            drawer.max - Vec2::new(16.0, 12.0),
        );
        ui.painter().text(
            body.min,
            Align2::LEFT_TOP,
            "Giovedì · oggi",
            FontId::proportional(16.0),
            Colors::TEXT,
        );
        let adv = Rect::from_min_size(
            Pos2::new(body.max.x - 110.0, body.min.y),
            Vec2::new(100.0, 34.0),
        );
        if ui
            .interact(adv, ui.id().with("adv"), Sense::click())
            .clicked()
        {
            self.advance();
        }
        ui.painter()
            .rect_filled(adv, CornerRadius::same(8), Colors::ACCENT);
        ui.painter().text(
            adv.center(),
            Align2::CENTER_CENTER,
            "Continua",
            FontId::proportional(12.0),
            Color32::from_rgb(0x1A, 0x12, 0x06),
        );
        let rows = [
            (
                "10:00",
                "Video · Real Valcastello",
                "Blocco basso, ampiezza stretta.",
            ),
            (
                "11:00",
                "Allenamento · senza palla",
                "Pressione sul loro terzino.",
            ),
            ("15:00", "Staff medico", "Home 6 è stanco."),
        ];
        let mut y = body.min.y + 44.0;
        for (h, t, d) in rows {
            ui.painter().text(
                Pos2::new(body.min.x, y),
                Align2::LEFT_TOP,
                h,
                FontId::monospace(11.0),
                Colors::MUTED,
            );
            ui.painter().text(
                Pos2::new(body.min.x + 52.0, y),
                Align2::LEFT_TOP,
                t,
                FontId::proportional(13.0),
                Colors::TEXT,
            );
            ui.painter().text(
                Pos2::new(body.min.x + 52.0, y + 18.0),
                Align2::LEFT_TOP,
                d,
                FontId::proportional(11.0),
                Colors::MUTED,
            );
            y += 48.0;
        }
        self.top_bar_a(ui, top);
    }

    fn paint_c_partita(&mut self, ui: &egui::Ui, full: Rect) {
        let (team, fg) = self.team_colors();
        let top = Rect::from_min_size(full.min, Vec2::new(full.width(), 48.0));
        ui.painter().rect_filled(top, CornerRadius::ZERO, team);
        ui.painter().text(
            Pos2::new(top.min.x + 16.0, top.center().y),
            Align2::LEFT_CENTER,
            "CV",
            FontId::proportional(14.0),
            Colors::ACCENT,
        );
        ui.painter().text(
            Pos2::new(top.min.x + 54.0, top.center().y),
            Align2::LEFT_CENTER,
            "62'",
            FontId::proportional(15.0),
            fg,
        );
        ui.painter().text(
            top.center(),
            Align2::CENTER_CENTER,
            format!("{}  0 – 1  Valcastello", self.club_name()),
            FontId::proportional(16.0),
            fg,
        );
        let pause = Rect::from_center_size(
            Pos2::new(top.max.x - 30.0, top.center().y),
            Vec2::splat(36.0),
        );
        if ui
            .interact(pause, ui.id().with("cpause"), Sense::click())
            .clicked()
        {
            self.match_playing = !self.match_playing;
        }
        ui.painter().text(
            pause.center(),
            Align2::CENTER_CENTER,
            if self.match_playing { "||" } else { "▶" },
            FontId::proportional(14.0),
            fg,
        );

        let left = Rect::from_min_max(
            Pos2::new(full.min.x, top.max.y),
            Pos2::new(full.min.x + 148.0, full.max.y),
        );
        let right = Rect::from_min_max(
            Pos2::new(full.max.x - 184.0, top.max.y),
            Pos2::new(full.max.x, full.max.y),
        );
        let pitch = Rect::from_min_max(
            Pos2::new(left.max.x + 4.0, top.max.y + 4.0),
            Pos2::new(right.min.x - 4.0, full.max.y - 4.0),
        );

        ui.painter().text(
            Pos2::new(left.min.x + 10.0, left.min.y + 12.0),
            Align2::LEFT_TOP,
            "ISTRUZIONI",
            FontId::proportional(10.0),
            Colors::MUTED,
        );
        let labels = [
            "Più pressione",
            "Linea più alta",
            "Allarga il gioco",
            "Palla lunga",
            "Perdi tempo",
            "Tutti avanti",
        ];
        for (i, lab) in labels.iter().enumerate() {
            let r = Rect::from_min_size(
                Pos2::new(left.min.x + 8.0, left.min.y + 30.0 + i as f32 * 46.0),
                Vec2::new(left.width() - 16.0, 38.0),
            );
            if ui
                .interact(r, ui.id().with(("cq", i)), Sense::click())
                .clicked()
            {
                self.quick[i] = !self.quick[i];
            }
            let on = self.quick[i];
            ui.painter().rect_filled(
                r,
                CornerRadius::same(8),
                if on { Colors::ACCENT } else { Colors::PANEL },
            );
            ui.painter().rect_stroke(
                r,
                CornerRadius::same(8),
                Stroke::new(1.0_f32, if on { Colors::ACCENT } else { Colors::LINE }),
                StrokeKind::Middle,
            );
            ui.painter().text(
                r.center(),
                Align2::CENTER_CENTER,
                *lab,
                FontId::proportional(11.0),
                if on {
                    Color32::from_rgb(0x1A, 0x12, 0x06)
                } else {
                    Colors::TEXT
                },
            );
        }

        Self::paint_pitch(ui, pitch, false);

        ui.painter().rect_filled(
            right.shrink2(Vec2::new(4.0, 4.0)),
            CornerRadius::same(10),
            Colors::PANEL,
        );
        ui.painter().rect_stroke(
            right.shrink2(Vec2::new(4.0, 4.0)),
            CornerRadius::same(10),
            Stroke::new(1.0_f32, Colors::LINE),
            StrokeKind::Middle,
        );
        ui.painter().line_segment(
            [
                Pos2::new(right.min.x + 4.0, right.min.y + 4.0),
                Pos2::new(right.max.x - 4.0, right.min.y + 4.0),
            ],
            Stroke::new(3.0_f32, Colors::ACCENT),
        );
        ui.painter().text(
            Pos2::new(right.min.x + 14.0, right.min.y + 16.0),
            Align2::LEFT_TOP,
            "Cronaca",
            FontId::proportional(13.0),
            Colors::TEXT,
        );
        let rows = [
            ("62'", "Home 9 calcia da fuori: alto.", false),
            ("58'", "Ammonito Valcastello 4.", false),
            ("52'", "Cambio: Home 15 per Home 6.", false),
            ("34'", "Gol! Valcastello 9 di testa.", true),
        ];
        let mut y = right.min.y + 42.0;
        for (t, x, gol) in rows {
            ui.painter().text(
                Pos2::new(right.min.x + 14.0, y),
                Align2::LEFT_TOP,
                t,
                FontId::monospace(11.0),
                Colors::MUTED,
            );
            ui.painter().text(
                Pos2::new(right.min.x + 42.0, y),
                Align2::LEFT_TOP,
                x,
                FontId::proportional(11.0),
                if gol { Colors::ACCENT } else { Colors::TEXT },
            );
            y += 34.0;
        }
    }

    fn paint_c_settimana(&mut self, ui: &egui::Ui, full: Rect) {
        let (team, fg) = self.team_colors();
        let top = Rect::from_min_size(full.min, Vec2::new(full.width(), 48.0));
        ui.painter().rect_filled(top, CornerRadius::ZERO, team);
        ui.painter().text(
            Pos2::new(top.min.x + 16.0, top.center().y),
            Align2::LEFT_CENTER,
            "Giovedì 13",
            FontId::proportional(15.0),
            fg,
        );
        ui.painter().text(
            top.center(),
            Align2::CENTER_CENTER,
            "settimana 11 · 2 giorni alla partita",
            FontId::proportional(13.0),
            fg.gamma_multiply(0.85),
        );
        let left = Rect::from_min_max(
            Pos2::new(full.min.x, top.max.y),
            Pos2::new(full.min.x + 120.0, full.max.y),
        );
        let right = Rect::from_min_max(
            Pos2::new(full.max.x - 220.0, top.max.y),
            Pos2::new(full.max.x, full.max.y),
        );
        let pitch = Rect::from_min_max(
            Pos2::new(left.max.x + 4.0, top.max.y + 4.0),
            Pos2::new(right.min.x - 4.0, full.max.y - 44.0),
        );
        let bottom = Rect::from_min_max(
            Pos2::new(left.max.x + 4.0, pitch.max.y + 4.0),
            Pos2::new(right.min.x - 4.0, full.max.y),
        );

        let tabs = ["Rosa", "Posta", "Giornata", "Istruz.", "Statist."];
        for (i, lab) in tabs.iter().enumerate() {
            let r = Rect::from_min_size(
                Pos2::new(left.min.x + 8.0, left.min.y + 12.0 + i as f32 * 46.0),
                Vec2::new(left.width() - 16.0, 38.0),
            );
            let active = i == 2;
            ui.painter().rect_filled(
                r,
                CornerRadius::same(8),
                if active {
                    Colors::PANEL2
                } else {
                    Colors::PANEL
                },
            );
            ui.painter().rect_stroke(
                r,
                CornerRadius::same(8),
                Stroke::new(1.0_f32, if active { Colors::ACCENT } else { Colors::LINE }),
                StrokeKind::Middle,
            );
            ui.painter().text(
                r.center(),
                Align2::CENTER_CENTER,
                *lab,
                FontId::proportional(12.0),
                Colors::TEXT,
            );
        }
        Self::paint_week_pitch(ui, pitch, false, self.phase);
        let phases = [
            (Phase::Con, "Con palla"),
            (Phase::Trans, "Transiz."),
            (Phase::Senza, "Senza"),
        ];
        let pw = bottom.width() / 4.0;
        for (i, (ph, lab)) in phases.iter().enumerate() {
            let r = Rect::from_min_size(
                Pos2::new(bottom.min.x + i as f32 * pw + 4.0, bottom.min.y + 4.0),
                Vec2::new(pw - 8.0, 32.0),
            );
            if ui
                .interact(r, ui.id().with(("cph", i)), Sense::click())
                .clicked()
            {
                self.phase = *ph;
            }
            let active = self.phase == *ph;
            ui.painter().rect_filled(
                r,
                CornerRadius::same(16),
                if active {
                    Colors::PANEL2
                } else {
                    Color32::TRANSPARENT
                },
            );
            ui.painter().rect_stroke(
                r,
                CornerRadius::same(16),
                Stroke::new(1.0_f32, if active { Colors::ACCENT } else { Colors::LINE }),
                StrokeKind::Middle,
            );
            ui.painter().text(
                r.center(),
                Align2::CENTER_CENTER,
                *lab,
                FontId::proportional(11.0),
                Colors::TEXT,
            );
        }
        let adv = Rect::from_min_size(
            Pos2::new(bottom.max.x - pw + 4.0, bottom.min.y + 4.0),
            Vec2::new(pw - 8.0, 32.0),
        );
        if ui
            .interact(adv, ui.id().with("cadv"), Sense::click())
            .clicked()
        {
            self.advance();
        }
        ui.painter()
            .rect_filled(adv, CornerRadius::same(16), Colors::ACCENT);
        ui.painter().text(
            adv.center(),
            Align2::CENTER_CENTER,
            "Continua",
            FontId::proportional(11.0),
            Color32::from_rgb(0x1A, 0x12, 0x06),
        );

        ui.painter().rect_filled(
            right.shrink2(Vec2::new(4.0, 4.0)),
            CornerRadius::same(10),
            Colors::PANEL,
        );
        ui.painter().text(
            Pos2::new(right.min.x + 14.0, right.min.y + 16.0),
            Align2::LEFT_TOP,
            "Prossima partita",
            FontId::proportional(13.0),
            Colors::TEXT,
        );
        ui.painter().text(
            Pos2::new(right.min.x + 14.0, right.min.y + 42.0),
            Align2::LEFT_TOP,
            format!("{} – Real Valcastello", self.club_name()),
            FontId::proportional(15.0),
            Colors::TEXT,
        );
        ui.painter().text(
            Pos2::new(right.min.x + 14.0, right.min.y + 68.0),
            Align2::LEFT_TOP,
            "Sabato 15 · 15:00 · in casa",
            FontId::proportional(11.0),
            Colors::MUTED,
        );
    }
}

impl eframe::App for AndroidApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let size = ctx.screen_rect().size();
        self.view = if size.x > size.y {
            ViewMode::C
        } else {
            ViewMode::A
        };
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(Colors::BG))
            .show(ctx, |ui| {
                let full = ui.available_rect_before_wrap();
                match (self.view, self.scenario) {
                    (ViewMode::A, Scenario::Partita) => self.paint_a_partita(ui, full),
                    (ViewMode::A, Scenario::Settimana) => self.paint_a_settimana(ui, full),
                    (ViewMode::C, Scenario::Partita) => self.paint_c_partita(ui, full),
                    (ViewMode::C, Scenario::Settimana) => self.paint_c_settimana(ui, full),
                }
            });
    }
}

#[cfg(target_os = "android")]
#[allow(unsafe_code)]
#[unsafe(no_mangle)]
fn android_main(android_app: winit::platform::android::activity::AndroidApp) {
    let options = eframe::NativeOptions {
        android_app: Some(android_app),
        ..Default::default()
    };
    if let Err(e) = eframe::run_native(
        "Calcio Vero",
        options,
        Box::new(|_cc| Ok(Box::new(AndroidApp::default()))),
    ) {
        eprintln!("ds-gui-android: {e}");
    }
}
