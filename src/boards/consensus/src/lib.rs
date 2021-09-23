use candid::{CandidType};
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, CandidType, Clone)]
pub enum  Consensus {
    POW,
    POA,
    POS
}

impl Consensus {
}
