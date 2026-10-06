use crate::tiebreak::{
    NotApplicableReason, Record, RuleId, RuleOutcome, TiebreakRule, record::group_ranked,
};
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

        let records: Vec<(TeamId, Record)> = tied
            .iter()
            .map(|&t| (t, state.record_against(t, &common)))
            .collect();

        RuleOutcome::from_groups(group_ranked(records, Record::cmp_pct))
    }
}
