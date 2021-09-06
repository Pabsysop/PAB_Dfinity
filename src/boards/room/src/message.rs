use candid::{CandidType};
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, CandidType)]
pub struct Message {
    pub user: String,
    pub text: String,
    pub send_at: u64,
    pub expired_at: u64
}
