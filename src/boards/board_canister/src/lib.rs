mod inter_call;
mod record;

use candid::{Principal, candid_method};
use ic_cdk_macros::*;
use ic_cdk::{id, storage};
use ic_cdk::api::caller;
use candid::CandidType;
use board::Board;
use inter_call::{mint_visa_nft_call};
use serde::__private::de::IdentifierDeserializer;
use visa::{Ticket};
use room::Room;
use serde::{Deserialize};
use record::Record;

type LifeCanisterId = Principal;

static mut NAIS: Principal = Principal::anonymous();
static mut OWNER: Principal = Principal::anonymous();

#[derive(Debug, Default, Deserialize, CandidType, Clone)]
struct Committee {
    pub chairman: Vec<LifeCanisterId>,
    pub member: Vec<LifeCanisterId>
}

#[derive(Debug, Default)]
pub struct Point(i32, i32, i32);

#[derive(Debug, Default)]
pub struct Population(Vec<Principal>);

#[derive(Default, Debug)]
struct BoardRooms(Vec<Room>);

#[derive(Default, Debug)]
struct Records(Vec<Record>);

#[derive(Deserialize, CandidType)]
struct UpgradePayload {
    rooms: Vec<Room>,
    records: Vec<Record>,
    population: Vec<Principal>,
    committee: Committee,
    board: Board
}

fn get() -> &'static Board { storage::get::<Board>() }

fn in_committee_chairman(person: &Principal) -> bool {
    let committee = storage::get::<Committee>();
    committee.chairman.contains(person)
}

fn in_committee(person: &Principal) -> bool {
    let committee = storage::get::<Committee>();
    committee.member.contains(person) || committee.chairman.contains(person)
}

fn in_population(person: &Principal) -> bool {
    let population = storage::get::<Population>();
    population.0.contains(person)
}

fn increase_population(person: Principal){
    let population = storage::get_mut::<Population>();
    population.0.push(person);
}

#[init]
#[candid_method(init)]
fn init(owner: Principal, chairman: Principal, nais: Principal) {
    unsafe {
        OWNER = owner;
        let committee = storage::get_mut::<Committee>();
        committee.chairman.push(owner);
        committee.chairman.push(chairman);
        NAIS = nais;
    }
    increase_population(owner);
    increase_population(chairman);
}

fn _only_owner() {
    unsafe {
       if OWNER != caller() {
           ic_cdk::trap("not owner");
       }
    }
}

fn _only_chairman() {
    if !in_committee_chairman(&caller()) {
        ic_cdk::trap("not in committee");
    }
}

#[update(name = "DelegateTo")]
#[candid_method(update, rename = "DelegateTo")]
fn delegate_to(moderator: Principal){
    _only_chairman();

    let committee = storage::get_mut::<Committee>();
    committee.chairman.push(moderator);
}

#[update(name = "GetBoardVisa")]
#[candid_method(update, rename = "GetBoardVisa")]
async fn get_board_visa() {
    unsafe { 
        match mint_visa_nft_call(NAIS, id()).await {
            Ok(_) => (),
            Err(e) => ic_cdk::trap(e.as_str())
        }
    }
}

#[derive(Deserialize, CandidType)]
enum RoomTopic {
    Rooms,
}

#[query(name = "Hi")]
#[candid_method(query, rename = "Hi")]
fn hi() -> (Vec<String>, Vec<Room>) {
    // match topic {
    //     RoomTopic::Rooms => {
            let board = get();
            let board_meta = vec![board.title.clone(), board.cover.clone(), board.about.clone()];
            let br = storage::get::<BoardRooms>();
            (board_meta, br.0.clone())
    //     }
    // }
}

#[query(name = "Fee")]
#[candid_method(query, rename = "Fee")]
fn fee() -> f64 {
    0.0
}

#[update(name = "Pay")]
#[candid_method(update, rename = "Pay")]
fn pay(amount: f64){

}

#[update(name = "OpenRoom")]
#[candid_method(update, rename = "OpenRoom")]
fn open_room(title: String, cover: Option<String>) -> String{
    _only_chairman();
    let br = storage::get_mut::<BoardRooms>();

    let id = (br.0.len() + 1).to_string();
    if in_population(&caller()) {
        let room = room::Room::build(id.clone(), title, cover, caller());
        br.0.push(room.clone());
        ic_cdk::println!("open room {} for {} in {}", room.id, room.owner.to_owned(), ic_cdk::id());
    }else{
        ic_cdk::trap("open room failure");
    }

    id
}

#[update(name = "RefreshRoom")]
#[candid_method(update, rename = "RefreshRoom")]
fn refresh_room(token: String, room_id: String){
    _only_chairman();

    let room = find_room(room_id);
    match room {
        Some(mut r) => (r.token = token),       
        None => ()
    }
}

#[query(name = "FindRoom")]
#[candid_method(query, rename = "FindRoom")]
fn find_room(room_id: String) -> Option<Room> {
    let br = storage::get::<BoardRooms>();
    for i in 0..br.0.len() {
        let room = br.0.get(i);
        match room {
            Some(r) =>  { 
                if r.id == room_id { 
                    return Some(r.clone())
                } 
            }
            None => ()
        }
    }
    None
}

#[update(name = "JoinRoom")]
#[candid_method(update, rename = "JoinRoom")]
fn join_room(ticket: Option<String>, room_id: String) -> String{
    let room = find_room(room_id);
    let token = match room {
        Some(mut r) => {
            if r.can_join(&caller(), ticket) {
                r.audiens.push(caller());
            }
            r.token
        }
        None => String::from("")
    };

    token
}

#[update(name = "LeaveRoom")]
#[candid_method(update, rename = "LeaveRoom")]
fn leave_room(room_id: String){
    let room = find_room(room_id);
    match room {
        Some(mut r) => {
            r.audiens.retain(|x| *x != caller());
        }
        None => ()
    }
}

#[update(name = "Speak")]
#[candid_method(update, rename = "Speak")]
fn speak(room_id: String){
    let room = find_room(room_id);
    match room {
        Some(mut r) => {
            if r.can_speak(&caller()) {
                r.speakers.push(caller());
            }
        }
        None => {}
    }
}

#[update(name = "Like")]
#[candid_method(update, rename = "Like")]
fn like(){

}

fn open_event(){
    _only_owner();
}

fn open_workshop(){
    _only_owner();
}

#[pre_upgrade]
fn pre_upgrade() {
    let committee = storage::get_mut::<Committee>();
    let popu = storage::get_mut::<Population>();
    let records = storage::get_mut::<Records>();
    let rooms = storage::get_mut::<BoardRooms>();
    let board = storage::get_mut::<Board>();

    let up = UpgradePayload {
        rooms: rooms.0.clone(),
        records: records.0.clone(),
        population: popu.0.clone(),
        committee: committee.clone(),
        board: board.clone(),
    };
   
    storage::stable_save((up, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}
