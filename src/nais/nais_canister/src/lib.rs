/**
 * Module     : nais.rs
 * Copyright  : 2021 PAB
 * License    : MIT
 * Maintainer : shiyong <shiyong0248@gmail.com>
 * Stability  : Experimental
 * Description: The UI canister on the "local" network is "r7inp-6aaaa-aaaaa-aaabq-cai" 
                nais canister_id rrkah-fqaaa-aaaaa-aaaaq-cai
 */

 mod inter_call;
 
 use ic_cdk::{storage, trap};
 use candid::{CandidType, Principal, candid_method, encode_args};
 use ic_cdk::api::{caller, id};
 use ic_cdk::api::call::call;
 use ic_cdk_macros::*;
 use nft::TransferResult;
 use std::string::String;
 use std::collections::HashMap;
 use inter_call::*;
 use inter_call::types::*;
 use serde::Deserialize;
 use easy_hasher::easy_hasher;
 
 
 static mut INITIALIZED: bool = false;
 static mut OWNER: Principal = Principal::anonymous();
 static mut FEE_TOKEN_ID: Principal = Principal::anonymous();
//  static mut PAB_TOKEN_CANISTER: Principal = Principal::anonymous();
//  static mut PAB_NFT_CANISTER: Principal = Principal::anonymous();
 const CYCLES_PER_TOKEN: u64 = 2000000000000;
 static mut POPULATION_QUANTITIES: u64 = 0;
 
 type LifeCanisterId = Principal;
 type BoardCanisterId = Principal;
 type NFTCanisterId = String;
 type GenesisCode = HashMap<String,(Option<String>, Option<String>)>;
 type CitizenCode = GenesisCode;
 type Living = HashMap<String, LifeCanisterId>;
 type Dead = Living;
 type Soul = Living;
 type Civilization = Vec<BoardCanisterId>;
 type BoardWASMBytes = WASMBytes;
 type LifeWASMBytes = WASMBytes;
 type VisaNFTWASMBytes = WASMBytes;
 type AvatarNFTWASMBytes = WASMBytes;
 type PABWalletWASMBytes = WASMBytes;
 
 #[derive(Deserialize, CandidType)]
 struct UpgradePayload {
     g_codes: GenesisCode,
     dead: Dead,
     livings: Living,
     souls: Soul,
     civils: Civilization,
     initialized: bool,
     owner: Principal,
     pab_token: Principal,
     pab_nft: Principal
 }
 
 #[init]
 #[candid_method(init)]
 fn init() {
     unsafe {
         OWNER = caller();
     }
 }
 
 fn _only_owner() {
     unsafe {
        if OWNER != caller() {
            ic_cdk::trap("not owner");
        }
     }
 }
 fn _must_initialized() {
     unsafe {
        if INITIALIZED != true {
            ic_cdk::trap("uninitialized");
        }
     }
 }
 
 fn increase_population(body: Principal, life: LifeCanisterId) {
     let population = storage::get_mut::<Living>();
     population.insert(body.to_text(),life);
     unsafe { POPULATION_QUANTITIES += 1; }
 }
 
 fn increase_civilization(board: BoardCanisterId) {
     let civils = storage::get_mut::<Civilization>();
     civils.push(board);
 }
 
 fn has_build_priviledge(applicant: Principal) -> bool {
     let population = storage::get::<Living>();
     population.contains_key(&applicant.to_text())
 }
 
 pub async fn new_canister() -> Result<CreateResult, String>  {
     let create_args = CreateCanisterArgs {
         cycles: CYCLES_PER_TOKEN,
         settings: CanisterSettings {
             controllers: Some(vec![id()]),
             compute_allocation: None,
             memory_allocation: None,
             freezing_threshold: None,
         },
     };
     create_canister_call(create_args).await
 }
 async fn new_life() -> Result<LifeCanisterId, String> {
     let life_bytes = storage::get::<LifeWASMBytes>();
     match &life_bytes.0 {
         None => {
             ic_cdk::trap("Humankind dose not emerge.");
         }
         Some(o) => {
             let create_args = CreateCanisterArgs {
                 cycles: CYCLES_PER_TOKEN,
                 settings: CanisterSettings {
                     controllers: Some(vec![id()]),
                     compute_allocation: None,
                     memory_allocation: None,
                     freezing_threshold: None,
                 },
             };
             let result = create_canister_call(create_args).await;
             match result {
                 Err(e) => ic_cdk::trap(&e),
                 Ok(create_result) => unsafe{
                    let install_args = encode_args((
                        id(),
                        POPULATION_QUANTITIES,
                    ))
                    .expect("Failed to encode arguments.");
                    match install_canister(&create_result.canister_id, o.clone().into_vec(),
                                            install_args).await
                    {
                        Ok(_) => { Ok(create_result.canister_id) }
                        Err(e) => ic_cdk::trap(format!("new life mission failed due to : {}",e).as_str())
                    }    
                 }
             }
        }
     }
 }
 async fn new_board() -> Result<BoardCanisterId, String> {
     let board_bytes = storage::get::<BoardWASMBytes>();
     match &board_bytes.0 {
         None => {
             ic_cdk::trap("civilization dose not emerge.");
         }
         Some(o) => {
            let create_args = CreateCanisterArgs {
                cycles: CYCLES_PER_TOKEN,
                settings: CanisterSettings {
                    controllers: Some(vec![id()]),
                    compute_allocation: None,
                    memory_allocation: None,
                    freezing_threshold: None,
                },
            };
           let result = create_canister_call(create_args).await;
            match result {
                Err(e) => ic_cdk::trap(&e),
                Ok(create_result) => unsafe{
                    let install_args = encode_args((
                        id(),
                    ))
                    .expect("Failed to encode arguments.");
                     match install_canister(&create_result.canister_id, o.clone().into_vec(),
                                            install_args).await
                     {
                         Ok(_) => { Ok(create_result.canister_id) }
                         Err(e) => ic_cdk::trap(format!("new board mission failed due to : {}",e).as_str())
                    }
                }
            }
         },
     }
 }
async fn new_nft_contract(wtype: WasmType) -> Result<NFTCanisterId, String> {
     let nft_bytes: &WASMBytes;
     match wtype {
         WasmType::VisaNFT => nft_bytes = storage::get::<VisaNFTWASMBytes>(),
         WasmType::AvatarNFT => nft_bytes = storage::get::<AvatarNFTWASMBytes>(),
         _ => { ic_cdk::trap("nft type error") }
     }
 
     match &nft_bytes.0 {
         None => {
             ic_cdk::trap("NFT dose not emerge.");
         }
         Some(o) => {
            let create_args = CreateCanisterArgs {
                cycles: CYCLES_PER_TOKEN,
                settings: CanisterSettings {
                    controllers: Some(vec![id()]),
                    compute_allocation: None,
                    memory_allocation: None,
                    freezing_threshold: None,
                },
            };
           let result = create_canister_call(create_args).await;
            match result {
                Err(e) => ic_cdk::trap(&e),
                Ok(create_result) => unsafe{
                    let meta = NFTContractMeta{name:"PAB Visa NFT", symbol:"PVN"};
                    let install_args = encode_args((
                        vec![id()], meta
                    ))
                    .expect("Failed to encode arguments.");
            
                    match install_canister(&create_result.canister_id, o.clone().into_vec(),
                                        install_args).await
                    {
                        Ok(_) => { Ok(create_result.canister_id.to_string()) }
                        Err(e) => ic_cdk::trap(format!("new nft mission failed due to : {}",e).as_str())
                    }
                }
            }
         },
     }
 }
 
 #[query(name = "GenesisCode")]
 #[candid_method(query, rename = "GenesisCode")]
fn genesis_code() -> String{
    _only_owner();
    _must_initialized();

    let mut code_str = String::from("");
    let codes = storage::get::<GenesisCode>();
    for (key, _value) in codes {
        code_str += key;
    }

    code_str
 }

 #[update(name = "Initialize")]
 #[candid_method(update, rename = "Initialize")]
 async fn initialize() -> Result<(), ()> {
     unsafe {
         if INITIALIZED != false {
             ic_cdk::trap("initialized");
         }
 
         INITIALIZED = true;
         OWNER = caller();
         let codes = storage::get_mut::<GenesisCode>();
         for _ in 1..14 {
            match call(Principal::management_canister(), "raw_rand", ())
            .await
            {
                Ok(b) => { 
                    let (bytes,): (Vec<u8>,) = b;
                    let code = easy_hasher::Hash::from_vec(&bytes).to_hex_string();
                    // code = code.replace(" ", "");
                    codes.insert(code, (None, None));
                },
                Err(e) => {ic_cdk::trap(e.1.as_str());}
            }
        }
  }

     Ok(())
}
 
 #[update(name = "ApplyCitizenship")]
 #[candid_method(update, rename = "ApplyCitizenship")]
 pub async fn apply_citizenship(code: String) -> Option<LifeCanisterId> {
     _must_initialized();
 
     let fail_msg = "you are not in the invite list";
     let applicant = caller();
     let population = storage::get::<Living>();
     if population.contains_key(&applicant.to_text()) {
         return population.get(&applicant.to_text()).copied();
     }
 
     let codes: &mut HashMap<String,(Option<String>, Option<String>)>;
     let gcodes = storage::get_mut::<GenesisCode>();
     if gcodes.contains_key(&code) {
         codes = gcodes;
     }else {
         let ccodes = storage::get_mut::<CitizenCode>();
         if ccodes.contains_key(&code) {
             codes = ccodes;
         }else{
             ic_cdk::trap(fail_msg);
         }
     }
     let inviter = codes.get(code.as_str()).unwrap_or(&(None,None)).clone().0;
     let life_can = codes.get(code.as_str()).unwrap_or(&(None,None)).clone().1;
     match life_can {
         None => {
             let life = new_life().await.unwrap();
             increase_population(applicant, life);
             codes.insert(code.clone(), (inviter, Some(life.to_text())));
             let nft_canister = storage::get::<NFTCanisterId>();
             let result = mint_citizen_nft(nft_canister, life).await;
             match result {
                 Err(e) => ic_cdk::trap(e.as_str()),
                 Ok(citizen_id) => return Some(life.clone())
             }
         }
         Some(_) => ic_cdk::trap(fail_msg)
     }
 }
 
 #[update(name = "DeployNFTContract")]
 #[candid_method(update, rename = "DeployNFTContract")]
 async fn deploy_nft_contract(wtype: WasmType) -> String{
     _only_owner();
 
     match new_nft_contract(wtype).await {
         Ok(n) => {
            let nft_canister = storage::get_mut::<NFTCanisterId>();
            nft_canister.clone_from(&n);
            n
         }
         Err(e) => ic_cdk::api::trap(e.as_str())
     }
 }
 
 #[update(name = "BuildCivilization")]
 #[candid_method(update, rename = "BuildCivilization")]
 async fn build_civilization() -> Option<BoardCanisterId> {
     _must_initialized();
 
     let applicant = caller();
     if !has_build_priviledge(applicant) {
         ic_cdk::trap("xxx");
     }
 
 //    let board_can = codes.get(code.as_str()).unwrap_or(&(None,None)).clone().1;
     let board = new_board().await.unwrap();
     increase_civilization(board);
 
     Some(board)
 }
 
 #[update(name = "WakeUp")]
 #[candid_method(update, rename = "WakeUp")]
 async fn wake_up(life_canister_id: LifeCanisterId){
     _only_owner();
 
     let arg = StartStopArgs {canister_id: life_canister_id };

     match start_call(arg).await {
         Ok(_) => {},
         Err(e) => ic_cdk::api::trap(e.as_str())
     }
 }
 
 #[update(name = "Sleep")]
 #[candid_method(update, rename = "Sleep")]
 async fn sleep(life_canister_id: LifeCanisterId){
     _only_owner();
     let arg = StartStopArgs {canister_id: life_canister_id };
     match stop_call(arg).await {
        Ok(_) => {},
        Err(e) => ic_cdk::api::trap(e.as_str())
    }
 }
 
 #[update(name = "ResumeBoard")]
 #[candid_method(update, rename = "ResumeBoard")]
 async fn resume_board(board_canister_id: LifeCanisterId){
     _only_owner();
     let arg = StartStopArgs {canister_id: board_canister_id };
     match start_call(arg).await {
        Ok(_) => {},
        Err(e) => ic_cdk::api::trap(e.as_str())
    }
}
 
 #[update(name = "PauseBoard")]
 #[candid_method(update, rename = "PauseBoard")]
 async fn pause_board(board_canister_id: BoardCanisterId){
     _only_owner();
     let arg = StartStopArgs {canister_id: board_canister_id };
     match stop_call(arg).await {
        Ok(_) => {},
        Err(e) => ic_cdk::api::trap(e.as_str())
    }
 }
 
 #[update(name = "setFeeTokenID")]
 #[candid_method(update, rename = "setFeeTokenID")]
 fn set_fee_token_id(token_id: Principal) {
     _must_initialized();
     _only_owner();
     unsafe { FEE_TOKEN_ID = token_id };
 }
 
 #[update(name = "uploadTokenWasm")]
 #[candid_method(update, rename = "uploadTokenWasm")]
 fn upload_token_wasm(args: TokenStoreWASMArgs) {
     _must_initialized();
     _only_owner();
 
     let token_bytes: &mut WASMBytes;
     match args.wasm_type {
         WasmType::Board => token_bytes = storage::get_mut::<BoardWASMBytes>(),
         WasmType::Life => token_bytes = storage::get_mut::<LifeWASMBytes>(),
         WasmType::AvatarNFT => token_bytes = storage::get_mut::<AvatarNFTWASMBytes>(),
         WasmType::VisaNFT => token_bytes = storage::get_mut::<VisaNFTWASMBytes>(),
         WasmType::PAB => token_bytes = storage::get_mut::<PABWalletWASMBytes>(),
     }
     token_bytes.0 = Some(serde_bytes::ByteBuf::from(args.wasm_module));
 }
 
 #[update(name = "IssuePAB")]
 #[candid_method(update, rename = "IssuePAB")]
 async fn issue_pab(args: IssueTokenArgs) -> Result<IssueResult, String> {
     _only_owner();
     _must_initialized();
 
     let wallet_bytes = storage::get::<PABWalletWASMBytes>();
     let wasm_module = match &wallet_bytes.0 {
         None => {
             ic_cdk::trap("No PAB wasm.");
         }
         Some(o) => o,
     };
 
     let create_args = CreateCanisterArgs {
         cycles: CYCLES_PER_TOKEN,
         settings: CanisterSettings {
             controllers: Some(vec![caller(), id()]),
             compute_allocation: None,
             memory_allocation: None,
             freezing_threshold: None,
         },
     };
 
     let create_result = create_canister_call(create_args).await?;
     let install_args = encode_args((
         args.name.clone(),
         "PAB".clone(),
         12.clone(),
         400000000.clone(),
         caller().clone(),
     )).expect("Failed to encode arguments.");
 
     match install_canister(
         &create_result.canister_id,
         wasm_module.clone().into_vec(),
         install_args,
     ).await
     {
         Ok(_) => {
             Ok(create_result)
         }
         Err(e) => Err(e),
     }
 }
 
 async fn _charge_token_issue_fee(
     spender_sub_account: Option<Subaccount>,
     from: String,
     to: String,
     value: u128,
 ) {
     unsafe {
         let result: Result<(TransferResult,), _> = call(
             FEE_TOKEN_ID,
             "transferFrom",
             (spender_sub_account, from, to, value),
         ).await;
         match result {
             Ok((tx_res, )) => {
                 match tx_res {
                     TransferResult::Ok(_, _) => {}
                     TransferResult::Err(_) => {
                         trap("charge issue fee failed");
                     }
                 };
             }
             Err(_) => todo!(),
         }
     }
 }
 
 #[cfg(any(target_arch = "wasm32", test))]
 fn main() {}
 
 #[cfg(not(any(target_arch = "wasm32", test)))]
 fn main() {
     candid::export_service!();
     std::print!("{}", __export_service());
 }
 