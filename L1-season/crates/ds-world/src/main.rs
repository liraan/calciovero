//! `ds-world` — the world pack from the command line.
//!
//! `ds-world export <folder> [--seed N]`: writes the shipped pack with every
//! field filled from the seed (a whole world to retouch).
//! `ds-world check <folder>`: validates a pack, and says where and why.

use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let seed = args
        .iter()
        .position(|a| a == "--seed")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(42);
    match (args.first().map(String::as_str), args.get(1)) {
        (Some("export"), Some(dir)) => {
            let pack = ds_world::Pack::v1();
            let players: Vec<ds_world::PlayerEntry> = (0..u32::try_from(pack.clubs.len())
                .unwrap_or(0))
                .flat_map(|club| {
                    (0..u32::try_from(pack.squad_roles.len()).unwrap_or(0))
                        .map(move |number| (club, number))
                })
                .map(|(club, number)| ds_world::PlayerEntry {
                    club,
                    number,
                    name: Some(pack.player_name(seed, u64::from(club * 100 + number))),
                    role: None,
                    born: None,
                })
                .collect();
            for (name, text) in pack.export(seed, &players) {
                let path = Path::new(dir).join(&name);
                if let Some(parent) = path.parent() {
                    if std::fs::create_dir_all(parent).is_err() {
                        eprintln!("cannot create {}", parent.display());
                        return ExitCode::FAILURE;
                    }
                }
                if std::fs::write(&path, text).is_err() {
                    eprintln!("cannot write {}", path.display());
                    return ExitCode::FAILURE;
                }
            }
            println!("world written to {dir} (seed {seed})");
            ExitCode::SUCCESS
        }
        (Some("check"), Some(dir)) => match ds_world::Pack::load_dir(Path::new(dir)) {
            Ok(p) => {
                println!("ok: {}, {} clubs", p.nation, p.clubs.len());
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("{e}");
                ExitCode::FAILURE
            }
        },
        _ => {
            eprintln!("usage: ds-world export <folder> [--seed N] | ds-world check <folder>");
            ExitCode::FAILURE
        }
    }
}
