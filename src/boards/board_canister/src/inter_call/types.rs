use ic_cdk::export::candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};
use std::fmt;

pub type TransactionId = u128;
#[derive(CandidType, Clone, Deserialize)]
pub struct Subaccount(pub [u8; 32]);
pub struct WalletWASMBytes(pub Option<serde_bytes::ByteBuf>);
pub struct BoardWASMBytes(pub Option<serde_bytes::ByteBuf>);
pub struct LifeWASMBytes(pub Option<serde_bytes::ByteBuf>);

impl Default for WalletWASMBytes {
    fn default() -> Self {
        WalletWASMBytes(None)
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
    #[serde(with = "serde_bytes")]
    pub wasm_module: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct IssueTokenArgs {
    pub subaccount: Option<Vec<u8>>,
    #[serde(with = "serde_bytes")]
    pub logo: Vec<u8>,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
    pub fee: Fee,
}

// Rate decimals = 8
// transferFee = amount * rate / 1000000
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

#[derive(CandidType, Deserialize, Default)]
struct NFTPayload {
    payload: Nat,
    staged_data: Vec<u8>
}
#[derive(CandidType, Deserialize, Default)]
struct NFTProperty {
    name: Nat,
    value: (),
    immutable: bool
}
#[derive(CandidType, Deserialize, Default)]
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
