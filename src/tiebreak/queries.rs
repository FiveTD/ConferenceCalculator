use super::record::Record;
use crate::{model::*, repository::ConferenceState};

/// (winner, loser) for a finished game, `None` if not played.
fn decided(game: &Game) -> Option<(TeamId, TeamId)> {
    match game.result {
        GameResult::Final { winner, .. } => {
            let loser = if winner == game.home {
                game.away
            } else {
                game.home
            };
            Some((winner, loser))
        }
        GameResult::Scheduled => None,
    }
}

impl ConferenceState {
    fn decided_games(&self) -> impl Iterator<Item = (TeamId, TeamId)> + '_ {
        self.games.iter().filter_map(decided)
    }

    fn record_where(&self, team: TeamId, counts: impl Fn(TeamId) -> bool) -> Record {
        let mut record = Record::default();
        for (winner, loser) in self.decided_games() {
            if winner == team && counts(loser) {
                record.wins += 1;
            } else if loser == team && counts(winner) {
                record.losses += 1;
            }
        }
        record
    }

    pub fn conference_record(&self, team: TeamId) -> Record {
        self.record_where(team, |_| true)
    }

    pub fn record_against(&self, team: TeamId, opponents: &[TeamId]) -> Record {
        self.record_where(team, |opp| opponents.contains(&opp))
    }

    pub fn have_all_played(&self, teams: &[TeamId]) -> bool {
        teams.iter().enumerate().all(|(i, &a)| {
            teams[i + 1..].iter().all(|&b| {
                self.decided_games()
                    .any(|(w, l)| (w == a && l == b) || (w == b && l == a))
            })
        })
    }
}
