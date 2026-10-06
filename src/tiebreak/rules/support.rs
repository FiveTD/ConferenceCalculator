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
            }
        })
        .collect()
}

/// Rank by winning percentage and package the results as evidence.
pub(super) fn outcome_by_record(results: Vec<TeamResult>) -> RuleOutcome {
    let ranked = results.iter().map(|r| (r.team, r.record)).collect();
    let groups = group_ranked(ranked, Record::cmp_pct);

    let evidence = results
        .into_iter()
        .map(|r| Evidence {
            team: r.team,
            value: MetricValue::Record(r.record),
            games: r.games,
        })
        .collect();

    RuleOutcome::from_groups(groups, evidence)
}
