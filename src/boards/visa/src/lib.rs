use candid::{CandidType, Principal};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub enum VisaType {
    Citizenship,
    BoardMember,
}

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Visa {
    pub visa_type: VisaType,
    pub board: Option<Principal>,
    pub expire_date: u32,
    pub issue_date: u32,
    pub nft_address: String
}

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Ticket {
    pub board: Principal,
    pub room: String,
    pub expire_date: u32,
    pub issue_date: u32,
    pub used: bool,
    pub nft_address: String
}
