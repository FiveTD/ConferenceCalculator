use super::{
    Record, TiebreakRule,
    record::group_ranked,
    rule::RuleOutcome,
    trace::{Ctx, Resolution, StepId},
};
use crate::{model::TeamId, repository::ConferenceState};

pub struct Procedure {
    pub two_team: Vec<Box<dyn TiebreakRule>>,
    pub multi_team: Vec<Box<dyn TiebreakRule>>,
}

impl Procedure {
    pub fn resolve(&self, state: &ConferenceState) -> Resolution {
        let standings: Vec<(TeamId, Record)> = state
            .teams
            .keys()
            .map(|&t| (t, state.conference_record(t)))
            .collect();

        let mut ctx = Ctx::default();
        let order: Vec<TeamId> = group_ranked(standings, Record::cmp_standings)
            .into_iter()
            .flat_map(|group| self.resolve_group(group, None, state, &mut ctx))
            .collect();

        ctx.finish(order)
    }

    fn resolve_group(
        &self,
        tied: Vec<TeamId>,
        parent: Option<StepId>,
        state: &ConferenceState,
        ctx: &mut Ctx,
    ) -> Vec<TeamId> {
        if tied.len() < 2 {
            return tied;
        }

        let rules = if tied.len() == 2 {
            &self.two_team
        } else {
            &self.multi_team
        };

        for rule in rules {
            let outcome = rule.apply(&tied, state);
            let step = ctx.record(parent, &tied, rule.id(), &outcome);

            if let RuleOutcome::Separated { groups, .. } = outcome {
                debug_assert!(
                    is_partition(&groups, &tied),
                    "rule broke the split contract"
                );
                return groups
                    .into_iter()
                    .flat_map(|g| self.resolve_group(g, Some(step), state, ctx))
                    .collect();
            }
        }

        tied // no rule separated them; stays in input order
    }
}

fn is_partition(groups: &[Vec<TeamId>], tied: &[TeamId]) -> bool {
    let mut flat: Vec<TeamId> = groups.iter().flatten().copied().collect();
    let mut expected = tied.to_vec();
    flat.sort();
    expected.sort();
    groups.len() >= 2 && flat == expected
}
