use candid::{CandidType};
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, CandidType)]
pub enum  Consensus {
    POW,
    POA,
    POS
}

impl Consensus {
}
