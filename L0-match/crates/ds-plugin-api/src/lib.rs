//! `ds-plugin-api` — the card of a plugin and its validator (ADR-113 §3).
//!
//! One card for the match's plugins (`ds-match-api`, the phases of a tick)
//! and for the season's (`ds-season-api`, the moments of a day), and one
//! validator (decision AQ): a name, a version, when it acts, what it needs
//! before it, what it reads (each reading mandatory or optional, with what it
//! does without), what it writes, its tier (decision X) and its declared
//! cost.
//!
//! # Design rules
//!
//! - No dependency on any `ds-*` crate (ADR-063 §3).
//! - No I/O, no clock, no randomness (ADR-035).

use std::collections::BTreeSet;

/// The tier of a plugin (decision X): what the player may switch off.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tier {
    /// Always loaded: without it there is no match or no season.
    Core,
    /// On by default, may be taken out (in the match: psychology,
    /// environment, knocks; in L1: body, discipline, week).
    Recommended,
    /// Off by default, may be added (contributors' plugins, heavy or niche
    /// modules).
    Optional,
}

/// A reading a plugin may go without, and what it does then (decision B).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptionalRead {
    /// What it reads.
    pub what: &'static str,
    /// What it does when it is not there.
    pub without: &'static str,
}

/// What a plugin says about itself, for the registry and the validator.
/// `S` is when it acts: the phases of a tick or the moments of a day.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginCard<S: 'static> {
    /// Its name, unique in the registry.
    pub name: &'static str,
    /// Its version; a change of behaviour is a new `SIM_VERSION` or
    /// `SEASON_VERSION` too.
    pub version: &'static str,
    /// When it acts.
    pub steps: &'static [S],
    /// The plugins that act before it (ADR-113 §4).
    pub depends: &'static [&'static str],
    /// What of the observable state it reads, and must find.
    pub reads: &'static [&'static str],
    /// What it reads if it is there (a plugin that may be switched off).
    pub optional_reads: &'static [OptionalRead],
    /// What of the observable state it writes (its own tables).
    pub writes: &'static [&'static str],
    /// Its tier (decision X).
    pub tier: Tier,
    /// Its declared cost, microseconds a match or a day (0: not measured
    /// yet), for the screen that shows what each choice costs.
    pub cost_us: u32,
    /// The keys of the commentary lines it gives.
    pub narrative: &'static [&'static str],
}

impl<S> PluginCard<S> {
    /// Whether the card is well formed: a name, a version, at least one
    /// step, and it does not depend on itself.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.name.is_empty()
            && !self.version.is_empty()
            && !self.steps.is_empty()
            && !self.depends.contains(&self.name)
    }
}

/// Why a set of cards is refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardError {
    /// A card is not well formed.
    Invalid(&'static str),
    /// Two cards have the same name.
    Duplicate(&'static str),
    /// A card depends on a plugin that is not loaded: (plugin, missing).
    MissingDependency(&'static str, &'static str),
    /// A core plugin depends on one that may be switched off: (core, other).
    CoreOnOptional(&'static str, &'static str),
    /// The dependencies make a cycle; the plugins left in it.
    Cycle(Vec<&'static str>),
}

/// The one validator (ADR-113 §3, decision AQ) and the order (§4): the
/// indices of `cards` in the order they act, each after what it depends
/// on, ties by name. The same cards give the same order, always.
///
/// # Errors
///
/// A [`CardError`] for the first fault found.
pub fn order<S>(cards: &[PluginCard<S>]) -> Result<Vec<usize>, CardError> {
    let mut names = BTreeSet::new();
    for c in cards {
        if !c.is_valid() {
            return Err(CardError::Invalid(c.name));
        }
        if !names.insert(c.name) {
            return Err(CardError::Duplicate(c.name));
        }
    }
    let index = |n: &str| cards.iter().position(|c| c.name == n);
    for c in cards {
        for &d in c.depends {
            let Some(i) = index(d) else {
                return Err(CardError::MissingDependency(c.name, d));
            };
            if c.tier == Tier::Core && cards[i].tier != Tier::Core {
                return Err(CardError::CoreOnOptional(c.name, d));
            }
        }
    }
    // Kahn's order, always taking the ready plugin with the smallest name.
    let mut done: Vec<usize> = Vec::with_capacity(cards.len());
    let mut left: BTreeSet<(&'static str, usize)> =
        cards.iter().enumerate().map(|(i, c)| (c.name, i)).collect();
    while !left.is_empty() {
        let ready = left.iter().copied().find(|&(_, i)| {
            cards[i]
                .depends
                .iter()
                .all(|d| done.iter().any(|&j| cards[j].name == *d))
        });
        let Some(next) = ready else {
            return Err(CardError::Cycle(left.iter().map(|(n, _)| *n).collect()));
        };
        left.remove(&next);
        done.push(next.1);
    }
    Ok(done)
}

#[cfg(test)]
mod tests {
    use super::{CardError, PluginCard, Tier, order};

    const fn card(
        name: &'static str,
        depends: &'static [&'static str],
        tier: Tier,
    ) -> PluginCard<u8> {
        PluginCard {
            name,
            version: "1",
            steps: &[0],
            depends,
            reads: &[],
            optional_reads: &[],
            writes: &[],
            tier,
            cost_us: 0,
            narrative: &[],
        }
    }

    #[test]
    fn a_card_needs_a_name_a_version_and_a_step() {
        let good = card("fatigue", &[], Tier::Core);
        assert!(good.is_valid());
        let no_step = PluginCard { steps: &[], ..good };
        assert!(!no_step.is_valid());
        let itself = PluginCard {
            depends: &["fatigue"],
            ..good
        };
        assert!(!itself.is_valid());
    }

    /// Each after what it depends on, ties by name, whatever the order of
    /// the registry.
    #[test]
    fn the_order_follows_the_dependencies_then_the_names() {
        let cards = [
            card("standings", &["matchday"], Tier::Core),
            card("matchday", &["calendar"], Tier::Core),
            card("body", &[], Tier::Recommended),
            card("calendar", &[], Tier::Core),
        ];
        let names: Vec<_> = order(&cards)
            .unwrap()
            .into_iter()
            .map(|i| cards[i].name)
            .collect();
        assert_eq!(names, ["body", "calendar", "matchday", "standings"]);
        let mut shuffled = cards.clone();
        shuffled.reverse();
        let again: Vec<_> = order(&shuffled)
            .unwrap()
            .into_iter()
            .map(|i| shuffled[i].name)
            .collect();
        assert_eq!(again, names);
    }

    #[test]
    fn the_validator_refuses_what_cannot_run() {
        let dup = [card("a", &[], Tier::Core), card("a", &[], Tier::Core)];
        assert_eq!(order(&dup), Err(CardError::Duplicate("a")));
        let missing = [card("a", &["b"], Tier::Core)];
        assert_eq!(order(&missing), Err(CardError::MissingDependency("a", "b")));
        let core_on_optional = [
            card("a", &["b"], Tier::Core),
            card("b", &[], Tier::Optional),
        ];
        assert_eq!(
            order(&core_on_optional),
            Err(CardError::CoreOnOptional("a", "b"))
        );
        let cycle = [card("a", &["b"], Tier::Core), card("b", &["a"], Tier::Core)];
        assert_eq!(order(&cycle), Err(CardError::Cycle(vec!["a", "b"])));
    }
}
