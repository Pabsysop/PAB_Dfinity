mod inter_call;

use ic_cdk::{storage};
use candid::{CandidType, Principal, candid_method};
use ic_cdk::api::{caller, time};
use ic_cdk_macros::*;
use human::{Human, Mood};
use visa::{ Visa, Ticket };
use nft::{NFT};
use serde::Deserialize;

static mut BORN: bool = false;
static mut BIRTHDAY: u64 = 0;
static mut OWNER: Principal = Principal::anonymous();
static mut INVITER: Principal = Principal::anonymous();
static mut PAB_TOKEN_CANISTER: Principal = Principal::anonymous();
static mut PAB_NFT_CANISTER: Principal = Principal::anonymous();
static mut LIFENO: u64 = 0;

type Visas = Vec<Visa>;
type Tickets = Vec<Ticket>;
type Citizenship = Visa;
type InviteCode = Vec<String>;
type Password = String;
type NFTs = Vec<NFT>;

#[derive(Deserialize, CandidType)]
struct UpgradePayload {
    human: Human,
    pass: String,
    nfts: NFTs,
}

#[init]
#[candid_method(init)]
fn init(owner: Principal, lifeno: u64, inviter: Option<Principal>, citizenship: Option<Visa>
        , pab: Option<Principal>, nft: Option<Principal>) {
    unsafe {
        OWNER = owner;
        LIFENO = lifeno;
        match inviter {
            Some(o) => INVITER =o,
            _ => {}
        }
        match pab {
            Some(o) => PAB_TOKEN_CANISTER =o,
            _ => {}
        }
        match nft {
            Some(o) => PAB_NFT_CANISTER =o,
            _ => {}
        }
        match citizenship {
            Some(o) => {
                let visas = storage::get_mut::<Visas>();
                visas.push(o)
            }
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

fn _must_borned() {
    unsafe {
       if BORN != true {
           ic_cdk::trap("uninitialized");
       }
    }
}

#[update(name = "Born")]
#[candid_method(update, rename = "Born")]
pub fn born() -> Result<String, ()>{
    _only_owner();

    let me = storage::get_mut::<Human>();
    unsafe {
        if BORN {
            ic_cdk::trap("Borned");
        }
        BORN = true;
        BIRTHDAY = time();
        let isborn = me.born(caller().to_text(), LIFENO);
        match isborn {
            Err(e) => ic_cdk::trap(e.to_string().as_str()),
            _ => ()
        }
    }

    Ok(me.clone().ontology.name)
}

#[update(name = "HowOldAreYou")]
#[candid_method(update, rename = "HowOldAreYou")]
pub fn how_old_are_you() -> u64{
    unsafe { time() - BIRTHDAY }
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
pub fn wake_up() -> Mood {
    _only_owner();

    let me = storage::get_mut::<Human>();
    me.mood = Mood::Clear;
    me.mood.clone()
}

#[update(name = "ReceiveNFT")]
#[candid_method(update, rename = "ReceiveNFT")]
pub fn rcv_nft(nft: NFT) {
    let nfts = storage::get_mut::<NFTs>();
    nfts.push(nft)
}

#[update(name = "Sleep")]
#[candid_method(update, rename = "Sleep")]
fn sleep() {
    _only_owner();
}

#[update(name = "Die")]
#[candid_method(update, rename = "Die")]
fn die(){
    _only_owner();
}

#[update(name = "Talk")]
#[candid_method(update, rename = "Talk")]
fn talk(channel: String){

}

#[update(name = "Listen")]
#[candid_method(update, rename = "Listen")]
fn listen(channel: String){

}

#[update(name = "Record")]
#[candid_method(update, rename = "Record")]
fn record(){
    _only_owner();
}

#[update(name = "Like")]
#[candid_method(update, rename = "Like")]
fn like(){
    _only_owner();
}

#[update(name = "See")]
#[candid_method(update, rename = "See")]
fn see(){}

#[pre_upgrade]
fn pre_upgrade() {
    let me = storage::get::<Human>();
    storage::stable_save((me,)).unwrap();
    return;
}

#[post_upgrade]
fn post_upgrade() {
    let me = storage::get_mut::<Human>();

    let res: Result<(Human,), String> = storage::stable_restore();
    match res {
        Ok((old,)) => {
            me.ontology = old.ontology;
            return;
        }
        Err(_) => return
    }
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}
