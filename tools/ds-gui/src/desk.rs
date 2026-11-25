//! The desk (decision BO, `DESIGN.md` "Le finestre e la scrivania"): the
//! career's screen. The tactical board is the base, where the plan is
//! decided; over it the windows (the squad, the table, the calendar, the
//! agenda) float, move, grow and close, opened from the column on the
//! left; the match opens in the coach's tablet. It reads the career's
//! plane B only (`ds_career`).

use std::fmt::Write as _;
use std::sync::mpsc::{Receiver, TryRecvError};

use ds_career::{Career, CareerError, Fixture, Lineup, Watch};
use eframe::egui::{self, Color32, Pos2, RichText, Stroke, Vec2};

use crate::app::App;
use crate::theme::Theme;

/// A window over the board.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    /// The day: the session, the results, what the coach must know.
    Agenda,
    /// The squad and the line-up.
    Squad,
    /// The table.
    Table,
    /// The calendar and the results.
    Calendar,
}

impl Pane {
    const ALL: [Self; 4] = [Self::Agenda, Self::Squad, Self::Table, Self::Calendar];

    const fn title(self) -> &'static str {
        match self {
            Self::Agenda => "Agenda",
            Self::Squad => "Rosa e formazione",
            Self::Table => "Classifica",
            Self::Calendar => "Calendario",
        }
    }

    const fn icon(self) -> &'static str {
        match self {
            Self::Agenda => "✉",
            Self::Squad => "👥",
            Self::Table => "≡",
            Self::Calendar => "▦",
        }
    }

    /// Where the composed screen the game gives puts it (decision BO), as
    /// shares of the board: the squad on the right, the table under the
    /// board; the agenda and the calendar, when opened, under it too.
    const fn place(self) -> (f32, f32, f32, f32) {
        match self {
            Self::Squad => (0.60, 0.0, 0.40, 1.0),
            Self::Table => (0.0, 0.70, 0.58, 0.30),
            Self::Agenda => (0.30, 0.70, 0.28, 0.30),
            Self::Calendar => (0.0, 0.70, 0.29, 0.30),
        }
    }
}

/// The column's groups and their windows (decision BO): only the windows
/// of the modules there are.
const RAIL: [(&str, &[Pane]); 2] = [
    ("SCRIVANIA", &[Pane::Agenda, Pane::Squad]),
    ("STAGIONE", &[Pane::Table, Pane::Calendar]),
];

/// The phase the board shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
    WithBall,
    WithoutBall,
}

/// The human club's match of the last day, played again and recorded on
/// the career's thread, to watch.
type Recorded = (Watch, ds_match::MatchResult, Vec<ds_match::TickRecord>);

/// A career back from its thread: the matches of its last day, and the
/// human club's match recorded if it played.
type Played = (
    Box<Career>,
    Result<(Vec<Fixture>, Option<Box<Recorded>>), CareerError>,
);

/// What the career is doing: waiting for the human player, or playing days
/// on another thread.
enum Work {
    Idle(Box<Career>),
    Busy(Receiver<Played>),
}

/// What the windows need to draw, read once a frame from the career.
struct View<'a> {
    career: &'a Career,
    clubs: Vec<ds_world::Club>,
}

/// The desk.
pub struct Desk {
    work: Option<Work>,
    open: Vec<Pane>,
    theme: Theme,
    themes: Vec<Theme>,
    /// The line-up being prepared: the vice's proposal, changed by hand.
    draft: Option<Lineup>,
    /// The first of two places clicked, to swap them.
    picked: Option<u32>,
    phase: Phase,
    /// The match open in the tablet.
    tablet: Option<Box<App>>,
    /// The last word to the human player (an error, a line-up taken).
    note: Option<String>,
    /// Where the career is saved.
    save_to: Option<std::path::PathBuf>,
}

fn club_name(view: &View<'_>, club: u32) -> String {
    view.clubs
        .get(club as usize)
        .map_or_else(|| format!("club {club}"), |c| c.name.clone())
}

/// A role as the screen says it, in Italian (the data keeps its label).
fn role_it(role: &str) -> &str {
    match role {
        "goalkeeper" => "Portiere",
        "full_back" => "Terzino",
        "centre_back" => "Difensore centrale",
        "wide_midfielder" => "Esterno",
        "central_midfielder" => "Centrocampista",
        "striker" => "Attaccante",
        other => other,
    }
}

/// A number with the decimal comma, one decimal.
fn decimal(v: f64) -> String {
    format!("{v:.1}").replace('.', ",")
}

const fn rgb(c: [u8; 3]) -> Color32 {
    Color32::from_rgb(c[0], c[1], c[2])
}

/// The day: the session held, the line-up refused, the results of the
/// last day, the bans of the human player's men.
fn agenda(ui: &mut egui::Ui, view: &View<'_>) {
    let c = view.career;
    let today = c.day().saturating_sub(1);
    ui.label(RichText::new(format!("Giorno {today}")).strong());
    for e in c.events().iter().filter(|e| e.day == today) {
        let get = |k: &str| {
            e.fields
                .iter()
                .find(|(f, _)| f == k)
                .map(|(_, v)| v.as_str())
        };
        let mine = get("club").and_then(|x| x.parse::<u32>().ok()) == Some(c.club());
        match e.kind.as_str() {
            "SessionHeld" if mine => {
                ui.label(format!("Seduta: {}", get("kind").unwrap_or("")));
            }
            "LineupRefused" if mine => {
                ui.colored_label(
                    Color32::LIGHT_RED,
                    format!("Formazione rifiutata: {}", get("why").unwrap_or("")),
                );
            }
            _ => {}
        }
    }
    let fixtures = c.fixtures();
    let mut played = fixtures
        .iter()
        .filter(|f| f.day == today && f.score.is_some())
        .peekable();
    if played.peek().is_some() {
        ui.separator();
        ui.label(RichText::new("Risultati").strong());
        for f in played {
            let (h, a) = f.score.unwrap_or((0, 0));
            ui.label(format!(
                "{} {h} – {a} {}",
                club_name(view, f.home),
                club_name(view, f.away)
            ));
        }
    }
    let banned: Vec<String> = c
        .squad()
        .into_iter()
        .filter(|m| m.banned)
        .map(|m| m.name)
        .collect();
    if !banned.is_empty() {
        ui.separator();
        ui.colored_label(
            Color32::LIGHT_RED,
            format!("Squalificati: {}", banned.join(", ")),
        );
    }
}

/// Swaps the two men clicked: two places of the line-up, or a man out of
/// it for one in.
fn swap(places: &mut [u32], first: u32, second: u32) {
    match (
        places.iter().position(|x| *x == first),
        places.iter().position(|x| *x == second),
    ) {
        (Some(a), Some(b)) => places.swap(a, b),
        (Some(a), None) => places[a] = second,
        (None, Some(b)) => places[b] = first,
        (None, None) => {}
    }
}

/// The squad and the line-up: the vice's proposal; two men clicked are
/// swapped; the line-up given for the next match.
fn squad(
    ui: &mut egui::Ui,
    view: &View<'_>,
    draft: &mut Option<Lineup>,
    picked: &mut Option<u32>,
    give: &mut Option<(Vec<u32>, u8, u8)>,
) {
    let c = view.career;
    if draft.is_none() {
        *draft = c.proposal();
    }
    let Some(lineup) = draft else {
        ui.label("Nessuna partita in programma.");
        return;
    };
    let men = c.squad();
    let roles = c.places();
    let mut clicked = None;
    egui::ScrollArea::vertical()
        .max_height(ui.available_height() - 40.0)
        .show(ui, |ui| {
            egui::Grid::new("squad")
                .striped(true)
                .num_columns(6)
                .show(ui, |ui| {
                    for h in ["", "Posto", "Giocatore", "Età", "Freschezza", ""] {
                        ui.label(RichText::new(h).small());
                    }
                    ui.end_row();
                    let outside = men
                        .iter()
                        .map(|m| m.id)
                        .filter(|id| !lineup.places.contains(id));
                    let rows: Vec<(Option<usize>, u32)> = lineup
                        .places
                        .iter()
                        .enumerate()
                        .map(|(i, id)| (Some(i), *id))
                        .chain(outside.map(|id| (None, id)))
                        .collect();
                    for (place, id) in rows {
                        let Some(man) = men.iter().find(|m| m.id == id) else {
                            continue;
                        };
                        let tag = match place {
                            Some(i) if i < 11 => format!("{}", i + 1),
                            Some(_) => "P".into(),
                            None => "–".into(),
                        };
                        let sel = *picked == Some(id);
                        if ui.selectable_label(sel, tag).clicked() {
                            clicked = Some(id);
                        }
                        ui.label(role_it(
                            place
                                .and_then(|i| roles.get(i))
                                .map_or(man.role.as_str(), |r| r),
                        ));
                        let name = if man.banned {
                            RichText::new(&man.name).color(Color32::LIGHT_RED)
                        } else {
                            RichText::new(&man.name)
                        };
                        if ui.selectable_label(sel, name).clicked() {
                            clicked = Some(id);
                        }
                        ui.label(man.age.to_string());
                        #[allow(clippy::cast_possible_truncation)]
                        let fresh = (1.0 - man.tired).clamp(0.0, 1.0) as f32;
                        ui.add(egui::ProgressBar::new(fresh).desired_width(70.0));
                        let mut flags = String::new();
                        if man.banned {
                            flags.push_str("squalificato ");
                        }
                        if man.cautions > 0 {
                            let _ = write!(flags, "{} amm.", man.cautions);
                        }
                        ui.label(RichText::new(flags).small());
                        ui.end_row();
                    }
                });
        });
    if let Some(id) = clicked {
        match picked.take() {
            None => *picked = Some(id),
            Some(first) if first == id => {}
            Some(first) => swap(&mut lineup.places, first, id),
        }
    }
    let mut reset = false;
    ui.horizontal(|ui| {
        if ui
            .button("Dai la formazione per la prossima partita")
            .clicked()
        {
            *give = Some((lineup.places.clone(), lineup.block, lineup.width));
        }
        if ui.button("Torna alla proposta del vice").clicked() {
            reset = true;
        }
    });
    if reset {
        *draft = c.proposal();
    }
}

/// The board, the base of the desk: the drawing with the ball or without
/// (`ds_model::Board`, the one the engine plays) with the eleven of the
/// line-up, the plan (block and width) decided here, and the vice's word
/// from the public table (plane B).
// The board and its controls read as one: the base of the desk.
#[allow(clippy::too_many_lines)]
fn board(
    ui: &mut egui::Ui,
    view: &View<'_>,
    theme: &Theme,
    draft: &mut Option<Lineup>,
    phase: &mut Phase,
) {
    if draft.is_none() {
        *draft = view.career.proposal();
    }
    ui.horizontal(|ui| {
        ui.selectable_value(phase, Phase::WithBall, "Con palla");
        ui.selectable_value(phase, Phase::WithoutBall, "Senza palla");
        if let Some(l) = draft.as_mut() {
            ui.separator();
            ui.label("Blocco");
            for (v, t) in [(0, "basso"), (1, "medio"), (2, "alto")] {
                ui.selectable_value(&mut l.block, v, t);
            }
            ui.separator();
            ui.label("Ampiezza");
            for (v, t) in [(0, "stretta"), (1, "normale"), (2, "larga")] {
                ui.selectable_value(&mut l.width, v, t);
            }
        }
    });
    let drawing = {
        let b = ds_model::Board::default();
        match phase {
            Phase::WithBall => b.with_ball,
            Phase::WithoutBall => b.without_ball,
        }
    };
    // The pitch over the left half of the board, under the windows.
    let avail = ui.available_rect_before_wrap();
    let pitch_w = (avail.width() * 0.47).max(200.0);
    let size = Vec2::new(
        pitch_w,
        (pitch_w * 0.65).min(avail.height() - 90.0).max(120.0),
    );
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let paint = ui.painter_at(rect);
    paint.rect_filled(rect, 4.0, theme.pitch);
    let line = Stroke::new(2.0_f32, theme.lines);
    paint.rect_stroke(rect, 4.0, line, egui::StrokeKind::Inside);
    paint.line_segment([rect.center_top(), rect.center_bottom()], line);
    paint.circle_stroke(rect.center(), rect.height() * 0.13, line);
    let at = |x: f32, y: f32| {
        Pos2::new(
            rect.width().mul_add(x, rect.left()),
            rect.height().mul_add(y, rect.top()),
        )
    };
    let (shirt, trim) = view
        .clubs
        .get(view.career.club() as usize)
        .map_or((theme.home, Color32::BLACK), |x| {
            (rgb(x.colours[0]), rgb(x.colours[1]))
        });
    let men = view.career.squad();
    for (i, spot) in drawing.iter().enumerate() {
        // The board's drawing: depth from the backs (0) to the forwards
        // (1000) over a quarter of the pitch from its quarter, width across
        // the middle seven tenths (the 4-4-2 grid's span, ds_model).
        let (x, y) = if i == 0 {
            (0.05, 0.5)
        } else {
            (
                (f32::from(spot.depth) / 1000.0).mul_add(0.5, 0.25),
                (f32::from(spot.width) / 1000.0).mul_add(0.7, 0.15),
            )
        };
        let spot_at = at(x, y);
        paint.circle_filled(
            spot_at,
            13.0,
            if i == 0 { theme.keeper_home } else { shirt },
        );
        paint.circle_stroke(
            spot_at,
            13.0,
            Stroke::new(2.5_f32, if i == 0 { Color32::BLACK } else { trim }),
        );
        paint.text(
            spot_at,
            egui::Align2::CENTER_CENTER,
            (i + 1).to_string(),
            egui::FontId::proportional(12.0),
            theme.player_text,
        );
        if let Some(man) = draft
            .as_ref()
            .and_then(|l| l.places.get(i))
            .and_then(|id| men.iter().find(|x| x.id == *id))
        {
            let surname = man.name.split(' ').next_back().unwrap_or("").to_string();
            paint.text(
                spot_at + Vec2::new(0.0, 20.0),
                egui::Align2::CENTER_CENTER,
                surname,
                egui::FontId::proportional(11.0),
                theme.lines,
            );
        }
    }
    // The vice's word, from what everyone sees: the points a match once
    // there are matches; before, his idea.
    if let (Some(l), Some(next)) = (draft.as_ref(), view.career.next_fixture()) {
        let opp = if next.home == view.career.club() {
            next.away
        } else {
            next.home
        };
        let table = view.career.table();
        let line = |club: u32| table.iter().find(|t| t.club == club).map(|t| t.row);
        let block = ["basso", "medio", "alto"][usize::from(l.block.min(2))];
        let width = ["stretta", "normale", "larga"][usize::from(l.width.min(2))];
        let said = match (line(opp), line(view.career.club())) {
            (Some(theirs), Some(ours)) if theirs.played > 0 && ours.played > 0 => format!(
                "{}: {} punti a partita, noi {}. Propongo il blocco {block} e l'ampiezza {width}.",
                club_name(view, opp),
                decimal(f64::from(theirs.points) / f64::from(theirs.played)),
                decimal(f64::from(ours.points) / f64::from(ours.played))
            ),
            _ => format!(
                "Prima partita contro {}: nessun dato ancora. Propongo la nostra idea, il blocco {block} e l'ampiezza {width}.",
                club_name(view, opp)
            ),
        };
        ui.add_space(6.0);
        egui::Frame::NONE
            .fill(theme.postit)
            .inner_margin(8.0)
            .corner_radius(6.0)
            .show(ui, |ui| {
                ui.set_max_width(pitch_w - 16.0);
                ui.colored_label(theme.postit_ink, RichText::new("IL VICE").small().strong());
                ui.colored_label(theme.postit_ink, said);
            });
    }
}

fn table(ui: &mut egui::Ui, view: &View<'_>) {
    egui::Grid::new("table")
        .striped(true)
        .num_columns(7)
        .show(ui, |ui| {
            for h in ["", "Squadra", "G", "V", "N", "P", "Pt"] {
                ui.label(RichText::new(h).small());
            }
            ui.end_row();
            for (i, t) in view.career.table().iter().enumerate() {
                let name = club_name(view, t.club);
                let name = if t.club == view.career.club() {
                    RichText::new(name).strong()
                } else {
                    RichText::new(name)
                };
                ui.label((i + 1).to_string());
                ui.label(name);
                for v in [t.row.played, t.row.won, t.row.drawn, t.row.lost] {
                    ui.label(v.to_string());
                }
                ui.label(RichText::new(t.row.points.to_string()).strong());
                ui.end_row();
            }
        });
}

fn calendar(ui: &mut egui::Ui, view: &View<'_>) {
    let me = view.career.club();
    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("calendar")
            .striped(true)
            .num_columns(3)
            .show(ui, |ui| {
                for f in view
                    .career
                    .fixtures()
                    .into_iter()
                    .filter(|f| f.home == me || f.away == me)
                {
                    ui.label(format!("{}ª", f.matchday + 1));
                    ui.label(format!(
                        "{} – {}",
                        club_name(view, f.home),
                        club_name(view, f.away)
                    ));
                    ui.label(f.score.map_or_else(
                        || format!("giorno {}", f.day),
                        |(h, a)| format!("{h} – {a}"),
                    ));
                    ui.end_row();
                }
            });
    });
}

impl Desk {
    /// The desk of `career`, with the composed screen the game gives
    /// (decision BO): the board, the squad and the table over it.
    #[must_use]
    pub fn new(career: Career, themes: Vec<Theme>, save_to: Option<std::path::PathBuf>) -> Self {
        Self {
            work: Some(Work::Idle(Box::new(career))),
            open: vec![Pane::Squad, Pane::Table],
            theme: themes[0].clone(),
            themes,
            draft: None,
            picked: None,
            phase: Phase::WithoutBall,
            tablet: None,
            note: None,
            save_to,
        }
    }

    /// Plays days on another thread: one, or up to the next match; the
    /// human club's match, if played, is recorded there too. The thread has
    /// the stack a match needs (the engine's, not the window's).
    fn play(&mut self, to_match: bool) {
        let Some(Work::Idle(mut career)) = self.work.take() else {
            return;
        };
        let (tx, rx) = std::sync::mpsc::channel();
        let spawned = std::thread::Builder::new()
            .stack_size(64 << 20)
            .spawn(move || {
                let r = if to_match {
                    career.advance_to_next_match()
                } else {
                    career.advance()
                };
                let r = r.and_then(|played| {
                    let mine = played
                        .iter()
                        .find(|f| f.home == career.club() || f.away == career.club());
                    let rec = match mine {
                        Some(f) => {
                            let w = career.watch(f)?;
                            let (result, records) = w.recorded();
                            Some(Box::new((w, result, records)))
                        }
                        None => None,
                    };
                    Ok((played, rec))
                });
                let _ = tx.send((career, r));
            });
        match spawned {
            Ok(_) => self.work = Some(Work::Busy(rx)),
            Err(e) => self.note = Some(format!("la giornata non parte: {e}")),
        }
        self.draft = None;
        self.picked = None;
        self.note = None;
    }

    fn poll(&mut self) {
        let Some(Work::Busy(rx)) = &self.work else {
            return;
        };
        match rx.try_recv() {
            Ok((career, r)) => {
                match r {
                    Ok((_, Some(rec))) => {
                        let (w, result, records) = *rec;
                        self.tablet = Some(Box::new(App::from_season(
                            self.themes.clone(),
                            w,
                            &result,
                            records,
                        )));
                    }
                    Ok((_, None)) => {}
                    Err(e) => self.note = Some(e.0),
                }
                if let Some(path) = &self.save_to {
                    if let Err(e) = std::fs::write(path, career.save()) {
                        self.note = Some(format!("salvataggio: {e}"));
                    }
                }
                self.work = Some(Work::Idle(career));
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => self.note = Some("la giornata si è fermata".into()),
        }
    }

    /// The desk, the whole window: the top bar, the column, the board, the
    /// windows over it, the tablet.
    #[allow(clippy::too_many_lines)]
    pub fn show(&mut self, ctx: &egui::Context) {
        self.poll();
        let career = match &self.work {
            Some(Work::Idle(c)) => Some(&**c),
            _ => None,
        };
        let clubs = career.map(Career::clubs).unwrap_or_default();
        let (busy, mut to_play) = (career.is_none(), None);
        egui::TopBottomPanel::top("desk-top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if let Some(c) = career {
                    let club = clubs.get(c.club() as usize);
                    let (a, b) = club.map_or((Color32::GRAY, Color32::DARK_GRAY), |x| {
                        (rgb(x.colours[0]), rgb(x.colours[1]))
                    });
                    let (r, _) = ui.allocate_exact_size(Vec2::splat(28.0), egui::Sense::hover());
                    ui.painter().circle_filled(r.center(), 13.0, a);
                    ui.painter()
                        .circle_stroke(r.center(), 13.0, Stroke::new(4.0_f32, b));
                    ui.label(
                        RichText::new(club.map_or("", |x| x.name.as_str()))
                            .size(18.0)
                            .strong(),
                    );
                    ui.label(
                        club.map_or(String::new(), |x| format!("{} · allenatore: tu", x.city)),
                    );
                    ui.separator();
                    ui.label(format!("Giorno {}", c.day()));
                    if let Some(n) = c.next_fixture() {
                        let names = (clubs.get(n.home as usize), clubs.get(n.away as usize));
                        ui.label(format!(
                            "Prossima: {}ª giornata, giorno {} · {} – {}",
                            n.matchday + 1,
                            n.day,
                            names.0.map_or("", |x| x.name.as_str()),
                            names.1.map_or("", |x| x.name.as_str())
                        ));
                    }
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if busy {
                        ui.spinner();
                        ui.label("si gioca…");
                    } else if career.is_some_and(|c| !c.is_over()) {
                        if ui
                            .button(RichText::new("Alla partita ▸").strong())
                            .clicked()
                        {
                            to_play = Some(true);
                        }
                        if ui.button("Avanti di un giorno").clicked() {
                            to_play = Some(false);
                        }
                    }
                    if let Some(n) = &self.note {
                        ui.colored_label(Color32::LIGHT_RED, n);
                    }
                });
            });
        });
        let mut toggle = None;
        egui::SidePanel::left("rail")
            .exact_width(176.0)
            .resizable(false)
            .show(ctx, |ui| {
                for (group, panes) in RAIL {
                    ui.add_space(8.0);
                    ui.label(RichText::new(group).small().color(self.theme.low));
                    for p in panes {
                        if ui
                            .selectable_label(
                                self.open.contains(p),
                                format!("{}  {}", p.icon(), p.title()),
                            )
                            .clicked()
                        {
                            toggle = Some(*p);
                        }
                    }
                }
            });
        let mut give = None;
        let board_rect = egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(self.theme.board).inner_margin(12.0))
            .show(ctx, |ui| {
                match career {
                    Some(c) => {
                        let view = View {
                            career: c,
                            clubs: clubs.clone(),
                        };
                        board(ui, &view, &self.theme, &mut self.draft, &mut self.phase);
                    }
                    None => {
                        ui.label("La giornata si sta giocando…");
                    }
                }
                ui.max_rect()
            })
            .inner;
        // The windows over the board: they move, grow and close; the column
        // opens them again.
        if let Some(c) = career {
            let view = View { career: c, clubs };
            for pane in Pane::ALL {
                if !self.open.contains(&pane) {
                    continue;
                }
                let (x, y, w, h) = pane.place();
                let mut still = true;
                egui::Window::new(format!("{} {}", pane.icon(), pane.title()))
                    .id(egui::Id::new(("desk-window", pane.title())))
                    .open(&mut still)
                    .default_pos(Pos2::new(
                        board_rect.width().mul_add(x, board_rect.left()),
                        board_rect.height().mul_add(y, board_rect.top()),
                    ))
                    .default_size(Vec2::new(board_rect.width() * w, board_rect.height() * h))
                    .resizable(true)
                    .collapsible(true)
                    .show(ctx, |ui| match pane {
                        Pane::Agenda => {
                            agenda(ui, &view);
                        }
                        Pane::Squad => {
                            squad(ui, &view, &mut self.draft, &mut self.picked, &mut give);
                        }
                        Pane::Table => {
                            table(ui, &view);
                        }
                        Pane::Calendar => {
                            calendar(ui, &view);
                        }
                    });
                if !still {
                    toggle = Some(pane);
                }
            }
        }
        if let Some(p) = toggle {
            if let Some(i) = self.open.iter().position(|x| *x == p) {
                self.open.remove(i);
            } else {
                self.open.push(p);
            }
        }
        if let (Some((places, block, width)), Some(Work::Idle(c))) = (give, &mut self.work) {
            self.note = Some(match c.give_lineup(&places, block, width) {
                Ok(()) => "Formazione data per la prossima partita.".into(),
                Err(e) => e.0,
            });
        }
        if let Some(to_match) = to_play {
            self.play(to_match);
        }
        self.tablet_ui(ctx);
        if busy {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }
    }

    /// The match in the coach's tablet (decision BO): over the desk, in the
    /// tablet's frame, the close button inside it; closed, the desk is back.
    fn tablet_ui(&mut self, ctx: &egui::Context) {
        let Some(app) = &mut self.tablet else { return };
        let screen = ctx.screen_rect();
        let bezel = screen.shrink2(Vec2::new(screen.width() * 0.05, screen.height() * 0.05));
        ctx.layer_painter(egui::LayerId::new(
            egui::Order::Middle,
            egui::Id::new("tablet-dim"),
        ))
        .rect_filled(screen, 0.0, Color32::from_black_alpha(170));
        let mut close = false;
        egui::Window::new("tablet")
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .fixed_rect(bezel)
            .frame(
                egui::Frame::NONE
                    .fill(Color32::from_rgb(18, 18, 18))
                    .corner_radius(36.0)
                    .stroke(Stroke::new(2.0_f32, Color32::from_gray(60)))
                    .inner_margin(egui::Margin {
                        left: 28,
                        right: 28,
                        top: 30,
                        bottom: 28,
                    }),
            )
            .show(ctx, |ui| {
                ui.set_min_size(ui.available_size());
                let top = ui.max_rect();
                // The close button on the bezel, inside its edge.
                let at = egui::Rect::from_min_size(
                    Pos2::new(top.right() - 150.0, top.top() - 26.0),
                    Vec2::new(150.0, 22.0),
                );
                if ui
                    .put(
                        at,
                        egui::Button::new("Chiudi il tablet ✕").corner_radius(10.0),
                    )
                    .clicked()
                {
                    close = true;
                }
                app.show(ui);
            });
        if close {
            self.tablet = None;
        }
    }
}
