use serde::{Deserialize, Serialize};

// use crate::api::CfbdGame;
use crate::model::team::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum GameResult {
    Scheduled,
    Final {
        winner: TeamId,
        home_points: u16,
        away_points: u16,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub home: TeamId,
    pub away: TeamId,
    pub result: GameResult,
}
