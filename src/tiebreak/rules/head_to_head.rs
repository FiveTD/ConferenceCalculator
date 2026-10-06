use crate::tiebreak::{Record, TiebreakRule, record::group_ranked};
use crate::{model::TeamId, repository::ConferenceState};

pub struct HeadToHead {
    pub require_round_robin: bool,
}

impl TiebreakRule for HeadToHead {
    fn split(&self, tied: &[TeamId], state: &ConferenceState) -> Option<Vec<Vec<TeamId>>> {
        if self.require_round_robin && !state.have_all_played(tied) {
            return None;
        }

        let records: Vec<(TeamId, Record)> = tied
            .iter()
            .map(|&t| (t, state.record_against(t, tied)))
            .collect();

        // A team that played nobody else has nothing to compare
        if records.iter().any(|(_, r)| r.games() == 0) {
            return None;
        }

        let groups = group_ranked(records, Record::cmp_pct);
        (groups.len() > 1).then_some(groups)
    }
}
