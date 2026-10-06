use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TeamId(u32);

impl From<u32> for TeamId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub id: TeamId,
    pub name: String,
    pub abbreviation: String,
    pub non_conference_wins: u8,
}
