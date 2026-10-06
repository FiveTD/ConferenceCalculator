use std::collections::HashMap;

use serde::Serialize;

use super::{
    resolution::{Placement, Resolution, TieStatus},
    rule::{RuleId, RuleOutcome},
};
use crate::model::TeamId;

/// Describes how a step affected the procedure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum StepEffect {
    /// The group was divided into ordered subgroups, each resolved further.
    Split,
    /// One team was placed ahead of the rest, restarting the procedure.
    Seeded(TeamId),
    /// Nothing was settled; the group moved to the next rule.
    Passed,
}

/// Position in `Resolution::trace`. Steps are only ever appended,
/// so `trace[id.0]` is always the step with that id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct StepId(pub usize);

#[derive(Debug, Clone, Serialize)]
pub struct TraceStep {
    pub id: StepId,
    /// The step whose split produced this tied group (`None` for groups
    /// that were already tied in the standings).
    pub parent: Option<StepId>,
    pub tied: Vec<TeamId>,
    pub rule: RuleId,
    pub outcome: RuleOutcome,
    pub effect: StepEffect,
}

impl TraceStep {
    /// Did this step actually separate the group?
    pub fn is_decisive(&self) -> bool {
        self.effect != StepEffect::Passed
    }
}

/// Scratch space for one `resolve` call. Produces a `Resolution` for the end caller.
#[derive(Default)]
pub(super) struct Ctx {
    trace: Vec<TraceStep>,
    steps_by_team: HashMap<TeamId, Vec<StepId>>,
    /// Groups that ran out of rules, in final order.
    unresolved: Vec<Vec<TeamId>>,
}

impl Ctx {
    pub(super) fn record(
        &mut self,
        parent: Option<StepId>,
        tied: &[TeamId],
        rule: RuleId,
        outcome: &RuleOutcome,
        effect: StepEffect,
    ) -> StepId {
        let id = StepId(self.trace.len());
        for &team in tied {
            self.steps_by_team.entry(team).or_default().push(id);
        }
        self.trace.push(TraceStep {
            id,
            parent,
            tied: tied.to_vec(),
            rule,
            outcome: outcome.clone(),
            effect,
        });
        id
    }

    pub(super) fn mark_unresolved(&mut self, tied: &[TeamId]) {
        self.unresolved.push(tied.to_vec());
    }

    pub(super) fn finish(mut self, order: Vec<TeamId>) -> Resolution {
        let group_of: HashMap<TeamId, usize> = self
            .unresolved
            .iter()
            .enumerate()
            .flat_map(|(g, teams)| teams.iter().map(move |&t| (t, g)))
            .collect();

        let mut placements: Vec<Placement> = Vec::with_capacity(order.len());
        for (i, team) in order.into_iter().enumerate() {
            let group = group_of.get(&team).copied();

            let place = match (group, placements.last()) {
                (Some(g), Some(prev)) if group_of.get(&prev.team) == Some(&g) => prev.place,
                _ => i + 1,
            };

            placements.push(Placement {
                team,
                place,
                steps: self.steps_by_team.remove(&team).unwrap_or_default(),
                status: if group.is_some() {
                    TieStatus::Unresolved
                } else {
                    TieStatus::Resolved
                },
            })
        }

        Resolution {
            placements,
            trace: self.trace,
        }
    }
}
