use serde::Serialize;

use super::record::Record;
use crate::model::TeamId;

/// Position in `ConferenceState::games`.
/// Only meaningful against its producing state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct GameId(pub usize);

/// The number a rule compared.
/// TODO: Point differential, current ranking, etc. as needed
#[derive(Debug, Clone, Serialize)]
pub enum MetricValue {
    Record(Record),
    Count(u8),
}

#[derive(Debug, Clone, Serialize)]
pub struct Evidence {
    pub team: TeamId,
    pub value: MetricValue,
    /// The games that produced `value`
    pub games: Vec<GameId>,
    /// The opponents this value was measured against (used for seeded leader)
    pub against: Vec<TeamId>,
}
