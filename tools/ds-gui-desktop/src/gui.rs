//! Desktop GUI - riferimento: `context/previews/gui-from-maintainer/Lavagna.dc.html`.
//! Lavagna tattica, barra di sistema, linguette a sinistra, post-it/finestra.
#![allow(missing_docs)]
#![allow(clippy::suboptimal_flops)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::needless_pass_by_ref_mut)]
#![allow(float_literal_f32_fallback)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::unused_self)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::manual_find)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::bool_to_int_with_if)]

use ds_career::Career;
use ds_world::Pack;

use ds_gui_view::{MatchView, Palette};
use eframe::egui::{
    self, Align2, Area, Color32, CornerRadius, FontId, Id, Order, Pos2, Rect, RichText, Sense,
    Stroke, StrokeKind, Vec2,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum ThemeId {
    Lavagna,
    Notte,
    Carta,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct BoardTheme {
    bg: Color32,
    glow_center: Color32,
    chalk: Color32,
    chalk_soft: Color32,
    bar: Color32,
    line: Color32,
    text: Color32,
    muted: Color32,
    accent: Color32,
    panel: Color32,
    panel2: Color32,
    pitch: Color32,
    pitch_line: Color32,
}

#[allow(clippy::missing_const_for_fn)]
impl BoardTheme {
    fn get(id: ThemeId) -> Self {
        match id {
            ThemeId::Lavagna => Self {
                bg: Color32::from_rgb(0x0F, 0x2A, 0x21),
                glow_center: Color32::from_rgb(0x1E, 0x4A, 0x3A),
                chalk: Color32::from_rgba_unmultiplied(246, 235, 211, 178),
                chalk_soft: Color32::from_rgba_unmultiplied(246, 235, 211, 140),
                bar: Color32::from_rgba_unmultiplied(16, 40, 32, 247),
                line: Color32::from_rgba_unmultiplied(246, 235, 211, 40),
                text: Color32::from_rgb(0xF6, 0xEB, 0xD3),
                muted: Color32::from_rgb(0xC2, 0xB7, 0x9D),
                accent: Color32::from_rgb(0xD9, 0xA2, 0x4A),
                panel: Color32::from_rgb(0x12, 0x3A, 0x2F),
                panel2: Color32::from_rgb(0x17, 0x45, 0x38),
                pitch: Color32::from_rgb(0x17, 0x3A, 0x2C),
                pitch_line: Color32::from_rgba_unmultiplied(233, 236, 239, 115),
            },
            ThemeId::Notte => Self {
                bg: Color32::from_rgb(0x0E, 0x11, 0x14),
                glow_center: Color32::from_rgb(0x1C, 0x23, 0x2A),
                chalk: Color32::from_rgba_unmultiplied(233, 236, 239, 107),
                chalk_soft: Color32::from_rgba_unmultiplied(233, 236, 239, 102),
                bar: Color32::from_rgba_unmultiplied(22, 27, 32, 247),
                line: Color32::from_rgb(0x2A, 0x32, 0x3B),
                text: Color32::from_rgb(0xE9, 0xEC, 0xEF),
                muted: Color32::from_rgb(0xA3, 0xAD, 0xB8),
                accent: Color32::from_rgb(0xF0, 0xB0, 0x4A),
                panel: Color32::from_rgb(0x16, 0x1B, 0x20),
                panel2: Color32::from_rgb(0x1D, 0x24, 0x2B),
                pitch: Color32::from_rgb(0x17, 0x3A, 0x2C),
                pitch_line: Color32::from_rgba_unmultiplied(233, 236, 239, 115),
            },
            ThemeId::Carta => Self {
                bg: Color32::from_rgb(0xF3, 0xED, 0xE1),
                glow_center: Color32::from_rgb(0xFB, 0xF7, 0xEF),
                chalk: Color32::from_rgba_unmultiplied(28, 38, 34, 140),
                chalk_soft: Color32::from_rgba_unmultiplied(28, 38, 34, 115),
                bar: Color32::from_rgba_unmultiplied(251, 247, 239, 247),
                line: Color32::from_rgb(0xD8, 0xCD, 0xB8),
                text: Color32::from_rgb(0x1C, 0x26, 0x22),
                muted: Color32::from_rgb(0x5B, 0x62, 0x5C),
                accent: Color32::from_rgb(0x1F, 0x5A, 0x45),
                panel: Color32::from_rgb(0xFB, 0xF7, 0xEF),
                panel2: Color32::from_rgb(0xEF, 0xE7, 0xD8),
                pitch: Color32::from_rgb(0x2F, 0x5A, 0x47),
                pitch_line: Color32::from_rgba_unmultiplied(251, 247, 239, 153),
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Nessuna,
    Con,
    Trans,
    Senza,
}

#[allow(clippy::missing_const_for_fn)]
impl Phase {
    fn label(self) -> &'static str {
        match self {
            Self::Nessuna => "Nessuna",
            Self::Con => "Con palla",
            Self::Trans => "Transizione",
            Self::Senza => "Senza palla",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Scenario {
    Settimana,
    Partita,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PostItKind {
    Giorno,
    Posta,
    Prossima,
    Rosa,
    Formazione,
    Istruzioni,
    Perche,
    Stats,
    Cronaca,
    Tiri,
    Eventi,
    Calore,
    Campo,
}

#[allow(clippy::missing_const_for_fn)]
impl PostItKind {
    const ALL: [Self; 13] = [
        Self::Giorno,
        Self::Posta,
        Self::Prossima,
        Self::Rosa,
        Self::Formazione,
        Self::Istruzioni,
        Self::Perche,
        Self::Stats,
        Self::Cronaca,
        Self::Tiri,
        Self::Eventi,
        Self::Calore,
        Self::Campo,
    ];

    fn title(self) -> &'static str {
        match self {
            Self::Giorno => "Giornata",
            Self::Posta => "Posta",
            Self::Prossima => "Prossima partita",
            Self::Rosa => "Rosa",
            Self::Formazione => "Energia",
            Self::Istruzioni => "Istruzioni rapide",
            Self::Perche => "Perché",
            Self::Stats => "Statistiche",
            Self::Cronaca => "Cronaca",
            Self::Tiri => "Mappa dei tiri",
            Self::Eventi => "Eventi",
            Self::Calore => "Mappa di calore",
            Self::Campo => "Campo",
        }
    }

    fn group(self) -> &'static str {
        match self {
            Self::Giorno | Self::Posta | Self::Prossima => "Scrivania",
            Self::Rosa | Self::Formazione => "Squadra",
            Self::Istruzioni | Self::Perche => "Tattica",
            _ => "Partita",
        }
    }

    fn band(self) -> Color32 {
        match self {
            Self::Giorno | Self::Posta | Self::Prossima | Self::Cronaca | Self::Calore => {
                Color32::from_rgb(0xF0, 0xB0, 0x4A)
            }
            Self::Rosa => Color32::from_rgb(0xE9, 0xEC, 0xEF),
            Self::Formazione | Self::Istruzioni => Color32::from_rgb(0x7F, 0xC7, 0x9A),
            Self::Stats | Self::Tiri | Self::Campo => Color32::from_rgb(0x6E, 0xA0, 0xFF),
            Self::Perche => Color32::from_rgb(0xF0, 0x8A, 0x5D),
            Self::Eventi => Color32::from_rgb(0xB4, 0x9C, 0xFF),
        }
    }

    fn tint(self, theme: ThemeId) -> Color32 {
        match theme {
            ThemeId::Lavagna => match self {
                Self::Rosa => Color32::from_rgb(0xFF, 0xFB, 0xEA),
                Self::Giorno | Self::Cronaca | Self::Posta => Color32::from_rgb(0xFF, 0xF3, 0xB0),
                Self::Istruzioni | Self::Formazione => Color32::from_rgb(0xE2, 0xF4, 0xD6),
                Self::Stats | Self::Tiri | Self::Campo => Color32::from_rgb(0xD7, 0xEC, 0xFF),
                Self::Perche | Self::Prossima => Color32::from_rgb(0xFF, 0xD8, 0xC2),
                Self::Eventi => Color32::from_rgb(0xED, 0xE3, 0xFF),
                Self::Calore => Color32::from_rgb(0xFF, 0xE6, 0xB3),
            },
            ThemeId::Notte => Color32::from_rgb(0x1D, 0x24, 0x2B),
            ThemeId::Carta => match self {
                Self::Rosa => Color32::from_rgb(0xFB, 0xF5, 0xE6),
                Self::Giorno | Self::Cronaca | Self::Posta => Color32::from_rgb(0xF6, 0xE7, 0xB4),
                Self::Istruzioni | Self::Formazione => Color32::from_rgb(0xDC, 0xE7, 0xD2),
                Self::Stats | Self::Tiri | Self::Campo => Color32::from_rgb(0xDC, 0xE6, 0xEA),
                Self::Perche | Self::Prossima => Color32::from_rgb(0xF1, 0xD8, 0xC6),
                Self::Eventi => Color32::from_rgb(0xE5, 0xDE, 0xEC),
                Self::Calore => Color32::from_rgb(0xF3, 0xE1, 0xBE),
            },
        }
    }

    fn default_size(self) -> Vec2 {
        match self {
            Self::Rosa => Vec2::new(560.0, 480.0),
            Self::Giorno => Vec2::new(360.0, 300.0),
            Self::Istruzioni => Vec2::new(360.0, 340.0),
            Self::Cronaca => Vec2::new(380.0, 460.0),
            Self::Stats => Vec2::new(520.0, 300.0),
            Self::Perche => Vec2::new(340.0, 220.0),
            Self::Formazione => Vec2::new(500.0, 300.0),
            Self::Tiri => Vec2::new(360.0, 480.0),
            Self::Eventi => Vec2::new(420.0, 460.0),
            Self::Calore => Vec2::new(540.0, 360.0),
            Self::Posta => Vec2::new(420.0, 340.0),
            Self::Prossima => Vec2::new(360.0, 240.0),
            Self::Campo => Vec2::new(420.0, 280.0),
        }
    }

    fn default_pos(self) -> Pos2 {
        match self {
            Self::Rosa => Pos2::new(90.0, 90.0),
            Self::Giorno => Pos2::new(680.0, 90.0),
            Self::Posta => Pos2::new(680.0, 410.0),
            Self::Prossima => Pos2::new(1060.0, 90.0),
            Self::Formazione => Pos2::new(90.0, 590.0),
            Self::Istruzioni => Pos2::new(1060.0, 350.0),
            Self::Perche => Pos2::new(1060.0, 710.0),
            Self::Stats => Pos2::new(620.0, 590.0),
            Self::Cronaca => Pos2::new(1160.0, 90.0),
            Self::Tiri => Pos2::new(620.0, 410.0),
            Self::Eventi => Pos2::new(760.0, 250.0),
            Self::Calore => Pos2::new(620.0, 500.0),
            Self::Campo => Pos2::new(760.0, 380.0),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum DockKind {
    Split,
    Tabs,
}

#[derive(Clone)]
struct DockGroup {
    id: u8,
    kind: DockKind,
    members: Vec<usize>, // indices into postits
    pos: Pos2,
    widths: Vec<f32>,
    height: f32,
    active: usize,
    open: bool,
    z: u8,
}

#[derive(Clone, Copy)]
struct PostItState {
    kind: PostItKind,
    open: bool,
    minimized: bool,
    pos: Pos2,
    size: Vec2,
    z: u8,
    dock: Option<u8>,
    big: bool,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct NoteStyle {
    paper: Color32,
    ink: Color32,
    muted: Color32,
    line: Color32,
    accent: Color32,
    band: Option<Color32>,
    radius: CornerRadius,
    border: Option<Stroke>,
}

fn dock_relation(a: Rect, b: Rect) -> Option<DockKind> {
    let center = a.center();
    let in_center = center.x > b.left() + b.width() * 0.25
        && center.x < b.right() - b.width() * 0.25
        && center.y > b.top() + b.height() * 0.2
        && center.y < b.bottom() - b.height() * 0.2;
    if in_center {
        return Some(DockKind::Tabs);
    }
    let v_overlap = a.bottom().min(b.bottom()) - a.top().max(b.top());
    if v_overlap > a.height().min(b.height()) * 0.4 {
        if (a.left() - b.right()).abs() < 44.0 || (a.right() - b.left()).abs() < 44.0 {
            return Some(DockKind::Split);
        }
    }
    None
}
fn note_style(theme: ThemeId, kind: PostItKind) -> NoteStyle {
    let band = Some(kind.band());
    match theme {
        ThemeId::Lavagna => NoteStyle {
            paper: kind.tint(theme),
            ink: Color32::from_rgb(0x1B, 0x1F, 0x23),
            muted: Color32::from_rgb(0x4B, 0x55, 0x63),
            line: Color32::from_rgba_unmultiplied(20, 20, 20, 30),
            accent: Color32::from_rgb(0xB4, 0x53, 0x09),
            band: None,
            radius: CornerRadius {
                nw: 3,
                ne: 3,
                sw: 10,
                se: 3,
            },
            border: None,
        },
        ThemeId::Notte => NoteStyle {
            paper: Color32::from_rgb(0x1D, 0x24, 0x2B),
            ink: Color32::from_rgb(0xE9, 0xEC, 0xEF),
            muted: Color32::from_rgb(0xA3, 0xAD, 0xB8),
            line: Color32::from_rgb(0x2A, 0x32, 0x3B),
            accent: Color32::from_rgb(0xF0, 0xB0, 0x4A),
            band,
            radius: CornerRadius::same(10),
            border: Some(Stroke::new(1.0, Color32::from_rgb(0x2A, 0x32, 0x3B))),
        },
        ThemeId::Carta => NoteStyle {
            paper: kind.tint(theme),
            ink: Color32::from_rgb(0x1C, 0x26, 0x22),
            muted: Color32::from_rgb(0x5B, 0x62, 0x5C),
            line: Color32::from_rgba_unmultiplied(28, 38, 34, 35),
            accent: Color32::from_rgb(0x1F, 0x5A, 0x45),
            band: None,
            radius: CornerRadius {
                nw: 2,
                ne: 2,
                sw: 8,
                se: 2,
            },
            border: Some(Stroke::new(
                1.0,
                Color32::from_rgba_unmultiplied(28, 38, 34, 30),
            )),
        },
    }
}

fn note_ink(theme: ThemeId) -> (Color32, Color32, Color32) {
    let ns = note_style(theme, PostItKind::Giorno);
    (ns.ink, ns.muted, ns.line)
}

fn ink_on(c: Color32) -> Color32 {
    let lum = 0.0722f32.mul_add(
        f32::from(c.b()),
        0.2126f32.mul_add(f32::from(c.r()), 0.7152 * f32::from(c.g())),
    );
    if lum > 150.0 {
        Color32::from_rgb(0x14, 0x14, 0x14)
    } else {
        Color32::WHITE
    }
}

fn palette(th: BoardTheme, home: Color32) -> Palette {
    let away = Color32::from_rgb(0x1D, 0x4E, 0xD8);
    Palette {
        grass: th.pitch,
        stripe: th.pitch.gamma_multiply(0.8),
        chalk: th.pitch_line,
        home,
        home_ink: ink_on(home),
        away,
        away_ink: ink_on(away),
        referee: th.text,
        ball: Color32::WHITE,
        time: th.text,
    }
}

fn project_mark(ui: &egui::Ui, rect: Rect, fg: Color32) {
    let p = ui.painter_at(rect);
    let s = rect.width().min(rect.height());
    let c = rect.center();
    let r = s * 0.38;
    let st = Stroke::new((s * 0.07).max(1.5), fg);
    p.circle_stroke(c, r, st);
    p.line_segment(
        [
            Pos2::new(c.x, r.mul_add(-0.65, c.y)),
            Pos2::new(c.x, r.mul_add(0.65, c.y)),
        ],
        st,
    );
    p.circle_stroke(c, r * 0.45, st);
    p.circle_filled(c, (s * 0.045).max(1.5), fg);
}

#[allow(dead_code)]
pub struct GuiApp {
    career: Option<Career>,
    selected_day: i32,
    view_day: i32,
    phase: Phase,
    scenario: Scenario,
    theme: ThemeId,
    instructions: [bool; 6],
    trans_persa: usize,
    trans_recup: usize,
    postits: Vec<PostItState>,
    docks: Vec<DockGroup>,
    z_counter: u8,
    dock_counter: u8,
    bar_pinned: bool,
    bar_hover: bool,
    start_open: bool,
    team_idx: usize,
    view: MatchView,
    formation: [[(f32, f32); 11]; 3], // Con, Trans, Senza
    drag_player: Option<usize>,
}

#[allow(clippy::missing_const_for_fn)]
impl GuiApp {
    fn new_career() -> Option<Career> {
        Career::new(Pack::v1(), 42, 0).ok()
    }

    fn team_color(&self) -> Color32 {
        match self.team_idx % 3 {
            0 => self
                .career
                .as_ref()
                .map_or(Color32::from_rgb(0x6E, 0x1E, 0x2A), |c| {
                    c.clubs().get(c.club() as usize).map_or(
                        Color32::from_rgb(0x6E, 0x1E, 0x2A),
                        |cl| {
                            Color32::from_rgb(cl.colours[0][0], cl.colours[0][1], cl.colours[0][2])
                        },
                    )
                }),
            1 => Color32::from_rgb(0x17, 0x3F, 0x73),
            _ => Color32::from_rgb(0x17, 0x4A, 0x31),
        }
    }
    #[allow(dead_code)]
    fn team_secondary(&self) -> Color32 {
        match self.team_idx % 3 {
            0 => Color32::from_rgb(0xF2, 0xE6, 0xD0),
            1 => Color32::from_rgb(0xFF, 0xFF, 0xFF),
            _ => Color32::from_rgb(0xF2, 0xC2, 0x30),
        }
    }

    fn advance(&mut self) {
        if let Some(c) = self.career.as_mut() {
            let _ = c.advance();
        }
        self.selected_day = self.current_day();
        self.view_day = self.selected_day;
        self.scenario = Scenario::Settimana;
    }

    fn go_match(&mut self) {
        self.scenario = Scenario::Partita;
        self.view_day = 5;
        self.selected_day = 5;
        self.view.rewind();
        self.phase = Phase::Nessuna;
        self.open_kind(PostItKind::Istruzioni);
        self.open_kind(PostItKind::Cronaca);
        self.open_kind(PostItKind::Stats);
    }

    fn open_kind(&mut self, kind: PostItKind) {
        self.z_counter = self.z_counter.saturating_add(1);
        let z = self.z_counter;
        if let Some(p) = self.postits.iter_mut().find(|p| p.kind == kind) {
            p.open = true;
            p.minimized = false;
            p.z = z;
        }
    }

    fn current_day(&self) -> i32 {
        self.career
            .as_ref()
            .map_or(0, |c| i32::try_from(c.day()).unwrap_or(i32::MAX))
    }

    /// Clock and score shown next to the playback buttons.
    fn scoreboard(&self) -> String {
        let (home, away) = self.view.score();
        format!("{} · {home} – {away}", self.view.clock())
    }
}

impl Default for GuiApp {
    fn default() -> Self {
        let career = Self::new_career();
        let selected_day = career
            .as_ref()
            .map_or(0, |c| i32::try_from(c.day()).unwrap_or(i32::MAX));
        let open = [PostItKind::Rosa, PostItKind::Giorno, PostItKind::Istruzioni];
        let mut postits = Vec::new();
        let mut z = 1u8;
        for k in PostItKind::ALL {
            postits.push(PostItState {
                kind: k,
                open: open.contains(&k),
                minimized: false,
                pos: k.default_pos(),
                size: k.default_size(),
                z,
                dock: None,
                big: true,
            });
            z = z.saturating_add(1);
        }
        Self {
            career,
            selected_day,
            view_day: selected_day,
            phase: Phase::Con,
            scenario: Scenario::Settimana,
            theme: ThemeId::Lavagna,
            instructions: [false; 6],
            trans_persa: 0,
            trans_recup: 0,
            postits,
            docks: Vec::new(),
            z_counter: z,
            dock_counter: 1,
            bar_pinned: true,
            bar_hover: false,
            start_open: false,
            team_idx: 0,
            view: MatchView::new(42).expect("the seed's fixture"),
            formation: [
                Self::formation_default(Phase::Con),
                Self::formation_default(Phase::Trans),
                Self::formation_default(Phase::Senza),
            ],
            drag_player: None,
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(BoardTheme::get(self.theme).bg))
            .show(ctx, |ui| self.view_lavagna(ui));
    }
}

impl GuiApp {
    fn view_lavagna(&mut self, ui: &mut egui::Ui) {
        if self.scenario == Scenario::Partita {
            let dt = ui.input(|i| i.stable_dt);
            self.view.advance(dt);
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(100));
        }
        let rect = ui.available_rect_before_wrap();
        let th = BoardTheme::get(self.theme);
        let team = self.team_color();
        let fg = ink_on(team);
        ui.painter_at(rect)
            .rect_filled(rect, CornerRadius::ZERO, th.bg);

        let top_h = 64.0_f32;
        let hover_pos = ui.input(|i| i.pointer.hover_pos());
        let in_sensor = hover_pos
            .is_some_and(|p| p.y <= rect.min.y + 10.0 && p.x >= rect.min.x && p.x <= rect.max.x);
        let in_bar = hover_pos.is_some_and(|p| {
            let bar_rect = Rect::from_min_size(
                Pos2::new(rect.min.x, rect.min.y),
                Vec2::new(rect.width(), top_h),
            );
            bar_rect.contains(p)
        });
        if self.bar_pinned {
            self.bar_hover = true;
        } else {
            self.bar_hover = in_sensor || in_bar;
        }
        let bar_shown = self.bar_pinned || self.bar_hover;
        let bar_y = if bar_shown {
            rect.min.y
        } else {
            rect.min.y - top_h + 6.0
        };
        let top = Rect::from_min_size(Pos2::new(rect.min.x, bar_y), Vec2::new(rect.width(), top_h));
        if bar_shown {
            self.paint_top_bar(ui, top, th, team, fg);
        }

        let board_top = if self.bar_pinned {
            rect.min.y + top_h
        } else {
            rect.min.y
        };
        let deck_w = 170.0_f32;
        let deck = Rect::from_min_max(
            Pos2::new(rect.min.x, board_top + 12.0),
            Pos2::new(rect.min.x + deck_w, rect.max.y - 12.0),
        );
        let board = Rect::from_min_max(Pos2::new(deck.max.x, board_top), rect.max);

        self.paint_deck(ui, deck, th);
        self.paint_pitch(ui, board, th);
        self.paint_postits(ui, board, th);

        if self.scenario == Scenario::Partita {
            egui::Area::new(Id::new("partita-playback"))
                .anchor(Align2::CENTER_BOTTOM, Vec2::new(0.0, -12.0))
                .show(ui.ctx(), |ui| {
                    egui::Frame::new()
                        .fill(th.bar)
                        .corner_radius(8.0)
                        .inner_margin(6.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(self.scoreboard());
                                let play = if self.view.is_playing() {
                                    "❚❚"
                                } else {
                                    "▶"
                                };
                                if ui.button(play).clicked() {
                                    self.view.toggle();
                                }
                                if ui.button("↺").clicked() {
                                    self.view.rewind();
                                }
                                if ui
                                    .button(format!("x{}", self.view.speed() as i32))
                                    .clicked()
                                {
                                    self.view.cycle_speed();
                                }
                                let follow = if self.view.is_following() {
                                    "segui palla"
                                } else {
                                    "camera libera"
                                };
                                // A drag on the board frees the camera; this gives it back.
                                if ui.button(follow).clicked() {
                                    self.view.follow_ball();
                                }
                                let mut tilt = self.view.tilt();
                                if ui
                                    .add(egui::Slider::new(&mut tilt, 0.0..=75.0).text("tilt"))
                                    .changed()
                                {
                                    self.view.set_tilt(tilt);
                                }
                            });
                        });
                });
        }
    }

    #[allow(dead_code)]
    fn bar_team(&self) -> (Color32, Color32, Color32) {
        let pcol = self.team_color();
        let scol = self.team_secondary();
        (pcol, scol, scol.gamma_multiply(0.7))
    }
    const fn day_defs() -> [(&'static str, &'static str, &'static str); 7] {
        [
            ("Lun", "Lunedì", "Allenamento"),
            ("Mar", "Martedì", "Allenamento"),
            ("Mer", "Mercoledì", "Rapporto"),
            ("Gio", "Giovedì", "Video"),
            ("Ven", "Venerdì", "Rifinitura"),
            ("Sab", "Sabato", "PARTITA"),
            ("Dom", "Domenica", "Riposo"),
        ]
    }
    #[allow(dead_code)]
    fn agenda_for(view: i32) -> Vec<(&'static str, &'static str, &'static str, Color32)> {
        let k = |kind: &str| match kind {
            "all" => Color32::from_rgb(0x3E, 0x9B, 0x6B),
            "par" => Color32::from_rgb(0xD9, 0xA2, 0x4A),
            "staff" => Color32::from_rgb(0x5B, 0x8D, 0xEF),
            "media" => Color32::from_rgb(0xA7, 0x7B, 0xD6),
            _ => Color32::from_rgb(0x9C, 0xA3, 0xAF),
        };
        let ag: [&[(&str, &str, &str, &str)]; 7] = [
            &[
                (
                    "09:30",
                    "Allenamento · tecnica",
                    "Possesso a due tocchi, torello 5 contro 2.",
                    "all",
                ),
                (
                    "11:30",
                    "Palestra",
                    "Recupero per chi ha giocato più di 70 minuti.",
                    "rip",
                ),
                (
                    "15:00",
                    "Riunione dello staff",
                    "Come è andata la partita di sabato.",
                    "staff",
                ),
            ],
            &[
                (
                    "10:00",
                    "Allenamento · tattica con palla",
                    "Uscita a tre contro la pressione alta, terzini alti.",
                    "all",
                ),
                (
                    "14:30",
                    "Video",
                    "La nostra ultima partita: i gol subiti da spazzata.",
                    "staff",
                ),
                (
                    "16:00",
                    "Colloquio con Home 6",
                    "Vuole più minuti.",
                    "staff",
                ),
            ],
            &[
                (
                    "10:00",
                    "Allenamento · fisico",
                    "Scatti brevi, cambi di direzione.",
                    "all",
                ),
                (
                    "12:00",
                    "Rapporto dell’osservatore",
                    "Bellini su un giovane: fascia ampia sul passaggio.",
                    "staff",
                ),
            ],
            &[
                (
                    "10:00",
                    "Video · Real Valcastello",
                    "Blocco basso, ampiezza stretta: la palla va mossa da una fascia all’altra.",
                    "staff",
                ),
                (
                    "11:00",
                    "Allenamento · tattica senza palla",
                    "Pressione sul loro terzino, linea a centrocampo.",
                    "all",
                ),
                (
                    "15:00",
                    "Staff medico",
                    "Home 6 è stanco: mercoledì 11,4 km.",
                    "staff",
                ),
            ],
            &[
                (
                    "10:30",
                    "Rifinitura · palle inattive",
                    "Angoli a uscire, punizioni sul secondo palo.",
                    "all",
                ),
                (
                    "12:30",
                    "Conferenza stampa",
                    "Le domande sui gol subiti.",
                    "media",
                ),
                ("18:00", "Convocazioni", "Scegli i 20 per sabato.", "staff"),
            ],
            &[
                ("12:00", "Pranzo della squadra", "", "rip"),
                (
                    "13:30",
                    "Riunione tecnica",
                    "Ripasso della tattica con e senza palla.",
                    "staff",
                ),
                (
                    "15:00",
                    "PARTITA · Home – Real Valcastello",
                    "In casa. Si scende in campo dalla panchina.",
                    "par",
                ),
            ],
            &[
                ("11:00", "Defaticante", "Per chi ha giocato sabato.", "all"),
                ("", "Riposo", "Il resto della squadra è libero.", "rip"),
            ],
        ];
        let i = (view.clamp(0, 6)) as usize;
        ag[i]
            .iter()
            .map(|(a, b, c, d)| (*a, *b, *c, k(d)))
            .collect()
    }

    fn paint_top_bar(
        &mut self,
        ui: &mut egui::Ui,
        rect: Rect,
        th: BoardTheme,
        team: Color32,
        fg: Color32,
    ) {
        let p = ui.painter_at(rect);
        p.rect_filled(rect, CornerRadius::ZERO, team.gamma_multiply(0.97));
        p.line_segment(
            [rect.left_bottom(), rect.right_bottom()],
            Stroke::new(1.0, th.line),
        );

        let mark = Rect::from_min_size(
            rect.min + Vec2::new(10.0, (rect.height() - 40.0) / 2.0),
            Vec2::splat(40.0),
        );
        let mark_resp = ui.interact(mark, ui.id().with("mark"), Sense::click());
        ui.painter_at(mark).rect_filled(
            mark,
            CornerRadius::same(8),
            if self.start_open {
                fg.gamma_multiply(0.12)
            } else {
                Color32::TRANSPARENT
            },
        );
        project_mark(ui, mark, th.accent);
        if mark_resp.clicked() {
            self.start_open = !self.start_open;
        }

        let title = self.career.as_ref().map_or_else(
            || "Calcio Vero".into(),
            |c| format!("{} · giornata {}", c.coach_name(c.club()), c.day()),
        );
        p.text(
            Pos2::new(mark.max.x + 12.0, rect.center().y - 8.0),
            Align2::LEFT_CENTER,
            title,
            FontId::proportional(14.0),
            fg,
        );
        p.text(
            Pos2::new(mark.max.x + 12.0, rect.center().y + 10.0),
            Align2::LEFT_CENTER,
            "Lavagna tattica",
            FontId::proportional(11.0),
            fg.gamma_multiply(0.75),
        );

        // day strip
        let day_defs = Self::day_defs();
        let day = self.current_day();
        let view = self.view_day.clamp(0, 6);
        let strip_x = rect.min.x + 340.0;
        for i in 0..7i32 {
            let def = day_defs[usize::try_from(i).unwrap_or(0)];
            let is_today = i == day.clamp(0, 6);
            let viewing = i == view;
            let r = Rect::from_min_size(
                Pos2::new(strip_x + i as f32 * 82.0, rect.center().y - 23.0),
                Vec2::new(78.0, 46.0),
            );
            let resp = ui.interact(r, ui.id().with(("day", i)), Sense::click());
            if resp.clicked() {
                self.view_day = i;
                self.selected_day = day - day.clamp(0, 6) + i;
                if i == 5 {
                    self.go_match();
                } else {
                    self.scenario = Scenario::Settimana;
                    self.open_kind(PostItKind::Giorno);
                }
            }
            let border = if is_today {
                Stroke::new(2.0, th.accent)
            } else {
                Stroke::new(1.0, if viewing { fg } else { th.line })
            };
            let bg = if viewing {
                fg.gamma_multiply(0.14)
            } else {
                Color32::TRANSPARENT
            };
            let tc = if i == 5 { th.accent } else { fg };
            let alpha = if i < day.clamp(0, 6) { 0.55 } else { 1.0 };
            ui.painter_at(r).rect_filled(r, CornerRadius::same(8), bg);
            ui.painter_at(r)
                .rect_stroke(r, CornerRadius::same(8), border, StrokeKind::Middle);
            ui.painter_at(r).text(
                Pos2::new(r.center().x, r.center().y - 7.0),
                Align2::CENTER_CENTER,
                def.0,
                FontId::proportional(10.0),
                tc.gamma_multiply(alpha),
            );
            ui.painter_at(r).text(
                Pos2::new(r.center().x, r.center().y + 8.0),
                Align2::CENTER_CENTER,
                if is_today { "Oggi" } else { def.2 },
                FontId::proportional(11.0),
                tc.gamma_multiply(alpha),
            );
        }

        // phase chips
        let phases: Vec<Phase> = if self.scenario == Scenario::Partita {
            vec![Phase::Nessuna, Phase::Con, Phase::Trans, Phase::Senza]
        } else {
            vec![Phase::Con, Phase::Trans, Phase::Senza]
        };
        let pw = 108.0_f32;
        let px = rect.max.x - pw * phases.len() as f32 - 210.0;
        for (i, ph) in phases.iter().enumerate() {
            let r = Rect::from_min_size(
                Pos2::new(px + i as f32 * pw, rect.center().y - 15.0),
                Vec2::new(pw - 4.0, 30.0),
            );
            let resp = ui.interact(r, ui.id().with(("ph", i)), Sense::click());
            if resp.clicked() {
                self.phase = *ph;
            }
            let active = self.phase == *ph;
            let bg = if active {
                th.accent
            } else {
                fg.gamma_multiply(0.12)
            };
            let tc = if active { ink_on(th.accent) } else { fg };
            ui.painter_at(r).rect_filled(r, CornerRadius::same(15), bg);
            ui.painter_at(r).text(
                r.center(),
                Align2::CENTER_CENTER,
                ph.label(),
                FontId::proportional(12.0),
                tc,
            );
        }

        // pin and advance
        let pin = Rect::from_min_size(
            Pos2::new(rect.max.x - 146.0, rect.center().y - 16.0),
            Vec2::new(36.0, 32.0),
        );
        let pin_r = ui.interact(pin, ui.id().with("pin"), Sense::click());
        ui.painter_at(pin).rect_filled(
            pin,
            CornerRadius::same(8),
            if self.bar_pinned {
                fg.gamma_multiply(0.18)
            } else {
                fg.gamma_multiply(0.08)
            },
        );
        ui.painter_at(pin).rect_stroke(
            pin,
            CornerRadius::same(8),
            Stroke::new(1.0, if self.bar_pinned { th.accent } else { th.line }),
            StrokeKind::Middle,
        );
        ui.painter_at(pin).text(
            pin.center(),
            Align2::CENTER_CENTER,
            if self.bar_pinned { "📌" } else { "▁" },
            FontId::proportional(13.0),
            fg,
        );
        if pin_r.clicked() {
            self.bar_pinned = !self.bar_pinned;
            self.bar_hover = false;
        }

        let adv = Rect::from_min_size(
            Pos2::new(rect.max.x - 52.0, rect.center().y - 16.0),
            Vec2::new(42.0, 32.0),
        );
        let adv_r = ui.interact(adv, ui.id().with("adv"), Sense::click());
        ui.painter_at(adv)
            .rect_filled(adv, CornerRadius::same(8), fg.gamma_multiply(0.14));
        ui.painter_at(adv).text(
            adv.center(),
            Align2::CENTER_CENTER,
            ">",
            FontId::proportional(14.0),
            fg,
        );
        if adv_r.clicked() {
            self.advance();
        }

        if self.start_open {
            self.paint_start_menu(ui, rect, th, team, fg);
        }
    }

    #[allow(clippy::unused_self)]
    fn paint_start_menu(
        &mut self,
        ui: &mut egui::Ui,
        bar: Rect,
        th: BoardTheme,
        team: Color32,
        fg: Color32,
    ) {
        let panel = Rect::from_min_size(
            Pos2::new(bar.min.x + 8.0, bar.max.y + 4.0),
            Vec2::new(280.0, 320.0),
        );
        Area::new(Id::new("start-menu"))
            .order(Order::Foreground)
            .fixed_pos(panel.min)
            .show(ui.ctx(), |ui| {
                egui::Frame::NONE
                    .fill(team.gamma_multiply(0.98))
                    .stroke(Stroke::new(1.0, th.line))
                    .corner_radius(CornerRadius::same(10))
                    .shadow(egui::epaint::Shadow {
                        offset: [0, 12],
                        blur: 28,
                        spread: 0,
                        color: Color32::from_rgba_unmultiplied(0, 0, 0, 70),
                    })
                    .show(ui, |ui| {
                        ui.set_min_size(panel.size());
                        ui.set_max_size(panel.size());
                        ui.add_space(8.0);
                        for (label, sub) in [
                            ("Salva la partita", "Il salvataggio di questa carriera"),
                            ("Carica una partita", "Un salvataggio precedente"),
                            ("Nuova carriera", "Scegli la squadra e il campionato"),
                            ("Impostazioni", "Lingua, audio, schermo, velocità"),
                            ("Guida", "Come si gioca"),
                            ("Esci dal gioco", "Chiede se salvare prima"),
                        ] {
                            let (r, _) = ui.allocate_exact_size(
                                Vec2::new(panel.width() - 16.0, 42.0),
                                Sense::click(),
                            );
                            let r = r.translate(Vec2::new(8.0, 0.0));
                            let rr = ui.interact(r, ui.id().with(label), Sense::click());
                            if rr.hovered() {
                                ui.painter().rect_filled(
                                    r,
                                    CornerRadius::same(6),
                                    fg.gamma_multiply(0.10),
                                );
                            }
                            ui.painter().text(
                                Pos2::new(r.min.x + 8.0, r.center().y - 6.0),
                                Align2::LEFT_CENTER,
                                label,
                                FontId::proportional(13.0),
                                fg,
                            );
                            ui.painter().text(
                                Pos2::new(r.min.x + 8.0, r.center().y + 9.0),
                                Align2::LEFT_CENTER,
                                sub,
                                FontId::proportional(10.5),
                                fg.gamma_multiply(0.65),
                            );
                            if rr.clicked() {
                                self.start_open = false;
                            }
                        }
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            ui.add_space(8.0);
                            ui.label(
                                RichText::new("Lavagna:")
                                    .color(fg.gamma_multiply(0.75))
                                    .size(11.0),
                            );
                            for (id, label) in [
                                (ThemeId::Lavagna, "Lavagna"),
                                (ThemeId::Notte, "Notte"),
                                (ThemeId::Carta, "Carta"),
                            ] {
                                let sel = self.theme == id;
                                let btn = egui::Button::new(
                                    RichText::new(label).size(11.0).color(if sel {
                                        Color32::BLACK
                                    } else {
                                        fg
                                    }),
                                )
                                .fill(if sel {
                                    th.accent
                                } else {
                                    fg.gamma_multiply(0.10)
                                });
                                if ui.add(btn).clicked() {
                                    self.theme = id;
                                }
                            }
                        });
                    });
            });
    }

    fn paint_deck(&mut self, ui: &mut egui::Ui, rect: Rect, th: BoardTheme) {
        let mut y = rect.min.y + 4.0;
        let mut last_group = "";
        let mut clicks: Vec<usize> = Vec::new();
        let mut reopen_dock: Option<u8> = None;
        for i in 0..self.postits.len() {
            let pi = self.postits[i];
            let hidden_by_dock = pi.dock.is_some()
                && self
                    .docks
                    .iter()
                    .any(|d| d.id == pi.dock.unwrap() && d.open);
            if (pi.open && !pi.minimized) || hidden_by_dock {
                continue;
            }
            if pi.kind.group() != last_group {
                last_group = pi.kind.group();
                ui.painter_at(rect).text(
                    Pos2::new(rect.min.x + 12.0, y + 8.0),
                    Align2::LEFT_CENTER,
                    last_group.to_uppercase(),
                    FontId::proportional(9.5),
                    th.accent,
                );
                y += 22.0;
            }
            let w = (rect.width() - 12.0).max(80.0);
            let r = Rect::from_min_size(Pos2::new(rect.min.x, y), Vec2::new(w, 36.0));
            let resp = ui.interact(r, ui.id().with(("deck", i)), Sense::click());
            let accent = pi.kind.band();
            let bg = if resp.hovered() {
                pi.kind.tint(self.theme).gamma_multiply(0.95)
            } else {
                pi.kind.tint(self.theme)
            };
            let (ink, _, _) = note_ink(self.theme);
            ui.painter_at(r).rect_filled(
                r,
                CornerRadius {
                    nw: 0,
                    ne: 8,
                    sw: 0,
                    se: 8,
                },
                bg,
            );
            ui.painter_at(r).rect_filled(
                Rect::from_min_max(r.min, Pos2::new(r.min.x + 4.0, r.max.y)),
                CornerRadius::ZERO,
                accent,
            );
            ui.painter_at(r).text(
                Pos2::new(r.min.x + 12.0, r.center().y),
                Align2::LEFT_CENTER,
                pi.kind.title(),
                FontId::proportional(12.0),
                ink,
            );
            if resp.clicked() {
                if let Some(did) = pi.dock {
                    reopen_dock = Some(did);
                } else {
                    clicks.push(i);
                }
            }
            y += 42.0;
        }
        let closed_docks: Vec<u8> = self
            .docks
            .iter()
            .filter(|d| !d.open)
            .map(|d| d.id)
            .collect();
        if !closed_docks.is_empty() {
            ui.painter_at(rect).text(
                Pos2::new(rect.min.x + 12.0, y + 8.0),
                Align2::LEFT_CENTER,
                "GRUPPI",
                FontId::proportional(9.5),
                th.accent,
            );
            y += 22.0;
            for did in closed_docks {
                let label = self.dock_label(did);
                let w = (rect.width() - 12.0).max(80.0);
                let r = Rect::from_min_size(Pos2::new(rect.min.x, y), Vec2::new(w, 36.0));
                let resp = ui.interact(r, ui.id().with(("dockdeck", did)), Sense::click());
                let bg = th.panel2;
                ui.painter_at(r).rect_filled(
                    r,
                    CornerRadius {
                        nw: 0,
                        ne: 8,
                        sw: 0,
                        se: 8,
                    },
                    bg,
                );
                ui.painter_at(r).rect_filled(
                    Rect::from_min_max(r.min, Pos2::new(r.min.x + 4.0, r.max.y)),
                    CornerRadius::ZERO,
                    th.accent,
                );
                ui.painter_at(r).text(
                    Pos2::new(r.min.x + 12.0, r.center().y),
                    Align2::LEFT_CENTER,
                    label,
                    FontId::proportional(11.0),
                    th.text,
                );
                if resp.clicked() {
                    reopen_dock = Some(did);
                }
                y += 42.0;
            }
        }
        if let Some(did) = reopen_dock {
            if let Some(d) = self.docks.iter_mut().find(|d| d.id == did) {
                d.open = true;
                self.z_counter = self.z_counter.wrapping_add(1);
                d.z = self.z_counter;
            }
        }
        for i in clicks {
            self.z_counter = self.z_counter.wrapping_add(1);
            self.postits[i].open = true;
            self.postits[i].minimized = false;
            self.postits[i].z = self.z_counter;
        }
    }

    fn paint_postits(&mut self, ui: &egui::Ui, board: Rect, th: BoardTheme) {
        let mut open: Vec<usize> = self
            .postits
            .iter()
            .enumerate()
            .filter(|(_, p)| p.open && !p.minimized && p.dock.is_none())
            .map(|(i, _)| i)
            .collect();
        open.sort_by_key(|&i| self.postits[i].z);
        let topz = open.last().map(|&i| self.postits[i].z).unwrap_or(0);
        let mut to_front: Option<usize> = None;
        let mut to_close: Option<usize> = None;
        let mut to_min: Option<usize> = None;
        let mut drop_target: Option<(usize, DockKind)> = None;
        let mut drop_hint: Option<Rect> = None;

        for &idx in &open {
            let pi = self.postits[idx];
            let id = Id::new(("pi", pi.kind as u8));
            let ns = note_style(self.theme, pi.kind);
            let is_top = pi.z == topz;
            let pos0 = board.min + pi.pos.to_vec2();
            let resp = Area::new(id)
                .order(Order::Middle)
                .current_pos(pos0)
                .movable(true)
                .constrain_to(board)
                .show(ui.ctx(), |ui| {
                    let shadow_a = if is_top { 90 } else { 55 };
                    let mut frame = egui::Frame::NONE
                        .fill(ns.paper)
                        .corner_radius(ns.radius)
                        .shadow(egui::epaint::Shadow {
                            offset: [0, 8],
                            blur: 22,
                            spread: 0,
                            color: Color32::from_rgba_unmultiplied(0, 0, 0, shadow_a),
                        });
                    if let Some(b) = ns.border {
                        frame = frame.stroke(b);
                    }
                    frame.show(ui, |ui| {
                        ui.set_min_size(pi.size);
                        ui.set_max_size(pi.size);
                        let rect = ui.max_rect();
                        let band_h = if ns.band.is_some() { 3.0 } else { 0.0 };
                        if let Some(bandc) = ns.band {
                            ui.painter().rect_filled(
                                Rect::from_min_size(rect.min, Vec2::new(rect.width(), band_h)),
                                CornerRadius {
                                    nw: ns.radius.nw,
                                    ne: ns.radius.ne,
                                    sw: 0,
                                    se: 0,
                                },
                                bandc,
                            );
                        }
                        let head = Rect::from_min_size(
                            rect.min + Vec2::new(0.0, band_h),
                            Vec2::new(rect.width(), 30.0),
                        );
                        ui.painter().rect_filled(
                            head,
                            CornerRadius {
                                nw: ns.radius.nw,
                                ne: ns.radius.ne,
                                sw: 0,
                                se: 0,
                            },
                            Color32::from_rgba_unmultiplied(0, 0, 0, 10),
                        );
                        ui.painter().text(
                            head.left_center() + Vec2::new(10.0, 0.0),
                            Align2::LEFT_CENTER,
                            pi.kind.title(),
                            FontId::proportional(13.0),
                            ns.ink,
                        );
                        let mk_btn = |dx: f32, label: &str, uid: &str, ui: &mut egui::Ui| {
                            let r = Rect::from_min_size(
                                Pos2::new(head.max.x - dx, head.min.y + 4.0),
                                Vec2::splat(22.0),
                            );
                            let clicked =
                                ui.interact(r, ui.id().with(uid), Sense::click()).clicked();
                            ui.painter().rect_filled(
                                r,
                                CornerRadius::same(4),
                                Color32::from_rgba_unmultiplied(0, 0, 0, 14),
                            );
                            ui.painter().text(
                                r.center(),
                                Align2::CENTER_CENTER,
                                label,
                                FontId::proportional(12.0),
                                ns.ink,
                            );
                            clicked
                        };
                        if mk_btn(78.0, if pi.big { "⤡" } else { "⤢" }, "big", ui) {
                            let p = &mut self.postits[idx];
                            p.big = !p.big;
                            p.size = if p.big {
                                p.kind.default_size().max(Vec2::new(420.0, 240.0))
                            } else {
                                Vec2::new(280.0, 180.0).max(p.kind.default_size() * 0.6)
                            };
                        }
                        if mk_btn(52.0, "–", "min", ui) {
                            to_min = Some(idx);
                        }
                        if mk_btn(26.0, "×", "close", ui) {
                            to_close = Some(idx);
                        }

                        let body = Rect::from_min_max(Pos2::new(rect.min.x, head.max.y), rect.max);
                        let mut child =
                            ui.new_child(egui::UiBuilder::new().max_rect(body.shrink(10.0)));
                        child.set_clip_rect(body.shrink(10.0));
                        let big = self.postits[idx].big || self.postits[idx].size.x >= 420.0;
                        self.paint_postit_body(&mut child, pi.kind, ns.ink, ns.muted, big);

                        let grip = Rect::from_min_size(
                            Pos2::new(rect.max.x - 16.0, rect.max.y - 16.0),
                            Vec2::splat(16.0),
                        );
                        let gr = ui.interact(grip, ui.id().with("grip"), Sense::drag());
                        let gc = ns.muted.gamma_multiply(0.7);
                        ui.painter().line_segment(
                            [
                                Pos2::new(grip.max.x - 3.0, grip.min.y + 5.0),
                                Pos2::new(grip.min.x + 5.0, grip.max.y - 3.0),
                            ],
                            Stroke::new(1.0, gc),
                        );
                        ui.painter().line_segment(
                            [
                                Pos2::new(grip.max.x - 3.0, grip.min.y + 9.0),
                                Pos2::new(grip.min.x + 9.0, grip.max.y - 3.0),
                            ],
                            Stroke::new(1.0, gc),
                        );
                        if gr.dragged() {
                            let d = gr.drag_delta();
                            let p = &mut self.postits[idx];
                            p.size.x = (p.size.x + d.x).clamp(220.0, 900.0);
                            p.size.y = (p.size.y + d.y).clamp(140.0, 700.0);
                            p.big = false;
                        }
                    });
                });
            self.postits[idx].pos = (resp.response.rect.min - board.min).to_pos2();
            if resp.response.clicked() || resp.response.dragged() {
                to_front = Some(idx);
            }
            if resp.response.dragged() {
                let r = Rect::from_min_size(
                    self.postits[idx].pos + board.min.to_vec2(),
                    self.postits[idx].size,
                );
                if let Some(k) = self.dock_target_kind(idx, r) {
                    drop_hint = self.dock_hint_rect(idx, r, k);
                }
            }
            if resp.response.drag_stopped() {
                let r = Rect::from_min_size(
                    self.postits[idx].pos + board.min.to_vec2(),
                    self.postits[idx].size,
                );
                if let Some(k) = self.dock_target_kind(idx, r) {
                    drop_target = Some((idx, k));
                }
            }
        }
        if let Some(r) = drop_hint {
            ui.painter_at(board).rect_stroke(
                r,
                CornerRadius::same(6),
                Stroke::new(2.0, th.accent),
                StrokeKind::Middle,
            );
        }
        if let Some(i) = to_front {
            self.z_counter = self.z_counter.saturating_add(1);
            self.postits[i].z = self.z_counter;
        }
        if let Some(i) = to_close {
            self.postits[i].open = false;
            self.postits[i].minimized = false;
            self.postits[i].dock = None;
        }
        if let Some(i) = to_min {
            self.postits[i].minimized = true;
        }
        if let Some((idx, kind)) = drop_target {
            self.apply_dock(idx, kind);
        }
        self.paint_docks(ui, board, th);
    }

    fn dock_target_kind(&self, idx: usize, rect: Rect) -> Option<DockKind> {
        for (j, p) in self.postits.iter().enumerate() {
            if j == idx || !p.open || p.minimized || p.dock.is_some() {
                continue;
            }
            let b = Rect::from_min_size(p.pos.to_vec2().to_pos2(), p.size);
            if let Some(k) = dock_relation(rect, b) {
                return Some(k);
            }
        }
        for d in &self.docks {
            if !d.open {
                continue;
            }
            let w = if d.kind == DockKind::Split {
                d.widths.iter().sum()
            } else {
                d.widths.first().copied().unwrap_or(400.0)
            };
            let b = Rect::from_min_size(d.pos, Vec2::new(w, d.height));
            if let Some(k) = dock_relation(rect, b) {
                return Some(k);
            }
        }
        None
    }

    fn dock_hint_rect(&self, idx: usize, rect: Rect, kind: DockKind) -> Option<Rect> {
        for (j, p) in self.postits.iter().enumerate() {
            if j == idx || !p.open || p.minimized || p.dock.is_some() {
                continue;
            }
            let b = Rect::from_min_size(p.pos.to_vec2().to_pos2(), p.size);
            if dock_relation(rect, b) == Some(kind) {
                return Some(match kind {
                    DockKind::Tabs => b.expand(4.0),
                    DockKind::Split => {
                        if rect.center().x < b.center().x {
                            Rect::from_min_max(
                                Pos2::new(b.min.x - rect.width() - 4.0, b.min.y),
                                Pos2::new(b.min.x - 4.0, b.max.y),
                            )
                        } else {
                            Rect::from_min_max(
                                Pos2::new(b.max.x + 4.0, b.min.y),
                                Pos2::new(b.max.x + 4.0 + rect.width(), b.max.y),
                            )
                        }
                    }
                });
            }
        }
        for d in &self.docks {
            if !d.open {
                continue;
            }
            let w = if d.kind == DockKind::Split {
                d.widths.iter().sum()
            } else {
                d.widths.first().copied().unwrap_or(400.0)
            };
            let b = Rect::from_min_size(d.pos, Vec2::new(w, d.height));
            if dock_relation(rect, b) == Some(kind) {
                return Some(match kind {
                    DockKind::Tabs => b.expand(4.0),
                    DockKind::Split => {
                        if rect.center().x < b.center().x {
                            Rect::from_min_max(
                                Pos2::new(b.min.x - rect.width() - 4.0, b.min.y),
                                Pos2::new(b.min.x - 4.0, b.max.y),
                            )
                        } else {
                            Rect::from_min_max(
                                Pos2::new(b.max.x + 4.0, b.min.y),
                                Pos2::new(b.max.x + 4.0 + rect.width(), b.max.y),
                            )
                        }
                    }
                });
            }
        }
        None
    }

    fn apply_dock(&mut self, idx: usize, kind: DockKind) {
        let rect = Rect::from_min_size(
            self.postits[idx].pos.to_vec2().to_pos2(),
            self.postits[idx].size,
        );
        let mut target_pi: Option<usize> = None;
        let mut target_dock: Option<u8> = None;
        for (j, p) in self.postits.iter().enumerate() {
            if j == idx || !p.open || p.minimized || p.dock.is_some() {
                continue;
            }
            let b = Rect::from_min_size(p.pos.to_vec2().to_pos2(), p.size);
            if dock_relation(rect, b) == Some(kind) {
                target_pi = Some(j);
                break;
            }
        }
        if target_pi.is_none() {
            for d in &self.docks {
                if !d.open {
                    continue;
                }
                let w = if d.kind == DockKind::Split {
                    d.widths.iter().sum()
                } else {
                    d.widths.first().copied().unwrap_or(400.0)
                };
                let b = Rect::from_min_size(d.pos, Vec2::new(w, d.height));
                if dock_relation(rect, b) == Some(kind) {
                    target_dock = Some(d.id);
                    break;
                }
            }
        }
        self.dock_counter = self.dock_counter.saturating_add(1);
        let did = self.dock_counter;
        self.z_counter = self.z_counter.saturating_add(1);
        let z = self.z_counter;
        if let Some(j) = target_pi {
            let b = Rect::from_min_size(
                self.postits[j].pos.to_vec2().to_pos2(),
                self.postits[j].size,
            );
            let (pos, widths, members) = match kind {
                DockKind::Tabs => (
                    b.min,
                    vec![b.width().max(self.postits[idx].size.x)],
                    vec![j, idx],
                ),
                DockKind::Split => {
                    let left = rect.center().x < b.center().x;
                    if left {
                        (
                            Pos2::new(b.min.x - self.postits[idx].size.x - 4.0, b.min.y),
                            vec![self.postits[idx].size.x, b.width()],
                            vec![idx, j],
                        )
                    } else {
                        (
                            b.min,
                            vec![b.width(), self.postits[idx].size.x],
                            vec![j, idx],
                        )
                    }
                }
            };
            let h = b.height().max(self.postits[idx].size.y);
            self.postits[j].dock = Some(did);
            self.postits[j].open = true;
            self.postits[idx].dock = Some(did);
            self.postits[idx].open = true;
            self.docks.push(DockGroup {
                id: did,
                kind,
                members,
                pos,
                widths,
                height: h,
                active: 0,
                open: true,
                z,
            });
        } else if let Some(tdid) = target_dock {
            if let Some(d) = self.docks.iter_mut().find(|d| d.id == tdid) {
                if kind == DockKind::Tabs || d.kind == DockKind::Tabs {
                    d.kind = DockKind::Tabs;
                    d.members.push(idx);
                    d.active = d.members.len() - 1;
                } else {
                    let b = Rect::from_min_size(d.pos, Vec2::new(d.widths.iter().sum(), d.height));
                    let left = rect.center().x < b.center().x;
                    if left {
                        d.pos.x -= self.postits[idx].size.x + 4.0;
                        d.widths.insert(0, self.postits[idx].size.x);
                        d.members.insert(0, idx);
                    } else {
                        d.widths.push(self.postits[idx].size.x);
                        d.members.push(idx);
                    }
                }
                d.z = z;
            }
            self.postits[idx].dock = Some(tdid);
            self.postits[idx].open = true;
        }
    }

    fn paint_docks(&mut self, ui: &egui::Ui, board: Rect, th: BoardTheme) {
        let mut dock_ids: Vec<u8> = self.docks.iter().filter(|d| d.open).map(|d| d.id).collect();
        dock_ids.sort_by_key(|id| self.docks.iter().find(|d| d.id == *id).map_or(0, |d| d.z));
        let topz = dock_ids
            .last()
            .and_then(|id| self.docks.iter().find(|d| d.id == *id))
            .map_or(0, |d| d.z);
        let mut to_front: Option<u8> = None;
        let mut close_dock: Option<u8> = None;

        for did in dock_ids {
            let Some(di) = self.docks.iter().position(|d| d.id == did) else {
                continue;
            };
            let d = self.docks[di].clone();
            let total_w = if d.kind == DockKind::Split {
                d.widths.iter().sum()
            } else {
                d.widths.first().copied().unwrap_or(420.0)
            };
            let size = Vec2::new(total_w, d.height);
            let id = Id::new(("dock", did));
            let is_top = d.z == topz;
            let pos0 = board.min + d.pos.to_vec2();
            let resp = Area::new(id)
                .order(Order::Middle)
                .current_pos(pos0)
                .movable(true)
                .constrain_to(board)
                .show(ui.ctx(), |ui| {
                    let frame = egui::Frame::NONE
                        .fill(th.panel2)
                        .corner_radius(CornerRadius::same(6))
                        .stroke(Stroke::new(1.0, th.line))
                        .shadow(egui::epaint::Shadow {
                            offset: [0, 8],
                            blur: 22,
                            spread: 0,
                            color: Color32::from_rgba_unmultiplied(
                                0,
                                0,
                                0,
                                if is_top { 80 } else { 45 },
                            ),
                        });
                    frame.show(ui, |ui| {
                        ui.set_min_size(size);
                        ui.set_max_size(size);
                        let rect = ui.max_rect();
                        match d.kind {
                            DockKind::Tabs => {
                                let head =
                                    Rect::from_min_size(rect.min, Vec2::new(rect.width(), 32.0));
                                ui.painter().rect_filled(
                                    head,
                                    CornerRadius {
                                        nw: 6,
                                        ne: 6,
                                        sw: 0,
                                        se: 0,
                                    },
                                    Color32::from_rgba_unmultiplied(0, 0, 0, 12),
                                );
                                let mut tx = head.min.x + 6.0;
                                let mut new_active = d.active;
                                for (mi, &midx) in d.members.iter().enumerate() {
                                    let Some(pit) = self.postits.get(midx) else {
                                        continue;
                                    };
                                    let label = pit.kind.title();
                                    let tw = (label.len() as f32 * 7.5 + 18.0).max(60.0);
                                    let tr = Rect::from_min_size(
                                        Pos2::new(tx, head.min.y + 4.0),
                                        Vec2::new(tw, 24.0),
                                    );
                                    let active = mi == d.active;
                                    if ui
                                        .interact(tr, ui.id().with(("tab", mi)), Sense::click())
                                        .clicked()
                                    {
                                        new_active = mi;
                                    }
                                    ui.painter().rect_filled(
                                        tr,
                                        CornerRadius::same(4),
                                        if active {
                                            th.accent.gamma_multiply(0.25)
                                        } else {
                                            Color32::TRANSPARENT
                                        },
                                    );
                                    ui.painter().text(
                                        tr.center(),
                                        Align2::CENTER_CENTER,
                                        label,
                                        FontId::proportional(11.0),
                                        if active { th.accent } else { th.text },
                                    );
                                    tx += tw + 4.0;
                                }
                                if let Some(dd) = self.docks.iter_mut().find(|x| x.id == did) {
                                    dd.active = new_active;
                                }
                                let close_r = Rect::from_min_size(
                                    Pos2::new(head.max.x - 26.0, head.min.y + 5.0),
                                    Vec2::splat(22.0),
                                );
                                if ui
                                    .interact(close_r, ui.id().with("dclose"), Sense::click())
                                    .clicked()
                                {
                                    close_dock = Some(did);
                                }
                                ui.painter().text(
                                    close_r.center(),
                                    Align2::CENTER_CENTER,
                                    "×",
                                    FontId::proportional(14.0),
                                    th.text,
                                );
                                let body =
                                    Rect::from_min_max(Pos2::new(rect.min.x, head.max.y), rect.max);
                                if let Some(&midx) = d.members.get(
                                    self.docks
                                        .iter()
                                        .find(|x| x.id == did)
                                        .map_or(d.active, |x| x.active),
                                ) {
                                    if let Some(pit) = self.postits.get(midx).copied() {
                                        let ns = note_style(self.theme, pit.kind);
                                        ui.painter().rect_filled(
                                            body,
                                            CornerRadius {
                                                nw: 0,
                                                ne: 0,
                                                sw: 6,
                                                se: 6,
                                            },
                                            ns.paper,
                                        );
                                        let mut child = ui.new_child(
                                            egui::UiBuilder::new().max_rect(body.shrink(10.0)),
                                        );
                                        child.set_clip_rect(body.shrink(10.0));
                                        let big = body.width() >= 420.0;
                                        self.paint_postit_body(
                                            &mut child, pit.kind, ns.ink, ns.muted, big,
                                        );
                                    }
                                }
                            }
                            DockKind::Split => {
                                let mut x = rect.min.x;
                                let mut close_clicked = false;
                                for (mi, &midx) in d.members.iter().enumerate() {
                                    let w = d.widths.get(mi).copied().unwrap_or(200.0);
                                    let pane = Rect::from_min_size(
                                        Pos2::new(x, rect.min.y),
                                        Vec2::new(w, rect.height()),
                                    );
                                    if mi > 0 {
                                        ui.painter().line_segment(
                                            [
                                                Pos2::new(pane.min.x, pane.min.y + 2.0),
                                                Pos2::new(pane.min.x, pane.max.y - 2.0),
                                            ],
                                            Stroke::new(1.0, th.line),
                                        );
                                    }
                                    if let Some(pit) = self.postits.get(midx).copied() {
                                        let ns = note_style(self.theme, pit.kind);
                                        ui.painter().rect_filled(
                                            pane,
                                            CornerRadius::ZERO,
                                            ns.paper,
                                        );
                                        let head = Rect::from_min_size(
                                            pane.min,
                                            Vec2::new(pane.width(), 28.0),
                                        );
                                        ui.painter().rect_filled(
                                            head,
                                            CornerRadius::ZERO,
                                            Color32::from_rgba_unmultiplied(0, 0, 0, 8),
                                        );
                                        ui.painter().text(
                                            head.left_center() + Vec2::new(8.0, 0.0),
                                            Align2::LEFT_CENTER,
                                            pit.kind.title(),
                                            FontId::proportional(12.0),
                                            ns.ink,
                                        );
                                        if mi == d.members.len() - 1 {
                                            let close_r = Rect::from_min_size(
                                                Pos2::new(head.max.x - 24.0, head.min.y + 3.0),
                                                Vec2::splat(22.0),
                                            );
                                            if ui
                                                .interact(
                                                    close_r,
                                                    ui.id().with("dsclose"),
                                                    Sense::click(),
                                                )
                                                .clicked()
                                            {
                                                close_clicked = true;
                                            }
                                            ui.painter().text(
                                                close_r.center(),
                                                Align2::CENTER_CENTER,
                                                "×",
                                                FontId::proportional(13.0),
                                                ns.ink,
                                            );
                                        }
                                        let body = Rect::from_min_max(
                                            Pos2::new(pane.min.x, head.max.y),
                                            pane.max,
                                        );
                                        let mut child = ui.new_child(
                                            egui::UiBuilder::new().max_rect(body.shrink(8.0)),
                                        );
                                        child.set_clip_rect(body.shrink(8.0));
                                        let big = body.width() >= 300.0;
                                        self.paint_postit_body(
                                            &mut child, pit.kind, ns.ink, ns.muted, big,
                                        );
                                    }
                                    x += w;
                                }
                                if close_clicked {
                                    close_dock = Some(did);
                                }
                            }
                        }
                    });
                });
            if let Some(dd) = self.docks.iter_mut().find(|x| x.id == did) {
                dd.pos = (resp.response.rect.min - board.min).to_pos2();
            }
            if resp.response.clicked() || resp.response.dragged() {
                to_front = Some(did);
            }
        }
        if let Some(did) = to_front {
            self.z_counter = self.z_counter.saturating_add(1);
            if let Some(dd) = self.docks.iter_mut().find(|x| x.id == did) {
                dd.z = self.z_counter;
            }
        }
        if let Some(did) = close_dock {
            if let Some(di) = self.docks.iter().position(|d| d.id == did) {
                let members = self.docks[di].members.clone();
                self.docks[di].open = false;
                for m in members {
                    if let Some(p) = self.postits.get_mut(m) {
                        p.dock = None;
                        p.open = false;
                    }
                }
            }
        }
    }

    fn dock_label(&self, id: u8) -> String {
        let Some(d) = self.docks.iter().find(|d| d.id == id) else {
            return "Gruppo".into();
        };
        let mut names: Vec<&str> = d
            .members
            .iter()
            .filter_map(|&i| self.postits.get(i).map(|p| p.kind.title()))
            .collect();
        if names.len() > 2 {
            names.truncate(2);
            return format!("{} +…", names.join(" + "));
        }
        names.join(" + ")
    }

    fn pitch_rect(&self, board: Rect) -> Rect {
        let board_h = board.height();
        let ph = (board_h - 60.0)
            .min((board.width() - 210.0) * 68.0 / 105.0)
            .max(200.0);
        let pw = ph * 105.0 / 68.0;
        Rect::from_min_size(
            Pos2::new(
                board.min.x + 190.0 + (board.width() - 190.0 - pw) / 2.0,
                board.min.y + (board_h - ph) / 2.0,
            ),
            Vec2::new(pw, ph),
        )
    }

    fn formation_default(phase: Phase) -> [(f32, f32); 11] {
        match phase {
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
            _ => [
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
        }
    }
    fn phase_idx(phase: Phase) -> usize {
        match phase {
            Phase::Trans => 1,
            Phase::Senza => 2,
            _ => 0,
        }
    }
    fn current_formation(&self) -> [(f32, f32); 11] {
        self.formation[Self::phase_idx(self.phase)]
    }

    #[allow(clippy::many_single_char_names)]
    fn paint_pitch(&mut self, ui: &mut egui::Ui, board: Rect, th: BoardTheme) {
        let field = self.pitch_rect(board);
        let p = ui.painter_at(board);
        p.rect_filled(field, CornerRadius::same(4), th.pitch.gamma_multiply(0.35));
        let line = Stroke::new(1.5, th.chalk_soft.gamma_multiply(0.8));
        let inner = field.shrink2(Vec2::new(field.width() * 0.03, field.height() * 0.04));
        p.rect_stroke(inner, CornerRadius::ZERO, line, StrokeKind::Middle);
        p.line_segment(
            [
                Pos2::new(inner.center().x, inner.top()),
                Pos2::new(inner.center().x, inner.bottom()),
            ],
            line,
        );
        p.circle_stroke(inner.center(), inner.height() * 0.135, line);
        let bw = inner.width() * 0.16;
        let bh = inner.height() * 0.52;
        let by = inner.center().y - bh / 2.0;
        p.rect_stroke(
            Rect::from_min_size(Pos2::new(inner.left(), by), Vec2::new(bw, bh)),
            CornerRadius::ZERO,
            line,
            StrokeKind::Middle,
        );
        p.rect_stroke(
            Rect::from_min_size(Pos2::new(inner.right() - bw, by), Vec2::new(bw, bh)),
            CornerRadius::ZERO,
            line,
            StrokeKind::Middle,
        );
        let sw = inner.width() * 0.055;
        let sh = inner.height() * 0.24;
        let sy = inner.center().y - sh / 2.0;
        p.rect_stroke(
            Rect::from_min_size(Pos2::new(inner.left(), sy), Vec2::new(sw, sh)),
            CornerRadius::ZERO,
            line,
            StrokeKind::Middle,
        );
        p.rect_stroke(
            Rect::from_min_size(Pos2::new(inner.right() - sw, sy), Vec2::new(sw, sh)),
            CornerRadius::ZERO,
            line,
            StrokeKind::Middle,
        );

        if self.scenario == Scenario::Partita {
            let palette = palette(th, self.team_color());
            self.view.paint(ui, field, &palette);
        } else {
            let opp = Self::opponent_formation(self.phase);
            for (gx, gy) in opp {
                let dot = Pos2::new(
                    field.left() + gx * field.width(),
                    field.top() + gy * field.height(),
                );
                p.circle_stroke(
                    dot,
                    10.0,
                    Stroke::new(1.5, Color32::from_rgba_unmultiplied(110, 160, 255, 140)),
                );
                p.text(
                    dot,
                    Align2::CENTER_CENTER,
                    "◆",
                    FontId::proportional(9.0),
                    Color32::from_rgba_unmultiplied(110, 160, 255, 180),
                );
            }
            let form = self.current_formation();
            let mut drag_new: Option<usize> = self.drag_player;
            for (i, (xp, yp)) in form.iter().enumerate() {
                let dot = Pos2::new(
                    field.left() + xp * field.width(),
                    field.top() + yp * field.height(),
                );
                let hit = Rect::from_center_size(dot, Vec2::splat(28.0));
                let resp = ui.interact(hit, ui.id().with(("ply", i)), Sense::drag());
                if resp.drag_started() {
                    drag_new = Some(i);
                }
                if resp.dragged() {
                    let d = resp.drag_delta();
                    let ph = Self::phase_idx(self.phase);
                    let f = &mut self.formation[ph][i];
                    f.0 = (f.0 + d.x / field.width()).clamp(0.02, 0.98);
                    f.1 = (f.1 + d.y / field.height()).clamp(0.02, 0.98);
                }
                if resp.drag_stopped() && drag_new == Some(i) {
                    drag_new = None;
                }
                let is_drag = drag_new == Some(i);
                let col = if is_drag {
                    th.accent.gamma_multiply(0.85)
                } else {
                    th.accent
                };
                p.circle_filled(dot, 12.0, col);
                p.text(
                    dot,
                    Align2::CENTER_CENTER,
                    ((b'A' + i as u8) as char).to_string(),
                    FontId::proportional(10.0),
                    ink_on(col),
                );
            }
            self.drag_player = drag_new;
            let ball = Pos2::new(field.left() + field.width() * 0.3, field.center().y);
            p.circle_filled(ball, 4.5, Color32::WHITE);
        }
    }

    fn opponent_formation(phase: Phase) -> [(f32, f32); 11] {
        match phase {
            Phase::Con => [
                (0.94, 0.50),
                (0.80, 0.20),
                (0.82, 0.38),
                (0.82, 0.62),
                (0.80, 0.80),
                (0.68, 0.30),
                (0.68, 0.50),
                (0.68, 0.70),
                (0.56, 0.35),
                (0.56, 0.65),
                (0.50, 0.50),
            ],
            Phase::Trans => [
                (0.94, 0.50),
                (0.76, 0.22),
                (0.74, 0.40),
                (0.74, 0.60),
                (0.76, 0.78),
                (0.60, 0.28),
                (0.58, 0.50),
                (0.60, 0.72),
                (0.46, 0.40),
                (0.46, 0.60),
                (0.40, 0.50),
            ],
            Phase::Senza => [
                (0.94, 0.50),
                (0.72, 0.18),
                (0.70, 0.38),
                (0.70, 0.62),
                (0.72, 0.82),
                (0.56, 0.24),
                (0.54, 0.42),
                (0.54, 0.58),
                (0.56, 0.76),
                (0.42, 0.44),
                (0.42, 0.56),
            ],
            Phase::Nessuna => [
                (0.94, 0.50),
                (0.80, 0.22),
                (0.80, 0.40),
                (0.80, 0.60),
                (0.80, 0.78),
                (0.66, 0.18),
                (0.68, 0.40),
                (0.68, 0.60),
                (0.66, 0.82),
                (0.58, 0.44),
                (0.58, 0.56),
            ],
        }
    }

    fn paint_postit_body(
        &mut self,
        ui: &mut egui::Ui,
        kind: PostItKind,
        ink: Color32,
        muted: Color32,
        big: bool,
    ) {
        match kind {
            PostItKind::Rosa => self.body_rosa(ui, ink, muted, big),
            PostItKind::Giorno => self.body_giornata(ui, ink, muted, big),
            PostItKind::Istruzioni => self.body_istruzioni(ui, ink, muted),
            PostItKind::Prossima => self.body_prossima(ui, ink, muted, big),
            PostItKind::Posta => self.body_posta(ui, ink, muted, big),
            PostItKind::Cronaca => self.body_cronaca(ui, ink, muted, big),
            PostItKind::Stats => self.body_stats(ui, ink, muted, big),
            PostItKind::Perche => self.body_perche(ui, ink, muted),
            PostItKind::Formazione => self.body_formazione(ui, ink, muted, big),
            PostItKind::Tiri => self.body_tiri(ui, ink, muted),
            PostItKind::Calore => self.body_calore(ui, ink, muted),
            PostItKind::Eventi => self.body_eventi(ui, ink, muted),
            PostItKind::Campo => self.body_campo(ui, ink, muted),
        }
    }

    fn body_giornata(&mut self, ui: &mut egui::Ui, ink: Color32, muted: Color32, big: bool) {
        let day_defs = Self::day_defs();
        let view = self.view_day.clamp(0, 6) as usize;
        let day = self.current_day().clamp(0, 6) as usize;
        let title = format!(
            "{}{}",
            day_defs[view].1,
            if view == day { " · oggi" } else { "" }
        );
        ui.label(RichText::new(title).color(ink).size(14.0).strong());
        ui.add_space(4.0);
        let agenda = Self::agenda_for(self.view_day);
        for (ora, cosa, dett, dot) in agenda {
            let (r, _) = ui.allocate_exact_size(
                Vec2::new(ui.available_width(), if big { 44.0 } else { 28.0 }),
                Sense::hover(),
            );
            ui.painter().line_segment(
                [Pos2::new(r.min.x, r.max.y), Pos2::new(r.max.x, r.max.y)],
                Stroke::new(1.0, muted.gamma_multiply(0.25)),
            );
            ui.painter().text(
                Pos2::new(r.min.x, r.min.y + 8.0),
                Align2::LEFT_TOP,
                ora,
                FontId::monospace(11.0),
                muted,
            );
            ui.painter()
                .circle_filled(Pos2::new(r.min.x + 52.0, r.min.y + 10.0), 4.0, dot);
            ui.painter().text(
                Pos2::new(r.min.x + 62.0, r.min.y + 6.0),
                Align2::LEFT_TOP,
                cosa,
                FontId::proportional(12.5),
                ink,
            );
            if big && !dett.is_empty() {
                ui.painter().text(
                    Pos2::new(r.min.x + 62.0, r.min.y + 24.0),
                    Align2::LEFT_TOP,
                    dett,
                    FontId::proportional(11.0),
                    muted,
                );
            }
        }
        if view == 5 {
            ui.add_space(6.0);
            let (r, resp) = ui.allocate_exact_size(Vec2::new(120.0, 32.0), Sense::click());
            ui.painter().rect_filled(r, CornerRadius::same(8), ink);
            ui.painter().text(
                r.center(),
                Align2::CENTER_CENTER,
                "Vai in panchina",
                FontId::proportional(12.0),
                Color32::from_rgb(255, 248, 230),
            );
            if resp.clicked() {
                self.go_match();
            }
        }
    }

    fn body_rosa(&self, ui: &mut egui::Ui, ink: Color32, muted: Color32, big: bool) {
        let rows: Vec<(String, String, String, u32, u32, String)> = if let Some(c) = &self.career {
            c.squad()
                .iter()
                .map(|m| {
                    (
                        m.shirt.to_string(),
                        m.name.clone(),
                        m.role.clone(),
                        m.age,
                        75u32,
                        "6.5".into(),
                    )
                })
                .collect()
        } else {
            vec![
                (
                    "1".into(),
                    "Portiere".into(),
                    "POR".into(),
                    28,
                    80,
                    "6.7".into(),
                ),
                (
                    "4".into(),
                    "Difensore".into(),
                    "DC".into(),
                    26,
                    76,
                    "6.6".into(),
                ),
                (
                    "10".into(),
                    "Regista".into(),
                    "CC".into(),
                    24,
                    82,
                    "7.0".into(),
                ),
            ]
        };
        if big {
            let (hdr, _) =
                ui.allocate_exact_size(Vec2::new(ui.available_width(), 18.0), Sense::hover());
            let cols = [26.0, 130.0, 44.0, 36.0, 90.0, 40.0];
            let labels = ["", "GIOCATORE", "RUOLO", "ETÀ", "ENERGIA", "MEDIA"];
            let mut x = hdr.min.x;
            for (i, l) in labels.iter().enumerate() {
                ui.painter().text(
                    Pos2::new(x, hdr.center().y),
                    Align2::LEFT_CENTER,
                    *l,
                    FontId::proportional(9.5),
                    muted,
                );
                x += cols[i];
            }
            ui.painter().line_segment(
                [
                    Pos2::new(hdr.min.x, hdr.max.y),
                    Pos2::new(hdr.max.x, hdr.max.y),
                ],
                Stroke::new(1.0, muted.gamma_multiply(0.3)),
            );
        }
        for (sh, name, role, age, fit, avg) in rows.iter().take(18) {
            let h = if big { 24.0 } else { 22.0 };
            let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), h), Sense::hover());
            ui.painter().line_segment(
                [Pos2::new(r.min.x, r.max.y), Pos2::new(r.max.x, r.max.y)],
                Stroke::new(1.0, muted.gamma_multiply(0.15)),
            );
            if big {
                let cols = [26.0, 130.0, 44.0, 36.0, 90.0, 40.0];
                let mut x = r.min.x;
                ui.painter().text(
                    Pos2::new(x, r.center().y),
                    Align2::LEFT_CENTER,
                    sh,
                    FontId::proportional(11.0),
                    Color32::from_rgb(0xB4, 0x53, 0x09),
                );
                x += cols[0];
                ui.painter().text(
                    Pos2::new(x, r.center().y),
                    Align2::LEFT_CENTER,
                    name,
                    FontId::proportional(11.5),
                    ink,
                );
                x += cols[1];
                ui.painter().text(
                    Pos2::new(x, r.center().y),
                    Align2::LEFT_CENTER,
                    role,
                    FontId::proportional(10.5),
                    muted,
                );
                x += cols[2];
                ui.painter().text(
                    Pos2::new(x, r.center().y),
                    Align2::LEFT_CENTER,
                    age.to_string(),
                    FontId::monospace(10.5),
                    ink,
                );
                x += cols[3];
                let bar = Rect::from_min_size(
                    Pos2::new(x, r.center().y - 2.5),
                    Vec2::new(cols[4] - 10.0, 5.0),
                );
                ui.painter()
                    .rect_filled(bar, CornerRadius::same(2), muted.gamma_multiply(0.25));
                ui.painter().rect_filled(
                    Rect::from_min_size(
                        bar.min,
                        Vec2::new(bar.width() * (*fit as f32) / 100.0, bar.height()),
                    ),
                    CornerRadius::same(2),
                    ink.gamma_multiply(0.7),
                );
                x += cols[4];
                ui.painter().text(
                    Pos2::new(x, r.center().y),
                    Align2::LEFT_CENTER,
                    avg,
                    FontId::monospace(10.5),
                    ink,
                );
            } else {
                ui.painter().text(
                    Pos2::new(r.min.x, r.center().y),
                    Align2::LEFT_CENTER,
                    role,
                    FontId::proportional(10.5),
                    muted,
                );
                ui.painter().text(
                    Pos2::new(r.min.x + 34.0, r.center().y),
                    Align2::LEFT_CENTER,
                    name,
                    FontId::proportional(11.5),
                    ink,
                );
            }
        }
        if big {
            ui.label(
                RichText::new("Titolari in chiaro, la panchina sbiadita. Dati di prova.")
                    .color(muted)
                    .size(10.5),
            );
        }
    }

    fn body_istruzioni(&mut self, ui: &mut egui::Ui, ink: Color32, muted: Color32) {
        ui.label(
            RichText::new("TRANSIZIONE")
                .color(muted)
                .size(10.0)
                .strong(),
        );
        for label in ["Palla persa", "Palla recuperata"] {
            ui.label(RichText::new(label).color(ink).size(12.0).strong());
            let opts: &[&str] = if label == "Palla persa" {
                &["Contropressing", "Ripiegare"]
            } else {
                &["Contropiede", "Tenere palla"]
            };
            ui.horizontal(|ui| {
                for (j, opt) in opts.iter().enumerate() {
                    let active = if label == "Palla persa" {
                        self.trans_persa == j
                    } else {
                        self.trans_recup == j
                    };
                    let bg = if active { ink } else { Color32::TRANSPARENT };
                    let tc = if active {
                        Color32::from_rgb(255, 248, 230)
                    } else {
                        ink
                    };
                    let btn = egui::Button::new(RichText::new(*opt).size(11.5).color(tc))
                        .fill(bg)
                        .stroke(Stroke::new(1.0, ink.gamma_multiply(0.3)));
                    if ui.add(btn).clicked() {
                        if label == "Palla persa" {
                            self.trans_persa = j;
                        } else {
                            self.trans_recup = j;
                        }
                    }
                }
            });
        }
        ui.add_space(4.0);
        ui.label(
            RichText::new("ISTRUZIONI RAPIDE")
                .color(muted)
                .size(10.0)
                .strong(),
        );
        let labs = [
            "Più pressione",
            "Linea più alta",
            "Allarga il gioco",
            "Palla lunga alle punte",
            "Perdi tempo",
            "Tutti in attacco",
        ];
        ui.horizontal_wrapped(|ui| {
            for (i, lab) in labs.iter().enumerate() {
                let active = self.instructions[i];
                let bg = if active { ink } else { Color32::TRANSPARENT };
                let tc = if active {
                    Color32::from_rgb(255, 248, 230)
                } else {
                    ink
                };
                let btn = egui::Button::new(RichText::new(*lab).size(11.0).color(tc))
                    .fill(bg)
                    .stroke(Stroke::new(1.0, ink.gamma_multiply(0.3)));
                if ui.add(btn).clicked() {
                    self.instructions[i] = !active;
                }
            }
        });
        let on: Vec<&str> = labs
            .iter()
            .enumerate()
            .filter_map(|(i, l)| self.instructions[i].then_some(*l))
            .collect();
        let note = if on.is_empty() {
            "Nessuna istruzione attiva. I giocatori le leggono dalla prossima azione.".to_string()
        } else {
            format!(
                "Attive: {}. I giocatori le leggono dalla prossima azione.",
                on.join(", ")
            )
        };
        ui.label(RichText::new(note).color(muted).size(10.5));
    }

    fn body_prossima(&self, ui: &mut egui::Ui, ink: Color32, muted: Color32, big: bool) {
        ui.label(
            RichText::new("Sabato 15:00 · in casa")
                .color(muted)
                .size(11.0),
        );
        ui.label(
            RichText::new("Home – Real Valcastello")
                .color(ink)
                .size(18.0)
                .strong(),
        );
        ui.label(
            RichText::new(
                "Loro: blocco basso, ampiezza stretta. Sulla lavagna li vedi tratteggiati.",
            )
            .color(ink)
            .size(12.0),
        );
        if big {
            ui.label(RichText::new("Le ultime tre contro un blocco basso: 0-0, 1-1, 2-1. La palla va mossa in fretta da una fascia all'altra.").color(muted).size(12.0));
        }
    }

    fn body_posta(&self, ui: &mut egui::Ui, ink: Color32, muted: Color32, big: bool) {
        for (from, subj, body) in [
            (
                "Osservatore Bellini",
                "Rapporto su un giovane",
                "Visto in 4 allenamenti: fascia ampia sul passaggio, consiglio un'altra settimana.",
            ),
            (
                "Staff medico",
                "Home 6 è stanco",
                "Mercoledì ha corso 11,4 km: nel secondo tempo calerà.",
            ),
            (
                "Domenica Sport",
                "Parlano di noi",
                "Tre gol subiti su cinque nascono da una spazzata ripresa.",
            ),
        ] {
            ui.label(RichText::new(from).color(muted).size(11.0));
            ui.label(RichText::new(subj).color(ink).size(13.0).strong());
            if big {
                ui.label(RichText::new(body).color(ink).size(12.0));
            }
            ui.add_space(4.0);
            let (r, _) =
                ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.0), Sense::hover());
            ui.painter()
                .rect_filled(r, CornerRadius::ZERO, muted.gamma_multiply(0.2));
        }
    }

    fn body_cronaca(&self, ui: &mut egui::Ui, ink: Color32, muted: Color32, big: bool) {
        let comm = [
            ("86'", "Corner per gli avversari"),
            ("82'", "Fuorigioco, si riparte"),
            ("78'", "Tiro fuori dal limite"),
            ("74'", "Cambio: entra Home 14"),
            ("70'", "GOL! Home 10 di testa"),
            ("62'", "Tiro forte da 31 m"),
            ("58'", "Ammonizione per fallo"),
            ("52'", "Cambio: fresco in campo"),
            ("34'", "GOL! Home 7 di testa"),
            ("21'", "Parata sul tiro dal limite"),
        ];
        if big {
            for (m, t) in comm {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(m).color(muted).size(12.0).monospace());
                    ui.label(RichText::new(t).color(ink).size(12.0));
                });
                ui.separator();
            }
        } else {
            for (m, t) in comm.iter().filter(|(_, t)| t.contains("GOL")) {
                ui.horizontal(|ui| {
                    ui.colored_label(Color32::from_rgb(0xD9, 0xA2, 0x4A), "●");
                    ui.label(RichText::new(*m).color(ink).size(12.0).monospace());
                    ui.label(RichText::new(*t).color(muted).size(12.0));
                });
            }
            ui.label(
                RichText::new("Allarga il pannello per la cronaca completa")
                    .color(muted)
                    .size(10.5),
            );
        }
    }

    fn body_stats(&self, ui: &mut egui::Ui, ink: Color32, muted: Color32, big: bool) {
        let stats = [
            ("Possesso", "54%", "46%", 54.0f32),
            ("Tiri", "11", "13", 46.0),
            ("In porta", "4", "5", 44.0),
            ("Passaggi", "412", "355", 54.0),
            ("Precisione", "84%", "79%", 52.0),
            ("Falli", "9", "12", 43.0),
            ("Angoli", "5", "6", 45.0),
            ("Fuorigioco", "2", "1", 67.0),
        ];
        let show = if big { &stats[..] } else { &stats[..3] };
        for (n, h, a, w) in show {
            ui.horizontal(|ui| {
                ui.label(RichText::new(*h).color(ink).size(11.0).monospace());
                ui.vertical(|ui| {
                    ui.label(RichText::new(*n).color(muted).size(9.5));
                    let (r, _) = ui.allocate_exact_size(
                        Vec2::new(ui.available_width().min(140.0), 5.0),
                        Sense::hover(),
                    );
                    ui.painter().rect_filled(
                        r,
                        CornerRadius::same(2),
                        Color32::from_rgb(0x6E, 0xA0, 0xFF),
                    );
                    ui.painter().rect_filled(
                        Rect::from_min_size(r.min, Vec2::new(r.width() * (*w / 100.0), r.height())),
                        CornerRadius::same(2),
                        Color32::from_rgb(0xF0, 0x8A, 0x5D),
                    );
                });
                ui.label(RichText::new(*a).color(ink).size(11.0).monospace());
            });
        }
    }

    fn body_perche(&self, ui: &mut egui::Ui, ink: Color32, muted: Color32) {
        ui.label(
            RichText::new("62' · AWAY 6")
                .color(Color32::from_rgb(0xB4, 0x53, 0x09))
                .size(10.0)
                .strong(),
        );
        ui.label(
            RichText::new("Tiro forte da 31 m")
                .color(ink)
                .size(15.0)
                .strong(),
        );
        ui.label(RichText::new("L'azione nasce 2 s prima: Home 6 spazza, loro la riconquistano. Nessun loro passaggio in mezzo.").color(ink).size(12.5));
        ui.add_space(4.0);
        let (r, _) = ui.allocate_exact_size(Vec2::new(140.0, 34.0), Sense::click());
        ui.painter().rect_stroke(
            r,
            CornerRadius::same(8),
            Stroke::new(1.0, Color32::from_rgb(0xB4, 0x53, 0x09)),
            StrokeKind::Middle,
        );
        ui.painter().text(
            r.center(),
            Align2::CENTER_CENTER,
            "Rivedi da 61:58",
            FontId::proportional(12.0),
            Color32::from_rgb(0xB4, 0x53, 0x09),
        );
        let _ = muted;
    }

    fn body_formazione(&self, ui: &mut egui::Ui, ink: Color32, muted: Color32, big: bool) {
        let xi = [
            ("A", "Portiere", "POR", 88u32),
            ("B", "Terzino Dx", "TD", 82),
            ("C", "Difensore", "DC", 79),
            ("D", "Difensore", "DC", 76),
            ("E", "Terzino Sx", "TS", 84),
            ("F", "Mediano", "CC", 73),
            ("G", "Regista", "CC", 81),
            ("H", "Mezzala", "CC", 77),
            ("I", "Ala Dx", "AD", 69),
            ("J", "Ala Sx", "AS", 85),
            ("K", "Punta", "ATT", 74),
        ];
        if big {
            ui.columns(2, |cols| {
                for (idx, (l, n, r, f)) in xi.iter().enumerate() {
                    let ui = &mut cols[idx % 2];
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(*l)
                                .color(Color32::from_rgb(0xB4, 0x53, 0x09))
                                .size(11.0)
                                .strong(),
                        );
                        ui.label(RichText::new(*n).color(ink).size(11.0));
                        ui.label(RichText::new(*r).color(muted).size(9.5));
                        let (br, _) = ui.allocate_exact_size(Vec2::new(40.0, 5.0), Sense::hover());
                        ui.painter().rect_filled(
                            br,
                            CornerRadius::same(2),
                            muted.gamma_multiply(0.25),
                        );
                        ui.painter().rect_filled(
                            Rect::from_min_size(
                                br.min,
                                Vec2::new(br.width() * (*f as f32) / 100.0, br.height()),
                            ),
                            CornerRadius::same(2),
                            ink.gamma_multiply(0.7),
                        );
                    });
                }
            });
        } else {
            ui.horizontal_wrapped(|ui| {
                for (l, _, _, f) in xi {
                    let c = Color32::from_rgb(
                        (31.0 + 0.7 * (f as f32)) as u8,
                        (90.0 + 0.2 * (f as f32)) as u8,
                        69,
                    );
                    let (r, _) = ui.allocate_exact_size(Vec2::splat(28.0), Sense::hover());
                    ui.painter().rect_filled(r, CornerRadius::same(6), c);
                    ui.painter().text(
                        r.center(),
                        Align2::CENTER_CENTER,
                        l,
                        FontId::proportional(11.0),
                        Color32::from_rgb(0xF6, 0xFB, 0xF2),
                    );
                }
            });
            ui.label(
                RichText::new("Il colore dice l'energia")
                    .color(muted)
                    .size(10.5),
            );
        }
    }

    fn body_tiri(&self, ui: &mut egui::Ui, _ink: Color32, muted: Color32) {
        let (r, _) = ui.allocate_exact_size(
            Vec2::new(ui.available_width(), ui.available_height().max(180.0)),
            Sense::hover(),
        );
        let pitch = Rect::from_center_size(
            r.center(),
            Vec2::new(r.width().min(320.0), r.height().min(200.0)),
        );
        ui.painter().rect_filled(
            pitch,
            CornerRadius::same(6),
            Color32::from_rgb(0x17, 0x3A, 0x2C),
        );
        ui.painter().rect_stroke(
            pitch.shrink(4.0),
            CornerRadius::same(2),
            Stroke::new(1.5, muted.gamma_multiply(0.6)),
            StrokeKind::Middle,
        );
        ui.painter().rect_stroke(
            Rect::from_min_size(
                Pos2::new(
                    pitch.right() - pitch.width() * 0.32,
                    pitch.top() + pitch.height() * 0.2,
                ),
                Vec2::new(pitch.width() * 0.3, pitch.height() * 0.6),
            ),
            CornerRadius::ZERO,
            Stroke::new(1.5, muted.gamma_multiply(0.6)),
            StrokeKind::Middle,
        );
        for (x, y, out, home) in [
            (0.82, 0.45, "gol", true),
            (0.75, 0.30, "fuori", true),
            (0.88, 0.55, "parata", false),
            (0.70, 0.60, "respinto", true),
            (0.90, 0.35, "palo", false),
        ] {
            let pos = Pos2::new(
                pitch.left() + x * pitch.width(),
                pitch.top() + y * pitch.height(),
            );
            let col = match out {
                "gol" => Color32::from_rgb(0xD9, 0x77, 0x06),
                "parata" => muted,
                "respinto" => Color32::from_rgb(0x6B, 0x72, 0x80),
                _ => Color32::from_rgb(0x9C, 0xA3, 0xAF),
            };
            ui.painter().circle_filled(pos, 5.0, col);
            ui.painter().circle_stroke(
                pos,
                5.0,
                Stroke::new(
                    1.5,
                    if home {
                        Color32::from_rgb(0xC2, 0x41, 0x0C)
                    } else {
                        Color32::from_rgb(0x1D, 0x4E, 0xD8)
                    },
                ),
            );
        }
    }

    fn body_calore(&self, ui: &mut egui::Ui, _ink: Color32, muted: Color32) {
        ui.label(
            RichText::new("Away 6 · una posizione al secondo")
                .color(muted)
                .size(11.0),
        );
        let (r, _) = ui.allocate_exact_size(
            Vec2::new(ui.available_width(), ui.available_height().max(160.0)),
            Sense::hover(),
        );
        let pitch = Rect::from_center_size(
            r.center(),
            Vec2::new(r.width().min(360.0), r.height().min(210.0)),
        );
        ui.painter().rect_filled(
            pitch,
            CornerRadius::same(6),
            Color32::from_rgb(0x17, 0x3A, 0x2C),
        );
        let cols = 12;
        let rows = 8;
        let cw = pitch.width() / cols as f32;
        let ch = pitch.height() / rows as f32;
        for ry in 0..rows {
            for cx in 0..cols {
                let a = (((cx as f32 - 7.0).abs() + (ry as f32 - 3.5).abs()) / 10.0)
                    .mul_add(-0.8, 0.85)
                    .clamp(0.0, 0.85);
                let cell = Rect::from_min_size(
                    Pos2::new(pitch.left() + cx as f32 * cw, pitch.top() + ry as f32 * ch),
                    Vec2::new(cw, ch),
                );
                ui.painter().rect_filled(
                    cell,
                    CornerRadius::ZERO,
                    Color32::from_rgba_unmultiplied(240, 176, 74, (a * 255.0) as u8),
                );
            }
        }
        ui.painter().rect_stroke(
            pitch.shrink(4.0),
            CornerRadius::same(2),
            Stroke::new(1.5, muted.gamma_multiply(0.6)),
            StrokeKind::Middle,
        );
        ui.painter().line_segment(
            [
                Pos2::new(pitch.center().x, pitch.top() + 4.0),
                Pos2::new(pitch.center().x, pitch.bottom() - 4.0),
            ],
            Stroke::new(1.5, muted.gamma_multiply(0.6)),
        );
    }

    fn body_eventi(&self, ui: &mut egui::Ui, ink: Color32, muted: Color32) {
        for (c, l, w, home) in [
            ("90'", "Fine partita", "—", true),
            ("86'", "Corner", "Avversari", false),
            ("82'", "Fuorigioco", "Home 10", true),
            ("78'", "Tiro fuori", "Away 9", false),
            ("74'", "Cambio", "Home 14 per 8", true),
            ("70'", "Gol", "Home 10", true),
            ("62'", "Tiro", "Away 6", false),
        ] {
            ui.horizontal(|ui| {
                ui.label(RichText::new(c).color(muted).size(11.0).monospace());
                ui.label(RichText::new(l).color(ink).size(11.5));
                ui.label(
                    RichText::new(w)
                        .color(if home {
                            Color32::from_rgb(0xC2, 0x41, 0x0C)
                        } else {
                            Color32::from_rgb(0x1D, 0x4E, 0xD8)
                        })
                        .size(11.0),
                );
            });
            ui.separator();
        }
    }

    fn body_campo(&self, ui: &mut egui::Ui, _ink: Color32, muted: Color32) {
        let (r, _) = ui.allocate_exact_size(
            Vec2::new(ui.available_width(), ui.available_height().max(180.0)),
            Sense::hover(),
        );
        let pitch = Rect::from_center_size(
            r.center(),
            Vec2::new(r.width().min(360.0), r.height().min(220.0)),
        );
        ui.painter().rect_filled(
            pitch,
            CornerRadius::same(6),
            Color32::from_rgb(0x17, 0x3A, 0x2C),
        );
        let pl = Stroke::new(1.5, muted.gamma_multiply(0.6));
        ui.painter().rect_stroke(
            pitch.shrink(6.0),
            CornerRadius::ZERO,
            pl,
            StrokeKind::Middle,
        );
        ui.painter().line_segment(
            [
                Pos2::new(pitch.center().x, pitch.top() + 6.0),
                Pos2::new(pitch.center().x, pitch.bottom() - 6.0),
            ],
            pl,
        );
        ui.painter()
            .circle_stroke(pitch.center(), pitch.height() * 0.12, pl);
        for (x, y, l, home) in [
            (0.08, 0.50, "1", true),
            (0.20, 0.20, "2", true),
            (0.20, 0.40, "3", true),
            (0.20, 0.60, "4", true),
            (0.20, 0.80, "5", true),
            (0.38, 0.30, "6", true),
            (0.38, 0.50, "7", true),
            (0.38, 0.70, "8", true),
            (0.55, 0.25, "9", true),
            (0.55, 0.75, "10", true),
            (0.60, 0.50, "11", true),
            (0.92, 0.50, "1", false),
            (0.80, 0.20, "2", false),
            (0.80, 0.40, "3", false),
            (0.80, 0.60, "4", false),
            (0.80, 0.80, "5", false),
        ] {
            let pos = Pos2::new(
                pitch.left() + x * pitch.width(),
                pitch.top() + y * pitch.height(),
            );
            let c = if home {
                Color32::from_rgb(0xF0, 0x8A, 0x5D)
            } else {
                Color32::from_rgb(0x6E, 0xA0, 0xFF)
            };
            let tc = if home {
                Color32::from_rgb(0x2A, 0x12, 0x08)
            } else {
                Color32::from_rgb(0x0B, 0x1B, 0x3A)
            };
            ui.painter().circle_filled(pos, 10.0, c);
            ui.painter()
                .text(pos, Align2::CENTER_CENTER, l, FontId::proportional(9.0), tc);
        }
        ui.painter().circle_filled(
            Pos2::new(pitch.left() + pitch.width() * 0.06, pitch.center().y),
            4.0,
            Color32::WHITE,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The desktop's scoreboard follows the record's playhead.
    #[test]
    fn the_scoreboard_comes_from_the_match_view() {
        let mut app = GuiApp::default();
        let before = app.scoreboard();
        app.view.advance(1_000.0);
        assert_ne!(app.scoreboard(), before);
        let (home, away) = app.view.score();
        assert!(app.scoreboard().contains(&format!("{home} – {away}")));
    }
}
