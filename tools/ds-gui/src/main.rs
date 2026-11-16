//! `ds-gui` — Calcio Vero's desktop GUI (ADR-055).
//!
//! The friendly match: the manager chooses, the match is played by the
//! engine, and he watches it on the tilted board of the 2.5D view with the
//! commentary and the reasons for every goal conceded.
//!
//! `ds-gui [--theme Lavagna|Notte|Carta] [--tilt DEG] [--seed N] [--at TICK] [--speed X]` It reads Plane B only (ADR-022 §3).

mod app;
mod interp;
mod scene;
mod theme;

/// The options of the command line; an unknown one is an error.
fn start() -> Result<app::Start, String> {
    let mut s = app::Start::default();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut value = || args.next().ok_or_else(|| format!("{a}: missing value"));
        match a.as_str() {
            "--theme" => s.theme = Some(value()?),
            "--tilt" => s.tilt = Some(value()?.parse().map_err(|e| format!("--tilt: {e}"))?),
            "--seed" => s.seed = Some(value()?.parse().map_err(|e| format!("--seed: {e}"))?),
            "--at" => s.tick = Some(value()?.parse().map_err(|e| format!("--at: {e}"))?),
            "--speed" => s.speed = Some(value()?.parse().map_err(|e| format!("--speed: {e}"))?),
            "--help" | "-h" => {
                println!(
                    "ds-gui [--theme Lavagna|Notte|Carta] [--tilt DEG] [--seed N] [--at TICK] [--speed X]"
                );
                std::process::exit(0);
            }
            other => return Err(format!("unknown option {other}")),
        }
    }
    Ok(s)
}

fn main() -> eframe::Result {
    let start = match start() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ds-gui: {e}");
            std::process::exit(2);
        }
    };
    let theme = match theme::Theme::all() {
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
        Box::new(move |_cc| Ok(Box::new(app::App::new(theme).with_start(&start)))),
    )
}
