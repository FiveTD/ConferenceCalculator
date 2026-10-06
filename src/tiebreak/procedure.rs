use super::{
    Record, TiebreakRule,
    record::group_ranked,
    resolution::Resolution,
    rule::{RuleId, RuleOutcome},
    trace::{Ctx, StepEffect, StepId},
};
use crate::{model::TeamId, repository::ConferenceState};

/// Specifies how tiebreakers break up tied groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reduction {
    /// A separating rule splits the group, and each subgroup is resolved independently.
    SplitAll,
    /// A rule counts only if it produces a single leader.
    /// The leader is seeded, and remaining teams restart the procedure.
    SeedLeader,
}

pub struct Procedure {
    pub two_team: Vec<Box<dyn TiebreakRule>>,
    pub multi_team: Vec<Box<dyn TiebreakRule>>,
    pub reduction: Reduction,
}

impl Procedure {
    pub fn new(
        two_team: Vec<Box<dyn TiebreakRule>>,
        multi_team: Vec<Box<dyn TiebreakRule>>,
    ) -> Self {
        Self {
            two_team,
            multi_team,
            reduction: Reduction::SplitAll,
        }
    }

    pub fn with_reduction(mut self, reduction: Reduction) -> Self {
        self.reduction = reduction;
        self
    }

    fn effect_of(&self, outcome: &RuleOutcome) -> StepEffect {
        let RuleOutcome::Separated { groups, .. } = outcome else {
            return StepEffect::Passed;
        };
        match self.reduction {
            Reduction::SplitAll => StepEffect::Split,
            Reduction::SeedLeader => match groups[0].as_slice() {
                [leader] => StepEffect::Seeded(*leader),
                _ => StepEffect::Passed, // no single leader: whole group continues
            },
        }
    }

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
            let effect = self.effect_of(&outcome);
            let step = ctx.record(parent, &tied, rule.id(), &outcome, effect);

            match (effect, outcome) {
                (StepEffect::Split, RuleOutcome::Separated { groups, .. }) => {
                    debug_assert!(
                        is_partition(&groups, &tied),
                        "rule broke the split contract"
                    );
                    return groups
                        .into_iter()
                        .flat_map(|g| self.resolve_group(g, Some(step), state, ctx))
                        .collect();
                }
                (StepEffect::Seeded(leader), RuleOutcome::Separated { groups, .. }) => {
                    let rest: Vec<TeamId> = groups
                        .into_iter()
                        .flatten()
                        .filter(|&t| t != leader)
                        .collect();
                    let mut order = vec![leader];
                    order.extend(self.resolve_group(rest, Some(step), state, ctx));
                    return order;
                }
                _ => {} // Passed: try the next rule
            }
        }

        ctx.mark_unresolved(&tied);
        tied // no rule separated them; stays in input order
    }

    /// (two-team rules, multi-team rules), in priority order.
    pub fn rule_ids(&self) -> (Vec<RuleId>, Vec<RuleId>) {
        (ids(&self.two_team), ids(&self.multi_team))
    }
}

fn is_partition(groups: &[Vec<TeamId>], tied: &[TeamId]) -> bool {
    let mut flat: Vec<TeamId> = groups.iter().flatten().copied().collect();
    let mut expected = tied.to_vec();
    flat.sort();
    expected.sort();
    groups.len() >= 2 && flat == expected
}

fn ids(rules: &[Box<dyn TiebreakRule>]) -> Vec<RuleId> {
    rules.iter().map(|r| r.id()).collect()
}
