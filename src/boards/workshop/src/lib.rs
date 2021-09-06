mod nft;
mod oracle;
mod record;
mod voice;
mod message;

use candid::{CandidType, Principal};
use serde::{Serialize, Deserialize};
use visa::Ticket;
use crate::message::Message;

static COMMON_TITLE: &str = "Anonymous Room";
static COMMON_VIEW: &str = "https://partyboard.org/media/blog/blog_2.jpg";

pub enum  WorkshopType {
    NFT,
    RECORD,
    MESSAGE
}

#[derive(Debug, Deserialize, Serialize, CandidType)]
pub struct Workshop {
    pub id: String,
    pub title: String,
    pub cover: String,
    pub tickets: Vec<Ticket>,
    pub owner: Principal,
    pub moderator: Vec<String>,
    pub members: Vec<String>,
    pub messages: Vec<Message>,
}

impl Default for Workshop {
    fn default() -> Workshop{
        Workshop {
            id: Default::default(),
            title: String::from(COMMON_TITLE),
            cover: String::from(COMMON_VIEW),
            members: vec![],
            tickets: vec![],
            owner: Principal::anonymous(),
            moderator: vec![],
            messages: vec![]
        }
    }

}

impl Workshop {

    fn build(&mut self, title: String, cover: String, owner: Principal, id: String){
        self.title = title;
        self.cover = cover;
        self.owner = owner;
        self.id = id;
    }

    pub fn add_member(&mut self, member_id: String){
        self.members.push(member_id)
    }

    pub fn can_start(&self, person: &String) -> bool {
        self.moderator.iter().any(|e| e == person)
    }
    
    pub fn with_channel(&mut self){}
    pub fn with_voice(&mut self){}
    pub fn with_consensus(&mut self){}
    pub fn with_market(&mut self){}
}
