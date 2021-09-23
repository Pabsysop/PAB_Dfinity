use candid::{CandidType, Principal};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub enum RecordType {
    Invite
}

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Record {
    pub r_type: RecordType,
    pub log: (u64, String, Principal, String)
}
