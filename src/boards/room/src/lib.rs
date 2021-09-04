mod group;
mod message;
mod voice;

use std::vec;

use group::Group;
use message::Message;
use visa::Ticket;
use candid::{CandidType, Principal};
use serde::{Serialize, Deserialize};

const BASE_FEE: u32 = 1;
static COMMON_TITLE: &str = "Anonymous Room";
static COMMON_VIEW: &str = "https://partyboard.org/media/blog/blog_2.jpg";
static DEPOSIT: i32 = 0;

trait  Plugin<T> {
    fn did();
    fn disable(){}
}

#[derive(Debug, Deserialize, Serialize, CandidType)]
pub struct Room {
    pub id: String,
    pub title: String,
    pub cover: String,
    pub tickets: Vec<Ticket>,
    pub owner: Principal,
    pub moderator: Vec<String>,
    pub members: Vec<String>,
    pub groups: Vec<Group>,
    pub channel: String,
    pub messages: Vec<Message>,
}
impl Default for Room {
    fn default() -> Room{
        Room {
            id: Default::default(),
            title: String::from(COMMON_TITLE),
            cover: String::from(COMMON_VIEW),
            owner: Principal::anonymous(),
            members: vec![],
            tickets: vec![],
            moderator: vec![],
            groups: vec![],
            channel:  Default::default(),
            messages: vec![]
        }
    }

}

impl Room {

    fn build(&mut self, title: String, cover: String, owner: Principal, id: String){
        self.title = title;
        self.cover = cover;
        self.owner = owner;
        self.id = id;
    }

    pub fn add_member(&mut self, member_id: String){
        self.members.push(member_id)
    }
    pub fn can_start(&self, person: &String) -> bool{
        self.moderator.iter().any(|e| e == person)
    }
    pub fn with_channel(&mut self){}
    pub fn add_group(){}
    pub fn join_group(){}
    pub fn with_voice(&mut self){}
    pub fn with_scene(&mut self){}
    pub fn with_consensus(&mut self){}
    pub fn with_market(&mut self){}
}
