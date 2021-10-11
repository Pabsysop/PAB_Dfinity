use candid::{CandidType};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Record(pub Vec<u8>);