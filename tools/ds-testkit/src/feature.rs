//! Feature identifiers for scenario registration and coverage.

use std::borrow::Cow;

/// A feature of the PCFM framework that a [`Scenario`](crate::Scenario) can
/// exercise.
///
/// The enum is `#[non_exhaustive]` so new variants can be added without
/// breaking downstream code. External plugins should use
/// [`Feature::Plugin`] with their own identifier.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Feature {
    // ── Core primitives ─────────────────────────────────────────────
    /// The RNG registry (`ds-core::rng`).
    CoreRng,
    /// The event bus (`ds-core::event_bus`).
    CoreEventBus,
    /// Entity identifiers (`ds-core::ids`).
    CoreIds,
    /// Hashing primitives (`ds-core::hash`).
    CoreHash,

    // ── L0 — Match ──────────────────────────────────────────────────
    /// The match engine (`ds-match`).
    L0Match,
    /// Save/load (`MatchSave`).
    L0Save,
    /// Ground truth generation (`pcfm-hidden`).
    L0GroundTruth,

    // ── L1 — Season / World / Roster ────────────────────────────────
    /// The world: nations, cities, stadiums.
    L1World,
    /// The season: competitions, calendar, standings.
    L1Season,
    /// The roster: full player attributes, staff, contracts.
    L1Roster,

    // ── L2 — Career ─────────────────────────────────────────────────
    /// The transfer market.
    L2Market,
    /// Club finances.
    L2Finance,
    /// The youth academy.
    L2Youth,
    /// Scouting and reports.
    L2Scouting,
    /// Player psychology.
    L2Psyche,
    /// Crowd and atmosphere.
    L2Crowd,
    /// Media and press.
    L2Media,
    /// AI managers and clubs.
    L2Ai,

    // ── L3 — Meta ───────────────────────────────────────────────────
    /// The module registry.
    L3Registry,
    /// Pack installation and validation.
    L3Packs,
    /// World generation.
    L3Generator,
    /// The in-game editor.
    L3Editor,
    /// Asset loading.
    L3Assets,
    /// The user interface.
    L3Ui,
    /// Statistics and aggregation.
    L3Statistics,
    /// Quality: performance, determinism, coverage.
    L3Quality,

    // ── External plugin ─────────────────────────────────────────────
    /// A feature contributed by an external plugin (e.g. `"gui"`, `"db"`).
    Plugin(String),
}

impl Feature {
    /// Every built-in feature (excluding [`Feature::Plugin`]).
    ///
    /// Used by `--coverage` to enumerate the features that exist. Plugin
    /// features are discovered dynamically from the registry, not from
    /// this list.
    pub const ALL_BUILTIN: &'static [Self] = &[
        Self::CoreRng,
        Self::CoreEventBus,
        Self::CoreIds,
        Self::CoreHash,
        Self::L0Match,
        Self::L0Save,
        Self::L0GroundTruth,
        Self::L1World,
        Self::L1Season,
        Self::L1Roster,
        Self::L2Market,
        Self::L2Finance,
        Self::L2Youth,
        Self::L2Scouting,
        Self::L2Psyche,
        Self::L2Crowd,
        Self::L2Media,
        Self::L2Ai,
        Self::L3Registry,
        Self::L3Packs,
        Self::L3Generator,
        Self::L3Editor,
        Self::L3Assets,
        Self::L3Ui,
        Self::L3Statistics,
        Self::L3Quality,
    ];

    /// Return every built-in feature.
    #[must_use]
    pub const fn all_builtin() -> &'static [Self] {
        Self::ALL_BUILTIN
    }

    /// Human-readable label, used in reports.
    ///
    /// Returns `Cow::Borrowed` for built-in features and `Cow::Owned` only
    /// for [`Feature::Plugin`], so the common case does not allocate.
    #[must_use]
    pub fn label(&self) -> Cow<'static, str> {
        match self {
            Self::CoreRng => Cow::Borrowed("core::rng"),
            Self::CoreEventBus => Cow::Borrowed("core::event_bus"),
            Self::CoreIds => Cow::Borrowed("core::ids"),
            Self::CoreHash => Cow::Borrowed("core::hash"),
            Self::L0Match => Cow::Borrowed("L0::match"),
            Self::L0Save => Cow::Borrowed("L0::save"),
            Self::L0GroundTruth => Cow::Borrowed("L0::ground_truth"),
            Self::L1World => Cow::Borrowed("L1::world"),
            Self::L1Season => Cow::Borrowed("L1::season"),
            Self::L1Roster => Cow::Borrowed("L1::roster"),
            Self::L2Market => Cow::Borrowed("L2::market"),
            Self::L2Finance => Cow::Borrowed("L2::finance"),
            Self::L2Youth => Cow::Borrowed("L2::youth"),
            Self::L2Scouting => Cow::Borrowed("L2::scouting"),
            Self::L2Psyche => Cow::Borrowed("L2::psyche"),
            Self::L2Crowd => Cow::Borrowed("L2::crowd"),
            Self::L2Media => Cow::Borrowed("L2::media"),
            Self::L2Ai => Cow::Borrowed("L2::ai"),
            Self::L3Registry => Cow::Borrowed("L3::registry"),
            Self::L3Packs => Cow::Borrowed("L3::packs"),
            Self::L3Generator => Cow::Borrowed("L3::generator"),
            Self::L3Editor => Cow::Borrowed("L3::editor"),
            Self::L3Assets => Cow::Borrowed("L3::assets"),
            Self::L3Ui => Cow::Borrowed("L3::ui"),
            Self::L3Statistics => Cow::Borrowed("L3::statistics"),
            Self::L3Quality => Cow::Borrowed("L3::quality"),
            Self::Plugin(name) => Cow::Owned(format!("plugin::{name}")),
        }
    }
}
