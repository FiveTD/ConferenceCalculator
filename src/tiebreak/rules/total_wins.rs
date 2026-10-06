use crate::tiebreak::{
    Evidence, MetricValue, TiebreakRule,
    record::group_ranked,
    rule::{RuleId, RuleOutcome},
};
use crate::{model::TeamId, repository::ConferenceState};

pub struct TotalWins;

impl TiebreakRule for TotalWins {
    fn id(&self) -> RuleId {
        RuleId::TotalWins
    }

    fn apply(&self, tied: &[TeamId], state: &ConferenceState) -> RuleOutcome {
        let wins: Vec<(TeamId, u8)> = tied.iter().map(|&t| (t, state.total_wins(t))).collect();
        let groups = group_ranked(wins.clone(), |a, b| a.cmp(&b));

        let evidence = wins
            .into_iter()
            .map(|(team, n)| Evidence {
                team,
                value: MetricValue::Count(n),
                games: Vec::new(),
                against: Vec::new(),
            })
            .collect();

        RuleOutcome::from_groups(groups, evidence)
    }
}
