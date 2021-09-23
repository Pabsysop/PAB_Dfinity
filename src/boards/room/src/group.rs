use candid::{CandidType};
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, CandidType, Clone)]
pub struct Group {
    pub group_members: Vec<String>
}
