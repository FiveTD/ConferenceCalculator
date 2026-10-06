use super::support::{outcome_by_record, results_against};
use crate::tiebreak::{NotApplicableReason, RuleId, RuleOutcome, TiebreakRule};
use crate::{model::TeamId, repository::ConferenceState};

pub struct CommonOpponents {
    pub min_common: usize,
}

impl TiebreakRule for CommonOpponents {
    fn id(&self) -> RuleId {
        RuleId::CommonOpponents
    }

    fn apply(&self, tied: &[TeamId], state: &ConferenceState) -> RuleOutcome {
        let common = state.common_opponents(tied);
        if common.len() < self.min_common.max(1) {
            return RuleOutcome::NotApplicable {
                reason: NotApplicableReason::TooFewCommonOpponents,
            };
        }

        outcome_by_record(results_against(tied, &common, state))
    }
}
