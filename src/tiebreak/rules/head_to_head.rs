use super::support::{outcome_by_record, results_against};
use crate::tiebreak::{
    TiebreakRule,
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

        let results = results_against(tied, tied, state);

        // A team that played nobody else has nothing to compare
        if results.iter().any(|r| r.record.games() == 0) {
            return RuleOutcome::NotApplicable {
                reason: NotApplicableReason::NoGamesAmongTied,
            };
        }

        outcome_by_record(results)
    }
}
