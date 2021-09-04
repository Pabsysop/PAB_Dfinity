use ic_cdk::export::Principal;
use url::Url;
use std::str::FromStr;
use visa::Ticket;
use uuid::Uuid;

const BASE_FEE: u32 = 1;
static COMMON_TITLE: &str = "Anonymous Event";
static COMMON_VIEW: &str = "https://partyboard.org/media/figure/post_9.jpg";

pub struct Event {
    id: String,
    title: String,
    owner: String,
    cover: String,
    pub tickets: Vec<Ticket>,
    pub moderator: Vec<String>,
    pub members: Vec<String>,
}

impl Event{

    pub fn default(owner: &str) -> Event {
        Event::build(String::from(COMMON_TITLE), String::from(COMMON_VIEW), String::from(owner))
    }

    fn build(title: String, cover: String, owner: String) -> Event {
        let event = Event {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            title,
            cover,
            members: vec![],
            tickets: vec![],
            owner,
            moderator: vec![],
        };

        event
    }

    pub fn add_member(&mut self, member_id: String){
        self.members.push(member_id)
    }
    pub fn can_start(&self, person: &String) -> bool{
        self.moderator.iter().any(|e| e == person)
    }
}
