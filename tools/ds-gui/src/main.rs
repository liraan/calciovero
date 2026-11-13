//! `ds-gui` — Calcio Vero's desktop GUI (ADR-055).
//!
//! The friendly match: the manager chooses, the match is played by the
//! engine, and he watches it on a 2D pitch with the commentary and the
//! reasons for every goal conceded. It reads Plane B only (ADR-022 §3).

mod app;
mod scene;
mod theme;

fn main() -> eframe::Result {
    let theme = match theme::Theme::load() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("ds-gui: theme.json: {e}");
            std::process::exit(2);
        }
    };
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Calcio Vero")
            .with_inner_size([1400.0, 820.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Calcio Vero",
        options,
        Box::new(move |_cc| Ok(Box::new(app::App::new(theme)))),
    )
}
