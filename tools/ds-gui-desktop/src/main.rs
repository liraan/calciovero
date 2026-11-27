//! `ds-gui-desktop` — Calcio Vero desktop GUI (ADR-061).
//!
//! Riferimento: `context/previews/gui-from-maintainer/Lavagna.dc.html`.

use ds_gui_desktop::gui::GuiApp;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Calcio Vero — Lavagna")
            .with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "Calcio Vero",
        options,
        Box::new(|_cc| Ok(Box::new(GuiApp::default()))),
    )
}
