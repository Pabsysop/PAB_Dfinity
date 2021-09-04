use uuid::Uuid;

use crate::channel::Channel;
use crate::group::Group;
use crate::message::Message;
use visa::Ticket;

mod group;
mod channel;
mod message;
mod voice;

const BASE_FEE: u32 = 1;
static COMMON_TITLE: &str = "Anonymous Room";
static COMMON_VIEW: &str = "https://partyboard.org/media/blog/blog_2.jpg";
static DEPOSIT: i32 = 0;

trait  Plugin<T> {
    fn did();
    fn disable(){}
}

pub struct Room {
    pub id: String,
    pub title: String,
    pub cover: String,
    pub tickets: Vec<Ticket>,
    pub owner: String,
    pub moderator: Vec<String>,
    pub members: Vec<String>,
    pub groups: Vec<Group>,
    pub channel: Channel,
    pub messages: Vec<Message>,
}

impl Room {

    pub fn default(owner: &str) -> Result<Room, String> {
        Room::build(String::from(COMMON_TITLE), String::from(COMMON_VIEW), String::from(owner))
    }

    fn build(title: String, cover: String, owner: String) -> Result<Room, String> {
        let room = Room {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            title,
            cover,
            members: vec![],
            groups: vec![],
            channel: Channel { id: "".to_string(), session: "".to_string() },
            tickets: vec![],
            owner,
            moderator: vec![],
            messages: vec![]
        };
        Ok(room)
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
