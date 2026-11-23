//! The registry of the match's plugins (ADR-063 §1, plan `PLUGINS.md`
//! step 3).
//!
//! Each of the eleven mandatory plugins has its card here, with the
//! phase it acts in and what of the observable state it reads and
//! writes, read from the code. Each plugin is a module of
//! `crate::plugins` with its type and its card (step 4, done 2026-11-15).
//! The tick does not call them in the order of the phases yet: see the
//! note of `ds_match_api::phase` (code audit B9); a uniform call per phase
//! through the registry waits for the observable state as a type of
//! `ds-match-api`.

use ds_match_api::{Phase, PluginCard, Weight};

/// The cards of the mandatory plugins, in the order of `ds_match_api::MANDATORY`.
pub const CARDS: [PluginCard; 11] = [
    PluginCard {
        name: "ball",
        version: "1",
        phases: &[Phase::Move],
        reads: &["ball", "pitch"],
        writes: &["ball"],
        weight: Weight::Core,
        narrative: &["woodwork"],
    },
    PluginCard {
        name: "body",
        version: "1",
        phases: &[Phase::Move, Phase::ResolveContacts],
        reads: &["positions", "acts", "bodies"],
        writes: &["positions", "velocities"],
        weight: Weight::Core,
        narrative: &[],
    },
    PluginCard {
        name: "perception",
        version: "1",
        phases: &[Phase::Perceive],
        reads: &["positions", "gaze"],
        writes: &["memory", "gaze"],
        weight: Weight::Core,
        narrative: &[],
    },
    PluginCard {
        name: "thought",
        version: "1",
        phases: &[Phase::DecideOffBall, Phase::DecideOnBall],
        reads: &["memory", "ball", "intention"],
        writes: &["acts", "choice"],
        weight: Weight::Core,
        narrative: &[],
    },
    PluginCard {
        name: "technique",
        version: "1",
        phases: &[Phase::Act, Phase::ResolveContacts],
        reads: &["choice", "ball", "positions"],
        writes: &["ball"],
        weight: Weight::Core,
        narrative: &["pass", "shot", "cross"],
    },
    PluginCard {
        name: "duels",
        version: "1",
        phases: &[Phase::ResolveContacts],
        reads: &["positions", "acts", "ball"],
        writes: &["ball", "possession"],
        weight: Weight::Core,
        narrative: &["duel", "dribble", "tackle"],
    },
    PluginCard {
        name: "keeper",
        version: "1",
        phases: &[Phase::DecideOffBall, Phase::ResolveContacts],
        reads: &["ball", "positions"],
        writes: &["ball", "possession"],
        weight: Weight::Core,
        narrative: &["save"],
    },
    PluginCard {
        name: "referee",
        version: "1",
        phases: &[Phase::Referee],
        reads: &["ball", "positions", "contacts"],
        writes: &["restart", "cards", "score"],
        weight: Weight::Core,
        narrative: &["foul", "card", "goal", "offside"],
    },
    PluginCard {
        name: "set_pieces",
        version: "1",
        phases: &[Phase::Referee],
        reads: &["restart", "positions"],
        writes: &["positions", "ball"],
        weight: Weight::Core,
        narrative: &["corner", "free_kick", "throw_in", "goal_kick", "penalty"],
    },
    PluginCard {
        name: "bench",
        version: "1",
        phases: &[Phase::Referee],
        reads: &["stoppage", "bodies", "instructions"],
        writes: &["squads", "intention"],
        weight: Weight::Core,
        narrative: &["substitution", "instruction"],
    },
    PluginCard {
        name: "fatigue",
        version: "1",
        phases: &[Phase::Move],
        // The characteristic it reads through `World::characteristic` is
        // declared here, and `anti_leak` checks it (ADR-063 §4, ADR-112).
        reads: &["velocities", "endurance", "condition"],
        writes: &["bodies"],
        weight: Weight::Core,
        narrative: &[],
    },
];

/// The cards of the plugins that may be switched off (ADR-063 §4).
///
/// In the order of `ds_match_api::OPTIONAL`. The environment has no code
/// yet: its card says what it will read and write (the weather and the
/// pitch). The knocks are made by the referee on the force of a contact,
/// as the actor reads it on the body (`GroundTruth::contact_force_milli`,
/// ADR-112), and the bench (OI-110); they cannot be switched off yet.
pub const OPTIONAL_CARDS: [PluginCard; 3] = [
    PluginCard {
        name: "psychology",
        version: "1",
        phases: &[Phase::DecideOffBall, Phase::Act],
        reads: &["courage", "composure", "bodies", "pressure"],
        writes: &["contest", "error"],
        weight: Weight::Detail,
        narrative: &[],
    },
    PluginCard {
        name: "environment",
        version: "0",
        phases: &[Phase::Move],
        reads: &["pitch", "weather"],
        writes: &["ball"],
        weight: Weight::Detail,
        narrative: &[],
    },
    PluginCard {
        name: "knocks",
        version: "0",
        phases: &[Phase::ResolveContacts],
        reads: &["contacts", "bodies"],
        writes: &["squads"],
        weight: Weight::Detail,
        narrative: &["injury"],
    },
];

/// The card of the plugin called `name`; for a name the registry does not
/// hold, a card with no phase, which `PluginCard::is_valid` refuses (the
/// test below proves the eleven of the contract are all here).
#[must_use]
pub fn card(name: &str) -> PluginCard {
    CARDS
        .iter()
        .chain(OPTIONAL_CARDS.iter())
        .find(|c| c.name == name)
        .cloned()
        .unwrap_or(PluginCard {
            name: "unknown",
            version: "0",
            phases: &[],
            reads: &[],
            writes: &[],
            weight: Weight::Detail,
            narrative: &[],
        })
}

#[cfg(test)]
mod tests {
    use super::CARDS;

    /// P1 (ADR-063 §4): the registry holds the eleven mandatory plugins,
    /// each with a valid card, in the contract's order.
    #[test]
    fn the_registry_holds_the_eleven_mandatory_plugins() {
        let names: Vec<&str> = CARDS.iter().map(|c| c.name).collect();
        assert_eq!(names, ds_match_api::MANDATORY);
        assert!(CARDS.iter().all(ds_match_api::PluginCard::is_valid));
        let optional: Vec<&str> = super::OPTIONAL_CARDS.iter().map(|c| c.name).collect();
        assert_eq!(optional, ds_match_api::OPTIONAL);
        assert!(
            super::OPTIONAL_CARDS
                .iter()
                .all(|c| c.is_valid() && c.weight == ds_match_api::Weight::Detail)
        );
    }
}
