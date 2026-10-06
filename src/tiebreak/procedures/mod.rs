mod basic;
mod big12;

use thiserror::Error;

use super::Procedure;
use crate::model::Conference;

#[derive(Debug, Error)]
pub enum ProcedureError {
    #[error("no tiebreak procedure defined for {conference:?} in {season}")]
    Unsupported { conference: Conference, season: u16 },
}

/// The tiebreak procedure for a conference in a given season.
pub fn procedure_for(conference: Conference, season: u16) -> Result<Procedure, ProcedureError> {
    match (conference, season) {
        (Conference::BigTwelve, 2024..) => Ok(big12::from_2024()),
        _ => Err(ProcedureError::Unsupported { conference, season }),
    }
}
