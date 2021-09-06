mod inter_call;

use candid::{CandidType, Principal, candid_method};
use ic_cdk_macros::*;
use ic_cdk::storage;
use ic_cdk::api::caller;
use board::Board;
use inter_call::{mint_visa_nft_call, send_nft_call, listen_to};
use nft::{NFTPayload, NftEgg, Property, Value};
use visa::{Ticket, Visa};
use room::Room;
use workshop::Workshop;

static mut PAB_TOKEN_CANISTER: Principal = Principal::anonymous();
static mut PAB_NFT_CANISTER: Principal = Principal::anonymous();

fn get() -> &'static Board { storage::get::<Board>() }

#[init]
#[candid_method(init)]
fn init(owner: Option<Principal>, pab: Option<Principal>, nft: Option<Principal>) {
    unsafe {
        match owner {
            Some(o) => get().add_chairman(o),
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
    }
}

fn _only_chairman() {
    unsafe {
        assert!(get().in_committee_chairman(&caller().to_text()), "caller is not the owner");
    }
}

fn delegate_to(controller: Principal){
    _only_chairman();

    let mut b = get();
    b.add_chairman(controller);
}

fn send_invite(code: String, to: Principal){
    _only_chairman();

    let mut b = get();
    b.add_invites((to.to_text(), code))
}

fn request_invite_code() {
    _only_chairman();

}

fn request_board_visa() {
    _only_chairman();

    let args = NftEgg{ 
        payload: NFTPayload{
            payload: 0,
            staged_data: vec![],
        }, 
        content_type: Default::default(), 
        owner: Principal::anonymous(), 
        properties: Property{
            name: Default::default(),
            value: Value::Empty,
            immutable: false,
        }, 
        is_private: false 
    };
    unsafe { mint_visa_nft_call(PAB_NFT_CANISTER, args); }
}

fn open_event(){
    if get().in_population(&caller().to_text()) {
        let event = event::Event::default();
        let board = storage::get_mut::<Board>();
        board.add_event(event)
    }
}

fn open_workshop(){
    if get().in_population(&caller().to_text()) {
        let workshop = Workshop::default();
        let board = storage::get_mut::<Board>();
        board.add_workshop(workshop)
    }
}
fn open_room(){
    if get().in_population(&caller().to_text()) {
        let room = room::Room::default();
        let board = storage::get_mut::<Board>();
        board.add_room(room)
    }
}
fn find_room(room_id: String) -> Option<Room> {
    let b = storage::get::<Board>();
    for i in 0..b.rooms.len() {
        let room: Option<&Room> = b.rooms.get(i);
        match room {
            Some(r) => if r.id == room_id { Some(r) }
            None => {None}
        }
    }
    None
}
fn join_room(ticket: Option<Ticket>, room_id: String){
    let room = find_room(room_id);
    match room {
        Some(mut r) => {
            match ticket {
                Some(t) => {
                    if r.tickets.iter().any(|e| e == t) {
                        r.add_member(caller().to_text())
                    }
                }
                None => {
                    if r.tickets.is_empty() {
                        r.add_member(caller().to_text())
                    }
                }
            }
        }
        None => {}
    }
}
fn start_talk(room_id: String){
    let room = find_room(room_id);
    match room {
        Some(r) => {
            if r.can_start(&caller().to_text()) {
                for i in r.members.len() {
                    match r.members.get(i) {
                        Some(m) => { listen_to(Principal::from_text(m)?, r.channel.session.clone()) }
                        _ => {}
                    }
                }
            }
        }
        _ => {}
    }
}
fn fee() -> f64 {}
fn pay(amount: f64){}
fn like(){}
fn relations(){}

#[pre_upgrade]
fn pre_upgrade() {
    let board = get();
    storage::stable_save((board,)).unwrap();
    return;
}

#[post_upgrade]
fn post_upgrade() {
    let board = storage::get_mut::<Board>();

    let res:Result<(Vec<Board>,), String> = storage::stable_restore();
    match res {
        Ok((old_posts,)) => {
            for post in old_posts {
                board.push(post);
            }
            return;
        }
        Err(_) => return
    }
}
