//! `ds-gui` — Calcio Vero's desktop GUI (ADR-055).
//!
//! The friendly match: the manager chooses, the match is played by the
//! engine, and he watches it on the tilted board of the 2.5D view with the
//! commentary and the reasons for every goal conceded.
//!
//! `ds-gui [--theme Lavagna|Notte|Carta] [--tilt DEG] [--seed N] [--at TICK] [--speed X] [--camera whole|close|above] [--slow] [--record DIR --frames N]` It reads Plane B only (ADR-022 §3).

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
            "--camera" => {
                s.camera = Some(match value()?.as_str() {
                    "whole" => app::Camera::Whole,
                    "close" => app::Camera::Close,
                    "above" => app::Camera::Above,
                    other => return Err(format!("--camera: {other}: whole, close or above")),
                });
            }
            "--slow" => s.slow = true,
            "--record" => {
                let dir = value()?;
                s.record = Some((dir, s.record.as_ref().map_or(600, |r| r.1)));
            }
            "--frames" => {
                let n = value()?.parse().map_err(|e| format!("--frames: {e}"))?;
                let dir = s.record.take().map_or_else(|| ".".to_string(), |r| r.0);
                s.record = Some((dir, n));
            }
            "--help" | "-h" => {
                println!(
                    "ds-gui [--theme Lavagna|Notte|Carta] [--tilt DEG] [--seed N] [--at TICK] [--speed X] [--camera whole|close|above] [--slow] [--record DIR --frames N]"
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
