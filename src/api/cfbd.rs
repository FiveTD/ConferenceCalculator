use serde::{Deserialize, de::DeserializeOwned};
use thiserror::Error;

use crate::model::*;

#[derive(Debug, Error)]
pub enum CfbdError {
    #[error("CFBD request failed: {0}")]
    Request(#[from] reqwest::Error),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CfbdTeam {
    pub id: u32,
    pub school: String,
    pub abbreviation: String,
}

impl From<CfbdTeam> for Team {
    fn from(value: CfbdTeam) -> Self {
        Self {
            id: value.id.into(),
            name: value.school,
            abbreviation: value.abbreviation,
            non_conference_wins: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CfbdGame {
    pub home_id: u32,
    pub away_id: u32,
    pub home_points: Option<u16>,
    pub away_points: Option<u16>,
    pub completed: bool,
    pub conference_game: bool,
}

impl CfbdGame {
    pub fn winner_id(&self) -> Option<TeamId> {
        if !self.completed {
            return None;
        }
        match (self.home_points, self.away_points) {
            (Some(h), Some(a)) if h > a => Some(self.home_id.into()),
            (Some(h), Some(a)) if a > h => Some(self.away_id.into()),
            _ => None,
        }
    }
}

impl From<CfbdGame> for Game {
    fn from(value: CfbdGame) -> Self {
        Self {
            home: value.home_id.into(),
            away: value.away_id.into(),
            result: if let Some(winner) = value.winner_id() {
                GameResult::Final {
                    winner,
                    home_points: value.home_points.unwrap_or_default(),
                    away_points: value.away_points.unwrap_or_default(),
                }
            } else {
                GameResult::Scheduled
            },
        }
    }
}

pub struct CfbdClient {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
}

impl CfbdClient {
    const DEFAULT_BASE_URL: &'static str = "https://api.collegefootballdata.com";

    pub fn new(api_key: String) -> Result<Self, CfbdError> {
        Self::with_base_url(api_key, Self::DEFAULT_BASE_URL)
    }

    pub fn with_base_url(api_key: String, base_url: impl Into<String>) -> Result<Self, CfbdError> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        Ok(Self {
            client,
            api_key,
            base_url: base_url.into().trim_end_matches('/').to_string(),
        })
    }

    async fn get_json<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<T, CfbdError> {
        Ok(self
            .client
            .get(format!("{}/{}", self.base_url, path))
            .bearer_auth(&self.api_key)
            .query(query)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    pub async fn get_teams(&self, conference: &str) -> Result<Vec<CfbdTeam>, CfbdError> {
        self.get_json("teams", &[("conference", conference.to_string())])
            .await
    }

    pub async fn get_games(&self, year: u16, conference: &str) -> Result<Vec<CfbdGame>, CfbdError> {
        self.get_json(
            "games",
            &[
                ("year", year.to_string()),
                ("conference", conference.to_string()),
            ],
        )
        .await
    }
}
