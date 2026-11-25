//! The game's shell (decision BN): the start, the choice of the club, the
//! desk of the career; the friendly from the start too.

use eframe::egui::{self, Color32, RichText, Stroke, Vec2};

use crate::app::App;
use crate::desk::Desk;
use crate::theme::Theme;

/// Where the career is saved: the user's data folder (`%APPDATA%` on
/// Windows, `$HOME/.local/share` elsewhere), `CalcioVero/career.json`.
fn save_path() -> Option<std::path::PathBuf> {
    let base = std::env::var_os("APPDATA")
        .map(std::path::PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME").map(|h| std::path::Path::new(&h).join(".local/share"))
        })?;
    let dir = base.join("CalcioVero");
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir.join("career.json"))
}

enum Screen {
    Start,
    /// The clubs of a new world, from its seed.
    Pick(u64, Vec<ds_world::Club>),
    Desk(Box<Desk>),
    Friendly(Box<App>),
}

/// The shell.
pub struct Shell {
    themes: Vec<Theme>,
    screen: Screen,
    error: Option<String>,
}

impl Shell {
    /// The shell at its start.
    #[must_use]
    pub const fn new(themes: Vec<Theme>) -> Self {
        Self {
            themes,
            screen: Screen::Start,
            error: None,
        }
    }

    /// The shell on the desk of the career of `seed`'s world and `club`
    /// (`--career SEED:CLUB`); on the start, with why, if it cannot.
    #[must_use]
    pub fn with_career(themes: Vec<Theme>, seed: u64, club: u32) -> Self {
        match ds_career::Career::new(ds_world::Pack::v1(), seed, club) {
            Ok(c) => Self {
                screen: Screen::Desk(Box::new(Desk::new(c, themes.clone(), save_path()))),
                themes,
                error: None,
            },
            Err(e) => Self {
                themes,
                screen: Screen::Start,
                error: Some(e.0),
            },
        }
    }

    fn start(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(ui.available_height() * 0.28);
            ui.label(RichText::new("Calcio Vero").size(48.0).strong());
            ui.label(
                RichText::new("Deep Simulator")
                    .size(16.0)
                    .color(self.themes[0].medium),
            );
            ui.add_space(30.0);
            let big = |t: &str| {
                egui::Button::new(RichText::new(t).size(18.0)).min_size(Vec2::new(260.0, 40.0))
            };
            if ui.add(big("Nuova carriera")).clicked() {
                // A new world each time: the seed is the moment, shown with
                // the clubs (the same seed, the same world).
                let seed = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_or(42, |d| d.as_secs());
                self.screen = Screen::Pick(seed, ds_world::make_clubs(&ds_world::Pack::v1(), seed));
            }
            let saved = save_path().filter(|p| p.exists());
            if let Some(path) = saved {
                if ui.add(big("Continua la carriera")).clicked() {
                    match std::fs::read_to_string(&path)
                        .map_err(|e| e.to_string())
                        .and_then(|t| {
                            ds_career::Career::load(ds_world::Pack::v1(), &t).map_err(|e| e.0)
                        }) {
                        Ok(c) => {
                            self.screen = Screen::Desk(Box::new(Desk::new(
                                c,
                                self.themes.clone(),
                                Some(path),
                            )));
                        }
                        Err(e) => self.error = Some(format!("il salvataggio non si apre: {e}")),
                    }
                }
            }
            if ui.add(big("Amichevole")).clicked() {
                self.screen = Screen::Friendly(Box::new(App::new(self.themes.clone())));
            }
            if let Some(e) = &self.error {
                ui.colored_label(Color32::LIGHT_RED, e);
            }
        });
    }

    fn pick(&self, ui: &mut egui::Ui, seed: u64, clubs: &[ds_world::Club]) -> Option<u32> {
        let mut chosen = None;
        ui.add_space(12.0);
        ui.label(
            RichText::new("Scegli il club che allenerai")
                .size(24.0)
                .strong(),
        );
        ui.label(RichText::new(format!("Mondo {seed} · Serie A")).color(self.themes[0].medium));
        ui.add_space(12.0);
        let first = ds_world::Pack::v1().series.first().map_or(0, |s| s.clubs);
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("clubs")
                .spacing(Vec2::splat(10.0))
                .show(ui, |ui| {
                    for (i, c) in clubs.iter().take(first as usize).enumerate() {
                        let a =
                            Color32::from_rgb(c.colours[0][0], c.colours[0][1], c.colours[0][2]);
                        let b =
                            Color32::from_rgb(c.colours[1][0], c.colours[1][1], c.colours[1][2]);
                        let r = ui
                            .scope(|ui| {
                                ui.set_min_size(Vec2::new(250.0, 56.0));
                                ui.horizontal(|ui| {
                                    let (dot, _) = ui.allocate_exact_size(
                                        Vec2::splat(36.0),
                                        egui::Sense::hover(),
                                    );
                                    ui.painter().circle_filled(dot.center(), 16.0, a);
                                    ui.painter().circle_stroke(
                                        dot.center(),
                                        16.0,
                                        Stroke::new(5.0_f32, b),
                                    );
                                    ui.vertical(|ui| {
                                        ui.label(RichText::new(&c.name).strong());
                                        ui.label(
                                            RichText::new(format!("{} · {}", c.city, c.stadium))
                                                .small(),
                                        );
                                    });
                                });
                            })
                            .response
                            .interact(egui::Sense::click());
                        if r.clicked() {
                            chosen = u32::try_from(i).ok();
                        }
                        if r.hovered() {
                            ui.painter().rect_stroke(
                                r.rect.expand(3.0),
                                6.0,
                                Stroke::new(1.5_f32, self.themes[0].critical),
                                egui::StrokeKind::Outside,
                            );
                        }
                        if i % 4 == 3 {
                            ui.end_row();
                        }
                    }
                });
        });
        chosen
    }
}

impl eframe::App for Shell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let theme = &self.themes[0];
        ctx.set_visuals(if theme.dark {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        });
        match &mut self.screen {
            Screen::Desk(desk) => desk.show(ctx),
            Screen::Friendly(app) => {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| app.show(ui));
            }
            Screen::Start => {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE.fill(theme.board))
                    .show(ctx, |ui| self.start(ui));
            }
            Screen::Pick(seed, clubs) => {
                let (seed, clubs) = (*seed, std::mem::take(clubs));
                let mut chosen = None;
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE.fill(theme.board).inner_margin(24.0))
                    .show(ctx, |ui| chosen = self.pick(ui, seed, &clubs));
                self.screen = match chosen
                    .map(|club| ds_career::Career::new(ds_world::Pack::v1(), seed, club))
                {
                    Some(Ok(c)) => {
                        Screen::Desk(Box::new(Desk::new(c, self.themes.clone(), save_path())))
                    }
                    Some(Err(e)) => {
                        self.error = Some(e.0);
                        Screen::Start
                    }
                    None => Screen::Pick(seed, clubs),
                };
            }
        }
    }
}
