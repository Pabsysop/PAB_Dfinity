mod assets;

use assets::Assets;
use Ontology::Ontology;
use serde::{Serialize, Deserialize};
use candid::{CandidType, Principal};
use ic_cdk::api::id;
use nft::NFT;

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Tag(Vec<NFT>);

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Honor(Vec<NFT>);

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Personality(Vec<NFT>);

static COMMON_NAME: &str = "@pab#";

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub enum Mood {
    Clear,
    Dizzy,
    Smart,
    Sleepwalking
}

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Connections {
    pub followers: Vec<(Principal, u64)>,
    pub followings: Vec<(Principal, u64)>,
}

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Human {
    pub name: String,
    pub ontology: Ontology,
    pub mood: Mood,
    pub tag: Tag,
    pub personality: Personality,
    pub honor: Honor,
    pub assets: Vec<Assets>,
    pub connections: Connections,
    pub value: f64
}

impl Default for Human {
    fn default() -> Self {
        Self { 
            name: Default::default(), 
            ontology: Default::default(), 
            mood: Mood::Clear,
            tag: Tag(vec![]),
            personality: Personality(vec![]), 
            honor: Honor(vec![]),
            assets: vec![],
            connections: Connections {
                    followers: vec![],
                    followings: vec![],
            },
            value: 0.0, 
        }
    }
}

impl Human {
    pub fn born(&mut self, lifeno: u64) {
        self.name = COMMON_NAME.to_string() + "#" + lifeno.to_string().as_str();
        self.ontology = Ontology::born(id().to_text());
        self.mood = Mood::Clear;

        self.ontology.set_gene(vec![0,0,1]);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn add_following(&mut self, f: Principal){
        if !self.connections.followings.iter()
        .any(|cf| cf.0 == f) {
            self.connections.followers.push((f,0))
        }
    }

    pub fn add_followers(&mut self, follower: Principal){
        if !self.connections.followers.iter()
        .any(|cf| cf.0 == follower) {
            self.connections.followers.push((follower,0))
        }
    }

}
