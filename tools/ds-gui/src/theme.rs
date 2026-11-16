//! The GUI's colours, read from `theme.json` (ADR-055; foundations §5: no
//! hardcoded colours).

use eframe::egui::Color32;
use serde::Deserialize;

/// The embedded theme.
const THEME_JSON: &str = include_str!("../theme.json");

/// The file: the themes, the default first.
#[derive(Debug, Clone, Deserialize)]
struct RawFile {
    themes: Vec<RawTheme>,
}

/// The stadium and the town round the pitch, as written in the file.
#[derive(Debug, Clone, Deserialize)]
struct RawStadium {
    sky: String,
    horizon: String,
    ground: String,
    track: String,
    concrete: String,
    roof: String,
    crowd: String,
    mast: String,
    lamps: String,
    city: [String; 4],
    window: String,
}

/// The colours of the stadium and of its town.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stadium {
    /// The sky at the top.
    pub sky: Color32,
    /// The sky at the horizon.
    pub horizon: Color32,
    /// The ground beyond the stadium.
    pub ground: Color32,
    /// The grass between the pitch and the stands.
    pub track: Color32,
    /// The stands' walls.
    pub concrete: Color32,
    /// The roofs.
    pub roof: Color32,
    /// The crowd's neutral clothes (the rest wear the sides' colours).
    pub crowd: Color32,
    /// The floodlights' masts.
    pub mast: Color32,
    /// The floodlights' lamps.
    pub lamps: Color32,
    /// The buildings: front, side, back, roof.
    pub city: [Color32; 4],
    /// A lit window.
    pub window: Color32,
}

/// A theme as written in the file: every colour a hex string.
#[derive(Debug, Clone, Deserialize)]
struct RawTheme {
    stadium: RawStadium,
    name: String,
    dark: bool,
    board: String,
    pitch: String,
    pitch_stripe: String,
    lines: String,
    home: String,
    away: String,
    keeper_home: String,
    keeper_away: String,
    player_text: String,
    carrier_ring: String,
    skin: String,
    ball: String,
    ball_shadow: String,
    referee: String,
    critical: String,
    high: String,
    medium: String,
    low: String,
}

/// The colours the GUI paints with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    /// Its name, as the window shows it.
    pub name: String,
    /// The interface around the board is dark.
    pub dark: bool,
    /// The background around the pitch.
    pub board: Color32,
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
    /// A figure's head.
    pub skin: Color32,
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
    /// The stadium and its town.
    pub stadium: Stadium,
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
    fn from_raw(r: &RawTheme) -> Result<Self, String> {
        Ok(Self {
            name: r.name.clone(),
            dark: r.dark,
            board: parse_hex(&r.board)?,
            pitch: parse_hex(&r.pitch)?,
            pitch_stripe: parse_hex(&r.pitch_stripe)?,
            lines: parse_hex(&r.lines)?,
            home: parse_hex(&r.home)?,
            away: parse_hex(&r.away)?,
            keeper_home: parse_hex(&r.keeper_home)?,
            keeper_away: parse_hex(&r.keeper_away)?,
            player_text: parse_hex(&r.player_text)?,
            carrier_ring: parse_hex(&r.carrier_ring)?,
            skin: parse_hex(&r.skin)?,
            ball: parse_hex(&r.ball)?,
            ball_shadow: parse_hex(&r.ball_shadow)?,
            referee: parse_hex(&r.referee)?,
            critical: parse_hex(&r.critical)?,
            high: parse_hex(&r.high)?,
            medium: parse_hex(&r.medium)?,
            low: parse_hex(&r.low)?,
            stadium: {
                let s = &r.stadium;
                Stadium {
                    sky: parse_hex(&s.sky)?,
                    horizon: parse_hex(&s.horizon)?,
                    ground: parse_hex(&s.ground)?,
                    track: parse_hex(&s.track)?,
                    concrete: parse_hex(&s.concrete)?,
                    roof: parse_hex(&s.roof)?,
                    crowd: parse_hex(&s.crowd)?,
                    mast: parse_hex(&s.mast)?,
                    lamps: parse_hex(&s.lamps)?,
                    city: [
                        parse_hex(&s.city[0])?,
                        parse_hex(&s.city[1])?,
                        parse_hex(&s.city[2])?,
                        parse_hex(&s.city[3])?,
                    ],
                    window: parse_hex(&s.window)?,
                }
            },
        })
    }

    /// The themes from a JSON text.
    ///
    /// # Errors
    ///
    /// If the JSON or a colour is invalid, or there is no theme.
    pub fn all_from_json(text: &str) -> Result<Vec<Self>, String> {
        let f: RawFile = serde_json::from_str(text).map_err(|e| e.to_string())?;
        let themes = f
            .themes
            .iter()
            .map(Self::from_raw)
            .collect::<Result<Vec<_>, _>>()?;
        if themes.is_empty() {
            return Err("no theme".to_string());
        }
        Ok(themes)
    }

    /// The embedded themes, the default first.
    ///
    /// # Errors
    ///
    /// If `theme.json` is invalid; `the_embedded_themes_are_valid` catches it
    /// at test time.
    pub fn all() -> Result<Vec<Self>, String> {
        Self::all_from_json(THEME_JSON)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The three themes of the board (DESIGN.md), the default first.
    #[test]
    fn the_embedded_themes_are_valid() {
        let themes = Theme::all().expect("themes");
        let names: Vec<&str> = themes.iter().map(|t| t.name.as_str()).collect();
        assert_eq!(names, ["Lavagna", "Notte", "Carta"]);
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
