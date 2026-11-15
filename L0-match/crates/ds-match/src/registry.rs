//! The registry of the match's plugins (ADR-063 §1, plan `PLUGINS.md`
//! step 3).
//!
//! Each of the eleven mandatory plugins has its card here, with the
//! phase it acts in and what of the observable state it reads and
//! writes, read from the code of today. The code itself is still inside
//! this crate; the plugins move behind the contract one at a time (step
//! 4), and the tick will then call them through this registry, phase by
//! phase.

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
        reads: &["velocities"],
        writes: &["bodies"],
        weight: Weight::Core,
        narrative: &[],
    },
];

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
    }
}
