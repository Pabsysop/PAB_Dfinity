use candid::{CandidType, Principal};
use serde::{Serialize, Deserialize};
use nft::NFT;

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub enum VisaType {
    Citizenship,
    BoardMember,
}

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Visa {
    pub visa_type: VisaType,
    pub board: Option<Principal>,
    pub expire_date: Option<u64>,
    pub issue_date: u64,
    pub nft: NFT
}
impl Default for Visa {
    fn default() -> Visa{
        Visa {
            visa_type: VisaType::BoardMember,
            board: None,
            expire_date: None,
            issue_date: 0,
            nft: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Ticket {
    pub id: String,
    pub room: String,
    pub expire_date: u32,
    pub issue_date: u32,
    pub used: bool,
    pub nft_address: String
}
impl Default for Ticket {
    fn default() -> Ticket{
        Ticket {
            id: Default::default(),
            room: Default::default(),
            expire_date: 0,
            issue_date: 0,
            nft_address: Default::default(),
            used: false,
        }
    }
}
