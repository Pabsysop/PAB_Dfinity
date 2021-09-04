use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};
use std::fmt;

pub type TransactionId = u128;

#[derive(CandidType, Clone, Deserialize)]
pub enum WasmType {
    PAB,
    Board,
    Life,
    AvatarNFT,
    VisaNFT
}

#[derive(CandidType, Clone, Deserialize)]
pub struct Subaccount(pub [u8; 32]);

pub struct WASMBytes(pub Option<serde_bytes::ByteBuf>);
impl Default for WASMBytes {
    fn default() -> Self {
        WASMBytes(None)
    }
}

#[derive(CandidType, Clone, Deserialize)]
pub struct CanisterSettings {
    pub controllers: Option<Vec<Principal>>,
    pub compute_allocation: Option<Nat>,
    pub memory_allocation: Option<Nat>,
    pub freezing_threshold: Option<Nat>,
}

#[derive(CandidType, Clone, Deserialize)]
pub struct CreateCanisterArgs {
    pub cycles: u64,
    pub settings: CanisterSettings,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateSettingsArgs {
    pub canister_id: Principal,
    pub settings: CanisterSettings,
}

#[derive(CandidType, Deserialize)]
pub struct StartStopArgs {
    pub canister_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct CreateResult {
    pub canister_id: Principal,
}

pub type IssueResult = CreateResult;

#[derive(CandidType, Deserialize)]
pub struct TokenStoreWASMArgs {
    pub wasm_type: WasmType,
    #[serde(with = "serde_bytes")]
    pub wasm_module: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct IssueTokenArgs {
    #[serde(with = "serde_bytes")]
    pub logo: Vec<u8>,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
    pub fee: Fee,
    pub owner: Principal,
}

#[derive(CandidType, Debug, Clone, Deserialize)]
pub enum Fee {
    Fixed(u128),
    RateWithLowestLimit(u128, u8),
}

impl fmt::Display for Fee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match &self {
            Fee::Fixed(_fee) => _fee.to_string(),
            Fee::RateWithLowestLimit(_fee, rate) => format!("{{lowest:{0},rate:{1}}}", _fee, rate),
        };
        write!(f, "{}", s)
    }
}

#[derive(CandidType, Deserialize)]
enum  Value {
    Int(i32),
    Nat(u32),
    Float(f32),
    Text(String),
    Bool(bool),
    Principal(Principal),
    Empty
}

#[derive(CandidType, Deserialize)]
pub struct TokenInfo {
    pub issuer: Principal,
    pub token_id: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
    pub fee: Fee,
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize)]
pub struct  Property {
    name : String,
    value : Value,
    immutable : bool
}
#[derive(CandidType, Deserialize)]
pub struct NFTPayload {
    payload: u8,
    staged_data: Vec<u8>
}
#[derive(CandidType, Deserialize)]
pub struct NFTProperty {
    name: u8,
    value: (),
    immutable: bool
}
#[derive(CandidType, Deserialize)]
pub struct  NftEgg {
    pub payload: NFTPayload,
    content_type: String,
    owner: Principal,
    properties: Property,
    is_private: bool
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
