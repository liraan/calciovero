//! Rendering of a `MatchResult` for human consumption.
//!
//! Two renderers:
//!
//! - `print_text`: a human-readable text representation (score, statistics,
//!   events, narrative).
//! - `print_json`: the `MatchResult` as JSON (for scripting and for
//!   integration with the future `pcfm-web`).
//!
//! Plus `write_html`: a self-contained HTML page (no external assets) that
//! can be opened in any browser.

use std::fmt::Write as _;

use ds_match::{MatchResult, NarrativeEntry};

/// Prints the match result as human-readable text.
///
/// # Errors
///
/// Never fails: returns `Ok(())` for symmetry with the other renderers.
pub fn print_text(result: &MatchResult) -> std::fmt::Result {
    let mut out = String::new();

    writeln!(out, "=== MATCH RESULT ===")?;
    writeln!(
        out,
        "Score: Home {} - {} Away",
        result.score.home, result.score.away
    )?;
    writeln!(
        out,
        "Possession: {:.1}% - {:.1}%",
        result.stats.possession.0 * 100.0,
        result.stats.possession.1 * 100.0
    )?;
    writeln!(
        out,
        "Shots: {} - {}",
        result.stats.shots.0, result.stats.shots.1
    )?;
    writeln!(out, "Events: {}", result.events.len())?;
    writeln!(out, "State hash: {}", hex(result.state_hash.as_bytes()))?;
    writeln!(out)?;
    writeln!(out, "=== NARRATIVE ===")?;
    for entry in &result.narrative {
        writeln!(out, "{}", format_narrative(entry))?;
    }

    print!("{out}");
    Ok(())
}

/// Prints the match result as JSON (one line, for scripting).
///
/// # Errors
///
/// Returns the serialisation error if `serde_json` fails. By construction,
/// `MatchResult` is serialisable, so this is unreachable in practice.
pub fn print_json(result: &MatchResult) -> Result<(), serde_json::Error> {
    let json = serde_json::to_string_pretty(result)?;
    println!("{json}");
    Ok(())
}

/// Formats a single narrative entry as `[minute] text`.
fn format_narrative(entry: &NarrativeEntry) -> String {
    format!("[{:>3}'] {}", entry.minute, entry.text)
}

/// Returns the lowercase hex representation of `bytes`.
fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(s, "{b:02x}");
    }
    s
}

/// Writes a self-contained HTML page to `path`.
///
/// The page contains inline CSS and inline JSON data. No external assets:
/// it works offline, from a `file://` URL, and from an email attachment.
///
/// # Errors
///
/// Returns an I/O error if the file cannot be written.
/// Validates that `path` stays inside the current working directory.
///
/// Rejects absolute paths and any `..` component. Canonicalises the parent
/// directory (which must exist) and checks that it is a descendant of the
/// current directory, so symlinks cannot escape either.
///
/// # Errors
///
/// Returns `InvalidInput` for an absolute path or a `..` component, and
/// `PermissionDenied` if the resolved parent escapes the working directory.
fn sanitize_output_path(path: &str) -> std::io::Result<std::path::PathBuf> {
    use std::path::{Component, Path};

    let p = Path::new(path);
    if p.is_absolute() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "absolute paths are not allowed; use a path relative to the current directory",
        ));
    }
    for comp in p.components() {
        if matches!(comp, Component::ParentDir) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "path traversal ('..') is not allowed",
            ));
        }
    }
    let parent = p
        .parent()
        .filter(|d| !d.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let file_name = p.file_name().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "path has no file name")
    })?;
    let canon_parent = std::fs::canonicalize(parent)?;
    let cwd = std::fs::canonicalize(std::env::current_dir()?)?;
    if !canon_parent.starts_with(&cwd) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "output path escapes the current working directory",
        ));
    }
    Ok(canon_parent.join(file_name))
}
pub fn write_html(result: &MatchResult, path: &str) -> std::io::Result<()> {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\">");
    html.push_str("<title>PCFM match</title>");
    html.push_str("<style>");
    html.push_str(
        "body{font-family:system-ui,sans-serif;max-width:900px;margin:2em auto;\
         padding:0 1em;color:#222;background:#fafafa}\
         h1{color:#0a3d62}h2{color:#1e5f8c;border-bottom:1px solid #ddd;padding-bottom:.2em}\
         .score{font-size:2em;font-weight:bold;color:#0a3d62}\
         .stats td{padding:.2em .8em}\
         .narrative li{padding:.2em 0}\
         .minute{display:inline-block;width:3em;color:#888}\
         .critical{font-weight:bold;color:#c0392b}\
         .high{color:#e67e22}\
         .medium{color:#2980b9}\
         .low{color:#7f8c8d}",
    );
    html.push_str("</style></head><body>");
    let _ = write!(
        html,
        "<h1>Match: Home {} - {} Away</h1>",
        result.score.home, result.score.away
    );
    let _ = write!(
        html,
        "<p class=\"score\">Possession: {:.1}% - {:.1}%</p>",
        result.stats.possession.0 * 100.0,
        result.stats.possession.1 * 100.0
    );
    html.push_str("<h2>Statistics</h2><table class=\"stats\">");
    let _ = write!(
        html,
        "<tr><td>Shots</td><td>{}</td><td>{}</td></tr>",
        result.stats.shots.0, result.stats.shots.1
    );
    let _ = write!(
        html,
        "<tr><td>Goals</td><td>{}</td><td>{}</td></tr>",
        result.stats.goals.0, result.stats.goals.1
    );
    html.push_str("</table>");
    html.push_str("<h2>Narrative</h2><ul class=\"narrative\">");
    for entry in &result.narrative {
        let cls = match entry.importance {
            ds_match::Importance::Critical => "critical",
            ds_match::Importance::High => "high",
            ds_match::Importance::Medium => "medium",
            ds_match::Importance::Low => "low",
        };
        let _ = write!(
            html,
            "<li class=\"{cls}\"><span class=\"minute\">{}'</span> {}</li>",
            entry.minute,
            html_escape(&entry.text)
        );
    }
    html.push_str("</ul>");
    let _ = write!(
        html,
        "<p>Events: {} · State hash: {}</p>",
        result.events.len(),
        hex(result.state_hash.as_bytes())
    );
    html.push_str("</body></html>");

    let safe_path = sanitize_output_path(path)?;
    std::fs::write(safe_path, html)
}

/// Minimal HTML escaping: `&`, `<`, `>`.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_rejects_absolute() {
        assert!(sanitize_output_path("/tmp/x.html").is_err());
    }

    #[test]
    fn sanitize_rejects_parent_dir() {
        assert!(sanitize_output_path("../x.html").is_err());
    }

    #[test]
    fn sanitize_accepts_relative() {
        assert!(sanitize_output_path("x.html").is_ok());
    }

    #[test]
    fn hex_of_zero_is_all_zeros() {
        let h = hex(&[0u8; 4]);
        assert_eq!(h, "00000000");
    }

    #[test]
    fn html_escape_works() {
        assert_eq!(html_escape("<a>&b</a>"), "&lt;a&gt;&amp;b&lt;/a&gt;");
    }

    #[test]
    fn format_narrative_pads_minute() {
        let entry = NarrativeEntry {
            minute: 7,
            event_id: ds_core::ids::EventId::new(1, 0),
            text: "GOAL!".to_string(),
            importance: ds_match::Importance::Critical,
        };
        assert_eq!(format_narrative(&entry), "[  7'] GOAL!");
    }
}
