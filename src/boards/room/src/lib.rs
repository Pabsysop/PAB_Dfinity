mod group;
mod message;
mod voice;

use std::{fmt::Debug, vec};
use group::Group;
use message::Message;
use candid::{CandidType, Principal};
use serde::{Serialize, Deserialize};
use visa::Ticket;

static COMMON_TITLE: &str = "Anonymous Room";
static COMMON_VIEW: &str = "https://partyboard.org/media/blog/blog_2.jpg";

trait  Plugin<T> {
    fn did();
    fn disable(){}
}

#[derive(Debug, Deserialize, Serialize, CandidType, Clone)]
pub struct Room {
    pub id: String,
    pub title: String,
    pub cover: String,
    pub owner: Principal,
    pub allows: Vec<Principal>,
    pub tickets: Vec<Ticket>,
    pub moderators: Vec<Principal>,
    pub speakers: Vec<Principal>,
    pub audiens: Vec<Principal>,
    pub groups: Vec<Group>,
    pub messages: Vec<Message>,
    pub fee: f64
}

impl Default for Room {
    fn default() -> Room{
        Room {
            id: Default::default(),
            title: String::from(COMMON_TITLE),
            cover: String::from(COMMON_VIEW),
            owner: Principal::anonymous(),
            speakers: vec![],
            audiens: vec![],
            moderators: vec![],
            groups: vec![],
            messages: vec![],
            tickets: vec![],
            allows: Default::default(),
            fee: 0.0
        }
    }

}

impl Room {

    pub fn build(id: String, title: String, cover: Option<String>, owner: Principal) -> Room{
        Room {
            id,
            title,
            cover:  cover.unwrap_or(String::from(COMMON_VIEW)),
            owner,
            speakers: vec![owner],
            audiens: vec![],
            moderators: vec![owner],
            groups: vec![],
            messages: vec![],
            tickets: vec![],
            allows: vec![owner],
            fee: 0.0
        }
    }

    pub fn can_join(&self, person: &Principal, ticket: Option<Ticket>) -> bool {
        self.allows.contains(person)
    }
    
    pub fn add_group(){}
    pub fn join_group(){}
    pub fn with_voice(&mut self){}
    pub fn with_scene(&mut self){}
    pub fn with_consensus(&mut self){}
    pub fn with_market(&mut self){}
}
