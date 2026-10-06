use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Conference {
    BigTen,
    BigTwelve,
    Acc,
    Sec,
}

impl Conference {
    pub fn cfbd_name(&self) -> &str {
        match self {
            Conference::BigTen => "B1G",
            Conference::BigTwelve => "B12",
            Conference::Acc => "ACC",
            Conference::Sec => "SEC",
        }
    }
}
