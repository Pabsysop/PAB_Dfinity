mod nft;
mod oracle;
mod record;
mod voice;
mod message;

use visa::Ticket;
use crate::message::Message;
use url::Url;
use uuid::Uuid;

static COMMON_TITLE: &str = "Anonymous Room";
static COMMON_VIEW: &str = "https://partyboard.org/media/blog/blog_2.jpg";

pub enum  WorkshopType {
    NFT,
    RECORD,
    MESSAGE
}

pub struct Workshop {
    pub id: String,
    pub title: String,
    pub cover: String,
    pub tickets: Vec<Ticket>,
    pub owner: String,
    pub moderator: Vec<String>,
    pub members: Vec<String>,
    pub messages: Vec<Message>,
}

impl Workshop {

    fn build(title: String, cover: String, owner: String) -> Result<Workshop, String> {
        let workshop = Workshop {
            id: Uuid::new_v4().to_hyphenated().to_string(),
            title,
            cover,
            members: vec![],
            tickets: vec![],
            owner,
            moderator: vec![],
            messages: vec![]
        };

        Ok(workshop)
    }

    pub fn default(owner: &str) -> Result<Workshop, String> {
        Workshop::build(String::from(COMMON_TITLE), String::from(COMMON_VIEW), String::from(owner))
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
