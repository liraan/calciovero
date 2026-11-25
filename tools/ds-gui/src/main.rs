//! `ds-gui` — Calcio Vero's desktop GUI (ADR-055).
//!
//! The game (decision BN): the start, a new career or the saved one, the
//! desk of the career with its windows (decision BO), the match in the
//! coach's tablet; and the friendly: the manager chooses, the match is
//! played by the engine, and he watches it on the tilted board of the 2.5D
//! view with the commentary and the reasons for every goal conceded.
//!
//! `ds-gui [--career SEED:CLUB] [--theme Lavagna|Notte|Carta] [--tilt DEG] [--seed N] [--at TICK] [--speed X] [--zoom 0..1] [--free] [--slow] [--record DIR --frames N]` It reads Plane B only (ADR-022 §3).

mod animation;
mod app;
mod desk;
mod interp;
mod scene;
mod shell;
mod stadium;
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
            "--zoom" => s.zoom = Some(value()?.parse().map_err(|e| format!("--zoom: {e}"))?),
            "--free" => s.follow = Some(false),
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
            "--career" => {
                let v = value()?;
                let (seed, club) = v.split_once(':').ok_or("--career: SEED:CLUB")?;
                s.career = Some((
                    seed.parse().map_err(|e| format!("--career: {e}"))?,
                    club.parse().map_err(|e| format!("--career: {e}"))?,
                ));
            }
            "--help" | "-h" => {
                println!(
                    "ds-gui [--career SEED:CLUB] [--theme Lavagna|Notte|Carta] [--tilt DEG] [--seed N] [--at TICK] [--speed X] [--zoom 0..1] [--free] [--slow] [--record DIR --frames N]"
                );
                std::process::exit(0);
            }
            other => return Err(format!("unknown option {other}")),
        }
    }
    Ok(s)
}

/// The crash report (`GAME.md`, 0.1: "il file di segnalazione"): a panic is
/// written, with where it happened, to `CalcioVero/crash.log` in the
/// user's data folder, besides the console.
fn report_crashes() {
    let Some(base) = std::env::var_os("APPDATA")
        .map(std::path::PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME").map(|h| std::path::Path::new(&h).join(".local/share"))
        })
    else {
        return;
    };
    let dir = base.join("CalcioVero");
    let default = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = std::fs::create_dir_all(&dir);
        let text = format!("{info}\n{}\n", std::backtrace::Backtrace::force_capture());
        let _ = std::fs::write(dir.join("crash.log"), text);
        default(info);
    }));
}

fn main() -> eframe::Result {
    report_crashes();
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
    // With no option, the game: the shell (decision BN). With options, the
    // friendly as the command line asks (the README's recording).
    let career = start.career;
    let plain = app::Start {
        career: None,
        ..start.clone()
    } == app::Start::default();
    eframe::run_native(
        "Calcio Vero",
        options,
        Box::new(move |_cc| {
            if let Some((seed, club)) = career {
                Ok(Box::new(shell::Shell::with_career(theme, seed, club)))
            } else if plain {
                Ok(Box::new(shell::Shell::new(theme)))
            } else {
                Ok(Box::new(app::App::new(theme).with_start(&start)))
            }
        }),
    )
}
