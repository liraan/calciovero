//! The GUI's colours, read from `theme.json` (ADR-055; foundations §5: no
//! hardcoded colours).

use eframe::egui::Color32;
use serde::Deserialize;

/// The embedded theme.
const THEME_JSON: &str = include_str!("../theme.json");

/// The theme as written in the file: every colour a hex string.
#[derive(Debug, Clone, Deserialize)]
struct RawTheme {
    pitch: String,
    pitch_stripe: String,
    lines: String,
    home: String,
    away: String,
    keeper_home: String,
    keeper_away: String,
    player_text: String,
    carrier_ring: String,
    ball: String,
    ball_shadow: String,
    referee: String,
    critical: String,
    high: String,
    medium: String,
    low: String,
}

/// The colours the GUI paints with.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    /// The grass.
    pub pitch: Color32,
    /// The grass of every other band.
    pub pitch_stripe: Color32,
    /// The markings.
    pub lines: Color32,
    /// The home side.
    pub home: Color32,
    /// The away side.
    pub away: Color32,
    /// The home goalkeeper.
    pub keeper_home: Color32,
    /// The away goalkeeper.
    pub keeper_away: Color32,
    /// The letter on a player.
    pub player_text: Color32,
    /// The ring around the man on the ball.
    pub carrier_ring: Color32,
    /// The ball.
    pub ball: Color32,
    /// The ball's shadow on the grass.
    pub ball_shadow: Color32,
    /// The referee.
    pub referee: Color32,
    /// Commentary: a goal, a penalty.
    pub critical: Color32,
    /// Commentary: a save, a card.
    pub high: Color32,
    /// Commentary: a shot, a foul.
    pub medium: Color32,
    /// Commentary: the rest.
    pub low: Color32,
}

/// A colour from `#rrggbb` or `#rrggbbaa`.
///
/// # Errors
///
/// If the string is not one of the two forms.
pub fn parse_hex(s: &str) -> Result<Color32, String> {
    let h = s
        .strip_prefix('#')
        .ok_or_else(|| format!("{s}: no leading #"))?;
    let byte = |i: usize| {
        h.get(i..i + 2)
            .and_then(|b| u8::from_str_radix(b, 16).ok())
            .ok_or_else(|| format!("{s}: not a hex colour"))
    };
    match h.len() {
        6 => Ok(Color32::from_rgb(byte(0)?, byte(2)?, byte(4)?)),
        8 => Ok(Color32::from_rgba_unmultiplied(
            byte(0)?,
            byte(2)?,
            byte(4)?,
            byte(6)?,
        )),
        _ => Err(format!("{s}: expected #rrggbb or #rrggbbaa")),
    }
}

impl Theme {
    /// The theme from a JSON text.
    ///
    /// # Errors
    ///
    /// If the JSON or a colour is invalid.
    pub fn from_json(text: &str) -> Result<Self, String> {
        let r: RawTheme = serde_json::from_str(text).map_err(|e| e.to_string())?;
        Ok(Self {
            pitch: parse_hex(&r.pitch)?,
            pitch_stripe: parse_hex(&r.pitch_stripe)?,
            lines: parse_hex(&r.lines)?,
            home: parse_hex(&r.home)?,
            away: parse_hex(&r.away)?,
            keeper_home: parse_hex(&r.keeper_home)?,
            keeper_away: parse_hex(&r.keeper_away)?,
            player_text: parse_hex(&r.player_text)?,
            carrier_ring: parse_hex(&r.carrier_ring)?,
            ball: parse_hex(&r.ball)?,
            ball_shadow: parse_hex(&r.ball_shadow)?,
            referee: parse_hex(&r.referee)?,
            critical: parse_hex(&r.critical)?,
            high: parse_hex(&r.high)?,
            medium: parse_hex(&r.medium)?,
            low: parse_hex(&r.low)?,
        })
    }

    /// The embedded theme.
    ///
    /// # Errors
    ///
    /// If `theme.json` is invalid; `the_embedded_theme_is_valid` catches it
    /// at test time.
    pub fn load() -> Result<Self, String> {
        Self::from_json(THEME_JSON)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_embedded_theme_is_valid() {
        assert!(Theme::load().is_ok());
    }

    #[test]
    fn hex_colours_parse_with_and_without_alpha() {
        assert_eq!(parse_hex("#ff0000"), Ok(Color32::from_rgb(255, 0, 0)));
        assert_eq!(
            parse_hex("#00000080"),
            Ok(Color32::from_rgba_unmultiplied(0, 0, 0, 128))
        );
        assert!(parse_hex("ff0000").is_err());
        assert!(parse_hex("#ff00").is_err());
    }
}
