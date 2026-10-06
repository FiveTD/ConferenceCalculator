use super::support::{rank, results_against};
use crate::tiebreak::{
    Record, TiebreakRule,
    record::group_ranked,
    rule::{NotApplicableReason, RuleId, RuleOutcome},
    rules::support::evidence_of,
};
use crate::{model::TeamId, repository::ConferenceState};

pub struct StandingsWalk;

impl TiebreakRule for StandingsWalk {
    fn id(&self) -> RuleId {
        RuleId::StandingsWalk
    }

    fn apply(&self, tied: &[TeamId], state: &ConferenceState) -> RuleOutcome {
        let common = state.common_opponents(tied);
        if common.is_empty() {
            return RuleOutcome::NotApplicable {
                reason: NotApplicableReason::TooFewCommonOpponents,
            };
        }

        // Equal win percentages form one placement group, which counts as a
        // single collective opponent.
        let standings: Vec<(TeamId, Record)> = state
            .teams
            .keys()
            .map(|&t| (t, state.conference_record(t)))
            .collect();

        let mut evidence = Vec::new();
        for group in group_ranked(standings, Record::cmp_pct) {
            let opponents: Vec<TeamId> = group.into_iter().filter(|t| common.contains(t)).collect();
            if opponents.is_empty() {
                continue;
            }

            let results = results_against(tied, &opponents, state);
            let ranked = rank(&results);
            evidence.extend(results.iter().map(evidence_of));

            if ranked[0].len() == 1 {
                return RuleOutcome::from_groups(ranked, evidence);
            }
        }
        RuleOutcome::NoSeparation { evidence } // walked everything, no single leader
    }
}
