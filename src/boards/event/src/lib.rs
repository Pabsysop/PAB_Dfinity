use visa::Ticket;
use candid::{CandidType, Principal};
use serde::{Serialize, Deserialize};

static COMMON_TITLE: &str = "Anonymous Event";
static COMMON_VIEW: &str = "https://partyboard.org/media/figure/post_9.jpg";

#[derive(Debug, Deserialize, Serialize, CandidType)]
pub struct Event {
    id: String,
    title: String,
    owner: Principal,
    cover: String,
    pub tickets: Vec<Ticket>,
    pub moderator: Vec<String>,
    pub members: Vec<String>,
}

impl Default for Event {
    fn default() -> Event{
        Event {
            id: Default::default(),
            title: String::from(COMMON_TITLE),
            cover: String::from(COMMON_VIEW),
            owner: Principal::anonymous(),
            members: vec![],
            tickets: vec![],
            moderator: vec![],
        }
    }

}

impl Event{

    pub fn add_member(&mut self, member_id: String){
        self.members.push(member_id)
    }
    pub fn can_start(&self, person: &String) -> bool{
        self.moderator.iter().any(|e| e == person)
    }
}
