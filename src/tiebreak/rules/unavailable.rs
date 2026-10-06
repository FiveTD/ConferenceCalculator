use crate::{
    model::TeamId,
    repository::ConferenceState,
    tiebreak::{NotApplicableReason, RuleId, RuleOutcome, TiebreakRule},
};

/// Covers uncomputable steps (SportSource, random draw, etc.)
pub struct Unavailable {
    pub rule: RuleId,
    pub reason: NotApplicableReason,
}

impl TiebreakRule for Unavailable {
    fn id(&self) -> RuleId {
        self.rule
    }
    fn apply(&self, _tied: &[TeamId], _state: &ConferenceState) -> RuleOutcome {
        RuleOutcome::NotApplicable {
            reason: self.reason,
        }
    }
}
