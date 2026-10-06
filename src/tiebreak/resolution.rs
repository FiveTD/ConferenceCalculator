use serde::Serialize;

use super::trace::{StepId, TraceStep};
use crate::model::TeamId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TieStatus {
    Resolved,
    Unresolved,
}

#[derive(Debug, Clone, Serialize)]
pub struct Placement {
    pub team: TeamId,
    /// 1-indexed. Teams in an unresolved tie share the top placement.
    pub place: usize,
    /// Every trace step involving this team, oldest first.
    pub steps: Vec<StepId>,
    pub status: TieStatus,
}

#[derive(Debug, Clone, Serialize)]
pub struct Resolution {
    pub placements: Vec<Placement>,
    pub trace: Vec<TraceStep>,
}

impl Resolution {
    /// Teams in final order.
    pub fn teams(&self) -> impl Iterator<Item = TeamId> + '_ {
        self.placements.iter().map(|p| p.team)
    }

    pub fn placement_of(&self, team: TeamId) -> Option<&Placement> {
        self.placements.iter().find(|p| p.team == team)
    }

    pub fn step(&self, id: StepId) -> &TraceStep {
        &self.trace[id.0]
    }

    /// The steps behind one placement, in order.
    pub fn steps_for<'a>(
        &'a self,
        placement: &'a Placement,
    ) -> impl Iterator<Item = &'a TraceStep> + 'a {
        placement.steps.iter().map(|&id| self.step(id))
    }

    /// How many splits deep a step is (0 for top level tie groups).
    pub fn depth(&self, step: &TraceStep) -> usize {
        let mut depth = 0;
        let mut current = step;
        while let Some(parent) = current.parent {
            current = &self.trace[parent.0];
            depth += 1
        }
        depth
    }
}
