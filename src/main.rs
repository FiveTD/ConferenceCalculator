pub mod api;
pub mod model;
pub mod repository;
pub mod tiebreak;

use std::path::Path;

use crate::api::CfbdClient;
use crate::model::*;
use crate::repository::{ConferenceState, ConferenceStateError};
use crate::tiebreak::{rules::*, *};

const CONFERENCE: Conference = Conference::BigTwelve;

impl ConferenceState {
    fn print_games(&self) {
        for game in &self.games {
            let home = &self.teams[&game.home];
            let away = &self.teams[&game.away];

            match game.result {
                GameResult::Final {
                    winner,
                    home_points,
                    away_points,
                } => {
                    let (w, w_pts, l, l_pts) = if winner == home.id {
                        (home, home_points, away, away_points)
                    } else {
                        (away, away_points, home, home_points)
                    };
                    println!("{} {}-{} {}", w.abbreviation, w_pts, l_pts, l.abbreviation);
                }
                GameResult::Scheduled => {
                    println!("{} @ {}", away.abbreviation, home.abbreviation);
                }
            }
        }
    }
}

fn load_from_file() -> Result<ConferenceState, ConferenceStateError> {
    let file_path = format!("{}.json", CONFERENCE.cfbd_name());
    ConferenceState::from_file(Path::new(&file_path))
}

async fn load_from_cfbd() -> Result<ConferenceState, Box<dyn std::error::Error>> {
    let cfbd_key = std::env::var("CFBD_KEY")?;
    let cfbd_client = CfbdClient::new(cfbd_key)?;
    let state = ConferenceState::from_cfbd(&cfbd_client, CONFERENCE, 2026).await?;

    let file_path = format!("{}.json", CONFERENCE.cfbd_name());
    state.to_file(Path::new(&file_path))?;
    Ok(state)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let state = load_from_file()?;
    // let state = load_from_cfbd().await?;
    state.print_games();

    let procedure = Procedure {
        two_team: vec![Box::new(HeadToHead {
            require_round_robin: true,
        })],
        multi_team: vec![Box::new(HeadToHead {
            require_round_robin: true,
        })],
    };
    for (idx, team) in procedure.resolve(&state).iter().enumerate() {
        println!("{}. {}", idx + 1, state.teams[team].name);
    }

    Ok(())
}
