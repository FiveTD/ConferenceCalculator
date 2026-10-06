use super::support::{TeamResult, outcome_by_record};
use crate::tiebreak::{
    Record, TiebreakRule,
    rule::{RuleId, RuleOutcome},
};
use crate::{model::TeamId, repository::ConferenceState};

pub struct OpponentsConferenceRecord;

impl TiebreakRule for OpponentsConferenceRecord {
    fn id(&self) -> RuleId {
        RuleId::OpponentsConferenceRecord
    }

    fn apply(&self, tied: &[TeamId], state: &ConferenceState) -> RuleOutcome {
        let results = tied
            .iter()
            .map(|&team| {
                let opponents: Vec<TeamId> = state.opponents_of(team).into_iter().collect();
                let record: Record = opponents.iter().map(|&o| state.conference_record(o)).sum();
                let (_, games) = state.results_against(team, &opponents);
                TeamResult {
                    team,
                    record,
                    games,
                    against: opponents,
                }
            })
            .collect();
        outcome_by_record(results)
    }
}
