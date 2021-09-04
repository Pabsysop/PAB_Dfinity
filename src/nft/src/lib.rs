use candid::CandidType;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct NFT {
    pub title: String,
    pub src: String,
    pub chain: String,
    pub price: f64,
    pub unit: String,
}

pub enum NFTType {
    CITIZEN,
    TICKET,
    VISA,
    ASSETS,
}
