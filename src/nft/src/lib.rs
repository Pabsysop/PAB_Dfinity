use candid::{CandidType, Principal};
use serde::{Serialize, Deserialize};
pub type TransactionId = u128;

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct NFT {
    pub id: String,
    pub src: NFTSrc,
}
impl Default for NFT {
    fn default() -> NFT{
        NFT {
            id: Default::default(),
            src: NFTSrc::DFINITY,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub enum NFTSrc {
    DFINITY,
    LOCAL,
}

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub enum NFTType {
    CITIZEN,
    TICKET,
    VISA,
    ASSETS,
}

#[derive(CandidType, Deserialize, Serialize)]
pub enum  Value {
    Int(i32),
    Nat(u32),
    Float(f32),
    Text(String),
    Bool(bool),
    Principal(Principal),
    Empty
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct  Property {
    pub name : String,
    pub value : Value,
    pub immutable : bool
}
#[derive(CandidType, Deserialize, Serialize)]
pub enum NFTPayload {
    Payload(Vec<u8>),
    StagedData
}
#[derive(CandidType, Deserialize, Serialize)]
pub struct NFTProperty {
    pub name: u8,
    pub value: (),
    pub immutable: bool
}
#[derive(CandidType, Deserialize)]
pub struct  NftEgg {
    pub payload: NFTPayload,
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub owner: Principal,
    pub properties: Vec<Property>,
    #[serde(rename = "isPrivate")]
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