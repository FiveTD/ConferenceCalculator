use serde::Serialize;

use super::rule::RuleId;
use crate::model::TeamId;

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
    pub separated: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Resolution {
    pub order: Vec<TeamId>,
    pub trace: Vec<TraceStep>,
}

impl Resolution {
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

/// Scratch space for one `resolve` call. Produces a `Resolution` for the end caller.
#[derive(Default)]
pub(super) struct Ctx {
    trace: Vec<TraceStep>,
}

impl Ctx {
    pub(super) fn record(
        &mut self,
        parent: Option<StepId>,
        tied: &[TeamId],
        rule: RuleId,
        separated: bool,
    ) -> StepId {
        let id = StepId(self.trace.len());
        self.trace.push(TraceStep {
            id,
            parent,
            tied: tied.to_vec(),
            rule,
            separated,
        });
        id
    }

    pub(super) fn finish(self, order: Vec<TeamId>) -> Resolution {
        Resolution {
            order,
            trace: self.trace,
        }
    }
}
