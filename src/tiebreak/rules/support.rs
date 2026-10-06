/* Code used by multiple rules */

use crate::tiebreak::{
    Record, RuleOutcome,
    evidence::{Evidence, GameId, MetricValue},
    record::group_ranked,
};
use crate::{model::TeamId, repository::ConferenceState};

pub(super) struct TeamResult {
    pub team: TeamId,
    pub record: Record,
    pub games: Vec<GameId>,
    pub against: Vec<TeamId>,
}

/// Each of `tied`'s results in games against `opponents`.
pub(super) fn results_against(
    tied: &[TeamId],
    opponents: &[TeamId],
    state: &ConferenceState,
) -> Vec<TeamResult> {
    tied.iter()
        .map(|&team| {
            let (record, games) = state.results_against(team, opponents);
            TeamResult {
                team,
                record,
                games,
                against: opponents.iter().copied().filter(|&t| t != team).collect(),
            }
        })
        .collect()
}

pub(super) fn evidence_of(r: &TeamResult) -> Evidence {
    Evidence {
        team: r.team,
        value: MetricValue::Record(r.record),
        games: r.games.clone(),
        against: r.against.clone(),
    }
}

pub(super) fn rank(results: &[TeamResult]) -> Vec<Vec<TeamId>> {
    group_ranked(
        results.iter().map(|r| (r.team, r.record)).collect(),
        Record::cmp_pct,
    )
}

/// Rank by winning percentage and package the results as evidence.
pub(super) fn outcome_by_record(results: Vec<TeamResult>) -> RuleOutcome {
    RuleOutcome::from_groups(rank(&results), results.iter().map(evidence_of).collect())
}
