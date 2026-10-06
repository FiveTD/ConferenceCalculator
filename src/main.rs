pub mod api;
pub mod model;
mod render;
pub mod repository;
pub mod tiebreak;

use std::path::Path;

use crate::api::CfbdClient;
use crate::model::*;
use crate::repository::{ConferenceState, ConferenceStateError};
use crate::tiebreak::{rules::*, *};

const CONFERENCE: Conference = Conference::BigTwelve;
const YEAR: u16 = 2025;

fn load_from_file() -> Result<ConferenceState, ConferenceStateError> {
    let file_path = format!("{}-{}.json", CONFERENCE.cfbd_name(), YEAR);
    ConferenceState::from_file(Path::new(&file_path))
}

async fn load_from_cfbd() -> Result<ConferenceState, Box<dyn std::error::Error>> {
    let cfbd_key = std::env::var("CFBD_KEY")?;
    let cfbd_client = CfbdClient::new(cfbd_key)?;
    let state = ConferenceState::from_cfbd(&cfbd_client, CONFERENCE, YEAR).await?;

    let file_path = format!("{}-{}.json", CONFERENCE.cfbd_name(), YEAR);
    state.to_file(Path::new(&file_path))?;
    Ok(state)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let state = load_from_file()?;
    // let state = load_from_cfbd().await?;
    // state.print_games();

    let procedure = Procedure {
        two_team: vec![
            Box::new(HeadToHead {
                require_round_robin: true,
            }),
            Box::new(CommonOpponents { min_common: 1 }),
        ],
        multi_team: vec![
            Box::new(HeadToHead {
                require_round_robin: true,
            }),
            Box::new(CommonOpponents { min_common: 1 }),
        ],
    };
    let resolution = procedure.resolve(&state);
    render::print_resolution(&state, &resolution);

    Ok(())
}
