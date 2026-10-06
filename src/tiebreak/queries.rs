use std::collections::BTreeSet;

use super::{evidence::GameId, record::Record};
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
    fn decided_games(&self) -> impl Iterator<Item = (GameId, TeamId, TeamId)> + '_ {
        self.games
            .iter()
            .enumerate()
            .filter_map(|(i, g)| decided(g).map(|(w, l)| (GameId(i), w, l)))
    }

    fn results_where(
        &self,
        team: TeamId,
        counts: impl Fn(TeamId) -> bool,
    ) -> (Record, Vec<GameId>) {
        let mut record = Record::default();
        let mut games = Vec::new();
        for (id, winner, loser) in self.decided_games() {
            if winner == team && counts(loser) {
                record.wins += 1;
                games.push(id);
            } else if loser == team && counts(winner) {
                record.losses += 1;
                games.push(id);
            }
        }
        (record, games)
    }

    /// Look up a game by `GameId`
    pub fn game(&self, id: GameId) -> Option<&Game> {
        self.games.get(id.0)
    }

    /// Record in all conference games.
    pub fn conference_record(&self, team: TeamId) -> Record {
        self.results_where(team, |_| true).0
    }

    /// Total wins, including non-conference.
    pub fn total_wins(&self, team: TeamId) -> u8 {
        self.conference_record(team).wins + u8::from(self.teams[&team].non_conference_wins)
    }

    /// Record in games against `opponents`.
    pub fn results_against(&self, team: TeamId, opponents: &[TeamId]) -> (Record, Vec<GameId>) {
        self.results_where(team, |opp| opponents.contains(&opp))
    }

    /// If all `teams` have completed a game against each other.
    pub fn have_all_played(&self, teams: &[TeamId]) -> bool {
        teams.iter().enumerate().all(|(i, &a)| {
            teams[i + 1..].iter().all(|&b| {
                self.decided_games()
                    .any(|(_, w, l)| (w == a && l == b) || (w == b && l == a))
            })
        })
    }

    /// Teams `team` has finished a game against.
    pub fn opponents_of(&self, team: TeamId) -> BTreeSet<TeamId> {
        self.decided_games()
            .filter_map(|(_, w, l)| {
                if w == team {
                    Some(l)
                } else if l == team {
                    Some(w)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Opponents that every team in `teams` has played (excluding themselves).
    pub fn common_opponents(&self, teams: &[TeamId]) -> Vec<TeamId> {
        let mut sets = teams.iter().map(|&t| self.opponents_of(t));
        let Some(first) = sets.next() else {
            return Vec::new();
        };
        sets.fold(first, |acc, s| acc.intersection(&s).copied().collect())
            .into_iter()
            .filter(|t| !teams.contains(t))
            .collect()
    }
}
