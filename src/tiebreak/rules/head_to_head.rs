use super::support::{outcome_by_record, results_against};
use crate::tiebreak::{
    TiebreakRule,
    rule::{NotApplicableReason, RuleId, RuleOutcome},
    rules::support::evidence_of,
};
use crate::{model::TeamId, repository::ConferenceState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IncompleteGroup {
    /// Not applicable unless every tied team played every other.
    Skip,
    /// Use whatever games exist among the tied teams.
    UseGames,
    /// Not applicable unless one team beat every other tied team; that team
    /// leads and the rest are left level.
    SeedUndefeated,
}

pub struct HeadToHead {
    pub incomplete: IncompleteGroup,
}

impl TiebreakRule for HeadToHead {
    fn id(&self) -> RuleId {
        RuleId::HeadToHead
    }

    fn apply(&self, tied: &[TeamId], state: &ConferenceState) -> RuleOutcome {
        let results = results_against(tied, tied, state);

        if !state.have_all_played(tied) {
            match self.incomplete {
                IncompleteGroup::UseGames => {}
                IncompleteGroup::Skip => {
                    return RuleOutcome::NotApplicable {
                        reason: NotApplicableReason::IncompleteRoundRobin,
                    };
                }
                IncompleteGroup::SeedUndefeated => {
                    let sweeper = results
                        .iter()
                        .find(|r| r.record.wins as usize == tied.len() - 1)
                        .map(|r| r.team);
                    return match sweeper {
                        Some(leader) => {
                            let rest = tied.iter().copied().filter(|&t| t != leader).collect();
                            RuleOutcome::from_groups(
                                vec![vec![leader], rest],
                                results.iter().map(evidence_of).collect(),
                            )
                        }
                        None => RuleOutcome::NotApplicable {
                            reason: NotApplicableReason::IncompleteRoundRobin,
                        },
                    };
                }
            }
        }

        // A team that played nobody else has nothing to compare
        if results.iter().any(|r| r.record.games() == 0) {
            return RuleOutcome::NotApplicable {
                reason: NotApplicableReason::NoGamesAmongTied,
            };
        }

        outcome_by_record(results)
    }
}
