use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufReader, BufWriter, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{api::*, model::*};

#[derive(Debug, Error)]
pub enum ConferenceStateError {
    #[error("API request failed: {0}")]
    Request(#[from] CfbdError),
    #[error("file error: {0}")]
    File(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConferenceState {
    pub conference: Conference,
    pub season: u16,
    pub teams: BTreeMap<TeamId, Team>,
    pub games: Vec<Game>,
}

impl ConferenceState {
    pub async fn from_cfbd(
        client: &CfbdClient,
        conference: Conference,
        season: u16,
    ) -> Result<Self, ConferenceStateError> {
        let name = conference.cfbd_name();
        let (cfbd_teams, cfbd_games) =
            tokio::try_join!(client.get_teams(name), client.get_games(season, name))?;

        let mut teams: BTreeMap<TeamId, Team> = cfbd_teams
            .into_iter()
            .map(|t| {
                let team = Team::from(t);
                (team.id, team)
            })
            .collect();

        let mut games = Vec::new();
        for (i, &g) in cfbd_games.iter().enumerate() {
            if g.conference_game {
                if i == cfbd_games.len() - 1 && g.week > cfbd_games[i - 1].week {
                    // Conference championship game, filter out (for now)
                    continue;
                }
                games.push(Game::from(g));
            } else if let Some(team) = g.winner_id().and_then(|w| teams.get_mut(&w)) {
                team.non_conference_wins += 1
            }
        }

        Ok(Self {
            conference,
            season,
            teams,
            games,
        })
    }

    pub fn from_file(path: &Path) -> Result<Self, ConferenceStateError> {
        let reader = BufReader::new(File::open(path)?);
        Ok(serde_json::from_reader(reader)?)
    }

    pub fn to_file(&self, path: &Path) -> Result<(), ConferenceStateError> {
        let tmp = path.with_extension("json.tmp");

        {
            let mut writer = BufWriter::new(File::create(&tmp)?);
            serde_json::to_writer_pretty(&mut writer, self)?;
            writer.flush()?; // auto-drop swallows errors, explicit error check
        }

        std::fs::rename(&tmp, path)?;
        Ok(())
    }
}
