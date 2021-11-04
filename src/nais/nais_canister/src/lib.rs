/**
 * Module     : nais.rs
 * Copyright  : 2021 PAB
 * License    : MIT
 * Maintainer : shiyong <shiyong0248@gmail.com>
 * Stability  : Experimental
 * Description: The UI canister on the "local" network is "r7inp-6aaaa-aaaaa-aaabq-cai" 
                nais canister_id rrkah-fqaaa-aaaaa-aaaaq-cai
                nft canister id rkp4c-7iaaa-aaaaa-aaaca-cai
                anderson canister id qaa6y-5yaaa-aaaaa-aaafa-cai
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
 use serde_bytes::ByteBuf;
 use easy_hasher::easy_hasher;
 
 static mut POPULATION_QUANTITIES: u64 = 0;
 static mut INITIALIZED: bool = false;
 static mut OWNER: Principal = Principal::anonymous();
 static mut FEE_TOKEN_ID: Principal = Principal::anonymous();
//  static mut PAB_TOKEN_CANISTER: Principal = Principal::anonymous();
//  static mut PAB_NFT_CANISTER: Principal = Principal::anonymous();
 const CYCLES_PER_TOKEN: u64 = 4000000000000;
 
 #[derive(Default, Debug)]
 struct GenesisCode(HashMap<String,(Option<String>, Option<Principal>)>);
 #[derive(Default, Debug)]
 struct CitizenCode(HashMap<String,(Option<String>, Option<Principal>)>);
 type LifeCanisterId = Principal;
 type Living = HashMap<String, LifeCanisterId>;
 type Dead = Living;
 type Soul = Living;
 type BoardCanisterId = Principal;
 type Civilization = Vec<BoardCanisterId>;
 
 #[derive(Default)]
 struct CanisterID {
    n_f_t_canister_id: String,
    p_a_b_token_canister_id: String,
    avatar_nft_canister_id: String
 }

 #[derive(Deserialize, CandidType)]
 struct UpgradePayload {
     g_codes: Vec<(String, Option<String>, Option<Principal>)>,
     dead: Vec<(String, Principal)>,
     livings: Vec<(String, Principal)>,
     souls: Vec<(String, Principal)>,
     civils: Civilization,
     initialized: bool,
     owner: Principal,
     pab_token: String,
     pab_nft: String,
     pab_avatar_nft: String,
     life_no: u64
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
 
 fn increase_population(applicant: Principal, life: LifeCanisterId) {
     let population = storage::get_mut::<Living>();
     population.insert(applicant.to_text(), life);
     unsafe { POPULATION_QUANTITIES += 1; }
 }
 
 fn increase_civilization(board: BoardCanisterId) {
     let civils = storage::get_mut::<Civilization>();
     civils.push(board);
 }
 
 fn _must_living_citizen(applicant: Principal, life: Option<LifeCanisterId>) {
    let population = storage::get::<Living>();
    if !population.contains_key(&applicant.to_text()) {
        ic_cdk::trap("not living in this metaverse");
    }        
    match life {
        None => (),
        Some(life_v) => {
            if life_v != population.get(&applicant.to_text()).unwrap().to_owned() {
                ic_cdk::trap("wrong wallet and life pair");
            }
        }
    }
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
 async fn new_life(caller: Principal, inviter: Option<String>) -> Result<LifeCanisterId, String> {
     let life_bytes = storage::get::<LifeWASMBytes>();
     let inviter_principal = inviter.map_or(None, 
        |id| Some(Principal::from_text(id).unwrap()).or(None)
    );
     match &life_bytes.0 {
         None => {
             ic_cdk::trap("Humankind code not emerge.");
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
                        caller,
                        POPULATION_QUANTITIES,
                        inviter_principal,
                        id(),
                    ))
                    .expect("Failed to encode arguments.");
                    match install_canister(&create_result.canister_id, o.clone().into_vec(),
                                            install_args, None).await
                    {
                        Ok(_) => { Ok(create_result.canister_id) }
                        Err(e) => ic_cdk::trap(format!("new life mission failed due to : {}",e).as_str())
                    }    
                 }
             }
        }
     }
 }
 async fn new_board(owner: Principal, chairman: Principal) -> Result<BoardCanisterId, String> {
     let board_bytes = storage::get::<BoardWASMBytes>();
     match &board_bytes.0 {
         None => {
             ic_cdk::trap("civilization code not emerge.");
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
                Ok(create_result) => {
                    let install_args = encode_args((
                        owner,
                        chairman,
                        id(),
                    ))
                    .expect("Failed to encode arguments.");
                     match install_canister(&create_result.canister_id, o.clone().into_vec(),
                                            install_args, None).await
                     {
                         Ok(_) => { Ok(create_result.canister_id) }
                         Err(e) => ic_cdk::trap(format!("new board mission failed due to : {}",e).as_str())
                    }
                }
            }
         },
     }
}
async fn new_nft_contract(wtype: WasmType) -> Result<String, String> {
     let nft_bytes: Option<serde_bytes::ByteBuf>;
     let name;
     let symbol;
     let canister_id;
     match wtype {
         WasmType::VisaNFT => {
             nft_bytes = storage::get::<VisaNFTWASMBytes>().0.clone();
             name = String::from("PAB Visa NFT");
             symbol = String::from("PVN");
             canister_id = Principal::from_text(String::from("vnuj4-hqaaa-aaaai-aa25a-cai")).unwrap();
          }
         WasmType::AvatarNFT => {
            nft_bytes = storage::get::<AvatarNFTWASMBytes>().0.clone();
            name = String::from("PAB Avatar NFT");
            symbol = String::from("PAN");
            canister_id = Principal::from_text(String::from("vkvpi-kiaaa-aaaai-aa25q-cai")).unwrap();
        }
         _ => { ic_cdk::trap("nft type error") }
     }
 
     match nft_bytes {
         None => {
             ic_cdk::trap("NFT code not emerge.");
         }
         Some(o) => {
            let meta = NFTContractMeta{name: name.to_string(), symbol: symbol.to_string()};
            let install_args = encode_args((
                id(), meta,
            )).unwrap_or(vec![]);
    
            match install_canister(&canister_id, o.clone().into_vec(),
                                install_args, None).await
            {
                Ok(_) => {
                    init_nft_canister(&canister_id, &id(), name.to_string(), symbol.to_string()).await?;
                    Ok(canister_id.to_string())
                } 
                Err(e) => ic_cdk::trap(format!("install nft contract mission failed due to : {}",e).as_str())
            }
         },
     }
 }

 async fn new_token_contract() -> Result<String, String> {
    let token_bytes = storage::get::<PABWalletWASMBytes>();
    let pab_token_canister_id = 
        Principal::from_text(String::from("v3wa4-myaaa-aaaai-qadlq-cai")).unwrap();

    match &token_bytes.0 {
        None => {
            ic_cdk::trap("PAB Token code not emerge.");
        }
        Some(o) => {
            let install_args = encode_args((
                "PartyBoard Currency", "PAB", 12 as u64, 20000000000 as u64, id(),
            )).unwrap_or(vec![]);
    
            match install_canister(&pab_token_canister_id, o.clone().into_vec(),
                                install_args, None).await
            {
                Ok(_) => Ok(pab_token_canister_id.to_string()),
                Err(e) => ic_cdk::trap(format!("create token contract mission failed due to : {}",e).as_str())
            }
        }
    }
}

 #[query(name = "GenesisCode")]
 #[candid_method(query, rename = "GenesisCode")]
fn genesis_code() -> String{
    _only_owner();
    _must_initialized();

    let mut code_str = String::from("");
    let codes = storage::get::<GenesisCode>();
    for (key, _value) in codes.0.clone() {
        code_str = code_str + &key + ",";
    }

    code_str
 }

 #[query(name = "Citizens")]
 #[candid_method(query, rename = "Citizens")]
fn citizens() -> String{
    _must_initialized();

    let mut id_str = String::from("");
    let livings = storage::get::<Living>();
    for (key, _value) in livings {
        id_str = id_str + &key + ",";
    }

    id_str
 }

 #[update(name = "GetInviteCodes")]
 #[candid_method(update, rename = "GetInviteCodes")]
 async fn get_invite_codes() -> Vec<String>{
    let citi_codes = storage::get_mut::<CitizenCode>();
    let mut codes: Vec<String> = vec![];
    for _ in 1..3 {
        match call(Principal::management_canister(), "raw_rand", ())
        .await
        {
            Ok(b) => { 
                let (bytes,): (Vec<u8>,) = b;
                let code = easy_hasher::Hash::from_vec(&bytes).to_hex_string();
                codes.push(code.clone());
                citi_codes.0.insert(code, (Some(caller().to_text()), None));
            },
            Err(e) => {ic_cdk::trap(e.1.as_str());}
        }
    }

    codes
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
                    codes.0.insert(code, (None, None));
                },
                Err(e) => {ic_cdk::trap(e.1.as_str());}
            }
        }
    }

    Ok(())
}
 
#[query(name = "Hi")]
#[candid_method(query, rename = "Hi")]
pub fn hi() -> String{
    _must_initialized();
    let canister = storage::get::<CanisterID>();
    unsafe {
        format!("Hi, {}; {}; {}; {}; {};", OWNER, POPULATION_QUANTITIES, canister.n_f_t_canister_id, 
                canister.p_a_b_token_canister_id, canister.avatar_nft_canister_id)
    }
}

#[update(name = "UpgradeCanister")]
#[candid_method(update, rename = "UpgradeCanister")]
pub async fn upgrade_canister(canister: Principal, wasm_type: WasmType){
    let o = match wasm_type {
        WasmType::Board => storage::get::<BoardWASMBytes>().0.clone().unwrap_or(ByteBuf::new()),
        WasmType::Life => storage::get::<LifeWASMBytes>().0.clone().unwrap_or(ByteBuf::new()),
        WasmType::AvatarNFT => storage::get::<AvatarNFTWASMBytes>().0.clone().unwrap_or(ByteBuf::new()),
        WasmType::VisaNFT => storage::get::<VisaNFTWASMBytes>().0.clone().unwrap_or(ByteBuf::new()),
        WasmType::PABToken =>  storage::get::<PABWalletWASMBytes>().0.clone().unwrap_or(ByteBuf::new()),
    };

    let install_args = encode_args(()).expect("Failed to encode arguments.");
     match install_canister(&canister, o.clone().into_vec(),
                            install_args, Some(InstallMode::Upgrade)).await
     {
         Ok(_) => (),
         Err(e) => ic_cdk::trap(format!("upgrade mission failed due to : {}",e).as_str())
    }

}

#[update(name = "ApplyCitizenship")]
#[candid_method(update, rename = "ApplyCitizenship")]
pub async fn apply_citizenship(code: String) -> Option<LifeCanisterId> {
     _must_initialized();
    
     let mut is_genisis = true;
     let applicant = caller();
     let population = storage::get::<Living>();
     if population.contains_key(&applicant.to_text()) {
         return population.get(&applicant.to_text()).copied();
     }
 
     let codes: &HashMap<String,(Option<String>, Option<Principal>)>;
     let gcodes = storage::get_mut::<GenesisCode>();
     let ccodes = storage::get_mut::<CitizenCode>();
     if gcodes.0.contains_key(&code) {
         codes = &gcodes.0;
     }else {
         is_genisis = false;
         if ccodes.0.contains_key(&code) {
             codes = &ccodes.0;
         }else{
             ic_cdk::trap("you are not in the invite list");
         }
     }
     
     let inviter = codes.get(code.as_str()).unwrap_or(&(None,None)).clone().0;
     let life_can = codes.get(code.as_str()).unwrap_or(&(None,None)).clone().1;
     match life_can {
        None => {
             let life = new_life(caller().clone(), inviter.clone()).await;
             match life {
                Ok(life) => {
                    increase_population(applicant, life);
                    let mut cp: HashMap<String,(Option<String>, Option<Principal>)> = HashMap::from(codes.clone());
                    cp.insert(code.clone(), (inviter.clone(), Some(life)));
                    let res = Principal::from_text(storage::get::<CanisterID>().n_f_t_canister_id.to_string());
                    match res {
                        Ok(nft_canister) => {
                            let result = mint_citizen_nft(&nft_canister, life).await;
                            match result {
                                Ok(nft_id) => {
                                    if is_genisis {
                                        gcodes.0 = cp;
                                    }else{
                                        ccodes.0 = cp;
                                    }
                                    let born = life_be_born(&life, nft_id).await;
                                    match born {
                                        Err(e) =>  ic_cdk::trap(e.as_str()),
                                        Ok(_) => ()
                                    }
                                    Some(life.clone())
                                }
                                Err(e) => ic_cdk::trap(e.as_str())
                            }
                        }
                        Err(e) => ic_cdk::trap(e.to_string().as_str()),
                    }
                }
                Err(e) => ic_cdk::trap(e.as_str()),
            }
        }
        Some(_) => life_can
    }
 }
 
 #[update(name = "DeployNFTContract")]
 #[candid_method(update, rename = "DeployNFTContract")]
 async fn deploy_nft_contract(wtype: WasmType) -> String{
     _only_owner();
 
     match new_nft_contract(wtype.clone()).await {
         Ok(n) => {
            let canister = storage::get_mut::<CanisterID>();
            match wtype {
                WasmType::AvatarNFT => canister.avatar_nft_canister_id = n.to_string(),
                WasmType::VisaNFT => canister.n_f_t_canister_id = n.to_string(),
                _ => ()
            }
            n
         }
         Err(e) => ic_cdk::api::trap(e.as_str())
     }
 }
 
 #[update(name = "DeployTokenContract")]
 #[candid_method(update, rename = "DeployTokenContract")]
 async fn deploy_token_contract() -> String{
     _only_owner();
 
     match new_token_contract().await {
         Ok(n) => {
            let canister = storage::get_mut::<CanisterID>();
            canister.p_a_b_token_canister_id = n.to_string();
            n
         }
         Err(e) => ic_cdk::api::trap(e.as_str())
     }
 }

 #[update(name = "BuildCivilization")]
 #[candid_method(update, rename = "BuildCivilization")]
 async fn build_civilization(chairman: Principal) -> BoardCanisterId {
     _must_initialized();
     _must_living_citizen(chairman, Some(caller()));
 
     let board = new_board(caller(), chairman).await;
     match board {
         Ok(board_canister_id) => {
            increase_civilization(board_canister_id);
            board_canister_id
         }
         Err(e) => ic_cdk::trap(e.as_str())
     }
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
 
 #[update(name = "uploadWasm")]
 #[candid_method(update, rename = "uploadWasm")]
 fn upload_wasm(args: StoreWASMArgs) {
     _must_initialized();
     _only_owner();
 
     match args.wasm_type {
         WasmType::Board => storage::get_mut::<BoardWASMBytes>().0 = Some(serde_bytes::ByteBuf::from(args.wasm_module.clone())),
         WasmType::Life => storage::get_mut::<LifeWASMBytes>().0 = Some(serde_bytes::ByteBuf::from(args.wasm_module.clone())),
         WasmType::AvatarNFT => storage::get_mut::<AvatarNFTWASMBytes>().0 = Some(serde_bytes::ByteBuf::from(args.wasm_module.clone())),
         WasmType::VisaNFT => storage::get_mut::<VisaNFTWASMBytes>().0 = Some(serde_bytes::ByteBuf::from(args.wasm_module.clone())),
         WasmType::PABToken =>  storage::get_mut::<PABWalletWASMBytes>().0 = Some(serde_bytes::ByteBuf::from(args.wasm_module.clone())),
     }
 }
 
 #[update(name = "RequestAvatarNft")]
 #[candid_method(update, rename = "RequestAvatarNft")]
 async fn request_avatar_nft(args: AvatarBytesArgs) -> String {
    _must_living_citizen(caller(), None);

    let res = Principal::from_text(storage::get::<CanisterID>().avatar_nft_canister_id.to_string());
    let avatar_id = match res {
        Ok(nft_canister) => {
            let result = mint_avatar_nft(&nft_canister, caller(), args.image_bytes).await;
                match result {
                    Ok(nft_id) => nft_id,
                    Err(e) => ic_cdk::trap(e.as_str())
                }
        }
        Err(e) => ic_cdk::trap(e.to_string().as_str()),
    };

    avatar_id
}

 #[update(name = "IssueToken")]
 #[candid_method(update, rename = "IssueToken")]
 async fn issue_token(args: IssueTokenArgs) -> Result<IssueResult, String> {
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
         args.symbol.clone(),
         args.decimals,
         args.total_supply,
         caller().clone(),
     )).expect("Failed to encode arguments.");
 
     match install_canister(
         &create_result.canister_id,
         wasm_module.clone().into_vec(),
         install_args, None
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
 
 #[query(name = "Balance")]
 #[candid_method(query, rename = "Balance")]
 fn balance() -> u64{
     ic_cdk::api::canister_balance()
 }

 #[cfg(any(target_arch = "wasm32", test))]
 fn main() {}
 
 #[cfg(not(any(target_arch = "wasm32", test)))]
 fn main() {
     candid::export_service!();
     std::print!("{}", __export_service());
 }
 
 #[pre_upgrade]
fn pre_upgrade() {
    let totals = unsafe{ POPULATION_QUANTITIES };
    let owner = unsafe{ OWNER };
    let init = unsafe{ INITIALIZED };
    let canister = storage::get::<CanisterID>();
    let mut civils = Vec::new();
    for v in storage::get_mut::<Civilization>().iter() {
        civils.push(*v);
    }
    let mut livings = Vec::new();
    for (k, v) in storage::get_mut::<Living>().iter() {
        livings.push((k.to_string(), *v));
    }
    let mut dead = Vec::new();
    for (k, v) in storage::get_mut::<Dead>().iter() {
        dead.push((k.to_string(), *v));
    }
    let mut soul = Vec::new();
    for (k, v) in storage::get_mut::<Soul>().iter() {
        soul.push((k.to_string(), *v));
    }
    let mut g_codes = Vec::new();
    for (k, v) in storage::get_mut::<GenesisCode>().0.iter() {
        let v1 = v.clone();
        g_codes.push((k.to_string(), v1.0, v1.1));
    }
    let up = UpgradePayload {
        g_codes,
        dead,
        livings,
        souls: soul,
        civils,
        initialized: init,
        owner,
        pab_nft: canister.n_f_t_canister_id.to_string(),
        pab_avatar_nft: canister.avatar_nft_canister_id.to_string(),
        pab_token: canister.p_a_b_token_canister_id.to_string(),
        life_no: totals
    };
    
    storage::stable_save((up, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (down, ) : (UpgradePayload, ) = storage::stable_restore().unwrap();
    unsafe {
        POPULATION_QUANTITIES = down.life_no;
        OWNER = down.owner;
        INITIALIZED = down.initialized;
    }
    storage::get_mut::<CanisterID>().n_f_t_canister_id.clone_from(&down.pab_nft);
    storage::get_mut::<CanisterID>().p_a_b_token_canister_id.clone_from(&down.pab_token);
    storage::get_mut::<CanisterID>().avatar_nft_canister_id.clone_from(&down.pab_avatar_nft);
    for v in down.civils {
        storage::get_mut::<Civilization>().push(v);
    }
    for (k, v) in down.livings {
        storage::get_mut::<Living>().insert(k, v);
    }
    for (k, v) in down.dead {
        storage::get_mut::<Dead>().insert(k, v);
    }
    for (k, v) in down.souls {
        storage::get_mut::<Soul>().insert(k, v);
    }
    for (k, v1, v2) in down.g_codes {
        let inner = (v1, v2);
        storage::get_mut::<GenesisCode>().0.insert(k, inner);
    }
}