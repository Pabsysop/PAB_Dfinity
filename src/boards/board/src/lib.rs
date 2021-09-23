use candid::{CandidType};
use serde::{Serialize, Deserialize};
use consensus::Consensus;

static COMMON_TITLE: &str = "Anonymous board";
static COMMON_VIEW: &str = "https://partyboard.org/media/blog/blog_2.jpg";

#[derive(Debug, Deserialize, Serialize, CandidType, Clone)]
pub struct Board {
    pub id: String,
    pub title: String,
    pub cover: String,
    consensuses: Consensus,
}

impl Default for Board {
    fn default() -> Board{
        Board {
            id: Default::default(),
            title: String::from(COMMON_TITLE),
            cover: String::from(COMMON_VIEW),
            consensuses: Consensus::POW,
        }
    }

}

impl Board {

    pub fn build(&mut self, title: String, cover: String, id: String) {
        self.title = title;
        self.cover = cover;
        self.id = id;
    }

    pub fn set_consensus(&mut self, con: Consensus){
        self.consensuses = con;
    }
}
