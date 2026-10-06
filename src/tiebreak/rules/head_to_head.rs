use crate::tiebreak::{
    Record, TiebreakRule,
    record::group_ranked,
    rule::{NotApplicableReason, RuleId, RuleOutcome},
};
use crate::{model::TeamId, repository::ConferenceState};

pub struct HeadToHead {
    pub require_round_robin: bool,
}

impl TiebreakRule for HeadToHead {
    fn id(&self) -> RuleId {
        RuleId::HeadToHead
    }

    fn apply(&self, tied: &[TeamId], state: &ConferenceState) -> RuleOutcome {
        if self.require_round_robin && !state.have_all_played(tied) {
            return RuleOutcome::NotApplicable {
                reason: NotApplicableReason::IncompleteRoundRobin,
            };
        }

        let records: Vec<(TeamId, Record)> = tied
            .iter()
            .map(|&t| (t, state.record_against(t, tied)))
            .collect();

        // A team that played nobody else has nothing to compare
        if records.iter().any(|(_, r)| r.games() == 0) {
            return RuleOutcome::NotApplicable {
                reason: NotApplicableReason::NoGamesAmongTied,
            };
        }

        RuleOutcome::from_groups(group_ranked(records, Record::cmp_pct))
    }
}
