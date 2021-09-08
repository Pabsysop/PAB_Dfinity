use candid::{CandidType, Nat, Principal};
use serde::{Deserialize};
use std::fmt;

pub type VisaNFTCanisterId = Principal;

#[derive(CandidType, Clone, Deserialize)]
pub struct NFTContractMeta {
    pub name: String, 
    pub symbol: String
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

#[derive(CandidType, Clone, Deserialize)]
pub enum WasmType {
    PABToken,
    Board,
    Life,
    AvatarNFT,
    VisaNFT
}

#[derive(CandidType, Deserialize)]
pub struct StoreWASMArgs {
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
