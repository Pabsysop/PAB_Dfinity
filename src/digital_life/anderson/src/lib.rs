mod inter_call;
mod record;

use std::string::String;
use std::collections::HashMap;
use ic_cdk::storage;
use candid::{CandidType, Principal, candid_method};
use ic_cdk::api::{caller, time};
use ic_cdk_macros::*;
use human::{Human, Mood};
use inter_call::{request_invite_code, create_board_call, open_room_call};
use visa::{Ticket, Visa, VisaType};
use nft::{NFT, NFTSrc, NFTType};
use record::Record;
use serde::Deserialize;

static mut BORN: bool = false;
static mut BIRTHDAY: u64 = 0;
static mut LIKES: u64 = 0;
static mut INCENTIVES: u64 = 0;
static mut TOTALLIKES: u64 = 0;
static mut TOTALINCENTIVES: u64 = 0;
static mut OWNER: Principal = Principal::anonymous();
static mut NAIS: Principal = Principal::anonymous();
static mut INVITER: Option<Principal> = None;
static mut PAB_TOKEN_CANISTER: Principal = Principal::anonymous();
static mut PAB_NFT_CANISTER: Principal = Principal::anonymous();
static mut LIFENO: u64 = 0;

type Tickets = Vec<Ticket>;
type Citizenship = Visa;
type Password = String;

#[derive(Default, Debug)]
struct MyRecords(Vec<Record>);

#[derive(Default, Debug)]
struct MyBoards(Vec<Principal>);

#[derive(Default, Debug)]
struct MyFollowings(Vec<Principal>);

#[derive(Default, Debug)]
struct MyFollowers(Vec<Principal>);

#[derive(Default, Debug)]
struct BoardMembers(HashMap<String,Option<Visa>>);

#[derive(Default, Debug)]
struct AvatarNFT(Vec<NFT>);

#[derive(Deserialize, CandidType)]
struct UpgradePayload {
    totalincentives: u64,
    totallikes: u64,
    nais: Principal,
    born: bool,
    birthday: u64,
    owner: Principal,
    inviter: Option<Principal>,
    life_no: u64,
    citizen: Citizenship,
    human: Human,
    seed: String,
    nfts: Vec<NFT>,
    visas: Vec<(String,Option<Visa>)>,
    tickets: Vec<Ticket>,
    boards: Vec<Principal>,
}

#[derive(Deserialize, CandidType)]
enum TalkTopic {
    AboutBoards,
    AboutPeople
}

#[init]
#[candid_method(init)]
fn init(owner: Principal, lifeno: u64, inviter: Option<Principal>, nais: Principal) {
    unsafe {
        OWNER = owner;
        LIFENO = lifeno;
        INVITER = inviter;
        NAIS = nais;
    }
}


#[update(name = "setToken")]
#[candid_method(update, rename = "setToken")]
fn set_token(pab:Option<Principal>, nft:Option<Principal>){
    unsafe {
        match pab {
            Some(o) => PAB_TOKEN_CANISTER =o,
            _ => {}
        }
        match nft {
            Some(o) => PAB_NFT_CANISTER =o,
            _ => {}
        }
    }
}

fn _only_owner() {
    unsafe {
       if OWNER != caller() {
           ic_cdk::trap("not owner");
       }
    }
}

fn _not_owner() {
    unsafe {
       if OWNER == caller() {
           ic_cdk::trap("not owner");
       }
    }
}

fn _only_nais() {
    unsafe {
       if NAIS != caller() {
           ic_cdk::trap("not owner");
       }
    }
}

fn _must_borned() {
    unsafe {
       if BORN != true {
           ic_cdk::trap("not born");
       }
    }
}

#[update(name = "Born")]
#[candid_method(update, rename = "Born")]
pub fn born(citizen_nft_id: String) -> String{
    
    let me = storage::get_mut::<Human>();
    unsafe {
        if BORN {
            ic_cdk::trap("Borned");
        }
        BORN = true;
        BIRTHDAY = time();
        me.born(LIFENO);
        
        match INVITER {
            Some(person) => me.add_following(person),
            None => ()
        }    
    }
    {
        let citizen_nft = NFT{ id: citizen_nft_id, src: NFTSrc::DFINITY };
        let citizen = storage::get_mut::<Citizenship>();
        citizen.visa_type = VisaType::Citizenship;
        citizen.nft = citizen_nft;
        citizen.issue_date = ic_cdk::api::time();
    }

    me.clone().name
}

#[query(name = "Hi")]
#[candid_method(query, rename = "Hi")]
pub fn hi() -> String{
    _must_borned();
    unsafe {
        format!("Hi, {}; {:?}; {}; {}; {};", LIFENO, INVITER, OWNER, BIRTHDAY, NAIS)
    }
}

#[query(name = "HowOldAreYou")]
#[candid_method(query, rename = "HowOldAreYou")]
pub fn how_old_are_you() -> u64{
    unsafe { time() - BIRTHDAY }
}

#[query(name = "WhatsYourName")]
#[candid_method(query, rename = "WhatsYourName")]
pub fn whats_your_name() -> String{
    let me = storage::get_mut::<Human>();
    me.clone().name
}

#[query(name = "LookLike")]
#[candid_method(query, rename = "LookLike")]
pub fn look_like() -> NFT{
    let me = storage::get::<AvatarNFT>();
    match me.0.get(0) {
        Some(v) => v.clone(),
        None => ic_cdk::trap("mask man")
    }
}

#[update(name = "Makeup")]
#[candid_method(update, rename = "Makeup")]
pub fn makeup(idx: String, src: NFTSrc){
    _only_owner();

    let view = NFT {
        id: idx,
        src
    };
    let me = storage::get_mut::<AvatarNFT>();
    me.0.insert(0, view);
}

#[update(name = "UpdatePassword")]
#[candid_method(update, rename = "UpdatePassword")]
pub fn update_password(pass: String){
    _only_owner();

    let password = storage::get_mut::<Password>();
    password.clone_from(&pass);
}

#[update(name = "WakeUp")]
#[candid_method(update, rename = "WakeUp")]
pub fn wake_up() {
    _only_owner();

    let me = storage::get_mut::<Human>();
    me.mood = Mood::Clear;
}

#[update(name = "ReceiveNFT")]
#[candid_method(update, rename = "ReceiveNFT")]
pub fn rcv_nft(nft_type: NFTType, board: Principal, nft_id: String) {
    match nft_type {
        NFTType::VISA => {
            let visa_nft = NFT{ id: nft_id, src: NFTSrc::DFINITY };
            let myvisas = storage::get_mut::<BoardMembers>();
            let bvisa = Visa {
                visa_type: VisaType::BoardMember,
                board: Some(board),
                expire_date: None,
                issue_date: ic_cdk::api::time(),
                nft: visa_nft,
            };
            myvisas.0.insert(board.to_string(), Some(bvisa));
        }
        _ => ()
    }
}

#[update(name = "ReceiveInviteCode")]
#[candid_method(update, rename = "ReceiveInviteCode")]
pub async fn rcv_invite_code() -> Vec<String>{
    unsafe {
        let res = request_invite_code(&NAIS).await;
        match res {
            Some(v) => v,
            None => vec![]
        }
    }
}

#[update(name = "Sleep")]
#[candid_method(update, rename = "Sleep")]
fn sleep() {
    _only_owner();
    let me = storage::get_mut::<Human>();
    me.mood = Mood::Dizzy;
}

#[update(name = "Die")]
#[candid_method(update, rename = "Die")]
fn die(){
//    _only_owner();
    let mb = storage::get_mut::<MyBoards>();
    mb.0.clear();
}

#[update(name = "CreateBoard")]
#[candid_method(update, rename = "CreateBoard")]
async fn create_board() -> Principal{
    _only_owner();

    unsafe {
        let board_id = create_board_call(&NAIS, caller()).await;
        match board_id {
            Some(id) => {
                let mb = storage::get_mut::<MyBoards>();
                mb.0.push(id);
                id
            }
            None => ic_cdk::trap("create board error")
        }
    }
}

#[update(name = "CreateRoom")]
#[candid_method(update, rename = "CreateRoom")]
async fn create_room(title: String, cover: Option<String>){
    _only_owner();

    let mb = storage::get_mut::<MyBoards>();
    if mb.0.len() <= 0 {
        unsafe {
            let board_id = create_board_call(&NAIS, caller()).await;
            match board_id {
                Some(id) => mb.0.push(id),
                None => ic_cdk::trap("create board error")
            }
        }
    }
    let _room_id = open_room_call(
        mb.0.get(0).unwrap(), title, cover
    ).await;
}

#[query(name = "Follows")]
#[candid_method(query, rename = "Follows")]
fn follows() -> (Vec<(Principal, u64)>, Vec<(Principal, u64)>){
    _only_owner();

    let mut followers: Vec<(Principal, u64)> = vec![];
    let mut followings: Vec<(Principal, u64)> = vec![];
    let me = storage::get::<Human>();
    match me.clone().connections {
        None => (),
        Some(c) => {
            followers = c.followers;
            followings = c.followings;
        }
    }

    (followers, followings)
}

#[update(name = "Follow")]
#[candid_method(update, rename = "Follow")]
async fn follow(f: Principal) {
    _only_owner();

    let me = storage::get_mut::<Human>();
    inter_call::follow(&f).await;
    me.add_following(f);
}

#[update(name = "FollowMe")]
#[candid_method(update, rename = "FollowMe")]
fn follow_me() {
    _not_owner();

    let me = storage::get_mut::<Human>();
    me.add_followers(caller());
}

#[query(name = "Talk")]
#[candid_method(query, rename = "Talk")]
fn talk(topic: TalkTopic) -> Vec<Principal> {
    match topic {
        TalkTopic::AboutBoards => {
            let mb = storage::get::<MyBoards>();
            mb.0.clone()
        }
        TalkTopic::AboutPeople => todo!(),
    }
}

#[update(name = "Listen")]
#[candid_method(update, rename = "Listen")]
pub async fn listen(board: Principal, room: String) -> String{
    _only_owner();
    let res = inter_call::listen(&board, room, None).await;
    match res {
        Ok(session) => session,
        Err(e) => ic_cdk::trap(e.as_str())
    }
}

#[update(name = "Speak")]
#[candid_method(update, rename = "Speak")]
pub async fn speak(board: Principal, room: String) -> String{
    _only_owner();
    let res = inter_call::speak(&board, room).await;
    match res {
        Ok(session) => session,
        Err(e) => ic_cdk::trap(e.as_str())
    }
}

#[update(name = "See")]
#[candid_method(update, rename = "See")]
fn see(){
    _only_owner();

}

#[update(name = "Like")]
#[candid_method(update, rename = "Like")]
fn like(){
    _not_owner();

    unsafe{
        LIKES += 1;
    }
}

#[query(name = "Likes")]
#[candid_method(query, rename = "Likes")]
fn likes() -> Vec<u64>{
    unsafe{
        vec![LIKES, TOTALLIKES, INCENTIVES, TOTALINCENTIVES]
    }
}

#[update(name = "Paid")]
#[candid_method(update, rename = "Paid")]
fn paid(){
    _only_nais();

    unsafe{
        TOTALLIKES += LIKES;
        TOTALINCENTIVES += INCENTIVES;
        LIKES = 0;
        INCENTIVES = 0;
    }
}

#[update(name = "Pay")]
#[candid_method(update, rename = "Pay")]
fn pay(){
    _only_nais();
    
    unsafe{
        INCENTIVES += 1;
    }
}

#[update(name = "Record")]
#[candid_method(update, rename = "Record")]
fn record(content: Vec<u8>){
    _only_owner();

    let records = storage::get_mut::<MyRecords>();
    records.0.push(Record(content));
}

#[pre_upgrade]
fn pre_upgrade() {
    let nais = unsafe { NAIS };
    let totallikes = unsafe { TOTALLIKES };
    let totalincentives = unsafe { TOTALINCENTIVES };
    let born = unsafe { BORN };
    let owner = unsafe { OWNER };
    let inviter = unsafe { INVITER };
    let life_no = unsafe { LIFENO };
    let birthday = unsafe { BIRTHDAY };
    let mut nfts = Vec::new();
    for v in storage::get::<AvatarNFT>().0.iter() {
        nfts.push(v.clone());
    }
    let mut visas = Vec::new();
    for v in storage::get::<BoardMembers>().0.iter() {
        visas.push((v.0.clone(), v.1.clone()));
    }
    let citi = storage::get::<Citizenship>();
    let citizen = citi;

    let mut tickets = Vec::new();
    for v in storage::get::<Tickets>().iter() {
        tickets.push(v.clone());
    }
    let seed = storage::get::<Password>();
    let human = storage::get::<Human>();
    let boards = storage::get::<MyBoards>();

    let up = UpgradePayload {
        nais,
        totallikes,
        totalincentives,
        born,
        birthday,
        owner,
        inviter,
        life_no,
        citizen: citizen.clone(),
        human: human.clone(),
        seed: seed.to_string(),
        nfts,
        visas,
        tickets,
        boards: boards.0.clone(),
    };
   
    storage::stable_save((up, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (down, ) : (UpgradePayload, ) = storage::stable_restore().unwrap();
    unsafe {
        INVITER = down.inviter;
        OWNER = down.owner;
        BORN = down.born;
        BIRTHDAY = down.birthday;
        LIFENO = down.life_no;
        TOTALINCENTIVES = down.totalincentives;
        TOTALLIKES = down.totallikes;
        NAIS = down.nais;
    }
    let citi = storage::get_mut::<Citizenship>();
    citi.clone_from(&down.citizen);
    let seed = storage::get_mut::<Password>();
    seed.clone_from(&down.seed);
    for v in down.nfts {
        storage::get_mut::<AvatarNFT>().0.push(v);
    }
    for v in down.visas {
        storage::get_mut::<BoardMembers>().0.insert(v.0, v.1);
    }
    for v in down.tickets {
        storage::get_mut::<Tickets>().push(v);
    }
    for v in down.boards {
        storage::get_mut::<MyBoards>().0.push(v);
    }

    let human = storage::get_mut::<Human>();
    human.clone_from(&down.human);
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}
