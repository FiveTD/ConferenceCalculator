use serde::Serialize;

use super::evidence::Evidence;
use crate::{model::TeamId, repository::ConferenceState};

/// Every tiebreaker rule gets a named ID to help with tracing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RuleId {
    HeadToHead,
    CommonOpponents,
    StandingsWalk,
    OpponentsConferenceRecord,
    TotalWins,
    SportSourceRating,
    CoinToss,
    // TODO
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum NotApplicableReason {
    IncompleteRoundRobin,
    NoGamesAmongTied,
    TooFewCommonOpponents,
    ExternalData, // i.e. SportSource rating
    RandomDraw,
    // TODO ?
}

#[derive(Debug, Clone, Serialize)]
pub enum RuleOutcome {
    /// Ordered subgroups, best first, always 2+
    Separated {
        groups: Vec<Vec<TeamId>>,
        evidence: Vec<Evidence>,
    },
    /// Rule applied, no separation
    NoSeparation { evidence: Vec<Evidence> },
    /// Rule did not apply
    NotApplicable { reason: NotApplicableReason },
}

impl RuleOutcome {
    /// Validator to ensure 2+ groups during separation.
    pub fn from_groups(groups: Vec<Vec<TeamId>>, evidence: Vec<Evidence>) -> Self {
        if groups.len() < 2 {
            RuleOutcome::NoSeparation { evidence }
        } else {
            RuleOutcome::Separated { groups, evidence }
        }
    }
}

pub trait TiebreakRule {
    fn id(&self) -> RuleId;

    /// Try to split `tied` (always 2+) teams into ordered subgroups, best first.
    ///
    /// Return `None` if the rule can't separate them (doesn't apply, or all are equal).
    /// When `Some`, there must be at least two groups, and together they must contain
    /// exactly the teams in `tied`.
    fn apply(&self, tied: &[TeamId], state: &ConferenceState) -> RuleOutcome;
}
