use candid::{CandidType};
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, CandidType)]
pub struct CommonScene {
    pub title: String,
    pub cover: String
}

impl CommonScene {
}
