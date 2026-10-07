pub mod api;
pub mod model;
mod render;
pub mod repository;
pub mod tiebreak;

use std::path::Path;

use crate::api::CfbdClient;
use crate::model::Conference;
use crate::repository::{ConferenceState, ConferenceStateError};
use crate::tiebreak::procedures;

const CONFERENCE: Conference = Conference::BigTwelve;
const SEASON: u16 = 2024;

fn file_name(conference: Conference, season: u16) -> String {
    format!("{}-{}.json", conference.cfbd_name(), season)
}

fn load_from_file(path: &Path) -> Result<ConferenceState, ConferenceStateError> {
    ConferenceState::from_file(path)
}

async fn load_from_cfbd(
    path: Option<&Path>,
) -> Result<ConferenceState, Box<dyn std::error::Error>> {
    let cfbd_key = std::env::var("CFBD_KEY")?;
    let cfbd_client = CfbdClient::new(cfbd_key)?;
    let state = ConferenceState::from_cfbd(&cfbd_client, CONFERENCE, SEASON).await?;

    if let Some(path) = path {
        state.to_file(path)?;
    }
    Ok(state)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let file_path = file_name(CONFERENCE, SEASON);
    let path = Path::new(file_path.as_str());
    let state = if std::fs::exists(path).is_ok_and(|f| f) {
        load_from_file(path)?
    } else {
        load_from_cfbd(Some(path)).await?
    };

    match procedures::procedure_for(CONFERENCE, SEASON) {
        Ok(procedure) => {
            let resolution = procedure.resolve(&state);
            render::print_resolution(&state, &resolution);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
