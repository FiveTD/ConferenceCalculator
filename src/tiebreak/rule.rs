use serde::Serialize;

use crate::{model::TeamId, repository::ConferenceState};

/// Every tiebreaker rule gets a named ID to help with tracing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RuleId {
    HeadToHead,
    // TODO
}

pub trait TiebreakRule {
    fn id(&self) -> RuleId;

    /// Try to split `tied` (always 2+) teams into ordered subgroups, best first.
    ///
    /// Return `None` if the rule can't separate them (doesn't apply, or all are equal).
    /// When `Some`, there must be at least two groups, and together they must contain
    /// exactly the teams in `tied`.
    fn split(&self, tied: &[TeamId], state: &ConferenceState) -> Option<Vec<Vec<TeamId>>>;
}
