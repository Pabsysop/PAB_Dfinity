use candid::{CandidType, Nat, Principal};
use serde::{Serialize, Deserialize};
pub type TransactionId = u128;

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
#[derive(CandidType, Deserialize)]
pub enum  Value {
    Int(i32),
    Nat(u32),
    Float(f32),
    Text(String),
    Bool(bool),
    Principal(Principal),
    Empty
}

#[derive(CandidType, Deserialize)]
pub struct  Property {
    pub name : String,
    pub value : Value,
    pub immutable : bool
}
#[derive(CandidType, Deserialize)]
pub struct NFTPayload {
    pub payload: u8,
    pub staged_data: Vec<u8>
}
#[derive(CandidType, Deserialize)]
pub struct NFTProperty {
    pub name: u8,
    pub value: (),
    pub immutable: bool
}
#[derive(CandidType, Deserialize)]
pub struct  NftEgg {
    pub payload: NFTPayload,
    pub content_type: String,
    pub owner: Principal,
    pub properties: Property,
    pub is_private: bool
}

#[derive(Deserialize, Serialize, Debug, Clone, CandidType)]
#[serde(rename_all = "camelCase")]
pub enum Error {
    InvalidSubaccount,
    InvalidTokenHolder,
    InvalidSpender,
    InvalidReceiver,
    InsufficientBalance,
    InsufficientAllowance,
    RejectedByHolder,
    RejectedByReceiver,
    CallFailed,
    NotifyFailed,
    QuantityTooSmall,
    Unknown,
}

#[derive(CandidType, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TransferResult {
    //transfer succeed, but call failed & notify failed
    Ok(TransactionId, Option<Vec<Error>>),
    Err(Error),
}